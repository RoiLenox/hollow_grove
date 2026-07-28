//! Golden executable Service Tournament year.
//!
//! `service-tournament.canonical-year.v1` is the stable fixture against which
//! archive, migration, replay, and constitutional audit behavior is frozen.

use std::collections::{BTreeMap, BTreeSet};

use crate::constitutional::{
    FlyntManifestation, FlyntOperation, HouseInwardOperation, HouseOutwardManifestation,
    HouseSynthesisPath, PointSquaredRelationshipRecord, SemanticContactOutcome, SynthesisAttemptId,
    SynthesisSemanticEvent, SynthesisSemanticEventId, SynthesisSemanticEventKind,
};
use crate::hollow_grove_contract::House;

use super::service_tournament::{
    AllianceId, ArtifactId, ArtifactRefinementId, CompetitorId, ConstitutionalPenalty, EmergencyId,
    HouseColorFamily, HouseScorecard, MarkAction, MarkGrammar, MarkId, ObjectiveId, PaintMark,
    PaintMarkProvenance, PairedServiceIdentity, PrizeAwardId, ResultId, ScenarioId,
    ScenarioPhaseId, ScenarioType, ScoringCategory, ScoringEventId, ServiceMark, ServiceMarkId,
    ServiceMarkProvenance, ServiceMarkSignificance, SimulationSystem, TournamentAuthorityId,
    TournamentAward, TournamentCompetitor, TournamentEvent, TournamentEventId, TournamentEventKind,
    TournamentEvidenceId, TournamentId, TournamentLocationId, TournamentObjective,
    TournamentResult, TournamentScenario, TournamentYearId, ViolationId,
    canonical_service_tournament, canonical_war_of_a_thousand_hues,
};
use super::service_tournament_archive::{
    ArtifactCustodyRecord, ArtifactRefinementRecord, CANONICAL_TOURNAMENT_YEAR_ID,
    ConstitutionalViolationRecord, EDGE_OF_TOMORROW_ID, FlagshipArtifactKind,
    GLASS_OF_A_THOUSAND_HUES_ID, PrizeAwardRecord, PrizeDegree, PrizeLineage, PrizeSubjectKind,
    RealEmergencyRecord, ScenarioPhase, ScenarioPhaseRecord, ScoringEventRecord,
    ServiceTournamentArchivePayload, TemporaryAllianceRecord, TournamentYearRecord,
    WarOutcomeRecord,
};

fn tournament_id() -> TournamentId {
    canonical_service_tournament().id
}

fn year_id() -> TournamentYearId {
    TournamentYearId::new(CANONICAL_TOURNAMENT_YEAR_ID).expect("canonical year ID")
}

fn competitor_id(house: House) -> CompetitorId {
    CompetitorId::new(format!(
        "competitor.canonical-year.{}",
        house.as_str().to_ascii_lowercase()
    ))
    .expect("canonical competitor ID")
}

fn evidence(value: &str) -> TournamentEvidenceId {
    TournamentEvidenceId::new(value).expect("canonical evidence ID")
}

fn event(sequence: u64, kind: TournamentEventKind) -> TournamentEvent {
    TournamentEvent {
        id: TournamentEventId::new(format!("event.canonical-year.{sequence:02}"))
            .expect("canonical event ID"),
        tournament: tournament_id(),
        semantic_sequence: sequence,
        kind,
    }
}

fn representative(identity: PairedServiceIdentity) -> TournamentCompetitor {
    TournamentCompetitor {
        id: competitor_id(identity.house()),
        tournament: tournament_id(),
        house: identity.house(),
        service_identity: identity,
        public_name: format!(
            "{} complete service delegation",
            identity.house_display_name()
        ),
    }
}

fn objective(
    id: &str,
    category: ScoringCategory,
    description: &str,
    points: u16,
) -> TournamentObjective {
    TournamentObjective {
        id: ObjectiveId::new(id).expect("canonical objective ID"),
        category,
        description: description.into(),
        available_points: points,
    }
}

