use std::io;
use std::path::Path;

use hollow_grove::current_synthesis_engine::{
    CurrentSynthesisState, EngineLens, PersistedCurrentSynthesisState,
    advance_current_synthesis_player_action_at, append_current_synthesis_tick_at,
    append_current_synthesis_ticks_at, build_bond_inspector_output, build_bond_list_output,
    build_bond_trace_output, build_cleopatra_pressures_output, build_cleopatra_status_output,
    build_cleopatra_trace_output, build_engine_output, build_npc_history_output,
    build_npc_inspector_output, build_npc_list_output, build_persisted_state_output,
    build_player_status_output, build_resource_aura_output, build_resource_current_output,
    build_resource_history_output, build_resource_residues_output, build_world_context_output,
    encode_current_synthesis_player_action, focus_current_synthesis_npc_at,
    load_current_synthesis_at, plan_current_synthesis_player_action_at, read_hueman_feedback_at,
    select_current_synthesis_scenario_at, write_view_artifacts,
};
use hollow_grove::current_synthesis_scenario::list_scenarios;
use hollow_grove::hollow_grove_contract::{
    build_hollow_grove_alignment_validation_report, build_hollow_grove_alignment_witness,
};
use hollow_grove::hueman_progression::{VerticalSliceState, write_vertical_slice_artifacts_at};
use hollow_grove::{
    build_being_object_validation_report, build_being_object_witness,
    build_civic_body_validation_report, build_civic_body_witness, build_civic_crisis_witness,
    build_embodied_action_witness, build_flow_glow_validation_report, build_flow_glow_witness,
    build_manager_language_validation_report, build_manager_language_witness,
    build_map_validation_report, build_map_witness, build_move_witness,
    build_player_location_witness, build_point_squared_witness,
    build_progression_validation_report, build_progression_witness,
    build_rule_of_twelve_validation_report, build_rule_of_twelve_witness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum CurrentSynthesisTuiCli {
    Help,
    ScenarioList,
    ScenarioUse(String),
    WorldContext,
    WorldWitness,
    WorldValidate,
    ProgressionWitness,
    ProgressionValidate,
    PointSquaredWitness,
    MapWitness,
    MapValidate,
    RuleOfTwelveWitness,
    RuleOfTwelveValidate,
    ManagerLanguageWitness,
    ManagerLanguageValidate,
    PlayerLocationWitness,
    BeingObjectWitness,
    BeingObjectValidate,
    MoveWitness,
    CivicBodyWitness,
    CivicBodyValidate,
    CivicCrisisWitness,
    FlowGlowWitness,
    FlowGlowValidate,
    EmbodiedActionWitness,
    Engine(EngineLens),
    BondList,
    BondInspect(String),
    BondTrace(String),
    ResourceAura,
    ResourceCurrent,
    ResourceResidues,
    ResourceHistory,
    PlayerStatus,
    PlayerPlan(String),
    PlayerMove(String),
    PlayerDecide(String),
    PlayerSupport(String),
    NpcList,
    NpcFocus(String),
    NpcInspect(String),
    NpcBlep(String),
    NpcHistory(String),
    CleopatraStatus,
    CleopatraTick(Option<String>),
    CleopatraRun(usize, Option<String>),
    CleopatraTrace(String),
    CleopatraPressures,
}

fn parse_cli<I>(args: I) -> Result<CurrentSynthesisTuiCli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(section) = args.next() else {
        return Ok(CurrentSynthesisTuiCli::Engine(EngineLens::Status));
    };

    match section.as_str() {
        "--help" | "-h" | "help" => Ok(CurrentSynthesisTuiCli::Help),
        "scenario" => match args.next().as_deref() {
            Some("list") => {
                require_no_extra(args, CurrentSynthesisTuiCli::ScenarioList, "scenario list")
            }
            Some("use") => {
                let scenario_id = args
                    .next()
                    .ok_or_else(|| String::from("scenario use requires <scenario-id>"))?;
                require_no_extra(
                    args,
                    CurrentSynthesisTuiCli::ScenarioUse(scenario_id),
                    "scenario use",
                )
            }
            Some(other) => Err(format!("unknown scenario command: {other}")),
            None => Err(String::from("scenario requires list or use")),
        },
        "world" => match args.next().as_deref() {
            Some("context") => {
                require_no_extra(args, CurrentSynthesisTuiCli::WorldContext, "world context")
            }
            Some("witness") => {
                require_no_extra(args, CurrentSynthesisTuiCli::WorldWitness, "world witness")
            }
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::WorldValidate,
                "world validate",
            ),
            Some(other) => Err(format!("unknown world command: {other}")),
            None => Err(String::from("world requires context, witness, or validate")),
        },
        "progression" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ProgressionWitness,
                "progression witness",
            ),
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ProgressionValidate,
                "progression validate",
            ),
            Some(other) => Err(format!("unknown progression command: {other}")),
            None => Err(String::from("progression requires witness or validate")),
        },
        "point-squared" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::PointSquaredWitness,
                "point-squared witness",
            ),
            Some(other) => Err(format!("unknown point-squared command: {other}")),
            None => Err(String::from("point-squared requires witness")),
        },
        "map" => match args.next().as_deref() {
            Some("witness") => {
                require_no_extra(args, CurrentSynthesisTuiCli::MapWitness, "map witness")
            }
            Some("validate") => {
                require_no_extra(args, CurrentSynthesisTuiCli::MapValidate, "map validate")
            }
            Some(other) => Err(format!("unknown map command: {other}")),
            None => Err(String::from("map requires witness or validate")),
        },
        "rule-of-twelve" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::RuleOfTwelveWitness,
                "rule-of-twelve witness",
            ),
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::RuleOfTwelveValidate,
                "rule-of-twelve validate",
            ),
            Some(other) => Err(format!("unknown rule-of-twelve command: {other}")),
            None => Err(String::from("rule-of-twelve requires witness or validate")),
        },
        "manager-language" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ManagerLanguageWitness,
                "manager-language witness",
            ),
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ManagerLanguageValidate,
                "manager-language validate",
            ),
            Some(other) => Err(format!("unknown manager-language command: {other}")),
            None => Err(String::from(
                "manager-language requires witness or validate",
            )),
        },
        "player-location" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::PlayerLocationWitness,
                "player-location witness",
            ),
            Some(other) => Err(format!("unknown player-location command: {other}")),
            None => Err(String::from("player-location requires witness")),
        },
        "being-object" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::BeingObjectWitness,
                "being-object witness",
            ),
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::BeingObjectValidate,
                "being-object validate",
            ),
            Some(other) => Err(format!("unknown being-object command: {other}")),
            None => Err(String::from("being-object requires witness or validate")),
        },
        "move" => match args.next().as_deref() {
            Some("witness") => {
                require_no_extra(args, CurrentSynthesisTuiCli::MoveWitness, "move witness")
            }
            Some(other) => Err(format!("unknown move command: {other}")),
            None => Err(String::from("move requires witness")),
        },
        "civic-body" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::CivicBodyWitness,
                "civic-body witness",
            ),
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::CivicBodyValidate,
                "civic-body validate",
            ),
            Some(other) => Err(format!("unknown civic-body command: {other}")),
            None => Err(String::from("civic-body requires witness or validate")),
        },
        "civic-crisis" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::CivicCrisisWitness,
                "civic-crisis witness",
            ),
            Some(other) => Err(format!("unknown civic-crisis command: {other}")),
            None => Err(String::from("civic-crisis requires witness")),
        },
        "flow-glow" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::FlowGlowWitness,
                "flow-glow witness",
            ),
            Some("validate") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::FlowGlowValidate,
                "flow-glow validate",
            ),
            Some(other) => Err(format!("unknown flow-glow command: {other}")),
            None => Err(String::from("flow-glow requires witness or validate")),
        },
        "embodied-action" => match args.next().as_deref() {
            Some("witness") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::EmbodiedActionWitness,
                "embodied-action witness",
            ),
            Some(other) => Err(format!("unknown embodied-action command: {other}")),
            None => Err(String::from("embodied-action requires witness")),
        },
        "engine" => {
            let lens = args.next().unwrap_or_else(|| String::from("status"));
            if args.next().is_some() {
                return Err(String::from("engine accepts at most one view argument"));
            }
            let lens = EngineLens::from_str(&lens)
                .ok_or_else(|| format!("unknown engine view: {lens}"))?;
            Ok(CurrentSynthesisTuiCli::Engine(lens))
        }
        "bond" => match args.next().as_deref() {
            Some("list") => require_no_extra(args, CurrentSynthesisTuiCli::BondList, "bond list"),
            Some("inspect") => {
                let bond_id = args
                    .next()
                    .ok_or_else(|| String::from("bond inspect requires <id>"))?;
                require_no_extra(
                    args,
                    CurrentSynthesisTuiCli::BondInspect(bond_id),
                    "bond inspect",
                )
            }
            Some("trace") => {
                let moment_id = args
                    .next()
                    .ok_or_else(|| String::from("bond trace requires <moment-id>"))?;
                require_no_extra(
                    args,
                    CurrentSynthesisTuiCli::BondTrace(moment_id),
                    "bond trace",
                )
            }
            Some(other) => Err(format!("unknown bond command: {other}")),
            None => Err(String::from("bond requires list, inspect, or trace")),
        },
        "resource" => match args.next().as_deref() {
            Some("aura") => {
                require_no_extra(args, CurrentSynthesisTuiCli::ResourceAura, "resource aura")
            }
            Some("current") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ResourceCurrent,
                "resource current",
            ),
            Some("residues") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ResourceResidues,
                "resource residues",
            ),
            Some("history") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::ResourceHistory,
                "resource history",
            ),
            Some(other) => Err(format!("unknown resource command: {other}")),
            None => Err(String::from(
                "resource requires aura, current, residues, or history",
            )),
        },
        "player" => match args.next().as_deref() {
            Some("status") => {
                require_no_extra(args, CurrentSynthesisTuiCli::PlayerStatus, "player status")
            }
            Some("plan") => {
                parse_player_action(args, "player plan", CurrentSynthesisTuiCli::PlayerPlan)
            }
            Some("move") => {
                parse_player_action(args, "player move", CurrentSynthesisTuiCli::PlayerMove)
            }
            Some("decide") => {
                parse_player_action(args, "player decide", CurrentSynthesisTuiCli::PlayerDecide)
            }
            Some("support") => parse_player_action(
                args,
                "player support",
                CurrentSynthesisTuiCli::PlayerSupport,
            ),
            Some(other) => Err(format!("unknown player command: {other}")),
            None => Err(String::from(
                "player requires status, plan, move, decide, or support",
            )),
        },
        "npc" => match args.next().as_deref() {
            Some("list") => require_no_extra(args, CurrentSynthesisTuiCli::NpcList, "npc list"),
            Some("focus") => {
                let npc_id = args
                    .next()
                    .ok_or_else(|| String::from("npc focus requires <npc-id>"))?;
                require_no_extra(args, CurrentSynthesisTuiCli::NpcFocus(npc_id), "npc focus")
            }
            Some("inspect") => {
                let npc_id = args
                    .next()
                    .ok_or_else(|| String::from("npc inspect requires <npc-id>"))?;
                require_no_extra(
                    args,
                    CurrentSynthesisTuiCli::NpcInspect(npc_id),
                    "npc inspect",
                )
            }
            Some("blep") => {
                let npc_id = args
                    .next()
                    .ok_or_else(|| String::from("npc blep requires <npc-id>"))?;
                require_no_extra(args, CurrentSynthesisTuiCli::NpcBlep(npc_id), "npc blep")
            }
            Some("history") => {
                let npc_id = args
                    .next()
                    .ok_or_else(|| String::from("npc history requires <npc-id>"))?;
                require_no_extra(
                    args,
                    CurrentSynthesisTuiCli::NpcHistory(npc_id),
                    "npc history",
                )
            }
            Some(other) => Err(format!("unknown npc command: {other}")),
            None => Err(String::from("npc requires list, inspect, blep, or history")),
        },
        "cleopatra" => match args.next().as_deref() {
            Some("status") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::CleopatraStatus,
                "cleopatra status",
            ),
            Some("tick") => {
                let npc_id = args.next();
                if let Some(extra) = args.next() {
                    return Err(format!(
                        "cleopatra tick received unexpected extra argument: {extra}"
                    ));
                }
                Ok(CurrentSynthesisTuiCli::CleopatraTick(npc_id))
            }
            Some("run") => {
                let count = args
                    .next()
                    .ok_or_else(|| String::from("cleopatra run requires <count>"))?;
                let count = parse_tick_count(&count)?;
                let npc_id = args.next();
                if let Some(extra) = args.next() {
                    return Err(format!(
                        "cleopatra run received unexpected extra argument: {extra}"
                    ));
                }
                Ok(CurrentSynthesisTuiCli::CleopatraRun(count, npc_id))
            }
            Some("trace") => {
                let npc_id = args
                    .next()
                    .ok_or_else(|| String::from("cleopatra trace requires <npc-id>"))?;
                require_no_extra(
                    args,
                    CurrentSynthesisTuiCli::CleopatraTrace(npc_id),
                    "cleopatra trace",
                )
            }
            Some("pressures") => require_no_extra(
                args,
                CurrentSynthesisTuiCli::CleopatraPressures,
                "cleopatra pressures",
            ),
            Some(other) => Err(format!("unknown cleopatra command: {other}")),
            None => Err(String::from(
                "cleopatra requires status, tick, run, trace, or pressures",
            )),
        },
        other => Err(format!("unknown current synthesis command: {other}")),
    }
}

