//! Officials and Outlaws is the Flynt constitutional domain layer.
//!
//! It models three mirrored pairs:
//!
//! - Manticorp ↔ Werewolves
//! - Mystery Men ↔ Gallows
//! - Mysteryguard ↔ Mermen
//!
//! Official and Outlaw describe constitutional orientation, not moral
//! alignment. The layer also records the foundational Flynt forms and higher
//! synthesis needed to produce Chimera. Executive succession then proceeds by
//! domain-level Chimera refinement into the Manticorp Form, constitutional
//! recognition of a candidate's proven mastery, and lawful accession to the
//! Tross office. Only the foundational Chimera synthesis is causal composition.

use std::collections::HashSet;
use std::fmt;

use hollow_grove_kernel::{
    CompositionCatalog, CompositionCatalogError, CompositionNode, CompositionNodeId,
    CompositionRecord, CompositionRecordId, ExternalRef, ScaleKey, StableKeyError,
};

pub const OFFICIAL_MANTICORP: &str = "officials-outlaws.official.manticorp";
pub const OFFICIAL_MYSTERY_MEN: &str = "officials-outlaws.official.mystery-men";
pub const OFFICIAL_MYSTERYGUARD: &str = "officials-outlaws.official.mysteryguard";

pub const OUTLAW_WEREWOLVES: &str = "officials-outlaws.outlaw.werewolves";
pub const OUTLAW_GALLOWS: &str = "officials-outlaws.outlaw.gallows";
pub const OUTLAW_MERMEN: &str = "officials-outlaws.outlaw.mermen";

pub const MIRROR_MANTICORP_WEREWOLVES: &str = "officials-outlaws.mirror.manticorp-werewolves";
pub const MIRROR_MYSTERY_MEN_GALLOWS: &str = "officials-outlaws.mirror.mystery-men-gallows";
pub const MIRROR_MYSTERYGUARD_MERMEN: &str = "officials-outlaws.mirror.mysteryguard-mermen";

pub const LINEAGE_GARGOYLE: &str = "officials-outlaws.lineage.gargoyle";
pub const LINEAGE_WEREWOLF: &str = "officials-outlaws.lineage.werewolf";
pub const LINEAGE_MERMAN: &str = "officials-outlaws.lineage.merman";

pub const FORM_GARGOYLE: &str = "officials-outlaws.form.gargoyle";
pub const FORM_WEREWOLF: &str = "officials-outlaws.form.werewolf";
pub const FORM_MERMAN: &str = "officials-outlaws.form.merman";
pub const FORM_CHIMERA: &str = "officials-outlaws.form.chimera";
pub const FORM_MANTICORP: &str = "officials-outlaws.form.manticorp";

pub const OFFICE_TROSS: &str = "officials-outlaws.office.tross";
pub const OFFICE_CHIMERA: &str = "officials-outlaws.office.chimera";

pub const RECIPE_CHIMERA: &str = "officials-outlaws.recipe.chimera";

pub const PERSON_CANONICAL_TROSS_CANDIDATE: &str =
    "officials-outlaws.person.canonical-tross-candidate";
pub const REFINEMENT_CHIMERA_TO_MANTICORP: &str =
    "officials-outlaws.refinement.chimera-to-manticorp";
pub const MASTERY_CHIMERA_TO_MANTICORP: &str = "officials-outlaws.mastery.chimera-to-manticorp";
pub const RECOGNITION_TROSS_SUCCESSION: &str = "officials-outlaws.recognition.tross-succession";
pub const ACCESSION_TROSS: &str = "officials-outlaws.accession.tross";
pub const WITNESS_CHIMERA_REFINEMENT: &str = "officials-outlaws.witness.chimera-refinement";
pub const WITNESS_TROSS_RECOGNITION: &str = "officials-outlaws.witness.tross-recognition";

const OFFICIAL_NAMESPACE: &str = "officials-outlaws.official.";
const OUTLAW_NAMESPACE: &str = "officials-outlaws.outlaw.";
const LINEAGE_NAMESPACE: &str = "officials-outlaws.lineage.";
const FORM_NAMESPACE: &str = "officials-outlaws.form.";
const OFFICE_NAMESPACE: &str = "officials-outlaws.office.";
const PERSON_NAMESPACE: &str = "officials-outlaws.person.";
const MIRROR_NAMESPACE: &str = "officials-outlaws.mirror.";
const RECIPE_NAMESPACE: &str = "officials-outlaws.recipe.";
const REFINEMENT_NAMESPACE: &str = "officials-outlaws.refinement.";
const MASTERY_NAMESPACE: &str = "officials-outlaws.mastery.";
const RECOGNITION_NAMESPACE: &str = "officials-outlaws.recognition.";
const ACCESSION_NAMESPACE: &str = "officials-outlaws.accession.";
const WITNESS_NAMESPACE: &str = "officials-outlaws.witness.";

/// A domain namespace failure is separate from the kernel's stable-key
/// validation. Character-level identity rules remain owned by the kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainIdError {
    KernelComposition(StableKeyError),
    WrongNamespace {
        value: String,
        expected: &'static str,
    },
}

impl fmt::Display for DomainIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KernelComposition(error) => error.fmt(formatter),
            Self::WrongNamespace { value, expected } => {
                write!(formatter, "{value} is outside the {expected} namespace")
            }
        }
    }
}

impl std::error::Error for DomainIdError {}

fn has_domain_namespace(value: &str, namespace: &'static str) -> bool {
    value
        .strip_prefix(namespace)
        .is_some_and(|remainder| !remainder.is_empty())
}

macro_rules! institutional_domain_id {
    ($name:ident, $namespace:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(ScaleKey);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, DomainIdError> {
                let value = value.into();
                let kernel =
                    ScaleKey::new(value.clone()).map_err(DomainIdError::KernelComposition)?;
                if !has_domain_namespace(&value, $namespace) {
                    return Err(DomainIdError::WrongNamespace {
                        value,
                        expected: $namespace,
                    });
                }
                Ok(Self(kernel))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[must_use]
            pub fn as_kernel(&self) -> &ScaleKey {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.as_str().fmt(formatter)
            }
        }
    };
}

macro_rules! composition_domain_id {
    ($name:ident, $kernel:ty, $namespace:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name($kernel);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, DomainIdError> {
                let value = value.into();
                let kernel =
                    <$kernel>::new(value.clone()).map_err(DomainIdError::KernelComposition)?;
                if !has_domain_namespace(&value, $namespace) {
                    return Err(DomainIdError::WrongNamespace {
                        value,
                        expected: $namespace,
                    });
                }
                Ok(Self(kernel))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[must_use]
            pub fn as_kernel(&self) -> &$kernel {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.as_str().fmt(formatter)
            }
        }
    };
}

institutional_domain_id!(OfficialId, OFFICIAL_NAMESPACE);
institutional_domain_id!(OutlawId, OUTLAW_NAMESPACE);
institutional_domain_id!(ConstitutionalOfficeId, OFFICE_NAMESPACE);
institutional_domain_id!(PersonId, PERSON_NAMESPACE);
institutional_domain_id!(MirrorPairId, MIRROR_NAMESPACE);
institutional_domain_id!(ChimeraRefinementId, REFINEMENT_NAMESPACE);
institutional_domain_id!(ExecutiveMasteryId, MASTERY_NAMESPACE);
institutional_domain_id!(ConstitutionalRecognitionId, RECOGNITION_NAMESPACE);
institutional_domain_id!(LawfulAccessionId, ACCESSION_NAMESPACE);
institutional_domain_id!(ConstitutionalWitnessId, WITNESS_NAMESPACE);
composition_domain_id!(LineageId, CompositionNodeId, LINEAGE_NAMESPACE);
composition_domain_id!(FormId, CompositionNodeId, FORM_NAMESPACE);
composition_domain_id!(SynthesisRecipeId, CompositionRecordId, RECIPE_NAMESPACE);

