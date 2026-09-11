use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use hollow_grove::House;
use hollow_grove::constitutional::canonical_visual_color_constitution;
use hollow_grove::hollow_deck::{HollowDeckCatalog, canonical_hollow_deck, validate_hollow_deck};
use hollow_grove::hollow_deck_state::{
    HollowDeckState, encode_hollow_deck_state, hollow_deck_state_path, load_hollow_deck_state,
    save_hollow_deck_state,
};
use hollow_grove::hollow_deck_tui::{
    NatureQuery, NatureSort, query_natures, render_alchemy, render_aura_sets, render_banes,
    render_compare, render_counterparts, render_cultivation_state, render_current_forms,
    render_flynt_identities, render_glaushouse, render_help, render_home, render_houses,
    render_institutions, render_nature_card_with_state, render_nature_index, render_progression,
    render_recipes, render_sandmanor, render_syntheses, render_synthesis_recipes, render_traits,
    render_validation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeckCli {
    Interactive,
    Home,
    Houses,
    Institutions(Option<String>),
    Glaushouse,
    FlyntIdentities(Option<String>),
    Natures(NatureQuery),
    Nature(String),
    Progression,
    Counterparts,
    Sandmanor,
    CurrentForms(Option<String>),
    AuraSets(Option<String>),
    Traits(Option<String>),
    Compare(String, String),
    Alchemy,
    Recipes,
    SynthesisRecipes,
    Syntheses,
    Banes,
    State,
    Cultivate(String, u16),
    SetGrowth(String, String),
    SetFeature(String, String, String),
    SetTechnique(String, String, String),
    LearnAura(String, String),
    EquipAura(String, String),
    UnequipAura(String, String),
    LearnRecipe(String),
    DiscoverSynthesisRecipe(String),
    RecordSynthesis(String, String, Vec<String>),
    SetGlaushouseAffiliation(String, Option<String>),
    Validate,
    ExportJson,
    ExportState,
    Help,
}

fn parse_cli(args: impl IntoIterator<Item = String>) -> Result<DeckCli, String> {
    let mut args = args.into_iter().collect::<Vec<_>>();
    if args.is_empty() {
        return Ok(DeckCli::Interactive);
    }
    let command = args.remove(0);
    match command.as_str() {
        "--help" | "-h" | "help" => no_extra(args, DeckCli::Help, "help"),
        "home" => no_extra(args, DeckCli::Home, "home"),
        "houses" => no_extra(args, DeckCli::Houses, "houses"),
        "institutions" | "institution" => {
            optional_name(args, "institutions", DeckCli::Institutions)
        }
        "glaushouse" | "glaüshouse" | "muse-seats" => {
            no_extra(args, DeckCli::Glaushouse, "glaushouse")
        }
        "flynt-identities" | "flynt-identity" => {
            optional_name(args, "flynt-identities", DeckCli::FlyntIdentities)
        }
        "natures" | "index" => parse_nature_query(args).map(DeckCli::Natures),
        "nature" | "card" => one_optional_name(args, "nature", DeckCli::Nature),
        "progression" => no_extra(args, DeckCli::Progression, "progression"),
        "counterparts" => no_extra(args, DeckCli::Counterparts, "counterparts"),
        "sandmanor" => no_extra(args, DeckCli::Sandmanor, "sandmanor"),
        "current-forms" | "current-form" => {
            optional_name(args, "current-forms", DeckCli::CurrentForms)
        }
        "aura-sets" | "aura-set" => optional_name(args, "aura-sets", DeckCli::AuraSets),
        "traits" | "primary-traits" => optional_name(args, "traits", DeckCli::Traits),
        "compare" => {
            if args.len() != 2 {
                Err("compare requires exactly <nature-a> <nature-b>".into())
            } else {
                Ok(DeckCli::Compare(args.remove(0), args.remove(0)))
            }
        }
        "alchemy" => no_extra(args, DeckCli::Alchemy, "alchemy"),
        "recipes" => no_extra(args, DeckCli::Recipes, "recipes"),
        "synthesis-recipes" => no_extra(args, DeckCli::SynthesisRecipes, "synthesis-recipes"),
        "syntheses" | "synthesis" => no_extra(args, DeckCli::Syntheses, "syntheses"),
        "banes" => no_extra(args, DeckCli::Banes, "banes"),
        "state" | "cultivation" => no_extra(args, DeckCli::State, "state"),
        "cultivate" => {
            require_count(&args, 2, "cultivate <nature> <basis-points>")?;
            let basis_points = args[1].parse::<u16>().map_err(|_| {
                "cultivate basis-points must be an integer from 0 to 10000".to_owned()
            })?;
            Ok(DeckCli::Cultivate(args[0].clone(), basis_points))
        }
        "set-feature" => {
            require_count(&args, 3, "set-feature <nature> <feature> <stage>")?;
            Ok(DeckCli::SetFeature(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
            ))
        }
        "set-growth" => {
            require_count(&args, 2, "set-growth <nature> <stage>")?;
            Ok(DeckCli::SetGrowth(args[0].clone(), args[1].clone()))
        }
        "set-technique" => {
            require_count(&args, 3, "set-technique <nature> <technique> <stage>")?;
            Ok(DeckCli::SetTechnique(
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
            ))
        }
        "learn-aura" => {
            require_count(&args, 2, "learn-aura <nature> <move>")?;
            Ok(DeckCli::LearnAura(args[0].clone(), args[1].clone()))
        }
        "equip-aura" => {
            require_count(&args, 2, "equip-aura <nature> <move>")?;
            Ok(DeckCli::EquipAura(args[0].clone(), args[1].clone()))
        }
        "unequip-aura" => {
            require_count(&args, 2, "unequip-aura <nature> <move>")?;
            Ok(DeckCli::UnequipAura(args[0].clone(), args[1].clone()))
        }
        "learn-recipe" => {
            require_count(&args, 1, "learn-recipe <recipe>")?;
            Ok(DeckCli::LearnRecipe(args[0].clone()))
        }
        "discover-synthesis-recipe" => {
            require_count(&args, 1, "discover-synthesis-recipe <synthesis-recipe>")?;
            Ok(DeckCli::DiscoverSynthesisRecipe(args[0].clone()))
        }
        "record-synthesis" => {
            if args.len() < 2 {
                return Err(
                    "record-synthesis requires <nature> <synthesis> [available-reagent ...]".into(),
                );
            }
            Ok(DeckCli::RecordSynthesis(
                args[0].clone(),
                args[1].clone(),
                args[2..].to_vec(),
            ))
        }
        "set-glaushouse-affiliation" => {
            if !(1..=2).contains(&args.len()) {
                return Err(
                    "usage: hollow_deck set-glaushouse-affiliation <seat> [presensor|preceptor]"
                        .into(),
                );
            }
            Ok(DeckCli::SetGlaushouseAffiliation(
                args[0].clone(),
                args.get(1).cloned(),
            ))
        }
        "validate" => no_extra(args, DeckCli::Validate, "validate"),
        "export-json" => no_extra(args, DeckCli::ExportJson, "export-json"),
        "export-state" => no_extra(args, DeckCli::ExportState, "export-state"),
        other => Err(format!("unknown Hollow Deck command: {other}")),
    }
}

fn require_count(args: &[String], expected: usize, usage: &str) -> Result<(), String> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(format!("usage: hollow_deck {usage}"))
    }
}

