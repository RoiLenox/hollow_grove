//! Terminal presentation for the canonical Hollow Deck catalog.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::hollow_deck::*;
use crate::hollow_deck_state::HollowDeckState;

const WIDTH: usize = 96;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NatureSort {
    #[default]
    Name,
    House,
    Dominion,
    Current,
    Aura,
    Trait,
    Total,
    Tier,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NatureQuery {
    pub search: Option<String>,
    pub house: Option<String>,
    pub dominion: Option<String>,
    pub current: Option<String>,
    pub aura: Option<String>,
    pub trait_name: Option<String>,
    pub total: Option<u16>,
    pub tier: Option<usize>,
    pub path: Option<String>,
    pub paired: Option<String>,
    pub principle: Option<String>,
    pub sort: NatureSort,
}

#[must_use]
pub fn query_natures<'a>(catalog: &'a HollowDeckCatalog, query: &NatureQuery) -> Vec<&'a Nature> {
    let mut matches = catalog
        .natures
        .iter()
        .filter(|nature| nature_matches(catalog, nature, query))
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| compare_natures(catalog, left, right, query.sort));
    matches
}

fn nature_matches(catalog: &HollowDeckCatalog, nature: &Nature, query: &NatureQuery) -> bool {
    let trait_name = catalog
        .primary_trait(&nature.primary_trait_id)
        .map(|item| item.name.as_str())
        .unwrap_or_default();
    let contextual_identities =
        catalog
            .flynt_visual_identity(&nature.id)
            .map_or_else(String::new, |profile| {
                format!(
                    "{} {} {} {} {} {}",
                    profile.mystery_men_identity.name,
                    profile.we_fairy_men_identity.name,
                    profile.gallows_identity.name,
                    profile.symbol_anchor,
                    profile.mythic_source,
                    profile.secondary_echoes.join(" ")
                )
            });
    let searchable = format!(
        "{} {} {} {} {} {} {} {} {}",
        nature.name,
        house_display_name(nature.house_id),
        nature.dominion_id.as_str(),
        nature.current_type.as_str(),
        nature.aura_material.as_str(),
        trait_name,
        nature.shared_pair_principle.as_deref().unwrap_or_default(),
        nature.paired_nature_id.as_deref().unwrap_or_default(),
        contextual_identities,
    );
    contains_if(&searchable, query.search.as_deref())
        && contains_if(house_display_name(nature.house_id), query.house.as_deref())
        && contains_if(nature.dominion_id.as_str(), query.dominion.as_deref())
        && contains_if(nature.current_type.as_str(), query.current.as_deref())
        && contains_if(nature.aura_material.as_str(), query.aura.as_deref())
        && contains_if(trait_name, query.trait_name.as_deref())
        && query
            .total
            .is_none_or(|total| nature.mature_stats.computed_total() == total)
        && query
            .tier
            .is_none_or(|tier| nature.evolution_index.map(|index| index + 1) == Some(tier))
        && contains_if(
            nature.evolution_path_id.as_deref().unwrap_or_default(),
            query.path.as_deref(),
        )
        && query.paired.as_deref().is_none_or(|paired| {
            nature.paired_nature_id.as_deref().is_some_and(|candidate| {
                paired.is_empty() || contains_case_insensitive(candidate, paired)
            })
        })
        && contains_if(
            nature.shared_pair_principle.as_deref().unwrap_or_default(),
            query.principle.as_deref(),
        )
}

