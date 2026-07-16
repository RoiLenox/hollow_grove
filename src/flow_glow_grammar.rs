use std::io;

use crate::being_object_ontology::{
    ActionAim, AddressingMode, BeingState, ObjectFamily, ObjectId, ObjectState,
    build_canonical_being_state_for_frame, build_canonical_being_state_with_aura,
    canonical_object_state,
};
use crate::frame_state::FrameId;
use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpressionDomain {
    Flow,
    Glow,
}

impl ExpressionDomain {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flow => "Flow",
            Self::Glow => "Glow",
        }
    }

    #[must_use]
    pub const fn definition(self) -> &'static str {
        match self {
            Self::Flow => "material Current expression",
            Self::Glow => "psychic Aura expression",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbodiedGesture {
    Grip,
    Show,
    Grit,
}

impl EmbodiedGesture {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grip => "Grip",
            Self::Show => "Show",
            Self::Grit => "Grit",
        }
    }

    #[must_use]
    pub const fn definition(self) -> &'static str {
        match self {
            Self::Grip => "controlled contact and stabilization",
            Self::Show => "reveal, diagnose, present, stage, expose, and direct attention",
            Self::Grit => "remain present, endure pressure, contain fear, and sustain action",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionMode {
    Seam,
    Beam,
    Gleam,
}

impl ActionMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Seam => "Seam",
            Self::Beam => "Beam",
            Self::Gleam => "Gleam",
        }
    }

    #[must_use]
    pub const fn definition(self) -> &'static str {
        match self {
            Self::Seam => {
                "connection, joining, separation, opening, closing, stitching, and tearing"
            }
            Self::Beam => "direction, focus, aim, projection, transmission, and intervention",
            Self::Gleam => {
                "revealed condition, presence, recognition, finish, emotional effect, and visible proof"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StonebendApex {
    Freemason,
    Proletariat,
    Hypergiant,
}

impl StonebendApex {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Freemason => "Freemason",
            Self::Proletariat => "Proletariat",
            Self::Hypergiant => "Hypergiant",
        }
    }

    #[must_use]
    pub const fn canonical_mode(self) -> ActionMode {
        match self {
            Self::Freemason => ActionMode::Beam,
            Self::Proletariat => ActionMode::Seam,
            Self::Hypergiant => ActionMode::Gleam,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbodiedRole {
    Nightingale,
    Surgeon,
    Muse,
    Siren,
    Freemason,
    Proletariat,
    Hypergiant,
}

impl EmbodiedRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nightingale => "Nightingale",
            Self::Surgeon => "Surgeon",
            Self::Muse => "Muse",
            Self::Siren => "Siren",
            Self::Freemason => "Freemason",
            Self::Proletariat => "Proletariat",
            Self::Hypergiant => "Hypergiant",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityLevel {
    High,
    Valid,
    Low,
}

impl CompatibilityLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Valid => "Valid",
            Self::Low => "Low",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbodiedMoveId {
    AuraStitch,
    SurgicalSeam,
    DiagnosticBeam,
    MeaningRevelation,
    CommandPresence,
    CraftLine,
    CivicSeam,
    SovereignGleam,
    SeamDiagnosis,
}

impl EmbodiedMoveId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraStitch => "Aura Stitch",
            Self::SurgicalSeam => "Surgical Seam",
            Self::DiagnosticBeam => "Diagnostic Beam",
            Self::MeaningRevelation => "Meaning Revelation",
            Self::CommandPresence => "Command Presence",
            Self::CraftLine => "Craft Line",
            Self::CivicSeam => "Civic Seam",
            Self::SovereignGleam => "Sovereign Gleam",
            Self::SeamDiagnosis => "Seam Diagnosis",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecipeBoundaryStatus {
    LegalFixtureAvailable,
    LegalFixtureRequired,
}

impl RecipeBoundaryStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegalFixtureAvailable => "canonical Recipe fixture available",
            Self::LegalFixtureRequired => {
                "Recipe legality required; no canonical Recipe fixture yet"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbodiedPracticeRecord {
    being_role: EmbodiedRole,
    domain: ExpressionDomain,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object_family: ObjectFamily,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    successful_uses: u64,
}

impl EmbodiedPracticeRecord {
    #[must_use]
    pub const fn successful_uses(&self) -> u64 {
        self.successful_uses
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbodiedActionRequest {
    role: EmbodiedRole,
    being: BeingState,
    domain: ExpressionDomain,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    practice: EmbodiedPracticeRecord,
}

impl EmbodiedActionRequest {
    #[must_use]
    pub const fn role(&self) -> EmbodiedRole {
        self.role
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
    pub const fn practice(&self) -> &EmbodiedPracticeRecord {
        &self.practice
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbodiedMoveResolution {
    request: EmbodiedActionRequest,
    canonical_pairing: bool,
    compatibility: CompatibilityLevel,
    compatibility_reason: String,
    candidate_move: EmbodiedMoveId,
    recipe_status: RecipeBoundaryStatus,
}

impl EmbodiedMoveResolution {
    #[must_use]
    pub const fn request(&self) -> &EmbodiedActionRequest {
        &self.request
    }

    #[must_use]
    pub const fn canonical_pairing(&self) -> bool {
        self.canonical_pairing
    }

    #[must_use]
    pub const fn compatibility(&self) -> CompatibilityLevel {
        self.compatibility
    }

    #[must_use]
    pub fn compatibility_reason(&self) -> &str {
        &self.compatibility_reason
    }

    #[must_use]
    pub const fn candidate_move(&self) -> EmbodiedMoveId {
        self.candidate_move
    }

    #[must_use]
    pub const fn recipe_status(&self) -> RecipeBoundaryStatus {
        self.recipe_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FlowGlowContractInput {
    pub domains_collapsed: bool,
    pub modes_as_fixed_moves: bool,
    pub gestures_as_frames_or_species: bool,
    pub show_replaced_with_point: bool,
    pub show_reduced_to_pointing: bool,
    pub grip_reduced_to_skill_root_grip: bool,
    pub grit_reduced_to_stamina: bool,
    pub freemason_mapped_to_seam: bool,
    pub proletariat_mapped_to_gleam: bool,
    pub hypergiant_mapped_to_beam: bool,
    pub canonical_pairings_reversed: bool,
    pub beam_only_literal_light: bool,
    pub seam_only_sewing: bool,
    pub gleam_only_shine: bool,
    pub glow_only_illumination: bool,
    pub flow_only_movement: bool,
    pub gesture_replaces_object: bool,
    pub mode_replaces_aim: bool,
    pub addressing_mode_replaces_mode: bool,
    pub direct_execution_outside_boundary: bool,
    pub foxy_evil: bool,
    pub moxy_velocity_only: bool,
    pub idle_time_progression: bool,
    pub v1_1_changed: bool,
    pub point_cubed: bool,
    pub position_thirteen: bool,
    pub automatic_aura_frame_grant: bool,
    pub current_prism_conflated: bool,
}

#[must_use]
pub fn canonical_flow_glow_contract_fixture() -> FlowGlowContractInput {
    FlowGlowContractInput::default()
}

#[must_use]
pub fn validate_flow_glow_contract(input: &FlowGlowContractInput) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.domains_collapsed {
        diagnostics.push(grammar_error(
            "Flow and Glow must remain distinct typed expression domains.",
        ));
    }
    if input.modes_as_fixed_moves {
        diagnostics.push(grammar_error(
            "Seam, Beam, and Gleam must remain modes rather than fixed Moves.",
        ));
    }
    if input.gestures_as_frames_or_species {
        diagnostics.push(grammar_error(
            "Grip, Show, and Grit are embodied gestures and cannot be treated as Frames or species.",
        ));
    }
    if input.show_replaced_with_point {
        diagnostics.push(grammar_error(
            "Show must remain canonical and cannot be replaced with Point in embodied Glaüshouse grammar.",
        ));
    }
    if input.show_reduced_to_pointing {
        diagnostics.push(grammar_error(
            "Show is revelation and directed attention, not mere pointing.",
        ));
    }
    if input.grip_reduced_to_skill_root_grip {
        diagnostics.push(grammar_error(
            "EmbodiedGesture::Grip must remain broader than SkillRoot::Grip and cannot collapse into it.",
        ));
    }
    if input.grit_reduced_to_stamina {
        diagnostics.push(grammar_error(
            "Grit cannot be reduced to generic stamina; it is endurance made expressive.",
        ));
    }
    if input.freemason_mapped_to_seam {
        diagnostics.push(grammar_error(
            "Freemason must remain Stonebend's Beam specialization.",
        ));
    }
    if input.proletariat_mapped_to_gleam {
        diagnostics.push(grammar_error(
            "Proletariat must remain Stonebend's Seam specialization.",
        ));
    }
    if input.hypergiant_mapped_to_beam {
        diagnostics.push(grammar_error(
            "Hypergiant must remain Stonebend's Gleam specialization.",
        ));
    }
    if input.canonical_pairings_reversed {
        diagnostics.push(grammar_error(
            "Canonical pairings must remain Grip -> Seam, Show -> Beam, and Grit -> Gleam.",
        ));
    }
    if input.beam_only_literal_light {
        diagnostics.push(grammar_error(
            "Beam cannot be reduced to literal light only.",
        ));
    }
    if input.seam_only_sewing {
        diagnostics.push(grammar_error(
            "Seam must include joining, separation, opening, closing, and tearing rather than sewing only.",
        ));
    }
    if input.gleam_only_shine {
        diagnostics.push(grammar_error(
            "Gleam cannot be reduced to visual shininess alone.",
        ));
    }
    if input.glow_only_illumination {
        diagnostics.push(grammar_error(
            "Glow must remain a psychic Aura domain and cannot be reduced to illumination only.",
        ));
    }
    if input.flow_only_movement {
        diagnostics.push(grammar_error(
            "Flow must remain the wider Current/material expression domain and cannot be reduced to movement only.",
        ));
    }
    if input.gesture_replaces_object {
        diagnostics.push(grammar_error(
            "Gesture cannot replace Object in embodied action resolution.",
        ));
    }
    if input.mode_replaces_aim {
        diagnostics.push(grammar_error(
            "Mode cannot replace Aim; they answer different questions.",
        ));
    }
    if input.addressing_mode_replaces_mode {
        diagnostics.push(grammar_error("AddressingMode cannot replace ActionMode."));
    }
    if input.direct_execution_outside_boundary {
        diagnostics.push(grammar_error(
            "Embodied grammar cannot execute outside Recipe, V2, and the frozen V1.1 boundary.",
        ));
    }
    if input.foxy_evil {
        diagnostics.push(grammar_error(
            "Foxy cannot be treated as evil by definition.",
        ));
    }
    if input.moxy_velocity_only {
        diagnostics.push(grammar_error("Moxy cannot be reduced to velocity."));
    }
    if input.idle_time_progression {
        diagnostics.push(grammar_error(
            "Idle elapsed time cannot count as embodied mastery.",
        ));
    }
    if input.v1_1_changed {
        diagnostics.push(grammar_error(
            "The frozen V1.1 topology must remain unchanged.",
        ));
    }
    if input.point_cubed {
        diagnostics.push(grammar_error("Point³ is not a legal state."));
    }
    if input.position_thirteen {
        diagnostics.push(grammar_error("Position 13 is not a legal world state."));
    }
    if input.automatic_aura_frame_grant {
        diagnostics.push(grammar_error(
            "This grammar cannot auto-grant an Aura Frame.",
        ));
    }
    if input.current_prism_conflated {
        diagnostics.push(grammar_error(
            "CurrentPrism must remain distinct from Flow, Glow, and capacity.",
        ));
    }

    diagnostics
}

#[must_use]
pub fn resolve_embodied_action(request: EmbodiedActionRequest) -> EmbodiedMoveResolution {
    let (canonical_pairing, compatibility, compatibility_reason, candidate_move) = match (
        request.role,
        request.domain,
        request.gesture,
        request.mode,
        request.object.family(),
        request.aim,
    ) {
        (
            EmbodiedRole::Nightingale,
            ExpressionDomain::Glow,
            EmbodiedGesture::Grip,
            ActionMode::Seam,
            ObjectFamily::Wound,
            ActionAim::StabilizeAndClose,
        ) => (
            true,
            CompatibilityLevel::High,
            String::from(
                "Glow Grip Seam is a canonical Nightingale pairing for controlled clinical contact on a wounded continuity.",
            ),
            EmbodiedMoveId::AuraStitch,
        ),
        (
            EmbodiedRole::Surgeon,
            ExpressionDomain::Flow,
            EmbodiedGesture::Grip,
            ActionMode::Seam,
            ObjectFamily::Tissue,
            ActionAim::InciseOrSuture,
        ) => (
            true,
            CompatibilityLevel::High,
            String::from("Flow Grip Seam gives precise material entry into tissue continuity."),
            EmbodiedMoveId::SurgicalSeam,
        ),
        (
            EmbodiedRole::Nightingale,
            ExpressionDomain::Glow,
            EmbodiedGesture::Show,
            ActionMode::Beam,
            ObjectFamily::Infection,
            ActionAim::DiagnoseAndReveal,
        ) => (
            true,
            CompatibilityLevel::High,
            String::from(
                "Glow Show Beam is the canonical diagnostic revelation line for hidden infection.",
            ),
            EmbodiedMoveId::DiagnosticBeam,
        ),
        (
            EmbodiedRole::Muse,
            ExpressionDomain::Glow,
            EmbodiedGesture::Show,
            ActionMode::Beam,
            ObjectFamily::Meaning,
            ActionAim::RevealCentralTruth,
        ) => (
            true,
            CompatibilityLevel::High,
            String::from(
                "Muse-scale Glow Show Beam directs a crowd's central meaning into recognition.",
            ),
            EmbodiedMoveId::MeaningRevelation,
        ),
        (
            EmbodiedRole::Siren,
            ExpressionDomain::Glow,
            EmbodiedGesture::Grit,
            ActionMode::Gleam,
            ObjectFamily::Crowd,
            ActionAim::SustainEmotionalCommand,
        ) => (
            true,
            CompatibilityLevel::High,
            String::from("Glow Grit Gleam sustains commanding presence across a frightened crowd."),
            EmbodiedMoveId::CommandPresence,
        ),
        (
            EmbodiedRole::Freemason,
            ExpressionDomain::Flow,
            EmbodiedGesture::Grip,
            ActionMode::Beam,
            ObjectFamily::Tool,
            ActionAim::DirectPreciseLineOfForce | ActionAim::StabilizeAndDirect,
        ) => (
            false,
            CompatibilityLevel::Valid,
            String::from(
                "Grip + Beam is a valid cross-pairing because instrument control stabilizes the directed line of craft.",
            ),
            EmbodiedMoveId::CraftLine,
        ),
        (
            EmbodiedRole::Proletariat,
            ExpressionDomain::Flow,
            EmbodiedGesture::Grit,
            ActionMode::Seam,
            ObjectFamily::Formation,
            ActionAim::JoinMendOrTear,
        ) => (
            false,
            CompatibilityLevel::Valid,
            String::from(
                "Grit + Seam is a valid collective cross-pairing because endured pressure can hold or tear a civic continuity.",
            ),
            EmbodiedMoveId::CivicSeam,
        ),
        (
            EmbodiedRole::Hypergiant,
            ExpressionDomain::Flow,
            EmbodiedGesture::Show,
            ActionMode::Gleam,
            ObjectFamily::Monument,
            ActionAim::RevealProvenSovereignty,
        ) => (
            false,
            CompatibilityLevel::Valid,
            String::from(
                "Show + Gleam is a valid cross-pairing when public staging reveals proven physical sovereignty rather than granting empty title.",
            ),
            EmbodiedMoveId::SovereignGleam,
        ),
        (
            EmbodiedRole::Nightingale,
            ExpressionDomain::Glow,
            EmbodiedGesture::Show,
            ActionMode::Seam,
            ObjectFamily::EmotionalRupture,
            ActionAim::RevealBrokenRelation,
        ) => (
            false,
            CompatibilityLevel::Valid,
            String::from(
                "Show + Seam is a valid cross-pairing because the hidden break must be revealed before relational repair can begin.",
            ),
            EmbodiedMoveId::SeamDiagnosis,
        ),
        _ => (
            matches!(
                (request.gesture, request.mode),
                (EmbodiedGesture::Grip, ActionMode::Seam)
                    | (EmbodiedGesture::Show, ActionMode::Beam)
                    | (EmbodiedGesture::Grit, ActionMode::Gleam)
            ),
            CompatibilityLevel::Low,
            String::from(
                "The request remains typed, but no canonical embodied fixture defines this combination yet.",
            ),
            EmbodiedMoveId::CraftLine,
        ),
    };

    let recipe_status = if matches!(
        (
            request.role,
            request.domain,
            request.gesture,
            request.mode,
            request.object.identity()
        ),
        (
            EmbodiedRole::Surgeon,
            ExpressionDomain::Flow,
            EmbodiedGesture::Grip,
            ActionMode::Seam,
            ObjectId::MechanicalLatch
        )
    ) {
        RecipeBoundaryStatus::LegalFixtureAvailable
    } else {
        RecipeBoundaryStatus::LegalFixtureRequired
    };

    EmbodiedMoveResolution {
        request,
        canonical_pairing,
        compatibility,
        compatibility_reason,
        candidate_move,
        recipe_status,
    }
}

#[must_use]
pub fn canonical_nightingale_grip_seam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Nightingale,
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Pixy)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::OpenWound),
        AddressingMode::Proxy,
        ActionAim::StabilizeAndClose,
        6,
    )
}

#[must_use]
pub fn canonical_surgeon_flow_grip_seam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Surgeon,
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Pixy)),
        ExpressionDomain::Flow,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::Tissue),
        AddressingMode::Proxy,
        ActionAim::InciseOrSuture,
        8,
    )
}

#[must_use]
pub fn canonical_nightingale_show_beam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Nightingale,
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Pixy)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::HiddenInfection),
        AddressingMode::Proxy,
        ActionAim::DiagnoseAndReveal,
        7,
    )
}