fn one_optional_name(
    args: Vec<String>,
    command: &str,
    constructor: fn(String) -> DeckCli,
) -> Result<DeckCli, String> {
    if args.len() == 1 {
        Ok(constructor(args[0].clone()))
    } else {
        Err(format!("{command} requires exactly <nature>"))
    }
}

fn optional_name(
    args: Vec<String>,
    command: &str,
    constructor: fn(Option<String>) -> DeckCli,
) -> Result<DeckCli, String> {
    if args.len() <= 1 {
        Ok(constructor(args.first().cloned()))
    } else {
        Err(format!("{command} accepts at most one <nature>"))
    }
}

fn no_extra(args: Vec<String>, cli: DeckCli, command: &str) -> Result<DeckCli, String> {
    if args.is_empty() {
        Ok(cli)
    } else {
        Err(format!("{command} does not accept additional arguments"))
    }
}

fn parse_nature_query(args: Vec<String>) -> Result<NatureQuery, String> {
    let mut query = NatureQuery::default();
    let mut index = 0;
    while index < args.len() {
        let flag = args[index].as_str();
        if flag == "--paired" {
            let value = args
                .get(index + 1)
                .filter(|value| !value.starts_with("--"))
                .cloned()
                .unwrap_or_default();
            query.paired = Some(value);
            index += if args
                .get(index + 1)
                .is_some_and(|value| !value.starts_with("--"))
            {
                2
            } else {
                1
            };
            continue;
        }
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("{flag} requires a value"))?
            .clone();
        match flag {
            "--search" => query.search = Some(value),
            "--house" => query.house = Some(value),
            "--dominion" => query.dominion = Some(value),
            "--current" => query.current = Some(value),
            "--aura" => query.aura = Some(value),
            "--trait" => query.trait_name = Some(value),
            "--total" => {
                query.total = Some(
                    value
                        .parse()
                        .map_err(|_| "--total requires an integer".to_owned())?,
                );
            }
            "--tier" => {
                query.tier = Some(
                    value
                        .parse()
                        .map_err(|_| "--tier requires an integer".to_owned())?,
                );
            }
            "--path" => query.path = Some(value),
            "--principle" => query.principle = Some(value),
            "--sort" => query.sort = parse_sort(&value)?,
            _ => return Err(format!("unknown Nature filter: {flag}")),
        }
        index += 2;
    }
    Ok(query)
}

