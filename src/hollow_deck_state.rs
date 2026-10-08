//! Persisted player cultivation state for the Hollow Deck.
//!
//! The canonical [`HollowDeckCatalog`] is immutable source data. This module
//! stores only sparse player-owned overlays: developed Current Forms, learned
//! and equipped Aura moves, discovered patterns, and known results.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::artifact_io::write_text_artifact;
use crate::hollow_deck::{
    AuraSet, CurrentForm, GlaushouseBranch, GlaushouseSeat, HollowDeckCatalog, StatBlock,
    achieved_growth_stages, compute_transformed_stats, get_growth_path_for_nature,
    get_growth_stage_index, validate_synthesis_requirements,
};
use crate::runtime_rampart::component_digest;

/// Persisted state format/version. This is deliberately independent from the
/// canonical catalog's semantic `HOLLOW_DECK_SCHEMA_VERSION`.
pub const HOLLOW_DECK_STATE_FORMAT: &str = "hollow-grove.hollow-deck-state";
pub const HOLLOW_DECK_STATE_VERSION: u16 = 1;
pub const HOLLOW_DECK_STATE_ENV: &str = "HOLLOW_DECK_STATE_PATH";
pub const DEFAULT_HOLLOW_DECK_STATE_PATH: &str =
    "artifacts/gameplay-saves/hollow-deck-state-v1.json";

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CurrentFormProgress {
    /// 0 is the prior rung/Hueman baseline and 10,000 is full expression.
    pub development_basis_points: u16,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub feature_stages: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub technique_stages: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub growth_stage: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AuraSetProgress {
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub learned_moves: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub equipped_moves: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlaushouseAffiliationState {
    pub seat: GlaushouseSeat,
    /// Role and Aura material derive from this value and are never stored separately.
    pub branch: Option<GlaushouseBranch>,
}

impl GlaushouseAffiliationState {
    #[must_use]
    pub fn title(&self) -> String {
        self.branch.map_or_else(
            || self.seat.entry_title(),
            |branch| branch.achieved_title(self.seat),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HollowDeckState {
    pub schema_version: u16,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub current_forms: BTreeMap<String, CurrentFormProgress>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub aura_sets: BTreeMap<String, AuraSetProgress>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub known_recipes: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub discovered_synthesis_recipes: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub known_syntheses: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glaushouse_affiliation: Option<GlaushouseAffiliationState>,
}

impl Default for HollowDeckState {
    fn default() -> Self {
        Self {
            schema_version: HOLLOW_DECK_STATE_VERSION,
            current_forms: BTreeMap::new(),
            aura_sets: BTreeMap::new(),
            known_recipes: BTreeSet::new(),
            discovered_synthesis_recipes: BTreeSet::new(),
            known_syntheses: BTreeSet::new(),
            glaushouse_affiliation: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HollowDeckStateMigration {
    pub migrated_gargoyle_wing_stage: bool,
    pub normalized_white_opal_statue: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HollowDeckStateEnvelope {
    format: String,
    version: u16,
    checksum: String,
    state: HollowDeckState,
}

#[derive(Debug)]
pub enum HollowDeckStateError {
    Io(std::io::Error),
    Json(serde_json::Error),
    UnsupportedFormat(String),
    UnsupportedVersion(u16),
    ChecksumMismatch,
    Invalid(Vec<String>),
    InvalidAction(String),
}

impl std::fmt::Display for HollowDeckStateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "Hollow Deck state I/O error: {error}"),
            Self::Json(error) => write!(formatter, "Hollow Deck state JSON error: {error}"),
            Self::UnsupportedFormat(format) => {
                write!(formatter, "unsupported Hollow Deck state format: {format}")
            }
            Self::UnsupportedVersion(version) => {
                write!(
                    formatter,
                    "unsupported Hollow Deck state version: {version}"
                )
            }
            Self::ChecksumMismatch => write!(formatter, "Hollow Deck state checksum mismatch"),
            Self::Invalid(errors) => write!(
                formatter,
                "invalid Hollow Deck state: {}",
                errors.join("; ")
            ),
            Self::InvalidAction(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for HollowDeckStateError {}

impl From<std::io::Error> for HollowDeckStateError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for HollowDeckStateError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl HollowDeckState {
    pub fn cultivate(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        development_basis_points: u16,
    ) -> Result<StatBlock, HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let stats = compute_transformed_stats(catalog, &nature.id, development_basis_points)
            .map_err(|error| HollowDeckStateError::InvalidAction(error.to_string()))?;
        self.current_forms
            .entry(nature.id.clone())
            .or_default()
            .development_basis_points = development_basis_points;
        Ok(stats)
    }

    pub fn set_growth_stage(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        stage: &str,
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let path = get_growth_path_for_nature(catalog, &nature.id).ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "{} has no unified biological growth path",
                nature.name
            ))
        })?;
        let index = get_growth_stage_index(path, stage).ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "invalid {} stage: {stage}; choose {}",
                path.name,
                path.milestones
                    .iter()
                    .map(|milestone| milestone.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })?;
        self.current_forms
            .entry(nature.id.clone())
            .or_default()
            .growth_stage = Some(path.milestones[index].id.clone());
        Ok(())
    }

    pub fn set_glaushouse_affiliation(
        &mut self,
        catalog: &HollowDeckCatalog,
        seat: &str,
        branch: Option<&str>,
    ) -> Result<GlaushouseAffiliationState, HollowDeckStateError> {
        let seat = resolve_glaushouse_seat(catalog, seat)?;
        let branch = branch.map(resolve_glaushouse_branch).transpose()?;
        let affiliation = GlaushouseAffiliationState { seat, branch };
        self.glaushouse_affiliation = Some(affiliation.clone());
        Ok(affiliation)
    }

    pub fn set_current_feature_stage(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        feature: &str,
        stage: &str,
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        if get_growth_path_for_nature(catalog, &nature.id).is_some()
            && token_matches(feature, "wings")
        {
            let growth_stage = if token_matches(stage, "buds") {
                "wing-buds"
            } else {
                stage
            };
            return self.set_growth_stage(catalog, &nature.id, growth_stage);
        }
        let form = catalog.current_form(&nature.id).ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "{} has no Current Form record",
                nature.name
            ))
        })?;
        let feature = form
            .current_features
            .iter()
            .find(|item| token_matches(&item.id, feature) || token_matches(&item.name, feature))
            .ok_or_else(|| {
                HollowDeckStateError::InvalidAction(format!(
                    "unknown Current feature for {}: {feature}",
                    nature.name
                ))
            })?;
        if feature.stages.is_empty() {
            return Err(HollowDeckStateError::InvalidAction(format!(
                "{} has no canonical stages yet",
                feature.name
            )));
        }
        let stage = resolve_value(&feature.stages, stage).ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "invalid {} stage: {stage}; choose {}",
                feature.name,
                feature.stages.join(", ")
            ))
        })?;
        self.current_forms
            .entry(nature.id.clone())
            .or_default()
            .feature_stages
            .insert(feature.id.clone(), stage);
        Ok(())
    }

    pub fn set_current_technique_stage(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        technique: &str,
        stage: &str,
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let form = catalog.current_form(&nature.id).ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "{} has no Current Form record",
                nature.name
            ))
        })?;
        let technique = form
            .current_techniques
            .iter()
            .find(|item| token_matches(&item.id, technique) || token_matches(&item.name, technique))
            .ok_or_else(|| {
                HollowDeckStateError::InvalidAction(format!(
                    "unknown Current technique for {}: {technique}",
                    nature.name
                ))
            })?;
        let stage = resolve_value(&technique.stages, stage).ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "invalid {} stage: {stage}; choose {}",
                technique.name,
                technique.stages.join(", ")
            ))
        })?;
        self.current_forms
            .entry(nature.id.clone())
            .or_default()
            .technique_stages
            .insert(technique.id.clone(), stage);
        Ok(())
    }

    pub fn learn_aura_move(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        aura_move: &str,
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let aura_move = resolve_aura_move(catalog, &nature.id, aura_move)?;
        self.aura_sets
            .entry(nature.id.clone())
            .or_default()
            .learned_moves
            .insert(aura_move.id.clone());
        Ok(())
    }

    pub fn equip_aura_move(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        aura_move: &str,
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let aura_move = resolve_aura_move(catalog, &nature.id, aura_move)?;
        let set = self.aura_sets.entry(nature.id.clone()).or_default();
        if !set.learned_moves.contains(&aura_move.id) {
            return Err(HollowDeckStateError::InvalidAction(format!(
                "{} must learn {} before equipping it",
                nature.name, aura_move.name
            )));
        }
        set.equipped_moves.insert(aura_move.id.clone());
        Ok(())
    }

    pub fn unequip_aura_move(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        aura_move: &str,
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let aura_move = resolve_aura_move(catalog, &nature.id, aura_move)?;
        self.aura_sets
            .entry(nature.id.clone())
            .or_default()
            .equipped_moves
            .remove(&aura_move.id);
        Ok(())
    }

    pub fn learn_recipe(
        &mut self,
        catalog: &HollowDeckCatalog,
        recipe: &str,
    ) -> Result<(), HollowDeckStateError> {
        let recipe = catalog
            .recipes
            .iter()
            .find(|item| token_matches(&item.id, recipe) || token_matches(&item.name, recipe))
            .ok_or_else(|| {
                HollowDeckStateError::InvalidAction(format!("unknown Recipe: {recipe}"))
            })?;
        self.known_recipes.insert(recipe.id.clone());
        Ok(())
    }

    pub fn discover_synthesis_recipe(
        &mut self,
        catalog: &HollowDeckCatalog,
        recipe: &str,
    ) -> Result<(), HollowDeckStateError> {
        let recipe = catalog
            .synthesis_recipes
            .iter()
            .find(|item| token_matches(&item.id, recipe) || token_matches(&item.name, recipe))
            .ok_or_else(|| {
                HollowDeckStateError::InvalidAction(format!("unknown Synthesis Recipe: {recipe}"))
            })?;
        self.discovered_synthesis_recipes.insert(recipe.id.clone());
        Ok(())
    }

    pub fn record_synthesis(
        &mut self,
        catalog: &HollowDeckCatalog,
        nature: &str,
        synthesis: &str,
        available_reagents: &[String],
    ) -> Result<(), HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let synthesis = catalog
            .syntheses
            .iter()
            .find(|item| token_matches(&item.id, synthesis) || token_matches(&item.name, synthesis))
            .ok_or_else(|| {
                HollowDeckStateError::InvalidAction(format!("unknown Synthesis: {synthesis}"))
            })?;
        if !synthesis.compatible_natures.is_empty()
            && !synthesis.compatible_natures.contains(&nature.id)
        {
            return Err(HollowDeckStateError::InvalidAction(format!(
                "{} is not compatible with {}",
                nature.name, synthesis.name
            )));
        }
        let current_form = self.current_form_for(catalog, &nature.id)?;
        let aura_set = self.aura_set_for(catalog, &nature.id)?;
        let current_capabilities = current_form
            .as_ref()
            .map(|form| {
                form.current_features
                    .iter()
                    .map(|item| item.id.as_str())
                    .chain(form.current_techniques.iter().map(|item| item.id.as_str()))
                    .chain(form.achieved_growth_stages.iter().map(String::as_str))
                    .chain(std::iter::once(nature.primary_trait_id.as_str()))
                    .collect::<BTreeSet<_>>()
            })
            .unwrap_or_default();
        let aura_capabilities = aura_set
            .as_ref()
            .map(|set| {
                set.learned_moves
                    .iter()
                    .map(String::as_str)
                    .collect::<BTreeSet<_>>()
            })
            .unwrap_or_default();
        let reagents = available_reagents
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let missing_current = synthesis
            .current_requirements
            .iter()
            .filter(|id| !current_capabilities.contains(id.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let missing_aura = synthesis
            .aura_requirements
            .iter()
            .filter(|id| !aura_capabilities.contains(id.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let missing_reagents = synthesis
            .reagent_requirements
            .iter()
            .filter(|id| !reagents.contains(id.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let missing_growth = synthesis
            .required_growth_stage
            .as_ref()
            .and_then(|requirement| {
                (requirement.nature_id != nature.id
                    || !current_capabilities.contains(requirement.milestone_id.as_str()))
                .then(|| requirement.milestone_id.clone())
            });
        if !missing_current.is_empty()
            || !missing_aura.is_empty()
            || !missing_reagents.is_empty()
            || missing_growth.is_some()
        {
            return Err(HollowDeckStateError::InvalidAction(format!(
                "requirements not met for {}: Current [{}], growth [{}], Aura [{}], reagents [{}]",
                synthesis.name,
                missing_current.join(", "),
                missing_growth.as_deref().unwrap_or_default(),
                missing_aura.join(", "),
                missing_reagents.join(", ")
            )));
        }
        if let Some(recipe_id) = synthesis.source_recipe_id.as_deref() {
            if !self.discovered_synthesis_recipes.contains(recipe_id) {
                return Err(HollowDeckStateError::InvalidAction(format!(
                    "discover Synthesis Recipe {recipe_id} before recording {}",
                    synthesis.name
                )));
            }
            let recipe = catalog
                .synthesis_recipes
                .iter()
                .find(|item| item.id == recipe_id)
                .ok_or_else(|| {
                    HollowDeckStateError::InvalidAction(format!(
                        "Synthesis {} references missing Recipe {recipe_id}",
                        synthesis.name
                    ))
                })?;
            let report = validate_synthesis_requirements(
                recipe,
                &nature.id,
                current_form.as_ref(),
                aura_set.as_ref(),
                available_reagents,
            );
            if !report.compatible
                || !report.missing_current_features.is_empty()
                || !report.missing_current_techniques.is_empty()
                || report.missing_growth_stage.is_some()
                || !report.missing_aura_moves.is_empty()
                || !report.missing_reagents.is_empty()
            {
                return Err(HollowDeckStateError::InvalidAction(format!(
                    "requirements not met for {}: features [{}], techniques [{}], growth [{}], Aura [{}], reagents [{}]",
                    synthesis.name,
                    report.missing_current_features.join(", "),
                    report.missing_current_techniques.join(", "),
                    report.missing_growth_stage.as_deref().unwrap_or_default(),
                    report.missing_aura_moves.join(", "),
                    report.missing_reagents.join(", ")
                )));
            }
        }
        self.known_syntheses.insert(synthesis.id.clone());
        Ok(())
    }

    pub fn current_form_for(
        &self,
        catalog: &HollowDeckCatalog,
        nature: &str,
    ) -> Result<Option<CurrentForm>, HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let Some(progress) = self.current_forms.get(&nature.id) else {
            return Ok(None);
        };
        let mut form = catalog.current_form(&nature.id).cloned().ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "{} has no Current Form record",
                nature.name
            ))
        })?;
        form.development_basis_points = progress.development_basis_points;
        form.computed_stats =
            compute_transformed_stats(catalog, &nature.id, progress.development_basis_points)
                .map_err(|error| HollowDeckStateError::InvalidAction(error.to_string()))?;
        form.current_growth_stage = progress.growth_stage.clone();
        form.achieved_growth_stages = match (
            get_growth_path_for_nature(catalog, &nature.id),
            progress.growth_stage.as_deref(),
        ) {
            (Some(path), Some(stage)) => achieved_growth_stages(path, stage).ok_or_else(|| {
                HollowDeckStateError::InvalidAction(format!(
                    "invalid {} growth stage: {stage}",
                    nature.name
                ))
            })?,
            _ => Vec::new(),
        };
        for feature in &mut form.current_features {
            feature.current_stage = progress.feature_stages.get(&feature.id).cloned();
        }
        for technique in &mut form.current_techniques {
            technique.current_stage = progress.technique_stages.get(&technique.id).cloned();
        }
        Ok(Some(form))
    }

    pub fn aura_set_for(
        &self,
        catalog: &HollowDeckCatalog,
        nature: &str,
    ) -> Result<Option<AuraSet>, HollowDeckStateError> {
        let nature = resolve_nature(catalog, nature)?;
        let Some(progress) = self.aura_sets.get(&nature.id) else {
            return Ok(None);
        };
        let mut set = catalog.aura_set(&nature.id).cloned().unwrap_or(AuraSet {
            nature_id: nature.id.clone(),
            learned_moves: Vec::new(),
            equipped_moves: Vec::new(),
            design_examples: Vec::new(),
        });
        set.learned_moves = progress.learned_moves.iter().cloned().collect();
        set.equipped_moves = progress.equipped_moves.iter().cloned().collect();
        Ok(Some(set))
    }
}

