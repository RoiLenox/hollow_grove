//! Local-only Ollama adapter for the semantic interpreter boundary.

use std::env;
use std::fmt;
use std::future::Future;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::pin::Pin;
use std::time::Duration;

use serde::Serialize;

use super::{
    InterpreterError, MAX_MODEL_RESPONSE_BYTES, ModelSemanticProposal,
    SemanticInterpretationRequest, SemanticInterpreter, parse_model_semantic_proposal,
};

pub const HOLLOW_GROVE_MODEL_ENV: &str = "HOLLOW_GROVE_MODEL";
pub const HOLLOW_GROVE_OLLAMA_URL_ENV: &str = "HOLLOW_GROVE_OLLAMA_URL";
pub const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434/api/generate";
pub const DEFAULT_HOLLOW_GROVE_OLLAMA_MODEL: &str = "hollow-grove";
pub const HOLLOW_GROVE_OLLAMA_MODEL_SCHEMA_VERSION: &str = "hollow-grove-ollama-model-v1";
pub const HOLLOW_GROVE_SEMANTIC_PROMPT_VERSION: &str = "hollow-grove-semantic-interpreter-v1";
pub const HOLLOW_GROVE_SEMANTIC_PROMPT_V1: &str =
    include_str!("../../ai/prompts/hollow-grove-semantic-interpreter-v1.txt");
pub const MAX_OLLAMA_HTTP_RESPONSE_BYTES: usize = 1_048_576;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaModelPackageError {
    InvalidModelReference(String),
    PromptDelimiterConflict,
}

impl fmt::Display for OllamaModelPackageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModelReference(value) => write!(
                formatter,
                "invalid Ollama model reference {value:?}; use an installed model name or tag"
            ),
            Self::PromptDelimiterConflict => {
                formatter.write_str("the semantic prompt cannot be embedded in an Ollama Modelfile")
            }
        }
    }
}

impl std::error::Error for OllamaModelPackageError {}

/// Validates a model reference before it is placed in a Modelfile or supplied
/// to the Ollama CLI. This surface intentionally accepts model names and tags,
/// not local filesystem paths.
pub fn validate_ollama_model_reference(value: &str) -> Result<(), OllamaModelPackageError> {
    let valid_length = !value.is_empty() && value.len() <= 256;
    let valid_first = value
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphanumeric);
    let valid_characters = value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/' | b':')
    });
    let valid_path_segments = value
        .split('/')
        .all(|segment| !matches!(segment, "" | "." | ".."));

    if valid_length && valid_first && valid_characters && valid_path_segments {
        Ok(())
    } else {
        Err(OllamaModelPackageError::InvalidModelReference(value.into()))
    }
}