fn parse_tick_count(raw: &str) -> Result<usize, String> {
    let count = raw
        .parse::<usize>()
        .map_err(|_| format!("invalid tick count: {raw}"))?;
    if count == 0 {
        Err(String::from("tick count must be greater than zero"))
    } else {
        Ok(count)
    }
}

fn require_no_extra(
    mut args: impl Iterator<Item = String>,
    cli: CurrentSynthesisTuiCli,
    command: &str,
) -> Result<CurrentSynthesisTuiCli, String> {
    if let Some(extra) = args.next() {
        Err(format!(
            "{command} does not accept additional arguments: {extra}"
        ))
    } else {
        Ok(cli)
    }
}

fn parse_player_action(
    args: impl Iterator<Item = String>,
    command: &str,
    ctor: fn(String) -> CurrentSynthesisTuiCli,
) -> Result<CurrentSynthesisTuiCli, String> {
    let action = args.collect::<Vec<_>>().join(" ");
    if action.trim().is_empty() {
        Err(format!("{command} requires <action-text>"))
    } else {
        Ok(ctor(action))
    }
}

fn usage() -> &'static str {
    "Usage: current_synthesis_tui <scenario|world|progression|point-squared|map|rule-of-twelve|manager-language|player-location|being-object|move|civic-body|civic-crisis|flow-glow|embodied-action|engine|bond|resource|player|npc|cleopatra> [args]\n\
     \n\
     Commands:\n\
       scenario list\n\
       scenario use <scenario-id>\n\
       world context\n\
       world witness\n\
       world validate\n\
       progression witness\n\
       progression validate\n\
       point-squared witness\n\
       map witness\n\
       map validate\n\
       rule-of-twelve witness\n\
       rule-of-twelve validate\n\
       manager-language witness\n\
       manager-language validate\n\
       player-location witness\n\
       being-object witness\n\
       being-object validate\n\
       move witness\n\
       civic-body witness\n\
       civic-body validate\n\
       civic-crisis witness\n\
       flow-glow witness\n\
       flow-glow validate\n\
       embodied-action witness\n\
       engine status|pleb|meta|blep\n\
       bond list\n\
       bond inspect <id>\n\
       bond trace <moment-id>\n\
       resource aura\n\
       resource current\n\
       resource residues\n\
       resource history\n\
       player status\n\
       player plan <action-text>\n\
       player move <action-text>\n\
       player decide <action-text>\n\
       player support <action-text>\n\
       npc list\n\
       npc focus <npc-id>\n\
       npc inspect <npc-id>\n\
       npc blep <npc-id>\n\
       npc history <npc-id>\n\
       cleopatra status\n\
       cleopatra tick [npc-id]\n\
       cleopatra run <count> [npc-id]\n\
       cleopatra trace <npc-id>\n\
       cleopatra pressures\n\
       \n\
     Action text accepts optional compact fields.\n\
     move: from=stonebend to=sandmanor line=aura-way pace=fast method=scout stance=quiet\n\
     decide: focus=alliance commitment=commit authority=shared signal=public\n\
     support: asset=pump beneficiary=sandmanor front=power intensity=heavy duration=hold\n\
       help"
}