fn parse_sort(value: &str) -> Result<NatureSort, String> {
    match value {
        "name" => Ok(NatureSort::Name),
        "house" => Ok(NatureSort::House),
        "dominion" => Ok(NatureSort::Dominion),
        "current" => Ok(NatureSort::Current),
        "aura" => Ok(NatureSort::Aura),
        "trait" => Ok(NatureSort::Trait),
        "total" => Ok(NatureSort::Total),
        "tier" => Ok(NatureSort::Tier),
        _ => Err(format!("unknown Nature sort: {value}")),
    }
}

fn run_cli(
    catalog: &HollowDeckCatalog,
    state: &mut HollowDeckState,
    state_path: &Path,
    cli: DeckCli,
) -> Result<String, String> {
    match cli {
        DeckCli::Interactive => Ok(render_home(catalog)),
        DeckCli::Home => Ok(render_home(catalog)),
        DeckCli::Houses => Ok(render_houses(catalog)),
        DeckCli::Institutions(house) => render_institutions(catalog, house.as_deref()),
        DeckCli::Glaushouse => Ok(render_glaushouse(catalog)),
        DeckCli::FlyntIdentities(nature) => render_flynt_identities(catalog, nature.as_deref()),
        DeckCli::Natures(query) => Ok(render_nature_index(catalog, &query)),
        DeckCli::Nature(nature) => render_nature_card_with_state(catalog, state, &nature),
        DeckCli::Progression => Ok(render_progression(catalog)),
        DeckCli::Counterparts => Ok(render_counterparts(catalog)),
        DeckCli::Sandmanor => Ok(render_sandmanor(catalog)),
        DeckCli::CurrentForms(nature) => render_current_forms(catalog, nature.as_deref()),
        DeckCli::AuraSets(nature) => render_aura_sets(catalog, nature.as_deref()),
        DeckCli::Traits(nature) => render_traits(catalog, nature.as_deref()),
        DeckCli::Compare(left, right) => render_compare(catalog, &left, &right),
        DeckCli::Alchemy => Ok(render_alchemy()),
        DeckCli::Recipes => Ok(render_recipes(catalog)),
        DeckCli::SynthesisRecipes => Ok(render_synthesis_recipes(catalog)),
        DeckCli::Syntheses => Ok(render_syntheses(catalog)),
        DeckCli::Banes => Ok(render_banes(catalog)),
        DeckCli::State => Ok(render_cultivation_state(catalog, state)),
        DeckCli::Cultivate(nature, basis_points) => {
            state
                .cultivate(catalog, &nature, basis_points)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::SetGrowth(nature, stage) => {
            state
                .set_growth_stage(catalog, &nature, &stage)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::SetFeature(nature, feature, stage) => {
            state
                .set_current_feature_stage(catalog, &nature, &feature, &stage)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::SetTechnique(nature, technique, stage) => {
            state
                .set_current_technique_stage(catalog, &nature, &technique, &stage)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::LearnAura(nature, aura_move) => {
            state
                .learn_aura_move(catalog, &nature, &aura_move)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::EquipAura(nature, aura_move) => {
            state
                .equip_aura_move(catalog, &nature, &aura_move)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::UnequipAura(nature, aura_move) => {
            state
                .unequip_aura_move(catalog, &nature, &aura_move)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::LearnRecipe(recipe) => {
            state
                .learn_recipe(catalog, &recipe)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            Ok(render_cultivation_state(catalog, state))
        }
        DeckCli::DiscoverSynthesisRecipe(recipe) => {
            state
                .discover_synthesis_recipe(catalog, &recipe)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            Ok(render_cultivation_state(catalog, state))
        }
        DeckCli::RecordSynthesis(nature, synthesis, reagents) => {
            state
                .record_synthesis(catalog, &nature, &synthesis, &reagents)
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            render_nature_card_with_state(catalog, state, &nature)
        }
        DeckCli::SetGlaushouseAffiliation(seat, branch) => {
            state
                .set_glaushouse_affiliation(catalog, &seat, branch.as_deref())
                .map_err(|error| error.to_string())?;
            persist_state(catalog, state, state_path)?;
            Ok(render_cultivation_state(catalog, state))
        }
        DeckCli::Validate => Ok(render_validation(catalog)),
        DeckCli::ExportJson => {
            serde_json::to_string_pretty(catalog).map_err(|error| error.to_string())
        }
        DeckCli::ExportState => {
            encode_hollow_deck_state(catalog, state).map_err(|error| error.to_string())
        }
        DeckCli::Help => Ok(render_help()),
    }
}

fn persist_state(
    catalog: &HollowDeckCatalog,
    state: &HollowDeckState,
    state_path: &Path,
) -> Result<(), String> {
    save_hollow_deck_state(catalog, state, state_path).map_err(|error| error.to_string())
}

const MENU: [(&str, Page); 17] = [
    ("Houses", Page::Houses),
    ("Institutions", Page::Institutions),
    ("Glaüshouse", Page::Glaushouse),
    ("Flynt Identities", Page::FlyntIdentities),
    ("Natures", Page::Natures),
    ("Cultivation", Page::Cultivation),
    ("Progression", Page::Progression),
    ("Current Forms", Page::CurrentForms),
    ("Aura Sets", Page::AuraSets),
    ("Primary Traits", Page::Traits),
    ("Compare", Page::Compare),
    ("Alchemy", Page::Alchemy),
    ("Recipes", Page::Recipes),
    ("Synthesis Recipes", Page::SynthesisRecipes),
    ("Syntheses", Page::Syntheses),
    ("Banes", Page::Banes),
    ("Help", Page::Help),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Houses,
    Institutions,
    Glaushouse,
    FlyntIdentities,
    Natures,
    NatureCard,
    Cultivation,
    Progression,
    CurrentForms,
    AuraSets,
    Traits,
    Compare,
    Alchemy,
    Recipes,
    SynthesisRecipes,
    Syntheses,
    Banes,
    Help,
}

struct InteractiveState {
    page: Page,
    menu_index: usize,
    nature_index: usize,
    query: NatureQuery,
    compare: [usize; 2],
    compare_side: usize,
    status: String,
}

impl Default for InteractiveState {
    fn default() -> Self {
        Self {
            page: Page::Home,
            menu_index: 0,
            nature_index: 0,
            query: NatureQuery::default(),
            compare: [0, 1],
            compare_side: 0,
            status: "Hollow Deck ready".into(),
        }
    }
}

impl InteractiveState {
    fn current_house(&self, catalog: &HollowDeckCatalog) -> Option<House> {
        match self.page {
            Page::NatureCard | Page::Natures => self
                .visible_natures(catalog)
                .get(self.nature_index)
                .map(|nature| nature.house_id),
            Page::Compare => catalog
                .natures
                .get(self.compare[self.compare_side])
                .map(|nature| nature.house_id),
            _ => None,
        }
    }

    fn visible_natures<'a>(
        &self,
        catalog: &'a HollowDeckCatalog,
    ) -> Vec<&'a hollow_grove::hollow_deck::Nature> {
        query_natures(catalog, &self.query)
    }
}

struct RawTerminal {
    saved: String,
    active: bool,
}

impl RawTerminal {
    fn enable() -> io::Result<Self> {
        let output = Command::new("stty")
            .arg("-g")
            .stdin(Stdio::inherit())
            .output()?;
        if !output.status.success() {
            return Err(io::Error::other("stty could not read terminal state"));
        }
        let saved = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let mut terminal = Self {
            saved,
            active: false,
        };
        terminal.resume()?;
        Ok(terminal)
    }

    fn suspend(&mut self) -> io::Result<()> {
        if self.active {
            let status = Command::new("stty").arg(&self.saved).status()?;
            if !status.success() {
                return Err(io::Error::other("stty could not restore terminal state"));
            }
            self.active = false;
        }
        Ok(())
    }

    fn resume(&mut self) -> io::Result<()> {
        if !self.active {
            let status = Command::new("stty").args(["raw", "-echo"]).status()?;
            if !status.success() {
                return Err(io::Error::other("stty could not enter raw mode"));
            }
            self.active = true;
        }
        Ok(())
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        let _ = self.suspend();
        println!("\x1b[?25h\x1b[0m");
        let _ = io::stdout().flush();
    }
}

fn run_interactive(
    catalog: &HollowDeckCatalog,
    deck_state: &mut HollowDeckState,
    state_path: &Path,
) -> io::Result<()> {
    let mut terminal = RawTerminal::enable()?;
    let mut state = InteractiveState::default();
    let mut stdin = io::stdin().lock();
    loop {
        draw_interactive(catalog, deck_state, &state)?;
        let key = read_key(&mut stdin)?;
        if key == Key::Quit {
            break;
        }
        if key == Key::Search && matches!(state.page, Page::Natures | Page::NatureCard) {
            terminal.suspend()?;
            print!("\x1b[?25h\nSearch Natures (blank clears): ");
            io::stdout().flush()?;
            let mut search = String::new();
            io::stdin().read_line(&mut search)?;
            state.query.search = (!search.trim().is_empty()).then(|| search.trim().to_owned());
            state.nature_index = 0;
            state.page = Page::Natures;
            state.status = format!(
                "search: {}",
                state.query.search.as_deref().unwrap_or("cleared")
            );
            terminal.resume()?;
            continue;
        }
        handle_key(catalog, deck_state, state_path, &mut state, key);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,
    Tab,
    Search,
    Compare,
    Help,
    Increase,
    Decrease,
    Quit,
    Other,
}

fn read_key(reader: &mut impl Read) -> io::Result<Key> {
    let mut byte = [0_u8; 1];
    reader.read_exact(&mut byte)?;
    Ok(match byte[0] {
        b'k' => Key::Up,
        b'j' => Key::Down,
        b'h' => Key::Back,
        b'l' => Key::Right,
        b'\r' | b'\n' => Key::Enter,
        b'\t' => Key::Tab,
        b'/' => Key::Search,
        b'c' => Key::Compare,
        b'?' => Key::Help,
        b'+' | b'=' => Key::Increase,
        b'-' => Key::Decrease,
        b'q' => Key::Quit,
        0x1b => {
            let mut sequence = [0_u8; 2];
            reader.read_exact(&mut sequence)?;
            match sequence {
                [b'[', b'A'] => Key::Up,
                [b'[', b'B'] => Key::Down,
                [b'[', b'C'] => Key::Right,
                [b'[', b'D'] => Key::Left,
                _ => Key::Other,
            }
        }
        _ => Key::Other,
    })
}

fn handle_key(
    catalog: &HollowDeckCatalog,
    deck_state: &mut HollowDeckState,
    state_path: &Path,
    state: &mut InteractiveState,
    key: Key,
) {
    if key == Key::Help {
        state.page = Page::Help;
        return;
    }
    match state.page {
        Page::Home => match key {
            Key::Up => state.menu_index = state.menu_index.checked_sub(1).unwrap_or(MENU.len() - 1),
            Key::Down => state.menu_index = (state.menu_index + 1) % MENU.len(),
            Key::Enter | Key::Right => state.page = MENU[state.menu_index].1,
            _ => {}
        },
        Page::Natures => {
            let count = state.visible_natures(catalog).len();
            match key {
                Key::Up if count > 0 => {
                    state.nature_index = state.nature_index.checked_sub(1).unwrap_or(count - 1)
                }
                Key::Down if count > 0 => state.nature_index = (state.nature_index + 1) % count,
                Key::Enter | Key::Right if count > 0 => state.page = Page::NatureCard,
                Key::Compare if count > 0 => {
                    if let Some(selected) = state.visible_natures(catalog).get(state.nature_index)
                        && let Some(index) = catalog
                            .natures
                            .iter()
                            .position(|nature| nature.id == selected.id)
                    {
                        state.compare[0] = index;
                        state.compare[1] = (index + 1) % catalog.natures.len();
                        state.page = Page::Compare;
                    }
                }
                Key::Back | Key::Left => state.page = Page::Home,
                _ => {}
            }
        }
        Page::NatureCard => {
            let count = state.visible_natures(catalog).len();
            match key {
                Key::Increase | Key::Decrease if count > 0 => {
                    if let Some(selected) = state.visible_natures(catalog).get(state.nature_index) {
                        let current = deck_state
                            .current_forms
                            .get(&selected.id)
                            .map_or(0, |progress| progress.development_basis_points);
                        let next = if key == Key::Increase {
                            current.saturating_add(500).min(10_000)
                        } else {
                            current.saturating_sub(500)
                        };
                        match deck_state
                            .cultivate(catalog, &selected.id, next)
                            .and_then(|_| save_hollow_deck_state(catalog, deck_state, state_path))
                        {
                            Ok(()) => {
                                state.status = format!(
                                    "{} Current cultivation: {}.{:02}%",
                                    selected.name,
                                    next / 100,
                                    next % 100
                                );
                            }
                            Err(error) => state.status = error.to_string(),
                        }
                    }
                }
                Key::Up if count > 0 => {
                    state.nature_index = state.nature_index.checked_sub(1).unwrap_or(count - 1)
                }
                Key::Down | Key::Right if count > 0 => {
                    state.nature_index = (state.nature_index + 1) % count
                }
                Key::Compare if count > 0 => {
                    if let Some(selected) = state.visible_natures(catalog).get(state.nature_index)
                        && let Some(index) = catalog
                            .natures
                            .iter()
                            .position(|nature| nature.id == selected.id)
                    {
                        state.compare[0] = index;
                        state.page = Page::Compare;
                    }
                }
                Key::Back | Key::Left => state.page = Page::Natures,
                _ => {}
            }
        }
        Page::Compare => match key {
            Key::Tab => state.compare_side = 1 - state.compare_side,
            Key::Up => {
                let side = state.compare_side;
                state.compare[side] = state.compare[side]
                    .checked_sub(1)
                    .unwrap_or(catalog.natures.len() - 1);
            }
            Key::Down | Key::Right => {
                let side = state.compare_side;
                state.compare[side] = (state.compare[side] + 1) % catalog.natures.len();
            }
            Key::Back | Key::Left => state.page = Page::Home,
            _ => {}
        },
        _ => match key {
            Key::Back | Key::Left => state.page = Page::Home,
            Key::Up => state.menu_index = state.menu_index.checked_sub(1).unwrap_or(MENU.len() - 1),
            Key::Down => state.menu_index = (state.menu_index + 1) % MENU.len(),
            Key::Enter | Key::Right => state.page = MENU[state.menu_index].1,
            _ => {}
        },
    }
}

fn draw_interactive(
    catalog: &HollowDeckCatalog,
    deck_state: &HollowDeckState,
    state: &InteractiveState,
) -> io::Result<()> {
    let body = interactive_body(catalog, deck_state, state);
    let accent = state.current_house(catalog).and_then(house_accent);
    let mut stdout = io::stdout().lock();
    write!(stdout, "\x1b[?25l\x1b[2J\x1b[H")?;
    if let Some((red, green, blue)) = accent {
        write!(stdout, "\x1b[38;2;{red};{green};{blue}m")?;
    }
    writeln!(stdout, "{body}\x1b[0m")?;
    writeln!(
        stdout,
        "  {}  │  ↑/k ↓/j  Enter/l open  h/← back  +/- cultivate  / search  c compare  ? help  q quit",
        state.status
    )?;
    stdout.flush()
}

fn interactive_body(
    catalog: &HollowDeckCatalog,
    deck_state: &HollowDeckState,
    state: &InteractiveState,
) -> String {
    match state.page {
        Page::Home => {
            let mut output = render_home(catalog);
            output.push_str("\n\n");
            for (index, (label, _)) in MENU.iter().enumerate() {
                output.push_str(if index == state.menu_index {
                    "  ▸ "
                } else {
                    "    "
                });
                output.push_str(label);
                output.push('\n');
            }
            output
        }
        Page::Houses => render_houses(catalog),
        Page::Institutions => {
            render_institutions(catalog, None).expect("canonical institutions render")
        }
        Page::Glaushouse => render_glaushouse(catalog),
        Page::FlyntIdentities => {
            render_flynt_identities(catalog, None).expect("canonical Flynt identities render")
        }
        Page::Natures => {
            let mut output = render_nature_index(catalog, &state.query);
            if let Some(selected) = state.visible_natures(catalog).get(state.nature_index) {
                output.push_str(&format!("\n\nSelected: {} / Enter for card", selected.name));
            }
            output
        }
        Page::NatureCard => state
            .visible_natures(catalog)
            .get(state.nature_index)
            .and_then(|nature| render_nature_card_with_state(catalog, deck_state, &nature.id).ok())
            .unwrap_or_else(|| render_nature_index(catalog, &state.query)),
        Page::Cultivation => render_cultivation_state(catalog, deck_state),
        Page::Progression => format!(
            "{}\n\n{}\n\n{}",
            render_progression(catalog),
            render_counterparts(catalog),
            render_sandmanor(catalog)
        ),
        Page::CurrentForms => {
            render_current_forms(catalog, None).expect("all Current Forms render")
        }
        Page::AuraSets => render_aura_sets(catalog, None).expect("all Aura Sets render"),
        Page::Traits => render_traits(catalog, None).expect("all Traits render"),
        Page::Compare => {
            let left = &catalog.natures[state.compare[0]].id;
            let right = &catalog.natures[state.compare[1]].id;
            format!(
                "Compare side {} active (Tab switches)\n\n{}",
                state.compare_side + 1,
                render_compare(catalog, left, right).expect("canonical Natures compare")
            )
        }
        Page::Alchemy => render_alchemy(),
        Page::Recipes => render_recipes(catalog),
        Page::SynthesisRecipes => render_synthesis_recipes(catalog),
        Page::Syntheses => render_syntheses(catalog),
        Page::Banes => render_banes(catalog),
        Page::Help => render_help(),
    }
}

fn house_accent(house: House) -> Option<(u8, u8, u8)> {
    canonical_visual_color_constitution()
        .renderer_palette(house)
        .ok()?
        .house_highlight
        .rgb()
        .ok()
        .map(|rgb| (rgb.red, rgb.green, rgb.blue))
}

fn main() -> io::Result<()> {
    let catalog = canonical_hollow_deck();
    if let Err(errors) = validate_hollow_deck(&catalog) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Hollow Deck seed validation failed: {errors:?}"),
        ));
    }
    let cli = parse_cli(std::env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    let state_path: PathBuf = hollow_deck_state_path();
    let mut state = load_hollow_deck_state(&catalog, &state_path)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if cli == DeckCli::Interactive && io::stdin().is_terminal() && io::stdout().is_terminal() {
        run_interactive(&catalog, &mut state, &state_path)
    } else {
        println!(
            "{}",
            run_cli(&catalog, &mut state, &state_path, cli)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_index_filters_and_compare() {
        let cli = parse_cli([
            "natures".into(),
            "--house".into(),
            "Flynt".into(),
            "--principle".into(),
            "Stillness".into(),
            "--sort".into(),
            "total".into(),
        ])
        .expect("filters should parse");
        let DeckCli::Natures(query) = cli else {
            panic!("expected Nature query");
        };
        assert_eq!(query.house.as_deref(), Some("Flynt"));
        assert_eq!(query.principle.as_deref(), Some("Stillness"));
        assert_eq!(query.sort, NatureSort::Total);

        assert_eq!(
            parse_cli(["compare".into(), "gargoyle".into(), "werewolf".into()])
                .expect("compare should parse"),
            DeckCli::Compare("gargoyle".into(), "werewolf".into())
        );
        assert_eq!(
            parse_cli(["glaushouse".into()]).expect("Glaüshouse should parse"),
            DeckCli::Glaushouse
        );
        assert_eq!(
            parse_cli(["muse-seats".into()]).expect("Muse seats alias should parse"),
            DeckCli::Glaushouse
        );
        assert_eq!(
            parse_cli(["institutions".into(), "stonebend".into()])
                .expect("institutional board command should parse"),
            DeckCli::Institutions(Some("stonebend".into()))
        );
        assert_eq!(
            parse_cli(["flynt-identities".into(), "gargoyle".into()])
                .expect("Flynt visual identity command should parse"),
            DeckCli::FlyntIdentities(Some("gargoyle".into()))
        );
        assert_eq!(
            parse_cli(["set-growth".into(), "gargoyle".into(), "Wing Buds".into()])
                .expect("unified growth command should parse"),
            DeckCli::SetGrowth("gargoyle".into(), "Wing Buds".into())
        );
    }

    #[test]
    fn command_views_render_the_deck_language() {
        let catalog = canonical_hollow_deck();
        let mut state = HollowDeckState::default();
        let path = Path::new("unused-in-read-only-tests.json");
        assert!(
            run_cli(&catalog, &mut state, path, DeckCli::Home)
                .unwrap()
                .contains("HOLLOW DECK")
        );
        assert!(
            run_cli(&catalog, &mut state, path, DeckCli::Alchemy)
                .unwrap()
                .contains("Glaüshouse (Jade / Emerald)")
        );
        assert!(
            run_cli(&catalog, &mut state, path, DeckCli::Glaushouse)
                .unwrap()
                .contains("Matron de Melony = Presensor / Jade")
        );
        assert!(
            run_cli(
                &catalog,
                &mut state,
                path,
                DeckCli::Institutions(Some("flynt".into())),
            )
            .unwrap()
            .contains("Hunter + Chief + Mariner → Chimera")
        );
        assert!(
            run_cli(
                &catalog,
                &mut state,
                path,
                DeckCli::FlyntIdentities(Some("werewolf".into())),
            )
            .unwrap()
            .contains("Beauty the Beast")
        );
        assert!(
            run_cli(
                &catalog,
                &mut state,
                path,
                DeckCli::Nature("gargoyle".into()),
            )
            .unwrap()
            .contains("White Opal Statue")
        );
    }
}