/// Builds the deterministic Ollama model package used by
/// `ollama run hollow-grove`.
///
/// This package creates an Aura/proposal surface. It does not contain catalog
/// mutation, constitutional confirmation, IDs, evidence resolution, or
/// Festival execution.
pub fn render_hollow_grove_ollama_modelfile(
    base_model: &str,
) -> Result<String, OllamaModelPackageError> {
    validate_ollama_model_reference(base_model)?;
    if HOLLOW_GROVE_SEMANTIC_PROMPT_V1.contains("\"\"\"") {
        return Err(OllamaModelPackageError::PromptDelimiterConflict);
    }

    let mut output = String::new();
    output.push_str("# ");
    output.push_str(HOLLOW_GROVE_OLLAMA_MODEL_SCHEMA_VERSION);
    output.push('\n');
    output.push_str("# Aura/proposal-only surface; Rust Festival remains authoritative.\n");
    output.push_str("FROM ");
    output.push_str(base_model);
    output.push('\n');
    output.push_str("PARAMETER temperature 0\n");
    output.push_str("SYSTEM \"\"\"\n");
    output.push_str(HOLLOW_GROVE_SEMANTIC_PROMPT_V1.trim_end());
    output.push_str(
        r#"

Direct Ollama model contract:

Treat the user's message as a semantic interpretation request.

Return exactly one JSON object with this shape:

{
  "scale_key": "concise-nonempty-contextual-scale",
  "proposed_position": {
    "x": 0,
    "y": 0,
    "z": 0
  },
  "operation": "locate",
  "evidence_refs": [],
  "semantic_tags": [],
  "summary": "concise noncanonical explanation",
  "confidence_millis": 0
}

Allowed operations are locate, inspect, translate, repair, compose, and transform.
X and Z must be integers from -1000 through 1000.
Y must be exactly zero.
Confidence must be an integer from 0 through 1000.
Use an empty evidence_refs array unless the request supplies references.
Never invent a reference.
The response is an untrusted Aura proposal, never a confirmed Current state.
Only the Rust Rampart and Festival may validate, witness, persist, or confirm it.
"#,
    );
    output.push_str("\"\"\"\n");
    Ok(output)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OllamaConfig {
    pub endpoint: String,
    pub model: String,
    pub timeout: Duration,
}

impl OllamaConfig {
    pub fn new(
        endpoint: impl Into<String>,
        model: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self, OllamaConfigError> {
        let value = Self {
            endpoint: endpoint.into(),
            model: model.into(),
            timeout,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn from_env() -> Result<Self, OllamaConfigError> {
        let model = env::var(HOLLOW_GROVE_MODEL_ENV)
            .map_err(|_| OllamaConfigError::MissingModelEnvironment)?;
        let endpoint =
            env::var(HOLLOW_GROVE_OLLAMA_URL_ENV).unwrap_or_else(|_| DEFAULT_OLLAMA_URL.into());
        Self::new(endpoint, model, Duration::from_secs(30))
    }

    pub fn validate(&self) -> Result<(), OllamaConfigError> {
        if self.model.trim().is_empty()
            || self.model.len() > 256
            || self.model.contains(['\r', '\n'])
        {
            return Err(OllamaConfigError::InvalidModel(self.model.clone()));
        }
        if self.timeout.is_zero() {
            return Err(OllamaConfigError::InvalidTimeout);
        }
        parse_local_http_endpoint(&self.endpoint)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaConfigError {
    MissingModelEnvironment,
    InvalidModel(String),
    InvalidTimeout,
    InvalidEndpoint(String),
    NonLocalEndpoint(String),
    InvalidPort(String),
}

impl fmt::Display for OllamaConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid Ollama configuration: {self:?}")
    }
}

impl std::error::Error for OllamaConfigError {}

pub trait OllamaTransport: Send + Sync {
    fn post_json<'a>(
        &'a self,
        endpoint: &'a str,
        payload: &'a [u8],
        timeout: Duration,
        maximum_response_bytes: usize,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, InterpreterError>> + Send + 'a>>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HttpOllamaTransport;

impl OllamaTransport for HttpOllamaTransport {
    fn post_json<'a>(
        &'a self,
        endpoint: &'a str,
        payload: &'a [u8],
        timeout: Duration,
        maximum_response_bytes: usize,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, InterpreterError>> + Send + 'a>> {
        Box::pin(
            async move { post_local_http_json(endpoint, payload, timeout, maximum_response_bytes) },
        )
    }
}

#[derive(Debug, Clone)]
pub struct OllamaSemanticInterpreter<T = HttpOllamaTransport> {
    config: OllamaConfig,
    transport: T,
    version: String,
}

impl OllamaSemanticInterpreter<HttpOllamaTransport> {
    pub fn new(config: OllamaConfig) -> Result<Self, OllamaConfigError> {
        Self::with_transport(config, HttpOllamaTransport)
    }
}

impl<T> OllamaSemanticInterpreter<T> {
    pub fn with_transport(config: OllamaConfig, transport: T) -> Result<Self, OllamaConfigError> {
        config.validate()?;
        let version = format!("ollama:{}", config.model);
        Ok(Self {
            config,
            transport,
            version,
        })
    }

    #[must_use]
    pub const fn config(&self) -> &OllamaConfig {
        &self.config
    }
}

impl<T> SemanticInterpreter for OllamaSemanticInterpreter<T>
where
    T: OllamaTransport,
{
    fn propose<'a>(
        &'a self,
        request: &'a SemanticInterpretationRequest,
    ) -> Pin<Box<dyn Future<Output = Result<ModelSemanticProposal, InterpreterError>> + Send + 'a>>
    {
        Box::pin(async move {
            let payload = build_ollama_request(&self.config.model, request)?;
            let response = self
                .transport
                .post_json(
                    &self.config.endpoint,
                    &payload,
                    self.config.timeout,
                    MAX_OLLAMA_HTTP_RESPONSE_BYTES,
                )
                .await?;
            parse_ollama_generate_response(&response)
        })
    }

    fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    system: String,
    prompt: String,
    stream: bool,
    format: serde_json::Value,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: u8,
}

fn build_ollama_request(
    model: &str,
    request: &SemanticInterpretationRequest,
) -> Result<Vec<u8>, InterpreterError> {
    let evidence_refs = request
        .evidence_refs
        .iter()
        .map(|reference| format!("{}.{}", reference.namespace, reference.key))
        .collect::<Vec<_>>();
    let prompt_context = serde_json::json!({
        "schema_version": super::SEMANTIC_PROPOSAL_SCHEMA_VERSION,
        "prompt_version": request.prompt_version,
        "user_intent": request.user_intent,
        "point_ref": request.point_ref,
        "scope": request.scope,
        "current_position": request.current_position,
        "retrieved_canon": request.retrieved_canon,
        "nearby_examples": request.nearby_examples,
        "evidence_refs": evidence_refs,
        "active_guide_context": request.active_guides,
        "guide_identity_law": request.guide_prompt_law,
        "required_response_shape": {
            "scale_key": "string",
            "proposed_position": {"x": "integer", "y": 0, "z": "integer"},
            "operation": "locate|inspect|translate|repair|compose|transform",
            "evidence_refs": ["supplied reference only"],
            "semantic_tags": ["noncanonical tag"],
            "summary": "concise noncanonical explanation",
            "confidence_millis": "integer 0 through 1000"
        }
    });
    let prompt = serde_json::to_string(&prompt_context)
        .map_err(|error| InterpreterError::MalformedModelResponse(error.to_string()))?;
    serde_json::to_vec(&OllamaGenerateRequest {
        model,
        system: format!(
            "{}\n\n{}",
            HOLLOW_GROVE_SEMANTIC_PROMPT_V1.trim_end(),
            request.guide_prompt_law
        ),
        prompt,
        stream: false,
        format: semantic_proposal_json_schema(request, &evidence_refs),
        options: OllamaOptions { temperature: 0 },
    })
    .map_err(|error| InterpreterError::MalformedModelResponse(error.to_string()))
}

fn semantic_proposal_json_schema(
    request: &SemanticInterpretationRequest,
    evidence_refs: &[String],
) -> serde_json::Value {
    let evidence_items = if evidence_refs.is_empty() {
        serde_json::json!({"type": "string"})
    } else {
        serde_json::json!({
            "type": "string",
            "enum": evidence_refs,
        })
    };
    serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "required": [
            "scale_key",
            "proposed_position",
            "operation",
            "evidence_refs",
            "semantic_tags",
            "summary",
            "confidence_millis"
        ],
        "properties": {
            "scale_key": {
                "type": "string",
                "const": request.scope.scale.as_str()
            },
            "proposed_position": {
                "type": "object",
                "additionalProperties": false,
                "required": ["x", "y", "z"],
                "properties": {
                    "x": {"type": "integer", "minimum": -1000, "maximum": 1000},
                    "y": {"type": "integer", "const": 0},
                    "z": {"type": "integer", "minimum": -1000, "maximum": 1000}
                }
            },
            "operation": {
                "type": "string",
                "enum": ["locate", "inspect", "translate", "repair", "compose", "transform"]
            },
            "evidence_refs": {
                "type": "array",
                "items": evidence_items
            },
            "semantic_tags": {
                "type": "array",
                "items": {
                    "type": "string"
                }
            },
            "summary": {
                "type": "string"
            },
            "confidence_millis": {
                "type": "integer",
                "minimum": 0,
                "maximum": 1000
            }
        }
    })
}

fn parse_ollama_generate_response(
    response: &[u8],
) -> Result<ModelSemanticProposal, InterpreterError> {
    if response.len() > MAX_OLLAMA_HTTP_RESPONSE_BYTES {
        return Err(InterpreterError::ResponseTooLarge {
            actual: response.len(),
            maximum: MAX_OLLAMA_HTTP_RESPONSE_BYTES,
        });
    }
    let envelope: serde_json::Value = serde_json::from_slice(response)
        .map_err(|error| InterpreterError::MalformedModelResponse(error.to_string()))?;
    let content = envelope
        .get("response")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            InterpreterError::MalformedModelResponse(
                "Ollama response is missing the response string".into(),
            )
        })?;
    if content.len() > MAX_MODEL_RESPONSE_BYTES {
        return Err(InterpreterError::ResponseTooLarge {
            actual: content.len(),
            maximum: MAX_MODEL_RESPONSE_BYTES,
        });
    }
    parse_model_semantic_proposal(content.as_bytes())
}

