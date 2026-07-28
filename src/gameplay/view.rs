use serde::{Deserialize, Serialize};

use crate::world::aura_basin::canonical_aura_basin;
use crate::world::aura_beach::canonical_aura_beach;
use crate::world::aura_field::canonical_aura_field;
use crate::world::extraction::{ExtractionMethod, extraction_site};
use crate::world::geography::canonical_constitutional_geography;
use crate::world::interior_surface::{InteriorSurfaceId, SurfacePoint};
use crate::world::route_network::{RouteNetwork, canonical_route_geometry};

use super::{
    ActiveIncarnationRef, BeingContinuityRecord, BoardwalkAuthorityClass, BoardwalkCase,
    BoardwalkCasePhase, BoardwalkChoice, BoardwalkEvidence, BoardwalkOutcomeRecord,
    BoardwalkRelationshipKind, CardinalDirection, DeepPressureAftermath, DeepPressureOutcomeRecord,
    DeepPressurePersonPresence, DeepPressurePhase, DeepPressureSettlementChoice, DeepPressureState,
    EvidenceJournalRecord, GameRevision, GameplayEvent, GameplayEventEnvelope,
    HollowGroveGameRuntime, HuemanFaculty, InteractionId, LivingWorldState, MAP_HEIGHT,
    MAP_TILE_SIZE, MAP_WIDTH, MAX_PARTY_MEMBERS, PartyActorId, PartyEvent, PartyMemberAvailability,
    RecruitmentCandidateId, RecruitmentDecision, RecruitmentDecisionReason, RecruitmentPath,
    RelationshipMemory, StonebendAuthorityClass, StonebendCasePhase, StonebendContinuityCase,
    StonebendContinuityChoice, StonebendEvidence, StonebendOutcomeRecord, TilePosition, WorldMapId,
    map_definition,
};