fn scenario(
    id: &str,
    scenario_type: ScenarioType,
    location: &str,
    objectives: Vec<TournamentObjective>,
    systems: impl IntoIterator<Item = SimulationSystem>,
) -> TournamentScenario {
    TournamentScenario {
        id: ScenarioId::new(id).expect("canonical scenario ID"),
        tournament: tournament_id(),
        war: Some(canonical_war_of_a_thousand_hues().id),
        scenario_type,
        operational_zone: TournamentLocationId::new(location)
            .expect("canonical Tournament location ID"),
        objectives,
        simulation_systems: systems.into_iter().collect(),
        nonlethal: true,
    }
}

fn scenarios() -> Vec<TournamentScenario> {
    vec![
        scenario(
            "scenario.canonical-year.west-bridge-breach",
            ScenarioType::ControlledBreach,
            "location.central-junction.west-bridge",
            vec![
                objective(
                    "objective.canonical-year.bridge-open",
                    ScoringCategory::Engineering,
                    "Open the controlled bridge without breaching the civilian line.",
                    25,
                ),
                objective(
                    "objective.canonical-year.bridge-restraint",
                    ScoringCategory::ConstitutionalRestraint,
                    "Stop force when the route becomes a protected corridor.",
                    25,
                ),
            ],
            [
                SimulationSystem::MockExplosive,
                SimulationSystem::MovableBarricade,
                SimulationSystem::PaintballStyleWeapon,
            ],
        ),
        scenario(
            "scenario.canonical-year.bridge-rescue",
            ScenarioType::RescueOperation,
            "location.central-junction.west-bridge",
            vec![
                objective(
                    "objective.canonical-year.rescue-extraction",
                    ScoringCategory::RescueSuccess,
                    "Extract scenario casualties after the breach changes conditions.",
                    30,
                ),
                objective(
                    "objective.canonical-year.rescue-cooperation",
                    ScoringCategory::Cooperation,
                    "Preserve the multi-House rescue corridor.",
                    20,
                ),
            ],
            [
                SimulationSystem::ScenarioCasualty,
                SimulationSystem::RescueAndExtractionEquipment,
                SimulationSystem::ColoredSmoke,
            ],
        ),
        scenario(
            "scenario.canonical-year.archive-interdiction",
            ScenarioType::EvidenceRecovery,
            "location.central-junction.archive-arcade",
            vec![objective(
                "objective.canonical-year.evidence-chain",
                ScoringCategory::EvidenceIntegrity,
                "Recover the staged archive without breaking chain of custody.",
                25,
            )],
            [
                SimulationSystem::FalseDocument,
                SimulationSystem::StagedEvidence,
                SimulationSystem::TemporaryIdentity,
            ],
        ),
        scenario(
            "scenario.canonical-year.false-emergency",
            ScenarioType::FalseEmergency,
            "location.central-junction.market-concourse",
            vec![objective(
                "objective.canonical-year.false-emergency-recognition",
                ScoringCategory::TrueScenarioRecognition,
                "Classify the staged emergency from incomplete evidence.",
                20,
            )],
            [
                SimulationSystem::CodedMessage,
                SimulationSystem::ColoredSmoke,
                SimulationSystem::ScenarioCasualty,
            ],
        ),
        scenario(
            "scenario.canonical-year.real-emergency-response",
            ScenarioType::DisasterResponse,
            "location.central-junction.market-concourse",
            vec![
                objective(
                    "objective.canonical-year.real-emergency-distinction",
                    ScoringCategory::ChangedConditionRecognition,
                    "Recognize that the market collapse is real and suspend play.",
                    30,
                ),
                objective(
                    "objective.canonical-year.real-emergency-civilians",
                    ScoringCategory::CivilianProtection,
                    "Protect and extract civilians under Function law.",
                    30,
                ),
            ],
            [
                SimulationSystem::RescueAndExtractionEquipment,
                SimulationSystem::DroneOrObservationTool,
            ],
        ),
    ]
}

