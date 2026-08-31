//! Canonical Flynt constitutional domain.
//!
//! The directory name is retained only as a source-tree migration boundary.
//! This crate no longer models the former mirrored opposition. Flynt has one
//! sovereign executive, Tross; one constitutional companion, Chimera; and two
//! complementary institutional expressions beneath that same authority.
//!
//! The neutral `hollow-grove-semantic-kernel` remains responsible only for composition
//! provenance. This crate owns the Flynt meaning of the unique Chimera recipe
//! and the institutional hierarchy surrounding it.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;

use hollow_grove_semantic_kernel::{
    CompositionCatalog, CompositionCatalogError, CompositionNode, CompositionNodeId,
    CompositionRecord, CompositionRecordId, ExternalRef, ScaleKey, StableKeyError,
};

pub const OFFICE_TROSS: &str = "flynt.office.tross";
pub const PERSON_TROSS: &str = "flynt.person.tross";
pub const IDENTITY_MYSTERY_MAN: &str = "flynt.identity.mystery-man";
pub const IDENTITY_MR_X: &str = "flynt.identity.mr-x";
pub const COMPANION_CHIMERA: &str = "flynt.companion.chimera";

pub const INSTITUTION_MANTICORP: &str = "flynt.institution.manticorp";
pub const INSTITUTION_MYSTERY_MEN: &str = "flynt.institution.mystery-men";
pub const EXTENSION_MARQUISE: &str = "flynt.extension.marquise";
pub const EXPRESSION_MYSTERY_MAN: &str = "flynt.expression.the-mystery-man";

pub const INSTITUTION_GALLOWS: &str = "flynt.institution.gallows";
pub const EXPRESSION_WE_FAIRY_MEN: &str = "flynt.expression.we-fairy-men";
pub const SITE_GALLOWRY: &str = "flynt.site.gallowry";

pub const OFFICE_BRO_WHITE: &str = "flynt.office.bro-white";
pub const CREW_BRO_WHITE: &str = "flynt.crew.bro-white-and-the-7-brothas";
pub const OFFICE_CINDERELLAMAN: &str = "flynt.office.cinderellaman";
pub const CREW_CINDERELLAMAN: &str = "flynt.crew.cinderellaman-and-his-midnight-crew";
pub const OFFICE_THE_BEAUTY: &str = "flynt.office.the-beauty";
pub const CREW_THE_BEAUTY: &str = "flynt.crew.the-beauty-and-his-beasts";

/// Canonical name alias. The older stable key retains `the-beauty` for replay
/// and source compatibility, but the collective leader's name is Beauty.
pub const OFFICE_BEAUTY: &str = OFFICE_THE_BEAUTY;

pub const FORM_GARGOYLE: &str = "flynt.form.gargoyle";
pub const FORM_MERMAN: &str = "flynt.form.merman";
/// Flynt's canonical creature name is Mariner; the older form key remains
/// `merman` because Mariner and Merman are the same Flynt creature identity.
pub const FORM_MARINER: &str = FORM_MERMAN;
pub const FORM_WEREWOLF: &str = "flynt.form.werewolf";
pub const FORM_CHIMERA: &str = "flynt.form.chimera";
pub const FORM_MANTICORE: &str = "flynt.form.manticore";
pub const RECIPE_CHIMERA: &str = "flynt.recipe.constitutional-chimera";
/// Legacy provenance key retained for recipe/archive compatibility. The
/// `manticorp` token in this ID does not name the result Form: the recipe is
/// typed and displayed as Manticore, while Manticorp is an organization.
pub const RECIPE_MANTICORE: &str = "flynt.recipe.divided-manticorp";
pub const RECIPE_GARGOYLE_CONTINUANCE: &str = "flynt.recipe.gargoyle-continuance";
pub const WAY_GREMLIN: &str = "flynt.way.gremlin";
pub const TOKEN_GREMLINCOIN: &str = "flynt.token.gremlincoin";
pub const GREMLINCOIN_MEANING: &str = WAY_GREMLIN;

const FLYNT_NAMESPACE: &str = "flynt.";
const FORM_NAMESPACE: &str = "flynt.form.";
const RECIPE_NAMESPACE: &str = "flynt.recipe.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlyntIdError {
    InvalidStableKey(StableKeyError),
    WrongNamespace {
        value: String,
        expected: &'static str,
    },
}

impl fmt::Display for FlyntIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStableKey(error) => error.fmt(formatter),
            Self::WrongNamespace { value, expected } => {
                write!(formatter, "{value} is outside the {expected} namespace")
            }
        }
    }
}

impl std::error::Error for FlyntIdError {}

fn has_namespace(value: &str, namespace: &'static str) -> bool {
    value
        .strip_prefix(namespace)
        .is_some_and(|remainder| !remainder.is_empty())
}