#[derive(Debug)]
struct LocalHttpEndpoint {
    host: String,
    port: u16,
    path: String,
}

fn parse_local_http_endpoint(endpoint: &str) -> Result<LocalHttpEndpoint, OllamaConfigError> {
    let remainder = endpoint
        .strip_prefix("http://")
        .ok_or_else(|| OllamaConfigError::InvalidEndpoint(endpoint.into()))?;
    let (authority, path) = remainder
        .split_once('/')
        .map_or((remainder, "/"), |(authority, _path)| {
            (authority, &remainder[authority.len()..])
        });
    if path.contains(['\r', '\n']) || authority.contains(['\r', '\n']) {
        return Err(OllamaConfigError::InvalidEndpoint(endpoint.into()));
    }
    let (host, port_text) = if let Some(port) = authority.strip_prefix("[::1]:") {
        ("::1", port)
    } else if authority == "[::1]" {
        ("::1", "80")
    } else {
        authority.rsplit_once(':').unwrap_or((authority, "80"))
    };
    if !matches!(host, "127.0.0.1" | "localhost" | "::1") {
        return Err(OllamaConfigError::NonLocalEndpoint(endpoint.into()));
    }
    let port = port_text
        .parse::<u16>()
        .map_err(|_| OllamaConfigError::InvalidPort(port_text.into()))?;
    Ok(LocalHttpEndpoint {
        host: host.into(),
        port,
        path: path.into(),
    })
}

