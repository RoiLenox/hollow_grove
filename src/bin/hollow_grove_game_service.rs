//! Local transport host for the authoritative Hollow Grove gameplay service.
//!
//! TCP mode binds only to a loopback address. Stdio mode exists for launchers,
//! fixtures, and deterministic integration tests. Both transports use one JSON
//! request and one JSON response per line.

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;

use hollow_grove::constitutional::{RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{DEFAULT_GAMEPLAY_SERVICE_ADDRESS, GameProtocolService};

const DEFAULT_SESSION_ID: &str = "session.hollow-grove.local";

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
    let mut service = GameProtocolService::new_with_roots(
        options.session_id,
        rule_set,
        options.save_root,
        options.world_root,
    )?;
    match options.mode {
        TransportMode::Stdio => {
            let stdin = io::stdin();
            let stdout = io::stdout();
            serve_lines(stdin.lock(), stdout.lock(), &mut service)?;
        }
        TransportMode::Listen(address) => serve_loopback(address, &mut service)?,
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
    service: &mut GameProtocolService,
) -> Result<(), Box<dyn std::error::Error>> {
    require_loopback(address.ip())?;
    let listener = TcpListener::bind(address)?;
    eprintln!(
        "Hollow Grove gameplay service listening on {} for session {}",
        listener.local_addr()?,
        service.session_id()
    );
    for connection in listener.incoming() {
        let stream = connection?;
        if !stream.peer_addr()?.ip().is_loopback() {
            continue;
        }
        serve_stream(stream, service)?;
    }
    Ok(())
}

fn serve_stream(
    stream: TcpStream,
    service: &mut GameProtocolService,
) -> Result<(), Box<dyn std::error::Error>> {
    stream.set_nodelay(true)?;
    let reader = BufReader::new(stream.try_clone()?);
    let writer = BufWriter::new(stream);
    serve_lines(reader, writer, service)?;
    Ok(())
}

fn serve_lines(
    mut reader: impl BufRead,
    mut writer: impl Write,
    service: &mut GameProtocolService,
) -> io::Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        let request = line.trim_end_matches(['\r', '\n']);
        let response = service.handle_json_line(request);
        writer.write_all(response.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
    Ok(())
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
    use super::*;

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
}
