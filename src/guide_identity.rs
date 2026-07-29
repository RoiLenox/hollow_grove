//! Two continuous Guide identities across Hollow Grove's inside/outside seam.
//!
//! Identity, form, and world layer are deliberately separate. Public names are
//! derived only from a constitutionally valid combination.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{ExternalRef, Position3};
use crate::constitutional::GrovePhase;

pub const GUIDE_IDENTITY_SCHEMA_VERSION: &str = "hollow-grove-guide-identity-v1";
pub const FEDERATION_CANONICAL_NAME: &str = "The Federation";
pub const CENTRAL_JUNCTION_ORIGIN: Position3 = Position3::ORIGIN;
pub const GUIDE_MODEL_PROMPT_LAW: &str = "There are two continuous Guide identities.\n\
\n\
Inside Hollow Grove:\n\
Roi and Roselina.\n\
\n\
Outside Hollow Grove:\n\
Roï and Rosaliña, their Seer forms.\n\
\n\
Roi and Roï are the same being.\n\
\n\
Roselina and Rosaliña are the same being.\n\
\n\
Use only the names supplied in the active Guide context.\n\
\n\
Do not invent a generic third Guide.\n\
\n\
Do not refer to the inside forms by the outside names.\n\
\n\
Do not refer to the outside forms by the inside names.\n\
\n\
Do not decide or alter Guide form.\n\
\n\
Guide form is supplied and confirmed by the Rust kernel.";

fn is_stable_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

