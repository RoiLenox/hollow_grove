use std::any::TypeId;
use std::collections::BTreeSet;

use hollow_grove::constitutional::{
    FlyntManifestation, FlyntOperation, GlaushouseManifestation, GlaushouseOperation,
    HouseInwardOperation, HouseOutwardManifestation, HouseSynthesisPath,
    HouseSynthesisSemanticError, HouseSynthesisSemanticRuntime, PointSquaredRelationshipRecord,
    SandmanorJurisdiction, SandmanorManifestation, SandmanorOperation, SandmanorSourceLineage,
    SemanticContactOutcome, StonebendManifestation, SynthesisAttemptId, SynthesisSemanticEvent,
    SynthesisSemanticEventId, SynthesisSemanticEventKind,
};
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::service_tournament::{
    ConstitutionalPenalty, HouseColorFamily, PairedServiceIdentity, ScoringCategory,
};
use hollow_grove::world::service_tournament_archive::{
    CANONICAL_TOURNAMENT_YEAR_ID, EDGE_OF_TOMORROW_ID, FlagshipArtifactKind,
    GLASS_OF_A_THOUSAND_HUES_ID, PrizeDegree, PrizeSubjectKind, SERVICE_TOURNAMENT_ARCHIVE_VERSION,
    ServiceTournamentArchiveError, decode_service_tournament_archive,
    encode_legacy_service_tournament_archive_v0, encode_service_tournament_archive, final_result,
    flagship_artifact, migrate_service_tournament_archive,
};
use hollow_grove::world::service_tournament_fixture::{
    canonical_service_tournament_archive_fixture, canonical_tournament_year_fixture,
};

fn decoded_fixture()
-> hollow_grove::world::service_tournament_archive::DecodedServiceTournamentArchive {
    let bytes =
        encode_service_tournament_archive(&canonical_service_tournament_archive_fixture()).unwrap();
    decode_service_tournament_archive(&bytes).unwrap()
}

fn year(
    archive: &hollow_grove::world::service_tournament_archive::DecodedServiceTournamentArchive,
) -> &hollow_grove::world::service_tournament_archive::TournamentYearState {
    archive.years.values().next().unwrap()
}

fn semantic_event(sequence: u64, kind: SynthesisSemanticEventKind) -> SynthesisSemanticEvent {
    SynthesisSemanticEvent {
        id: SynthesisSemanticEventId::new(format!("semantic-test.event.{sequence}")).unwrap(),
        semantic_sequence: sequence,
        kind,
    }
}

fn aim_event(
    attempt_id: &SynthesisAttemptId,
    path: HouseSynthesisPath,
    before: Option<HouseOutwardManifestation>,
) -> SynthesisSemanticEvent {
    semantic_event(
        0,
        SynthesisSemanticEventKind::Aim {
            attempt_id: attempt_id.clone(),
            path,
            operation: path.inward_operation(),
            recipe_id: "recipe.semantic-test.v1".into(),
            compiler_id: "compiler.semantic-test.v1".into(),
            script_id: "script.semantic-test.v1".into(),
            aim_id: "aim.semantic-test.v1".into(),
            manifestation_before: before,
            evidence_references: ["evidence.semantic-test.aim".into()].into_iter().collect(),
        },
    )
}

fn fire_event(attempt_id: &SynthesisAttemptId) -> SynthesisSemanticEvent {
    semantic_event(
        1,
        SynthesisSemanticEventKind::Fire {
            attempt_id: attempt_id.clone(),
            fire_id: "fire.semantic-test.v1".into(),
        },
    )
}

fn contact_event(
    attempt_id: &SynthesisAttemptId,
    outcome: SemanticContactOutcome,
) -> SynthesisSemanticEvent {
    semantic_event(
        2,
        SynthesisSemanticEventKind::Contact {
            attempt_id: attempt_id.clone(),
            outcome,
            accepted_by: "authority.semantic-test.v1".into(),
            evidence_references: ["evidence.semantic-test.contact".into()]
                .into_iter()
                .collect(),
        },
    )
}

