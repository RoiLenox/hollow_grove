use serde::{Deserialize, Serialize};

use crate::world_map_geometry::{
    canonical_rotation_contract_fixture, validate_hollow_grove_rotation_contract,
};
use crate::{BeingId, CurrentDepthId, FrameId};

const WORLD_CONTEXT_DOCUMENT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");

const REQUIRED_WORLD_CONTEXT_FRAGMENTS: &[&str] = &[
    "Current = blood",
    "Hollow = pus",
    "Aura = air / pressure / light",
    "Whole\n→ Hollowing\n→ Hollow + Hollowed",
    "Frame = living mech only",
    "Scene = active event",
    "Structure = built or grown form",
    "System = continuing operation",
    "Diamond claims",
    "Crystal measures",
    "Jade clears",
    "Opal shimmers",
    "The Nightingales are Glaüshouse's constitutional nursing and clinical-care",
    "Minorians are Gnomes.",
    "Minoans are Elves.",
    "Pus is Hollow.",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Substance {
    Current,
    Hollow,
    Aura,
}

impl Substance {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "Current",
            Self::Hollow => "Hollow",
            Self::Aura => "Aura",
        }
    }

    pub const fn canonical_identity(self) -> &'static str {
        match self {
            Self::Current => "blood",
            Self::Hollow => "pus",
            Self::Aura => "air / pressure / light",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityCategory {
    Frame,
    Scene,
    Structure,
    System,
}

impl EntityCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Frame => "Frame",
            Self::Scene => "Scene",
            Self::Structure => "Structure",
            Self::System => "System",
        }
    }

    pub const fn canonical_definition(self) -> &'static str {
        match self {
            Self::Frame => "named living mech only",
            Self::Scene => "active situation",
            Self::Structure => "built or grown arrangement",
            Self::System => "continuing operational relationship",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum House {
    Stonebend,
    Sandmanor,
    Glaushouse,
    Flynt,
}

impl House {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stonebend => "Stonebend",
            Self::Sandmanor => "Sandmanor",
            Self::Glaushouse => "Glaushouse",
            Self::Flynt => "Flynt",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseRock {
    Diamond,
    Crystal,
    Jade,
    Opal,
}

impl HouseRock {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diamond => "Diamond",
            Self::Crystal => "Crystal",
            Self::Jade => "Jade",
            Self::Opal => "Opal",
        }
    }

    pub const fn canonical_operation(self) -> &'static str {
        match self {
            Self::Diamond => "claims",
            Self::Crystal => "measures",
            Self::Jade => "clears",
            Self::Opal => "shimmers",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HollowingMode {
    Extraction,
    Containment,
    ManufactureFromCurrent,
    GenericRefinement,
    EmptySpaceCreation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseAnchor {
    Marrow,
    BloodProduction,
    Hollowing,
    Mucus,
    Lymph,
    Diagnosis,
    Allocation,
    Movement,
    Adaptation,
}

impl HouseAnchor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Marrow => "marrow",
            Self::BloodProduction => "blood production",
            Self::Hollowing => "Hollowing",
            Self::Mucus => "mucus",
            Self::Lymph => "lymph",
            Self::Diagnosis => "diagnosis",
            Self::Allocation => "allocation",
            Self::Movement => "movement",
            Self::Adaptation => "adaptation",
        }
    }

    pub const fn canonical_house(self) -> House {
        match self {
            Self::Marrow | Self::BloodProduction | Self::Hollowing => House::Stonebend,
            Self::Mucus | Self::Lymph => House::Glaushouse,
            Self::Diagnosis | Self::Allocation => House::Sandmanor,
            Self::Movement | Self::Adaptation => House::Flynt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NightingaleIdentity {
    WhiteBloodCells,
    GenericNurseSpecies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NightingaleFunction {
    RecognizeThreats,
    DefendFrame,
    ClearDamagedMaterial,
    PrepareTissueForRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorPeople {
    Minorian,
    Minoan,
}

impl SandmanorPeople {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minorian => "Minorian",
            Self::Minoan => "Minoan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lineage {
    Gnome,
    Elf,
}

impl Lineage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gnome => "Gnome",
            Self::Elf => "Elf",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Profession {
    Surgeon,
    Radiologist,
    Nurse,
    Pharmacist,
    Anesthetist,
    TraumaResponder,
    RehabilitationSpecialist,
    Neurologist,
    GeneralDoctor,
    EmergencyPhysician,
}

impl Profession {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surgeon => "Surgeon",
            Self::Radiologist => "Radiologist",
            Self::Nurse => "Nurse",
            Self::Pharmacist => "Pharmacist",
            Self::Anesthetist => "Anesthetist",
            Self::TraumaResponder => "TraumaResponder",
            Self::RehabilitationSpecialist => "RehabilitationSpecialist",
            Self::Neurologist => "Neurologist",
            Self::GeneralDoctor => "GeneralDoctor",
            Self::EmergencyPhysician => "EmergencyPhysician",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfessionAccessRule {
    Open,
    LockedToSpecies,
    LockedToNativeHouse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CivilizationModel {
    Connected,
    IsolatedHouses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CareLevel {
    LocalCareCenter,
    AdvancedGlaushouseFacility,
}

impl CareLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCareCenter => "LocalCareCenter",
            Self::AdvancedGlaushouseFacility => "AdvancedGlaushouseFacility",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CareFunction {
    Rest,
    Stabilize,
    Clean,
    Replenish,
    Return,
    ReferSeriousCases,
    Diagnose,
    Clear,
    Reconstruct,
    Rehabilitate,
    Transform,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstanceIdentityClaim {
    pub substance: Substance,
    pub identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingClaim {
    pub mode: HollowingMode,
    pub extracts_useful_interior: bool,
    pub preserves_outer_form: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityClaim {
    pub name: String,
    pub category: EntityCategory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameGrammarClaim {
    pub subject: String,
    pub category: EntityCategory,
    pub uses_frame_flow_glow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseAnchorClaim {
    pub anchor: HouseAnchor,
    pub primary_house: House,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseRockClaim {
    pub house: House,
    pub rock: HouseRock,
    pub operation: String,
    pub replaces_substance: Option<Substance>,
    pub visible_style: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightingaleClaim {
    pub identity: NightingaleIdentity,
    pub origin: House,
    pub medium: Substance,
    pub functions: Vec<NightingaleFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanorPeopleClaim {
    pub people: SandmanorPeople,
    pub lineage: Lineage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnomeProgressionClaim {
    pub has_evolution_ladder: bool,
    pub evolves_into_gargoyle: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfessionClaim {
    pub species: String,
    pub profession: Profession,
    pub medical_discipline: House,
    pub access_rule: ProfessionAccessRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CareLevelClaim {
    pub care_level: CareLevel,
    pub functions: Vec<CareFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowGroveAlignmentInput {
    pub substance_identity_claims: Vec<SubstanceIdentityClaim>,
    pub hollowing_claims: Vec<HollowingClaim>,
    pub entity_claims: Vec<EntityClaim>,
    pub frame_grammar_claims: Vec<FrameGrammarClaim>,
    pub house_anchor_claims: Vec<HouseAnchorClaim>,
    pub house_rock_claims: Vec<HouseRockClaim>,
    pub nightingale_claims: Vec<NightingaleClaim>,
    pub sandmanor_people_claims: Vec<SandmanorPeopleClaim>,
    pub gnome_progression_claims: Vec<GnomeProgressionClaim>,
    pub profession_claims: Vec<ProfessionClaim>,
    pub civilization_claims: Vec<CivilizationModel>,
    pub care_level_claims: Vec<CareLevelClaim>,
}

impl Default for HollowGroveAlignmentInput {
    fn default() -> Self {
        Self {
            substance_identity_claims: Vec::new(),
            hollowing_claims: Vec::new(),
            entity_claims: Vec::new(),
            frame_grammar_claims: Vec::new(),
            house_anchor_claims: Vec::new(),
            house_rock_claims: Vec::new(),
            nightingale_claims: Vec::new(),
            sandmanor_people_claims: Vec::new(),
            gnome_progression_claims: Vec::new(),
            profession_claims: Vec::new(),
            civilization_claims: Vec::new(),
            care_level_claims: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthIdentityValue {
    HeldForm,
    BloodCirculation,
    FeltDepth,
    Pus,
    Hollow,
    Death,
    EmptySpace,
}

impl DepthIdentityValue {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeldForm => "held bodily form",
            Self::BloodCirculation => "blood and living circulation",
            Self::FeltDepth => "felt inward condition",
            Self::Pus => "pus",
            Self::Hollow => "Hollow",
            Self::Death => "death",
            Self::EmptySpace => "empty space",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentDepthOwnershipClaim {
    pub depth: CurrentDepthId,
    pub house: House,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentDepthIdentityClaim {
    pub depth: CurrentDepthId,
    pub identity: DepthIdentityValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRelationClaim {
    pub derived_from_hollow_current: bool,
    pub derived_from_abyss: bool,
    pub exclusive_owner: Option<House>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSpeedSemanticClaim {
    pub running_speed_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraIlluminationClaim {
    pub treated_as_current_depth: bool,
    pub reveals_hollow_current: bool,
    pub reveals_current: bool,
    pub reveals_abyss: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointSquaredSemanticClaim {
    pub current_capacity_delta: i16,
    pub aura_capacity_delta: i16,
    pub auto_maximizes_development: bool,
    pub auto_grants_frame: Option<FrameId>,
    pub creates_separate_horizon_state: bool,
    pub includes_self: bool,
    pub includes_surroundings: bool,
    pub miss_grants_ascension: bool,
    pub failed_recipe_grants_ascension: bool,
    pub replaces_being: Option<BeingId>,
    pub stabilizes_into_next_point: bool,
    pub introduces_higher_point_vocabulary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HollowGroveProgressionContractInput {
    pub depth_ownership_claims: Vec<CurrentDepthOwnershipClaim>,
    pub depth_identity_claims: Vec<CurrentDepthIdentityClaim>,
    pub current_relation_claims: Vec<CurrentRelationClaim>,
    pub current_speed_claims: Vec<CurrentSpeedSemanticClaim>,
    pub aura_illumination_claims: Vec<AuraIlluminationClaim>,
    pub point_squared_claims: Vec<PointSquaredSemanticClaim>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignmentDiagnosticCode {
    MissingWorldContextFragment,
    SubstanceIdentityMismatch,
    HollowingMismatch,
    InvalidFrameCategory,
    FrameGrammarLeak,
    HouseAnchorMismatch,
    HouseRockMismatch,
    NightingaleMismatch,
    SandmanorPeopleMismatch,
    GnomeProgressionMismatch,
    ProfessionLockMismatch,
    CivilizationMismatch,
    CareLevelMismatch,
    CurrentDepthMismatch,
    CurrentSpeedMismatch,
    AuraIlluminationMismatch,
    PointSquaredAscensionMismatch,
    RotationalMapMismatch,
    RuleOfTwelveMismatch,
    ManagerLanguageMismatch,
    PlayerSpatialMismatch,
    BeingObjectOntologyMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignmentDiagnostic {
    pub code: AlignmentDiagnosticCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingFixture {
    pub label: String,
    pub interior_resources_present: bool,
    pub outer_form_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingResult {
    pub hollow_extracted: bool,
    pub hollowed_remainder_preserved: bool,
}

pub fn build_world_context_output() -> String {
    String::from(WORLD_CONTEXT_DOCUMENT)
}

pub fn build_hollow_grove_alignment_witness() -> String {
    String::from(
        "HOLLOW GROVE ALIGNMENT WITNESS\n\n\
         Root Doctrine:\n\
         Hollow Current holds.\n\
         Current flows.\n\
         Abyss feels.\n\
         Aura reveals.\n\
         Point² expands the self and the world together.\n\n\
         Substances:\n\
         Current = blood\n\
         Hollow = pus\n\
         Aura = air / pressure / light\n\n\
         Current Depths:\n\
         Hollow Current = Stonebend = held bodily form\n\
         Current = derivative between Stonebend and Glaüshouse\n\
         Current Speed = the operating tempo of Current\n\
         Abyss = Glaüshouse = felt inward condition\n\n\
         Aura Dimensions:\n\
         Aura Shine = outward presence\n\
         Aura View = surrounding world and horizon\n\
         Inner Aura = inward identity and clarity\n\n\
         Rotational Map:\n\
         Ranina = the exact center.\n\
         Twelve positions complete one full rotation.\n\
         Stonebend = Position 1.\n\
         Glaüshouse threshold = Position 6.\n\
         Glaüshouse = Position 7.\n\
         Point² opens the next ring around the same center.\n\n\
         Extraction:\n\
         Whole\n\
         → Hollowing\n\
         → Hollow + Hollowed\n\n\
         Ontology:\n\
         Frame = named living mech only\n\
         Scene = active situation\n\
         Structure = built or grown arrangement\n\
         System = continuing operational relationship\n\n\
         House Rocks:\n\
         Diamond claims\n\
         Crystal measures\n\
         Jade clears\n\
         Opal shimmers\n\n\
         Medicine:\n\
         Glaüshouse owns professional medicine.\n\
         The whole Grove practices it.\n\n\
         Example:\n\
         GargoyleSurgeon = Frame\n\
         Operation = Scene\n\
         Hospital = Structure\n\
         MedicalNetwork = System\n\n\
         Point² Consequence:\n\
         Current Capacity +1\n\
         Aura Capacity +1\n\
         Point² stabilizes into the next Point.\n",
    )
}

pub fn build_hollow_grove_alignment_validation_report() -> String {
    let mut diagnostics = validate_current_world_context_alignment();
    diagnostics.extend(validate_hollow_grove_alignment(
        &canonical_root_alignment_fixture(),
    ));
    diagnostics.extend(validate_hollow_grove_alignment(
        &canonical_ontology_fixture(),
    ));
    diagnostics.extend(validate_hollow_grove_alignment(
        &canonical_house_goods_fixture(),
    ));
    diagnostics.extend(validate_hollow_grove_alignment(
        &canonical_medicine_fixture(),
    ));
    diagnostics.extend(validate_hollow_grove_progression_contract(
        &canonical_progression_contract_fixture(),
    ));
    diagnostics.extend(validate_hollow_grove_rotation_contract(
        &canonical_rotation_contract_fixture(),
    ));
    if diagnostics.is_empty() {
        String::from(
            "# Hollow Grove Alignment Validation\n\n\
             - status: pass\n\
             - world context: aligned with canonical root laws\n\
             - canonical fixtures: aligned\n\
             - current-depth contract: pass\n\
             - aura illumination contract: pass\n\
             - Point² semantic contract: pass\n\
             - Ranina center contract: pass\n\
             - twelve-position rotation contract: pass\n\
             - semantic contract: enforceable\n",
        )
    } else {
        let mut output = String::from("# Hollow Grove Alignment Validation\n\n- status: fail\n");
        for diagnostic in diagnostics {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        output
    }
}

pub fn validate_current_world_context_alignment() -> Vec<AlignmentDiagnostic> {
    validate_world_context_document(WORLD_CONTEXT_DOCUMENT)
}

pub fn validate_world_context_document(text: &str) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();
    for fragment in REQUIRED_WORLD_CONTEXT_FRAGMENTS {
        if !text.contains(fragment) {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::MissingWorldContextFragment,
                message: format!(
                    "world context document is missing required fragment `{fragment}`"
                ),
            });
        }
    }
    diagnostics
}

pub fn build_hollow_grove_progression_witness() -> String {
    String::from(
        "HOLLOW GROVE PROGRESSION CONTRACT WITNESS\n\n\
         Hollow Current = Stonebend = life held in form.\n\
         Current = derivative between Stonebend and Glaüshouse.\n\
         Current Speed = the operating tempo of Current.\n\
         Abyss = Glaüshouse = life felt in depth.\n\
         Aura reveals every Current depth.\n\
         Point² raises Current Capacity and Aura Capacity together.\n\
         Point² includes the Hueman and the reachable world.\n\
         Point² stabilizes into the next Point.\n\
         Ranina remains the fixed center across every ring.\n",
    )
}

pub fn validate_hollow_grove_progression_contract(
    input: &HollowGroveProgressionContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    for claim in &input.depth_ownership_claims {
        match claim.depth {
            CurrentDepthId::HollowCurrent if claim.house != House::Stonebend => {
                diagnostics.push(AlignmentDiagnostic {
                    code: AlignmentDiagnosticCode::CurrentDepthMismatch,
                    message: format!(
                        "Hollow Current belongs to Stonebend, got {}",
                        claim.house.as_str()
                    ),
                });
            }
            CurrentDepthId::Abyss if claim.house != House::Glaushouse => {
                diagnostics.push(AlignmentDiagnostic {
                    code: AlignmentDiagnosticCode::CurrentDepthMismatch,
                    message: format!("Abyss belongs to Glaüshouse, got {}", claim.house.as_str()),
                });
            }
            CurrentDepthId::Current => {
                diagnostics.push(AlignmentDiagnostic {
                    code: AlignmentDiagnosticCode::CurrentDepthMismatch,
                    message: format!(
                        "Current is a derivative between Stonebend and Glaüshouse and cannot belong exclusively to {}",
                        claim.house.as_str()
                    ),
                });
            }
            _ => {}
        }
    }

    for claim in &input.depth_identity_claims {
        let valid = match claim.depth {
            CurrentDepthId::HollowCurrent => claim.identity == DepthIdentityValue::HeldForm,
            CurrentDepthId::Current => claim.identity == DepthIdentityValue::BloodCirculation,
            CurrentDepthId::Abyss => claim.identity == DepthIdentityValue::FeltDepth,
        };
        if !valid {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::CurrentDepthMismatch,
                message: format!(
                    "{} must remain {}, got {}",
                    claim.depth.as_str(),
                    match claim.depth {
                        CurrentDepthId::HollowCurrent => DepthIdentityValue::HeldForm.as_str(),
                        CurrentDepthId::Current => DepthIdentityValue::BloodCirculation.as_str(),
                        CurrentDepthId::Abyss => DepthIdentityValue::FeltDepth.as_str(),
                    },
                    claim.identity.as_str()
                ),
            });
        }
    }

    for claim in &input.current_relation_claims {
        if !claim.derived_from_hollow_current || !claim.derived_from_abyss {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::CurrentDepthMismatch,
                message: String::from("Current must be derived between Hollow Current and Abyss."),
            });
        }
        if let Some(owner) = claim.exclusive_owner {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::CurrentDepthMismatch,
                message: format!(
                    "Current cannot belong exclusively to {}; it remains the living derivative between Stonebend and Glaüshouse.",
                    owner.as_str()
                ),
            });
        }
    }

    for claim in &input.current_speed_claims {
        if claim.running_speed_only {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::CurrentSpeedMismatch,
                message: String::from(
                    "Current Speed cannot mean only running speed; it is the operating tempo of Current.",
                ),
            });
        }
    }

    for claim in &input.aura_illumination_claims {
        if claim.treated_as_current_depth {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::AuraIlluminationMismatch,
                message: String::from(
                    "Aura is revelatory and cannot become another Current depth.",
                ),
            });
        }
        if !claim.reveals_hollow_current || !claim.reveals_current || !claim.reveals_abyss {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::AuraIlluminationMismatch,
                message: String::from(
                    "Aura must reveal Hollow Current, Current, and Abyss together.",
                ),
            });
        }
    }

    for claim in &input.point_squared_claims {
        if claim.current_capacity_delta != 1 || claim.aura_capacity_delta != 1 {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from(
                    "Point² must raise Current Capacity and Aura Capacity together by one.",
                ),
            });
        }
        if claim.auto_maximizes_development {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from(
                    "Point² opens new development circles but does not automatically complete them.",
                ),
            });
        }
        if let Some(frame) = claim.auto_grants_frame {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: format!(
                    "Point² cannot automatically grant {:?}; the next Frame must still be discovered or stabilized legally.",
                    frame
                ),
            });
        }
        if claim.creates_separate_horizon_state {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from(
                    "Point² cannot create Horizon² or a separate horizon state; the next horizon is already contained within Point².",
                ),
            });
        }
        if !claim.includes_self || !claim.includes_surroundings {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from("Point² must expand the self and the world together."),
            });
        }
        if claim.miss_grants_ascension {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from("Miss cannot grant Point² ascension."),
            });
        }
        if claim.failed_recipe_grants_ascension {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from("Failed Recipe execution cannot grant Point² ascension."),
            });
        }
        if claim.replaces_being.is_some() {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from("Point² cannot replace BeingId::Hueman."),
            });
        }
        if !claim.stabilizes_into_next_point {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from("Point² must stabilize into the next Point."),
            });
        }
        if claim.introduces_higher_point_vocabulary {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::PointSquaredAscensionMismatch,
                message: String::from(
                    "Point³ or higher-point vocabulary cannot be introduced here.",
                ),
            });
        }
    }

    diagnostics
}

