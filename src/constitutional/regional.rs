//! Region-bound Sandmanor Synthesis governed by constitutional authority.
//!
//! This aggregate reuses the existing typed Sandmanor lineage contract. It
//! adds execution, authority, evidence, regional assignment, persistence, and
//! replay without changing the recursion kernel or bounded Recipe engine.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hollow_grove_contract::House;
use crate::institution::{InstitutionCatalog, InstitutionId, SiteId};
use crate::lineage_contract::{
    SandmanorForm, SandmanorLineage, SandmanorTransitionError, validate_sandmanor_transition,
};
use crate::world::house_institutions::{glaushouse_medical_civilization_id, sandmen_id};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstitutionalRegion {
    /// Legacy schema spelling for the one geographic region displayed as
    /// `Aura Field`. Renaming this variant would break archived events.
    AuraFields,
    AuraBeach,
    AuraSea,
}

impl ConstitutionalRegion {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraFields => "Aura Field",
            Self::AuraBeach => "Aura Beach",
            Self::AuraSea => "Aura Sea",
        }
    }

    #[must_use]
    pub fn site_id(self) -> Option<SiteId> {
        // The plural Aura Field site ID predates the singular geographic lock.
        // It remains stable for persistence and never denotes multiple sites.
        let value = match self {
            Self::AuraFields => "site.sandmanor.aura-fields",
            Self::AuraBeach => "site.sandmanor.aura-beach",
            Self::AuraSea => return None,
        };
        Some(SiteId::new(value).expect("canonical regional site ID"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionalStandingKind {
    Established,
    Visitor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalJurisdictionSnapshot {
    pub region: ConstitutionalRegion,
    pub site: SiteId,
    pub institution: InstitutionId,
    pub house: House,
    pub observed_at: CausalPosition,
    pub evidence: Vec<EvidenceRef>,
}

impl RegionalJurisdictionSnapshot {
    pub fn from_catalog(
        catalog: &InstitutionCatalog,
        region: ConstitutionalRegion,
        observed_at: CausalPosition,
        evidence: Vec<EvidenceRef>,
    ) -> Result<Self, RegionalSynthesisError> {
        let site_id =
            region
                .site_id()
                .ok_or(RegionalSynthesisError::RegionCannotBePrimaryStanding(
                    region,
                ))?;
        let site = catalog
            .sites
            .iter()
            .find(|candidate| candidate.id == site_id)
            .ok_or_else(|| RegionalSynthesisError::MissingRegionalSite(site_id.clone()))?;
        let institution = site
            .controlled_by
            .clone()
            .ok_or_else(|| RegionalSynthesisError::UncontrolledRegionalSite(site_id.clone()))?;
        if institution != sandmen_id() || site.house != House::Sandmanor {
            return Err(RegionalSynthesisError::InvalidRegionalController {
                region,
                institution,
                house: site.house,
            });
        }
        if evidence.is_empty() {
            return Err(RegionalSynthesisError::MissingEvidence(
                "regional jurisdiction",
            ));
        }
        Ok(Self {
            region,
            site: site.id.clone(),
            institution: sandmen_id(),
            house: House::Sandmanor,
            observed_at,
            evidence,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectEvidence {
    pub subject: RegionalBeingId,
    pub reference: EvidenceRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalStanding {
    pub region: ConstitutionalRegion,
    pub kind: RegionalStandingKind,
    pub jurisdiction: RegionalJurisdictionSnapshot,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraFieldsDuty {
    TendAuraCrops,
    GuardFieldBoundary,
    CarryFieldLoad,
    MaintainFieldRoute,
    GuardHarvest,
    ProtectFieldWorker,
    StabilizeFieldCurrent,
}

impl AuraFieldsDuty {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TendAuraCrops => "tend Aura crops",
            Self::GuardFieldBoundary => "guard field boundaries",
            Self::CarryFieldLoad => "carry field loads",
            Self::MaintainFieldRoute => "open and maintain field routes",
            Self::GuardHarvest => "guard harvests",
            Self::ProtectFieldWorker => "protect field workers",
            Self::StabilizeFieldCurrent => "stabilize field Current",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraBeachDuty {
    RoamAuraBeach,
    PatrolShoreline,
    GuardAuraSeaAccess,
    WatchCoastalRoute,
    EscortTraveler,
    RecognizeHorizonChange,
    DefendCoastalIncursion,
    MaintainLandSeaBoundary,
}

impl AuraBeachDuty {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoamAuraBeach => "roam the Aura Beach",
            Self::PatrolShoreline => "patrol the shoreline",
            Self::GuardAuraSeaAccess => "guard Aura Sea access",
            Self::WatchCoastalRoute => "watch coastal routes",
            Self::EscortTraveler => "escort travelers",
            Self::RecognizeHorizonChange => "recognize horizon changes",
            Self::DefendCoastalIncursion => "defend against coastal incursions",
            Self::MaintainLandSeaBoundary => "maintain the land-sea boundary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraSeaGuardianshipDuty {
    GuardSeaAccess,
    WatchHorizon,
    DefendSeaBoundary,
    MaintainLandSeaPassage,
}

impl AuraSeaGuardianshipDuty {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuardSeaAccess => "guard access to the Aura Sea",
            Self::WatchHorizon => "watch the Aura Sea horizon",
            Self::DefendSeaBoundary => "defend the Aura Sea boundary",
            Self::MaintainLandSeaPassage => "maintain lawful land-sea passage",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionalFunction {
    AuraFieldsStewardshipAndDefense,
    AuraBeachPatrolAndAuraSeaGuardianship,
}

impl RegionalFunction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraFieldsStewardshipAndDefense => {
                "Aura Field stewardship, work, maintenance, and defense"
            }
            Self::AuraBeachPatrolAndAuraSeaGuardianship => {
                "Aura Beach patrol and Aura Sea guardianship"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraFieldsStewardship {
    pub steward: RegionalBeingId,
    pub region: ConstitutionalRegion,
    pub duties: BTreeSet<AuraFieldsDuty>,
    pub authority: HouseDecisionId,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraBeachOccupation {
    pub occupant: RegionalBeingId,
    pub region: ConstitutionalRegion,
    pub duties: BTreeSet<AuraBeachDuty>,
    pub authority: HouseDecisionId,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraSeaGuardianship {
    pub guardian: RegionalBeingId,
    pub region: ConstitutionalRegion,
    pub duties: BTreeSet<AuraSeaGuardianshipDuty>,
    pub authority: HouseDecisionId,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionalAssignment {
    Minotaur {
        stewardship: AuraFieldsStewardship,
    },
    Centaur {
        beach_occupation: AuraBeachOccupation,
        sea_guardianship: AuraSeaGuardianship,
    },
}

impl RegionalAssignment {
    #[must_use]
    pub const fn function(&self) -> RegionalFunction {
        match self {
            Self::Minotaur { .. } => RegionalFunction::AuraFieldsStewardshipAndDefense,
            Self::Centaur { .. } => RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalLineageEntry {
    pub being: RegionalBeingId,
    pub form: SandmanorForm,
    pub synthesis: Option<RegionalSynthesisId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionalBeingStatus {
    Active,
    SynthesizedInto(RegionalBeingId),
    Tombstoned(TombstoneId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalBeingRecord {
    pub id: RegionalBeingId,
    pub form: SandmanorForm,
    pub lineage: SandmanorLineage,
    pub predecessor: Option<RegionalBeingId>,
    pub lineage_history: Vec<RegionalLineageEntry>,
    pub standing: RegionalStanding,
    pub assignment: Option<RegionalAssignment>,
    pub status: RegionalBeingStatus,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalBeingRegistration {
    pub id: RegionalBeingId,
    pub form: SandmanorForm,
    pub standing: RegionalStanding,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalSynthesisPrerequisites {
    pub standing: SubjectEvidence,
    pub lineage: SubjectEvidence,
    pub readiness: SubjectEvidence,
    pub constitutional_rule: SubjectEvidence,
    pub supporting: Vec<SubjectEvidence>,
}

impl RegionalSynthesisPrerequisites {
    fn all(&self) -> impl Iterator<Item = &SubjectEvidence> {
        [
            &self.standing,
            &self.lineage,
            &self.readiness,
            &self.constitutional_rule,
        ]
        .into_iter()
        .chain(self.supporting.iter())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalSynthesisAuthority {
    pub sandmanor_proof: HouseDecision,
    pub glaushouse_resolution: HouseDecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionalSynthesisRule {
    GnomeToMinotaurAuraFields,
    ElfToCentaurAuraBeach,
}

impl RegionalSynthesisRule {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GnomeToMinotaurAuraFields => "regional.gnome-minotaur.aura-fields.v1",
            Self::ElfToCentaurAuraBeach => "regional.elf-centaur.aura-beach.v1",
        }
    }

    #[must_use]
    pub const fn predecessor(self) -> SandmanorForm {
        match self {
            Self::GnomeToMinotaurAuraFields => SandmanorForm::Gnome,
            Self::ElfToCentaurAuraBeach => SandmanorForm::Elf,
        }
    }

    #[must_use]
    pub const fn result(self) -> SandmanorForm {
        match self {
            Self::GnomeToMinotaurAuraFields => SandmanorForm::Minotaur,
            Self::ElfToCentaurAuraBeach => SandmanorForm::Centaur,
        }
    }

    #[must_use]
    pub const fn region(self) -> ConstitutionalRegion {
        match self {
            Self::GnomeToMinotaurAuraFields => ConstitutionalRegion::AuraFields,
            Self::ElfToCentaurAuraBeach => ConstitutionalRegion::AuraBeach,
        }
    }

    #[must_use]
    pub const fn function(self) -> RegionalFunction {
        match self {
            Self::GnomeToMinotaurAuraFields => RegionalFunction::AuraFieldsStewardshipAndDefense,
            Self::ElfToCentaurAuraBeach => RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship,
        }
    }

    pub fn for_transition(
        predecessor: SandmanorForm,
        result: SandmanorForm,
    ) -> Result<Self, RegionalSynthesisError> {
        match (predecessor, result) {
            (SandmanorForm::Gnome, SandmanorForm::Minotaur) => Ok(Self::GnomeToMinotaurAuraFields),
            (SandmanorForm::Elf, SandmanorForm::Centaur) => Ok(Self::ElfToCentaurAuraBeach),
            _ => {
                validate_sandmanor_transition(predecessor.frame(), result.frame())
                    .map_err(RegionalSynthesisError::Lineage)?;
                Err(RegionalSynthesisError::UnratifiedRegionalRule {
                    predecessor,
                    result,
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalSynthesisCommand {
    pub id: RegionalSynthesisId,
    pub predecessor: RegionalBeingId,
    pub result: RegionalBeingId,
    pub expected_predecessor_form: SandmanorForm,
    pub requested_result_form: SandmanorForm,
    pub requested_region: ConstitutionalRegion,
    pub requested_function: RegionalFunction,
    pub prerequisites: RegionalSynthesisPrerequisites,
    pub authority: RegionalSynthesisAuthority,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalSynthesisRecord {
    pub command: RegionalSynthesisCommand,
    pub rule: RegionalSynthesisRule,
    pub result: RegionalBeingRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalTombstoneRecord {
    pub being: RegionalBeingId,
    pub tombstone: TombstoneId,
    pub evidence: Vec<SubjectEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Event payloads remain direct values so replay equality and the public event
// contract do not acquire variant-specific allocation semantics.
#[allow(clippy::large_enum_variant)]
pub enum RegionalEvent {
    BeingRegistered(RegionalBeingRegistration),
    SynthesisCompleted(RegionalSynthesisRecord),
    BeingTombstoned(RegionalTombstoneRecord),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalEventEnvelope {
    pub id: RegionalEventId,
    pub sequence: u64,
    pub causal_position: CausalPosition,
    pub rule_set: RuleSetId,
    pub payload: RegionalEvent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalEventMetadata {
    pub id: RegionalEventId,
    pub causal_position: CausalPosition,
    pub rule_set: RuleSetId,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegionalSynthesisRuntime {
    events: Vec<RegionalEventEnvelope>,
    beings: BTreeMap<RegionalBeingId, RegionalBeingRecord>,
    event_ids: BTreeSet<RegionalEventId>,
    synthesis_ids: BTreeSet<RegionalSynthesisId>,
    decision_ids: BTreeSet<HouseDecisionId>,
}

impl RegionalSynthesisRuntime {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_being(
        &mut self,
        metadata: RegionalEventMetadata,
        registration: RegionalBeingRegistration,
    ) -> Result<&RegionalEventEnvelope, RegionalSynthesisError> {
        let payload = RegionalEvent::BeingRegistered(registration);
        if let Some(position) = self.retry_position(&metadata, &payload)? {
            return Ok(&self.events[position]);
        }
        self.validate_metadata(&metadata)?;
        let RegionalEvent::BeingRegistered(registration) = &payload else {
            unreachable!()
        };
        self.validate_registration(registration, metadata.causal_position)?;
        let record = RegionalBeingRecord {
            id: registration.id.clone(),
            form: registration.form,
            lineage: registration.form.lineage(),
            predecessor: None,
            lineage_history: vec![RegionalLineageEntry {
                being: registration.id.clone(),
                form: registration.form,
                synthesis: None,
            }],
            standing: registration.standing.clone(),
            assignment: None,
            status: RegionalBeingStatus::Active,
            evidence: registration.evidence.clone(),
        };
        let envelope = self.envelope(metadata, payload)?;
        self.event_ids.insert(envelope.id.clone());
        self.beings.insert(record.id.clone(), record);
        self.events.push(envelope);
        Ok(self.events.last().expect("regional event was appended"))
    }

    pub fn synthesize(
        &mut self,
        metadata: RegionalEventMetadata,
        command: RegionalSynthesisCommand,
    ) -> Result<&RegionalEventEnvelope, RegionalSynthesisError> {
        if let Some(position) = self.events.iter().position(|event| {
            event.id == metadata.id
                && matches!(
                    &event.payload,
                    RegionalEvent::SynthesisCompleted(record) if record.command == command
                )
                && event.causal_position == metadata.causal_position
                && event.rule_set == metadata.rule_set
        }) {
            return Ok(&self.events[position]);
        }
        if self.event_ids.contains(&metadata.id) {
            return Err(RegionalSynthesisError::EventIdConflict(metadata.id));
        }
        self.validate_metadata(&metadata)?;
        if self.synthesis_ids.contains(&command.id) {
            return Err(RegionalSynthesisError::SynthesisIdConflict(command.id));
        }

        let predecessor = self
            .beings
            .get(&command.predecessor)
            .ok_or_else(|| RegionalSynthesisError::UnknownBeing(command.predecessor.clone()))?
            .clone();
        let record = self.reduce_synthesis(&predecessor, command, metadata.causal_position)?;
        let payload = RegionalEvent::SynthesisCompleted(record.clone());
        let envelope = self.envelope(metadata, payload)?;

        let predecessor_record = self
            .beings
            .get_mut(&record.command.predecessor)
            .expect("validated predecessor must remain present");
        predecessor_record.status = RegionalBeingStatus::SynthesizedInto(record.result.id.clone());
        self.decision_ids
            .insert(record.command.authority.sandmanor_proof.id.clone());
        self.decision_ids
            .insert(record.command.authority.glaushouse_resolution.id.clone());
        self.synthesis_ids.insert(record.command.id.clone());
        self.event_ids.insert(envelope.id.clone());
        self.beings
            .insert(record.result.id.clone(), record.result.clone());
        self.events.push(envelope);
        Ok(self.events.last().expect("regional event was appended"))
    }

    pub fn tombstone_being(
        &mut self,
        metadata: RegionalEventMetadata,
        record: RegionalTombstoneRecord,
    ) -> Result<&RegionalEventEnvelope, RegionalSynthesisError> {
        let payload = RegionalEvent::BeingTombstoned(record.clone());
        if let Some(position) = self.retry_position(&metadata, &payload)? {
            return Ok(&self.events[position]);
        }
        self.validate_metadata(&metadata)?;
        validate_subject_evidence(&record.being, &record.evidence, "regional Tombstone")?;
        let being = self
            .beings
            .get(&record.being)
            .ok_or_else(|| RegionalSynthesisError::UnknownBeing(record.being.clone()))?;
        if !matches!(being.status, RegionalBeingStatus::Active) {
            return Err(RegionalSynthesisError::BeingNotActive(record.being));
        }
        let envelope = self.envelope(metadata, payload)?;
        self.beings
            .get_mut(&record.being)
            .expect("validated being must remain present")
            .status = RegionalBeingStatus::Tombstoned(record.tombstone);
        self.event_ids.insert(envelope.id.clone());
        self.events.push(envelope);
        Ok(self.events.last().expect("regional event was appended"))
    }

    pub fn replay(
        events: impl IntoIterator<Item = RegionalEventEnvelope>,
    ) -> Result<Self, RegionalSynthesisError> {
        let mut runtime = Self::new();
        for expected in events {
            let metadata = RegionalEventMetadata {
                id: expected.id.clone(),
                causal_position: expected.causal_position,
                rule_set: expected.rule_set.clone(),
            };
            let actual = match expected.payload.clone() {
                RegionalEvent::BeingRegistered(registration) => {
                    runtime.register_being(metadata, registration)?
                }
                RegionalEvent::SynthesisCompleted(record) => {
                    let actual = runtime.synthesize(metadata, record.command)?;
                    if actual.payload != expected.payload {
                        return Err(RegionalSynthesisError::ReplayDivergence(expected.id));
                    }
                    actual
                }
                RegionalEvent::BeingTombstoned(record) => {
                    runtime.tombstone_being(metadata, record)?
                }
            };
            if actual.sequence != expected.sequence || actual.payload != expected.payload {
                return Err(RegionalSynthesisError::ReplayDivergence(expected.id));
            }
        }
        Ok(runtime)
    }

    #[must_use]
    pub fn events(&self) -> &[RegionalEventEnvelope] {
        &self.events
    }

    #[must_use]
    pub fn being(&self, id: &RegionalBeingId) -> Option<&RegionalBeingRecord> {
        self.beings.get(id)
    }

    #[must_use]
    pub fn synthesis(&self, id: &RegionalSynthesisId) -> Option<&RegionalSynthesisRecord> {
        self.events.iter().find_map(|event| match &event.payload {
            RegionalEvent::SynthesisCompleted(record) if &record.command.id == id => Some(record),
            _ => None,
        })
    }

    #[must_use]
    pub fn stewardship(&self, id: &RegionalBeingId) -> Option<&AuraFieldsStewardship> {
        match self.beings.get(id)?.assignment.as_ref()? {
            RegionalAssignment::Minotaur { stewardship } => Some(stewardship),
            RegionalAssignment::Centaur { .. } => None,
        }
    }

    #[must_use]
    pub fn beach_occupation(&self, id: &RegionalBeingId) -> Option<&AuraBeachOccupation> {
        match self.beings.get(id)?.assignment.as_ref()? {
            RegionalAssignment::Centaur {
                beach_occupation, ..
            } => Some(beach_occupation),
            RegionalAssignment::Minotaur { .. } => None,
        }
    }

    #[must_use]
    pub fn guardianship(&self, id: &RegionalBeingId) -> Option<&AuraSeaGuardianship> {
        match self.beings.get(id)?.assignment.as_ref()? {
            RegionalAssignment::Centaur {
                sea_guardianship, ..
            } => Some(sea_guardianship),
            RegionalAssignment::Minotaur { .. } => None,
        }
    }

    #[must_use]
    pub fn lineage(&self, id: &RegionalBeingId) -> Option<&[RegionalLineageEntry]> {
        Some(&self.beings.get(id)?.lineage_history)
    }

    /// Verifies a read-only Aura Field authority claim against reducer state.
    pub fn require_stewardship(
        &self,
        id: &RegionalBeingId,
    ) -> Result<&AuraFieldsStewardship, RegionalSynthesisError> {
        self.beings
            .get(id)
            .ok_or_else(|| RegionalSynthesisError::UnknownBeing(id.clone()))?;
        self.stewardship(id)
            .ok_or_else(|| RegionalSynthesisError::AssignmentNotHeld {
                being: id.clone(),
                assignment: "Aura Field stewardship",
            })
    }

    /// Verifies a read-only Aura Sea authority claim against reducer state.
    pub fn require_guardianship(
        &self,
        id: &RegionalBeingId,
    ) -> Result<&AuraSeaGuardianship, RegionalSynthesisError> {
        self.beings
            .get(id)
            .ok_or_else(|| RegionalSynthesisError::UnknownBeing(id.clone()))?;
        self.guardianship(id)
            .ok_or_else(|| RegionalSynthesisError::AssignmentNotHeld {
                being: id.clone(),
                assignment: "Aura Sea guardianship",
            })
    }

    fn reduce_synthesis(
        &self,
        predecessor: &RegionalBeingRecord,
        command: RegionalSynthesisCommand,
        at: CausalPosition,
    ) -> Result<RegionalSynthesisRecord, RegionalSynthesisError> {
        if command.predecessor == command.result {
            return Err(RegionalSynthesisError::ResultReusesPredecessorIdentity);
        }
        if self.beings.contains_key(&command.result) {
            return Err(RegionalSynthesisError::BeingIdConflict(command.result));
        }
        if predecessor.status != RegionalBeingStatus::Active {
            return Err(RegionalSynthesisError::BeingNotActive(
                predecessor.id.clone(),
            ));
        }
        if predecessor.form != command.expected_predecessor_form {
            return Err(RegionalSynthesisError::PredecessorFormMismatch {
                expected: command.expected_predecessor_form,
                actual: predecessor.form,
            });
        }
        let rule =
            RegionalSynthesisRule::for_transition(predecessor.form, command.requested_result_form)?;
        validate_sandmanor_transition(predecessor.form.frame(), rule.result().frame())
            .map_err(RegionalSynthesisError::Lineage)?;
        if command.requested_region != rule.region() {
            return Err(RegionalSynthesisError::WrongSynthesisRegion {
                required: rule.region(),
                actual: command.requested_region,
            });
        }
        if predecessor.standing.region != rule.region()
            || predecessor.standing.kind != RegionalStandingKind::Established
        {
            return Err(RegionalSynthesisError::InsufficientRegionalStanding {
                being: predecessor.id.clone(),
                required: rule.region(),
                actual: predecessor.standing.region,
                kind: predecessor.standing.kind,
            });
        }
        validate_jurisdiction(&predecessor.standing.jurisdiction, at)?;
        if command.requested_function != rule.function() {
            return Err(RegionalSynthesisError::WrongRegionalFunction {
                required: rule.function(),
                actual: command.requested_function,
            });
        }
        validate_authority(&command.authority, at, &self.decision_ids)?;
        validate_subject_evidence(
            &predecessor.id,
            &predecessor.standing.evidence,
            "regional standing",
        )?;
        let prerequisite_evidence: Vec<_> = command.prerequisites.all().cloned().collect();
        validate_subject_evidence(
            &predecessor.id,
            &prerequisite_evidence,
            "regional Synthesis prerequisites",
        )?;
        validate_subject_evidence(&predecessor.id, &command.evidence, "regional Synthesis")?;

        let mut lineage_history = predecessor.lineage_history.clone();
        lineage_history.push(RegionalLineageEntry {
            being: command.result.clone(),
            form: rule.result(),
            synthesis: Some(command.id.clone()),
        });
        if lineage_history.first().map(|entry| entry.being.as_str())
            != predecessor
                .lineage_history
                .first()
                .map(|entry| entry.being.as_str())
        {
            return Err(RegionalSynthesisError::LineageErased);
        }

        let assignment = canonical_assignment(rule, &command);
        let result = RegionalBeingRecord {
            id: command.result.clone(),
            form: rule.result(),
            lineage: predecessor.lineage,
            predecessor: Some(predecessor.id.clone()),
            lineage_history,
            standing: RegionalStanding {
                region: rule.region(),
                kind: RegionalStandingKind::Established,
                jurisdiction: predecessor.standing.jurisdiction.clone(),
                evidence: predecessor.standing.evidence.clone(),
            },
            assignment: Some(assignment),
            status: RegionalBeingStatus::Active,
            evidence: command.evidence.clone(),
        };
        validate_result(&result, rule)?;
        Ok(RegionalSynthesisRecord {
            command,
            rule,
            result,
        })
    }

    fn validate_registration(
        &self,
        registration: &RegionalBeingRegistration,
        at: CausalPosition,
    ) -> Result<(), RegionalSynthesisError> {
        if self.beings.contains_key(&registration.id) {
            return Err(RegionalSynthesisError::BeingIdConflict(
                registration.id.clone(),
            ));
        }
        if !matches!(registration.form, SandmanorForm::Gnome | SandmanorForm::Elf) {
            return Err(RegionalSynthesisError::OriginRegistrationRequired(
                registration.form,
            ));
        }
        validate_jurisdiction(&registration.standing.jurisdiction, at)?;
        if registration.standing.region != registration.standing.jurisdiction.region {
            return Err(RegionalSynthesisError::StandingJurisdictionMismatch);
        }
        validate_subject_evidence(
            &registration.id,
            &registration.standing.evidence,
            "regional standing",
        )?;
        validate_subject_evidence(
            &registration.id,
            &registration.evidence,
            "regional Being registration",
        )?;
        Ok(())
    }

    fn retry_position(
        &self,
        metadata: &RegionalEventMetadata,
        payload: &RegionalEvent,
    ) -> Result<Option<usize>, RegionalSynthesisError> {
        if let Some(position) = self.events.iter().position(|event| event.id == metadata.id) {
            let existing = &self.events[position];
            if existing.causal_position == metadata.causal_position
                && existing.rule_set == metadata.rule_set
                && &existing.payload == payload
            {
                Ok(Some(position))
            } else {
                Err(RegionalSynthesisError::EventIdConflict(metadata.id.clone()))
            }
        } else {
            Ok(None)
        }
    }

    fn validate_metadata(
        &self,
        metadata: &RegionalEventMetadata,
    ) -> Result<(), RegionalSynthesisError> {
        if let Some(last) = self.events.last() {
            if metadata.causal_position < last.causal_position {
                return Err(RegionalSynthesisError::CausalRegression {
                    previous: last.causal_position,
                    actual: metadata.causal_position,
                });
            }
            if metadata.rule_set != last.rule_set {
                return Err(RegionalSynthesisError::RuleSetMismatch {
                    expected: last.rule_set.clone(),
                    actual: metadata.rule_set.clone(),
                });
            }
        }
        Ok(())
    }

    fn envelope(
        &self,
        metadata: RegionalEventMetadata,
        payload: RegionalEvent,
    ) -> Result<RegionalEventEnvelope, RegionalSynthesisError> {
        let sequence = u64::try_from(self.events.len())
            .map_err(|_| RegionalSynthesisError::SequenceOverflow)?;
        Ok(RegionalEventEnvelope {
            id: metadata.id,
            sequence,
            causal_position: metadata.causal_position,
            rule_set: metadata.rule_set,
            payload,
        })
    }
}

fn validate_jurisdiction(
    jurisdiction: &RegionalJurisdictionSnapshot,
    at: CausalPosition,
) -> Result<(), RegionalSynthesisError> {
    let required_site = jurisdiction.region.site_id().ok_or(
        RegionalSynthesisError::RegionCannotBePrimaryStanding(jurisdiction.region),
    )?;
    if jurisdiction.site != required_site
        || jurisdiction.institution != sandmen_id()
        || jurisdiction.house != House::Sandmanor
    {
        return Err(RegionalSynthesisError::StandingJurisdictionMismatch);
    }
    if jurisdiction.observed_at > at {
        return Err(RegionalSynthesisError::JurisdictionFromFuture);
    }
    if jurisdiction.evidence.is_empty() {
        return Err(RegionalSynthesisError::MissingEvidence(
            "regional jurisdiction",
        ));
    }
    Ok(())
}

fn validate_authority(
    authority: &RegionalSynthesisAuthority,
    at: CausalPosition,
    prior_decisions: &BTreeSet<HouseDecisionId>,
) -> Result<(), RegionalSynthesisError> {
    authority
        .sandmanor_proof
        .require_accepted(HouseFunction::Prove)
        .map_err(RegionalSynthesisError::HouseLaw)?;
    authority
        .glaushouse_resolution
        .require_accepted(HouseFunction::Resolve)
        .map_err(RegionalSynthesisError::HouseLaw)?;
    if authority.sandmanor_proof.authority.institution.as_ref() != Some(&sandmen_id()) {
        return Err(RegionalSynthesisError::WrongAuthorityInstitution {
            function: HouseFunction::Prove,
            required: sandmen_id(),
            actual: authority.sandmanor_proof.authority.institution.clone(),
        });
    }
    let glaushouse = glaushouse_medical_civilization_id();
    if authority
        .glaushouse_resolution
        .authority
        .institution
        .as_ref()
        != Some(&glaushouse)
    {
        return Err(RegionalSynthesisError::WrongAuthorityInstitution {
            function: HouseFunction::Resolve,
            required: glaushouse,
            actual: authority
                .glaushouse_resolution
                .authority
                .institution
                .clone(),
        });
    }
    if authority.sandmanor_proof.causal_position > at
        || authority.glaushouse_resolution.causal_position > at
    {
        return Err(RegionalSynthesisError::AuthorityFromFuture);
    }
    if authority.sandmanor_proof.id == authority.glaushouse_resolution.id
        || prior_decisions.contains(&authority.sandmanor_proof.id)
        || prior_decisions.contains(&authority.glaushouse_resolution.id)
    {
        return Err(RegionalSynthesisError::AuthorityDecisionConflict);
    }
    Ok(())
}

fn validate_subject_evidence(
    subject: &RegionalBeingId,
    evidence: &[SubjectEvidence],
    context: &'static str,
) -> Result<(), RegionalSynthesisError> {
    if evidence.is_empty() {
        return Err(RegionalSynthesisError::MissingEvidence(context));
    }
    if let Some(mismatch) = evidence.iter().find(|entry| &entry.subject != subject) {
        return Err(RegionalSynthesisError::EvidenceSubjectMismatch {
            expected: subject.clone(),
            actual: mismatch.subject.clone(),
        });
    }
    Ok(())
}

fn canonical_assignment(
    rule: RegionalSynthesisRule,
    command: &RegionalSynthesisCommand,
) -> RegionalAssignment {
    match rule {
        RegionalSynthesisRule::GnomeToMinotaurAuraFields => RegionalAssignment::Minotaur {
            stewardship: AuraFieldsStewardship {
                steward: command.result.clone(),
                region: ConstitutionalRegion::AuraFields,
                duties: BTreeSet::from([
                    AuraFieldsDuty::TendAuraCrops,
                    AuraFieldsDuty::GuardFieldBoundary,
                    AuraFieldsDuty::CarryFieldLoad,
                    AuraFieldsDuty::MaintainFieldRoute,
                    AuraFieldsDuty::GuardHarvest,
                    AuraFieldsDuty::ProtectFieldWorker,
                    AuraFieldsDuty::StabilizeFieldCurrent,
                ]),
                authority: command.authority.glaushouse_resolution.id.clone(),
                evidence: command.evidence.clone(),
            },
        },
        RegionalSynthesisRule::ElfToCentaurAuraBeach => RegionalAssignment::Centaur {
            beach_occupation: AuraBeachOccupation {
                occupant: command.result.clone(),
                region: ConstitutionalRegion::AuraBeach,
                duties: BTreeSet::from([
                    AuraBeachDuty::RoamAuraBeach,
                    AuraBeachDuty::PatrolShoreline,
                    AuraBeachDuty::GuardAuraSeaAccess,
                    AuraBeachDuty::WatchCoastalRoute,
                    AuraBeachDuty::EscortTraveler,
                    AuraBeachDuty::RecognizeHorizonChange,
                    AuraBeachDuty::DefendCoastalIncursion,
                    AuraBeachDuty::MaintainLandSeaBoundary,
                ]),
                authority: command.authority.glaushouse_resolution.id.clone(),
                evidence: command.evidence.clone(),
            },
            sea_guardianship: AuraSeaGuardianship {
                guardian: command.result.clone(),
                region: ConstitutionalRegion::AuraSea,
                duties: BTreeSet::from([
                    AuraSeaGuardianshipDuty::GuardSeaAccess,
                    AuraSeaGuardianshipDuty::WatchHorizon,
                    AuraSeaGuardianshipDuty::DefendSeaBoundary,
                    AuraSeaGuardianshipDuty::MaintainLandSeaPassage,
                ]),
                authority: command.authority.glaushouse_resolution.id.clone(),
                evidence: command.evidence.clone(),
            },
        },
    }
}

fn validate_result(
    result: &RegionalBeingRecord,
    rule: RegionalSynthesisRule,
) -> Result<(), RegionalSynthesisError> {
    if result.form != rule.result()
        || result.lineage != rule.result().lineage()
        || result.predecessor.is_none()
        || result.lineage_history.len() < 2
    {
        return Err(RegionalSynthesisError::InvalidSynthesisResult);
    }
    match (&result.assignment, rule) {
        (
            Some(RegionalAssignment::Minotaur { stewardship }),
            RegionalSynthesisRule::GnomeToMinotaurAuraFields,
        ) if stewardship.steward == result.id
            && stewardship.region == ConstitutionalRegion::AuraFields
            && stewardship.duties.len() == 7 =>
        {
            Ok(())
        }
        (
            Some(RegionalAssignment::Centaur {
                beach_occupation,
                sea_guardianship,
            }),
            RegionalSynthesisRule::ElfToCentaurAuraBeach,
        ) if beach_occupation.occupant == result.id
            && beach_occupation.region == ConstitutionalRegion::AuraBeach
            && beach_occupation.duties.len() == 8
            && sea_guardianship.guardian == result.id
            && sea_guardianship.region == ConstitutionalRegion::AuraSea
            && sea_guardianship.duties.len() == 4 =>
        {
            Ok(())
        }
        _ => Err(RegionalSynthesisError::InvalidRegionalAssignment),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionalSynthesisError {
    MissingRegionalSite(SiteId),
    UncontrolledRegionalSite(SiteId),
    InvalidRegionalController {
        region: ConstitutionalRegion,
        institution: InstitutionId,
        house: House,
    },
    RegionCannotBePrimaryStanding(ConstitutionalRegion),
    StandingJurisdictionMismatch,
    JurisdictionFromFuture,
    MissingEvidence(&'static str),
    EvidenceSubjectMismatch {
        expected: RegionalBeingId,
        actual: RegionalBeingId,
    },
    EventIdConflict(RegionalEventId),
    SynthesisIdConflict(RegionalSynthesisId),
    BeingIdConflict(RegionalBeingId),
    UnknownBeing(RegionalBeingId),
    BeingNotActive(RegionalBeingId),
    OriginRegistrationRequired(SandmanorForm),
    ResultReusesPredecessorIdentity,
    PredecessorFormMismatch {
        expected: SandmanorForm,
        actual: SandmanorForm,
    },
    Lineage(SandmanorTransitionError),
    UnratifiedRegionalRule {
        predecessor: SandmanorForm,
        result: SandmanorForm,
    },
    WrongSynthesisRegion {
        required: ConstitutionalRegion,
        actual: ConstitutionalRegion,
    },
    InsufficientRegionalStanding {
        being: RegionalBeingId,
        required: ConstitutionalRegion,
        actual: ConstitutionalRegion,
        kind: RegionalStandingKind,
    },
    WrongRegionalFunction {
        required: RegionalFunction,
        actual: RegionalFunction,
    },
    HouseLaw(HouseLawError),
    WrongAuthorityInstitution {
        function: HouseFunction,
        required: InstitutionId,
        actual: Option<InstitutionId>,
    },
    AuthorityFromFuture,
    AuthorityDecisionConflict,
    LineageErased,
    InvalidSynthesisResult,
    InvalidRegionalAssignment,
    AssignmentNotHeld {
        being: RegionalBeingId,
        assignment: &'static str,
    },
    CausalRegression {
        previous: CausalPosition,
        actual: CausalPosition,
    },
    RuleSetMismatch {
        expected: RuleSetId,
        actual: RuleSetId,
    },
    SequenceOverflow,
    ReplayDivergence(RegionalEventId),
}

impl RegionalSynthesisError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRegionalSite(_) => "REGIONAL_MISSING_SITE",
            Self::UncontrolledRegionalSite(_) => "REGIONAL_UNCONTROLLED_SITE",
            Self::InvalidRegionalController { .. } => "REGIONAL_INVALID_CONTROLLER",
            Self::RegionCannotBePrimaryStanding(_) => "REGIONAL_INVALID_PRIMARY_STANDING",
            Self::StandingJurisdictionMismatch => "REGIONAL_STANDING_JURISDICTION_MISMATCH",
            Self::JurisdictionFromFuture => "REGIONAL_JURISDICTION_FROM_FUTURE",
            Self::MissingEvidence(_) => "REGIONAL_MISSING_EVIDENCE",
            Self::EvidenceSubjectMismatch { .. } => "REGIONAL_EVIDENCE_SUBJECT_MISMATCH",
            Self::EventIdConflict(_) => "REGIONAL_EVENT_ID_CONFLICT",
            Self::SynthesisIdConflict(_) => "REGIONAL_SYNTHESIS_ID_CONFLICT",
            Self::BeingIdConflict(_) => "REGIONAL_BEING_ID_CONFLICT",
            Self::UnknownBeing(_) => "REGIONAL_UNKNOWN_BEING",
            Self::BeingNotActive(_) => "REGIONAL_BEING_NOT_ACTIVE",
            Self::OriginRegistrationRequired(_) => "REGIONAL_PREDECESSOR_LINEAGE_REQUIRED",
            Self::ResultReusesPredecessorIdentity => "REGIONAL_RESULT_REUSES_PREDECESSOR_IDENTITY",
            Self::PredecessorFormMismatch { .. } => "REGIONAL_PREDECESSOR_FORM_MISMATCH",
            Self::Lineage(_) => "REGIONAL_ILLEGAL_LINEAGE_TRANSITION",
            Self::UnratifiedRegionalRule { .. } => "REGIONAL_UNRATIFIED_SYNTHESIS_RULE",
            Self::WrongSynthesisRegion { .. } => "REGIONAL_WRONG_SYNTHESIS_REGION",
            Self::InsufficientRegionalStanding { .. } => "REGIONAL_INSUFFICIENT_STANDING",
            Self::WrongRegionalFunction { .. } => "REGIONAL_WRONG_FUNCTION",
            Self::HouseLaw(_) => "REGIONAL_HOUSE_LAW_REJECTED",
            Self::WrongAuthorityInstitution { .. } => "REGIONAL_WRONG_AUTHORITY_INSTITUTION",
            Self::AuthorityFromFuture => "REGIONAL_AUTHORITY_FROM_FUTURE",
            Self::AuthorityDecisionConflict => "REGIONAL_AUTHORITY_DECISION_CONFLICT",
            Self::LineageErased => "REGIONAL_LINEAGE_ERASED",
            Self::InvalidSynthesisResult => "REGIONAL_INVALID_SYNTHESIS_RESULT",
            Self::InvalidRegionalAssignment => "REGIONAL_INVALID_ASSIGNMENT",
            Self::AssignmentNotHeld { .. } => "REGIONAL_ASSIGNMENT_NOT_HELD",
            Self::CausalRegression { .. } => "REGIONAL_CAUSAL_REGRESSION",
            Self::RuleSetMismatch { .. } => "REGIONAL_RULE_SET_MISMATCH",
            Self::SequenceOverflow => "REGIONAL_SEQUENCE_OVERFLOW",
            Self::ReplayDivergence(_) => "REGIONAL_REPLAY_DIVERGENCE",
        }
    }
}

#[must_use]
pub const fn sandmanor_form_name(form: SandmanorForm) -> &'static str {
    match form {
        SandmanorForm::Gnome => "Gnome",
        SandmanorForm::Minotaur => "Minotaur",
        SandmanorForm::Hecaton => "Hecaton",
        SandmanorForm::Elf => "Elf",
        SandmanorForm::Centaur => "Centaur",
        SandmanorForm::Pegasus => "Pegasus",
    }
}

impl fmt::Display for RegionalSynthesisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "regional Synthesis rejected: {self:?}")
    }
}

impl std::error::Error for RegionalSynthesisError {}