#[test]
fn canonical_year_has_exactly_four_unsplit_fixed_representatives() {
    let fixture = canonical_tournament_year_fixture();
    assert_eq!(fixture.id.as_str(), CANONICAL_TOURNAMENT_YEAR_ID);
    assert_eq!(fixture.representatives.len(), 4);
    assert_eq!(
        fixture
            .representatives
            .iter()
            .map(|representative| representative.house)
            .collect::<BTreeSet<_>>()
            .len(),
        4
    );
    assert_eq!(
        fixture
            .representatives
            .iter()
            .map(|representative| representative.service_identity)
            .collect::<BTreeSet<_>>(),
        PairedServiceIdentity::ALL.into_iter().collect()
    );
    assert!(fixture.representatives.iter().all(|representative| {
        representative.service_identity.house() == representative.house
            && representative
                .service_identity
                .paired_reference()
                .contains(" & ")
    }));
}

#[test]
fn war_and_every_archived_scenario_are_nonlethal_with_fixed_colors() {
    let archive = decoded_fixture();
    let state = year(&archive);
    assert!(state.war_result.nonlethal);
    assert!(
        state
            .tournament_runtime
            .scenarios()
            .values()
            .all(|scenario| scenario.nonlethal)
    );
    assert_eq!(
        HouseColorFamily::for_house(House::Stonebend),
        HouseColorFamily::Blue
    );
    assert_eq!(
        HouseColorFamily::for_house(House::Sandmanor),
        HouseColorFamily::Red
    );
    assert_eq!(
        HouseColorFamily::for_house(House::Glaushouse),
        HouseColorFamily::Green
    );
    assert_eq!(
        HouseColorFamily::for_house(House::Flynt),
        HouseColorFamily::Black
    );
}

#[test]
fn paint_layer_order_and_complete_service_mark_provenance_survive_archive() {
    let archive = decoded_fixture();
    let state = year(&archive);
    let service_mark = state
        .tournament_runtime
        .service_marks()
        .values()
        .next()
        .unwrap();
    let layers = service_mark
        .ordered_paint_layers
        .iter()
        .map(|id| state.tournament_runtime.marks().get(id).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        layers.iter().map(|mark| mark.house).collect::<Vec<_>>(),
        vec![
            House::Flynt,
            House::Stonebend,
            House::Sandmanor,
            House::Glaushouse
        ]
    );
    assert_eq!(service_mark.provenance.source_marks.len(), 4);
    assert_eq!(service_mark.provenance.source_action_events.len(), 4);
    assert_eq!(service_mark.participants.len(), 4);
    assert!(!service_mark.provenance.evidence.is_empty());
    assert!(!service_mark.constitutional_significance.is_empty());
    assert!(!service_mark.account.is_empty());
}

#[test]
fn real_emergency_is_distinguished_from_simulation_and_suspends_play() {
    let archive = decoded_fixture();
    let emergency = year(&archive).real_emergencies.values().next().unwrap();
    assert!(emergency.initially_interpreted_as_simulation);
    assert!(emergency.determined_real);
    assert!(emergency.simulation_suspended);
    assert_ne!(
        emergency.simulation_scenario_id,
        emergency.response_scenario_id
    );
}

#[test]
fn constitutional_scoring_is_not_eliminations_and_can_invalidate_tactical_victory() {
    let archive = decoded_fixture();
    let state = year(&archive);
    let result = final_result(state).unwrap();
    assert!(
        result
            .scorecards
            .values()
            .all(|scorecard| scorecard.scores.len() == ScoringCategory::ALL.len())
    );
    assert_eq!(state.war_result.tactical_leader, House::Flynt);
    assert_eq!(state.war_result.constitutional_champion, House::Glaushouse);
    assert!(state.constitutional_violations.values().any(|violation| {
        violation.house == House::Flynt
            && violation.violation == ConstitutionalPenalty::ExcessiveForce
            && violation.points_deduction > 0
            && violation.invalidates_tactical_victory
    }));
    assert!(
        !ScoringCategory::ALL
            .into_iter()
            .any(|category| format!("{category:?}").contains("Elimination"))
    );
}

