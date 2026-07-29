use std::fs::{self, OpenOptions};
use std::future::Future;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::composition::{CompositionCatalog, ExternalRef, ScaleKey};
use hollow_grove::runtime_federation::{
    DEFAULT_FEDERATION_HTTP_ADDRESS, DEFAULT_HOLLOW_GROVE_OLLAMA_MODEL, DEFAULT_OLLAMA_URL,
    DeterministicSemanticInterpreter, FEDERATION_HTTP_API_SCHEMA_VERSION,
    FederationApiCycleRequest, FederationHttpApi, HOLLOW_GROVE_MODEL_ENV,
    HOLLOW_GROVE_OLLAMA_URL_ENV, LocalFederationCycleInput, MAX_FEDERATION_HTTP_REQUEST_BODY_BYTES,
    OllamaConfig, OllamaSemanticInterpreter, SemanticDecision, SemanticInterpreter,
    parse_model_semantic_proposal, render_hollow_grove_ollama_modelfile,
    run_local_federation_cycle, validate_ollama_model_reference,
};
use hollow_grove::world::composition_witnesses::canonical_composition_witness_catalog;
use hollow_grove::{
    Position3, decode_semantic_provenance_snapshot, decode_semantic_witness_jsonl,
    encode_semantic_provenance_snapshot, format_semantic_dataset_record, inspect_semantic_dataset,
    semantic_dataset_history, semantic_dataset_nearby, validate_semantic_witness_jsonl,
    write_bytes_atomically, write_semantic_dataset_split,
};

const DEFAULT_SEMANTIC_FEDERATION_STATE_PATH: &str = "artifacts/semantic_federation_state_v1.json";
const FEDERATION_CLI_RESULT_SCHEMA_VERSION: &str = "hollow-grove-federation-cli-result-v1";
const MAX_FEDERATION_HTTP_HEADER_BYTES: usize = 16_384;
const MAX_FEDERATION_HTTP_CLIENTS: usize = 16;
const FEDERATION_HTTP_CLIENT_TIMEOUT: Duration = Duration::from_secs(70);
const FEDERATION_HTTP_WRITE_TIMEOUT: Duration = Duration::from_secs(10);

fn usage() -> &'static str {
    "Usage: hollow_grove_semantic <semantic|federation|model|dataset> <command> [args]\n\
     \n\
     Commands:\n\
       semantic inspect <jsonl> <point-ref> <scale> [--json]\n\
       semantic history <jsonl> <point-ref> <scale> [--json]\n\
       semantic nearby <jsonl> <scale> <x> <y> <z> [limit] [--json]\n\
       federation run <point-ref> <scale> <intent> [--state <path>] [--proposal-json <path>] [--json]\n\
       federation serve [--listen 127.0.0.1:PORT] [--state <path>]\n\
       model show-modelfile <base-model>\n\
       model install-ollama <base-model> [model-name]\n\
       model validate-response <json> [--json]\n\
       dataset validate-semantic <jsonl>\n\
       dataset split-semantic <jsonl> <train-jsonl> <validation-jsonl> <seed> [validation-millis]\n\
     \n\
     All semantic dataset commands are read-only. Canonical mutation remains\n\
     exclusively inside Federation and Festival. Federation uses Ollama unless\n\
     --proposal-json supplies an offline deterministic proposal for testing.\n\
     The HTTP service is restricted to loopback interfaces."
}

struct TemporaryModelfile {
    path: std::path::PathBuf,
}

impl TemporaryModelfile {
    fn create(contents: &str) -> io::Result<Self> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| io::Error::other(error.to_string()))?
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "hollow-grove-Modelfile-{}-{nonce}",
            std::process::id()
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
        Ok(Self { path })
    }
}

impl Drop for TemporaryModelfile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn read_dataset(path: &str) -> io::Result<Vec<hollow_grove::SemanticWitnessDatasetRecord>> {
    let bytes = fs::read(Path::new(path))?;
    decode_semantic_witness_jsonl(&bytes).map_err(|error| invalid(error.to_string()))
}

fn has_json_flag(args: &[String]) -> bool {
    args.last().is_some_and(|argument| argument == "--json")
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FederationRunOptions {
    point_ref: String,
    scale: String,
    intent: String,
    state_path: PathBuf,
    proposal_path: Option<PathBuf>,
    json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FederationServeOptions {
    listen: SocketAddr,
    state_path: PathBuf,
}

fn parse_federation_run_options(args: &[String]) -> io::Result<FederationRunOptions> {
    let mut positional = Vec::new();
    let mut state_path = PathBuf::from(DEFAULT_SEMANTIC_FEDERATION_STATE_PATH);
    let mut proposal_path = None;
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--json" if !json => {
                json = true;
                index += 1;
            }
            "--state" if index + 1 < args.len() => {
                state_path = PathBuf::from(&args[index + 1]);
                index += 2;
            }
            "--proposal-json" if index + 1 < args.len() => {
                proposal_path = Some(PathBuf::from(&args[index + 1]));
                index += 2;
            }
            value if value.starts_with("--") => {
                return Err(invalid(format!("unknown federation option: {value}")));
            }
            value => {
                positional.push(value.to_owned());
                index += 1;
            }
        }
    }
    if positional.len() != 3 {
        return Err(invalid(usage()));
    }
    Ok(FederationRunOptions {
        point_ref: positional.remove(0),
        scale: positional.remove(0),
        intent: positional.remove(0),
        state_path,
        proposal_path,
        json,
    })
}

