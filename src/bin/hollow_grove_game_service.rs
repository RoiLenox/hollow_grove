//! Local transport host for the authoritative Hollow Grove gameplay service.
//!
//! TCP mode binds only to a loopback address. Stdio mode exists for launchers,
//! fixtures, and deterministic integration tests. Both transports use one JSON
//! request and one JSON response per line.

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use hollow_grove::constitutional::{RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    DEFAULT_GAMEPLAY_SERVICE_ADDRESS, GameProtocolService, MAX_GAMEPLAY_MESSAGE_BYTES,
};

const DEFAULT_SESSION_ID: &str = "session.hollow-grove.local";
const MAX_CONCURRENT_CLIENTS: usize = 16;
const CLIENT_IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const CLIENT_WRITE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
enum TransportMode {
    Stdio,
    Listen(SocketAddr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Options {
    mode: TransportMode,
    session_id: String,
    save_root: PathBuf,
    world_root: PathBuf,
}

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("hollow-grove game service failed: {error}");
        std::process::exit(1);
    }
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<(), Box<dyn std::error::Error>> {
    let Some(options) = parse_options(arguments)? else {
        print_help();
        return Ok(());
    };
    let rule_set = RuleSetId::new(V2_RULE_SET)?;
    let service = GameProtocolService::new_with_roots(
        options.session_id,
        rule_set,
        options.save_root,
        options.world_root,
    )?;
    match options.mode {
        TransportMode::Stdio => {
            let mut service = service;
            let stdin = io::stdin();
            let stdout = io::stdout();
            serve_lines(stdin.lock(), stdout.lock(), &mut service)?;
        }
        TransportMode::Listen(address) => serve_loopback(address, service)?,
    }
    Ok(())
}

fn parse_options(arguments: impl IntoIterator<Item = String>) -> Result<Option<Options>, String> {
    let default_address = DEFAULT_GAMEPLAY_SERVICE_ADDRESS
        .parse::<SocketAddr>()
        .expect("the canonical local gameplay address is valid");
    let mut options = Options {
        mode: TransportMode::Listen(default_address),
        session_id: DEFAULT_SESSION_ID.into(),
        save_root: PathBuf::from("artifacts/gameplay-saves"),
        world_root: PathBuf::from("."),
    };
    let mut arguments = arguments.into_iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--help" | "-h" => return Ok(None),
            "--stdio" => options.mode = TransportMode::Stdio,
            "--listen" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--listen requires a socket address".to_owned())?;
                let address = value
                    .parse::<SocketAddr>()
                    .map_err(|_| format!("invalid --listen socket address: {value}"))?;
                require_loopback(address.ip())?;
                options.mode = TransportMode::Listen(address);
            }
            "--session" => {
                options.session_id = arguments
                    .next()
                    .ok_or_else(|| "--session requires a stable session ID".to_owned())?;
            }
            "--save-root" => {
                options.save_root = PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--save-root requires a directory".to_owned())?,
                );
            }
            "--world-root" => {
                options.world_root = PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--world-root requires a directory".to_owned())?,
                );
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }
    if let TransportMode::Listen(address) = options.mode {
        require_loopback(address.ip())?;
    }
    Ok(Some(options))
}

fn require_loopback(ip: IpAddr) -> Result<(), String> {
    if ip.is_loopback() {
        Ok(())
    } else {
        Err(format!(
            "gameplay service refuses non-loopback listen address {ip}"
        ))
    }
}

fn serve_loopback(
    address: SocketAddr,
    service: GameProtocolService,
) -> Result<(), Box<dyn std::error::Error>> {
    require_loopback(address.ip())?;
    let listener = TcpListener::bind(address)?;
    let service = Arc::new(Mutex::new(service));
    let active_clients = Arc::new(AtomicUsize::new(0));
    eprintln!(
        "Hollow Grove gameplay service listening on {} for session {}",
        listener.local_addr()?,
        service
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .session_id()
    );
    for connection in listener.incoming() {
        let stream = connection?;
        if !stream.peer_addr()?.ip().is_loopback() {
            continue;
        }
        let Some(permit) = ConnectionPermit::acquire(Arc::clone(&active_clients)) else {
            eprintln!(
                "Hollow Grove gameplay service refused a client because {} are already active",
                MAX_CONCURRENT_CLIENTS
            );
            continue;
        };
        let service = Arc::clone(&service);
        thread::Builder::new()
            .name("hollow-grove-game-client".into())
            .spawn(move || {
                let _permit = permit;
                if let Err(error) = serve_stream(stream, service)
                    && !matches!(
                        error.kind(),
                        io::ErrorKind::TimedOut
                            | io::ErrorKind::WouldBlock
                            | io::ErrorKind::ConnectionAborted
                            | io::ErrorKind::ConnectionReset
                            | io::ErrorKind::BrokenPipe
                    )
                {
                    eprintln!("Hollow Grove gameplay client disconnected: {error}");
                }
            })?;
    }
    Ok(())
}