#[test]
fn no_result_prize_or_custody_record_transfers_sovereignty_or_ownership() {
    let archive = decoded_fixture();
    let state = year(&archive);
    assert!(!state.war_result.transfers_permanent_sovereignty);
    assert!(!final_result(state).unwrap().transfers_permanent_sovereignty);
    assert!(state.prize_awards.values().all(|award| {
        !award.grants_sovereignty
            && !award.transfers_ownership
            && award
                .flagship_custody
                .as_ref()
                .is_none_or(|custody| !custody.grants_sovereignty && !custody.custody_is_ownership)
    }));
}

#[test]
fn edge_and_glass_keep_stable_coequal_identities_through_refinement() {
    let archive = decoded_fixture();
    let state = year(&archive);
    let edge = flagship_artifact(state, FlagshipArtifactKind::EdgeOfTomorrow).unwrap();
    let glass = flagship_artifact(state, FlagshipArtifactKind::GlassOfAThousandHues).unwrap();
    assert_eq!(edge.id.as_str(), EDGE_OF_TOMORROW_ID);
    assert_eq!(glass.id.as_str(), GLASS_OF_A_THOUSAND_HUES_ID);
    assert!(edge.completed_synthesis && glass.completed_synthesis);
    assert_eq!(edge.refinements.len(), 1);
    assert_eq!(glass.refinements.len(), 1);
    assert_ne!(
        edge.custody.as_ref().unwrap().custodian_id,
        glass.custody.as_ref().unwrap().custodian_id
    );
}

#[test]
fn prize_degrees_cannot_claim_more_realized_synthesis_than_their_subject() {
    assert!(PrizeDegree::FlagshipArtifact.is_completed_synthesis());
    assert!(PrizeDegree::CompletedSynthesis.is_completed_synthesis());
    assert!(!PrizeDegree::SynthesisRecipePattern.is_completed_synthesis());
    assert!(!PrizeDegree::SynthesisCore.is_completed_synthesis());
    assert_eq!(
        PrizeDegree::SynthesisRecipePattern.required_subject_kind(),
        PrizeSubjectKind::SynthesisRecipePattern
    );
    assert_eq!(
        PrizeDegree::SynthesisCore.required_subject_kind(),
        PrizeSubjectKind::SynthesisCore
    );

    let mut invalid = canonical_service_tournament_archive_fixture();
    let recipe = invalid.years[0]
        .prize_awards
        .iter_mut()
        .find(|award| award.degree == PrizeDegree::SynthesisRecipePattern)
        .unwrap();
    recipe.subject_kind = PrizeSubjectKind::CompletedSynthesis;
    assert!(matches!(
        encode_service_tournament_archive(&invalid),
        Err(ServiceTournamentArchiveError::InvalidPrizeAward(_))
    ));
}

#[test]
fn house_paths_separate_inward_operation_from_outward_manifestation() {
    assert_eq!(
        HouseSynthesisPath::Flynt.inward_operation(),
        HouseInwardOperation::Flynt(FlyntOperation::Resynce)
    );
    assert_eq!(
        HouseSynthesisPath::Flynt.outward_manifestation(),
        HouseOutwardManifestation::Flynt(FlyntManifestation::Recog)
    );
    assert_ne!(
        format!("{:?}", HouseSynthesisPath::Flynt.inward_operation()),
        format!("{:?}", HouseSynthesisPath::Flynt.outward_manifestation())
    );
}

#[test]
fn prefog_and_prefig_keep_exact_lineage_manifestation_and_jurisdiction_paths() {
    let min = HouseSynthesisPath::SandmanorMinorian;
    assert_eq!(
        min.inward_operation(),
        HouseInwardOperation::Sandmanor(SandmanorOperation::Prefog)
    );
    assert_eq!(
        min.outward_manifestation(),
        HouseOutwardManifestation::Sandmanor(SandmanorManifestation::Minotaur)
    );
    assert_eq!(
        min.sandmanor_source_lineage(),
        Some(SandmanorSourceLineage::Gnome)
    );
    assert_eq!(
        min.sandmanor_jurisdiction(),
        Some(SandmanorJurisdiction::AuraFields)
    );

    let minoan = HouseSynthesisPath::SandmanorMinoan;
    assert_eq!(
        minoan.inward_operation(),
        HouseInwardOperation::Sandmanor(SandmanorOperation::Prefig)
    );
    assert_eq!(
        minoan.outward_manifestation(),
        HouseOutwardManifestation::Sandmanor(SandmanorManifestation::Centaur)
    );
    assert_eq!(
        minoan.sandmanor_source_lineage(),
        Some(SandmanorSourceLineage::Elf)
    );
    assert_eq!(
        minoan.sandmanor_jurisdiction(),
        Some(SandmanorJurisdiction::AuraBeachAndCurrentSea)
    );
}

