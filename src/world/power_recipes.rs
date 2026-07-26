//! Additive Current–Aura power Recipe constitution.
//!
//! A power is an observable outcome. A Recipe is the constitutional method
//! used to reach it. This module deliberately declares and validates methods;
//! it does not emit [`crate::synthesis_recipe::SynthesisScript`] values, mutate
//! a [`crate::FrameState`], grant House authority, or choose for a player.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::hueman_faculties::{FacultyAuthority, HuemanFaculty};

pub const POWER_RECIPE_SOURCE: &str = "HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md";
pub const POWER_RECIPE_ARCHIVE_FORMAT: &str = "HGPRC";
pub const POWER_RECIPE_ARCHIVE_SCHEMA_VERSION: u16 = 1;
pub const MAX_PREPARED_POWER_RECIPES: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CurrentPowerRoot {
    Somatokinesis,
    Generation,
    Shapeshifting,
    Biokinesis,
    Telekinesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AuraPowerRoot {
    Psychometry,
    Dreamwalking,
    Illusion,
    Clairvoyance,
    Telepathy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FacultyPowerRoot {
    pub faculty: HuemanFaculty,
    pub authority: FacultyAuthority,
    pub current: CurrentPowerRoot,
    pub aura: AuraPowerRoot,
    pub principle: &'static str,
}

pub const FACULTY_POWER_ROOTS: [FacultyPowerRoot; 5] = [
    FacultyPowerRoot {
        faculty: HuemanFaculty::Presynce,
        authority: FacultyAuthority::Stonebend,
        current: CurrentPowerRoot::Somatokinesis,
        aura: AuraPowerRoot::Psychometry,
        principle: "presence held in physical form",
    },
    FacultyPowerRoot {
        faculty: HuemanFaculty::Prefog,
        authority: FacultyAuthority::SandmanorMinorian,
        current: CurrentPowerRoot::Generation,
        aura: AuraPowerRoot::Dreamwalking,
        principle: "what forms inwardly before it receives a visible figure",
    },
    FacultyPowerRoot {
        faculty: HuemanFaculty::Prefig,
        authority: FacultyAuthority::SandmanorMinoan,
        current: CurrentPowerRoot::Shapeshifting,
        aura: AuraPowerRoot::Illusion,
        principle: "presenting the figure before it becomes fully real",
    },
    FacultyPowerRoot {
        faculty: HuemanFaculty::Precog,
        authority: FacultyAuthority::Glaushouse,
        current: CurrentPowerRoot::Biokinesis,
        aura: AuraPowerRoot::Clairvoyance,
        principle: "perceiving a condition before or beneath its completed appearance",
    },
    FacultyPowerRoot {
        faculty: HuemanFaculty::Resynce,
        authority: FacultyAuthority::Flynt,
        current: CurrentPowerRoot::Telekinesis,
        aura: AuraPowerRoot::Telepathy,
        principle: "synchronization across separation",
    },
];

#[must_use]
pub fn faculty_power_root(faculty: HuemanFaculty) -> FacultyPowerRoot {
    FACULTY_POWER_ROOTS
        .iter()
        .copied()
        .find(|definition| definition.faculty == faculty)
        .expect("all typed Hueman faculties have a power-root definition")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PowerOutcome {
    Flight,
    Invisibility,
    Healing,
    SuperSpeed,
}

impl PowerOutcome {
    pub const ALL: [Self; 4] = [
        Self::Flight,
        Self::Invisibility,
        Self::Healing,
        Self::SuperSpeed,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipeProvenance {
    Original,
    HouseRefinement,
    InstitutionalTechnique,
    CrossHouseRecipe,
    PersonalRefinement,
    DisputedInvention,
    ProhibitedRecipe,
    IllegalSynthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipeStanding {
    Proposed,
    Taught,
    Provisional,
    Recognized,
    Disputed,
    Prohibited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UnlockRequirement {
    Capacity,
    Proof,
    Recognition,
    CompatibleFormAndFrame,
    DiscoveredOrTaughtRecipe,
}

pub const UNIVERSAL_UNLOCK_REQUIREMENTS: [UnlockRequirement; 5] = [
    UnlockRequirement::Capacity,
    UnlockRequirement::Proof,
    UnlockRequirement::Recognition,
    UnlockRequirement::CompatibleFormAndFrame,
    UnlockRequirement::DiscoveredOrTaughtRecipe,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecipeRestriction {
    AddressableEvidence,
    CapacityBound,
    CompatibleFormAndFrame,
    ConsentWhenAnotherBeingIsAffected,
    HouseProof,
    GlaushouseClearanceForLivingIntervention,
    StonebendRecognitionForStableForm,
    NoOutcomeOwnership,
    NoAutomaticAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceCost {
    Light,
    Moderate,
    Heavy,
    Extreme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrainBand {
    Low,
    Guarded,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RangeBand {
    SelfOnly,
    Touch,
    Near,
    Field,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationBand {
    Momentary,
    Sustained,
    Scene,
    PersistentUntilReversed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrecisionBand {
    Broad,
    Directed,
    Fine,
    Surgical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeCapacity {
    pub form: u8,
    pub frame: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeLimits {
    pub current_cost: ResourceCost,
    pub aura_cost: ResourceCost,
    pub strain: StrainBand,
    pub range: RangeBand,
    pub duration: DurationBand,
    pub precision: PrecisionBand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeContribution {
    pub faculty: HuemanFaculty,
    pub authority: FacultyAuthority,
    pub current_root: CurrentPowerRoot,
    pub current_method: String,
    pub aura_root: AuraPowerRoot,
    pub aura_method: String,
    pub processor_principle: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerRecipeDefinition {
    pub id: String,
    pub name: String,
    pub outcome: PowerOutcome,
    pub provenance: RecipeProvenance,
    pub standing: RecipeStanding,
    pub contributions: Vec<RecipeContribution>,
    pub capacity: RecipeCapacity,
    pub limits: RecipeLimits,
    pub unlock_requirements: Vec<UnlockRequirement>,
    pub restrictions: Vec<RecipeRestriction>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub trace: String,
    pub risks: Vec<String>,
}

impl PowerRecipeDefinition {
    #[must_use]
    pub fn is_cross_house(&self) -> bool {
        self.contributions.len() > 1 || self.provenance == RecipeProvenance::CrossHouseRecipe
    }

    #[must_use]
    pub fn processors(&self) -> Vec<HuemanFaculty> {
        self.contributions
            .iter()
            .map(|contribution| contribution.faculty)
            .collect()
    }

    /// Outcome labels carry no authority. Only a Recipe's processors and
    /// evidence describe the method's constitutional identity.
    #[must_use]
    pub const fn outcome_grants_house_ownership(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerRecipeError {
    EmptyId,
    EmptyName(String),
    MissingContribution(String),
    DuplicateProcessor(String, HuemanFaculty),
    FacultyAuthorityMismatch(String, HuemanFaculty),
    CurrentRootMismatch(String, HuemanFaculty),
    AuraRootMismatch(String, HuemanFaculty),
    MissingMethod(String, HuemanFaculty),
    InvalidCapacity(String),
    MissingUnlockRequirement(String, UnlockRequirement),
    MissingRestriction(String, RecipeRestriction),
    MissingTradeoff(String),
    DuplicateRecipeId(String),
    DuplicateOutcomeTrace(PowerOutcome, String),
    MissingOutcome(PowerOutcome),
    InsufficientCompetingMethods(PowerOutcome),
    OutcomeLacksCompetingAuthorities(PowerOutcome),
    InvalidCrossHouseRecipe(String),
    Json(String),
    UnsupportedFormat(String),
    UnsupportedSchema(u16),
    ChecksumMismatch,
}

impl std::fmt::Display for PowerRecipeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "power Recipe constitution rejected state: {self:?}"
        )
    }
}

impl std::error::Error for PowerRecipeError {}

pub fn validate_power_recipe(recipe: &PowerRecipeDefinition) -> Result<(), PowerRecipeError> {
    if recipe.id.trim().is_empty() {
        return Err(PowerRecipeError::EmptyId);
    }
    if recipe.name.trim().is_empty() {
        return Err(PowerRecipeError::EmptyName(recipe.id.clone()));
    }
    if recipe.contributions.is_empty() {
        return Err(PowerRecipeError::MissingContribution(recipe.id.clone()));
    }

    let mut processors = Vec::new();
    for contribution in &recipe.contributions {
        if processors.contains(&contribution.faculty) {
            return Err(PowerRecipeError::DuplicateProcessor(
                recipe.id.clone(),
                contribution.faculty,
            ));
        }
        processors.push(contribution.faculty);
        let root = faculty_power_root(contribution.faculty);
        if contribution.authority != root.authority {
            return Err(PowerRecipeError::FacultyAuthorityMismatch(
                recipe.id.clone(),
                contribution.faculty,
            ));
        }
        if contribution.current_root != root.current {
            return Err(PowerRecipeError::CurrentRootMismatch(
                recipe.id.clone(),
                contribution.faculty,
            ));
        }
        if contribution.aura_root != root.aura {
            return Err(PowerRecipeError::AuraRootMismatch(
                recipe.id.clone(),
                contribution.faculty,
            ));
        }
        if contribution.current_method.trim().is_empty()
            || contribution.aura_method.trim().is_empty()
            || contribution.processor_principle.trim().is_empty()
        {
            return Err(PowerRecipeError::MissingMethod(
                recipe.id.clone(),
                contribution.faculty,
            ));
        }
    }

    if recipe.capacity.form == 0
        || recipe.capacity.frame == 0
        || recipe.capacity.form > 4
        || recipe.capacity.frame > 4
    {
        return Err(PowerRecipeError::InvalidCapacity(recipe.id.clone()));
    }
    for requirement in UNIVERSAL_UNLOCK_REQUIREMENTS {
        if !recipe.unlock_requirements.contains(&requirement) {
            return Err(PowerRecipeError::MissingUnlockRequirement(
                recipe.id.clone(),
                requirement,
            ));
        }
    }
    for restriction in [
        RecipeRestriction::AddressableEvidence,
        RecipeRestriction::CapacityBound,
        RecipeRestriction::CompatibleFormAndFrame,
        RecipeRestriction::HouseProof,
        RecipeRestriction::NoOutcomeOwnership,
        RecipeRestriction::NoAutomaticAuthority,
    ] {
        if !recipe.restrictions.contains(&restriction) {
            return Err(PowerRecipeError::MissingRestriction(
                recipe.id.clone(),
                restriction,
            ));
        }
    }
    if recipe.strengths.is_empty()
        || recipe.weaknesses.is_empty()
        || recipe.trace.trim().is_empty()
        || recipe.risks.is_empty()
    {
        return Err(PowerRecipeError::MissingTradeoff(recipe.id.clone()));
    }
    if recipe.is_cross_house()
        && (recipe.contributions.len() < 2
            || recipe.provenance != RecipeProvenance::CrossHouseRecipe)
    {
        return Err(PowerRecipeError::InvalidCrossHouseRecipe(recipe.id.clone()));
    }
    Ok(())
}

pub fn validate_power_recipe_catalog(
    recipes: &[PowerRecipeDefinition],
) -> Result<(), PowerRecipeError> {
    let mut ids = BTreeSet::new();
    let mut traces: BTreeMap<PowerOutcome, BTreeSet<String>> = BTreeMap::new();
    let mut methods: BTreeMap<PowerOutcome, usize> = BTreeMap::new();
    let mut authorities: BTreeMap<PowerOutcome, Vec<FacultyAuthority>> = BTreeMap::new();

    for recipe in recipes {
        validate_power_recipe(recipe)?;
        if !ids.insert(recipe.id.clone()) {
            return Err(PowerRecipeError::DuplicateRecipeId(recipe.id.clone()));
        }
        if !traces
            .entry(recipe.outcome)
            .or_default()
            .insert(recipe.trace.clone())
        {
            return Err(PowerRecipeError::DuplicateOutcomeTrace(
                recipe.outcome,
                recipe.trace.clone(),
            ));
        }
        *methods.entry(recipe.outcome).or_default() += 1;
        let outcome_authorities = authorities.entry(recipe.outcome).or_default();
        for contribution in &recipe.contributions {
            if !outcome_authorities.contains(&contribution.authority) {
                outcome_authorities.push(contribution.authority);
            }
        }
    }

    for outcome in PowerOutcome::ALL {
        let count = methods
            .get(&outcome)
            .copied()
            .ok_or(PowerRecipeError::MissingOutcome(outcome))?;
        let required = match outcome {
            PowerOutcome::Flight => 5,
            PowerOutcome::Invisibility | PowerOutcome::Healing | PowerOutcome::SuperSpeed => 4,
        };
        if count < required {
            return Err(PowerRecipeError::InsufficientCompetingMethods(outcome));
        }
        if authorities.get(&outcome).map_or(0, Vec::len) < 3 {
            return Err(PowerRecipeError::OutcomeLacksCompetingAuthorities(outcome));
        }
    }
    Ok(())
}

#[must_use]
pub fn draft_power_recipe_catalog() -> Vec<PowerRecipeDefinition> {
    vec![
        recipe(
            "recipe.flight.resynce-propulsion",
            "Resynce Propulsion",
            PowerOutcome::Flight,
            RecipeProvenance::InstitutionalTechnique,
            vec![contribution(
                HuemanFaculty::Resynce,
                "telekinetic lift and thrust applied to the practitioner's body",
                "synchronized intention maintains a coherent remote force-vector",
            )],
            profile(
                2,
                2,
                ResourceCost::Heavy,
                ResourceCost::Moderate,
                StrainBand::High,
                RangeBand::SelfOnly,
                DurationBand::Sustained,
                PrecisionBand::Directed,
            ),
            "fast vector changes and no external anatomy",
            "continuous concentration and force expenditure",
            "directed pressure wake with synchronized-intention echo",
            "loss of synchronization produces an abrupt fall",
        ),
        recipe(
            "recipe.flight.presynce-density",
            "Stonebend Density Flight",
            PowerOutcome::Flight,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Presynce,
                "body density and muscular force are redistributed into repeated lift",
                "psychometric contact reads the body's load and structural history",
            )],
            profile(
                3,
                3,
                ResourceCost::Heavy,
                ResourceCost::Light,
                StrainBand::Critical,
                RangeBand::SelfOnly,
                DurationBand::Sustained,
                PrecisionBand::Directed,
            ),
            "resists impact and preserves bodily control",
            "short range and severe structural strain",
            "compressed surface and load-bearing Current imprint",
            "misjudged density can injure the Frame on landing",
        ),
        recipe(
            "recipe.flight.precog-adaptation",
            "Glaüshouse Aerial Adaptation",
            PowerOutcome::Flight,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Precog,
                "living tissue is adapted for lift, respiration, and control",
                "clairvoyant diagnosis tracks hidden stress during the adaptation",
            )],
            profile(
                3,
                3,
                ResourceCost::Moderate,
                ResourceCost::Moderate,
                StrainBand::High,
                RangeBand::SelfOnly,
                DurationBand::PersistentUntilReversed,
                PrecisionBand::Surgical,
            ),
            "efficient sustained flight after successful adaptation",
            "slow preparation and clinical recovery requirements",
            "living graft seams and diagnostic Aura afterimage",
            "rejection or maladaptation can compromise breathing and balance",
        ),
        recipe(
            "recipe.flight.prefog-wings",
            "Prefog Generated Wings",
            PowerOutcome::Flight,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Prefog,
                "wings or a living carrier are cultivated into material support",
                "dream imagery supplies the inward pattern before generation",
            )],
            profile(
                2,
                2,
                ResourceCost::Heavy,
                ResourceCost::Moderate,
                StrainBand::Guarded,
                RangeBand::Near,
                DurationBand::Scene,
                PrecisionBand::Fine,
            ),
            "may carry another consenting Being and remain after concentration ends",
            "requires cultivation time, space, and viable material",
            "organic construction residue with remembered dream motif",
            "unstable incubation may produce a carrier unsuited to local conditions",
        ),
        recipe(
            "recipe.flight.prefig-aerial-form",
            "Prefig Aerial Form",
            PowerOutcome::Flight,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Prefig,
                "the body assumes an aerial figure capable of physical lift",
                "illusion presents and coordinates the selected figure",
            )],
            profile(
                3,
                3,
                ResourceCost::Moderate,
                ResourceCost::Heavy,
                StrainBand::High,
                RangeBand::SelfOnly,
                DurationBand::Scene,
                PrecisionBand::Fine,
            ),
            "adaptable aerial forms and strong maneuverability",
            "form compatibility limits which figures can become physically stable",
            "morphic residue beneath a fading glamour outline",
            "a provisional figure can fail under prolonged physical resistance",
        ),
        recipe(
            "recipe.invisibility.prefig-glamour",
            "Prefig Veiling Glamour",
            PowerOutcome::Invisibility,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Prefig,
                "surface posture and partial shape support the projected disguise",
                "illusion redirects sensory interpretation away from the subject",
            )],
            profile(
                2,
                2,
                ResourceCost::Light,
                ResourceCost::Heavy,
                StrainBand::Guarded,
                RangeBand::Near,
                DurationBand::Sustained,
                PrecisionBand::Fine,
            ),
            "can veil appearance without altering surrounding matter",
            "does not hide physical contact, tracks, or structural evidence",
            "sensory discontinuity and glamour afterimage",
            "contradictory evidence can collapse the projection at once",
        ),
        recipe(
            "recipe.invisibility.precog-avoidance",
            "Precog Observation Avoidance",
            PowerOutcome::Invisibility,
            RecipeProvenance::InstitutionalTechnique,
            vec![contribution(
                HuemanFaculty::Precog,
                "biological movement is moderated to reduce observable cues",
                "clairvoyance anticipates probable sight-lines and attention",
            )],
            profile(
                2,
                2,
                ResourceCost::Moderate,
                ResourceCost::Moderate,
                StrainBand::Guarded,
                RangeBand::Field,
                DurationBand::Sustained,
                PrecisionBand::Surgical,
            ),
            "leaves perception uncoerced and works around many detection media",
            "is avoidance rather than literal optical disappearance",
            "diagnostic route projection with unusually controlled vital rhythm",
            "unexpected observers or uncertain evidence break the route forecast",
        ),
        recipe(
            "recipe.invisibility.resynce-recognition",
            "Resynce Recognition Suppression",
            PowerOutcome::Invisibility,
            RecipeProvenance::DisputedInvention,
            vec![contribution(
                HuemanFaculty::Resynce,
                "telekinetic micro-corrections reduce attention-catching movement",
                "telepathic synchronization suppresses recognition of the subject",
            )],
            profile(
                3,
                3,
                ResourceCost::Moderate,
                ResourceCost::Heavy,
                StrainBand::High,
                RangeBand::Field,
                DurationBand::Sustained,
                PrecisionBand::Directed,
            ),
            "can conceal a coordinated group from ordinary recognition",
            "touches other minds and therefore carries a strict consent boundary",
            "recognition gap with a relational synchronization echo",
            "coercive suppression is prohibited and may constitute Illegal Synthesis",
        ),
        recipe(
            "recipe.invisibility.presynce-surface",
            "Presynce Visible-Surface Alteration",
            PowerOutcome::Invisibility,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Presynce,
                "the physical visible surface bends, textures, and colors itself",
                "psychometry reads adjacent material so the surface can answer it",
            )],
            profile(
                3,
                3,
                ResourceCost::Heavy,
                ResourceCost::Moderate,
                StrainBand::High,
                RangeBand::Touch,
                DurationBand::Scene,
                PrecisionBand::Fine,
            ),
            "creates a physical camouflage that remains visible to non-mental sensors",
            "must continually answer the immediate surface and lighting",
            "altered physical surface with copied material-memory imprint",
            "poor structural matching exposes the subject and may damage skin",
        ),
        recipe(
            "recipe.healing.presynce-stable-form",
            "Presynce Stable-Form Restoration",
            PowerOutcome::Healing,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Presynce,
                "the body is reinforced toward a previously stable physical form",
                "psychometry reads structural history impressed into the injured body",
            )],
            profile(
                3,
                3,
                ResourceCost::Heavy,
                ResourceCost::Moderate,
                StrainBand::High,
                RangeBand::Touch,
                DurationBand::Scene,
                PrecisionBand::Fine,
            ),
            "strong restoration of fractures and known structural damage",
            "cannot safely infer a living condition never held by the patient",
            "restored load pattern with an older structural-memory echo",
            "forcing a disputed prior form can violate identity continuity",
        ),
        recipe(
            "recipe.healing.precog-repair",
            "Precog Living-System Repair",
            PowerOutcome::Healing,
            RecipeProvenance::InstitutionalTechnique,
            vec![contribution(
                HuemanFaculty::Precog,
                "biokinesis performs bounded living intervention and repair",
                "clairvoyance diagnoses hidden state and developing conditions",
            )],
            profile(
                3,
                3,
                ResourceCost::Moderate,
                ResourceCost::Heavy,
                StrainBand::High,
                RangeBand::Touch,
                DurationBand::Scene,
                PrecisionBand::Surgical,
            ),
            "highest diagnostic precision for complex living injury",
            "requires evidence, consent, clinical Clearance, and follow-up",
            "biological intervention seam with diagnostic Aura record",
            "an incorrect hidden-state model may worsen a developing condition",
        ),
        recipe(
            "recipe.healing.prefog-tissue",
            "Prefog Replacement Tissue",
            PowerOutcome::Healing,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Prefog,
                "replacement tissue is generated and incubated from viable material",
                "dreamwalking preserves the patient's inward body image as one input",
            )],
            profile(
                3,
                3,
                ResourceCost::Heavy,
                ResourceCost::Moderate,
                StrainBand::Guarded,
                RangeBand::Touch,
                DurationBand::PersistentUntilReversed,
                PrecisionBand::Surgical,
            ),
            "can replace tissue that no longer remains available for direct repair",
            "requires incubation, compatibility testing, and later integration",
            "cultivation matrix carrying an interior memory motif",
            "generated tissue may be viable yet constitutionally incompatible",
        ),
        recipe(
            "recipe.healing.resynce-pattern",
            "Resynce Viable-Pattern Synchronization",
            PowerOutcome::Healing,
            RecipeProvenance::CrossHouseRecipe,
            vec![
                contribution(
                    HuemanFaculty::Resynce,
                    "telekinesis maintains alignment across damaged systems",
                    "telepathy synchronizes intention and sensation with a viable pattern",
                ),
                contribution(
                    HuemanFaculty::Precog,
                    "biokinesis supplies bounded correction to the synchronized systems",
                    "clairvoyance verifies that the reference pattern remains viable",
                ),
            ],
            profile(
                4,
                4,
                ResourceCost::Heavy,
                ResourceCost::Heavy,
                StrainBand::Critical,
                RangeBand::Near,
                DurationBand::Sustained,
                PrecisionBand::Surgical,
            ),
            "coordinates distributed damage that a local repair cannot reach alone",
            "depends on a proven viable pattern and cross-House coordination",
            "relational phase-lock braided with a diagnostic projection",
            "synchronizing to an unsuitable pattern can propagate harm",
        ),
        recipe(
            "recipe.speed.resynce-motion",
            "Resynce Motion Synchronization",
            PowerOutcome::SuperSpeed,
            RecipeProvenance::InstitutionalTechnique,
            vec![contribution(
                HuemanFaculty::Resynce,
                "telekinesis redirects force between intent, body, and terrain",
                "telepathy synchronizes intended and perceived movement",
            )],
            profile(
                2,
                2,
                ResourceCost::Heavy,
                ResourceCost::Moderate,
                StrainBand::High,
                RangeBand::SelfOnly,
                DurationBand::Sustained,
                PrecisionBand::Directed,
            ),
            "rapid acceleration and responsive changes of direction",
            "does not independently reinforce the body or predict the route",
            "force-vector wake with intent-motion phase echo",
            "acceleration can exceed the Frame's safe tolerance",
        ),
        recipe(
            "recipe.speed.precog-route",
            "Precog Anticipated Route",
            PowerOutcome::SuperSpeed,
            RecipeProvenance::InstitutionalTechnique,
            vec![contribution(
                HuemanFaculty::Precog,
                "biokinesis reduces reaction delay and prepares probable movement",
                "clairvoyance anticipates the route from disclosed evidence",
            )],
            profile(
                2,
                2,
                ResourceCost::Moderate,
                ResourceCost::Heavy,
                StrainBand::High,
                RangeBand::Field,
                DurationBand::Sustained,
                PrecisionBand::Surgical,
            ),
            "safe route choice can make ordinary motion appear supernaturally fast",
            "speed falls sharply when evidence or route stability is poor",
            "probability corridor with accelerated biological rhythm",
            "an uncertain projection can commit the body to the wrong route",
        ),
        recipe(
            "recipe.speed.presynce-reinforcement",
            "Presynce Acceleration Reinforcement",
            PowerOutcome::SuperSpeed,
            RecipeProvenance::HouseRefinement,
            vec![contribution(
                HuemanFaculty::Presynce,
                "somatokinesis reinforces the body against acceleration and impact",
                "psychometry reads load transfer through body and terrain",
            )],
            profile(
                3,
                3,
                ResourceCost::Heavy,
                ResourceCost::Light,
                StrainBand::Critical,
                RangeBand::SelfOnly,
                DurationBand::Sustained,
                PrecisionBand::Directed,
            ),
            "survives impacts and acceleration that defeat softer methods",
            "reinforcement alone supplies limited propulsion and route awareness",
            "dense load-bearing Current print across each footfall",
            "repeated load spikes can create delayed structural damage",
        ),
        recipe(
            "recipe.speed.integrated",
            "Lawful Integrated Super Speed",
            PowerOutcome::SuperSpeed,
            RecipeProvenance::CrossHouseRecipe,
            vec![
                contribution(
                    HuemanFaculty::Resynce,
                    "telekinesis supplies propulsion and redirection",
                    "telepathy synchronizes intention with movement",
                ),
                contribution(
                    HuemanFaculty::Precog,
                    "biokinesis prepares the body's developing condition",
                    "clairvoyance anticipates the evidence-supported route",
                ),
                contribution(
                    HuemanFaculty::Presynce,
                    "somatokinesis reinforces the accelerating body",
                    "psychometry reads changing load through terrain and Frame",
                ),
            ],
            profile(
                4,
                4,
                ResourceCost::Extreme,
                ResourceCost::Extreme,
                StrainBand::Critical,
                RangeBand::Field,
                DurationBand::Sustained,
                PrecisionBand::Surgical,
            ),
            "combines propulsion, route anticipation, and bodily survival",
            "requires multi-Faculty capacity, proof, recognition, and coordination",
            "braided force, probability, and structural-memory wake",
            "a processor falling out of synchronization can cascade into catastrophic impact",
        ),
    ]
}