pub fn perform_canonical_hollowing(fixture: &HollowingFixture) -> HollowingResult {
    HollowingResult {
        hollow_extracted: fixture.interior_resources_present,
        hollowed_remainder_preserved: fixture.outer_form_present,
    }
}

pub fn validate_hollow_grove_alignment(
    input: &HollowGroveAlignmentInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    for claim in &input.substance_identity_claims {
        let expected = claim.substance.canonical_identity();
        if claim.identity != expected {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::SubstanceIdentityMismatch,
                message: format!(
                    "{} must remain `{expected}`, got `{}`",
                    claim.substance.as_str(),
                    claim.identity
                ),
            });
        }
    }

    for claim in &input.hollowing_claims {
        if claim.mode != HollowingMode::Extraction
            || !claim.extracts_useful_interior
            || !claim.preserves_outer_form
        {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::HollowingMismatch,
                message: String::from(
                    "Hollowing must extract the useful interior while preserving the outer form",
                ),
            });
        }
    }

    for claim in &input.entity_claims {
        if claim.category == EntityCategory::Frame && !is_named_living_mech_name(&claim.name) {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::InvalidFrameCategory,
                message: format!(
                    "`{}` cannot classify as Frame; Frame is reserved for named living-mech forms",
                    claim.name
                ),
            });
        }
    }

    for claim in &input.frame_grammar_claims {
        if claim.uses_frame_flow_glow && claim.category != EntityCategory::Frame {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::FrameGrammarLeak,
                message: format!(
                    "`{}` cannot use formal Frame / Flow / Glow grammar because it is a {}",
                    claim.subject,
                    claim.category.as_str()
                ),
            });
        }
    }

    for claim in &input.house_anchor_claims {
        let expected = claim.anchor.canonical_house();
        if claim.primary_house != expected {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::HouseAnchorMismatch,
                message: format!(
                    "{} must remain anchored in {}, got {}",
                    claim.anchor.as_str(),
                    expected.as_str(),
                    claim.primary_house.as_str()
                ),
            });
        }
    }

    for claim in &input.house_rock_claims {
        let expected_rock = canonical_rock_for_house(claim.house);
        if claim.rock != expected_rock {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::HouseRockMismatch,
                message: format!(
                    "{} must map to {}, got {}",
                    claim.house.as_str(),
                    expected_rock.as_str(),
                    claim.rock.as_str()
                ),
            });
        }
        if claim.operation != claim.rock.canonical_operation() {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::HouseRockMismatch,
                message: format!(
                    "{} must {} rather than `{}`",
                    claim.rock.as_str(),
                    claim.rock.canonical_operation(),
                    claim.operation
                ),
            });
        }
        if let Some(substance) = claim.replaces_substance {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::HouseRockMismatch,
                message: format!(
                    "{} cannot replace {}; House rocks inform design but do not replace the primary substances",
                    claim.rock.as_str(),
                    substance.as_str()
                ),
            });
        }
        if claim.house == House::Flynt && claim.rock == HouseRock::Opal {
            let style = claim.visible_style.as_deref().unwrap_or("");
            if !style.contains("transform")
                && !style.contains("modular")
                && !style.contains("motion")
            {
                diagnostics.push(AlignmentDiagnostic {
                    code: AlignmentDiagnosticCode::HouseRockMismatch,
                    message: String::from(
                        "Flyntian Opal goods must remain transforming, modular, or motion-oriented",
                    ),
                });
            }
        }
    }

    for claim in &input.nightingale_claims {
        if claim.identity != NightingaleIdentity::WhiteBloodCells
            || claim.origin != House::Glaushouse
            || claim.medium != Substance::Current
            || !claim
                .functions
                .contains(&NightingaleFunction::RecognizeThreats)
            || !claim.functions.contains(&NightingaleFunction::DefendFrame)
            || !claim
                .functions
                .contains(&NightingaleFunction::ClearDamagedMaterial)
            || !claim
                .functions
                .contains(&NightingaleFunction::PrepareTissueForRecovery)
        {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::NightingaleMismatch,
                message: String::from(
                    "The Nightingales must remain Glaüshouse's constitutional clinical institution, symbolically aligned with white blood cells carried through Current; this does not make generic nurses a species",
                ),
            });
        }
    }

    for claim in &input.sandmanor_people_claims {
        let expected = match claim.people {
            SandmanorPeople::Minorian => Lineage::Gnome,
            SandmanorPeople::Minoan => Lineage::Elf,
        };
        if claim.lineage != expected {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::SandmanorPeopleMismatch,
                message: format!(
                    "{} must remain {}, got {}",
                    claim.people.as_str(),
                    expected.as_str(),
                    claim.lineage.as_str()
                ),
            });
        }
    }

    for claim in &input.gnome_progression_claims {
        if claim.evolves_into_gargoyle {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::GnomeProgressionMismatch,
                message: String::from(
                    "Gnomes may follow the Sandmanor Minotaur and Hecaton lineage but must not transform into Gargoyles",
                ),
            });
        }
    }

    for claim in &input.profession_claims {
        if claim.access_rule != ProfessionAccessRule::Open {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::ProfessionLockMismatch,
                message: format!(
                    "{} {} cannot be species- or native-house-locked",
                    claim.species,
                    claim.profession.as_str()
                ),
            });
        }
        if claim.medical_discipline != House::Glaushouse {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::ProfessionLockMismatch,
                message: format!(
                    "{} {} must remain under Glaushouse medical discipline",
                    claim.species,
                    claim.profession.as_str()
                ),
            });
        }
    }

    for claim in &input.civilization_claims {
        if *claim != CivilizationModel::Connected {
            diagnostics.push(AlignmentDiagnostic {
                code: AlignmentDiagnosticCode::CivilizationMismatch,
                message: String::from(
                    "The four Houses must remain one connected civilization rather than isolated nations",
                ),
            });
        }
    }

    for claim in &input.care_level_claims {
        let required = required_care_functions(claim.care_level);
        for function in required {
            if !claim.functions.contains(function) {
                diagnostics.push(AlignmentDiagnostic {
                    code: AlignmentDiagnosticCode::CareLevelMismatch,
                    message: format!(
                        "{} is missing required function {:?}",
                        claim.care_level.as_str(),
                        function
                    ),
                });
            }
        }
    }

    diagnostics
}

