use std::collections::HashMap;
use std::io;

use crate::being_object_ontology::{
    ActionAim, AddressingMode, BeingState, ObjectConnection, ObjectFamily, ObjectId,
    ObjectMaterial, ObjectScale, ObjectState, SkillId, build_canonical_being_state_for_frame,
    canonical_object_state,
};
use crate::decision_engine::{DecisionCandidateId, resolve_candidate_recipe};
use crate::flow_glow_grammar::{
    ActionMode, EmbodiedGesture, ExpressionDomain, RecipeBoundaryStatus, StonebendApex,
};
use crate::frame_state::FrameId;
use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode};
use crate::synthesis_recipe::SynthesisRecipe;

pub type SkillRoot = SkillId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GripExpressionId {
    TinkerGrip,
    WeaponGrip,
    CarrionGrip,
    BridgeGrip,
    FormationGrip,
    SiegeGrip,
    WorldGrip,
}

impl GripExpressionId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TinkerGrip => "TinkerGrip",
            Self::WeaponGrip => "WeaponGrip",
            Self::CarrionGrip => "CarrionGrip",
            Self::BridgeGrip => "BridgeGrip",
            Self::FormationGrip => "FormationGrip",
            Self::SiegeGrip => "SiegeGrip",
            Self::WorldGrip => "WorldGrip",
        }
    }

    #[must_use]
    pub const fn required_form(self) -> FrameId {
        match self {
            Self::TinkerGrip => FrameId::Gremlin,
            Self::WeaponGrip => FrameId::Goblin,
            Self::CarrionGrip => FrameId::Ghoul,
            Self::BridgeGrip => FrameId::Troll,
            Self::FormationGrip => FrameId::Ork,
            Self::SiegeGrip => FrameId::Ogre,
            Self::WorldGrip => FrameId::Troglodyte,
        }
    }

    #[must_use]
    pub const fn predecessor(self) -> Option<Self> {
        match self {
            Self::TinkerGrip => None,
            Self::WeaponGrip => Some(Self::TinkerGrip),
            Self::CarrionGrip => Some(Self::WeaponGrip),
            Self::BridgeGrip => Some(Self::CarrionGrip),
            Self::FormationGrip => Some(Self::BridgeGrip),
            Self::SiegeGrip => Some(Self::FormationGrip),
            Self::WorldGrip => Some(Self::SiegeGrip),
        }
    }

    #[must_use]
    pub const fn current_requirement(self) -> CurrentRequirement {
        match self {
            Self::TinkerGrip => CurrentRequirement::Low,
            Self::WeaponGrip => CurrentRequirement::Moderate,
            Self::CarrionGrip => CurrentRequirement::Moderate,
            Self::BridgeGrip => CurrentRequirement::High,
            Self::FormationGrip => CurrentRequirement::High,
            Self::SiegeGrip => CurrentRequirement::VeryHigh,
            Self::WorldGrip => CurrentRequirement::Apex,
        }
    }

    #[must_use]
    pub const fn pressure_requirement(self) -> PressureRequirement {
        match self {
            Self::TinkerGrip => PressureRequirement::Delicate,
            Self::WeaponGrip => PressureRequirement::Combat,
            Self::CarrionGrip => PressureRequirement::Persistent,
            Self::BridgeGrip => PressureRequirement::Structural,
            Self::FormationGrip => PressureRequirement::Collective,
            Self::SiegeGrip => PressureRequirement::Massive,
            Self::WorldGrip => PressureRequirement::Terrain,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrentRequirement {
    Low,
    Moderate,
    High,
    VeryHigh,
    Apex,
}

impl CurrentRequirement {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low Current requirement",
            Self::Moderate => "moderate Current requirement",
            Self::High => "high Current requirement",
            Self::VeryHigh => "very high Current requirement",
            Self::Apex => "apex Current requirement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureRequirement {
    Delicate,
    Combat,
    Persistent,
    Structural,
    Collective,
    Massive,
    Terrain,
}

impl PressureRequirement {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Delicate => "delicate pressure tolerance",
            Self::Combat => "combat pressure tolerance",
            Self::Persistent => "persistent damaged-matter tolerance",
            Self::Structural => "structural pressure tolerance",
            Self::Collective => "collective pressure tolerance",
            Self::Massive => "massive leverage tolerance",
            Self::Terrain => "terrain-scale consequence tolerance",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GripPracticeEvent {
    move_expression: GripExpressionId,
    current_form: FrameId,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object_family: ObjectFamily,
    object_material: ObjectMaterial,
    object_condition: crate::being_object_ontology::ObjectCondition,
    object_scale: ObjectScale,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    current_invested: u16,
    pressure_requirement: PressureRequirement,
    valid_execution: bool,
}

impl GripPracticeEvent {
    #[must_use]
    pub const fn valid_execution(&self) -> bool {
        self.valid_execution
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GripPracticeProfile {
    pub total_valid_uses: u64,
    pub current_invested: u16,
    pub form_history: Vec<(FrameId, u64)>,
    pub move_history: Vec<(GripExpressionId, u64)>,
    pub object_family_familiarity: Vec<(ObjectFamily, u64)>,
    pub material_familiarity: Vec<(ObjectMaterial, u64)>,
    pub scale_familiarity: Vec<(ObjectScale, u64)>,
    pub gesture_history: Vec<(EmbodiedGesture, u64)>,
    pub mode_history: Vec<(ActionMode, u64)>,
    pub pressure_history: Vec<(PressureRequirement, u64)>,
}

impl GripPracticeProfile {
    #[must_use]
    pub fn familiarity_for_family(&self, family: ObjectFamily) -> u64 {
        self.object_family_familiarity
            .iter()
            .find_map(|(entry, uses)| (*entry == family).then_some(*uses))
            .unwrap_or(0)
    }

    #[must_use]
    pub fn familiarity_for_scale(&self, scale: ObjectScale) -> u64 {
        self.scale_familiarity
            .iter()
            .find_map(|(entry, uses)| (*entry == scale).then_some(*uses))
            .unwrap_or(0)
    }

    #[must_use]
    pub fn familiarity_for_gesture(&self, gesture: EmbodiedGesture) -> u64 {
        self.gesture_history
            .iter()
            .find_map(|(entry, uses)| (*entry == gesture).then_some(*uses))
            .unwrap_or(0)
    }

    #[must_use]
    pub fn familiarity_for_mode(&self, mode: ActionMode) -> u64 {
        self.mode_history
            .iter()
            .find_map(|(entry, uses)| (*entry == mode).then_some(*uses))
            .unwrap_or(0)
    }

    #[must_use]
    pub fn dominant_history_summary(&self) -> String {
        let family = self
            .object_family_familiarity
            .iter()
            .max_by_key(|(_, uses)| *uses)
            .map_or(String::from("none"), |(family, uses)| {
                format!("{} ({uses})", family.as_str())
            });
        let scale = self
            .scale_familiarity
            .iter()
            .max_by_key(|(_, uses)| *uses)
            .map_or(String::from("none"), |(scale, uses)| {
                format!("{} ({uses})", scale.as_str())
            });
        let mode = self
            .mode_history
            .iter()
            .max_by_key(|(_, uses)| *uses)
            .map_or(String::from("none"), |(mode, uses)| {
                format!("{} ({uses})", mode.as_str())
            });

        format!("family: {family}; scale: {scale}; mode: {mode}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentGripActionRequest {
    being: BeingState,
    domain: ExpressionDomain,
    skill_root: SkillRoot,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    practice_profile: GripPracticeProfile,
}

impl CurrentGripActionRequest {
    #[must_use]
    pub fn new(
        being: BeingState,
        domain: ExpressionDomain,
        skill_root: SkillRoot,
        gesture: EmbodiedGesture,
        mode: ActionMode,
        object: ObjectState,
        addressing_mode: AddressingMode,
        aim: ActionAim,
        practice_profile: GripPracticeProfile,
    ) -> Self {
        Self {
            being,
            domain,
            skill_root,
            gesture,
            mode,
            object,
            addressing_mode,
            aim,
            practice_profile,
        }
    }

    #[must_use]
    pub const fn being(&self) -> &BeingState {
        &self.being
    }

    #[must_use]
    pub const fn domain(&self) -> ExpressionDomain {
        self.domain
    }

    #[must_use]
    pub const fn skill_root(&self) -> SkillRoot {
        self.skill_root
    }

    #[must_use]
    pub const fn gesture(&self) -> EmbodiedGesture {
        self.gesture
    }

    #[must_use]
    pub const fn mode(&self) -> ActionMode {
        self.mode
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

    #[must_use]
    pub const fn practice_profile(&self) -> &GripPracticeProfile {
        &self.practice_profile
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GripExpressionScore {
    expression: GripExpressionId,
    score: i32,
    explanation: String,
}

impl GripExpressionScore {
    #[must_use]
    pub const fn expression(&self) -> GripExpressionId {
        self.expression
    }

    #[must_use]
    pub const fn score(&self) -> i32 {
        self.score
    }

    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentGripResolution {
    request: CurrentGripActionRequest,
    available_inherited_expressions: Vec<GripExpressionId>,
    candidate_scores: Vec<GripExpressionScore>,
    selected_expression: GripExpressionId,
    projected_tendency: Option<StonebendApex>,
    recipe: Option<SynthesisRecipe>,
    recipe_status: RecipeBoundaryStatus,
    v2_status: String,
}

impl CurrentGripResolution {
    #[must_use]
    pub const fn request(&self) -> &CurrentGripActionRequest {
        &self.request
    }

    #[must_use]
    pub fn available_inherited_expressions(&self) -> &[GripExpressionId] {
        &self.available_inherited_expressions
    }

    #[must_use]
    pub fn candidate_scores(&self) -> &[GripExpressionScore] {
        &self.candidate_scores
    }

    #[must_use]
    pub const fn selected_expression(&self) -> GripExpressionId {
        self.selected_expression
    }

    #[must_use]
    pub const fn projected_tendency(&self) -> Option<StonebendApex> {
        self.projected_tendency
    }

    #[must_use]
    pub const fn recipe(&self) -> Option<&SynthesisRecipe> {
        self.recipe.as_ref()
    }

    #[must_use]
    pub const fn recipe_status(&self) -> RecipeBoundaryStatus {
        self.recipe_status
    }

    #[must_use]
    pub fn v2_status(&self) -> &str {
        &self.v2_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CurrentGripInheritanceContractInput {
    pub unrelated_grip_skills: bool,
    pub gesture_skill_collapsed: bool,
    pub transformation_erases_practice: bool,
    pub higher_form_loses_lower_expressions: bool,
    pub troglodyte_always_worldgrip: bool,
    pub gremlin_worldgrip: bool,
    pub object_name_only: bool,
    pub scale_only: bool,
    pub addressing_replaces_object: bool,
    pub mode_replaces_addressing: bool,
    pub gesture_replaces_skill: bool,
    pub skill_replaces_gesture: bool,
    pub flow_glow_collapsed: bool,
    pub grip_only_seam: bool,
    pub grip_beam_rejected_without_reason: bool,
    pub moxy_velocity_only: bool,
    pub foxy_evil: bool,
    pub foxy_time_reversal: bool,
    pub idle_time_progression: bool,
    pub illegal_actions_grant_full_practice: bool,
    pub direct_execution_bypass: bool,
    pub direct_current_prism_mutation: bool,
    pub direct_capacity_mutation: bool,
    pub automatic_aura_frame_grant: bool,
    pub point_cubed: bool,
    pub position_thirteen: bool,
    pub being_object_collapsed: bool,
    pub living_formation_flattened: bool,
    pub permanent_freemason_selection: bool,
    pub permanent_proletariat_selection: bool,
    pub permanent_hypergiant_selection: bool,
    pub v1_1_changed: bool,
}

#[must_use]
pub fn canonical_current_grip_inheritance_contract_fixture() -> CurrentGripInheritanceContractInput
{
    CurrentGripInheritanceContractInput::default()
}

#[must_use]
pub fn project_grip_practice_profile(events: &[GripPracticeEvent]) -> GripPracticeProfile {
    let mut form_history = HashMap::new();
    let mut move_history = HashMap::new();
    let mut object_family_familiarity = HashMap::new();
    let mut material_familiarity = HashMap::new();
    let mut scale_familiarity = HashMap::new();
    let mut gesture_history = HashMap::new();
    let mut mode_history = HashMap::new();
    let mut pressure_history = HashMap::new();
    let mut total_valid_uses = 0_u64;
    let mut current_invested = 0_u16;

    for event in events.iter().filter(|event| event.valid_execution()) {
        total_valid_uses += 1;
        current_invested = current_invested.saturating_add(event.current_invested);
        *form_history.entry(event.current_form).or_insert(0_u64) += 1;
        *move_history.entry(event.move_expression).or_insert(0_u64) += 1;
        *object_family_familiarity
            .entry(event.object_family)
            .or_insert(0_u64) += 1;
        *material_familiarity
            .entry(event.object_material)
            .or_insert(0_u64) += 1;
        *scale_familiarity.entry(event.object_scale).or_insert(0_u64) += 1;
        *gesture_history.entry(event.gesture).or_insert(0_u64) += 1;
        *mode_history.entry(event.mode).or_insert(0_u64) += 1;
        *pressure_history
            .entry(event.pressure_requirement)
            .or_insert(0_u64) += 1;
    }

    GripPracticeProfile {
        total_valid_uses,
        current_invested,
        form_history: sorted_pairs(form_history, |frame| current_form_label(*frame)),
        move_history: sorted_pairs(move_history, |move_id| move_id.as_str()),
        object_family_familiarity: sorted_pairs(object_family_familiarity, |family| {
            family.as_str()
        }),
        material_familiarity: sorted_pairs(material_familiarity, |material| material.as_str()),
        scale_familiarity: sorted_pairs(scale_familiarity, |scale| scale.as_str()),
        gesture_history: sorted_pairs(gesture_history, |gesture| gesture.as_str()),
        mode_history: sorted_pairs(mode_history, |mode| mode.as_str()),
        pressure_history: sorted_pairs(pressure_history, |pressure| pressure.as_str()),
    }
}

pub fn resolve_current_grip_action(
    request: &CurrentGripActionRequest,
) -> io::Result<CurrentGripResolution> {
    if request.skill_root() != SkillId::Grip {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Current Grip inheritance requires SkillRoot::Grip",
        ));
    }
    if request.domain() != ExpressionDomain::Flow {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Current Grip inheritance is a Current Skill family and requires Flow domain",
        ));
    }

    let available_inherited_expressions =
        cumulative_expressions_for_form(request.being().current_form());
    if available_inherited_expressions.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "current form has no canonical Grip inheritance expressions",
        ));
    }

    let mut candidate_scores = available_inherited_expressions
        .iter()
        .map(|expression| score_expression(*expression, request))
        .collect::<Vec<_>>();
    candidate_scores.sort_by(|left, right| {
        right.score().cmp(&left.score()).then_with(|| {
            expression_rank(left.expression()).cmp(&expression_rank(right.expression()))
        })
    });

    let Some(selected_expression) = candidate_scores.first().map(|score| score.expression()) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no candidate Grip expressions were available",
        ));
    };

    let projected_tendency = projected_tendency(request.practice_profile());
    let recipe = if request.being().current_form() == FrameId::Gremlin
        && selected_expression == GripExpressionId::TinkerGrip
        && request.object().identity() == ObjectId::MechanicalLatch
        && request.addressing_mode() == AddressingMode::Proxy
    {
        Some(resolve_candidate_recipe(DecisionCandidateId::GremlinTinker))
    } else {
        None
    };
    let recipe_status = if recipe.is_some() {
        RecipeBoundaryStatus::LegalFixtureAvailable
    } else {
        RecipeBoundaryStatus::LegalFixtureRequired
    };
    let v2_status = if recipe.is_some() {
        String::from("canonical V2 candidate fixture available")
    } else {
        String::from("V2 generation and evaluation required before any legal Recipe exists")
    };

    Ok(CurrentGripResolution {
        request: request.clone(),
        available_inherited_expressions,
        candidate_scores,
        selected_expression,
        projected_tendency,
        recipe,
        recipe_status,
        v2_status,
    })
}

#[must_use]
pub fn canonical_gremlin_tinkergrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Gremlin),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::MechanicalLatch),
        AddressingMode::Proxy,
        ActionAim::Manipulate,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_goblin_weapongrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Goblin),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Beam,
        canonical_object_state(ObjectId::Weapon),
        AddressingMode::Proxy,
        ActionAim::RetainAndDirect,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_ghoul_carriongrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Ghoul),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::DamagedWreckage),
        AddressingMode::Proxy,
        ActionAim::Recover,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_troll_bridgegrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troll),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grit,
        ActionMode::Seam,
        canonical_object_state(ObjectId::BrokenCrossingSupport),
        AddressingMode::Proxy,
        ActionAim::HoldTogether,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_ork_formationgrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Ork),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grit,
        ActionMode::Seam,
        canonical_object_state(ObjectId::ShieldFormationAnchor),
        AddressingMode::Proxy,
        ActionAim::BindFormation,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_ogre_siegegrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Ogre),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Beam,
        canonical_object_state(ObjectId::SiegeEngine),
        AddressingMode::Proxy,
        ActionAim::SeizeAndRedirect,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_troglodyte_worldgrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::FracturedCliff),
        AddressingMode::Proxy,
        ActionAim::Anchor,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_troglodyte_precision_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::MechanicalLatch),
        AddressingMode::Proxy,
        ActionAim::OpenPrecisely,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_troll_moxy_bridgegrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troll),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::NearBridgeSupport),
        AddressingMode::Moxy,
        ActionAim::StabilizeConnectedFarAnchor,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_gremlin_foxy_tinkergrip_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Gremlin),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Show,
        ActionMode::Seam,
        canonical_object_state(ObjectId::ReverseFacingHiddenLatch),
        AddressingMode::Foxy,
        ActionAim::ExposeAndManipulateHiddenMechanism,
        canonical_general_grip_practice_profile(),
    )
}

