// Each legacy tool remains a self-contained module during the transition to the
// consolidated executable. Some intentionally load the same shared support
// source under their own private module namespace.
#![allow(clippy::duplicate_mod)]

use std::env;
use std::fmt::Display;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, ExitCode};

trait IntoToolExit {
    fn into_tool_exit(self) -> ExitCode;
}

impl IntoToolExit for () {
    fn into_tool_exit(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}

impl IntoToolExit for ExitCode {
    fn into_tool_exit(self) -> ExitCode {
        self
    }
}

impl<E: Display> IntoToolExit for Result<(), E> {
    fn into_tool_exit(self) -> ExitCode {
        match self {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("hg-dev: {error}");
                ExitCode::FAILURE
            }
        }
    }
}

macro_rules! tool_modules {
    ($($name:ident => $path:literal),+ $(,)?) => {
        $(#[path = $path] mod $name;)+

        fn run_tool(name: &str) -> Option<ExitCode> {
            match name {
                $(stringify!($name) => Some($name::tool_main().into_tool_exit()),)+
                _ => None,
            }
        }

        const TOOL_NAMES: &[&str] = &[$(stringify!($name)),+];
    };
}

tool_modules!(
    adaptive_midpoint_hinge_audit => "adaptive_midpoint_hinge_audit.rs",
    adaptive_topology_state_audit => "adaptive_topology_state_audit.rs",
    adversarial_verification => "adversarial_verification.rs",
    aura_field_audit => "aura_field_audit.rs",
    aura_surfaces_audit => "aura_surfaces_audit.rs",
    central_junction_constitutional_audit => "central_junction_constitutional_audit.rs",
    central_junction_seasonal_audit => "central_junction_seasonal_audit.rs",
    client_desktop_status => "client_desktop_status.rs",
    client_prompt_artifact => "client_prompt_artifact.rs",
    client_snapshot => "client_snapshot.rs",
    constitutional_geography_audit => "constitutional_geography_audit.rs",
    consumer_boundary => "consumer_boundary.rs",
    consumer_export => "consumer_export.rs",
    consumer_prompt => "consumer_prompt.rs",
    consumer_tree => "consumer_tree.rs",
    current_synthesis => "current_synthesis.rs",
    current_synthesis_activation_gate => "current_synthesis_activation_gate.rs",
    current_synthesis_base => "current_synthesis_base.rs",
    current_synthesis_behavior_rules => "current_synthesis_behavior_rules.rs",
    current_synthesis_benchmark => "current_synthesis_benchmark.rs",
    current_synthesis_choice => "current_synthesis_choice.rs",
    current_synthesis_clients => "current_synthesis_clients.rs",
    current_synthesis_collision_relay => "current_synthesis_collision_relay.rs",
    current_synthesis_consequence => "current_synthesis_consequence.rs",
    current_synthesis_contract => "current_synthesis_contract.rs",
    current_synthesis_execution_spec => "current_synthesis_execution_spec.rs",
    current_synthesis_operational => "current_synthesis_operational.rs",
    current_synthesis_preview => "current_synthesis_preview.rs",
    current_synthesis_readiness => "current_synthesis_readiness.rs",
    current_synthesis_selection => "current_synthesis_selection.rs",
    current_synthesis_sequence => "current_synthesis_sequence.rs",
    current_synthesis_state => "current_synthesis_state.rs",
    current_synthesis_topology => "current_synthesis_topology.rs",
    current_synthesis_transition_pm_to_le => "current_synthesis_transition_pm_to_le.rs",
    deep_pressure_audit => "deep_pressure_audit.rs",
    flynt_constitutional_audit => "flynt_constitutional_audit.rs",
    function_junction_permanence_audit => "function_junction_permanence_audit.rs",
    glaushouse_constitutional_audit => "glaushouse_constitutional_audit.rs",
    hollow_grove_functional_lore_audit => "hollow_grove_functional_lore_audit.rs",
    hollow_grove_route_network_audit => "hollow_grove_route_network_audit.rs",
    hollow_grove_world_point_audit => "hollow_grove_world_point_audit.rs",
    hueman => "hueman.rs",
    hueman_archetype_lens => "hueman_archetype_lens.rs",
    hueman_aura_behavior => "hueman_aura_behavior.rs",
    hueman_aura_triad => "hueman_aura_triad.rs",
    hueman_boundary => "hueman_boundary.rs",
    hueman_crossover_scenes => "hueman_crossover_scenes.rs",
    hueman_flynt_constitution => "hueman_flynt_constitution.rs",
    hueman_fourway => "hueman_fourway.rs",
    hueman_glaushouse_roles => "hueman_glaushouse_roles.rs",
    hueman_inverse_circle => "hueman_inverse_circle.rs",
    hueman_link_physics => "hueman_link_physics.rs",
    hueman_motion_map => "hueman_motion_map.rs",
    hueman_path_crossovers => "hueman_path_crossovers.rs",
    hueman_procedural_uplift => "hueman_procedural_uplift.rs",
    hueman_sandmanor_roles => "hueman_sandmanor_roles.rs",
    hueman_scene_drift => "hueman_scene_drift.rs",
    hueman_scene_intent => "hueman_scene_intent.rs",
    hueman_scene_presence => "hueman_scene_presence.rs",
    hueman_slice_demo => "hueman_slice_demo.rs",
    hueman_start_choices => "hueman_start_choices.rs",
    hueman_start_paths => "hueman_start_paths.rs",
    hueman_stonebend_roles => "hueman_stonebend_roles.rs",
    hueman_vertical_slice => "hueman_vertical_slice.rs",
    living_world_audit => "living_world_audit.rs",
    party_recruitment_audit => "party_recruitment_audit.rs",
    runtime_rampart_audit => "runtime_rampart_audit.rs",
    sandmanor_constitutional_audit => "sandmanor_constitutional_audit.rs",
    spatial_topology_v2_audit => "spatial_topology_v2_audit.rs",
    service_tournament_constitutional_audit => "service_tournament_constitutional_audit.rs",
    stonebend_constitutional_audit => "stonebend_constitutional_audit.rs",
    stonebend_three_gates_audit => "stonebend_three_gates_audit.rs",
    stonebend_title_lifecycle_audit => "stonebend_title_lifecycle_audit.rs",
    visual_color_constitution => "visual_color_constitution.rs",
);

fn usage() {
    eprintln!("Usage: hg-dev <tool> [args]\n\nTools:");
    for name in TOOL_NAMES {
        eprintln!("  {name}");
    }
}

fn main() -> ExitCode {
    let arguments: Vec<_> = env::args_os().collect();
    let invoked_as = arguments
        .first()
        .and_then(|argument| Path::new(argument).file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("hg-dev");

    if invoked_as != "hg-dev" && invoked_as != "hg_dev" {
        return run_tool(invoked_as).unwrap_or_else(|| {
            eprintln!("hg-dev: unknown tool alias {invoked_as}");
            ExitCode::FAILURE
        });
    }

    let Some(tool) = arguments.get(1).and_then(|argument| argument.to_str()) else {
        usage();
        return ExitCode::FAILURE;
    };
    if tool == "--list" {
        for name in TOOL_NAMES {
            println!("{name}");
        }
        return ExitCode::SUCCESS;
    }
    if !TOOL_NAMES.contains(&tool) {
        eprintln!("hg-dev: unknown tool {tool}");
        usage();
        return ExitCode::FAILURE;
    }

    let executable = match env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("hg-dev: could not locate the current executable: {error}");
            return ExitCode::FAILURE;
        }
    };
    let status = Command::new(executable)
        .arg0(tool)
        .args(arguments.iter().skip(2))
        .status();
    match status {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("hg-dev: could not start {tool}: {error}");
            ExitCode::FAILURE
        }
    }
}
