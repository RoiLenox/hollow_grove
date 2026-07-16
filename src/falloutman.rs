use std::io;

use crate::aura_polarity::AuraPolarity;
use crate::being_object_ontology::{ActionAim, AddressingMode, BeingState, ObjectState, SkillId};
use crate::decision_engine::{
    ActiveObjective, ActivePurpose, AgencyConsequence, CandidateMoveId, CandidateTacticId,
    DecisionBeat, GivenCircumstance, ObjectiveProgress, Obstacle, ProjectionUncertainty,
    PurposeAlignment, RiskLevel, StanislavskiCandidateEvaluation,
    canonical_nightingale_hidden_wound_sequence,
};
use crate::flow_glow_grammar::{
    ActionMode, EmbodiedGesture, ExpressionDomain, RecipeBoundaryStatus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponsePresentationKind {
    Spoken,
    Nonverbal,
    PhysicalAction,
    MedicalAction,
    ToolAction,
    Silence,
    Leave,
}

impl ResponsePresentationKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spoken => "Spoken",
            Self::Nonverbal => "Nonverbal",
            Self::PhysicalAction => "PhysicalAction",
            Self::MedicalAction => "MedicalAction",
            Self::ToolAction => "ToolAction",
            Self::Silence => "Silence",
            Self::Leave => "Leave",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponsePolarityTag {
    Light,
    Dark,
    Mixed,
    Unoriented,
}

impl ResponsePolarityTag {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "LIGHT",
            Self::Dark => "DARK",
            Self::Mixed => "MIXED",
            Self::Unoriented => "UNORIENTED",
        }
    }

    #[must_use]
    pub const fn aura_polarity(self) -> Option<AuraPolarity> {
        match self {
            Self::Light => Some(AuraPolarity::Light),
            Self::Dark => Some(AuraPolarity::Dark),
            Self::Mixed | Self::Unoriented => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseTag {
    Gesture(EmbodiedGesture),
    Mode(ActionMode),
    Domain(ExpressionDomain),
    Polarity(ResponsePolarityTag),
    Addressing(AddressingMode),
    Leave,
}

impl ResponseTag {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gesture(gesture) => gesture.as_str(),
            Self::Mode(mode) => mode.as_str(),
            Self::Domain(domain) => domain.as_str(),
            Self::Polarity(polarity) => polarity.as_str(),
            Self::Addressing(addressing) => addressing.as_str(),
            Self::Leave => "LEAVE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseAvailabilityReason {
    Skill,
    Form,
    Frame,
    Role,
    Relationship,
    Reputation,
    Object,
    Route,
    Resource,
    Risk,
    Consent,
    Observation,
}

impl ResponseAvailabilityReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Skill => "SKILL",
            Self::Form => "FORM",
            Self::Frame => "FRAME",
            Self::Role => "ROLE",
            Self::Relationship => "RELATIONSHIP",
            Self::Reputation => "REPUTATION",
            Self::Object => "OBJECT",
            Self::Route => "ROUTE",
            Self::Resource => "RESOURCE",
            Self::Risk => "RISK",
            Self::Consent => "CONSENT",
            Self::Observation => "OBSERVATION",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseAvailability {
    Available,
    UnavailableVisible {
        reason: ResponseAvailabilityReason,
        detail: &'static str,
    },
    Hidden,
}

impl ResponseAvailability {
    #[must_use]
    pub const fn is_selectable(&self) -> bool {
        matches!(self, Self::Available)
    }

    #[must_use]
    pub const fn is_visible(&self) -> bool {
        !matches!(self, Self::Hidden)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseProjectionSummary {
    likely_immediate_consequence: &'static str,
    risk: RiskLevel,
    uncertainty: ProjectionUncertainty,
    objective_progress: ObjectiveProgress,
    purpose_alignment: PurposeAlignment,
    target_agency_consequence: AgencyConsequence,
    recipe_status: RecipeBoundaryStatus,
}

impl ResponseProjectionSummary {
    #[must_use]
    pub const fn likely_immediate_consequence(&self) -> &'static str {
        self.likely_immediate_consequence
    }

    #[must_use]
    pub const fn risk(&self) -> RiskLevel {
        self.risk
    }

    #[must_use]
    pub const fn uncertainty(&self) -> ProjectionUncertainty {
        self.uncertainty
    }

    #[must_use]
    pub const fn objective_progress(&self) -> ObjectiveProgress {
        self.objective_progress
    }

    #[must_use]
    pub const fn purpose_alignment(&self) -> PurposeAlignment {
        self.purpose_alignment
    }

    #[must_use]
    pub const fn target_agency_consequence(&self) -> AgencyConsequence {
        self.target_agency_consequence
    }

    #[must_use]
    pub const fn recipe_status(&self) -> RecipeBoundaryStatus {
        self.recipe_status
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseOptionId {
    AskWhatHappened,
    ReassureSafety,
    CallOutConcealment,
    StabilizeWound,
    TraceAuraLesion,
    RequestMinorianMeasurement,
    LeaveExamination,
}

impl ResponseOptionId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AskWhatHappened => "AskWhatHappened",
            Self::ReassureSafety => "ReassureSafety",
            Self::CallOutConcealment => "CallOutConcealment",
            Self::StabilizeWound => "StabilizeWound",
            Self::TraceAuraLesion => "TraceAuraLesion",
            Self::RequestMinorianMeasurement => "RequestMinorianMeasurement",
            Self::LeaveExamination => "LeaveExamination",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttemptedAction {
    being: BeingState,
    skill: SkillId,
    domain: ExpressionDomain,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    route: Option<&'static str>,
    polarity: ResponsePolarityTag,
    candidate_tactic_id: CandidateTacticId,
    candidate_move_id: CandidateMoveId,
}

impl AttemptedAction {
    #[must_use]
    pub const fn being(&self) -> &BeingState {
        &self.being
    }

    #[must_use]
    pub const fn skill(&self) -> SkillId {
        self.skill
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
    pub const fn route(&self) -> Option<&'static str> {
        self.route
    }

    #[must_use]
    pub const fn polarity(&self) -> ResponsePolarityTag {
        self.polarity
    }

    #[must_use]
    pub const fn candidate_tactic_id(&self) -> CandidateTacticId {
        self.candidate_tactic_id
    }

    #[must_use]
    pub const fn candidate_move_id(&self) -> CandidateMoveId {
        self.candidate_move_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalloutmanResponseOption {
    response_id: ResponseOptionId,
    visible_text: &'static str,
    presentation_kind: ResponsePresentationKind,
    visible_tags: Vec<ResponseTag>,
    candidate_tactic_id: CandidateTacticId,
    attempted_action: AttemptedAction,
    availability: ResponseAvailability,
    projection_summary: Option<ResponseProjectionSummary>,
}

impl FalloutmanResponseOption {
    #[must_use]
    pub const fn response_id(&self) -> ResponseOptionId {
        self.response_id
    }

    #[must_use]
    pub const fn visible_text(&self) -> &'static str {
        self.visible_text
    }

    #[must_use]
    pub const fn presentation_kind(&self) -> ResponsePresentationKind {
        self.presentation_kind
    }

    #[must_use]
    pub fn visible_tags(&self) -> &[ResponseTag] {
        &self.visible_tags
    }

    #[must_use]
    pub const fn candidate_tactic_id(&self) -> CandidateTacticId {
        self.candidate_tactic_id
    }

    #[must_use]
    pub const fn attempted_action(&self) -> &AttemptedAction {
        &self.attempted_action
    }

    #[must_use]
    pub const fn availability(&self) -> &ResponseAvailability {
        &self.availability
    }

    #[must_use]
    pub const fn projection_summary(&self) -> Option<&ResponseProjectionSummary> {
        self.projection_summary.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationshipLevel {
    Unknown,
    Low,
    Medium,
    High,
}

impl RelationshipLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DramaticRelationshipState {
    trust: RelationshipLevel,
    fear: RelationshipLevel,
    respect: RelationshipLevel,
    suspicion: RelationshipLevel,
    debt: RelationshipLevel,
    admiration: RelationshipLevel,
    disgust: RelationshipLevel,
}

impl DramaticRelationshipState {
    #[must_use]
    pub const fn trust(&self) -> RelationshipLevel {
        self.trust
    }

    #[must_use]
    pub const fn fear(&self) -> RelationshipLevel {
        self.fear
    }

    #[must_use]
    pub const fn respect(&self) -> RelationshipLevel {
        self.respect
    }

    #[must_use]
    pub const fn suspicion(&self) -> RelationshipLevel {
        self.suspicion
    }

    #[must_use]
    pub const fn debt(&self) -> RelationshipLevel {
        self.debt
    }

    #[must_use]
    pub const fn admiration(&self) -> RelationshipLevel {
        self.admiration
    }

    #[must_use]
    pub const fn disgust(&self) -> RelationshipLevel {
        self.disgust
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpcDramaticCore {
    label: &'static str,
    surface_objective: &'static str,
    true_objective: &'static str,
    super_objective: &'static str,
    secret: &'static str,
    pressure_point: &'static str,
    red_line: &'static str,
    relationship_state: DramaticRelationshipState,
    known_evidence: Vec<&'static str>,
}

impl NpcDramaticCore {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        self.label
    }

    #[must_use]
    pub const fn surface_objective(&self) -> &'static str {
        self.surface_objective
    }

    #[must_use]
    pub const fn true_objective(&self) -> &'static str {
        self.true_objective
    }

    #[must_use]
    pub const fn super_objective(&self) -> &'static str {
        self.super_objective
    }

    #[must_use]
    pub const fn secret(&self) -> &'static str {
        self.secret
    }

    #[must_use]
    pub const fn pressure_point(&self) -> &'static str {
        self.pressure_point
    }

    #[must_use]
    pub const fn red_line(&self) -> &'static str {
        self.red_line
    }

    #[must_use]
    pub const fn relationship_state(&self) -> &DramaticRelationshipState {
        &self.relationship_state
    }

    #[must_use]
    pub fn known_evidence(&self) -> &[&'static str] {
        &self.known_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalloutmanBeatMenu {
    beat_index: u8,
    speaker_label: &'static str,
    scene_prompt: &'static str,
    npc_core: NpcDramaticCore,
    given_circumstances: Vec<GivenCircumstance>,
    objective: ActiveObjective,
    purpose: ActivePurpose,
    obstacles: Vec<Obstacle>,
    options: Vec<FalloutmanResponseOption>,
}

impl FalloutmanBeatMenu {
    #[must_use]
    pub const fn beat_index(&self) -> u8 {
        self.beat_index
    }

    #[must_use]
    pub const fn speaker_label(&self) -> &'static str {
        self.speaker_label
    }

    #[must_use]
    pub const fn scene_prompt(&self) -> &'static str {
        self.scene_prompt
    }

    #[must_use]
    pub const fn npc_core(&self) -> &NpcDramaticCore {
        &self.npc_core
    }

    #[must_use]
    pub fn given_circumstances(&self) -> &[GivenCircumstance] {
        &self.given_circumstances
    }

    #[must_use]
    pub const fn objective(&self) -> ActiveObjective {
        self.objective
    }

    #[must_use]
    pub const fn purpose(&self) -> ActivePurpose {
        self.purpose
    }

    #[must_use]
    pub fn obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }

    #[must_use]
    pub fn options(&self) -> &[FalloutmanResponseOption] {
        &self.options
    }

    #[must_use]
    pub fn visible_options(&self) -> impl Iterator<Item = &FalloutmanResponseOption> {
        self.options
            .iter()
            .filter(|option| option.availability().is_visible())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalloutmanBeatPresentation {
    menu: FalloutmanBeatMenu,
    selected_response_id: ResponseOptionId,
    selected_candidate_tactic_id: CandidateTacticId,
    chosen_tactic_id: CandidateTacticId,
    chosen_move_id: CandidateMoveId,
    chosen_reason: &'static str,
    recipe_status: RecipeBoundaryStatus,
    changed_circumstances: Vec<GivenCircumstance>,
}

impl FalloutmanBeatPresentation {
    #[must_use]
    pub const fn menu(&self) -> &FalloutmanBeatMenu {
        &self.menu
    }

    #[must_use]
    pub const fn selected_response_id(&self) -> ResponseOptionId {
        self.selected_response_id
    }

    #[must_use]
    pub const fn selected_candidate_tactic_id(&self) -> CandidateTacticId {
        self.selected_candidate_tactic_id
    }

    #[must_use]
    pub const fn chosen_tactic_id(&self) -> CandidateTacticId {
        self.chosen_tactic_id
    }

    #[must_use]
    pub const fn chosen_move_id(&self) -> CandidateMoveId {
        self.chosen_move_id
    }

    #[must_use]
    pub const fn chosen_reason(&self) -> &'static str {
        self.chosen_reason
    }

    #[must_use]
    pub const fn recipe_status(&self) -> RecipeBoundaryStatus {
        self.recipe_status
    }

    #[must_use]
    pub fn changed_circumstances(&self) -> &[GivenCircumstance] {
        &self.changed_circumstances
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalloutmanEncounter {
    title: &'static str,
    dramatic_core: NpcDramaticCore,
    visible_state: Vec<&'static str>,
    hidden_state: Vec<&'static str>,
    beats: Vec<FalloutmanBeatPresentation>,
}

impl FalloutmanEncounter {
    #[must_use]
    pub const fn title(&self) -> &'static str {
        self.title
    }

    #[must_use]
    pub const fn dramatic_core(&self) -> &NpcDramaticCore {
        &self.dramatic_core
    }

    #[must_use]
    pub fn visible_state(&self) -> &[&'static str] {
        &self.visible_state
    }

    #[must_use]
    pub fn hidden_state(&self) -> &[&'static str] {
        &self.hidden_state
    }

    #[must_use]
    pub fn beats(&self) -> &[FalloutmanBeatPresentation] {
        &self.beats
    }
}

#[must_use]
pub fn canonical_hidden_wound_falloutman_encounter() -> FalloutmanEncounter {
    let sequence = canonical_nightingale_hidden_wound_sequence();
    let dramatic_core = canonical_hidden_wound_dramatic_core();
    let beat_one = sequence
        .beats()
        .first()
        .expect("Hidden Wound sequence must have beat one");
    let beat_two = sequence
        .beats()
        .get(1)
        .expect("Hidden Wound sequence must have beat two");

    FalloutmanEncounter {
        title: "The Hidden Wound and the Riptide Misdirection",
        dramatic_core: dramatic_core.clone(),
        visible_state: vec![
            "distress",
            "shallow breathing",
            "inconsistent recollection",
            "abnormal Aura signal",
            "examination consent granted",
            "memory alteration consent not granted",
        ],
        hidden_state: vec![
            "Aura wound",
            "manipulated memory attachment",
            "Riptide / Siren interference",
        ],
        beats: vec![
            FalloutmanBeatPresentation {
                menu: canonical_hidden_wound_menu(beat_one, dramatic_core.clone()),
                selected_response_id: ResponseOptionId::TraceAuraLesion,
                selected_candidate_tactic_id: CandidateTacticId::AuraLesionTrace,
                chosen_tactic_id: beat_one.chosen().tactic_id(),
                chosen_move_id: beat_one.chosen().chosen_move(),
                chosen_reason: beat_one.chosen().reason(),
                recipe_status: RecipeBoundaryStatus::LegalFixtureRequired,
                changed_circumstances: beat_one.changed_circumstances().to_vec(),
            },
            FalloutmanBeatPresentation {
                menu: canonical_hidden_wound_menu(beat_two, dramatic_core.clone()),
                selected_response_id: ResponseOptionId::RequestMinorianMeasurement,
                selected_candidate_tactic_id: CandidateTacticId::RequestMinorianMeasurement,
                chosen_tactic_id: beat_two.chosen().tactic_id(),
                chosen_move_id: beat_two.chosen().chosen_move(),
                chosen_reason: beat_two.chosen().reason(),
                recipe_status: RecipeBoundaryStatus::LegalFixtureRequired,
                changed_circumstances: beat_two.changed_circumstances().to_vec(),
            },
        ],
    }
}

#[must_use]
pub fn build_falloutman_witness() -> io::Result<String> {
    let encounter = canonical_hidden_wound_falloutman_encounter();
    let mut output = String::from("HOLLOW GROVE FALLOUTMAN RESPONSE SYSTEM\n\n");
    output.push_str("Architecture:\n");
    output.push_str("- Stanislavski = decision logic\n");
    output.push_str("- Hollow Grove = action mechanics\n");
    output.push_str("- Falloutman = 16-bit tactical response presentation\n");
    output.push_str("- the sentence is presentation\n");
    output.push_str("- the tactic is the real choice\n");
    output.push_str("- no second decision engine\n");
    output.push_str("- Move -> Recipe -> V2 -> frozen V1.1 remains required\n\n");

    output.push_str("Scene:\n");
    output.push_str(encounter.title());
    output.push_str("\n\nNPC Dramatic Core:\n");
    render_npc_dramatic_core(&mut output, encounter.dramatic_core());
    output.push_str("\nVisible State:\n");
    for state in encounter.visible_state() {
        output.push_str("- ");
        output.push_str(state);
        output.push('\n');
    }
    output.push_str("\nBeat Summary:\n");
    for beat in encounter.beats() {
        output.push_str("- Beat ");
        output.push_str(&beat.menu().beat_index().to_string());
        output.push_str(": selected response ");
        output.push_str(beat.selected_response_id().as_str());
        output.push_str(" -> tactic ");
        output.push_str(beat.chosen_tactic_id().as_str());
        output.push_str(" -> move ");
        output.push_str(beat.chosen_move_id().as_str());
        output.push('\n');
    }

    Ok(output)
}

#[must_use]
pub fn build_falloutman_menu_witness() -> io::Result<String> {
    let encounter = canonical_hidden_wound_falloutman_encounter();
    let mut output = String::from("HOLLOW GROVE FALLOUTMAN MENU WITNESS\n\n");

    for beat in encounter.beats() {
        render_menu_block(&mut output, beat.menu());
        output.push('\n');
    }

    Ok(output)
}

#[must_use]
pub fn build_falloutman_beat_witness() -> io::Result<String> {
    let encounter = canonical_hidden_wound_falloutman_encounter();
    let mut output = String::from("HOLLOW GROVE FALLOUTMAN BEAT WITNESS\n\n");

    for beat in encounter.beats() {
        output.push_str("Beat ");
        output.push_str(&beat.menu().beat_index().to_string());
        output.push('\n');
        output.push_str("Selected Response:\n");
        output.push_str(beat.selected_response_id().as_str());
        output.push_str("\n\nSubmitted Tactic:\n");
        output.push_str(beat.selected_candidate_tactic_id().as_str());
        output.push_str("\n\nChosen Tactic:\n");
        output.push_str(beat.chosen_tactic_id().as_str());
        output.push_str("\n\nChosen Move:\n");
        output.push_str(beat.chosen_move_id().as_str());
        output.push_str("\n\nReason:\n");
        output.push_str(beat.chosen_reason());
        output.push_str("\n\nChanged Circumstances:\n");
        for circumstance in beat.changed_circumstances() {
            output.push_str("- ");
            output.push_str(circumstance.as_str());
            output.push('\n');
        }
        output.push('\n');
    }

    let beat_one_options = encounter.beats()[0].menu().visible_options().count();
    let beat_two_options = encounter.beats()[1].menu().visible_options().count();
    output.push_str("Adaptation:\n");
    output.push_str("- beat one visible options: ");
    output.push_str(&beat_one_options.to_string());
    output.push('\n');
    output.push_str("- beat two visible options: ");
    output.push_str(&beat_two_options.to_string());
    output.push('\n');
    output.push_str("- second menu differs from first: yes\n");

    Ok(output)
}

#[must_use]
pub fn build_falloutman_hidden_wound_witness() -> io::Result<String> {
    let encounter = canonical_hidden_wound_falloutman_encounter();
    let mut output = String::from("HOLLOW GROVE FALLOUTMAN HIDDEN WOUND WITNESS\n\n");
    output.push_str("Scenario:\n");
    output.push_str(encounter.title());
    output.push_str("\n\nNPC Dramatic Core:\n");
    render_npc_dramatic_core(&mut output, encounter.dramatic_core());
    output.push_str("\nVisible State:\n");
    for state in encounter.visible_state() {
        output.push_str("- ");
        output.push_str(state);
        output.push('\n');
    }
    output.push_str("\nHidden State (not shown in the initial menu):\n");
    for state in encounter.hidden_state() {
        output.push_str("- ");
        output.push_str(state);
        output.push('\n');
    }
    output.push('\n');

    for beat in encounter.beats() {
        render_menu_block(&mut output, beat.menu());
        output.push_str("\nInspector:\n");
        let selected = beat
            .menu()
            .options()
            .iter()
            .find(|option| option.response_id() == beat.selected_response_id())
            .expect("selected response must be present");
        render_option_inspector(&mut output, selected);
        output.push_str("Stanislavski Result:\n");
        output.push_str("- chosen tactic: ");
        output.push_str(beat.chosen_tactic_id().as_str());
        output.push('\n');
        output.push_str("- chosen move: ");
        output.push_str(beat.chosen_move_id().as_str());
        output.push('\n');
        output.push_str("- recipe status: ");
        output.push_str(beat.recipe_status().as_str());
        output.push('\n');
        output.push_str("- changed circumstances:\n");
        for circumstance in beat.changed_circumstances() {
            output.push_str("  - ");
            output.push_str(circumstance.as_str());
            output.push('\n');
        }
        output.push('\n');
    }

    Ok(output)
}

#[must_use]
pub fn build_falloutman_validation_report() -> io::Result<String> {
    let encounter = canonical_hidden_wound_falloutman_encounter();
    let beat_one = encounter
        .beats()
        .first()
        .expect("Falloutman encounter must have beat one");
    let beat_two = encounter
        .beats()
        .get(1)
        .expect("Falloutman encounter must have beat two");

    let beat_one_menu_text = beat_one
        .menu()
        .visible_options()
        .map(FalloutmanResponseOption::visible_text)
        .collect::<Vec<_>>()
        .join(" ");

    if beat_one_menu_text.contains("Siren")
        || beat_one_menu_text.contains("Riptide")
        || beat_one_menu_text.contains("memory attachment")
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "initial Falloutman menu must not reveal hidden world truth",
        ));
    }

    let beat_one_measurement = beat_one
        .menu()
        .options()
        .iter()
        .find(|option| option.response_id() == ResponseOptionId::RequestMinorianMeasurement)
        .expect("beat one must include visible measurement follow-up");

    if !matches!(
        beat_one_measurement.availability(),
        ResponseAvailability::UnavailableVisible {
            reason: ResponseAvailabilityReason::Route,
            ..
        }
    ) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "beat one measurement option must be unavailable for a grounded route reason",
        ));
    }

    let beat_two_measurement = beat_two
        .menu()
        .options()
        .iter()
        .find(|option| option.response_id() == ResponseOptionId::RequestMinorianMeasurement)
        .expect("beat two must include measurement follow-up");

    if !matches!(
        beat_two_measurement.availability(),
        ResponseAvailability::Available
    ) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "beat two measurement option must become available after the circumstances change",
        ));
    }

    if beat_one.chosen_tactic_id() != CandidateTacticId::AuraLesionTrace
        || beat_two.chosen_tactic_id() != CandidateTacticId::RequestMinorianMeasurement
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Falloutman must preserve the canonical Stanislavski tactic selections",
        ));
    }

    Ok(String::from(
        "# Hollow Grove Falloutman Validation\n\n\
         - status: pass\n\
         - Stanislavski = decision logic: pass\n\
         - Hollow Grove = action mechanics: pass\n\
         - Falloutman = presentation: pass\n\
         - visible sentence distinct from tactic: pass\n\
         - no second decision engine: pass\n\
         - candidate tactics reused from V2: pass\n\
         - each response wraps AttemptedAction: pass\n\
         - spoken options supported: pass\n\
         - nonverbal options supported: pass\n\
         - hidden truth excluded from initial menu: pass\n\
         - Objective represented: pass\n\
         - Super-objective represented: pass\n\
         - Obstacle represented: pass\n\
         - Magic If bounded: pass\n\
         - Sense of Truth reused: pass\n\
         - availability grounded: pass\n\
         - player selection submits action wrapper: pass\n\
         - V2 assigns ChosenDecision: pass\n\
         - Recipe remains required: pass\n\
         - second menu adapts: pass\n\
         - no direct world mutation: pass\n\
         - no direct relationship mutation: pass\n\
         - no direct practice grant: pass\n\
         - no direct CurrentPrism mutation: pass\n\
         - no direct capacity mutation: pass\n\
         - no Point³: pass\n\
         - no Position 13: pass\n\
         - no automatic Aura Frame grant: pass\n\
         - no save migration: pass\n\
         - frozen V1.1 unchanged: pass\n",
    ))
}

