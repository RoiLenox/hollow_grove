//! Golden Function Junction, House Season, synchronization, and Permanence
//! fixture layered over the frozen Central Junction annual archive.

use crate::ScaleKey;
use crate::constitutional::{
    AuthoritativeTimestamp, CalendarEvidenceId, CanonicalYearId, GroveCycleAuthorityId,
    GroveCycleEvidenceId, GroveCycleId, GroveCycleProvenanceId, GroveCycleRecord,
    GroveCycleResolution, GroveCycleSubjectId, GrovePhase, GroveStateId, SeasonalAnchor,
};
use crate::hollow_grove_contract::House;

use super::central_junction_seasonal_functions::{GreatFunctionId, GreatFunctionKind};
use super::function_junction::{
    CENTRAL_JUNCTION_PLACE_ID, CheckpointId, CheckpointStatus, FunctionJunctionId,
    FunctionJunctionRecord, HouseSeasonId, HouseSeasonRecord, JunctionAuthorityId, PracticalJokeId,
    PracticalJokeTransition, SynchronizationPhase, WorldCheckpointRecord, WorldLayer,
};
use super::function_junction_archive::{
    FunctionJunctionAnnualRecord, FunctionJunctionArchivePayload,
};
use super::permanence::{
    AttestationStatus, PermanenceAttestation, PermanenceAttestationId, PermanenceAuthorityId,
    PermanenceLaw, PermanencePetition, PermanencePetitionId, PermanenceSeal, PermanenceSealId,
    PermanenceTombstone, PermanenceTombstoneId, PermanentChangeId, PermanentChangeKind,
    PermanentChangeRecord, PermanentSubjectId, PermanentVersionId, StonebendSealOffice,
    YieldProtectionId,
};
use super::seasonal_functions_archive::CANONICAL_ANNUAL_CYCLE_ID;
use super::seasonal_functions_fixture::canonical_seasonal_archive_fixture;

fn year_id() -> CanonicalYearId {
    CanonicalYearId::new(CANONICAL_ANNUAL_CYCLE_ID).expect("canonical year ID")
}

fn timestamp(value: &str) -> AuthoritativeTimestamp {
    AuthoritativeTimestamp::new(value).expect("canonical timestamp")
}

fn evidence(value: &str) -> CalendarEvidenceId {
    CalendarEvidenceId::new(value).expect("canonical evidence ID")
}

fn function_id(kind: GreatFunctionKind) -> GreatFunctionId {
    GreatFunctionId::new(kind.stable_id()).expect("canonical Great Function ID")
}

fn junction_id(anchor: SeasonalAnchor) -> FunctionJunctionId {
    FunctionJunctionId::new(match anchor {
        SeasonalAnchor::WinterSolstice => "function-junction.canonical-year.winter-solstice",
        SeasonalAnchor::SpringEquinox => "function-junction.canonical-year.spring-equinox",
        SeasonalAnchor::SummerSolstice => "function-junction.canonical-year.summer-solstice",
        SeasonalAnchor::AutumnEquinox => "function-junction.canonical-year.autumn-equinox",
    })
    .expect("canonical Function Junction ID")
}

fn season_id(house: House) -> HouseSeasonId {
    HouseSeasonId::new(match house {
        House::Glaushouse => "season.canonical-year.glaushouse",
        House::Stonebend => "season.canonical-year.stonebend",
        House::Sandmanor => "season.canonical-year.sandmanor",
        House::Flynt => "season.canonical-year.flynt",
    })
    .expect("canonical House Season ID")
}

fn prior_season_id(house: House) -> HouseSeasonId {
    HouseSeasonId::new(match house {
        House::Flynt => "season.previous-year.flynt",
        House::Glaushouse => "season.canonical-year.glaushouse",
        House::Stonebend => "season.canonical-year.stonebend",
        House::Sandmanor => "season.canonical-year.sandmanor",
    })
    .expect("canonical outgoing House Season ID")
}

fn anchor_time(anchor: SeasonalAnchor) -> &'static str {
    match anchor {
        SeasonalAnchor::WinterSolstice => "2046-12-21T15:58:00Z",
        SeasonalAnchor::SpringEquinox => "2047-03-20T13:52:00Z",
        SeasonalAnchor::SummerSolstice => "2047-06-21T07:31:00Z",
        SeasonalAnchor::AutumnEquinox => "2047-09-23T00:08:00Z",
    }
}