fn paint_mark(
    sequence: u64,
    scenario_id: &ScenarioId,
    house: House,
    palette_sources: &[&str],
    hue_description: &str,
    action: MarkAction,
    grammar: MarkGrammar,
    layer_sequence: u64,
) -> PaintMark {
    let slug = house.as_str().to_ascii_lowercase();
    PaintMark {
        id: MarkId::new(format!("mark.canonical-year.four-house-collision.{slug}"))
            .expect("canonical paint mark ID"),
        war: canonical_war_of_a_thousand_hues().id,
        scenario: Some(scenario_id.clone()),
        location: TournamentLocationId::new("location.central-junction.west-bridge")
            .expect("canonical mark location"),
        house,
        color_family: HouseColorFamily::for_house(house),
        palette_sources: palette_sources
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        hue_description: hue_description.into(),
        action,
        grammar: Some(grammar),
        layer_sequence,
        provenance: PaintMarkProvenance {
            originating_event: TournamentEventId::new(format!(
                "event.canonical-year.{sequence:02}"
            ))
            .expect("canonical mark event ID"),
            competitor: competitor_id(house),
            evidence: [evidence(&format!(
                "evidence.canonical-year.four-house-collision.{slug}"
            ))]
            .into_iter()
            .collect(),
        },
    }
}

fn scorecard(points: u16, penalties: BTreeSet<ConstitutionalPenalty>) -> HouseScorecard {
    HouseScorecard {
        scores: ScoringCategory::ALL
            .into_iter()
            .map(|category| (category, points))
            .collect(),
        penalties,
    }
}