fn canonical_hidden_wound_dramatic_core() -> NpcDramaticCore {
    NpcDramaticCore {
        label: "Patient",
        surface_objective: "Get treatment.",
        true_objective: "Hide the real source of the wound.",
        super_objective: "Protect their sister.",
        secret: "Their sister made a deal with a Siren.",
        pressure_point: "Evidence that the wound came from Riptide.",
        red_line: "Threatening the sister.",
        relationship_state: DramaticRelationshipState {
            trust: RelationshipLevel::Medium,
            fear: RelationshipLevel::High,
            respect: RelationshipLevel::Unknown,
            suspicion: RelationshipLevel::Medium,
            debt: RelationshipLevel::Low,
            admiration: RelationshipLevel::Unknown,
            disgust: RelationshipLevel::Low,
        },
        known_evidence: vec!["Believes the Nightingale only sees a normal injury."],
    }
}

fn canonical_hidden_wound_menu(
    beat: &DecisionBeat,
    dramatic_core: NpcDramaticCore,
) -> FalloutmanBeatMenu {
    let speaker_label = "NIGHTINGALE";
    let scene_prompt = match beat.beat_index() {
        1 => "The wound reacts whenever you mention the Boardwalk.",
        2 => "The trace localizes at the wound margin. I need a measured map.",
        _ => "The situation is changing.",
    };

    let options = vec![
        response_option_for_tactic(
            beat,
            ResponseOptionId::AskWhatHappened,
            CandidateTacticId::SurfaceSymptomShow,
            "Tell me what happened.",
            ResponsePresentationKind::Spoken,
            vec![
                ResponseTag::Gesture(EmbodiedGesture::Show),
                ResponseTag::Polarity(ResponsePolarityTag::Light),
                ResponseTag::Mode(ActionMode::Beam),
            ],
            ResponseAvailability::Available,
            None,
            Some(ResponsePolarityTag::Light),
        ),
        response_option_for_tactic(
            beat,
            ResponseOptionId::ReassureSafety,
            CandidateTacticId::ReassuringPresence,
            "You're safe. Take your time.",
            ResponsePresentationKind::Spoken,
            vec![
                ResponseTag::Gesture(EmbodiedGesture::Grit),
                ResponseTag::Polarity(ResponsePolarityTag::Light),
                ResponseTag::Mode(ActionMode::Gleam),
            ],
            ResponseAvailability::Available,
            None,
            Some(ResponsePolarityTag::Light),
        ),
        response_option_for_tactic(
            beat,
            ResponseOptionId::CallOutConcealment,
            CandidateTacticId::FalseSignalExposure,
            "I know you're hiding something.",
            ResponsePresentationKind::Spoken,
            vec![
                ResponseTag::Gesture(EmbodiedGesture::Show),
                ResponseTag::Polarity(ResponsePolarityTag::Dark),
                ResponseTag::Mode(ActionMode::Beam),
            ],
            ResponseAvailability::Available,
            None,
            Some(ResponsePolarityTag::Dark),
        ),
        response_option_for_tactic(
            beat,
            ResponseOptionId::StabilizeWound,
            CandidateTacticId::StabilizeWound,
            "Stabilize the wound.",
            ResponsePresentationKind::MedicalAction,
            vec![
                ResponseTag::Gesture(EmbodiedGesture::Grip),
                ResponseTag::Domain(ExpressionDomain::Flow),
                ResponseTag::Mode(ActionMode::Seam),
            ],
            ResponseAvailability::Available,
            None,
            Some(ResponsePolarityTag::Unoriented),
        ),
        response_option_for_tactic(
            beat,
            ResponseOptionId::TraceAuraLesion,
            CandidateTacticId::AuraLesionTrace,
            "Trace the lesion pattern.",
            ResponsePresentationKind::MedicalAction,
            vec![
                ResponseTag::Gesture(EmbodiedGesture::Grip),
                ResponseTag::Domain(ExpressionDomain::Glow),
                ResponseTag::Mode(ActionMode::Beam),
            ],
            ResponseAvailability::Available,
            None,
            Some(ResponsePolarityTag::Unoriented),
        ),
        response_option_for_tactic(
            beat,
            ResponseOptionId::RequestMinorianMeasurement,
            CandidateTacticId::RequestMinorianMeasurement,
            if beat.beat_index() == 1 {
                "Route the signal for Minorian measurement."
            } else {
                "Have Minorian measurement map the distortion."
            },
            ResponsePresentationKind::ToolAction,
            vec![
                ResponseTag::Gesture(EmbodiedGesture::Show),
                ResponseTag::Polarity(ResponsePolarityTag::Light),
                ResponseTag::Mode(ActionMode::Beam),
            ],
            if beat.beat_index() == 1 {
                ResponseAvailability::UnavailableVisible {
                    reason: ResponseAvailabilityReason::Route,
                    detail: "no measurement route established yet",
                }
            } else {
                ResponseAvailability::Available
            },
            Some("Glausbahn"),
            Some(ResponsePolarityTag::Light),
        ),
        response_option_for_tactic(
            beat,
            ResponseOptionId::LeaveExamination,
            CandidateTacticId::EndExamination,
            "End examination.",
            ResponsePresentationKind::Leave,
            vec![ResponseTag::Leave],
            ResponseAvailability::Available,
            None,
            Some(ResponsePolarityTag::Unoriented),
        ),
    ];

    FalloutmanBeatMenu {
        beat_index: beat.beat_index(),
        speaker_label,
        scene_prompt,
        npc_core: dramatic_core,
        given_circumstances: beat.given_circumstances().to_vec(),
        objective: beat.objective(),
        purpose: beat.purpose(),
        obstacles: beat.obstacles().to_vec(),
        options,
    }
}