pub fn canonical_hollowing_fixture() -> HollowingFixture {
    HollowingFixture {
        label: String::from("WholeCreature"),
        interior_resources_present: true,
        outer_form_present: true,
    }
}

pub fn canonical_material_fixture() -> HollowGroveAlignmentInput {
    HollowGroveAlignmentInput {
        substance_identity_claims: vec![
            SubstanceIdentityClaim {
                substance: Substance::Current,
                identity: String::from("blood"),
            },
            SubstanceIdentityClaim {
                substance: Substance::Hollow,
                identity: String::from("pus"),
            },
            SubstanceIdentityClaim {
                substance: Substance::Aura,
                identity: String::from("air / pressure / light"),
            },
        ],
        ..HollowGroveAlignmentInput::default()
    }
}

pub fn canonical_ontology_fixture() -> HollowGroveAlignmentInput {
    HollowGroveAlignmentInput {
        entity_claims: vec![
            EntityClaim {
                name: String::from("GargoyleSurgeon"),
                category: EntityCategory::Frame,
            },
            EntityClaim {
                name: String::from("EmergencyOperation"),
                category: EntityCategory::Scene,
            },
            EntityClaim {
                name: String::from("GlaushouseHospital"),
                category: EntityCategory::Structure,
            },
            EntityClaim {
                name: String::from("MedicalCareNetwork"),
                category: EntityCategory::System,
            },
        ],
        frame_grammar_claims: vec![FrameGrammarClaim {
            subject: String::from("GargoyleSurgeon"),
            category: EntityCategory::Frame,
            uses_frame_flow_glow: true,
        }],
        ..HollowGroveAlignmentInput::default()
    }
}

