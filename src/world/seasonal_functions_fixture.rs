//! Golden full-year Central Junction fixture.
//!
//! The annual record composes, rather than replaces, the frozen Service
//! Tournament canonical-year payload.

use std::collections::BTreeSet;

use crate::constitutional::{
    AnchorObservationId, AstronomicalAnchorObservation, AuthoritativeTimestamp,
    CalendarAuthorityId, CalendarEvidenceId, CanonicalYearBoundary, CanonicalYearId,
    CanonicalYearRecord, ObservedCivilDate, SeasonalAnchor,
};
use crate::hollow_grove_contract::House;

use super::central_junction_seasonal_functions::{
    FunctionActivity, FunctionPhase, FunctionPhaseId, FunctionPhaseRecord, GreatFunctionId,
    GreatFunctionKind, GreatFunctionRecord, IncarnationalPrinciple, SeasonalEventId,
    SeasonalRecognitionId, SeasonalVenueId,
};
use super::current_sea_passage::{
    BOARDWALK_ROUTE_ID, BoardwalkPassage, BoardwalkPassageId, BoardwalkRouteId,
    CURRENT_SEA_REGION_ID, CurrentSeaAuthorityId, CurrentSeaClearanceId, CurrentSeaEvent,
    CurrentSeaEventId, CurrentSeaForce, CurrentSeaForceDirection, CurrentSeaForceId,
    CurrentSeaProvenanceId, CurrentSeaRegionId, CurrentSeaState, CurrentSeaTravelerId,
    FlyntRecogEventId, FlyntRecogRecord, FlyntResynceEventId, FlyntResynceRecord,
    GlaushouseClearanceRecord, ManifestationConditionId,
};
use super::seasonal_functions_archive::{
    CANONICAL_ANNUAL_CYCLE_ID, CanonicalAnnualCycleRecord, SeasonalArchivePayload,
    SeasonalNestingRecord, SeasonalRecognitionRecord, SeasonalRecognitionSubject,
};
use super::service_tournament::{
    ArtifactId, ResultId, ServiceMarkId, TournamentYearId, canonical_service_tournament,
    canonical_war_of_a_thousand_hues,
};
use super::service_tournament_archive::{
    EDGE_OF_TOMORROW_ID, GLASS_OF_A_THOUSAND_HUES_ID, decode_service_tournament_archive,
    encode_service_tournament_archive,
};
use super::service_tournament_fixture::canonical_service_tournament_archive_fixture;
use super::way_back::{
    WayBackAuthorityId, WayBackClearanceId, WayBackDirection, WayBackExpression, WayBackPassage,
    WayBackPassageId, WayBackProvenanceId, WayBackRouteId, WayBackSupportId, WayBackSupportRecord,
    WayBackSupportRole, WayBackTravelerId, canonical_way_back_route,
};
use super::world_point_fixture::canonical_world_point_archive_fixture;

fn year_id() -> CanonicalYearId {
    CanonicalYearId::new(CANONICAL_ANNUAL_CYCLE_ID).expect("canonical annual-cycle ID")
}

fn next_year_id() -> CanonicalYearId {
    CanonicalYearId::new("central-junction.canonical-year.2048.v1").expect("next canonical year ID")
}

fn timestamp(value: &str) -> AuthoritativeTimestamp {
    AuthoritativeTimestamp::new(value).expect("canonical fixture timestamp")
}

fn civil_date(value: &str) -> ObservedCivilDate {
    ObservedCivilDate::new(value).expect("canonical observed civil date")
}

fn evidence(value: &str) -> CalendarEvidenceId {
    CalendarEvidenceId::new(value).expect("canonical calendar evidence ID")
}

fn authority() -> CalendarAuthorityId {
    CalendarAuthorityId::new("authority.central-junction.canonical-calendar")
        .expect("canonical calendar authority ID")
}

fn observation(
    anchor: SeasonalAnchor,
    id: &str,
    instant: &str,
    date: &str,
) -> AstronomicalAnchorObservation {
    AstronomicalAnchorObservation {
        id: AnchorObservationId::new(id).expect("canonical observation ID"),
        canonical_year_id: year_id(),
        anchor,
        astronomical_instant: timestamp(instant),
        observed_civil_date: civil_date(date),
        supplied_by: authority(),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
    }
}

