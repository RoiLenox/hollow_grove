use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    BondActivation, BondEvent, BondFormation, BondId, BondParticipant, BondTerm, BondValidation,
    CausalPosition, ConstitutionalEventId, ConstitutionalRuntime, ConstitutionalRuntimeError,
    EventMetadata, EvidenceRef, HouseDecision, HouseFunction, InitialCurrent,
    InstitutionalJurisdictionSnapshot, ObligationId, ParticipantId, ParticipantKind, PermissionId,
    RoleId, RuleSetId, Sign, SignedQuantity, UnitId, WaveId, WaveRecord, scenario_house_decision,
};
use crate::hollow_grove_contract::House;
use crate::world::geography::{ConstitutionalRouteId, ConstitutionalRouteVerb};
use crate::world::house_institutions::stonebend_constitution_id;
use crate::world::session::WorldSession;

use super::InteractionId;

pub const RETURNING_GOON_PARTICIPANT_ID: &str = "participant.boardwalk.returning-goon";
pub const GIMP_PARTICIPANT_ID: &str = "participant.boardwalk.gimp";
pub const PIMP_PARTICIPANT_ID: &str = "participant.boardwalk.pimp";
pub const BOARDWALK_PATRONAGE_BOND_ID: &str = "bond.boardwalk.pimp-patronage.case-a";
pub const BOARDWALK_GOON_BOND_ID: &str = "bond.boardwalk.returning-goon.case-a";
pub const BOARDWALK_LIMITED_COOPERATION_BOND_ID: &str = "bond.boardwalk.limited-cooperation.case-a";
pub const BOARDWALK_RETURN_CASE_ID: &str = "case.boardwalk.returning-goon.v1";
pub const RETURNING_GOON_BEING_ID: &str = "being.boardwalk.returning-goon.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkCaseId {
    ReturningGoonV1,
}

impl BoardwalkCaseId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturningGoonV1 => BOARDWALK_RETURN_CASE_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkBeingId {
    ReturningGoonV1,
}