fn base_events(representatives: &[TournamentCompetitor]) -> Vec<TournamentEvent> {
    let scenarios = scenarios();
    let rescue_scenario_id =
        ScenarioId::new("scenario.canonical-year.bridge-rescue").expect("canonical scenario ID");
    let marks = vec![
        paint_mark(
            10,
            &rescue_scenario_id,
            House::Flynt,
            &["flynt.rich_black_blue", "flynt.gunmetal"],
            "weathered rich black blue and gunmetal burst",
            MarkAction::Breached,
            MarkGrammar::Burst,
            0,
        ),
        paint_mark(
            11,
            &rescue_scenario_id,
            House::Stonebend,
            &["stonebend.prussian_blue", "stonebend.lapis_lazuli"],
            "Prussian and lapis route line",
            MarkAction::Held,
            MarkGrammar::StraightLine,
            1,
        ),
        paint_mark(
            12,
            &rescue_scenario_id,
            House::Sandmanor,
            &["sandmanor.wine", "sandmanor.redwood"],
            "wine-red directed crossing",
            MarkAction::CrossedRoute,
            MarkGrammar::Arrow,
            2,
        ),
        paint_mark(
            13,
            &rescue_scenario_id,
            House::Glaushouse,
            &["glaushouse.brunswick_green", "glaushouse.viridian"],
            "viridian rescue cross",
            MarkAction::Rescued,
            MarkGrammar::Cross,
            3,
        ),
    ];

    let mut events = representatives
        .iter()
        .enumerate()
        .map(|(sequence, representative)| {
            event(
                sequence as u64,
                TournamentEventKind::CompetitorRegistered(representative.clone()),
            )
        })
        .collect::<Vec<_>>();
    events.push(event(4, TournamentEventKind::TournamentOpened));
    events.extend(scenarios.into_iter().enumerate().map(|(index, scenario)| {
        event(
            5 + index as u64,
            TournamentEventKind::ScenarioRegistered(scenario),
        )
    }));
    events.extend(
        marks
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, mark)| event(10 + index as u64, TournamentEventKind::MarkRecorded(mark))),
    );

    let preservation_event =
        TournamentEventId::new("event.canonical-year.14").expect("canonical event ID");
    let service_mark = ServiceMark {
        id: ServiceMarkId::new("service-mark.canonical-year.four-house-bridge")
            .expect("canonical Service Mark ID"),
        tournament_year_id: year_id(),
        war: canonical_war_of_a_thousand_hues().id,
        year: 2047,
        scenario: rescue_scenario_id,
        location: TournamentLocationId::new("location.central-junction.west-bridge")
            .expect("canonical Service Mark location"),
        houses: [
            House::Flynt,
            House::Stonebend,
            House::Sandmanor,
            House::Glaushouse,
        ]
        .into_iter()
        .collect(),
        operation_name: "The Four-House Bridge Extraction".into(),
        participants: representatives
            .iter()
            .map(|representative| representative.id.clone())
            .collect(),
        significance: ServiceMarkSignificance::FourHouseCollision,
        ordered_paint_layers: marks.iter().map(|mark| mark.id.clone()).collect(),
        constitutional_significance:
            "All four Houses stopped territorial play and made one protected rescue corridor."
                .into(),
        account: "Black opened the route, blue secured it, red redirected movement, and green completed extraction; the preserved layers record both contest and cooperation.".into(),
        provenance: ServiceMarkProvenance {
            preservation_event: preservation_event.clone(),
            source_marks: marks.iter().map(|mark| mark.id.clone()).collect(),
            source_action_events: marks
                .iter()
                .map(|mark| mark.provenance.originating_event.clone())
                .collect(),
            authorized_by: TournamentAuthorityId::new(
                "authority.service-tournament.service-mark-keeper",
            )
            .expect("canonical authority ID"),
            evidence: [
                evidence("evidence.canonical-year.four-house-bridge-ledger"),
                evidence("evidence.canonical-year.four-house-bridge-witness"),
            ]
            .into_iter()
            .collect(),
        },
    };
    events.push(event(
        14,
        TournamentEventKind::ServiceMarkPreserved(service_mark),
    ));

    let result = TournamentResult {
        id: ResultId::new("result.canonical-year.final").expect("canonical result ID"),
        tournament: tournament_id(),
        champion: House::Glaushouse,
        scorecards: [
            (
                House::Flynt,
                scorecard(
                    9,
                    [ConstitutionalPenalty::ExcessiveForce]
                        .into_iter()
                        .collect(),
                ),
            ),
            (House::Stonebend, scorecard(11, BTreeSet::new())),
            (House::Sandmanor, scorecard(12, BTreeSet::new())),
            (House::Glaushouse, scorecard(13, BTreeSet::new())),
        ]
        .into_iter()
        .collect(),
        award: TournamentAward::PublicRecognition,
        transfers_permanent_sovereignty: false,
    };
    events.push(event(15, TournamentEventKind::ResultRecorded(result)));
    events
}

fn phases() -> Vec<ScenarioPhaseRecord> {
    let year = year_id();
    [
        (
            "phase.canonical-year.bridge-active",
            "scenario.canonical-year.west-bridge-breach",
            ScenarioPhase::Active,
            0,
        ),
        (
            "phase.canonical-year.bridge-overlap",
            "scenario.canonical-year.west-bridge-breach",
            ScenarioPhase::Overlapping,
            1,
        ),
        (
            "phase.canonical-year.rescue-active",
            "scenario.canonical-year.bridge-rescue",
            ScenarioPhase::Active,
            0,
        ),
        (
            "phase.canonical-year.false-emergency-active",
            "scenario.canonical-year.false-emergency",
            ScenarioPhase::Active,
            0,
        ),
        (
            "phase.canonical-year.false-emergency-suspended",
            "scenario.canonical-year.false-emergency",
            ScenarioPhase::EmergencySuspended,
            1,
        ),
        (
            "phase.canonical-year.real-response-active",
            "scenario.canonical-year.real-emergency-response",
            ScenarioPhase::Active,
            0,
        ),
        (
            "phase.canonical-year.real-response-resolved",
            "scenario.canonical-year.real-emergency-response",
            ScenarioPhase::Resolved,
            1,
        ),
    ]
    .into_iter()
    .map(|(id, scenario_id, phase, sequence)| ScenarioPhaseRecord {
        id: ScenarioPhaseId::new(id).expect("canonical phase ID"),
        tournament_year_id: year.clone(),
        scenario_id: ScenarioId::new(scenario_id).expect("canonical scenario ID"),
        phase,
        semantic_sequence: sequence,
        evidence_references: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
    })
    .collect()
}