fn post_local_http_json(
    endpoint: &str,
    payload: &[u8],
    timeout: Duration,
    maximum_response_bytes: usize,
) -> Result<Vec<u8>, InterpreterError> {
    let endpoint = parse_local_http_endpoint(endpoint)
        .map_err(|_| InterpreterError::InterpreterUnavailable)?;
    let address = resolve_local_address(&endpoint)?;
    let mut stream = TcpStream::connect_timeout(&address, timeout).map_err(map_io_error)?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(map_io_error)?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(map_io_error)?;
    let host_header = if endpoint.host == "::1" {
        format!("[::1]:{}", endpoint.port)
    } else {
        format!("{}:{}", endpoint.host, endpoint.port)
    };
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nAccept: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n",
        endpoint.path,
        host_header,
        payload.len()
    );
    stream.write_all(request.as_bytes()).map_err(map_io_error)?;
    stream.write_all(payload).map_err(map_io_error)?;
    stream.flush().map_err(map_io_error)?;

    let maximum_wire_bytes = maximum_response_bytes.saturating_add(65_536);
    let mut wire = Vec::new();
    stream
        .take((maximum_wire_bytes + 1) as u64)
        .read_to_end(&mut wire)
        .map_err(map_io_error)?;
    if wire.len() > maximum_wire_bytes {
        return Err(InterpreterError::ResponseTooLarge {
            actual: wire.len(),
            maximum: maximum_wire_bytes,
        });
    }
    decode_http_response(&wire, maximum_response_bytes)
}