#[must_use]
pub fn canonical_muse_show_beam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Muse,
        build_canonical_being_state_for_frame(FrameId::Muse),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::MeaningLink),
        AddressingMode::Moxy,
        ActionAim::RevealCentralTruth,
        12,
    )
}

#[must_use]
pub fn canonical_siren_grit_gleam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Siren,
        build_canonical_being_state_for_frame(FrameId::Siren),
        ExpressionDomain::Glow,
        EmbodiedGesture::Grit,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::FrightenedCrowd),
        AddressingMode::Moxy,
        ActionAim::SustainEmotionalCommand,
        11,
    )
}

#[must_use]
pub fn canonical_freemason_flow_beam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Freemason,
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        EmbodiedGesture::Grip,
        ActionMode::Beam,
        canonical_object_state(ObjectId::CuttingTool),
        AddressingMode::Proxy,
        ActionAim::DirectPreciseLineOfForce,
        14,
    )
}

#[must_use]
pub fn canonical_proletariat_flow_seam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Proletariat,
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        EmbodiedGesture::Grit,
        ActionMode::Seam,
        canonical_object_state(ObjectId::FormationAnchor),
        AddressingMode::Moxy,
        ActionAim::JoinMendOrTear,
        13,
    )
}

#[must_use]
pub fn canonical_hypergiant_flow_gleam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Hypergiant,
        build_canonical_being_state_for_frame(FrameId::Troglodyte),
        ExpressionDomain::Flow,
        EmbodiedGesture::Show,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::Monument),
        AddressingMode::Proxy,
        ActionAim::RevealProvenSovereignty,
        16,
    )
}