fn parse_federation_serve_options(args: &[String]) -> io::Result<FederationServeOptions> {
    let mut listen = DEFAULT_FEDERATION_HTTP_ADDRESS
        .parse::<SocketAddr>()
        .expect("the canonical Federation HTTP address is valid");
    let mut state_path = PathBuf::from(DEFAULT_SEMANTIC_FEDERATION_STATE_PATH);
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--listen" if index + 1 < args.len() => {
                listen = args[index + 1]
                    .parse::<SocketAddr>()
                    .map_err(|_| invalid("listen address must be IP:PORT"))?;
                require_loopback(listen.ip())?;
                index += 2;
            }
            "--state" if index + 1 < args.len() => {
                state_path = PathBuf::from(&args[index + 1]);
                index += 2;
            }
            value => {
                return Err(invalid(format!(
                    "unknown or incomplete federation serve option: {value}"
                )));
            }
        }
    }
    require_loopback(listen.ip())?;
    Ok(FederationServeOptions { listen, state_path })
}

fn require_loopback(ip: IpAddr) -> io::Result<()> {
    if ip.is_loopback() {
        Ok(())
    } else {
        Err(invalid(format!(
            "Federation HTTP service refuses non-loopback listen address {ip}"
        )))
    }
}

fn parse_external_reference(value: &str) -> io::Result<ExternalRef> {
    let Some((namespace, key)) = value.split_once('.') else {
        return Err(invalid(
            "point-ref must contain a namespace and key, for example point.hueman.origin",
        ));
    };
    ExternalRef::new(namespace, key).map_err(|error| invalid(error.to_string()))
}

fn load_federation_catalog(path: &Path) -> io::Result<CompositionCatalog> {
    let mut catalog = canonical_composition_witness_catalog()
        .map_err(|error| io::Error::other(error.to_string()))?;
    match fs::read(path) {
        Ok(bytes) => {
            decode_semantic_provenance_snapshot(&bytes, &mut catalog)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    Ok(catalog)
}

fn ollama_config() -> io::Result<OllamaConfig> {
    let model = std::env::var(HOLLOW_GROVE_MODEL_ENV)
        .unwrap_or_else(|_| DEFAULT_HOLLOW_GROVE_OLLAMA_MODEL.into());
    let endpoint =
        std::env::var(HOLLOW_GROVE_OLLAMA_URL_ENV).unwrap_or_else(|_| DEFAULT_OLLAMA_URL.into());
    OllamaConfig::new(endpoint, model, Duration::from_secs(60))
        .map_err(|error| invalid(error.to_string()))
}

#[derive(Debug)]
struct FederationHttpRequest {
    method: String,
    path: String,
    content_type: Option<String>,
    body: Vec<u8>,
}

#[derive(Debug)]
struct FederationHttpProtocolError {
    status: u16,
    error_type: &'static str,
    message: String,
}

impl FederationHttpProtocolError {
    fn new(status: u16, error_type: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            error_type,
            message: message.into(),
        }
    }
}

fn read_federation_http_request(
    reader: &mut impl BufRead,
) -> Result<FederationHttpRequest, FederationHttpProtocolError> {
    let headers = read_federation_http_headers(reader)?;
    let headers = std::str::from_utf8(&headers).map_err(|_| {
        FederationHttpProtocolError::new(400, "malformed_http", "HTTP headers must be UTF-8")
    })?;
    let mut lines = headers.split("\r\n");
    let request_line = lines.next().ok_or_else(|| {
        FederationHttpProtocolError::new(400, "malformed_http", "missing HTTP request line")
    })?;
    let request_parts = request_line.split_whitespace().collect::<Vec<_>>();
    if request_parts.len() != 3
        || !matches!(request_parts[2], "HTTP/1.0" | "HTTP/1.1")
        || request_parts[0].is_empty()
        || request_parts[1].is_empty()
    {
        return Err(FederationHttpProtocolError::new(
            400,
            "malformed_http",
            "invalid HTTP request line",
        ));
    }

    let mut content_length = None;
    let mut content_type = None;
    let mut transfer_encoding = None;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let Some((name, value)) = line.split_once(':') else {
            return Err(FederationHttpProtocolError::new(
                400,
                "malformed_http",
                "invalid HTTP header",
            ));
        };
        let name = name.trim();
        let value = value.trim();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            return Err(FederationHttpProtocolError::new(
                400,
                "malformed_http",
                "invalid HTTP header name",
            ));
        }
        if name.eq_ignore_ascii_case("content-length") {
            if content_length.is_some() {
                return Err(FederationHttpProtocolError::new(
                    400,
                    "malformed_http",
                    "duplicate Content-Length",
                ));
            }
            content_length = Some(value.parse::<usize>().map_err(|_| {
                FederationHttpProtocolError::new(400, "malformed_http", "invalid Content-Length")
            })?);
        } else if name.eq_ignore_ascii_case("content-type") {
            if content_type.is_some() {
                return Err(FederationHttpProtocolError::new(
                    400,
                    "malformed_http",
                    "duplicate Content-Type",
                ));
            }
            content_type = Some(value.to_owned());
        } else if name.eq_ignore_ascii_case("transfer-encoding") {
            transfer_encoding = Some(value.to_owned());
        }
    }
    if transfer_encoding.is_some() {
        return Err(FederationHttpProtocolError::new(
            400,
            "unsupported_transfer_encoding",
            "Transfer-Encoding is not supported; send Content-Length",
        ));
    }
    let body_length = content_length.unwrap_or(0);
    if body_length > MAX_FEDERATION_HTTP_REQUEST_BODY_BYTES {
        return Err(FederationHttpProtocolError::new(
            413,
            "request_too_large",
            format!(
                "request body exceeds {} bytes",
                MAX_FEDERATION_HTTP_REQUEST_BODY_BYTES
            ),
        ));
    }
    if request_parts[0] == "POST" && content_length.is_none() {
        return Err(FederationHttpProtocolError::new(
            411,
            "content_length_required",
            "POST requests require Content-Length",
        ));
    }
    let mut body = vec![0; body_length];
    reader.read_exact(&mut body).map_err(|error| {
        FederationHttpProtocolError::new(
            400,
            "malformed_http",
            format!("could not read complete request body: {error}"),
        )
    })?;
    Ok(FederationHttpRequest {
        method: request_parts[0].into(),
        path: request_parts[1].into(),
        content_type,
        body,
    })
}

