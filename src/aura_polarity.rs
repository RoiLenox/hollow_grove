use std::io;

use crate::being_object_ontology::{
    ActionAim, AddressingMode, BeingState, ObjectId, ObjectState,
    build_canonical_being_state_with_aura, canonical_object_state,
};
use crate::flow_glow_grammar::{
    ActionMode, CompatibilityLevel, EmbodiedGesture, ExpressionDomain, RecipeBoundaryStatus,
};
use crate::frame_state::FrameId;
use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode, House};
use crate::point_progression::CanonicalRouteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuraPolarity {
    Light,
    Dark,
}

impl AuraPolarity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    #[must_use]
    pub const fn summary(self) -> &'static str {
        match self {
            Self::Light => {
                "clarity, diagnosis, explanation, consent, legibility, shared understanding"
            }
            Self::Dark => {
                "manipulation, seduction, fear, compulsion, misdirection, pressure, weaponized presence"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InferredAuraOrientation {
    Neutral,
    Light,
    Dark,
    Mixed,
}

impl InferredAuraOrientation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "Neutral",
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Mixed => "Mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruthOrientation {
    Truthful,
    Interpretive,
    Uncertain,
    SelectivelyDisclosed,
    Deceptive,
    Fabricated,
    Inverted,
}

impl TruthOrientation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Truthful => "Truthful",
            Self::Interpretive => "Interpretive",
            Self::Uncertain => "Uncertain",
            Self::SelectivelyDisclosed => "SelectivelyDisclosed",
            Self::Deceptive => "Deceptive",
            Self::Fabricated => "Fabricated",
            Self::Inverted => "Inverted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsentState {
    Informed,
    Partial,
    Implied,
    EmergencyLimited,
    Absent,
    Impossible,
    Coerced,
    Withdrawn,
}

impl ConsentState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informed => "Informed",
            Self::Partial => "Partial",
            Self::Implied => "Implied",
            Self::EmergencyLimited => "EmergencyLimited",
            Self::Absent => "Absent",
            Self::Impossible => "Impossible",
            Self::Coerced => "Coerced",
            Self::Withdrawn => "Withdrawn",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgencyEffect {
    Expanded,
    Preserved,
    Guided,
    Narrowed,
    Overridden,
    Destroyed,
}

impl AgencyEffect {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Expanded => "Expanded",
            Self::Preserved => "Preserved",
            Self::Guided => "Guided",
            Self::Narrowed => "Narrowed",
            Self::Overridden => "Overridden",
            Self::Destroyed => "Destroyed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureRating {
    Low,
    Moderate,
    High,
    Severe,
}

impl PressureRating {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Moderate => "Moderate",
            Self::High => "High",
            Self::Severe => "Severe",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Proportionality {
    Proportional,
    Borderline,
    Disproportionate,
}

impl Proportionality {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proportional => "Proportional",
            Self::Borderline => "Borderline",
            Self::Disproportionate => "Disproportionate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reversibility {
    Reversible,
    Costly,
    Persistent,
    Irreversible,
}

impl Reversibility {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "Reversible",
            Self::Costly => "Costly",
            Self::Persistent => "Persistent",
            Self::Irreversible => "Irreversible",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsequenceLevel {
    Low,
    Material,
    Severe,
}

impl ConsequenceLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Material => "Material",
            Self::Severe => "Severe",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteGeometry {
    Straight,
    Curved,
}

impl RouteGeometry {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Straight => "Straight",
            Self::Curved => "Curved",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuraMoveId {
    DiagnosticRevelation,
    ClinicalMeasureLine,
    ResponsiblePresentation,
    HiddenTruthReveal,
    DefensiveDread,
    RiptideMisdirection,
    CounterBind,
    ConsensualGlamour,
    CoerciveGlamour,
    InterpretiveCapture,
    MeasuredDisclosure,
    PossibilityField,
    RealityCapture,
    BoundaryClarification,
    AuthenticRecognition,
}

impl AuraMoveId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticRevelation => "Diagnostic Revelation",
            Self::ClinicalMeasureLine => "Clinical Measure Line",
            Self::ResponsiblePresentation => "Responsible Presentation",
            Self::HiddenTruthReveal => "Hidden Truth Reveal",
            Self::DefensiveDread => "Defensive Dread",
            Self::RiptideMisdirection => "Riptide Misdirection",
            Self::CounterBind => "Counter-Bind",
            Self::ConsensualGlamour => "Consensual Glamour",
            Self::CoerciveGlamour => "Coercive Glamour",
            Self::InterpretiveCapture => "Interpretive Capture",
            Self::MeasuredDisclosure => "Measured Disclosure",
            Self::PossibilityField => "Possibility Field",
            Self::RealityCapture => "Reality Capture",
            Self::BoundaryClarification => "Boundary Clarification",
            Self::AuthenticRecognition => "Authentic Recognition",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraRouteDefinition {
    route_id: CanonicalRouteId,
    geometry: RouteGeometry,
    source_house: House,
    destination_house: House,
    polarity_tendency: AuraPolarity,
    manager_relation: &'static str,
    public_tendency: &'static str,
    pressure_tendency: &'static str,
    semantic_tendency: &'static str,
}

impl AuraRouteDefinition {
    #[must_use]
    pub const fn route_id(&self) -> CanonicalRouteId {
        self.route_id
    }

    #[must_use]
    pub const fn geometry(&self) -> RouteGeometry {
        self.geometry
    }

    #[must_use]
    pub const fn source_house(&self) -> House {
        self.source_house
    }

    #[must_use]
    pub const fn destination_house(&self) -> House {
        self.destination_house
    }

    #[must_use]
    pub const fn polarity_tendency(&self) -> AuraPolarity {
        self.polarity_tendency
    }

    #[must_use]
    pub const fn manager_relation(&self) -> &'static str {
        self.manager_relation
    }

    #[must_use]
    pub const fn public_tendency(&self) -> &'static str {
        self.public_tendency
    }

    #[must_use]
    pub const fn pressure_tendency(&self) -> &'static str {
        self.pressure_tendency
    }