#[must_use]
pub fn competing_recipes_for_outcome(
    recipes: &[PowerRecipeDefinition],
    outcome: PowerOutcome,
) -> Vec<&PowerRecipeDefinition> {
    recipes
        .iter()
        .filter(|recipe| recipe.outcome == outcome)
        .collect()
}

fn contribution(
    faculty: HuemanFaculty,
    current_method: &str,
    aura_method: &str,
) -> RecipeContribution {
    let root = faculty_power_root(faculty);
    RecipeContribution {
        faculty,
        authority: root.authority,
        current_root: root.current,
        current_method: current_method.into(),
        aura_root: root.aura,
        aura_method: aura_method.into(),
        processor_principle: root.principle.into(),
    }
}

#[allow(clippy::too_many_arguments)]
fn profile(
    form: u8,
    frame: u8,
    current_cost: ResourceCost,
    aura_cost: ResourceCost,
    strain: StrainBand,
    range: RangeBand,
    duration: DurationBand,
    precision: PrecisionBand,
) -> (RecipeCapacity, RecipeLimits) {
    (
        RecipeCapacity { form, frame },
        RecipeLimits {
            current_cost,
            aura_cost,
            strain,
            range,
            duration,
            precision,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn recipe(
    id: &str,
    name: &str,
    outcome: PowerOutcome,
    provenance: RecipeProvenance,
    contributions: Vec<RecipeContribution>,
    (capacity, limits): (RecipeCapacity, RecipeLimits),
    strength: &str,
    weakness: &str,
    trace: &str,
    risk: &str,
) -> PowerRecipeDefinition {
    let mut restrictions = vec![
        RecipeRestriction::AddressableEvidence,
        RecipeRestriction::CapacityBound,
        RecipeRestriction::CompatibleFormAndFrame,
        RecipeRestriction::HouseProof,
        RecipeRestriction::NoOutcomeOwnership,
        RecipeRestriction::NoAutomaticAuthority,
    ];
    if outcome == PowerOutcome::Healing {
        restrictions.extend([
            RecipeRestriction::ConsentWhenAnotherBeingIsAffected,
            RecipeRestriction::GlaushouseClearanceForLivingIntervention,
            RecipeRestriction::StonebendRecognitionForStableForm,
        ]);
    }
    if id == "recipe.invisibility.resynce-recognition" {
        restrictions.push(RecipeRestriction::ConsentWhenAnotherBeingIsAffected);
    }
    PowerRecipeDefinition {
        id: id.into(),
        name: name.into(),
        outcome,
        provenance,
        standing: RecipeStanding::Proposed,
        contributions,
        capacity,
        limits,
        unlock_requirements: UNIVERSAL_UNLOCK_REQUIREMENTS.to_vec(),
        restrictions,
        strengths: vec![strength.into()],
        weaknesses: vec![weakness.into()],
        trace: trace.into(),
        risks: vec![risk.into()],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PowerRecipeInstitutionId {
    StonebendUniversalProvingArena,
    SandmanorContestOfImprovement,
    GlaushouseClinicalPracticum,
    FlyntManticorpAcademy,
}

impl PowerRecipeInstitutionId {
    pub const ALL: [Self; 4] = [
        Self::StonebendUniversalProvingArena,
        Self::SandmanorContestOfImprovement,
        Self::GlaushouseClinicalPracticum,
        Self::FlyntManticorpAcademy,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerRecipeInstitution {
    pub id: PowerRecipeInstitutionId,
    pub name: String,
    pub faculties: Vec<HuemanFaculty>,
    pub functions: Vec<String>,
    pub may_claim_an_outcome: bool,
}

#[must_use]
pub fn draft_power_recipe_institutions() -> [PowerRecipeInstitution; 4] {
    [
        PowerRecipeInstitution {
            id: PowerRecipeInstitutionId::StonebendUniversalProvingArena,
            name: "Stonebend Universal Proving Arena".into(),
            faculties: vec![HuemanFaculty::Presynce],
            functions: vec![
                "subject Recipes and Forms to public resistance".into(),
                "witness stability, restraint, repeatability, reciprocity, and identity".into(),
                "distinguish innovation from Illegal Synthesis".into(),
            ],
            may_claim_an_outcome: false,
        },
        PowerRecipeInstitution {
            id: PowerRecipeInstitutionId::SandmanorContestOfImprovement,
            name: "Sandmanor Contest of Improvement".into(),
            faculties: vec![HuemanFaculty::Prefog, HuemanFaculty::Prefig],
            functions: vec![
                "compare competing methods through reciprocal cultivation".into(),
                "return failed Prefig evidence to Prefog without erasure".into(),
                "produce House proof without choosing for a capable Being".into(),
            ],
            may_claim_an_outcome: false,
        },
        PowerRecipeInstitution {
            id: PowerRecipeInstitutionId::GlaushouseClinicalPracticum,
            name: "Glaüshouse Clinical Practicum".into(),
            faculties: vec![HuemanFaculty::Precog],
            functions: vec![
                "diagnose living compatibility and developing conditions".into(),
                "create and monitor host-specific Sympiote grafts".into(),
                "clear living intervention and direct recovery".into(),
            ],
            may_claim_an_outcome: false,
        },
        PowerRecipeInstitution {
            id: PowerRecipeInstitutionId::FlyntManticorpAcademy,
            name: "Manticorp Academy".into(),
            faculties: vec![HuemanFaculty::Resynce],
            functions: vec![
                "train Telekinesis and Telepathy through Resynce".into(),
                "train resistance, persistence, and recognition".into(),
                "train operational coordination and lawful accession".into(),
            ],
            may_claim_an_outcome: false,
        },
    ]
}

pub fn validate_power_recipe_institutions(
    institutions: &[PowerRecipeInstitution],
) -> Result<(), PowerRecipeInstitutionError> {
    let mut ids = BTreeSet::new();
    let mut faculties = Vec::new();
    for institution in institutions {
        if !ids.insert(institution.id) {
            return Err(PowerRecipeInstitutionError::DuplicateInstitution(
                institution.id,
            ));
        }
        if institution.name.trim().is_empty()
            || institution.functions.is_empty()
            || institution
                .functions
                .iter()
                .any(|function| function.trim().is_empty())
        {
            return Err(PowerRecipeInstitutionError::MissingFunction(institution.id));
        }
        if institution.may_claim_an_outcome {
            return Err(PowerRecipeInstitutionError::OutcomeOwnershipClaim(
                institution.id,
            ));
        }
        let expected_faculties: &[HuemanFaculty] = match institution.id {
            PowerRecipeInstitutionId::StonebendUniversalProvingArena => &[HuemanFaculty::Presynce],
            PowerRecipeInstitutionId::SandmanorContestOfImprovement => {
                &[HuemanFaculty::Prefog, HuemanFaculty::Prefig]
            }
            PowerRecipeInstitutionId::GlaushouseClinicalPracticum => &[HuemanFaculty::Precog],
            PowerRecipeInstitutionId::FlyntManticorpAcademy => &[HuemanFaculty::Resynce],
        };
        if institution.faculties != expected_faculties {
            return Err(PowerRecipeInstitutionError::FacultyMismatch(institution.id));
        }
        for faculty in &institution.faculties {
            if !faculties.contains(faculty) {
                faculties.push(*faculty);
            }
        }
    }
    for id in PowerRecipeInstitutionId::ALL {
        if !ids.contains(&id) {
            return Err(PowerRecipeInstitutionError::MissingInstitution(id));
        }
    }
    for root in FACULTY_POWER_ROOTS {
        if !faculties.contains(&root.faculty) {
            return Err(PowerRecipeInstitutionError::MissingFaculty(root.faculty));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerRecipeInstitutionError {
    DuplicateInstitution(PowerRecipeInstitutionId),
    MissingInstitution(PowerRecipeInstitutionId),
    MissingFunction(PowerRecipeInstitutionId),
    OutcomeOwnershipClaim(PowerRecipeInstitutionId),
    FacultyMismatch(PowerRecipeInstitutionId),
    MissingFaculty(HuemanFaculty),
}

impl std::fmt::Display for PowerRecipeInstitutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "power Recipe institution rejected state: {self:?}"
        )
    }
}

impl std::error::Error for PowerRecipeInstitutionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerWheelRing {
    Instinct,
    Technique,
    Discipline,
    Synthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerWheelNodeKind {
    CurrentBranch,
    AuraBranch,
    CombinedSynthesis,
    HouseRefinement,
    InstitutionalTechnique,
    CrossHouseBridge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerWheelNode {
    pub id: String,
    pub faculty: Option<HuemanFaculty>,
    pub ring: PowerWheelRing,
    pub kind: PowerWheelNodeKind,
    pub recipe_id: Option<String>,
    pub unlock_requirements: Vec<UnlockRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerWheelSector {
    pub faculty: HuemanFaculty,
    pub current_root: CurrentPowerRoot,
    pub aura_root: AuraPowerRoot,
    pub nodes: Vec<PowerWheelNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerWheel {
    pub sectors: Vec<PowerWheelSector>,
    pub cross_house_bridges: Vec<PowerWheelNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerWheelError {
    InvalidCatalog(PowerRecipeError),
    MissingSector(HuemanFaculty),
    DuplicateSector(HuemanFaculty),
    RootMismatch(HuemanFaculty),
    MissingRootBranch(HuemanFaculty),
    DuplicateNode(String),
    UnknownRecipe(String),
    RecipeNotRepresented(String),
    RecipeRepresentedMoreThanOnce(String),
    InvalidRecipePlacement(String),
    MissingNodeRequirement(String, UnlockRequirement),
}

impl std::fmt::Display for PowerWheelError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Power Wheel rejected state: {self:?}")
    }
}

impl std::error::Error for PowerWheelError {}

#[must_use]
pub fn draft_power_wheel(recipes: &[PowerRecipeDefinition]) -> PowerWheel {
    let mut sectors = FACULTY_POWER_ROOTS
        .iter()
        .map(|root| PowerWheelSector {
            faculty: root.faculty,
            current_root: root.current,
            aura_root: root.aura,
            nodes: vec![
                PowerWheelNode {
                    id: format!("wheel.{:?}.current", root.faculty).to_lowercase(),
                    faculty: Some(root.faculty),
                    ring: PowerWheelRing::Instinct,
                    kind: PowerWheelNodeKind::CurrentBranch,
                    recipe_id: None,
                    unlock_requirements: vec![UnlockRequirement::Capacity],
                },
                PowerWheelNode {
                    id: format!("wheel.{:?}.aura", root.faculty).to_lowercase(),
                    faculty: Some(root.faculty),
                    ring: PowerWheelRing::Instinct,
                    kind: PowerWheelNodeKind::AuraBranch,
                    recipe_id: None,
                    unlock_requirements: vec![UnlockRequirement::Capacity],
                },
            ],
        })
        .collect::<Vec<_>>();
    let mut cross_house_bridges = Vec::new();

    for recipe in recipes {
        let node = PowerWheelNode {
            id: format!("wheel.node.{}", recipe.id),
            faculty: (!recipe.is_cross_house()).then(|| recipe.contributions[0].faculty),
            ring: if recipe.is_cross_house() {
                PowerWheelRing::Synthesis
            } else if matches!(
                recipe.outcome,
                PowerOutcome::Flight | PowerOutcome::SuperSpeed
            ) {
                PowerWheelRing::Discipline
            } else {
                PowerWheelRing::Technique
            },
            kind: if recipe.is_cross_house() {
                PowerWheelNodeKind::CrossHouseBridge
            } else {
                match recipe.provenance {
                    RecipeProvenance::InstitutionalTechnique => {
                        PowerWheelNodeKind::InstitutionalTechnique
                    }
                    RecipeProvenance::HouseRefinement => PowerWheelNodeKind::HouseRefinement,
                    _ => PowerWheelNodeKind::CombinedSynthesis,
                }
            },
            recipe_id: Some(recipe.id.clone()),
            unlock_requirements: recipe.unlock_requirements.clone(),
        };
        if recipe.is_cross_house() {
            cross_house_bridges.push(node);
        } else if let Some(sector) = sectors
            .iter_mut()
            .find(|sector| sector.faculty == recipe.contributions[0].faculty)
        {
            sector.nodes.push(node);
        }
    }

    PowerWheel {
        sectors,
        cross_house_bridges,
    }
}

pub fn validate_power_wheel(
    wheel: &PowerWheel,
    recipes: &[PowerRecipeDefinition],
) -> Result<(), PowerWheelError> {
    validate_power_recipe_catalog(recipes).map_err(PowerWheelError::InvalidCatalog)?;
    let recipe_by_id = recipes
        .iter()
        .map(|recipe| (recipe.id.as_str(), recipe))
        .collect::<BTreeMap<_, _>>();
    let mut represented = BTreeMap::<String, usize>::new();
    let mut node_ids = BTreeSet::new();

    for root in FACULTY_POWER_ROOTS {
        let matching = wheel
            .sectors
            .iter()
            .filter(|sector| sector.faculty == root.faculty)
            .collect::<Vec<_>>();
        if matching.is_empty() {
            return Err(PowerWheelError::MissingSector(root.faculty));
        }
        if matching.len() > 1 {
            return Err(PowerWheelError::DuplicateSector(root.faculty));
        }
        let sector = matching[0];
        if sector.current_root != root.current || sector.aura_root != root.aura {
            return Err(PowerWheelError::RootMismatch(root.faculty));
        }
        if !sector.nodes.iter().any(|node| {
            node.kind == PowerWheelNodeKind::CurrentBranch && node.ring == PowerWheelRing::Instinct
        }) || !sector.nodes.iter().any(|node| {
            node.kind == PowerWheelNodeKind::AuraBranch && node.ring == PowerWheelRing::Instinct
        }) {
            return Err(PowerWheelError::MissingRootBranch(root.faculty));
        }
    }

    for node in wheel
        .sectors
        .iter()
        .flat_map(|sector| sector.nodes.iter())
        .chain(wheel.cross_house_bridges.iter())
    {
        if !node_ids.insert(node.id.clone()) {
            return Err(PowerWheelError::DuplicateNode(node.id.clone()));
        }
        let Some(recipe_id) = &node.recipe_id else {
            continue;
        };
        let recipe = recipe_by_id
            .get(recipe_id.as_str())
            .ok_or_else(|| PowerWheelError::UnknownRecipe(recipe_id.clone()))?;
        *represented.entry(recipe_id.clone()).or_default() += 1;
        if recipe.is_cross_house() != matches!(node.kind, PowerWheelNodeKind::CrossHouseBridge) {
            return Err(PowerWheelError::InvalidRecipePlacement(recipe_id.clone()));
        }
        if !recipe.is_cross_house() && node.faculty != Some(recipe.contributions[0].faculty) {
            return Err(PowerWheelError::InvalidRecipePlacement(recipe_id.clone()));
        }
        for requirement in UNIVERSAL_UNLOCK_REQUIREMENTS {
            if !node.unlock_requirements.contains(&requirement) {
                return Err(PowerWheelError::MissingNodeRequirement(
                    node.id.clone(),
                    requirement,
                ));
            }
        }
    }

    for recipe in recipes {
        match represented.get(&recipe.id).copied().unwrap_or(0) {
            0 => return Err(PowerWheelError::RecipeNotRepresented(recipe.id.clone())),
            1 => {}
            _ => {
                return Err(PowerWheelError::RecipeRepresentedMoreThanOnce(
                    recipe.id.clone(),
                ));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PowerWheelProgress {
    pub capacity: u8,
    pub proof: bool,
    pub recognition: bool,
    pub compatible_form_and_frame: bool,
    pub discovered_or_taught_recipes: BTreeSet<String>,
}

impl PowerWheelProgress {
    #[must_use]
    pub fn missing_requirements(&self, recipe: &PowerRecipeDefinition) -> Vec<UnlockRequirement> {
        let mut missing = Vec::new();
        let capacity = recipe.capacity.form.max(recipe.capacity.frame);
        if self.capacity < capacity {
            missing.push(UnlockRequirement::Capacity);
        }
        if !self.proof {
            missing.push(UnlockRequirement::Proof);
        }
        if !self.recognition {
            missing.push(UnlockRequirement::Recognition);
        }
        if !self.compatible_form_and_frame {
            missing.push(UnlockRequirement::CompatibleFormAndFrame);
        }
        if !self.discovered_or_taught_recipes.contains(&recipe.id) {
            missing.push(UnlockRequirement::DiscoveredOrTaughtRecipe);
        }
        missing
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PreparedPowerSet {
    recipe_ids: Vec<String>,
}

impl PreparedPowerSet {
    #[must_use]
    pub fn recipe_ids(&self) -> &[String] {
        &self.recipe_ids
    }

    pub fn equip(
        &mut self,
        recipe_id: &str,
        catalog: &[PowerRecipeDefinition],
        progress: &PowerWheelProgress,
    ) -> Result<(), PreparedPowerSetError> {
        let recipe = catalog
            .iter()
            .find(|recipe| recipe.id == recipe_id)
            .ok_or_else(|| PreparedPowerSetError::UnknownRecipe(recipe_id.into()))?;
        if recipe.standing == RecipeStanding::Prohibited
            || matches!(
                recipe.provenance,
                RecipeProvenance::ProhibitedRecipe | RecipeProvenance::IllegalSynthesis
            )
        {
            return Err(PreparedPowerSetError::ProhibitedRecipe(recipe_id.into()));
        }
        if self.recipe_ids.iter().any(|existing| existing == recipe_id) {
            return Err(PreparedPowerSetError::AlreadyPrepared(recipe_id.into()));
        }
        if self.recipe_ids.len() >= MAX_PREPARED_POWER_RECIPES {
            return Err(PreparedPowerSetError::PreparedSetFull);
        }
        let missing = progress.missing_requirements(recipe);
        if !missing.is_empty() {
            return Err(PreparedPowerSetError::MissingRequirements {
                recipe_id: recipe_id.into(),
                missing,
            });
        }
        self.recipe_ids.push(recipe_id.into());
        Ok(())
    }

    pub fn unequip(&mut self, recipe_id: &str) -> bool {
        let Some(index) = self
            .recipe_ids
            .iter()
            .position(|existing| existing == recipe_id)
        else {
            return false;
        };
        self.recipe_ids.remove(index);
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreparedPowerSetError {
    UnknownRecipe(String),
    ProhibitedRecipe(String),
    AlreadyPrepared(String),
    PreparedSetFull,
    MissingRequirements {
        recipe_id: String,
        missing: Vec<UnlockRequirement>,
    },
}

impl std::fmt::Display for PreparedPowerSetError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "prepared Power Wheel set rejected state: {self:?}"
        )
    }
}

impl std::error::Error for PreparedPowerSetError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PowerRecipeArchive {
    format: String,
    schema_version: u16,
    checksum: String,
    recipes: Vec<PowerRecipeDefinition>,
}

pub fn encode_power_recipe_catalog(
    recipes: &[PowerRecipeDefinition],
) -> Result<Vec<u8>, PowerRecipeError> {
    validate_power_recipe_catalog(recipes)?;
    let checksum = checksum(recipes)?;
    serde_json::to_vec(&PowerRecipeArchive {
        format: POWER_RECIPE_ARCHIVE_FORMAT.into(),
        schema_version: POWER_RECIPE_ARCHIVE_SCHEMA_VERSION,
        checksum,
        recipes: recipes.to_vec(),
    })
    .map_err(|error| PowerRecipeError::Json(error.to_string()))
}

pub fn decode_power_recipe_catalog(
    bytes: &[u8],
) -> Result<Vec<PowerRecipeDefinition>, PowerRecipeError> {
    let archive: PowerRecipeArchive =
        serde_json::from_slice(bytes).map_err(|error| PowerRecipeError::Json(error.to_string()))?;
    if archive.format != POWER_RECIPE_ARCHIVE_FORMAT {
        return Err(PowerRecipeError::UnsupportedFormat(archive.format));
    }
    if archive.schema_version != POWER_RECIPE_ARCHIVE_SCHEMA_VERSION {
        return Err(PowerRecipeError::UnsupportedSchema(archive.schema_version));
    }
    if archive.checksum != checksum(&archive.recipes)? {
        return Err(PowerRecipeError::ChecksumMismatch);
    }
    validate_power_recipe_catalog(&archive.recipes)?;
    Ok(archive.recipes)
}

pub fn replay_power_recipe_catalog(
    recipes: &[PowerRecipeDefinition],
) -> Result<Vec<PowerRecipeDefinition>, PowerRecipeError> {
    decode_power_recipe_catalog(&encode_power_recipe_catalog(recipes)?)
}

fn checksum<T: Serialize + ?Sized>(value: &T) -> Result<String, PowerRecipeError> {
    let bytes =
        serde_json::to_vec(value).map_err(|error| PowerRecipeError::Json(error.to_string()))?;
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Ok(format!("{hash:016x}"))
}