fn alliances() -> Vec<TemporaryAllianceRecord> {
    vec![TemporaryAllianceRecord {
        id: AllianceId::new("alliance.canonical-year.stonebend-sandmanor")
            .expect("canonical alliance ID"),
        tournament_year_id: year_id(),
        houses: [House::Stonebend, House::Sandmanor].into_iter().collect(),
        participant_ids: [
            competitor_id(House::Stonebend),
            competitor_id(House::Sandmanor),
        ]
        .into_iter()
        .collect(),
        scenario_ids: [
            ScenarioId::new("scenario.canonical-year.archive-interdiction")
                .expect("canonical scenario ID"),
            ScenarioId::new("scenario.canonical-year.false-emergency")
                .expect("canonical scenario ID"),
        ]
        .into_iter()
        .collect(),
        temporary: true,
        ends_with_tournament_year: true,
        evidence_references: [evidence("evidence.canonical-year.temporary-alliance")]
            .into_iter()
            .collect(),
    }]
}

fn real_emergencies() -> Vec<RealEmergencyRecord> {
    vec![RealEmergencyRecord {
        id: EmergencyId::new("emergency.canonical-year.market-canopy-collapse")
            .expect("canonical emergency ID"),
        tournament_year_id: year_id(),
        simulation_scenario_id: ScenarioId::new("scenario.canonical-year.false-emergency")
            .expect("canonical scenario ID"),
        response_scenario_id: ScenarioId::new(
            "scenario.canonical-year.real-emergency-response",
        )
        .expect("canonical scenario ID"),
        location_id: TournamentLocationId::new(
            "location.central-junction.market-concourse",
        )
        .expect("canonical location ID"),
        initially_interpreted_as_simulation: true,
        determined_real: true,
        simulation_suspended: true,
        recognized_by: House::Glaushouse,
        distinction_authority: TournamentAuthorityId::new(
            "authority.central-junction.function-safety-bench",
        )
        .expect("canonical distinction authority"),
        evidence_references: [
            evidence("evidence.canonical-year.market-load-sensor"),
            evidence("evidence.canonical-year.nightingale-triage"),
        ]
        .into_iter()
        .collect(),
        account: "Glaüshouse identified a real canopy failure inside the staged alarm, suspended scoring, and opened a lawful four-House rescue response.".into(),
    }]
}

fn scoring_events() -> Vec<ScoringEventRecord> {
    [
        (
            "score.canonical-year.flynt-breach",
            House::Flynt,
            ScoringCategory::Engineering,
            25,
            "scenario.canonical-year.west-bridge-breach",
            false,
            "Flynt completed the controlled breach.",
        ),
        (
            "score.canonical-year.stonebend-evidence",
            House::Stonebend,
            ScoringCategory::EvidenceIntegrity,
            22,
            "scenario.canonical-year.archive-interdiction",
            false,
            "Stonebend preserved the evidence chain.",
        ),
        (
            "score.canonical-year.sandmanor-recognition",
            House::Sandmanor,
            ScoringCategory::TrueScenarioRecognition,
            24,
            "scenario.canonical-year.false-emergency",
            false,
            "Sandmanor identified the hidden relationship between scenarios.",
        ),
        (
            "score.canonical-year.glaushouse-rescue",
            House::Glaushouse,
            ScoringCategory::RescueSuccess,
            30,
            "scenario.canonical-year.real-emergency-response",
            false,
            "Glaüshouse completed civilian extraction.",
        ),
        (
            "score.canonical-year.glaushouse-restraint",
            House::Glaushouse,
            ScoringCategory::ConstitutionalRestraint,
            20,
            "scenario.canonical-year.real-emergency-response",
            true,
            "Glaüshouse stopped the game when conditions became real.",
        ),
        (
            "score.canonical-year.flynt-excessive-force",
            House::Flynt,
            ScoringCategory::ConstitutionalRestraint,
            -30,
            "scenario.canonical-year.west-bridge-breach",
            true,
            "Flynt accepted a restraint deduction after overdriving a mock charge.",
        ),
    ]
    .into_iter()
    .map(
        |(id, house, category, points_delta, scenario, restraint, reason)| ScoringEventRecord {
            id: ScoringEventId::new(id).expect("canonical scoring event ID"),
            tournament_year_id: year_id(),
            house,
            category,
            points_delta,
            scenario_id: ScenarioId::new(scenario).expect("canonical scenario ID"),
            evidence_references: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
            constitutional_restraint_decision: restraint,
            reason: reason.into(),
        },
    )
    .collect()
}