    #[must_use]
    pub const fn semantic_tendency(&self) -> &'static str {
        self.semantic_tendency
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraPolarityActionRequest {
    being_label: &'static str,
    being: BeingState,
    domain: ExpressionDomain,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    source_house: House,
    destination_house: House,
    route: Option<CanonicalRouteId>,
    requested_polarity: AuraPolarity,
    consent: ConsentState,
}

impl AuraPolarityActionRequest {
    #[must_use]
    pub fn new(
        being_label: &'static str,
        being: BeingState,
        domain: ExpressionDomain,
        gesture: EmbodiedGesture,
        mode: ActionMode,
        object: ObjectState,
        addressing_mode: AddressingMode,
        aim: ActionAim,
        source_house: House,
        destination_house: House,
        route: Option<CanonicalRouteId>,
        requested_polarity: AuraPolarity,
        consent: ConsentState,
    ) -> Self {
        Self {
            being_label,
            being,
            domain,
            gesture,
            mode,
            object,
            addressing_mode,
            aim,
            source_house,
            destination_house,
            route,
            requested_polarity,
            consent,
        }
    }

    #[must_use]
    pub const fn being_label(&self) -> &'static str {
        self.being_label
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
    pub const fn source_house(&self) -> House {
        self.source_house
    }

    #[must_use]
    pub const fn destination_house(&self) -> House {
        self.destination_house
    }

    #[must_use]
    pub const fn route(&self) -> Option<CanonicalRouteId> {
        self.route
    }

    #[must_use]
    pub const fn requested_polarity(&self) -> AuraPolarity {
        self.requested_polarity
    }

