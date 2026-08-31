use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use hollow_grove::constitutional::SeasonalAnchor;
use hollow_grove::world::adaptive_topology_live::{
    ADAPTIVE_TOPOLOGY_SEASONAL_EVENT_ARTIFACT_PATH, AdaptiveTopologyLiveStepReport,
    process_adaptive_topology_live_once, submit_adaptive_topology_seasonal_event,
};
use hollow_grove::world::function_junction_archive::encode_function_junction_archive;
use hollow_grove::world::function_junction_fixture::canonical_function_junction_archive_fixture;

const DEFAULT_POLL_MS: u64 = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cli {
    Help,
    Step {
        root: PathBuf,
    },
    Serve {
        root: PathBuf,
        poll_interval: Duration,
        cycles: Option<usize>,
        quiet: bool,
    },
    Submit {
        root: PathBuf,
        archive: PathBuf,
        junction_id: String,
    },
    ExportRehearsalArchive {
        output: PathBuf,
    },
}

fn parse_cli<I>(args: I) -> Result<Cli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        return Ok(Cli::Help);
    };
    if matches!(command.as_str(), "--help" | "-h" | "help") {
        return Ok(Cli::Help);
    }

    let mut root = PathBuf::from(".");
    let mut poll_ms = DEFAULT_POLL_MS;
    let mut cycles = None;
    let mut quiet = false;
    let mut archive = None;
    let mut junction_id = None;
    let mut output = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--root" => {
                root = PathBuf::from(
                    args.next()
                        .ok_or_else(|| String::from("missing value for --root"))?,
                );
            }
            "--poll-ms" if command == "serve" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --poll-ms"))?;
                poll_ms = value
                    .parse()
                    .map_err(|_| format!("invalid u64 for --poll-ms: {value}"))?;
                if poll_ms == 0 {
                    return Err(String::from("--poll-ms must be greater than zero"));
                }
            }
            "--cycles" if command == "serve" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --cycles"))?;
                let count = value
                    .parse()
                    .map_err(|_| format!("invalid usize for --cycles: {value}"))?;
                if count == 0 {
                    return Err(String::from("--cycles must be greater than zero"));
                }
                cycles = Some(count);
            }
            "--quiet" if command == "serve" => quiet = true,
            "--archive" if command == "submit" => {
                archive =
                    Some(PathBuf::from(args.next().ok_or_else(|| {
                        String::from("missing value for --archive")
                    })?));
            }
            "--junction" if command == "submit" => {
                junction_id = Some(
                    args.next()
                        .ok_or_else(|| String::from("missing value for --junction"))?,
                );
            }
            "--output" if command == "export-rehearsal-archive" => {
                output = Some(PathBuf::from(
                    args.next()
                        .ok_or_else(|| String::from("missing value for --output"))?,
                ));
            }
            _ => return Err(format!("unknown argument for {command}: {argument}")),
        }
    }

    match command.as_str() {
        "step" => Ok(Cli::Step { root }),
        "serve" => Ok(Cli::Serve {
            root,
            poll_interval: Duration::from_millis(poll_ms),
            cycles,
            quiet,
        }),
        "submit" => Ok(Cli::Submit {
            root,
            archive: archive.ok_or_else(|| String::from("submit requires --archive PATH"))?,
            junction_id: junction_id
                .ok_or_else(|| String::from("submit requires --junction ID"))?,
        }),
        "export-rehearsal-archive" => Ok(Cli::ExportRehearsalArchive {
            output: output
                .ok_or_else(|| String::from("export-rehearsal-archive requires --output PATH"))?,
        }),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn print_help() {
    println!(
        "adaptive_topology_live\n\n\
         Usage:\n  \
         adaptive_topology_live serve [--root PATH] [--poll-ms N] [--cycles N] [--quiet]\n  \
         adaptive_topology_live step [--root PATH]\n  \
         adaptive_topology_live submit --archive PATH --junction ID [--root PATH]\n  \
         adaptive_topology_live export-rehearsal-archive --output PATH\n\n\
         The serve loop prepares kernel-sampled route geometry, waits for Godot's exact\n\
         collision/navigation acknowledgement, and atomically commits each pose."
    );
}

fn print_report(report: &AdaptiveTopologyLiveStepReport) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout().lock();
    serde_json::to_writer_pretty(&mut stdout, report)?;
    writeln!(stdout)?;
    stdout.flush()?;
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = parse_cli(env::args().skip(1))
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;
    match cli {
        Cli::Help => print_help(),
        Cli::Step { root } => print_report(&process_adaptive_topology_live_once(&root)?)?,
        Cli::Serve {
            root,
            poll_interval,
            cycles,
            quiet,
        } => {
            let mut completed_cycles = 0_usize;
            let mut previous_report = None;
            loop {
                let report = process_adaptive_topology_live_once(&root)?;
                if !quiet && previous_report.as_ref() != Some(&report) {
                    print_report(&report)?;
                }
                previous_report = Some(report);
                completed_cycles += 1;
                if cycles.is_some_and(|limit| completed_cycles >= limit) {
                    break;
                }
                thread::sleep(poll_interval);
            }
        }
        Cli::Submit {
            root,
            archive,
            junction_id,
        } => {
            let archive_bytes = fs::read(&archive)?;
            submit_adaptive_topology_seasonal_event(&root, &archive_bytes, &junction_id)?;
            println!(
                "submitted {junction_id} to {}",
                root.join(ADAPTIVE_TOPOLOGY_SEASONAL_EVENT_ARTIFACT_PATH)
                    .display()
            );
        }
        Cli::ExportRehearsalArchive { output } => {
            let fixture = canonical_function_junction_archive_fixture();
            let spring_junction = fixture.annual_records[0]
                .junctions
                .iter()
                .find(|junction| junction.anchor == SeasonalAnchor::SpringEquinox)
                .expect("canonical rehearsal fixture contains Spring")
                .junction_id
                .as_str()
                .to_owned();
            hollow_grove::write_bytes_atomically(
                &output,
                &encode_function_junction_archive(&fixture)?,
            )?;
            println!("archive: {}", output.display());
            println!("spring_junction: {spring_junction}");
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("adaptive_topology_live: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bounded_serve_and_submit_commands() {
        assert_eq!(
            parse_cli(
                [
                    "serve",
                    "--root",
                    "/tmp/grove",
                    "--poll-ms",
                    "25",
                    "--cycles",
                    "4",
                    "--quiet"
                ]
                .map(String::from)
            )
            .unwrap(),
            Cli::Serve {
                root: PathBuf::from("/tmp/grove"),
                poll_interval: Duration::from_millis(25),
                cycles: Some(4),
                quiet: true,
            }
        );
        assert!(matches!(
            parse_cli(
                ["submit", "--archive", "year.json", "--junction", "spring"].map(String::from)
            )
            .unwrap(),
            Cli::Submit { .. }
        ));
        assert!(matches!(
            parse_cli(["export-rehearsal-archive", "--output", "year.json"].map(String::from))
                .unwrap(),
            Cli::ExportRehearsalArchive { .. }
        ));
    }

    #[test]
    fn rejects_zero_poll_and_incomplete_submission() {
        assert!(parse_cli(["serve", "--poll-ms", "0"].map(String::from)).is_err());
        assert!(parse_cli(["submit", "--archive", "year.json"].map(String::from)).is_err());
        assert!(parse_cli(["export-rehearsal-archive"].map(String::from)).is_err());
    }
}