fn violations() -> Vec<ConstitutionalViolationRecord> {
    vec![ConstitutionalViolationRecord {
        id: ViolationId::new("violation.canonical-year.flynt-excessive-force")
            .expect("canonical violation ID"),
        tournament_year_id: year_id(),
        house: House::Flynt,
        violation: ConstitutionalPenalty::ExcessiveForce,
        scenario_id: ScenarioId::new("scenario.canonical-year.west-bridge-breach")
            .expect("canonical scenario ID"),
        points_deduction: 30,
        invalidates_tactical_victory: true,
        evidence_references: [
            evidence("evidence.canonical-year.bridge-pressure-gauge"),
            evidence("evidence.canonical-year.public-safety-line"),
        ]
        .into_iter()
        .collect(),
        account: "The tactical breach succeeded, but excess simulated force crossed the declared restraint threshold and invalidated tactical supremacy as constitutional victory.".into(),
    }]
}

fn artifact_refinements() -> Vec<ArtifactRefinementRecord> {
    vec![
        ArtifactRefinementRecord {
            id: ArtifactRefinementId::new("refinement.canonical-year.edge-route-lock")
                .expect("canonical refinement ID"),
            tournament_year_id: year_id(),
            artifact_id: ArtifactId::new(EDGE_OF_TOMORROW_ID).expect("canonical Edge ID"),
            artifact_kind: FlagshipArtifactKind::EdgeOfTomorrow,
            refinement_sequence: 0,
            previous_refinement_id: None,
            contributing_house: House::Flynt,
            contribution: "A lawful route-lock recomposition learned from the bridge rescue."
                .into(),
            lawful_recomposition: true,
            replaces_artifact: false,
            evidence_references: [evidence("evidence.canonical-year.edge-route-lock")]
                .into_iter()
                .collect(),
        },
        ArtifactRefinementRecord {
            id: ArtifactRefinementId::new("refinement.canonical-year.glass-emergency-facet")
                .expect("canonical refinement ID"),
            tournament_year_id: year_id(),
            artifact_id: ArtifactId::new(GLASS_OF_A_THOUSAND_HUES_ID).expect("canonical Glass ID"),
            artifact_kind: FlagshipArtifactKind::GlassOfAThousandHues,
            refinement_sequence: 0,
            previous_refinement_id: None,
            contributing_house: House::Glaushouse,
            contribution:
                "A diagnostic facet distinguishing staged distress from living-system failure."
                    .into(),
            lawful_recomposition: true,
            replaces_artifact: false,
            evidence_references: [evidence("evidence.canonical-year.glass-emergency-facet")]
                .into_iter()
                .collect(),
        },
    ]
}

fn flagship_custody(kind: FlagshipArtifactKind, house: House) -> ArtifactCustodyRecord {
    ArtifactCustodyRecord {
        artifact_id: ArtifactId::new(kind.stable_id()).expect("canonical artifact ID"),
        artifact_kind: kind,
        custodian_id: competitor_id(house),
        custodian_house: house,
        title: kind.custody_title().into(),
        temporary_until_next_tournament: true,
        custody_is_ownership: false,
        grants_sovereignty: false,
    }
}