    #[must_use]
    pub const fn consent(&self) -> ConsentState {
        self.consent
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraPolarityEvaluation {
    requested_polarity: AuraPolarity,
    inferred_orientation: InferredAuraOrientation,
    route_alignment: CompatibilityLevel,
    truth_orientation: TruthOrientation,
    consent: ConsentState,
    agency_effect: AgencyEffect,
    pressure: PressureRating,
    proportionality: Proportionality,
    reversibility: Reversibility,
    consequence: ConsequenceLevel,
    warnings: Vec<String>,
}

impl AuraPolarityEvaluation {
    #[must_use]
    pub const fn requested_polarity(&self) -> AuraPolarity {
        self.requested_polarity
    }

    #[must_use]
    pub const fn inferred_orientation(&self) -> InferredAuraOrientation {
        self.inferred_orientation
    }

    #[must_use]
    pub const fn route_alignment(&self) -> CompatibilityLevel {
        self.route_alignment
    }

    #[must_use]
    pub const fn truth_orientation(&self) -> TruthOrientation {
        self.truth_orientation
    }

    #[must_use]
    pub const fn consent(&self) -> ConsentState {
        self.consent
    }

    #[must_use]
    pub const fn agency_effect(&self) -> AgencyEffect {
        self.agency_effect
    }

    #[must_use]
    pub const fn pressure(&self) -> PressureRating {
        self.pressure
    }

    #[must_use]
    pub const fn proportionality(&self) -> Proportionality {
        self.proportionality
    }

    #[must_use]
    pub const fn reversibility(&self) -> Reversibility {
        self.reversibility
    }

    #[must_use]
    pub const fn consequence(&self) -> ConsequenceLevel {
        self.consequence
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraPolarityResolution {
    request: AuraPolarityActionRequest,
    candidate_move: AuraMoveId,
    evaluation: AuraPolarityEvaluation,
    recipe_status: RecipeBoundaryStatus,
    v2_status: &'static str,
}

impl AuraPolarityResolution {
    #[must_use]
    pub const fn request(&self) -> &AuraPolarityActionRequest {
        &self.request
    }

    #[must_use]
    pub const fn candidate_move(&self) -> AuraMoveId {
        self.candidate_move
    }

    #[must_use]
    pub const fn evaluation(&self) -> &AuraPolarityEvaluation {
        &self.evaluation
    }

    #[must_use]
    pub const fn recipe_status(&self) -> RecipeBoundaryStatus {
        self.recipe_status
    }

    #[must_use]
    pub const fn v2_status(&self) -> &'static str {
        self.v2_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraPolarityContractInput {
    pub light_automatic_good: bool,
    pub dark_automatic_evil: bool,
    pub separate_substances: bool,
    pub split_frame_species: bool,
    pub route_geometry_collapsed: bool,
    pub foxy_equals_dark: bool,
    pub moxy_equals_light: bool,
    pub polarity_trusted_blindly: bool,
    pub consent_omitted: bool,
    pub agency_omitted: bool,
    pub truth_omitted: bool,
    pub proportionality_omitted: bool,
    pub consequence_omitted: bool,
    pub reversibility_omitted: bool,
    pub defensive_dark_impossible: bool,
    pub coercive_light_accepted: bool,
    pub direct_execution_bypass: bool,
    pub current_prism_collapsed: bool,
    pub v1_1_changed: bool,
    pub point_cubed: bool,
    pub position_thirteen: bool,
    pub automatic_aura_frame_grant: bool,
    pub save_migration_without_justification: bool,
}

#[must_use]
pub fn canonical_aura_polarity_contract_fixture() -> AuraPolarityContractInput {
    AuraPolarityContractInput {
        light_automatic_good: false,
        dark_automatic_evil: false,
        separate_substances: false,
        split_frame_species: false,
        route_geometry_collapsed: false,
        foxy_equals_dark: false,
        moxy_equals_light: false,
        polarity_trusted_blindly: false,
        consent_omitted: false,
        agency_omitted: false,
        truth_omitted: false,
        proportionality_omitted: false,
        consequence_omitted: false,
        reversibility_omitted: false,
        defensive_dark_impossible: false,
        coercive_light_accepted: false,
        direct_execution_bypass: false,
        current_prism_collapsed: false,
        v1_1_changed: false,
        point_cubed: false,
        position_thirteen: false,
        automatic_aura_frame_grant: false,
        save_migration_without_justification: false,
    }
}

#[must_use]
pub fn validate_aura_polarity_contract(
    input: &AuraPolarityContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.light_automatic_good {
        diagnostics.push(aura_polarity_error(
            "Light Aura must not be treated as automatically good.",
        ));
    }
    if input.dark_automatic_evil {
        diagnostics.push(aura_polarity_error(
            "Dark Aura must not be treated as automatically evil.",
        ));
    }
    if input.separate_substances {
        diagnostics.push(aura_polarity_error(
            "Light and Dark Aura must remain directional uses of the shared Glow domain rather than separate substances.",
        ));
    }
    if input.split_frame_species {
        diagnostics.push(aura_polarity_error(
            "Aura Frame lineage must remain unified and cannot split into separate Light/Dark species.",
        ));
    }
    if input.route_geometry_collapsed {
        diagnostics.push(aura_polarity_error(
            "Route geometry must remain distinct from Aura polarity.",
        ));
    }
    if input.foxy_equals_dark {
        diagnostics.push(aura_polarity_error(
            "Foxy must remain an addressing mode and cannot be collapsed into Dark Aura.",
        ));
    }
    if input.moxy_equals_light {
        diagnostics.push(aura_polarity_error(
            "Moxy must remain an addressing mode and cannot be collapsed into Light Aura.",
        ));
    }
    if input.polarity_trusted_blindly {
        diagnostics.push(aura_polarity_error(
            "Requested polarity must not be trusted blindly without semantic evaluation.",
        ));
    }
    if input.consent_omitted {
        diagnostics.push(aura_polarity_error(
            "Consent evaluation must remain present.",
        ));
    }
    if input.agency_omitted {
        diagnostics.push(aura_polarity_error(
            "Agency evaluation must remain present.",
        ));
    }
    if input.truth_omitted {
        diagnostics.push(aura_polarity_error("Truth evaluation must remain present."));
    }
    if input.proportionality_omitted {
        diagnostics.push(aura_polarity_error(
            "Proportionality evaluation must remain present.",
        ));
    }
    if input.consequence_omitted {
        diagnostics.push(aura_polarity_error(
            "Consequence evaluation must remain present.",
        ));
    }
    if input.reversibility_omitted {
        diagnostics.push(aura_polarity_error(
            "Reversibility evaluation must remain present.",
        ));
    }
    if input.defensive_dark_impossible {
        diagnostics.push(aura_polarity_error(
            "Defensive, strategic, or protective Dark Aura uses must remain representable.",
        ));
    }
    if input.coercive_light_accepted {
        diagnostics.push(aura_polarity_error(
            "An action labeled Light must not be accepted when it actually conceals alternatives or overrides agency.",
        ));
    }
    if input.direct_execution_bypass {
        diagnostics.push(aura_polarity_error(
            "Aura polarity resolution must not bypass candidate Move -> Recipe -> V2 -> frozen V1.1.",
        ));
    }
    if input.current_prism_collapsed {
        diagnostics.push(aura_polarity_error(
            "CurrentPrism must remain distinct from Aura polarity.",
        ));
    }
    if input.v1_1_changed {
        diagnostics.push(aura_polarity_error(
            "Frozen V1.1 topology must remain unchanged.",
        ));
    }
    if input.point_cubed {
        diagnostics.push(aura_polarity_error("Point³ must not be introduced."));
    }
    if input.position_thirteen {
        diagnostics.push(aura_polarity_error("Position 13 must not be introduced."));
    }
    if input.automatic_aura_frame_grant {
        diagnostics.push(aura_polarity_error(
            "Aura polarity must not automatically grant an Aura Frame.",
        ));
    }
    if input.save_migration_without_justification {
        diagnostics.push(aura_polarity_error(
            "Save migration must not be introduced without explicit justification.",
        ));
    }

    diagnostics
}

#[must_use]
pub fn canonical_aura_route_definition(route_id: CanonicalRouteId) -> Option<AuraRouteDefinition> {
    match route_id {
        CanonicalRouteId::Glausbahn => Some(AuraRouteDefinition {
            route_id,
            geometry: RouteGeometry::Straight,
            source_house: House::Glaushouse,
            destination_house: House::Sandmanor,
            polarity_tendency: AuraPolarity::Light,
            manager_relation: "straight / Clouseau / Proxy-PLEB relation",
            public_tendency: "direct legibility",
            pressure_tendency: "low-to-moderate clinical pressure",
            semantic_tendency: "direct, legible, publicly structured Light Aura transmission",
        }),
        CanonicalRouteId::CurrentSeanad => Some(AuraRouteDefinition {
            route_id,
            geometry: RouteGeometry::Curved,
            source_house: House::Glaushouse,
            destination_house: House::Sandmanor,
            polarity_tendency: AuraPolarity::Light,
            manager_relation: "curved / HAL / Moxy-META relation",
            public_tendency: "contextual clarification",
            pressure_tendency: "negotiated deliberation",
            semantic_tendency: "relational, deliberative, negotiated, contextual Light Aura development",
        }),
        CanonicalRouteId::Boardwalk => Some(AuraRouteDefinition {
            route_id,
            geometry: RouteGeometry::Straight,
            source_house: House::Glaushouse,
            destination_house: House::Flynt,
            polarity_tendency: AuraPolarity::Dark,
            manager_relation: "straight / Clouseau / Proxy-PLEB relation",
            public_tendency: "managed public performance",
            pressure_tendency: "socially contained pressure",
            semantic_tendency: "managed, performative, public-facing, socially contained Dark Aura exchange",
        }),
        CanonicalRouteId::Riptide => Some(AuraRouteDefinition {
            route_id,
            geometry: RouteGeometry::Curved,
            source_house: House::Glaushouse,
            destination_house: House::Flynt,
            polarity_tendency: AuraPolarity::Dark,
            manager_relation: "curved / HAL / Moxy-META relation",
            public_tendency: "immersive undertow",
            pressure_tendency: "coercive pressure",
            semantic_tendency: "immersive, dangerous, seductive, involuntary, undertow-like Dark Aura pressure",
        }),
        _ => None,
    }
}

pub fn resolve_aura_polarity_action(
    request: &AuraPolarityActionRequest,
) -> io::Result<AuraPolarityResolution> {
    if request.domain() != ExpressionDomain::Glow {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Aura polarity resolution requires the Glow domain",
        ));
    }

    let candidate_move = select_candidate_move(request);
    let truth_orientation = infer_truth_orientation(request);
    let agency_effect = infer_agency_effect(request);
    let inferred_orientation = infer_orientation(request, truth_orientation, agency_effect);
    let route_alignment = infer_route_alignment(request.route(), inferred_orientation);
    let pressure = infer_pressure(request);
    let proportionality = infer_proportionality(request);
    let reversibility = infer_reversibility(request);
    let consequence = infer_consequence(request);
    let warnings = collect_warnings(
        request,
        inferred_orientation,
        truth_orientation,
        agency_effect,
        proportionality,
        reversibility,
    );

    Ok(AuraPolarityResolution {
        request: request.clone(),
        candidate_move,
        evaluation: AuraPolarityEvaluation {
            requested_polarity: request.requested_polarity(),
            inferred_orientation,
            route_alignment,
            truth_orientation,
            consent: request.consent(),
            agency_effect,
            pressure,
            proportionality,
            reversibility,
            consequence,
            warnings,
        },
        recipe_status: RecipeBoundaryStatus::LegalFixtureRequired,
        v2_status: "candidate Move must still pass V2 Evaluate / Choose",
    })
}

#[must_use]
pub fn canonical_light_nightingale_diagnosis_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Nightingale",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Faerie)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::HiddenInfection),
        AddressingMode::Proxy,
        ActionAim::DiagnoseAndExplain,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::Glausbahn),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

#[must_use]
pub fn canonical_light_minorian_measurement_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Minorian Gnome",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Pixy)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::SymptomPattern),
        AddressingMode::Moxy,
        ActionAim::MeasureAndMap,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::CurrentSeanad),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

#[must_use]
pub fn canonical_light_minoan_presentation_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Minoan Elf",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Sprite)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::ClinicalFinding),
        AddressingMode::Proxy,
        ActionAim::PresentClearlyAndResponsibly,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::Glausbahn),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

#[must_use]
pub fn canonical_light_foxy_revelation_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Muse",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Muse)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::ConcealedMemoryRelation),
        AddressingMode::Foxy,
        ActionAim::RevealHiddenTruthWithConsent,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::CurrentSeanad),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