fn checkpoint(anchor: SeasonalAnchor, layer: WorldLayer) -> WorldCheckpointRecord {
    let anchor_slug = match anchor {
        SeasonalAnchor::WinterSolstice => "winter-solstice",
        SeasonalAnchor::SpringEquinox => "spring-equinox",
        SeasonalAnchor::SummerSolstice => "summer-solstice",
        SeasonalAnchor::AutumnEquinox => "autumn-equinox",
    };
    let layer_slug = match layer {
        WorldLayer::Physical => "physical",
        WorldLayer::Digital => "digital",
    };
    let id = format!("checkpoint.canonical-year.{anchor_slug}.{layer_slug}");
    WorldCheckpointRecord {
        checkpoint_id: CheckpointId::new(&id).expect("canonical checkpoint ID"),
        canonical_year_id: year_id(),
        layer,
        status: CheckpointStatus::Completed,
        occurred_at: timestamp(anchor_time(anchor)),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
    }
}

#[must_use]
pub fn canonical_function_junctions_fixture() -> Vec<FunctionJunctionRecord> {
    SeasonalAnchor::ALL
        .into_iter()
        .map(|anchor| {
            let (outgoing_house, incoming_house, function_kind) =
                super::function_junction::seasonal_handoff(anchor);
            let id = junction_id(anchor);
            FunctionJunctionRecord {
                junction_id: id.clone(),
                canonical_year_id: year_id(),
                anchor,
                great_function_id: function_id(function_kind),
                outgoing_house,
                incoming_house,
                outgoing_season_id: prior_season_id(outgoing_house),
                incoming_season_id: season_id(incoming_house),
                physical_checkpoint: checkpoint(anchor, WorldLayer::Physical),
                digital_checkpoint: checkpoint(anchor, WorldLayer::Digital),
                synchronization_phase: SynchronizationPhase::for_anchor(anchor),
                grove_phase: super::function_junction::grove_phase_for_anchor(anchor),
                authority_ids: [
                    JunctionAuthorityId::new("authority.hollow-grove.compromise")
                        .expect("Compromise authority ID"),
                    JunctionAuthorityId::new(format!(
                        "authority.{}.seasonal-handoff",
                        incoming_house.as_str().to_ascii_lowercase()
                    ))
                    .expect("incoming House authority ID"),
                ]
                .into_iter()
                .collect(),
                evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
                outgoing_season_closed: true,
                incoming_season_opened: true,
                completed: true,
                physical_place_id: CENTRAL_JUNCTION_PLACE_ID.into(),
                is_geographic_location: false,
                is_great_function: false,
                transfers_sovereignty: false,
            }
        })
        .collect()
}

#[must_use]
pub fn canonical_house_seasons_fixture() -> Vec<HouseSeasonRecord> {
    vec![
        HouseSeasonRecord {
            season_id: season_id(House::Glaushouse),
            canonical_year_id: year_id(),
            house: House::Glaushouse,
            opened_by: junction_id(SeasonalAnchor::WinterSolstice),
            closed_by: junction_id(SeasonalAnchor::SpringEquinox),
            opens_at: timestamp(anchor_time(SeasonalAnchor::WinterSolstice)),
            closes_at: timestamp(anchor_time(SeasonalAnchor::SpringEquinox)),
            evidence_ids: [evidence("evidence.season.canonical-year.glaushouse")]
                .into_iter()
                .collect(),
        },
        HouseSeasonRecord {
            season_id: season_id(House::Stonebend),
            canonical_year_id: year_id(),
            house: House::Stonebend,
            opened_by: junction_id(SeasonalAnchor::SpringEquinox),
            closed_by: junction_id(SeasonalAnchor::SummerSolstice),
            opens_at: timestamp(anchor_time(SeasonalAnchor::SpringEquinox)),
            closes_at: timestamp(anchor_time(SeasonalAnchor::SummerSolstice)),
            evidence_ids: [evidence("evidence.season.canonical-year.stonebend")]
                .into_iter()
                .collect(),
        },
        HouseSeasonRecord {
            season_id: season_id(House::Sandmanor),
            canonical_year_id: year_id(),
            house: House::Sandmanor,
            opened_by: junction_id(SeasonalAnchor::SummerSolstice),
            closed_by: junction_id(SeasonalAnchor::AutumnEquinox),
            opens_at: timestamp(anchor_time(SeasonalAnchor::SummerSolstice)),
            closes_at: timestamp(anchor_time(SeasonalAnchor::AutumnEquinox)),
            evidence_ids: [evidence("evidence.season.canonical-year.sandmanor")]
                .into_iter()
                .collect(),
        },
        HouseSeasonRecord {
            season_id: season_id(House::Flynt),
            canonical_year_id: year_id(),
            house: House::Flynt,
            opened_by: junction_id(SeasonalAnchor::AutumnEquinox),
            closed_by: FunctionJunctionId::new("function-junction.next-year.winter-solstice")
                .expect("next Winter Function Junction ID"),
            opens_at: timestamp(anchor_time(SeasonalAnchor::AutumnEquinox)),
            closes_at: timestamp("2047-12-21T21:43:00Z"),
            evidence_ids: [evidence("evidence.season.canonical-year.flynt")]
                .into_iter()
                .collect(),
        },
    ]
}