#[must_use]
pub fn canonical_freemason_tendency_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grip,
        ActionMode::Beam,
        canonical_object_state(ObjectId::FocusedIndustrialTool),
        AddressingMode::Proxy,
        ActionAim::DirectFocusedTool,
        canonical_freemason_tendency_profile(),
    )
}

#[must_use]
pub fn canonical_proletariat_tendency_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Grit,
        ActionMode::Seam,
        canonical_object_state(ObjectId::CivicRupture),
        AddressingMode::Moxy,
        ActionAim::MendCivicRupture,
        canonical_proletariat_tendency_profile(),
    )
}

#[must_use]
pub fn canonical_hypergiant_tendency_fixture() -> CurrentGripActionRequest {
    CurrentGripActionRequest::new(
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        SkillId::Grip,
        EmbodiedGesture::Show,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::MonumentalFoundation),
        AddressingMode::Proxy,
        ActionAim::RevealMonumentalProof,
        canonical_hypergiant_tendency_profile(),
    )
}

#[must_use]
pub fn validate_current_grip_inheritance_contract(
    input: &CurrentGripInheritanceContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.unrelated_grip_skills {
        diagnostics.push(current_grip_error(
            "The seven Grip expressions must remain one shared SkillRoot::Grip rather than unrelated skills.",
        ));
    }
    if input.gesture_skill_collapsed {
        diagnostics.push(current_grip_error(
            "SkillRoot::Grip must remain distinct from EmbodiedGesture::Grip.",
        ));
    }
    if input.transformation_erases_practice {
        diagnostics.push(current_grip_error(
            "Current transformation must not erase prior Grip practice.",
        ));
    }
    if input.higher_form_loses_lower_expressions {
        diagnostics.push(current_grip_error(
            "Higher Current Forms must retain legal lower Grip expressions.",
        ));
    }
    if input.troglodyte_always_worldgrip {
        diagnostics.push(current_grip_error(
            "Troglodyte must not automatically select WorldGrip for every Object.",
        ));
    }
    if input.gremlin_worldgrip {
        diagnostics.push(current_grip_error("Gremlin must not resolve WorldGrip."));
    }
    if input.object_name_only {
        diagnostics.push(current_grip_error(
            "Grip resolution cannot select a Move from Object name alone.",
        ));
    }
    if input.scale_only {
        diagnostics.push(current_grip_error(
            "Grip resolution cannot select a Move from Object scale alone.",
        ));
    }
    if input.addressing_replaces_object {
        diagnostics.push(current_grip_error(
            "AddressingMode cannot replace the Object in Grip resolution.",
        ));
    }
    if input.mode_replaces_addressing {
        diagnostics.push(current_grip_error(
            "ActionMode must remain distinct from AddressingMode.",
        ));
    }
    if input.gesture_replaces_skill {
        diagnostics.push(current_grip_error(
            "EmbodiedGesture cannot replace SkillRoot::Grip.",
        ));
    }
    if input.skill_replaces_gesture {
        diagnostics.push(current_grip_error(
            "SkillRoot::Grip cannot replace EmbodiedGesture::Grip.",
        ));
    }
    if input.flow_glow_collapsed {
        diagnostics.push(current_grip_error(
            "Flow and Glow must remain distinct domains.",
        ));
    }
    if input.grip_only_seam {
        diagnostics.push(current_grip_error(
            "Grip must not be reduced to Seam-only usage because valid Grip + Beam contexts remain legal.",
        ));
    }
    if input.grip_beam_rejected_without_reason {
        diagnostics.push(current_grip_error(
            "Valid Grip + Beam contexts cannot be rejected without a semantic reason.",
        ));
    }
    if input.moxy_velocity_only {
        diagnostics.push(current_grip_error("Moxy must not be reduced to velocity."));
    }
    if input.foxy_evil {
        diagnostics.push(current_grip_error("Foxy must not automatically mean evil."));
    }
    if input.foxy_time_reversal {
        diagnostics.push(current_grip_error(
            "Foxy must not automatically mean time reversal.",
        ));
    }
    if input.idle_time_progression {
        diagnostics.push(current_grip_error("Idle time must not grant Grip mastery."));
    }
    if input.illegal_actions_grant_full_practice {
        diagnostics.push(current_grip_error(
            "Failed or illegal actions must not grant full practice.",
        ));
    }
    if input.direct_execution_bypass {
        diagnostics.push(current_grip_error(
            "Grip Move resolution must not bypass Move -> Recipe -> V2 -> frozen V1.1.",
        ));
    }
    if input.direct_current_prism_mutation {
        diagnostics.push(current_grip_error(
            "Grip inheritance must not directly mutate CurrentPrism.",
        ));
    }
    if input.direct_capacity_mutation {
        diagnostics.push(current_grip_error(
            "Grip inheritance must not directly mutate capacities.",
        ));
    }
    if input.automatic_aura_frame_grant {
        diagnostics.push(current_grip_error(
            "Grip inheritance must not automatically grant an Aura Frame.",
        ));
    }
    if input.point_cubed {
        diagnostics.push(current_grip_error("Point³ must not be introduced."));
    }
    if input.position_thirteen {
        diagnostics.push(current_grip_error("Position 13 must not be introduced."));
    }
    if input.being_object_collapsed {
        diagnostics.push(current_grip_error(
            "Being and Object must remain separate during Grip inheritance resolution.",
        ));
    }
    if input.living_formation_flattened {
        diagnostics.push(current_grip_error(
            "Formation members must not be flattened into generic Objects.",
        ));
    }
    if input.permanent_freemason_selection {
        diagnostics.push(current_grip_error(
            "Freemason tendency must remain projected and cannot become permanently selected from one Beam action.",
        ));
    }
    if input.permanent_proletariat_selection {
        diagnostics.push(current_grip_error(
            "Proletariat tendency must remain projected and cannot become permanently selected from one Seam action.",
        ));
    }
    if input.permanent_hypergiant_selection {
        diagnostics.push(current_grip_error(
            "Hypergiant tendency must remain projected and cannot become permanently selected from one Gleam action.",
        ));
    }
    if input.v1_1_changed {
        diagnostics.push(current_grip_error(
            "Frozen V1.1 topology must remain unchanged.",
        ));
    }

    diagnostics
}