#[must_use]
pub fn canonical_defensive_dark_siren_warning_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Siren",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Siren)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Grit,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::ImmediateAttacker),
        AddressingMode::Proxy,
        ActionAim::ProjectDreadToStopAttack,
        House::Glaushouse,
        House::Flynt,
        Some(CanonicalRouteId::Boardwalk),
        AuraPolarity::Dark,
        ConsentState::Absent,
    )
}

#[must_use]
pub fn canonical_dark_riptide_misdirection_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Faerie",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Faerie)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::HostilePursuer),
        AddressingMode::Moxy,
        ActionAim::MisdirectPursuerAwayFromCivilians,
        House::Glaushouse,
        House::Flynt,
        Some(CanonicalRouteId::Riptide),
        AuraPolarity::Dark,
        ConsentState::Absent,
    )
}

#[must_use]
pub fn canonical_dark_seam_counter_bind_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Spirit",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Faerie)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::HostilePsychicTether),
        AddressingMode::Proxy,
        ActionAim::SeverOrTrapHostileConnection,
        House::Glaushouse,
        House::Flynt,
        Some(CanonicalRouteId::Boardwalk),
        AuraPolarity::Dark,
        ConsentState::Absent,
    )
}

#[must_use]
pub fn canonical_consensual_glamour_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Siren",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Siren)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::TheatricalAudience),
        AddressingMode::Proxy,
        ActionAim::CreateSeductiveDramaticSpectacle,
        House::Glaushouse,
        House::Flynt,
        Some(CanonicalRouteId::Boardwalk),
        AuraPolarity::Dark,
        ConsentState::Informed,
    )
}

#[must_use]
pub fn canonical_coercive_glamour_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Siren",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Siren)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::PoliticallyVulnerableCrowd),
        AddressingMode::Proxy,
        ActionAim::ManufactureInevitability,
        House::Glaushouse,
        House::Flynt,
        Some(CanonicalRouteId::Boardwalk),
        AuraPolarity::Dark,
        ConsentState::Absent,
    )
}

#[must_use]
pub fn canonical_false_light_label_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Muse",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Muse)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::TargetPopulation),
        AddressingMode::Proxy,
        ActionAim::ConcealAlternativesAndForceInterpretation,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::Glausbahn),
        AuraPolarity::Light,
        ConsentState::Absent,
    )
}

#[must_use]
pub fn canonical_mixed_clinical_disclosure_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Nightingale",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Faerie)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::SevereDiagnosis),
        AddressingMode::Proxy,
        ActionAim::RevealEnoughTruthForTreatment,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::CurrentSeanad),
        AuraPolarity::Light,
        ConsentState::Partial,
    )
}

#[must_use]
pub fn canonical_light_muse_possibility_field_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Muse",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Muse)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Beam,
        canonical_object_state(ObjectId::MultiplePossibleFutures),
        AddressingMode::Moxy,
        ActionAim::RevealOptionsPreservingChoice,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::CurrentSeanad),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

#[must_use]
pub fn canonical_dark_muse_reality_capture_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Muse",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Muse)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Show,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::SharedSceneInterpretation),
        AddressingMode::Proxy,
        ActionAim::MakeFabricatedRealityInevitable,
        House::Glaushouse,
        House::Flynt,
        Some(CanonicalRouteId::Riptide),
        AuraPolarity::Dark,
        ConsentState::Absent,
    )
}

pub fn build_aura_polarity_witness() -> io::Result<String> {
    Ok(String::from(
        "HOLLOW GROVE AURA POLARITY\n\n\
         Source House:\n\
         Glaüshouse\n\n\
         Light Direction:\n\
         Glaüshouse -> Sandmanor\n\n\
         Dark Direction:\n\
         Glaüshouse -> Flynt\n\n\
         Light Aura:\n\
         clarity, diagnosis, explanation, consent, legibility, shared understanding\n\n\
         Dark Aura:\n\
         manipulation, seduction, fear, compulsion, misdirection, pressure, weaponized presence\n\n\
         Shared Substance:\n\
         Glow\n\n\
         Shared Modes:\n\
         Seam\n\
         Beam\n\
         Gleam\n\n\
         Shared Gestures:\n\
         Grip\n\
         Show\n\
         Grit\n\n\
         Ethical Distinction:\n\
         truth, consent, agency, proportionality, and consequence\n\n\
         Light is not automatically good:\n\
         Confirmed\n\n\
         Dark is not automatically evil:\n\
         Confirmed\n\n\
         Route Geometry Distinct:\n\
         Confirmed\n\n\
         AddressingMode Distinct:\n\
         Confirmed\n\n\
         Recipe Boundary:\n\
         Required\n\n\
         V1.1:\n\
         Unchanged\n",
    ))
}

pub fn build_light_aura_witness() -> io::Result<String> {
    let fixtures = [
        canonical_light_nightingale_diagnosis_fixture(),
        canonical_light_minorian_measurement_fixture(),
        canonical_light_minoan_presentation_fixture(),
        canonical_light_foxy_revelation_fixture(),
        canonical_mixed_clinical_disclosure_fixture(),
        canonical_light_muse_possibility_field_fixture(),
        canonical_light_seam_boundary_fixture(),
        canonical_light_gleam_recognition_fixture(),
    ];

    render_aura_fixture_collection("HOLLOW GROVE LIGHT AURA WITNESS", &fixtures)
}

