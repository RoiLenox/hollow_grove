use std::fs::{self, OpenOptions};
use std::future::Future;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::composition::{CompositionCatalog, ExternalRef, ScaleKey};
use hollow_grove::runtime_federation::{
    DEFAULT_HOLLOW_GROVE_OLLAMA_MODEL, DEFAULT_OLLAMA_URL, DeterministicSemanticInterpreter,
    HOLLOW_GROVE_MODEL_ENV, HOLLOW_GROVE_OLLAMA_URL_ENV, LocalFederationCycleInput, OllamaConfig,
    OllamaSemanticInterpreter, SemanticDecision, parse_model_semantic_proposal,
    render_hollow_grove_ollama_modelfile, run_local_federation_cycle,
    validate_ollama_model_reference,
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

fn usage() -> &'static str {
    "Usage: hollow_grove_semantic <semantic|federation|model|dataset> <command> [args]\n\
     \n\
     Commands:\n\
       semantic inspect <jsonl> <point-ref> <scale> [--json]\n\
       semantic history <jsonl> <point-ref> <scale> [--json]\n\
       semantic nearby <jsonl> <scale> <x> <y> <z> [limit] [--json]\n\
       federation run <point-ref> <scale> <intent> [--state <path>] [--proposal-json <path>] [--json]\n\
       model show-modelfile <base-model>\n\
       model install-ollama <base-model> [model-name]\n\
       model validate-response <json> [--json]\n\
       dataset validate-semantic <jsonl>\n\
       dataset split-semantic <jsonl> <train-jsonl> <validation-jsonl> <seed> [validation-millis]\n\
     \n\
     All semantic dataset commands are read-only. Canonical mutation remains\n\
     exclusively inside Federation and Festival. Federation uses Ollama unless\n\
     --proposal-json supplies an offline deterministic proposal for testing."
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
    use super::usage;

    #[test]
    fn usage_exposes_read_only_semantic_surfaces() {
        let usage = usage();
        assert!(usage.contains("semantic inspect"));
        assert!(usage.contains("semantic history"));
        assert!(usage.contains("semantic nearby"));
        assert!(usage.contains("federation run"));
        assert!(usage.contains("model show-modelfile"));
        assert!(usage.contains("model install-ollama"));
        assert!(usage.contains("model validate-response"));
        assert!(usage.contains("dataset validate-semantic"));
        assert!(usage.contains("dataset split-semantic"));
        assert!(usage.contains("read-only"));
    }
}