/// Stable presentation event names. Variants without an implemented reducer
/// are schema reservations only and cannot be emitted as canonical facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameplayEventKind {
    IdentityEstablished,
    MovementAccepted,
    MovementRejected,
    InteractionOpened,
    EncounterOpened,
    ActionResolved,
    BondChanged,
    PartyChanged,
    LeadChanged,
    RecognitionGranted,
    SynthesisAvailable,
    FrameChanged,
    RegionEntered,
    FacultyObserved,
    ChoiceSupported,
    ChoiceCommitted,
    WorldStateChanged,
    SnapshotSaved,
    SnapshotLoaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventAuthority {
    Canonical,
    Projection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BeingIdentityView {
    pub continuity_id: String,
    pub incarnation_kind: String,
    pub regional_being_id: Option<String>,
    pub participant_id: String,
    pub institutional_being_id: String,
}

impl BeingIdentityView {
    #[must_use]
    pub fn from_record(record: &BeingContinuityRecord) -> Self {
        let (incarnation_kind, regional_being_id) = match record.incarnation() {
            ActiveIncarnationRef::Hueman => ("Hueman".into(), None),
            ActiveIncarnationRef::Regional(regional) => {
                ("Regional".into(), Some(regional.as_str().into()))
            }
        };
        Self {
            continuity_id: record.id().as_str().into(),
            incarnation_kind,
            regional_being_id,
            participant_id: record.domain_refs().participant().as_str().into(),
            institutional_being_id: record.domain_refs().institutional().as_str().into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameplayEventView {
    pub kind: GameplayEventKind,
    pub authority: EventAuthority,
    pub source_event_id: Option<String>,
    pub sequence: Option<u64>,
    pub revision: u64,
    pub subject: Option<BeingIdentityView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<InteractionView>,
}

impl GameplayEventView {
    #[must_use]
    pub fn from_canonical(event: &GameplayEventEnvelope) -> Self {
        let (kind, subject, interaction) = match &event.payload {
            GameplayEvent::HuemanIdentityEstablished { identity }
            | GameplayEvent::RegionalBeingIdentityEstablished { identity, .. } => (
                GameplayEventKind::IdentityEstablished,
                Some(BeingIdentityView::from_record(identity)),
                None,
            ),
            GameplayEvent::HuemanMovementResolved { from, to } => (
                if from.x == to.x && from.y == to.y {
                    GameplayEventKind::MovementRejected
                } else {
                    GameplayEventKind::MovementAccepted
                },
                None,
                None,
            ),
            GameplayEvent::HuemanInteractionOpened { target, .. } => (
                GameplayEventKind::InteractionOpened,
                None,
                Some(InteractionView::from_target(*target)),
            ),
            GameplayEvent::HuemanMapEntered { .. } => {
                (GameplayEventKind::RegionEntered, None, None)
            }
            GameplayEvent::BoardwalkFacultyDisclosed { .. } => {
                (GameplayEventKind::FacultyObserved, None, None)
            }
            GameplayEvent::StonebendFacultyDisclosed { .. } => {
                (GameplayEventKind::FacultyObserved, None, None)
            }
            GameplayEvent::BoardwalkOptionSupported { .. } => {
                (GameplayEventKind::ChoiceSupported, None, None)
            }
            GameplayEvent::StonebendContinuityOptionSupported { .. } => {
                (GameplayEventKind::ChoiceSupported, None, None)
            }
            GameplayEvent::ReturningGoonChoiceCommitted {
                relationship_bond, ..
            } => (
                if relationship_bond.is_some() {
                    GameplayEventKind::BondChanged
                } else {
                    GameplayEventKind::ChoiceCommitted
                },
                None,
                None,
            ),
            GameplayEvent::StonebendContinuityDeterminationCommitted { .. } => {
                (GameplayEventKind::ChoiceCommitted, None, None)
            }
            GameplayEvent::LivingWorldChanged {
                event: super::LivingWorldEvent::CaseSupportRecorded { .. },
                ..
            } => (GameplayEventKind::ChoiceSupported, None, None),
            GameplayEvent::LivingWorldChanged { .. } => {
                (GameplayEventKind::WorldStateChanged, None, None)
            }
            GameplayEvent::DeepPressureChanged {
                event: super::DeepPressureEvent::SettlementSupportRecorded { .. },
            } => (GameplayEventKind::ChoiceSupported, None, None),
            GameplayEvent::DeepPressureChanged { .. } => {
                (GameplayEventKind::ChoiceCommitted, None, None)
            }
            GameplayEvent::PartyChanged { event } => (
                match event {
                    PartyEvent::RecruitmentDecided { .. }
                    | PartyEvent::MemberSelected { .. }
                    | PartyEvent::ShiftRecoveryApplied { .. } => GameplayEventKind::PartyChanged,
                    PartyEvent::LeadChanged { .. } => GameplayEventKind::LeadChanged,
                    PartyEvent::FieldActionResolved { .. } => GameplayEventKind::ActionResolved,
                },
                None,
                None,
            ),
        };
        Self {
            kind,
            authority: EventAuthority::Canonical,
            source_event_id: Some(event.id.as_str().into()),
            sequence: Some(event.sequence),
            revision: event.revision.get(),
            subject,
            interaction,
        }
    }

    #[must_use]
    pub fn snapshot_loaded(revision: GameRevision) -> Self {
        Self {
            kind: GameplayEventKind::SnapshotLoaded,
            authority: EventAuthority::Projection,
            source_event_id: None,
            sequence: None,
            revision: revision.get(),
            subject: None,
            interaction: None,
        }
    }

    #[must_use]
    pub fn snapshot_saved(revision: GameRevision) -> Self {
        Self {
            kind: GameplayEventKind::SnapshotSaved,
            authority: EventAuthority::Projection,
            source_event_id: None,
            sequence: None,
            revision: revision.get(),
            subject: None,
            interaction: None,
        }
    }
}

/// Presentation copy for an interaction already resolved by the runtime.
///
/// The anonymous guide projects Aura Ridge culture without establishing a
/// canonical named NPC or granting institutional authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InteractionView {
    pub target_id: String,
    pub speaker: String,
    pub pages: Vec<String>,
}

impl InteractionView {
    #[must_use]
    pub fn from_target(target: InteractionId) -> Self {
        Self::from_target_and_cases(target, None, None)
    }

    #[must_use]
    pub fn from_target_and_case(target: InteractionId, case: Option<&BoardwalkCase>) -> Self {
        Self::from_target_and_cases(target, case, None)
    }

    #[must_use]
    pub fn from_target_and_cases(
        target: InteractionId,
        boardwalk_case: Option<&BoardwalkCase>,
        stonebend_case: Option<&StonebendContinuityCase>,
    ) -> Self {
        Self::from_target_and_campaigns(target, boardwalk_case, stonebend_case, None)
    }

    #[must_use]
    pub fn from_target_and_campaigns(
        target: InteractionId,
        boardwalk_case: Option<&BoardwalkCase>,
        stonebend_case: Option<&StonebendContinuityCase>,
        deep_pressure: Option<&DeepPressureState>,
    ) -> Self {
        match target {
            InteractionId::DeepPressurePerson(person) => {
                let record = super::deep_pressure_statement_record(
                    super::DeepPressureStatementId::Person(person),
                    super::LivingClock {
                        day: 1,
                        shift: super::WorkShift::Dawn,
                    },
                );
                let mut pages =
                    vec![format!("{:?}", record.speech_classification.unwrap()).to_uppercase()];
                pages.extend(wrap_dialogue(&record.claim.to_uppercase(), 36));
                pages.extend(wrap_dialogue(
                    &format!("LIMIT: {}", record.uncertainty).to_uppercase(),
                    36,
                ));
                if let Some(memory) =
                    deep_pressure.and_then(|campaign| campaign.relationships.get(&person))
                {
                    pages.push(format!("CONDITION: {:?}", memory.condition).to_uppercase());
                    if let Some(remembered) = memory.remembered_outcomes.last() {
                        pages.extend(wrap_dialogue(
                            &format!("I REMEMBER: {remembered}").to_uppercase(),
                            36,
                        ));
                    }
                    if let Some(promise) = memory.unresolved_promises.iter().next() {
                        pages.extend(wrap_dialogue(
                            &format!("STILL OWED: {promise}").to_uppercase(),
                            36,
                        ));
                    }
                }
                Self {
                    target_id: person.stable_id().into(),
                    speaker: person.display_name().to_uppercase(),
                    pages,
                }
            }
            InteractionId::AuraRidgeWitnessMarker => Self {
                target_id: "interaction.aura-ridge.witness-marker".into(),
                speaker: "AURA RIDGE".into(),
                pages: vec![
                    "WITNESS".into(),
                    "PUBLIC SIGHT. EQUAL GAZE.".into(),
                    "CURRENT SEA CERTIFIES ELSEWHERE.".into(),
                ],
            },
            InteractionId::RidgefolkGuide => Self {
                target_id: "interaction.aura-ridge.ridgefolk-guide".into(),
                speaker: "RIDGEFOLK GUIDE".into(),
                pages: vec![
                    "AURA RIDGE MEANS WITNESS.".into(),
                    "WE MEET EACH OTHER IN EQUAL GAZE.".into(),
                    "RESTORATION CAN RETURN TO SIGHT.".into(),
                ],
            },
            InteractionId::BoardwalkDischargeAdvocate => Self {
                target_id: "interaction.boardwalk.discharge-advocate".into(),
                speaker: "DISCHARGE ADVOCATE".into(),
                pages: vec![
                    "THE RETURN WAS VOLUNTARY.".into(),
                    "NO PATRON OR RETINUE OWNS A PERSON.".into(),
                    "HEAR EVERY OFFER BEFORE SUPPORT.".into(),
                ],
            },
            InteractionId::BoardwalkPimp => Self {
                target_id: "interaction.boardwalk.pimp".into(),
                speaker: "PIMP".into(),
                pages: vec![
                    "MY AURA OPENS DOORS, NOT CAGES.".into(),
                    "PATRONAGE ENDS WHEN CONSENT ENDS.".into(),
                    "DELIBERATE DECEPTION / DEEP PRESSURE:".into(),
                    "A CERTIFICATE DOES NOT ERASE RESTITUTION.".into(),
                ],
            },
            InteractionId::BoardwalkHoeWitness => Self {
                target_id: "interaction.boardwalk.hoe-witness".into(),
                speaker: "HOE WITNESS".into(),
                pages: vec![
                    "I CARRY THE PIMP'S AURA BY CHOICE.".into(),
                    "MY WORK, TESTIMONY, AND EXIT ARE MINE.".into(),
                ],
            },
            InteractionId::BoardwalkGimp => Self {
                target_id: "interaction.boardwalk.gimp".into(),
                speaker: "GIMP".into(),
                pages: vec![
                    "MY BODY IS AFFLICTED. MY TITLE IS NOT.".into(),
                    "A FINITE BOND CAN HOLD CURRENT SAFELY.".into(),
                    "LOCAL TRADITION / DEEP PRESSURE:".into(),
                    "EVERY GOON MUST KNOW THE EDGE AND EXIT.".into(),
                ],
            },
            InteractionId::BoardwalkGoonWitness => Self {
                target_id: "interaction.boardwalk.goon-witness".into(),
                speaker: "GOON WITNESS".into(),
                pages: vec![
                    "GOON IS COMMON BOARDWALK SPEECH.".into(),
                    "A GOON BOND MAKES A RETINUE LAWFUL.".into(),
                    "IT HAS A TERM, DUTIES, AND AN EXIT.".into(),
                ],
            },
            InteractionId::BoardwalkFacultyStation => Self {
                target_id: "interaction.boardwalk.faculty-station".into(),
                speaker: "FIVE FACULTIES".into(),
                pages: vec![
                    "F: DISCLOSE REASON THROUGH WILL.".into(),
                    "A CHOICE NEEDS MORE THAN ONE KIND OF SIGHT.".into(),
                ],
            },
            InteractionId::BoardwalkReturningGoon => {
                let pages = match boardwalk_case.and_then(BoardwalkCase::committed_choice) {
                    Some(BoardwalkChoice::PimpPatronage) => vec![
                        "I CHOSE PATRONAGE AND KEPT MY EXIT.".into(),
                        "THE PIMP'S AURA OPENS MY NEXT DOOR.".into(),
                    ],
                    Some(BoardwalkChoice::GoonBond) => vec![
                        "I CHOSE THE GIMP'S FINITE GOON BOND.".into(),
                        "MY CURRENT IS HELD, NEVER OWNED.".into(),
                    ],
                    Some(BoardwalkChoice::LimitedCooperation) => vec![
                        "I CHOSE ONE JOB, NOT A HOUSEHOLD.".into(),
                        "COOPERATION ENDS AT THE AGREED EDGE.".into(),
                    ],
                    Some(BoardwalkChoice::IndependentReturn) => vec![
                        "I CHOSE MY OWN RETURN.".into(),
                        "WITNESS ME WITHOUT CLAIMING ME.".into(),
                    ],
                    None => vec![
                        "I WILL HEAR WHAT YOU FOUND.".into(),
                        "YOU MAY SUPPORT. I WILL DECIDE.".into(),
                    ],
                };
                Self {
                    target_id: "interaction.boardwalk.returning-goon".into(),
                    speaker: "RETURNING GOON".into(),
                    pages,
                }
            }
            InteractionId::CurrentSeaGeraldRegistrar => Self {
                target_id: "interaction.current-sea.gerald-registrar".into(),
                speaker: "GERALD REGISTRAR".into(),
                pages: vec![
                    "STONEBEND NAMES. A MIRROR DOES NOT.".into(),
                    "RESTORATION IS EVIDENCE, NOT IDENTITY.".into(),
                    "NO NAME HERE CAN GRANT A TITLE.".into(),
                ],
            },
            InteractionId::CurrentSeaMercyDeep => {
                let pages = match stonebend_case.and_then(StonebendContinuityCase::committed_choice)
                {
                    Some(StonebendContinuityChoice::AffirmExistingName) => vec![
                        "I AM MERCY DEEP, BEFORE AND AFTER.".into(),
                        "THE SEAL HOLDS MY NAME, NOT MY BODY.".into(),
                    ],
                    Some(StonebendContinuityChoice::ProvisionalTransformedFormName) => vec![
                        "AFTERTIDE NAMES THIS FORM FOR NOW.".into(),
                        "MERCY DEEP REMAINS IN THE RECORD.".into(),
                    ],
                    Some(StonebendContinuityChoice::ReferIdentityConflict) => vec![
                        "UNCERTAINTY DID NOT MAKE ME VANISH.".into(),
                        "I WILL SPEAK AGAIN AT HIGH REVIEW.".into(),
                    ],
                    None => vec![
                        "I REMEMBER MY NAME: MERCY DEEP.".into(),
                        "THE CROWD SAW CHANGE. IT DID NOT CHOOSE.".into(),
                        "YOU MAY SUPPORT. STONEBEND MUST NAME.".into(),
                    ],
                };
                Self {
                    target_id: "interaction.current-sea.mercy-deep".into(),
                    speaker: "MERCY DEEP".into(),
                    pages,
                }
            }
            InteractionId::CurrentSeaDepthWitness => Self {
                target_id: "interaction.current-sea.crowd-witness".into(),
                speaker: "CROWD WITNESS".into(),
                pages: vec![
                    "I SAW ONE CURRENT HOLD THROUGH THE CROWD.".into(),
                    "I CANNOT NAME WHAT THAT CONTINUITY MEANS.".into(),
                ],
            },
            InteractionId::CurrentSeaNameLedger => Self {
                target_id: "interaction.current-sea.name-ledger".into(),
                speaker: "NAME LEDGER".into(),
                pages: vec![
                    "MERCY DEEP. CIVIC NAME. PRIOR FORM.".into(),
                    "HISTORY PRESENT. NO TITLE ATTACHED.".into(),
                ],
            },
            InteractionId::CurrentSeaMercuryMirror => Self {
                target_id: "interaction.current-sea.mercury-mirror".into(),
                speaker: "MERCURY MIRROR".into(),
                pages: vec![
                    "CORRESPONDENCE: STRONG. DIFFERENCE: REAL.".into(),
                    "A MIRROR VERIFIES. IT NEVER DECIDES.".into(),
                ],
            },
            InteractionId::CurrentSeaRestorationArchive => Self {
                target_id: "interaction.current-sea.restoration-archive".into(),
                speaker: "RESTORATION ARCHIVE".into(),
                pages: vec![
                    "GLAUSHOUSE: RESTORATION COMPLETE.".into(),
                    "CLINICAL SUCCESS SETTLES NO NAME.".into(),
                ],
            },
            InteractionId::CurrentSeaFacultyStation => Self {
                target_id: "interaction.current-sea.faculty-station".into(),
                speaker: "FIVE FACULTIES".into(),
                pages: vec![
                    "F: DISCLOSE REASON THROUGH WILL.".into(),
                    "FACULTIES REVEAL. THEY DO NOT CHOOSE.".into(),
                ],
            },
            InteractionId::AuraFieldFacility(facility_id) => {
                let field = canonical_aura_field().expect("canonical Aura Field");
                let facility = field
                    .facility(facility_id)
                    .expect("canonical Aura Field facility");
                Self {
                    target_id: facility.id.stable_id().into(),
                    speaker: facility.name.to_uppercase(),
                    pages: vec![
                        format!(
                            "{} / INSIDE THE ONE AURA FIELD.",
                            facility.kind.as_str().to_uppercase()
                        ),
                        facility.function.to_uppercase(),
                        "WORK MAY PROVE A METHOD. IT DOES NOT CREATE TITLE.".into(),
                    ],
                }
            }
            InteractionId::AuraBeachFacility(facility_id) => {
                let beach = canonical_aura_beach().expect("canonical Aura Beach");
                let facility = beach
                    .facility(facility_id)
                    .expect("canonical Aura Beach facility");
                Self {
                    target_id: facility.id.stable_id().into(),
                    speaker: facility.name.to_uppercase(),
                    pages: vec![
                        format!(
                            "{} / INSIDE AURA BEACH.",
                            facility.kind.as_str().to_uppercase()
                        ),
                        facility.function.to_uppercase(),
                        "COASTAL PROOF DOES NOT CREATE SYNTHESIS OR TITLE.".into(),
                    ],
                }
            }
            InteractionId::AuraBasinFacility(facility_id) => {
                let basin = canonical_aura_basin().expect("canonical Aura Basin");
                let facility = basin
                    .facility(facility_id)
                    .expect("canonical Aura Basin facility");
                Self {
                    target_id: facility.id.stable_id().into(),
                    speaker: facility.name.to_uppercase(),
                    pages: vec![
                        format!(
                            "{} / INSIDE AURA BASIN.",
                            facility.kind.as_str().to_uppercase()
                        ),
                        facility.function.to_uppercase(),
                        "SURVIVAL, SALVAGE, OR VICTORY CREATES NO AUTHORITY.".into(),
                    ],
                }
            }
            InteractionId::ExtractionFacility { site, facility } => {
                let definition = extraction_site(site).expect("canonical extraction site");
                Self {
                    target_id: format!("{}.{}", site.stable_id(), facility.stable_id()),
                    speaker: facility.display_name().to_uppercase(),
                    pages: vec![
                        format!(
                            "{} / {}.",
                            definition.method.analogy().to_uppercase(),
                            definition.resource.display_name().to_uppercase()
                        ),
                        definition.constitutional_function.to_uppercase(),
                        "CUSTODY IS RECORDED. ROUTE ACCESS CREATES NO MINERAL TITLE.".into(),
                        "LIVING BLOOD IS NEVER AN EXTRACTED RESOURCE.".into(),
                    ],
                }
            }
            InteractionId::ConstitutionalRouteWitness(route) => {
                let geography = canonical_constitutional_geography()
                    .expect("canonical constitutional route geography");
                let definition = geography
                    .route(route)
                    .expect("canonical constitutional route definition");
                Self {
                    target_id: format!("interaction.route.{}.witness", route.stable_id()),
                    speaker: route.display_name().to_uppercase(),
                    pages: vec![
                        format!(
                            "{} / {}",
                            route.display_name().to_uppercase(),
                            definition.verb.as_str().to_uppercase()
                        ),
                        definition.purpose.to_uppercase(),
                        format!(
                            "{} SHAPE. GEOMETRY IS NOT AUTHORITY.",
                            canonical_route_geometry(route).as_str().to_uppercase()
                        ),
                    ],
                }
            }
        }
    }
}

fn wrap_dialogue(value: &str, width: usize) -> Vec<String> {
    let mut pages = Vec::new();
    let mut page = String::new();
    for word in value.split_whitespace() {
        if !page.is_empty() && page.len() + 1 + word.len() > width {
            pages.push(page);
            page = String::new();
        }
        if !page.is_empty() {
            page.push(' ');
        }
        page.push_str(word);
    }
    if !page.is_empty() {
        pages.push(page);
    }
    pages
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoardwalkCaseView {
    pub case_id: String,
    pub returning_goon_id: String,
    pub phase: BoardwalkCasePhase,
    pub evidence: Vec<BoardwalkEvidence>,
    pub faculties: Vec<HuemanFaculty>,
    pub ready_for_support: bool,
    pub supported_choice: Option<BoardwalkChoice>,
    pub committed_choice: Option<BoardwalkChoice>,
    pub goon_bond_id: Option<String>,
    pub outcome: Option<BoardwalkOutcomeView>,
    pub decision_maker_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoardwalkOutcomeView {
    pub outcome_id: String,
    pub authority_class: BoardwalkAuthorityClass,
    pub jurisdiction: String,
    pub participants: Vec<String>,
    pub dominant_verb: String,
    pub lawful_state_change: String,
    pub relationship_kind: Option<BoardwalkRelationshipKind>,
    pub relationship_bond_id: Option<String>,
    pub relationship_term_end: Option<u64>,
    pub glaushouse_clearance_id: String,
    pub glaushouse_authority_actor: String,
    pub flynt_recognition_id: String,
    pub flynt_authority_actor: String,
    pub player_support_is_nonbinding: bool,
    pub faculty_uncertainties: Vec<String>,
    pub persistence_and_replay: String,
    pub presentation: String,
    pub failure_and_refusal: Vec<String>,
}

impl BoardwalkOutcomeView {
    #[must_use]
    pub fn from_record(outcome: &BoardwalkOutcomeRecord) -> Self {
        Self {
            outcome_id: outcome.id.as_str().into(),
            authority_class: outcome.authority_class,
            jurisdiction: outcome.jurisdiction.stable_id().into(),
            participants: outcome
                .participants
                .iter()
                .map(|value| value.as_str().into())
                .collect(),
            dominant_verb: outcome.dominant_verb.as_str().into(),
            lawful_state_change: outcome.lawful_state_change.into(),
            relationship_kind: outcome
                .relationship
                .as_ref()
                .map(|relationship| relationship.kind),
            relationship_bond_id: outcome
                .relationship
                .as_ref()
                .map(|relationship| relationship.bond.as_str().into()),
            relationship_term_end: outcome
                .relationship
                .as_ref()
                .map(|relationship| relationship.term_end.get()),
            glaushouse_clearance_id: outcome.glaushouse_discharge_clearance.id.as_str().into(),
            glaushouse_authority_actor: outcome
                .glaushouse_discharge_clearance
                .authority
                .actor
                .as_str()
                .into(),
            flynt_recognition_id: outcome.flynt_recognition.id.as_str().into(),
            flynt_authority_actor: outcome.flynt_recognition.authority.actor.as_str().into(),
            player_support_is_nonbinding: outcome.player_support_is_nonbinding,
            faculty_uncertainties: outcome
                .faculty_uncertainties
                .iter()
                .map(|value| (*value).into())
                .collect(),
            persistence_and_replay: outcome.persistence_and_replay.into(),
            presentation: outcome.presentation.into(),
            failure_and_refusal: outcome
                .failure_and_refusal
                .iter()
                .map(|value| (*value).into())
                .collect(),
        }
    }
}

impl BoardwalkCaseView {
    #[must_use]
    pub fn from_case(case: &BoardwalkCase) -> Self {
        Self {
            case_id: case.id().as_str().into(),
            returning_goon_id: case.returning_goon().as_str().into(),
            phase: case.phase(),
            evidence: case.evidence().iter().copied().collect(),
            faculties: case.faculties().iter().copied().collect(),
            ready_for_support: case.is_ready(),
            supported_choice: case.supported_choice(),
            committed_choice: case.committed_choice(),
            goon_bond_id: case
                .goon_bond()
                .map(|commit| commit.bond.as_str().to_owned()),
            outcome: case.outcome().map(BoardwalkOutcomeView::from_record),
            decision_maker_id: super::RETURNING_GOON_PARTICIPANT_ID.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StonebendCaseView {
    pub case_id: String,
    pub subject_id: String,
    pub subject_name: String,
    pub phase: StonebendCasePhase,
    pub evidence: Vec<StonebendEvidence>,
    pub faculties: Vec<HuemanFaculty>,
    pub ready_for_support: bool,
    pub supported_choice: Option<StonebendContinuityChoice>,
    pub committed_choice: Option<StonebendContinuityChoice>,
    pub outcome: Option<StonebendOutcomeView>,
    pub player_support_is_advisory: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StonebendOutcomeView {
    pub outcome_id: String,
    pub authority_class: StonebendAuthorityClass,
    pub jurisdiction: String,
    pub dominant_verb: String,
    pub lawful_state_change: String,
    pub stonebend_decision_id: String,
    pub stonebend_authority_actor: String,
    pub decision_record_id: String,
    pub seal_record_id: String,
    pub active_names: Vec<String>,
    pub provisional_names: Vec<String>,
    pub title_granted: bool,
    pub player_support_is_nonbinding: bool,
    pub faculty_uncertainties: Vec<String>,
    pub persistence_and_replay: String,
    pub presentation: String,
    pub failure_and_refusal: Vec<String>,
}

impl StonebendOutcomeView {
    #[must_use]
    pub fn from_record(outcome: &StonebendOutcomeRecord) -> Self {
        Self {
            outcome_id: outcome.id.as_str().into(),
            authority_class: outcome.authority_class,
            jurisdiction: outcome.jurisdiction.stable_id().into(),
            dominant_verb: outcome.dominant_verb.as_str().into(),
            lawful_state_change: outcome.lawful_state_change.into(),
            stonebend_decision_id: outcome.stonebend_naming.id.as_str().into(),
            stonebend_authority_actor: outcome.stonebend_naming.authority.actor.as_str().into(),
            decision_record_id: outcome.determination.decision.id.as_str().into(),
            seal_record_id: outcome.determination.seal.id.as_str().into(),
            active_names: outcome
                .determination
                .name_records
                .iter()
                .filter(|record| record.status == crate::world::stonebend::NameStatus::Active)
                .map(|record| record.name.clone())
                .collect(),
            provisional_names: outcome
                .determination
                .name_records
                .iter()
                .filter(|record| record.status == crate::world::stonebend::NameStatus::Provisional)
                .map(|record| record.name.clone())
                .collect(),
            title_granted: outcome.title_granted,
            player_support_is_nonbinding: outcome.player_support_is_nonbinding,
            faculty_uncertainties: outcome
                .faculty_uncertainties
                .iter()
                .map(|value| (*value).into())
                .collect(),
            persistence_and_replay: outcome.persistence_and_replay.into(),
            presentation: outcome.presentation.into(),
            failure_and_refusal: outcome
                .failure_and_refusal
                .iter()
                .map(|value| (*value).into())
                .collect(),
        }
    }
}

impl StonebendCaseView {
    #[must_use]
    pub fn from_case(case: &StonebendContinuityCase) -> Self {
        Self {
            case_id: case.id().as_str().into(),
            subject_id: case.subject().as_str().into(),
            subject_name: super::MERCY_DEEP_EXISTING_NAME.into(),
            phase: case.phase(),
            evidence: case.evidence().iter().copied().collect(),
            faculties: case.faculties().iter().copied().collect(),
            ready_for_support: case.is_ready(),
            supported_choice: case.supported_choice(),
            committed_choice: case.committed_choice(),
            outcome: case.outcome().map(StonebendOutcomeView::from_record),
            player_support_is_advisory: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntentCapabilityView {
    pub intent_type: String,
    pub available: bool,
    pub unavailable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TilePositionView {
    pub x: u16,
    pub y: u16,
    pub facing: CardinalDirection,
}

impl From<TilePosition> for TilePositionView {
    fn from(position: TilePosition) -> Self {
        Self {
            x: position.x,
            y: position.y,
            facing: position.facing,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OverworldView {
    pub map_id: String,
    pub width: u16,
    pub height: u16,
    pub tile_size: u16,
    pub tile_rows: Vec<String>,
    pub player: TilePositionView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteView {
    pub route_id: String,
    pub display_name: String,
    pub geometry: String,
    pub presentation_geometry: String,
    pub endpoints: Vec<String>,
    pub dominant_verb: String,
    pub purpose: String,
    pub process: Vec<String>,
}

impl RouteView {
    #[must_use]
    pub fn from_map(map: super::WorldMapId) -> Option<Self> {
        let route = map.route()?;
        let geography =
            canonical_constitutional_geography().expect("canonical constitutional geography");
        let definition = geography
            .route(route)
            .expect("canonical constitutional route");
        let network = RouteNetwork::canonical().expect("canonical route network");
        let segment = network.segment(route).expect("canonical route segment");
        Some(Self {
            route_id: route.stable_id().into(),
            display_name: route.display_name().into(),
            geometry: segment.geometry.as_str().into(),
            presentation_geometry: segment.geometry.presentation_term().into(),
            endpoints: segment
                .endpoints
                .iter()
                .map(|house| format!("{house:?}"))
                .collect(),
            dominant_verb: definition.verb.as_str().into(),
            purpose: definition.purpose.into(),
            process: definition
                .process
                .iter()
                .map(|stage| (*stage).into())
                .collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfacePointView {
    pub x_per_mille: u16,
    pub y_per_mille: u16,
}

impl From<SurfacePoint> for SurfacePointView {
    fn from(point: SurfacePoint) -> Self {
        Self {
            x_per_mille: point.x_per_mille,
            y_per_mille: point.y_per_mille,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceFacilityView {
    pub facility_id: String,
    pub display_name: String,
    pub kind: String,
    pub function: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InteriorSurfaceView {
    pub surface_id: String,
    pub display_name: String,
    pub singular_region: bool,
    pub dominant_house: String,
    pub regional_attribution: String,
    pub boundary: Vec<SurfacePointView>,
    pub access_routes: Vec<String>,
    pub facilities: Vec<SurfaceFacilityView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractionSiteView {
    pub site_id: String,
    pub display_name: String,
    pub map_id: String,
    pub route_id: String,
    pub method: ExtractionMethod,
    pub method_analogy: String,
    pub resource: String,
    pub constitutional_function: String,
    pub facilities: Vec<String>,
    pub principal_hazards: Vec<String>,
    pub route_limit: String,
    pub nested_inside_stairway_complex: bool,
}

impl ExtractionSiteView {
    #[must_use]
    pub fn from_map(map: WorldMapId) -> Option<Self> {
        let site_id = map.extraction()?;
        let site = extraction_site(site_id).expect("canonical extraction site");
        Some(Self {
            site_id: site.id.stable_id().into(),
            display_name: site.id.display_name().into(),
            map_id: site.id.map_id().into(),
            route_id: site.id.route().stable_id().into(),
            method: site.method,
            method_analogy: site.method.analogy().into(),
            resource: site.resource.display_name().into(),
            constitutional_function: site.constitutional_function.into(),
            facilities: site
                .facilities
                .iter()
                .map(|facility| facility.display_name().into())
                .collect(),
            principal_hazards: site
                .principal_hazards
                .iter()
                .map(|hazard| (*hazard).into())
                .collect(),
            route_limit: site.route_limit.into(),
            nested_inside_stairway_complex: site.id.is_nested_descent(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalExitView {
    pub exit_x: u16,
    pub exit_y: u16,
    pub destination_map_id: String,
    pub destination_label: String,
}

impl InteriorSurfaceView {
    #[must_use]
    pub fn from_map(map: super::WorldMapId) -> Option<Self> {
        match map.surface()? {
            InteriorSurfaceId::AuraField => {
                let field = canonical_aura_field().expect("canonical Aura Field");
                Some(Self {
                    surface_id: field.id.stable_id().into(),
                    display_name: field.id.display_name().into(),
                    singular_region: true,
                    dominant_house: format!("{:?}", field.dominant_house),
                    regional_attribution: field.regional_attribution.into(),
                    boundary: field.boundary.into_iter().map(Into::into).collect(),
                    access_routes: field
                        .access_routes
                        .into_iter()
                        .map(|route| route.stable_id().into())
                        .collect(),
                    facilities: field
                        .facilities
                        .iter()
                        .map(|facility| SurfaceFacilityView {
                            facility_id: facility.id.stable_id().into(),
                            display_name: facility.name.into(),
                            kind: facility.kind.as_str().into(),
                            function: facility.function.into(),
                        })
                        .collect(),
                })
            }
            InteriorSurfaceId::AuraBeach => {
                let beach = canonical_aura_beach().expect("canonical Aura Beach");
                Some(Self {
                    surface_id: beach.id.stable_id().into(),
                    display_name: beach.id.display_name().into(),
                    singular_region: true,
                    dominant_house: format!("{:?}", beach.dominant_house),
                    regional_attribution: beach.regional_attribution.into(),
                    boundary: beach.boundary.into_iter().map(Into::into).collect(),
                    access_routes: beach
                        .access_routes
                        .into_iter()
                        .map(|route| route.stable_id().into())
                        .collect(),
                    facilities: beach
                        .facilities
                        .iter()
                        .map(|facility| SurfaceFacilityView {
                            facility_id: facility.id.stable_id().into(),
                            display_name: facility.name.into(),
                            kind: facility.kind.as_str().into(),
                            function: facility.function.into(),
                        })
                        .collect(),
                })
            }
            InteriorSurfaceId::AuraBasin => {
                let basin = canonical_aura_basin().expect("canonical Aura Basin");
                Some(Self {
                    surface_id: basin.id.stable_id().into(),
                    display_name: basin.id.display_name().into(),
                    singular_region: true,
                    dominant_house: format!("{:?}", basin.dominant_house),
                    regional_attribution: basin.regional_attribution.into(),
                    boundary: basin.boundary.into_iter().map(Into::into).collect(),
                    access_routes: basin
                        .access_routes
                        .into_iter()
                        .map(|route| route.stable_id().into())
                        .collect(),
                    facilities: basin
                        .facilities
                        .iter()
                        .map(|facility| SurfaceFacilityView {
                            facility_id: facility.id.stable_id().into(),
                            display_name: facility.name.into(),
                            kind: facility.kind.as_str().into(),
                            function: facility.function.into(),
                        })
                        .collect(),
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameView {
    pub revision: u64,
    pub identity_count: usize,
    pub regional_event_count: usize,
    pub hueman: Option<BeingIdentityView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overworld: Option<OverworldView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<InteractionView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boardwalk_case: Option<BoardwalkCaseView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stonebend_case: Option<StonebendCaseView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<RouteView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<InteriorSurfaceView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extraction_site: Option<ExtractionSiteView>,
    pub living_world: LivingWorldState,
    pub deep_pressure: DeepPressureView,
    pub party: PartyView,
    pub physical_exits: Vec<PhysicalExitView>,
    pub capabilities: Vec<IntentCapabilityView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeepPressureView {
    pub case_id: String,
    pub revision: u64,
    pub phase: DeepPressurePhase,
    pub journal: Vec<EvidenceJournalRecord>,
    pub integrated_case_count: usize,
    pub required_case_count: usize,
    pub missing_required_statement_count: usize,
    pub relationships: Vec<RelationshipMemory>,
    pub ready_for_settlement_support: bool,
    pub supported_settlement: Option<DeepPressureSettlementChoice>,
    pub production_under_review_available: bool,
    pub outcome: Option<DeepPressureOutcomeRecord>,
    pub aftermath: Option<DeepPressureAftermath>,
    pub present_people: Vec<DeepPressurePersonPresence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyMemberView {
    pub candidate_id: RecruitmentCandidateId,
    pub person_stable_id: String,
    pub display_name: String,
    pub role: String,
    pub continuity_id: String,
    pub availability: PartyMemberAvailability,
    pub field_action_id: String,
    pub is_selected: bool,
    pub is_lead: bool,
    pub agency_boundaries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecruitmentCandidateView {
    pub candidate_id: RecruitmentCandidateId,
    pub stable_id: String,
    pub person_stable_id: String,
    pub display_name: String,
    pub role: String,
    pub continuity_id: String,
    pub field_action_id: String,
    pub accepted_paths: Vec<RecruitmentPath>,
    pub present_on_current_map: bool,
    pub is_current_interaction: bool,
    pub recruited: bool,
    pub decision: Option<RecruitmentDecision>,
    pub decision_reason: Option<RecruitmentDecisionReason>,
    pub relationship_affinity: i8,
    pub relationship_reliability: i8,
    pub condition: super::CharacterCondition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyView {
    pub revision: u64,
    pub max_members: usize,
    pub member_count: usize,
    pub hueman_continuity_id: Option<String>,
    pub selected_continuity_id: String,
    pub lead_continuity_id: String,
    pub members: Vec<PartyMemberView>,
    pub candidates: Vec<RecruitmentCandidateView>,
    pub field_actions: Vec<super::FieldActionRecord>,
}

impl PartyView {
    #[must_use]
    pub fn from_runtime(runtime: &HollowGroveGameRuntime) -> Self {
        let party = runtime.party();
        let present = runtime
            .scheduled_people()
            .into_iter()
            .map(|presence| presence.person_id)
            .collect::<std::collections::BTreeSet<_>>();
        let members = party
            .companions
            .iter()
            .map(|member| {
                let actor = PartyActorId::Companion(member.candidate_id);
                PartyMemberView {
                    candidate_id: member.candidate_id,
                    person_stable_id: member.person_id.stable_id().into(),
                    display_name: member.person_id.display_name().into(),
                    role: member.candidate_id.role().into(),
                    continuity_id: member.continuity_id.clone(),
                    availability: member.availability,
                    field_action_id: member.field_action.stable_id().into(),
                    is_selected: party.selected == actor,
                    is_lead: party.lead == actor,
                    agency_boundaries: member.agency_boundaries.clone(),
                }
            })
            .collect();
        let candidates = RecruitmentCandidateId::ALL
            .into_iter()
            .map(|candidate| {
                let person = candidate.person();
                let memory = runtime
                    .deep_pressure()
                    .relationships
                    .get(&person)
                    .expect("every recruitment candidate has Deep Pressure memory");
                let decision = party.recruitment_decisions.get(&candidate);
                RecruitmentCandidateView {
                    candidate_id: candidate,
                    stable_id: candidate.stable_id().into(),
                    person_stable_id: person.stable_id().into(),
                    display_name: person.display_name().into(),
                    role: candidate.role().into(),
                    continuity_id: candidate.continuity_id().into(),
                    field_action_id: candidate.action().stable_id().into(),
                    accepted_paths: candidate.accepted_paths().to_vec(),
                    present_on_current_map: present.contains(&person),
                    is_current_interaction: runtime.active_interaction()
                        == Some(InteractionId::DeepPressurePerson(person)),
                    recruited: party.is_recruited(candidate),
                    decision: decision.map(|record| record.decision),
                    decision_reason: decision.map(|record| record.reason),
                    relationship_affinity: memory.affinity,
                    relationship_reliability: memory.reliability,
                    condition: memory.condition,
                }
            })
            .collect();
        Self {
            revision: party.revision,
            max_members: MAX_PARTY_MEMBERS,
            member_count: party.member_count(),
            hueman_continuity_id: party.hueman_continuity_id.clone(),
            selected_continuity_id: party
                .actor_continuity_id(party.selected)
                .unwrap_or_else(|| PartyActorId::Hueman.continuity_id())
                .into(),
            lead_continuity_id: party
                .actor_continuity_id(party.lead)
                .unwrap_or_else(|| PartyActorId::Hueman.continuity_id())
                .into(),
            members,
            candidates,
            field_actions: party.field_actions.clone(),
        }
    }
}

impl GameView {
    #[must_use]
    pub fn from_runtime(
        runtime: &HollowGroveGameRuntime,
        capabilities: Vec<IntentCapabilityView>,
    ) -> Self {
        Self {
            revision: runtime.revision().get(),
            identity_count: runtime.identity_count(),
            regional_event_count: runtime.regional().events().len(),
            hueman: runtime.hueman().map(BeingIdentityView::from_record),
            overworld: runtime.hueman_position().map(|position| {
                let map = map_definition(runtime.hueman_map());
                OverworldView {
                    map_id: map.id.as_str().into(),
                    width: MAP_WIDTH,
                    height: MAP_HEIGHT,
                    tile_size: MAP_TILE_SIZE,
                    tile_rows: map.projected_rows_with_cases(
                        runtime
                            .boardwalk_case()
                            .and_then(BoardwalkCase::committed_choice),
                        runtime
                            .stonebend_case()
                            .and_then(StonebendContinuityCase::committed_choice),
                    ),
                    player: position.into(),
                }
            }),
            interaction: runtime.active_interaction().map(|target| {
                InteractionView::from_target_and_campaigns(
                    target,
                    runtime.boardwalk_case(),
                    runtime.stonebend_case(),
                    Some(runtime.deep_pressure()),
                )
            }),
            boardwalk_case: runtime.boardwalk_case().map(BoardwalkCaseView::from_case),
            stonebend_case: runtime.stonebend_case().map(StonebendCaseView::from_case),
            route: runtime
                .hueman_position()
                .and_then(|_| RouteView::from_map(runtime.hueman_map())),
            surface: runtime
                .hueman_position()
                .and_then(|_| InteriorSurfaceView::from_map(runtime.hueman_map())),
            extraction_site: runtime
                .hueman_position()
                .and_then(|_| ExtractionSiteView::from_map(runtime.hueman_map())),
            living_world: runtime.living_world().clone(),
            deep_pressure: {
                let campaign = runtime.deep_pressure();
                DeepPressureView {
                    case_id: campaign.case_id.clone(),
                    revision: campaign.revision,
                    phase: campaign.phase(),
                    journal: campaign.journal.clone(),
                    integrated_case_count: campaign.integrated_resolutions.len(),
                    required_case_count: super::LivingCaseId::ALL.len(),
                    missing_required_statement_count: campaign.missing_required_statements().len(),
                    relationships: campaign.relationships.values().cloned().collect(),
                    ready_for_settlement_support: campaign.ready_for_settlement_support(),
                    supported_settlement: campaign.supported_settlement,
                    production_under_review_available: campaign.settlement_choice_available(
                        DeepPressureSettlementChoice::ProductionUnderReview,
                    ),
                    outcome: campaign.outcome.clone(),
                    aftermath: campaign
                        .outcome
                        .as_ref()
                        .map(|outcome| outcome.aftermath.clone()),
                    present_people: runtime
                        .hueman_position()
                        .map(|_| runtime.scheduled_people())
                        .unwrap_or_default(),
                }
            },
            party: PartyView::from_runtime(runtime),
            physical_exits: runtime
                .hueman_position()
                .map(|_| {
                    let exit = map_definition(runtime.hueman_map()).spawn;
                    runtime
                        .physical_exit_destinations()
                        .into_iter()
                        .map(|destination| PhysicalExitView {
                            exit_x: exit.x,
                            exit_y: exit.y,
                            destination_map_id: destination.as_str().into(),
                            destination_label: destination
                                .extraction()
                                .map(|site| site.display_name())
                                .or_else(|| {
                                    destination.surface().map(|surface| surface.display_name())
                                })
                                .or_else(|| destination.route().map(|route| route.display_name()))
                                .unwrap_or("Hollow Grove")
                                .into(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            capabilities,
        }
    }
}