#[must_use]
pub fn canonical_calendar_year_fixture() -> CanonicalYearRecord {
    CanonicalYearRecord {
        id: year_id(),
        opens_at: timestamp("2046-12-21T15:58:00Z"),
        closes_at: timestamp("2047-12-21T21:43:00Z"),
        anchor_observations: vec![
            observation(
                SeasonalAnchor::WinterSolstice,
                "observation.canonical-year.winter-solstice",
                "2046-12-21T15:58:00Z",
                "2046-12-21",
            ),
            observation(
                SeasonalAnchor::SpringEquinox,
                "observation.canonical-year.spring-equinox",
                "2047-03-20T13:52:00Z",
                "2047-03-20",
            ),
            observation(
                SeasonalAnchor::SummerSolstice,
                "observation.canonical-year.summer-solstice",
                "2047-06-21T07:31:00Z",
                "2047-06-21",
            ),
            observation(
                SeasonalAnchor::AutumnEquinox,
                "observation.canonical-year.autumn-equinox",
                "2047-09-23T00:08:00Z",
                "2047-09-22",
            ),
        ],
        boundary: CanonicalYearBoundary {
            next_year_id: next_year_id(),
            next_winter_solstice: timestamp("2047-12-21T21:43:00Z"),
            observed_civil_date: civil_date("2047-12-21"),
            supplied_by: authority(),
            evidence_ids: [evidence(
                "evidence.canonical-year.next-winter-solstice-boundary",
            )]
            .into_iter()
            .collect(),
            closes_previous_and_opens_next: true,
        },
    }
}

fn function_id(kind: GreatFunctionKind) -> GreatFunctionId {
    GreatFunctionId::new(kind.stable_id()).expect("canonical Great Function ID")
}

fn phase_record(
    kind: GreatFunctionKind,
    phase: FunctionPhase,
    occurs_at: &str,
) -> FunctionPhaseRecord {
    let kind_slug = match kind {
        GreatFunctionKind::WayBack => "way-back",
        GreatFunctionKind::Initiation => "initiation",
        GreatFunctionKind::Gathering => "gathering",
        GreatFunctionKind::FestivalOfMirrors => "festival-of-mirrors",
    };
    let phase_slug = match phase {
        FunctionPhase::Preparation => "preparation",
        FunctionPhase::Gathering => "gathering",
        FunctionPhase::Eve => "eve",
        FunctionPhase::AstronomicalApex => "astronomical-apex",
        FunctionPhase::Celebration => "celebration",
        FunctionPhase::ReturnOrDeparture => "return-or-departure",
        FunctionPhase::Archive => "archive",
    };
    let id = format!("phase.canonical-year.{kind_slug}.{phase_slug}");
    FunctionPhaseRecord {
        id: FunctionPhaseId::new(&id).expect("canonical Function phase ID"),
        function_id: function_id(kind),
        phase,
        occurs_at: timestamp(occurs_at),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
    }
}

fn phases(kind: GreatFunctionKind) -> Vec<FunctionPhaseRecord> {
    let timestamps: [&str; 7] = match kind {
        GreatFunctionKind::WayBack => [
            "2046-12-18T00:00:00Z",
            "2046-12-19T12:00:00Z",
            "2046-12-20T18:00:00Z",
            "2046-12-21T15:58:00Z",
            "2046-12-22T12:00:00Z",
            "2046-12-23T12:00:00Z",
            "2046-12-24T18:00:00Z",
        ],
        GreatFunctionKind::Initiation => [
            "2047-03-17T00:00:00Z",
            "2047-03-18T12:00:00Z",
            "2047-03-19T18:00:00Z",
            "2047-03-20T13:52:00Z",
            "2047-03-21T12:00:00Z",
            "2047-03-22T12:00:00Z",
            "2047-03-23T18:00:00Z",
        ],
        GreatFunctionKind::Gathering => [
            "2047-06-14T00:00:00Z",
            "2047-06-17T12:00:00Z",
            "2047-06-20T18:00:00Z",
            "2047-06-21T07:31:00Z",
            "2047-06-23T12:00:00Z",
            "2047-06-27T12:00:00Z",
            "2047-06-28T18:00:00Z",
        ],
        GreatFunctionKind::FestivalOfMirrors => [
            "2047-09-19T00:00:00Z",
            "2047-09-20T12:00:00Z",
            "2047-09-22T00:00:00Z",
            "2047-09-23T00:08:00Z",
            "2047-09-24T12:00:00Z",
            "2047-09-26T12:00:00Z",
            "2047-09-27T18:00:00Z",
        ],
    };
    FunctionPhase::ALL
        .into_iter()
        .zip(timestamps)
        .map(|(phase, occurs_at)| phase_record(kind, phase, occurs_at))
        .collect()
}