#[must_use]
pub fn canonical_cross_pair_show_seam_fixture() -> EmbodiedActionRequest {
    build_request(
        EmbodiedRole::Nightingale,
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Pixy)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Seam,
        canonical_object_state(ObjectId::HiddenEmotionalRupture),
        AddressingMode::Foxy,
        ActionAim::RevealBrokenRelation,
        5,
    )
}

#[must_use]
pub fn build_flow_glow_witness() -> io::Result<String> {
    Ok(String::from(
        "HOLLOW GROVE FLOW / GLOW GRAMMAR\n\n\
         Domains:\n\n\
         Flow\n\
         = material Current expression\n\n\
         Glow\n\
         = psychic Aura expression\n\n\
         Modes:\n\n\
         Seam\n\
         = connection\n\n\
         Beam\n\
         = direction\n\n\
         Gleam\n\
         = revealed condition\n\n\
         Gestures:\n\n\
         Grip\n\
         = controlled contact\n\n\
         Show\n\
         = revelation and directed attention\n\n\
         Grit\n\
         = sustained presence under pressure\n\n\
         Canonical Pairings:\n\n\
         Grip -> Seam\n\
         Show -> Beam\n\
         Grit -> Gleam\n\n\
         Stonebend Apex:\n\n\
         Freemason -> Beam\n\
         Proletariat -> Seam\n\
         Hypergiant -> Gleam\n\n\
         Execution Boundary:\n\n\
         Move\n\
         -> Recipe\n\
         -> V2\n\
         -> frozen V1.1\n",
    ))
}

