//! Playable orchestration for Hueman's first bounded Synthesis treasure hunt.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::recipe_discovery::{
    EvidenceConfidence, FragmentKind, GargoyleHuntStage, RecipeDiscovery, RecipeFragment,
    RecipeHouse, RecipeTrial, TrialOutcome,
};

pub const WINGED_GARGOYLE_RECIPE_ID: &str = "recipe.flynt.winged-gargoyle.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WingedGargoylePilotAction {
    RecoverStonebendStructure,
    RecoverSandmanorMeasurement,
    ObtainGlaushouseCompatibility,
    EarnFlyntWitness,
    RecoverStonebendMaterial,
    PreserveSandmanorMethod,
    RecordStableTrial,
    GrantClinicalClearance,
    CompleteSafeFlightAndReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WingedGargoylePilotForm {
    Hueman,
    Gargoyle,
    WingedGargoyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WingedGargoylePilotEvent {
    pub action: WingedGargoylePilotAction,
    pub stage: GargoyleHuntStage,
    pub form: WingedGargoylePilotForm,
    pub fragment_count: usize,
    pub capability_id: Option<String>,
    pub power_recipe_id: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WingedGargoylePilotState {
    discovery: RecipeDiscovery,
    form: WingedGargoylePilotForm,
    capability_id: Option<String>,
    power_recipe_id: Option<String>,
    completed: bool,
}

impl Default for WingedGargoylePilotState {
    fn default() -> Self {
        Self {
            discovery: RecipeDiscovery::winged_gargoyle(),
            form: WingedGargoylePilotForm::Hueman,
            capability_id: None,
            power_recipe_id: None,
            completed: false,
        }
    }
}

impl WingedGargoylePilotState {
    #[must_use]
    pub fn recipe_id(&self) -> &str {
        self.discovery.recipe_id()
    }

    #[must_use]
    pub const fn stage(&self) -> GargoyleHuntStage {
        self.discovery.stage()
    }

    #[must_use]
    pub const fn form(&self) -> WingedGargoylePilotForm {
        self.form
    }

    #[must_use]
    pub fn fragment_count(&self) -> usize {
        self.discovery.fragments().len()
    }

    #[must_use]
    pub fn capability_id(&self) -> Option<&str> {
        self.capability_id.as_deref()
    }

    #[must_use]
    pub fn power_recipe_id(&self) -> Option<&str> {
        self.power_recipe_id.as_deref()
    }

    #[must_use]
    pub const fn completed(&self) -> bool {
        self.completed
    }

    #[must_use]
    pub const fn next_action(&self) -> Option<WingedGargoylePilotAction> {
        if self.completed {
            return None;
        }
        Some(match self.discovery.stage() {
            GargoyleHuntStage::Rumor => WingedGargoylePilotAction::RecoverStonebendStructure,
            GargoyleHuntStage::StonebendScarWall => {
                WingedGargoylePilotAction::RecoverSandmanorMeasurement
            }
            GargoyleHuntStage::SandmanorSkyArchive => {
                WingedGargoylePilotAction::ObtainGlaushouseCompatibility
            }
            GargoyleHuntStage::GlaushouseRecoveryLedger => {
                WingedGargoylePilotAction::EarnFlyntWitness
            }
            GargoyleHuntStage::FlyntStormWitness => {
                WingedGargoylePilotAction::RecoverStonebendMaterial
            }
            GargoyleHuntStage::MaterialSearch => WingedGargoylePilotAction::PreserveSandmanorMethod,
            GargoyleHuntStage::IncompleteRecipe => WingedGargoylePilotAction::RecordStableTrial,
            GargoyleHuntStage::TrialSynthesis => WingedGargoylePilotAction::GrantClinicalClearance,
            GargoyleHuntStage::FinalClearance => {
                WingedGargoylePilotAction::CompleteSafeFlightAndReturn
            }
            GargoyleHuntStage::SafeFlightAndReturn => return None,
        })
    }

    pub fn advance(
        &mut self,
        action: WingedGargoylePilotAction,
    ) -> Result<WingedGargoylePilotEvent, WingedGargoylePilotError> {
        let mut candidate = self.clone();
        let event = candidate.advance_in_place(action)?;
        *self = candidate;
        Ok(event)
    }

    fn advance_in_place(
        &mut self,
        action: WingedGargoylePilotAction,
    ) -> Result<WingedGargoylePilotEvent, WingedGargoylePilotError> {
        let expected = self
            .next_action()
            .ok_or(WingedGargoylePilotError::AlreadyCompleted)?;
        if action != expected {
            return Err(WingedGargoylePilotError::UnexpectedAction {
                expected,
                actual: action,
            });
        }

        match action {
            WingedGargoylePilotAction::RecoverStonebendStructure => self.add_fragment(
                "fragment.stonebend.scar-wall-structure",
                RecipeHouse::Stonebend,
                FragmentKind::Structure,
                EvidenceConfidence::Corroborated,
                "Stonebend scar-wall survey",
                "The grounded load path can carry a bounded wing frame.",
            )?,
            WingedGargoylePilotAction::RecoverSandmanorMeasurement => self.add_fragment(
                "fragment.sandmanor.sky-archive-measurement",
                RecipeHouse::Sandmanor,
                FragmentKind::Measurement,
                EvidenceConfidence::Corroborated,
                "Sandmanor sky archive",
                "Lift, mass, and Current circulation remain inside the recorded envelope.",
            )?,
            WingedGargoylePilotAction::ObtainGlaushouseCompatibility => self.add_fragment(
                "fragment.glaushouse.recovery-compatibility",
                RecipeHouse::Glaushouse,
                FragmentKind::Compatibility,
                EvidenceConfidence::Cleared,
                "Glaushouse recovery ledger",
                "Hueman's Aura pattern is compatible with a reversible stone form.",
            )?,
            WingedGargoylePilotAction::EarnFlyntWitness => self.add_fragment(
                "fragment.flynt.storm-witness",
                RecipeHouse::Flynt,
                FragmentKind::Witness,
                EvidenceConfidence::Witnessed,
                "Flynt storm witness",
                "A witnessed Resynce pulse held wings without severing grounded identity.",
            )?,
            WingedGargoylePilotAction::RecoverStonebendMaterial => self.add_fragment(
                "fragment.stonebend.gargoyle-material",
                RecipeHouse::Stonebend,
                FragmentKind::Material,
                EvidenceConfidence::Corroborated,
                "Stonebend material register",
                "The selected stone shell releases cleanly under the return condition.",
            )?,
            WingedGargoylePilotAction::PreserveSandmanorMethod => self.add_fragment(
                "fragment.sandmanor.return-method",
                RecipeHouse::Sandmanor,
                FragmentKind::Method,
                EvidenceConfidence::Corroborated,
                "Sandmanor preserved method",
                "The procedure orders grounding, lift, safe release, and witnessed recovery.",
            )?,
            WingedGargoylePilotAction::RecordStableTrial => {
                self.discovery
                    .record_trial(RecipeTrial {
                        recipe_id: WINGED_GARGOYLE_RECIPE_ID.into(),
                        outcome: TrialOutcome::StableDiscovery,
                        recovery_debt: 0,
                        disclosed_clue: Some(
                            "safe return remains possible after a bounded flight".into(),
                        ),
                        requires_clearance: true,
                    })
                    .map_err(WingedGargoylePilotError::Recipe)?;
                self.form = WingedGargoylePilotForm::Gargoyle;
            }
            WingedGargoylePilotAction::GrantClinicalClearance => {
                self.discovery
                    .grant_clinical_clearance()
                    .map_err(WingedGargoylePilotError::Recipe)?;
                let candidate = self
                    .discovery
                    .winged_gargoyle_candidates()
                    .and_then(|candidates| candidates.into_iter().next())
                    .ok_or(WingedGargoylePilotError::MissingUnlockCandidate)?;
                self.capability_id = Some(candidate.capability_id.into());
                self.power_recipe_id = Some(candidate.power_recipe_id.into());
                self.form = WingedGargoylePilotForm::WingedGargoyle;
            }
            WingedGargoylePilotAction::CompleteSafeFlightAndReturn => {
                self.discovery
                    .complete_safe_flight_and_return()
                    .map_err(WingedGargoylePilotError::Recipe)?;
                self.form = WingedGargoylePilotForm::Hueman;
                self.completed = true;
            }
        }
        Ok(self.event(action))
    }

    pub fn replay(
        &mut self,
        expected: &WingedGargoylePilotEvent,
    ) -> Result<(), WingedGargoylePilotError> {
        let actual = self.advance(expected.action)?;
        if actual != *expected {
            return Err(WingedGargoylePilotError::ReplayDivergence);
        }
        Ok(())
    }

    fn add_fragment(
        &mut self,
        id: &str,
        house: RecipeHouse,
        kind: FragmentKind,
        confidence: EvidenceConfidence,
        provenance: &str,
        statement: &str,
    ) -> Result<(), WingedGargoylePilotError> {
        self.discovery
            .add_fragment(RecipeFragment {
                id: id.into(),
                recipe_id: WINGED_GARGOYLE_RECIPE_ID.into(),
                house,
                kind,
                confidence,
                provenance: provenance.into(),
                statement: statement.into(),
            })
            .map_err(WingedGargoylePilotError::Recipe)
    }

    fn event(&self, action: WingedGargoylePilotAction) -> WingedGargoylePilotEvent {
        WingedGargoylePilotEvent {
            action,
            stage: self.stage(),
            form: self.form,
            fragment_count: self.fragment_count(),
            capability_id: self.capability_id.clone(),
            power_recipe_id: self.power_recipe_id.clone(),
            completed: self.completed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WingedGargoylePilotError {
    AlreadyCompleted,
    UnexpectedAction {
        expected: WingedGargoylePilotAction,
        actual: WingedGargoylePilotAction,
    },
    Recipe(&'static str),
    MissingUnlockCandidate,
    ReplayDivergence,
}

impl fmt::Display for WingedGargoylePilotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "winged Gargoyle pilot rejected action: {self:?}")
    }
}

impl std::error::Error for WingedGargoylePilotError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_pilot_unlocks_flight_and_returns_as_hueman() {
        let mut state = WingedGargoylePilotState::default();
        while let Some(action) = state.next_action() {
            state.advance(action).unwrap();
        }
        assert!(state.completed());
        assert_eq!(state.form(), WingedGargoylePilotForm::Hueman);
        assert_eq!(state.fragment_count(), 6);
        assert_eq!(state.capability_id(), Some("capability.flynt.telekinesis"));
        assert_eq!(
            state.power_recipe_id(),
            Some("recipe.flight.resynce-propulsion")
        );
    }

    #[test]
    fn canonical_pilot_rejects_out_of_order_actions_transactionally() {
        let mut state = WingedGargoylePilotState::default();
        let before = state.clone();
        assert!(
            state
                .advance(WingedGargoylePilotAction::RecordStableTrial)
                .is_err()
        );
        assert_eq!(state, before);
    }
}
