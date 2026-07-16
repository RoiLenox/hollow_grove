use std::collections::BTreeMap;
use std::io;
use std::sync::OnceLock;

use crate::hollow_grove_content::validate_current_synthesis_scenario;

pub const DEFAULT_SCENARIO_ID: &str = "scout_valley_vertical_slice";

const SCOUT_VALLEY_VERTICAL_SLICE_SCENARIO: &str =
    include_str!("../scenarios/current_synthesis/scout_valley_vertical_slice.txt");
const FLOODED_QUARRY_NIGHT_WATCH_SCENARIO: &str =
    include_str!("../scenarios/current_synthesis/flooded_quarry_night_watch.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioNpcDefinition {
    pub id: String,
    pub name: String,
    pub role: String,
    pub faction: String,
    pub location: String,
    pub condition: String,
    pub needs: Vec<String>,
    pub memories: Vec<String>,
    pub relationships: Vec<String>,
    pub perceived_world: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioDefinition {
    pub id: String,
    pub title: String,
    pub default_focused_npc_id: String,
    pub player_need: String,
    pub faction_conditions: Vec<String>,
    pub settlement_conditions: Vec<String>,
    pub war_conditions: Vec<String>,
    pub npcs: Vec<ScenarioNpcDefinition>,
}

#[derive(Debug, Default)]
struct ScenarioNpcBuilder {
    id: String,
    name: String,
    role: String,
    faction: String,
    location: String,
    condition: String,
    needs: Vec<String>,
    memories: Vec<String>,
    relationships: Vec<String>,
    perceived_world: Vec<String>,
}

static SCENARIO_CATALOG: OnceLock<Result<Vec<ScenarioDefinition>, String>> = OnceLock::new();

pub fn load_scenario(id: &str) -> io::Result<ScenarioDefinition> {
    scenario_catalog()?
        .iter()
        .find(|scenario| scenario.id == id)
        .cloned()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported scenario_id: {id}"),
            )
        })
}

pub fn list_scenarios() -> io::Result<Vec<ScenarioDefinition>> {
    Ok(scenario_catalog()?.clone())
}

fn scenario_catalog() -> io::Result<&'static Vec<ScenarioDefinition>> {
    match SCENARIO_CATALOG.get_or_init(|| {
        [
            SCOUT_VALLEY_VERTICAL_SLICE_SCENARIO,
            FLOODED_QUARRY_NIGHT_WATCH_SCENARIO,
        ]
        .into_iter()
        .map(parse_scenario_definition)
        .collect::<io::Result<Vec<_>>>()
        .map_err(|error| error.to_string())
    }) {
        Ok(catalog) => Ok(catalog),
        Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error.clone())),
    }
}