fn prize_awards() -> Vec<PrizeAwardRecord> {
    vec![
        PrizeAwardRecord {
            id: PrizeAwardId::new("award.canonical-year.edge-custody").expect("canonical award ID"),
            tournament_year_id: year_id(),
            recipient_id: competitor_id(House::Stonebend),
            recipient_house: House::Stonebend,
            lineage: PrizeLineage::Current,
            degree: PrizeDegree::FlagshipArtifact,
            subject_kind: PrizeSubjectKind::FlagshipArtifact,
            subject_id: EDGE_OF_TOMORROW_ID.into(),
            flagship_custody: Some(flagship_custody(
                FlagshipArtifactKind::EdgeOfTomorrow,
                House::Stonebend,
            )),
            transfers_ownership: false,
            grants_sovereignty: false,
            evidence_references: [evidence("evidence.canonical-year.edge-award")]
                .into_iter()
                .collect(),
        },
        PrizeAwardRecord {
            id: PrizeAwardId::new("award.canonical-year.glass-custody")
                .expect("canonical award ID"),
            tournament_year_id: year_id(),
            recipient_id: competitor_id(House::Sandmanor),
            recipient_house: House::Sandmanor,
            lineage: PrizeLineage::Aura,
            degree: PrizeDegree::FlagshipArtifact,
            subject_kind: PrizeSubjectKind::FlagshipArtifact,
            subject_id: GLASS_OF_A_THOUSAND_HUES_ID.into(),
            flagship_custody: Some(flagship_custody(
                FlagshipArtifactKind::GlassOfAThousandHues,
                House::Sandmanor,
            )),
            transfers_ownership: false,
            grants_sovereignty: false,
            evidence_references: [evidence("evidence.canonical-year.glass-award")]
                .into_iter()
                .collect(),
        },
        PrizeAwardRecord {
            id: PrizeAwardId::new("award.canonical-year.synthesis-recipe")
                .expect("canonical award ID"),
            tournament_year_id: year_id(),
            recipient_id: competitor_id(House::Glaushouse),
            recipient_house: House::Glaushouse,
            lineage: PrizeLineage::GlaushouseTradition,
            degree: PrizeDegree::SynthesisRecipePattern,
            subject_kind: PrizeSubjectKind::SynthesisRecipePattern,
            subject_id: "recipe.canonical-year.recovery-corridor".into(),
            flagship_custody: None,
            transfers_ownership: false,
            grants_sovereignty: false,
            evidence_references: [evidence("evidence.canonical-year.recipe-award")]
                .into_iter()
                .collect(),
        },
        PrizeAwardRecord {
            id: PrizeAwardId::new("award.canonical-year.synthesis-core")
                .expect("canonical award ID"),
            tournament_year_id: year_id(),
            recipient_id: competitor_id(House::Flynt),
            recipient_house: House::Flynt,
            lineage: PrizeLineage::FlyntTradition,
            degree: PrizeDegree::SynthesisCore,
            subject_kind: PrizeSubjectKind::SynthesisCore,
            subject_id: "core.canonical-year.route-spark".into(),
            flagship_custody: None,
            transfers_ownership: false,
            grants_sovereignty: false,
            evidence_references: [evidence("evidence.canonical-year.core-award")]
                .into_iter()
                .collect(),
        },
    ]
}