impl BoardwalkBeingId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturningGoonV1 => RETURNING_GOON_BEING_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BoardwalkEvidence {
    VoluntaryDischarge,
    PimpOffer,
    HoeTestimony,
    GimpOffer,
    GoonTestimony,
    ReturningGoonBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HuemanFaculty {
    Reason,
    Memory,
    Imagination,
    Perception,
    Will,
}

impl HuemanFaculty {
    pub const ALL: [Self; 5] = [
        Self::Reason,
        Self::Memory,
        Self::Imagination,
        Self::Perception,
        Self::Will,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkChoice {
    PimpPatronage,
    GoonBond,
    LimitedCooperation,
    IndependentReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkOutcomeId {
    PimpPatronageV1,
    GoonBondV1,
    LimitedCooperationV1,
    IndependentReturnV1,
}

impl BoardwalkOutcomeId {
    #[must_use]
    pub const fn for_choice(choice: BoardwalkChoice) -> Self {
        match choice {
            BoardwalkChoice::PimpPatronage => Self::PimpPatronageV1,
            BoardwalkChoice::GoonBond => Self::GoonBondV1,
            BoardwalkChoice::LimitedCooperation => Self::LimitedCooperationV1,
            BoardwalkChoice::IndependentReturn => Self::IndependentReturnV1,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PimpPatronageV1 => "outcome.boardwalk.pimp-patronage.v1",
            Self::GoonBondV1 => "outcome.boardwalk.goon-bond.v1",
            Self::LimitedCooperationV1 => "outcome.boardwalk.limited-cooperation.v1",
            Self::IndependentReturnV1 => "outcome.boardwalk.independent-return.v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkAuthorityClass {
    VoluntaryPatronage,
    FiniteGoonBond,
    ScopedCooperation,
    CivicSelfDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkRelationshipKind {
    PimpPatronage,
    GoonBond,
    LimitedCooperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoardwalkCasePhase {
    Open,
    ReadyForSupport,
    SupportRecorded,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardwalkRelationshipCommit {
    pub kind: BoardwalkRelationshipKind,
    pub bond: BondId,
    pub formed_event: ConstitutionalEventId,
    pub validated_event: ConstitutionalEventId,
    pub activated_event: ConstitutionalEventId,
    pub term_end: CausalPosition,
}

pub type GoonBondCommit = BoardwalkRelationshipCommit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardwalkOutcomeRecord {
    pub id: BoardwalkOutcomeId,
    pub case: BoardwalkCaseId,
    pub subject: BoardwalkBeingId,
    pub choice: BoardwalkChoice,
    pub authority_class: BoardwalkAuthorityClass,
    pub jurisdiction: ConstitutionalRouteId,
    pub participants: Vec<ParticipantId>,
    pub dominant_verb: ConstitutionalRouteVerb,
    pub trigger: &'static str,
    pub evidence: Vec<BoardwalkEvidence>,
    pub faculty_uncertainties: Vec<&'static str>,
    pub player_support_is_nonbinding: bool,
    pub lawful_state_change: &'static str,
    pub relationship: Option<BoardwalkRelationshipCommit>,
    pub glaushouse_discharge_clearance: HouseDecision,
    pub flynt_recognition: HouseDecision,
    pub persistence_and_replay: &'static str,
    pub presentation: &'static str,
    pub failure_and_refusal: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardwalkCase {
    id: BoardwalkCaseId,
    returning_goon: BoardwalkBeingId,
    evidence: BTreeSet<BoardwalkEvidence>,
    faculties: BTreeSet<HuemanFaculty>,
    supported_choice: Option<BoardwalkChoice>,
    committed_choice: Option<BoardwalkChoice>,
    outcome: Option<BoardwalkOutcomeRecord>,
}

impl Default for BoardwalkCase {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardwalkCase {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: BoardwalkCaseId::ReturningGoonV1,
            returning_goon: BoardwalkBeingId::ReturningGoonV1,
            evidence: BTreeSet::new(),
            faculties: BTreeSet::new(),
            supported_choice: None,
            committed_choice: None,
            outcome: None,
        }
    }

    #[must_use]
    pub const fn id(&self) -> BoardwalkCaseId {
        self.id
    }

    #[must_use]
    pub const fn returning_goon(&self) -> BoardwalkBeingId {
        self.returning_goon
    }

    #[must_use]
    pub fn phase(&self) -> BoardwalkCasePhase {
        if self.committed_choice.is_some() {
            BoardwalkCasePhase::Resolved
        } else if self.supported_choice.is_some() {
            BoardwalkCasePhase::SupportRecorded
        } else if self.is_ready() {
            BoardwalkCasePhase::ReadyForSupport
        } else {
            BoardwalkCasePhase::Open
        }
    }

    #[must_use]
    pub fn evidence(&self) -> &BTreeSet<BoardwalkEvidence> {
        &self.evidence
    }

    #[must_use]
    pub fn faculties(&self) -> &BTreeSet<HuemanFaculty> {
        &self.faculties
    }

    #[must_use]
    pub const fn supported_choice(&self) -> Option<BoardwalkChoice> {
        self.supported_choice
    }

    #[must_use]
    pub const fn committed_choice(&self) -> Option<BoardwalkChoice> {
        self.committed_choice
    }

    #[must_use]
    pub fn goon_bond(&self) -> Option<&GoonBondCommit> {
        match self.outcome.as_ref() {
            Some(outcome) if outcome.choice == BoardwalkChoice::GoonBond => {
                outcome.relationship.as_ref()
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn outcome(&self) -> Option<&BoardwalkOutcomeRecord> {
        self.outcome.as_ref()
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.evidence.len() == 6 && self.faculties.len() == HuemanFaculty::ALL.len()
    }

    pub fn observe_interaction(&mut self, target: InteractionId) -> Option<BoardwalkEvidence> {
        let evidence = match target {
            InteractionId::BoardwalkDischargeAdvocate => {
                Some(BoardwalkEvidence::VoluntaryDischarge)
            }
            InteractionId::BoardwalkPimp => Some(BoardwalkEvidence::PimpOffer),
            InteractionId::BoardwalkHoeWitness => Some(BoardwalkEvidence::HoeTestimony),
            InteractionId::BoardwalkGimp => Some(BoardwalkEvidence::GimpOffer),
            InteractionId::BoardwalkGoonWitness => Some(BoardwalkEvidence::GoonTestimony),
            InteractionId::BoardwalkReturningGoon => Some(BoardwalkEvidence::ReturningGoonBoundary),
            _ => None,
        };
        if self.committed_choice.is_none()
            && let Some(value) = evidence
        {
            self.evidence.insert(value);
        }
        evidence
    }

    pub fn disclose_faculty(&mut self, faculty: HuemanFaculty) -> Result<(), BoardwalkCaseError> {
        if self.committed_choice.is_some() {
            return Err(BoardwalkCaseError::AlreadyResolved);
        }
        self.faculties.insert(faculty);
        Ok(())
    }

    pub fn support(&mut self, choice: BoardwalkChoice) -> Result<(), BoardwalkCaseError> {
        if self.committed_choice.is_some() {
            return Err(BoardwalkCaseError::AlreadyResolved);
        }
        if !self.is_ready() {
            return Err(BoardwalkCaseError::CaseNotReady);
        }
        self.supported_choice = Some(choice);
        Ok(())
    }

    pub fn commit_returning_goon_choice(
        &mut self,
        at: CausalPosition,
        rule_set: &RuleSetId,
        constitutional: &mut ConstitutionalRuntime,
    ) -> Result<BoardwalkChoice, BoardwalkCaseError> {
        self.commit_returning_goon_choice_with_authority(
            at,
            rule_set,
            constitutional,
            &WorldSession::canonical(),
        )
    }

    pub fn commit_returning_goon_choice_with_authority(
        &mut self,
        at: CausalPosition,
        rule_set: &RuleSetId,
        constitutional: &mut ConstitutionalRuntime,
        authority_world: &WorldSession,
    ) -> Result<BoardwalkChoice, BoardwalkCaseError> {
        if self.committed_choice.is_some() {
            return Err(BoardwalkCaseError::AlreadyResolved);
        }
        let choice = self
            .supported_choice
            .ok_or(BoardwalkCaseError::SupportRequired)?;
        self.outcome = Some(resolve_boardwalk_outcome(
            self,
            choice,
            at,
            rule_set,
            constitutional,
            authority_world,
        )?);
        self.committed_choice = Some(choice);
        Ok(choice)
    }
}

fn resolve_boardwalk_outcome(
    case: &BoardwalkCase,
    choice: BoardwalkChoice,
    at: CausalPosition,
    rule_set: &RuleSetId,
    runtime: &mut ConstitutionalRuntime,
    authority_world: &WorldSession,
) -> Result<BoardwalkOutcomeRecord, BoardwalkCaseError> {
    let authority_catalog = &authority_world.institutional().catalog;
    let key = choice_key(choice);
    let glaushouse_discharge_clearance = scenario_house_decision(
        authority_catalog,
        &format!("boardwalk.{key}.discharge-clearance"),
        HouseFunction::Clear,
        at.get(),
    )
    .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))?;
    let flynt_recognition = scenario_house_decision(
        authority_catalog,
        &format!("boardwalk.{key}.recognition"),
        HouseFunction::Recognize,
        at.get(),
    )
    .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))?;
    let relationship = match choice {
        BoardwalkChoice::IndependentReturn => None,
        _ => Some(form_active_boardwalk_bond(
            choice,
            at,
            rule_set,
            runtime,
            authority_world,
        )?),
    };
    let (
        authority_class,
        participants,
        lawful_state_change,
        presentation,
        faculty_uncertainties,
        failure_and_refusal,
    ) = outcome_terms(choice);
    Ok(BoardwalkOutcomeRecord {
        id: BoardwalkOutcomeId::for_choice(choice),
        case: case.id,
        subject: case.returning_goon,
        choice,
        authority_class,
        jurisdiction: ConstitutionalRouteId::Boardwalk,
        participants: participants
            .into_iter()
            .map(|participant| stable(participant, ParticipantId::new))
            .collect::<Result<_, _>>()?,
        dominant_verb: ConstitutionalRouteVerb::Return,
        trigger: "the Returning Goon commits after evidence and all five Hueman faculties are disclosed",
        evidence: case.evidence.iter().copied().collect(),
        faculty_uncertainties,
        player_support_is_nonbinding: true,
        lawful_state_change,
        relationship,
        glaushouse_discharge_clearance,
        flynt_recognition,
        persistence_and_replay: "the canonical choice event, outcome ID, authority snapshot, and optional Bond replay together",
        presentation,
        failure_and_refusal,
    })
}

fn form_active_boardwalk_bond(
    choice: BoardwalkChoice,
    at: CausalPosition,
    rule_set: &RuleSetId,
    runtime: &mut ConstitutionalRuntime,
    authority_world: &WorldSession,
) -> Result<GoonBondCommit, BoardwalkCaseError> {
    if at.get() == 0 {
        return Err(BoardwalkCaseError::CausalPositionRequired);
    }
    let authority_catalog = &authority_world.institutional().catalog;
    let specification =
        relationship_specification(choice).ok_or(BoardwalkCaseError::IndependentReturnHasNoBond)?;
    let bond = stable(specification.bond_id, BondId::new)?;
    let wave = stable(specification.wave_id, WaveId::new)?;
    let wave_position = CausalPosition::new(at.get() - 1);
    runtime.record_wave(WaveRecord {
        id: wave.clone(),
        origin: evidence("returning-goon-choice-wave")?,
        causal_position: wave_position,
    })?;

    let returning_goon = stable(RETURNING_GOON_PARTICIPANT_ID, ParticipantId::new)?;
    let gimp = stable(GIMP_PARTICIPANT_ID, ParticipantId::new)?;
    let pimp = stable(PIMP_PARTICIPANT_ID, ParticipantId::new)?;
    let current_unit = stable("unit.current", UnitId::new)?;
    let institution = stonebend_constitution_id();
    let term_end = CausalPosition::new(
        at.get()
            .checked_add(specification.duration)
            .ok_or(BoardwalkCaseError::CausalOverflow)?,
    );
    let formation = BondFormation {
        id: bond.clone(),
        initiating_wave: wave,
        governing_house: House::Stonebend,
        governing_institution: institution.clone(),
        jurisdiction: InstitutionalJurisdictionSnapshot::from_catalog(
            authority_catalog,
            &institution,
            at,
            vec![evidence("goon-bond-jurisdiction")?],
        )
        .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))?,
        parent_bonds: vec![],
        inheritance_evidence: vec![],
        participants: relationship_participants(specification.kind, &returning_goon, &pimp, &gimp)?,
        obligations: specification
            .obligations
            .iter()
            .map(|value| stable(value, ObligationId::new))
            .collect::<Result<_, _>>()?,
        permissions: specification
            .permissions
            .iter()
            .map(|value| stable(value, PermissionId::new))
            .collect::<Result<_, _>>()?,
        term: BondTerm::Finite { end: term_end },
        current_unit: current_unit.clone(),
        aura_unit: stable("unit.aura", UnitId::new)?,
        starting_current: vec![InitialCurrent {
            owner: returning_goon.clone(),
            custodian: match specification.kind {
                BoardwalkRelationshipKind::PimpPatronage => pimp,
                BoardwalkRelationshipKind::GoonBond => gimp,
                BoardwalkRelationshipKind::LimitedCooperation => returning_goon.clone(),
            },
            quantity: SignedQuantity::new(Sign::Positive, 1, current_unit)
                .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))?,
            evidence: vec![evidence(specification.current_evidence)?],
        }],
        initial_aura: vec![],
        evidence: vec![
            evidence("returning-goon-capacity")?,
            evidence("returning-goon-voluntary-choice")?,
            evidence(specification.term_evidence)?,
        ],
        stonebend_naming: scenario_house_decision(
            authority_catalog,
            &format!("boardwalk.{}.name", specification.key),
            HouseFunction::Name,
            at.get(),
        )
        .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))?,
    };

    let formed_event = stable(specification.formed_event, ConstitutionalEventId::new)?;
    runtime.append(
        bond.clone(),
        constitutional_metadata(formed_event.clone(), at, rule_set),
        BondEvent::Formed(formation),
    )?;

    let validated_event = stable(specification.validated_event, ConstitutionalEventId::new)?;
    runtime.append(
        bond.clone(),
        constitutional_metadata(validated_event.clone(), at, rule_set),
        BondEvent::Validated(BondValidation {
            sandmanor_proof: scenario_house_decision(
                authority_catalog,
                &format!("boardwalk.{}.prove", specification.key),
                HouseFunction::Prove,
                at.get(),
            )
            .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))?,
            evidence: vec![evidence(specification.proof_evidence)?],
        }),
    )?;