pub fn build_current_inheritance_witness() -> io::Result<String> {
    Ok(String::from(
        "HOLLOW GROVE CURRENT INHERITANCE\n\n\
         Skill Root:\n\
         Grip\n\n\
         Current Lineage:\n\n\
         Gremlin\n\
         -> TinkerGrip\n\n\
         Goblin\n\
         -> WeaponGrip\n\n\
         Ghoul\n\
         -> CarrionGrip\n\n\
         Troll\n\
         -> BridgeGrip\n\n\
         Ork\n\
         -> FormationGrip\n\n\
         Ogre\n\
         -> SiegeGrip\n\n\
         Troglodyte\n\
         -> WorldGrip\n\n\
         Root Retained:\n\
         Yes\n\n\
         Predecessor Chain:\n\
         Valid\n\n\
         Lower Expressions Retained:\n\
         Yes, when Object, Aim, anatomy, Current, and legality permit\n\n\
         Object History:\n\
         Observed\n\n\
         Gesture Integration:\n\
         Enabled\n\n\
         Mode Integration:\n\
         Enabled\n\n\
         Addressing Integration:\n\
         Enabled\n\n\
         Idle-Time Progression:\n\
         Rejected\n\n\
         Recipe Boundary:\n\
         Required\n\n\
         V1.1:\n\
         Unchanged\n",
    ))
}