#[must_use]
pub fn hollow_deck_state_path() -> PathBuf {
    std::env::var_os(HOLLOW_DECK_STATE_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_HOLLOW_DECK_STATE_PATH))
}

pub fn validate_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    state: &HollowDeckState,
) -> Result<(), HollowDeckStateError> {
    let mut normalized = state.clone();
    normalize_legacy_hollow_deck_state(catalog, &mut normalized);
    validate_normalized_hollow_deck_state(catalog, &normalized)
}

fn validate_normalized_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    state: &HollowDeckState,
) -> Result<(), HollowDeckStateError> {
    let mut errors = Vec::new();
    if state.schema_version != HOLLOW_DECK_STATE_VERSION {
        errors.push(format!(
            "schema version {} is not supported",
            state.schema_version
        ));
    }
    for (nature_id, progress) in &state.current_forms {
        let Some(form) = catalog.current_form(nature_id) else {
            errors.push(format!("unknown Current Form Nature: {nature_id}"));
            continue;
        };
        if progress.development_basis_points > 10_000 {
            errors.push(format!("{nature_id} Current development exceeds 10,000"));
        }
        match (
            get_growth_path_for_nature(catalog, nature_id),
            progress.growth_stage.as_deref(),
        ) {
            (Some(path), Some(stage)) if get_growth_stage_index(path, stage).is_none() => {
                errors.push(format!("invalid {nature_id} unified growth stage: {stage}"));
            }
            (None, Some(stage)) => {
                errors.push(format!(
                    "{nature_id} has no unified growth path for {stage}"
                ));
            }
            _ => {}
        }
        if get_growth_path_for_nature(catalog, nature_id).is_some()
            && !progress.feature_stages.is_empty()
        {
            errors.push(format!(
                "{nature_id} unified growth cannot be stored as independent feature stages"
            ));
        }
        validate_stages(
            nature_id,
            "feature",
            &progress.feature_stages,
            form.current_features
                .iter()
                .map(|item| (&item.id, &item.stages)),
            &mut errors,
        );
        validate_stages(
            nature_id,
            "technique",
            &progress.technique_stages,
            form.current_techniques
                .iter()
                .map(|item| (&item.id, &item.stages)),
            &mut errors,
        );
    }
    for (nature_id, progress) in &state.aura_sets {
        if catalog.nature(nature_id).is_none() {
            errors.push(format!("unknown Aura Set Nature: {nature_id}"));
            continue;
        }
        for move_id in &progress.learned_moves {
            if resolve_aura_move(catalog, nature_id, move_id).is_err() {
                errors.push(format!("invalid Aura move {move_id} for {nature_id}"));
            }
        }
        if !progress.equipped_moves.is_subset(&progress.learned_moves) {
            errors.push(format!(
                "{nature_id} has equipped Aura moves that are not learned"
            ));
        }
    }
    validate_ids(
        "Recipe",
        &state.known_recipes,
        &catalog
            .recipes
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        &mut errors,
    );
    if let Some(affiliation) = &state.glaushouse_affiliation
        && !catalog
            .glaushouse_structure
            .seats
            .contains(&affiliation.seat)
    {
        errors.push("Glaüshouse affiliation references a noncanonical seat".into());
    }
    validate_ids(
        "Synthesis Recipe",
        &state.discovered_synthesis_recipes,
        &catalog
            .synthesis_recipes
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        &mut errors,
    );
    validate_ids(
        "Synthesis",
        &state.known_syntheses,
        &catalog
            .syntheses
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        &mut errors,
    );
    if errors.is_empty() {
        Ok(())
    } else {
        Err(HollowDeckStateError::Invalid(errors))
    }
}