    let activated_event = stable(specification.activated_event, ConstitutionalEventId::new)?;
    runtime.append(
        bond.clone(),
        constitutional_metadata(activated_event.clone(), at, rule_set),
        BondEvent::Activated(BondActivation {
            evidence: vec![evidence(specification.activation_evidence)?],
        }),
    )?;

    Ok(BoardwalkRelationshipCommit {
        kind: specification.kind,
        bond,
        formed_event,
        validated_event,
        activated_event,
        term_end,
    })
}

#[derive(Debug, Clone, Copy)]
struct RelationshipSpecification {
    kind: BoardwalkRelationshipKind,
    key: &'static str,
    bond_id: &'static str,
    wave_id: &'static str,
    duration: u64,
    obligations: &'static [&'static str],
    permissions: &'static [&'static str],
    current_evidence: &'static str,
    term_evidence: &'static str,
    proof_evidence: &'static str,
    activation_evidence: &'static str,
    formed_event: &'static str,
    validated_event: &'static str,
    activated_event: &'static str,
}

fn relationship_specification(choice: BoardwalkChoice) -> Option<RelationshipSpecification> {
    match choice {
        BoardwalkChoice::PimpPatronage => Some(RelationshipSpecification {
            kind: BoardwalkRelationshipKind::PimpPatronage,
            key: "pimp-patronage",
            bond_id: BOARDWALK_PATRONAGE_BOND_ID,
            wave_id: "wave.boardwalk.pimp-patronage-choice",
            duration: 60,
            obligations: &[
                "obligation.boardwalk.patronage.disclose-opportunity",
                "obligation.boardwalk.patronage.disclose-compensation",
                "obligation.boardwalk.patronage.no-ownership",
                "obligation.boardwalk.patronage.no-role-or-sexual-inference",
                "obligation.boardwalk.patronage.preserve-independent-identity",
                "obligation.boardwalk.patronage.no-retaliation-for-exit",
            ],
            permissions: &[
                "permission.boardwalk.patronage.leave",
                "permission.boardwalk.patronage.revoke",
                "permission.boardwalk.patronage.challenge",
                "permission.boardwalk.patronage.refuse-opportunity",
            ],
            current_evidence: "pimp-patronage-returning-goon-retains-current",
            term_evidence: "pimp-patronage-finite-term",
            proof_evidence: "pimp-patronage-terms-proven-within-scope",
            activation_evidence: "returning-goon-accepted-voluntary-patronage",
            formed_event: "event.boardwalk.pimp-patronage.form",
            validated_event: "event.boardwalk.pimp-patronage.validate",
            activated_event: "event.boardwalk.pimp-patronage.activate",
        }),
        BoardwalkChoice::GoonBond => Some(RelationshipSpecification {
            kind: BoardwalkRelationshipKind::GoonBond,
            key: "goon-bond",
            bond_id: BOARDWALK_GOON_BOND_ID,
            wave_id: "wave.boardwalk.returning-goon-choice",
            duration: 100,
            obligations: &[
                "obligation.boardwalk.return-safe",
                "obligation.boardwalk.no-ownership",
                "obligation.boardwalk.escort",
                "obligation.boardwalk.mutual-defense",
                "obligation.boardwalk.public-return-appearance",
                "obligation.boardwalk.truthful-risk-disclosure",
            ],
            permissions: &[
                "permission.boardwalk.leave",
                "permission.boardwalk.challenge",
                "permission.boardwalk.refuse-unlawful-force",
            ],
            current_evidence: "goon-bond-starting-current",
            term_evidence: "goon-bond-finite-term",
            proof_evidence: "goon-bond-proof",
            activation_evidence: "returning-goon-accepted-active-bond",
            formed_event: "event.boardwalk.goon-bond.form",
            validated_event: "event.boardwalk.goon-bond.validate",
            activated_event: "event.boardwalk.goon-bond.activate",
        }),
        BoardwalkChoice::LimitedCooperation => Some(RelationshipSpecification {
            kind: BoardwalkRelationshipKind::LimitedCooperation,
            key: "limited-cooperation",
            bond_id: BOARDWALK_LIMITED_COOPERATION_BOND_ID,
            wave_id: "wave.boardwalk.limited-cooperation-choice",
            duration: 24,
            obligations: &[
                "obligation.boardwalk.cooperation.one-disclosed-job",
                "obligation.boardwalk.cooperation.disclose-risk-and-payment",
                "obligation.boardwalk.cooperation.mutual-safety",
                "obligation.boardwalk.cooperation.no-ownership",
                "obligation.boardwalk.cooperation.no-affiliation-inference",
                "obligation.boardwalk.cooperation.no-auto-renewal",
                "obligation.boardwalk.cooperation.stop-at-agreed-edge",
            ],
            permissions: &[
                "permission.boardwalk.cooperation.leave-at-edge",
                "permission.boardwalk.cooperation.challenge-scope",
                "permission.boardwalk.cooperation.refuse-scope-expansion",
            ],
            current_evidence: "limited-cooperation-self-custodied-current",
            term_evidence: "limited-cooperation-short-finite-term",
            proof_evidence: "limited-cooperation-scope-proven",
            activation_evidence: "returning-goon-accepted-one-scoped-job",
            formed_event: "event.boardwalk.limited-cooperation.form",
            validated_event: "event.boardwalk.limited-cooperation.validate",
            activated_event: "event.boardwalk.limited-cooperation.activate",
        }),
        BoardwalkChoice::IndependentReturn => None,
    }
}