fn contains_if(haystack: &str, needle: Option<&str>) -> bool {
    needle.is_none_or(|needle| contains_case_insensitive(haystack, needle))
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn compare_natures(
    catalog: &HollowDeckCatalog,
    left: &Nature,
    right: &Nature,
    sort: NatureSort,
) -> Ordering {
    let ordering = match sort {
        NatureSort::Name => left.name.cmp(&right.name),
        NatureSort::House => {
            house_display_name(left.house_id).cmp(house_display_name(right.house_id))
        }
        NatureSort::Dominion => left.dominion_id.as_str().cmp(right.dominion_id.as_str()),
        NatureSort::Current => left.current_type.as_str().cmp(right.current_type.as_str()),
        NatureSort::Aura => left
            .aura_material
            .as_str()
            .cmp(right.aura_material.as_str()),
        NatureSort::Trait => get_primary_trait(catalog, &left.id)
            .map(|item| item.name.as_str())
            .cmp(&get_primary_trait(catalog, &right.id).map(|item| item.name.as_str())),
        NatureSort::Total => left
            .mature_stats
            .computed_total()
            .cmp(&right.mature_stats.computed_total()),
        NatureSort::Tier => left.evolution_index.cmp(&right.evolution_index),
    };
    ordering.then_with(|| left.name.cmp(&right.name))
}

#[must_use]
pub fn render_home(catalog: &HollowDeckCatalog) -> String {
    let mut lines = vec![
        "H U E M A N   →   N A T U R E   →   C U R R E N T  F O R M  +  A U R A  S E T".into(),
        format!(
            "Hueman baseline  STR {:>3}   SPD {:>3}   ATK {:>3}   DEF {:>3}   TOTAL {}",
            catalog.hueman_stats.strength,
            catalog.hueman_stats.speed,
            catalog.hueman_stats.attack,
            catalog.hueman_stats.defense,
            catalog.hueman_stats.computed_total()
        ),
        "".into(),
    ];
    for house in &catalog.houses {
        lines.push(format!(
            "{:<12} {:<17} {:<17} {:<9} {}",
            house.display_name.to_uppercase(),
            house
                .dominions
                .iter()
                .map(|dominion| dominion.as_str())
                .collect::<Vec<_>>()
                .join(" / "),
            house.current_type.as_str(),
            house_aura_names(house),
            house.organization
        ));
    }
    lines.extend([
        "".into(),
        "Current Form = what the BODY can do.  Aura Set = what the MIND can do.".into(),
        "They cultivate independently. Evolution Path is a separate Nature-to-Nature relationship."
            .into(),
        "".into(),
        "Houses • Institutions • Natures • Progression • Current Forms • Aura Sets • Traits • Compare"
            .into(),
        "Alchemy • Recipes • Synthesis Recipes • Syntheses • Banes".into(),
    ]);
    boxed("HOLLOW DECK  /  HOME", &lines)
}

#[must_use]
pub fn render_houses(catalog: &HollowDeckCatalog) -> String {
    let mut lines = Vec::new();
    for house in &catalog.houses {
        lines.extend([
            house.display_name.to_uppercase(),
            format!(
                "  Dominion     {}",
                house
                    .dominions
                    .iter()
                    .map(|item| item.as_str())
                    .collect::<Vec<_>>()
                    .join(" / ")
            ),
            format!("  Current      {}", house.current_type.as_str()),
            format!("  Aura         {}", house_aura_names(house)),
            format!(
                "  Direction    {}",
                house
                    .direction
                    .as_deref()
                    .unwrap_or("not specified in the Deck")
            ),
            format!("  Structure    {}", house.organization),
            "".into(),
        ]);
    }
    lines.extend([
        "CURRENT AXES".into(),
        "  Hollow Current   Stonebend + Glaüshouse".into(),
        "  Regular Current  Sandmanor + Flynt".into(),
        "  Current type alone creates no matchup advantage.".into(),
    ]);
    boxed("HOLLOW DECK  /  HOUSES", &lines)
}

#[must_use]
pub fn render_nature_index(catalog: &HollowDeckCatalog, query: &NatureQuery) -> String {
    let matches = query_natures(catalog, query);
    let mut lines = vec![
        format!(
            "{:<13} {:<12} {:<16} {:>3} {:>3} {:>3} {:>3} {:>5}",
            "NATURE", "HOUSE", "PRIMARY TRAIT", "STR", "SPD", "ATK", "DEF", "TOTAL"
        ),
        "─".repeat(WIDTH - 4),
    ];
    for nature in &matches {
        let trait_name = get_primary_trait(catalog, &nature.id)
            .map(|item| item.name.as_str())
            .unwrap_or("TODO");
        lines.push(format!(
            "{:<13} {:<12} {:<16} {:>3} {:>3} {:>3} {:>3} {:>5}",
            nature.name,
            house_display_name(nature.house_id),
            trait_name,
            nature.mature_stats.strength,
            nature.mature_stats.speed,
            nature.mature_stats.attack,
            nature.mature_stats.defense,
            nature.mature_stats.computed_total(),
        ));
    }
    lines.push(String::new());
    lines.push(format!(
        "{} of {} Natures  •  sort: {:?}",
        matches.len(),
        catalog.natures.len(),
        query.sort
    ));
    boxed("HOLLOW DECK  /  NATURE INDEX", &lines)
}

pub fn render_nature_card(catalog: &HollowDeckCatalog, id_or_name: &str) -> Result<String, String> {
    let nature = catalog
        .nature(id_or_name)
        .ok_or_else(|| format!("unknown Nature: {id_or_name}"))?;
    let trait_entry = get_primary_trait(catalog, &nature.id)
        .ok_or_else(|| format!("{} has no Primary Trait", nature.name))?;
    let mut sections = vec![vec![
        format!("NATURE       {}", nature.name.to_uppercase()),
        format!("House        {}", house_display_name(nature.house_id)),
        format!("Dominion     {}", nature.dominion_id.as_str()),
        format!("Current      {}", nature.current_type.as_str()),
        format!("Aura         {}", nature.aura_material.as_str()),
        format!("PRIMARY TRAIT  {}", trait_entry.name.to_uppercase()),
        format!("  {}", trait_entry.mechanical_effect),
        String::new(),
        stat_line(nature.mature_stats),
    ]];
    if let Some(branch) = get_glaushouse_branch_for_nature(catalog, &nature.id) {
        sections[0].insert(
            5,
            format!(
                "Precog Branch  {} / {} / {}",
                branch.as_str(),
                branch.role_family(),
                branch.aura_material().as_str()
            ),
        );
    }
    if let Some(profile) = catalog.flynt_visual_identity(&nature.id) {
        let institution = resolve_flynt_principal_institutional_stack(catalog, &nature.id)
            .expect("validated principal Flynt institutional stack");
        sections.push(vec![
            "CONTEXTUAL IDENTITIES  /  SAME PERSON AND NATURE".into(),
            format!("  Nature        {}", nature.name),
            format!("  Citizenship   {}", institution.citizenship.as_str()),
            format!("  Service       {}", institution.service_history.as_str()),
            format!("  Elite order   {}", institution.elite_order.as_str()),
            format!("  Sect          {}", institution.sect.as_str()),
            format!("  Mystery Men   {}", profile.mystery_men_identity.name),
            format!("  We Fairy Men  {}", profile.we_fairy_men_identity.name),
            format!("  The Gallows   {}", profile.gallows_identity.name),
            format!("  Symbol        {}", profile.symbol_anchor),
            format!("  Mythic source {}", profile.mythic_source),
            format!(
                "  Echoes        {}",
                list_or_none(&profile.secondary_echoes, "none")
            ),
            "  Organization changes social presentation, not biological Nature or growth.".into(),
        ]);
    }

    let current_form = catalog.current_form(&nature.id);
    let mut current_lines = vec!["CURRENT FORM  /  BODY".into()];
    match current_form {
        Some(form)
            if form.growth_path_id.is_some()
                || !form.current_features.is_empty()
                || !form.current_techniques.is_empty() =>
        {
            if let Some(path) = get_growth_path_for_nature(catalog, &nature.id) {
                current_lines.push(format!("  UNIFIED GROWTH  {}", path.name));
                current_lines.extend(growth_checklist(path, form.current_growth_stage.as_deref()));
                current_lines.push(
                    "  White Opal Statue is inherent Fortification growth, not Synthesis.".into(),
                );
            }
            for feature in &form.current_features {
                let stages = if feature.stages.is_empty() {
                    "stage not specified".into()
                } else {
                    feature.stages.join(" → ")
                };
                current_lines.push(format!("  FEATURE    {:<22} {stages}", feature.name));
            }
            for technique in &form.current_techniques {
                current_lines.push(format!(
                    "  TECHNIQUE  {:<22} {}",
                    technique.name,
                    technique.stages.join(" → ")
                ));
            }
        }
        _ => current_lines.push("  Canonical physical features not yet specified (TODO).".into()),
    }
    sections.push(current_lines);

    let mut aura_lines = vec!["AURA SET  /  MIND".into()];
    match catalog.aura_set(&nature.id) {
        Some(set) => {
            aura_lines.push(format!(
                "  Learned:   {}",
                list_or_none(&set.learned_moves, "none recorded")
            ));
            aura_lines.push(format!(
                "  Equipped:  {}",
                list_or_none(&set.equipped_moves, "none recorded")
            ));
            if !set.design_examples.is_empty() {
                aura_lines.push(format!(
                    "  Design examples (not locked canon): {}",
                    set.design_examples.join(", ")
                ));
            }
        }
        None => aura_lines.push("  No Aura Set record (TODO).".into()),
    }
    sections.push(aura_lines);

    let mut progression = vec!["EVOLUTION / PROGRESSION".into()];
    if let Some(path) = get_evolution_path(catalog, &nature.id) {
        progression.push(format!(
            "  {}",
            names_for_ids(catalog, &path.nature_ids).join(" → ")
        ));
        progression.push(format!(
            "  Tier {} of {}",
            get_evolution_tier(catalog, &nature.id).unwrap_or(0),
            path.nature_ids.len()
        ));
    } else {
        progression
            .push("  No linear evolution path. Flynt is organized by reciprocal pairs.".into());
    }
    sections.push(progression);

    let mut relation = vec!["PAIR / COUNTERPART".into()];
    if let Some(pair) = get_pair_relationship(catalog, &nature.id) {
        relation.push(format!(
            "  {}  ↔  {}  •  shared law: {}",
            name_for_id(catalog, &pair.nature_a),
            name_for_id(catalog, &pair.nature_b),
            pair.shared_principle
        ));
        for interaction in &pair.special_interactions {
            relation.push(format!(
                "  {}: {}",
                interaction.name, interaction.mechanical_effect
            ));
        }
    }
    if let Some(counterpart) = get_cross_house_counterpart(catalog, &nature.id) {
        relation.push(format!(
            "  Cross-House counterpart: {} / {} / total {}",
            counterpart.name,
            house_display_name(counterpart.house_id),
            counterpart.mature_stats.computed_total()
        ));
    }
    if relation.len() == 1 {
        relation.push("  No pair or cross-House counterpart.".into());
    }
    sections.push(relation);

    let compatible_syntheses = catalog
        .syntheses
        .iter()
        .filter(|item| item.compatible_natures.iter().any(|id| id == &nature.id))
        .collect::<Vec<_>>();
    sections.push({
        let mut lines = vec!["SYNTHESIS".into()];
        if compatible_syntheses.is_empty() {
            lines.push("  No named Synthesis indexed (TODO).".into());
        } else {
            for synthesis in compatible_syntheses {
                lines.push(format!(
                    "  {}  [{}]  {}",
                    synthesis.name,
                    synthesis.result_type.as_str(),
                    synthesis.description
                ));
            }
        }
        lines
    });

    let compatible_recipes = get_compatible_synthesis_recipes(catalog, &nature.id);
    sections.push({
        let mut lines = vec!["SYNTHESIS RECIPES".into()];
        if compatible_recipes.is_empty() {
            lines.push("  No compatible collectible pattern indexed (TODO).".into());
        } else {
            for recipe in compatible_recipes {
                lines.push(format!(
                    "  {}  •  {}  •  discovered: {}",
                    recipe.name,
                    recipe.result_type.as_str(),
                    yes_no(recipe.is_discovered)
                ));
            }
        }
        lines
    });

    let banes = catalog
        .banes
        .iter()
        .filter(|bane| bane.target_natures.iter().any(|id| id == &nature.id))
        .collect::<Vec<_>>();
    sections.push({
        let mut lines = vec!["BANES".into()];
        if banes.is_empty() {
            lines.push("  No canonical Bane assigned (TODO).".into());
        } else {
            for bane in banes {
                lines.push(format!("  {} — {}", bane.name, bane.mechanical_effect));
            }
        }
        lines
    });

    let (strong, weak, neutral) = material_relations(nature.aura_material);
    sections.push(vec![
        "ALCHEMY MATCHUP  /  AURA + APPLICABLE SYNTHESIS".into(),
        format!(
            "  Strong vs {strong}  •  Weak vs {weak}  •  Neutral vs {neutral} and same material"
        ),
        "  Pure Current / physical actions do not automatically receive this modifier.".into(),
    ]);

    sections.push(vec![
        "VISUAL / LORE NOTES".into(),
        format!(
            "  {}",
            nature
                .visual_description
                .as_deref()
                .unwrap_or("Visual description not yet specified (TODO).")
        ),
        format!(
            "  {}",
            nature
                .notes
                .as_deref()
                .unwrap_or("Additional notes not yet specified (TODO).")
        ),
    ]);

    Ok(sectioned_box("HOLLOW DECK  /  NATURE CARD", &sections))
}

pub fn render_nature_card_with_state(
    catalog: &HollowDeckCatalog,
    state: &HollowDeckState,
    id_or_name: &str,
) -> Result<String, String> {
    let nature = catalog
        .nature(id_or_name)
        .ok_or_else(|| format!("unknown Nature: {id_or_name}"))?;
    let mut output = render_nature_card(catalog, &nature.id)?;
    let current_form = state
        .current_form_for(catalog, &nature.id)
        .map_err(|error| error.to_string())?;
    let aura_set = state
        .aura_set_for(catalog, &nature.id)
        .map_err(|error| error.to_string())?;
    let mut lines = Vec::new();
    if let Some(form) = current_form {
        lines.push(format!(
            "Current cultivation  {:>3}.{:02}%  {}",
            form.development_basis_points / 100,
            form.development_basis_points % 100,
            stat_line(form.computed_stats)
        ));
        if let Some(path) = get_growth_path_for_nature(catalog, &nature.id) {
            lines.extend(growth_checklist(path, form.current_growth_stage.as_deref()));
        }
        for feature in form.current_features {
            if let Some(stage) = feature.current_stage {
                lines.push(format!("  FEATURE    {:<22} {stage}", feature.name));
            }
        }
        for technique in form.current_techniques {
            if let Some(stage) = technique.current_stage {
                lines.push(format!("  TECHNIQUE  {:<22} {stage}", technique.name));
            }
        }
    } else {
        lines.push("Current cultivation  not begun (Hueman baseline remains active)".into());
    }
    if let Some(set) = aura_set {
        lines.push(format!(
            "Aura learned         {}",
            move_names(catalog, &set.learned_moves)
        ));
        lines.push(format!(
            "Aura equipped        {}",
            move_names(catalog, &set.equipped_moves)
        ));
    } else {
        lines.push("Aura learned         none".into());
        lines.push("Aura equipped        none".into());
    }
    let known_syntheses = catalog
        .syntheses
        .iter()
        .filter(|item| state.known_syntheses.contains(&item.id))
        .filter(|item| {
            item.compatible_natures.is_empty() || item.compatible_natures.contains(&nature.id)
        })
        .map(|item| item.name.as_str())
        .collect::<Vec<_>>();
    lines.push(format!(
        "Known Syntheses       {}",
        if known_syntheses.is_empty() {
            "none".into()
        } else {
            known_syntheses.join(", ")
        }
    ));
    output.push_str("\n\n");
    output.push_str(&boxed("HOLLOW DECK  /  ACTIVE CULTIVATION", &lines));
    Ok(output)
}

#[must_use]
pub fn render_cultivation_state(catalog: &HollowDeckCatalog, state: &HollowDeckState) -> String {
    let mut sections = Vec::new();
    let mut current = vec!["CURRENT FORMS  /  BODY".into()];
    if state.current_forms.is_empty() {
        current.push("  No cultivated Current Forms recorded.".into());
    }
    for nature in &catalog.natures {
        let Some(progress) = state.current_forms.get(&nature.id) else {
            continue;
        };
        let stats = state
            .current_form_for(catalog, &nature.id)
            .ok()
            .flatten()
            .map(|form| stat_line(form.computed_stats))
            .unwrap_or_else(|| "invalid active stats".into());
        current.push(format!(
            "  {:<13} {:>3}.{:02}%  {stats}",
            nature.name,
            progress.development_basis_points / 100,
            progress.development_basis_points % 100
        ));
        for (feature, stage) in &progress.feature_stages {
            current.push(format!("    FEATURE    {feature:<20} {stage}"));
        }
        for (technique, stage) in &progress.technique_stages {
            current.push(format!("    TECHNIQUE  {technique:<20} {stage}"));
        }
        if let Some(stage) = &progress.growth_stage {
            current.push(format!("    UNIFIED GROWTH  {stage}"));
        }
    }
    sections.push(current);

    let mut aura = vec!["AURA SETS  /  MIND".into()];
    if state.aura_sets.is_empty() {
        aura.push("  No learned Aura moves recorded.".into());
    }
    for nature in &catalog.natures {
        let Some(set) = state.aura_sets.get(&nature.id) else {
            continue;
        };
        aura.push(format!(
            "  {:<13} learned: {}",
            nature.name,
            move_names(
                catalog,
                &set.learned_moves.iter().cloned().collect::<Vec<_>>()
            )
        ));
        aura.push(format!(
            "  {:<13} equipped: {}",
            "",
            move_names(
                catalog,
                &set.equipped_moves.iter().cloned().collect::<Vec<_>>()
            )
        ));
    }
    sections.push(aura);

    let affiliation = match &state.glaushouse_affiliation {
        Some(affiliation) => {
            let mut lines = vec![
                "GLAÜSHOUSE AFFILIATION  /  DERIVED, NON-DUPLICATED".into(),
                format!("  Seat         {}", affiliation.seat.as_str()),
                format!("  Title        {}", affiliation.title()),
            ];
            if let Some(branch) = affiliation.branch {
                lines.extend([
                    format!("  Branch       {}", branch.as_str()),
                    format!("  Role family  {}", branch.role_family()),
                    format!("  Aura         {}", branch.aura_material().as_str()),
                ]);
            } else {
                lines.push("  Branch       not yet selected; entry remains Nightingale".into());
            }
            lines
        }
        None => vec![
            "GLAÜSHOUSE AFFILIATION".into(),
            "  No seat affiliation recorded.".into(),
        ],
    };
    sections.push(affiliation);

    sections.push(vec![
        "DISCOVERY / KNOWLEDGE".into(),
        format!(
            "  Ordinary Recipes       {}",
            known_names(
                &state.known_recipes,
                catalog.recipes.iter().map(|item| (&item.id, &item.name))
            )
        ),
        format!(
            "  Synthesis Recipes      {}",
            known_names(
                &state.discovered_synthesis_recipes,
                catalog
                    .synthesis_recipes
                    .iter()
                    .map(|item| (&item.id, &item.name))
            )
        ),
        format!(
            "  Syntheses              {}",
            known_names(
                &state.known_syntheses,
                catalog.syntheses.iter().map(|item| (&item.id, &item.name))
            )
        ),
    ]);
    sectioned_box("HOLLOW DECK  /  CULTIVATION STATE", &sections)
}

#[must_use]
pub fn render_progression(catalog: &HollowDeckCatalog) -> String {
    let mut sections = Vec::new();
    for path in catalog
        .evolution_paths
        .iter()
        .filter(|path| path.house_id != crate::House::Glaushouse)
    {
        sections.push(vec![
            format!(
                "{} / {}",
                path.name.to_uppercase(),
                path.dominion_id.as_str()
            ),
            names_for_ids(catalog, &path.nature_ids).join(" → "),
            path.nature_ids
                .iter()
                .map(|id| {
                    let nature = catalog.nature(id).expect("canonical path member");
                    get_primary_trait(catalog, id)
                        .map(|item| item.name.clone())
                        .unwrap_or_else(|| nature.primary_trait_id.clone())
                })
                .collect::<Vec<_>>()
                .join(" → "),
        ]);
    }
    for branch_path in &catalog.glaushouse_structure.branch_paths {
        let branch = branch_path.branch;
        let path = catalog
            .evolution_path(&branch_path.evolution_path_id)
            .expect("canonical Glaüshouse branch path");
        sections.push(vec![
            format!(
                "GLAÜSHOUSE / {} / {} / {}",
                branch.as_str().to_uppercase(),
                branch.role_family().to_uppercase(),
                branch.aura_material().as_str().to_uppercase()
            ),
            names_for_ids(catalog, &path.nature_ids).join(" ↓ "),
            path.nature_ids
                .iter()
                .map(|id| {
                    get_primary_trait(catalog, id)
                        .expect("canonical branch trait")
                        .name
                        .clone()
                })
                .collect::<Vec<_>>()
                .join(" ↓ "),
        ]);
    }
    sections.push(vec![
        "FLYNT / RESYNCE / RECIPROCAL PAIRS (NOT AN EVOLUTION LADDER)".into(),
        "STILLNESS    Gargoyle ↔ Medusa       Fortification ↔ Petrification".into(),
        "CIRCULATION  Werewolf ↔ Vampire      Siphone ↔ Transfusion".into(),
        "POSITION     Mariner ↔ Mermaid       Anchoring ↔ Attraction".into(),
        format!(
            "FORM         {} ↔ {}     Convergence ↔ Divergence",
            name_for_id(catalog, "manticorp"),
            name_for_id(catalog, "chimera")
        ),
    ]);
    sectioned_box("HOLLOW DECK  /  PROGRESSION BOARD", &sections)
}

#[must_use]
pub fn render_glaushouse(catalog: &HollowDeckCatalog) -> String {
    let structure = &catalog.glaushouse_structure;
    let mut sections = vec![vec![
        "HOUSE OF THE MUSES".into(),
        format!("  Direction   {}", structure.direction),
        format!("  Faculty     {}", structure.umbrella_faculty.as_str()),
        "  Current     Hollow Current".into(),
        "  Aura        Jade / Emerald (coequal; one alchemical slot)".into(),
        "  Entry       Nightingale de <Seat>".into(),
    ]];
    for seat in &structure.seats {
        sections.push(vec![
            seat.as_str().to_uppercase(),
            format!("  Entry: {}", seat.entry_title()),
            format!(
                "  ├── {} = {} / {}",
                GlaushouseBranch::Presensor.achieved_title(*seat),
                GlaushouseBranch::Presensor.as_str(),
                GlaushouseBranch::Presensor.aura_material().as_str()
            ),
            format!(
                "  └── {} = {} / {}",
                GlaushouseBranch::Preceptor.achieved_title(*seat),
                GlaushouseBranch::Preceptor.as_str(),
                GlaushouseBranch::Preceptor.aura_material().as_str()
            ),
        ]);
    }
    sections.push(vec![
        "BRANCH RELATIONSHIP".into(),
        "  Matron is the seat-specific Presensor title; it is not a later stage.".into(),
        "  Marshall is the seat-specific Preceptor title; it is not a later stage.".into(),
        "  Seat, branch, role title, Aura material, Nature, and Current Form remain separate."
            .into(),
    ]);
    sectioned_box("HOLLOW DECK  /  GLAÜSHOUSE", &sections)
}

pub fn render_institutions(
    catalog: &HollowDeckCatalog,
    house: Option<&str>,
) -> Result<String, String> {
    let mut sections = vec![vec![
        "INSTITUTIONAL AXES  /  NEVER ONE GENERIC RANK".into(),
        format!("  Biology       {}", catalog.institutional_axes.biology),
        format!(
            "  Training      {}",
            catalog.institutional_axes.institutional_service
        ),
        format!(
            "  Status        {}",
            catalog.institutional_axes.institutional_mastery
        ),
        format!(
            "  Discipline    {}",
            catalog.institutional_axes.specialist_discipline
        ),
        format!(
            "  Organization  {}",
            catalog.institutional_axes.operating_organization
        ),
        format!(
            "  Alias         {}",
            catalog.institutional_axes.social_alias
        ),
        format!(
            "  Rule          {}",
            catalog.institutional_axes.renderer_invariant
        ),
    ]];
    match house.map(normalize_house_selector).as_deref() {
        None => {
            push_glaushouse_institution_sections(catalog, &mut sections);
            push_flynt_institution_sections(catalog, &mut sections);
            push_stonebend_institution_sections(catalog, &mut sections);
            push_sandmanor_institution_sections(catalog, &mut sections);
        }
        Some("glaushouse") => push_glaushouse_institution_sections(catalog, &mut sections),
        Some("flynt") => push_flynt_institution_sections(catalog, &mut sections),
        Some("stonebend") => push_stonebend_institution_sections(catalog, &mut sections),
        Some("sandmanor") => push_sandmanor_institution_sections(catalog, &mut sections),
        Some(other) => {
            return Err(format!(
                "no Hollow Deck institutional board for: {other}; expected glaushouse, flynt, stonebend, or sandmanor"
            ));
        }
    }
    Ok(sectioned_box(
        "HOLLOW DECK  /  INSTITUTIONAL GRAMMARS",
        &sections,
    ))
}

fn normalize_house_selector(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace('ü', "u")
        .replace([' ', '_'], "-")
}

fn push_glaushouse_institution_sections(
    catalog: &HollowDeckCatalog,
    sections: &mut Vec<Vec<String>>,
) {
    let structure = &catalog.glaushouse_structure;
    sections.push(vec![
        "GLAÜSHOUSE  /  SEAT THEN COEQUAL BRANCH".into(),
        format!("  {}", structure.entry_title),
        "                 │".into(),
        "             ┌───┴───┐".into(),
        "             │       │".into(),
        "          Matron   Marshall".into(),
        "         Presensor Preceptor".into(),
        "           Jade     Emerald".into(),
        "  Nature paths are branch-specific; Matron and Marshall are parallel titles.".into(),
    ]);
}

fn push_flynt_institution_sections(catalog: &HollowDeckCatalog, sections: &mut Vec<Vec<String>>) {
    let hierarchy = &catalog.flynt_institution;
    sections.push(vec![
        "FLYNT  /  FORMAL SERVICE".into(),
        format!(
            "  {}  citizen baseline",
            hierarchy.citizen_baseline.as_str()
        ),
        "    ↓".into(),
        format!(
            "  {}  general corps/service",
            hierarchy.service_status.as_str()
        ),
        format!("    ↓  {}", hierarchy.capacity_requirement),
        format!("  {}  earned elite status", hierarchy.elite_status.as_str()),
        "    ↓  select one parallel specialist sect".into(),
        "  Hunter  |  Chief  |  Mariner".into(),
        "       ╲       │       ╱".into(),
        "             Chimera".into(),
        "                ↓".into(),
        "             Manticorp".into(),
        "  Every Mystery Man was a Corpman; not every Corpman becomes a Mystery Man.".into(),
        "  Corpman is general service. Manticorp is the perfected apex.".into(),
    ]);
    for tradition in &hierarchy.sects {
        let nature = catalog
            .nature(&tradition.nature_id)
            .expect("validated Flynt sect Nature");
        let primary_trait = catalog
            .primary_trait(&tradition.primary_trait_id)
            .expect("validated Flynt sect Primary Trait");
        sections.push(vec![
            format!("{}  /  PARALLEL MYSTERY MEN SECT", tradition.sect.as_str()),
            format!("  Nature       {}", nature.name),
            format!("  Trait        {}", primary_trait.name),
            format!("  Vocation     {}", tradition.vocation.join(" • ")),
            format!("  Visual       {}", tradition.visual_direction.join(" • ")),
        ]);
    }
    sections.push(vec![
        "FLYNT  /  ELITE CONVERGENCE".into(),
        format!(
            "  {} + {} + {} → {}",
            hierarchy.convergence.required_sects[0].as_str(),
            hierarchy.convergence.required_sects[1].as_str(),
            hierarchy.convergence.required_sects[2].as_str(),
            hierarchy.convergence.convergence_identity
        ),
        format!("  {}", hierarchy.convergence.convergence_meaning),
        format!(
            "  {} → {}",
            hierarchy.convergence.convergence_identity, hierarchy.convergence.perfected_identity
        ),
        format!("  {}", hierarchy.convergence.perfection_meaning),
        format!(
            "  Apex bearer association  {}",
            hierarchy.convergence.apex_association
        ),
        format!(
            "  Compatibility  {}",
            hierarchy.convergence.legacy_pair_compatibility
        ),
    ]);
    let mut sorcery = vec![
        "FLYNT  /  SORCERY — SYNTHESIS MANUFACTURING AND FORMULATION".into(),
        "  MALE         FEMALE         FUNCTION".into(),
    ];
    for pair in &hierarchy.sorcery.paired_ladder {
        sorcery.push(format!(
            "  {:<12} ↔ {:<13} {}",
            pair.male_identity.as_str(),
            pair.female_identity.as_str(),
            pair.function.as_str()
        ));
        sorcery.push(format!(
            "    Capabilities  {}",
            pair.capabilities.join(" • ")
        ));
    }
    sorcery.extend([
        format!("  Governance    {}", hierarchy.sorcery.governance),
        format!("  Danger        {}", hierarchy.sorcery.systemic_danger),
        format!(
            "  Boundary      {}",
            hierarchy.sorcery.sanctioned_practitioner_boundary
        ),
        format!("  Axis          {}", hierarchy.sorcery.axis_boundary),
    ]);
    sections.push(sorcery);
    sections.push(
        std::iter::once("FLYNT  /  VICE AND SYNTHESIS ECOSYSTEM".into())
            .chain(
                hierarchy
                    .sorcery
                    .ecosystem_roles
                    .iter()
                    .map(|profile| format!("  {:<14} {}", profile.role.as_str(), profile.function)),
            )
            .collect(),
    );
    let mut contextual = vec![
        "FLYNT  /  CONTEXTUAL IDENTITIES — NOT CAREER STEPS".into(),
        format!("  {:<10} {:<20} {}", "SECT", "WE FAIRY MEN", "THE GALLOWS"),
    ];
    for tradition in &hierarchy.sects {
        let profile = catalog
            .flynt_visual_identity(&tradition.nature_id)
            .expect("validated Flynt contextual profile");
        contextual.push(format!(
            "  {:<10} {:<20} {}",
            tradition.sect.as_str(),
            profile.we_fairy_men_identity.name,
            profile.gallows_identity.name
        ));
    }
    sections.push(contextual);
    for context in &hierarchy.operating_contexts {
        sections.push(vec![
            format!("{}  /  OPERATING CONTEXT", context.organization.as_str()),
            format!("  Social basis  {}", context.social_basis.join(" • ")),
            format!("  Environment   {}", context.environment.join(" • ")),
            format!("  Relationship  {}", context.relationship_to_hierarchy),
        ]);
    }
}

fn push_stonebend_institution_sections(
    catalog: &HollowDeckCatalog,
    sections: &mut Vec<Vec<String>>,
) {
    let hierarchy = &catalog.stonebend_institution;
    sections.push(vec![
        "STONEBEND  /  ENTRY AND PAIRED COMPETENCY".into(),
        format!(
            "  {}  citizen baseline",
            hierarchy.citizen_baseline.as_str()
        ),
        "    ↓".into(),
        format!("  {}  formal training", hierarchy.training_entry.as_str()),
        "    ↓".into(),
        "  Mason Current + Mage Diamond Aura → Freemason / Superstar".into(),
        "  Pugilist Current + Foreman Diamond Aura → Proliteriate / Rockstar".into(),
        "  Mason and Mage are parallel. Pugilist and Foreman are parallel.".into(),
        "  Applicant is retired from active canon.".into(),
    ]);
    for order in &hierarchy.orders {
        let current = catalog
            .stonebend_competency(order.current_competency)
            .expect("validated Stonebend Current competency");
        let aura = catalog
            .stonebend_competency(order.aura_competency)
            .expect("validated Stonebend Aura competency");
        sections.push(vec![
            format!(
                "{} / {}  /  COMPLETED SYNTHESIS MASTERY",
                order.mastery.as_str(),
                order.cultural_identity
            ),
            format!(
                "  {} / {}  {}",
                current.competency.as_str(),
                current.channel.as_str(),
                current.concepts.join(" • ")
            ),
            format!(
                "  {} / {}  {}",
                aura.competency.as_str(),
                aura.channel.as_str(),
                aura.concepts.join(" • ")
            ),
            format!("  Synthesis     {}", order.synthesis_meaning),
            format!("  Visual        {}", order.visual_direction),
        ]);
    }
    sections.push(vec![
        "STONEBEND  /  APEX".into(),
        "  Freemason / Superstar  +  Proliteriate / Rockstar  →  Hypergiant".into(),
        format!("  {}", hierarchy.apex.meaning),
        format!(
            "  Visual        {}",
            hierarchy.apex.visual_direction.join(" • ")
        ),
        format!("  Avoid         {}", hierarchy.apex.avoid.join(" • ")),
        "  Hypergiant is institutional apex/sovereign identity, not a ninth Nature rung.".into(),
    ]);
    sections.push(vec![
        "STONEBEND  /  PRESYNCE AND THE CUT".into(),
        format!("  Faculty       {}", hierarchy.philosophy.faculty.as_str()),
        format!(
            "  Current       {}",
            hierarchy.philosophy.current_type.as_str()
        ),
        format!(
            "  Aura          {}",
            hierarchy.philosophy.aura_material.as_str()
        ),
        format!("  Philosophy    {}", hierarchy.philosophy.philosophy),
        format!("  Process       {}", hierarchy.philosophy.process),
        format!("  Presynce      {}", hierarchy.philosophy.presynce_meaning),
        format!("  The Cut       {}", hierarchy.philosophy.cut_meaning),
        format!(
            "  Boundary      {}",
            hierarchy.philosophy.hollowing_boundary
        ),
        format!("  Compatibility {}", hierarchy.constitutional_compatibility),
    ]);
    sections.push(vec![
        "STONEBEND  /  BIOLOGY REMAINS SEPARATE".into(),
        "  Gremlin → Goblin → Ghoul → Spectre → Troll → Orc → Ogre → Troglodyte".into(),
        "  A Troll may separately train as Mason, Mage, Freemason, Pugilist, Foreman, or Proliteriate."
            .into(),
    ]);
}

fn push_sandmanor_institution_sections(
    catalog: &HollowDeckCatalog,
    sections: &mut Vec<Vec<String>>,
) {
    let hierarchy = &catalog.sandmanor_institution;
    sections.push(vec![
        "SANDMANOR  /  TWO COMPLETE WAYS OF LIFE".into(),
        format!("  Direction     {}", hierarchy.direction),
        format!("  Current       {}", hierarchy.current_type.as_str()),
        format!("  Aura          {}", hierarchy.aura_material.as_str()),
        format!("  Philosophy    {}", hierarchy.physical_philosophy),
        "  Minorian and Minoan are parallel cultures, not sequential ranks.".into(),
    ]);
    for path in &hierarchy.cultures {
        let evolution = catalog
            .evolution_path(&path.nature_path_id)
            .expect("validated Sandmanor Nature path");
        let nature_names = evolution
            .nature_ids
            .iter()
            .map(|id| name_for_id(catalog, id))
            .collect::<Vec<_>>()
            .join(" → ");
        let trait_names = evolution
            .nature_ids
            .iter()
            .map(|id| {
                get_primary_trait(catalog, id)
                    .expect("validated Sandmanor Primary Trait")
                    .name
                    .as_str()
            })
            .collect::<Vec<_>>()
            .join(" → ");
        sections.push(vec![
            format!(
                "{}  /  {}",
                path.culture.as_str().to_uppercase(),
                path.faculty.faculty.as_str().to_uppercase()
            ),
            format!("  Orientation       {}", path.orientation.join(" • ")),
            format!(
                "  Social / Current  {}",
                path.social_path
                    .iter()
                    .map(|stage| stage.identity.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ")
            ),
            format!(
                "  Aura / Faculty    {}",
                path.aura_path
                    .iter()
                    .map(|stage| stage.identity.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ")
            ),
            format!("  Nature            {nature_names}"),
            format!("  Primary Trait     {trait_names}"),
            format!("  Faculty question  “{}”", path.faculty.core_question),
            format!("  Faculty function  {}", path.faculty.function),
            format!(
                "  Boundary          {}",
                path.faculty.excluded_interpretation
            ),
        ]);
    }
    let mut social_pairs = vec!["SANDMANOR  /  SOCIAL PAIRING SYMMETRY".into()];
    social_pairs.extend(hierarchy.social_pairings.iter().map(|pairing| {
        format!(
            "  {} ↔ {}  /  {}",
            pairing.minorian.as_str(),
            pairing.minoan.as_str(),
            pairing.contrast
        )
    }));
    sections.push(social_pairs);

    let mut aura_pairs = vec!["SANDMANOR  /  AURA PAIRING SYMMETRY".into()];
    aura_pairs.extend(hierarchy.aura_pairings.iter().map(|pairing| {
        format!(
            "  {} ↔ {}  /  {}",
            pairing.minorian.as_str(),
            pairing.minoan.as_str(),
            pairing.contrast
        )
    }));
    sections.push(aura_pairs);

    let synthesis = &hierarchy.sovereign_synthesis;
    sections.push(vec![
        "SANDMANOR  /  SOVEREIGN SYNTHESIS".into(),
        format!("  {}", synthesis.synthesis_formula),
        format!("  {}", synthesis.sovereign_formula),
        format!("  Meaning       {}", synthesis.meaning),
        format!("  Principle     {}", synthesis.core_principle.join(" • ")),
        format!("  Social rule   {}", synthesis.social_boundary),
        format!("  Nature rule   {}", synthesis.nature_boundary),
        format!("  Axis rule     {}", hierarchy.axis_invariant),
    ]);
}

pub fn render_flynt_identities(
    catalog: &HollowDeckCatalog,
    nature: Option<&str>,
) -> Result<String, String> {
    let profiles =
        match nature {
            Some(nature) => vec![catalog.flynt_visual_identity(nature).ok_or_else(|| {
                format!("no principal Flynt visual identity profile for: {nature}")
            })?],
            None => catalog.flynt_visual_identities.iter().collect(),
        };
    let mut sections = vec![vec![
        "NATURE DEFINES THE BODY; ORGANIZATION DEFINES SOCIAL PRESENTATION".into(),
        "Contextual names are one person's identities, never separate Natures or evolution stages."
            .into(),
        format!(
            "{:<12} {:<14} {:<20} {:<16}",
            "NATURE", "MYSTERY MEN", "WE FAIRY MEN", "THE GALLOWS"
        ),
        "─".repeat(68),
    ]];
    for profile in profiles {
        let nature = catalog
            .nature(&profile.nature_id)
            .expect("validated Flynt identity Nature");
        sections.push(vec![
            format!(
                "{:<12} {:<14} {:<20} {:<16}",
                nature.name,
                profile.mystery_men_identity.name,
                profile.we_fairy_men_identity.name,
                profile.gallows_identity.name
            ),
            format!("  Symbol        {}", profile.symbol_anchor),
            format!("  Mythic source {}", profile.mythic_source),
            format!(
                "  Echoes        {}",
                list_or_none(&profile.secondary_echoes, "none")
            ),
            format!(
                "  Mystery Men  {}",
                profile.mystery_men_identity.visual_direction
            ),
            format!(
                "  We Fairy Men {}",
                profile.we_fairy_men_identity.visual_direction
            ),
            format!(
                "  The Gallows  {}",
                profile.gallows_identity.visual_direction
            ),
            format!("  Motif rule   {}", profile.motif_direction),
        ]);
    }
    for organization in FlyntOrganization::ALL {
        let language = catalog
            .flynt_visual_language(organization)
            .expect("validated Flynt institutional visual language");
        sections.push(vec![
            format!(
                "{}  /  VISUAL LANGUAGE",
                organization.as_str().to_uppercase()
            ),
            format!("  Character    {}", language.visual_character.join(" • ")),
            format!("  Social power {}", language.social_power.join(" • ")),
            format!("  Wardrobe     {}", language.wardrobe.join(" • ")),
            format!("  Environment  {}", language.environment.join(" • ")),
            format!("  Animation    {}", language.animation_direction),
            format!("  Avoid        {}", language.avoid.join(" • ")),
        ]);
    }
    Ok(sectioned_box(
        "HOLLOW DECK  /  FLYNT CONTEXTUAL IDENTITIES",
        &sections,
    ))
}

#[must_use]
pub fn render_counterparts(catalog: &HollowDeckCatalog) -> String {
    let mut lines = vec![
        "These are matched Natures from DIFFERENT Houses, not members of one House.".into(),
        "".into(),
        format!(
            "{:>4}  {:<16} {:^7} {:<16} {:>6}",
            "TIER", "STONEBEND", "", "GLAÜSHOUSE", "BUDGET"
        ),
        "─".repeat(65),
    ];
    for counterpart in &catalog.cross_house_counterparts {
        lines.push(format!(
            "{:>4}  {:<16} {:^7} {:<16} {:>6}",
            counterpart.tier,
            name_for_id(catalog, &counterpart.nature_a),
            "↔",
            name_for_id(catalog, &counterpart.nature_b),
            counterpart.stat_budget
        ));
    }
    lines.extend([
        "".into(),
        "Troglodyte / Hollowing creates ABSENCE.".into(),
        "Muse / Hope creates POSSIBILITY.".into(),
    ]);
    boxed("HOLLOW DECK  /  STONEBEND ↔ GLAÜSHOUSE", &lines)
}

#[must_use]
pub fn render_sandmanor(catalog: &HollowDeckCatalog) -> String {
    render_institutions(catalog, Some("sandmanor"))
        .expect("canonical Sandmanor institutional board")
}

pub fn render_current_forms(
    catalog: &HollowDeckCatalog,
    nature: Option<&str>,
) -> Result<String, String> {
    if let Some(nature) = nature {
        let nature = catalog
            .nature(nature)
            .ok_or_else(|| format!("unknown Nature: {nature}"))?;
        let form = catalog
            .current_form(&nature.id)
            .ok_or_else(|| format!("{} has no Current Form record", nature.name))?;
        let mut sections = vec![vec![
            format!(
                "{} / {}",
                nature.name.to_uppercase(),
                nature.current_type.as_str()
            ),
            stat_line(form.computed_stats),
            "Current Form is physical configuration; it is not the Evolution Path.".into(),
        ]];
        if let Some(path) = get_growth_path_for_nature(catalog, &nature.id) {
            let mut growth = vec![
                "UNIFIED BIOLOGICAL GROWTH  /  INHERITED MILESTONES".into(),
                "  Continuous development basis points remain independent from this discrete stage."
                    .into(),
            ];
            growth.extend(growth_checklist(path, form.current_growth_stage.as_deref()));
            sections.push(growth);
        }
        let mut features = vec!["ANATOMICAL / PHYSICAL FEATURES".into()];
        if form.current_features.is_empty() {
            features.push("  Canonical features not yet specified (TODO).".into());
        }
        for feature in &form.current_features {
            features.push(format!(
                "  {:<24} {:<16} {}",
                feature.name,
                format!("{:?}", feature.category),
                if feature.stages.is_empty() {
                    "stages TODO".into()
                } else {
                    feature.stages.join(" → ")
                }
            ));
            if let Some(visual) = &feature.visual_effects {
                features.push(format!("    {visual}"));
            }
        }
        sections.push(features);
        let mut techniques = vec!["ACTIVE CURRENT TECHNIQUES".into()];
        if form.current_techniques.is_empty() {
            techniques.push("  No canonical active Current technique specified (TODO).".into());
        }
        for technique in &form.current_techniques {
            techniques.push(format!(
                "  {}: {}",
                technique.name,
                technique.stages.join(" → ")
            ));
            techniques.push(format!("    {}", technique.description));
        }
        sections.push(techniques);
        Ok(sectioned_box("HOLLOW DECK  /  CURRENT FORM", &sections))
    } else {
        let mut lines = vec![
            "Current Form = what the transformed BODY presently is and can physically do.".into(),
            String::new(),
        ];
        for form in &catalog.current_forms {
            let nature = name_for_id(catalog, &form.nature_id);
            lines.push(format!(
                "{:<14} {:>2} features  {:>2} techniques  {:<14} TOTAL {:>3}",
                nature,
                form.current_features.len(),
                form.current_techniques.len(),
                form.current_growth_stage
                    .as_deref()
                    .unwrap_or("no growth path"),
                form.computed_stats.computed_total()
            ));
        }
        Ok(boxed("HOLLOW DECK  /  CURRENT FORMS", &lines))
    }
}

pub fn render_aura_sets(
    catalog: &HollowDeckCatalog,
    nature: Option<&str>,
) -> Result<String, String> {
    let mut lines = vec![
        "Aura Set = what the MIND can do. It develops independently from Current Form.".into(),
        "".into(),
    ];
    let sets = catalog
        .aura_sets
        .iter()
        .filter(|set| {
            nature.is_none_or(|needle| {
                catalog
                    .nature(needle)
                    .is_some_and(|n| n.id == set.nature_id)
            })
        })
        .collect::<Vec<_>>();
    if nature.is_some() && sets.is_empty() {
        return Err(format!("unknown Nature: {}", nature.unwrap_or_default()));
    }
    for set in sets {
        lines.push(name_for_id(catalog, &set.nature_id).to_uppercase());
        lines.push(format!(
            "  Learned   {}",
            list_or_none(&set.learned_moves, "none recorded")
        ));
        lines.push(format!(
            "  Equipped  {}",
            list_or_none(&set.equipped_moves, "none recorded")
        ));
        if !set.design_examples.is_empty() {
            lines.push(format!(
                "  Design examples / not locked: {}",
                set.design_examples.join(", ")
            ));
        }
        lines.push(String::new());
    }
    Ok(boxed("HOLLOW DECK  /  AURA SETS", &lines))
}

#[must_use]
pub fn render_traits(catalog: &HollowDeckCatalog, nature: Option<&str>) -> Result<String, String> {
    let natures = if let Some(nature) = nature {
        vec![
            catalog
                .nature(nature)
                .ok_or_else(|| format!("unknown Nature: {nature}"))?,
        ]
    } else {
        catalog.natures.iter().collect()
    };
    let lines = natures
        .into_iter()
        .map(|nature| {
            let trait_entry = get_primary_trait(catalog, &nature.id).expect("validated trait");
            format!(
                "{:<14} {:<18} {}",
                nature.name, trait_entry.name, trait_entry.mechanical_effect
            )
        })
        .collect::<Vec<_>>();
    Ok(boxed("HOLLOW DECK  /  PRIMARY TRAITS", &lines))
}

pub fn render_compare(
    catalog: &HollowDeckCatalog,
    left: &str,
    right: &str,
) -> Result<String, String> {
    let left = catalog
        .nature(left)
        .ok_or_else(|| format!("unknown Nature: {left}"))?;
    let right = catalog
        .nature(right)
        .ok_or_else(|| format!("unknown Nature: {right}"))?;
    let left_trait = get_primary_trait(catalog, &left.id).expect("validated trait");
    let right_trait = get_primary_trait(catalog, &right.id).expect("validated trait");
    let mut lines = vec![
        format!(
            "{:<39} {}",
            left.name.to_uppercase(),
            right.name.to_uppercase()
        ),
        format!(
            "{:<39} {}",
            format!("House      {}", house_display_name(left.house_id)),
            format!("House      {}", house_display_name(right.house_id))
        ),
        format!(
            "{:<39} {}",
            format!("Dominion   {}", left.dominion_id.as_str()),
            format!("Dominion   {}", right.dominion_id.as_str())
        ),
        format!(
            "{:<39} {}",
            format!("Current    {}", left.current_type.as_str()),
            format!("Current    {}", right.current_type.as_str())
        ),
        format!(
            "{:<39} {}",
            format!("Aura       {}", left.aura_material.as_str()),
            format!("Aura       {}", right.aura_material.as_str())
        ),
        format!(
            "{:<39} {}",
            format!("Trait      {}", left_trait.name),
            format!("Trait      {}", right_trait.name)
        ),
        "".into(),
        format!(
            "{:<8} {:>8} {:>8} {:>10}",
            "STAT", left.name, right.name, "LEFT-RIGHT"
        ),
    ];
    for (name, l, r) in [
        (
            "STR",
            left.mature_stats.strength,
            right.mature_stats.strength,
        ),
        ("SPD", left.mature_stats.speed, right.mature_stats.speed),
        ("ATK", left.mature_stats.attack, right.mature_stats.attack),
        ("DEF", left.mature_stats.defense, right.mature_stats.defense),
    ] {
        lines.push(format!(
            "{:<8} {:>8} {:>8} {:>+10}",
            name,
            l,
            r,
            i16::from(l) - i16::from(r)
        ));
    }
    lines.push(format!(
        "{:<8} {:>8} {:>8} {:>+10}",
        "TOTAL",
        left.mature_stats.computed_total(),
        right.mature_stats.computed_total(),
        i32::from(left.mature_stats.computed_total())
            - i32::from(right.mature_stats.computed_total())
    ));
    lines.extend([
        "".into(),
        format!(
            "Aura relationship: {} {} against {} (Aura/Synthesis only)",
            left.aura_material.as_str(),
            get_aura_matchup(left.aura_material, right.aura_material).label(),
            right.aura_material.as_str()
        ),
        relationship_summary(catalog, left, right),
        format!(
            "Evolution tiers: {} / {}",
            get_evolution_tier(catalog, &left.id)
                .map_or_else(|| "pair-based".into(), |tier| tier.to_string()),
            get_evolution_tier(catalog, &right.id)
                .map_or_else(|| "pair-based".into(), |tier| tier.to_string())
        ),
    ]);
    Ok(boxed("HOLLOW DECK  /  COMPARE", &lines))
}

#[must_use]
pub fn render_alchemy() -> String {
    let mut lines = vec![
        "Diamond  →  Glaüshouse (Jade / Emerald)  →  Opal  →  Crystal  →  Diamond".into(),
        "Jade and Emerald share one alchemical slot and are neutral to each other.".into(),
        "".into(),
    ];
    for material in AuraMaterial::ALL {
        let (strong, weak, neutral) = material_relations(material);
        lines.extend([
            material.as_str().to_uppercase(),
            format!("  strong vs {strong:<8}  +10%  ×1.10"),
            format!("  weak vs   {weak:<8}  -10%  ×0.90"),
            format!("  neutral vs {neutral:<8} and same material  0%  ×1.00"),
            String::new(),
        ]);
    }
    lines.extend([
        "SOFT MATCHUP: applies to Aura effects and applicable Synthesis effects.".into(),
        "It does not automatically modify purely Current-based physical actions.".into(),
    ]);
    boxed("HOLLOW DECK  /  AURA ALCHEMY", &lines)
}

#[must_use]
pub fn render_recipes(catalog: &HollowDeckCatalog) -> String {
    let mut lines = vec![
        "Ordinary real-world crafting. Recipes are not Synthesis Recipes.".into(),
        "".into(),
    ];
    if catalog.recipes.is_empty() {
        lines.push("No canonical ordinary Recipes are seeded yet.".into());
        lines.push(
            "The first-class Recipe schema is ready for food, tools, clothing, equipment,".into(),
        );
        lines.push(
            "hardware, weapons, furniture, repairs, construction, medicine, and materials.".into(),
        );
    }
    for recipe in &catalog.recipes {
        lines.extend([
            format!("{} / {}", recipe.name.to_uppercase(), recipe.category),
            format!("  {}", recipe.description),
            format!(
                "  Result: {} ×{}",
                recipe.result_item, recipe.result_quantity
            ),
            format!("  Known: {}", yes_no(recipe.known)),
            String::new(),
        ]);
    }
    boxed("HOLLOW DECK  /  RECIPES", &lines)
}

#[must_use]
pub fn render_synthesis_recipes(catalog: &HollowDeckCatalog) -> String {
    let mut sections = vec![vec![
        "Collectible supernatural patterns. Knowledge persists; consumed reagents do not.".into(),
        "A valid pattern may use Current only, Aura only, or Current + Aura.".into(),
    ]];
    for recipe in &catalog.synthesis_recipes {
        sections.push(vec![
            format!(
                "{}  [{}]",
                recipe.name.to_uppercase(),
                recipe.result_type.as_str()
            ),
            format!(
                "  Source      {:?} / {}",
                recipe.source_type, recipe.source_description
            ),
            format!("  Discovered  {}", yes_no(recipe.is_discovered)),
            format!(
                "  Natures     {}",
                list_or_none(&recipe.compatible_natures, "any")
            ),
            format!(
                "  Current     features: {} / techniques: {}",
                list_or_none(&recipe.required_current_features, "none"),
                list_or_none(&recipe.required_current_techniques, "none")
            ),
            format!(
                "  Growth      {}",
                recipe.required_growth_stage.as_ref().map_or_else(
                    || "none".into(),
                    |requirement| format!(
                        "{} at or beyond {}",
                        name_for_id(catalog, &requirement.nature_id),
                        requirement.milestone_id
                    )
                )
            ),
            format!(
                "  Aura        {}",
                list_or_none(&recipe.required_aura_moves, "none")
            ),
            format!(
                "  Reagents    optional: {} / consumed: {}",
                list_or_none(&recipe.optional_reagents, "none"),
                list_or_none(&recipe.consumed_reagents, "none")
            ),
            format!("  Result      {}", recipe.result_synthesis),
            format!("  Effect      {}", recipe.mechanical_effect),
            format!("  Discovery   {}", recipe.lore_text),
        ]);
    }
    sectioned_box("HOLLOW DECK  /  SYNTHESIS RECIPES", &sections)
}

#[must_use]
pub fn render_syntheses(catalog: &HollowDeckCatalog) -> String {
    let mut sections = Vec::new();
    for synthesis in &catalog.syntheses {
        sections.push(vec![
            format!(
                "{}  [{}]",
                synthesis.name.to_uppercase(),
                synthesis.result_type.as_str()
            ),
            format!(
                "  Source Recipe  {}",
                synthesis
                    .source_recipe_id
                    .as_deref()
                    .unwrap_or("not yet specified (TODO)")
            ),
            format!(
                "  Natures        {}",
                list_or_none(&synthesis.compatible_natures, "any")
            ),
            format!(
                "  Current        {}",
                list_or_none(&synthesis.current_requirements, "none")
            ),
            format!(
                "  Growth         {}",
                synthesis.required_growth_stage.as_ref().map_or_else(
                    || "none".into(),
                    |requirement| format!(
                        "{} at or beyond {}",
                        name_for_id(catalog, &requirement.nature_id),
                        requirement.milestone_id
                    )
                )
            ),
            format!(
                "  Aura           {}",
                list_or_none(&synthesis.aura_requirements, "none")
            ),
            format!(
                "  Reagents       {}",
                list_or_none(&synthesis.reagent_requirements, "none")
            ),
            format!("  Effect         {}", synthesis.effects.join("; ")),
            format!("  {}", synthesis.description),
        ]);
    }
    sectioned_box("HOLLOW DECK  /  SYNTHESES", &sections)
}

#[must_use]
pub fn render_banes(catalog: &HollowDeckCatalog) -> String {
    let mut sections = vec![vec![
        "Banes counter a Nature's defining mechanism; they are not arbitrary bonus damage.".into(),
    ]];
    for bane in &catalog.banes {
        sections.push(vec![
            format!("{}", bane.name.to_uppercase()),
            format!("  Target Natures  {}", bane.target_natures.join(", ")),
            format!(
                "  Target Traits   {}",
                bane.target_primary_traits.join(", ")
            ),
            format!("  Effect Type     {}", bane.effect_type),
            format!("  Mechanical      {}", bane.mechanical_effect),
            format!(
                "  Duration        {}",
                bane.duration.as_deref().unwrap_or("configurable / TODO")
            ),
            format!(
                "  Delivery        {}",
                bane.delivery_method
                    .as_deref()
                    .unwrap_or("configurable / TODO")
            ),
            format!(
                "  Strength        {}",
                bane.strength.as_deref().unwrap_or("configurable / TODO")
            ),
            format!(
                "  Lore            {}",
                bane.lore_notes.as_deref().unwrap_or("TODO")
            ),
        ]);
    }
    sectioned_box("HOLLOW DECK  /  BANES", &sections)
}

#[must_use]
pub fn render_validation(catalog: &HollowDeckCatalog) -> String {
    let lines = match validate_hollow_deck(catalog) {
        Ok(()) => vec![
            "PASS — canonical Hollow Deck data is internally consistent.".into(),
            format!(
                "{} Houses / {} Dominions / {} Natures / {} Primary Traits",
                catalog.houses.len(),
                catalog.dominions.len(),
                catalog.natures.len(),
                catalog.primary_traits.len()
            ),
            format!(
                "{} paths / {} Flynt pairs / {} cross-House counterparts",
                catalog.evolution_paths.len(),
                catalog.pair_relationships.len(),
                catalog.cross_house_counterparts.len()
            ),
            "Hueman 50/50/50/50 • stat range 0–100 • full-expression ceiling 320".into(),
            "Glaüshouse: South • Precog • Jade / Emerald as coequal alchemical-slot materials"
                .into(),
        ],
        Err(errors) => {
            let mut lines = vec![format!("FAIL — {} validation error(s)", errors.len())];
            lines.extend(
                errors
                    .into_iter()
                    .map(|error| format!("{}: {}", error.code, error.message)),
            );
            lines
        }
    };
    boxed("HOLLOW DECK  /  VALIDATION", &lines)
}

#[must_use]
pub fn render_help() -> String {
    boxed(
        "HOLLOW DECK  /  HELP",
        &[
            "Interactive: hollow_deck".into(),
            "Integrated:  hollow-grove deck".into(),
            "".into(),
            "home | houses | institutions [glaushouse|flynt|stonebend|sandmanor]".into(),
            "glaushouse | flynt-identities [nature] | natures [filters]".into(),
            "nature <name> | progression".into(),
            "counterparts | sandmanor | current-forms [nature] | aura-sets [nature]".into(),
            "traits [nature] | compare <nature-a> <nature-b> | alchemy | recipes".into(),
            "synthesis-recipes | syntheses | banes | state | validate | export-json".into(),
            "export-state | help".into(),
            "".into(),
            "Cultivation state commands (persist atomically):".into(),
            "  cultivate <nature> <basis-points>  (0–10000)".into(),
            "  set-feature <nature> <feature> <stage>".into(),
            "  set-growth <nature> <milestone>  (ordered, inherited anatomy)".into(),
            "  set-technique <nature> <technique> <stage>".into(),
            "  learn-aura | equip-aura | unequip-aura  <nature> <move>".into(),
            "  learn-recipe <recipe> | discover-synthesis-recipe <pattern>".into(),
            "  record-synthesis <nature> <synthesis> [available-reagent ...]".into(),
            "  set-glaushouse-affiliation <seat> [presensor|preceptor]".into(),
            "  Override save location with HOLLOW_DECK_STATE_PATH=/path/to/state.json".into(),
            "".into(),
            "Nature filters:".into(),
            "  --search TEXT --house HOUSE --dominion DOMINION --current TYPE --aura MATERIAL"
                .into(),
            "  --trait TRAIT --total N --tier N --path PATH --paired [NATURE] --principle LAW"
                .into(),
            "  --sort name|house|dominion|current|aura|trait|total|tier".into(),
            "".into(),
            "Interactive keys: ↑/k ↓/j select • Enter/l open • ←/h back • Tab switch compare side"
                .into(),
            "                  +/- cultivate by 5% on a Nature card • / search Natures".into(),
            "                  c compare selected • ? help • q quit".into(),
        ],
    )
}

fn relationship_summary(catalog: &HollowDeckCatalog, left: &Nature, right: &Nature) -> String {
    if let Some(pair) = get_pair_relationship(catalog, &left.id)
        && ((pair.nature_a == left.id && pair.nature_b == right.id)
            || (pair.nature_b == left.id && pair.nature_a == right.id))
    {
        return format!("Paired relationship: {}", pair.shared_principle);
    }
    if get_cross_house_counterpart(catalog, &left.id).is_some_and(|nature| nature.id == right.id) {
        return "Cross-House counterpart: yes".into();
    }
    "Paired / counterpart relationship: none".into()
}

fn material_relations(material: AuraMaterial) -> (String, String, String) {
    match material {
        AuraMaterial::Diamond => ("Jade / Emerald".into(), "Crystal".into(), "Opal".into()),
        AuraMaterial::Jade => ("Opal".into(), "Diamond".into(), "Crystal / Emerald".into()),
        AuraMaterial::Emerald => ("Opal".into(), "Diamond".into(), "Crystal / Jade".into()),
        AuraMaterial::Opal => ("Crystal".into(), "Jade / Emerald".into(), "Diamond".into()),
        AuraMaterial::Crystal => ("Diamond".into(), "Opal".into(), "Jade / Emerald".into()),
    }
}

fn house_aura_names(house: &HouseEntry) -> String {
    house
        .aura_materials
        .iter()
        .map(|material| material.as_str())
        .collect::<Vec<_>>()
        .join(" / ")
}

fn growth_checklist(path: &UnifiedGrowthPath, current_stage: Option<&str>) -> Vec<String> {
    let current_index = current_stage.and_then(|stage| get_growth_stage_index(path, stage));
    let mut lines = path
        .milestones
        .iter()
        .enumerate()
        .map(|(index, milestone)| {
            format!(
                "    [{}] {}",
                if current_index.is_some_and(|current| index <= current) {
                    '✓'
                } else {
                    ' '
                },
                milestone.name
            )
        })
        .collect::<Vec<_>>();
    lines.push(format!(
        "    Current Stage: {}",
        current_index
            .and_then(|index| path.milestones.get(index))
            .map(|milestone| milestone.name.as_str())
            .unwrap_or("Hueman / not yet cultivated")
    ));
    lines
}

fn stat_line(stats: StatBlock) -> String {
    format!(
        "STR {:>3}   SPD {:>3}   ATK {:>3}   DEF {:>3}   TOTAL {:>3}",
        stats.strength,
        stats.speed,
        stats.attack,
        stats.defense,
        stats.computed_total()
    )
}

fn name_for_id(catalog: &HollowDeckCatalog, id: &str) -> String {
    catalog
        .nature(id)
        .map(|nature| nature.name.clone())
        .unwrap_or_else(|| id.to_owned())
}

fn names_for_ids(catalog: &HollowDeckCatalog, ids: &[String]) -> Vec<String> {
    ids.iter().map(|id| name_for_id(catalog, id)).collect()
}

fn move_names(catalog: &HollowDeckCatalog, ids: &[String]) -> String {
    if ids.is_empty() {
        return "none".into();
    }
    ids.iter()
        .map(|id| {
            catalog
                .aura_moves
                .iter()
                .find(|item| item.id == *id)
                .map(|item| item.name.as_str())
                .unwrap_or(id)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn known_names<'a>(
    known: &BTreeSet<String>,
    entries: impl Iterator<Item = (&'a String, &'a String)>,
) -> String {
    let names = entries
        .filter(|(id, _)| known.contains(id.as_str()))
        .map(|(_, name)| name.as_str())
        .collect::<Vec<_>>();
    if names.is_empty() {
        "none".into()
    } else {
        names.join(", ")
    }
}

fn list_or_none(items: &[String], none: &str) -> String {
    if items.is_empty() {
        none.into()
    } else {
        items.join(", ")
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn boxed(title: &str, lines: &[String]) -> String {
    sectioned_box(title, &[lines.to_vec()])
}

fn sectioned_box(title: &str, sections: &[Vec<String>]) -> String {
    let inner = WIDTH - 2;
    let mut output = String::new();
    output.push('┌');
    output.push_str(&"─".repeat(inner));
    output.push_str("┐\n");
    output.push_str(&box_line(title, inner));
    for section in sections {
        output.push('├');
        output.push_str(&"─".repeat(inner));
        output.push_str("┤\n");
        for line in section {
            for wrapped in wrap_line(line, inner.saturating_sub(2)) {
                output.push_str(&box_line(&format!(" {wrapped}"), inner));
            }
        }
    }
    output.push('└');
    output.push_str(&"─".repeat(inner));
    output.push('┘');
    output
}

fn box_line(contents: &str, width: usize) -> String {
    let clipped = truncate(contents, width);
    let padding = width.saturating_sub(display_width(&clipped));
    format!("│{clipped}{}│\n", " ".repeat(padding))
}

fn wrap_line(line: &str, width: usize) -> Vec<String> {
    if display_width(line) <= width {
        return vec![line.to_owned()];
    }
    let indent = line
        .chars()
        .take_while(|character| character.is_whitespace())
        .count();
    let prefix = " ".repeat(indent.min(width / 2));
    let mut wrapped = Vec::new();
    let mut current = String::new();
    for word in line.split_whitespace() {
        let needed = if current.is_empty() {
            display_width(word)
        } else {
            display_width(&current) + 1 + display_width(word)
        };
        if needed > width && !current.is_empty() {
            wrapped.push(current);
            current = prefix.clone();
        }
        if !current.trim().is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        wrapped.push(current);
    }
    wrapped
}

fn truncate(value: &str, width: usize) -> String {
    if display_width(value) <= width {
        return value.to_owned();
    }
    value
        .chars()
        .take(width.saturating_sub(1))
        .collect::<String>()
        + "…"
}

fn display_width(value: &str) -> usize {
    value.chars().count()
}
