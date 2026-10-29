//! Recorded Flynt Synthesis qualifications and authenticated execution outcomes.
//! This module does not execute a Recipe or grant clinical/Factory permissions.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::governance::{AmbisynthCredential, Attestation, Form, FormMastery, Pair};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", content = "form", rename_all = "kebab-case")]
pub enum BodilyForm {
    Sibling(Form),
    Chimera,
    Manticorp,
}

/// Preserve terminology provenance; historical wording is never a fourth pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PairTerm {
    Ambisynth,
    Bisynth,
}

/// A new interpretation never rewrites historical combined/perfected recipes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConvergenceInterpretation {
    RevolutionC99B,
    LegacyCombinedThenPerfected,
    EnactedChimeraPrerequisite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Convergence {
    Chimera,
    Manticorp,
}
impl Convergence {
    /// Display aliases share one outcome, Recipe and qualification record.
    pub const fn names(self) -> &'static [&'static str] {
        match self {
            Self::Chimera => &["Trisynthesis", "The Trifecta"],
            Self::Manticorp => &["Manticorp Synthesis"],
        }
    }
    pub const fn recipe_id(self) -> &'static str {
        match self {
            Self::Chimera => "flynt.recipe.revolution-c.trifecta",
            Self::Manticorp => "flynt.recipe.revolution-c.manticorp",
        }
    }
    pub const fn recipe_name(self) -> &'static str {
        match self {
            Self::Chimera => "The Trifecta Ritual",
            Self::Manticorp => "Manticorp Synthesis Recipe",
        }
    }
}

/// Identity/qualification metadata, not a manufacturing license or combat item.
pub const TRIPLE_CROWN_COMPONENTS: [(Form, &str); 3] = [
    (Form::Gargoyle, "Bolo"),
    (Form::Werewolf, "Crossbow"),
    (Form::Merman, "Harpoon"),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConvergenceVerification {
    pub person: String,
    pub outcome: Convergence,
    pub interpretation: ConvergenceInterpretation,
    pub recipe_id: String,
    pub recipe_revision: String,
    /// Authenticated successful execution of this exact subject/Recipe/outcome.
    pub execution: Attestation,
    /// Independent integration/competence certification, not ingredient count.
    pub competence: Attestation,
}
impl PairTerm {
    pub const fn active_name(self) -> &'static str {
        "Ambisynth"
    }
}

/// A receipt from a separately authorized execution adapter. That adapter must
/// authenticate the actual outcome and enforce applicable clinical clearance,
/// consent, material and Recipe rules. Recording a receipt cannot execute them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressionRecord {
    pub person: String,
    pub form: BodilyForm,
    pub previous: Option<BodilyForm>,
    pub execution: Attestation,
    pub consent: Attestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct SynthesisState {
    pub forms: BTreeMap<(String, Form), FormMastery>,
    pub pairs: BTreeMap<(String, Pair), (AmbisynthCredential, PairTerm)>,
    pub expressed: BTreeMap<String, BodilyForm>,
    pub interpretation: Option<ConvergenceInterpretation>,
    pub convergences: BTreeMap<(String, Convergence), ConvergenceVerification>,
}