fn joke(
    id: &str,
    outgoing: GreatFunctionKind,
    incoming: GreatFunctionKind,
    question: &str,
    answer: Option<&str>,
    at: &str,
) -> PracticalJokeTransition {
    PracticalJokeTransition {
        joke_id: PracticalJokeId::new(id).expect("canonical Practical Joke ID"),
        canonical_year_id: year_id(),
        outgoing_function_id: function_id(outgoing),
        incoming_function_id: function_id(incoming),
        question: question.into(),
        answer: answer.map(str::to_owned),
        witnessed_at: timestamp(at),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
        replaces_astronomical_anchor: false,
        replaces_function_junction: false,
    }
}

#[must_use]
pub fn canonical_practical_jokes_fixture() -> Vec<PracticalJokeTransition> {
    vec![
        joke(
            "practical-joke.canonical-year.way-back",
            GreatFunctionKind::WayBack,
            GreatFunctionKind::Initiation,
            "Welcome home where?",
            None,
            "2046-12-24T17:00:00Z",
        ),
        joke(
            "practical-joke.canonical-year.initiation",
            GreatFunctionKind::Initiation,
            GreatFunctionKind::Gathering,
            "Are we eating or what?",
            None,
            "2047-03-23T17:00:00Z",
        ),
        joke(
            "practical-joke.canonical-year.gathering",
            GreatFunctionKind::Gathering,
            GreatFunctionKind::FestivalOfMirrors,
            "So…who won?",
            Some("Whoever cleans this up."),
            "2047-06-28T17:00:00Z",
        ),
        joke(
            "practical-joke.canonical-year.festival-of-mirrors",
            GreatFunctionKind::FestivalOfMirrors,
            GreatFunctionKind::WayBack,
            "Congratulations. You look terrible.",
            None,
            "2047-09-27T17:00:00Z",
        ),
    ]
}

fn grove_cycle_id(value: &str) -> GroveCycleId {
    GroveCycleId::new(value).expect("canonical Grove cycle ID")
}

fn grove_state(value: &str) -> GroveStateId {
    GroveStateId::new(value).expect("canonical Grove state ID")
}

fn grove_cycle(
    cycle_id: &str,
    prior_state: &str,
    attempted_state: &str,
    confirmed_state: &str,
    resolution: GroveCycleResolution,
    opened_at: &str,
    completed_at: &str,
) -> GroveCycleRecord {
    GroveCycleRecord {
        cycle_id: grove_cycle_id(cycle_id),
        subject_id: GroveCycleSubjectId::new("subject.grove-cycle.bridge-control")
            .expect("canonical Grove cycle subject ID"),
        scale: ScaleKey::new("scale.object").expect("canonical open ScaleKey"),
        current_phase: GrovePhase::TheFestival,
        phase_history: GrovePhase::ALL.to_vec(),
        prior_state_id: grove_state(prior_state),
        attempted_state_id: Some(grove_state(attempted_state)),
        confirmed_state_id: Some(grove_state(confirmed_state)),
        next_way_back_state_id: Some(grove_state(confirmed_state)),
        resolution,
        evidence_ids: [GroveCycleEvidenceId::new(format!("evidence.{cycle_id}"))
            .expect("canonical Grove cycle evidence ID")]
        .into_iter()
        .collect(),
        authority_ids: [
            GroveCycleAuthorityId::new("authority.hollow-grove.compromise")
                .expect("canonical Grove cycle authority ID"),
        ]
        .into_iter()
        .collect(),
        provenance_ids: [
            GroveCycleProvenanceId::new(format!("provenance.{cycle_id}"))
                .expect("canonical Grove cycle provenance ID"),
        ]
        .into_iter()
        .collect(),
        opened_at: timestamp(opened_at),
        completed_at: Some(timestamp(completed_at)),
        rendering_may_advance_phase: false,
    }
}