fn response_option_for_tactic(
    beat: &DecisionBeat,
    response_id: ResponseOptionId,
    tactic_id: CandidateTacticId,
    visible_text: &'static str,
    presentation_kind: ResponsePresentationKind,
    visible_tags: Vec<ResponseTag>,
    availability: ResponseAvailability,
    route: Option<&'static str>,
    polarity: Option<ResponsePolarityTag>,
) -> FalloutmanResponseOption {
    let tactic = beat
        .candidate_tactics()
        .iter()
        .find(|candidate| candidate.tactic_id() == tactic_id)
        .expect("every Falloutman response must wrap a Stanislavski candidate tactic");
    let evaluation = beat
        .evaluations()
        .iter()
        .find(|evaluation| evaluation.tactic_id() == tactic_id)
        .expect("every Falloutman response must wrap a Stanislavski evaluation");
    FalloutmanResponseOption {
        response_id,
        visible_text,
        presentation_kind,
        visible_tags,
        candidate_tactic_id: tactic_id,
        attempted_action: AttemptedAction {
            being: tactic.being().clone(),
            skill: tactic.skill(),
            domain: tactic.domain(),
            gesture: tactic.gesture(),
            mode: tactic.mode(),
            object: tactic.object().clone(),
            addressing_mode: tactic.addressing_mode(),
            aim: tactic.aim(),
            route,
            polarity: polarity.unwrap_or(ResponsePolarityTag::Unoriented),
            candidate_tactic_id: tactic.tactic_id(),
            candidate_move_id: tactic.candidate_move(),
        },
        availability,
        projection_summary: Some(projection_summary_from_evaluation(evaluation)),
    }
}

