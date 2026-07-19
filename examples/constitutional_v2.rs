use std::env;
use std::process::ExitCode;

use hollow_grove::constitutional::*;

fn application_error(error: impl std::fmt::Display) -> ScenarioError {
    ScenarioError(error.to_string())
}

fn request(id: impl Into<String>, command: TuiCommand) -> TuiRequest {
    TuiRequest {
        id: id.into(),
        command,
    }
}

fn submit(
    service: &mut ConstitutionalApplicationService,
    id: impl Into<String>,
    command: TuiCommand,
) -> Result<TuiResponse, ScenarioError> {
    service
        .execute(request(id, command))
        .map_err(application_error)
}

fn print_response(response: &TuiResponse) {
    println!("Status: {}", response.status.as_str());
    for event in &response.events {
        println!("{}", event.encode_line());
    }
}

fn select(
    service: &mut ConstitutionalApplicationService,
    scenario: &str,
) -> Result<(), ScenarioError> {
    submit(
        service,
        format!("request.select.{scenario}"),
        TuiCommand::RunScenario {
            scenario: scenario.into(),
        },
    )?;
    Ok(())
}

fn scenario_command(command: &str, scenario: String) -> Option<TuiCommand> {
    Some(match command {
        "replay" | "compare-live-replay" => TuiCommand::ReplayScenario { scenario },
        "persist" => TuiCommand::PersistScenario { scenario },
        "migrate" => TuiCommand::MigrateScenario { scenario },
        "show-authority" => TuiCommand::InspectAuthority { scenario },
        "show-evidence" => TuiCommand::InspectEvidence { scenario },
        "show-polarity" => TuiCommand::InspectPolarity { scenario },
        "show-lineage" => TuiCommand::InspectLineage { scenario },
        "show-region" => TuiCommand::InspectRegion { scenario },
        "show-synthesis" => TuiCommand::InspectTrace { scenario },
        "show-stewardship" => TuiCommand::InspectStewardship { scenario },
        "show-guardianship" => TuiCommand::InspectGuardianship { scenario },
        _ => return None,
    })
}

fn print_compact(name: &str, trace: &ConstitutionalTrace) {
    let accepted = trace
        .transitions
        .iter()
        .filter(|transition| transition.disposition == TraceDisposition::Accepted)
        .count();
    println!(
        "{}\taccepted={}\trejected={}\treplay={}\tpersistence={}",
        name,
        accepted,
        trace.transitions.len() - accepted,
        if trace.live_replay_equivalent {
            "Equivalent"
        } else {
            "Divergent"
        },
        if trace.canonical_persistence {
            "Canonical"
        } else {
            "Unavailable"
        }
    );
}

/// The combined proof intentionally retains direct typed inspection. Ordinary
/// commands use the application service; this proof demonstrates that its
/// constituent production APIs remain independently composable.
fn end_to_end() -> Result<(), ScenarioError> {
    println!("HOLLOW GROVE V2 END-TO-END CONSTITUTIONAL PROOF");
    let wave = run_kernel_wave_scenario()?;
    println!(
        "[KERNEL] wave={} constitutional-events={} implicit-current=no",
        wave.wave.as_str(),
        wave.constitutional_event_count
    );

    let bond = run_ordinary_lifecycle()?;
    print_compact("ordinary-lifecycle", &trace_bond_scenario(&bond)?);

    let gnome = run_gnome_minotaur_scenario()?;
    print_compact("gnome-minotaur", &trace_regional_scenario(&gnome)?);
    println!(
        "[REGIONAL] Gnome -> Minotaur; region=Aura Fields; stewardship={}; lineage={}",
        gnome
            .runtime
            .stewardship(&gnome.result)
            .map_or(0, |value| value.duties.len()),
        gnome.runtime.lineage(&gnome.result).map_or(0, <[_]>::len)
    );

    let elf = run_elf_centaur_scenario()?;
    print_compact("elf-centaur", &trace_regional_scenario(&elf)?);
    println!(
        "[REGIONAL] Elf -> Centaur; region=Aura Beach; beach-duties={}; sea-guardianship={}; lineage={}",
        elf.runtime
            .beach_occupation(&elf.result)
            .map_or(0, |value| value.duties.len()),
        elf.runtime
            .guardianship(&elf.result)
            .map_or(0, |value| value.duties.len()),
        elf.runtime.lineage(&elf.result).map_or(0, <[_]>::len)
    );

    for name in ["gnome-centaur", "elf-minotaur"] {
        let rejected = run_rejected_regional_scenario(name)?;
        println!(
            "[REJECTED] {} code={} state-changed={}",
            name,
            rejected.error.code(),
            rejected.event_count_before != rejected.event_count_after
        );
    }
    println!("Trace Authority: Reports reducer inputs and outcomes; decides no law");
    println!("Result: Live state, persistence, and replay are canonically equivalent");
    Ok(())
}