pub fn encode_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    state: &HollowDeckState,
) -> Result<String, HollowDeckStateError> {
    let mut state = state.clone();
    normalize_legacy_hollow_deck_state(catalog, &mut state);
    validate_normalized_hollow_deck_state(catalog, &state)?;
    let state_bytes = serde_json::to_vec(&state)?;
    let envelope = HollowDeckStateEnvelope {
        format: HOLLOW_DECK_STATE_FORMAT.into(),
        version: HOLLOW_DECK_STATE_VERSION,
        checksum: component_digest(&state_bytes),
        state,
    };
    let mut json = serde_json::to_string_pretty(&envelope)?;
    json.push('\n');
    Ok(json)
}

pub fn decode_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    bytes: &[u8],
) -> Result<HollowDeckState, HollowDeckStateError> {
    let envelope: HollowDeckStateEnvelope = serde_json::from_slice(bytes)?;
    if envelope.format != HOLLOW_DECK_STATE_FORMAT {
        return Err(HollowDeckStateError::UnsupportedFormat(envelope.format));
    }
    if envelope.version != HOLLOW_DECK_STATE_VERSION {
        return Err(HollowDeckStateError::UnsupportedVersion(envelope.version));
    }
    let actual = component_digest(&serde_json::to_vec(&envelope.state)?);
    if envelope.checksum != actual {
        return Err(HollowDeckStateError::ChecksumMismatch);
    }
    let mut state = envelope.state;
    normalize_legacy_hollow_deck_state(catalog, &mut state);
    validate_normalized_hollow_deck_state(catalog, &state)?;
    Ok(state)
}