fn read_federation_http_headers(
    reader: &mut impl BufRead,
) -> Result<Vec<u8>, FederationHttpProtocolError> {
    let mut headers = Vec::with_capacity(1024);
    loop {
        let available = reader.fill_buf().map_err(|error| {
            FederationHttpProtocolError::new(
                400,
                "malformed_http",
                format!("could not read HTTP headers: {error}"),
            )
        })?;
        if available.is_empty() {
            return Err(FederationHttpProtocolError::new(
                400,
                "malformed_http",
                "connection closed before HTTP headers completed",
            ));
        }
        let mut consumed = 0;
        let mut completed = false;
        for byte in available {
            if headers.len() == MAX_FEDERATION_HTTP_HEADER_BYTES {
                return Err(FederationHttpProtocolError::new(
                    431,
                    "headers_too_large",
                    format!(
                        "HTTP headers exceed {} bytes",
                        MAX_FEDERATION_HTTP_HEADER_BYTES
                    ),
                ));
            }
            headers.push(*byte);
            consumed += 1;
            if headers.ends_with(b"\r\n\r\n") {
                completed = true;
                break;
            }
        }
        reader.consume(consumed);
        if completed {
            return Ok(headers);
        }
    }
}

fn content_type_is_json(content_type: Option<&str>) -> bool {
    content_type.is_some_and(|value| {
        value
            .split(';')
            .next()
            .is_some_and(|mime| mime.trim().eq_ignore_ascii_case("application/json"))
    })
}

fn federation_http_error_json(error_type: &str, message: impl Into<String>) -> serde_json::Value {
    serde_json::json!({
        "error": {
            "type": error_type,
            "message": message.into(),
        }
    })
}

fn federation_http_response(
    status: u16,
    body: &serde_json::Value,
    allow: Option<&str>,
) -> io::Result<Vec<u8>> {
    let body = serde_json::to_vec(body).map_err(|error| io::Error::other(error.to_string()))?;
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        411 => "Length Required",
        413 => "Content Too Large",
        415 => "Unsupported Media Type",
        422 => "Unprocessable Content",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Error",
    };
    let mut response = format!(
        "HTTP/1.1 {status} {reason}\r\n\
         Content-Type: application/json; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         Cache-Control: no-store\r\n\
         X-Content-Type-Options: nosniff\r\n",
        body.len()
    );
    if let Some(allow) = allow {
        response.push_str("Allow: ");
        response.push_str(allow);
        response.push_str("\r\n");
    }
    response.push_str("\r\n");
    let mut wire = response.into_bytes();
    wire.extend_from_slice(&body);
    Ok(wire)
}

fn handle_federation_http_request<I>(
    request: FederationHttpRequest,
    api: &Arc<Mutex<FederationHttpApi<I>>>,
    model: &str,
) -> io::Result<Vec<u8>>
where
    I: SemanticInterpreter + Clone,
{
    let response = match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/health" | "/v1/health") => {
            let api = api
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            federation_http_response(
                200,
                &serde_json::json!({
                    "schema_version": FEDERATION_HTTP_API_SCHEMA_VERSION,
                    "status": "ok",
                    "authority": "rust_festival",
                    "model": model,
                    "state_path": api.state_path().display().to_string(),
                }),
                None,
            )
        }
        ("GET", "/v1/models") => federation_http_response(
            200,
            &serde_json::json!({
                "object": "list",
                "data": [{
                    "id": model,
                    "object": "model",
                    "owned_by": "hollow-grove",
                    "role": "untrusted_semantic_interpreter"
                }]
            }),
            None,
        ),
        ("POST", "/v1/federation/cycles") => {
            if !content_type_is_json(request.content_type.as_deref()) {
                federation_http_response(
                    415,
                    &federation_http_error_json(
                        "unsupported_media_type",
                        "Content-Type must be application/json",
                    ),
                    None,
                )
            } else {
                match serde_json::from_slice::<FederationApiCycleRequest>(&request.body) {
                    Ok(request) => {
                        let result = {
                            let mut api = api
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner);
                            block_on(api.execute(request))
                        };
                        match result {
                            Ok(result) => federation_http_response(
                                200,
                                &serde_json::to_value(result)
                                    .map_err(|error| io::Error::other(error.to_string()))?,
                                None,
                            ),
                            Err(error) => {
                                let status = error.http_status();
                                let error_type = error.error_type();
                                let message = if status >= 500 && status != 502 {
                                    "the local Federation service could not complete the request"
                                        .into()
                                } else {
                                    error.to_string()
                                };
                                eprintln!("Hollow Grove Federation API request failed: {error}");
                                federation_http_response(
                                    status,
                                    &federation_http_error_json(error_type, message),
                                    None,
                                )
                            }
                        }
                    }
                    Err(error) => federation_http_response(
                        400,
                        &federation_http_error_json(
                            "invalid_request",
                            format!("request JSON does not match the V1 schema: {error}"),
                        ),
                        None,
                    ),
                }
            }
        }
        ("GET", "/v1/federation/cycles") => federation_http_response(
            405,
            &federation_http_error_json("method_not_allowed", "use POST for /v1/federation/cycles"),
            Some("POST"),
        ),
        (_, "/health" | "/v1/health" | "/v1/models" | "/v1/federation/cycles") => {
            federation_http_response(
                405,
                &federation_http_error_json(
                    "method_not_allowed",
                    "the requested HTTP method is not supported for this route",
                ),
                Some("GET, POST"),
            )
        }
        _ => federation_http_response(
            404,
            &federation_http_error_json("not_found", "unknown Hollow Grove API route"),
            None,
        ),
    }?;
    Ok(response)
}

