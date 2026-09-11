//! Canonical Hollow Deck gameplay catalog.
//!
//! The Deck is an additive, presentation-neutral index over Hollow Grove's
//! supernatural cultivation system.  It deliberately keeps Current Form
//! (body), Aura Set (mind), ordinary Recipes, collectible Synthesis Recipes,
//! and Synthesis results as separate types.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

pub const HOLLOW_DECK_SCHEMA_VERSION: &str = "1.5.0";
pub const HUEMAN_STATS: StatBlock = StatBlock::new(50, 50, 50, 50);
pub const FULL_EXPRESSION_STAT_CEILING: u16 = 320;
pub const FAVORABLE_AURA_MODIFIER: f32 = 1.10;
pub const UNFAVORABLE_AURA_MODIFIER: f32 = 0.90;
pub const NEUTRAL_AURA_MODIFIER: f32 = 1.00;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dominion {
    Presynce,
    Prefog,
    Prefig,
    Precog,
    Resynce,
}

impl Dominion {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presynce => "Presynce",
            Self::Prefog => "Prefog",
            Self::Prefig => "Prefig",
            Self::Precog => "Precog",
            Self::Resynce => "Resynce",
        }
    }
}

impl From<Dominion> for crate::world::hueman_faculties::HuemanFaculty {
    fn from(value: Dominion) -> Self {
        match value {
            Dominion::Presynce => Self::Presynce,
            Dominion::Prefog => Self::Prefog,
            Dominion::Prefig => Self::Prefig,
            Dominion::Precog => Self::Precog,
            Dominion::Resynce => Self::Resynce,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentType {
    Regular,
    Hollow,
}

impl CurrentType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Regular => "Regular Current",
            Self::Hollow => "Hollow Current",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuraMaterial {
    Diamond,
    Crystal,
    Jade,
    Emerald,
    Opal,
}

impl AuraMaterial {
    pub const ALL: [Self; 5] = [
        Self::Diamond,
        Self::Jade,
        Self::Emerald,
        Self::Opal,
        Self::Crystal,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diamond => "Diamond",
            Self::Crystal => "Crystal",
            Self::Jade => "Jade",
            Self::Emerald => "Emerald",
            Self::Opal => "Opal",
        }
    }

    #[must_use]
    pub const fn alchemy_slot(self) -> AlchemySlot {
        match self {
            Self::Diamond => AlchemySlot::Stonebend,
            Self::Crystal => AlchemySlot::Sandmanor,
            Self::Jade | Self::Emerald => AlchemySlot::Glaushouse,
            Self::Opal => AlchemySlot::Flynt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlchemySlot {
    Stonebend,
    Glaushouse,
    Flynt,
    Sandmanor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatBlock {
    pub strength: u8,
    pub speed: u8,
    pub attack: u8,
    pub defense: u8,
}

impl StatBlock {
    #[must_use]
    pub const fn new(strength: u8, speed: u8, attack: u8, defense: u8) -> Self {
        Self {
            strength,
            speed,
            attack,
            defense,
        }
    }

    #[must_use]
    pub const fn computed_total(self) -> u16 {
        self.strength as u16 + self.speed as u16 + self.attack as u16 + self.defense as u16
    }
}

#[must_use]
pub const fn get_stat_total(stats: StatBlock) -> u16 {
    stats.computed_total()
}

pub fn validate_stat_block(stats: StatBlock) -> Result<(), DeckValidationError> {
    if stats.strength > 100 || stats.speed > 100 || stats.attack > 100 || stats.defense > 100 {
        return Err(DeckValidationError::new(
            "stat.range",
            "STR, SPD, ATK, and DEF must each remain between 0 and 100",
        ));
    }
    if stats.computed_total() > FULL_EXPRESSION_STAT_CEILING {
        return Err(DeckValidationError::new(
            "stat.ceiling",
            format!(
                "stat total {} exceeds the full-expression ceiling of {FULL_EXPRESSION_STAT_CEILING}",
                stats.computed_total()
            ),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseEntry {
    pub id: House,
    pub display_name: String,
    pub dominions: Vec<Dominion>,
    pub current_type: CurrentType,
    pub aura_materials: Vec<AuraMaterial>,
    /// Canonical compass placement. Catalogs since 1.1 always populate this;
    /// the option preserves the shape accepted from earlier catalog projections.
    pub direction: Option<String>,
    pub organization: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlaushouseSeat {
    Calli,
    Cleo,
    Melony,
    Teri,
    Talia,
}

impl GlaushouseSeat {
    pub const ALL: [Self; 5] = [
        Self::Calli,
        Self::Cleo,
        Self::Melony,
        Self::Teri,
        Self::Talia,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Calli => "Calli",
            Self::Cleo => "Cleo",
            Self::Melony => "Melony",
            Self::Teri => "Teri",
            Self::Talia => "Talia",
        }
    }

    #[must_use]
    pub fn entry_title(self) -> String {
        format!("Nightingale de {}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlaushouseBranch {
    Presensor,
    Preceptor,
}

impl GlaushouseBranch {
    pub const ALL: [Self; 2] = [Self::Presensor, Self::Preceptor];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presensor => "Presensor",
            Self::Preceptor => "Preceptor",
        }
    }

    #[must_use]
    pub const fn role_family(self) -> &'static str {
        match self {
            Self::Presensor => "Matron",
            Self::Preceptor => "Marshall",
        }
    }

    #[must_use]
    pub const fn aura_material(self) -> AuraMaterial {
        match self {
            Self::Presensor => AuraMaterial::Jade,
            Self::Preceptor => AuraMaterial::Emerald,
        }
    }

    #[must_use]
    pub fn achieved_title(self, seat: GlaushouseSeat) -> String {
        format!("{} de {}", self.role_family(), seat.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlaushouseBranchPath {
    pub branch: GlaushouseBranch,
    pub evolution_path_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlaushouseStructure {
    pub direction: String,
    pub umbrella_faculty: Dominion,
    pub seats: Vec<GlaushouseSeat>,
    pub entry_title: String,
    pub branch_paths: Vec<GlaushouseBranchPath>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandmanorCulture {
    Minorian,
    Minoan,
}

impl SandmanorCulture {
    pub const ALL: [Self; 2] = [Self::Minorian, Self::Minoan];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minorian => "Minorian",
            Self::Minoan => "Minoan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandmanorMasteryTier {
    Baseline,
    Intermediate,
    Apex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandmanorSocialIdentity {
    Hearthkeeper,
    Householder,
    Husbandman,
    BeachBum,
    PartyBoy,
    Socialite,
}

impl SandmanorSocialIdentity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hearthkeeper => "Hearthkeeper",
            Self::Householder => "Householder",
            Self::Husbandman => "Husbandman",
            Self::BeachBum => "Beach Bum",
            Self::PartyBoy => "Party Boy",
            Self::Socialite => "Socialite",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandmanorAuraMastery {
    Geomancer,
    Labyrinthian,
    Demiurge,
    Acolyte,
    Navigator,
    Ascendant,
}

impl SandmanorAuraMastery {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Geomancer => "Geomancer",
            Self::Labyrinthian => "Labyrinthian",
            Self::Demiurge => "Demiurge",
            Self::Acolyte => "Acolyte",
            Self::Navigator => "Navigator",
            Self::Ascendant => "Ascendant",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandmanorSovereignIdentity {
    Transcendent,
    Sandman,
}

impl SandmanorSovereignIdentity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transcendent => "The Transcendent",
            Self::Sandman => "The Sandman",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorSocialStage {
    pub identity: SandmanorSocialIdentity,
    pub tier: SandmanorMasteryTier,
    pub meaning: String,
    pub visual_direction: Vec<String>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorAuraStage {
    pub identity: SandmanorAuraMastery,
    pub tier: SandmanorMasteryTier,
    pub meaning: String,
    pub question: String,
    pub examples: Vec<String>,
    pub visual_direction: Vec<String>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorFacultyModel {
    pub faculty: Dominion,
    pub core_question: String,
    pub function: String,
    pub examples: Vec<String>,
    pub excluded_interpretation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorCulturePath {
    pub culture: SandmanorCulture,
    pub orientation: Vec<String>,
    pub faculty: SandmanorFacultyModel,
    pub social_path: Vec<SandmanorSocialStage>,
    pub social_progression: Vec<String>,
    pub aura_path: Vec<SandmanorAuraStage>,
    pub aura_progression: Vec<String>,
    pub nature_path_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorSocialPairing {
    pub minorian: SandmanorSocialIdentity,
    pub minoan: SandmanorSocialIdentity,
    pub contrast: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorAuraPairing {
    pub minorian: SandmanorAuraMastery,
    pub minoan: SandmanorAuraMastery,
    pub contrast: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorSovereignSynthesis {
    pub peer_apexes: Vec<SandmanorAuraMastery>,
    pub synthesis_identity: SandmanorSovereignIdentity,
    pub synthesis_formula: String,
    pub sovereign_identity: SandmanorSovereignIdentity,
    pub sovereign_embodiment_of: SandmanorSovereignIdentity,
    pub sovereign_formula: String,
    pub meaning: String,
    pub core_principle: Vec<String>,
    pub social_boundary: String,
    pub nature_boundary: String,
    pub visual_direction: Vec<String>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandmanorInstitutionalHierarchy {
    pub direction: String,
    pub current_type: CurrentType,
    pub aura_material: AuraMaterial,
    pub physical_philosophy: String,
    pub visual_character: Vec<String>,
    pub cultures: Vec<SandmanorCulturePath>,
    pub social_pairings: Vec<SandmanorSocialPairing>,
    pub aura_pairings: Vec<SandmanorAuraPairing>,
    pub sovereign_synthesis: SandmanorSovereignSynthesis,
    pub axis_invariant: String,
}

/// Canonical meanings for axes that must never be collapsed into one rank.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstitutionalAxisModel {
    pub biology: String,
    pub biological_growth: String,
    pub citizenship: String,
    pub institutional_service: String,
    pub institutional_mastery: String,
    pub specialist_discipline: String,
    pub synthesis_apex: String,
    pub operating_organization: String,
    pub social_alias: String,
    pub visual_presentation: String,
    pub renderer_invariant: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntCitizenship {
    Warden,
}

impl FlyntCitizenship {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Warden => "Warden",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntServiceStatus {
    Corpman,
}

impl FlyntServiceStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Corpman => "Corpman",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntEliteStatus {
    MysteryMan,
}

impl FlyntEliteStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MysteryMan => "Mystery Man",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntSect {
    Hunter,
    Chief,
    Mariner,
}

impl FlyntSect {
    pub const ALL: [Self; 3] = [Self::Hunter, Self::Chief, Self::Mariner];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hunter => "Hunter",
            Self::Chief => "Chief",
            Self::Mariner => "Mariner",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntSectTradition {
    pub sect: FlyntSect,
    pub nature_id: String,
    pub primary_trait_id: String,
    pub vocation: Vec<String>,
    pub visual_direction: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntOperatingOrganization {
    WeFairyMen,
    Gallows,
    Marquise,
}

impl FlyntOperatingOrganization {
    pub const ALL: [Self; 3] = [Self::WeFairyMen, Self::Gallows, Self::Marquise];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WeFairyMen => "We Fairy Men",
            Self::Gallows => "The Gallows",
            Self::Marquise => "The Marquise",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntOperatingContext {
    pub organization: FlyntOperatingOrganization,
    pub social_basis: Vec<String>,
    pub environment: Vec<String>,
    pub tradition_affinity: Option<FlyntSect>,
    pub relationship_to_hierarchy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntConvergenceMastery {
    pub required_sects: Vec<FlyntSect>,
    pub convergence_identity: String,
    pub convergence_nature_id: String,
    pub convergence_meaning: String,
    pub convergence_visual_direction: String,
    pub perfected_from_identity: String,
    pub perfected_identity: String,
    pub perfected_nature_id: String,
    pub perfection_meaning: String,
    pub perfection_visual_direction: String,
    pub apex_association: String,
    pub legacy_pair_compatibility: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntSorceryIdentity {
    Wizard,
    Witch,
    Warlock,
    Enchantress,
    Sorcerer,
    Sorceress,
}

impl FlyntSorceryIdentity {
    pub const ALL: [Self; 6] = [
        Self::Wizard,
        Self::Witch,
        Self::Warlock,
        Self::Enchantress,
        Self::Sorcerer,
        Self::Sorceress,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wizard => "Wizard",
            Self::Witch => "Witch",
            Self::Warlock => "Warlock",
            Self::Enchantress => "Enchantress",
            Self::Sorcerer => "Sorcerer",
            Self::Sorceress => "Sorceress",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntSorceryFunction {
    EstablishedManufacture,
    PatternModification,
    PatternAuthorship,
}

impl FlyntSorceryFunction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EstablishedManufacture => {
                "Manufacture established Synthesis Recipes and products"
            }
            Self::PatternModification => "Alter existing Synthesis patterns",
            Self::PatternAuthorship => "Author new Synthesis Recipes and patterns",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntSorceryPair {
    pub male_identity: FlyntSorceryIdentity,
    pub female_identity: FlyntSorceryIdentity,
    pub function: FlyntSorceryFunction,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntEcosystemRole {
    Tross,
    MysteryMen,
    Sorcery,
    Gallows,
    WeFairyMen,
    Marquise,
}

impl FlyntEcosystemRole {
    pub const ALL: [Self; 6] = [
        Self::Tross,
        Self::MysteryMen,
        Self::Sorcery,
        Self::Gallows,
        Self::WeFairyMen,
        Self::Marquise,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tross => "The Tross",
            Self::MysteryMen => "Mystery Men",
            Self::Sorcery => "Sorcery",
            Self::Gallows => "The Gallows",
            Self::WeFairyMen => "We Fairy Men",
            Self::Marquise => "The Marquise",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntEcosystemRoleProfile {
    pub role: FlyntEcosystemRole,
    pub function: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntSorcerySystem {
    pub domain: String,
    pub paired_ladder: Vec<FlyntSorceryPair>,
    pub ecosystem_roles: Vec<FlyntEcosystemRoleProfile>,
    pub governance: String,
    pub systemic_danger: String,
    pub sanctioned_practitioner_boundary: String,
    pub axis_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntInstitutionalHierarchy {
    pub citizen_baseline: FlyntCitizenship,
    pub service_status: FlyntServiceStatus,
    pub service_prerequisite: FlyntCitizenship,
    pub capacity_requirement: String,
    pub elite_status: FlyntEliteStatus,
    pub elite_prerequisite: FlyntServiceStatus,
    pub sect_prerequisite: FlyntEliteStatus,
    pub sects: Vec<FlyntSectTradition>,
    pub convergence: FlyntConvergenceMastery,
    pub sorcery: FlyntSorcerySystem,
    pub operating_contexts: Vec<FlyntOperatingContext>,
    pub citizen_visual_direction: Vec<String>,
    pub service_visual_direction: Vec<String>,
    pub elite_visual_direction: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntPrincipalInstitutionalStack {
    pub nature_id: String,
    pub citizenship: FlyntCitizenship,
    pub service_history: FlyntServiceStatus,
    pub elite_order: FlyntEliteStatus,
    pub sect: FlyntSect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StonebendCitizenship {
    Gerald,
}

impl StonebendCitizenship {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gerald => "Gerald",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StonebendTrainingStatus {
    Apprentice,
}

impl StonebendTrainingStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Apprentice => "Apprentice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstitutionalChannel {
    Current,
    Aura,
}

impl InstitutionalChannel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "Current",
            Self::Aura => "Aura",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StonebendCompetency {
    Mason,
    Mage,
    Pugilist,
    Foreman,
}

impl StonebendCompetency {
    pub const ALL: [Self; 4] = [Self::Mason, Self::Mage, Self::Pugilist, Self::Foreman];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mason => "Mason",
            Self::Mage => "Mage",
            Self::Pugilist => "Pugilist",
            Self::Foreman => "Foreman",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendCompetencyProfile {
    pub competency: StonebendCompetency,
    pub channel: InstitutionalChannel,
    pub aura_material: Option<AuraMaterial>,
    pub concepts: Vec<String>,
    pub visual_direction: Vec<String>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StonebendMastery {
    Freemason,
    Proliteriate,
}

impl StonebendMastery {
    pub const ALL: [Self; 2] = [Self::Freemason, Self::Proliteriate];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Freemason => "Freemason",
            Self::Proliteriate => "Proliteriate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendOrderPath {
    pub mastery: StonebendMastery,
    pub cultural_identity: String,
    pub current_competency: StonebendCompetency,
    pub aura_competency: StonebendCompetency,
    pub synthesis_meaning: String,
    pub visual_direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendApexMastery {
    pub peer_inputs: Vec<StonebendMastery>,
    pub required_cultural_identities: Vec<String>,
    pub identity: String,
    pub meaning: String,
    pub visual_direction: Vec<String>,
    pub avoid: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendPhysicalPhilosophy {
    pub faculty: Dominion,
    pub current_type: CurrentType,
    pub aura_material: AuraMaterial,
    pub philosophy: String,
    pub process: String,
    pub presynce_meaning: String,
    pub presynce_questions: Vec<String>,
    pub cut_meaning: String,
    pub cut_examples: Vec<String>,
    pub hollowing_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendInstitutionalHierarchy {
    pub citizen_baseline: StonebendCitizenship,
    pub training_entry: StonebendTrainingStatus,
    pub training_prerequisite: StonebendCitizenship,
    pub competency_prerequisite: StonebendTrainingStatus,
    pub competencies: Vec<StonebendCompetencyProfile>,
    pub orders: Vec<StonebendOrderPath>,
    pub apex: StonebendApexMastery,
    pub philosophy: StonebendPhysicalPhilosophy,
    pub citizen_visual_direction: Vec<String>,
    pub apprentice_visual_direction: Vec<String>,
    pub constitutional_compatibility: String,
}

impl StonebendInstitutionalHierarchy {
    #[must_use]
    pub fn contains_active_identity(&self, identity: &str) -> bool {
        let needle = normalized(identity);
        normalized(self.citizen_baseline.as_str()) == needle
            || normalized(self.training_entry.as_str()) == needle
            || self
                .competencies
                .iter()
                .any(|entry| normalized(entry.competency.as_str()) == needle)
            || self.orders.iter().any(|entry| {
                normalized(entry.mastery.as_str()) == needle
                    || normalized(&entry.cultural_identity) == needle
            })
            || normalized(&self.apex.identity) == needle
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedStonebendOrderMastery {
    pub nature_id: String,
    pub citizenship: StonebendCitizenship,
    pub training_status: StonebendTrainingStatus,
    pub current_competency: StonebendCompetency,
    pub aura_competency: StonebendCompetency,
    pub synthesis_mastery: StonebendMastery,
    pub cultural_identity: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlyntOrganization {
    MysteryMen,
    WeFairyMen,
    Gallows,
}

impl FlyntOrganization {
    pub const ALL: [Self; 3] = [Self::MysteryMen, Self::WeFairyMen, Self::Gallows];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MysteryMen => "Mystery Men",
            Self::WeFairyMen => "We Fairy Men",
            Self::Gallows => "The Gallows",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntSocialIdentity {
    pub id: String,
    pub name: String,
    pub organization: FlyntOrganization,
    pub visual_direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntVisualIdentityProfile {
    /// Canonical biological Nature. Contextual identities never replace this ID.
    pub nature_id: String,
    pub mystery_men_identity: FlyntSocialIdentity,
    pub we_fairy_men_identity: FlyntSocialIdentity,
    pub gallows_identity: FlyntSocialIdentity,
    pub symbol_anchor: String,
    pub mythic_source: String,
    pub secondary_echoes: Vec<String>,
    pub motif_direction: String,
}

impl FlyntVisualIdentityProfile {
    #[must_use]
    pub const fn identity(&self, organization: FlyntOrganization) -> &FlyntSocialIdentity {
        match organization {
            FlyntOrganization::MysteryMen => &self.mystery_men_identity,
            FlyntOrganization::WeFairyMen => &self.we_fairy_men_identity,
            FlyntOrganization::Gallows => &self.gallows_identity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntInstitutionalVisualLanguage {
    pub organization: FlyntOrganization,
    pub visual_character: Vec<String>,
    pub social_power: Vec<String>,
    pub wardrobe: Vec<String>,
    pub environment: Vec<String>,
    pub animation_direction: String,
    pub avoid: Vec<String>,
}

/// Render-layer selection. Biological and social identity remain distinct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntArtSelection {
    pub nature_id: String,
    pub growth_stage: Option<String>,
    pub organization: FlyntOrganization,
    pub outfit: Option<String>,
    pub accessories: Vec<String>,
    pub environment: Option<String>,
    pub animation_state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedFlyntArtIdentity {
    pub nature_id: String,
    pub growth_stage: Option<String>,
    pub citizenship: FlyntCitizenship,
    pub service_history: FlyntServiceStatus,
    pub elite_order: FlyntEliteStatus,
    pub sect: FlyntSect,
    pub organization: FlyntOrganization,
    pub social_identity_id: String,
    pub social_identity: String,
    pub identity_direction: String,
    pub outfit: String,
    pub accessories: Vec<String>,
    pub environment: String,
    pub animation_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimaryTrait {
    pub id: String,
    pub name: String,
    pub mechanical_effect: String,
    pub lore_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nature {
    pub id: String,
    pub name: String,
    pub house_id: House,
    pub dominion_id: Dominion,
    pub current_type: CurrentType,
    pub aura_material: AuraMaterial,
    pub primary_trait_id: String,
    pub mature_stats: StatBlock,
    pub evolution_path_id: Option<String>,
    pub evolution_index: Option<usize>,
    pub paired_nature_id: Option<String>,
    pub shared_pair_principle: Option<String>,
    pub visual_description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionPath {
    pub id: String,
    pub name: String,
    pub house_id: House,
    pub dominion_id: Dominion,
    pub nature_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialInteraction {
    pub name: String,
    pub mechanical_effect: String,
    pub counterplay: Option<String>,
    pub configurable_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairRelationship {
    pub nature_a: String,
    pub nature_b: String,
    pub shared_principle: String,
    pub primary_trait_a: String,
    pub primary_trait_b: String,
    pub special_interactions: Vec<SpecialInteraction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossHouseCounterpart {
    pub nature_a: String,
    pub nature_b: String,
    pub tier: usize,
    pub stat_budget: u16,
    pub conceptual_contrast: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentFeatureCategory {
    Anatomy,
    WorkingSurface,
    Structure,
    Locomotion,
    BodyState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct StatAdjustment {
    pub strength: i16,
    pub speed: i16,
    pub attack: i16,
    pub defense: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentFeature {
    pub id: String,
    pub name: String,
    pub category: CurrentFeatureCategory,
    pub stages: Vec<String>,
    pub current_stage: Option<String>,
    pub stat_effects: Option<StatAdjustment>,
    pub visual_effects: Option<String>,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentTechnique {
    pub id: String,
    pub name: String,
    pub stages: Vec<String>,
    pub current_stage: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthMilestone {
    pub id: String,
    pub name: String,
    pub visual_effect: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedGrowthPath {
    pub id: String,
    pub nature_id: String,
    pub name: String,
    pub milestones: Vec<GrowthMilestone>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthStageRequirement {
    pub nature_id: String,
    pub milestone_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentForm {
    pub nature_id: String,
    pub current_features: Vec<CurrentFeature>,
    pub current_techniques: Vec<CurrentTechnique>,
    pub growth_path_id: Option<String>,
    pub current_growth_stage: Option<String>,
    pub achieved_growth_stages: Vec<String>,
    /// 0 is Hueman neutrality/the prior rung and 10_000 is this Nature's full expression.
    pub development_basis_points: u16,
    pub computed_stats: StatBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuraMoveMode {
    Active,
    Passive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonStatus {
    Canonical,
    DesignExample,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuraMove {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub mode: Option<AuraMoveMode>,
    pub compatible_natures: Vec<String>,
    pub compatible_dominions: Vec<Dominion>,
    pub effects: Vec<String>,
    pub requirements: Vec<String>,
    pub canon_status: CanonStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuraSet {
    pub nature_id: String,
    pub learned_moves: Vec<String>,
    pub equipped_moves: Vec<String>,
    pub design_examples: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub item: String,
    pub quantity: u16,
}

/// Ordinary real-world crafting instructions. This is intentionally not a
/// Synthesis Recipe and has no supernatural requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub tools: Vec<String>,
    pub station: Option<String>,
    pub requirements: Vec<String>,
    pub result_item: String,
    pub result_quantity: u16,
    pub discovery_source: Option<String>,
    pub known: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynthesisRecipeSourceType {
    Book,
    Page,
    Diagram,
    Blueprint,
    Drawing,
    StickDrawing,
    HistoricalDocument,
    Artifact,
    Schematic,
    Inscription,
    Map,
    Chart,
    Pattern,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SynthesisResultType {
    Ability,
    Characteristic,
}

impl SynthesisResultType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ability => "Ability",
            Self::Characteristic => "Characteristic",
        }
    }
}

/// Collectible supernatural pattern. It is kept separate from both ordinary
/// [`Recipe`] and the older executable Recipe compiler in `synthesis_recipe`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynthesisRecipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_type: SynthesisRecipeSourceType,
    pub source_description: String,
    pub is_discovered: bool,
    pub rarity: Option<String>,
    pub unique_source: bool,
    pub compatible_natures: Vec<String>,
    pub required_current_features: Vec<String>,
    pub required_current_techniques: Vec<String>,
    pub required_growth_stage: Option<GrowthStageRequirement>,
    pub required_aura_moves: Vec<String>,
    pub optional_reagents: Vec<String>,
    pub consumed_reagents: Vec<String>,
    pub result_type: SynthesisResultType,
    pub result_synthesis: String,
    pub mechanical_effect: String,
    pub lore_text: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Synthesis {
    pub id: String,
    pub name: String,
    pub result_type: SynthesisResultType,
    pub source_recipe_id: Option<String>,
    pub compatible_natures: Vec<String>,
    pub current_requirements: Vec<String>,
    pub required_growth_stage: Option<GrowthStageRequirement>,
    pub aura_requirements: Vec<String>,
    pub reagent_requirements: Vec<String>,
    pub effects: Vec<String>,
    pub description: String,
    pub lore_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bane {
    pub id: String,
    pub name: String,
    pub target_natures: Vec<String>,
    pub target_primary_traits: Vec<String>,
    pub effect_type: String,
    pub mechanical_effect: String,
    pub duration: Option<String>,
    pub delivery_method: Option<String>,
    pub strength: Option<String>,
    pub lore_notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraMatchup {
    Favorable,
    Unfavorable,
    Neutral,
}

impl AuraMatchup {
    #[must_use]
    pub const fn modifier(self) -> f32 {
        match self {
            Self::Favorable => FAVORABLE_AURA_MODIFIER,
            Self::Unfavorable => UNFAVORABLE_AURA_MODIFIER,
            Self::Neutral => NEUTRAL_AURA_MODIFIER,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Favorable => "Strong (+10%)",
            Self::Unfavorable => "Weak (-10%)",
            Self::Neutral => "Neutral (0%)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectChannel {
    Aura,
    SynthesisWithAura,
    CurrentPhysical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HollowDeckCatalog {
    pub schema_version: String,
    pub hueman_stats: StatBlock,
    pub institutional_axes: InstitutionalAxisModel,
    pub houses: Vec<HouseEntry>,
    pub glaushouse_structure: GlaushouseStructure,
    pub sandmanor_institution: SandmanorInstitutionalHierarchy,
    pub flynt_institution: FlyntInstitutionalHierarchy,
    pub stonebend_institution: StonebendInstitutionalHierarchy,
    pub dominions: Vec<Dominion>,
    pub natures: Vec<Nature>,
    pub primary_traits: Vec<PrimaryTrait>,
    pub evolution_paths: Vec<EvolutionPath>,
    pub pair_relationships: Vec<PairRelationship>,
    pub cross_house_counterparts: Vec<CrossHouseCounterpart>,
    pub flynt_visual_identities: Vec<FlyntVisualIdentityProfile>,
    pub flynt_visual_languages: Vec<FlyntInstitutionalVisualLanguage>,
    pub current_forms: Vec<CurrentForm>,
    pub growth_paths: Vec<UnifiedGrowthPath>,
    pub aura_moves: Vec<AuraMove>,
    pub aura_sets: Vec<AuraSet>,
    pub recipes: Vec<Recipe>,
    pub synthesis_recipes: Vec<SynthesisRecipe>,
    pub syntheses: Vec<Synthesis>,
    pub banes: Vec<Bane>,
}

impl HollowDeckCatalog {
    #[must_use]
    pub fn nature(&self, id_or_name: &str) -> Option<&Nature> {
        let needle = normalized(id_or_name);
        self.natures.iter().find(|nature| {
            normalized(&nature.id) == needle
                || normalized(&nature.name) == needle
                || (needle == "ork" && nature.id == "orc")
        })
    }

    #[must_use]
    pub fn primary_trait(&self, id_or_name: &str) -> Option<&PrimaryTrait> {
        let needle = normalized(id_or_name);
        self.primary_traits
            .iter()
            .find(|item| normalized(&item.id) == needle || normalized(&item.name) == needle)
    }

    #[must_use]
    pub fn evolution_path(&self, id_or_name: &str) -> Option<&EvolutionPath> {
        let needle = normalized(id_or_name);
        self.evolution_paths
            .iter()
            .find(|item| normalized(&item.id) == needle || normalized(&item.name) == needle)
    }

    #[must_use]
    pub fn current_form(&self, nature_id: &str) -> Option<&CurrentForm> {
        let nature_id = self.nature(nature_id)?.id.as_str();
        self.current_forms
            .iter()
            .find(|form| form.nature_id == nature_id)
    }

    #[must_use]
    pub fn growth_path(&self, id_or_nature: &str) -> Option<&UnifiedGrowthPath> {
        let needle = normalized(id_or_nature);
        self.growth_paths.iter().find(|path| {
            normalized(&path.id) == needle
                || normalized(&path.name) == needle
                || normalized(&path.nature_id) == needle
        })
    }

    #[must_use]
    pub fn flynt_visual_identity(&self, nature: &str) -> Option<&FlyntVisualIdentityProfile> {
        let nature_id = self.nature(nature)?.id.as_str();
        self.flynt_visual_identities
            .iter()
            .find(|profile| profile.nature_id == nature_id)
    }

    #[must_use]
    pub fn sandmanor_culture(&self, culture: SandmanorCulture) -> Option<&SandmanorCulturePath> {
        self.sandmanor_institution
            .cultures
            .iter()
            .find(|path| path.culture == culture)
    }

    #[must_use]
    pub fn sandmanor_social_stage(
        &self,
        identity: SandmanorSocialIdentity,
    ) -> Option<&SandmanorSocialStage> {
        self.sandmanor_institution
            .cultures
            .iter()
            .flat_map(|path| &path.social_path)
            .find(|stage| stage.identity == identity)
    }

    #[must_use]
    pub fn sandmanor_aura_stage(
        &self,
        identity: SandmanorAuraMastery,
    ) -> Option<&SandmanorAuraStage> {
        self.sandmanor_institution
            .cultures
            .iter()
            .flat_map(|path| &path.aura_path)
            .find(|stage| stage.identity == identity)
    }

    #[must_use]
    pub fn flynt_visual_language(
        &self,
        organization: FlyntOrganization,
    ) -> Option<&FlyntInstitutionalVisualLanguage> {
        self.flynt_visual_languages
            .iter()
            .find(|language| language.organization == organization)
    }

    #[must_use]
    pub fn flynt_sect(&self, sect: FlyntSect) -> Option<&FlyntSectTradition> {
        self.flynt_institution
            .sects
            .iter()
            .find(|tradition| tradition.sect == sect)
    }

    #[must_use]
    pub fn flynt_sect_for_nature(&self, nature: &str) -> Option<&FlyntSectTradition> {
        let nature_id = self.nature(nature)?.id.as_str();
        self.flynt_institution
            .sects
            .iter()
            .find(|tradition| tradition.nature_id == nature_id)
    }

    #[must_use]
    pub fn stonebend_competency(
        &self,
        competency: StonebendCompetency,
    ) -> Option<&StonebendCompetencyProfile> {
        self.stonebend_institution
            .competencies
            .iter()
            .find(|profile| profile.competency == competency)
    }

    #[must_use]
    pub fn stonebend_order(&self, mastery: StonebendMastery) -> Option<&StonebendOrderPath> {
        self.stonebend_institution
            .orders
            .iter()
            .find(|path| path.mastery == mastery)
    }

    #[must_use]
    pub fn aura_set(&self, nature_id: &str) -> Option<&AuraSet> {
        let nature_id = self.nature(nature_id)?.id.as_str();
        self.aura_sets.iter().find(|set| set.nature_id == nature_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckValidationError {
    pub code: String,
    pub message: String,
}

impl DeckValidationError {
    fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DeckValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for DeckValidationError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisRequirementReport {
    pub compatible: bool,
    pub missing_current_features: Vec<String>,
    pub missing_current_techniques: Vec<String>,
    pub missing_growth_stage: Option<String>,
    pub missing_aura_moves: Vec<String>,
    pub missing_reagents: Vec<String>,
}

#[must_use]
pub fn canonical_hollow_deck() -> HollowDeckCatalog {
    seed_catalog()
}

#[must_use]
pub fn house_display_name(house: House) -> &'static str {
    match house {
        House::Stonebend => "Stonebend",
        House::Sandmanor => "Sandmanor",
        House::Glaushouse => "Glaüshouse",
        House::Flynt => "Flynt",
    }
}

#[must_use]
pub fn get_house_for_nature(catalog: &HollowDeckCatalog, nature: &str) -> Option<House> {
    catalog.nature(nature).map(|nature| nature.house_id)
}

#[must_use]
pub fn get_dominion_for_nature(catalog: &HollowDeckCatalog, nature: &str) -> Option<Dominion> {
    catalog.nature(nature).map(|nature| nature.dominion_id)
}

#[must_use]
pub fn get_current_type_for_nature(
    catalog: &HollowDeckCatalog,
    nature: &str,
) -> Option<CurrentType> {
    catalog.nature(nature).map(|nature| nature.current_type)
}

#[must_use]
pub fn get_aura_material_for_nature(
    catalog: &HollowDeckCatalog,
    nature: &str,
) -> Option<AuraMaterial> {
    catalog.nature(nature).map(|nature| nature.aura_material)
}

#[must_use]
pub fn get_glaushouse_branch_for_nature(
    catalog: &HollowDeckCatalog,
    nature: &str,
) -> Option<GlaushouseBranch> {
    let nature = catalog.nature(nature)?;
    if nature.house_id != House::Glaushouse {
        return None;
    }
    catalog
        .glaushouse_structure
        .branch_paths
        .iter()
        .find(|branch| nature.evolution_path_id.as_deref() == Some(&branch.evolution_path_id))
        .map(|branch| branch.branch)
}

#[must_use]
pub fn get_primary_trait<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Option<&'a PrimaryTrait> {
    let nature = catalog.nature(nature)?;
    catalog.primary_trait(&nature.primary_trait_id)
}

#[must_use]
pub fn get_evolution_path<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Option<&'a EvolutionPath> {
    let nature = catalog.nature(nature)?;
    catalog.evolution_path(nature.evolution_path_id.as_deref()?)
}

#[must_use]
pub fn get_evolution_tier(catalog: &HollowDeckCatalog, nature: &str) -> Option<usize> {
    catalog
        .nature(nature)?
        .evolution_index
        .map(|index| index + 1)
}

#[must_use]
pub fn get_previous_nature<'a>(catalog: &'a HollowDeckCatalog, nature: &str) -> Option<&'a Nature> {
    let nature = catalog.nature(nature)?;
    let path = get_evolution_path(catalog, &nature.id)?;
    let index = nature.evolution_index?;
    index
        .checked_sub(1)
        .and_then(|previous| catalog.nature(&path.nature_ids[previous]))
}

#[must_use]
pub fn get_next_nature<'a>(catalog: &'a HollowDeckCatalog, nature: &str) -> Option<&'a Nature> {
    let nature = catalog.nature(nature)?;
    let path = get_evolution_path(catalog, &nature.id)?;
    let next = nature.evolution_index? + 1;
    path.nature_ids
        .get(next)
        .and_then(|nature_id| catalog.nature(nature_id))
}

#[must_use]
pub fn get_pair_relationship<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Option<&'a PairRelationship> {
    let nature_id = catalog.nature(nature)?.id.as_str();
    catalog
        .pair_relationships
        .iter()
        .find(|pair| pair.nature_a == nature_id || pair.nature_b == nature_id)
}

#[must_use]
pub fn get_cross_house_counterpart<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Option<&'a Nature> {
    let nature_id = catalog.nature(nature)?.id.as_str();
    let counterpart = catalog.cross_house_counterparts.iter().find_map(|pair| {
        if pair.nature_a == nature_id {
            Some(pair.nature_b.as_str())
        } else if pair.nature_b == nature_id {
            Some(pair.nature_a.as_str())
        } else {
            None
        }
    })?;
    catalog.nature(counterpart)
}

#[must_use]
pub fn get_aura_matchup(source: AuraMaterial, target: AuraMaterial) -> AuraMatchup {
    let source = source.alchemy_slot();
    let target = target.alchemy_slot();
    if source == target {
        return AuraMatchup::Neutral;
    }
    match (source, target) {
        (AlchemySlot::Stonebend, AlchemySlot::Glaushouse)
        | (AlchemySlot::Glaushouse, AlchemySlot::Flynt)
        | (AlchemySlot::Flynt, AlchemySlot::Sandmanor)
        | (AlchemySlot::Sandmanor, AlchemySlot::Stonebend) => AuraMatchup::Favorable,
        (AlchemySlot::Glaushouse, AlchemySlot::Stonebend)
        | (AlchemySlot::Flynt, AlchemySlot::Glaushouse)
        | (AlchemySlot::Sandmanor, AlchemySlot::Flynt)
        | (AlchemySlot::Stonebend, AlchemySlot::Sandmanor) => AuraMatchup::Unfavorable,
        _ => AuraMatchup::Neutral,
    }
}

#[must_use]
pub fn get_growth_path_for_nature<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Option<&'a UnifiedGrowthPath> {
    let form = catalog.current_form(nature)?;
    catalog.growth_path(form.growth_path_id.as_deref()?)
}

#[must_use]
pub fn get_growth_stage_index(path: &UnifiedGrowthPath, milestone: &str) -> Option<usize> {
    let needle = normalized(milestone);
    path.milestones
        .iter()
        .position(|stage| normalized(&stage.id) == needle || normalized(&stage.name) == needle)
}

#[must_use]
pub fn achieved_growth_stages(path: &UnifiedGrowthPath, milestone: &str) -> Option<Vec<String>> {
    let index = get_growth_stage_index(path, milestone)?;
    Some(
        path.milestones[..=index]
            .iter()
            .map(|stage| stage.id.clone())
            .collect(),
    )
}

pub fn resolve_flynt_principal_institutional_stack(
    catalog: &HollowDeckCatalog,
    nature: &str,
) -> Result<FlyntPrincipalInstitutionalStack, DeckValidationError> {
    let nature = catalog.nature(nature).ok_or_else(|| {
        DeckValidationError::new(
            "flynt_institution.nature",
            format!("unknown Nature: {nature}"),
        )
    })?;
    let tradition = catalog.flynt_sect_for_nature(&nature.id).ok_or_else(|| {
        DeckValidationError::new(
            "flynt_institution.sect",
            format!(
                "{} is not a principal Mystery Men sect tradition",
                nature.name
            ),
        )
    })?;
    Ok(FlyntPrincipalInstitutionalStack {
        nature_id: nature.id.clone(),
        citizenship: catalog.flynt_institution.citizen_baseline,
        service_history: catalog.flynt_institution.service_status,
        elite_order: catalog.flynt_institution.elite_status,
        sect: tradition.sect,
    })
}

pub fn resolve_stonebend_order_mastery(
    catalog: &HollowDeckCatalog,
    nature: &str,
    mastery: StonebendMastery,
) -> Result<ResolvedStonebendOrderMastery, DeckValidationError> {
    let nature = catalog.nature(nature).ok_or_else(|| {
        DeckValidationError::new(
            "stonebend_institution.nature",
            format!("unknown Nature: {nature}"),
        )
    })?;
    if nature.house_id != House::Stonebend {
        return Err(DeckValidationError::new(
            "stonebend_institution.nature",
            format!("{} is not a Stonebend Nature", nature.name),
        ));
    }
    let order = catalog.stonebend_order(mastery).ok_or_else(|| {
        DeckValidationError::new(
            "stonebend_institution.order",
            format!("missing {} institutional mastery", mastery.as_str()),
        )
    })?;
    Ok(ResolvedStonebendOrderMastery {
        nature_id: nature.id.clone(),
        citizenship: catalog.stonebend_institution.citizen_baseline,
        training_status: catalog.stonebend_institution.training_entry,
        current_competency: order.current_competency,
        aura_competency: order.aura_competency,
        synthesis_mastery: order.mastery,
        cultural_identity: order.cultural_identity.clone(),
    })
}

pub fn resolve_flynt_art_identity(
    catalog: &HollowDeckCatalog,
    selection: &FlyntArtSelection,
) -> Result<ResolvedFlyntArtIdentity, DeckValidationError> {
    let nature = catalog.nature(&selection.nature_id).ok_or_else(|| {
        DeckValidationError::new(
            "flynt_visual.nature",
            format!("unknown biological Nature: {}", selection.nature_id),
        )
    })?;
    let profile = catalog.flynt_visual_identity(&nature.id).ok_or_else(|| {
        DeckValidationError::new(
            "flynt_visual.profile",
            format!("{} has no Flynt contextual identity profile", nature.name),
        )
    })?;
    let language = catalog
        .flynt_visual_language(selection.organization)
        .ok_or_else(|| {
            DeckValidationError::new(
                "flynt_visual.language",
                format!(
                    "{} has no institutional visual language",
                    selection.organization.as_str()
                ),
            )
        })?;
    let growth_stage = match selection.growth_stage.as_deref() {
        Some(stage) => {
            let path = get_growth_path_for_nature(catalog, &nature.id).ok_or_else(|| {
                DeckValidationError::new(
                    "flynt_visual.growth",
                    format!("{} has no canonical biological growth path", nature.name),
                )
            })?;
            let index = get_growth_stage_index(path, stage).ok_or_else(|| {
                DeckValidationError::new(
                    "flynt_visual.growth",
                    format!("invalid {} growth stage: {stage}", nature.name),
                )
            })?;
            Some(path.milestones[index].id.clone())
        }
        None => None,
    };
    let identity = profile.identity(selection.organization);
    let institution = resolve_flynt_principal_institutional_stack(catalog, &nature.id)?;
    let outfit = selection
        .outfit
        .clone()
        .unwrap_or_else(|| language.wardrobe.join(", "));
    let accessories = if selection.accessories.is_empty() {
        vec![format!(
            "{} motif — {}",
            profile.symbol_anchor, profile.motif_direction
        )]
    } else {
        selection.accessories.clone()
    };
    let environment = selection
        .environment
        .clone()
        .unwrap_or_else(|| language.environment.join(", "));
    let animation_state = selection
        .animation_state
        .clone()
        .unwrap_or_else(|| language.animation_direction.clone());

    Ok(ResolvedFlyntArtIdentity {
        nature_id: nature.id.clone(),
        growth_stage,
        citizenship: institution.citizenship,
        service_history: institution.service_history,
        elite_order: institution.elite_order,
        sect: institution.sect,
        organization: selection.organization,
        social_identity_id: identity.id.clone(),
        social_identity: identity.name.clone(),
        identity_direction: identity.visual_direction.clone(),
        outfit,
        accessories,
        environment,
        animation_state,
    })
}

#[must_use]
pub fn get_aura_modifier(source: AuraMaterial, target: AuraMaterial) -> f32 {
    get_aura_matchup(source, target).modifier()
}

#[must_use]
pub fn get_applicable_aura_modifier(
    source: AuraMaterial,
    target: AuraMaterial,
    channel: EffectChannel,
) -> f32 {
    match channel {
        EffectChannel::Aura | EffectChannel::SynthesisWithAura => get_aura_modifier(source, target),
        EffectChannel::CurrentPhysical => NEUTRAL_AURA_MODIFIER,
    }
}

#[must_use]
pub fn get_compatible_synthesis_recipes<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Vec<&'a SynthesisRecipe> {
    let Some(nature) = catalog.nature(nature) else {
        return Vec::new();
    };
    catalog
        .synthesis_recipes
        .iter()
        .filter(|recipe| {
            recipe.compatible_natures.is_empty()
                || recipe
                    .compatible_natures
                    .iter()
                    .any(|candidate| candidate == &nature.id)
        })
        .collect()
}

#[must_use]
pub fn validate_synthesis_requirements(
    recipe: &SynthesisRecipe,
    nature_id: &str,
    current_form: Option<&CurrentForm>,
    aura_set: Option<&AuraSet>,
    available_reagents: &[String],
) -> SynthesisRequirementReport {
    let compatible = recipe.compatible_natures.is_empty()
        || recipe
            .compatible_natures
            .iter()
            .any(|candidate| candidate == nature_id);
    let current_features = current_form
        .map(|form| {
            form.current_features
                .iter()
                .map(|feature| feature.id.as_str())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let current_techniques = current_form
        .map(|form| {
            form.current_techniques
                .iter()
                .map(|technique| technique.id.as_str())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let aura_moves = aura_set
        .map(|set| {
            set.learned_moves
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let achieved_growth = current_form
        .map(|form| {
            form.achieved_growth_stages
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let reagents = available_reagents
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    SynthesisRequirementReport {
        compatible,
        missing_current_features: missing(&recipe.required_current_features, &current_features),
        missing_current_techniques: missing(
            &recipe.required_current_techniques,
            &current_techniques,
        ),
        missing_growth_stage: recipe
            .required_growth_stage
            .as_ref()
            .and_then(|requirement| {
                (requirement.nature_id != nature_id
                    || !achieved_growth.contains(requirement.milestone_id.as_str()))
                .then(|| requirement.milestone_id.clone())
            }),
        missing_aura_moves: missing(&recipe.required_aura_moves, &aura_moves),
        missing_reagents: missing(&recipe.consumed_reagents, &reagents),
    }
}

fn missing(required: &[String], present: &BTreeSet<&str>) -> Vec<String> {
    required
        .iter()
        .filter(|requirement| !present.contains(requirement.as_str()))
        .cloned()
        .collect()
}

pub fn compute_transformed_stats(
    catalog: &HollowDeckCatalog,
    nature: &str,
    development_basis_points: u16,
) -> Result<StatBlock, DeckValidationError> {
    if development_basis_points > 10_000 {
        return Err(DeckValidationError::new(
            "current_form.development",
            "Current Form development must be between 0 and 10,000 basis points",
        ));
    }
    let nature = catalog.nature(nature).ok_or_else(|| {
        DeckValidationError::new("nature.unknown", format!("unknown Nature: {nature}"))
    })?;
    let origin = get_previous_nature(catalog, &nature.id)
        .map(|previous| previous.mature_stats)
        .unwrap_or(HUEMAN_STATS);
    let progress = development_basis_points as f32 / 10_000.0;
    Ok(StatBlock::new(
        interpolate(origin.strength, nature.mature_stats.strength, progress),
        interpolate(origin.speed, nature.mature_stats.speed, progress),
        interpolate(origin.attack, nature.mature_stats.attack, progress),
        interpolate(origin.defense, nature.mature_stats.defense, progress),
    ))
}

fn interpolate(from: u8, to: u8, progress: f32) -> u8 {
    (f32::from(from) + (f32::from(to) - f32::from(from)) * progress).round() as u8
}

pub fn validate_hollow_deck(catalog: &HollowDeckCatalog) -> Result<(), Vec<DeckValidationError>> {
    let mut errors = Vec::new();
    if catalog.schema_version != HOLLOW_DECK_SCHEMA_VERSION {
        errors.push(DeckValidationError::new(
            "catalog.schema_version",
            format!(
                "catalog schema must be {HOLLOW_DECK_SCHEMA_VERSION}, not {}",
                catalog.schema_version
            ),
        ));
    }
    if catalog.hueman_stats != HUEMAN_STATS || catalog.hueman_stats.computed_total() != 200 {
        errors.push(DeckValidationError::new(
            "hueman.baseline",
            "Hueman must remain 50/50/50/50 with a total of 200",
        ));
    }

    let expected_houses = [
        (
            House::Stonebend,
            vec![Dominion::Presynce],
            CurrentType::Hollow,
            vec![AuraMaterial::Diamond],
            Some("North"),
        ),
        (
            House::Sandmanor,
            vec![Dominion::Prefog, Dominion::Prefig],
            CurrentType::Regular,
            vec![AuraMaterial::Crystal],
            Some("East"),
        ),
        (
            House::Glaushouse,
            vec![Dominion::Precog],
            CurrentType::Hollow,
            vec![AuraMaterial::Jade, AuraMaterial::Emerald],
            Some("South"),
        ),
        (
            House::Flynt,
            vec![Dominion::Resynce],
            CurrentType::Regular,
            vec![AuraMaterial::Opal],
            Some("West"),
        ),
    ];
    for (house, dominions, current, aura, direction) in expected_houses {
        match catalog.houses.iter().find(|entry| entry.id == house) {
            Some(entry)
                if entry.dominions == dominions
                    && entry.current_type == current
                    && entry.aura_materials == aura
                    && entry.direction.as_deref() == direction => {}
            _ => errors.push(DeckValidationError::new(
                "house.matrix",
                format!(
                    "{} has an invalid Dominion/Current/Aura assignment",
                    house.as_str()
                ),
            )),
        }
    }

    if catalog.glaushouse_structure.direction != "South"
        || catalog.glaushouse_structure.umbrella_faculty != Dominion::Precog
        || catalog.glaushouse_structure.seats != GlaushouseSeat::ALL
        || catalog.glaushouse_structure.entry_title != "Nightingale de <Seat>"
        || catalog.glaushouse_structure.branch_paths
            != [
                GlaushouseBranchPath {
                    branch: GlaushouseBranch::Presensor,
                    evolution_path_id: "glaushouse-presensor".into(),
                },
                GlaushouseBranchPath {
                    branch: GlaushouseBranch::Preceptor,
                    evolution_path_id: "glaushouse-preceptor".into(),
                },
            ]
    {
        errors.push(DeckValidationError::new(
            "glaushouse.structure",
            "Glaüshouse must keep South, Precog, five ordered seats, and two equal branch paths",
        ));
    }

    validate_institutional_axis_model(catalog, &mut errors);
    validate_sandmanor_institution(catalog, &mut errors);
    validate_flynt_institution(catalog, &mut errors);
    validate_stonebend_institution(catalog, &mut errors);

    let expected_flynt_identities = [
        (
            "gargoyle",
            ("hunter", "Hunter"),
            ("bro-white", "Bro White"),
            ("johnny-apple", "Johnny Apple"),
            "Apple",
            "Snow White's poisoned apple",
            &["Johnny Appleseed"][..],
        ),
        (
            "werewolf",
            ("chief", "Chief"),
            ("beauty-the-beast", "Beauty the Beast"),
            ("tommy-rose", "Tommy Rose"),
            "Rose",
            "Beauty and the Beast",
            &[][..],
        ),
        (
            "mariner",
            ("mariner", "Mariner"),
            ("cindarellaman", "Cindarellaman"),
            ("jimmy-pumps", "Jimmy Pumps"),
            "Shoe / pump",
            "Cinderella's glass slipper",
            &["Pumpkin carriage", "Midnight", "Midnight Crew"][..],
        ),
    ];
    if catalog.flynt_visual_identities.len() != expected_flynt_identities.len() {
        errors.push(DeckValidationError::new(
            "flynt_visual.matrix",
            "Flynt must expose exactly three principal contextual identity profiles",
        ));
    }
    for (nature_id, mystery, rural, gallows, symbol, mythic, echoes) in expected_flynt_identities {
        let valid = catalog
            .flynt_visual_identities
            .iter()
            .find(|profile| profile.nature_id == nature_id)
            .is_some_and(|profile| {
                catalog
                    .nature(nature_id)
                    .is_some_and(|nature| nature.house_id == House::Flynt)
                    && identity_matches(
                        &profile.mystery_men_identity,
                        FlyntOrganization::MysteryMen,
                        mystery,
                    )
                    && identity_matches(
                        &profile.we_fairy_men_identity,
                        FlyntOrganization::WeFairyMen,
                        rural,
                    )
                    && identity_matches(
                        &profile.gallows_identity,
                        FlyntOrganization::Gallows,
                        gallows,
                    )
                    && profile.symbol_anchor == symbol
                    && profile.mythic_source == mythic
                    && profile
                        .secondary_echoes
                        .iter()
                        .map(String::as_str)
                        .eq(echoes.iter().copied())
                    && !profile.motif_direction.is_empty()
            });
        if !valid {
            errors.push(DeckValidationError::new(
                "flynt_visual.matrix",
                format!("{nature_id} has an invalid contextual visual identity profile"),
            ));
        }
    }
    let language_organizations = catalog
        .flynt_visual_languages
        .iter()
        .map(|language| language.organization)
        .collect::<BTreeSet<_>>();
    if language_organizations != FlyntOrganization::ALL.into_iter().collect()
        || catalog.flynt_visual_languages.len() != FlyntOrganization::ALL.len()
        || catalog.flynt_visual_languages.iter().any(|language| {
            language.visual_character.is_empty()
                || language.social_power.is_empty()
                || language.wardrobe.is_empty()
                || language.environment.is_empty()
                || language.animation_direction.is_empty()
                || language.avoid.is_empty()
        })
    {
        errors.push(DeckValidationError::new(
            "flynt_visual.language",
            "Mystery Men, We Fairy Men, and Gallows require distinct complete visual languages",
        ));
    }

    unique_ids(
        "nature.id",
        catalog.natures.iter().map(|item| item.id.as_str()),
        &mut errors,
    );
    unique_ids(
        "trait.id",
        catalog.primary_traits.iter().map(|item| item.id.as_str()),
        &mut errors,
    );
    unique_ids(
        "path.id",
        catalog.evolution_paths.iter().map(|item| item.id.as_str()),
        &mut errors,
    );

    for nature in &catalog.natures {
        if let Err(error) = validate_stat_block(nature.mature_stats) {
            errors.push(DeckValidationError::new(
                error.code,
                format!("{}: {}", nature.name, error.message),
            ));
        }
        let Some(house) = catalog
            .houses
            .iter()
            .find(|house| house.id == nature.house_id)
        else {
            errors.push(DeckValidationError::new(
                "nature.house",
                format!("{} references an unknown House", nature.name),
            ));
            continue;
        };
        if nature.current_type != house.current_type
            || !house.aura_materials.contains(&nature.aura_material)
            || !house.dominions.contains(&nature.dominion_id)
        {
            errors.push(DeckValidationError::new(
                "nature.house_projection",
                format!("{} does not match its House matrix", nature.name),
            ));
        }
        if catalog.primary_trait(&nature.primary_trait_id).is_none() {
            errors.push(DeckValidationError::new(
                "nature.trait",
                format!("{} references an unknown Primary Trait", nature.name),
            ));
        }
        if nature.house_id == House::Flynt && nature.mature_stats.computed_total() != 320 {
            errors.push(DeckValidationError::new(
                "flynt.budget",
                format!("{} must total 320", nature.name),
            ));
        }
        if nature.house_id == House::Glaushouse
            && get_glaushouse_branch_for_nature(catalog, &nature.id)
                .is_none_or(|branch| branch.aura_material() != nature.aura_material)
        {
            errors.push(DeckValidationError::new(
                "glaushouse.branch_projection",
                format!("{} does not match its Precog branch Aura", nature.name),
            ));
        }
    }

    for path in &catalog.evolution_paths {
        for (index, nature_id) in path.nature_ids.iter().enumerate() {
            match catalog.nature(nature_id) {
                Some(nature)
                    if nature.evolution_path_id.as_deref() == Some(path.id.as_str())
                        && nature.evolution_index == Some(index)
                        && nature.house_id == path.house_id
                        && nature.dominion_id == path.dominion_id => {}
                _ => errors.push(DeckValidationError::new(
                    "path.member",
                    format!("{} has an invalid member at tier {}", path.name, index + 1),
                )),
            }
        }
    }

    for pair in &catalog.pair_relationships {
        let a = catalog.nature(&pair.nature_a);
        let b = catalog.nature(&pair.nature_b);
        match (a, b) {
            (Some(a), Some(b))
                if a.paired_nature_id.as_deref() == Some(b.id.as_str())
                    && b.paired_nature_id.as_deref() == Some(a.id.as_str())
                    && a.shared_pair_principle.as_deref()
                        == Some(pair.shared_principle.as_str())
                    && b.shared_pair_principle.as_deref()
                        == Some(pair.shared_principle.as_str()) => {}
            _ => errors.push(DeckValidationError::new(
                "pair.reciprocal",
                format!("{} pair is not reciprocal", pair.shared_principle),
            )),
        }
    }

    for counterpart in &catalog.cross_house_counterparts {
        match (
            catalog.nature(&counterpart.nature_a),
            catalog.nature(&counterpart.nature_b),
        ) {
            (Some(a), Some(b))
                if a.house_id != b.house_id
                    && a.mature_stats.computed_total() == counterpart.stat_budget
                    && b.mature_stats.computed_total() == counterpart.stat_budget => {}
            _ => errors.push(DeckValidationError::new(
                "counterpart.budget",
                format!("counterpart tier {} is invalid", counterpart.tier),
            )),
        }
    }

    for form in &catalog.current_forms {
        if catalog.nature(&form.nature_id).is_none() {
            errors.push(DeckValidationError::new(
                "current_form.nature",
                format!("Current Form references unknown Nature {}", form.nature_id),
            ));
        }
        if form.development_basis_points > 10_000 {
            errors.push(DeckValidationError::new(
                "current_form.development",
                format!("{} exceeds full development", form.nature_id),
            ));
        }
        match form.growth_path_id.as_deref() {
            Some(path_id) => match catalog.growth_path(path_id) {
                Some(path) if path.nature_id == form.nature_id => {
                    if let Some(stage) = form.current_growth_stage.as_deref() {
                        let expected = achieved_growth_stages(path, stage);
                        if expected.as_deref() != Some(&form.achieved_growth_stages) {
                            errors.push(DeckValidationError::new(
                                "growth.inheritance",
                                format!(
                                    "{} growth does not inherit every prior milestone",
                                    form.nature_id
                                ),
                            ));
                        }
                    }
                }
                _ => errors.push(DeckValidationError::new(
                    "growth.path",
                    format!(
                        "{} references an invalid unified growth path",
                        form.nature_id
                    ),
                )),
            },
            None if form.current_growth_stage.is_some()
                || !form.achieved_growth_stages.is_empty() =>
            {
                errors.push(DeckValidationError::new(
                    "growth.unowned",
                    format!("{} has growth state without a unified path", form.nature_id),
                ));
            }
            None => {}
        }
    }

    unique_ids(
        "growth.id",
        catalog.growth_paths.iter().map(|item| item.id.as_str()),
        &mut errors,
    );
    for path in &catalog.growth_paths {
        if catalog.nature(&path.nature_id).is_none() || path.milestones.is_empty() {
            errors.push(DeckValidationError::new(
                "growth.definition",
                format!("{} has no valid Nature or milestones", path.name),
            ));
        }
    }

    let gargoyle_growth = [
        ("fangs", "Fangs"),
        ("claws", "Claws"),
        ("horns", "Horns"),
        ("wing-buds", "Wing Buds"),
        ("winglets", "Winglets"),
        ("wings", "Wings"),
        ("mature-fortification", "Mature Fortification"),
        ("white-opal-statue-mastery", "White Opal Statue Mastery"),
    ];
    let valid_gargoyle_growth = catalog.growth_path("gargoyle").is_some_and(|path| {
        path.nature_id == "gargoyle"
            && path.milestones.len() == gargoyle_growth.len()
            && path
                .milestones
                .iter()
                .zip(gargoyle_growth)
                .all(|(actual, expected)| actual.id == expected.0 && actual.name == expected.1)
    });
    if !valid_gargoyle_growth {
        errors.push(DeckValidationError::new(
            "gargoyle.growth_order",
            "Gargoyle must use the exact eight-stage unified biological growth order",
        ));
    }

    for (kind, name, requirement) in catalog
        .synthesis_recipes
        .iter()
        .filter_map(|recipe| {
            recipe
                .required_growth_stage
                .as_ref()
                .map(|requirement| ("Synthesis Recipe", recipe.name.as_str(), requirement))
        })
        .chain(catalog.syntheses.iter().filter_map(|synthesis| {
            synthesis
                .required_growth_stage
                .as_ref()
                .map(|requirement| ("Synthesis", synthesis.name.as_str(), requirement))
        }))
    {
        let valid = get_growth_path_for_nature(catalog, &requirement.nature_id)
            .is_some_and(|path| get_growth_stage_index(path, &requirement.milestone_id).is_some());
        if !valid {
            errors.push(DeckValidationError::new(
                "growth.requirement",
                format!("{kind} {name} references an invalid unified growth milestone"),
            ));
        }
    }

    for synthesis in &catalog.syntheses {
        if let Some(source_recipe_id) = &synthesis.source_recipe_id
            && !catalog
                .synthesis_recipes
                .iter()
                .any(|recipe| &recipe.id == source_recipe_id)
        {
            errors.push(DeckValidationError::new(
                "synthesis.recipe",
                format!("{} references unknown Synthesis Recipe", synthesis.name),
            ));
        }
        if synthesis.id == "white-opal-statue" {
            errors.push(DeckValidationError::new(
                "gargoyle.inherent_state",
                "White Opal Statue is inherent Fortification growth, not a Synthesis",
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn unique_ids<'a>(
    code: &str,
    ids: impl IntoIterator<Item = &'a str>,
    errors: &mut Vec<DeckValidationError>,
) {
    let mut known = BTreeSet::new();
    for id in ids {
        if !known.insert(id) {
            errors.push(DeckValidationError::new(
                code,
                format!("duplicate canonical id: {id}"),
            ));
        }
    }
}

fn identity_matches(
    identity: &FlyntSocialIdentity,
    organization: FlyntOrganization,
    expected: (&str, &str),
) -> bool {
    identity.organization == organization
        && identity.id == expected.0
        && identity.name == expected.1
        && !identity.visual_direction.is_empty()
}

fn validate_institutional_axis_model(
    catalog: &HollowDeckCatalog,
    errors: &mut Vec<DeckValidationError>,
) {
    let axes = &catalog.institutional_axes;
    if [
        axes.biology.as_str(),
        axes.biological_growth.as_str(),
        axes.citizenship.as_str(),
        axes.institutional_service.as_str(),
        axes.institutional_mastery.as_str(),
        axes.specialist_discipline.as_str(),
        axes.synthesis_apex.as_str(),
        axes.operating_organization.as_str(),
        axes.social_alias.as_str(),
        axes.visual_presentation.as_str(),
        axes.renderer_invariant.as_str(),
    ]
    .iter()
    .any(|value| value.is_empty())
    {
        errors.push(DeckValidationError::new(
            "institution.axes",
            "biology, growth, citizenship, service, mastery, discipline, apex, organization, alias, and visual presentation must remain distinct",
        ));
    }
}

fn validate_sandmanor_institution(
    catalog: &HollowDeckCatalog,
    errors: &mut Vec<DeckValidationError>,
) {
    let hierarchy = &catalog.sandmanor_institution;
    if hierarchy.direction != "East"
        || hierarchy.current_type != CurrentType::Regular
        || hierarchy.aura_material != AuraMaterial::Crystal
        || hierarchy.physical_philosophy != "The Way"
        || hierarchy.visual_character.is_empty()
        || hierarchy.axis_invariant.is_empty()
        || hierarchy.cultures.len() != SandmanorCulture::ALL.len()
    {
        errors.push(DeckValidationError::new(
            "sandmanor.structure",
            "Sandmanor must remain East, Regular Current, Crystal, The Way, and two complete cultural paths",
        ));
    }

    let expected_paths = [
        (
            SandmanorCulture::Minorian,
            Dominion::Prefog,
            "What is going to be needed?",
            [
                SandmanorSocialIdentity::Hearthkeeper,
                SandmanorSocialIdentity::Householder,
                SandmanorSocialIdentity::Husbandman,
            ],
            [
                SandmanorAuraMastery::Geomancer,
                SandmanorAuraMastery::Labyrinthian,
                SandmanorAuraMastery::Demiurge,
            ],
            "sandmanor-minorian",
            ["gnome", "faun", "minotaur", "hecaton"],
            ["grip", "traction", "leverage", "multiplication"],
            ["maintain", "organize", "steward"],
            ["reads place", "reads system", "makes system"],
        ),
        (
            SandmanorCulture::Minoan,
            Dominion::Prefig,
            "What is this becoming?",
            [
                SandmanorSocialIdentity::BeachBum,
                SandmanorSocialIdentity::PartyBoy,
                SandmanorSocialIdentity::Socialite,
            ],
            [
                SandmanorAuraMastery::Acolyte,
                SandmanorAuraMastery::Navigator,
                SandmanorAuraMastery::Ascendant,
            ],
            "sandmanor-minoan",
            ["elf", "satyr", "centaur", "pegasus"],
            ["balance", "agility", "momentum", "flight"],
            ["move freely", "create connection", "master networks"],
            [
                "recognizes possibility",
                "moves through possibility",
                "surpasses existing possibility",
            ],
        ),
    ];
    let expected_tiers = [
        SandmanorMasteryTier::Baseline,
        SandmanorMasteryTier::Intermediate,
        SandmanorMasteryTier::Apex,
    ];
    for (
        culture,
        faculty,
        question,
        social,
        aura,
        nature_path_id,
        natures,
        traits,
        social_progression,
        aura_progression,
    ) in expected_paths
    {
        let valid = catalog.sandmanor_culture(culture).is_some_and(|path| {
            let evolution = catalog.evolution_path(&path.nature_path_id);
            path.faculty.faculty == faculty
                && path.faculty.core_question == question
                && !path.orientation.is_empty()
                && !path.faculty.function.is_empty()
                && !path.faculty.examples.is_empty()
                && !path.faculty.excluded_interpretation.is_empty()
                && path
                    .social_path
                    .iter()
                    .map(|stage| stage.identity)
                    .eq(social)
                && path
                    .social_path
                    .iter()
                    .map(|stage| stage.tier)
                    .eq(expected_tiers)
                && path
                    .social_path
                    .iter()
                    .all(|stage| !stage.meaning.is_empty() && !stage.visual_direction.is_empty())
                && path.social_progression == social_progression
                && path.aura_path.iter().map(|stage| stage.identity).eq(aura)
                && path
                    .aura_path
                    .iter()
                    .map(|stage| stage.tier)
                    .eq(expected_tiers)
                && path.aura_path.iter().all(|stage| {
                    !stage.meaning.is_empty()
                        && !stage.question.is_empty()
                        && !stage.examples.is_empty()
                        && !stage.visual_direction.is_empty()
                })
                && path.aura_progression == aura_progression
                && path.nature_path_id == nature_path_id
                && evolution.is_some_and(|evolution| evolution.nature_ids == natures)
                && natures
                    .into_iter()
                    .zip(traits)
                    .all(|(nature_id, trait_id)| {
                        catalog.nature(nature_id).is_some_and(|nature| {
                            nature.house_id == House::Sandmanor
                                && nature.primary_trait_id == trait_id
                        })
                    })
        });
        if !valid {
            errors.push(DeckValidationError::new(
                "sandmanor.culture_path",
                format!(
                    "{} must keep its exact social, Aura, and independent Nature paths",
                    culture.as_str()
                ),
            ));
        }
    }

    let expected_social_pairings = [
        (
            SandmanorSocialIdentity::Hearthkeeper,
            SandmanorSocialIdentity::BeachBum,
        ),
        (
            SandmanorSocialIdentity::Householder,
            SandmanorSocialIdentity::PartyBoy,
        ),
        (
            SandmanorSocialIdentity::Husbandman,
            SandmanorSocialIdentity::Socialite,
        ),
    ];
    if hierarchy.social_pairings.len() != expected_social_pairings.len()
        || hierarchy
            .social_pairings
            .iter()
            .zip(expected_social_pairings)
            .any(|(pairing, expected)| {
                (pairing.minorian, pairing.minoan) != expected || pairing.contrast.is_empty()
            })
    {
        errors.push(DeckValidationError::new(
            "sandmanor.social_pairing",
            "Hearthkeeper/Beach Bum, Householder/Party Boy, and Husbandman/Socialite must remain exact conceptual peers",
        ));
    }

    let expected_aura_pairings = [
        (
            SandmanorAuraMastery::Geomancer,
            SandmanorAuraMastery::Acolyte,
        ),
        (
            SandmanorAuraMastery::Labyrinthian,
            SandmanorAuraMastery::Navigator,
        ),
        (
            SandmanorAuraMastery::Demiurge,
            SandmanorAuraMastery::Ascendant,
        ),
    ];
    if hierarchy.aura_pairings.len() != expected_aura_pairings.len()
        || hierarchy
            .aura_pairings
            .iter()
            .zip(expected_aura_pairings)
            .any(|(pairing, expected)| {
                (pairing.minorian, pairing.minoan) != expected || pairing.contrast.is_empty()
            })
    {
        errors.push(DeckValidationError::new(
            "sandmanor.aura_pairing",
            "Geomancer/Acolyte, Labyrinthian/Navigator, and Demiurge/Ascendant must remain exact conceptual peers",
        ));
    }

    let synthesis = &hierarchy.sovereign_synthesis;
    if synthesis.peer_apexes
        != [
            SandmanorAuraMastery::Demiurge,
            SandmanorAuraMastery::Ascendant,
        ]
        || synthesis.synthesis_identity != SandmanorSovereignIdentity::Transcendent
        || synthesis.synthesis_formula != "Demiurge + Ascendant → The Transcendent"
        || synthesis.sovereign_identity != SandmanorSovereignIdentity::Sandman
        || synthesis.sovereign_embodiment_of != SandmanorSovereignIdentity::Transcendent
        || synthesis.sovereign_formula != "The Transcendent → The Sandman"
        || synthesis.meaning.is_empty()
        || synthesis.core_principle.len() != 3
        || synthesis.social_boundary.is_empty()
        || synthesis.nature_boundary.is_empty()
        || synthesis.visual_direction.is_empty()
        || synthesis.avoid.is_empty()
        || catalog.nature("The Transcendent").is_some()
        || catalog.nature("The Sandman").is_some()
    {
        errors.push(DeckValidationError::new(
            "sandmanor.sovereign_synthesis",
            "Demiurge and Ascendant must remain peer Aura apexes whose synthesis is The Transcendent, sovereignly embodied as The Sandman outside both Nature paths",
        ));
    }
}

fn validate_flynt_institution(catalog: &HollowDeckCatalog, errors: &mut Vec<DeckValidationError>) {
    let hierarchy = &catalog.flynt_institution;
    if hierarchy.citizen_baseline != FlyntCitizenship::Warden
        || hierarchy.service_status != FlyntServiceStatus::Corpman
        || hierarchy.service_prerequisite != FlyntCitizenship::Warden
        || hierarchy.capacity_requirement != "Prove Capacity"
        || hierarchy.elite_status != FlyntEliteStatus::MysteryMan
        || hierarchy.elite_prerequisite != FlyntServiceStatus::Corpman
        || hierarchy.sect_prerequisite != FlyntEliteStatus::MysteryMan
        || hierarchy.citizen_visual_direction.is_empty()
        || hierarchy.service_visual_direction.is_empty()
        || hierarchy.elite_visual_direction.is_empty()
    {
        errors.push(DeckValidationError::new(
            "flynt_institution.entry",
            "Flynt formal entry must remain Warden → Corpman → prove Capacity → Mystery Man",
        ));
    }

    let expected_sects = [
        (FlyntSect::Hunter, "gargoyle", "fortification"),
        (FlyntSect::Chief, "werewolf", "siphone"),
        (FlyntSect::Mariner, "mariner", "anchoring"),
    ];
    if hierarchy.sects.len() != expected_sects.len() {
        errors.push(DeckValidationError::new(
            "flynt_institution.sects",
            "Mystery Men must expose exactly three parallel specialist sects",
        ));
    }
    for (sect, nature_id, trait_id) in expected_sects {
        let valid = hierarchy
            .sects
            .iter()
            .find(|tradition| tradition.sect == sect)
            .is_some_and(|tradition| {
                tradition.nature_id == nature_id
                    && tradition.primary_trait_id == trait_id
                    && !tradition.vocation.is_empty()
                    && !tradition.visual_direction.is_empty()
                    && catalog.nature(nature_id).is_some_and(|nature| {
                        nature.house_id == House::Flynt && nature.primary_trait_id == trait_id
                    })
            });
        if !valid {
            errors.push(DeckValidationError::new(
                "flynt_institution.sects",
                format!(
                    "{} must map to {nature_id} / {trait_id} as one parallel Mystery Men sect",
                    sect.as_str()
                ),
            ));
        }
    }

    let convergence = &hierarchy.convergence;
    if convergence.required_sects != FlyntSect::ALL
        || convergence.convergence_identity != "Chimera"
        || convergence.convergence_nature_id != "chimera"
        || convergence.perfected_from_identity != "Chimera"
        || convergence.perfected_identity != "Manticorp"
        || convergence.perfected_nature_id != "manticorp"
        || convergence.convergence_meaning.is_empty()
        || convergence.convergence_visual_direction.is_empty()
        || convergence.perfection_meaning.is_empty()
        || convergence.perfection_visual_direction.is_empty()
        || convergence.apex_association != "The Tross"
        || convergence.legacy_pair_compatibility.is_empty()
        || catalog.nature(&convergence.convergence_nature_id).is_none()
        || catalog.nature(&convergence.perfected_nature_id).is_none()
        || normalized(hierarchy.service_status.as_str())
            == normalized(&convergence.perfected_identity)
    {
        errors.push(DeckValidationError::new(
            "flynt_institution.convergence",
            "Hunter + Chief + Mariner must converge as Chimera, then perfect as Manticorp; Corpman is distinct",
        ));
    }

    let sorcery = &hierarchy.sorcery;
    let expected_sorcery_pairs = [
        (
            FlyntSorceryIdentity::Wizard,
            FlyntSorceryIdentity::Witch,
            FlyntSorceryFunction::EstablishedManufacture,
            &[
                "manufacture established Synthesis Recipes",
                "manufacture established Synthesis products",
            ][..],
        ),
        (
            FlyntSorceryIdentity::Warlock,
            FlyntSorceryIdentity::Enchantress,
            FlyntSorceryFunction::PatternModification,
            &[
                "combine",
                "counterfeit",
                "corrupt",
                "strengthen",
                "weaken",
                "repurpose",
                "reformulate",
            ][..],
        ),
        (
            FlyntSorceryIdentity::Sorcerer,
            FlyntSorceryIdentity::Sorceress,
            FlyntSorceryFunction::PatternAuthorship,
            &[
                "author new Synthesis Recipes",
                "author new Synthesis patterns",
            ][..],
        ),
    ];
    let sorcery_pairs_valid = hierarchy.sorcery.paired_ladder.len() == 3
        && hierarchy
            .sorcery
            .paired_ladder
            .iter()
            .zip(expected_sorcery_pairs)
            .all(|(pair, (male, female, function, capabilities))| {
                pair.male_identity == male
                    && pair.female_identity == female
                    && pair.function == function
                    && pair
                        .capabilities
                        .iter()
                        .map(String::as_str)
                        .eq(capabilities.iter().copied())
            });
    if sorcery.domain != "Flynt synthesis manufacturing and formulation"
        || !sorcery_pairs_valid
        || sorcery.axis_boundary
            != "Sorcery is neither a Sandmanor Aura ladder, a Nature progression, nor a Mystery Men sect or rank."
        || sorcery.sanctioned_practitioner_boundary
            != "Sanctioned sorcery practitioners are not inherently criminal, hostile, villainous, or anti-Tross."
        || sorcery.governance != "The Tross governs the wider Flynt vice and synthesis ecosystem."
        || sorcery.systemic_danger != "Uncontrolled synthesis is the key systemic danger."
        || FlyntSorceryIdentity::ALL
            .iter()
            .any(|identity| catalog.nature(identity.as_str()).is_some())
    {
        errors.push(DeckValidationError::new(
            "flynt_institution.sorcery",
            "Flynt sorcery must remain the paired Wizard/Witch → Warlock/Enchantress → Sorcerer/Sorceress synthesis-manufacturing ladder, separate from Nature and Mystery Men",
        ));
    }

    let expected_ecosystem_roles = [
        (
            FlyntEcosystemRole::Tross,
            "sovereign direction and governance",
        ),
        (
            FlyntEcosystemRole::MysteryMen,
            "enforcement, interdiction, investigation, and regulation",
        ),
        (
            FlyntEcosystemRole::Sorcery,
            "manufacture, formulation, pattern modification, and pattern authorship",
        ),
        (
            FlyntEcosystemRole::Gallows,
            "urban vice, leverage, distribution, and commerce",
        ),
        (
            FlyntEcosystemRole::WeFairyMen,
            "rural public legitimacy, mutual aid, and protection",
        ),
        (
            FlyntEcosystemRole::Marquise,
            "maritime operating and routing extension",
        ),
    ];
    if sorcery.ecosystem_roles.len() != FlyntEcosystemRole::ALL.len()
        || !sorcery
            .ecosystem_roles
            .iter()
            .zip(expected_ecosystem_roles)
            .all(|(profile, (role, function))| profile.role == role && profile.function == function)
    {
        errors.push(DeckValidationError::new(
            "flynt_institution.sorcery_ecosystem",
            "The Tross, Mystery Men, Sorcery, The Gallows, We Fairy Men, and The Marquise must retain distinct Flynt ecosystem functions",
        ));
    }

    let operating_organizations = hierarchy
        .operating_contexts
        .iter()
        .map(|context| context.organization)
        .collect::<BTreeSet<_>>();
    let marquise_valid = hierarchy
        .operating_contexts
        .iter()
        .find(|context| context.organization == FlyntOperatingOrganization::Marquise)
        .is_some_and(|context| context.tradition_affinity == Some(FlyntSect::Mariner));
    if operating_organizations != FlyntOperatingOrganization::ALL.into_iter().collect()
        || hierarchy.operating_contexts.len() != FlyntOperatingOrganization::ALL.len()
        || hierarchy.operating_contexts.iter().any(|context| {
            context.social_basis.is_empty()
                || context.environment.is_empty()
                || context.relationship_to_hierarchy.is_empty()
        })
        || !marquise_valid
    {
        errors.push(DeckValidationError::new(
            "flynt_institution.operating_context",
            "We Fairy Men, The Gallows, and The Marquise are complete operating contexts, not career ranks",
        ));
    }
}

fn validate_stonebend_institution(
    catalog: &HollowDeckCatalog,
    errors: &mut Vec<DeckValidationError>,
) {
    let hierarchy = &catalog.stonebend_institution;
    if hierarchy.citizen_baseline != StonebendCitizenship::Gerald
        || hierarchy.training_entry != StonebendTrainingStatus::Apprentice
        || hierarchy.training_prerequisite != StonebendCitizenship::Gerald
        || hierarchy.competency_prerequisite != StonebendTrainingStatus::Apprentice
        || hierarchy.contains_active_identity("Applicant")
        || hierarchy.citizen_visual_direction.is_empty()
        || hierarchy.apprentice_visual_direction.is_empty()
        || hierarchy.constitutional_compatibility.is_empty()
    {
        errors.push(DeckValidationError::new(
            "stonebend_institution.entry",
            "Stonebend active entry must remain Gerald → Apprentice with Applicant retired",
        ));
    }

    let expected_competencies = [
        (
            StonebendCompetency::Mason,
            InstitutionalChannel::Current,
            None,
        ),
        (
            StonebendCompetency::Mage,
            InstitutionalChannel::Aura,
            Some(AuraMaterial::Diamond),
        ),
        (
            StonebendCompetency::Pugilist,
            InstitutionalChannel::Current,
            None,
        ),
        (
            StonebendCompetency::Foreman,
            InstitutionalChannel::Aura,
            Some(AuraMaterial::Diamond),
        ),
    ];
    if hierarchy.competencies.len() != expected_competencies.len() {
        errors.push(DeckValidationError::new(
            "stonebend_institution.competency",
            "Stonebend must expose exactly Mason, Mage, Pugilist, and Foreman competencies",
        ));
    }
    for (competency, channel, material) in expected_competencies {
        let valid = hierarchy
            .competencies
            .iter()
            .find(|profile| profile.competency == competency)
            .is_some_and(|profile| {
                profile.channel == channel
                    && profile.aura_material == material
                    && !profile.concepts.is_empty()
                    && !profile.visual_direction.is_empty()
                    && !profile.avoid.is_empty()
            });
        if !valid {
            errors.push(DeckValidationError::new(
                "stonebend_institution.competency",
                format!(
                    "{} must remain a complete {} competency",
                    competency.as_str(),
                    channel.as_str()
                ),
            ));
        }
    }

    let expected_orders = [
        (
            StonebendMastery::Freemason,
            "Superstar",
            StonebendCompetency::Mason,
            StonebendCompetency::Mage,
        ),
        (
            StonebendMastery::Proliteriate,
            "Rockstar",
            StonebendCompetency::Pugilist,
            StonebendCompetency::Foreman,
        ),
    ];
    if hierarchy.orders.len() != expected_orders.len() {
        errors.push(DeckValidationError::new(
            "stonebend_institution.order",
            "Stonebend must expose exactly two peer completed Synthesis masteries",
        ));
    }
    for (mastery, cultural_identity, current, aura) in expected_orders {
        let valid = hierarchy
            .orders
            .iter()
            .find(|path| path.mastery == mastery)
            .is_some_and(|path| {
                path.cultural_identity == cultural_identity
                    && path.current_competency == current
                    && path.aura_competency == aura
                    && !path.synthesis_meaning.is_empty()
                    && !path.visual_direction.is_empty()
            });
        if !valid {
            errors.push(DeckValidationError::new(
                "stonebend_institution.order",
                format!("invalid {} paired-competency mastery", mastery.as_str()),
            ));
        }
    }

    let apex = &hierarchy.apex;
    let stonebend_path_is_exact = catalog.evolution_path("stonebend").is_some_and(|path| {
        path.nature_ids
            == [
                "gremlin",
                "goblin",
                "ghoul",
                "spectre",
                "troll",
                "orc",
                "ogre",
                "troglodyte",
            ]
    });
    if apex.peer_inputs != StonebendMastery::ALL
        || apex.required_cultural_identities != ["Superstar", "Rockstar"]
        || apex.identity != "Hypergiant"
        || apex.meaning.is_empty()
        || apex.visual_direction.is_empty()
        || apex.avoid.is_empty()
        || catalog.nature("Hypergiant").is_some()
        || !stonebend_path_is_exact
    {
        errors.push(DeckValidationError::new(
            "stonebend_institution.apex",
            "Freemason/Superstar + Proliteriate/Rockstar must produce institutional Hypergiant without altering the Nature ladder",
        ));
    }

    let philosophy = &hierarchy.philosophy;
    if philosophy.faculty != Dominion::Presynce
        || philosophy.current_type != CurrentType::Hollow
        || philosophy.aura_material != AuraMaterial::Diamond
        || philosophy.philosophy != "The Cut"
        || philosophy.process != "Hollowing"
        || philosophy.presynce_meaning.is_empty()
        || philosophy.presynce_questions.len() != 8
        || philosophy.cut_meaning.is_empty()
        || philosophy.cut_examples.len() != 4
        || philosophy.hollowing_boundary.is_empty()
    {
        errors.push(DeckValidationError::new(
            "stonebend_institution.philosophy",
            "Stonebend must preserve Presynce, Hollow Current, Diamond, The Cut, and Hollowing",
        ));
    }
}

fn normalized(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace([' ', '_', 'ü'], "-")
        .replace("--", "-")
}

fn seed_catalog() -> HollowDeckCatalog {
    crate::hollow_deck_seed::seed_catalog()
}