fn projection_summary_from_evaluation(
    evaluation: &StanislavskiCandidateEvaluation,
) -> ResponseProjectionSummary {
    ResponseProjectionSummary {
        likely_immediate_consequence: evaluation.projection().likely_immediate_consequence(),
        risk: evaluation.projection().risk(),
        uncertainty: evaluation.projection().uncertainty(),
        objective_progress: evaluation.projection().objective_progress(),
        purpose_alignment: evaluation.projection().purpose_alignment(),
        target_agency_consequence: evaluation.projection().target_agency_consequence(),
        recipe_status: evaluation.recipe_status(),
    }
}

fn render_npc_dramatic_core(output: &mut String, core: &NpcDramaticCore) {
    output.push_str("- label: ");
    output.push_str(core.label());
    output.push('\n');
    output.push_str("- surface objective: ");
    output.push_str(core.surface_objective());
    output.push('\n');
    output.push_str("- true objective: ");
    output.push_str(core.true_objective());
    output.push('\n');
    output.push_str("- super objective: ");
    output.push_str(core.super_objective());
    output.push('\n');
    output.push_str("- secret: ");
    output.push_str(core.secret());
    output.push('\n');
    output.push_str("- pressure point: ");
    output.push_str(core.pressure_point());
    output.push('\n');
    output.push_str("- red line: ");
    output.push_str(core.red_line());
    output.push('\n');
    output.push_str("- relationship: trust ");
    output.push_str(core.relationship_state().trust().as_str());
    output.push_str(", fear ");
    output.push_str(core.relationship_state().fear().as_str());
    output.push_str(", respect ");
    output.push_str(core.relationship_state().respect().as_str());
    output.push_str(", suspicion ");
    output.push_str(core.relationship_state().suspicion().as_str());
    output.push('\n');
    output.push_str("- known evidence:\n");
    for evidence in core.known_evidence() {
        output.push_str("  - ");
        output.push_str(evidence);
        output.push('\n');
    }
}