fn interval(kind: GreatFunctionKind) -> (&'static str, &'static str, &'static str) {
    match kind {
        GreatFunctionKind::WayBack => (
            "2046-12-17T00:00:00Z",
            "2046-12-21T15:58:00Z",
            "2046-12-25T00:00:00Z",
        ),
        GreatFunctionKind::Initiation => (
            "2047-03-16T00:00:00Z",
            "2047-03-20T13:52:00Z",
            "2047-03-24T00:00:00Z",
        ),
        GreatFunctionKind::Gathering => (
            "2047-06-13T00:00:00Z",
            "2047-06-21T07:31:00Z",
            "2047-06-29T00:00:00Z",
        ),
        GreatFunctionKind::FestivalOfMirrors => (
            "2047-09-18T00:00:00Z",
            "2047-09-23T00:08:00Z",
            "2047-09-28T00:00:00Z",
        ),
    }
}

fn activities(kind: GreatFunctionKind) -> BTreeSet<FunctionActivity> {
    use FunctionActivity as Activity;
    match kind {
        GreatFunctionKind::WayBack => [
            Activity::PublicJoy,
            Activity::CommunalMeal,
            Activity::FamilyGathering,
            Activity::MemorialObservance,
            Activity::PublicHealing,
            Activity::Reconciliation,
            Activity::BondReview,
            Activity::LanternAndGlassLight,
            Activity::GentleRecoveryTrial,
        ]
        .into_iter()
        .collect(),
        GreatFunctionKind::Initiation => [
            Activity::Hospitality,
            Activity::GiftExchange,
            Activity::PublicNaming,
            Activity::LawfulInitiation,
            Activity::FoundationDedication,
            Activity::Chartering,
            Activity::ArchitectureExhibition,
            Activity::StructuralTrial,
        ]
        .into_iter()
        .collect(),
        GreatFunctionKind::Gathering => [
            Activity::PublicJoy,
            Activity::CommunalMeal,
            Activity::Music,
            Activity::Market,
            Activity::Performance,
            Activity::CulturalExchange,
            Activity::FamilyGathering,
            Activity::Hospitality,
            Activity::SharedCommunion,
            Activity::Ceremony,
            Activity::Diplomacy,
            Activity::HouseExhibition,
            Activity::PublicSynthesisPresentation,
            Activity::AthleticCivicEvent,
            Activity::ServiceTournament,
        ]
        .into_iter()
        .collect(),
        GreatFunctionKind::FestivalOfMirrors => [
            Activity::Parade,
            Activity::HouseExhibition,
            Activity::ConstitutionalReview,
            Activity::PublicRecognition,
            Activity::Commissioning,
            Activity::ServiceMarkPreservation,
            Activity::ArtifactCustodyRecognition,
            Activity::FailureReflection,
            Activity::ArchivePreparation,
            Activity::EngineeringDemonstration,
            Activity::RecipeRatification,
        ]
        .into_iter()
        .collect(),
    }
}

fn event_ids(kind: GreatFunctionKind) -> BTreeSet<SeasonalEventId> {
    let values: &[&str] = match kind {
        GreatFunctionKind::WayBack => &[
            "event.canonical-year.way-back.memorial",
            "event.canonical-year.way-back.healing",
        ],
        GreatFunctionKind::Initiation => &[
            "event.canonical-year.initiation.naming",
            "event.canonical-year.initiation.foundation",
        ],
        GreatFunctionKind::Gathering => &[
            "event.canonical-year.gathering.shared-table",
            "event.canonical-year.gathering.market",
            "event.canonical-year.gathering.performance",
            "event.canonical-year.gathering.ceremony",
            "event.canonical-year.gathering.service-tournament",
        ],
        GreatFunctionKind::FestivalOfMirrors => &[
            "event.canonical-year.festival.service-mark",
            "event.canonical-year.festival.edge",
            "event.canonical-year.festival.glass",
            "event.canonical-year.festival.review",
        ],
    };
    values
        .iter()
        .map(|value| SeasonalEventId::new(*value).expect("canonical seasonal event ID"))
        .collect()
}