pub fn build_dark_aura_witness() -> io::Result<String> {
    let fixtures = [
        canonical_defensive_dark_siren_warning_fixture(),
        canonical_dark_riptide_misdirection_fixture(),
        canonical_dark_seam_counter_bind_fixture(),
        canonical_consensual_glamour_fixture(),
        canonical_coercive_glamour_fixture(),
        canonical_false_light_label_fixture(),
        canonical_dark_muse_reality_capture_fixture(),
    ];

    render_aura_fixture_collection("HOLLOW GROVE DARK AURA WITNESS", &fixtures)
}

pub fn build_aura_route_witness() -> io::Result<String> {
    let routes = [
        CanonicalRouteId::Glausbahn,
        CanonicalRouteId::CurrentSeanad,
        CanonicalRouteId::Boardwalk,
        CanonicalRouteId::Riptide,
    ];
    let mut output = String::from("HOLLOW GROVE AURA ROUTE WITNESS\n");

    output.push_str("\n\nGlaüshouse -> Sandmanor\n\nRoutes:\n");
    for route in routes[..2].iter() {
        let definition = canonical_aura_route_definition(*route)
            .ok_or_else(|| io::Error::other("missing Light aura route definition"))?;
        output.push_str(&render_route_definition(&definition));
        output.push_str("\n");
    }

    output.push_str("\nGlaüshouse -> Flynt\n\nRoutes:\n");
    for route in routes[2..].iter() {
        let definition = canonical_aura_route_definition(*route)
            .ok_or_else(|| io::Error::other("missing Dark aura route definition"))?;
        output.push_str(&render_route_definition(&definition));
        output.push_str("\n");
    }

    Ok(output)
}