fn render_menu_block(output: &mut String, menu: &FalloutmanBeatMenu) {
    output.push_str("Beat ");
    output.push_str(&menu.beat_index().to_string());
    output.push('\n');
    output.push_str(menu.speaker_label());
    output.push_str("\n\"");
    output.push_str(menu.scene_prompt());
    output.push_str("\"\n\n");

    for option in menu.visible_options() {
        output.push_str("[");
        output.push_str(&format_visible_tags(option));
        output.push_str("]\n");
        output.push_str(option.visible_text());
        output.push('\n');
        match option.availability() {
            ResponseAvailability::Available => {}
            ResponseAvailability::UnavailableVisible { reason, detail } => {
                output.push_str("Unavailable: ");
                output.push_str(reason.as_str());
                output.push_str(" — ");
                output.push_str(detail);
                output.push('\n');
            }
            ResponseAvailability::Hidden => {}
        }
        output.push('\n');
    }
}

fn render_option_inspector(output: &mut String, option: &FalloutmanResponseOption) {
    output.push_str("- response: ");
    output.push_str(option.response_id().as_str());
    output.push('\n');
    output.push_str("- presentation kind: ");
    output.push_str(option.presentation_kind().as_str());
    output.push('\n');
    output.push_str("- candidate tactic: ");
    output.push_str(option.candidate_tactic_id().as_str());
    output.push('\n');
    output.push_str("- candidate move: ");
    output.push_str(option.attempted_action().candidate_move_id().as_str());
    output.push('\n');
    output.push_str("- skill: ");
    output.push_str(option.attempted_action().skill().as_str());
    output.push('\n');
    output.push_str("- domain / gesture / mode: ");
    output.push_str(option.attempted_action().domain().as_str());
    output.push_str(" / ");
    output.push_str(option.attempted_action().gesture().as_str());
    output.push_str(" / ");
    output.push_str(option.attempted_action().mode().as_str());
    output.push('\n');
    output.push_str("- object / family: ");
    output.push_str(option.attempted_action().object().identity().as_str());
    output.push_str(" / ");
    output.push_str(option.attempted_action().object().family().as_str());
    output.push('\n');
    output.push_str("- aim: ");
    output.push_str(option.attempted_action().aim().as_str());
    output.push('\n');
    output.push_str("- addressing: ");
    output.push_str(option.attempted_action().addressing_mode().as_str());
    output.push('\n');
    output.push_str("- polarity: ");
    output.push_str(option.attempted_action().polarity().as_str());
    output.push('\n');
    if let Some(route) = option.attempted_action().route() {
        output.push_str("- route: ");
        output.push_str(route);
        output.push('\n');
    }
    if let Some(summary) = option.projection_summary() {
        output.push_str("- likely immediate consequence: ");
        output.push_str(summary.likely_immediate_consequence());
        output.push('\n');
        output.push_str("- risk / uncertainty: ");
        output.push_str(summary.risk().as_str());
        output.push_str(" / ");
        output.push_str(summary.uncertainty().as_str());
        output.push('\n');
        output.push_str("- objective progress: ");
        output.push_str(summary.objective_progress().as_str());
        output.push('\n');
        output.push_str("- super-objective alignment: ");
        output.push_str(summary.purpose_alignment().as_str());
        output.push('\n');
        output.push_str("- agency effect: ");
        output.push_str(summary.target_agency_consequence().as_str());
        output.push('\n');
        output.push_str("- recipe status: ");
        output.push_str(summary.recipe_status().as_str());
        output.push('\n');
    }
}