fn serve_federation_http_stream<I>(
    stream: TcpStream,
    api: Arc<Mutex<FederationHttpApi<I>>>,
    model: Arc<str>,
) -> io::Result<()>
where
    I: SemanticInterpreter + Clone,
{
    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(FEDERATION_HTTP_CLIENT_TIMEOUT))?;
    stream.set_write_timeout(Some(FEDERATION_HTTP_WRITE_TIMEOUT))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    let response = match read_federation_http_request(&mut reader) {
        Ok(request) => handle_federation_http_request(request, &api, &model)?,
        Err(error) => federation_http_response(
            error.status,
            &federation_http_error_json(error.error_type, error.message),
            None,
        )?,
    };
    writer.write_all(&response)?;
    writer.flush()
}

struct FederationHttpConnectionPermit {
    active_clients: Arc<AtomicUsize>,
}

impl FederationHttpConnectionPermit {
    fn acquire(active_clients: Arc<AtomicUsize>) -> Option<Self> {
        active_clients
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                (count < MAX_FEDERATION_HTTP_CLIENTS).then_some(count + 1)
            })
            .ok()
            .map(|_| Self { active_clients })
    }
}

impl Drop for FederationHttpConnectionPermit {
    fn drop(&mut self) {
        self.active_clients.fetch_sub(1, Ordering::AcqRel);
    }
}

fn serve_federation_http_loopback<I>(
    address: SocketAddr,
    api: FederationHttpApi<I>,
    model: impl Into<Arc<str>>,
) -> io::Result<()>
where
    I: SemanticInterpreter + Clone + 'static,
{
    require_loopback(address.ip())?;
    let listener = TcpListener::bind(address)?;
    let local_address = listener.local_addr()?;
    let api = Arc::new(Mutex::new(api));
    let model = model.into();
    let active_clients = Arc::new(AtomicUsize::new(0));
    eprintln!(
        "Hollow Grove Federation HTTP API listening on http://{local_address}\n\
         POST /v1/federation/cycles\n\
         GET  /v1/models\n\
         GET  /health"
    );
    for connection in listener.incoming() {
        let stream = connection?;
        if !stream.peer_addr()?.ip().is_loopback() {
            continue;
        }
        let Some(permit) = FederationHttpConnectionPermit::acquire(Arc::clone(&active_clients))
        else {
            let mut stream = stream;
            let response = federation_http_response(
                429,
                &federation_http_error_json(
                    "too_many_requests",
                    "too many concurrent local clients",
                ),
                None,
            )?;
            let _ = stream.write_all(&response);
            continue;
        };
        let api = Arc::clone(&api);
        let model = Arc::clone(&model);
        thread::Builder::new()
            .name("hollow-grove-federation-http".into())
            .spawn(move || {
                let _permit = permit;
                if let Err(error) = serve_federation_http_stream(stream, api, model)
                    && !matches!(
                        error.kind(),
                        io::ErrorKind::TimedOut
                            | io::ErrorKind::WouldBlock
                            | io::ErrorKind::ConnectionAborted
                            | io::ErrorKind::ConnectionReset
                            | io::ErrorKind::BrokenPipe
                    )
                {
                    eprintln!("Hollow Grove Federation HTTP client disconnected: {error}");
                }
            })?;
    }
    Ok(())
}

fn run_federation(args: &[String]) -> io::Result<()> {
    match args.first().map(String::as_str) {
        Some("help" | "--help" | "-h") | None => {
            println!("{}", usage());
            Ok(())
        }
        Some("run") => {
            let options = parse_federation_run_options(&args[1..])?;
            let point_ref = parse_external_reference(&options.point_ref)?;
            let scale =
                ScaleKey::new(options.scale.clone()).map_err(|error| invalid(error.to_string()))?;
            let input = LocalFederationCycleInput::new(point_ref, scale, options.intent.clone())
                .map_err(|error| invalid(error.to_string()))?;
            let mut catalog = load_federation_catalog(&options.state_path)?;
            let result = if let Some(proposal_path) = &options.proposal_path {
                let proposal = fs::read(proposal_path)?;
                block_on(run_local_federation_cycle(
                    DeterministicSemanticInterpreter::from_json(&proposal),
                    &input,
                    &mut catalog,
                ))
            } else {
                let interpreter = OllamaSemanticInterpreter::new(ollama_config()?)
                    .map_err(|error| invalid(error.to_string()))?;
                block_on(run_local_federation_cycle(
                    interpreter,
                    &input,
                    &mut catalog,
                ))
            }
            .map_err(|error| io::Error::other(error.to_string()))?;
            let snapshot = encode_semantic_provenance_snapshot(&catalog)
                .map_err(|error| io::Error::other(error.to_string()))?;
            write_bytes_atomically(&options.state_path, &snapshot)?;
            if options.json {
                println!(
                    "{}",
                    serde_json::to_string(&federation_result_json(&result, &options.state_path))
                        .map_err(|error| io::Error::other(error.to_string()))?
                );
            } else {
                print_federation_result(&result, &options.state_path);
            }
            Ok(())
        }
        Some("serve") => {
            let options = parse_federation_serve_options(&args[1..])?;
            let config = ollama_config()?;
            let model = config.model.clone();
            let interpreter = OllamaSemanticInterpreter::new(config)
                .map_err(|error| invalid(error.to_string()))?;
            let api = FederationHttpApi::from_state_path(interpreter, options.state_path)
                .map_err(|error| io::Error::other(error.to_string()))?;
            serve_federation_http_loopback(options.listen, api, model)
        }
        _ => Err(invalid(usage())),
    }
}