/// Constitutional status. It deliberately carries no moral judgment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalOrientation {
    Official,
    Outlaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SharedFunction {
    MartialForce,
    HiddenThreatHuntingInvestigationAndControl,
    MaritimeMovementPursuitAndSovereignty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfficialInstitution {
    pub id: OfficialId,
    pub name: String,
    pub function: SharedFunction,
}

impl OfficialInstitution {
    #[must_use]
    pub const fn orientation(&self) -> ConstitutionalOrientation {
        ConstitutionalOrientation::Official
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlawExpression {
    pub id: OutlawId,
    pub name: String,
    pub function: SharedFunction,
    pub constitutional_expression: String,
}

impl OutlawExpression {
    #[must_use]
    pub const fn orientation(&self) -> ConstitutionalOrientation {
        ConstitutionalOrientation::Outlaw
    }
}

/// The endpoint sum used only for symmetric mirror queries and registration.
/// Institutions, forms, lineages, offices, and people remain separate types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstitutionalEntityId {
    Official(OfficialId),
    Outlaw(OutlawId),
}

impl ConstitutionalEntityId {
    #[must_use]
    pub const fn orientation(&self) -> ConstitutionalOrientation {
        match self {
            Self::Official(_) => ConstitutionalOrientation::Official,
            Self::Outlaw(_) => ConstitutionalOrientation::Outlaw,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Official(id) => id.as_str(),
            Self::Outlaw(id) => id.as_str(),
        }
    }
}

impl From<OfficialId> for ConstitutionalEntityId {
    fn from(value: OfficialId) -> Self {
        Self::Official(value)
    }
}

impl From<OutlawId> for ConstitutionalEntityId {
    fn from(value: OutlawId) -> Self {
        Self::Outlaw(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorPair {
    pub id: MirrorPairId,
    pub official: OfficialId,
    pub outlaw: OutlawId,
    pub shared_function: SharedFunction,
    pub distinction: String,
}

impl MirrorPair {
    #[must_use]
    pub fn counterpart(&self, entity: &ConstitutionalEntityId) -> Option<ConstitutionalEntityId> {
        match entity {
            ConstitutionalEntityId::Official(id) if id == &self.official => {
                Some(ConstitutionalEntityId::Outlaw(self.outlaw.clone()))
            }
            ConstitutionalEntityId::Outlaw(id) if id == &self.outlaw => {
                Some(ConstitutionalEntityId::Official(self.official.clone()))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormKind {
    Foundational,
    HigherSynthesis,
    PerfectedComposite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationForm {
    pub id: FormId,
    pub name: String,
    pub kind: FormKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationLineage {
    pub id: LineageId,
    pub name: String,
    pub base_form: FormId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisBase {
    pub lineage: LineageId,
    pub form: FormId,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineageRelationKind {
    DirectExpression,
    Influence,
}

/// Records the direct Werewolf/Merman institutional expressions and the
/// Gargoyle influence on Gallows without equating an institution and lineage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlawLineageRelation {
    pub outlaw: OutlawId,
    pub lineage: LineageId,
    pub kind: LineageRelationKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalOfficeKind {
    Executive,
    SecondInCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalOffice {
    pub id: ConstitutionalOfficeId,
    pub title: String,
    pub kind: ConstitutionalOfficeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisRecipe {
    pub id: SynthesisRecipeId,
    pub sources: Vec<FormId>,
    pub result: FormId,
}

/// Perfected dimensions of Manticorp mastery. These are qualities of the
/// achieved form, never transformation forms or synthesis ingredients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ManticorpMasteryAspect {
    Lion,
    Eagle,
    Hydra,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalWitness {
    pub id: ConstitutionalWitnessId,
    pub statement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChimeraRefinement {
    pub id: ChimeraRefinementId,
    pub source: FormId,
    pub perfected_aspects: Vec<ManticorpMasteryAspect>,
    pub result: FormId,
    pub evidence: Vec<ConstitutionalWitness>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutiveMastery {
    pub id: ExecutiveMasteryId,
    pub candidate: PersonId,
    pub completed_chimera: FormId,
    pub refinement: ChimeraRefinement,
    pub resulting_manticorp: FormId,
}

/// Constitutional acknowledgment of an already-proven executive mastery.
/// Registration never creates or changes the referenced form achievement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalRecognition {
    pub id: ConstitutionalRecognitionId,
    pub candidate: PersonId,
    pub mastery: ExecutiveMasteryId,
    pub achieved_manticorp: FormId,
    pub office: ConstitutionalOfficeId,
    pub witnesses: Vec<ConstitutionalWitness>,
}

/// Lawful office accession is downstream of recognition and is not an office,
/// form, person, or causal composition record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LawfulAccession {
    pub id: LawfulAccessionId,
    pub candidate: PersonId,
    pub recognition: ConstitutionalRecognitionId,
    pub office: ConstitutionalOfficeId,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegistryError {
    DuplicateOfficial(OfficialId),
    DuplicateOutlaw(OutlawId),
    DuplicateLineage(LineageId),
    DuplicateForm(FormId),
    DuplicateOffice(ConstitutionalOfficeId),
    DuplicateMirrorId(MirrorPairId),
    DuplicateMirrorPair {
        official: OfficialId,
        outlaw: OutlawId,
    },
    DuplicateRecipe(SynthesisRecipeId),
    DuplicateRecipeSource {
        recipe: SynthesisRecipeId,
        source: FormId,
    },
    DuplicateSynthesisBase(LineageId),
    DuplicateLineageRelation {
        outlaw: OutlawId,
        lineage: LineageId,
    },
    DuplicateExecutiveMastery(ExecutiveMasteryId),
    DuplicateChimeraRefinement(ChimeraRefinementId),
    DuplicateRecognition(ConstitutionalRecognitionId),
    DuplicateAccession(LawfulAccessionId),
    SameMirrorEntity(String),
    MirrorRequiresOppositeOrientations,
    UnknownOfficial(OfficialId),
    UnknownOutlaw(OutlawId),
    UnknownLineage(LineageId),
    UnknownForm(FormId),
    UnknownOffice(ConstitutionalOfficeId),
    ChimeraSynthesisRequired,
    ManticorpCannotBeDirectlySynthesized,
    RecognitionRequiresMastery(ExecutiveMasteryId),
    AccessionRequiresRecognition(ConstitutionalRecognitionId),
    OfficialMirrorCount {
        official: OfficialId,
        count: usize,
    },
    OutlawMirrorCount {
        outlaw: OutlawId,
        count: usize,
    },
    MirrorFunctionMismatch(MirrorPairId),
    CanonicalRosterMismatch(&'static str),
    WrongCanonicalMirror(OfficialId),
    ManticorpInstitutionFormCollision,
    ChimeraFormOfficeCollision,
    InvalidFoundationalBases,
    InvalidLineageRelations,
    InvalidChimeraRecipe,
    InvalidExecutiveMastery,
    InvalidConstitutionalRecognition,
    InvalidLawfulAccession,
    KernelComposition(CompositionCatalogError),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOfficial(id) => write!(formatter, "duplicate Official: {id}"),
            Self::DuplicateOutlaw(id) => write!(formatter, "duplicate Outlaw: {id}"),
            Self::DuplicateLineage(id) => write!(formatter, "duplicate lineage: {id}"),
            Self::DuplicateForm(id) => write!(formatter, "duplicate form: {id}"),
            Self::DuplicateOffice(id) => write!(formatter, "duplicate office: {id}"),
            Self::DuplicateMirrorId(id) => write!(formatter, "duplicate mirror ID: {id}"),
            Self::DuplicateMirrorPair { official, outlaw } => {
                write!(formatter, "duplicate mirror pair: {official} ↔ {outlaw}")
            }
            Self::DuplicateRecipe(id) => write!(formatter, "duplicate recipe: {id}"),
            Self::DuplicateRecipeSource { recipe, source } => {
                write!(formatter, "duplicate source {source} in recipe {recipe}")
            }
            Self::DuplicateSynthesisBase(id) => {
                write!(formatter, "duplicate synthesis base lineage: {id}")
            }
            Self::DuplicateLineageRelation { outlaw, lineage } => {
                write!(
                    formatter,
                    "duplicate lineage relation: {outlaw} / {lineage}"
                )
            }
            Self::DuplicateExecutiveMastery(id) => {
                write!(formatter, "duplicate executive mastery: {id}")
            }
            Self::DuplicateChimeraRefinement(id) => {
                write!(formatter, "duplicate Chimera refinement: {id}")
            }
            Self::DuplicateRecognition(id) => {
                write!(formatter, "duplicate constitutional recognition: {id}")
            }
            Self::DuplicateAccession(id) => {
                write!(formatter, "duplicate lawful accession: {id}")
            }
            Self::SameMirrorEntity(id) => write!(formatter, "an entity cannot mirror itself: {id}"),
            Self::MirrorRequiresOppositeOrientations => {
                formatter.write_str("a mirror requires one Official and one Outlaw")
            }
            Self::UnknownOfficial(id) => write!(formatter, "unknown Official: {id}"),
            Self::UnknownOutlaw(id) => write!(formatter, "unknown Outlaw: {id}"),
            Self::UnknownLineage(id) => write!(formatter, "unknown lineage: {id}"),
            Self::UnknownForm(id) => write!(formatter, "unknown form: {id}"),
            Self::UnknownOffice(id) => write!(formatter, "unknown office: {id}"),
            Self::ChimeraSynthesisRequired => {
                formatter.write_str("executive mastery requires the completed Chimera synthesis")
            }
            Self::ManticorpCannotBeDirectlySynthesized => formatter.write_str(
                "Manticorp Form is achieved by Chimera refinement, not a synthesis recipe",
            ),
            Self::RecognitionRequiresMastery(id) => {
                write!(
                    formatter,
                    "constitutional recognition requires proven mastery: {id}"
                )
            }
            Self::AccessionRequiresRecognition(id) => {
                write!(
                    formatter,
                    "lawful accession requires constitutional recognition: {id}"
                )
            }
            Self::OfficialMirrorCount { official, count } => {
                write!(
                    formatter,
                    "Official {official} has {count} canonical mirrors"
                )
            }
            Self::OutlawMirrorCount { outlaw, count } => {
                write!(formatter, "Outlaw {outlaw} has {count} canonical mirrors")
            }
            Self::MirrorFunctionMismatch(id) => {
                write!(
                    formatter,
                    "mirror function does not match both endpoints: {id}"
                )
            }
            Self::CanonicalRosterMismatch(kind) => {
                write!(
                    formatter,
                    "the canonical {kind} roster is incomplete or extended"
                )
            }
            Self::WrongCanonicalMirror(id) => {
                write!(formatter, "wrong canonical mirror for {id}")
            }
            Self::ManticorpInstitutionFormCollision => {
                formatter.write_str("Manticorp institution and Manticorp Form collided")
            }
            Self::ChimeraFormOfficeCollision => {
                formatter.write_str("Chimera form and Chimera office collided")
            }
            Self::InvalidFoundationalBases => {
                formatter.write_str("the foundational synthesis bases are not canonical")
            }
            Self::InvalidLineageRelations => {
                formatter.write_str("the outlaw-lineage relations are not canonical")
            }
            Self::InvalidChimeraRecipe => {
                formatter.write_str("the Chimera recipe is not canonical")
            }
            Self::InvalidExecutiveMastery => {
                formatter.write_str("the Tross executive mastery is not canonical")
            }
            Self::InvalidConstitutionalRecognition => {
                formatter.write_str("the Tross constitutional recognition is not canonical")
            }
            Self::InvalidLawfulAccession => {
                formatter.write_str("the Tross lawful accession is not canonical")
            }
            Self::KernelComposition(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for RegistryError {}

impl From<CompositionCatalogError> for RegistryError {
    fn from(value: CompositionCatalogError) -> Self {
        Self::KernelComposition(value)
    }
}

/// Mutable construction boundary for the locked domain. Identity and causal
/// provenance are projected through Hollow Grove's public API.
#[derive(Debug)]
pub struct OfficialsOutlawsRegistry {
    officials: Vec<OfficialInstitution>,
    outlaws: Vec<OutlawExpression>,
    mirrors: Vec<MirrorPair>,
    lineages: Vec<TransformationLineage>,
    forms: Vec<TransformationForm>,
    synthesis_bases: Vec<SynthesisBase>,
    lineage_relations: Vec<OutlawLineageRelation>,
    offices: Vec<ConstitutionalOffice>,
    recipes: Vec<SynthesisRecipe>,
    executive_masteries: Vec<ExecutiveMastery>,
    recognitions: Vec<ConstitutionalRecognition>,
    accessions: Vec<LawfulAccession>,
    composition: CompositionCatalog,
}

impl Default for OfficialsOutlawsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl OfficialsOutlawsRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            officials: Vec::new(),
            outlaws: Vec::new(),
            mirrors: Vec::new(),
            lineages: Vec::new(),
            forms: Vec::new(),
            synthesis_bases: Vec::new(),
            lineage_relations: Vec::new(),
            offices: Vec::new(),
            recipes: Vec::new(),
            executive_masteries: Vec::new(),
            recognitions: Vec::new(),
            accessions: Vec::new(),
            composition: CompositionCatalog::new(),
        }
    }

    pub fn from_entries(entries: CanonicalEntries) -> Result<Self, RegistryError> {
        let mut registry = Self::new();
        for official in entries.officials {
            registry.register_official(official)?;
        }
        for outlaw in entries.outlaws {
            registry.register_outlaw(outlaw)?;
        }
        for form in entries.forms {
            registry.register_form(form)?;
        }
        for lineage in entries.lineages {
            registry.register_lineage(lineage)?;
        }
        for synthesis_base in entries.synthesis_bases {
            registry.register_synthesis_base(synthesis_base)?;
        }
        for relation in entries.lineage_relations {
            registry.register_lineage_relation(relation)?;
        }
        for office in entries.offices {
            registry.register_office(office)?;
        }
        for mirror in entries.mirrors {
            registry.register_mirror_pair(mirror)?;
        }
        for recipe in entries.recipes {
            registry.register_recipe(recipe)?;
        }
        for mastery in entries.executive_masteries {
            registry.register_executive_mastery(mastery)?;
        }
        for recognition in entries.recognitions {
            registry.register_recognition(recognition)?;
        }
        for accession in entries.accessions {
            registry.register_accession(accession)?;
        }
        Ok(registry)
    }

    pub fn register_official(
        &mut self,
        official: OfficialInstitution,
    ) -> Result<(), RegistryError> {
        if self.official(&official.id).is_some() {
            return Err(RegistryError::DuplicateOfficial(official.id));
        }
        self.officials.push(official);
        Ok(())
    }

    pub fn register_outlaw(&mut self, outlaw: OutlawExpression) -> Result<(), RegistryError> {
        if self.outlaw(&outlaw.id).is_some() {
            return Err(RegistryError::DuplicateOutlaw(outlaw.id));
        }
        self.outlaws.push(outlaw);
        Ok(())
    }

    pub fn register_form(&mut self, form: TransformationForm) -> Result<(), RegistryError> {
        if self.form(&form.id).is_some() {
            return Err(RegistryError::DuplicateForm(form.id));
        }
        self.composition.insert_node(CompositionNode {
            id: form.id.as_kernel().clone(),
            object: ExternalRef::new("officials-outlaws", form.id.as_str())
                .expect("the domain namespace is a valid kernel external reference"),
            scale: ScaleKey::new("scale.transformation-form")
                .expect("the form scale is a valid kernel stable key"),
        })?;
        self.forms.push(form);
        Ok(())
    }

    pub fn register_lineage(
        &mut self,
        lineage: TransformationLineage,
    ) -> Result<(), RegistryError> {
        if self.lineage(&lineage.id).is_some() {
            return Err(RegistryError::DuplicateLineage(lineage.id));
        }
        if self.form(&lineage.base_form).is_none() {
            return Err(RegistryError::UnknownForm(lineage.base_form));
        }
        self.lineages.push(lineage);
        Ok(())
    }

    pub fn register_synthesis_base(
        &mut self,
        synthesis_base: SynthesisBase,
    ) -> Result<(), RegistryError> {
        if self
            .synthesis_bases
            .iter()
            .any(|entry| entry.lineage == synthesis_base.lineage)
        {
            return Err(RegistryError::DuplicateSynthesisBase(
                synthesis_base.lineage,
            ));
        }
        if self.lineage(&synthesis_base.lineage).is_none() {
            return Err(RegistryError::UnknownLineage(synthesis_base.lineage));
        }
        if self.form(&synthesis_base.form).is_none() {
            return Err(RegistryError::UnknownForm(synthesis_base.form));
        }
        self.synthesis_bases.push(synthesis_base);
        Ok(())
    }

    pub fn register_lineage_relation(
        &mut self,
        relation: OutlawLineageRelation,
    ) -> Result<(), RegistryError> {
        if self
            .lineage_relations
            .iter()
            .any(|entry| entry.outlaw == relation.outlaw && entry.lineage == relation.lineage)
        {
            return Err(RegistryError::DuplicateLineageRelation {
                outlaw: relation.outlaw,
                lineage: relation.lineage,
            });
        }
        if self.outlaw(&relation.outlaw).is_none() {
            return Err(RegistryError::UnknownOutlaw(relation.outlaw));
        }
        if self.lineage(&relation.lineage).is_none() {
            return Err(RegistryError::UnknownLineage(relation.lineage));
        }
        self.lineage_relations.push(relation);
        Ok(())
    }

    pub fn register_office(&mut self, office: ConstitutionalOffice) -> Result<(), RegistryError> {
        if self.office(&office.id).is_some() {
            return Err(RegistryError::DuplicateOffice(office.id));
        }
        self.offices.push(office);
        Ok(())
    }

    pub fn register_mirror_pair(&mut self, pair: MirrorPair) -> Result<(), RegistryError> {
        if pair.official.as_str() == pair.outlaw.as_str() {
            return Err(RegistryError::SameMirrorEntity(
                pair.official.as_str().to_owned(),
            ));
        }
        if self.official(&pair.official).is_none() {
            return Err(RegistryError::UnknownOfficial(pair.official));
        }
        if self.outlaw(&pair.outlaw).is_none() {
            return Err(RegistryError::UnknownOutlaw(pair.outlaw));
        }
        if self.mirrors.iter().any(|entry| entry.id == pair.id) {
            return Err(RegistryError::DuplicateMirrorId(pair.id));
        }
        if self
            .mirrors
            .iter()
            .any(|entry| entry.official == pair.official && entry.outlaw == pair.outlaw)
        {
            return Err(RegistryError::DuplicateMirrorPair {
                official: pair.official,
                outlaw: pair.outlaw,
            });
        }
        self.mirrors.push(pair);
        Ok(())
    }

    /// Accepts either endpoint order, normalizes it to one semantic pair, and
    /// therefore rejects reverse duplicates exactly like forward duplicates.
    pub fn register_mirror_between(
        &mut self,
        id: MirrorPairId,
        left: ConstitutionalEntityId,
        right: ConstitutionalEntityId,
        shared_function: SharedFunction,
        distinction: impl Into<String>,
    ) -> Result<(), RegistryError> {
        if left == right {
            return Err(RegistryError::SameMirrorEntity(left.as_str().to_owned()));
        }
        let (official, outlaw) = match (left, right) {
            (
                ConstitutionalEntityId::Official(official),
                ConstitutionalEntityId::Outlaw(outlaw),
            )
            | (
                ConstitutionalEntityId::Outlaw(outlaw),
                ConstitutionalEntityId::Official(official),
            ) => (official, outlaw),
            _ => return Err(RegistryError::MirrorRequiresOppositeOrientations),
        };
        self.register_mirror_pair(MirrorPair {
            id,
            official,
            outlaw,
            shared_function,
            distinction: distinction.into(),
        })
    }

    pub fn register_recipe(&mut self, mut recipe: SynthesisRecipe) -> Result<(), RegistryError> {
        if self.recipe(&recipe.id).is_some() {
            return Err(RegistryError::DuplicateRecipe(recipe.id));
        }
        if recipe.result.as_str() == FORM_MANTICORP {
            return Err(RegistryError::ManticorpCannotBeDirectlySynthesized);
        }
        recipe
            .sources
            .sort_by(|left, right| left.as_str().cmp(right.as_str()));
        for duplicate in recipe.sources.windows(2) {
            if duplicate[0] == duplicate[1] {
                return Err(RegistryError::DuplicateRecipeSource {
                    recipe: recipe.id,
                    source: duplicate[0].clone(),
                });
            }
        }
        for source in &recipe.sources {
            if self.form(source).is_none() {
                return Err(RegistryError::UnknownForm(source.clone()));
            }
        }
        if self.form(&recipe.result).is_none() {
            return Err(RegistryError::UnknownForm(recipe.result));
        }

        self.composition.insert_record(CompositionRecord {
            id: recipe.id.as_kernel().clone(),
            sources: recipe
                .sources
                .iter()
                .map(|source| source.as_kernel().clone())
                .collect(),
            result: recipe.result.as_kernel().clone(),
            operation: ExternalRef::new("officials-outlaws", "constitutional-synthesis")
                .expect("the synthesis operation is a valid kernel external reference"),
            evidence: None,
        })?;
        self.recipes.push(recipe);
        Ok(())
    }

    pub fn register_executive_mastery(
        &mut self,
        mut mastery: ExecutiveMastery,
    ) -> Result<(), RegistryError> {
        if self
            .executive_masteries
            .iter()
            .any(|entry| entry.id == mastery.id)
        {
            return Err(RegistryError::DuplicateExecutiveMastery(mastery.id));
        }
        if self
            .executive_masteries
            .iter()
            .any(|entry| entry.refinement.id == mastery.refinement.id)
        {
            return Err(RegistryError::DuplicateChimeraRefinement(
                mastery.refinement.id,
            ));
        }
        mastery.refinement.perfected_aspects.sort_unstable();
        if !self.recipe_has_exact_sources(
            RECIPE_CHIMERA,
            &[FORM_GARGOYLE, FORM_WEREWOLF, FORM_MERMAN],
            FORM_CHIMERA,
        ) {
            return Err(RegistryError::ChimeraSynthesisRequired);
        }
        let chimera = FormId::new(FORM_CHIMERA).expect("canonical Chimera form ID");
        let manticorp = FormId::new(FORM_MANTICORP).expect("canonical Manticorp form ID");
        if self.form(&chimera).is_none() {
            return Err(RegistryError::UnknownForm(chimera));
        }
        if self.form(&manticorp).is_none() {
            return Err(RegistryError::UnknownForm(manticorp));
        }
        if mastery.completed_chimera.as_str() != FORM_CHIMERA
            || mastery.refinement.source != mastery.completed_chimera
            || mastery.refinement.result.as_str() != FORM_MANTICORP
            || mastery.resulting_manticorp != mastery.refinement.result
            || !has_canonical_mastery_aspects(&mastery.refinement.perfected_aspects)
            || mastery.refinement.evidence.is_empty()
            || has_invalid_witnesses(&mastery.refinement.evidence)
        {
            return Err(RegistryError::InvalidExecutiveMastery);
        }
        self.executive_masteries.push(mastery);
        Ok(())
    }

    pub fn register_recognition(
        &mut self,
        recognition: ConstitutionalRecognition,
    ) -> Result<(), RegistryError> {
        if self
            .recognitions
            .iter()
            .any(|entry| entry.id == recognition.id)
        {
            return Err(RegistryError::DuplicateRecognition(recognition.id));
        }
        if self.office(&recognition.office).is_none() {
            return Err(RegistryError::UnknownOffice(recognition.office));
        }
        if self.form(&recognition.achieved_manticorp).is_none() {
            return Err(RegistryError::UnknownForm(recognition.achieved_manticorp));
        }
        let mastery = self
            .executive_mastery(&recognition.mastery)
            .ok_or_else(|| {
                RegistryError::RecognitionRequiresMastery(recognition.mastery.clone())
            })?;
        if recognition.candidate != mastery.candidate
            || recognition.achieved_manticorp != mastery.resulting_manticorp
            || recognition.achieved_manticorp.as_str() != FORM_MANTICORP
            || recognition.office.as_str() != OFFICE_TROSS
            || recognition.witnesses.is_empty()
            || has_invalid_witnesses(&recognition.witnesses)
        {
            return Err(RegistryError::InvalidConstitutionalRecognition);
        }
        self.recognitions.push(recognition);
        Ok(())
    }

    pub fn register_accession(&mut self, accession: LawfulAccession) -> Result<(), RegistryError> {
        if self.accessions.iter().any(|entry| entry.id == accession.id) {
            return Err(RegistryError::DuplicateAccession(accession.id));
        }
        if self.office(&accession.office).is_none() {
            return Err(RegistryError::UnknownOffice(accession.office));
        }
        let recognition = self.recognition(&accession.recognition).ok_or_else(|| {
            RegistryError::AccessionRequiresRecognition(accession.recognition.clone())
        })?;
        if accession.candidate != recognition.candidate
            || accession.office != recognition.office
            || accession.office.as_str() != OFFICE_TROSS
        {
            return Err(RegistryError::InvalidLawfulAccession);
        }
        self.accessions.push(accession);
        Ok(())
    }

    #[must_use]
    pub fn officials(&self) -> &[OfficialInstitution] {
        &self.officials
    }

    #[must_use]
    pub fn outlaws(&self) -> &[OutlawExpression] {
        &self.outlaws
    }

    #[must_use]
    pub fn mirror_pairs(&self) -> &[MirrorPair] {
        &self.mirrors
    }

    #[must_use]
    pub fn lineages(&self) -> &[TransformationLineage] {
        &self.lineages
    }

    #[must_use]
    pub fn forms(&self) -> &[TransformationForm] {
        &self.forms
    }

    #[must_use]
    pub fn synthesis_bases(&self) -> &[SynthesisBase] {
        &self.synthesis_bases
    }

    #[must_use]
    pub fn lineage_relations(&self) -> &[OutlawLineageRelation] {
        &self.lineage_relations
    }

    #[must_use]
    pub fn offices(&self) -> &[ConstitutionalOffice] {
        &self.offices
    }

    #[must_use]
    pub fn recipes(&self) -> &[SynthesisRecipe] {
        &self.recipes
    }

    #[must_use]
    pub fn executive_masteries(&self) -> &[ExecutiveMastery] {
        &self.executive_masteries
    }

    #[must_use]
    pub fn recognitions(&self) -> &[ConstitutionalRecognition] {
        &self.recognitions
    }

    #[must_use]
    pub fn accessions(&self) -> &[LawfulAccession] {
        &self.accessions
    }

    /// Read-only access to the kernel-owned causal provenance projection.
    #[must_use]
    pub fn composition_catalog(&self) -> &CompositionCatalog {
        &self.composition
    }

    #[must_use]
    pub fn official(&self, id: &OfficialId) -> Option<&OfficialInstitution> {
        self.officials.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn outlaw(&self, id: &OutlawId) -> Option<&OutlawExpression> {
        self.outlaws.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn lineage(&self, id: &LineageId) -> Option<&TransformationLineage> {
        self.lineages.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn form(&self, id: &FormId) -> Option<&TransformationForm> {
        self.forms.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn office(&self, id: &ConstitutionalOfficeId) -> Option<&ConstitutionalOffice> {
        self.offices.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn recipe(&self, id: &SynthesisRecipeId) -> Option<&SynthesisRecipe> {
        self.recipes.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn executive_mastery(&self, id: &ExecutiveMasteryId) -> Option<&ExecutiveMastery> {
        self.executive_masteries
            .iter()
            .find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn recognition(
        &self,
        id: &ConstitutionalRecognitionId,
    ) -> Option<&ConstitutionalRecognition> {
        self.recognitions.iter().find(|entry| &entry.id == id)
    }

    #[must_use]
    pub fn lawfully_holds_office(
        &self,
        candidate: &PersonId,
        office: &ConstitutionalOfficeId,
    ) -> bool {
        self.accessions
            .iter()
            .any(|entry| &entry.candidate == candidate && &entry.office == office)
    }

    #[must_use]
    pub fn mirror_of(&self, entity: &ConstitutionalEntityId) -> Option<ConstitutionalEntityId> {
        self.mirrors
            .iter()
            .find_map(|pair| pair.counterpart(entity))
    }

    pub fn validate(&self) -> Result<(), RegistryError> {
        self.validate_rosters()?;

        let mut mirror_ids = HashSet::new();
        let mut mirror_endpoints = HashSet::new();
        for pair in &self.mirrors {
            if pair.official.as_str() == pair.outlaw.as_str() {
                return Err(RegistryError::SameMirrorEntity(
                    pair.official.as_str().to_owned(),
                ));
            }
            if !mirror_ids.insert(pair.id.clone()) {
                return Err(RegistryError::DuplicateMirrorId(pair.id.clone()));
            }
            if !mirror_endpoints.insert((pair.official.clone(), pair.outlaw.clone())) {
                return Err(RegistryError::DuplicateMirrorPair {
                    official: pair.official.clone(),
                    outlaw: pair.outlaw.clone(),
                });
            }
            let official = self
                .official(&pair.official)
                .ok_or_else(|| RegistryError::UnknownOfficial(pair.official.clone()))?;
            let outlaw = self
                .outlaw(&pair.outlaw)
                .ok_or_else(|| RegistryError::UnknownOutlaw(pair.outlaw.clone()))?;
            if pair.shared_function != official.function || pair.shared_function != outlaw.function
            {
                return Err(RegistryError::MirrorFunctionMismatch(pair.id.clone()));
            }
        }

        for official in &self.officials {
            let count = self
                .mirrors
                .iter()
                .filter(|pair| pair.official == official.id)
                .count();
            if count != 1 {
                return Err(RegistryError::OfficialMirrorCount {
                    official: official.id.clone(),
                    count,
                });
            }
        }
        for outlaw in &self.outlaws {
            let count = self
                .mirrors
                .iter()
                .filter(|pair| pair.outlaw == outlaw.id)
                .count();
            if count != 1 {
                return Err(RegistryError::OutlawMirrorCount {
                    outlaw: outlaw.id.clone(),
                    count,
                });
            }
        }

        self.validate_canonical_mirror(OFFICIAL_MANTICORP, OUTLAW_WEREWOLVES)?;
        self.validate_canonical_mirror(OFFICIAL_MYSTERY_MEN, OUTLAW_GALLOWS)?;
        self.validate_canonical_mirror(OFFICIAL_MYSTERYGUARD, OUTLAW_MERMEN)?;

        if OFFICIAL_MANTICORP == FORM_MANTICORP {
            return Err(RegistryError::ManticorpInstitutionFormCollision);
        }
        if FORM_CHIMERA == OFFICE_CHIMERA {
            return Err(RegistryError::ChimeraFormOfficeCollision);
        }
        if !self.has_canonical_foundational_bases() {
            return Err(RegistryError::InvalidFoundationalBases);
        }
        if !self.has_canonical_lineage_relations() {
            return Err(RegistryError::InvalidLineageRelations);
        }
        if !self.recipe_has_exact_sources(
            RECIPE_CHIMERA,
            &[FORM_GARGOYLE, FORM_WEREWOLF, FORM_MERMAN],
            FORM_CHIMERA,
        ) {
            return Err(RegistryError::InvalidChimeraRecipe);
        }
        let manticorp = FormId::new(FORM_MANTICORP).expect("canonical Manticorp form ID");
        if !self
            .composition
            .records_producing_result(manticorp.as_kernel())
            .is_empty()
        {
            return Err(RegistryError::ManticorpCannotBeDirectlySynthesized);
        }
        if !self.has_canonical_executive_mastery() {
            return Err(RegistryError::InvalidExecutiveMastery);
        }
        if !self.has_canonical_recognition() {
            return Err(RegistryError::InvalidConstitutionalRecognition);
        }
        if !self.has_canonical_accession() {
            return Err(RegistryError::InvalidLawfulAccession);
        }
        Ok(())
    }

    fn validate_rosters(&self) -> Result<(), RegistryError> {
        if !same_keys(
            self.officials.iter().map(|entry| entry.id.as_str()),
            &[
                OFFICIAL_MANTICORP,
                OFFICIAL_MYSTERY_MEN,
                OFFICIAL_MYSTERYGUARD,
            ],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("Official"));
        }
        if !same_keys(
            self.outlaws.iter().map(|entry| entry.id.as_str()),
            &[OUTLAW_WEREWOLVES, OUTLAW_GALLOWS, OUTLAW_MERMEN],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("Outlaw"));
        }
        if !same_keys(
            self.mirrors.iter().map(|entry| entry.id.as_str()),
            &[
                MIRROR_MANTICORP_WEREWOLVES,
                MIRROR_MYSTERY_MEN_GALLOWS,
                MIRROR_MYSTERYGUARD_MERMEN,
            ],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("mirror"));
        }
        if !same_keys(
            self.lineages.iter().map(|entry| entry.id.as_str()),
            &[LINEAGE_GARGOYLE, LINEAGE_WEREWOLF, LINEAGE_MERMAN],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("lineage"));
        }
        if !same_keys(
            self.forms.iter().map(|entry| entry.id.as_str()),
            &[
                FORM_GARGOYLE,
                FORM_WEREWOLF,
                FORM_MERMAN,
                FORM_CHIMERA,
                FORM_MANTICORP,
            ],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("form"));
        }
        if !same_keys(
            self.offices.iter().map(|entry| entry.id.as_str()),
            &[OFFICE_TROSS, OFFICE_CHIMERA],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("office"));
        }
        if !same_keys(
            self.recipes.iter().map(|entry| entry.id.as_str()),
            &[RECIPE_CHIMERA],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("recipe"));
        }
        if !same_keys(
            self.executive_masteries
                .iter()
                .map(|entry| entry.id.as_str()),
            &[MASTERY_CHIMERA_TO_MANTICORP],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("mastery"));
        }
        if !same_keys(
            self.recognitions.iter().map(|entry| entry.id.as_str()),
            &[RECOGNITION_TROSS_SUCCESSION],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("recognition"));
        }
        if !same_keys(
            self.accessions.iter().map(|entry| entry.id.as_str()),
            &[ACCESSION_TROSS],
        ) {
            return Err(RegistryError::CanonicalRosterMismatch("accession"));
        }
        Ok(())
    }

    fn validate_canonical_mirror(
        &self,
        official_key: &str,
        outlaw_key: &str,
    ) -> Result<(), RegistryError> {
        let official = OfficialId::new(official_key).expect("canonical Official ID");
        let expected = OutlawId::new(outlaw_key).expect("canonical Outlaw ID");
        if self.mirror_of(&official.clone().into()) != Some(expected.into()) {
            return Err(RegistryError::WrongCanonicalMirror(official));
        }
        Ok(())
    }

    fn has_canonical_foundational_bases(&self) -> bool {
        let expected = [
            (LINEAGE_GARGOYLE, FORM_GARGOYLE),
            (LINEAGE_WEREWOLF, FORM_WEREWOLF),
            (LINEAGE_MERMAN, FORM_MERMAN),
        ];
        self.synthesis_bases.len() == expected.len()
            && expected.iter().all(|(lineage, form)| {
                self.synthesis_bases
                    .iter()
                    .any(|entry| entry.lineage.as_str() == *lineage && entry.form.as_str() == *form)
            })
    }

    fn has_canonical_lineage_relations(&self) -> bool {
        let expected = [
            (
                OUTLAW_WEREWOLVES,
                LINEAGE_WEREWOLF,
                LineageRelationKind::DirectExpression,
            ),
            (
                OUTLAW_GALLOWS,
                LINEAGE_GARGOYLE,
                LineageRelationKind::Influence,
            ),
            (
                OUTLAW_MERMEN,
                LINEAGE_MERMAN,
                LineageRelationKind::DirectExpression,
            ),
        ];
        self.lineage_relations.len() == expected.len()
            && expected.iter().all(|(outlaw, lineage, kind)| {
                self.lineage_relations.iter().any(|entry| {
                    entry.outlaw.as_str() == *outlaw
                        && entry.lineage.as_str() == *lineage
                        && entry.kind == *kind
                })
            })
    }

    fn recipe_has_exact_sources(&self, recipe: &str, sources: &[&str], result: &str) -> bool {
        let id = SynthesisRecipeId::new(recipe).expect("canonical recipe ID");
        let Some(recipe) = self.recipe(&id) else {
            return false;
        };
        same_keys(recipe.sources.iter().map(FormId::as_str), sources)
            && recipe.result.as_str() == result
    }

    fn has_canonical_executive_mastery(&self) -> bool {
        self.executive_masteries.len() == 1
            && self.executive_masteries.iter().any(|mastery| {
                mastery.id.as_str() == MASTERY_CHIMERA_TO_MANTICORP
                    && mastery.candidate.as_str() == PERSON_CANONICAL_TROSS_CANDIDATE
                    && mastery.completed_chimera.as_str() == FORM_CHIMERA
                    && mastery.refinement.id.as_str() == REFINEMENT_CHIMERA_TO_MANTICORP
                    && mastery.refinement.source.as_str() == FORM_CHIMERA
                    && mastery.refinement.result.as_str() == FORM_MANTICORP
                    && mastery.resulting_manticorp.as_str() == FORM_MANTICORP
                    && has_canonical_mastery_aspects(&mastery.refinement.perfected_aspects)
                    && same_keys(
                        mastery
                            .refinement
                            .evidence
                            .iter()
                            .map(|entry| entry.id.as_str()),
                        &[WITNESS_CHIMERA_REFINEMENT],
                    )
            })
    }

    fn has_canonical_recognition(&self) -> bool {
        self.recognitions.len() == 1
            && self.recognitions.iter().any(|recognition| {
                recognition.id.as_str() == RECOGNITION_TROSS_SUCCESSION
                    && recognition.candidate.as_str() == PERSON_CANONICAL_TROSS_CANDIDATE
                    && recognition.mastery.as_str() == MASTERY_CHIMERA_TO_MANTICORP
                    && recognition.achieved_manticorp.as_str() == FORM_MANTICORP
                    && recognition.office.as_str() == OFFICE_TROSS
                    && same_keys(
                        recognition.witnesses.iter().map(|entry| entry.id.as_str()),
                        &[WITNESS_TROSS_RECOGNITION],
                    )
            })
    }

    fn has_canonical_accession(&self) -> bool {
        self.accessions.len() == 1
            && self.accessions.iter().any(|accession| {
                accession.id.as_str() == ACCESSION_TROSS
                    && accession.candidate.as_str() == PERSON_CANONICAL_TROSS_CANDIDATE
                    && accession.recognition.as_str() == RECOGNITION_TROSS_SUCCESSION
                    && accession.office.as_str() == OFFICE_TROSS
            })
    }
}

fn has_canonical_mastery_aspects(aspects: &[ManticorpMasteryAspect]) -> bool {
    let actual: HashSet<_> = aspects.iter().copied().collect();
    let expected = HashSet::from([
        ManticorpMasteryAspect::Lion,
        ManticorpMasteryAspect::Eagle,
        ManticorpMasteryAspect::Hydra,
    ]);
    aspects.len() == expected.len() && actual == expected
}

fn has_invalid_witnesses(witnesses: &[ConstitutionalWitness]) -> bool {
    let mut ids = HashSet::new();
    witnesses
        .iter()
        .any(|entry| entry.statement.trim().is_empty() || !ids.insert(&entry.id))
}

fn same_keys<'a>(actual: impl Iterator<Item = &'a str>, expected: &[&str]) -> bool {
    let actual: HashSet<_> = actual.collect();
    let expected: HashSet<_> = expected.iter().copied().collect();
    actual.len() == expected.len() && actual == expected
}

/// Canonical records are exposed as data so callers can choose registration
/// order without changing identity or meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalEntries {
    pub officials: Vec<OfficialInstitution>,
    pub outlaws: Vec<OutlawExpression>,
    pub mirrors: Vec<MirrorPair>,
    pub lineages: Vec<TransformationLineage>,
    pub forms: Vec<TransformationForm>,
    pub synthesis_bases: Vec<SynthesisBase>,
    pub lineage_relations: Vec<OutlawLineageRelation>,
    pub offices: Vec<ConstitutionalOffice>,
    pub recipes: Vec<SynthesisRecipe>,
    pub executive_masteries: Vec<ExecutiveMastery>,
    pub recognitions: Vec<ConstitutionalRecognition>,
    pub accessions: Vec<LawfulAccession>,
}

#[must_use]
pub fn canonical_entries() -> CanonicalEntries {
    let official_manticorp = OfficialId::new(OFFICIAL_MANTICORP).expect("canonical ID");
    let official_mystery_men = OfficialId::new(OFFICIAL_MYSTERY_MEN).expect("canonical ID");
    let official_mysteryguard = OfficialId::new(OFFICIAL_MYSTERYGUARD).expect("canonical ID");
    let outlaw_werewolves = OutlawId::new(OUTLAW_WEREWOLVES).expect("canonical ID");
    let outlaw_gallows = OutlawId::new(OUTLAW_GALLOWS).expect("canonical ID");
    let outlaw_mermen = OutlawId::new(OUTLAW_MERMEN).expect("canonical ID");

    let lineage_gargoyle = LineageId::new(LINEAGE_GARGOYLE).expect("canonical ID");
    let lineage_werewolf = LineageId::new(LINEAGE_WEREWOLF).expect("canonical ID");
    let lineage_merman = LineageId::new(LINEAGE_MERMAN).expect("canonical ID");

    let form_gargoyle = FormId::new(FORM_GARGOYLE).expect("canonical ID");
    let form_werewolf = FormId::new(FORM_WEREWOLF).expect("canonical ID");
    let form_merman = FormId::new(FORM_MERMAN).expect("canonical ID");
    let form_chimera = FormId::new(FORM_CHIMERA).expect("canonical ID");
    let form_manticorp = FormId::new(FORM_MANTICORP).expect("canonical ID");

    let office_tross = ConstitutionalOfficeId::new(OFFICE_TROSS).expect("canonical ID");
    let candidate =
        PersonId::new(PERSON_CANONICAL_TROSS_CANDIDATE).expect("canonical candidate ID");
    let mastery_id =
        ExecutiveMasteryId::new(MASTERY_CHIMERA_TO_MANTICORP).expect("canonical mastery ID");
    let recognition_id = ConstitutionalRecognitionId::new(RECOGNITION_TROSS_SUCCESSION)
        .expect("canonical recognition ID");

    CanonicalEntries {
        officials: vec![
            OfficialInstitution {
                id: official_manticorp.clone(),
                name: "Manticorp".into(),
                function: SharedFunction::MartialForce,
            },
            OfficialInstitution {
                id: official_mystery_men.clone(),
                name: "Mystery Men".into(),
                function: SharedFunction::HiddenThreatHuntingInvestigationAndControl,
            },
            OfficialInstitution {
                id: official_mysteryguard.clone(),
                name: "Mysteryguard".into(),
                function: SharedFunction::MaritimeMovementPursuitAndSovereignty,
            },
        ],
        outlaws: vec![
            OutlawExpression {
                id: outlaw_werewolves.clone(),
                name: "Werewolves".into(),
                function: SharedFunction::MartialForce,
                constitutional_expression: "pack-based, instinctive, non-state".into(),
            },
            OutlawExpression {
                id: outlaw_gallows.clone(),
                name: "Gallows".into(),
                function: SharedFunction::HiddenThreatHuntingInvestigationAndControl,
                constitutional_expression: "covert, clan-based, underworld".into(),
            },
            OutlawExpression {
                id: outlaw_mermen.clone(),
                name: "Mermen".into(),
                function: SharedFunction::MaritimeMovementPursuitAndSovereignty,
                constitutional_expression: "sovereign, crew-based, pirate or free-ocean".into(),
            },
        ],
        mirrors: vec![
            MirrorPair {
                id: MirrorPairId::new(MIRROR_MANTICORP_WEREWOLVES).expect("canonical ID"),
                official: official_manticorp,
                outlaw: outlaw_werewolves.clone(),
                shared_function: SharedFunction::MartialForce,
                distinction: "lawful military institution ↔ pack-based, instinctive, non-state expression".into(),
            },
            MirrorPair {
                id: MirrorPairId::new(MIRROR_MYSTERY_MEN_GALLOWS).expect("canonical ID"),
                official: official_mystery_men,
                outlaw: outlaw_gallows.clone(),
                shared_function: SharedFunction::HiddenThreatHuntingInvestigationAndControl,
                distinction: "lawful public hunters and investigators ↔ covert, clan-based, underworld expression".into(),
            },
            MirrorPair {
                id: MirrorPairId::new(MIRROR_MYSTERYGUARD_MERMEN).expect("canonical ID"),
                official: official_mysteryguard,
                outlaw: outlaw_mermen.clone(),
                shared_function: SharedFunction::MaritimeMovementPursuitAndSovereignty,
                distinction: "lawful maritime enforcement ↔ sovereign, crew-based, pirate or free-ocean expression".into(),
            },
        ],
        lineages: vec![
            TransformationLineage {
                id: lineage_gargoyle.clone(),
                name: "Gargoyle".into(),
                base_form: form_gargoyle.clone(),
            },
            TransformationLineage {
                id: lineage_werewolf.clone(),
                name: "Werewolf".into(),
                base_form: form_werewolf.clone(),
            },
            TransformationLineage {
                id: lineage_merman.clone(),
                name: "Merman".into(),
                base_form: form_merman.clone(),
            },
        ],
        forms: vec![
            TransformationForm {
                id: form_gargoyle.clone(),
                name: "Gargoyle".into(),
                kind: FormKind::Foundational,
            },
            TransformationForm {
                id: form_werewolf.clone(),
                name: "Werewolf".into(),
                kind: FormKind::Foundational,
            },
            TransformationForm {
                id: form_merman.clone(),
                name: "Merman".into(),
                kind: FormKind::Foundational,
            },
            TransformationForm {
                id: form_chimera.clone(),
                name: "Chimera".into(),
                kind: FormKind::HigherSynthesis,
            },
            TransformationForm {
                id: form_manticorp.clone(),
                name: "Manticorp Form".into(),
                kind: FormKind::PerfectedComposite,
            },
        ],
        synthesis_bases: vec![
            SynthesisBase {
                lineage: lineage_gargoyle.clone(),
                form: form_gargoyle,
                name: "Gargoyle".into(),
            },
            SynthesisBase {
                lineage: lineage_werewolf.clone(),
                form: form_werewolf,
                name: "Werewolf".into(),
            },
            SynthesisBase {
                lineage: lineage_merman.clone(),
                form: form_merman,
                name: "Merman".into(),
            },
        ],
        lineage_relations: vec![
            OutlawLineageRelation {
                outlaw: outlaw_werewolves,
                lineage: lineage_werewolf,
                kind: LineageRelationKind::DirectExpression,
            },
            OutlawLineageRelation {
                outlaw: outlaw_gallows,
                lineage: lineage_gargoyle,
                kind: LineageRelationKind::Influence,
            },
            OutlawLineageRelation {
                outlaw: outlaw_mermen,
                lineage: lineage_merman,
                kind: LineageRelationKind::DirectExpression,
            },
        ],
        offices: vec![
            ConstitutionalOffice {
                id: office_tross.clone(),
                title: "Tross".into(),
                kind: ConstitutionalOfficeKind::Executive,
            },
            ConstitutionalOffice {
                id: ConstitutionalOfficeId::new(OFFICE_CHIMERA).expect("canonical ID"),
                title: "Chimera".into(),
                kind: ConstitutionalOfficeKind::SecondInCommand,
            },
        ],
        recipes: vec![SynthesisRecipe {
            id: SynthesisRecipeId::new(RECIPE_CHIMERA).expect("canonical ID"),
            sources: vec![
                FormId::new(FORM_GARGOYLE).expect("canonical ID"),
                FormId::new(FORM_WEREWOLF).expect("canonical ID"),
                FormId::new(FORM_MERMAN).expect("canonical ID"),
            ],
            result: form_chimera.clone(),
        }],
        executive_masteries: vec![ExecutiveMastery {
            id: mastery_id.clone(),
            candidate: candidate.clone(),
            completed_chimera: form_chimera.clone(),
            refinement: ChimeraRefinement {
                id: ChimeraRefinementId::new(REFINEMENT_CHIMERA_TO_MANTICORP)
                    .expect("canonical refinement ID"),
                source: form_chimera,
                perfected_aspects: vec![
                    ManticorpMasteryAspect::Lion,
                    ManticorpMasteryAspect::Eagle,
                    ManticorpMasteryAspect::Hydra,
                ],
                result: form_manticorp.clone(),
                evidence: vec![ConstitutionalWitness {
                    id: ConstitutionalWitnessId::new(WITNESS_CHIMERA_REFINEMENT)
                        .expect("canonical mastery witness ID"),
                    statement: "completed Chimera refinement into the Manticorp Form".into(),
                }],
            },
            resulting_manticorp: form_manticorp.clone(),
        }],
        recognitions: vec![ConstitutionalRecognition {
            id: recognition_id.clone(),
            candidate: candidate.clone(),
            mastery: mastery_id,
            achieved_manticorp: form_manticorp,
            office: office_tross.clone(),
            witnesses: vec![ConstitutionalWitness {
                id: ConstitutionalWitnessId::new(WITNESS_TROSS_RECOGNITION)
                    .expect("canonical recognition witness ID"),
                statement: "recognized previously proven Manticorp mastery".into(),
            }],
        }],
        accessions: vec![LawfulAccession {
            id: LawfulAccessionId::new(ACCESSION_TROSS).expect("canonical accession ID"),
            candidate,
            recognition: recognition_id,
            office: office_tross,
        }],
    }
}

pub fn canonical_registry() -> Result<OfficialsOutlawsRegistry, RegistryError> {
    let registry = OfficialsOutlawsRegistry::from_entries(canonical_entries())?;
    registry.validate()?;
    Ok(registry)
}