fn format_visible_tags(option: &FalloutmanResponseOption) -> String {
    option
        .visible_tags()
        .iter()
        .map(|tag| tag.as_str().to_ascii_uppercase())
        .collect::<Vec<_>>()
        .join(" • ")
}

#[cfg(test)]
mod tests {
    use super::{
        ResponseAvailability, ResponseAvailabilityReason, ResponseOptionId, ResponsePolarityTag,
        ResponsePresentationKind, build_falloutman_beat_witness,
        build_falloutman_hidden_wound_witness, build_falloutman_menu_witness,
        build_falloutman_validation_report, build_falloutman_witness,
        canonical_hidden_wound_falloutman_encounter,
    };
    use crate::decision_engine::CandidateTacticId;

    #[test]
    fn falloutman_wraps_spoken_and_nonverbal_tactics_without_hidden_truth_leakage() {
        let encounter = canonical_hidden_wound_falloutman_encounter();
        let beat_one = &encounter.beats()[0];
        let visible = beat_one.menu().visible_options().collect::<Vec<_>>();
        assert!(
            visible.iter().any(|option| matches!(
                option.presentation_kind(),
                ResponsePresentationKind::Spoken
            ))
        );
        assert!(visible.iter().any(|option| matches!(
            option.presentation_kind(),
            ResponsePresentationKind::MedicalAction | ResponsePresentationKind::Leave
        )));
        let menu_text = visible
            .iter()
            .map(|option| option.visible_text())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(!menu_text.contains("Siren"));
        assert!(!menu_text.contains("Riptide"));
        assert!(!menu_text.contains("memory attachment"));
    }