fn phase_name(phase: hollow_grove::constitutional::GrovePhase) -> &'static str {
    use hollow_grove::constitutional::GrovePhase;
    match phase {
        GrovePhase::TheWayBack => "The Way Back",
        GrovePhase::TheInitiation => "The Initiation",
        GrovePhase::TheGathering => "The Gathering",
        GrovePhase::TheFestival => "The Festival",
    }
}

fn external_label(reference: &ExternalRef) -> String {
    format!("{}.{}", reference.namespace, reference.key)
}

fn position_json(position: Position3) -> serde_json::Value {
    serde_json::json!({
        "x": position.x().get(),
        "y": position.y().get(),
        "z": position.z().get(),
    })
}

fn federation_result_json(
    result: &hollow_grove::runtime_federation::FederationResult,
    state_path: &Path,
) -> serde_json::Value {
    let decision = match &result.festival.decision {
        SemanticDecision::Confirmed(confirmed) => serde_json::json!({
            "status": "confirmed",
            "proposal_ref": external_label(&confirmed.proposal_id),
            "point_ref": external_label(&confirmed.point_ref),
            "scale_key": confirmed.scope.scale.as_str(),
            "previous_position": confirmed.previous_position.map(position_json),
            "confirmed_position": position_json(confirmed.confirmed_position),
            "translation": confirmed.translation.map(|translation| serde_json::json!({
                "dx": translation.dx().get(),
                "dy": translation.dy().get(),
                "dz": translation.dz().get(),
            })),
            "composition_ref": external_label(&confirmed.composition_ref),
            "witness_ref": external_label(&confirmed.witness_ref),
        }),
        SemanticDecision::Rejected(rejected) => serde_json::json!({
            "status": "rejected",
            "proposal_ref": external_label(&rejected.proposal_id),
            "point_ref": external_label(&rejected.point_ref),
            "scale_key": rejected.scope.scale.as_str(),
            "proposed_position": position_json(rejected.proposed_position),
            "reasons": rejected
                .reasons
                .iter()
                .map(|reason| format!("{reason:?}"))
                .collect::<Vec<_>>(),
            "witness_ref": external_label(&rejected.witness_ref),
        }),
    };
    serde_json::json!({
        "schema_version": FEDERATION_CLI_RESULT_SCHEMA_VERSION,
        "cycle_id": result.next_way_back.cycle_id.as_str(),
        "phases": result.phase_trace.iter().copied().map(phase_name).collect::<Vec<_>>(),
        "decision": decision,
        "next_position": result.next_way_back.current_position.map(position_json),
        "state_path": state_path.display().to_string(),
    })
}

fn print_federation_result(
    result: &hollow_grove::runtime_federation::FederationResult,
    state_path: &Path,
) {
    println!("Cycle:\n{}", result.next_way_back.cycle_id.as_str());
    println!("\nPhases:");
    for phase in &result.phase_trace {
        println!("- {}", phase_name(*phase));
    }
    match &result.festival.decision {
        SemanticDecision::Confirmed(confirmed) => {
            println!("\nStatus:\nConfirmed");
            println!(
                "\nPosition:\nX {:+}\nY {:+}\nZ {:+}",
                confirmed.confirmed_position.x().get(),
                confirmed.confirmed_position.y().get(),
                confirmed.confirmed_position.z().get(),
            );
            if let Some(translation) = confirmed.translation {
                println!(
                    "\nMovement:\nDX {:+}\nDY {:+}\nDZ {:+}",
                    translation.dx().get(),
                    translation.dy().get(),
                    translation.dz().get(),
                );
            }
            println!("\nWitness:\n{}", external_label(&confirmed.witness_ref));
        }
        SemanticDecision::Rejected(rejected) => {
            println!("\nStatus:\nRejected");
            println!("\nReasons:");
            for reason in &rejected.reasons {
                println!("- {reason:?}");
            }
            println!("\nWitness:\n{}", external_label(&rejected.witness_ref));
        }
    }
    println!("\nState:\n{}", state_path.display());
}