fn resolve_local_address(endpoint: &LocalHttpEndpoint) -> Result<SocketAddr, InterpreterError> {
    (endpoint.host.as_str(), endpoint.port)
        .to_socket_addrs()
        .map_err(map_io_error)?
        .find(|address| address.ip().is_loopback())
        .ok_or(InterpreterError::InterpreterUnavailable)
}

fn decode_http_response(
    wire: &[u8],
    maximum_response_bytes: usize,
) -> Result<Vec<u8>, InterpreterError> {
    let header_end = wire
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| {
            InterpreterError::MalformedModelResponse("invalid Ollama HTTP response".into())
        })?;
    let headers = std::str::from_utf8(&wire[..header_end])
        .map_err(|error| InterpreterError::MalformedModelResponse(error.to_string()))?;
    let status = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|status| status.parse::<u16>().ok())
        .ok_or_else(|| {
            InterpreterError::MalformedModelResponse("invalid Ollama HTTP status".into())
        })?;
    let body = &wire[header_end + 4..];
    let is_chunked = headers.lines().any(|line| {
        line.split_once(':').is_some_and(|(name, value)| {
            name.eq_ignore_ascii_case("transfer-encoding")
                && value.trim().eq_ignore_ascii_case("chunked")
        })
    });
    let body = if is_chunked {
        decode_chunked_body(body, maximum_response_bytes)?
    } else {
        body.to_vec()
    };
    if body.len() > maximum_response_bytes {
        return Err(InterpreterError::ResponseTooLarge {
            actual: body.len(),
            maximum: maximum_response_bytes,
        });
    }
    if !(200..300).contains(&status) {
        if status == 408 || status == 504 {
            return Err(InterpreterError::InterpreterTimeout);
        }
        if status == 400 || status == 422 {
            let explanation = String::from_utf8_lossy(&body);
            let explanation = explanation.trim().chars().take(1_024).collect::<String>();
            return Err(InterpreterError::MalformedModelResponse(format!(
                "Ollama HTTP {status}: {explanation}"
            )));
        }
        return Err(InterpreterError::InterpreterUnavailable);
    }
    Ok(body)
}

fn decode_chunked_body(
    mut body: &[u8],
    maximum_response_bytes: usize,
) -> Result<Vec<u8>, InterpreterError> {
    let mut decoded = Vec::new();
    loop {
        let line_end = body
            .windows(2)
            .position(|window| window == b"\r\n")
            .ok_or_else(|| {
                InterpreterError::MalformedModelResponse("invalid chunked Ollama response".into())
            })?;
        let size_text = std::str::from_utf8(&body[..line_end])
            .map_err(|error| InterpreterError::MalformedModelResponse(error.to_string()))?;
        let size_text = size_text.split(';').next().unwrap_or(size_text);
        let size = usize::from_str_radix(size_text.trim(), 16).map_err(|_| {
            InterpreterError::MalformedModelResponse("invalid Ollama chunk size".into())
        })?;
        body = &body[line_end + 2..];
        if size == 0 {
            return Ok(decoded);
        }
        if body.len() < size + 2 || &body[size..size + 2] != b"\r\n" {
            return Err(InterpreterError::MalformedModelResponse(
                "truncated Ollama chunk".into(),
            ));
        }
        if decoded.len().saturating_add(size) > maximum_response_bytes {
            return Err(InterpreterError::ResponseTooLarge {
                actual: decoded.len().saturating_add(size),
                maximum: maximum_response_bytes,
            });
        }
        decoded.extend_from_slice(&body[..size]);
        body = &body[size + 2..];
    }
}

fn map_io_error(error: std::io::Error) -> InterpreterError {
    if matches!(
        error.kind(),
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
    ) {
        InterpreterError::InterpreterTimeout
    } else {
        InterpreterError::InterpreterUnavailable
    }
}
