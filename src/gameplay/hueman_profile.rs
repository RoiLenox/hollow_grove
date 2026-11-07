//! Persistent cosmetics and the introductory projection over existing gameplay events.
use super::{GameplayEvent, HollowGroveGameRuntime, InteractionId, SurveyFramePhase};
use crate::world::geography::ConstitutionalRouteId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HuemanHair {
    pub style: String,
    pub color: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HuemanAppearance {
    pub skin: String,
    pub face: String,
    pub hair: HuemanHair,
}
impl HuemanAppearance {
    pub fn validate(&self) -> Result<(), String> {
        if ![
            "light_neutral",
            "light_warm",
            "medium_golden",
            "medium_warm",
            "deep_warm",
            "deep_neutral",
        ]
        .contains(&self.skin.as_str())
            || !["calm", "bright", "focused", "freckled"].contains(&self.face.as_str())
            || !["shaved", "crop", "curls", "side_part", "long"].contains(&self.hair.style.as_str())
            || !["black", "brown", "auburn", "blond"].contains(&self.hair.color.as_str())
        {
            return Err("Unknown Skin, Face or Hair preset".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HuemanProfile {
    pub schema_version: u16,
    pub avatar_id: String,
    pub appearance: HuemanAppearance,
}
impl HuemanProfile {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.avatar_id.len() != 39
            || !self.avatar_id.starts_with("avatar.")
            || !self.avatar_id[7..]
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("Invalid Hueman profile version or avatar identity".into());
        }
        self.appearance.validate()
    }
    pub fn continuity_id(&self) -> String {
        format!("being-continuity.{}", self.avatar_id)
    }
    pub fn participant_id(&self) -> String {
        format!("participant.{}", self.avatar_id)
    }
    pub fn institutional_id(&self) -> String {
        format!("being.{}", self.avatar_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InitiationObjective {
    pub id: String,
    pub text: String,
    pub complete: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InitiationView {
    pub mission_id: String,
    pub title: String,
    pub objectives: Vec<InitiationObjective>,
    pub objective: String,
    pub complete: bool,
    pub recovery_case_id: String,
}
impl InitiationView {
    pub fn from_runtime(runtime: &HollowGroveGameRuntime) -> Option<Self> {
        runtime.hueman_profile()?;
        let moved = runtime.events().iter().any(|e| {
            matches!(&e.payload,
            GameplayEvent::HuemanMovementResolved { from, to } if from.x != to.x || from.y != to.y)
        });
        let interacted = runtime.events().iter().any(|e| {
            matches!(
                &e.payload,
                GameplayEvent::HuemanInteractionOpened {
                    target: InteractionId::ConstitutionalRouteWitness(
                        ConstitutionalRouteId::AuraWay
                    ),
                    ..
                }
            )
        });
        let frame = runtime.survey_frame();
        let objectives: Vec<_> = [
            ("arrive", "Arrive in Stonebend on Aura Way.", true),
            ("move", "Move with WASD or arrow keys.", moved),
            (
                "interact",
                "Face the Aura Way witness marker; press Enter.",
                interacted,
            ),
            (
                "need",
                "Press O for Tess's repair bench; receive Mercy's need.",
                frame.phase() != SurveyFramePhase::AwaitingNeed,
            ),
            (
                "recover",
                "Recover the frame parts in Tess's custody.",
                frame.object_revision() > 0,
            ),
        ]
        .into_iter()
        .map(|(id, text, complete)| InitiationObjective {
            id: id.into(),
            text: text.into(),
            complete,
        })
        .collect();
        let complete = objectives.iter().all(|o| o.complete);
        let objective = objectives
            .iter()
            .find(|o| !o.complete)
            .map(|o| o.text.clone())
            .unwrap_or_else(|| {
                "Introduction complete. Continue the repair case with O; F5 saves.".into()
            });
        Some(Self {
            mission_id: "mission.01.the-initiation".into(),
            title: "The Initiation".into(),
            objectives,
            objective,
            complete,
            recovery_case_id: crate::world::survey_frame::SURVEY_FRAME_CASE_ID.into(),
        })
    }
}