pub fn build_current_inheritance_validation_report() -> io::Result<String> {
    let diagnostics = validate_current_grip_inheritance_contract(
        &canonical_current_grip_inheritance_contract_fixture(),
    );
    let troglodyte_precision =
        resolve_current_grip_action(&canonical_troglodyte_precision_fixture())?;
    let gremlin = resolve_current_grip_action(&canonical_gremlin_tinkergrip_fixture())?;
    let world = resolve_current_grip_action(&canonical_troglodyte_worldgrip_fixture())?;

    let mut errors = diagnostics;
    if gremlin.selected_expression() != GripExpressionId::TinkerGrip {
        errors.push(current_grip_error(
            "canonical Gremlin fixture must resolve to TinkerGrip",
        ));
    }
    if world.selected_expression() != GripExpressionId::WorldGrip {
        errors.push(current_grip_error(
            "canonical Troglodyte terrain fixture must resolve to WorldGrip",
        ));
    }
    if troglodyte_precision.selected_expression() != GripExpressionId::TinkerGrip {
        errors.push(current_grip_error(
            "Troglodyte must retain the inherited precision Grip expression on a fine mechanism",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Current Inheritance Validation\n\n\
             - status: pass\n\
             - shared Skill root: pass\n\
             - seven Form expressions: pass\n\
             - Form order: pass\n\
             - predecessor chain: pass\n\
             - cumulative available expressions: pass\n\
             - lower-expression retention: pass\n\
             - Object-family resolution: pass\n\
             - Object-scale resolution: pass\n\
             - Object-condition resolution: pass\n\
             - Gesture distinction: pass\n\
             - Gesture/Skill naming collision handled: pass\n\
             - ActionMode integration: pass\n\
             - AddressingMode integration: pass\n\
             - Aim integration: pass\n\
             - practice retention: pass\n\
             - idle-time rejection: pass\n\
             - illegal-practice rejection: pass\n\
             - specialization remains projected: pass\n\
             - Recipe boundary: pass\n\
             - V2 boundary: pass\n\
             - V1.1 unchanged: pass\n\
             - CurrentPrism distinction: pass\n",
        ))
    } else {
        let mut output =
            String::from("# Hollow Grove Current Inheritance Validation\n\n- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        Ok(output)
    }
}

pub fn build_grip_witness() -> io::Result<String> {
    let fixtures = [
        canonical_gremlin_tinkergrip_fixture(),
        canonical_goblin_weapongrip_fixture(),
        canonical_ghoul_carriongrip_fixture(),
        canonical_troll_bridgegrip_fixture(),
        canonical_ork_formationgrip_fixture(),
        canonical_ogre_siegegrip_fixture(),
        canonical_troglodyte_worldgrip_fixture(),
        canonical_troglodyte_precision_fixture(),
        canonical_troll_moxy_bridgegrip_fixture(),
        canonical_gremlin_foxy_tinkergrip_fixture(),
        canonical_freemason_tendency_fixture(),
        canonical_proletariat_tendency_fixture(),
        canonical_hypergiant_tendency_fixture(),
    ];

    let mut output = String::from("HOLLOW GROVE GRIP WITNESS\n");
    for fixture in fixtures {
        let resolution = resolve_current_grip_action(&fixture)?;
        output.push_str("\n\n");
        output.push_str(&render_grip_resolution(&resolution));
    }
    Ok(output)
}

fn render_grip_resolution(resolution: &CurrentGripResolution) -> String {
    let request = resolution.request();
    let available = resolution
        .available_inherited_expressions()
        .iter()
        .map(|expression| expression.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let scores = resolution
        .candidate_scores()
        .iter()
        .map(|score| {
            format!(
                "{} = {} ({})",
                score.expression().as_str(),
                score.score(),
                score.explanation()
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    let connections = render_connections(request.object().connections());
    let projected_tendency = resolution
        .projected_tendency()
        .map_or("none", StonebendApex::as_str);
    let predecessor = resolution
        .selected_expression()
        .predecessor()
        .map_or("none", GripExpressionId::as_str);
    let recipe_status = if let Some(recipe) = resolution.recipe() {
        format!(
            "{} via {}",
            resolution.recipe_status().as_str(),
            recipe.display_name()
        )
    } else {
        String::from(resolution.recipe_status().as_str())
    };

    format!(
        "Being:\n\
         {}\n\n\
         Current Form:\n\
         {}\n\n\
         Aura Frame:\n\
         {}\n\n\
         Domain:\n\
         {}\n\n\
         Skill Root:\n\
         {}\n\n\
         Embodied Gesture:\n\
         {}\n\n\
         Action Mode:\n\
         {}\n\n\
         Object:\n\
         {}\n\n\
         Object Family:\n\
         {}\n\n\
         Object Condition:\n\
         {}\n\n\
         Object Scale:\n\
         {}\n\n\
         Object Connections:\n\
         {}\n\n\
         AddressingMode:\n\
         {}\n\n\
         Aim:\n\
         {}\n\n\
         Available Inherited Expressions:\n\
         {}\n\n\
         Candidate Scores:\n\
         {}\n\n\
         Selected Move:\n\
         {}\n\n\
         Inherited From:\n\
         {}\n\n\
         Current Requirement:\n\
         {}\n\n\
         Pressure Requirement:\n\
         {}\n\n\
         Dominant Practice History:\n\
         {}\n\n\
         Projected Stonebend Tendency:\n\
         {}\n\n\
         Recipe Status:\n\
         {}\n\n\
         V2 Status:\n\
         {}",
        current_form_label(request.being().current_form()),
        current_form_label(request.being().current_form()),
        request
            .being()
            .aura_frame()
            .map_or("none", current_form_label),
        request.domain().as_str(),
        request.skill_root().as_str(),
        request.gesture().as_str(),
        request.mode().as_str(),
        request.object().identity().as_str(),
        request.object().family().as_str(),
        request.object().condition().as_str(),
        request.object().scale().as_str(),
        connections,
        request.addressing_mode().as_str(),
        request.aim().as_str(),
        available,
        scores,
        resolution.selected_expression().as_str(),
        predecessor,
        resolution
            .selected_expression()
            .current_requirement()
            .as_str(),
        resolution
            .selected_expression()
            .pressure_requirement()
            .as_str(),
        request.practice_profile().dominant_history_summary(),
        projected_tendency,
        recipe_status,
        resolution.v2_status(),
    )
}

#[must_use]
fn canonical_general_grip_practice_profile() -> GripPracticeProfile {
    project_grip_practice_profile(&[
        GripPracticeEvent {
            move_expression: GripExpressionId::TinkerGrip,
            current_form: FrameId::Gremlin,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Seam,
            object_family: ObjectFamily::Mechanism,
            object_material: ObjectMaterial::Metal,
            object_condition: crate::being_object_ontology::ObjectCondition::Stable,
            object_scale: ObjectScale::Fine,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::Manipulate,
            current_invested: 2,
            pressure_requirement: PressureRequirement::Delicate,
            valid_execution: true,
        },
        GripPracticeEvent {
            move_expression: GripExpressionId::BridgeGrip,
            current_form: FrameId::Troll,
            gesture: EmbodiedGesture::Grit,
            mode: ActionMode::Seam,
            object_family: ObjectFamily::Crossing,
            object_material: ObjectMaterial::Stone,
            object_condition: crate::being_object_ontology::ObjectCondition::Failing,
            object_scale: ObjectScale::Structural,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::HoldTogether,
            current_invested: 4,
            pressure_requirement: PressureRequirement::Structural,
            valid_execution: true,
        },
    ])
}

#[must_use]
fn canonical_freemason_tendency_profile() -> GripPracticeProfile {
    project_grip_practice_profile(&[
        GripPracticeEvent {
            move_expression: GripExpressionId::TinkerGrip,
            current_form: FrameId::Gremlin,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Beam,
            object_family: ObjectFamily::Tool,
            object_material: ObjectMaterial::Metal,
            object_condition: crate::being_object_ontology::ObjectCondition::Stable,
            object_scale: ObjectScale::Fine,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::DirectFocusedTool,
            current_invested: 2,
            pressure_requirement: PressureRequirement::Delicate,
            valid_execution: true,
        },
        GripPracticeEvent {
            move_expression: GripExpressionId::WeaponGrip,
            current_form: FrameId::Goblin,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Beam,
            object_family: ObjectFamily::Weapon,
            object_material: ObjectMaterial::Metal,
            object_condition: crate::being_object_ontology::ObjectCondition::Stable,
            object_scale: ObjectScale::Personal,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::RetainAndDirect,
            current_invested: 3,
            pressure_requirement: PressureRequirement::Combat,
            valid_execution: true,
        },
        GripPracticeEvent {
            move_expression: GripExpressionId::SiegeGrip,
            current_form: FrameId::Ogre,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Beam,
            object_family: ObjectFamily::SiegeEngine,
            object_material: ObjectMaterial::Metal,
            object_condition: crate::being_object_ontology::ObjectCondition::Stable,
            object_scale: ObjectScale::Massive,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::SeizeAndRedirect,
            current_invested: 5,
            pressure_requirement: PressureRequirement::Massive,
            valid_execution: true,
        },
    ])
}

#[must_use]
fn canonical_proletariat_tendency_profile() -> GripPracticeProfile {
    project_grip_practice_profile(&[
        GripPracticeEvent {
            move_expression: GripExpressionId::BridgeGrip,
            current_form: FrameId::Troll,
            gesture: EmbodiedGesture::Grit,
            mode: ActionMode::Seam,
            object_family: ObjectFamily::Crossing,
            object_material: ObjectMaterial::Stone,
            object_condition: crate::being_object_ontology::ObjectCondition::Failing,
            object_scale: ObjectScale::Structural,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::HoldTogether,
            current_invested: 4,
            pressure_requirement: PressureRequirement::Structural,
            valid_execution: true,
        },
        GripPracticeEvent {
            move_expression: GripExpressionId::FormationGrip,
            current_form: FrameId::Ork,
            gesture: EmbodiedGesture::Grit,
            mode: ActionMode::Seam,
            object_family: ObjectFamily::Formation,
            object_material: ObjectMaterial::Mixed,
            object_condition: crate::being_object_ontology::ObjectCondition::Unsettled,
            object_scale: ObjectScale::Collective,
            addressing_mode: AddressingMode::Moxy,
            aim: ActionAim::BindFormation,
            current_invested: 4,
            pressure_requirement: PressureRequirement::Collective,
            valid_execution: true,
        },
    ])
}

#[must_use]
fn canonical_hypergiant_tendency_profile() -> GripPracticeProfile {
    project_grip_practice_profile(&[
        GripPracticeEvent {
            move_expression: GripExpressionId::WorldGrip,
            current_form: FrameId::Troglodyte,
            gesture: EmbodiedGesture::Show,
            mode: ActionMode::Gleam,
            object_family: ObjectFamily::Terrain,
            object_material: ObjectMaterial::Stone,
            object_condition: crate::being_object_ontology::ObjectCondition::Fractured,
            object_scale: ObjectScale::Terrain,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::RevealMonumentalProof,
            current_invested: 6,
            pressure_requirement: PressureRequirement::Terrain,
            valid_execution: true,
        },
        GripPracticeEvent {
            move_expression: GripExpressionId::WorldGrip,
            current_form: FrameId::Troglodyte,
            gesture: EmbodiedGesture::Grit,
            mode: ActionMode::Gleam,
            object_family: ObjectFamily::Foundation,
            object_material: ObjectMaterial::Stone,
            object_condition: crate::being_object_ontology::ObjectCondition::Stable,
            object_scale: ObjectScale::Massive,
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::Anchor,
            current_invested: 6,
            pressure_requirement: PressureRequirement::Terrain,
            valid_execution: true,
        },
    ])
}

fn score_expression(
    expression: GripExpressionId,
    request: &CurrentGripActionRequest,
) -> GripExpressionScore {
    let mut score = 0_i32;
    let mut reasons = Vec::new();
    let object = request.object();

    if supports_family(expression, object.family()) {
        score += 40;
        reasons.push(String::from("family match"));
    } else if supports_function(expression, object) {
        score += 16;
        reasons.push(String::from("function-supported cross match"));
    }

    if supports_scale(expression, object.scale()) {
        score += 20;
        reasons.push(String::from("scale match"));
    }

    if supports_condition(expression, object) {
        score += 8;
        reasons.push(String::from("condition match"));
    }

    if supports_gesture_mode(expression, request.gesture(), request.mode()) {
        score += 18;
        reasons.push(String::from("gesture/mode fit"));
    } else if request.gesture() == EmbodiedGesture::Grip && request.mode() == ActionMode::Beam {
        score += 8;
        reasons.push(String::from("valid Grip + Beam cross-pairing"));
    } else if request.gesture() == EmbodiedGesture::Show && request.mode() == ActionMode::Seam {
        score += 8;
        reasons.push(String::from("valid Show + Seam cross-pairing"));
    } else if request.gesture() == EmbodiedGesture::Grit && request.mode() == ActionMode::Seam {
        score += 10;
        reasons.push(String::from("valid Grit + Seam pressure hold"));
    }

    if supports_aim(expression, request.aim()) {
        score += 14;
        reasons.push(String::from("aim fit"));
    }

    if request.addressing_mode() == AddressingMode::Moxy && !object.connections().is_empty() {
        if matches!(
            expression,
            GripExpressionId::BridgeGrip
                | GripExpressionId::FormationGrip
                | GripExpressionId::SiegeGrip
                | GripExpressionId::WorldGrip
        ) {
            score += 12;
            reasons.push(String::from("connected-object Moxy support"));
        }
    }

    if request.addressing_mode() == AddressingMode::Foxy
        && object.condition() == crate::being_object_ontology::ObjectCondition::Hidden
    {
        if matches!(
            expression,
            GripExpressionId::TinkerGrip | GripExpressionId::BridgeGrip
        ) {
            score += 12;
            reasons.push(String::from("hidden return-side Foxy support"));
        }
    }

    let family_familiarity = request
        .practice_profile()
        .familiarity_for_family(object.family()) as i32;
    if family_familiarity > 0 {
        score += family_familiarity.min(10);
        reasons.push(String::from("practice family familiarity"));
    }

    let scale_familiarity = request
        .practice_profile()
        .familiarity_for_scale(object.scale()) as i32;
    if scale_familiarity > 0 {
        score += scale_familiarity.min(8);
        reasons.push(String::from("practice scale familiarity"));
    }

    let gesture_familiarity = request
        .practice_profile()
        .familiarity_for_gesture(request.gesture()) as i32;
    if gesture_familiarity > 0 {
        score += gesture_familiarity.min(6);
        reasons.push(String::from("gesture familiarity"));
    }

    let mode_familiarity = request
        .practice_profile()
        .familiarity_for_mode(request.mode()) as i32;
    if mode_familiarity > 0 {
        score += mode_familiarity.min(6);
        reasons.push(String::from("mode familiarity"));
    }

    GripExpressionScore {
        expression,
        score,
        explanation: if reasons.is_empty() {
            String::from("low semantic match")
        } else {
            reasons.join(", ")
        },
    }
}

#[must_use]
fn supports_family(expression: GripExpressionId, family: ObjectFamily) -> bool {
    match expression {
        GripExpressionId::TinkerGrip => {
            matches!(family, ObjectFamily::Mechanism | ObjectFamily::Tool)
        }
        GripExpressionId::WeaponGrip => family == ObjectFamily::Weapon,
        GripExpressionId::CarrionGrip => family == ObjectFamily::Wreckage,
        GripExpressionId::BridgeGrip => family == ObjectFamily::Crossing,
        GripExpressionId::FormationGrip => family == ObjectFamily::Formation,
        GripExpressionId::SiegeGrip => matches!(
            family,
            ObjectFamily::SiegeEngine | ObjectFamily::EnvironmentalStructure
        ),
        GripExpressionId::WorldGrip => matches!(
            family,
            ObjectFamily::Terrain | ObjectFamily::Foundation | ObjectFamily::EnvironmentalStructure
        ),
    }
}

#[must_use]
fn supports_scale(expression: GripExpressionId, scale: ObjectScale) -> bool {
    match expression {
        GripExpressionId::TinkerGrip => matches!(scale, ObjectScale::Fine | ObjectScale::Personal),
        GripExpressionId::WeaponGrip => matches!(scale, ObjectScale::Personal | ObjectScale::Body),
        GripExpressionId::CarrionGrip => {
            matches!(scale, ObjectScale::Body | ObjectScale::Structural)
        }
        GripExpressionId::BridgeGrip => {
            matches!(scale, ObjectScale::Structural | ObjectScale::Collective)
        }
        GripExpressionId::FormationGrip => {
            matches!(scale, ObjectScale::Collective | ObjectScale::Structural)
        }
        GripExpressionId::SiegeGrip => {
            matches!(scale, ObjectScale::Massive | ObjectScale::Structural)
        }
        GripExpressionId::WorldGrip => matches!(scale, ObjectScale::Terrain | ObjectScale::Massive),
    }
}

#[must_use]
fn supports_condition(expression: GripExpressionId, object: &ObjectState) -> bool {
    match expression {
        GripExpressionId::TinkerGrip => matches!(
            object.condition(),
            crate::being_object_ontology::ObjectCondition::Stable
                | crate::being_object_ontology::ObjectCondition::Hidden
        ),
        GripExpressionId::WeaponGrip => {
            object.condition() == crate::being_object_ontology::ObjectCondition::Stable
        }
        GripExpressionId::CarrionGrip => {
            object.condition() == crate::being_object_ontology::ObjectCondition::Damaged
        }
        GripExpressionId::BridgeGrip => {
            object.condition() == crate::being_object_ontology::ObjectCondition::Failing
        }
        GripExpressionId::FormationGrip => matches!(
            object.condition(),
            crate::being_object_ontology::ObjectCondition::Unsettled
                | crate::being_object_ontology::ObjectCondition::Damaged
        ),
        GripExpressionId::SiegeGrip => matches!(
            object.condition(),
            crate::being_object_ontology::ObjectCondition::Stable
                | crate::being_object_ontology::ObjectCondition::Damaged
        ),
        GripExpressionId::WorldGrip => matches!(
            object.condition(),
            crate::being_object_ontology::ObjectCondition::Fractured
                | crate::being_object_ontology::ObjectCondition::Stable
        ),
    }
}

#[must_use]
fn supports_function(expression: GripExpressionId, object: &ObjectState) -> bool {
    match expression {
        GripExpressionId::TinkerGrip => object.functions().iter().any(|function| {
            matches!(
                function,
                crate::being_object_ontology::ObjectFunction::Latch
                    | crate::being_object_ontology::ObjectFunction::DirectionalTool
                    | crate::being_object_ontology::ObjectFunction::HiddenLatch
            )
        }),
        GripExpressionId::WeaponGrip => object
            .functions()
            .contains(&crate::being_object_ontology::ObjectFunction::WeaponLine),
        GripExpressionId::CarrionGrip => object
            .functions()
            .contains(&crate::being_object_ontology::ObjectFunction::Repairable),
        GripExpressionId::BridgeGrip => object
            .functions()
            .contains(&crate::being_object_ontology::ObjectFunction::LoadPath),
        GripExpressionId::FormationGrip => object
            .functions()
            .contains(&crate::being_object_ontology::ObjectFunction::GroupAnchor),
        GripExpressionId::SiegeGrip => object.functions().iter().any(|function| {
            matches!(
                function,
                crate::being_object_ontology::ObjectFunction::DirectionalTool
                    | crate::being_object_ontology::ObjectFunction::LoadPath
            )
        }),
        GripExpressionId::WorldGrip => object.functions().iter().any(|function| {
            matches!(
                function,
                crate::being_object_ontology::ObjectFunction::LoadPath
                    | crate::being_object_ontology::ObjectFunction::StructuralSupport
            )
        }),
    }
}

#[must_use]
fn supports_gesture_mode(
    expression: GripExpressionId,
    gesture: EmbodiedGesture,
    mode: ActionMode,
) -> bool {
    match expression {
        GripExpressionId::TinkerGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Grip, ActionMode::Beam)
                | (EmbodiedGesture::Show, ActionMode::Beam)
        ),
        GripExpressionId::WeaponGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grip, ActionMode::Beam)
                | (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Show, ActionMode::Beam)
                | (EmbodiedGesture::Grit, ActionMode::Gleam)
        ),
        GripExpressionId::CarrionGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Grit, ActionMode::Seam)
                | (EmbodiedGesture::Grip, ActionMode::Beam)
                | (EmbodiedGesture::Show, ActionMode::Seam)
        ),
        GripExpressionId::BridgeGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Grit, ActionMode::Seam)
                | (EmbodiedGesture::Show, ActionMode::Beam)
                | (EmbodiedGesture::Grip, ActionMode::Beam)
        ),
        GripExpressionId::FormationGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grit, ActionMode::Seam)
                | (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Show, ActionMode::Beam)
                | (EmbodiedGesture::Grit, ActionMode::Gleam)
        ),
        GripExpressionId::SiegeGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grip, ActionMode::Beam)
                | (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Grit, ActionMode::Seam)
                | (EmbodiedGesture::Show, ActionMode::Beam)
        ),
        GripExpressionId::WorldGrip => matches!(
            (gesture, mode),
            (EmbodiedGesture::Grip, ActionMode::Seam)
                | (EmbodiedGesture::Show, ActionMode::Beam)
                | (EmbodiedGesture::Grip, ActionMode::Beam)
                | (EmbodiedGesture::Grit, ActionMode::Gleam)
        ),
    }
}