struct ConnectionPermit {
    active_clients: Arc<AtomicUsize>,
}

impl ConnectionPermit {
    fn acquire(active_clients: Arc<AtomicUsize>) -> Option<Self> {
        active_clients
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                (count < MAX_CONCURRENT_CLIENTS).then_some(count + 1)
            })
            .ok()
            .map(|_| Self { active_clients })
    }
}

impl Drop for ConnectionPermit {
    fn drop(&mut self) {
        self.active_clients.fetch_sub(1, Ordering::AcqRel);
    }
}

fn serve_stream(stream: TcpStream, service: Arc<Mutex<GameProtocolService>>) -> io::Result<()> {
    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(CLIENT_IDLE_TIMEOUT))?;
    stream.set_write_timeout(Some(CLIENT_WRITE_TIMEOUT))?;
    let reader = BufReader::new(stream.try_clone()?);
    let writer = BufWriter::new(stream);
    serve_shared_lines(reader, writer, service)
}

fn serve_shared_lines(
    reader: impl BufRead,
    writer: impl Write,
    service: Arc<Mutex<GameProtocolService>>,
) -> io::Result<()> {
    serve_lines_with(reader, writer, |request| {
        service
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .handle_json_line(request)
    })
}

fn serve_lines(
    reader: impl BufRead,
    writer: impl Write,
    service: &mut GameProtocolService,
) -> io::Result<()> {
    serve_lines_with(reader, writer, |request| service.handle_json_line(request))
}

fn serve_lines_with(
    mut reader: impl BufRead,
    mut writer: impl Write,
    mut handle: impl FnMut(&str) -> String,
) -> io::Result<()> {
    let mut line = Vec::with_capacity(MAX_GAMEPLAY_MESSAGE_BYTES.min(8 * 1024));
    loop {
        let request = match read_protocol_line(&mut reader, &mut line)? {
            ProtocolLine::EndOfStream => break,
            ProtocolLine::Message(request) | ProtocolLine::Oversized(request) => request,
        };
        let response = handle(&request);
        writer.write_all(response.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
    Ok(())
}

enum ProtocolLine {
    EndOfStream,
    Message(String),
    Oversized(String),
}

fn read_protocol_line(reader: &mut impl BufRead, line: &mut Vec<u8>) -> io::Result<ProtocolLine> {
    line.clear();
    let wire_limit = MAX_GAMEPLAY_MESSAGE_BYTES + 3;
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return if line.is_empty() {
                Ok(ProtocolLine::EndOfStream)
            } else {
                Ok(finish_protocol_line(line, false))
            };
        }

        let chunk_length = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let copy_length = chunk_length.min(wire_limit - line.len());
        line.extend_from_slice(&available[..copy_length]);
        reader.consume(copy_length);

        if copy_length == chunk_length && line.last() == Some(&b'\n') {
            return Ok(finish_protocol_line(line, true));
        }
        if line.len() == wire_limit {
            drain_through_newline(reader)?;
            return Ok(finish_protocol_line(line, false));
        }
    }
}

fn finish_protocol_line(line: &mut Vec<u8>, terminated: bool) -> ProtocolLine {
    if terminated {
        line.pop();
        if line.last() == Some(&b'\r') {
            line.pop();
        }
    }
    if line.len() > MAX_GAMEPLAY_MESSAGE_BYTES {
        line.truncate(MAX_GAMEPLAY_MESSAGE_BYTES + 1);
        return ProtocolLine::Oversized(String::from_utf8_lossy(line).into_owned());
    }

    ProtocolLine::Message(String::from_utf8_lossy(line).into_owned())
}

fn drain_through_newline(reader: &mut impl BufRead) -> io::Result<()> {
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return Ok(());
        }
        let consumed = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let terminated = consumed <= available.len() && available.get(consumed - 1) == Some(&b'\n');
        reader.consume(consumed);
        if terminated {
            return Ok(());
        }
    }
}

