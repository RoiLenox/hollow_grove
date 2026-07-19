use std::hint::black_box;
use std::time::{Duration, Instant};

use hollow_grove::constitutional::*;
use hollow_grove::lineage_contract::SandmanorForm;

#[derive(Debug)]
struct Measurement {
    label: &'static str,
    iterations: usize,
    average: Duration,
    median: Duration,
    worst: Duration,
}

fn measure(
    label: &'static str,
    iterations: usize,
    mut operation: impl FnMut() -> Result<(), ScenarioError>,
) -> Result<Measurement, ScenarioError> {
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let started = Instant::now();
        operation()?;
        samples.push(started.elapsed());
    }
    samples.sort_unstable();
    let total: u128 = samples.iter().map(Duration::as_nanos).sum();
    let average_nanos = total / u128::try_from(iterations).unwrap_or(1);
    let average = Duration::from_nanos(u64::try_from(average_nanos).unwrap_or(u64::MAX));
    Ok(Measurement {
        label,
        iterations,
        average,
        median: samples[iterations / 2],
        worst: *samples.last().unwrap_or(&Duration::ZERO),
    })
}

fn micros(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1_000_000.0
}

fn print_measurement(value: &Measurement) {
    println!(
        "{}\titerations={}\taverage_us={:.3}\tmedian_us={:.3}\tworst_us={:.3}",
        value.label,
        value.iterations,
        micros(value.average),
        micros(value.median),
        micros(value.worst)
    );
}

fn registration_history(event_count: usize) -> Result<Vec<RegionalEventEnvelope>, ScenarioError> {
    let world = hollow_grove::world::institutional_access_fixture();
    let mut runtime = RegionalSynthesisRuntime::new();
    for index in 0..event_count {
        let being = RegionalBeingId::new(format!("being.benchmark.{event_count}.{index}"))
            .map_err(|error| ScenarioError(error.to_string()))?;
        runtime
            .register_being(
                scenario_regional_metadata(
                    &format!("benchmark.{event_count}.{index}"),
                    u64::try_from(index + 1).unwrap_or(u64::MAX),
                ),
                scenario_regional_registration(
                    &world.catalog,
                    being,
                    SandmanorForm::Gnome,
                    ConstitutionalRegion::AuraFields,
                    RegionalStandingKind::Established,
                )?,
            )
            .map_err(|error| ScenarioError(error.to_string()))?;
    }
    Ok(runtime.events().to_vec())
}

