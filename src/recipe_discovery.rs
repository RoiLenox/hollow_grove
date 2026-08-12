//! Treasure-hunt Recipe discovery above the constitutional runtime boundary.

use crate::capability_taxonomy::FLYNT_TELEKINESIS;
use crate::world::power_recipes::RESYNCE_PROPULSION_RECIPE_ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecipeHouse {
    Stonebend,
    Sandmanor,
    Glaushouse,
    Flynt,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentKind {
    Rumor,
    Structure,
    Measurement,
    Compatibility,
    Witness,
    Material,
    Method,
    Return,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceConfidence {
    Rumored,
    Partial,
    Corroborated,
    Witnessed,
    Cleared,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrialOutcome {
    StableDiscovery,
    VariantDiscovery,
    PartialForm,
    HarmfulRemainder,
    NewClue,
    Refusal,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GargoyleHuntStage {
    Rumor,
    StonebendScarWall,
    SandmanorSkyArchive,
    GlaushouseRecoveryLedger,
    FlyntStormWitness,
    MaterialSearch,
    IncompleteRecipe,
    TrialSynthesis,
    FinalClearance,
    SafeFlightAndReturn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeFragment {
    pub id: String,
    pub recipe_id: String,
    pub house: RecipeHouse,
    pub kind: FragmentKind,
    pub confidence: EvidenceConfidence,
    pub provenance: String,
    pub statement: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeTrial {
    pub recipe_id: String,
    pub outcome: TrialOutcome,
    pub recovery_debt: u16,
    pub disclosed_clue: Option<String>,
    pub requires_clearance: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeDiscovery {
    recipe_id: String,
    fragments: Vec<RecipeFragment>,
    stage: GargoyleHuntStage,
    trial: Option<RecipeTrial>,
    clearance_granted: bool,
    maintenance: String,
    return_condition: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityUnlock {
    pub capability_id: &'static str,
    pub power_recipe_id: &'static str,
    pub requires_trial: TrialOutcome,
    pub requires_clearance: bool,
}

impl RecipeDiscovery {
    pub fn winged_gargoyle() -> Self {
        Self {
            recipe_id: "recipe.flynt.winged-gargoyle.v1".into(),
            fragments: Vec::new(),
            stage: GargoyleHuntStage::Rumor,
            trial: None,
            clearance_granted: false,
            maintenance:
                "Current circulation, wing inspection, Aura orientation, and recovery rest".into(),
            return_condition:
                "safe release, grounded identity, witnessed recovery, and no unresolved refusal"
                    .into(),
        }
    }

    pub fn recipe_id(&self) -> &str {
        &self.recipe_id
    }

    pub fn fragments(&self) -> &[RecipeFragment] {
        &self.fragments
    }

    pub const fn stage(&self) -> GargoyleHuntStage {
        self.stage
    }

    pub const fn trial(&self) -> Option<&RecipeTrial> {
        self.trial.as_ref()
    }

    pub const fn clearance_granted(&self) -> bool {
        self.clearance_granted
    }

    pub fn maintenance(&self) -> &str {
        &self.maintenance
    }

    pub fn return_condition(&self) -> &str {
        &self.return_condition
    }

    pub fn add_fragment(&mut self, fragment: RecipeFragment) -> Result<(), &'static str> {
        if fragment.recipe_id != self.recipe_id {
            return Err("fragment belongs to another Recipe");
        }
        if !stable_id(&fragment.id)
            || fragment.statement.trim().is_empty()
            || fragment.provenance.trim().is_empty()
        {
            return Err("fragment requires a stable ID, statement, and provenance");
        }
        if self.fragments.iter().any(|known| known.id == fragment.id) {
            return Err("duplicate Recipe fragment");
        }
        let (expected_house, expected_kind, minimum_confidence, next_stage) = match self.stage {
            GargoyleHuntStage::Rumor => (
                RecipeHouse::Stonebend,
                FragmentKind::Structure,
                EvidenceConfidence::Corroborated,
                GargoyleHuntStage::StonebendScarWall,
            ),
            GargoyleHuntStage::StonebendScarWall => (
                RecipeHouse::Sandmanor,
                FragmentKind::Measurement,
                EvidenceConfidence::Corroborated,
                GargoyleHuntStage::SandmanorSkyArchive,
            ),
            GargoyleHuntStage::SandmanorSkyArchive => (
                RecipeHouse::Glaushouse,
                FragmentKind::Compatibility,
                EvidenceConfidence::Cleared,
                GargoyleHuntStage::GlaushouseRecoveryLedger,
            ),
            GargoyleHuntStage::GlaushouseRecoveryLedger => (
                RecipeHouse::Flynt,
                FragmentKind::Witness,
                EvidenceConfidence::Witnessed,
                GargoyleHuntStage::FlyntStormWitness,
            ),
            GargoyleHuntStage::FlyntStormWitness => (
                RecipeHouse::Stonebend,
                FragmentKind::Material,
                EvidenceConfidence::Corroborated,
                GargoyleHuntStage::MaterialSearch,
            ),
            GargoyleHuntStage::MaterialSearch => (
                RecipeHouse::Sandmanor,
                FragmentKind::Method,
                EvidenceConfidence::Corroborated,
                GargoyleHuntStage::IncompleteRecipe,
            ),
            _ => return Err("Recipe discovery is not accepting another fragment"),
        };
        if fragment.house != expected_house || fragment.kind != expected_kind {
            return Err("fragment does not match the current treasure-hunt stage");
        }
        if fragment.confidence < minimum_confidence {
            return Err("fragment confidence is below the stage requirement");
        }
        self.fragments.push(fragment);
        self.stage = next_stage;
        Ok(())
    }
    pub fn record_trial(&mut self, trial: RecipeTrial) -> Result<(), &'static str> {
        if trial.recipe_id != self.recipe_id {
            return Err("trial belongs to another Recipe");
        }
        if self.trial.is_some() {
            return Err("Recipe trial already recorded");
        }
        if self.stage != GargoyleHuntStage::IncompleteRecipe {
            return Err("Recipe evidence and method are incomplete");
        }
        if !trial.requires_clearance {
            return Err("winged Gargoyle trial must preserve clinical clearance");
        }
        self.trial = Some(trial);
        self.stage = GargoyleHuntStage::TrialSynthesis;
        Ok(())
    }

    pub fn grant_clinical_clearance(&mut self) -> Result<(), &'static str> {
        if !self.can_request_clearance() {
            return Err("Recipe is not ready for clinical clearance");
        }
        self.clearance_granted = true;
        self.stage = GargoyleHuntStage::FinalClearance;
        Ok(())
    }
    pub fn can_request_clearance(&self) -> bool {
        self.stage == GargoyleHuntStage::TrialSynthesis
            && self.fragments.len() == 6
            && self.trial.as_ref().is_some_and(|trial| {
                trial.outcome == TrialOutcome::StableDiscovery && trial.requires_clearance
            })
    }

    /// Returns the pilot capability candidate only after ordered evidence, a
    /// stable trial, and the separate clinical Clearance step all succeed.
    pub fn winged_gargoyle_candidates(&self) -> Option<Vec<CapabilityUnlock>> {
        let trial = self.trial.as_ref()?;
        if !self.clearance_granted
            || !matches!(
                self.stage,
                GargoyleHuntStage::FinalClearance | GargoyleHuntStage::SafeFlightAndReturn
            )
            || self.fragments.len() != 6
            || trial.outcome != TrialOutcome::StableDiscovery
            || !trial.requires_clearance
        {
            return None;
        }
        Some(vec![CapabilityUnlock {
            capability_id: FLYNT_TELEKINESIS.id,
            power_recipe_id: RESYNCE_PROPULSION_RECIPE_ID,
            requires_trial: trial.outcome,
            requires_clearance: true,
        }])
    }

    pub fn complete_safe_flight_and_return(&mut self) -> Result<(), &'static str> {
        if self.stage != GargoyleHuntStage::FinalClearance || !self.clearance_granted {
            return Err("safe flight requires final clinical clearance");
        }
        self.stage = GargoyleHuntStage::SafeFlightAndReturn;
        Ok(())
    }
}

fn stable_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fragment(id: &str, house: RecipeHouse, kind: FragmentKind) -> RecipeFragment {
        RecipeFragment {
            id: id.into(),
            recipe_id: "recipe.flynt.winged-gargoyle.v1".into(),
            house,
            kind,
            confidence: match kind {
                FragmentKind::Compatibility => EvidenceConfidence::Cleared,
                FragmentKind::Witness => EvidenceConfidence::Witnessed,
                _ => EvidenceConfidence::Corroborated,
            },
            provenance: "pilot witness record".into(),
            statement: "bounded Recipe evidence".into(),
        }
    }
    #[test]
    fn winged_gargoyle_requires_four_house_evidence_before_clearance() {
        let mut discovery = RecipeDiscovery::winged_gargoyle();
        for (id, house, kind) in [
            ("stone", RecipeHouse::Stonebend, FragmentKind::Structure),
            ("sand", RecipeHouse::Sandmanor, FragmentKind::Measurement),
            ("glau", RecipeHouse::Glaushouse, FragmentKind::Compatibility),
            ("flynt", RecipeHouse::Flynt, FragmentKind::Witness),
            ("material", RecipeHouse::Stonebend, FragmentKind::Material),
            ("method", RecipeHouse::Sandmanor, FragmentKind::Method),
        ] {
            discovery.add_fragment(fragment(id, house, kind)).unwrap();
        }
        assert!(!discovery.can_request_clearance());
        discovery
            .record_trial(RecipeTrial {
                recipe_id: discovery.recipe_id.clone(),
                outcome: TrialOutcome::StableDiscovery,
                recovery_debt: 0,
                disclosed_clue: Some("safe return is possible".into()),
                requires_clearance: true,
            })
            .unwrap();
        assert!(discovery.can_request_clearance());
        assert!(discovery.winged_gargoyle_candidates().is_none());
        discovery.grant_clinical_clearance().unwrap();
        assert_eq!(discovery.stage(), GargoyleHuntStage::FinalClearance);
        let candidates = discovery.winged_gargoyle_candidates().unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].capability_id, "capability.flynt.telekinesis");
        assert_eq!(
            candidates[0].power_recipe_id,
            "recipe.flight.resynce-propulsion"
        );
        assert!(
            crate::world::power_recipes::draft_power_recipe_catalog()
                .iter()
                .any(|recipe| recipe.id == candidates[0].power_recipe_id)
        );
        discovery.complete_safe_flight_and_return().unwrap();
        assert_eq!(discovery.stage(), GargoyleHuntStage::SafeFlightAndReturn);
    }

    #[test]
    fn winged_gargoyle_rejects_out_of_order_or_weak_evidence_and_early_trials() {
        let mut discovery = RecipeDiscovery::winged_gargoyle();
        assert!(
            discovery
                .add_fragment(fragment(
                    "sand",
                    RecipeHouse::Sandmanor,
                    FragmentKind::Measurement,
                ))
                .is_err()
        );
        assert!(
            discovery
                .record_trial(RecipeTrial {
                    recipe_id: discovery.recipe_id.clone(),
                    outcome: TrialOutcome::StableDiscovery,
                    recovery_debt: 0,
                    disclosed_clue: None,
                    requires_clearance: true,
                })
                .is_err()
        );
        let mut weak = fragment("stone", RecipeHouse::Stonebend, FragmentKind::Structure);
        weak.confidence = EvidenceConfidence::Partial;
        assert!(discovery.add_fragment(weak).is_err());
        assert_eq!(discovery.stage(), GargoyleHuntStage::Rumor);
    }
}