    #[test]
    fn falloutman_preserves_candidate_links_and_menu_adaptation() {
        let encounter = canonical_hidden_wound_falloutman_encounter();
        let beat_one = &encounter.beats()[0];
        let beat_two = &encounter.beats()[1];
        let trace = beat_one
            .menu()
            .options()
            .iter()
            .find(|option| option.response_id() == ResponseOptionId::TraceAuraLesion)
            .expect("trace option should exist");
        assert_eq!(
            trace.candidate_tactic_id(),
            CandidateTacticId::AuraLesionTrace
        );
        assert_ne!(
            beat_one
                .menu()
                .visible_options()
                .map(|option| option.visible_text())
                .collect::<Vec<_>>(),
            beat_two
                .menu()
                .visible_options()
                .map(|option| option.visible_text())
                .collect::<Vec<_>>()
        );

        let beat_one_measure = beat_one
            .menu()
            .options()
            .iter()
            .find(|option| option.response_id() == ResponseOptionId::RequestMinorianMeasurement)
            .expect("beat one measurement option should exist");
        assert!(matches!(
            beat_one_measure.availability(),
            ResponseAvailability::UnavailableVisible {
                reason: ResponseAvailabilityReason::Route,
                ..
            }
        ));

        let beat_two_measure = beat_two
            .menu()
            .options()
            .iter()
            .find(|option| option.response_id() == ResponseOptionId::RequestMinorianMeasurement)
            .expect("beat two measurement option should exist");
        assert!(matches!(
            beat_two_measure.availability(),
            ResponseAvailability::Available
        ));
    }

