use std::io;

use crate::decision_engine::{DecisionCandidateId, resolve_candidate_recipe};
use crate::frame_state::{BeingId, CurrentPrism, FlowId, FrameId, FrameState, GlowId};
use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode};
use crate::manager_domain::{Manager, ManagerDomain};
use crate::point::Point;
use crate::point_progression::PointProgressionState;
use crate::synthesis_execution::{SynthesisExecution, execute_synthesis_recipe};
use crate::synthesis_recipe::SynthesisRecipe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectId {
    MechanicalLatch,
    FracturedCliff,
    Weapon,
    DamagedWreckage,
    BrokenCrossingSupport,
    NearBridgeSupport,
    FailingFarAnchor,
    ShieldFormationAnchor,
    SiegeEngine,
    ReverseFacingHiddenLatch,
    FocusedIndustrialTool,
    CivicRupture,
    MonumentalFoundation,
    DoorMechanism,
    ControlSystem,
    StoneObject,
    DamagedMechanism,
    StoneArchitecture,
    OpenWound,
    Tissue,
    HiddenInfection,
    SymptomPattern,
    ClinicalFinding,
    ConcealedMemoryRelation,
    ImmediateAttacker,
    HostilePursuer,
    HostilePsychicTether,
    TheatricalAudience,
    PoliticallyVulnerableCrowd,
    SevereDiagnosis,
    MultiplePossibleFutures,
    SharedSceneInterpretation,
    TargetPopulation,
    MeaningLink,
    FrightenedCrowd,
    CuttingTool,
    FormationAnchor,
    Monument,
    HiddenEmotionalRupture,
}

impl ObjectId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MechanicalLatch => "Mechanical Latch",
            Self::FracturedCliff => "Fractured Cliff",
            Self::Weapon => "Weapon",
            Self::DamagedWreckage => "Damaged Wreckage",
            Self::BrokenCrossingSupport => "Broken Crossing Support",
            Self::NearBridgeSupport => "Near Bridge Support",
            Self::FailingFarAnchor => "Failing Far Anchor",
            Self::ShieldFormationAnchor => "Shield Formation Anchor",
            Self::SiegeEngine => "Siege Engine",
            Self::ReverseFacingHiddenLatch => "Reverse-facing Hidden Latch",
            Self::FocusedIndustrialTool => "Focused Industrial Tool",
            Self::CivicRupture => "Civic Rupture",
            Self::MonumentalFoundation => "Monumental Foundation",
            Self::DoorMechanism => "Door Mechanism",
            Self::ControlSystem => "Control System",
            Self::StoneObject => "Stone Object",
            Self::DamagedMechanism => "Damaged Mechanism",
            Self::StoneArchitecture => "Stone Architecture",
            Self::OpenWound => "Open Wound",
            Self::Tissue => "Tissue",
            Self::HiddenInfection => "Hidden Infection",
            Self::SymptomPattern => "Symptom Pattern",
            Self::ClinicalFinding => "Clinical Finding",
            Self::ConcealedMemoryRelation => "Concealed Memory Relation",
            Self::ImmediateAttacker => "Immediate Attacker",
            Self::HostilePursuer => "Hostile Pursuer",
            Self::HostilePsychicTether => "Hostile Psychic Tether",
            Self::TheatricalAudience => "Theatrical Audience",
            Self::PoliticallyVulnerableCrowd => "Politically Vulnerable Crowd",
            Self::SevereDiagnosis => "Severe Diagnosis",
            Self::MultiplePossibleFutures => "Multiple Possible Futures",
            Self::SharedSceneInterpretation => "Shared Scene Interpretation",
            Self::TargetPopulation => "Target Population",
            Self::MeaningLink => "Hidden Meaning",
            Self::FrightenedCrowd => "Frightened Crowd",
            Self::CuttingTool => "Cutting Tool",
            Self::FormationAnchor => "Formation Anchor",
            Self::Monument => "Monument",
            Self::HiddenEmotionalRupture => "Hidden Emotional Rupture",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Mechanism,
    Terrain,
    Structure,
    MaterialConstruct,
    System,
    Body,
    Tool,
    Collective,
    SymbolicRelation,
}

impl ObjectKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mechanism => "Mechanism",
            Self::Terrain => "Terrain",
            Self::Structure => "Structure",
            Self::MaterialConstruct => "MaterialConstruct",
            Self::System => "System",
            Self::Body => "Body",
            Self::Tool => "Tool",
            Self::Collective => "Collective",
            Self::SymbolicRelation => "SymbolicRelation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectMaterial {
    Metal,
    Stone,
    Flesh,
    Social,
    Mixed,
}

impl ObjectMaterial {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Metal => "Metal",
            Self::Stone => "Stone",
            Self::Flesh => "Flesh",
            Self::Social => "Social",
            Self::Mixed => "Mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectCondition {
    Stable,
    Damaged,
    Fractured,
    Failing,
    Refined,
    Restored,
    Open,
    Hidden,
    Frightened,
    Unsettled,
    Coercive,
    Diagnosed,
    Concealed,
    Threatening,
    Vulnerable,
}

impl ObjectCondition {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "Stable",
            Self::Damaged => "Damaged",
            Self::Fractured => "Fractured",
            Self::Failing => "Failing",
            Self::Refined => "Refined",
            Self::Restored => "Restored",
            Self::Open => "Open",
            Self::Hidden => "Hidden",
            Self::Frightened => "Frightened",
            Self::Unsettled => "Unsettled",
            Self::Coercive => "Coercive",
            Self::Diagnosed => "Diagnosed",
            Self::Concealed => "Concealed",
            Self::Threatening => "Threatening",
            Self::Vulnerable => "Vulnerable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFunction {
    Latch,
    AccessControl,
    ClimbSurface,
    GuardSupport,
    Repairable,
    WeaponLine,
    LoadPath,
    StructuralSupport,
    Counterweight,
    HiddenLatch,
    OpenBoundary,
    HiddenCondition,
    DirectionalTool,
    GroupAnchor,
    TitleSurface,
    RelationalFault,
    CollectivePresence,
    DiagnosticPattern,
    PresentationSurface,
    PsychicTether,
    ThreatVector,
    AudienceAttention,
    ChoiceField,
    SceneNarrative,
    MeaningField,
}

impl ObjectFunction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Latch => "Latch",
            Self::AccessControl => "AccessControl",
            Self::ClimbSurface => "ClimbSurface",
            Self::GuardSupport => "GuardSupport",
            Self::Repairable => "Repairable",
            Self::WeaponLine => "WeaponLine",
            Self::LoadPath => "LoadPath",
            Self::StructuralSupport => "StructuralSupport",
            Self::Counterweight => "Counterweight",
            Self::HiddenLatch => "HiddenLatch",
            Self::OpenBoundary => "OpenBoundary",
            Self::HiddenCondition => "HiddenCondition",
            Self::DirectionalTool => "DirectionalTool",
            Self::GroupAnchor => "GroupAnchor",
            Self::TitleSurface => "TitleSurface",
            Self::RelationalFault => "RelationalFault",
            Self::CollectivePresence => "CollectivePresence",
            Self::DiagnosticPattern => "DiagnosticPattern",
            Self::PresentationSurface => "PresentationSurface",
            Self::PsychicTether => "PsychicTether",
            Self::ThreatVector => "ThreatVector",
            Self::AudienceAttention => "AudienceAttention",
            Self::ChoiceField => "ChoiceField",
            Self::SceneNarrative => "SceneNarrative",
            Self::MeaningField => "MeaningField",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectConnection {
    target: ObjectId,
    relation: &'static str,
}

impl ObjectConnection {
    #[must_use]
    pub const fn new(target: ObjectId, relation: &'static str) -> Self {
        Self { target, relation }
    }

    #[must_use]
    pub const fn target(self) -> ObjectId {
        self.target
    }

    #[must_use]
    pub const fn relation(self) -> &'static str {
        self.relation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectState {
    identity: ObjectId,
    family: ObjectFamily,
    scale: ObjectScale,
    kind: ObjectKind,
    material: ObjectMaterial,
    condition: ObjectCondition,
    functions: Vec<ObjectFunction>,
    connections: Vec<ObjectConnection>,
    modification_history: Vec<String>,
    synthesis_history: Vec<String>,
}

impl ObjectState {
    #[must_use]
    pub fn new(
        identity: ObjectId,
        family: ObjectFamily,
        scale: ObjectScale,
        kind: ObjectKind,
        material: ObjectMaterial,
        condition: ObjectCondition,
        functions: Vec<ObjectFunction>,
        connections: Vec<ObjectConnection>,
        modification_history: Vec<String>,
        synthesis_history: Vec<String>,
    ) -> Self {
        Self {
            identity,
            family,
            scale,
            kind,
            material,
            condition,
            functions,
            connections,
            modification_history,
            synthesis_history,
        }
    }

    #[must_use]
    pub const fn identity(&self) -> ObjectId {
        self.identity
    }

    #[must_use]
    pub const fn family(&self) -> ObjectFamily {
        self.family
    }

    #[must_use]
    pub const fn scale(&self) -> ObjectScale {
        self.scale
    }

    #[must_use]
    pub const fn kind(&self) -> ObjectKind {
        self.kind
    }