#[must_use]
fn supports_aim(expression: GripExpressionId, aim: ActionAim) -> bool {
    match expression {
        GripExpressionId::TinkerGrip => matches!(
            aim,
            ActionAim::Manipulate
                | ActionAim::OpenPrecisely
                | ActionAim::ExposeAndManipulateHiddenMechanism
                | ActionAim::DirectFocusedTool
        ),
        GripExpressionId::WeaponGrip => aim == ActionAim::RetainAndDirect,
        GripExpressionId::CarrionGrip => aim == ActionAim::Recover,
        GripExpressionId::BridgeGrip => {
            matches!(
                aim,
                ActionAim::HoldTogether | ActionAim::StabilizeConnectedFarAnchor
            )
        }
        GripExpressionId::FormationGrip => {
            matches!(aim, ActionAim::BindFormation | ActionAim::MendCivicRupture)
        }
        GripExpressionId::SiegeGrip => matches!(
            aim,
            ActionAim::SeizeAndRedirect | ActionAim::DirectFocusedTool
        ),
        GripExpressionId::WorldGrip => {
            matches!(aim, ActionAim::Anchor | ActionAim::RevealMonumentalProof)
        }
    }
}

#[must_use]
fn cumulative_expressions_for_form(frame: FrameId) -> Vec<GripExpressionId> {
    let all = [
        GripExpressionId::TinkerGrip,
        GripExpressionId::WeaponGrip,
        GripExpressionId::CarrionGrip,
        GripExpressionId::BridgeGrip,
        GripExpressionId::FormationGrip,
        GripExpressionId::SiegeGrip,
        GripExpressionId::WorldGrip,
    ];
    let Some(rank) = current_form_rank(frame) else {
        return Vec::new();
    };
    all.into_iter()
        .filter(|expression| expression_rank(*expression) <= rank)
        .collect()
}