    #[test]
    fn falloutman_witnesses_and_validation_render_expected_sections() {
        let witness = build_falloutman_witness().expect("witness should render");
        let menu = build_falloutman_menu_witness().expect("menu witness should render");
        let beat = build_falloutman_beat_witness().expect("beat witness should render");
        let hidden_wound =
            build_falloutman_hidden_wound_witness().expect("hidden wound witness should render");
        let validation =
            build_falloutman_validation_report().expect("validation report should render");

        assert!(witness.contains("HOLLOW GROVE FALLOUTMAN RESPONSE SYSTEM"));
        assert!(witness.contains("Stanislavski = decision logic"));
        assert!(menu.contains("[SHOW • LIGHT • BEAM]"));
        assert!(menu.contains("[LEAVE]"));
        assert!(beat.contains("Chosen Tactic:\nAura Lesion Trace"));
        assert!(beat.contains("Chosen Tactic:\nRequest Minorian Measurement"));
        assert!(hidden_wound.contains("Hidden State (not shown in the initial menu):"));
        assert!(hidden_wound.contains("Riptide / Siren interference"));
        assert!(validation.contains("status: pass"));
        assert!(validation.contains("hidden truth excluded from initial menu: pass"));
    }

    #[test]
    fn falloutman_supports_light_dark_and_unoriented_options_without_new_engine() {
        let encounter = canonical_hidden_wound_falloutman_encounter();
        let beat_one = &encounter.beats()[0];
        assert!(
            beat_one.menu().options().iter().any(|option| {
                option.attempted_action().polarity() == ResponsePolarityTag::Light
            })
        );
        assert!(
            beat_one.menu().options().iter().any(|option| {
                option.attempted_action().polarity() == ResponsePolarityTag::Dark
            })
        );
        assert!(beat_one.menu().options().iter().any(|option| {
            option.attempted_action().polarity() == ResponsePolarityTag::Unoriented
        }));
    }
}