fn great_function(kind: GreatFunctionKind) -> GreatFunctionRecord {
    let (opens_at, apex_at, closes_at) = interval(kind);
    GreatFunctionRecord {
        function_id: function_id(kind),
        canonical_year_id: year_id(),
        kind,
        anchor: kind.anchor(),
        presiding_house: kind.presiding_house(),
        canonical_name: kind.canonical_name().into(),
        aliases: kind
            .aliases()
            .iter()
            .map(|alias| (*alias).to_owned())
            .collect(),
        sacred_motion: kind.sacred_motion(),
        dimensions: kind.dimensions(),
        opens_at: timestamp(opens_at),
        apex_at: timestamp(apex_at),
        closes_at: timestamp(closes_at),
        phases: phases(kind),
        activities: activities(kind),
        event_ids: event_ids(kind),
        venue_ids: kind
            .venue_ids()
            .iter()
            .map(|id| SeasonalVenueId::new(*id).expect("canonical seasonal venue ID"))
            .collect(),
        evidence_ids: [evidence(&format!(
            "evidence.canonical-year.{}",
            kind.stable_id()
        ))]
        .into_iter()
        .collect(),
        incarnational_principle: (kind == GreatFunctionKind::Initiation)
            .then_some(IncarnationalPrinciple::EternalChristmas),
        presiding_house_owns_central_junction: false,
        transfers_permanent_sovereignty: false,
    }
}

#[must_use]
pub fn canonical_great_functions_fixture() -> Vec<GreatFunctionRecord> {
    GreatFunctionKind::ALL
        .into_iter()
        .map(great_function)
        .collect()
}

fn recognition(
    id: &str,
    event_id: &str,
    subject: SeasonalRecognitionSubject,
    account: &str,
) -> SeasonalRecognitionRecord {
    SeasonalRecognitionRecord {
        id: SeasonalRecognitionId::new(id).expect("canonical recognition ID"),
        canonical_year_id: year_id(),
        festival_function_id: function_id(GreatFunctionKind::FestivalOfMirrors),
        event_id: SeasonalEventId::new(event_id).expect("canonical recognition event ID"),
        subject,
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
        grants_permanent_sovereignty: false,
        account: account.into(),
    }
}

fn way_back_support(id: &str, house: House, role: WayBackSupportRole) -> WayBackSupportRecord {
    WayBackSupportRecord {
        support_id: WayBackSupportId::new(id).expect("canonical Way Back support ID"),
        house,
        role,
        authority_id: WayBackAuthorityId::new(&format!("authority.{id}"))
            .expect("canonical Way Back support authority ID"),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
        claims_route_ownership: false,
    }
}