macro_rules! stable_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, GuidePresenceError> {
                let value = value.into();
                if is_stable_id(&value) {
                    Ok(Self(value))
                } else {
                    Err(GuidePresenceError::InvalidStableId(value))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl TryFrom<String> for $name {
            type Error = GuidePresenceError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

stable_id!(StableGuideIdentityId);
stable_id!(StableTransitionId);
stable_id!(StableGuideSessionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideIdentity {
    Roi,
    Roselina,
}

impl GuideIdentity {
    pub const ALL: [Self; 2] = [Self::Roi, Self::Roselina];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Roi => "guide.roi",
            Self::Roselina => "guide.roselina",
        }
    }

    pub fn stable_identity_id(self) -> StableGuideIdentityId {
        StableGuideIdentityId::new(self.stable_id()).expect("canonical Guide IDs are stable")
    }

    pub fn external_ref(self) -> ExternalRef {
        let key = self
            .stable_id()
            .strip_prefix("guide.")
            .expect("canonical Guide IDs use the guide namespace");
        ExternalRef::new("guide", key).expect("canonical Guide references are valid")
    }

    #[must_use]
    pub const fn jurisdiction(self) -> GuideJurisdiction {
        match self {
            Self::Roi => GuideJurisdiction::When,
            Self::Roselina => GuideJurisdiction::WhereWhat,
        }
    }

    fn public_name_for_form(self, form: GuideForm) -> &'static str {
        match (self, form) {
            (Self::Roi, GuideForm::HollowGroveGuide) => "Roi",
            (Self::Roi, GuideForm::Seer) => "Roï",
            (Self::Roselina, GuideForm::HollowGroveGuide) => "Roselina",
            (Self::Roselina, GuideForm::Seer) => "Rosaliña",
        }
    }

    fn from_external_ref(reference: &ExternalRef) -> Result<Self, GuidePresenceError> {
        match (reference.namespace.as_str(), reference.key.as_str()) {
            ("guide", "roi") => Ok(Self::Roi),
            ("guide", "roselina") => Ok(Self::Roselina),
            _ => Err(GuidePresenceError::UnknownGuideReference(reference.clone())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideJurisdiction {
    When,
    WhereWhat,
}

impl GuideJurisdiction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::When => "when",
            Self::WhereWhat => "where_what",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideForm {
    HollowGroveGuide,
    Seer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorldLayer {
    OutsideHollowGrove,
    CentralJunction,
    HollowGrove,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideIdentityRecord {
    pub identity_id: StableGuideIdentityId,
    pub identity: GuideIdentity,
}

impl GuideIdentityRecord {
    #[must_use]
    pub fn canonical(identity: GuideIdentity) -> Self {
        Self {
            identity_id: identity.stable_identity_id(),
            identity,
        }
    }

    pub fn validate(&self) -> Result<(), GuidePresenceError> {
        if self.identity_id.as_str() == self.identity.stable_id() {
            Ok(())
        } else {
            Err(GuidePresenceError::IdentityIdMismatch {
                identity: self.identity,
                actual: self.identity_id.as_str().into(),
            })
        }
    }
}

#[must_use]
pub fn canonical_guide_identity_records() -> [GuideIdentityRecord; 2] {
    [
        GuideIdentityRecord::canonical(GuideIdentity::Roi),
        GuideIdentityRecord::canonical(GuideIdentity::Roselina),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuidePresence {
    pub identity: GuideIdentity,
    pub form: GuideForm,
    pub layer: WorldLayer,
}

impl GuidePresence {
    pub fn new(
        identity: GuideIdentity,
        form: GuideForm,
        layer: WorldLayer,
    ) -> Result<Self, GuidePresenceError> {
        let presence = Self {
            identity,
            form,
            layer,
        };
        presence.validate()?;
        Ok(presence)
    }

    #[must_use]
    pub const fn inside(identity: GuideIdentity) -> Self {
        Self {
            identity,
            form: GuideForm::HollowGroveGuide,
            layer: WorldLayer::HollowGrove,
        }
    }

    #[must_use]
    pub const fn outside(identity: GuideIdentity) -> Self {
        Self {
            identity,
            form: GuideForm::Seer,
            layer: WorldLayer::OutsideHollowGrove,
        }
    }

    pub fn validate(&self) -> Result<(), GuidePresenceError> {
        match (self.form, self.layer) {
            (GuideForm::HollowGroveGuide, WorldLayer::HollowGrove)
            | (GuideForm::Seer, WorldLayer::OutsideHollowGrove) => Ok(()),
            (_, WorldLayer::CentralJunction) => {
                Err(GuidePresenceError::JunctionRequiresExplicitTransition)
            }
            _ => Err(GuidePresenceError::InvalidIdentityFormLayer {
                identity: self.identity,
                form: self.form,
                layer: self.layer,
            }),
        }
    }

    pub fn canonical_name(&self) -> Result<&'static str, GuidePresenceError> {
        self.validate()?;
        Ok(self.identity.public_name_for_form(self.form))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JunctionTransitionState {
    Approaching,
    AtOrigin,
    FormChanging,
    Completed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JunctionGuidePresence {
    pub presence: GuidePresence,
    pub transition_ref: ExternalRef,
    pub state: JunctionTransitionState,
}

impl JunctionGuidePresence {
    pub fn new(
        identity: GuideIdentity,
        form: GuideForm,
        transition_ref: ExternalRef,
        state: JunctionTransitionState,
    ) -> Result<Self, GuidePresenceError> {
        let presence = Self {
            presence: GuidePresence {
                identity,
                form,
                layer: WorldLayer::CentralJunction,
            },
            transition_ref,
            state,
        };
        presence.validate()?;
        Ok(presence)
    }

    pub fn validate(&self) -> Result<(), GuidePresenceError> {
        if self.presence.layer != WorldLayer::CentralJunction {
            return Err(GuidePresenceError::InvalidJunctionLayer);
        }
        if !matches!(
            self.state,
            JunctionTransitionState::Approaching
                | JunctionTransitionState::AtOrigin
                | JunctionTransitionState::FormChanging
        ) {
            return Err(GuidePresenceError::JunctionStateIsNotTransitional(
                self.state,
            ));
        }
        Ok(())
    }

    pub fn transitional_name(&self) -> Result<&'static str, GuidePresenceError> {
        self.validate()?;
        Ok(self
            .presence
            .identity
            .public_name_for_form(self.presence.form))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideFormState {
    pub identity_ref: ExternalRef,
    pub form: GuideForm,
    pub layer: WorldLayer,
    pub transition_witness_ref: Option<ExternalRef>,
}

impl GuideFormState {
    pub fn canonical(
        identity: GuideIdentity,
        form: GuideForm,
        layer: WorldLayer,
    ) -> Result<Self, GuidePresenceError> {
        GuidePresence::new(identity, form, layer)?;
        Ok(Self {
            identity_ref: identity.external_ref(),
            form,
            layer,
            transition_witness_ref: None,
        })
    }

    pub fn identity(&self) -> Result<GuideIdentity, GuidePresenceError> {
        GuideIdentity::from_external_ref(&self.identity_ref)
    }

    pub fn presence(&self) -> Result<GuidePresence, GuidePresenceError> {
        GuidePresence::new(self.identity()?, self.form, self.layer)
    }

    pub fn canonical_name(&self) -> Result<&'static str, GuidePresenceError> {
        self.presence()?.canonical_name()
    }

    pub fn validate(&self) -> Result<(), GuidePresenceError> {
        self.presence().map(|_| ())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideTransitionDirection {
    EnterHollowGrove,
    LeaveHollowGrove,
}

impl GuideTransitionDirection {
    #[must_use]
    pub const fn source(self) -> (GuideForm, WorldLayer) {
        match self {
            Self::EnterHollowGrove => (GuideForm::Seer, WorldLayer::OutsideHollowGrove),
            Self::LeaveHollowGrove => (GuideForm::HollowGroveGuide, WorldLayer::HollowGrove),
        }
    }

    #[must_use]
    pub const fn destination(self) -> (GuideForm, WorldLayer) {
        match self {
            Self::EnterHollowGrove => (GuideForm::HollowGroveGuide, WorldLayer::HollowGrove),
            Self::LeaveHollowGrove => (GuideForm::Seer, WorldLayer::OutsideHollowGrove),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTransitionProof {
    pub witness_ref: ExternalRef,
    pub confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideFormTransition {
    pub transition_id: StableTransitionId,
    pub identity_ref: ExternalRef,
    pub from_name: String,
    pub from_form: GuideForm,
    pub from_layer: WorldLayer,
    pub direction: GuideTransitionDirection,
    pub to_name: String,
    pub to_form: GuideForm,
    pub to_layer: WorldLayer,
    pub session_ref: ExternalRef,
    pub source_witness_ref: Option<ExternalRef>,
    pub continuity_proof: GuideTransitionProof,
    pub embodiment_proof: GuideTransitionProof,
}

impl GuideFormTransition {
    pub fn identity(&self) -> Result<GuideIdentity, GuidePresenceError> {
        GuideIdentity::from_external_ref(&self.identity_ref)
    }

    pub fn validate(&self) -> Result<(), GuideTransitionError> {
        let identity = self
            .identity()
            .map_err(GuideTransitionError::InvalidPresence)?;
        if (self.from_form, self.from_layer) != self.direction.source()
            || (self.to_form, self.to_layer) != self.direction.destination()
        {
            return Err(GuideTransitionError::InvalidDirectionState {
                identity,
                direction: self.direction,
            });
        }
        let from_name = GuidePresence::new(identity, self.from_form, self.from_layer)
            .and_then(|presence| presence.canonical_name())
            .map_err(GuideTransitionError::InvalidPresence)?;
        let to_name = GuidePresence::new(identity, self.to_form, self.to_layer)
            .and_then(|presence| presence.canonical_name())
            .map_err(GuideTransitionError::InvalidPresence)?;
        if self.from_name != from_name || self.to_name != to_name {
            return Err(GuideTransitionError::PublicNameMismatch(identity));
        }
        if !self.continuity_proof.confirmed {
            return Err(GuideTransitionError::ContinuityUnproven(identity));
        }
        if !self.embodiment_proof.confirmed {
            return Err(GuideTransitionError::EmbodimentUnproven(identity));
        }
        Ok(())
    }

    pub fn from_name(&self) -> Result<&'static str, GuideTransitionError> {
        let identity = self
            .identity()
            .map_err(GuideTransitionError::InvalidPresence)?;
        GuidePresence::new(identity, self.from_form, self.from_layer)
            .and_then(|presence| presence.canonical_name())
            .map_err(GuideTransitionError::InvalidPresence)
    }

    pub fn to_name(&self) -> Result<&'static str, GuideTransitionError> {
        let identity = self
            .identity()
            .map_err(GuideTransitionError::InvalidPresence)?;
        GuidePresence::new(identity, self.to_form, self.to_layer)
            .and_then(|presence| presence.canonical_name())
            .map_err(GuideTransitionError::InvalidPresence)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairedGuideTransition {
    pub transition_id: StableTransitionId,
    pub direction: GuideTransitionDirection,
    pub roi_transition: GuideFormTransition,
    pub roselina_transition: GuideFormTransition,
    pub session_ref: ExternalRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTransitionEvidence {
    pub roi_continuity_ref: ExternalRef,
    pub roi_embodiment_ref: ExternalRef,
    pub roselina_continuity_ref: ExternalRef,
    pub roselina_embodiment_ref: ExternalRef,
    pub source_witness_ref: Option<ExternalRef>,
}

impl PairedGuideTransition {
    pub fn proposed(
        session: &DualGuideSession,
        transition_id: StableTransitionId,
        direction: GuideTransitionDirection,
        evidence: GuideTransitionEvidence,
    ) -> Result<Self, GuideTransitionError> {
        session
            .validate()
            .map_err(GuideTransitionError::InvalidPresence)?;
        let session_ref = session.external_ref();
        let (from_form, from_layer) = direction.source();
        let (to_form, to_layer) = direction.destination();
        let transition =
            |identity: GuideIdentity, continuity_ref: ExternalRef, embodiment_ref: ExternalRef| {
                GuideFormTransition {
                    transition_id: transition_id.clone(),
                    identity_ref: identity.external_ref(),
                    from_name: GuidePresence::new(identity, from_form, from_layer)
                        .and_then(|presence| presence.canonical_name())
                        .expect("transition source is canonical")
                        .into(),
                    from_form,
                    from_layer,
                    direction,
                    to_name: GuidePresence::new(identity, to_form, to_layer)
                        .and_then(|presence| presence.canonical_name())
                        .expect("transition destination is canonical")
                        .into(),
                    to_form,
                    to_layer,
                    session_ref: session_ref.clone(),
                    source_witness_ref: evidence.source_witness_ref.clone(),
                    continuity_proof: GuideTransitionProof {
                        witness_ref: continuity_ref,
                        confirmed: true,
                    },
                    embodiment_proof: GuideTransitionProof {
                        witness_ref: embodiment_ref,
                        confirmed: true,
                    },
                }
            };
        Ok(Self {
            transition_id: transition_id.clone(),
            direction,
            roi_transition: transition(
                GuideIdentity::Roi,
                evidence.roi_continuity_ref,
                evidence.roi_embodiment_ref,
            ),
            roselina_transition: transition(
                GuideIdentity::Roselina,
                evidence.roselina_continuity_ref,
                evidence.roselina_embodiment_ref,
            ),
            session_ref,
        })
    }

    pub fn external_ref(&self) -> ExternalRef {
        ExternalRef::new("guide-transition", self.transition_id.as_str())
            .expect("stable transition IDs make valid external references")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DualGuideSession {
    pub session_id: StableGuideSessionId,
    pub roi_state: GuideFormState,
    pub roselina_state: GuideFormState,
    pub current_layer: WorldLayer,
    pub focused_object_ref: Option<ExternalRef>,
    pub current_witness_ref: Option<ExternalRef>,
    pub pending_transition_ref: Option<ExternalRef>,
}

impl DualGuideSession {
    pub fn outside(session_id: StableGuideSessionId) -> Self {
        Self {
            session_id,
            roi_state: GuideFormState::canonical(
                GuideIdentity::Roi,
                GuideForm::Seer,
                WorldLayer::OutsideHollowGrove,
            )
            .expect("outside Roi state is canonical"),
            roselina_state: GuideFormState::canonical(
                GuideIdentity::Roselina,
                GuideForm::Seer,
                WorldLayer::OutsideHollowGrove,
            )
            .expect("outside Roselina state is canonical"),
            current_layer: WorldLayer::OutsideHollowGrove,
            focused_object_ref: None,
            current_witness_ref: None,
            pending_transition_ref: None,
        }
    }

    pub fn inside(session_id: StableGuideSessionId) -> Self {
        Self {
            session_id,
            roi_state: GuideFormState::canonical(
                GuideIdentity::Roi,
                GuideForm::HollowGroveGuide,
                WorldLayer::HollowGrove,
            )
            .expect("inside Roi state is canonical"),
            roselina_state: GuideFormState::canonical(
                GuideIdentity::Roselina,
                GuideForm::HollowGroveGuide,
                WorldLayer::HollowGrove,
            )
            .expect("inside Roselina state is canonical"),
            current_layer: WorldLayer::HollowGrove,
            focused_object_ref: None,
            current_witness_ref: None,
            pending_transition_ref: None,
        }
    }

    pub fn external_ref(&self) -> ExternalRef {
        ExternalRef::new("guide-session", self.session_id.as_str())
            .expect("stable session IDs make valid external references")
    }

    pub fn validate(&self) -> Result<(), GuidePresenceError> {
        self.validate_restorable()?;
        if self.pending_transition_ref.is_some() {
            return Err(GuidePresenceError::ExternallyPendingTransition);
        }
        Ok(())
    }

    pub fn validate_restorable(&self) -> Result<(), GuidePresenceError> {
        self.roi_state.validate()?;
        self.roselina_state.validate()?;
        if self.roi_state.identity()? != GuideIdentity::Roi
            || self.roselina_state.identity()? != GuideIdentity::Roselina
        {
            return Err(GuidePresenceError::SessionIdentityOrder);
        }
        if self.current_layer == WorldLayer::CentralJunction
            || self.roi_state.layer != self.current_layer
            || self.roselina_state.layer != self.current_layer
            || self.roi_state.form != self.roselina_state.form
        {
            return Err(GuidePresenceError::NonCanonicalPairedSession);
        }
        Ok(())
    }

    pub fn active_context(&self) -> Result<ActiveGuideContext, GuidePresenceError> {
        ActiveGuideContext::from_session(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WayBackGuideContext {
    pub roi_state: GuideFormState,
    pub roselina_state: GuideFormState,
    pub session_ref: ExternalRef,
    pub current_layer: WorldLayer,
    pub pending_transition: Option<ExternalRef>,
}

impl WayBackGuideContext {
    pub fn restore(session: &DualGuideSession) -> Result<Self, GuidePresenceError> {
        session.validate_restorable()?;
        Ok(Self {
            roi_state: session.roi_state.clone(),
            roselina_state: session.roselina_state.clone(),
            session_ref: session.external_ref(),
            current_layer: session.current_layer,
            pending_transition: session.pending_transition_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DualGuideSessionArchive {
    pub schema_version: String,
    pub session: DualGuideSession,
}

pub fn encode_dual_guide_session(
    session: &DualGuideSession,
) -> Result<Vec<u8>, GuidePresenceError> {
    session.validate_restorable()?;
    serde_json::to_vec_pretty(&DualGuideSessionArchive {
        schema_version: GUIDE_IDENTITY_SCHEMA_VERSION.into(),
        session: session.clone(),
    })
    .map_err(|error| GuidePresenceError::Serialization(error.to_string()))
}

pub fn decode_dual_guide_session(bytes: &[u8]) -> Result<DualGuideSession, GuidePresenceError> {
    let archive = serde_json::from_slice::<DualGuideSessionArchive>(bytes)
        .map_err(|error| GuidePresenceError::Serialization(error.to_string()))?;
    if archive.schema_version != GUIDE_IDENTITY_SCHEMA_VERSION {
        return Err(GuidePresenceError::UnsupportedSchemaVersion(
            archive.schema_version,
        ));
    }
    archive.session.validate_restorable()?;
    Ok(archive.session)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideFestivalDecision {
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideSessionSnapshot {
    pub session_ref: ExternalRef,
    pub layer: WorldLayer,
    pub roi_name: String,
    pub roselina_name: String,
    pub roi_identity_ref: ExternalRef,
    pub roselina_identity_ref: ExternalRef,
}

impl GuideSessionSnapshot {
    pub fn from_session(session: &DualGuideSession) -> Result<Self, GuidePresenceError> {
        session.validate()?;
        Ok(Self {
            session_ref: session.external_ref(),
            layer: session.current_layer,
            roi_name: session.roi_state.canonical_name()?.into(),
            roselina_name: session.roselina_state.canonical_name()?.into(),
            roi_identity_ref: session.roi_state.identity_ref.clone(),
            roselina_identity_ref: session.roselina_state.identity_ref.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTransitionWitness {
    pub transition_id: StableTransitionId,
    pub witness_ref: ExternalRef,
    pub direction: GuideTransitionDirection,
    pub session_ref: ExternalRef,
    pub roi_transition: GuideFormTransition,
    pub roselina_transition: GuideFormTransition,
    pub junction_states: Vec<JunctionTransitionState>,
    pub continuity_assessment: String,
    pub embodiment_assessment: String,
    pub decision: GuideFestivalDecision,
    pub resulting_session: GuideSessionSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RejectedGuideTransition {
    pub transition_id: StableTransitionId,
    pub direction: GuideTransitionDirection,
    pub session_ref: ExternalRef,
    pub decision: GuideFestivalDecision,
    pub reasons: Vec<GuideTransitionError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status", content = "record")]
pub enum PairedGuideTransitionOutcome {
    Confirmed(Box<GuideTransitionWitness>),
    Rejected(RejectedGuideTransition),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTransitionCycleResult {
    pub direction: GuideTransitionDirection,
    pub way_back_before: WayBackGuideContext,
    pub initiation_transition_ref: ExternalRef,
    pub gathering_valid: bool,
    pub festival: PairedGuideTransitionOutcome,
    pub way_back_after: WayBackGuideContext,
    pub phase_trace: Vec<GrovePhase>,
}

pub fn run_paired_guide_transition_cycle(
    session: &mut DualGuideSession,
    transition: &PairedGuideTransition,
) -> Result<GuideTransitionCycleResult, GuidePresenceError> {
    let way_back_before = WayBackGuideContext::restore(session)?;
    let initiation_transition_ref = transition.external_ref();
    let (gathering_valid, festival) = match stage_paired_guide_transition(session, transition) {
        Ok(()) => (true, perform_paired_guide_transition(session, transition)),
        Err(error) => (
            false,
            PairedGuideTransitionOutcome::Rejected(RejectedGuideTransition {
                transition_id: transition.transition_id.clone(),
                direction: transition.direction,
                session_ref: transition.session_ref.clone(),
                decision: GuideFestivalDecision::Rejected,
                reasons: vec![error],
            }),
        ),
    };
    let way_back_after = WayBackGuideContext::restore(session)?;
    Ok(GuideTransitionCycleResult {
        direction: transition.direction,
        way_back_before,
        initiation_transition_ref,
        gathering_valid,
        festival,
        way_back_after,
        phase_trace: GrovePhase::ALL.to_vec(),
    })
}

pub fn stage_paired_guide_transition(
    session: &mut DualGuideSession,
    transition: &PairedGuideTransition,
) -> Result<(), GuideTransitionError> {
    if session.pending_transition_ref.is_some() {
        return Err(GuideTransitionError::PendingTransitionAlreadyExists);
    }
    if let Some(error) = validate_paired_transition(session, transition)
        .into_iter()
        .next()
    {
        return Err(error);
    }
    session.pending_transition_ref = Some(transition.external_ref());
    Ok(())
}

pub fn perform_paired_guide_transition(
    session: &mut DualGuideSession,
    transition: &PairedGuideTransition,
) -> PairedGuideTransitionOutcome {
    let mut reasons = validate_paired_transition(session, transition);
    if reasons.is_empty() {
        let witness_ref = ExternalRef::new(
            "guide-transition-witness",
            transition.transition_id.as_str(),
        )
        .expect("stable transition IDs make valid witness references");
        let (_, destination_layer) = transition.direction.destination();
        let mut candidate = session.clone();
        candidate.roi_state = GuideFormState {
            identity_ref: GuideIdentity::Roi.external_ref(),
            form: transition.roi_transition.to_form,
            layer: transition.roi_transition.to_layer,
            transition_witness_ref: Some(witness_ref.clone()),
        };
        candidate.roselina_state = GuideFormState {
            identity_ref: GuideIdentity::Roselina.external_ref(),
            form: transition.roselina_transition.to_form,
            layer: transition.roselina_transition.to_layer,
            transition_witness_ref: Some(witness_ref.clone()),
        };
        candidate.current_layer = destination_layer;
        candidate.current_witness_ref = Some(witness_ref.clone());
        candidate.pending_transition_ref = None;
        if let Err(error) = candidate.validate() {
            reasons.push(GuideTransitionError::InvalidPresence(error));
        } else {
            let resulting_session = GuideSessionSnapshot::from_session(&candidate)
                .expect("validated candidate session has a snapshot");
            *session = candidate;
            return PairedGuideTransitionOutcome::Confirmed(Box::new(GuideTransitionWitness {
                transition_id: transition.transition_id.clone(),
                witness_ref,
                direction: transition.direction,
                session_ref: transition.session_ref.clone(),
                roi_transition: transition.roi_transition.clone(),
                roselina_transition: transition.roselina_transition.clone(),
                junction_states: vec![
                    JunctionTransitionState::Approaching,
                    JunctionTransitionState::AtOrigin,
                    JunctionTransitionState::FormChanging,
                    JunctionTransitionState::Completed,
                ],
                continuity_assessment: "confirmed".into(),
                embodiment_assessment: "confirmed".into(),
                decision: GuideFestivalDecision::Confirmed,
                resulting_session,
            }));
        }
    }
    if session.pending_transition_ref.as_ref() == Some(&transition.external_ref()) {
        session.pending_transition_ref = None;
    }
    PairedGuideTransitionOutcome::Rejected(RejectedGuideTransition {
        transition_id: transition.transition_id.clone(),
        direction: transition.direction,
        session_ref: transition.session_ref.clone(),
        decision: GuideFestivalDecision::Rejected,
        reasons,
    })
}

fn validate_paired_transition(
    session: &DualGuideSession,
    transition: &PairedGuideTransition,
) -> Vec<GuideTransitionError> {
    let mut errors = Vec::new();
    if let Err(error) = session.validate_restorable() {
        errors.push(GuideTransitionError::InvalidPresence(error));
        return errors;
    }
    if transition.session_ref != session.external_ref()
        || transition.roi_transition.session_ref != session.external_ref()
        || transition.roselina_transition.session_ref != session.external_ref()
    {
        errors.push(GuideTransitionError::SessionMismatch);
    }
    if session
        .pending_transition_ref
        .as_ref()
        .is_some_and(|pending| pending != &transition.external_ref())
    {
        errors.push(GuideTransitionError::PendingTransitionMismatch);
    }
    if transition.roi_transition.transition_id != transition.transition_id
        || transition.roselina_transition.transition_id != transition.transition_id
        || transition.roi_transition.direction != transition.direction
        || transition.roselina_transition.direction != transition.direction
    {
        errors.push(GuideTransitionError::PairMetadataMismatch);
    }
    for (expected_identity, individual, current_state) in [
        (
            GuideIdentity::Roi,
            &transition.roi_transition,
            &session.roi_state,
        ),
        (
            GuideIdentity::Roselina,
            &transition.roselina_transition,
            &session.roselina_state,
        ),
    ] {
        if let Err(error) = individual.validate() {
            errors.push(error);
            continue;
        }
        if individual.identity().ok() != Some(expected_identity) {
            errors.push(GuideTransitionError::PairIdentityMismatch(
                expected_identity,
            ));
        }
        if individual.from_form != current_state.form
            || individual.from_layer != current_state.layer
            || individual.identity_ref != current_state.identity_ref
        {
            errors.push(GuideTransitionError::SourceStateMismatch(expected_identity));
        }
    }
    errors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideSpeaker {
    Roi,
    Roselina,
    Both,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveGuideDescriptor {
    pub identity: GuideIdentity,
    pub identity_id: StableGuideIdentityId,
    pub form: GuideForm,
    pub display_name: String,
    pub jurisdiction: GuideJurisdiction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveGuideContext {
    pub layer: WorldLayer,
    pub guides: Vec<ActiveGuideDescriptor>,
    pub seam_transition: Option<JunctionTransitionState>,
    pub transition_ref: Option<ExternalRef>,
}

impl ActiveGuideContext {
    pub fn from_session(session: &DualGuideSession) -> Result<Self, GuidePresenceError> {
        session.validate()?;
        let guides = active_descriptors(&session.roi_state, &session.roselina_state)?;
        Ok(Self {
            layer: session.current_layer,
            guides,
            seam_transition: None,
            transition_ref: None,
        })
    }

    pub fn during_transition(
        session: &DualGuideSession,
        transition: &PairedGuideTransition,
        state: JunctionTransitionState,
    ) -> Result<Self, GuideTransitionError> {
        if let Some(error) = validate_paired_transition(session, transition)
            .into_iter()
            .next()
        {
            return Err(error);
        }
        if !matches!(
            state,
            JunctionTransitionState::Approaching
                | JunctionTransitionState::AtOrigin
                | JunctionTransitionState::FormChanging
        ) {
            return Err(GuideTransitionError::InvalidJunctionState(state));
        }
        let guides = active_descriptors(&session.roi_state, &session.roselina_state)
            .map_err(GuideTransitionError::InvalidPresence)?;
        let context = Self {
            layer: WorldLayer::CentralJunction,
            guides,
            seam_transition: Some(state),
            transition_ref: Some(transition.external_ref()),
        };
        context
            .validate()
            .map_err(GuideTransitionError::InvalidPresence)?;
        Ok(context)
    }

    pub fn validate(&self) -> Result<(), GuidePresenceError> {
        if self.guides.len() != 2 {
            return Err(GuidePresenceError::GuideContextCount(self.guides.len()));
        }
        let identities = self
            .guides
            .iter()
            .map(|guide| guide.identity)
            .collect::<BTreeSet<_>>();
        if identities != GuideIdentity::ALL.into_iter().collect() {
            return Err(GuidePresenceError::GuideContextIdentitySet);
        }
        for guide in &self.guides {
            let canonical_name = if self.layer == WorldLayer::CentralJunction {
                let transition_ref = self
                    .transition_ref
                    .clone()
                    .ok_or(GuidePresenceError::GuideContextTransitionState)?;
                let state = self
                    .seam_transition
                    .ok_or(GuidePresenceError::GuideContextTransitionState)?;
                JunctionGuidePresence::new(guide.identity, guide.form, transition_ref, state)?
                    .transitional_name()?
            } else {
                if self.seam_transition.is_some() || self.transition_ref.is_some() {
                    return Err(GuidePresenceError::GuideContextTransitionState);
                }
                GuidePresence::new(guide.identity, guide.form, self.layer)?.canonical_name()?
            };
            if guide.identity_id.as_str() != guide.identity.stable_id()
                || guide.display_name != canonical_name
                || guide.jurisdiction != guide.identity.jurisdiction()
            {
                return Err(GuidePresenceError::GuideContextProjection(guide.identity));
            }
        }
        Ok(())
    }

    pub fn display_name(&self, speaker: GuideSpeaker) -> Result<String, GuidePresenceError> {
        self.validate()?;
        match speaker {
            GuideSpeaker::Roi => self
                .guides
                .iter()
                .find(|guide| guide.identity == GuideIdentity::Roi)
                .map(|guide| guide.display_name.clone())
                .ok_or(GuidePresenceError::GuideContextIdentitySet),
            GuideSpeaker::Roselina => self
                .guides
                .iter()
                .find(|guide| guide.identity == GuideIdentity::Roselina)
                .map(|guide| guide.display_name.clone())
                .ok_or(GuidePresenceError::GuideContextIdentitySet),
            GuideSpeaker::Both => {
                if self.layer == WorldLayer::CentralJunction {
                    return Err(GuidePresenceError::JunctionRequiresExplicitTransition);
                }
                Ok(format!(
                    "{} + {}",
                    self.display_name(GuideSpeaker::Roi)?.to_uppercase(),
                    self.display_name(GuideSpeaker::Roselina)?.to_uppercase()
                ))
            }
        }
    }

    pub fn model_context_json(&self) -> Result<String, GuidePresenceError> {
        self.validate()?;
        serde_json::to_string(self)
            .map_err(|error| GuidePresenceError::Serialization(error.to_string()))
    }
}

fn active_descriptors(
    roi_state: &GuideFormState,
    roselina_state: &GuideFormState,
) -> Result<Vec<ActiveGuideDescriptor>, GuidePresenceError> {
    [roi_state, roselina_state]
        .into_iter()
        .map(|state| {
            let identity = state.identity()?;
            Ok(ActiveGuideDescriptor {
                identity,
                identity_id: identity.stable_identity_id(),
                form: state.form,
                display_name: state.canonical_name()?.into(),
                jurisdiction: identity.jurisdiction(),
            })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualSeerPresentation {
    pub context: ActiveGuideContext,
}

impl DualSeerPresentation {
    pub fn outside(session_id: StableGuideSessionId) -> Self {
        Self {
            context: DualGuideSession::outside(session_id)
                .active_context()
                .expect("canonical outside session has a presentation context"),
        }
    }

    pub fn render_tui(&self) -> Result<String, GuidePresenceError> {
        self.context.validate()?;
        if self.context.layer != WorldLayer::OutsideHollowGrove {
            return Err(GuidePresenceError::OutsidePresentationRequiresSeers);
        }
        let roi_name = self.context.display_name(GuideSpeaker::Roi)?;
        let roselina_name = self.context.display_name(GuideSpeaker::Roselina)?;
        Ok(format!(
            "THE SEERS\n\n\
             {roi_name}\n\
             Where have we been, what remains continuous, and what comes next?\n\n\
             {roselina_name}\n\
             Where are we now, what exists here, and what can be made real?\n\n\
             interface: TUI\n\
             authority: {}\n",
            FEDERATION_CANONICAL_NAME
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyGuideIdentityRecord {
    pub legacy_identity_id: String,
    pub identity_hint: GuideIdentity,
    pub public_name: String,
    pub form: Option<GuideForm>,
    pub layer: Option<WorldLayer>,
    pub historical_witness_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideIdentityMigrationMetadata {
    pub identity: GuideIdentity,
    pub canonical_identity_id: StableGuideIdentityId,
    pub historical_identity_ids: Vec<String>,
    pub historical_public_names: Vec<String>,
    pub historical_witness_texts: Vec<String>,
    pub corrected_source_conflicts: Vec<String>,
    pub corrected_interpretation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideIdentityMigration {
    pub schema_version: String,
    pub identity_records: Vec<GuideIdentityRecord>,
    pub form_states: Vec<GuideFormState>,
    pub metadata: Vec<GuideIdentityMigrationMetadata>,
}

pub fn migrate_legacy_guide_identities(
    legacy: &[LegacyGuideIdentityRecord],
    current_layer: WorldLayer,
) -> Result<GuideIdentityMigration, GuideMigrationError> {
    if !matches!(
        current_layer,
        WorldLayer::OutsideHollowGrove | WorldLayer::HollowGrove
    ) {
        return Err(GuideMigrationError::AmbiguousTransitionLayer);
    }
    let mut grouped = BTreeMap::<GuideIdentity, Vec<&LegacyGuideIdentityRecord>>::new();
    let mut corrected_source_conflicts = BTreeMap::<GuideIdentity, Vec<String>>::new();
    for record in legacy {
        let (identity, alias_form, alias_layer) = match record.public_name.as_str() {
            "Roi" => (
                GuideIdentity::Roi,
                GuideForm::HollowGroveGuide,
                WorldLayer::HollowGrove,
            ),
            "Roï" => (
                GuideIdentity::Roi,
                GuideForm::Seer,
                WorldLayer::OutsideHollowGrove,
            ),
            "Roselina" => (
                GuideIdentity::Roselina,
                GuideForm::HollowGroveGuide,
                WorldLayer::HollowGrove,
            ),
            "Rosaliña" | "Rosalina" => (
                GuideIdentity::Roselina,
                GuideForm::Seer,
                WorldLayer::OutsideHollowGrove,
            ),
            unknown => return Err(GuideMigrationError::UnknownLegacyName(unknown.into())),
        };
        if record.identity_hint != identity {
            return Err(GuideMigrationError::IdentityEvidenceMismatch {
                legacy_identity_id: record.legacy_identity_id.clone(),
                expected: identity,
                supplied: record.identity_hint,
            });
        }
        if record.form.is_some_and(|form| form != alias_form)
            || record.layer.is_some_and(|layer| layer != alias_layer)
        {
            corrected_source_conflicts
                .entry(identity)
                .or_default()
                .push(format!(
                    "{} retained for audit: legacy name/form/layer interpretation was corrected",
                    record.legacy_identity_id
                ));
        }
        grouped.entry(identity).or_default().push(record);
    }
    for identity in GuideIdentity::ALL {
        if !grouped.contains_key(&identity) {
            return Err(GuideMigrationError::MissingIdentity(identity));
        }
    }

    let (form, layer) = match current_layer {
        WorldLayer::OutsideHollowGrove => (GuideForm::Seer, WorldLayer::OutsideHollowGrove),
        WorldLayer::HollowGrove => (GuideForm::HollowGroveGuide, WorldLayer::HollowGrove),
        WorldLayer::CentralJunction => unreachable!("transition layer rejected above"),
    };
    let identity_records = canonical_guide_identity_records().to_vec();
    let form_states = GuideIdentity::ALL
        .into_iter()
        .map(|identity| {
            GuideFormState::canonical(identity, form, layer)
                .expect("migration target uses a canonical form/layer")
        })
        .collect();
    let metadata = GuideIdentity::ALL
        .into_iter()
        .map(|identity| {
            let records = &grouped[&identity];
            let mut historical_identity_ids = records
                .iter()
                .map(|record| record.legacy_identity_id.clone())
                .collect::<Vec<_>>();
            historical_identity_ids.sort();
            historical_identity_ids.dedup();
            let mut historical_public_names = records
                .iter()
                .map(|record| record.public_name.clone())
                .collect::<Vec<_>>();
            historical_public_names.sort();
            historical_public_names.dedup();
            let mut historical_witness_texts = records
                .iter()
                .filter_map(|record| record.historical_witness_text.clone())
                .collect::<Vec<_>>();
            historical_witness_texts.sort();
            historical_witness_texts.dedup();
            GuideIdentityMigrationMetadata {
                identity,
                canonical_identity_id: identity.stable_identity_id(),
                historical_identity_ids,
                historical_public_names,
                historical_witness_texts,
                corrected_source_conflicts: corrected_source_conflicts
                    .remove(&identity)
                    .unwrap_or_default(),
                corrected_interpretation: match identity {
                    GuideIdentity::Roi => "Roi and Roï are one continuous being.".into(),
                    GuideIdentity::Roselina => {
                        "Roselina and Rosaliña are one continuous being.".into()
                    }
                },
            }
        })
        .collect();
    Ok(GuideIdentityMigration {
        schema_version: GUIDE_IDENTITY_SCHEMA_VERSION.into(),
        identity_records,
        form_states,
        metadata,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuideMigrationError {
    UnknownLegacyName(String),
    IdentityEvidenceMismatch {
        legacy_identity_id: String,
        expected: GuideIdentity,
        supplied: GuideIdentity,
    },
    MissingIdentity(GuideIdentity),
    AmbiguousTransitionLayer,
}

impl fmt::Display for GuideMigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Guide identity migration rejected record: {self:?}"
        )
    }
}

impl std::error::Error for GuideMigrationError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuidePresenceError {
    InvalidStableId(String),
    IdentityIdMismatch {
        identity: GuideIdentity,
        actual: String,
    },
    UnknownGuideReference(ExternalRef),
    InvalidIdentityFormLayer {
        identity: GuideIdentity,
        form: GuideForm,
        layer: WorldLayer,
    },
    JunctionRequiresExplicitTransition,
    InvalidJunctionLayer,
    JunctionStateIsNotTransitional(JunctionTransitionState),
    SessionIdentityOrder,
    NonCanonicalPairedSession,
    ExternallyPendingTransition,
    GuideContextCount(usize),
    GuideContextIdentitySet,
    GuideContextProjection(GuideIdentity),
    GuideContextTransitionState,
    OutsidePresentationRequiresSeers,
    Serialization(String),
    UnsupportedSchemaVersion(String),
}

impl fmt::Display for GuidePresenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid Guide identity/form/layer state: {self:?}"
        )
    }
}

impl std::error::Error for GuidePresenceError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuideTransitionError {
    InvalidPresence(GuidePresenceError),
    InvalidDirectionState {
        identity: GuideIdentity,
        direction: GuideTransitionDirection,
    },
    ContinuityUnproven(GuideIdentity),
    EmbodimentUnproven(GuideIdentity),
    SessionMismatch,
    PairMetadataMismatch,
    PairIdentityMismatch(GuideIdentity),
    SourceStateMismatch(GuideIdentity),
    PublicNameMismatch(GuideIdentity),
    PendingTransitionAlreadyExists,
    PendingTransitionMismatch,
    InvalidJunctionState(JunctionTransitionState),
}

impl fmt::Display for GuideTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Guide form transition rejected: {self:?}")
    }
}

impl std::error::Error for GuideTransitionError {}