fn sync_hueman_slice_for_current_synthesis_scenario(
    root: &Path,
    scenario_id: &str,
) -> io::Result<()> {
    let state = VerticalSliceState::for_current_synthesis_scenario(scenario_id)?;
    let _ = write_vertical_slice_artifacts_at(root, &state)?;
    Ok(())
}

fn load_state(root: &Path) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let hueman_feedback = read_hueman_feedback_at(root)?;
    load_current_synthesis_at(root, hueman_feedback)
}

fn tick_state(
    root: &Path,
    npc_id: Option<&str>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let hueman_feedback = read_hueman_feedback_at(root)?;
    let (persisted, state) = append_current_synthesis_tick_at(root, npc_id, hueman_feedback)?;
    write_view_artifacts(root, &persisted, &state)?;
    Ok((persisted, state))
}

fn advance_player_action_state(
    root: &Path,
    action_kind: &str,
    action_label: &str,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let hueman_feedback = read_hueman_feedback_at(root)?;
    let encoded = encode_current_synthesis_player_action(action_kind, action_label)?;
    let (persisted, state) =
        advance_current_synthesis_player_action_at(root, &encoded, None, hueman_feedback)?;
    write_view_artifacts(root, &persisted, &state)?;
    Ok((persisted, state))
}

fn run_cli(root: &Path, cli: CurrentSynthesisTuiCli) -> io::Result<String> {
    match cli {
        CurrentSynthesisTuiCli::Help => Ok(String::from(usage())),
        CurrentSynthesisTuiCli::ScenarioList => {
            let scenarios = list_scenarios()?;
            let mut output = String::from("# Scenario List\n\n");
            for scenario in scenarios {
                let marker = if scenario.id == PersistedCurrentSynthesisState::primary().scenario_id
                {
                    " [default]"
                } else {
                    ""
                };
                output.push_str(&format!(
                    "- {} (`{}`) npcs={} need={}{}\n",
                    scenario.title,
                    scenario.id,
                    scenario.npcs.len(),
                    scenario.player_need,
                    marker
                ));
            }
            Ok(output)
        }
        CurrentSynthesisTuiCli::ScenarioUse(scenario_id) => {
            sync_hueman_slice_for_current_synthesis_scenario(root, &scenario_id)?;
            let hueman_feedback = read_hueman_feedback_at(root)?;
            let (persisted, state) =
                select_current_synthesis_scenario_at(root, &scenario_id, hueman_feedback)?;
            write_view_artifacts(root, &persisted, &state)?;
            Ok(format!(
                "# Scenario Selected\n\n\
                 - scenario: `{}`\n\
                 - focused npc: `{}`\n\
                 - completed ticks: {}\n\n\
                 {}",
                persisted.scenario_id,
                persisted.focused_npc_id,
                persisted.completed_ticks,
                build_persisted_state_output(&persisted)
            ))
        }
        CurrentSynthesisTuiCli::WorldContext => Ok(build_world_context_output()),
        CurrentSynthesisTuiCli::WorldWitness => Ok(build_hollow_grove_alignment_witness()),
        CurrentSynthesisTuiCli::WorldValidate => {
            Ok(build_hollow_grove_alignment_validation_report())
        }
        CurrentSynthesisTuiCli::ProgressionWitness => build_progression_witness(),
        CurrentSynthesisTuiCli::ProgressionValidate => build_progression_validation_report(),
        CurrentSynthesisTuiCli::PointSquaredWitness => build_point_squared_witness(),
        CurrentSynthesisTuiCli::MapWitness => build_map_witness(),
        CurrentSynthesisTuiCli::MapValidate => build_map_validation_report(),
        CurrentSynthesisTuiCli::RuleOfTwelveWitness => build_rule_of_twelve_witness(),
        CurrentSynthesisTuiCli::RuleOfTwelveValidate => build_rule_of_twelve_validation_report(),
        CurrentSynthesisTuiCli::ManagerLanguageWitness => Ok(build_manager_language_witness()),
        CurrentSynthesisTuiCli::ManagerLanguageValidate => {
            Ok(build_manager_language_validation_report())
        }
        CurrentSynthesisTuiCli::PlayerLocationWitness => build_player_location_witness(),
        CurrentSynthesisTuiCli::BeingObjectWitness => build_being_object_witness(),
        CurrentSynthesisTuiCli::BeingObjectValidate => build_being_object_validation_report(),
        CurrentSynthesisTuiCli::MoveWitness => build_move_witness(),
        CurrentSynthesisTuiCli::CivicBodyWitness => build_civic_body_witness(),
        CurrentSynthesisTuiCli::CivicBodyValidate => build_civic_body_validation_report(),
        CurrentSynthesisTuiCli::CivicCrisisWitness => build_civic_crisis_witness(),
        CurrentSynthesisTuiCli::FlowGlowWitness => build_flow_glow_witness(),
        CurrentSynthesisTuiCli::FlowGlowValidate => build_flow_glow_validation_report(),
        CurrentSynthesisTuiCli::EmbodiedActionWitness => build_embodied_action_witness(),
        CurrentSynthesisTuiCli::Engine(lens) => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_engine_output(&state, lens))
        }
        CurrentSynthesisTuiCli::BondList => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_bond_list_output(&state))
        }
        CurrentSynthesisTuiCli::BondInspect(bond_id) => {
            let (_persisted, state) = load_state(root)?;
            build_bond_inspector_output(&state, &bond_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("unknown bond id: {bond_id}"),
                )
            })
        }
        CurrentSynthesisTuiCli::BondTrace(moment_id) => {
            let (_persisted, state) = load_state(root)?;
            build_bond_trace_output(&state, &moment_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("unknown moment id: {moment_id}"),
                )
            })
        }
        CurrentSynthesisTuiCli::ResourceAura => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_resource_aura_output(&state))
        }
        CurrentSynthesisTuiCli::ResourceCurrent => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_resource_current_output(&state))
        }
        CurrentSynthesisTuiCli::ResourceResidues => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_resource_residues_output(&state))
        }
        CurrentSynthesisTuiCli::ResourceHistory => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_resource_history_output(&state))
        }
        CurrentSynthesisTuiCli::PlayerStatus => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_player_status_output(&state))
        }
        CurrentSynthesisTuiCli::PlayerPlan(action_label) => {
            let hueman_feedback = read_hueman_feedback_at(root)?;
            let (persisted, state) =
                plan_current_synthesis_player_action_at(root, &action_label, hueman_feedback)?;
            write_view_artifacts(root, &persisted, &state)?;
            Ok(format!(
                "# Player Action Planned\n\n\
                 - action: {}\n\
                 - pending count: {}\n",
                action_label,
                state.planned_player_actions().len()
            ))
        }
        CurrentSynthesisTuiCli::PlayerMove(action_label) => {
            let (persisted, state) = advance_player_action_state(root, "move", &action_label)?;
            Ok(format!(
                "# Player Move Advanced\n\n\
                 - action: {}\n\
                 - completed ticks: {}\n\
                 - focused npc: `{}`\n\n\
                 {}\n\n## Persisted State\n\n{}",
                action_label,
                state.completed_ticks(),
                state.focused_npc_id(),
                build_engine_output(&state, EngineLens::Status),
                build_persisted_state_output(&persisted)
            ))
        }
        CurrentSynthesisTuiCli::PlayerDecide(action_label) => {
            let (persisted, state) = advance_player_action_state(root, "decide", &action_label)?;
            Ok(format!(
                "# Player Decision Advanced\n\n\
                 - action: {}\n\
                 - completed ticks: {}\n\
                 - focused npc: `{}`\n\n\
                 {}\n\n## Persisted State\n\n{}",
                action_label,
                state.completed_ticks(),
                state.focused_npc_id(),
                build_engine_output(&state, EngineLens::Status),
                build_persisted_state_output(&persisted)
            ))
        }
        CurrentSynthesisTuiCli::PlayerSupport(action_label) => {
            let (persisted, state) = advance_player_action_state(root, "support", &action_label)?;
            Ok(format!(
                "# Player Support Advanced\n\n\
                 - action: {}\n\
                 - completed ticks: {}\n\
                 - focused npc: `{}`\n\n\
                 {}\n\n## Persisted State\n\n{}",
                action_label,
                state.completed_ticks(),
                state.focused_npc_id(),
                build_engine_output(&state, EngineLens::Status),
                build_persisted_state_output(&persisted)
            ))
        }
        CurrentSynthesisTuiCli::NpcList => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_npc_list_output(&state))
        }
        CurrentSynthesisTuiCli::NpcFocus(npc_id) => {
            let hueman_feedback = read_hueman_feedback_at(root)?;
            let (persisted, state) =
                focus_current_synthesis_npc_at(root, &npc_id, hueman_feedback)?;
            write_view_artifacts(root, &persisted, &state)?;
            Ok(format!(
                "# NPC Focus Updated\n\n\
                 - focused npc: `{}`\n",
                state.focused_npc_id()
            ))
        }
        CurrentSynthesisTuiCli::NpcInspect(npc_id) | CurrentSynthesisTuiCli::NpcBlep(npc_id) => {
            let (_persisted, state) = load_state(root)?;
            build_npc_inspector_output(&state, &npc_id).ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, format!("unknown npc id: {npc_id}"))
            })
        }
        CurrentSynthesisTuiCli::NpcHistory(npc_id) => {
            let (_persisted, state) = load_state(root)?;
            build_npc_history_output(&state, &npc_id).ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, format!("unknown npc id: {npc_id}"))
            })
        }
        CurrentSynthesisTuiCli::CleopatraStatus => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_cleopatra_status_output(&state))
        }
        CurrentSynthesisTuiCli::CleopatraTick(npc_id) => {
            let (persisted, state) = tick_state(root, npc_id.as_deref())?;
            Ok(format!(
                "{}\n\n## Persisted State\n\n{}",
                build_cleopatra_status_output(&state),
                build_persisted_state_output(&persisted)
            ))
        }
        CurrentSynthesisTuiCli::CleopatraRun(count, npc_id) => {
            let hueman_feedback = read_hueman_feedback_at(root)?;
            let (persisted, state) =
                append_current_synthesis_ticks_at(root, npc_id.as_deref(), count, hueman_feedback)?;
            write_view_artifacts(root, &persisted, &state)?;
            Ok(format!(
                "# Cleopatra Run Complete\n\n\
                 - ticks applied: {}\n\
                 - focused npc: `{}`\n\
                 - completed ticks: {}\n\n\
                 {}\n\n## Persisted State\n\n{}",
                count,
                state.focused_npc_id(),
                state.completed_ticks(),
                build_cleopatra_status_output(&state),
                build_persisted_state_output(&persisted)
            ))
        }
        CurrentSynthesisTuiCli::CleopatraTrace(npc_id) => {
            let (_persisted, state) = load_state(root)?;
            build_cleopatra_trace_output(&state, &npc_id).ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, format!("unknown npc id: {npc_id}"))
            })
        }
        CurrentSynthesisTuiCli::CleopatraPressures => {
            let (_persisted, state) = load_state(root)?;
            Ok(build_cleopatra_pressures_output(&state))
        }
    }
}