fn run_all(service: &mut ConstitutionalApplicationService) -> Result<(), ScenarioError> {
    println!("HOLLOW GROVE V2 COMPLETE SCENARIO RUN");
    for descriptor in SCENARIO_CATALOG {
        let response = submit(
            service,
            format!("request.run-all.{}", descriptor.name),
            TuiCommand::RunScenario {
                scenario: descriptor.name.into(),
            },
        )?;
        println!("{}\tstatus={}", descriptor.name, response.status.as_str());
    }
    let audit = submit(service, "request.run-all.audit", TuiCommand::Audit)?;
    print_response(&audit);
    Ok(())
}

fn usage() {
    println!("Usage: cargo run --example constitutional_v2 -- <command> [scenario]");
    println!("Commands: catalog, run, run-all, trace, replay, persist, migrate,");
    println!("  compare-live-replay, show-illegal, show-authority, show-evidence,");
    println!("  show-polarity, show-lineage, show-wave, show-region, show-synthesis,");
    println!("  show-guardianship, show-stewardship, audit");
}

fn execute(args: &[String]) -> Result<(), ScenarioError> {
    let command = args.first().map_or("catalog", String::as_str);
    let mut service = ConstitutionalApplicationService::new("session.constitutional-v2")
        .map_err(application_error)?;
    match command {
        "catalog" => print_response(&submit(
            &mut service,
            "request.catalog",
            TuiCommand::Catalog,
        )?),
        "run-all" => return run_all(&mut service),
        "audit" => print_response(&submit(&mut service, "request.audit", TuiCommand::Audit)?),
        "run" | "trace" => {
            let scenario = args
                .get(1)
                .ok_or_else(|| ScenarioError(format!("{command} requires a scenario")))?;
            if scenario == "end-to-end" {
                return end_to_end();
            }
            print_response(&submit(
                &mut service,
                "request.command",
                TuiCommand::RunScenario {
                    scenario: scenario.clone(),
                },
            )?);
        }
        "show-illegal" => {
            let scenario = args
                .get(1)
                .ok_or_else(|| ScenarioError("show-illegal requires a scenario".into()))?;
            let response = submit(
                &mut service,
                "request.command",
                TuiCommand::RunScenario {
                    scenario: scenario.clone(),
                },
            )?;
            if response.status != ApplicationResponseStatus::ConstitutionallyRejected {
                return Err(ScenarioError(format!(
                    "scenario {scenario} was not constitutionally rejected"
                )));
            }
            print_response(&response);
        }
        "show-wave" => print_response(&submit(
            &mut service,
            "request.command",
            TuiCommand::RunScenario {
                scenario: "kernel-wave".into(),
            },
        )?),
        "help" | "--help" | "-h" => usage(),
        _ => {
            let scenario = args
                .get(1)
                .ok_or_else(|| ScenarioError(format!("{command} requires a scenario")))?;
            let requested = scenario_command(command, scenario.clone())
                .ok_or_else(|| ScenarioError(format!("unknown command: {command}")))?;
            select(&mut service, scenario)?;
            print_response(&submit(&mut service, "request.command", requested)?);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();
    match execute(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("constitutional_v2: {error}");
            usage();
            ExitCode::FAILURE
        }
    }
}