pub fn normalize_legacy_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    state: &mut HollowDeckState,
) -> HollowDeckStateMigration {
    let mut migration = HollowDeckStateMigration::default();
    let legacy_wing_stage = state
        .current_forms
        .get("gargoyle")
        .and_then(|progress| progress.feature_stages.get("wings"))
        .and_then(|stage| {
            if token_matches(stage, "buds") {
                Some("wing-buds")
            } else if token_matches(stage, "winglets") {
                Some("winglets")
            } else if token_matches(stage, "wings") {
                Some("wings")
            } else {
                None
            }
        });
    if let Some(stage) = legacy_wing_stage {
        state
            .current_forms
            .get_mut("gargoyle")
            .expect("legacy Gargoyle progress was just observed")
            .feature_stages
            .remove("wings");
        advance_growth_stage(catalog, state, "gargoyle", stage);
        migration.migrated_gargoyle_wing_stage = true;
    }
    if state.known_syntheses.remove("white-opal-statue") {
        advance_growth_stage(catalog, state, "gargoyle", "white-opal-statue-mastery");
        migration.normalized_white_opal_statue = true;
    }
    migration
}

pub fn load_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    path: &Path,
) -> Result<HollowDeckState, HollowDeckStateError> {
    match fs::read(path) {
        Ok(bytes) => decode_hollow_deck_state(catalog, &bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(HollowDeckState::default())
        }
        Err(error) => Err(error.into()),
    }
}