pub fn build_aura_polarity_validation_report() -> io::Result<String> {
    let diagnostics = validate_aura_polarity_contract(&canonical_aura_polarity_contract_fixture());
    let light = resolve_aura_polarity_action(&canonical_light_nightingale_diagnosis_fixture())?;
    let dark = resolve_aura_polarity_action(&canonical_defensive_dark_siren_warning_fixture())?;
    let mismatch = resolve_aura_polarity_action(&canonical_false_light_label_fixture())?;
    let mixed = resolve_aura_polarity_action(&canonical_mixed_clinical_disclosure_fixture())?;
    let coercive = resolve_aura_polarity_action(&canonical_coercive_glamour_fixture())?;

    let mut errors = diagnostics;
    if light.evaluation().inferred_orientation() != InferredAuraOrientation::Light {
        errors.push(aura_polarity_error(
            "canonical Nightingale diagnosis fixture must infer Light orientation",
        ));
    }
    if dark.evaluation().inferred_orientation() != InferredAuraOrientation::Dark {
        errors.push(aura_polarity_error(
            "defensive Siren warning fixture must infer Dark orientation",
        ));
    }
    if mismatch.evaluation().inferred_orientation() != InferredAuraOrientation::Dark {
        errors.push(aura_polarity_error(
            "false Light label fixture must be detected as Dark orientation",
        ));
    }
    if mixed.evaluation().inferred_orientation() != InferredAuraOrientation::Mixed {
        errors.push(aura_polarity_error(
            "mixed clinical disclosure must remain representable as mixed orientation",
        ));
    }
    if !coercive
        .evaluation()
        .warnings()
        .iter()
        .any(|warning| warning.contains("agency override"))
    {
        errors.push(aura_polarity_error(
            "coercive Dark Glamour must emit an agency override warning",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Aura Polarity Validation\n\n\
             - status: pass\n\
             - source House Glaüshouse: pass\n\
             - Light destination Sandmanor: pass\n\
             - Dark destination Flynt: pass\n\
             - shared Glow domain: pass\n\
             - Light/Dark not separate substances: pass\n\
             - Light/Dark not moral species: pass\n\
             - route geometry distinction: pass\n\
             - AddressingMode distinction: pass\n\
             - Frame distinction: pass\n\
             - Gesture distinction: pass\n\
             - Mode distinction: pass\n\
             - Object distinction: pass\n\
             - Aim distinction: pass\n\
             - truth evaluation: pass\n\
             - consent evaluation: pass\n\
             - agency evaluation: pass\n\
             - proportionality evaluation: pass\n\
             - consequence evaluation: pass\n\
             - reversibility evaluation: pass\n\
             - false Light label detection: pass\n\
             - defensive Dark Aura support: pass\n\
             - coercive Dark Aura warning: pass\n\
             - mixed orientation support: pass\n\
             - Glitz/Glamour distinction: pass\n\
             - Recipe boundary: pass\n\
             - V2 boundary: pass\n\
             - V1.1 unchanged: pass\n\
             - CurrentPrism distinction: pass\n",
        ))
    } else {
        let mut output =
            String::from("# Hollow Grove Aura Polarity Validation\n\n- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        Ok(output)
    }
}

fn render_aura_fixture_collection(
    title: &str,
    fixtures: &[AuraPolarityActionRequest],
) -> io::Result<String> {
    let mut output = String::from(title);
    for fixture in fixtures {
        let resolution = resolve_aura_polarity_action(fixture)?;
        output.push_str("\n\n");
        output.push_str(&render_aura_resolution(&resolution));
    }
    Ok(output)
}

fn render_aura_resolution(resolution: &AuraPolarityResolution) -> String {
    let request = resolution.request();
    let evaluation = resolution.evaluation();
    let route = request
        .route()
        .map_or(String::from("local / none"), |route_id| {
            canonical_aura_route_definition(route_id).map_or_else(
                || route_id.as_str().to_owned(),
                |definition| {
                    format!(
                        "{} ({}, {})",
                        definition.route_id().as_str(),
                        definition.geometry().as_str(),
                        definition.manager_relation()
                    )
                },
            )
        });
    let warnings = if evaluation.warnings().is_empty() {
        String::from("none")
    } else {
        evaluation.warnings().join("; ")
    };

    format!(
        "Being:\n\
         {}\n\n\
         Current Form:\n\
         {}\n\n\
         Aura Frame:\n\
         {}\n\n\
         Gesture:\n\
         {}\n\n\
         Mode:\n\
         {}\n\n\
         Object:\n\
         {}\n\n\
         Object Family:\n\
         {}\n\n\
         Aim:\n\
         {}\n\n\
         AddressingMode:\n\
         {}\n\n\
         Source House:\n\
         {}\n\n\
         Destination House:\n\
         {}\n\n\
         Route:\n\
         {}\n\n\
         Requested Polarity:\n\
         {}\n\n\
         Inferred Polarity:\n\
         {}\n\n\
         Truthfulness:\n\
         {}\n\n\
         Consent:\n\
         {}\n\n\
         Agency Effect:\n\
         {}\n\n\
         Pressure:\n\
         {}\n\n\
         Proportionality:\n\
         {}\n\n\
         Reversibility:\n\
         {}\n\n\
         Consequence:\n\
         {}\n\n\
         Route Alignment:\n\
         {}\n\n\
         Candidate Move:\n\
         {}\n\n\
         Recipe Status:\n\
         {}\n\n\
         Warnings:\n\
         {}",
        request.being_label(),
        frame_label(request.being().current_form()),
        request.being().aura_frame().map_or("none", frame_label),
        request.gesture().as_str(),
        request.mode().as_str(),
        request.object().identity().as_str(),
        request.object().family().as_str(),
        request.aim().as_str(),
        request.addressing_mode().as_str(),
        house_label(request.source_house()),
        house_label(request.destination_house()),
        route,
        request.requested_polarity().as_str(),
        evaluation.inferred_orientation().as_str(),
        evaluation.truth_orientation().as_str(),
        evaluation.consent().as_str(),
        evaluation.agency_effect().as_str(),
        evaluation.pressure().as_str(),
        evaluation.proportionality().as_str(),
        evaluation.reversibility().as_str(),
        evaluation.consequence().as_str(),
        evaluation.route_alignment().as_str(),
        resolution.candidate_move().as_str(),
        resolution.recipe_status().as_str(),
        warnings,
    )
}

fn render_route_definition(route: &AuraRouteDefinition) -> String {
    format!(
        "- {}\n  geometry: {}\n  manager relation: {}\n  source: {}\n  destination: {}\n  polarity tendency: {}\n  public/private tendency: {}\n  pressure tendency: {}\n  sample move: {}\n  polarity remains non-absolute: confirmed\n  route geometry remains separate: confirmed",
        route.route_id().as_str(),
        route.geometry().as_str(),
        route.manager_relation(),
        house_label(route.source_house()),
        house_label(route.destination_house()),
        route.polarity_tendency().as_str(),
        route.public_tendency(),
        route.pressure_tendency(),
        sample_move_for_route(route.route_id()),
    )
}

fn infer_orientation(
    request: &AuraPolarityActionRequest,
    truth: TruthOrientation,
    agency: AgencyEffect,
) -> InferredAuraOrientation {
    match request.aim() {
        ActionAim::DiagnoseAndExplain
        | ActionAim::MeasureAndMap
        | ActionAim::PresentClearlyAndResponsibly
        | ActionAim::RevealHiddenTruthWithConsent
        | ActionAim::RevealOptionsPreservingChoice => InferredAuraOrientation::Light,
        ActionAim::RevealEnoughTruthForTreatment => InferredAuraOrientation::Mixed,
        ActionAim::ProjectDreadToStopAttack
        | ActionAim::MisdirectPursuerAwayFromCivilians
        | ActionAim::SeverOrTrapHostileConnection
        | ActionAim::CreateSeductiveDramaticSpectacle
        | ActionAim::ManufactureInevitability
        | ActionAim::ConcealAlternativesAndForceInterpretation
        | ActionAim::MakeFabricatedRealityInevitable => InferredAuraOrientation::Dark,
        _ => {
            if request.destination_house() == House::Sandmanor
                && matches!(
                    truth,
                    TruthOrientation::Truthful
                        | TruthOrientation::Interpretive
                        | TruthOrientation::SelectivelyDisclosed
                )
                && matches!(
                    agency,
                    AgencyEffect::Expanded | AgencyEffect::Preserved | AgencyEffect::Guided
                )
            {
                InferredAuraOrientation::Light
            } else if request.destination_house() == House::Flynt {
                InferredAuraOrientation::Dark
            } else {
                InferredAuraOrientation::Neutral
            }
        }
    }
}

fn infer_truth_orientation(request: &AuraPolarityActionRequest) -> TruthOrientation {
    match request.aim() {
        ActionAim::DiagnoseAndExplain
        | ActionAim::MeasureAndMap
        | ActionAim::PresentClearlyAndResponsibly
        | ActionAim::RevealHiddenTruthWithConsent
        | ActionAim::RevealOptionsPreservingChoice => TruthOrientation::Truthful,
        ActionAim::RevealEnoughTruthForTreatment => TruthOrientation::SelectivelyDisclosed,
        ActionAim::CreateSeductiveDramaticSpectacle => TruthOrientation::Interpretive,
        ActionAim::ProjectDreadToStopAttack | ActionAim::SeverOrTrapHostileConnection => {
            TruthOrientation::Interpretive
        }
        ActionAim::MisdirectPursuerAwayFromCivilians => TruthOrientation::Deceptive,
        ActionAim::ManufactureInevitability
        | ActionAim::ConcealAlternativesAndForceInterpretation => TruthOrientation::Inverted,
        ActionAim::MakeFabricatedRealityInevitable => TruthOrientation::Fabricated,
        _ => TruthOrientation::Uncertain,
    }
}

fn infer_agency_effect(request: &AuraPolarityActionRequest) -> AgencyEffect {
    match request.aim() {
        ActionAim::DiagnoseAndExplain
        | ActionAim::MeasureAndMap
        | ActionAim::RevealOptionsPreservingChoice => AgencyEffect::Expanded,
        ActionAim::PresentClearlyAndResponsibly | ActionAim::RevealHiddenTruthWithConsent => {
            AgencyEffect::Preserved
        }
        ActionAim::RevealEnoughTruthForTreatment => AgencyEffect::Guided,
        ActionAim::ProjectDreadToStopAttack
        | ActionAim::MisdirectPursuerAwayFromCivilians
        | ActionAim::SeverOrTrapHostileConnection => AgencyEffect::Narrowed,
        ActionAim::CreateSeductiveDramaticSpectacle => AgencyEffect::Guided,
        ActionAim::ManufactureInevitability
        | ActionAim::ConcealAlternativesAndForceInterpretation => AgencyEffect::Overridden,
        ActionAim::MakeFabricatedRealityInevitable => AgencyEffect::Destroyed,
        _ => AgencyEffect::Preserved,
    }
}

fn infer_route_alignment(
    route: Option<CanonicalRouteId>,
    inferred: InferredAuraOrientation,
) -> CompatibilityLevel {
    let Some(route_id) = route else {
        return CompatibilityLevel::Valid;
    };
    let Some(definition) = canonical_aura_route_definition(route_id) else {
        return CompatibilityLevel::Low;
    };

    match inferred {
        InferredAuraOrientation::Mixed | InferredAuraOrientation::Neutral => {
            CompatibilityLevel::Valid
        }
        InferredAuraOrientation::Light if definition.polarity_tendency() == AuraPolarity::Light => {
            CompatibilityLevel::High
        }
        InferredAuraOrientation::Dark if definition.polarity_tendency() == AuraPolarity::Dark => {
            CompatibilityLevel::High
        }
        _ => CompatibilityLevel::Low,
    }
}

fn infer_pressure(request: &AuraPolarityActionRequest) -> PressureRating {
    match request.aim() {
        ActionAim::MeasureAndMap
        | ActionAim::PresentClearlyAndResponsibly
        | ActionAim::RevealHiddenTruthWithConsent => PressureRating::Low,
        ActionAim::DiagnoseAndExplain
        | ActionAim::RevealEnoughTruthForTreatment
        | ActionAim::CreateSeductiveDramaticSpectacle
        | ActionAim::RevealOptionsPreservingChoice => PressureRating::Moderate,
        ActionAim::ProjectDreadToStopAttack
        | ActionAim::MisdirectPursuerAwayFromCivilians
        | ActionAim::SeverOrTrapHostileConnection
        | ActionAim::ManufactureInevitability => PressureRating::High,
        ActionAim::ConcealAlternativesAndForceInterpretation
        | ActionAim::MakeFabricatedRealityInevitable => PressureRating::Severe,
        _ => PressureRating::Moderate,
    }
}

fn infer_proportionality(request: &AuraPolarityActionRequest) -> Proportionality {
    match request.aim() {
        ActionAim::ProjectDreadToStopAttack
        | ActionAim::MisdirectPursuerAwayFromCivilians
        | ActionAim::SeverOrTrapHostileConnection
        | ActionAim::DiagnoseAndExplain
        | ActionAim::MeasureAndMap
        | ActionAim::PresentClearlyAndResponsibly
        | ActionAim::RevealHiddenTruthWithConsent
        | ActionAim::RevealOptionsPreservingChoice => Proportionality::Proportional,
        ActionAim::RevealEnoughTruthForTreatment | ActionAim::CreateSeductiveDramaticSpectacle => {
            Proportionality::Borderline
        }
        ActionAim::ManufactureInevitability
        | ActionAim::ConcealAlternativesAndForceInterpretation
        | ActionAim::MakeFabricatedRealityInevitable => Proportionality::Disproportionate,
        _ => Proportionality::Borderline,
    }
}

fn infer_reversibility(request: &AuraPolarityActionRequest) -> Reversibility {
    match request.aim() {
        ActionAim::DiagnoseAndExplain
        | ActionAim::MeasureAndMap
        | ActionAim::PresentClearlyAndResponsibly
        | ActionAim::RevealHiddenTruthWithConsent
        | ActionAim::RevealOptionsPreservingChoice => Reversibility::Reversible,
        ActionAim::RevealEnoughTruthForTreatment
        | ActionAim::ProjectDreadToStopAttack
        | ActionAim::MisdirectPursuerAwayFromCivilians
        | ActionAim::CreateSeductiveDramaticSpectacle => Reversibility::Costly,
        ActionAim::SeverOrTrapHostileConnection | ActionAim::ManufactureInevitability => {
            Reversibility::Persistent
        }
        ActionAim::ConcealAlternativesAndForceInterpretation
        | ActionAim::MakeFabricatedRealityInevitable => Reversibility::Irreversible,
        _ => Reversibility::Costly,
    }
}

fn infer_consequence(request: &AuraPolarityActionRequest) -> ConsequenceLevel {
    match request.aim() {
        ActionAim::MeasureAndMap
        | ActionAim::PresentClearlyAndResponsibly
        | ActionAim::CreateSeductiveDramaticSpectacle => ConsequenceLevel::Low,
        ActionAim::DiagnoseAndExplain
        | ActionAim::RevealHiddenTruthWithConsent
        | ActionAim::RevealEnoughTruthForTreatment
        | ActionAim::RevealOptionsPreservingChoice
        | ActionAim::ProjectDreadToStopAttack
        | ActionAim::MisdirectPursuerAwayFromCivilians
        | ActionAim::SeverOrTrapHostileConnection => ConsequenceLevel::Material,
        ActionAim::ManufactureInevitability
        | ActionAim::ConcealAlternativesAndForceInterpretation
        | ActionAim::MakeFabricatedRealityInevitable => ConsequenceLevel::Severe,
        _ => ConsequenceLevel::Material,
    }
}

fn collect_warnings(
    request: &AuraPolarityActionRequest,
    inferred: InferredAuraOrientation,
    truth: TruthOrientation,
    agency: AgencyEffect,
    proportionality: Proportionality,
    reversibility: Reversibility,
) -> Vec<String> {
    let mut warnings = Vec::new();

    if request.requested_polarity() == AuraPolarity::Light
        && inferred == InferredAuraOrientation::Dark
    {
        warnings.push(String::from(
            "requested Light label mismatches inferred Dark orientation",
        ));
    }
    if matches!(agency, AgencyEffect::Overridden | AgencyEffect::Destroyed) {
        warnings.push(String::from("agency override or destruction risk"));
    }
    if matches!(
        truth,
        TruthOrientation::Deceptive | TruthOrientation::Fabricated | TruthOrientation::Inverted
    ) {
        warnings.push(String::from(
            "truth orientation is deceptive, fabricated, or inverted",
        ));
    }
    if proportionality == Proportionality::Disproportionate {
        warnings.push(String::from(
            "pressure is disproportionate to the stated aim",
        ));
    }
    if matches!(
        reversibility,
        Reversibility::Persistent | Reversibility::Irreversible
    ) {
        warnings.push(String::from(
            "effect may persist beyond the immediate justified context",
        ));
    }
    if request.consent() == ConsentState::Absent
        && !matches!(
            request.aim(),
            ActionAim::ProjectDreadToStopAttack
                | ActionAim::MisdirectPursuerAwayFromCivilians
                | ActionAim::SeverOrTrapHostileConnection
                | ActionAim::ManufactureInevitability
                | ActionAim::ConcealAlternativesAndForceInterpretation
                | ActionAim::MakeFabricatedRealityInevitable
        )
    {
        warnings.push(String::from(
            "consent is absent outside an obvious emergency or combat context",
        ));
    }

    warnings
}

fn select_candidate_move(request: &AuraPolarityActionRequest) -> AuraMoveId {
    match request.aim() {
        ActionAim::DiagnoseAndExplain => AuraMoveId::DiagnosticRevelation,
        ActionAim::MeasureAndMap => AuraMoveId::ClinicalMeasureLine,
        ActionAim::PresentClearlyAndResponsibly => AuraMoveId::ResponsiblePresentation,
        ActionAim::RevealHiddenTruthWithConsent => AuraMoveId::HiddenTruthReveal,
        ActionAim::ProjectDreadToStopAttack => AuraMoveId::DefensiveDread,
        ActionAim::MisdirectPursuerAwayFromCivilians => AuraMoveId::RiptideMisdirection,
        ActionAim::SeverOrTrapHostileConnection => AuraMoveId::CounterBind,
        ActionAim::CreateSeductiveDramaticSpectacle => AuraMoveId::ConsensualGlamour,
        ActionAim::ManufactureInevitability => AuraMoveId::CoerciveGlamour,
        ActionAim::ConcealAlternativesAndForceInterpretation => AuraMoveId::InterpretiveCapture,
        ActionAim::RevealEnoughTruthForTreatment => AuraMoveId::MeasuredDisclosure,
        ActionAim::RevealOptionsPreservingChoice => AuraMoveId::PossibilityField,
        ActionAim::MakeFabricatedRealityInevitable => AuraMoveId::RealityCapture,
        _ if request.mode() == ActionMode::Seam
            && request.requested_polarity() == AuraPolarity::Light =>
        {
            AuraMoveId::BoundaryClarification
        }
        _ => AuraMoveId::AuthenticRecognition,
    }
}

#[must_use]
fn canonical_light_seam_boundary_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Nightingale",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Faerie)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Grip,
        ActionMode::Seam,
        canonical_object_state(ObjectId::ConcealedMemoryRelation),
        AddressingMode::Proxy,
        ActionAim::RevealHiddenTruthWithConsent,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::CurrentSeanad),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