#[must_use]
pub fn canonical_grove_cycles_fixture() -> Vec<GroveCycleRecord> {
    vec![
        grove_cycle(
            "grove-cycle.bridge-control.rejected",
            "state.bridge-control.v1",
            "state.bridge-control.invalid-attempt",
            "state.bridge-control.v1",
            GroveCycleResolution::Rejected,
            "2047-06-21T08:00:00Z",
            "2047-06-21T09:00:00Z",
        ),
        grove_cycle(
            "grove-cycle.bridge-control.accepted",
            "state.bridge-control.v1",
            "state.bridge-control.v2",
            "state.bridge-control.v2",
            GroveCycleResolution::Accepted,
            "2047-06-21T10:00:00Z",
            "2047-06-21T11:00:00Z",
        ),
    ]
}

fn permanent_subject() -> PermanentSubjectId {
    PermanentSubjectId::new("subject.central-junction.four-house-bridge")
        .expect("canonical permanent subject ID")
}

fn permanence_authority(value: &str) -> PermanenceAuthorityId {
    PermanenceAuthorityId::new(value).expect("canonical permanence authority ID")
}

fn attestation_id(law: PermanenceLaw) -> PermanenceAttestationId {
    PermanenceAttestationId::new(match law {
        PermanenceLaw::Identity => "attestation.permanence.bridge.identity",
        PermanenceLaw::Pattern => "attestation.permanence.bridge.pattern",
        PermanenceLaw::Integrity => "attestation.permanence.bridge.integrity",
        PermanenceLaw::Recognition => "attestation.permanence.bridge.recognition",
    })
    .expect("canonical permanence attestation ID")
}

#[must_use]
pub fn canonical_permanence_attestations_fixture() -> Vec<PermanenceAttestation> {
    PermanenceLaw::ALL
        .into_iter()
        .map(|law| {
            let id = attestation_id(law);
            PermanenceAttestation {
                attestation_id: id.clone(),
                canonical_year_id: year_id(),
                subject_id: permanent_subject(),
                law,
                authority_house: law.authority_house(),
                authority_id: permanence_authority(match law {
                    PermanenceLaw::Identity => "authority.stonebend.freemason",
                    PermanenceLaw::Pattern => "authority.sandmanor.pattern-review",
                    PermanenceLaw::Integrity => "authority.glaushouse.integrity-clearance",
                    PermanenceLaw::Recognition => "authority.flynt.recog-commission",
                }),
                status: AttestationStatus::Accepted,
                evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
                attested_at: timestamp(match law {
                    PermanenceLaw::Identity => "2047-09-23T08:00:00Z",
                    PermanenceLaw::Pattern => "2047-09-23T09:00:00Z",
                    PermanenceLaw::Integrity => "2047-09-23T10:00:00Z",
                    PermanenceLaw::Recognition => "2047-09-23T11:00:00Z",
                }),
                provenance_ids: [evidence(&format!("provenance.{id}"))]
                    .into_iter()
                    .collect(),
            }
        })
        .collect()
}

fn petition_id() -> PermanencePetitionId {
    PermanencePetitionId::new("petition.permanence.bridge")
        .expect("canonical Permanence Petition ID")
}

#[must_use]
pub fn canonical_permanence_petition_fixture() -> PermanencePetition {
    PermanencePetition {
        petition_id: petition_id(),
        canonical_year_id: year_id(),
        subject_id: permanent_subject(),
        claim_authority_id: permanence_authority("authority.stonebend.freemason"),
        freemason_claim: true,
        attestation_ids: PermanenceLaw::ALL.into_iter().map(attestation_id).collect(),
        evidence_ids: [evidence("evidence.petition.permanence.bridge")]
            .into_iter()
            .collect(),
        opened_at: timestamp("2047-09-23T07:00:00Z"),
    }
}

fn version(value: &str) -> PermanentVersionId {
    PermanentVersionId::new(value).expect("canonical permanent version ID")
}