fn relationship_participants(
    kind: BoardwalkRelationshipKind,
    returning_goon: &ParticipantId,
    pimp: &ParticipantId,
    gimp: &ParticipantId,
) -> Result<Vec<BondParticipant>, BoardwalkCaseError> {
    let returning_goon = BondParticipant {
        id: returning_goon.clone(),
        kind: ParticipantKind::Npc,
        roles: vec![stable("role.boardwalk.goon", RoleId::new)?],
    };
    let pimp = BondParticipant {
        id: pimp.clone(),
        kind: ParticipantKind::Npc,
        roles: vec![stable("role.boardwalk.pimp", RoleId::new)?],
    };
    let gimp = BondParticipant {
        id: gimp.clone(),
        kind: ParticipantKind::Npc,
        roles: vec![stable("role.boardwalk.gimp", RoleId::new)?],
    };
    Ok(match kind {
        BoardwalkRelationshipKind::PimpPatronage => vec![returning_goon, pimp],
        BoardwalkRelationshipKind::GoonBond => vec![returning_goon, gimp],
        BoardwalkRelationshipKind::LimitedCooperation => vec![returning_goon, pimp, gimp],
    })
}

type OutcomeTerms = (
    BoardwalkAuthorityClass,
    Vec<&'static str>,
    &'static str,
    &'static str,
    Vec<&'static str>,
    Vec<&'static str>,
);