fn print_help() {
    println!(
        "Hollow Grove authoritative gameplay service\n\n\
         Usage:\n\
           hollow_grove_game_service [--listen 127.0.0.1:PORT] [--session ID] [--save-root DIR] [--world-root DIR]\n\
           hollow_grove_game_service --stdio [--session ID] [--save-root DIR] [--world-root DIR]\n\n\
         The service accepts one protocol-V1 JSON request per line. TCP mode\n\
         is restricted to loopback interfaces."
    );
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};
    use std::sync::mpsc::{Receiver, Sender};

    use super::*;
    use hollow_grove::gameplay::{ProtocolResponseEnvelope, ProtocolResponseStatus};

    fn test_service(session_id: &str) -> GameProtocolService {
        GameProtocolService::new(
            session_id,
            RuleSetId::new(V2_RULE_SET).expect("V2 rule set is valid"),
        )
        .expect("test service is valid")
    }

    fn sync_request(session_id: &str, request_id: &str) -> String {
        serde_json::json!({
            "protocol_version": 1,
            "session_id": session_id,
            "request_id": request_id,
            "expected_revision": 0,
            "intent": {"type": "SyncIntent"}
        })
        .to_string()
    }

    #[test]
    fn defaults_to_the_canonical_loopback_service() {
        let options = parse_options(Vec::new()).unwrap().unwrap();
        assert_eq!(options.session_id, DEFAULT_SESSION_ID);
        assert_eq!(options.save_root, PathBuf::from("artifacts/gameplay-saves"));
        assert_eq!(options.world_root, PathBuf::from("."));
        assert_eq!(
            options.mode,
            TransportMode::Listen(DEFAULT_GAMEPLAY_SERVICE_ADDRESS.parse().unwrap())
        );
    }

    #[test]
    fn stdio_and_session_are_explicitly_selectable() {
        let options = parse_options([
            "--stdio".into(),
            "--session".into(),
            "session.test".into(),
            "--save-root".into(),
            "/tmp/hollow-grove-test-saves".into(),
            "--world-root".into(),
            "/tmp/hollow-grove-test-world".into(),
        ])
        .unwrap()
        .unwrap();
        assert_eq!(options.mode, TransportMode::Stdio);
        assert_eq!(options.session_id, "session.test");
        assert_eq!(
            options.save_root,
            PathBuf::from("/tmp/hollow-grove-test-saves")
        );
        assert_eq!(
            options.world_root,
            PathBuf::from("/tmp/hollow-grove-test-world")
        );
    }

    #[test]
    fn non_loopback_listener_is_rejected() {
        let error = parse_options(["--listen".into(), "0.0.0.0:47819".into()]).unwrap_err();
        assert!(error.contains("non-loopback"));
    }

    #[test]
    fn oversized_line_is_bounded_and_the_following_request_is_still_served() {
        let mut input = vec![b'x'; MAX_GAMEPLAY_MESSAGE_BYTES + 256];
        input.push(b'\n');
        input.extend_from_slice(sync_request("session.bounded", "request.after-limit").as_bytes());
        input.push(b'\n');

        let mut output = Vec::new();
        serve_lines(
            Cursor::new(input),
            &mut output,
            &mut test_service("session.bounded"),
        )
        .expect("bounded input should be served");

        let responses = String::from_utf8(output).expect("responses are UTF-8");
        let responses = responses
            .lines()
            .map(|line| {
                serde_json::from_str::<ProtocolResponseEnvelope>(line)
                    .expect("response should use the protocol envelope")
            })
            .collect::<Vec<_>>();
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[0].status, ProtocolResponseStatus::Rejected);
        assert_eq!(responses[0].request_id, "unparseable");
        assert_eq!(responses[1].status, ProtocolResponseStatus::Completed);
        assert_eq!(responses[1].request_id, "request.after-limit");
    }

    struct BlockingReader {
        ready: Option<Sender<()>>,
        release: Receiver<()>,
    }

    impl Read for BlockingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            if let Some(ready) = self.ready.take() {
                ready.send(()).expect("idle reader should report readiness");
            }
            self.release
                .recv()
                .expect("idle reader should receive release");
            Ok(0)
        }
    }

    #[test]
    fn idle_client_does_not_monopolize_the_authoritative_service() {
        let (idle_ready_sender, idle_ready_receiver) = std::sync::mpsc::channel();
        let (idle_release_sender, idle_release_receiver) = std::sync::mpsc::channel();
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        let service = Arc::new(Mutex::new(test_service("session.concurrent")));

        let idle_reader = BufReader::new(BlockingReader {
            ready: Some(idle_ready_sender),
            release: idle_release_receiver,
        });
        let idle_service = Arc::clone(&service);
        let idle_thread =
            thread::spawn(move || serve_shared_lines(idle_reader, io::sink(), idle_service));
        idle_ready_receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("idle client should begin waiting for a request");

        let request = sync_request("session.concurrent", "request.concurrent");
        let active_service = Arc::clone(&service);
        thread::spawn(move || {
            let mut response = Vec::new();
            serve_shared_lines(
                Cursor::new(format!("{request}\n")),
                &mut response,
                active_service,
            )
            .expect("active request should be served");
            response_sender
                .send(response)
                .expect("active response should return to the test");
        });

        let response = response_receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("active response should arrive while the first client is idle");
        let response = String::from_utf8(response).expect("active response should be UTF-8");
        let response: ProtocolResponseEnvelope =
            serde_json::from_str(&response).expect("active response should parse");
        assert_eq!(response.status, ProtocolResponseStatus::Completed);

        idle_release_sender
            .send(())
            .expect("idle client should be released");
        idle_thread
            .join()
            .expect("idle client thread should not panic")
            .expect("idle client should close cleanly");
    }
}