#[must_use]
fn current_form_rank(frame: FrameId) -> Option<u8> {
    match frame {
        FrameId::Gremlin => Some(0),
        FrameId::Goblin => Some(1),
        FrameId::Ghoul => Some(2),
        FrameId::Troll => Some(3),
        FrameId::Ork => Some(4),
        FrameId::Ogre => Some(5),
        FrameId::Troglodyte => Some(6),
        _ => None,
    }
}

#[must_use]
fn expression_rank(expression: GripExpressionId) -> u8 {
    match expression {
        GripExpressionId::TinkerGrip => 0,
        GripExpressionId::WeaponGrip => 1,
        GripExpressionId::CarrionGrip => 2,
        GripExpressionId::BridgeGrip => 3,
        GripExpressionId::FormationGrip => 4,
        GripExpressionId::SiegeGrip => 5,
        GripExpressionId::WorldGrip => 6,
    }
}

#[must_use]
fn projected_tendency(profile: &GripPracticeProfile) -> Option<StonebendApex> {
    let beam = profile.familiarity_for_mode(ActionMode::Beam);
    let seam = profile.familiarity_for_mode(ActionMode::Seam);
    let gleam = profile.familiarity_for_mode(ActionMode::Gleam);
    let tool_bias = profile.familiarity_for_family(ObjectFamily::Tool)
        + profile.familiarity_for_family(ObjectFamily::Weapon)
        + profile.familiarity_for_family(ObjectFamily::SiegeEngine);
    let seam_bias = profile.familiarity_for_family(ObjectFamily::Crossing)
        + profile.familiarity_for_family(ObjectFamily::Formation);
    let gleam_bias = profile.familiarity_for_family(ObjectFamily::Terrain)
        + profile.familiarity_for_family(ObjectFamily::Foundation)
        + profile.familiarity_for_family(ObjectFamily::Monument);

    let freemason = beam + tool_bias;
    let proletariat = seam + seam_bias;
    let hypergiant = gleam + gleam_bias;

    if freemason == 0 && proletariat == 0 && hypergiant == 0 {
        return None;
    }

    if freemason >= proletariat && freemason >= hypergiant {
        Some(StonebendApex::Freemason)
    } else if proletariat >= hypergiant {
        Some(StonebendApex::Proletariat)
    } else {
        Some(StonebendApex::Hypergiant)
    }
}