fn main() -> io::Result<()> {
    let cli = parse_cli(std::env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    println!("{}", run_cli(Path::new("."), cli)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{CurrentSynthesisTuiCli, parse_cli, run_cli, tick_state, usage};

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn parser_accepts_engine_and_cleopatra_commands() {
        assert_eq!(
            parse_cli([String::from("scenario"), String::from("list")])
                .expect("scenario list should parse"),
            CurrentSynthesisTuiCli::ScenarioList
        );
        assert_eq!(
            parse_cli([String::from("world"), String::from("context")])
                .expect("world context should parse"),
            CurrentSynthesisTuiCli::WorldContext
        );
        assert_eq!(
            parse_cli([String::from("world"), String::from("witness")])
                .expect("world witness should parse"),
            CurrentSynthesisTuiCli::WorldWitness
        );
        assert_eq!(
            parse_cli([String::from("world"), String::from("validate")])
                .expect("world validate should parse"),
            CurrentSynthesisTuiCli::WorldValidate
        );
        assert_eq!(
            parse_cli([String::from("progression"), String::from("witness")])
                .expect("progression witness should parse"),
            CurrentSynthesisTuiCli::ProgressionWitness
        );
        assert_eq!(
            parse_cli([String::from("progression"), String::from("validate")])
                .expect("progression validate should parse"),
            CurrentSynthesisTuiCli::ProgressionValidate
        );
        assert_eq!(
            parse_cli([String::from("point-squared"), String::from("witness")])
                .expect("point-squared witness should parse"),
            CurrentSynthesisTuiCli::PointSquaredWitness
        );
        assert_eq!(
            parse_cli([String::from("map"), String::from("witness")])
                .expect("map witness should parse"),
            CurrentSynthesisTuiCli::MapWitness
        );
        assert_eq!(
            parse_cli([String::from("map"), String::from("validate")])
                .expect("map validate should parse"),
            CurrentSynthesisTuiCli::MapValidate
        );
        assert_eq!(
            parse_cli([String::from("rule-of-twelve"), String::from("witness")])
                .expect("rule-of-twelve witness should parse"),
            CurrentSynthesisTuiCli::RuleOfTwelveWitness
        );
        assert_eq!(
            parse_cli([String::from("rule-of-twelve"), String::from("validate")])
                .expect("rule-of-twelve validate should parse"),
            CurrentSynthesisTuiCli::RuleOfTwelveValidate
        );
        assert_eq!(
            parse_cli([String::from("manager-language"), String::from("witness")])
                .expect("manager-language witness should parse"),
            CurrentSynthesisTuiCli::ManagerLanguageWitness
        );
        assert_eq!(
            parse_cli([String::from("manager-language"), String::from("validate")])
                .expect("manager-language validate should parse"),
            CurrentSynthesisTuiCli::ManagerLanguageValidate
        );
        assert_eq!(
            parse_cli([String::from("player-location"), String::from("witness")])
                .expect("player-location witness should parse"),
            CurrentSynthesisTuiCli::PlayerLocationWitness
        );
        assert_eq!(
            parse_cli([String::from("being-object"), String::from("witness")])
                .expect("being-object witness should parse"),
            CurrentSynthesisTuiCli::BeingObjectWitness
        );
        assert_eq!(
            parse_cli([String::from("being-object"), String::from("validate")])
                .expect("being-object validate should parse"),
            CurrentSynthesisTuiCli::BeingObjectValidate
        );
        assert_eq!(
            parse_cli([String::from("move"), String::from("witness")])
                .expect("move witness should parse"),
            CurrentSynthesisTuiCli::MoveWitness
        );
        assert_eq!(
            parse_cli([String::from("civic-body"), String::from("witness")])
                .expect("civic-body witness should parse"),
            CurrentSynthesisTuiCli::CivicBodyWitness
        );
        assert_eq!(
            parse_cli([String::from("civic-body"), String::from("validate")])
                .expect("civic-body validate should parse"),
            CurrentSynthesisTuiCli::CivicBodyValidate
        );
        assert_eq!(
            parse_cli([String::from("civic-crisis"), String::from("witness")])
                .expect("civic-crisis witness should parse"),
            CurrentSynthesisTuiCli::CivicCrisisWitness
        );
        assert_eq!(
            parse_cli([String::from("flow-glow"), String::from("witness")])
                .expect("flow-glow witness should parse"),
            CurrentSynthesisTuiCli::FlowGlowWitness
        );
        assert_eq!(
            parse_cli([String::from("flow-glow"), String::from("validate")])
                .expect("flow-glow validate should parse"),
            CurrentSynthesisTuiCli::FlowGlowValidate
        );
        assert_eq!(
            parse_cli([String::from("embodied-action"), String::from("witness")])
                .expect("embodied-action witness should parse"),
            CurrentSynthesisTuiCli::EmbodiedActionWitness
        );
        assert_eq!(
            parse_cli([String::from("engine"), String::from("blep")]).expect("engine should parse"),
            CurrentSynthesisTuiCli::Engine(
                hollow_grove::current_synthesis_engine::EngineLens::Blep
            )
        );
        assert_eq!(
            parse_cli([String::from("bond"), String::from("list")]).expect("bond should parse"),
            CurrentSynthesisTuiCli::BondList
        );
        assert_eq!(
            parse_cli([
                String::from("player"),
                String::from("plan"),
                String::from("brace"),
                String::from("the"),
                String::from("intake"),
                String::from("ladder"),
            ])
            .expect("player plan should parse"),
            CurrentSynthesisTuiCli::PlayerPlan(String::from("brace the intake ladder"))
        );
        assert_eq!(
            parse_cli([
                String::from("player"),
                String::from("move"),
                String::from("cross"),
                String::from("the"),
                String::from("flooded"),
                String::from("rim"),
            ])
            .expect("player move should parse"),
            CurrentSynthesisTuiCli::PlayerMove(String::from("cross the flooded rim"))
        );
        assert_eq!(
            parse_cli([
                String::from("player"),
                String::from("decide"),
                String::from("signal"),
                String::from("the"),
                String::from("upper"),
                String::from("crew"),
            ])
            .expect("player decide should parse"),
            CurrentSynthesisTuiCli::PlayerDecide(String::from("signal the upper crew"))
        );
        assert_eq!(
            parse_cli([
                String::from("npc"),
                String::from("focus"),
                String::from("route_warden_04")
            ])
            .expect("npc focus should parse"),
            CurrentSynthesisTuiCli::NpcFocus(String::from("route_warden_04"))
        );
        assert_eq!(
            parse_cli([
                String::from("cleopatra"),
                String::from("tick"),
                String::from("route_warden_04")
            ])
            .expect("tick should parse"),
            CurrentSynthesisTuiCli::CleopatraTick(Some(String::from("route_warden_04")))
        );
        assert_eq!(
            parse_cli([
                String::from("cleopatra"),
                String::from("run"),
                String::from("3"),
                String::from("route_warden_04")
            ])
            .expect("run should parse"),
            CurrentSynthesisTuiCli::CleopatraRun(3, Some(String::from("route_warden_04")))
        );
    }

    #[test]
    fn usage_mentions_inspection_commands() {
        let usage = usage();
        assert!(usage.contains("scenario list"));
        assert!(usage.contains("scenario use <scenario-id>"));
        assert!(usage.contains("world context"));
        assert!(usage.contains("world witness"));
        assert!(usage.contains("world validate"));
        assert!(usage.contains("progression witness"));
        assert!(usage.contains("progression validate"));
        assert!(usage.contains("point-squared witness"));
        assert!(usage.contains("map witness"));
        assert!(usage.contains("map validate"));
        assert!(usage.contains("rule-of-twelve witness"));
        assert!(usage.contains("rule-of-twelve validate"));
        assert!(usage.contains("manager-language witness"));
        assert!(usage.contains("manager-language validate"));
        assert!(usage.contains("player-location witness"));
        assert!(usage.contains("being-object witness"));
        assert!(usage.contains("being-object validate"));
        assert!(usage.contains("move witness"));
        assert!(usage.contains("civic-body witness"));
        assert!(usage.contains("civic-body validate"));
        assert!(usage.contains("civic-crisis witness"));
        assert!(usage.contains("flow-glow witness"));
        assert!(usage.contains("flow-glow validate"));
        assert!(usage.contains("embodied-action witness"));
        assert!(usage.contains("engine status|pleb|meta|blep"));
        assert!(usage.contains("bond inspect <id>"));
        assert!(usage.contains("resource history"));
        assert!(usage.contains("player plan <action-text>"));
        assert!(usage.contains("player move <action-text>"));
        assert!(usage.contains("player decide <action-text>"));
        assert!(usage.contains("player support <action-text>"));
        assert!(usage.contains("npc focus <npc-id>"));
        assert!(usage.contains("cleopatra tick [npc-id]"));
        assert!(usage.contains("cleopatra run <count> [npc-id]"));
    }

    #[test]
    fn command_surface_bootstraps_and_renders_views() {
        let root = unique_temp_dir("current-synthesis-tui-run");
        let scenario_list = run_cli(&root, CurrentSynthesisTuiCli::ScenarioList)
            .expect("scenario list should succeed");
        let world_context = run_cli(&root, CurrentSynthesisTuiCli::WorldContext)
            .expect("world context should succeed");
        let world_witness = run_cli(&root, CurrentSynthesisTuiCli::WorldWitness)
            .expect("world witness should succeed");
        let world_validate = run_cli(&root, CurrentSynthesisTuiCli::WorldValidate)
            .expect("world validate should succeed");
        let progression_witness = run_cli(&root, CurrentSynthesisTuiCli::ProgressionWitness)
            .expect("progression witness should succeed");
        let progression_validate = run_cli(&root, CurrentSynthesisTuiCli::ProgressionValidate)
            .expect("progression validate should succeed");
        let point_squared_witness = run_cli(&root, CurrentSynthesisTuiCli::PointSquaredWitness)
            .expect("point-squared witness should succeed");
        let map_witness =
            run_cli(&root, CurrentSynthesisTuiCli::MapWitness).expect("map witness should succeed");
        let map_validate = run_cli(&root, CurrentSynthesisTuiCli::MapValidate)
            .expect("map validate should succeed");
        let rule_of_twelve_witness = run_cli(&root, CurrentSynthesisTuiCli::RuleOfTwelveWitness)
            .expect("rule-of-twelve witness should succeed");
        let rule_of_twelve_validate = run_cli(&root, CurrentSynthesisTuiCli::RuleOfTwelveValidate)
            .expect("rule-of-twelve validate should succeed");
        let manager_language_witness =
            run_cli(&root, CurrentSynthesisTuiCli::ManagerLanguageWitness)
                .expect("manager-language witness should succeed");
        let manager_language_validate =
            run_cli(&root, CurrentSynthesisTuiCli::ManagerLanguageValidate)
                .expect("manager-language validate should succeed");
        let player_location_witness = run_cli(&root, CurrentSynthesisTuiCli::PlayerLocationWitness)
            .expect("player-location witness should succeed");
        let being_object_witness = run_cli(&root, CurrentSynthesisTuiCli::BeingObjectWitness)
            .expect("being-object witness should succeed");
        let being_object_validate = run_cli(&root, CurrentSynthesisTuiCli::BeingObjectValidate)
            .expect("being-object validate should succeed");
        let move_witness = run_cli(&root, CurrentSynthesisTuiCli::MoveWitness)
            .expect("move witness should succeed");
        let civic_body_witness = run_cli(&root, CurrentSynthesisTuiCli::CivicBodyWitness)
            .expect("civic-body witness should succeed");
        let civic_body_validate = run_cli(&root, CurrentSynthesisTuiCli::CivicBodyValidate)
            .expect("civic-body validate should succeed");
        let civic_crisis_witness = run_cli(&root, CurrentSynthesisTuiCli::CivicCrisisWitness)
            .expect("civic-crisis witness should succeed");
        let flow_glow_witness = run_cli(&root, CurrentSynthesisTuiCli::FlowGlowWitness)
            .expect("flow-glow witness should succeed");
        let flow_glow_validate = run_cli(&root, CurrentSynthesisTuiCli::FlowGlowValidate)
            .expect("flow-glow validate should succeed");
        let embodied_action_witness = run_cli(&root, CurrentSynthesisTuiCli::EmbodiedActionWitness)
            .expect("embodied-action witness should succeed");
        let status = run_cli(
            &root,
            CurrentSynthesisTuiCli::Engine(
                hollow_grove::current_synthesis_engine::EngineLens::Status,
            ),
        )
        .expect("engine status should succeed");
        let bond_list =
            run_cli(&root, CurrentSynthesisTuiCli::BondList).expect("bond list should succeed");
        let npc = run_cli(
            &root,
            CurrentSynthesisTuiCli::NpcInspect(String::from("route_warden_04")),
        )
        .expect("npc inspect should succeed");

        assert!(scenario_list.contains("Scenario List"));
        assert!(scenario_list.contains("flooded_quarry_night_watch"));
        assert!(world_context.contains("Current = blood"));
        assert!(world_context.contains("Hollow = pus"));
        assert!(world_witness.contains("HOLLOW GROVE ALIGNMENT WITNESS"));
        assert!(world_validate.contains("status: pass"));
        assert!(progression_witness.contains("HOLLOW GROVE PROGRESSION WITNESS"));
        assert!(progression_validate.contains("status: pass"));
        assert!(point_squared_witness.contains("HOLLOW GROVE POINT² ASCENSION WITNESS"));
        assert!(map_witness.contains("HOLLOW GROVE ROTATIONAL MAP WITNESS"));
        assert!(map_validate.contains("status: pass"));
        assert!(rule_of_twelve_witness.contains("HOLLOW GROVE RULE OF TWELVE"));
        assert!(rule_of_twelve_validate.contains("status: pass"));
        assert!(manager_language_witness.contains("HOLLOW GROVE MANAGER LANGUAGE"));
        assert!(manager_language_validate.contains("status: pass"));
        assert!(player_location_witness.contains("PLAYER SPATIAL INTERPRETATION"));
        assert!(being_object_witness.contains("HOLLOW GROVE BEING / OBJECT ONTOLOGY"));
        assert!(being_object_validate.contains("status: pass"));
        assert!(move_witness.contains("HOLLOW GROVE MOVE WITNESS"));
        assert!(civic_body_witness.contains("HOLLOW GROVE CIVIC BODY"));
        assert!(civic_body_validate.contains("status: pass"));
        assert!(civic_crisis_witness.contains("World Breach"));
        assert!(flow_glow_witness.contains("HOLLOW GROVE FLOW / GLOW GRAMMAR"));
        assert!(flow_glow_validate.contains("status: pass"));
        assert!(embodied_action_witness.contains("HOLLOW GROVE EMBODIED ACTION WITNESS"));
        assert!(status.contains("Current Synthesis Engine"));
        assert!(bond_list.contains("Bond List"));
        assert!(npc.contains("NPC Inspector"));
        assert!(
            !root
                .join("artifacts/current_synthesis_engine_status.md")
                .exists(),
            "read-only commands should not regenerate the full view bundle"
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn cleopatra_tick_advances_the_persisted_state() {
        let root = unique_temp_dir("current-synthesis-tui-tick");
        let (before, _) = tick_state(&root, None).expect("first tick should succeed");
        let (after, _) = tick_state(&root, None).expect("second tick should succeed");
        assert_eq!(after.completed_ticks, before.completed_ticks + 1);
        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn cleopatra_run_advances_multiple_ticks_in_one_command() {
        let root = unique_temp_dir("current-synthesis-tui-run");
        let output = run_cli(&root, CurrentSynthesisTuiCli::CleopatraRun(3, None))
            .expect("batched run should succeed");
        let persisted = fs::read_to_string(root.join("artifacts/current_synthesis_tui_state.txt"))
            .expect("persisted state should exist");

        assert!(output.contains("Cleopatra Run Complete"));
        assert!(output.contains("ticks applied: 3"));
        assert!(persisted.contains("completed_ticks: 4"));

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn scenario_use_switches_the_persisted_state() {
        let root = unique_temp_dir("current-synthesis-tui-scenario");
        let output = run_cli(
            &root,
            CurrentSynthesisTuiCli::ScenarioUse(String::from("flooded_quarry_night_watch")),
        )
        .expect("scenario switch should succeed");
        assert!(output.contains("flooded_quarry_night_watch"));
        let persisted = fs::read_to_string(root.join("artifacts/current_synthesis_tui_state.txt"))
            .expect("persisted state should exist");
        assert!(persisted.contains("scenario_id: flooded_quarry_night_watch"));
        assert!(persisted.contains("focused_npc_id: quarry_foreman_01"));
        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn player_plan_and_npc_focus_append_first_class_events() {
        let root = unique_temp_dir("current-synthesis-tui-events");
        run_cli(
            &root,
            CurrentSynthesisTuiCli::PlayerPlan(String::from("brace the intake ladder")),
        )
        .expect("player plan should succeed");
        let focus = run_cli(
            &root,
            CurrentSynthesisTuiCli::NpcFocus(String::from("route_warden_04")),
        )
        .expect("npc focus should succeed");
        let player_status = run_cli(&root, CurrentSynthesisTuiCli::PlayerStatus)
            .expect("player status should succeed");
        let event_log = fs::read_to_string(root.join("artifacts/current_synthesis_events.txt"))
            .expect("event log should exist");

        assert!(focus.contains("route_warden_04"));
        assert!(player_status.contains("Player Status"));
        assert!(player_status.contains("Recommended Next Step"));
        assert!(player_status.contains("brace the intake ladder"));
        assert!(event_log.contains("player_action_planned"));
        assert!(event_log.contains("focused_npc_changed"));

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn player_move_advances_the_triad_in_one_command() {
        let root = unique_temp_dir("current-synthesis-tui-player-move");
        let output = run_cli(
            &root,
            CurrentSynthesisTuiCli::PlayerMove(String::from("cross the flooded rim")),
        )
        .expect("player move should succeed");
        let event_log = fs::read_to_string(root.join("artifacts/current_synthesis_events.txt"))
            .expect("event log should exist");
        let cleopatra = fs::read_to_string(root.join("artifacts/current_synthesis_cleopatra.md"))
            .expect("cleopatra artifact should exist");

        assert!(output.contains("Player Move Advanced"));
        assert!(output.contains("player posture: movement-first"));
        assert!(output.contains("Clouseau:"));
        assert!(output.contains("HAL:"));
        assert!(event_log.contains("player_action_planned"));
        assert!(event_log.contains("cleopatra_ticked"));
        assert!(cleopatra.contains("coordinated relay count: 2"));
        assert!(cleopatra.contains("[coordinated]"));

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }
}