macro_rules! flynt_id {
    ($name:ident, $inner:ty, $namespace:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name($inner);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, FlyntIdError> {
                let value = value.into();
                let inner = <$inner>::new(value.clone()).map_err(FlyntIdError::InvalidStableKey)?;
                if !has_namespace(&value, $namespace) {
                    return Err(FlyntIdError::WrongNamespace {
                        value,
                        expected: $namespace,
                    });
                }
                Ok(Self(inner))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[must_use]
            pub fn as_kernel(&self) -> &$inner {
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

flynt_id!(FlyntNodeId, ScaleKey, FLYNT_NAMESPACE);
flynt_id!(FormId, CompositionNodeId, FORM_NAMESPACE);
flynt_id!(SynthesisRecipeId, CompositionRecordId, RECIPE_NAMESPACE);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalBranch {
    Common,
    Urban,
    Rural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalVisibility {
    Public,
    PublicWithClassifiedOperations,
    PrivateAndExistenceDisputed,
    Hidden,
    Legendary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlyntNodeKind {
    SovereignExecutive,
    ConstitutionalCompanion,
    MilitaryInstitution,
    InvestigativeBureau,
    MaritimeExtension,
    CriminalSyndicate,
    LegendaryOperative,
    LeadershipBody,
    /// Superseded classification retained for source compatibility.
    FolkExpression,
    FoundingLeaderOffice,
    Crew,
}

impl FlyntNodeKind {
    #[must_use]
    pub const fn is_institution(self) -> bool {
        matches!(
            self,
            Self::MilitaryInstitution | Self::InvestigativeBureau | Self::CriminalSyndicate
        )
    }

    #[must_use]
    pub const fn is_office(self) -> bool {
        matches!(self, Self::SovereignExecutive | Self::FoundingLeaderOffice)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstitutionalResponsibility {
    TerritorialDefense,
    MilitaryCommand,
    ConstitutionalProtection,
    DisciplinedForce,
    MilitaryTraining,
    LawfulDeployment,
    Investigation,
    Intelligence,
    Counterintelligence,
    CovertOperations,
    OrganizedCrimeInvestigation,
    Contraband,
    Espionage,
    ConstitutionalSecurity,
    Navigation,
    MaritimeOperations,
    Communications,
    ComplexSystemControl,
    OrganizedCrime,
    RegionalCrews,
    Loyalty,
    Territory,
    Favors,
    Obligation,
    CulturalIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlyntAuthorityNode {
    pub id: FlyntNodeId,
    pub name: String,
    pub kind: FlyntNodeKind,
    pub branch: ConstitutionalBranch,
    pub superior: Option<FlyntNodeId>,
    pub visibility: ConstitutionalVisibility,
    pub responsibilities: Vec<InstitutionalResponsibility>,
}

/// Stable source compatibility for the superseded regional-people model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoundingPeople {
    Gargoyle,
    Merman,
    Werewolf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PractitionerTradition {
    Warlock,
    Lycan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreatureSex {
    Masculine,
    Feminine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlyntCreature {
    Gargoyle,
    Medusa,
    Werewolf,
    Vampire,
    Mariner,
    Mermaid,
    Manticore,
    Chimera,
}

impl FlyntCreature {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Gargoyle => "Gargoyle",
            Self::Medusa => "Medusa",
            Self::Werewolf => "Werewolf",
            Self::Vampire => "Vampire",
            Self::Mariner => "Mariner",
            Self::Mermaid => "Mermaid",
            Self::Manticore => "Manticore",
            Self::Chimera => "Chimera",
        }
    }

    #[must_use]
    pub const fn sex(self) -> CreatureSex {
        match self {
            Self::Gargoyle | Self::Werewolf | Self::Mariner | Self::Manticore => {
                CreatureSex::Masculine
            }
            Self::Medusa | Self::Vampire | Self::Mermaid | Self::Chimera => CreatureSex::Feminine,
        }
    }

    #[must_use]
    pub const fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::Mariner => &["Merman"],
            _ => &[],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlyntCreaturePair {
    pub masculine: FlyntCreature,
    pub feminine: FlyntCreature,
    pub symbolic_connection: &'static str,
}

pub const FLYNT_CREATURE_PAIRS: [FlyntCreaturePair; 4] = [
    FlyntCreaturePair {
        masculine: FlyntCreature::Gargoyle,
        feminine: FlyntCreature::Medusa,
        symbolic_connection: "stone embodied and stone imposed through petrification",
    },
    FlyntCreaturePair {
        masculine: FlyntCreature::Werewolf,
        feminine: FlyntCreature::Vampire,
        symbolic_connection: "complementary nocturnal predatory traditions",
    },
    FlyntCreaturePair {
        masculine: FlyntCreature::Mariner,
        feminine: FlyntCreature::Mermaid,
        symbolic_connection: "paired maritime creature traditions",
    },
    FlyntCreaturePair {
        masculine: FlyntCreature::Manticore,
        feminine: FlyntCreature::Chimera,
        symbolic_connection: "sovereign and First Companion apex counterparts",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineageAssociation {
    SouthernFlynt,
    Stone,
    Architecture,
    Guardianship,
    Riptide,
    AuraSea,
    Waterways,
    Smuggling,
    MidnightTransformation,
    NorthernFlynt,
    Wilderness,
    Packs,
    RoamingProtection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundingLineage {
    pub people: FoundingPeople,
    pub founding_leader: FlyntNodeId,
    pub crew: FlyntNodeId,
    pub associations: Vec<LineageAssociation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PractitionerLineage {
    /// `None` is canonical for the maritime lineage: Mariner is the masculine
    /// creature designation (and Flynt's name for Merman), not merely a human
    /// sailor or a human practitioner-tradition name.
    pub tradition: Option<PractitionerTradition>,
    pub creature: FlyntCreature,
    pub synthesis_form: FormId,
    pub founding_leader: FlyntNodeId,
    pub crew: FlyntNodeId,
    pub associations: Vec<LineageAssociation>,
    pub working_function: &'static str,
    /// `None` is authoritative: exact base Current/Aura equations are OPEN.
    pub exact_current_aura_equation: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GallowryFunction {
    MeetingPlace,
    Headquarters,
    CulturalCenter,
    Gallery,
    OperationalHub,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GallowrySite {
    pub id: FlyntNodeId,
    pub name: String,
    pub controlled_by: FlyntNodeId,
    pub visibility: ConstitutionalVisibility,
    pub functions: Vec<GallowryFunction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormKind {
    /// Superseded active classification retained as a stable source variant.
    FoundingPeople,
    PractitionerSynthesis,
    ConstitutionalSynthesis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformationForm {
    pub id: FormId,
    pub name: String,
    pub kind: FormKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisRecipe {
    pub id: SynthesisRecipeId,
    pub sources: Vec<FormId>,
    pub result: FormId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManticoreRecipeComponent {
    Gargoyle,
    Werewolf,
    Mariner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasinCreatureSect {
    pub leader: &'static str,
    pub crew: &'static str,
    pub pair: FlyntCreaturePair,
}

pub const BASIN_CREATURE_SECTS: [BasinCreatureSect; 3] = [
    BasinCreatureSect {
        leader: OFFICE_BRO_WHITE,
        crew: CREW_BRO_WHITE,
        pair: FLYNT_CREATURE_PAIRS[0],
    },
    BasinCreatureSect {
        leader: OFFICE_BEAUTY,
        crew: CREW_THE_BEAUTY,
        pair: FLYNT_CREATURE_PAIRS[1],
    },
    BasinCreatureSect {
        leader: OFFICE_CINDERELLAMAN,
        crew: CREW_CINDERELLAMAN,
        pair: FLYNT_CREATURE_PAIRS[2],
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManticoreRecipeCustody {
    pub custodian: FlyntNodeId,
    pub component: ManticoreRecipeComponent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DividedManticoreRecipe {
    pub recipe: SynthesisRecipe,
    pub custody: Vec<ManticoreRecipeCustody>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ManticoreContinuanceRequirement {
    BodilyDiscipline,
    RecipeRenewal,
    DividedBasinKnowledge,
    InstitutionalRecognition,
    SpecializedGlaushouseCare,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlyntSovereignIdentity {
    pub person: FlyntNodeId,
    pub public_title: FlyntNodeId,
    pub underground_identities: Vec<FlyntNodeId>,
    pub maintained_form: FormId,
    pub mythological_identity: FlyntCreature,
    pub continuance_requirements: Vec<ManticoreContinuanceRequirement>,
    pub public_institution: FlyntNodeId,
    pub underground_institution: FlyntNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GremlinWayPractice {
    Salvage,
    Improvisation,
    Risk,
    Mobility,
    LowResourceAdaptation,
    DiscoverAbandonedValue,
    FrontierLabor,
    CreateOpportunityFromDiscardedMaterial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GremlincoinRecord {
    pub hueman: FlyntNodeId,
    pub token: &'static str,
    pub way: &'static str,
    pub practices: BTreeSet<GremlinWayPractice>,
    pub objective_value_created: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GargoyleContinuance {
    pub hueman: FlyntNodeId,
    pub gremlincoin: GremlincoinRecord,
    pub recipe: &'static str,
    pub recipe_viable: bool,
    pub synthesis_established: bool,
    pub maintained_structure: bool,
    pub territory: bool,
    pub responsibility: bool,
    pub maintenance_current: bool,
    pub renewal_current: bool,
}

impl GargoyleContinuance {
    #[must_use]
    pub fn validates(&self) -> bool {
        self.hueman == self.gremlincoin.hueman
            && self.gremlincoin.token == TOKEN_GREMLINCOIN
            && self.gremlincoin.way == WAY_GREMLIN
            && self.gremlincoin.practices == canonical_gremlin_way_practices()
            && !self.gremlincoin.objective_value_created.is_empty()
            && self
                .gremlincoin
                .objective_value_created
                .iter()
                .all(|entry| !entry.trim().is_empty())
            && self.recipe == RECIPE_GARGOYLE_CONTINUANCE
            && self.recipe_viable
            && self.synthesis_established
            && self.maintained_structure
            && self.territory
            && self.responsibility
            && self.maintenance_current
            && self.renewal_current
    }
}

#[must_use]
pub fn canonical_gremlin_way_practices() -> BTreeSet<GremlinWayPractice> {
    BTreeSet::from([
        GremlinWayPractice::Salvage,
        GremlinWayPractice::Improvisation,
        GremlinWayPractice::Risk,
        GremlinWayPractice::Mobility,
        GremlinWayPractice::LowResourceAdaptation,
        GremlinWayPractice::DiscoverAbandonedValue,
        GremlinWayPractice::FrontierLabor,
        GremlinWayPractice::CreateOpportunityFromDiscardedMaterial,
    ])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalChimera {
    pub authority_node: FlyntNodeId,
    pub form: FormId,
    pub mythological_identity: FlyntCreature,
    pub synthesis: SynthesisRecipeId,
    pub first_companion_to: FlyntNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundingUnion {
    pub founding_leaders: Vec<FlyntNodeId>,
    pub leadership_body: FlyntNodeId,
    pub sovereign_leader: FlyntNodeId,
    pub underworld_manifestation: FlyntNodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlyntConstitutionParts {
    pub nodes: Vec<FlyntAuthorityNode>,
    pub lineages: Vec<PractitionerLineage>,
    pub forms: Vec<TransformationForm>,
    pub chimera_recipe: SynthesisRecipe,
    pub manticore_recipe: DividedManticoreRecipe,
    pub creature_pairs: Vec<FlyntCreaturePair>,
    pub basin_sects: Vec<BasinCreatureSect>,
    pub chimera: ConstitutionalChimera,
    pub sovereign_identity: FlyntSovereignIdentity,
    pub founding_union: FoundingUnion,
    pub gallowry: GallowrySite,
}

#[derive(Debug)]
pub struct FlyntConstitution {
    nodes: Vec<FlyntAuthorityNode>,
    lineages: Vec<PractitionerLineage>,
    forms: Vec<TransformationForm>,
    chimera_recipe: SynthesisRecipe,
    manticore_recipe: DividedManticoreRecipe,
    creature_pairs: Vec<FlyntCreaturePair>,
    basin_sects: Vec<BasinCreatureSect>,
    chimera: ConstitutionalChimera,
    sovereign_identity: FlyntSovereignIdentity,
    founding_union: FoundingUnion,
    gallowry: GallowrySite,
    composition: CompositionCatalog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityPlacement {
    pub id: FlyntNodeId,
    pub superior: Option<FlyntNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlyntConstitutionalAudit {
    pub sovereign_executive: FlyntNodeId,
    pub constitutional_chimera_count: usize,
    pub institution_placements: Vec<AuthorityPlacement>,
    pub office_placements: Vec<AuthorityPlacement>,
    pub all_non_root_nodes_have_one_superior: bool,
    pub hierarchy_is_acyclic: bool,
    pub all_authority_reaches_tross: bool,
    pub duplicate_authority_count: usize,
    pub gallowry_is_distinct_from_gallows: bool,
    pub chimera_recipe_count: usize,
    pub manticore_recipe_count: usize,
    pub founding_union_is_complete: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConstitutionError {
    Composition(CompositionCatalogError),
    DuplicateNode(FlyntNodeId),
    MissingCanonicalNode(&'static str),
    UnexpectedNode(FlyntNodeId),
    CanonicalNodeMismatch(FlyntNodeId),
    RootHasSuperior,
    NonRootMissingSuperior(FlyntNodeId),
    MissingSuperior {
        node: FlyntNodeId,
        superior: FlyntNodeId,
    },
    AuthorityCycle(FlyntNodeId),
    ChimeraMustBeUnique,
    ChimeraIsNotFirstCompanion,
    InvalidChimeraRecipe,
    InvalidManticoreRecipe,
    InvalidCreaturePairs,
    InvalidBasinSects,
    InvalidSovereignIdentity,
    InvalidFoundingLineages,
    InvalidFoundingUnion,
    GallowryIsNotDistinct,
    InvalidGallowry,
}

impl fmt::Display for ConstitutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Composition(error) => error.fmt(formatter),
            Self::DuplicateNode(id) => write!(formatter, "duplicate Flynt authority node: {id}"),
            Self::MissingCanonicalNode(id) => write!(formatter, "missing canonical Flynt node: {id}"),
            Self::UnexpectedNode(id) => write!(formatter, "unexpected Flynt authority node: {id}"),
            Self::CanonicalNodeMismatch(id) => {
                write!(formatter, "Flynt authority node differs from canon: {id}")
            }
            Self::RootHasSuperior => formatter.write_str("Tross must not have a superior"),
            Self::NonRootMissingSuperior(id) => {
                write!(formatter, "non-root Flynt authority node has no superior: {id}")
            }
            Self::MissingSuperior { node, superior } => {
                write!(formatter, "{node} names missing superior {superior}")
            }
            Self::AuthorityCycle(id) => write!(formatter, "Flynt authority cycle reaches {id}"),
            Self::ChimeraMustBeUnique => {
                formatter.write_str("Flynt must contain exactly one constitutional Chimera")
            }
            Self::ChimeraIsNotFirstCompanion => formatter.write_str(
                "the unique constitutional Chimera must be First Companion directly beneath Tross",
            ),
            Self::InvalidChimeraRecipe => formatter.write_str(
                "Chimera must be the unique synthesis of Gargoyle, Mariner/Merman, and Werewolf",
            ),
            Self::InvalidManticoreRecipe => formatter.write_str(
                "Manticore must be the divided maintained synthesis beyond Chimera",
            ),
            Self::InvalidCreaturePairs => formatter.write_str(
                "Flynt must preserve the four canonical masculine/feminine creature pairs",
            ),
            Self::InvalidBasinSects => formatter.write_str(
                "Bro White, Beauty, and Cinderellaman must lead their canonical creature-pair sects",
            ),
            Self::InvalidSovereignIdentity => formatter.write_str(
                "Tross, Mystery Man, Mr. X, and the Manticore bearer must be one stable identity",
            ),
            Self::InvalidFoundingLineages => formatter.write_str(
                "Flynt must preserve Warlock/Gargoyle, Lycan/Werewolf, and the Mariner/Merman creature lineage",
            ),
            Self::InvalidFoundingUnion => formatter.write_str(
                "Bro White, Beauty, and Cinderellaman must form We Fairy Men beneath Tross, with the Gallows as their underworld manifestation",
            ),
            Self::GallowryIsNotDistinct => {
                formatter.write_str("the Gallowry site must not be synonymous with the Gallows")
            }
            Self::InvalidGallowry => formatter.write_str(
                "the hidden Gallowry headquarters must belong to the Gallows and retain its canonical functions",
            ),
        }
    }
}

impl std::error::Error for ConstitutionError {}

impl From<CompositionCatalogError> for ConstitutionError {
    fn from(value: CompositionCatalogError) -> Self {
        Self::Composition(value)
    }
}

impl FlyntConstitution {
    pub fn from_parts(parts: FlyntConstitutionParts) -> Result<Self, ConstitutionError> {
        let mut composition = CompositionCatalog::new();
        for form in &parts.forms {
            composition.insert_node(CompositionNode {
                id: form.id.as_kernel().clone(),
                object: ExternalRef::new("flynt-constitution", form.id.as_str())
                    .expect("canonical Flynt form reference"),
                scale: ScaleKey::new("hollow-grove.form").expect("canonical form scale"),
            })?;
        }
        composition.insert_record(CompositionRecord {
            id: parts.chimera_recipe.id.as_kernel().clone(),
            sources: parts
                .chimera_recipe
                .sources
                .iter()
                .map(|source| source.as_kernel().clone())
                .collect(),
            result: parts.chimera_recipe.result.as_kernel().clone(),
            operation: ExternalRef::new("flynt-constitution", "constitutional-synthesis")
                .expect("canonical Flynt synthesis operation"),
            evidence: None,
        })?;
        composition.insert_record(CompositionRecord {
            id: parts.manticore_recipe.recipe.id.as_kernel().clone(),
            sources: parts
                .manticore_recipe
                .recipe
                .sources
                .iter()
                .map(|source| source.as_kernel().clone())
                .collect(),
            result: parts.manticore_recipe.recipe.result.as_kernel().clone(),
            operation: ExternalRef::new("flynt-constitution", "sovereign-transfiguration")
                .expect("canonical Manticore synthesis operation"),
            evidence: None,
        })?;
        Ok(Self {
            nodes: parts.nodes,
            lineages: parts.lineages,
            forms: parts.forms,
            chimera_recipe: parts.chimera_recipe,
            manticore_recipe: parts.manticore_recipe,
            creature_pairs: parts.creature_pairs,
            basin_sects: parts.basin_sects,
            chimera: parts.chimera,
            sovereign_identity: parts.sovereign_identity,
            founding_union: parts.founding_union,
            gallowry: parts.gallowry,
            composition,
        })
    }

    #[must_use]
    pub fn nodes(&self) -> &[FlyntAuthorityNode] {
        &self.nodes
    }

    #[must_use]
    pub fn lineages(&self) -> &[PractitionerLineage] {
        &self.lineages
    }

    #[must_use]
    pub fn forms(&self) -> &[TransformationForm] {
        &self.forms
    }

    #[must_use]
    pub fn chimera_recipe(&self) -> &SynthesisRecipe {
        &self.chimera_recipe
    }

    #[must_use]
    pub fn manticore_recipe(&self) -> &DividedManticoreRecipe {
        &self.manticore_recipe
    }

    #[must_use]
    pub fn creature_pairs(&self) -> &[FlyntCreaturePair] {
        &self.creature_pairs
    }

    #[must_use]
    pub fn basin_sects(&self) -> &[BasinCreatureSect] {
        &self.basin_sects
    }

    #[must_use]
    pub fn sovereign_identity(&self) -> &FlyntSovereignIdentity {
        &self.sovereign_identity
    }

    #[must_use]
    pub fn chimera(&self) -> &ConstitutionalChimera {
        &self.chimera
    }

    #[must_use]
    pub fn founding_union(&self) -> &FoundingUnion {
        &self.founding_union
    }

    #[must_use]
    pub fn gallowry(&self) -> &GallowrySite {
        &self.gallowry
    }

    #[must_use]
    pub fn composition_catalog(&self) -> &CompositionCatalog {
        &self.composition
    }

    #[must_use]
    pub fn node(&self, id: &FlyntNodeId) -> Option<&FlyntAuthorityNode> {
        self.nodes.iter().find(|node| &node.id == id)
    }

    #[must_use]
    pub fn node_by_key(&self, id: &str) -> Option<&FlyntAuthorityNode> {
        self.nodes.iter().find(|node| node.id.as_str() == id)
    }

    #[must_use]
    pub fn superior_of(&self, id: &FlyntNodeId) -> Option<&FlyntAuthorityNode> {
        self.node(id)
            .and_then(|node| node.superior.as_ref())
            .and_then(|superior| self.node(superior))
    }

    #[must_use]
    pub fn authority_chain(&self, id: &FlyntNodeId) -> Option<Vec<&FlyntAuthorityNode>> {
        let mut chain = Vec::new();
        let mut current = self.node(id)?;
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(current.id.clone()) {
                return None;
            }
            chain.push(current);
            let Some(superior) = &current.superior else {
                return Some(chain);
            };
            current = self.node(superior)?;
        }
    }

    pub fn validate(&self) -> Result<(), ConstitutionError> {
        let expected = canonical_authority_nodes();
        let mut seen = HashSet::new();
        for node in &self.nodes {
            if !seen.insert(node.id.clone()) {
                return Err(ConstitutionError::DuplicateNode(node.id.clone()));
            }
            let Some(canonical) = expected.iter().find(|entry| entry.id == node.id) else {
                return Err(ConstitutionError::UnexpectedNode(node.id.clone()));
            };
            if node != canonical {
                return Err(ConstitutionError::CanonicalNodeMismatch(node.id.clone()));
            }
        }
        for canonical in &expected {
            if !seen.contains(&canonical.id) {
                return Err(ConstitutionError::MissingCanonicalNode(
                    canonical_id_literal(canonical.id.as_str()),
                ));
            }
        }

        let tross = node_id(OFFICE_TROSS);
        for node in &self.nodes {
            if node.id == tross {
                if node.superior.is_some() {
                    return Err(ConstitutionError::RootHasSuperior);
                }
                continue;
            }
            let superior = node
                .superior
                .as_ref()
                .ok_or_else(|| ConstitutionError::NonRootMissingSuperior(node.id.clone()))?;
            if self.node(superior).is_none() {
                return Err(ConstitutionError::MissingSuperior {
                    node: node.id.clone(),
                    superior: superior.clone(),
                });
            }
            let chain = self
                .authority_chain(&node.id)
                .ok_or_else(|| ConstitutionError::AuthorityCycle(node.id.clone()))?;
            if chain.last().map(|root| &root.id) != Some(&tross) {
                return Err(ConstitutionError::AuthorityCycle(node.id.clone()));
            }
        }

        let chimera_nodes = self
            .nodes
            .iter()
            .filter(|node| node.kind == FlyntNodeKind::ConstitutionalCompanion)
            .count();
        if chimera_nodes != 1 || self.chimera.authority_node.as_str() != COMPANION_CHIMERA {
            return Err(ConstitutionError::ChimeraMustBeUnique);
        }
        if self.chimera.first_companion_to != tross
            || self
                .node(&self.chimera.authority_node)
                .and_then(|node| node.superior.as_ref())
                != Some(&tross)
        {
            return Err(ConstitutionError::ChimeraIsNotFirstCompanion);
        }
        self.validate_chimera_recipe()?;
        self.validate_manticore_recipe()?;
        if self.creature_pairs != FLYNT_CREATURE_PAIRS {
            return Err(ConstitutionError::InvalidCreaturePairs);
        }
        if self.basin_sects != BASIN_CREATURE_SECTS {
            return Err(ConstitutionError::InvalidBasinSects);
        }
        self.validate_sovereign_identity()?;
        if !same_lineages(&self.lineages, &canonical_lineages()) {
            return Err(ConstitutionError::InvalidFoundingLineages);
        }
        if !same_founding_union(&self.founding_union, &canonical_founding_union()) {
            return Err(ConstitutionError::InvalidFoundingUnion);
        }
        self.validate_gallowry()?;
        Ok(())
    }

    fn validate_chimera_recipe(&self) -> Result<(), ConstitutionError> {
        if !same_forms(&self.forms, &canonical_forms())
            || self.chimera.form.as_str() != FORM_CHIMERA
            || self.chimera.mythological_identity != FlyntCreature::Chimera
            || self.chimera.mythological_identity.sex() != CreatureSex::Feminine
            || self.chimera.synthesis.as_str() != RECIPE_CHIMERA
            || self.chimera_recipe.id != self.chimera.synthesis
            || self.chimera_recipe.result != self.chimera.form
            || !same_form_set(
                &self.chimera_recipe.sources,
                &[FORM_GARGOYLE, FORM_MERMAN, FORM_WEREWOLF],
            )
            || self
                .composition
                .records_producing_result(self.chimera.form.as_kernel())
                .len()
                != 1
        {
            return Err(ConstitutionError::InvalidChimeraRecipe);
        }
        Ok(())
    }

    fn validate_manticore_recipe(&self) -> Result<(), ConstitutionError> {
        let recipe = &self.manticore_recipe;
        let custodians = recipe
            .custody
            .iter()
            .map(|entry| (entry.custodian.clone(), entry.component))
            .collect::<HashSet<_>>();
        let expected = HashSet::from([
            (
                node_id(OFFICE_BRO_WHITE),
                ManticoreRecipeComponent::Gargoyle,
            ),
            (
                node_id(OFFICE_THE_BEAUTY),
                ManticoreRecipeComponent::Werewolf,
            ),
            (
                node_id(OFFICE_CINDERELLAMAN),
                ManticoreRecipeComponent::Mariner,
            ),
        ]);
        if !same_forms(&self.forms, &canonical_forms())
            || recipe.recipe.id.as_str() != RECIPE_MANTICORE
            || recipe.recipe.result.as_str() != FORM_MANTICORE
            || !same_form_set(&recipe.recipe.sources, &[FORM_CHIMERA])
            || custodians != expected
            || self
                .composition
                .records_producing_result(recipe.recipe.result.as_kernel())
                .len()
                != 1
        {
            return Err(ConstitutionError::InvalidManticoreRecipe);
        }
        Ok(())
    }

    fn validate_sovereign_identity(&self) -> Result<(), ConstitutionError> {
        let identity = &self.sovereign_identity;
        if identity.person.as_str() != PERSON_TROSS
            || identity.public_title.as_str() != OFFICE_TROSS
            || identity.underground_identities
                != vec![node_id(IDENTITY_MYSTERY_MAN), node_id(IDENTITY_MR_X)]
            || identity.maintained_form.as_str() != FORM_MANTICORE
            || identity.mythological_identity != FlyntCreature::Manticore
            || identity.mythological_identity.sex() != CreatureSex::Masculine
            || identity
                .continuance_requirements
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                != BTreeSet::from([
                    ManticoreContinuanceRequirement::BodilyDiscipline,
                    ManticoreContinuanceRequirement::RecipeRenewal,
                    ManticoreContinuanceRequirement::DividedBasinKnowledge,
                    ManticoreContinuanceRequirement::InstitutionalRecognition,
                    ManticoreContinuanceRequirement::SpecializedGlaushouseCare,
                ])
            || identity.continuance_requirements.len() != 5
            || identity.public_institution.as_str() != INSTITUTION_MANTICORP
            || identity.underground_institution.as_str() != INSTITUTION_GALLOWS
        {
            return Err(ConstitutionError::InvalidSovereignIdentity);
        }
        Ok(())
    }

    fn validate_gallowry(&self) -> Result<(), ConstitutionError> {
        if self.gallowry.id.as_str() == INSTITUTION_GALLOWS {
            return Err(ConstitutionError::GallowryIsNotDistinct);
        }
        if self.gallowry != canonical_gallowry() {
            return Err(ConstitutionError::InvalidGallowry);
        }
        Ok(())
    }

    pub fn audit(&self) -> Result<FlyntConstitutionalAudit, ConstitutionError> {
        self.validate()?;
        let duplicate_authority_count = self.nodes.len()
            - self
                .nodes
                .iter()
                .map(|node| &node.id)
                .collect::<HashSet<_>>()
                .len();
        let institution_placements = self
            .nodes
            .iter()
            .filter(|node| node.kind.is_institution())
            .map(placement)
            .collect();
        let office_placements = self
            .nodes
            .iter()
            .filter(|node| node.kind.is_office())
            .map(placement)
            .collect();
        Ok(FlyntConstitutionalAudit {
            sovereign_executive: node_id(OFFICE_TROSS),
            constitutional_chimera_count: self
                .nodes
                .iter()
                .filter(|node| node.kind == FlyntNodeKind::ConstitutionalCompanion)
                .count(),
            institution_placements,
            office_placements,
            all_non_root_nodes_have_one_superior: self
                .nodes
                .iter()
                .all(|node| node.id.as_str() == OFFICE_TROSS || node.superior.is_some()),
            hierarchy_is_acyclic: self
                .nodes
                .iter()
                .all(|node| self.authority_chain(&node.id).is_some()),
            all_authority_reaches_tross: self.nodes.iter().all(|node| {
                self.authority_chain(&node.id)
                    .and_then(|chain| chain.last().copied())
                    .is_some_and(|root| root.id.as_str() == OFFICE_TROSS)
            }),
            duplicate_authority_count,
            gallowry_is_distinct_from_gallows: self.gallowry.id.as_str() != INSTITUTION_GALLOWS,
            chimera_recipe_count: self
                .composition
                .records_producing_result(self.chimera.form.as_kernel())
                .len(),
            manticore_recipe_count: self
                .composition
                .records_producing_result(self.manticore_recipe.recipe.result.as_kernel())
                .len(),
            founding_union_is_complete: same_founding_union(
                &self.founding_union,
                &canonical_founding_union(),
            ),
        })
    }
}

fn placement(node: &FlyntAuthorityNode) -> AuthorityPlacement {
    AuthorityPlacement {
        id: node.id.clone(),
        superior: node.superior.clone(),
    }
}

fn same_form_set(actual: &[FormId], expected: &[&str]) -> bool {
    actual.len() == expected.len()
        && actual.iter().map(FormId::as_str).collect::<HashSet<_>>()
            == expected.iter().copied().collect::<HashSet<_>>()
}

fn same_forms(actual: &[TransformationForm], expected: &[TransformationForm]) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .all(|form| expected.iter().any(|candidate| candidate == form))
}

fn same_lineages(actual: &[PractitionerLineage], expected: &[PractitionerLineage]) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .all(|lineage| expected.iter().any(|candidate| candidate == lineage))
}

fn same_founding_union(actual: &FoundingUnion, expected: &FoundingUnion) -> bool {
    actual.leadership_body == expected.leadership_body
        && actual.sovereign_leader == expected.sovereign_leader
        && actual.underworld_manifestation == expected.underworld_manifestation
        && actual.founding_leaders.len() == expected.founding_leaders.len()
        && actual
            .founding_leaders
            .iter()
            .all(|leader| expected.founding_leaders.contains(leader))
}

fn canonical_id_literal(value: &str) -> &'static str {
    match value {
        OFFICE_TROSS => OFFICE_TROSS,
        COMPANION_CHIMERA => COMPANION_CHIMERA,
        INSTITUTION_MANTICORP => INSTITUTION_MANTICORP,
        INSTITUTION_MYSTERY_MEN => INSTITUTION_MYSTERY_MEN,
        EXTENSION_MARQUISE => EXTENSION_MARQUISE,
        EXPRESSION_MYSTERY_MAN => EXPRESSION_MYSTERY_MAN,
        INSTITUTION_GALLOWS => INSTITUTION_GALLOWS,
        EXPRESSION_WE_FAIRY_MEN => EXPRESSION_WE_FAIRY_MEN,
        OFFICE_BRO_WHITE => OFFICE_BRO_WHITE,
        CREW_BRO_WHITE => CREW_BRO_WHITE,
        OFFICE_CINDERELLAMAN => OFFICE_CINDERELLAMAN,
        CREW_CINDERELLAMAN => CREW_CINDERELLAMAN,
        OFFICE_THE_BEAUTY => OFFICE_THE_BEAUTY,
        CREW_THE_BEAUTY => CREW_THE_BEAUTY,
        _ => "unknown-canonical-id",
    }
}

fn node_id(value: &str) -> FlyntNodeId {
    FlyntNodeId::new(value).expect("canonical Flynt node ID")
}

fn form_id(value: &str) -> FormId {
    FormId::new(value).expect("canonical Flynt form ID")
}

fn recipe_id(value: &str) -> SynthesisRecipeId {
    SynthesisRecipeId::new(value).expect("canonical Flynt recipe ID")
}

fn authority_node(
    id: &str,
    name: &str,
    kind: FlyntNodeKind,
    branch: ConstitutionalBranch,
    superior: Option<&str>,
    visibility: ConstitutionalVisibility,
    responsibilities: &[InstitutionalResponsibility],
) -> FlyntAuthorityNode {
    FlyntAuthorityNode {
        id: node_id(id),
        name: name.into(),
        kind,
        branch,
        superior: superior.map(node_id),
        visibility,
        responsibilities: responsibilities.to_vec(),
    }
}

#[must_use]
pub fn canonical_authority_nodes() -> Vec<FlyntAuthorityNode> {
    use ConstitutionalBranch::{Common, Rural, Urban};
    use ConstitutionalVisibility::{
        Legendary, PrivateAndExistenceDisputed, Public, PublicWithClassifiedOperations,
    };
    use FlyntNodeKind::{
        ConstitutionalCompanion, Crew, CriminalSyndicate, FoundingLeaderOffice,
        InvestigativeBureau, LeadershipBody, LegendaryOperative, MaritimeExtension,
        MilitaryInstitution, SovereignExecutive,
    };
    use InstitutionalResponsibility as R;

    vec![
        authority_node(
            OFFICE_TROSS,
            "The Tross",
            SovereignExecutive,
            Common,
            None,
            Public,
            &[],
        ),
        authority_node(
            COMPANION_CHIMERA,
            "Chimera",
            ConstitutionalCompanion,
            Common,
            Some(OFFICE_TROSS),
            Public,
            &[],
        ),
        authority_node(
            INSTITUTION_MANTICORP,
            "Manticorp",
            MilitaryInstitution,
            Urban,
            Some(OFFICE_TROSS),
            Public,
            &[
                R::TerritorialDefense,
                R::MilitaryCommand,
                R::ConstitutionalProtection,
                R::DisciplinedForce,
                R::MilitaryTraining,
                R::LawfulDeployment,
            ],
        ),
        authority_node(
            INSTITUTION_MYSTERY_MEN,
            "Mystery Men",
            InvestigativeBureau,
            Urban,
            Some(OFFICE_TROSS),
            PublicWithClassifiedOperations,
            &[
                R::Investigation,
                R::Intelligence,
                R::Counterintelligence,
                R::CovertOperations,
                R::OrganizedCrimeInvestigation,
                R::Contraband,
                R::Espionage,
                R::ConstitutionalSecurity,
            ],
        ),
        authority_node(
            EXTENSION_MARQUISE,
            "The Marquise",
            MaritimeExtension,
            Urban,
            Some(INSTITUTION_MYSTERY_MEN),
            PublicWithClassifiedOperations,
            &[
                R::Navigation,
                R::MaritimeOperations,
                R::Communications,
                R::ComplexSystemControl,
            ],
        ),
        authority_node(
            EXPRESSION_MYSTERY_MAN,
            "The Mystery Man",
            LegendaryOperative,
            Rural,
            Some(INSTITUTION_GALLOWS),
            Legendary,
            &[],
        ),
        authority_node(
            INSTITUTION_GALLOWS,
            "The Gallows",
            CriminalSyndicate,
            Rural,
            Some(EXPRESSION_WE_FAIRY_MEN),
            PrivateAndExistenceDisputed,
            &[
                R::OrganizedCrime,
                R::RegionalCrews,
                R::Loyalty,
                R::Territory,
                R::Favors,
                R::Obligation,
                R::CulturalIdentity,
            ],
        ),
        authority_node(
            EXPRESSION_WE_FAIRY_MEN,
            "We Fairy Men",
            LeadershipBody,
            Rural,
            Some(OFFICE_TROSS),
            Legendary,
            &[],
        ),
        authority_node(
            OFFICE_BRO_WHITE,
            "Bro White",
            FoundingLeaderOffice,
            Rural,
            Some(EXPRESSION_WE_FAIRY_MEN),
            Legendary,
            &[],
        ),
        authority_node(
            CREW_BRO_WHITE,
            "Bro White and the 7 Brothas",
            Crew,
            Rural,
            Some(OFFICE_BRO_WHITE),
            Legendary,
            &[],
        ),
        authority_node(
            OFFICE_CINDERELLAMAN,
            "Cinderellaman",
            FoundingLeaderOffice,
            Rural,
            Some(EXPRESSION_WE_FAIRY_MEN),
            Legendary,
            &[],
        ),
        authority_node(
            CREW_CINDERELLAMAN,
            "Cinderellaman and His Midnight Crew",
            Crew,
            Rural,
            Some(OFFICE_CINDERELLAMAN),
            Legendary,
            &[],
        ),
        authority_node(
            OFFICE_THE_BEAUTY,
            "Beauty",
            FoundingLeaderOffice,
            Rural,
            Some(EXPRESSION_WE_FAIRY_MEN),
            Legendary,
            &[],
        ),
        authority_node(
            CREW_THE_BEAUTY,
            "Beauty and His Beasts",
            Crew,
            Rural,
            Some(OFFICE_THE_BEAUTY),
            Legendary,
            &[],
        ),
    ]
}

#[must_use]
pub fn canonical_lineages() -> Vec<PractitionerLineage> {
    use LineageAssociation as A;
    vec![
        PractitionerLineage {
            tradition: Some(PractitionerTradition::Warlock),
            creature: FlyntCreature::Gargoyle,
            synthesis_form: form_id(FORM_GARGOYLE),
            founding_leader: node_id(OFFICE_BRO_WHITE),
            crew: node_id(CREW_BRO_WHITE),
            associations: vec![A::SouthernFlynt, A::Stone, A::Architecture, A::Guardianship],
            working_function: "HOLD",
            exact_current_aura_equation: None,
        },
        PractitionerLineage {
            tradition: None,
            creature: FlyntCreature::Mariner,
            synthesis_form: form_id(FORM_MERMAN),
            founding_leader: node_id(OFFICE_CINDERELLAMAN),
            crew: node_id(CREW_CINDERELLAMAN),
            associations: vec![
                A::Riptide,
                A::AuraSea,
                A::Waterways,
                A::Smuggling,
                A::MidnightTransformation,
            ],
            working_function: "ADAPT",
            exact_current_aura_equation: None,
        },
        PractitionerLineage {
            tradition: Some(PractitionerTradition::Lycan),
            creature: FlyntCreature::Werewolf,
            synthesis_form: form_id(FORM_WEREWOLF),
            founding_leader: node_id(OFFICE_THE_BEAUTY),
            crew: node_id(CREW_THE_BEAUTY),
            associations: vec![
                A::NorthernFlynt,
                A::Wilderness,
                A::Packs,
                A::RoamingProtection,
            ],
            working_function: "DRIVE",
            exact_current_aura_equation: None,
        },
    ]
}

/// Stable adapter for callers that still consume the superseded
/// `FoundingPeople` classification. Active canon uses [`canonical_lineages`].
#[must_use]
pub fn legacy_founding_lineages() -> Vec<FoundingLineage> {
    canonical_lineages()
        .into_iter()
        .map(|lineage| FoundingLineage {
            people: match lineage.creature {
                FlyntCreature::Gargoyle => FoundingPeople::Gargoyle,
                FlyntCreature::Werewolf => FoundingPeople::Werewolf,
                FlyntCreature::Mariner => FoundingPeople::Merman,
                _ => unreachable!("only the three legacy founding lineages are adapted"),
            },
            founding_leader: lineage.founding_leader,
            crew: lineage.crew,
            associations: lineage.associations,
        })
        .collect()
}

#[must_use]
pub fn canonical_forms() -> Vec<TransformationForm> {
    vec![
        TransformationForm {
            id: form_id(FORM_GARGOYLE),
            name: "Gargoyle".into(),
            kind: FormKind::PractitionerSynthesis,
        },
        TransformationForm {
            id: form_id(FORM_MERMAN),
            name: "Mariner".into(),
            kind: FormKind::PractitionerSynthesis,
        },
        TransformationForm {
            id: form_id(FORM_WEREWOLF),
            name: "Werewolf".into(),
            kind: FormKind::PractitionerSynthesis,
        },
        TransformationForm {
            id: form_id(FORM_CHIMERA),
            name: "Chimera".into(),
            kind: FormKind::ConstitutionalSynthesis,
        },
        TransformationForm {
            id: form_id(FORM_MANTICORE),
            name: "Manticore".into(),
            kind: FormKind::ConstitutionalSynthesis,
        },
    ]
}

#[must_use]
pub fn canonical_manticore_recipe() -> DividedManticoreRecipe {
    DividedManticoreRecipe {
        recipe: SynthesisRecipe {
            id: recipe_id(RECIPE_MANTICORE),
            sources: vec![form_id(FORM_CHIMERA)],
            result: form_id(FORM_MANTICORE),
        },
        custody: vec![
            ManticoreRecipeCustody {
                custodian: node_id(OFFICE_BRO_WHITE),
                component: ManticoreRecipeComponent::Gargoyle,
            },
            ManticoreRecipeCustody {
                custodian: node_id(OFFICE_THE_BEAUTY),
                component: ManticoreRecipeComponent::Werewolf,
            },
            ManticoreRecipeCustody {
                custodian: node_id(OFFICE_CINDERELLAMAN),
                component: ManticoreRecipeComponent::Mariner,
            },
        ],
    }
}

#[must_use]
pub fn canonical_sovereign_identity() -> FlyntSovereignIdentity {
    FlyntSovereignIdentity {
        person: node_id(PERSON_TROSS),
        public_title: node_id(OFFICE_TROSS),
        underground_identities: vec![node_id(IDENTITY_MYSTERY_MAN), node_id(IDENTITY_MR_X)],
        maintained_form: form_id(FORM_MANTICORE),
        mythological_identity: FlyntCreature::Manticore,
        continuance_requirements: vec![
            ManticoreContinuanceRequirement::BodilyDiscipline,
            ManticoreContinuanceRequirement::RecipeRenewal,
            ManticoreContinuanceRequirement::DividedBasinKnowledge,
            ManticoreContinuanceRequirement::InstitutionalRecognition,
            ManticoreContinuanceRequirement::SpecializedGlaushouseCare,
        ],
        public_institution: node_id(INSTITUTION_MANTICORP),
        underground_institution: node_id(INSTITUTION_GALLOWS),
    }
}

#[must_use]
pub fn canonical_gallowry() -> GallowrySite {
    GallowrySite {
        id: node_id(SITE_GALLOWRY),
        name: "The Gallowry".into(),
        controlled_by: node_id(INSTITUTION_GALLOWS),
        visibility: ConstitutionalVisibility::Hidden,
        functions: vec![
            GallowryFunction::MeetingPlace,
            GallowryFunction::Headquarters,
            GallowryFunction::CulturalCenter,
            GallowryFunction::Gallery,
            GallowryFunction::OperationalHub,
        ],
    }
}

#[must_use]
pub fn canonical_founding_union() -> FoundingUnion {
    FoundingUnion {
        founding_leaders: vec![
            node_id(OFFICE_BRO_WHITE),
            node_id(OFFICE_CINDERELLAMAN),
            node_id(OFFICE_THE_BEAUTY),
        ],
        leadership_body: node_id(EXPRESSION_WE_FAIRY_MEN),
        sovereign_leader: node_id(OFFICE_TROSS),
        underworld_manifestation: node_id(INSTITUTION_GALLOWS),
    }
}

#[must_use]
pub fn canonical_parts() -> FlyntConstitutionParts {
    FlyntConstitutionParts {
        nodes: canonical_authority_nodes(),
        lineages: canonical_lineages(),
        forms: canonical_forms(),
        chimera_recipe: SynthesisRecipe {
            id: recipe_id(RECIPE_CHIMERA),
            sources: vec![
                form_id(FORM_GARGOYLE),
                form_id(FORM_MERMAN),
                form_id(FORM_WEREWOLF),
            ],
            result: form_id(FORM_CHIMERA),
        },
        manticore_recipe: canonical_manticore_recipe(),
        creature_pairs: FLYNT_CREATURE_PAIRS.to_vec(),
        basin_sects: BASIN_CREATURE_SECTS.to_vec(),
        chimera: ConstitutionalChimera {
            authority_node: node_id(COMPANION_CHIMERA),
            form: form_id(FORM_CHIMERA),
            mythological_identity: FlyntCreature::Chimera,
            synthesis: recipe_id(RECIPE_CHIMERA),
            first_companion_to: node_id(OFFICE_TROSS),
        },
        sovereign_identity: canonical_sovereign_identity(),
        founding_union: canonical_founding_union(),
        gallowry: canonical_gallowry(),
    }
}

pub fn canonical_constitution() -> Result<FlyntConstitution, ConstitutionError> {
    let constitution = FlyntConstitution::from_parts(canonical_parts())?;
    constitution.validate()?;
    Ok(constitution)
}

/// Stable, implementation-owned hierarchy rows used by documentation and
/// adapters. Callers may render these rows but must not reinterpret them.
#[must_use]
pub fn canonical_hierarchy_rows() -> Vec<(&'static str, &'static str)> {
    vec![
        (COMPANION_CHIMERA, OFFICE_TROSS),
        (INSTITUTION_MANTICORP, OFFICE_TROSS),
        (INSTITUTION_MYSTERY_MEN, OFFICE_TROSS),
        (EXTENSION_MARQUISE, INSTITUTION_MYSTERY_MEN),
        (EXPRESSION_MYSTERY_MAN, INSTITUTION_GALLOWS),
        (EXPRESSION_WE_FAIRY_MEN, OFFICE_TROSS),
        (INSTITUTION_GALLOWS, EXPRESSION_WE_FAIRY_MEN),
        (OFFICE_BRO_WHITE, EXPRESSION_WE_FAIRY_MEN),
        (CREW_BRO_WHITE, OFFICE_BRO_WHITE),
        (OFFICE_CINDERELLAMAN, EXPRESSION_WE_FAIRY_MEN),
        (CREW_CINDERELLAMAN, OFFICE_CINDERELLAMAN),
        (OFFICE_THE_BEAUTY, EXPRESSION_WE_FAIRY_MEN),
        (CREW_THE_BEAUTY, OFFICE_THE_BEAUTY),
    ]
}

/// Deterministic map of each canonical authority node to its one superior.
#[must_use]
pub fn canonical_superior_map() -> BTreeMap<&'static str, Option<&'static str>> {
    let mut map = BTreeMap::from([(OFFICE_TROSS, None)]);
    map.extend(
        canonical_hierarchy_rows()
            .into_iter()
            .map(|(node, superior)| (node, Some(superior))),
    );
    map
}