fn run_semantic(args: &[String]) -> io::Result<()> {
    match args.first().map(String::as_str) {
        Some("help" | "--help" | "-h") | None => {
            println!("{}", usage());
            Ok(())
        }
        Some("inspect") if args.len() == 4 || args.len() == 5 && has_json_flag(args) => {
            let records = read_dataset(&args[1])?;
            let record = inspect_semantic_dataset(&records, &args[2], &args[3])
                .map_err(|error| invalid(error.to_string()))?;
            if has_json_flag(args) {
                println!(
                    "{}",
                    serde_json::to_string(record).map_err(|error| invalid(error.to_string()))?
                );
            } else {
                println!("{}", format_semantic_dataset_record(record));
            }
            Ok(())
        }
        Some("history") if args.len() == 4 || args.len() == 5 && has_json_flag(args) => {
            let records = read_dataset(&args[1])?;
            let history = semantic_dataset_history(&records, &args[2], &args[3]);
            if has_json_flag(args) {
                println!(
                    "{}",
                    serde_json::to_string(&history).map_err(|error| invalid(error.to_string()))?
                );
            } else if history.is_empty() {
                return Err(invalid("no semantic history matched the Point and scale"));
            } else {
                for (index, record) in history.iter().enumerate() {
                    if index > 0 {
                        println!("\n---\n");
                    }
                    println!("{}", format_semantic_dataset_record(record));
                }
            }
            Ok(())
        }
        Some("nearby") if (args.len() == 6 || args.len() == 7 || args.len() == 8) => {
            let json = has_json_flag(args);
            let positional_end = if json { args.len() - 1 } else { args.len() };
            if positional_end != 6 && positional_end != 7 {
                return Err(invalid(usage()));
            }
            let records = read_dataset(&args[1])?;
            let x = args[3]
                .parse::<i16>()
                .map_err(|_| invalid("x must be an integer"))?;
            let y = args[4]
                .parse::<i16>()
                .map_err(|_| invalid("y must be an integer"))?;
            let z = args[5]
                .parse::<i16>()
                .map_err(|_| invalid("z must be an integer"))?;
            let center =
                Position3::from_raw(x, y, z).map_err(|error| invalid(error.to_string()))?;
            let limit = if positional_end == 7 {
                args[6]
                    .parse::<usize>()
                    .map_err(|_| invalid("limit must be a nonnegative integer"))?
            } else {
                8
            };
            let nearby = semantic_dataset_nearby(&records, &args[2], center, limit.min(64));
            if json {
                println!(
                    "{}",
                    serde_json::to_string(&nearby).map_err(|error| invalid(error.to_string()))?
                );
            } else {
                for reading in nearby {
                    println!(
                        "{}\t{}\t{}\t{}",
                        reading.point_ref,
                        reading.scale_key,
                        reading.squared_distance,
                        reading.witness_ref
                    );
                }
            }
            Ok(())
        }
        _ => Err(invalid(usage())),
    }
}

fn run_model(args: &[String]) -> io::Result<()> {
    match args.first().map(String::as_str) {
        Some("help" | "--help" | "-h") | None => {
            println!("{}", usage());
            Ok(())
        }
        Some("show-modelfile") if args.len() == 2 => {
            let modelfile = render_hollow_grove_ollama_modelfile(&args[1])
                .map_err(|error| invalid(error.to_string()))?;
            print!("{modelfile}");
            Ok(())
        }
        Some("install-ollama") if args.len() == 2 || args.len() == 3 => {
            let base_model = &args[1];
            let model_name = args
                .get(2)
                .map_or(DEFAULT_HOLLOW_GROVE_OLLAMA_MODEL, String::as_str);
            validate_ollama_model_reference(model_name)
                .map_err(|error| invalid(error.to_string()))?;
            let modelfile = render_hollow_grove_ollama_modelfile(base_model)
                .map_err(|error| invalid(error.to_string()))?;
            let temporary = TemporaryModelfile::create(&modelfile)?;
            let status = Command::new("ollama")
                .args(["create", model_name, "-f"])
                .arg(&temporary.path)
                .status()
                .map_err(|error| {
                    io::Error::new(
                        error.kind(),
                        format!(
                            "could not run Ollama; install it and start `ollama serve`: {error}"
                        ),
                    )
                })?;
            if !status.success() {
                return Err(io::Error::other(format!(
                    "Ollama could not create {model_name:?} from {base_model:?}; \
                     make sure `ollama serve` is running and the base model is installed"
                )));
            }
            println!(
                "Installed proposal-only Ollama model {model_name:?} from {base_model:?}.\n\
                 Run: ollama run {model_name} --format json\n\
                 Canonical confirmation still requires Hollow Grove Federation and Festival."
            );
            Ok(())
        }
        Some("validate-response") if args.len() == 2 || args.len() == 3 && has_json_flag(args) => {
            let bytes = fs::read(Path::new(&args[1]))?;
            let proposal = parse_model_semantic_proposal(&bytes)
                .map_err(|error| invalid(error.to_string()))?;
            if has_json_flag(args) {
                println!(
                    "{}",
                    serde_json::to_string(&proposal).map_err(|error| invalid(error.to_string()))?
                );
            } else {
                println!(
                    "Valid syntactic proposal\nScale: {}\nPosition: [{}, {}, {}]\nStatus: Proposed (not confirmed)",
                    proposal.scale_key,
                    proposal.proposed_position.x().get(),
                    proposal.proposed_position.y().get(),
                    proposal.proposed_position.z().get()
                );
            }
            Ok(())
        }
        _ => Err(invalid(usage())),
    }
}

