//! Measure command transactions, cloning, encoding and replay with disposable history.
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;

use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameplayCommand, GameplayEventId,
    GameplayEventMetadata, HollowGroveGameRuntime, encode_gameplay_archive,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::session::WorldSession;
use serde::Serialize;

type Error = Box<dyn std::error::Error>;

fn metadata(at: u64) -> Result<GameplayEventMetadata, Error> {
    Ok(GameplayEventMetadata {
        id: GameplayEventId::new(format!("event.history-profile.{at}"))?,
        causal_position: CausalPosition::new(at),
    })
}

fn identity(suffix: &str) -> Result<GameplayCommand, Error> {
    Ok(GameplayCommand::EstablishHuemanIdentity {
        continuity: BeingContinuityId::new(format!("continuity.history-profile.{suffix}"))?,
        participant: ParticipantId::new(format!("participant.history-profile.{suffix}"))?,
        institutional: InstitutionalBeingId::new(format!("being.history-profile.{suffix}"))?,
    })
}

fn archive(runtime: &HollowGroveGameRuntime) -> Result<String, Error> {
    Ok(encode_gameplay_archive(
        runtime.rule_set(),
        runtime.events(),
        runtime.world_session(),
    )?)
}

#[derive(Serialize)]
struct Timing {
    operation: &'static str,
    median_us: f64,
    p95_us: f64,
    max_us: f64,
}

fn measure(
    operation: &'static str,
    samples: usize,
    warmup: usize,
    mut sample: impl FnMut() -> Result<f64, Error>,
) -> Result<Timing, Error> {
    let mut values = Vec::with_capacity(samples);
    for index in 0..samples + warmup {
        let value = sample()?;
        if index >= warmup {
            values.push(value);
        }
    }
    values.sort_by(f64::total_cmp);
    Ok(Timing {
        operation,
        median_us: values[values.len() / 2],
        p95_us: values[(values.len() * 95).div_ceil(100).saturating_sub(1)],
        max_us: values[values.len() - 1],
    })
}

#[derive(Serialize)]
struct HistoryProfile {
    movement_history: usize,
    event_count: usize,
    archive_bytes: usize,
    replay_equivalent: bool,
    failed_command_unchanged: bool,
    timings: Vec<Timing>,
}

fn main() -> Result<(), Error> {
    let mut world_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut histories = vec![0, 128, 1024, 4096];
    let mut samples: usize = 20;
    let mut warmup: usize = 3;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--world-root" => world_root = args.next().ok_or("missing world root")?.into(),
            "--histories" => {
                histories = args
                    .next()
                    .ok_or("missing histories")?
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()?
            }
            "--samples" => samples = args.next().ok_or("missing samples")?.parse()?,
            "--warmup" => warmup = args.next().ok_or("missing warmup")?.parse()?,
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }
    if samples == 0 || histories.is_empty() || samples.checked_add(warmup).is_none() {
        return Err("positive samples and nonempty histories required".into());
    }
    let world = WorldSession::load_or_canonical_at(&world_root)?;
    let mut profiles = Vec::new();
    for history in histories {
        let mut runtime =
            HollowGroveGameRuntime::with_world_session(RuleSetId::new(V2_RULE_SET)?, world.clone());
        runtime.execute(metadata(1)?, identity("hueman")?)?;
        for index in 0..history {
            runtime.execute(
                metadata(u64::try_from(index)? + 2)?,
                GameplayCommand::MoveHueman {
                    direction: if index % 2 == 0 {
                        CardinalDirection::East
                    } else {
                        CardinalDirection::West
                    },
                },
            )?;
        }
        let encoded = archive(&runtime)?;
        let restored = GameApplicationService::from_archive(&encoded)?;
        if restored.encode_archive()? != encoded {
            return Err("fixture replay differs".into());
        }
        let next = u64::try_from(runtime.events().len())? + 1;
        let mut refused = runtime.clone();
        if refused
            .execute(metadata(next)?, identity("another")?)
            .is_ok()
            || archive(&refused)? != encoded
        {
            return Err("failed command changed fixture".into());
        }
        let mut timings = Vec::new();
        timings.push(measure("aggregate_clone", samples, warmup, || {
            let start = Instant::now();
            let cloned = black_box(runtime.clone());
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            black_box(cloned.events().len());
            Ok(elapsed)
        })?);
        timings.push(measure("accepted_command", samples, warmup, || {
            // Fixture fork and command construction are deliberately outside timing.
            let mut trial = runtime.clone();
            let meta = metadata(next)?;
            let command = GameplayCommand::MoveHueman {
                direction: CardinalDirection::East,
            };
            let start = Instant::now();
            black_box(trial.execute(meta, command)?);
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            if trial.events().len() != runtime.events().len() + 1 {
                return Err("missing committed event".into());
            }
            Ok(elapsed)
        })?);
        timings.push(measure("rejected_command", samples, warmup, || {
            let mut trial = runtime.clone();
            let meta = metadata(next)?;
            let command = identity("another")?;
            let start = Instant::now();
            let rejected = black_box(trial.execute(meta, command)).is_err();
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            if !rejected
                || trial.events() != runtime.events()
                || trial.revision() != runtime.revision()
            {
                return Err("rejected command changed history".into());
            }
            Ok(elapsed)
        })?);
        timings.push(measure("archive_encode", samples, warmup, || {
            let start = Instant::now();
            let output = black_box(archive(&runtime)?);
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            black_box(output.len());
            Ok(elapsed)
        })?);
        timings.push(measure("decode_and_replay", samples, warmup, || {
            let start = Instant::now();
            let restored = black_box(GameApplicationService::from_archive(&encoded)?);
            let elapsed = start.elapsed().as_secs_f64() * 1e6;
            if restored.events() != runtime.events() {
                return Err("replay changed events".into());
            }
            Ok(elapsed)
        })?);
        profiles.push(HistoryProfile {
            movement_history: history,
            event_count: runtime.events().len(),
            archive_bytes: encoded.len(),
            replay_equivalent: true,
            failed_command_unchanged: true,
            timings,
        });
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": 1, "workload": "gameplay_history_transactions",
            "samples": samples, "warmup": warmup, "disposable_state": true,
            "fixture_setup_timed": false, "fixture_fork_timed": false,
            "profiles": profiles
        }))?
    );
    Ok(())
}