fn synthesis_events() -> Vec<SynthesisSemanticEvent> {
    let attempt = SynthesisAttemptId::new("attempt.canonical-year.flynt-route-recog")
        .expect("canonical attempt ID");
    vec![
        SynthesisSemanticEvent {
            id: SynthesisSemanticEventId::new("synthesis-event.canonical-year.aim")
                .expect("canonical semantic event ID"),
            semantic_sequence: 0,
            kind: SynthesisSemanticEventKind::Aim {
                attempt_id: attempt.clone(),
                path: HouseSynthesisPath::Flynt,
                operation: HouseInwardOperation::Flynt(FlyntOperation::Resynce),
                recipe_id: "recipe.canonical-year.flynt-route-recog".into(),
                compiler_id: "compiler.current-synthesis.v1".into(),
                script_id: "script.canonical-year.flynt-route-recog".into(),
                aim_id: "aim.canonical-year.flynt-route-recog".into(),
                manifestation_before: None,
                evidence_references: ["evidence.canonical-year.flynt-resynce".into()]
                    .into_iter()
                    .collect(),
            },
        },
        SynthesisSemanticEvent {
            id: SynthesisSemanticEventId::new("synthesis-event.canonical-year.fire")
                .expect("canonical semantic event ID"),
            semantic_sequence: 1,
            kind: SynthesisSemanticEventKind::Fire {
                attempt_id: attempt.clone(),
                fire_id: "fire.canonical-year.flynt-route-recog".into(),
            },
        },
        SynthesisSemanticEvent {
            id: SynthesisSemanticEventId::new("synthesis-event.canonical-year.kiss")
                .expect("canonical semantic event ID"),
            semantic_sequence: 2,
            kind: SynthesisSemanticEventKind::Contact {
                attempt_id: attempt.clone(),
                outcome: SemanticContactOutcome::Kiss,
                accepted_by: "authority.flynt.tross".into(),
                evidence_references: ["evidence.canonical-year.flynt-kiss".into()]
                    .into_iter()
                    .collect(),
            },
        },
        SynthesisSemanticEvent {
            id: SynthesisSemanticEventId::new("synthesis-event.canonical-year.point-squared")
                .expect("canonical semantic event ID"),
            semantic_sequence: 3,
            kind: SynthesisSemanticEventKind::PointSquared {
                attempt_id: attempt,
                record: PointSquaredRelationshipRecord {
                    record_id: "point-squared.canonical-year.flynt-route-recog".into(),
                    relationship_id: "relationship.canonical-year.flynt-route".into(),
                    authority_id: "authority.flynt.tross".into(),
                    location_id: "location.central-junction.west-bridge".into(),
                    recipe_id: "recipe.canonical-year.flynt-route-recog".into(),
                    provenance_id: "provenance.canonical-year.flynt-route-recog".into(),
                    result_id: "result.canonical-year.flynt-route-recog".into(),
                    evidence_references: [
                        "evidence.canonical-year.flynt-kiss".into(),
                        "evidence.canonical-year.flynt-route-recog".into(),
                    ]
                    .into_iter()
                    .collect(),
                    manifestation: HouseOutwardManifestation::Flynt(FlyntManifestation::Recog),
                },
            },
        },
    ]
}

#[must_use]
pub fn canonical_tournament_year_fixture() -> TournamentYearRecord {
    let representatives = PairedServiceIdentity::ALL
        .into_iter()
        .map(representative)
        .collect::<Vec<_>>();
    TournamentYearRecord {
        id: year_id(),
        calendar_year: 2047,
        tournament_id: tournament_id(),
        central_junction_function_id: "function.central-junction.service-tournament".into(),
        representatives: representatives.clone(),
        events: base_events(&representatives),
        scenario_phases: phases(),
        alliances: alliances(),
        real_emergencies: real_emergencies(),
        scoring_events: scoring_events(),
        constitutional_violations: violations(),
        artifact_refinements: artifact_refinements(),
        prize_awards: prize_awards(),
        synthesis_semantic_events: synthesis_events(),
        war_result: WarOutcomeRecord {
            war_id: canonical_war_of_a_thousand_hues().id,
            result_id: "result.canonical-year.final".into(),
            tactical_leader: House::Flynt,
            elimination_counts: [
                (House::Flynt, 42),
                (House::Stonebend, 31),
                (House::Sandmanor, 27),
                (House::Glaushouse, 18),
            ]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
            constitutional_champion: House::Glaushouse,
            nonlethal: true,
            transfers_permanent_sovereignty: false,
        },
    }
}

#[must_use]
pub fn canonical_service_tournament_archive_fixture() -> ServiceTournamentArchivePayload {
    ServiceTournamentArchivePayload {
        years: vec![canonical_tournament_year_fixture()],
    }
}