fn parse_scenario_definition(contents: &str) -> io::Result<ScenarioDefinition> {
    let mut id = None;
    let mut title = None;
    let mut default_focused_npc_id = None;
    let mut player_need = None;
    let mut faction_conditions = Vec::new();
    let mut settlement_conditions = Vec::new();
    let mut war_conditions = Vec::new();
    let mut npc_order = Vec::new();
    let mut npc_builders = BTreeMap::<String, ScenarioNpcBuilder>::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("scenario line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "scenario_id" => id = Some(value.to_owned()),
            "title" => title = Some(value.to_owned()),
            "default_focused_npc_id" => default_focused_npc_id = Some(value.to_owned()),
            "player_need" => player_need = Some(value.to_owned()),
            "faction_condition" => faction_conditions.push(value.to_owned()),
            "settlement_condition" => settlement_conditions.push(value.to_owned()),
            "war_condition" => war_conditions.push(value.to_owned()),
            "npc" => {
                let parts = split_npc_parts(value, 6, key)?;
                let npc_id = parts[0].to_owned();
                if npc_builders.contains_key(&npc_id) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("duplicate npc id in scenario: {npc_id}"),
                    ));
                }
                npc_order.push(npc_id.clone());
                npc_builders.insert(
                    npc_id.clone(),
                    ScenarioNpcBuilder {
                        id: npc_id,
                        name: parts[1].to_owned(),
                        role: parts[2].to_owned(),
                        faction: parts[3].to_owned(),
                        location: parts[4].to_owned(),
                        condition: parts[5].to_owned(),
                        ..ScenarioNpcBuilder::default()
                    },
                );
            }
            "npc_need" => {
                let (npc_id, entry) = split_npc_entry(value, key)?;
                npc_builders
                    .get_mut(npc_id)
                    .ok_or_else(|| unknown_npc_error(npc_id, key))?
                    .needs
                    .push(entry.to_owned());
            }
            "npc_memory" => {
                let (npc_id, entry) = split_npc_entry(value, key)?;
                npc_builders
                    .get_mut(npc_id)
                    .ok_or_else(|| unknown_npc_error(npc_id, key))?
                    .memories
                    .push(entry.to_owned());
            }
            "npc_relationship" => {
                let (npc_id, entry) = split_npc_entry(value, key)?;
                npc_builders
                    .get_mut(npc_id)
                    .ok_or_else(|| unknown_npc_error(npc_id, key))?
                    .relationships
                    .push(entry.to_owned());
            }
            "npc_world" => {
                let (npc_id, entry) = split_npc_entry(value, key)?;
                npc_builders
                    .get_mut(npc_id)
                    .ok_or_else(|| unknown_npc_error(npc_id, key))?
                    .perceived_world
                    .push(entry.to_owned());
            }
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown scenario key: {other}"),
                ));
            }
        }
    }

    let id = id.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "scenario missing scenario_id")
    })?;
    let default_focused_npc_id = default_focused_npc_id.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "scenario missing default_focused_npc_id",
        )
    })?;
    let title = title
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "scenario missing title"))?;
    let player_need = player_need.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "scenario missing player_need")
    })?;

    let mut npcs = Vec::with_capacity(npc_order.len());
    for npc_id in npc_order {
        let npc = npc_builders.remove(&npc_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("scenario lost npc definition for {npc_id}"),
            )
        })?;
        npcs.push(ScenarioNpcDefinition {
            id: npc.id,
            name: npc.name,
            role: npc.role,
            faction: npc.faction,
            location: npc.location,
            condition: npc.condition,
            needs: npc.needs,
            memories: npc.memories,
            relationships: npc.relationships,
            perceived_world: npc.perceived_world,
        });
    }

    if npcs.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "scenario must define at least one npc",
        ));
    }
    if !npcs.iter().any(|npc| npc.id == default_focused_npc_id) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "scenario default_focused_npc_id does not match any npc: {default_focused_npc_id}"
            ),
        ));
    }

    let scenario = ScenarioDefinition {
        id,
        title,
        default_focused_npc_id,
        player_need,
        faction_conditions,
        settlement_conditions,
        war_conditions,
        npcs,
    };
    validate_current_synthesis_scenario(&scenario)?;
    Ok(scenario)
}

fn split_npc_parts<'a>(value: &'a str, expected: usize, key: &str) -> io::Result<Vec<&'a str>> {
    let parts = value
        .split('|')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.len() != expected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "{key} expects {expected} pipe-delimited fields, got {}",
                parts.len()
            ),
        ));
    }
    Ok(parts)
}

fn split_npc_entry<'a>(value: &'a str, key: &str) -> io::Result<(&'a str, &'a str)> {
    let parts = split_npc_parts(value, 2, key)?;
    Ok((parts[0], parts[1]))
}

fn unknown_npc_error(npc_id: &str, key: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{key} references unknown npc id: {npc_id}"),
    )
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_SCENARIO_ID, list_scenarios, load_scenario};

    #[test]
    fn default_scenario_loads_multiple_npcs_from_data() {
        let scenario = load_scenario(DEFAULT_SCENARIO_ID).expect("scenario should load");
        assert_eq!(scenario.id, DEFAULT_SCENARIO_ID);
        assert_eq!(scenario.default_focused_npc_id, "route_warden_04");
        assert!(scenario.npcs.len() >= 2);
        assert!(
            scenario
                .npcs
                .iter()
                .any(|npc| npc.id == "shelter_triage_lead_02")
        );
    }

    #[test]
    fn scenario_catalog_lists_multiple_entries() {
        let scenarios = list_scenarios().expect("scenario catalog should load");
        assert!(scenarios.len() >= 2);
        assert!(
            scenarios
                .iter()
                .any(|scenario| scenario.id == "flooded_quarry_night_watch")
        );
    }
}