    #[must_use]
    pub const fn material(&self) -> ObjectMaterial {
        self.material
    }

    #[must_use]
    pub const fn condition(&self) -> ObjectCondition {
        self.condition
    }

    #[must_use]
    pub fn functions(&self) -> &[ObjectFunction] {
        &self.functions
    }

    #[must_use]
    pub fn connections(&self) -> &[ObjectConnection] {
        &self.connections
    }

    #[must_use]
    pub fn modification_history(&self) -> &[String] {
        &self.modification_history
    }

    #[must_use]
    pub fn synthesis_history(&self) -> &[String] {
        &self.synthesis_history
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFamily {
    Mechanism,
    Tool,
    Weapon,
    Wreckage,
    Crossing,
    Formation,
    SiegeEngine,
    Terrain,
    Foundation,
    EnvironmentalStructure,
    StoneStructure,
    Wound,
    Tissue,
    Infection,
    SymptomPattern,
    ClinicalFinding,
    MemoryRelation,
    Attacker,
    Pursuer,
    PsychicTether,
    Audience,
    Population,
    FutureField,
    SceneInterpretation,
    Crowd,
    Monument,
    Meaning,
    EmotionalRupture,
}

impl ObjectFamily {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mechanism => "Mechanism",
            Self::Tool => "Tool",
            Self::Weapon => "Weapon",
            Self::Wreckage => "Wreckage",
            Self::Crossing => "Crossing",
            Self::Formation => "Formation",
            Self::SiegeEngine => "SiegeEngine",
            Self::Terrain => "Terrain",
            Self::Foundation => "Foundation",
            Self::EnvironmentalStructure => "EnvironmentalStructure",
            Self::StoneStructure => "StoneStructure",
            Self::Wound => "Wound",
            Self::Tissue => "Tissue",
            Self::Infection => "Infection",
            Self::SymptomPattern => "SymptomPattern",
            Self::ClinicalFinding => "ClinicalFinding",
            Self::MemoryRelation => "MemoryRelation",
            Self::Attacker => "Attacker",
            Self::Pursuer => "Pursuer",
            Self::PsychicTether => "PsychicTether",
            Self::Audience => "Audience",
            Self::Population => "Population",
            Self::FutureField => "FutureField",
            Self::SceneInterpretation => "SceneInterpretation",
            Self::Crowd => "Crowd",
            Self::Monument => "Monument",
            Self::Meaning => "Meaning",
            Self::EmotionalRupture => "EmotionalRupture",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectScale {
    Fine,
    Personal,
    Body,
    Structural,
    Collective,
    Massive,
    Terrain,
}

impl ObjectScale {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fine => "Fine",
            Self::Personal => "Personal",
            Self::Body => "Body",
            Self::Structural => "Structural",
            Self::Collective => "Collective",
            Self::Massive => "Massive",
            Self::Terrain => "Terrain",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillId {
    Grip,
    Repair,
    Brace,
    Guard,
    Climb,
}

impl SkillId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grip => "Grip",
            Self::Repair => "Repair",
            Self::Brace => "Brace",
            Self::Guard => "Guard",
            Self::Climb => "Climb",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureExposure {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PracticeContext {
    Mechanical,
    Architectural,
    Cliffside,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillPractice {
    skill: SkillId,
    being: BeingId,
    object_family: ObjectFamily,
    current_invested: u16,
    aura_invested: u16,
    repetitions: u64,
    successful_uses: u64,
    pressure_exposure: PressureExposure,
    contexts: Vec<PracticeContext>,
}

impl SkillPractice {
    #[must_use]
    pub fn canonical_grip_mechanism_practice() -> Self {
        Self {
            skill: SkillId::Grip,
            being: BeingId::Hueman,
            object_family: ObjectFamily::Mechanism,
            current_invested: 2,
            aura_invested: 0,
            repetitions: 5,
            successful_uses: 5,
            pressure_exposure: PressureExposure::Medium,
            contexts: vec![PracticeContext::Mechanical],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeingTrait {
    PrecisionGrip,
    StoneTalons,
    ArchitecturalPosture,
}

impl BeingTrait {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrecisionGrip => "Precision Grip",
            Self::StoneTalons => "Stone Talons",
            Self::ArchitecturalPosture => "Architectural Posture",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingEmbodiment {
    silhouette: &'static str,
    anatomy: &'static str,
    surface: &'static str,
    posture: &'static str,
    movement: &'static str,
    aura: &'static str,
    traits: Vec<BeingTrait>,
}

impl BeingEmbodiment {
    #[must_use]
    pub fn from_frame(frame: FrameId) -> Self {
        match frame {
            FrameId::Gremlin => Self {
                silhouette: "compact tinker silhouette",
                anatomy: "mechanical grip-forward anatomy",
                surface: "worked skin and tool wear",
                posture: "crouched repair stance",
                movement: "quick latch-focused motion",
                aura: "tight local glow",
                traits: vec![BeingTrait::PrecisionGrip],
            },
            FrameId::Troglodyte => Self {
                silhouette: "world-bearing giant silhouette",
                anatomy: "massive cliff-binding anatomy",
                surface: "stone-scored world surface",
                posture: "load-bearing stance",
                movement: "anchored world-scale motion",
                aura: "deep pressure glow",
                traits: vec![BeingTrait::StoneTalons, BeingTrait::ArchitecturalPosture],
            },
            _ => Self {
                silhouette: "Hueman silhouette",
                anatomy: "Hueman anatomy",
                surface: "Hueman surface",
                posture: "Hueman posture",
                movement: "Hueman movement",
                aura: "Hueman aura",
                traits: Vec::new(),
            },
        }
    }

    #[must_use]
    pub fn traits(&self) -> &[BeingTrait] {
        &self.traits
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingInheritance {
    current_lineage: &'static str,
    paired_aura_frame: Option<FrameId>,
    paired_aura_lineage: Option<&'static str>,
    preserved_skill_roots: Vec<SkillId>,
}

impl BeingInheritance {
    #[must_use]
    pub fn from_frame(frame: FrameId) -> Self {
        match frame {
            FrameId::Gremlin => Self {
                current_lineage: "Gremlin",
                paired_aura_frame: Some(FrameId::Pixy),
                paired_aura_lineage: Some("Pixy"),
                preserved_skill_roots: vec![SkillId::Grip],
            },
            FrameId::Goblin => Self {
                current_lineage: "Goblin",
                paired_aura_frame: Some(FrameId::Sprite),
                paired_aura_lineage: Some("Sprite"),
                preserved_skill_roots: vec![SkillId::Grip],
            },
            FrameId::Ghoul => Self {
                current_lineage: "Ghoul",
                paired_aura_frame: None,
                paired_aura_lineage: Some("Spirit"),
                preserved_skill_roots: vec![SkillId::Grip],
            },
            FrameId::Troll => Self {
                current_lineage: "Troll",
                paired_aura_frame: Some(FrameId::Faerie),
                paired_aura_lineage: Some("Faerie"),
                preserved_skill_roots: vec![SkillId::Grip, SkillId::Brace],
            },
            FrameId::Ork => Self {
                current_lineage: "Ork",
                paired_aura_frame: Some(FrameId::Nymph),
                paired_aura_lineage: Some("Nymph"),
                preserved_skill_roots: vec![SkillId::Grip, SkillId::Guard],
            },
            FrameId::Ogre => Self {
                current_lineage: "Ogre",
                paired_aura_frame: Some(FrameId::Siren),
                paired_aura_lineage: Some("Siren"),
                preserved_skill_roots: vec![SkillId::Grip, SkillId::Guard],
            },
            FrameId::Troglodyte => Self {
                current_lineage: "Troglodyte",
                paired_aura_frame: Some(FrameId::Muse),
                paired_aura_lineage: Some("Muse"),
                preserved_skill_roots: vec![SkillId::Grip, SkillId::Brace, SkillId::Climb],
            },
            _ => Self {
                current_lineage: "Hueman",
                paired_aura_frame: None,
                paired_aura_lineage: None,
                preserved_skill_roots: Vec::new(),
            },
        }
    }

    #[must_use]
    pub const fn current_lineage(&self) -> &'static str {
        self.current_lineage
    }

    #[must_use]
    pub const fn paired_aura_frame(&self) -> Option<FrameId> {
        self.paired_aura_frame
    }

    #[must_use]
    pub const fn paired_aura_lineage(&self) -> Option<&'static str> {
        self.paired_aura_lineage
    }

    #[must_use]
    pub fn preserved_skill_roots(&self) -> &[SkillId] {
        &self.preserved_skill_roots
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingState {
    identity: BeingId,
    current_form: FrameId,
    aura_frame: Option<FrameId>,
    flow_learnset: Vec<FlowId>,
    glow_learnset: Vec<GlowId>,
    current_prism: CurrentPrism,
    stable_point_level: u16,
    inheritance: BeingInheritance,
    embodiment: BeingEmbodiment,
}

impl BeingState {
    #[must_use]
    pub const fn identity(&self) -> BeingId {
        self.identity
    }

    #[must_use]
    pub const fn current_form(&self) -> FrameId {
        self.current_form
    }

    #[must_use]
    pub const fn aura_frame(&self) -> Option<FrameId> {
        self.aura_frame
    }

    #[must_use]
    pub fn flow_learnset(&self) -> &[FlowId] {
        &self.flow_learnset
    }

    #[must_use]
    pub fn glow_learnset(&self) -> &[GlowId] {
        &self.glow_learnset
    }

    #[must_use]
    pub const fn current_prism(&self) -> &CurrentPrism {
        &self.current_prism
    }

    #[must_use]
    pub const fn stable_point_level(&self) -> u16 {
        self.stable_point_level
    }

    #[must_use]
    pub const fn inheritance(&self) -> &BeingInheritance {
        &self.inheritance
    }

    #[must_use]
    pub const fn embodiment(&self) -> &BeingEmbodiment {
        &self.embodiment
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressingMode {
    Proxy,
    Moxy,
    Foxy,
}

impl AddressingMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proxy => "Proxy",
            Self::Moxy => "Moxy",
            Self::Foxy => "Foxy",
        }
    }

    #[must_use]
    pub const fn manager(self) -> Manager {
        match self {
            Self::Proxy => Manager::Clouseau,
            Self::Moxy => Manager::Hal,
            Self::Foxy => Manager::Cleopatra,
        }
    }

    #[must_use]
    pub const fn domain(self) -> ManagerDomain {
        match self {
            Self::Proxy => ManagerDomain::Pleb,
            Self::Moxy => ManagerDomain::Meta,
            Self::Foxy => ManagerDomain::Blep,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionAim {
    SecureLatch,
    ScaleWorld,
    RestoreSystem,
    ReturnFunction,
    BraceArchitecture,
    Manipulate,
    OpenPrecisely,
    RetainAndDirect,
    Recover,
    HoldTogether,
    BindFormation,
    SeizeAndRedirect,
    Anchor,
    StabilizeConnectedFarAnchor,
    ExposeAndManipulateHiddenMechanism,
    DirectFocusedTool,
    MendCivicRupture,
    RevealMonumentalProof,
    StabilizeAndClose,
    InciseOrSuture,
    DiagnoseAndReveal,
    RevealCentralTruth,
    SustainEmotionalCommand,
    DirectPreciseLineOfForce,
    JoinMendOrTear,
    RevealProvenSovereignty,
    StabilizeAndDirect,
    RevealBrokenRelation,
    DiagnoseAndExplain,
    MeasureAndMap,
    PresentClearlyAndResponsibly,
    RevealHiddenTruthWithConsent,
    ProjectDreadToStopAttack,
    MisdirectPursuerAwayFromCivilians,
    SeverOrTrapHostileConnection,
    CreateSeductiveDramaticSpectacle,
    ManufactureInevitability,
    ConcealAlternativesAndForceInterpretation,
    RevealEnoughTruthForTreatment,
    RevealOptionsPreservingChoice,
    MakeFabricatedRealityInevitable,
}

impl ActionAim {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecureLatch => "SecureLatch",
            Self::ScaleWorld => "ScaleWorld",
            Self::RestoreSystem => "RestoreSystem",
            Self::ReturnFunction => "ReturnFunction",
            Self::BraceArchitecture => "BraceArchitecture",
            Self::Manipulate => "Manipulate",
            Self::OpenPrecisely => "OpenPrecisely",
            Self::RetainAndDirect => "RetainAndDirect",
            Self::Recover => "Recover",
            Self::HoldTogether => "HoldTogether",
            Self::BindFormation => "BindFormation",
            Self::SeizeAndRedirect => "SeizeAndRedirect",
            Self::Anchor => "Anchor",
            Self::StabilizeConnectedFarAnchor => "StabilizeConnectedFarAnchor",
            Self::ExposeAndManipulateHiddenMechanism => "ExposeAndManipulateHiddenMechanism",
            Self::DirectFocusedTool => "DirectFocusedTool",
            Self::MendCivicRupture => "MendCivicRupture",
            Self::RevealMonumentalProof => "RevealMonumentalProof",
            Self::StabilizeAndClose => "StabilizeAndClose",
            Self::InciseOrSuture => "InciseOrSuture",
            Self::DiagnoseAndReveal => "DiagnoseAndReveal",
            Self::RevealCentralTruth => "RevealCentralTruth",
            Self::SustainEmotionalCommand => "SustainEmotionalCommand",
            Self::DirectPreciseLineOfForce => "DirectPreciseLineOfForce",
            Self::JoinMendOrTear => "JoinMendOrTear",
            Self::RevealProvenSovereignty => "RevealProvenSovereignty",
            Self::StabilizeAndDirect => "StabilizeAndDirect",
            Self::RevealBrokenRelation => "RevealBrokenRelation",
            Self::DiagnoseAndExplain => "DiagnoseAndExplain",
            Self::MeasureAndMap => "MeasureAndMap",
            Self::PresentClearlyAndResponsibly => "PresentClearlyAndResponsibly",
            Self::RevealHiddenTruthWithConsent => "RevealHiddenTruthWithConsent",
            Self::ProjectDreadToStopAttack => "ProjectDreadToStopAttack",
            Self::MisdirectPursuerAwayFromCivilians => "MisdirectPursuerAwayFromCivilians",
            Self::SeverOrTrapHostileConnection => "SeverOrTrapHostileConnection",
            Self::CreateSeductiveDramaticSpectacle => "CreateSeductiveDramaticSpectacle",
            Self::ManufactureInevitability => "ManufactureInevitability",
            Self::ConcealAlternativesAndForceInterpretation => {
                "ConcealAlternativesAndForceInterpretation"
            }
            Self::RevealEnoughTruthForTreatment => "RevealEnoughTruthForTreatment",
            Self::RevealOptionsPreservingChoice => "RevealOptionsPreservingChoice",
            Self::MakeFabricatedRealityInevitable => "MakeFabricatedRealityInevitable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingObjectAction {
    being: BeingState,
    skill: SkillId,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
}

impl BeingObjectAction {
    #[must_use]
    pub fn new(
        being: BeingState,
        skill: SkillId,
        object: ObjectState,
        addressing_mode: AddressingMode,
        aim: ActionAim,
    ) -> Self {
        Self {
            being,
            skill,
            object,
            addressing_mode,
            aim,
        }
    }

    #[must_use]
    pub const fn being(&self) -> &BeingState {
        &self.being
    }

    #[must_use]
    pub const fn skill(&self) -> SkillId {
        self.skill
    }

    #[must_use]
    pub const fn object(&self) -> &ObjectState {
        &self.object
    }

    #[must_use]
    pub const fn addressing_mode(&self) -> AddressingMode {
        self.addressing_mode
    }

    #[must_use]
    pub const fn aim(&self) -> ActionAim {
        self.aim
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingObjectObservation {
    being: BeingState,
    object: ObjectState,
    skill_root: SkillId,
    addressing_mode: AddressingMode,
    aim: ActionAim,
}

impl BeingObjectObservation {
    #[must_use]
    pub const fn being(&self) -> &BeingState {
        &self.being
    }

    #[must_use]
    pub const fn object(&self) -> &ObjectState {
        &self.object
    }

    #[must_use]
    pub const fn skill_root(&self) -> SkillId {
        self.skill_root
    }

    #[must_use]
    pub const fn addressing_mode(&self) -> AddressingMode {
        self.addressing_mode
    }

    #[must_use]
    pub const fn aim(&self) -> ActionAim {
        self.aim
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolvedMoveId {
    TinkerGrip,
    WorldGrip,
    SystemRepair,
    ReturnRepair,
}

impl ResolvedMoveId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TinkerGrip => "TinkerGrip",
            Self::WorldGrip => "World Grip",
            Self::SystemRepair => "System Repair",
            Self::ReturnRepair => "Return Repair",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingObjectMoveResolution {
    observation: BeingObjectObservation,
    inherited_skill_root: SkillId,
    resolved_move: ResolvedMoveId,
    required_current: u16,
    required_aura: u16,
    recipe: Option<SynthesisRecipe>,
    decision_candidate: Option<DecisionCandidateId>,
    recipe_legal: bool,
    recipe_reason: String,
    execution: Option<SynthesisExecution>,
}

impl BeingObjectMoveResolution {
    #[must_use]
    pub const fn observation(&self) -> &BeingObjectObservation {
        &self.observation
    }

    #[must_use]
    pub const fn inherited_skill_root(&self) -> SkillId {
        self.inherited_skill_root
    }

    #[must_use]
    pub const fn resolved_move(&self) -> ResolvedMoveId {
        self.resolved_move
    }

    #[must_use]
    pub const fn required_current(&self) -> u16 {
        self.required_current
    }

    #[must_use]
    pub const fn required_aura(&self) -> u16 {
        self.required_aura
    }

    #[must_use]
    pub const fn recipe(&self) -> Option<&SynthesisRecipe> {
        self.recipe.as_ref()
    }

    #[must_use]
    pub const fn decision_candidate(&self) -> Option<DecisionCandidateId> {
        self.decision_candidate
    }

    #[must_use]
    pub const fn recipe_legal(&self) -> bool {
        self.recipe_legal
    }

    #[must_use]
    pub fn recipe_reason(&self) -> &str {
        &self.recipe_reason
    }

    #[must_use]
    pub const fn execution(&self) -> Option<&SynthesisExecution> {
        self.execution.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HollowingRefinement {
    Precision,
    Stability,
    MaterialLightening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HollowingTarget {
    Being(BeingId),
    Object(ObjectId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HollowingOperation {
    target: HollowingTarget,
    refinement: HollowingRefinement,
}

impl HollowingOperation {
    #[must_use]
    pub const fn new(target: HollowingTarget, refinement: HollowingRefinement) -> Self {
        Self { target, refinement }
    }

    #[must_use]
    pub const fn target(self) -> HollowingTarget {
        self.target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitTransferRule {
    ObjectTraitToBeing(&'static str),
    BeingTraitToObject(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingObjectSynthesisSpec {
    being: BeingState,
    object: ObjectState,
    skill_requirements: Vec<SkillId>,
    transfer_rules: Vec<TraitTransferRule>,
    current_cost: u16,
    aura_cost: u16,
    candidate_recipe: Option<SynthesisRecipe>,
}

impl BeingObjectSynthesisSpec {
    #[must_use]
    pub fn transfer_rules(&self) -> &[TraitTransferRule] {
        &self.transfer_rules
    }

    #[must_use]
    pub const fn candidate_recipe(&self) -> Option<&SynthesisRecipe> {
        self.candidate_recipe.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingObjectContractInput {
    pub object_collapsed_into_being: bool,
    pub universal_appearance_state_only: bool,
    pub skill_without_relation: bool,
    pub move_identical_to_skill: bool,
    pub hollowing_crosses_boundary: bool,
    pub synthesis_bypasses_recipe_legality: bool,
    pub proxy_replaces_object: bool,
    pub moxy_velocity_only: bool,
    pub foxy_automatically_evil: bool,
    pub object_mutation_without_identity_or_history: bool,
    pub erases_hueman_identity: bool,
    pub inherited_skill_roots_discarded: bool,
    pub idle_time_grants_major_progression: bool,
}

impl Default for BeingObjectContractInput {
    fn default() -> Self {
        Self {
            object_collapsed_into_being: false,
            universal_appearance_state_only: false,
            skill_without_relation: false,
            move_identical_to_skill: false,
            hollowing_crosses_boundary: false,
            synthesis_bypasses_recipe_legality: false,
            proxy_replaces_object: false,
            moxy_velocity_only: false,
            foxy_automatically_evil: false,
            object_mutation_without_identity_or_history: false,
            erases_hueman_identity: false,
            inherited_skill_roots_discarded: false,
            idle_time_grants_major_progression: false,
        }
    }
}

#[must_use]
pub fn canonical_being_object_contract_fixture() -> BeingObjectContractInput {
    BeingObjectContractInput::default()
}

#[must_use]
pub fn build_being_state(point: &Point) -> BeingState {
    let frame_state = point.frame_state();
    BeingState {
        identity: frame_state.being(),
        current_form: frame_state.frame(),
        aura_frame: paired_aura_frame(frame_state.frame()),
        flow_learnset: frame_state.flow_learnset().to_vec(),
        glow_learnset: frame_state.glow_learnset().to_vec(),
        current_prism: frame_state.prism().clone(),
        stable_point_level: point.progression().stable_point_level(),
        inheritance: BeingInheritance::from_frame(frame_state.frame()),
        embodiment: BeingEmbodiment::from_frame(frame_state.frame()),
    }
}

#[must_use]
pub fn build_canonical_being_state_for_frame(frame: FrameId) -> BeingState {
    build_canonical_being_state_with_aura(frame, paired_aura_frame(frame))
}

#[must_use]
pub fn build_canonical_being_state_with_aura(
    frame: FrameId,
    aura_frame: Option<FrameId>,
) -> BeingState {
    BeingState {
        identity: BeingId::Hueman,
        current_form: frame,
        aura_frame,
        flow_learnset: canonical_flow_learnset(frame),
        glow_learnset: aura_frame.map_or_else(Vec::new, canonical_glow_learnset),
        current_prism: CurrentPrism::origin(),
        stable_point_level: 1,
        inheritance: BeingInheritance::from_frame(frame),
        embodiment: BeingEmbodiment::from_frame(frame),
    }
}

#[must_use]
pub fn canonical_object_state(identity: ObjectId) -> ObjectState {
    match identity {
        ObjectId::MechanicalLatch => ObjectState::new(
            identity,
            ObjectFamily::Mechanism,
            ObjectScale::Fine,
            ObjectKind::Mechanism,
            ObjectMaterial::Metal,
            ObjectCondition::Stable,
            vec![ObjectFunction::Latch, ObjectFunction::Repairable],
            Vec::new(),
            vec![String::from("maintained latch assembly")],
            Vec::new(),
        ),
        ObjectId::FracturedCliff => ObjectState::new(
            identity,
            ObjectFamily::Terrain,
            ObjectScale::Terrain,
            ObjectKind::Terrain,
            ObjectMaterial::Stone,
            ObjectCondition::Fractured,
            vec![
                ObjectFunction::ClimbSurface,
                ObjectFunction::GuardSupport,
                ObjectFunction::LoadPath,
                ObjectFunction::StructuralSupport,
            ],
            Vec::new(),
            vec![String::from("fracture line recorded")],
            Vec::new(),
        ),
        ObjectId::Weapon => ObjectState::new(
            identity,
            ObjectFamily::Weapon,
            ObjectScale::Personal,
            ObjectKind::Tool,
            ObjectMaterial::Metal,
            ObjectCondition::Stable,
            vec![ObjectFunction::WeaponLine, ObjectFunction::DirectionalTool],
            Vec::new(),
            vec![String::from(
                "weapon remains ready for retention and redirection",
            )],
            Vec::new(),
        ),
        ObjectId::DamagedWreckage => ObjectState::new(
            identity,
            ObjectFamily::Wreckage,
            ObjectScale::Body,
            ObjectKind::MaterialConstruct,
            ObjectMaterial::Mixed,
            ObjectCondition::Damaged,
            vec![
                ObjectFunction::Repairable,
                ObjectFunction::StructuralSupport,
            ],
            Vec::new(),
            vec![String::from(
                "wreckage remains damaged and resistant to release",
            )],
            Vec::new(),
        ),
        ObjectId::BrokenCrossingSupport => ObjectState::new(
            identity,
            ObjectFamily::Crossing,
            ObjectScale::Structural,
            ObjectKind::Structure,
            ObjectMaterial::Stone,
            ObjectCondition::Failing,
            vec![ObjectFunction::LoadPath, ObjectFunction::StructuralSupport],
            vec![ObjectConnection::new(
                ObjectId::FailingFarAnchor,
                "holds_toward",
            )],
            vec![String::from("crossing support is failing at the join")],
            Vec::new(),
        ),
        ObjectId::NearBridgeSupport => ObjectState::new(
            identity,
            ObjectFamily::Crossing,
            ObjectScale::Structural,
            ObjectKind::Structure,
            ObjectMaterial::Stone,
            ObjectCondition::Failing,
            vec![ObjectFunction::LoadPath, ObjectFunction::StructuralSupport],
            vec![ObjectConnection::new(
                ObjectId::FailingFarAnchor,
                "anchors_toward",
            )],
            vec![String::from(
                "near support still stands but the far anchor is failing",
            )],
            Vec::new(),
        ),
        ObjectId::FailingFarAnchor => ObjectState::new(
            identity,
            ObjectFamily::Crossing,
            ObjectScale::Structural,
            ObjectKind::Structure,
            ObjectMaterial::Stone,
            ObjectCondition::Failing,
            vec![ObjectFunction::LoadPath, ObjectFunction::StructuralSupport],
            Vec::new(),
            vec![String::from("far anchor is taking too much load")],
            Vec::new(),
        ),
        ObjectId::ShieldFormationAnchor => ObjectState::new(
            identity,
            ObjectFamily::Formation,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Mixed,
            ObjectCondition::Unsettled,
            vec![ObjectFunction::GroupAnchor, ObjectFunction::GuardSupport],
            Vec::new(),
            vec![String::from(
                "shield formation anchor needs collective continuity",
            )],
            Vec::new(),
        ),
        ObjectId::SiegeEngine => ObjectState::new(
            identity,
            ObjectFamily::SiegeEngine,
            ObjectScale::Massive,
            ObjectKind::Structure,
            ObjectMaterial::Metal,
            ObjectCondition::Stable,
            vec![
                ObjectFunction::DirectionalTool,
                ObjectFunction::LoadPath,
                ObjectFunction::StructuralSupport,
            ],
            Vec::new(),
            vec![String::from("siege engine resists ordinary leverage")],
            Vec::new(),
        ),
        ObjectId::ReverseFacingHiddenLatch => ObjectState::new(
            identity,
            ObjectFamily::Mechanism,
            ObjectScale::Fine,
            ObjectKind::Mechanism,
            ObjectMaterial::Metal,
            ObjectCondition::Hidden,
            vec![ObjectFunction::HiddenLatch, ObjectFunction::Latch],
            vec![ObjectConnection::new(
                ObjectId::MechanicalLatch,
                "returns_from",
            )],
            vec![String::from(
                "reverse-facing hidden latch sits beneath the visible plate",
            )],
            Vec::new(),
        ),
        ObjectId::FocusedIndustrialTool => ObjectState::new(
            identity,
            ObjectFamily::Tool,
            ObjectScale::Massive,
            ObjectKind::Tool,
            ObjectMaterial::Metal,
            ObjectCondition::Stable,
            vec![ObjectFunction::DirectionalTool, ObjectFunction::LoadPath],
            Vec::new(),
            vec![String::from("industrial tool requires a stable beam line")],
            Vec::new(),
        ),
        ObjectId::CivicRupture => ObjectState::new(
            identity,
            ObjectFamily::Formation,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Damaged,
            vec![ObjectFunction::GroupAnchor, ObjectFunction::RelationalFault],
            Vec::new(),
            vec![String::from(
                "civic continuity has torn under shared pressure",
            )],
            Vec::new(),
        ),
        ObjectId::MonumentalFoundation => ObjectState::new(
            identity,
            ObjectFamily::Foundation,
            ObjectScale::Massive,
            ObjectKind::Structure,
            ObjectMaterial::Stone,
            ObjectCondition::Stable,
            vec![ObjectFunction::TitleSurface, ObjectFunction::LoadPath],
            Vec::new(),
            vec![String::from("foundation awaits visible proof of burden")],
            Vec::new(),
        ),
        ObjectId::DoorMechanism => ObjectState::new(
            identity,
            ObjectFamily::Mechanism,
            ObjectScale::Personal,
            ObjectKind::Mechanism,
            ObjectMaterial::Mixed,
            ObjectCondition::Damaged,
            vec![ObjectFunction::AccessControl, ObjectFunction::Repairable],
            vec![ObjectConnection::new(
                ObjectId::ControlSystem,
                "connected_to",
            )],
            vec![String::from("control linkage drifted off true")],
            Vec::new(),
        ),
        ObjectId::ControlSystem => ObjectState::new(
            identity,
            ObjectFamily::Mechanism,
            ObjectScale::Structural,
            ObjectKind::System,
            ObjectMaterial::Mixed,
            ObjectCondition::Stable,
            vec![ObjectFunction::AccessControl],
            Vec::new(),
            vec![String::from("control system online")],
            Vec::new(),
        ),
        ObjectId::StoneObject => ObjectState::new(
            identity,
            ObjectFamily::StoneStructure,
            ObjectScale::Personal,
            ObjectKind::MaterialConstruct,
            ObjectMaterial::Stone,
            ObjectCondition::Stable,
            vec![ObjectFunction::GuardSupport],
            Vec::new(),
            vec![String::from("stone object mined and stabilized")],
            Vec::new(),
        ),
        ObjectId::DamagedMechanism => ObjectState::new(
            identity,
            ObjectFamily::Mechanism,
            ObjectScale::Personal,
            ObjectKind::Mechanism,
            ObjectMaterial::Metal,
            ObjectCondition::Damaged,
            vec![ObjectFunction::Repairable],
            Vec::new(),
            vec![String::from("gear train slipped off track")],
            Vec::new(),
        ),
        ObjectId::StoneArchitecture => ObjectState::new(
            identity,
            ObjectFamily::StoneStructure,
            ObjectScale::Structural,
            ObjectKind::Structure,
            ObjectMaterial::Stone,
            ObjectCondition::Stable,
            vec![ObjectFunction::GuardSupport, ObjectFunction::ClimbSurface],
            Vec::new(),
            vec![String::from("stone architecture holds the seam")],
            vec![String::from("candidate Gargoyle lineage contact")],
        ),
        ObjectId::OpenWound => ObjectState::new(
            identity,
            ObjectFamily::Wound,
            ObjectScale::Body,
            ObjectKind::Body,
            ObjectMaterial::Flesh,
            ObjectCondition::Open,
            vec![ObjectFunction::OpenBoundary, ObjectFunction::Repairable],
            Vec::new(),
            vec![String::from("wound remains open and unstable")],
            Vec::new(),
        ),
        ObjectId::Tissue => ObjectState::new(
            identity,
            ObjectFamily::Tissue,
            ObjectScale::Body,
            ObjectKind::Body,
            ObjectMaterial::Flesh,
            ObjectCondition::Stable,
            vec![ObjectFunction::Repairable],
            Vec::new(),
            vec![String::from("tissue ready for precise intervention")],
            Vec::new(),
        ),
        ObjectId::HiddenInfection => ObjectState::new(
            identity,
            ObjectFamily::Infection,
            ObjectScale::Body,
            ObjectKind::Body,
            ObjectMaterial::Flesh,
            ObjectCondition::Hidden,
            vec![ObjectFunction::HiddenCondition],
            Vec::new(),
            vec![String::from("infection hides beneath apparent recovery")],
            Vec::new(),
        ),
        ObjectId::SymptomPattern => ObjectState::new(
            identity,
            ObjectFamily::SymptomPattern,
            ObjectScale::Structural,
            ObjectKind::System,
            ObjectMaterial::Mixed,
            ObjectCondition::Diagnosed,
            vec![
                ObjectFunction::DiagnosticPattern,
                ObjectFunction::MeaningField,
            ],
            vec![ObjectConnection::new(
                ObjectId::ClinicalFinding,
                "maps_toward",
            )],
            vec![String::from(
                "symptom pattern remains measurable but still carries uncertainty",
            )],
            Vec::new(),
        ),
        ObjectId::ClinicalFinding => ObjectState::new(
            identity,
            ObjectFamily::ClinicalFinding,
            ObjectScale::Personal,
            ObjectKind::System,
            ObjectMaterial::Mixed,
            ObjectCondition::Diagnosed,
            vec![
                ObjectFunction::PresentationSurface,
                ObjectFunction::DiagnosticPattern,
            ],
            Vec::new(),
            vec![String::from(
                "clinical finding is ready for legible presentation",
            )],
            Vec::new(),
        ),
        ObjectId::ConcealedMemoryRelation => ObjectState::new(
            identity,
            ObjectFamily::MemoryRelation,
            ObjectScale::Personal,
            ObjectKind::SymbolicRelation,
            ObjectMaterial::Social,
            ObjectCondition::Concealed,
            vec![
                ObjectFunction::RelationalFault,
                ObjectFunction::MeaningField,
                ObjectFunction::HiddenCondition,
            ],
            Vec::new(),
            vec![String::from(
                "memory relation remains concealed beneath the visible account",
            )],
            Vec::new(),
        ),
        ObjectId::ImmediateAttacker => ObjectState::new(
            identity,
            ObjectFamily::Attacker,
            ObjectScale::Body,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Threatening,
            vec![
                ObjectFunction::ThreatVector,
                ObjectFunction::CollectivePresence,
            ],
            Vec::new(),
            vec![String::from(
                "immediate attacker is pressing the scene toward violence",
            )],
            Vec::new(),
        ),
        ObjectId::HostilePursuer => ObjectState::new(
            identity,
            ObjectFamily::Pursuer,
            ObjectScale::Body,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Threatening,
            vec![
                ObjectFunction::ThreatVector,
                ObjectFunction::DirectionalTool,
            ],
            Vec::new(),
            vec![String::from(
                "hostile pursuer remains focused on the fleeing route",
            )],
            Vec::new(),
        ),
        ObjectId::HostilePsychicTether => ObjectState::new(
            identity,
            ObjectFamily::PsychicTether,
            ObjectScale::Personal,
            ObjectKind::SymbolicRelation,
            ObjectMaterial::Social,
            ObjectCondition::Coercive,
            vec![
                ObjectFunction::PsychicTether,
                ObjectFunction::RelationalFault,
            ],
            Vec::new(),
            vec![String::from(
                "hostile tether narrows the target's available psychic distance",
            )],
            Vec::new(),
        ),
        ObjectId::TheatricalAudience => ObjectState::new(
            identity,
            ObjectFamily::Audience,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Stable,
            vec![
                ObjectFunction::AudienceAttention,
                ObjectFunction::CollectivePresence,
            ],
            Vec::new(),
            vec![String::from(
                "audience attention is present within a consensual performance frame",
            )],
            Vec::new(),
        ),
        ObjectId::PoliticallyVulnerableCrowd => ObjectState::new(
            identity,
            ObjectFamily::Population,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Vulnerable,
            vec![
                ObjectFunction::AudienceAttention,
                ObjectFunction::CollectivePresence,
            ],
            Vec::new(),
            vec![String::from(
                "crowd vulnerability makes attention pressure materially consequential",
            )],
            Vec::new(),
        ),
        ObjectId::SevereDiagnosis => ObjectState::new(
            identity,
            ObjectFamily::ClinicalFinding,
            ObjectScale::Personal,
            ObjectKind::System,
            ObjectMaterial::Mixed,
            ObjectCondition::Diagnosed,
            vec![
                ObjectFunction::DiagnosticPattern,
                ObjectFunction::PresentationSurface,
                ObjectFunction::MeaningField,
            ],
            Vec::new(),
            vec![String::from(
                "diagnosis is true but psychologically heavy to disclose all at once",
            )],
            Vec::new(),
        ),
        ObjectId::MultiplePossibleFutures => ObjectState::new(
            identity,
            ObjectFamily::FutureField,
            ObjectScale::Collective,
            ObjectKind::SymbolicRelation,
            ObjectMaterial::Social,
            ObjectCondition::Unsettled,
            vec![ObjectFunction::ChoiceField, ObjectFunction::MeaningField],
            Vec::new(),
            vec![String::from(
                "future field remains plural and requires uncertainty to stay visible",
            )],
            Vec::new(),
        ),
        ObjectId::SharedSceneInterpretation => ObjectState::new(
            identity,
            ObjectFamily::SceneInterpretation,
            ObjectScale::Collective,
            ObjectKind::SymbolicRelation,
            ObjectMaterial::Social,
            ObjectCondition::Coercive,
            vec![ObjectFunction::SceneNarrative, ObjectFunction::MeaningField],
            Vec::new(),
            vec![String::from(
                "scene interpretation can be authored toward false inevitability",
            )],
            Vec::new(),
        ),
        ObjectId::TargetPopulation => ObjectState::new(
            identity,
            ObjectFamily::Population,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Vulnerable,
            vec![
                ObjectFunction::AudienceAttention,
                ObjectFunction::MeaningField,
            ],
            Vec::new(),
            vec![String::from(
                "population can still compare alternatives if they remain visible",
            )],
            Vec::new(),
        ),
        ObjectId::MeaningLink => ObjectState::new(
            identity,
            ObjectFamily::Meaning,
            ObjectScale::Collective,
            ObjectKind::SymbolicRelation,
            ObjectMaterial::Social,
            ObjectCondition::Hidden,
            vec![ObjectFunction::RelationalFault],
            Vec::new(),
            vec![String::from("crowd meaning remains linked but unread")],
            Vec::new(),
        ),
        ObjectId::FrightenedCrowd => ObjectState::new(
            identity,
            ObjectFamily::Crowd,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Social,
            ObjectCondition::Frightened,
            vec![ObjectFunction::CollectivePresence],
            Vec::new(),
            vec![String::from("crowd morale fractured by pressure")],
            Vec::new(),
        ),
        ObjectId::CuttingTool => ObjectState::new(
            identity,
            ObjectFamily::Tool,
            ObjectScale::Personal,
            ObjectKind::Tool,
            ObjectMaterial::Metal,
            ObjectCondition::Stable,
            vec![ObjectFunction::DirectionalTool, ObjectFunction::Repairable],
            Vec::new(),
            vec![String::from("cutting tool aligned for beam work")],
            Vec::new(),
        ),
        ObjectId::FormationAnchor => ObjectState::new(
            identity,
            ObjectFamily::Formation,
            ObjectScale::Collective,
            ObjectKind::Collective,
            ObjectMaterial::Mixed,
            ObjectCondition::Unsettled,
            vec![ObjectFunction::GroupAnchor, ObjectFunction::GuardSupport],
            Vec::new(),
            vec![String::from("formation anchor destabilized under pressure")],
            Vec::new(),
        ),
        ObjectId::Monument => ObjectState::new(
            identity,
            ObjectFamily::Monument,
            ObjectScale::Structural,
            ObjectKind::Structure,
            ObjectMaterial::Stone,
            ObjectCondition::Stable,
            vec![ObjectFunction::TitleSurface, ObjectFunction::GuardSupport],
            Vec::new(),
            vec![String::from("monument awaits proven public consequence")],
            Vec::new(),
        ),
        ObjectId::HiddenEmotionalRupture => ObjectState::new(
            identity,
            ObjectFamily::EmotionalRupture,
            ObjectScale::Collective,
            ObjectKind::SymbolicRelation,
            ObjectMaterial::Social,
            ObjectCondition::Hidden,
            vec![
                ObjectFunction::RelationalFault,
                ObjectFunction::HiddenCondition,
            ],
            Vec::new(),
            vec![String::from(
                "the broken relation is concealed beneath surface calm",
            )],
            Vec::new(),
        ),
    }
}

#[must_use]
pub fn observe_being_object_action(
    point: &Point,
    action: &BeingObjectAction,
) -> BeingObjectObservation {
    let _derived_being = build_being_state(point);
    BeingObjectObservation {
        being: action.being.clone(),
        object: action.object.clone(),
        skill_root: action.skill,
        addressing_mode: action.addressing_mode,
        aim: action.aim,
    }
}

pub fn resolve_being_object_action(
    point: &Point,
    action: &BeingObjectAction,
) -> io::Result<BeingObjectMoveResolution> {
    let observation = observe_being_object_action(point, action);
    let inherited_skill_root = action.skill;

    let (
        resolved_move,
        required_current,
        required_aura,
        decision_candidate,
        recipe_legal,
        recipe_reason,
    ) = match (
        action.being.current_form(),
        action.skill,
        action.object.identity(),
        action.addressing_mode,
    ) {
        (FrameId::Gremlin, SkillId::Grip, ObjectId::MechanicalLatch, AddressingMode::Proxy) => (
            ResolvedMoveId::TinkerGrip,
            2,
            0,
            Some(DecisionCandidateId::GremlinTinker),
            true,
            String::from("Legal canonical Proxy grip on a local mechanism."),
        ),
        (FrameId::Troglodyte, SkillId::Grip, ObjectId::FracturedCliff, AddressingMode::Proxy) => (
            ResolvedMoveId::WorldGrip,
            4,
            0,
            None,
            false,
            String::from(
                "Resolved move is inherited, but no canonical Recipe fixture exists yet and mastery/legality must be proven separately.",
            ),
        ),
        (FrameId::Gremlin, SkillId::Repair, ObjectId::DoorMechanism, AddressingMode::Moxy) => (
            ResolvedMoveId::SystemRepair,
            2,
            1,
            None,
            false,
            String::from(
                "Moxy repair addresses the connection beyond the local mechanism and cannot auto-repair without a legal Recipe.",
            ),
        ),
        (FrameId::Gremlin, SkillId::Repair, ObjectId::DamagedMechanism, AddressingMode::Foxy) => (
            ResolvedMoveId::ReturnRepair,
            2,
            1,
            None,
            false,
            String::from(
                "Foxy repair addresses return or inverse state and cannot become legal automatically.",
            ),
        ),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no canonical Being/Object move mapping exists for this action",
            ));
        }
    };

    let recipe = decision_candidate.map(resolve_candidate_recipe);
    let execution = if let Some(recipe) = recipe.as_ref() {
        Some(execute_synthesis_recipe(point, recipe).map_err(|error| {
            io::Error::other(format!("move Recipe execution failed: {error:?}"))
        })?)
    } else {
        None
    };

    Ok(BeingObjectMoveResolution {
        observation,
        inherited_skill_root,
        resolved_move,
        required_current,
        required_aura,
        recipe,
        decision_candidate,
        recipe_legal,
        recipe_reason,
        execution,
    })
}

#[must_use]
pub fn canonical_gremlin_proxy_action_fixture() -> BeingObjectAction {
    let point = canonical_gremlin_point();
    BeingObjectAction::new(
        build_being_state(&point),
        SkillId::Grip,
        canonical_object_state(ObjectId::MechanicalLatch),
        AddressingMode::Proxy,
        ActionAim::SecureLatch,
    )
}

#[must_use]
pub fn canonical_troglodyte_proxy_action_fixture() -> BeingObjectAction {
    let point = canonical_troglodyte_point();
    BeingObjectAction::new(
        build_being_state(&point),
        SkillId::Grip,
        canonical_object_state(ObjectId::FracturedCliff),
        AddressingMode::Proxy,
        ActionAim::ScaleWorld,
    )
}

#[must_use]
pub fn canonical_moxy_repair_fixture() -> BeingObjectAction {
    let point = canonical_gremlin_point();
    BeingObjectAction::new(
        build_being_state(&point),
        SkillId::Repair,
        canonical_object_state(ObjectId::DoorMechanism),
        AddressingMode::Moxy,
        ActionAim::RestoreSystem,
    )
}

#[must_use]
pub fn canonical_foxy_repair_fixture() -> BeingObjectAction {
    let point = canonical_gremlin_point();
    BeingObjectAction::new(
        build_being_state(&point),
        SkillId::Repair,
        canonical_object_state(ObjectId::DamagedMechanism),
        AddressingMode::Foxy,
        ActionAim::ReturnFunction,
    )
}

#[must_use]
pub fn canonical_hollow_object_fixture() -> HollowingOperation {
    HollowingOperation::new(
        HollowingTarget::Object(ObjectId::StoneObject),
        HollowingRefinement::MaterialLightening,
    )
}

#[must_use]
pub fn canonical_hollow_being_fixture() -> HollowingOperation {
    HollowingOperation::new(
        HollowingTarget::Being(BeingId::Hueman),
        HollowingRefinement::Precision,
    )
}

#[must_use]
pub fn canonical_cross_boundary_synthesis_fixture() -> BeingObjectSynthesisSpec {
    BeingObjectSynthesisSpec {
        being: build_being_state(&canonical_gremlin_point()),
        object: canonical_object_state(ObjectId::StoneArchitecture),
        skill_requirements: vec![SkillId::Brace, SkillId::Guard, SkillId::Climb],
        transfer_rules: vec![
            TraitTransferRule::ObjectTraitToBeing("stone anatomy"),
            TraitTransferRule::ObjectTraitToBeing("talons"),
            TraitTransferRule::BeingTraitToObject("living guard behavior"),
        ],
        current_cost: 2,
        aura_cost: 2,
        candidate_recipe: Some(resolve_candidate_recipe(DecisionCandidateId::GremlinTinker)),
    }
}

#[must_use]
pub fn validate_being_object_contract(
    input: &BeingObjectContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.object_collapsed_into_being {
        diagnostics.push(being_object_error(
            "Being and Object must remain independently addressable and cannot collapse into one enum meaning.",
        ));
    }
    if input.universal_appearance_state_only {
        diagnostics.push(being_object_error(
            "Being and Object cannot be represented only as one universal appearance state.",
        ));
    }
    if input.skill_without_relation {
        diagnostics.push(being_object_error(
            "A Skill must remain a practiced relation between Being and Object.",
        ));
    }
    if input.move_identical_to_skill {
        diagnostics.push(being_object_error(
            "Move cannot be treated as identical to Skill; Move is the Form- or Frame-specific expression of Skill on an Object.",
        ));
    }
    if input.hollowing_crosses_boundary {
        diagnostics.push(being_object_error(
            "Hollowing refines one side and cannot silently transfer traits across the Being/Object boundary.",
        ));
    }
    if input.synthesis_bypasses_recipe_legality {
        diagnostics.push(being_object_error(
            "Synthesis cannot bypass Recipe legality or the existing V2 → V1.1 boundary.",
        ));
    }
    if input.proxy_replaces_object {
        diagnostics.push(being_object_error(
            "Proxy is an addressing mode and cannot replace the Object.",
        ));
    }
    if input.moxy_velocity_only {
        diagnostics.push(being_object_error(
            "Moxy must describe what the Object connects toward and cannot be reduced to velocity only.",
        ));
    }
    if input.foxy_automatically_evil {
        diagnostics.push(being_object_error(
            "Foxy cannot automatically mean evil; its root meaning is reflection, inversion, and return.",
        ));
    }
    if input.object_mutation_without_identity_or_history {
        diagnostics.push(being_object_error(
            "Object mutation must preserve Object identity and history.",
        ));
    }
    if input.erases_hueman_identity {
        diagnostics.push(being_object_error(
            "Being mutation cannot erase persistent Hueman identity.",
        ));
    }
    if input.inherited_skill_roots_discarded {
        diagnostics.push(being_object_error(
            "Natural inheritance must preserve practiced Skill roots.",
        ));
    }
    if input.idle_time_grants_major_progression {
        diagnostics.push(being_object_error(
            "Idle elapsed time cannot grant major embodiment progression without practice events.",
        ));
    }

    diagnostics
}

pub fn build_being_object_witness() -> io::Result<String> {
    let synthesis = canonical_cross_boundary_synthesis_fixture();
    Ok(format!(
        "HOLLOW GROVE BEING / OBJECT ONTOLOGY\n\n\
         Being:\n\
         Living practiced Hueman identity\n\n\
         Object:\n\
         External addressable thing or construct\n\n\
         Skill:\n\
         Practiced relation between Being and Object\n\n\
         Move:\n\
         Form- or Frame-specific expression of Skill on Object\n\n\
         Hollowing:\n\
         Refines Being or Object without crossing the boundary\n\n\
         Synthesis:\n\
         Allows qualities to cross between Being and Object\n\n\
         Addressing:\n\
         Proxy = immediate Object\n\
         Moxy = what the Object connects toward\n\
         Foxy = inverse/reflected/return Object relation\n\n\
         Canonical doctrine:\n\
         Being acts.\n\
         Object receives or participates.\n\
         Skill relates them.\n\
         Move expresses the relation.\n\
         Hollowing perfects a side.\n\
         Synthesis crosses the boundary.\n\n\
         Cross-Boundary Fixture:\n\
         Being: {}\n\
         Object: {}\n\
         Skills: Brace / Guard / Climb\n\
         Paired Aura Lineage: {}\n\
         Transfer Rules: {}\n\
         Candidate Recipe Boundary: {}\n",
        frame_label(synthesis.being.current_form()),
        synthesis.object.identity().as_str(),
        synthesis
            .being
            .inheritance()
            .paired_aura_lineage()
            .unwrap_or("unset"),
        synthesis.transfer_rules().len(),
        synthesis
            .candidate_recipe()
            .map(SynthesisRecipe::display_name)
            .unwrap_or("unset"),
    ))
}

pub fn build_being_object_validation_report() -> io::Result<String> {
    let diagnostics = validate_being_object_contract(&canonical_being_object_contract_fixture());
    let gremlin = resolve_being_object_action(
        &canonical_gremlin_point(),
        &canonical_gremlin_proxy_action_fixture(),
    )?;
    let moxy =
        resolve_being_object_action(&canonical_gremlin_point(), &canonical_moxy_repair_fixture())?;
    let foxy =
        resolve_being_object_action(&canonical_gremlin_point(), &canonical_foxy_repair_fixture())?;
    let hollow_object = canonical_hollow_object_fixture();
    let hollow_being = canonical_hollow_being_fixture();
    let synthesis = canonical_cross_boundary_synthesis_fixture();

    let mut errors = diagnostics;
    if !matches!(hollow_object.target(), HollowingTarget::Object(_)) {
        errors.push(being_object_error(
            "canonical Hollow Object fixture must target Object only",
        ));
    }
    if !matches!(hollow_being.target(), HollowingTarget::Being(_)) {
        errors.push(being_object_error(
            "canonical Hollow Being fixture must target Being only",
        ));
    }
    if synthesis.transfer_rules().is_empty() || synthesis.candidate_recipe().is_none() {
        errors.push(being_object_error(
            "canonical Synthesis fixture must represent cross-boundary transfer through a Recipe boundary",
        ));
    }
    if gremlin.resolved_move() != ResolvedMoveId::TinkerGrip || !gremlin.recipe_legal() {
        errors.push(being_object_error(
            "canonical Gremlin Grip fixture must resolve to legal TinkerGrip",
        ));
    }
    if moxy.recipe_legal() || foxy.recipe_legal() {
        errors.push(being_object_error(
            "Moxy and Foxy repair fixtures must not auto-become legal",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Being / Object Validation\n\n\
             - status: pass\n\
             - Being and Object separation: pass\n\
             - Skill relation semantics: pass\n\
             - Move expression semantics: pass\n\
             - Hollowing target rules: pass\n\
             - Synthesis cross-boundary rules: pass\n\
             - Proxy / Moxy / Foxy addressing: pass\n\
             - natural inheritance Skill roots: pass\n\
             - Hueman identity persistence: pass\n\
             - V1.1 topology unchanged: pass\n",
        ))
    } else {
        let mut output =
            String::from("# Hollow Grove Being / Object Validation\n\n- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        Ok(output)
    }
}

pub fn build_move_witness() -> io::Result<String> {
    let point = canonical_gremlin_point();
    let action = canonical_gremlin_proxy_action_fixture();
    let resolution = resolve_being_object_action(&point, &action)?;
    let execution_boundary = if resolution.execution().is_some() {
        "V1.1 unchanged"
    } else {
        "V1.1 not entered"
    };
    Ok(format!(
        "HOLLOW GROVE MOVE WITNESS\n\n\
         Being:\n\
         {}\n\n\
         Skill:\n\
         {}\n\n\
         Object:\n\
         {}\n\n\
         Mode:\n\
         {}\n\n\
         Inherited Skill Root:\n\
         {}\n\n\
         Resolved Move:\n\
         {}\n\n\
         Recipe Status:\n\
         {}\n\n\
         Execution Boundary:\n\
         {}\n",
        frame_label(resolution.observation().being().current_form()),
        resolution.observation().skill_root().as_str(),
        resolution.observation().object().identity().as_str(),
        resolution.observation().addressing_mode().as_str(),
        resolution.inherited_skill_root().as_str(),
        resolution.resolved_move().as_str(),
        if resolution.recipe_legal() {
            "Legal"
        } else {
            resolution.recipe_reason()
        },
        execution_boundary,
    ))
}

fn being_object_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::BeingObjectOntologyMismatch,
        message: message.into(),
    }
}

#[must_use]
fn paired_aura_frame(frame: FrameId) -> Option<FrameId> {
    match frame {
        FrameId::Gremlin => Some(FrameId::Pixy),
        FrameId::Goblin => Some(FrameId::Sprite),
        FrameId::Troll => Some(FrameId::Faerie),
        FrameId::Ork => Some(FrameId::Nymph),
        FrameId::Ogre => Some(FrameId::Siren),
        FrameId::Troglodyte => Some(FrameId::Muse),
        _ => None,
    }
}

#[must_use]
fn canonical_flow_learnset(frame: FrameId) -> Vec<FlowId> {
    match frame {
        FrameId::Gremlin => vec![FlowId::TinkerGrip],
        FrameId::Troglodyte => vec![FlowId::TinkerGrip, FlowId::Stonefold],
        _ => Vec::new(),
    }
}

#[must_use]
fn canonical_glow_learnset(frame: FrameId) -> Vec<GlowId> {
    match frame {
        FrameId::Pixy => vec![GlowId::Confusion],
        FrameId::Siren => vec![GlowId::Projection],
        FrameId::Muse => vec![GlowId::MuseChorus],
        _ => Vec::new(),
    }
}

#[must_use]
fn canonical_gremlin_point() -> Point {
    Point::with_domain_state(
        FrameState::new(
            FrameId::Gremlin,
            CurrentPrism::new(3, 1, 1, 1, 1),
            vec![FlowId::TinkerGrip],
            Vec::new(),
        ),
        PointProgressionState::origin(),
        crate::point_progression::ReachableWorldState::origin(),
    )
}

#[must_use]
fn canonical_troglodyte_point() -> Point {
    Point::with_domain_state(
        FrameState::new(
            FrameId::Troglodyte,
            CurrentPrism::new(6, 3, 2, 2, 2),
            vec![FlowId::TinkerGrip, FlowId::Stonefold],
            Vec::new(),
        ),
        PointProgressionState::origin(),
        crate::point_progression::ReachableWorldState::origin(),
    )
}

#[must_use]
fn frame_label(frame: FrameId) -> &'static str {
    match frame {
        FrameId::Hueman => "Hueman",
        FrameId::Gremlin => "Gremlin",
        FrameId::Goblin => "Goblin",
        FrameId::Ghoul => "Ghoul",
        FrameId::Troll => "Troll",
        FrameId::Ork => "Ork",
        FrameId::Ogre => "Ogre",
        FrameId::Troglodyte => "Troglodyte",
        FrameId::Pixy => "Pixy",
        FrameId::Sprite => "Sprite",
        FrameId::Faerie => "Faerie",
        FrameId::Nymph => "Nymph",
        FrameId::Siren => "Siren",
        FrameId::Muse => "Muse",
        FrameId::Werewolf => "Werewolf",
        FrameId::Gargoyle => "Gargoyle",
        FrameId::Merman => "Merman",
        FrameId::Chimera => "Chimera",
        FrameId::Manticore => "Manticore",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddressingMode, BeingObjectContractInput, HollowingTarget, ObjectId, ResolvedMoveId,
        SkillId, build_being_object_validation_report, build_being_object_witness,
        build_move_witness, canonical_being_object_contract_fixture,
        canonical_cross_boundary_synthesis_fixture, canonical_foxy_repair_fixture,
        canonical_gremlin_proxy_action_fixture, canonical_hollow_being_fixture,
        canonical_hollow_object_fixture, canonical_moxy_repair_fixture,
        canonical_troglodyte_proxy_action_fixture, resolve_being_object_action,
        validate_being_object_contract,
    };

    #[test]
    fn gremlin_tinker_grip_fixture_resolves_through_existing_recipe_boundary() {
        let point = super::canonical_gremlin_point();
        let action = canonical_gremlin_proxy_action_fixture();
        let resolution =
            resolve_being_object_action(&point, &action).expect("resolution should succeed");

        assert_eq!(
            resolution.observation().being().current_form(),
            crate::FrameId::Gremlin
        );
        assert_eq!(resolution.observation().skill_root(), SkillId::Grip);
        assert_eq!(
            resolution.observation().object().identity(),
            ObjectId::MechanicalLatch
        );
        assert_eq!(
            resolution.observation().addressing_mode(),
            AddressingMode::Proxy
        );
        assert_eq!(resolution.resolved_move(), ResolvedMoveId::TinkerGrip);
        assert!(resolution.recipe_legal());
        assert_eq!(
            resolution
                .recipe()
                .expect("recipe should exist")
                .recipe_id(),
            "gremlin_tinker"
        );
        assert!(resolution.execution().is_some());
    }

    #[test]
    fn troglodyte_world_grip_preserves_the_same_skill_root_at_larger_scale() {
        let point = super::canonical_troglodyte_point();
        let action = canonical_troglodyte_proxy_action_fixture();
        let resolution =
            resolve_being_object_action(&point, &action).expect("resolution should succeed");

        assert_eq!(resolution.inherited_skill_root(), SkillId::Grip);
        assert_eq!(resolution.resolved_move(), ResolvedMoveId::WorldGrip);
        assert!(resolution.required_current() > 2);
        assert!(!resolution.recipe_legal());
    }

    #[test]
    fn moxy_repair_addresses_the_connected_system_without_auto_legality() {
        let point = super::canonical_gremlin_point();
        let action = canonical_moxy_repair_fixture();
        let resolution =
            resolve_being_object_action(&point, &action).expect("resolution should succeed");

        assert_eq!(
            resolution.observation().addressing_mode(),
            AddressingMode::Moxy
        );
        assert_eq!(
            resolution.observation().object().connections()[0].target(),
            ObjectId::ControlSystem
        );
        assert!(!resolution.recipe_legal());
    }

    #[test]
    fn foxy_repair_addresses_return_relation_without_auto_legality() {
        let point = super::canonical_gremlin_point();
        let action = canonical_foxy_repair_fixture();
        let resolution =
            resolve_being_object_action(&point, &action).expect("resolution should succeed");

        assert_eq!(
            resolution.observation().addressing_mode(),
            AddressingMode::Foxy
        );
        assert_eq!(resolution.resolved_move(), ResolvedMoveId::ReturnRepair);
        assert!(!resolution.recipe_legal());
    }

    #[test]
    fn hollow_object_refines_only_the_object_side() {
        let operation = canonical_hollow_object_fixture();
        assert!(matches!(
            operation.target(),
            HollowingTarget::Object(ObjectId::StoneObject)
        ));
    }

    #[test]
    fn hollow_being_refines_only_the_being_side() {
        let operation = canonical_hollow_being_fixture();
        assert!(matches!(
            operation.target(),
            HollowingTarget::Being(crate::BeingId::Hueman)
        ));
    }

    #[test]
    fn synthesis_fixture_represents_cross_boundary_transfer_without_bypassing_recipe() {
        let synthesis = canonical_cross_boundary_synthesis_fixture();
        assert!(!synthesis.transfer_rules().is_empty());
        assert!(synthesis.candidate_recipe().is_some());
    }

    #[test]
    fn canonical_being_object_contract_fixture_passes() {
        assert!(
            validate_being_object_contract(&canonical_being_object_contract_fixture()).is_empty()
        );
    }

    #[test]
    fn contradiction_fixtures_fail_with_explicit_messages() {
        let contradictions = [
            (
                "object collapsed",
                BeingObjectContractInput {
                    object_collapsed_into_being: true,
                    ..BeingObjectContractInput::default()
                },
                "independently addressable",
            ),
            (
                "skill relation missing",
                BeingObjectContractInput {
                    skill_without_relation: true,
                    ..BeingObjectContractInput::default()
                },
                "practiced relation between Being and Object",
            ),
            (
                "move identical to skill",
                BeingObjectContractInput {
                    move_identical_to_skill: true,
                    ..BeingObjectContractInput::default()
                },
                "Move cannot be treated as identical to Skill",
            ),
            (
                "hollowing crosses boundary",
                BeingObjectContractInput {
                    hollowing_crosses_boundary: true,
                    ..BeingObjectContractInput::default()
                },
                "Hollowing refines one side",
            ),
            (
                "synthesis bypasses legality",
                BeingObjectContractInput {
                    synthesis_bypasses_recipe_legality: true,
                    ..BeingObjectContractInput::default()
                },
                "cannot bypass Recipe legality",
            ),
            (
                "proxy replaces object",
                BeingObjectContractInput {
                    proxy_replaces_object: true,
                    ..BeingObjectContractInput::default()
                },
                "Proxy is an addressing mode",
            ),
            (
                "moxy velocity only",
                BeingObjectContractInput {
                    moxy_velocity_only: true,
                    ..BeingObjectContractInput::default()
                },
                "cannot be reduced to velocity only",
            ),
            (
                "foxy auto evil",
                BeingObjectContractInput {
                    foxy_automatically_evil: true,
                    ..BeingObjectContractInput::default()
                },
                "Foxy cannot automatically mean evil",
            ),
            (
                "object history discarded",
                BeingObjectContractInput {
                    object_mutation_without_identity_or_history: true,
                    ..BeingObjectContractInput::default()
                },
                "must preserve Object identity and history",
            ),
            (
                "hueman identity erased",
                BeingObjectContractInput {
                    erases_hueman_identity: true,
                    ..BeingObjectContractInput::default()
                },
                "cannot erase persistent Hueman identity",
            ),
            (
                "inheritance discarded",
                BeingObjectContractInput {
                    inherited_skill_roots_discarded: true,
                    ..BeingObjectContractInput::default()
                },
                "must preserve practiced Skill roots",
            ),
            (
                "idle time progression",
                BeingObjectContractInput {
                    idle_time_grants_major_progression: true,
                    ..BeingObjectContractInput::default()
                },
                "Idle elapsed time cannot grant major embodiment progression",
            ),
        ];

        for (label, input, expected) in contradictions {
            let diagnostics = validate_being_object_contract(&input);
            assert!(!diagnostics.is_empty(), "{label} should fail");
            assert!(
                diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.message.contains(expected)),
                "{label} should mention `{expected}`"
            );
        }
    }

    #[test]
    fn witnesses_and_validation_surfaces_render() {
        let witness = build_being_object_witness().expect("being/object witness should build");
        assert!(witness.contains("HOLLOW GROVE BEING / OBJECT ONTOLOGY"));

        let report =
            build_being_object_validation_report().expect("being/object validation should build");
        assert!(report.contains("status: pass"));

        let move_witness = build_move_witness().expect("move witness should build");
        assert!(move_witness.contains("Resolved Move"));
        assert!(move_witness.contains("TinkerGrip"));
    }
}