#[must_use]
pub fn canonical_permanence_seal_fixture() -> PermanenceSeal {
    PermanenceSeal {
        seal_id: PermanenceSealId::new("seal.permanence.bridge.v1")
            .expect("canonical Permanence Seal ID"),
        petition_id: petition_id(),
        canonical_year_id: year_id(),
        subject_id: permanent_subject(),
        version_id: version("version.permanence.bridge.v1"),
        issuing_house: House::Stonebend,
        office: StonebendSealOffice::DiamondHypergiant,
        stonebend_authority_id: permanence_authority("authority.stonebend.diamond-hypergiant"),
        supporting_attestation_ids: PermanenceLaw::ALL.into_iter().map(attestation_id).collect(),
        evidence_ids: [evidence("evidence.seal.permanence.bridge.v1")]
            .into_iter()
            .collect(),
        yield_protection_id: YieldProtectionId::new("yield-protection.proliteriate.bridge")
            .expect("canonical Proliteriate Yield protection ID"),
        sealed_at: timestamp("2047-09-23T12:00:00Z"),
        immutable: false,
    }
}

fn change(
    id: &str,
    kind: PermanentChangeKind,
    prior: &str,
    result: &str,
    at: &str,
    tombstone_id: Option<&str>,
) -> PermanentChangeRecord {
    PermanentChangeRecord {
        change_id: PermanentChangeId::new(id).expect("canonical permanent change ID"),
        canonical_year_id: year_id(),
        subject_id: permanent_subject(),
        kind,
        prior_version_id: version(prior),
        result_version_id: version(result),
        authorizing_house: House::Stonebend,
        stonebend_authority_id: Some(permanence_authority(
            "authority.stonebend.diamond-hypergiant",
        )),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
        changed_at: timestamp(at),
        preserves_prior_version: true,
        tombstone_id: tombstone_id
            .map(|id| PermanenceTombstoneId::new(id).expect("canonical Permanence Tombstone ID")),
    }
}

#[must_use]
pub fn canonical_permanence_changes_fixture() -> Vec<PermanentChangeRecord> {
    vec![
        change(
            "change.permanence.bridge.amendment",
            PermanentChangeKind::Amendment,
            "version.permanence.bridge.v1",
            "version.permanence.bridge.v2-amended",
            "2047-09-24T10:00:00Z",
            None,
        ),
        change(
            "change.permanence.bridge.succession",
            PermanentChangeKind::Succession,
            "version.permanence.bridge.v2-amended",
            "version.permanence.bridge.v3-succeeded",
            "2047-09-25T10:00:00Z",
            None,
        ),
        change(
            "change.permanence.bridge.dissolution",
            PermanentChangeKind::Dissolution,
            "version.permanence.bridge.v3-succeeded",
            "version.permanence.bridge.v4-dissolved",
            "2047-09-26T10:00:00Z",
            Some("tombstone.permanence.bridge"),
        ),
    ]
}

#[must_use]
pub fn canonical_permanence_tombstone_fixture() -> PermanenceTombstone {
    PermanenceTombstone {
        tombstone_id: PermanenceTombstoneId::new("tombstone.permanence.bridge")
            .expect("canonical Permanence Tombstone ID"),
        canonical_year_id: year_id(),
        subject_id: permanent_subject(),
        final_version_id: version("version.permanence.bridge.v4-dissolved"),
        dissolution_change_id: PermanentChangeId::new("change.permanence.bridge.dissolution")
            .expect("canonical dissolution change ID"),
        evidence_ids: [evidence("evidence.tombstone.permanence.bridge")]
            .into_iter()
            .collect(),
        recorded_at: timestamp("2047-09-26T10:00:00Z"),
        silently_deletes_history: false,
    }
}

#[must_use]
pub fn canonical_function_junction_annual_record_fixture() -> FunctionJunctionAnnualRecord {
    FunctionJunctionAnnualRecord {
        canonical_year_id: year_id(),
        seasonal_archive: canonical_seasonal_archive_fixture(),
        junctions: canonical_function_junctions_fixture(),
        seasons: canonical_house_seasons_fixture(),
        practical_jokes: canonical_practical_jokes_fixture(),
        grove_cycles: canonical_grove_cycles_fixture(),
        permanence_attestations: canonical_permanence_attestations_fixture(),
        permanence_petitions: vec![canonical_permanence_petition_fixture()],
        permanence_seals: vec![canonical_permanence_seal_fixture()],
        permanence_changes: canonical_permanence_changes_fixture(),
        permanence_tombstones: vec![canonical_permanence_tombstone_fixture()],
    }
}

#[must_use]
pub fn canonical_function_junction_archive_fixture() -> FunctionJunctionArchivePayload {
    FunctionJunctionArchivePayload {
        annual_records: vec![canonical_function_junction_annual_record_fixture()],
    }
}