fn outcome_terms(choice: BoardwalkChoice) -> OutcomeTerms {
    match choice {
        BoardwalkChoice::PimpPatronage => (
            BoardwalkAuthorityClass::VoluntaryPatronage,
            vec![RETURNING_GOON_PARTICIPANT_ID, PIMP_PARTICIPANT_ID],
            "a finite, revocable patronage Bond becomes active without creating Hoe identity, ownership, or permanent affiliation",
            "the Pimp and Returning Goon stand together while the exit route remains visibly open",
            vec![
                "future opportunity quality remains uncertain",
                "social dependence risk remains visible",
            ],
            vec![
                "the Returning Goon may refuse any offered opportunity",
                "withdrawal ends patronage scope without retaliation",
                "no conduct may be interpreted as sexual consent or Hoe affiliation",
            ],
        ),
        BoardwalkChoice::GoonBond => (
            BoardwalkAuthorityClass::FiniteGoonBond,
            vec![RETURNING_GOON_PARTICIPANT_ID, GIMP_PARTICIPANT_ID],
            "a finite Goon Bond becomes active with escort, mutual defense, disclosed risk, challenge, and exit",
            "the Returning Goon joins the Gimp's goons for the sealed term without becoming property",
            vec![
                "the Gimp's condition and future capacity remain reviewable",
                "threat conditions beyond the Boardwalk remain uncertain",
            ],
            vec![
                "the Returning Goon may refuse unlawful force",
                "either participant may challenge default or exceeded scope",
                "the Gimp's condition never converts custody into ownership",
            ],
        ),
        BoardwalkChoice::LimitedCooperation => (
            BoardwalkAuthorityClass::ScopedCooperation,
            vec![
                RETURNING_GOON_PARTICIPANT_ID,
                PIMP_PARTICIPANT_ID,
                GIMP_PARTICIPANT_ID,
            ],
            "one short, disclosed three-party job becomes active and cannot silently become affiliation",
            "the Returning Goon occupies the space between both sides until the agreed edge",
            vec![
                "the job's outcome remains uncertain",
                "neither faction's later offer is part of the agreement",
            ],
            vec![
                "scope expansion requires a new lawful agreement",
                "the Bond cannot auto-renew",
                "any participant may stop when safety or disclosed scope fails",
            ],
        ),
        BoardwalkChoice::IndependentReturn => (
            BoardwalkAuthorityClass::CivicSelfDirection,
            vec![RETURNING_GOON_PARTICIPANT_ID],
            "Flynt recognizes the return while the Returning Goon lawfully refuses patronage, Bond, affiliation, and service",
            "the Returning Goon stands alone in the public return lane with no claimant beside them",
            vec![
                "future employment and protection remain open",
                "independence does not imply isolation or incapacity",
            ],
            vec![
                "refusal creates no debt, retaliation, or adverse status",
                "witness does not become custody",
                "recognition does not become Title, ownership, or compelled affiliation",
            ],
        ),
    }
}

