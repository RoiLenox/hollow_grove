//! First-pass Stonebend foundation for Aura Way, the Aether–Current
//! continuum, lawful material Hollowing, and geographic stone refraction.
//!
//! The parent module remains authoritative for Stonebend government, Names,
//! Titles, subject Hollowing, Seals, and succession. This module reuses its
//! stable evidence, decision, and Seal identities without completing those
//! systems or changing runtime resources, routes, combat, or Synthesis.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::being_object_ontology::HollowingRefinement;
use crate::institution::IdentityId;
use crate::world::geography::ConstitutionalRouteId;

use super::{DecisionRecordId, EvidenceRecordId, SealRecordId};

pub const FOUNDATION_SOURCE: &str = "STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerticalPole {
    Aether,
    Bathos,
}

impl VerticalPole {
    #[must_use]
    pub const fn physical_manifestation(self) -> PhysicalManifestation {
        match self {
            Self::Aether => PhysicalManifestation::Aura,
            Self::Bathos => PhysicalManifestation::Current,
        }
    }

    #[must_use]
    pub const fn landmark(self) -> VerticalLandmark {
        match self {
            Self::Aether => VerticalLandmark::MtAura,
            Self::Bathos => VerticalLandmark::Riptide,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PhysicalManifestation {
    Aura,
    Current,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MediumState {
    Current,
    Aether,
}

impl MediumState {
    #[must_use]
    pub const fn accepts_burden(self, burden: BurdenState) -> bool {
        match self {
            Self::Current => matches!(burden, BurdenState::Heavy),
            Self::Aether => matches!(burden, BurdenState::Refined | BurdenState::WeightlessLimit),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BurdenState {
    Heavy,
    Refined,
    WeightlessLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerticalLandmark {
    MtAura,
    Riptide,
}

impl VerticalLandmark {
    #[must_use]
    pub const fn route(self) -> ConstitutionalRouteId {
        match self {
            Self::MtAura => ConstitutionalRouteId::MntAura,
            Self::Riptide => ConstitutionalRouteId::Riptide,
        }
    }

    /// The summit and the downward pole are shared constitutional symbols.
    /// Their route boundaries remain defined by constitutional geography, but
    /// neither symbol is owned by a House.
    #[must_use]
    pub const fn constitutional_owner(self) -> Option<crate::hollow_grove_contract::House> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdvancementRouteKind {
    StandardAuraWay,
    ExceptionalAlternative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdvancementLandmark {
    Sandmanor,
    AuraWay,
    MtAura,
    StonebendGate,
    Stonebend,
}

pub const SANDMANOR_TO_STONEBEND: [AdvancementLandmark; 5] = [
    AdvancementLandmark::Sandmanor,
    AdvancementLandmark::AuraWay,
    AdvancementLandmark::MtAura,
    AdvancementLandmark::StonebendGate,
    AdvancementLandmark::Stonebend,
];

impl AdvancementLandmark {
    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::Sandmanor => 0,
            Self::AuraWay => 1,
            Self::MtAura => 2,
            Self::StonebendGate => 3,
            Self::Stonebend => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraWayStage {
    Prerequisite,
    Education,
    SupervisedPractice,
    Examination,
    DemonstratedResponsibility,
    RecognitionEligible,
}

pub const AURA_WAY_STAGE_ORDER: [AuraWayStage; 6] = [
    AuraWayStage::Prerequisite,
    AuraWayStage::Education,
    AuraWayStage::SupervisedPractice,
    AuraWayStage::Examination,
    AuraWayStage::DemonstratedResponsibility,
    AuraWayStage::RecognitionEligible,
];

impl AuraWayStage {
    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::Prerequisite => 0,
            Self::Education => 1,
            Self::SupervisedPractice => 2,
            Self::Examination => 3,
            Self::DemonstratedResponsibility => 4,
            Self::RecognitionEligible => 5,
        }
    }

    #[must_use]
    pub const fn predecessor(self) -> Option<Self> {
        match self {
            Self::Prerequisite => None,
            Self::Education => Some(Self::Prerequisite),
            Self::SupervisedPractice => Some(Self::Education),
            Self::Examination => Some(Self::SupervisedPractice),
            Self::DemonstratedResponsibility => Some(Self::Examination),
            Self::RecognitionEligible => Some(Self::DemonstratedResponsibility),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraWayStageEvidence {
    pub stage: AuraWayStage,
    pub evidence: EvidenceRecordId,
    pub supervising_authority: IdentityId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraWayPath {
    pub id: IdentityId,
    pub candidate: IdentityId,
    pub profession: String,
    pub route_kind: AdvancementRouteKind,
    pub stage_evidence: Vec<AuraWayStageEvidence>,
}

impl AuraWayPath {
    pub fn validate(&self) -> Result<(), FoundationError> {
        if self.route_kind != AdvancementRouteKind::StandardAuraWay {
            return Err(FoundationError::AuraWayCannotBeExceptional);
        }
        if self.profession.trim().is_empty() {
            return Err(FoundationError::MissingProfession);
        }

        let mut stages = BTreeSet::new();
        for record in &self.stage_evidence {
            if !stages.insert(record.stage) {
                return Err(FoundationError::DuplicateAuraWayStage(record.stage));
            }
            if let Some(predecessor) = record.stage.predecessor()
                && !self
                    .stage_evidence
                    .iter()
                    .any(|candidate| candidate.stage == predecessor)
            {
                return Err(FoundationError::MissingAuraWayStage(predecessor));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn semantic_evidence(&self) -> Vec<&AuraWayStageEvidence> {
        let mut evidence = self.stage_evidence.iter().collect::<Vec<_>>();
        evidence.sort_by_key(|record| record.stage.semantic_order());
        evidence
    }

    #[must_use]
    pub fn is_recognition_eligible(&self) -> bool {
        self.validate().is_ok()
            && AURA_WAY_STAGE_ORDER.iter().all(|stage| {
                self.stage_evidence
                    .iter()
                    .any(|evidence| evidence.stage == *stage)
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendRecognition {
    pub id: IdentityId,
    pub path: IdentityId,
    pub candidate: IdentityId,
    pub seal: SealRecordId,
    pub declares_metaphysical_perfection: bool,
}

pub fn recognize_aura_way_completion(
    path: &AuraWayPath,
    recognition_id: IdentityId,
    seal: SealRecordId,
) -> Result<StonebendRecognition, FoundationError> {
    path.validate()?;
    if !path.is_recognition_eligible() {
        let missing = AURA_WAY_STAGE_ORDER
            .iter()
            .find(|stage| {
                !path
                    .stage_evidence
                    .iter()
                    .any(|evidence| evidence.stage == **stage)
            })
            .copied()
            .unwrap_or(AuraWayStage::RecognitionEligible);
        return Err(FoundationError::MissingAuraWayStage(missing));
    }

    Ok(StonebendRecognition {
        id: recognition_id,
        path: path.id.clone(),
        candidate: path.candidate.clone(),
        seal,
        declares_metaphysical_perfection: false,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FractionRole {
    Essential,
    RemovableBurden,
    Inert,
    Unstable,
    Contaminant,
}

impl FractionRole {
    #[must_use]
    pub const fn lawfully_removable(self) -> bool {
        !matches!(self, Self::Essential)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingFraction {
    pub id: IdentityId,
    pub role: FractionRole,
    pub quantity: u64,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentBatch {
    pub id: IdentityId,
    /// Stable identity of the common Aether–Current medium carried through
    /// every lawful transformation.
    pub medium_lineage: IdentityId,
    pub source: IdentityId,
    pub quantity: u64,
    pub burden: BurdenState,
    pub fractions: Vec<HollowingFraction>,
    pub lawful_custodian: IdentityId,
    pub extraction_evidence: Vec<EvidenceRecordId>,
}

impl CurrentBatch {
    pub fn validate(&self) -> Result<(), FoundationError> {
        if !MediumState::Current.accepts_burden(self.burden) {
            return Err(FoundationError::InvalidBurdenState {
                medium: MediumState::Current,
                burden: self.burden,
            });
        }
        if self.quantity == 0 || self.fractions.is_empty() {
            return Err(FoundationError::EmptyCurrentBatch);
        }
        let mut identities = BTreeSet::new();
        let mut total = 0_u64;
        for fraction in &self.fractions {
            if fraction.quantity == 0 || fraction.description.trim().is_empty() {
                return Err(FoundationError::InvalidFraction(fraction.id.clone()));
            }
            if !identities.insert(fraction.id.clone()) {
                return Err(FoundationError::DuplicateFraction(fraction.id.clone()));
            }
            total = total
                .checked_add(fraction.quantity)
                .ok_or(FoundationError::FractionQuantityOverflow)?;
        }
        if total != self.quantity {
            return Err(FoundationError::FractionQuantityMismatch {
                batch: self.quantity,
                fractions: total,
            });
        }
        if !self
            .fractions
            .iter()
            .any(|fraction| fraction.role == FractionRole::Essential)
        {
            return Err(FoundationError::MissingEssentialFraction);
        }
        if self.extraction_evidence.is_empty() {
            return Err(FoundationError::MissingExtractionEvidence);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingAuthorization {
    pub authority: DecisionRecordId,
    pub source_current: IdentityId,
    pub operator: IdentityId,
    pub allowed_removed_fractions: BTreeSet<IdentityId>,
    pub minimum_preserved_quantity: u64,
    pub evidence: Vec<EvidenceRecordId>,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HollowingOperationOutcome {
    Completed,
    FailedWithoutMisconduct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofStatus {
    Unmeasured,
    Measured,
    Recognized,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofEvidence {
    /// Measurement proves concentration, refinement, or potency.
    pub measurement: EvidenceRecordId,
    /// Process evidence proves authorization, provenance, and performance.
    pub process: EvidenceRecordId,
    pub status: ProofStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingRequest {
    pub process: IdentityId,
    pub result_aether: IdentityId,
    pub declared_source_current: IdentityId,
    pub refinement: HollowingRefinement,
    pub requested_removals: Vec<IdentityId>,
    pub outcome: HollowingOperationOutcome,
    pub proof: ProofEvidence,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchProvenance {
    pub source_current: IdentityId,
    pub medium_lineage: IdentityId,
    pub process: IdentityId,
    pub authority: DecisionRecordId,
    pub evidence: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AetherBatch {
    pub id: IdentityId,
    pub medium_lineage: IdentityId,
    pub state: MediumState,
    pub burden: BurdenState,
    pub source_current: IdentityId,
    pub preserved_fractions: Vec<HollowingFraction>,
    pub removed_fractions: Vec<HollowingFraction>,
    pub proof: ProofEvidence,
    pub provenance: BatchProvenance,
    pub seal: SealRecordId,
}

impl AetherBatch {
    #[must_use]
    pub fn is_recognized(&self) -> bool {
        self.state == MediumState::Aether
            && MediumState::Aether.accepts_burden(self.burden)
            && self.proof.status == ProofStatus::Recognized
    }
}

pub fn hollow_current(
    source: &CurrentBatch,
    authorization: &HollowingAuthorization,
    request: &HollowingRequest,
) -> Result<AetherBatch, HollowingError> {
    source
        .validate()
        .map_err(HollowingError::InvalidSourceBatch)?;

    if !authorization.active
        || authorization.source_current != source.id
        || authorization.evidence.is_empty()
    {
        return Err(HollowingError::UnauthorizedExtraction);
    }
    if request.declared_source_current != source.id {
        return Err(HollowingError::FalsifiedProvenance);
    }
    if request.refinement != HollowingRefinement::MaterialLightening {
        return Err(HollowingError::UnsupportedMaterialRefinement);
    }

    let fractions = source
        .fractions
        .iter()
        .map(|fraction| (fraction.id.clone(), fraction))
        .collect::<BTreeMap<_, _>>();
    let mut requested = BTreeSet::new();
    let mut removal_quantity = 0_u64;
    for identity in &request.requested_removals {
        if !requested.insert(identity.clone()) {
            return Err(HollowingError::DuplicateRemoval(identity.clone()));
        }
        let fraction = fractions
            .get(identity)
            .ok_or_else(|| HollowingError::UnknownFraction(identity.clone()))?;
        if !fraction.role.lawfully_removable() {
            return Err(HollowingError::EssentialFractionRemoval(identity.clone()));
        }
        if !authorization.allowed_removed_fractions.contains(identity) {
            return Err(HollowingError::ExceedsAuthorizedScope(identity.clone()));
        }
        removal_quantity = removal_quantity
            .checked_add(fraction.quantity)
            .ok_or(HollowingError::OverHollowing)?;
    }
    let preserved_quantity = source
        .quantity
        .checked_sub(removal_quantity)
        .ok_or(HollowingError::OverHollowing)?;
    if preserved_quantity < authorization.minimum_preserved_quantity {
        return Err(HollowingError::OverHollowing);
    }

    if request.outcome == HollowingOperationOutcome::FailedWithoutMisconduct {
        return Err(HollowingError::LawfulProcessFailure);
    }
    if request.proof.status != ProofStatus::Recognized {
        return Err(HollowingError::UnrecognizedProof);
    }

    let mut preserved_fractions = Vec::new();
    let mut removed_fractions = Vec::new();
    for fraction in fractions.into_values() {
        if requested.contains(&fraction.id) {
            removed_fractions.push(fraction.clone());
        } else {
            preserved_fractions.push(fraction.clone());
        }
    }
    if !preserved_fractions
        .iter()
        .any(|fraction| fraction.role == FractionRole::Essential)
    {
        return Err(HollowingError::OverHollowing);
    }

    let mut process_evidence = authorization.evidence.clone();
    process_evidence.push(request.proof.process.clone());
    process_evidence.sort();
    process_evidence.dedup();

    Ok(AetherBatch {
        id: request.result_aether.clone(),
        medium_lineage: source.medium_lineage.clone(),
        state: MediumState::Aether,
        burden: BurdenState::Refined,
        source_current: source.id.clone(),
        preserved_fractions,
        removed_fractions,
        proof: request.proof.clone(),
        provenance: BatchProvenance {
            source_current: source.id.clone(),
            medium_lineage: source.medium_lineage.clone(),
            process: request.process.clone(),
            authority: authorization.authority.clone(),
            evidence: process_evidence,
        },
        seal: request.seal.clone(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HollowingFindingKind {
    UnauthorizedExtraction,
    EssentialFractionRemoval,
    FalsifiedProvenance,
    ExceededScope,
    OverHollowing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IllegalHollowingFinding {
    pub source_current: IdentityId,
    pub process: IdentityId,
    pub kind: HollowingFindingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HollowingError {
    InvalidSourceBatch(FoundationError),
    UnauthorizedExtraction,
    FalsifiedProvenance,
    UnsupportedMaterialRefinement,
    DuplicateRemoval(IdentityId),
    UnknownFraction(IdentityId),
    EssentialFractionRemoval(IdentityId),
    ExceedsAuthorizedScope(IdentityId),
    OverHollowing,
    LawfulProcessFailure,
    UnrecognizedProof,
}

impl HollowingError {
    #[must_use]
    pub const fn is_illegal_hollowing(&self) -> bool {
        matches!(
            self,
            Self::UnauthorizedExtraction
                | Self::FalsifiedProvenance
                | Self::EssentialFractionRemoval(_)
                | Self::ExceedsAuthorizedScope(_)
                | Self::OverHollowing
        )
    }

    #[must_use]
    pub fn finding(
        &self,
        source_current: IdentityId,
        process: IdentityId,
    ) -> Option<IllegalHollowingFinding> {
        let kind = match self {
            Self::UnauthorizedExtraction => HollowingFindingKind::UnauthorizedExtraction,
            Self::FalsifiedProvenance => HollowingFindingKind::FalsifiedProvenance,
            Self::EssentialFractionRemoval(_) => HollowingFindingKind::EssentialFractionRemoval,
            Self::ExceedsAuthorizedScope(_) => HollowingFindingKind::ExceededScope,
            Self::OverHollowing => HollowingFindingKind::OverHollowing,
            Self::InvalidSourceBatch(_)
            | Self::UnsupportedMaterialRefinement
            | Self::DuplicateRemoval(_)
            | Self::UnknownFraction(_)
            | Self::LawfulProcessFailure
            | Self::UnrecognizedProof => return None,
        };
        Some(IllegalHollowingFinding {
            source_current,
            process,
            kind,
        })
    }
}

impl fmt::Display for HollowingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSourceBatch(error) => write!(formatter, "invalid Current batch: {error}"),
            Self::UnauthorizedExtraction => formatter.write_str("Hollowing is unauthorized"),
            Self::FalsifiedProvenance => formatter.write_str("source provenance was falsified"),
            Self::UnsupportedMaterialRefinement => {
                formatter.write_str("Current-to-Aether Hollowing requires material lightening")
            }
            Self::DuplicateRemoval(id) => write!(formatter, "fraction {id} was requested twice"),
            Self::UnknownFraction(id) => write!(formatter, "fraction {id} does not exist"),
            Self::EssentialFractionRemoval(id) => {
                write!(formatter, "essential fraction {id} cannot be removed")
            }
            Self::ExceedsAuthorizedScope(id) => {
                write!(formatter, "fraction {id} exceeds authorized scope")
            }
            Self::OverHollowing => formatter.write_str("Hollowing removed the supporting whole"),
            Self::LawfulProcessFailure => {
                formatter.write_str("authorized Hollowing failed without misconduct")
            }
            Self::UnrecognizedProof => formatter.write_str("resulting proof is not recognized"),
        }
    }
}

impl std::error::Error for HollowingError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StoneBehavior {
    Variable,
    Concentrating,
    Resonant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RefractiveProperty {
    ContextualVariation,
    MixedBands,
    Adaptation,
    Concentration,
    Coherence,
    Stability,
    Resonance,
    Repeatability,
    Synchronization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneFormationContext {
    pub geography: IdentityId,
    pub pressure: String,
    pub heat: String,
    pub impurities: String,
    pub environmental_history: String,
}

impl StoneFormationContext {
    fn validate(&self) -> Result<(), FoundationError> {
        if [
            &self.pressure,
            &self.heat,
            &self.impurities,
            &self.environmental_history,
        ]
        .iter()
        .any(|value| value.trim().is_empty())
        {
            return Err(FoundationError::IncompleteFormationContext);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalStoneProfile {
    pub id: IdentityId,
    pub common_name: String,
    pub formation: StoneFormationContext,
    pub behavior: StoneBehavior,
    pub properties: BTreeSet<RefractiveProperty>,
    pub lattice_preserved: bool,
}

impl RegionalStoneProfile {
    pub fn validate(&self) -> Result<(), FoundationError> {
        self.formation.validate()?;
        if self.common_name.trim().is_empty() || self.properties.is_empty() {
            return Err(FoundationError::IncompleteStoneProfile);
        }
        if !self.lattice_preserved {
            return Err(FoundationError::StoneLatticeDestroyed);
        }
        Ok(())
    }

    /// Foundational stone behaviors are nonexclusive natural examples. This
    /// pass deliberately introduces no final House-to-stone assignment.
    #[must_use]
    pub const fn final_house_assignment(&self) -> Option<crate::hollow_grove_contract::House> {
        None
    }

    #[must_use]
    pub const fn requires_melting_for_aura(&self) -> bool {
        false
    }
}

#[must_use]
pub fn opal_profile(id: IdentityId, formation: StoneFormationContext) -> RegionalStoneProfile {
    RegionalStoneProfile {
        id,
        common_name: "Opal".into(),
        formation,
        behavior: StoneBehavior::Variable,
        properties: BTreeSet::from([
            RefractiveProperty::ContextualVariation,
            RefractiveProperty::MixedBands,
            RefractiveProperty::Adaptation,
        ]),
        lattice_preserved: true,
    }
}

#[must_use]
pub fn diamond_profile(id: IdentityId, formation: StoneFormationContext) -> RegionalStoneProfile {
    RegionalStoneProfile {
        id,
        common_name: "Diamond".into(),
        formation,
        behavior: StoneBehavior::Concentrating,
        properties: BTreeSet::from([
            RefractiveProperty::Concentration,
            RefractiveProperty::Coherence,
            RefractiveProperty::Stability,
        ]),
        lattice_preserved: true,
    }
}

#[must_use]
pub fn quartz_profile(id: IdentityId, formation: StoneFormationContext) -> RegionalStoneProfile {
    RegionalStoneProfile {
        id,
        common_name: "Quartz".into(),
        formation,
        behavior: StoneBehavior::Resonant,
        properties: BTreeSet::from([
            RefractiveProperty::Resonance,
            RefractiveProperty::Repeatability,
            RefractiveProperty::Synchronization,
        ]),
        lattice_preserved: true,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraManifestation {
    pub id: IdentityId,
    pub physical_state: PhysicalManifestation,
    pub source_aether: IdentityId,
    pub source_current: IdentityId,
    pub medium_lineage: IdentityId,
    pub stone_profile: IdentityId,
    pub formation_geography: IdentityId,
    pub behavior: StoneBehavior,
    pub properties: BTreeSet<RefractiveProperty>,
}

pub fn refract_aether(
    aether: &AetherBatch,
    stone: &RegionalStoneProfile,
    manifestation_id: IdentityId,
) -> Result<AuraManifestation, FoundationError> {
    if !aether.is_recognized() {
        return Err(FoundationError::UnrecognizedAether);
    }
    stone.validate()?;

    Ok(AuraManifestation {
        id: manifestation_id,
        physical_state: PhysicalManifestation::Aura,
        source_aether: aether.id.clone(),
        source_current: aether.source_current.clone(),
        medium_lineage: aether.medium_lineage.clone(),
        stone_profile: stone.id.clone(),
        formation_geography: stone.formation.geography.clone(),
        behavior: stone.behavior,
        properties: stone.properties.clone(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationError {
    AuraWayCannotBeExceptional,
    MissingProfession,
    DuplicateAuraWayStage(AuraWayStage),
    MissingAuraWayStage(AuraWayStage),
    InvalidBurdenState {
        medium: MediumState,
        burden: BurdenState,
    },
    EmptyCurrentBatch,
    InvalidFraction(IdentityId),
    DuplicateFraction(IdentityId),
    FractionQuantityOverflow,
    FractionQuantityMismatch {
        batch: u64,
        fractions: u64,
    },
    MissingEssentialFraction,
    MissingExtractionEvidence,
    IncompleteFormationContext,
    IncompleteStoneProfile,
    StoneLatticeDestroyed,
    UnrecognizedAether,
}

impl fmt::Display for FoundationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuraWayCannotBeExceptional => {
                formatter.write_str("Aura Way is the standard route, not an exceptional shortcut")
            }
            Self::MissingProfession => formatter.write_str("Aura Way profession is empty"),
            Self::DuplicateAuraWayStage(stage) => {
                write!(formatter, "Aura Way stage {stage:?} appears twice")
            }
            Self::MissingAuraWayStage(stage) => {
                write!(formatter, "Aura Way stage {stage:?} is missing")
            }
            Self::InvalidBurdenState { medium, burden } => {
                write!(formatter, "{burden:?} is invalid for {medium:?}")
            }
            Self::EmptyCurrentBatch => formatter.write_str("Current batch is empty"),
            Self::InvalidFraction(id) => write!(formatter, "fraction {id} is invalid"),
            Self::DuplicateFraction(id) => write!(formatter, "fraction {id} appears twice"),
            Self::FractionQuantityOverflow => formatter.write_str("fraction quantity overflowed"),
            Self::FractionQuantityMismatch { batch, fractions } => {
                write!(
                    formatter,
                    "batch quantity {batch} differs from fractions {fractions}"
                )
            }
            Self::MissingEssentialFraction => {
                formatter.write_str("Current batch has no essential fraction")
            }
            Self::MissingExtractionEvidence => {
                formatter.write_str("Current batch has no extraction evidence")
            }
            Self::IncompleteFormationContext => {
                formatter.write_str("stone formation context is incomplete")
            }
            Self::IncompleteStoneProfile => formatter.write_str("stone profile is incomplete"),
            Self::StoneLatticeDestroyed => {
                formatter.write_str("ordinary Aura manifestation requires a preserved lattice")
            }
            Self::UnrecognizedAether => formatter.write_str("Aether batch is not recognized"),
        }
    }
}

impl std::error::Error for FoundationError {}

/// Executable first-pass conformance independent of full Stonebend government.
pub fn validate_foundation() -> Result<(), FoundationError> {
    if VerticalPole::Aether.landmark() != VerticalLandmark::MtAura
        || VerticalPole::Bathos.landmark() != VerticalLandmark::Riptide
        || VerticalPole::Aether.physical_manifestation() != PhysicalManifestation::Aura
        || VerticalPole::Bathos.physical_manifestation() != PhysicalManifestation::Current
        || VerticalLandmark::MtAura.constitutional_owner().is_some()
        || VerticalLandmark::Riptide.constitutional_owner().is_some()
        || !MediumState::Current.accepts_burden(BurdenState::Heavy)
        || !MediumState::Aether.accepts_burden(BurdenState::Refined)
        || MediumState::Current == MediumState::Aether
    {
        return Err(FoundationError::InvalidBurdenState {
            medium: MediumState::Aether,
            burden: BurdenState::Heavy,
        });
    }
    Ok(())
}