fn run_dataset(args: &[String]) -> io::Result<()> {
    match args {
        [] => {
            println!("{}", usage());
            Ok(())
        }
        [help] if matches!(help.as_str(), "help" | "--help" | "-h") => {
            println!("{}", usage());
            Ok(())
        }
        [command, path] if command == "validate-semantic" => {
            let bytes = fs::read(Path::new(path))?;
            let count = validate_semantic_witness_jsonl(&bytes)
                .map_err(|error| invalid(error.to_string()))?;
            println!("Valid hollow-grove-semantic-witness-v1 JSONL: {count} records");
            Ok(())
        }
        [command, input, train, validation, seed]
        | [command, input, train, validation, seed, _]
            if command == "split-semantic" =>
        {
            let seed = seed
                .parse::<u64>()
                .map_err(|_| invalid("seed must be an unsigned integer"))?;
            let validation_millis = args.get(5).map_or(Ok(100), |value| {
                value
                    .parse::<u16>()
                    .map_err(|_| invalid("validation-millis must be an integer from 0 to 1000"))
            })?;
            let split = write_semantic_dataset_split(
                Path::new(input),
                Path::new(train),
                Path::new(validation),
                seed,
                validation_millis,
            )
            .map_err(|error| invalid(error.to_string()))?;
            println!(
                "Prepared deterministic split: {} train, {} validation, seed {}, validation-millis {}",
                split.train_records, split.validation_records, split.seed, split.validation_millis
            );
            Ok(())
        }
        _ => Err(invalid(usage())),
    }
}