pub fn canonical_house_goods_fixture() -> HollowGroveAlignmentInput {
    HollowGroveAlignmentInput {
        house_rock_claims: vec![HouseRockClaim {
            house: House::Flynt,
            rock: HouseRock::Opal,
            operation: String::from("shimmers"),
            replaces_substance: None,
            visible_style: Some(String::from("transforming modular motion-oriented")),
        }],
        ..HollowGroveAlignmentInput::default()
    }
}

pub fn canonical_medicine_fixture() -> HollowGroveAlignmentInput {
    HollowGroveAlignmentInput {
        profession_claims: vec![
            ProfessionClaim {
                species: String::from("Elf"),
                profession: Profession::Radiologist,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
            ProfessionClaim {
                species: String::from("Gargoyle"),
                profession: Profession::Surgeon,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
            ProfessionClaim {
                species: String::from("Werewolf"),
                profession: Profession::Nurse,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
            ProfessionClaim {
                species: String::from("Gnome"),
                profession: Profession::EmergencyPhysician,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
        ],
        civilization_claims: vec![CivilizationModel::Connected],
        care_level_claims: vec![
            CareLevelClaim {
                care_level: CareLevel::LocalCareCenter,
                functions: vec![
                    CareFunction::Rest,
                    CareFunction::Stabilize,
                    CareFunction::Clean,
                    CareFunction::Replenish,
                    CareFunction::Return,
                    CareFunction::ReferSeriousCases,
                ],
            },
            CareLevelClaim {
                care_level: CareLevel::AdvancedGlaushouseFacility,
                functions: vec![
                    CareFunction::Diagnose,
                    CareFunction::Clear,
                    CareFunction::Reconstruct,
                    CareFunction::Rehabilitate,
                    CareFunction::Transform,
                ],
            },
        ],
        ..HollowGroveAlignmentInput::default()
    }
}

pub fn canonical_root_alignment_fixture() -> HollowGroveAlignmentInput {
    let mut input = canonical_material_fixture();
    input.hollowing_claims.push(HollowingClaim {
        mode: HollowingMode::Extraction,
        extracts_useful_interior: true,
        preserves_outer_form: true,
    });
    input.house_anchor_claims.extend([
        HouseAnchorClaim {
            anchor: HouseAnchor::Marrow,
            primary_house: House::Stonebend,
        },
        HouseAnchorClaim {
            anchor: HouseAnchor::Mucus,
            primary_house: House::Glaushouse,
        },
        HouseAnchorClaim {
            anchor: HouseAnchor::Lymph,
            primary_house: House::Glaushouse,
        },
        HouseAnchorClaim {
            anchor: HouseAnchor::Diagnosis,
            primary_house: House::Sandmanor,
        },
        HouseAnchorClaim {
            anchor: HouseAnchor::Movement,
            primary_house: House::Flynt,
        },
    ]);
    input.nightingale_claims.push(NightingaleClaim {
        identity: NightingaleIdentity::WhiteBloodCells,
        origin: House::Glaushouse,
        medium: Substance::Current,
        functions: vec![
            NightingaleFunction::RecognizeThreats,
            NightingaleFunction::DefendFrame,
            NightingaleFunction::ClearDamagedMaterial,
            NightingaleFunction::PrepareTissueForRecovery,
        ],
    });
    input.sandmanor_people_claims.extend([
        SandmanorPeopleClaim {
            people: SandmanorPeople::Minorian,
            lineage: Lineage::Gnome,
        },
        SandmanorPeopleClaim {
            people: SandmanorPeople::Minoan,
            lineage: Lineage::Elf,
        },
    ]);
    input.gnome_progression_claims.push(GnomeProgressionClaim {
        has_evolution_ladder: false,
        evolves_into_gargoyle: false,
    });
    input
}

pub fn canonical_progression_contract_fixture() -> HollowGroveProgressionContractInput {
    HollowGroveProgressionContractInput {
        depth_ownership_claims: vec![
            CurrentDepthOwnershipClaim {
                depth: CurrentDepthId::HollowCurrent,
                house: House::Stonebend,
            },
            CurrentDepthOwnershipClaim {
                depth: CurrentDepthId::Abyss,
                house: House::Glaushouse,
            },
        ],
        depth_identity_claims: vec![
            CurrentDepthIdentityClaim {
                depth: CurrentDepthId::HollowCurrent,
                identity: DepthIdentityValue::HeldForm,
            },
            CurrentDepthIdentityClaim {
                depth: CurrentDepthId::Current,
                identity: DepthIdentityValue::BloodCirculation,
            },
            CurrentDepthIdentityClaim {
                depth: CurrentDepthId::Abyss,
                identity: DepthIdentityValue::FeltDepth,
            },
        ],
        current_relation_claims: vec![CurrentRelationClaim {
            derived_from_hollow_current: true,
            derived_from_abyss: true,
            exclusive_owner: None,
        }],
        current_speed_claims: vec![CurrentSpeedSemanticClaim {
            running_speed_only: false,
        }],
        aura_illumination_claims: vec![AuraIlluminationClaim {
            treated_as_current_depth: false,
            reveals_hollow_current: true,
            reveals_current: true,
            reveals_abyss: true,
        }],
        point_squared_claims: vec![PointSquaredSemanticClaim {
            current_capacity_delta: 1,
            aura_capacity_delta: 1,
            auto_maximizes_development: false,
            auto_grants_frame: None,
            creates_separate_horizon_state: false,
            includes_self: true,
            includes_surroundings: true,
            miss_grants_ascension: false,
            failed_recipe_grants_ascension: false,
            replaces_being: None,
            stabilizes_into_next_point: true,
            introduces_higher_point_vocabulary: false,
        }],
    }
}

fn canonical_rock_for_house(house: House) -> HouseRock {
    match house {
        House::Stonebend => HouseRock::Diamond,
        House::Sandmanor => HouseRock::Crystal,
        House::Glaushouse => HouseRock::Jade,
        House::Flynt => HouseRock::Opal,
    }
}

fn required_care_functions(level: CareLevel) -> &'static [CareFunction] {
    match level {
        CareLevel::LocalCareCenter => &[
            CareFunction::Rest,
            CareFunction::Stabilize,
            CareFunction::Clean,
            CareFunction::Replenish,
            CareFunction::Return,
            CareFunction::ReferSeriousCases,
        ],
        CareLevel::AdvancedGlaushouseFacility => &[
            CareFunction::Diagnose,
            CareFunction::Clear,
            CareFunction::Reconstruct,
            CareFunction::Rehabilitate,
            CareFunction::Transform,
        ],
    }
}

fn is_named_living_mech_name(name: &str) -> bool {
    const ILLEGAL_FRAME_TOKENS: &[&str] = &[
        "City",
        "Hospital",
        "Road",
        "Government",
        "Battle",
        "Marketplace",
        "MedicalNetwork",
        "AuraBasin",
    ];
    const FRAME_IDS: [FrameId; 24] = [
        FrameId::Hueman,
        FrameId::Gremlin,
        FrameId::Goblin,
        FrameId::Ghoul,
        FrameId::Troll,
        FrameId::Ork,
        FrameId::Ogre,
        FrameId::Troglodyte,
        FrameId::Pixy,
        FrameId::Sprite,
        FrameId::Faerie,
        FrameId::Nymph,
        FrameId::Siren,
        FrameId::Muse,
        FrameId::Werewolf,
        FrameId::Gargoyle,
        FrameId::Merman,
        FrameId::Chimera,
        FrameId::Gnome,
        FrameId::Minotaur,
        FrameId::Hecaton,
        FrameId::Elf,
        FrameId::Centaur,
        FrameId::Pegasus,
    ];

    if ILLEGAL_FRAME_TOKENS
        .iter()
        .any(|token| name.contains(token))
    {
        return false;
    }

    FRAME_IDS.iter().any(|frame| {
        let frame_name = format!("{frame:?}");
        name.eq_ignore_ascii_case(&frame_name)
            || name
                .strip_prefix(&frame_name)
                .is_some_and(|suffix| !suffix.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::{
        AlignmentDiagnosticCode, AuraIlluminationClaim, CurrentDepthIdentityClaim,
        CurrentDepthOwnershipClaim, CurrentRelationClaim, CurrentSpeedSemanticClaim,
        DepthIdentityValue, HollowGroveProgressionContractInput, House, PointSquaredSemanticClaim,
        canonical_progression_contract_fixture, validate_hollow_grove_progression_contract,
    };
    use crate::{BeingId, CurrentDepthId, FrameId};

    #[test]
    fn canonical_current_depth_ownership_passes() {
        let diagnostics =
            validate_hollow_grove_progression_contract(&canonical_progression_contract_fixture());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn hollow_current_pus_and_hollow_claims_fail() {
        for identity in [DepthIdentityValue::Pus, DepthIdentityValue::Hollow] {
            let diagnostics =
                validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                    depth_identity_claims: vec![CurrentDepthIdentityClaim {
                        depth: CurrentDepthId::HollowCurrent,
                        identity,
                    }],
                    ..HollowGroveProgressionContractInput::default()
                });
            assert!(diagnostics.iter().any(|diagnostic| {
                diagnostic.code == AlignmentDiagnosticCode::CurrentDepthMismatch
                    && diagnostic.message.contains("Hollow Current")
            }));
        }
    }

    #[test]
    fn abyss_death_empty_space_and_wrong_house_fail() {
        let depth_identity_failures = [DepthIdentityValue::Death, DepthIdentityValue::EmptySpace];
        for identity in depth_identity_failures {
            let diagnostics =
                validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                    depth_identity_claims: vec![CurrentDepthIdentityClaim {
                        depth: CurrentDepthId::Abyss,
                        identity,
                    }],
                    ..HollowGroveProgressionContractInput::default()
                });
            assert!(diagnostics.iter().any(|diagnostic| {
                diagnostic.code == AlignmentDiagnosticCode::CurrentDepthMismatch
                    && diagnostic.message.contains("Abyss")
            }));
        }

        let diagnostics =
            validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                depth_ownership_claims: vec![CurrentDepthOwnershipClaim {
                    depth: CurrentDepthId::Abyss,
                    house: House::Stonebend,
                }],
                ..HollowGroveProgressionContractInput::default()
            });
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == AlignmentDiagnosticCode::CurrentDepthMismatch
                && diagnostic.message.contains("Abyss belongs to Glaüshouse")
        }));
    }

    #[test]
    fn hollow_current_wrong_house_and_current_wrong_owner_fail() {
        let hollow_current =
            validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                depth_ownership_claims: vec![CurrentDepthOwnershipClaim {
                    depth: CurrentDepthId::HollowCurrent,
                    house: House::Glaushouse,
                }],
                ..HollowGroveProgressionContractInput::default()
            });
        assert!(hollow_current.iter().any(|diagnostic| {
            diagnostic.code == AlignmentDiagnosticCode::CurrentDepthMismatch
                && diagnostic
                    .message
                    .contains("Hollow Current belongs to Stonebend")
        }));

        for house in [House::Stonebend, House::Glaushouse, House::Flynt] {
            let diagnostics =
                validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                    current_relation_claims: vec![CurrentRelationClaim {
                        derived_from_hollow_current: true,
                        derived_from_abyss: true,
                        exclusive_owner: Some(house),
                    }],
                    ..HollowGroveProgressionContractInput::default()
                });
            assert!(diagnostics.iter().any(|diagnostic| {
                diagnostic.code == AlignmentDiagnosticCode::CurrentDepthMismatch
                    && diagnostic.message.contains("cannot belong exclusively")
            }));
        }
    }

    #[test]
    fn current_speed_and_aura_depth_contradictions_fail() {
        let speed =
            validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                current_speed_claims: vec![CurrentSpeedSemanticClaim {
                    running_speed_only: true,
                }],
                ..HollowGroveProgressionContractInput::default()
            });
        assert!(speed.iter().any(|diagnostic| {
            diagnostic.code == AlignmentDiagnosticCode::CurrentSpeedMismatch
        }));

        let aura =
            validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                aura_illumination_claims: vec![AuraIlluminationClaim {
                    treated_as_current_depth: true,
                    reveals_hollow_current: true,
                    reveals_current: true,
                    reveals_abyss: true,
                }],
                ..HollowGroveProgressionContractInput::default()
            });
        assert!(aura.iter().any(|diagnostic| {
            diagnostic.code == AlignmentDiagnosticCode::AuraIlluminationMismatch
        }));
    }

    #[test]
    fn illegal_point_squared_meanings_fail() {
        let contradictions = vec![
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 0,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 0,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: true,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: Some(FrameId::Troglodyte),
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: true,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: true,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: true,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: Some(BeingId::Hueman),
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: false,
            },
            PointSquaredSemanticClaim {
                current_capacity_delta: 1,
                aura_capacity_delta: 1,
                auto_maximizes_development: false,
                auto_grants_frame: None,
                creates_separate_horizon_state: false,
                includes_self: true,
                includes_surroundings: true,
                miss_grants_ascension: false,
                failed_recipe_grants_ascension: false,
                replaces_being: None,
                stabilizes_into_next_point: true,
                introduces_higher_point_vocabulary: true,
            },
        ];

        for contradiction in contradictions {
            let diagnostics =
                validate_hollow_grove_progression_contract(&HollowGroveProgressionContractInput {
                    point_squared_claims: vec![contradiction],
                    ..HollowGroveProgressionContractInput::default()
                });
            assert!(diagnostics.iter().any(|diagnostic| {
                diagnostic.code == AlignmentDiagnosticCode::PointSquaredAscensionMismatch
            }));
        }
    }
}
