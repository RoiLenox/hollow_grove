use std::env;
use std::process::ExitCode;

use hollow_grove::constitutional::*;

fn usage() {
    println!("Usage: cargo run --example constitutional_v2_service -- <command> [scenario]");
    println!("Commands: catalog, run, trace, replay, persist, migrate, authority,");
    println!("  evidence, polarity, lineage, region, stewardship, guardianship, audit");
}

fn scenario_command(command: &str, scenario: String) -> Option<TuiCommand> {
    Some(match command {
        "run" => TuiCommand::RunScenario { scenario },
        "trace" => TuiCommand::InspectTrace { scenario },
        "replay" => TuiCommand::ReplayScenario { scenario },
        "persist" => TuiCommand::PersistScenario { scenario },
        "migrate" => TuiCommand::MigrateScenario { scenario },
        "authority" => TuiCommand::InspectAuthority { scenario },
        "evidence" => TuiCommand::InspectEvidence { scenario },
        "polarity" => TuiCommand::InspectPolarity { scenario },
        "lineage" => TuiCommand::InspectLineage { scenario },
        "region" => TuiCommand::InspectRegion { scenario },
        "stewardship" => TuiCommand::InspectStewardship { scenario },
        "guardianship" => TuiCommand::InspectGuardianship { scenario },
        _ => return None,
    })
}

fn submit(
    service: &mut ConstitutionalApplicationService,
    id: &str,
    command: TuiCommand,
) -> Result<TuiResponse, ApplicationServiceError> {
    service.execute(TuiRequest {
        id: id.into(),
        command,
    })
}

fn execute(args: &[String]) -> Result<Vec<TuiResponse>, ApplicationServiceError> {
    let command = args.first().map_or("catalog", String::as_str);
    let mut service = ConstitutionalApplicationService::new("session.constitutional-v2-example")?;
    match command {
        "catalog" => {
            submit(&mut service, "request.catalog", TuiCommand::Catalog).map(|value| vec![value])
        }
        "audit" => {
            submit(&mut service, "request.audit", TuiCommand::Audit).map(|value| vec![value])
        }
        "help" | "--help" | "-h" => {
            usage();
            submit(&mut service, "request.catalog", TuiCommand::Catalog).map(|value| vec![value])
        }
        _ => {
            let scenario = args
                .get(1)
                .ok_or_else(|| {
                    ApplicationServiceError::Scenario(format!("{command} requires a scenario"))
                })?
                .clone();
            let requested = scenario_command(command, scenario.clone()).ok_or_else(|| {
                ApplicationServiceError::Scenario(format!("unknown command: {command}"))
            })?;
            let mut responses = Vec::new();
            if command != "run" {
                responses.push(submit(
                    &mut service,
                    "request.select",
                    TuiCommand::RunScenario {
                        scenario: scenario.clone(),
                    },
                )?);
            }
            responses.push(submit(&mut service, "request.command", requested)?);
            Ok(responses)
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();
    match execute(&args) {
        Ok(responses) => {
            for response in responses {
                println!("Status: {}", response.status.as_str());
                for event in response.events {
                    println!("{}", event.encode_line());
                }
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("constitutional_v2_service: {error}");
            usage();
            ExitCode::FAILURE
        }
    }
}