fn main() -> Result<(), ScenarioError> {
    println!("HOLLOW GROVE V2 PERFORMANCE CHARACTERIZATION");
    println!(
        "clock=std::time::Instant\tprofile={}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );

    let world = hollow_grove::world::institutional_access_fixture();
    print_measurement(&measure("single_transition_reduction", 500, || {
        let being = RegionalBeingId::new("being.benchmark.single")
            .map_err(|error| ScenarioError(error.to_string()))?;
        let mut runtime = RegionalSynthesisRuntime::new();
        runtime
            .register_being(
                scenario_regional_metadata("benchmark.single", 1),
                scenario_regional_registration(
                    &world.catalog,
                    being,
                    SandmanorForm::Gnome,
                    ConstitutionalRegion::AuraFields,
                    RegionalStandingKind::Established,
                )?,
            )
            .map_err(|error| ScenarioError(error.to_string()))?;
        black_box(runtime);
        Ok(())
    })?);

    print_measurement(&measure("full_lawful_lifecycle", 100, || {
        black_box(run_ordinary_lifecycle()?);
        Ok(())
    })?);
    print_measurement(&measure("full_lifecycle_with_persistence", 100, || {
        let scenario = run_ordinary_lifecycle()?;
        let archive = encode_constitutional_archive(&scenario.runtime)
            .map_err(|error| ScenarioError(error.to_string()))?;
        black_box(
            decode_constitutional_archive(&archive)
                .map_err(|error| ScenarioError(error.to_string()))?,
        );
        Ok(())
    })?);

    let gnome = run_gnome_minotaur_scenario()?;
    let elf = run_elf_centaur_scenario()?;
    let legacy = encode_legacy_regional_archive_v0(&gnome.runtime)
        .map_err(|error| ScenarioError(error.to_string()))?;
    print_measurement(&measure("regional_migration", 500, || {
        black_box(
            migrate_regional_archive(&legacy).map_err(|error| ScenarioError(error.to_string()))?,
        );
        Ok(())
    })?);
    print_measurement(&measure("regional_digest", 1_000, || {
        black_box(
            regional_archive_digest(&gnome.runtime)
                .map_err(|error| ScenarioError(error.to_string()))?,
        );
        Ok(())
    })?);
    print_measurement(&measure("trace_generation", 500, || {
        black_box(trace_regional_scenario(&gnome)?);
        Ok(())
    })?);

    let retry_record = gnome
        .runtime
        .synthesis(&gnome.synthesis)
        .expect("benchmark scenario synthesis")
        .clone();
    let retry_event = gnome.runtime.events()[1].clone();
    let mut retry_runtime = gnome.runtime.clone();
    print_measurement(&measure("idempotent_retry", 1_000, || {
        black_box(
            retry_runtime
                .synthesize(
                    RegionalEventMetadata {
                        id: retry_event.id.clone(),
                        causal_position: retry_event.causal_position,
                        rule_set: retry_event.rule_set.clone(),
                    },
                    retry_record.command.clone(),
                )
                .map_err(|error| ScenarioError(error.to_string()))?,
        );
        Ok(())
    })?);

    print_measurement(&measure("scenario_catalog_execution", 50, || {
        black_box(run_ordinary_lifecycle()?);
        black_box(run_default_challenge_scenario()?);
        black_box(run_premature_maturity_scenario()?);
        black_box(run_terminal_renewal_rejection()?);
        for (current, aura) in [
            (Sign::Positive, Sign::Positive),
            (Sign::Positive, Sign::Negative),
            (Sign::Negative, Sign::Positive),
            (Sign::Negative, Sign::Negative),
        ] {
            black_box(run_polarity_scenario(current, aura)?);
        }
        black_box(run_gnome_minotaur_scenario()?);
        black_box(run_elf_centaur_scenario()?);
        for rejected in [
            "gnome-centaur",
            "elf-minotaur",
            "gnome-minotaur-wrong-region",
            "elf-centaur-wrong-region",
            "synthesis-without-authority",
            "synthesis-without-evidence",
            "synthesis-mismatched-evidence",
        ] {
            black_box(run_rejected_regional_scenario(rejected)?);
        }
        black_box(run_rejected_assignment_scenario("minotaur-sea-claim")?);
        black_box(run_rejected_assignment_scenario("centaur-fields-claim")?);
        Ok(())
    })?);

    print_measurement(&measure("gnome_to_minotaur", 500, || {
        black_box(run_gnome_minotaur_scenario()?);
        Ok(())
    })?);
    print_measurement(&measure("elf_to_centaur", 500, || {
        black_box(run_elf_centaur_scenario()?);
        Ok(())
    })?);
    print_measurement(&measure("regional_persistence", 500, || {
        let archive = encode_regional_archive(&elf.runtime)
            .map_err(|error| ScenarioError(error.to_string()))?;
        black_box(
            decode_regional_archive(&archive).map_err(|error| ScenarioError(error.to_string()))?,
        );
        Ok(())
    })?);
    print_measurement(&measure("regional_replay", 500, || {
        black_box(
            RegionalSynthesisRuntime::replay(elf.runtime.events().iter().cloned())
                .map_err(|error| ScenarioError(error.to_string()))?,
        );
        Ok(())
    })?);
    print_measurement(&measure("regional_lineage_inspection", 10_000, || {
        black_box(elf.runtime.lineage(&elf.result).expect("Centaur lineage"));
        Ok(())
    })?);
    print_measurement(&measure("stewardship_lookup", 10_000, || {
        black_box(
            gnome
                .runtime
                .stewardship(&gnome.result)
                .expect("Minotaur stewardship"),
        );
        Ok(())
    })?);
    print_measurement(&measure("guardianship_lookup", 10_000, || {
        black_box(
            elf.runtime
                .guardianship(&elf.result)
                .expect("Centaur guardianship"),
        );
        Ok(())
    })?);
    print_measurement(&measure("rejected_cross_regional", 1_000, || {
        black_box(run_rejected_regional_scenario("gnome-centaur")?);
        Ok(())
    })?);

    for event_count in [10_usize, 100, 1_000, 10_000] {
        let events = registration_history(event_count)?;
        let iterations = match event_count {
            10 => 200,
            100 => 100,
            1_000 => 20,
            _ => 3,
        };
        let label = match event_count {
            10 => "replay_10_events",
            100 => "replay_100_events",
            1_000 => "replay_1000_events",
            _ => "replay_10000_events",
        };
        print_measurement(&measure(label, iterations, || {
            black_box(
                RegionalSynthesisRuntime::replay(events.iter().cloned())
                    .map_err(|error| ScenarioError(error.to_string()))?,
            );
            Ok(())
        })?);
    }
    Ok(())
}