const fn choice_key(choice: BoardwalkChoice) -> &'static str {
    match choice {
        BoardwalkChoice::PimpPatronage => "pimp-patronage",
        BoardwalkChoice::GoonBond => "goon-bond",
        BoardwalkChoice::LimitedCooperation => "limited-cooperation",
        BoardwalkChoice::IndependentReturn => "independent-return",
    }
}

fn constitutional_metadata(
    id: ConstitutionalEventId,
    at: CausalPosition,
    rule_set: &RuleSetId,
) -> EventMetadata {
    EventMetadata {
        id,
        causal_position: at,
        rule_set: rule_set.clone(),
    }
}

fn evidence(key: &str) -> Result<EvidenceRef, BoardwalkCaseError> {
    EvidenceRef::new("gameplay", key)
        .map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))
}

fn stable<T>(
    value: &str,
    constructor: impl FnOnce(String) -> Result<T, crate::constitutional::ConstitutionalIdError>,
) -> Result<T, BoardwalkCaseError> {
    constructor(value.into()).map_err(|error| BoardwalkCaseError::Constitutional(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardwalkCaseError {
    CaseNotReady,
    SupportRequired,
    AlreadyResolved,
    CausalPositionRequired,
    CausalOverflow,
    IndependentReturnHasNoBond,
    Constitutional(String),
}

impl From<ConstitutionalRuntimeError> for BoardwalkCaseError {
    fn from(value: ConstitutionalRuntimeError) -> Self {
        Self::Constitutional(value.to_string())
    }
}

impl std::fmt::Display for BoardwalkCaseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Boardwalk case rejected action: {self:?}")
    }
}

impl std::error::Error for BoardwalkCaseError {}