#[must_use]
pub fn canonical_way_back_passages_fixture() -> Vec<WayBackPassage> {
    let route_id = WayBackRouteId::new(super::way_back::AURA_WAY_ROUTE_ID).expect("Aura Way ID");
    vec![
        WayBackPassage {
            passage_id: WayBackPassageId::new("passage.canonical-year.way-back.descent")
                .expect("canonical descent passage ID"),
            canonical_year_id: Some(year_id()),
            traveler_id: WayBackTravelerId::new("traveler.canonical-year.way-back.descent")
                .expect("canonical descent traveler ID"),
            direction: WayBackDirection::DescendingFromStonebend,
            route_id: route_id.clone(),
            origin_house: House::Stonebend,
            destination_house: House::Flynt,
            expression: WayBackExpression::AuraWay,
            function_id: Some(function_id(GreatFunctionKind::WayBack)),
            clearance_ids: [
                WayBackClearanceId::new("clearance.glaushouse.way-back.descent")
                    .expect("canonical descent clearance ID"),
            ]
            .into_iter()
            .collect(),
            evidence_ids: [evidence("evidence.canonical-year.way-back.descent")]
                .into_iter()
                .collect(),
            constitutional_authority_id: WayBackAuthorityId::new(
                "authority.compromise.way-back.descent",
            )
            .expect("canonical descent authority ID"),
            provenance_id: WayBackProvenanceId::new("provenance.canonical-year.way-back.descent")
                .expect("canonical descent provenance ID"),
            support: vec![way_back_support(
                "support.glaushouse.way-back.descent-care",
                House::Glaushouse,
                WayBackSupportRole::GlaushouseCareAndClearance,
            )],
            opened_at: timestamp("2046-12-20T08:00:00Z"),
            completed_at: Some(timestamp("2046-12-21T08:00:00Z")),
        },
        WayBackPassage {
            passage_id: WayBackPassageId::new("passage.canonical-year.way-back.ascent")
                .expect("canonical ascent passage ID"),
            canonical_year_id: Some(year_id()),
            traveler_id: WayBackTravelerId::new("traveler.canonical-year.way-back.ascent")
                .expect("canonical ascent traveler ID"),
            direction: WayBackDirection::AscendingFromFlynt,
            route_id,
            origin_house: House::Flynt,
            destination_house: House::Stonebend,
            expression: WayBackExpression::StairwayToHeaven,
            function_id: Some(function_id(GreatFunctionKind::WayBack)),
            clearance_ids: BTreeSet::new(),
            evidence_ids: [evidence("evidence.canonical-year.way-back.ascent")]
                .into_iter()
                .collect(),
            constitutional_authority_id: WayBackAuthorityId::new(
                "authority.compromise.way-back.ascent",
            )
            .expect("canonical ascent authority ID"),
            provenance_id: WayBackProvenanceId::new("provenance.canonical-year.way-back.ascent")
                .expect("canonical ascent provenance ID"),
            support: vec![way_back_support(
                "support.sandmanor.way-back.ascent-arrangement",
                House::Sandmanor,
                WayBackSupportRole::SandmanorArrangement,
            )],
            opened_at: timestamp("2046-12-22T08:00:00Z"),
            completed_at: Some(timestamp("2046-12-23T08:00:00Z")),
        },
    ]
}

fn current_authority(value: &str) -> CurrentSeaAuthorityId {
    CurrentSeaAuthorityId::new(value).expect("canonical Current Sea authority ID")
}

fn current_provenance(value: &str) -> CurrentSeaProvenanceId {
    CurrentSeaProvenanceId::new(value).expect("canonical Current Sea provenance ID")
}

fn current_event(
    id: &str,
    state: CurrentSeaState,
    force: Option<CurrentSeaForce>,
    force_direction: Option<CurrentSeaForceDirection>,
    destination_house: Option<House>,
    occurred_at: &str,
) -> CurrentSeaEvent {
    CurrentSeaEvent {
        event_id: CurrentSeaEventId::new(id).expect("canonical Current Sea event ID"),
        canonical_year_id: Some(year_id()),
        region_id: CurrentSeaRegionId::new(CURRENT_SEA_REGION_ID)
            .expect("canonical Current Sea region ID"),
        state,
        force,
        force_id: force.map(|force| {
            CurrentSeaForceId::new(force.stable_id()).expect("canonical Current Sea force ID")
        }),
        force_direction,
        origin_house: House::Glaushouse,
        destination_house,
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
        constitutional_authority_id: current_authority("authority.compromise.current-sea"),
        provenance_id: current_provenance(&format!("provenance.{id}")),
        occurred_at: timestamp(occurred_at),
    }
}

#[must_use]
pub fn canonical_current_sea_events_fixture() -> Vec<CurrentSeaEvent> {
    vec![
        current_event(
            "current-sea-event.canonical-year.setting",
            CurrentSeaState::Setting,
            None,
            None,
            None,
            "2046-12-21T16:00:00Z",
        ),
        current_event(
            "current-sea-event.canonical-year.riptide-traveler",
            CurrentSeaState::Rising,
            Some(CurrentSeaForce::Riptide),
            Some(CurrentSeaForceDirection::FromGlaushouseTowardFlynt),
            Some(House::Flynt),
            "2046-12-22T06:00:00Z",
        ),
        current_event(
            "current-sea-event.canonical-year.riptide-unaccompanied",
            CurrentSeaState::Rising,
            Some(CurrentSeaForce::Riptide),
            Some(CurrentSeaForceDirection::FromGlaushouseTowardFlynt),
            Some(House::Flynt),
            "2046-12-22T06:15:00Z",
        ),
        current_event(
            "current-sea-event.canonical-year.undertow-hazard",
            CurrentSeaState::Disturbed,
            Some(CurrentSeaForce::Undertow),
            Some(CurrentSeaForceDirection::DownwardOrBeneath),
            None,
            "2046-12-22T06:30:00Z",
        ),
    ]
}