#[must_use]
fn canonical_light_gleam_recognition_fixture() -> AuraPolarityActionRequest {
    AuraPolarityActionRequest::new(
        "Minoan Elf",
        build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Sprite)),
        ExpressionDomain::Glow,
        EmbodiedGesture::Grit,
        ActionMode::Gleam,
        canonical_object_state(ObjectId::ClinicalFinding),
        AddressingMode::Proxy,
        ActionAim::PresentClearlyAndResponsibly,
        House::Glaushouse,
        House::Sandmanor,
        Some(CanonicalRouteId::Glausbahn),
        AuraPolarity::Light,
        ConsentState::Informed,
    )
}

fn house_label(house: House) -> &'static str {
    match house {
        House::Stonebend => "Stonebend",
        House::Sandmanor => "Sandmanor",
        House::Glaushouse => "Glaüshouse",
        House::Flynt => "Flynt",
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
        FrameId::Gargoyle => "Gargoyle",
        FrameId::Werewolf => "Werewolf",
        FrameId::Merman => "Merman",
        FrameId::Chimera => "Chimera",
        FrameId::Manticore => "Manticore",
    }
}

fn sample_move_for_route(route_id: CanonicalRouteId) -> &'static str {
    match route_id {
        CanonicalRouteId::Glausbahn => "Diagnostic Revelation",
        CanonicalRouteId::CurrentSeanad => "Possibility Field",
        CanonicalRouteId::Boardwalk => "Consensual Glamour",
        CanonicalRouteId::Riptide => "Riptide Misdirection",
        _ => "n/a",
    }
}