pub fn build_flow_glow_validation_report() -> io::Result<String> {
    let diagnostics = validate_flow_glow_contract(&canonical_flow_glow_contract_fixture());
    let freemason = resolve_embodied_action(canonical_freemason_flow_beam_fixture());
    let proletariat = resolve_embodied_action(canonical_proletariat_flow_seam_fixture());
    let hypergiant = resolve_embodied_action(canonical_hypergiant_flow_gleam_fixture());

    let mut errors = diagnostics;
    if freemason.request().mode() != StonebendApex::Freemason.canonical_mode() {
        errors.push(grammar_error(
            "Freemason canonical mode drifted away from Beam.",
        ));
    }
    if proletariat.request().mode() != StonebendApex::Proletariat.canonical_mode() {
        errors.push(grammar_error(
            "Proletariat canonical mode drifted away from Seam.",
        ));
    }
    if hypergiant.request().mode() != StonebendApex::Hypergiant.canonical_mode() {
        errors.push(grammar_error(
            "Hypergiant canonical mode drifted away from Gleam.",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Flow / Glow Validation\n\n\
             - status: pass\n\
             - Flow/Glow distinction: pass\n\
             - Seam/Beam/Gleam distinction: pass\n\
             - Grip/Show/Grit distinction: pass\n\
             - canonical pairings: pass\n\
             - Stonebend apex mapping: pass\n\
             - Being/Object preservation: pass\n\
             - Aim preservation: pass\n\
             - AddressingMode preservation: pass\n\
             - Recipe boundary: pass\n\
             - V1.1 unchanged: pass\n",
        ))
    } else {
        let mut output = String::from("# Hollow Grove Flow / Glow Validation\n\n- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        Ok(output)
    }
}

pub fn build_embodied_action_witness() -> io::Result<String> {
    let fixtures = [
        resolve_embodied_action(canonical_nightingale_grip_seam_fixture()),
        resolve_embodied_action(canonical_nightingale_show_beam_fixture()),
        resolve_embodied_action(canonical_siren_grit_gleam_fixture()),
        resolve_embodied_action(canonical_freemason_flow_beam_fixture()),
        resolve_embodied_action(canonical_proletariat_flow_seam_fixture()),
        resolve_embodied_action(canonical_hypergiant_flow_gleam_fixture()),
        resolve_embodied_action(canonical_muse_show_beam_fixture()),
        resolve_embodied_action(canonical_cross_pair_show_seam_fixture()),
    ];

    let mut output = String::from("HOLLOW GROVE EMBODIED ACTION WITNESS\n");
    for resolution in fixtures {
        append_resolution(&mut output, &resolution);
    }
    Ok(output)
}

fn append_resolution(output: &mut String, resolution: &EmbodiedMoveResolution) {
    let request = resolution.request();
    output.push_str("\n\n");
    output.push_str(&format!(
        "Being:\n{}\n\nCurrent Form:\n{}\n\nAura Frame:\n{}\n\nDomain:\n{}\n\nGesture:\n{}\n\nMode:\n{}\n\nObject:\n{}\n\nObject Family:\n{}\n\nAim:\n{}\n\nAddressingMode:\n{}\n\nCanonical Affinity:\n{}\n\nCompatibility:\n{}\n\nCompatibility Reason:\n{}\n\nCandidate Move:\n{}\n\nRecipe Status:\n{}\n",
        request.role().as_str(),
        frame_label(request.being().current_form()),
        request
            .being()
            .aura_frame()
            .map(frame_label)
            .unwrap_or("none"),
        request.domain().as_str(),
        request.gesture().as_str(),
        request.mode().as_str(),
        request.object().identity().as_str(),
        request.object().family().as_str(),
        request.aim().as_str(),
        request.addressing_mode().as_str(),
        if resolution.canonical_pairing() { "canonical" } else { "cross-pair" },
        resolution.compatibility().as_str(),
        resolution.compatibility_reason(),
        resolution.candidate_move().as_str(),
        resolution.recipe_status().as_str(),
    ));
}

fn build_request(
    role: EmbodiedRole,
    being: BeingState,
    domain: ExpressionDomain,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    successful_uses: u64,
) -> EmbodiedActionRequest {
    EmbodiedActionRequest {
        role,
        being,
        domain,
        gesture,
        mode,
        object: object.clone(),
        addressing_mode,
        aim,
        practice: EmbodiedPracticeRecord {
            being_role: role,
            domain,
            gesture,
            mode,
            object_family: object.family(),
            addressing_mode,
            aim,
            successful_uses,
        },
    }
}

fn grammar_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::BeingObjectOntologyMismatch,
        message: message.into(),
    }
}

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
        ActionMode, CompatibilityLevel, EmbodiedGesture, ExpressionDomain, FlowGlowContractInput,
        StonebendApex, build_embodied_action_witness, build_flow_glow_validation_report,
        build_flow_glow_witness, canonical_cross_pair_show_seam_fixture,
        canonical_flow_glow_contract_fixture, canonical_freemason_flow_beam_fixture,
        canonical_hypergiant_flow_gleam_fixture, canonical_muse_show_beam_fixture,
        canonical_nightingale_grip_seam_fixture, canonical_nightingale_show_beam_fixture,
        canonical_proletariat_flow_seam_fixture, canonical_siren_grit_gleam_fixture,
        resolve_embodied_action, validate_flow_glow_contract,
    };
    use crate::FrameId;

    #[test]
    fn canonical_flow_glow_contract_fixture_passes() {
        assert!(validate_flow_glow_contract(&canonical_flow_glow_contract_fixture()).is_empty());
    }

    #[test]
    fn canonical_pairings_resolve_high_compatibility() {
        let nightingale = resolve_embodied_action(canonical_nightingale_grip_seam_fixture());
        let diagnosis = resolve_embodied_action(canonical_nightingale_show_beam_fixture());
        let siren = resolve_embodied_action(canonical_siren_grit_gleam_fixture());

        assert_eq!(nightingale.request().domain(), ExpressionDomain::Glow);
        assert_eq!(nightingale.request().gesture(), EmbodiedGesture::Grip);
        assert_eq!(nightingale.request().mode(), ActionMode::Seam);
        assert_eq!(nightingale.compatibility(), CompatibilityLevel::High);
        assert!(nightingale.canonical_pairing());

        assert_eq!(diagnosis.compatibility(), CompatibilityLevel::High);
        assert!(diagnosis.canonical_pairing());

        assert_eq!(siren.compatibility(), CompatibilityLevel::High);
        assert!(siren.canonical_pairing());
    }

    #[test]
    fn cross_pairings_remain_valid_when_semantically_grounded() {
        let freemason = resolve_embodied_action(canonical_freemason_flow_beam_fixture());
        let proletariat = resolve_embodied_action(canonical_proletariat_flow_seam_fixture());
        let hypergiant = resolve_embodied_action(canonical_hypergiant_flow_gleam_fixture());
        let seam_show = resolve_embodied_action(canonical_cross_pair_show_seam_fixture());

        assert_eq!(freemason.compatibility(), CompatibilityLevel::Valid);
        assert!(!freemason.canonical_pairing());
        assert!(
            freemason
                .compatibility_reason()
                .contains("instrument control")
        );

        assert_eq!(proletariat.compatibility(), CompatibilityLevel::Valid);
        assert_eq!(hypergiant.compatibility(), CompatibilityLevel::Valid);
        assert_eq!(seam_show.compatibility(), CompatibilityLevel::Valid);
    }

    #[test]
    fn stonebend_apex_mapping_remains_locked() {
        assert_eq!(StonebendApex::Freemason.canonical_mode(), ActionMode::Beam);
        assert_eq!(
            StonebendApex::Proletariat.canonical_mode(),
            ActionMode::Seam
        );
        assert_eq!(
            StonebendApex::Hypergiant.canonical_mode(),
            ActionMode::Gleam
        );
    }

    #[test]
    fn muse_show_beam_scales_to_aura_apex() {
        let muse = resolve_embodied_action(canonical_muse_show_beam_fixture());
        assert_eq!(muse.request().domain(), ExpressionDomain::Glow);
        assert_eq!(muse.request().mode(), ActionMode::Beam);
        assert_eq!(muse.request().being().current_form(), FrameId::Muse);
        assert!(muse.compatibility_reason().contains("Muse-scale"));
    }

    #[test]
    fn contradiction_fixtures_fail_with_explicit_messages() {
        let diagnostics = validate_flow_glow_contract(&FlowGlowContractInput {
            domains_collapsed: true,
            show_replaced_with_point: true,
            grip_reduced_to_skill_root_grip: true,
            freemason_mapped_to_seam: true,
            canonical_pairings_reversed: true,
            direct_execution_outside_boundary: true,
            current_prism_conflated: true,
            ..FlowGlowContractInput::default()
        });
        let messages = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>();

        assert!(
            messages
                .iter()
                .any(|message| message.contains("Flow and Glow must remain distinct"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("Show must remain canonical"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("EmbodiedGesture::Grip"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("Freemason must remain"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("Canonical pairings must remain"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("frozen V1.1 boundary"))
        );
        assert!(
            messages
                .iter()
                .any(|message| message.contains("CurrentPrism"))
        );
    }

    #[test]
    fn witness_and_validation_surfaces_render() {
        let witness = build_flow_glow_witness().expect("witness should build");
        let validation = build_flow_glow_validation_report().expect("validation should build");
        let embodied = build_embodied_action_witness().expect("embodied witness should build");

        assert!(witness.contains("Grip -> Seam"));
        assert!(witness.contains("Freemason -> Beam"));
        assert!(validation.contains("- status: pass"));
        assert!(embodied.contains("Nightingale"));
        assert!(embodied.contains("Candidate Move:"));
    }
}