fn canonical_current_sea_fixture_records() -> (
    Vec<GlaushouseClearanceRecord>,
    Vec<BoardwalkPassage>,
    Vec<FlyntResynceRecord>,
    Vec<FlyntRecogRecord>,
) {
    let traveler_id = CurrentSeaTravelerId::new("traveler.canonical-year.boardwalk.cleared-return")
        .expect("canonical Boardwalk traveler ID");
    let clearance_id = CurrentSeaClearanceId::new("clearance.glaushouse.canonical-year.boardwalk")
        .expect("canonical Glaushouse clearance ID");
    let passage_id =
        BoardwalkPassageId::new("passage.canonical-year.boardwalk.glaushouse-to-flynt")
            .expect("canonical Boardwalk passage ID");
    let resynce_id = FlyntResynceEventId::new("resynce.canonical-year.boardwalk-traveler")
        .expect("canonical Resynce ID");

    let clearances = vec![GlaushouseClearanceRecord {
        clearance_id: clearance_id.clone(),
        traveler_id: traveler_id.clone(),
        current_event_id: CurrentSeaEventId::new("current-sea-event.canonical-year.setting")
            .expect("canonical setting event ID"),
        evidence_ids: [evidence(
            "evidence.clearance.glaushouse.canonical-year.boardwalk",
        )]
        .into_iter()
        .collect(),
        constitutional_authority_id: current_authority(
            "authority.glaushouse.current-sea-clearance",
        ),
        provenance_id: current_provenance(
            "provenance.clearance.glaushouse.canonical-year.boardwalk",
        ),
        cleared_at: timestamp("2046-12-22T05:30:00Z"),
        grants_flynt_recog: false,
    }];
    let passages = vec![BoardwalkPassage {
        passage_id: passage_id.clone(),
        canonical_year_id: Some(year_id()),
        traveler_id: traveler_id.clone(),
        route_id: BoardwalkRouteId::new(BOARDWALK_ROUTE_ID).expect("canonical Boardwalk route ID"),
        origin_house: House::Glaushouse,
        destination_house: House::Flynt,
        associated_current_event_id: Some(
            CurrentSeaEventId::new("current-sea-event.canonical-year.riptide-traveler")
                .expect("canonical associated Riptide event ID"),
        ),
        clearance_ids: [clearance_id].into_iter().collect(),
        evidence_ids: [evidence(
            "evidence.passage.canonical-year.boardwalk.glaushouse-to-flynt",
        )]
        .into_iter()
        .collect(),
        constitutional_authority_id: current_authority("authority.compromise.boardwalk-passage"),
        provenance_id: current_provenance(
            "provenance.passage.canonical-year.boardwalk.glaushouse-to-flynt",
        ),
        opened_at: timestamp("2046-12-22T06:05:00Z"),
        completed_at: Some(timestamp("2046-12-22T09:00:00Z")),
        arrived_at_destination: true,
        grants_automatic_recog: false,
    }];
    let resynce_events = vec![FlyntResynceRecord {
        event_id: resynce_id.clone(),
        traveler_id: traveler_id.clone(),
        boardwalk_passage_id: passage_id,
        evidence_ids: [evidence(
            "evidence.resynce.canonical-year.boardwalk-traveler",
        )]
        .into_iter()
        .collect(),
        constitutional_authority_id: current_authority("authority.flynt.resynce-review"),
        provenance_id: current_provenance("provenance.resynce.canonical-year.boardwalk-traveler"),
        occurred_at: timestamp("2046-12-22T10:00:00Z"),
        accepted: true,
    }];
    let recog_events = vec![FlyntRecogRecord {
        event_id: FlyntRecogEventId::new("recog.canonical-year.boardwalk-traveler")
            .expect("canonical Recog ID"),
        traveler_id,
        resynce_event_id: resynce_id,
        manifestation_condition_ids: [
            ManifestationConditionId::new("condition.recog.operational-coherence")
                .expect("canonical Recog condition ID"),
            ManifestationConditionId::new("condition.recog.lawful-recognition")
                .expect("canonical Recog condition ID"),
        ]
        .into_iter()
        .collect(),
        evidence_ids: [evidence("evidence.recog.canonical-year.boardwalk-traveler")]
            .into_iter()
            .collect(),
        constitutional_authority_id: current_authority("authority.flynt.recog-review"),
        provenance_id: current_provenance("provenance.recog.canonical-year.boardwalk-traveler"),
        occurred_at: timestamp("2046-12-22T12:00:00Z"),
        accepted: true,
    }];
    (clearances, passages, resynce_events, recog_events)
}