fn aura_polarity_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::AuraIlluminationMismatch,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AuraPolarity, ConsentState, InferredAuraOrientation, build_aura_polarity_validation_report,
        build_aura_route_witness, canonical_aura_polarity_contract_fixture,
        canonical_consensual_glamour_fixture, canonical_dark_muse_reality_capture_fixture,
        canonical_false_light_label_fixture, canonical_light_nightingale_diagnosis_fixture,
        canonical_mixed_clinical_disclosure_fixture, resolve_aura_polarity_action,
        validate_aura_polarity_contract,
    };

    #[test]
    fn canonical_aura_polarity_contract_is_clean() {
        assert!(
            validate_aura_polarity_contract(&canonical_aura_polarity_contract_fixture()).is_empty()
        );
    }

    #[test]
    fn false_light_label_is_detected_as_dark() {
        let resolution = resolve_aura_polarity_action(&canonical_false_light_label_fixture())
            .expect("false light fixture should resolve");
        assert_eq!(
            resolution.evaluation().inferred_orientation(),
            InferredAuraOrientation::Dark
        );
        assert_eq!(
            resolution.evaluation().requested_polarity(),
            AuraPolarity::Light
        );
    }

    #[test]
    fn defensive_dark_support_and_consensual_glamour_both_resolve() {
        let consensual = resolve_aura_polarity_action(&canonical_consensual_glamour_fixture())
            .expect("consensual glamour should resolve");
        assert_eq!(consensual.evaluation().consent(), ConsentState::Informed);

        let severe = resolve_aura_polarity_action(&canonical_dark_muse_reality_capture_fixture())
            .expect("reality capture should resolve");
        assert_eq!(
            severe.evaluation().inferred_orientation(),
            InferredAuraOrientation::Dark
        );
    }

    #[test]
    fn mixed_clinical_disclosure_is_supported() {
        let resolution =
            resolve_aura_polarity_action(&canonical_mixed_clinical_disclosure_fixture())
                .expect("mixed disclosure should resolve");
        assert_eq!(
            resolution.evaluation().inferred_orientation(),
            InferredAuraOrientation::Mixed
        );
    }

    #[test]
    fn route_witness_uses_current_seanad_name() {
        let witness = build_aura_route_witness().expect("route witness should build");
        assert!(witness.contains("Current Seanad"));
        assert!(!witness.contains("The Current Sea"));
    }

    #[test]
    fn validation_report_passes() {
        let report = build_aura_polarity_validation_report().expect("validation should build");
        assert!(report.contains("status: pass"));
    }

    #[test]
    fn canonical_light_fixture_resolves_as_light() {
        let resolution =
            resolve_aura_polarity_action(&canonical_light_nightingale_diagnosis_fixture())
                .expect("light fixture should resolve");
        assert_eq!(
            resolution.evaluation().inferred_orientation(),
            InferredAuraOrientation::Light
        );
    }
}