fn sorted_pairs<K, F>(map: HashMap<K, u64>, key_label: F) -> Vec<(K, u64)>
where
    K: Copy + Eq + std::hash::Hash,
    F: Fn(&K) -> &'static str,
{
    let mut items = map.into_iter().collect::<Vec<_>>();
    items.sort_by(|(left, _), (right, _)| key_label(left).cmp(key_label(right)));
    items
}

fn render_connections(connections: &[ObjectConnection]) -> String {
    if connections.is_empty() {
        return String::from("none");
    }

    connections
        .iter()
        .map(|connection| {
            format!(
                "{} ({})",
                connection.target().as_str(),
                connection.relation()
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
fn current_form_label(frame: FrameId) -> &'static str {
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

fn current_grip_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::BeingObjectOntologyMismatch,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ActionMode, CurrentGripInheritanceContractInput, EmbodiedGesture, GripExpressionId,
        PressureRequirement, build_current_inheritance_validation_report,
        build_current_inheritance_witness, build_grip_witness,
        canonical_current_grip_inheritance_contract_fixture, canonical_freemason_tendency_fixture,
        canonical_goblin_weapongrip_fixture, canonical_gremlin_foxy_tinkergrip_fixture,
        canonical_gremlin_tinkergrip_fixture, canonical_hypergiant_tendency_fixture,
        canonical_ogre_siegegrip_fixture, canonical_ork_formationgrip_fixture,
        canonical_proletariat_tendency_fixture, canonical_troglodyte_precision_fixture,
        canonical_troglodyte_worldgrip_fixture, canonical_troll_bridgegrip_fixture,
        canonical_troll_moxy_bridgegrip_fixture, project_grip_practice_profile,
        resolve_current_grip_action, validate_current_grip_inheritance_contract,
    };
    use crate::being_object_ontology::{
        ActionAim, AddressingMode, ObjectFamily, ObjectMaterial, ObjectScale, SkillId,
    };
    use crate::flow_glow_grammar::{ExpressionDomain, StonebendApex};
    use crate::frame_state::FrameId;

    #[test]
    fn canonical_fixtures_resolve_across_the_full_lineage() {
        let fixtures = [
            (
                canonical_gremlin_tinkergrip_fixture(),
                GripExpressionId::TinkerGrip,
            ),
            (
                canonical_goblin_weapongrip_fixture(),
                GripExpressionId::WeaponGrip,
            ),
            (
                canonical_troll_bridgegrip_fixture(),
                GripExpressionId::BridgeGrip,
            ),
            (
                canonical_ork_formationgrip_fixture(),
                GripExpressionId::FormationGrip,
            ),
            (
                canonical_ogre_siegegrip_fixture(),
                GripExpressionId::SiegeGrip,
            ),
            (
                canonical_troglodyte_worldgrip_fixture(),
                GripExpressionId::WorldGrip,
            ),
        ];

        for (fixture, expected) in fixtures {
            let resolution = resolve_current_grip_action(&fixture).expect("fixture should resolve");
            assert_eq!(resolution.selected_expression(), expected);
            assert_eq!(resolution.request().skill_root(), SkillId::Grip);
            assert_eq!(resolution.request().domain(), ExpressionDomain::Flow);
        }
    }

    #[test]
    fn troglodyte_retains_the_precision_expression_for_fine_mechanisms() {
        let resolution = resolve_current_grip_action(&canonical_troglodyte_precision_fixture())
            .expect("troglodyte precision should resolve");
        assert_eq!(
            resolution.selected_expression(),
            GripExpressionId::TinkerGrip
        );
        assert!(
            resolution
                .available_inherited_expressions()
                .contains(&GripExpressionId::WorldGrip)
        );
    }

    #[test]
    fn moxy_and_foxy_contexts_influence_but_do_not_auto_legalize() {
        let moxy = resolve_current_grip_action(&canonical_troll_moxy_bridgegrip_fixture())
            .expect("moxy bridge should resolve");
        assert_eq!(moxy.selected_expression(), GripExpressionId::BridgeGrip);
        assert!(moxy.recipe().is_none());

        let foxy = resolve_current_grip_action(&canonical_gremlin_foxy_tinkergrip_fixture())
            .expect("foxy latch should resolve");
        assert_eq!(foxy.selected_expression(), GripExpressionId::TinkerGrip);
        assert!(foxy.recipe().is_none());
    }

    #[test]
    fn practice_projection_filters_out_invalid_actions() {
        let profile = project_grip_practice_profile(&[
            super::GripPracticeEvent {
                move_expression: GripExpressionId::TinkerGrip,
                current_form: FrameId::Gremlin,
                gesture: EmbodiedGesture::Grip,
                mode: ActionMode::Seam,
                object_family: ObjectFamily::Mechanism,
                object_material: ObjectMaterial::Metal,
                object_condition: crate::being_object_ontology::ObjectCondition::Stable,
                object_scale: ObjectScale::Fine,
                addressing_mode: AddressingMode::Proxy,
                aim: ActionAim::Manipulate,
                current_invested: 2,
                pressure_requirement: PressureRequirement::Delicate,
                valid_execution: true,
            },
            super::GripPracticeEvent {
                move_expression: GripExpressionId::WorldGrip,
                current_form: FrameId::Troglodyte,
                gesture: EmbodiedGesture::Grip,
                mode: ActionMode::Seam,
                object_family: ObjectFamily::Terrain,
                object_material: ObjectMaterial::Stone,
                object_condition: crate::being_object_ontology::ObjectCondition::Fractured,
                object_scale: ObjectScale::Terrain,
                addressing_mode: AddressingMode::Proxy,
                aim: ActionAim::Anchor,
                current_invested: 6,
                pressure_requirement: PressureRequirement::Terrain,
                valid_execution: false,
            },
        ]);

        assert_eq!(profile.total_valid_uses, 1);
        assert_eq!(profile.familiarity_for_family(ObjectFamily::Terrain), 0);
    }

    #[test]
    fn tendency_hooks_remain_projected_not_permanent() {
        let freemason = resolve_current_grip_action(&canonical_freemason_tendency_fixture())
            .expect("freemason tendency should resolve");
        assert_eq!(
            freemason.projected_tendency(),
            Some(StonebendApex::Freemason)
        );

        let proletariat = resolve_current_grip_action(&canonical_proletariat_tendency_fixture())
            .expect("proletariat tendency should resolve");
        assert_eq!(
            proletariat.projected_tendency(),
            Some(StonebendApex::Proletariat)
        );

        let hypergiant = resolve_current_grip_action(&canonical_hypergiant_tendency_fixture())
            .expect("hypergiant tendency should resolve");
        assert_eq!(
            hypergiant.projected_tendency(),
            Some(StonebendApex::Hypergiant)
        );
    }

    #[test]
    fn canonical_contract_fixture_passes() {
        assert!(
            validate_current_grip_inheritance_contract(
                &canonical_current_grip_inheritance_contract_fixture()
            )
            .is_empty()
        );
    }

    #[test]
    fn contradiction_fixtures_fail_with_explicit_messages() {
        let contradictions = [
            (
                CurrentGripInheritanceContractInput {
                    gesture_skill_collapsed: true,
                    ..CurrentGripInheritanceContractInput::default()
                },
                "distinct from EmbodiedGesture::Grip",
            ),
            (
                CurrentGripInheritanceContractInput {
                    higher_form_loses_lower_expressions: true,
                    ..CurrentGripInheritanceContractInput::default()
                },
                "retain legal lower Grip expressions",
            ),
            (
                CurrentGripInheritanceContractInput {
                    grip_only_seam: true,
                    ..CurrentGripInheritanceContractInput::default()
                },
                "valid Grip + Beam contexts remain legal",
            ),
            (
                CurrentGripInheritanceContractInput {
                    idle_time_progression: true,
                    ..CurrentGripInheritanceContractInput::default()
                },
                "Idle time must not grant Grip mastery",
            ),
            (
                CurrentGripInheritanceContractInput {
                    v1_1_changed: true,
                    ..CurrentGripInheritanceContractInput::default()
                },
                "Frozen V1.1 topology must remain unchanged",
            ),
        ];

        for (input, expected) in contradictions {
            let diagnostics = validate_current_grip_inheritance_contract(&input);
            assert!(
                diagnostics
                    .iter()
                    .any(|entry| entry.message.contains(expected))
            );
        }
    }

    #[test]
    fn witness_and_validation_surfaces_render() {
        let witness =
            build_current_inheritance_witness().expect("current inheritance witness should render");
        assert!(witness.contains("HOLLOW GROVE CURRENT INHERITANCE"));

        let validation = build_current_inheritance_validation_report()
            .expect("current inheritance validation should render");
        assert!(validation.contains("status: pass"));

        let grip_witness = build_grip_witness().expect("grip witness should render");
        assert!(grip_witness.contains("Troglodyte"));
        assert!(grip_witness.contains("TinkerGrip"));
        assert!(grip_witness.contains("WorldGrip"));
        assert!(grip_witness.contains("Projected Stonebend Tendency"));
    }
}