pub fn save_hollow_deck_state(
    catalog: &HollowDeckCatalog,
    state: &HollowDeckState,
    path: &Path,
) -> Result<(), HollowDeckStateError> {
    let encoded = encode_hollow_deck_state(catalog, state)?;
    write_text_artifact(path, &encoded)?;
    Ok(())
}

fn resolve_nature<'a>(
    catalog: &'a HollowDeckCatalog,
    nature: &str,
) -> Result<&'a crate::hollow_deck::Nature, HollowDeckStateError> {
    catalog
        .nature(nature)
        .ok_or_else(|| HollowDeckStateError::InvalidAction(format!("unknown Nature: {nature}")))
}

fn resolve_aura_move<'a>(
    catalog: &'a HollowDeckCatalog,
    nature_id: &str,
    aura_move: &str,
) -> Result<&'a crate::hollow_deck::AuraMove, HollowDeckStateError> {
    let aura_move = catalog
        .aura_moves
        .iter()
        .find(|item| token_matches(&item.id, aura_move) || token_matches(&item.name, aura_move))
        .ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!("unknown Aura move: {aura_move}"))
        })?;
    if !aura_move.compatible_natures.is_empty()
        && !aura_move
            .compatible_natures
            .iter()
            .any(|id| id == nature_id)
    {
        return Err(HollowDeckStateError::InvalidAction(format!(
            "Aura move {} is not compatible with {nature_id}",
            aura_move.name
        )));
    }
    Ok(aura_move)
}