#[test]
fn stonebend_manifestation_precog_and_glaushouse_operation_precog_are_type_distinct() {
    assert_ne!(
        TypeId::of::<StonebendManifestation>(),
        TypeId::of::<GlaushouseOperation>()
    );
    assert_eq!(
        HouseSynthesisPath::Stonebend.outward_manifestation(),
        HouseOutwardManifestation::Stonebend(StonebendManifestation::Precog)
    );
    assert_eq!(
        HouseSynthesisPath::Glaushouse.inward_operation(),
        HouseInwardOperation::Glaushouse(GlaushouseOperation::Precog)
    );
}

#[test]
fn sympiote_is_the_only_canonical_glaushouse_spelling() {
    const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
    const SEMANTICS: &str = include_str!("../src/constitutional/house_synthesis_semantics.rs");
    assert!(CORE.contains("expression: Sympiote"));
    assert!(SEMANTICS.contains("GlaushouseManifestation::Sympiote"));
    assert_eq!(
        HouseSynthesisPath::Glaushouse.outward_manifestation(),
        HouseOutwardManifestation::Glaushouse(GlaushouseManifestation::Sympiote)
    );
}

#[test]
fn miss_leaves_manifestation_unchanged_and_cannot_claim_point_squared() {
    let attempt = SynthesisAttemptId::new("attempt.semantic-test.miss").unwrap();
    let before = HouseOutwardManifestation::Flynt(FlyntManifestation::Recog);
    let events = vec![
        aim_event(&attempt, HouseSynthesisPath::Flynt, Some(before)),
        fire_event(&attempt),
        contact_event(&attempt, SemanticContactOutcome::Miss),
    ];
    let mut runtime = HouseSynthesisSemanticRuntime::replay(&events).unwrap();
    let state = runtime.attempts().get(&attempt).unwrap();
    assert_eq!(state.canonical_manifestation, Some(before));

    let point = semantic_event(
        3,
        SynthesisSemanticEventKind::PointSquared {
            attempt_id: attempt.clone(),
            record: PointSquaredRelationshipRecord {
                record_id: "point-squared.semantic-test.miss".into(),
                relationship_id: "relationship.semantic-test.miss".into(),
                authority_id: "authority.semantic-test.v1".into(),
                location_id: "location.semantic-test.v1".into(),
                recipe_id: "recipe.semantic-test.v1".into(),
                provenance_id: "provenance.semantic-test.miss".into(),
                result_id: "result.semantic-test.miss".into(),
                evidence_references: ["evidence.semantic-test.miss".into()].into_iter().collect(),
                manifestation: before,
            },
        },
    );
    assert!(matches!(
        runtime.apply(point),
        Err(HouseSynthesisSemanticError::InvalidPointSquared(_))
    ));
}

