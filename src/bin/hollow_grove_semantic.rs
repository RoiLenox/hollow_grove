use std::fs::{self, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::runtime_federation::{
    DEFAULT_HOLLOW_GROVE_OLLAMA_MODEL, parse_model_semantic_proposal,
    render_hollow_grove_ollama_modelfile, validate_ollama_model_reference,
};
use hollow_grove::{
    Position3, decode_semantic_witness_jsonl, format_semantic_dataset_record,
    inspect_semantic_dataset, semantic_dataset_history, semantic_dataset_nearby,
    validate_semantic_witness_jsonl, write_semantic_dataset_split,
};

fn usage() -> &'static str {
    "Usage: hollow_grove_semantic <semantic|model|dataset> <command> [args]\n\
     \n\
     Commands:\n\
       semantic inspect <jsonl> <point-ref> <scale> [--json]\n\
       semantic history <jsonl> <point-ref> <scale> [--json]\n\
       semantic nearby <jsonl> <scale> <x> <y> <z> [limit] [--json]\n\
       model show-modelfile <base-model>\n\
       model install-ollama <base-model> [model-name]\n\
       model validate-response <json> [--json]\n\
       dataset validate-semantic <jsonl>\n\
       dataset split-semantic <jsonl> <train-jsonl> <validation-jsonl> <seed> [validation-millis]\n\
     \n\
     All semantic dataset commands are read-only. Canonical mutation remains\n\
     exclusively inside Federation and Festival."
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
        assert!(usage.contains("model show-modelfile"));
        assert!(usage.contains("model install-ollama"));
        assert!(usage.contains("model validate-response"));
        assert!(usage.contains("dataset validate-semantic"));
        assert!(usage.contains("dataset split-semantic"));
        assert!(usage.contains("read-only"));
    }
}