fn main() -> io::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("semantic") => run_semantic(&args[1..]),
        Some("federation") => run_federation(&args[1..]),
        Some("model") => run_model(&args[1..]),
        Some("dataset") => run_dataset(&args[1..]),
        Some("help" | "--help" | "-h") | None => {
            println!("{}", usage());
            Ok(())
        }
        _ => Err(invalid(usage())),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use hollow_grove::Position3;
    use hollow_grove::runtime_federation::{ModelSemanticProposal, ProposedSemanticOperation};

    fn response_json(response: &[u8]) -> serde_json::Value {
        let delimiter = response
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap();
        serde_json::from_slice(&response[delimiter + 4..]).unwrap()
    }

    fn unique_test_state(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir()
            .join(format!("{label}-{}-{nonce}", std::process::id()))
            .join("state.json")
    }

    #[test]
    fn usage_exposes_read_only_semantic_surfaces() {
        let usage = usage();
        assert!(usage.contains("semantic inspect"));
        assert!(usage.contains("semantic history"));
        assert!(usage.contains("semantic nearby"));
        assert!(usage.contains("federation run"));
        assert!(usage.contains("federation serve"));
        assert!(usage.contains("model show-modelfile"));
        assert!(usage.contains("model install-ollama"));
        assert!(usage.contains("model validate-response"));
        assert!(usage.contains("dataset validate-semantic"));
        assert!(usage.contains("dataset split-semantic"));
        assert!(usage.contains("read-only"));
    }

    #[test]
    fn federation_http_listener_is_loopback_only() {
        let options = parse_federation_serve_options(&[]).unwrap();
        assert_eq!(
            options.listen,
            DEFAULT_FEDERATION_HTTP_ADDRESS
                .parse::<SocketAddr>()
                .unwrap()
        );
        let error = parse_federation_serve_options(&["--listen".into(), "0.0.0.0:47820".into()])
            .unwrap_err();
        assert!(error.to_string().contains("non-loopback"));
    }

    #[test]
    fn federation_http_parser_bounds_headers_and_body() {
        let mut missing_length = Cursor::new(
            b"POST /v1/federation/cycles HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{}",
        );
        let error = read_federation_http_request(&mut missing_length).unwrap_err();
        assert_eq!(error.status, 411);

        let oversized_headers = vec![b'x'; MAX_FEDERATION_HTTP_HEADER_BYTES + 1];
        let error = read_federation_http_request(&mut Cursor::new(oversized_headers)).unwrap_err();
        assert_eq!(error.status, 431);

        let oversized_body = format!(
            "POST /v1/federation/cycles HTTP/1.1\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\r\n",
            MAX_FEDERATION_HTTP_REQUEST_BODY_BYTES + 1
        );
        let error = read_federation_http_request(&mut Cursor::new(oversized_body)).unwrap_err();
        assert_eq!(error.status, 413);
    }

    #[test]
    fn health_and_model_discovery_do_not_contact_the_interpreter() {
        let state = unique_test_state("hollow-grove-http-discovery");
        let api = FederationHttpApi::from_state_path(
            DeterministicSemanticInterpreter::unavailable(),
            &state,
        )
        .unwrap();
        let api = Arc::new(Mutex::new(api));
        let health = handle_federation_http_request(
            FederationHttpRequest {
                method: "GET".into(),
                path: "/health".into(),
                content_type: None,
                body: Vec::new(),
            },
            &api,
            "hollow-grove",
        )
        .unwrap();
        assert!(health.starts_with(b"HTTP/1.1 200 OK\r\n"));
        assert_eq!(response_json(&health)["authority"], "rust_festival");

        let models = handle_federation_http_request(
            FederationHttpRequest {
                method: "GET".into(),
                path: "/v1/models".into(),
                content_type: None,
                body: Vec::new(),
            },
            &api,
            "hollow-grove",
        )
        .unwrap();
        assert_eq!(response_json(&models)["data"][0]["id"], "hollow-grove");
        assert!(!state.exists());
    }

    #[test]
    fn post_cycle_uses_federation_and_persists_a_witness() {
        let state = unique_test_state("hollow-grove-http-cycle");
        let request = FederationApiCycleRequest {
            point_ref: "point.hueman.origin".into(),
            scale_key: "structural-repair".into(),
            intent: "Restore the frame through the HTTP boundary.".into(),
            context_ref: None,
        };
        let input = LocalFederationCycleInput::new(
            ExternalRef::new("point", "hueman.origin").unwrap(),
            ScaleKey::new(&request.scale_key).unwrap(),
            &request.intent,
        )
        .unwrap();
        let evidence = input.evidence_ref().unwrap();
        let proposal = ModelSemanticProposal {
            scale_key: request.scale_key.clone(),
            proposed_position: Position3::from_raw(-120, 0, 580).unwrap(),
            operation: ProposedSemanticOperation::Repair,
            evidence_refs: vec![external_label(&evidence)],
            semantic_tags: vec!["repair".into()],
            summary: "The request restores embodied support.".into(),
            confidence_millis: 850,
        };
        let api = FederationHttpApi::from_state_path(
            DeterministicSemanticInterpreter::responding_with(proposal),
            &state,
        )
        .unwrap();
        let api = Arc::new(Mutex::new(api));
        let response = handle_federation_http_request(
            FederationHttpRequest {
                method: "POST".into(),
                path: "/v1/federation/cycles".into(),
                content_type: Some("application/json".into()),
                body: serde_json::to_vec(&request).unwrap(),
            },
            &api,
            "hollow-grove",
        )
        .unwrap();
        assert!(response.starts_with(b"HTTP/1.1 200 OK\r\n"));
        let body = response_json(&response);
        assert_eq!(body["decision"]["status"], "confirmed");
        assert_eq!(
            body["phases"],
            serde_json::json!([
                "The Way Back",
                "The Initiation",
                "The Gathering",
                "The Festival"
            ])
        );
        assert!(state.is_file());
        fs::remove_dir_all(state.parent().unwrap()).unwrap();
    }

    #[test]
    fn post_cycle_rejects_non_json_and_unknown_fields_before_mutation() {
        let state = unique_test_state("hollow-grove-http-invalid");
        let api = FederationHttpApi::from_state_path(
            DeterministicSemanticInterpreter::unavailable(),
            &state,
        )
        .unwrap();
        let api = Arc::new(Mutex::new(api));
        let unsupported = handle_federation_http_request(
            FederationHttpRequest {
                method: "POST".into(),
                path: "/v1/federation/cycles".into(),
                content_type: Some("text/plain".into()),
                body: b"{}".to_vec(),
            },
            &api,
            "hollow-grove",
        )
        .unwrap();
        assert!(unsupported.starts_with(b"HTTP/1.1 415 Unsupported Media Type\r\n"));

        let unknown = handle_federation_http_request(
            FederationHttpRequest {
                method: "POST".into(),
                path: "/v1/federation/cycles".into(),
                content_type: Some("application/json; charset=utf-8".into()),
                body: serde_json::to_vec(&serde_json::json!({
                    "point_ref": "point.hueman.origin",
                    "scale_key": "structural-repair",
                    "intent": "Restore the frame.",
                    "canonical_id": "model.assigned"
                }))
                .unwrap(),
            },
            &api,
            "hollow-grove",
        )
        .unwrap();
        assert!(unknown.starts_with(b"HTTP/1.1 400 Bad Request\r\n"));
        assert!(!state.exists());
    }

    #[test]
    fn concurrent_cycle_handlers_serialize_through_one_catalog() {
        let state = unique_test_state("hollow-grove-http-concurrent");
        let request = FederationApiCycleRequest {
            point_ref: "point.hueman.origin".into(),
            scale_key: "structural-repair".into(),
            intent: "Hold one stable semantic position.".into(),
            context_ref: None,
        };
        let input = LocalFederationCycleInput::new(
            ExternalRef::new("point", "hueman.origin").unwrap(),
            ScaleKey::new(&request.scale_key).unwrap(),
            &request.intent,
        )
        .unwrap();
        let evidence = input.evidence_ref().unwrap();
        let proposal = ModelSemanticProposal {
            scale_key: request.scale_key.clone(),
            proposed_position: Position3::from_raw(0, 0, 500).unwrap(),
            operation: ProposedSemanticOperation::Repair,
            evidence_refs: vec![external_label(&evidence)],
            semantic_tags: vec!["support".into()],
            summary: "The request holds embodied support.".into(),
            confidence_millis: 850,
        };
        let api = FederationHttpApi::from_state_path(
            DeterministicSemanticInterpreter::responding_with(proposal),
            &state,
        )
        .unwrap();
        let api = Arc::new(Mutex::new(api));
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let mut threads = Vec::new();
        for _ in 0..2 {
            let api = Arc::clone(&api);
            let barrier = Arc::clone(&barrier);
            let request = request.clone();
            threads.push(thread::spawn(move || {
                barrier.wait();
                handle_federation_http_request(
                    FederationHttpRequest {
                        method: "POST".into(),
                        path: "/v1/federation/cycles".into(),
                        content_type: Some("application/json".into()),
                        body: serde_json::to_vec(&request).unwrap(),
                    },
                    &api,
                    "hollow-grove",
                )
                .unwrap()
            }));
        }
        barrier.wait();
        for thread in threads {
            let response = thread.join().unwrap();
            assert!(response.starts_with(b"HTTP/1.1 200 OK\r\n"));
            assert_eq!(response_json(&response)["decision"]["status"], "confirmed");
        }
        let api = api
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(api.catalog().semantic_position_records().len(), 2);
        assert_eq!(api.catalog().semantic_witnesses().len(), 2);
        drop(api);
        fs::remove_dir_all(state.parent().unwrap()).unwrap();
    }
}
