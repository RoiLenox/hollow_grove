//! Reproducible clinical/practice witness using disposable output, never real saves.
use std::path::{Path, PathBuf};

use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, GameApplicationService, GameplayCommand, GameplayEventId,
    GameplayEventMetadata, LoreInspection, SympioteAdaptation, SympioteBoardwalkCamp,
    SympioteFoundationAction as A, SympioteFoundationView, SympiotePracticeDirection,
    migrate_gameplay_archive,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::session::WorldSession;

fn step(
    service: &mut GameApplicationService,
    command: GameplayCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let at = service.revision().get() + 1;
    service.execute(
        GameplayEventMetadata {
            id: GameplayEventId::new(format!("event.sympiote-witness.{at}"))?,
            causal_position: CausalPosition::new(at),
        },
        command,
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args().nth(1).map(PathBuf::from);
    let world = WorldSession::load_or_canonical_at(Path::new(env!("CARGO_MANIFEST_DIR")))?;
    let mut service =
        GameApplicationService::with_world_session(RuleSetId::new(V2_RULE_SET)?, world);
    step(
        &mut service,
        GameplayCommand::EstablishHuemanIdentity {
            continuity: BeingContinuityId::new("continuity.sympiote-witness.hueman")?,
            participant: ParticipantId::new("participant.sympiote-witness.hueman")?,
            institutional: InstitutionalBeingId::new("being.sympiote-witness.hueman")?,
        },
    )?;
    // V4 and V5 envelope payloads match for a history without foundation events.
    let before_v5 = service.encode_archive()?;
    let mut before: serde_json::Value = serde_json::from_str(&before_v5)?;
    before["schema_version"] = 4.into();
    let before = serde_json::to_string_pretty(&before)?;
    let migrated = migrate_gameplay_archive(&before)?;
    let untouched = GameApplicationService::from_archive(&migrated)?;
    assert!(!untouched.runtime().sympiote_foundation().form_established());
    for action in [
        A::RequestAssessment,
        A::ConsentToFoundation,
        A::AskClinicianToEstablish,
        A::Rest,
        A::AskClinicianToDischarge,
        A::ExploreDirection {
            direction: SympiotePracticeDirection::Flynt,
        },
        A::ConsiderBoardwalkOffer {
            camp: SympioteBoardwalkCamp::PimpAndTrooners,
        },
        A::ConsiderBoardwalkOffer {
            camp: SympioteBoardwalkCamp::GimpAndGooners,
        },
        A::Practice {
            adaptation: SympioteAdaptation::Coordination,
        },
        A::Rest,
        A::Practice {
            adaptation: SympioteAdaptation::Coordination,
        },
        A::Rest,
    ] {
        step(
            &mut service,
            GameplayCommand::AdvanceSympioteFoundation { action },
        )?;
    }
    let after = service.encode_archive()?;
    let replayed = GameApplicationService::from_archive(&after)?;
    assert_eq!(
        replayed.runtime().sympiote_foundation(),
        service.runtime().sympiote_foundation()
    );
    let view = SympioteFoundationView::from_runtime(replayed.runtime());
    assert!(view.form_established && view.comfortable_use && !view.needs_rest);
    if let Some(output) = output {
        std::fs::create_dir_all(&output)?;
        std::fs::write(output.join("before-v4.json"), before)?;
        std::fs::write(output.join("migrated-v6.json"), migrated)?;
        std::fs::write(output.join("comfortable-use-v6.json"), after)?;
        std::fs::write(
            output.join("lore-inspection.json"),
            serde_json::to_string_pretty(&LoreInspection::from_runtime(Some(replayed.runtime()))?)?,
        )?;
    }
    println!("{}", serde_json::to_string_pretty(&view)?);
    Ok(())
}