#[test]
fn kiss_applies_exactly_one_lawful_manifestation_and_point_squared_provenance() {
    let attempt = SynthesisAttemptId::new("attempt.semantic-test.kiss").unwrap();
    let mut runtime = HouseSynthesisSemanticRuntime::replay(&[
        aim_event(&attempt, HouseSynthesisPath::Flynt, None),
        fire_event(&attempt),
        contact_event(&attempt, SemanticContactOutcome::Kiss),
    ])
    .unwrap();
    assert_eq!(
        runtime
            .attempts()
            .get(&attempt)
            .unwrap()
            .canonical_manifestation,
        Some(HouseOutwardManifestation::Flynt(FlyntManifestation::Recog))
    );
    let mut duplicate = contact_event(&attempt, SemanticContactOutcome::Kiss);
    duplicate.id = SynthesisSemanticEventId::new("semantic-test.event.duplicate-kiss").unwrap();
    duplicate.semantic_sequence = 3;
    assert!(matches!(
        runtime.apply(duplicate),
        Err(HouseSynthesisSemanticError::InvalidContact(_))
    ));
    runtime
        .apply(semantic_event(
            3,
            SynthesisSemanticEventKind::PointSquared {
                attempt_id: attempt.clone(),
                record: PointSquaredRelationshipRecord {
                    record_id: "point-squared.semantic-test.kiss".into(),
                    relationship_id: "relationship.semantic-test.kiss".into(),
                    authority_id: "authority.semantic-test.v1".into(),
                    location_id: "location.semantic-test.v1".into(),
                    recipe_id: "recipe.semantic-test.v1".into(),
                    provenance_id: "provenance.semantic-test.kiss".into(),
                    result_id: "result.semantic-test.kiss".into(),
                    evidence_references: ["evidence.semantic-test.kiss".into()]
                        .into_iter()
                        .collect(),
                    manifestation: HouseOutwardManifestation::Flynt(FlyntManifestation::Recog),
                },
            },
        ))
        .unwrap();
    assert!(
        runtime
            .attempts()
            .get(&attempt)
            .unwrap()
            .point_squared
            .is_some()
    );
}

#[test]
fn replay_and_opposite_insertion_order_produce_identical_state_and_bytes() {
    let payload = canonical_service_tournament_archive_fixture();
    let bytes = encode_service_tournament_archive(&payload).unwrap();
    let first = decode_service_tournament_archive(&bytes).unwrap();
    let second = decode_service_tournament_archive(&bytes).unwrap();
    assert_eq!(first.years, second.years);

    let mut reversed = payload;
    let year = &mut reversed.years[0];
    year.representatives.reverse();
    year.events.reverse();
    year.scenario_phases.reverse();
    year.alliances.reverse();
    year.real_emergencies.reverse();
    year.scoring_events.reverse();
    year.constitutional_violations.reverse();
    year.artifact_refinements.reverse();
    year.prize_awards.reverse();
    year.synthesis_semantic_events.reverse();
    assert_eq!(encode_service_tournament_archive(&reversed).unwrap(), bytes);
}

#[test]
fn archive_has_explicit_version_checksum_tamper_detection_and_idempotent_migration() {
    let payload = canonical_service_tournament_archive_fixture();
    let bytes = encode_service_tournament_archive(&payload).unwrap();
    let decoded = decode_service_tournament_archive(&bytes).unwrap();
    assert_eq!(decoded.archive_version, SERVICE_TOURNAMENT_ARCHIVE_VERSION);
    assert_eq!(decoded.checksum.len(), 16);
    assert_eq!(migrate_service_tournament_archive(&bytes).unwrap(), bytes);

    let legacy = encode_legacy_service_tournament_archive_v0(&payload).unwrap();
    let migrated = migrate_service_tournament_archive(&legacy).unwrap();
    assert_eq!(
        decode_service_tournament_archive(&migrated)
            .unwrap()
            .archive_version,
        SERVICE_TOURNAMENT_ARCHIVE_VERSION
    );

    let text = String::from_utf8(bytes).unwrap();
    let tampered = text.replace("\"calendar_year\":2047", "\"calendar_year\":2048");
    assert!(matches!(
        decode_service_tournament_archive(tampered.as_bytes()),
        Err(ServiceTournamentArchiveError::ChecksumMismatch)
    ));
}

#[test]
fn malformed_or_contradictory_roster_is_rejected_before_archive_creation() {
    let mut payload = canonical_service_tournament_archive_fixture();
    payload.years[0].representatives[0].service_identity =
        PairedServiceIdentity::StonebendDeaAirForce;
    assert!(matches!(
        encode_service_tournament_archive(&payload),
        Err(ServiceTournamentArchiveError::InvalidRepresentatives(_))
    ));
}