#[must_use]
pub fn canonical_annual_cycle_fixture() -> CanonicalAnnualCycleRecord {
    let tournament_archive = canonical_service_tournament_archive_fixture();
    let tournament_bytes = encode_service_tournament_archive(&tournament_archive)
        .expect("canonical Tournament archive must encode");
    let tournament_checksum = decode_service_tournament_archive(&tournament_bytes)
        .expect("canonical Tournament archive must decode")
        .checksum;
    let (glaushouse_clearances, boardwalk_passages, flynt_resynce_events, flynt_recog_events) =
        canonical_current_sea_fixture_records();
    CanonicalAnnualCycleRecord {
        id: year_id(),
        calendar: canonical_calendar_year_fixture(),
        functions: canonical_great_functions_fixture(),
        way_back_route: canonical_way_back_route(),
        way_back_passages: canonical_way_back_passages_fixture(),
        current_sea_events: canonical_current_sea_events_fixture(),
        glaushouse_clearances,
        boardwalk_passages,
        flynt_resynce_events,
        flynt_recog_events,
        world_point_archive: canonical_world_point_archive_fixture(),
        nesting: SeasonalNestingRecord {
            gathering_function_id: function_id(GreatFunctionKind::Gathering),
            service_tournament_id: canonical_service_tournament().id,
            service_tournament_year_id: TournamentYearId::new(
                "service-tournament.canonical-year.v1",
            )
            .expect("canonical Tournament year ID"),
            service_tournament_event_id: SeasonalEventId::new(
                "event.canonical-year.gathering.service-tournament",
            )
            .expect("canonical Tournament seasonal event ID"),
            war_id: canonical_war_of_a_thousand_hues().id,
            tournament_result_id: ResultId::new("result.canonical-year.final")
                .expect("canonical Tournament result ID"),
            tournament_archive_checksum: tournament_checksum,
        },
        recognitions: vec![
            recognition(
                "recognition.canonical-year.service-mark",
                "event.canonical-year.festival.service-mark",
                SeasonalRecognitionSubject::ServiceMark(
                    ServiceMarkId::new("service-mark.canonical-year.four-house-bridge")
                        .expect("canonical Service Mark ID"),
                ),
                "The Festival preserves and reads the Four-House Bridge Extraction.",
            ),
            recognition(
                "recognition.canonical-year.edge",
                "event.canonical-year.festival.edge",
                SeasonalRecognitionSubject::EdgeOfTomorrow(
                    ArtifactId::new(EDGE_OF_TOMORROW_ID).expect("canonical Edge ID"),
                ),
                "The Festival confirms the Edge custody and lawful route-lock refinement.",
            ),
            recognition(
                "recognition.canonical-year.glass",
                "event.canonical-year.festival.glass",
                SeasonalRecognitionSubject::GlassOfAThousandHues(
                    ArtifactId::new(GLASS_OF_A_THOUSAND_HUES_ID).expect("canonical Glass ID"),
                ),
                "The Festival confirms the Glass custody and emergency-diagnostic facet.",
            ),
        ],
        tournament_archive,
    }
}

#[must_use]
pub fn canonical_seasonal_archive_fixture() -> SeasonalArchivePayload {
    SeasonalArchivePayload {
        annual_cycles: vec![canonical_annual_cycle_fixture()],
    }
}

#[must_use]
pub const fn presiding_house(kind: GreatFunctionKind) -> House {
    kind.presiding_house()
}