fn resolve_glaushouse_seat(
    catalog: &HollowDeckCatalog,
    seat: &str,
) -> Result<GlaushouseSeat, HollowDeckStateError> {
    catalog
        .glaushouse_structure
        .seats
        .iter()
        .copied()
        .find(|candidate| token_matches(candidate.as_str(), seat))
        .ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "unknown Glaüshouse seat: {seat}; choose {}",
                catalog
                    .glaushouse_structure
                    .seats
                    .iter()
                    .map(|candidate| candidate.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

fn resolve_glaushouse_branch(branch: &str) -> Result<GlaushouseBranch, HollowDeckStateError> {
    GlaushouseBranch::ALL
        .into_iter()
        .find(|candidate| token_matches(candidate.as_str(), branch))
        .ok_or_else(|| {
            HollowDeckStateError::InvalidAction(format!(
                "unknown Glaüshouse branch: {branch}; choose Presensor or Preceptor"
            ))
        })
}

fn advance_growth_stage(
    catalog: &HollowDeckCatalog,
    state: &mut HollowDeckState,
    nature_id: &str,
    stage: &str,
) {
    let Some(path) = get_growth_path_for_nature(catalog, nature_id) else {
        return;
    };
    let Some(next_index) = get_growth_stage_index(path, stage) else {
        return;
    };
    let progress = state.current_forms.entry(nature_id.into()).or_default();
    let current_index = progress
        .growth_stage
        .as_deref()
        .and_then(|current| get_growth_stage_index(path, current));
    if current_index.is_none_or(|current| current < next_index) {
        progress.growth_stage = Some(path.milestones[next_index].id.clone());
    }
}

fn resolve_value(values: &[String], candidate: &str) -> Option<String> {
    values
        .iter()
        .find(|value| token_matches(value, candidate))
        .cloned()
}

fn token_matches(left: &str, right: &str) -> bool {
    normalize_token(left) == normalize_token(right)
}

fn normalize_token(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn validate_stages<'a>(
    nature_id: &str,
    kind: &str,
    selected: &BTreeMap<String, String>,
    canonical: impl Iterator<Item = (&'a String, &'a Vec<String>)>,
    errors: &mut Vec<String>,
) {
    let canonical = canonical.collect::<BTreeMap<_, _>>();
    for (id, stage) in selected {
        match canonical.get(id) {
            Some(stages) if stages.contains(stage) => {}
            Some(_) => errors.push(format!("invalid {nature_id} {kind} stage: {id}={stage}")),
            None => errors.push(format!("unknown {nature_id} {kind}: {id}")),
        }
    }
}

fn validate_ids(
    kind: &str,
    selected: &BTreeSet<String>,
    canonical: &BTreeSet<&str>,
    errors: &mut Vec<String>,
) {
    for id in selected {
        if !canonical.contains(id.as_str()) {
            errors.push(format!("unknown {kind}: {id}"));
        }
    }
}
