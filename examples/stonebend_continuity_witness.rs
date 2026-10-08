//! Disposable replay witnesses for Stonebend's three existing Current Sea dispositions.
use std::path::{Path, PathBuf};

use hollow_grove::architecture_audit::ArchitectureCoverageReport;
use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameplayCommand, GameplayEventId,
    GameplayEventMetadata, HuemanFaculty, LoreInspection, StonebendContinuityChoice, WorldMapId,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::session::WorldSession;

type Error = Box<dyn std::error::Error>;

fn execute(service: &mut GameApplicationService, command: GameplayCommand) -> Result<(), Error> {
    let at = u64::try_from(service.events().len())? + 1;
    service.execute(
        GameplayEventMetadata {
            id: GameplayEventId::new(format!("event.current-sea.witness.{at}"))?,
            causal_position: CausalPosition::new(at),
        },
        command,
    )?;
    Ok(())
}

fn walk(
    service: &mut GameApplicationService,
    direction: CardinalDirection,
    steps: usize,
) -> Result<(), Error> {
    for _ in 0..steps {
        execute(service, GameplayCommand::MoveHueman { direction })?;
    }
    Ok(())
}

fn observe(service: &mut GameApplicationService) -> Result<(), Error> {
    walk(service, CardinalDirection::North, 1)?;
    execute(service, GameplayCommand::InteractHueman)
}

fn witness(
    root: &Path,
    choice: StonebendContinuityChoice,
) -> Result<GameApplicationService, Error> {
    let world = WorldSession::load_or_canonical_at(root)?;
    let mut service =
        GameApplicationService::with_world_session(RuleSetId::new(V2_RULE_SET)?, world);
    execute(
        &mut service,
        GameplayCommand::EstablishHuemanIdentity {
            continuity: BeingContinuityId::new("continuity.current-sea.witness")?,
            participant: ParticipantId::new("participant.current-sea.witness")?,
            institutional: InstitutionalBeingId::new("being.current-sea.witness")?,
        },
    )?;
    execute(
        &mut service,
        GameplayCommand::EnterMap {
            map: WorldMapId::CurrentSeaDeepCertificationLanding,
        },
    )?;
    // Approach and inspect the restoration archive, Name ledger, Mirror,
    // public-circulation witness and Mercy Deep through ordinary map commands.
    walk(&mut service, CardinalDirection::West, 5)?;
    walk(&mut service, CardinalDirection::North, 1)?;
    observe(&mut service)?;
    walk(&mut service, CardinalDirection::East, 1)?;
    walk(&mut service, CardinalDirection::North, 4)?;
    walk(&mut service, CardinalDirection::West, 1)?;
    observe(&mut service)?;
    walk(&mut service, CardinalDirection::East, 10)?;
    observe(&mut service)?;
    walk(&mut service, CardinalDirection::West, 1)?;
    walk(&mut service, CardinalDirection::North, 5)?;
    observe(&mut service)?;
    walk(&mut service, CardinalDirection::West, 5)?;
    observe(&mut service)?;
    for faculty in HuemanFaculty::ALL {
        execute(
            &mut service,
            GameplayCommand::DiscloseFacultyObservation { faculty },
        )?;
    }
    execute(
        &mut service,
        GameplayCommand::SupportStonebendContinuityOption { choice },
    )?;
    execute(
        &mut service,
        GameplayCommand::AskStonebendToDetermineContinuity,
    )?;
    Ok(service)
}

fn main() -> Result<(), Error> {
    let mut args = std::env::args().skip(1);
    let output = args
        .next()
        .map(PathBuf::from)
        .ok_or("Usage: cargo run --example stonebend_continuity_witness -- OUTPUT_DIRECTORY")?;
    if args.next().is_some() {
        return Err("only one output directory is accepted".into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(&output)?;
    let mut cases = Vec::new();
    for (key, choice) in [
        ("existing", StonebendContinuityChoice::AffirmExistingName),
        (
            "provisional",
            StonebendContinuityChoice::ProvisionalTransformedFormName,
        ),
        ("referral", StonebendContinuityChoice::ReferIdentityConflict),
    ] {
        let service = witness(root, choice)?;
        let archive = service.encode_archive()?;
        let replayed = GameApplicationService::from_archive(&archive)?;
        let inspection = LoreInspection::from_runtime(Some(replayed.runtime()))?;
        let execution = inspection
            .definitions
            .iter()
            .find(|item| item.lore_id == hollow_grove::gameplay::CURRENT_SEA_CONTINUITY_LORE_ID)
            .and_then(|item| item.stonebend_execution.as_ref())
            .ok_or("missing execution")?;
        if replayed.encode_archive()? != archive
            || !execution.complete
            || execution
                .outcome
                .as_ref()
                .is_none_or(|outcome| outcome.title_granted)
        {
            return Err("continuity witness failed equivalence or scope checks".into());
        }
        let coverage = ArchitectureCoverageReport::inspect(root, Some(replayed.runtime()))?;
        if !coverage.source_links_passed {
            return Err("architecture source links failed".into());
        }
        let filename = format!("{key}-gameplay-v5.json");
        std::fs::write(output.join(&filename), &archive)?;
        std::fs::write(
            output.join(format!("{key}-lore-v3.json")),
            serde_json::to_string_pretty(&inspection)?,
        )?;
        std::fs::write(
            output.join(format!("{key}-architecture-v1.json")),
            serde_json::to_string_pretty(&coverage)?,
        )?;
        cases.push(serde_json::json!({"choice": choice, "archive": filename,
            "event_count": service.events().len(), "archive_bytes": archive.len(),
            "replay_byte_equivalent": true, "bounded_disposition_complete": execution.complete,
            "high_review_pending": execution.high_review_pending, "title_granted": false}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": 1, "disposable_state": true, "world_source": root,
            "output_directory": output, "cases": cases
        }))?
    );
    Ok(())
}
