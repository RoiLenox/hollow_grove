use std::{
    io,
    path::{Path, PathBuf},
};

use hollow_grove::{
    current_synthesis_engine::{CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH, parse_persisted_state},
    hueman_progression::{
        HUEMAN_SLICE_STATE_ARTIFACT_PATH, HUEMAN_SLICE_STATUS_ARTIFACT_PATH, SlicePhase,
        VerticalSliceState, build_vertical_slice_follow_up_report,
        build_vertical_slice_progress_report, parse_vertical_slice_state,
        write_vertical_slice_artifacts_at,
    },
    hueman_slice::{SliceResolutionPath, list_vertical_slices},
    read_text_artifact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum SliceDemoCli {
    Help,
    ScenarioList,
    ScenarioUse(String),
    Status,
    Next,
    NextStart,
    NextComplete,
    Walk(Option<SliceResolutionPath>),
    Reset,
    Survey,
    Gather,
    Refine,
    Name(Option<String>),
    Prove(Option<SliceResolutionPath>),
    Clear,
    Deploy(Option<SliceResolutionPath>),
    Recognize,
    Unlock,
}

fn parse_slice_demo_cli<I>(args: I) -> Result<SliceDemoCli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter().collect::<Vec<_>>().into_iter();
    let Some(command) = args.next() else {
        return Ok(SliceDemoCli::Walk(None));
    };

    match command.as_str() {
        "--help" | "-h" | "help" => Ok(SliceDemoCli::Help),
        "scenario" => match args.next().as_deref() {
            Some("list") => {
                require_no_extra_args(args, SliceDemoCli::ScenarioList, "scenario list")
            }
            Some("use") => {
                let slice_id = args
                    .next()
                    .ok_or_else(|| String::from("scenario use requires <slice-id>"))?;
                require_no_extra_args(args, SliceDemoCli::ScenarioUse(slice_id), "scenario use")
            }
            Some(other) => Err(format!("unknown hueman slice scenario command: {other}")),
            None => Err(String::from("scenario requires list or use")),
        },
        "status" => require_no_extra_args(args, SliceDemoCli::Status, "status"),
        "next" => require_no_extra_args(args, SliceDemoCli::Next, "next"),
        "next-start" => require_no_extra_args(args, SliceDemoCli::NextStart, "next-start"),
        "next-complete" => require_no_extra_args(args, SliceDemoCli::NextComplete, "next-complete"),
        "walk" => Ok(SliceDemoCli::Walk(parse_optional_resolution_arg(
            args, "walk",
        )?)),
        "reset" => require_no_extra_args(args, SliceDemoCli::Reset, "reset"),
        "survey" => require_no_extra_args(args, SliceDemoCli::Survey, "survey"),
        "gather" => require_no_extra_args(args, SliceDemoCli::Gather, "gather"),
        "refine" => require_no_extra_args(args, SliceDemoCli::Refine, "refine"),
        "prove" => Ok(SliceDemoCli::Prove(parse_optional_resolution_arg(
            args, "prove",
        )?)),
        "clear" => require_no_extra_args(args, SliceDemoCli::Clear, "clear"),
        "deploy" => Ok(SliceDemoCli::Deploy(parse_optional_resolution_arg(
            args, "deploy",
        )?)),
        "recognize" => require_no_extra_args(args, SliceDemoCli::Recognize, "recognize"),
        "unlock" => require_no_extra_args(args, SliceDemoCli::Unlock, "unlock"),
        "name" => {
            let provided = args.collect::<Vec<_>>();
            if provided.is_empty() {
                Ok(SliceDemoCli::Name(None))
            } else {
                Ok(SliceDemoCli::Name(Some(provided.join(" "))))
            }
        }
        other => Err(format!("unknown hueman slice demo command: {other}")),
    }
}

fn parse_optional_resolution_arg(
    mut args: impl Iterator<Item = String>,
    command: &str,
) -> Result<Option<SliceResolutionPath>, String> {
    let Some(first) = args.next() else {
        return Ok(None);
    };
    if let Some(extra) = args.next() {
        return Err(format!(
            "{command} accepts at most one resolution path argument, got extra value: {extra}"
        ));
    }
    SliceResolutionPath::from_str(&first)
        .map(Some)
        .ok_or_else(|| format!("unknown {command} resolution path: {first}"))
}

fn require_no_extra_args(
    mut args: impl Iterator<Item = String>,
    cli: SliceDemoCli,
    command: &str,
) -> Result<SliceDemoCli, String> {
    if let Some(extra) = args.next() {
        Err(format!(
            "{command} does not accept additional arguments: {extra}"
        ))
    } else {
        Ok(cli)
    }
}

fn usage() -> &'static str {
    "Usage: hueman_slice_demo [scenario list|scenario use <slice-id>|status|next|next-start|next-complete|walk [route|defense]|reset|survey|gather|refine|name [tool name]|prove [route|defense]|clear|deploy [route|defense]|recognize|unlock]\n\
     \n\
     Commands:\n\
       scenario list   list the available Hueman slice scenarios\n\
       scenario use    switch the persisted Hueman slice scenario and reset its progress\n\
       status   print the persisted Aura Ridge slice state, bootstrapping it if needed\n\
       next     print the currently unlocked branch follow-up task, if any\n\
       next-start    begin the unlocked follow-up task for the selected branch\n\
       next-complete complete the in-progress follow-up task for the selected branch\n\
       walk     resume or run the full slice sequence and persist the final state; defaults to the route branch\n\
       reset    restore the persisted slice state to the initial Hueman starting position\n\
       survey   advance NeedObserved -> SeamSurveyed\n\
       gather   advance SeamSurveyed -> InputsGathered using the slice input requirements\n\
       refine   advance InputsGathered -> OpalOilRefined\n\
       name     advance OpalOilRefined -> ToolNamed, using the canonical tool name by default\n\
       prove    advance ToolNamed -> ToolProven; pass `route` or `defense` to choose the proof branch\n\
       clear    advance ToolProven -> ToolCleared\n\
       deploy   advance ToolCleared -> ToolDeployed; pass `route` or `defense` to choose the field resolution path\n\
       recognize advance ToolDeployed -> RecognitionEarned\n\
       unlock   advance RecognitionEarned -> CurrentFormUnlocked\n\
       help     print this help"
}

fn bootstrap_state_from_current_synthesis(root: &Path) -> io::Result<VerticalSliceState> {
    let contents = read_text_artifact(&root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH))?;
    let persisted = parse_persisted_state(&contents)?;
    VerticalSliceState::for_current_synthesis_scenario(&persisted.scenario_id)
}

fn read_or_create_state_at(root: &Path) -> io::Result<VerticalSliceState> {
    let path = root.join(HUEMAN_SLICE_STATE_ARTIFACT_PATH);
    match read_text_artifact(&path) {
        Ok(contents) => parse_vertical_slice_state(&contents),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let state = match bootstrap_state_from_current_synthesis(root) {
                Ok(state) => state,
                Err(bootstrap_error) if bootstrap_error.kind() == io::ErrorKind::NotFound => {
                    VerticalSliceState::primary()
                }
                Err(error) => return Err(error),
            };
            sync_state_artifacts_at(root, &state)?;
            Ok(state)
        }
        Err(error) => Err(error),
    }
}

fn sync_state_artifacts_at(
    root: &Path,
    state: &VerticalSliceState,
) -> io::Result<(PathBuf, PathBuf)> {
    write_vertical_slice_artifacts_at(root, state)
}

fn run_status_at(root: &Path) -> io::Result<String> {
    let state = read_or_create_state_at(root)?;
    Ok(format!(
        "{}\n\n- status artifact: `{}`\n",
        build_vertical_slice_progress_report(&state),
        root.join(HUEMAN_SLICE_STATUS_ARTIFACT_PATH).display()
    ))
}

fn run_scenario_list_at() -> io::Result<String> {
    let mut output = String::from("# Hueman Slice Scenarios\n\n");
    for slice in list_vertical_slices() {
        output.push_str(&format!(
            "- {} (`{}`) form={} tool={}\n",
            slice.title, slice.id, slice.current_form, slice.crafted_object
        ));
    }
    Ok(output)
}

fn run_scenario_use_at(root: &Path, slice_id: &str) -> io::Result<String> {
    let state = VerticalSliceState::for_slice_id(slice_id)?;
    let (state_path, status_path) = sync_state_artifacts_at(root, &state)?;
    Ok(format!(
        "# Hueman Slice Scenario Selected\n\n\
         - slice: `{}`\n\
         - title: {}\n\
         - persisted state: `{}`\n\
         - status artifact: `{}`\n\n\
         {}",
        state.spec().id,
        state.spec().title,
        state_path.display(),
        status_path.display(),
        build_vertical_slice_progress_report(&state)
    ))
}

fn run_next_at(root: &Path) -> io::Result<String> {
    let state = read_or_create_state_at(root)?;
    Ok(format!(
        "{}\n\n- status artifact: `{}`\n",
        build_vertical_slice_follow_up_report(&state),
        root.join(HUEMAN_SLICE_STATUS_ARTIFACT_PATH).display()
    ))
}

fn run_next_start_at(root: &Path) -> io::Result<String> {
    let mut state = read_or_create_state_at(root)?;
    let option = state
        .resolution_path()
        .and_then(|path| state.spec().resolution_option(path))
        .ok_or_else(|| io::Error::other("no branch follow-up is available yet"))?;
    state
        .start_follow_up_task()
        .map_err(|error| io::Error::other(error.to_string()))?;
    let (state_path, status_path) = sync_state_artifacts_at(root, &state)?;
    Ok(format!(
        "# Hueman Slice Follow-Up\n\n\
         ## Start\n\n\
         - {}\n\
         - persisted state: `{}`\n\
         - status artifact: `{}`\n\n\
         {}\n",
        option.follow_up_task_start,
        state_path.display(),
        status_path.display(),
        build_vertical_slice_follow_up_report(&state)
    ))
}

fn run_next_complete_at(root: &Path) -> io::Result<String> {
    let mut state = read_or_create_state_at(root)?;
    let option = state
        .resolution_path()
        .and_then(|path| state.spec().resolution_option(path))
        .ok_or_else(|| io::Error::other("no branch follow-up is available yet"))?;
    state
        .complete_follow_up_task()
        .map_err(|error| io::Error::other(error.to_string()))?;
    let (state_path, status_path) = sync_state_artifacts_at(root, &state)?;
    Ok(format!(
        "# Hueman Slice Follow-Up\n\n\
         ## Complete\n\n\
         - {}\n\
         - persisted state: `{}`\n\
         - status artifact: `{}`\n\n\
         {}\n",
        option.follow_up_task_completion,
        state_path.display(),
        status_path.display(),
        build_vertical_slice_follow_up_report(&state)
    ))
}

fn run_walk_at(root: &Path, resolution_path: Option<SliceResolutionPath>) -> io::Result<String> {
    let mut state = read_or_create_state_at(root)?;
    let mut output = String::from(
        "# Hueman Slice Demo\n\n\
         ## Walk\n\n\
         - observe the active slice need\n",
    );

    if state.phase() == SlicePhase::CurrentFormUnlocked {
        output.push_str("- persisted state already reached the first Gremlin unlock\n\n");
    } else {
        advance_state_to_completion(&mut state, &mut output, resolution_path)?;
    }

    let (state_path, status_path) = sync_state_artifacts_at(root, &state)?;
    output.push_str(&format!(
        "- persisted state: `{}`\n\
         - status artifact: `{}`\n\n",
        state_path.display(),
        status_path.display()
    ));

    output.push_str("## Final Report\n\n");
    output.push_str(&build_vertical_slice_progress_report(&state));

    Ok(output)
}

fn run_reset_at(root: &Path) -> io::Result<String> {
    let state = match bootstrap_state_from_current_synthesis(root) {
        Ok(state) => state,
        Err(error) if error.kind() == io::ErrorKind::NotFound => VerticalSliceState::primary(),
        Err(error) => return Err(error),
    };
    let (state_path, status_path) = sync_state_artifacts_at(root, &state)?;
    Ok(format!(
        "# Hueman Slice Demo\n\n\
         ## Reset\n\n\
         - restored the slice to the initial Hueman state\n\
         - persisted state: `{}`\n\
         - status artifact: `{}`\n\n\
         {}\n",
        state_path.display(),
        status_path.display(),
        build_vertical_slice_progress_report(&state)
    ))
}

fn run_step_at(root: &Path, step: SliceDemoCli) -> io::Result<String> {
    let mut state = read_or_create_state_at(root)?;
    let heading = match &step {
        SliceDemoCli::Survey => "Survey",
        SliceDemoCli::Gather => "Gather",
        SliceDemoCli::Refine => "Refine",
        SliceDemoCli::Name(_) => "Name",
        SliceDemoCli::Prove(_) => "Prove",
        SliceDemoCli::Clear => "Clear",
        SliceDemoCli::Deploy(_) => "Deploy",
        SliceDemoCli::Recognize => "Recognize",
        SliceDemoCli::Unlock => "Unlock",
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid step command",
            ));
        }
    };

    let action_summary = match step {
        SliceDemoCli::Survey => {
            state
                .survey_safe_seam()
                .map_err(|error| io::Error::other(error.to_string()))?;
            String::from("surveyed the active seam through the slice Aura View")
        }
        SliceDemoCli::Gather => {
            let spec = state.spec();
            state
                .gather_inputs(
                    spec.required_regular_current_units,
                    spec.required_holographic_aura_units,
                )
                .map_err(|error| io::Error::other(error.to_string()))?;
            format!(
                "gathered {} Regular Current units and {} Aura units",
                spec.required_regular_current_units, spec.required_holographic_aura_units
            )
        }
        SliceDemoCli::Refine => {
            state
                .refine_opal_oil()
                .map_err(|error| io::Error::other(error.to_string()))?;
            format!("refined the first {} unit", state.spec().signature_resource)
        }
        SliceDemoCli::Name(provided_name) => {
            let tool_name =
                provided_name.unwrap_or_else(|| state.spec().crafted_object.to_string());
            state
                .name_tool(&tool_name)
                .map_err(|error| io::Error::other(error.to_string()))?;
            format!("named the tool `{tool_name}`")
        }
        SliceDemoCli::Prove(resolution_path) => {
            let chosen_path = resolution_path.unwrap_or(state.spec().default_resolution_path);
            state
                .prove_tool_for(chosen_path)
                .map_err(|error| io::Error::other(error.to_string()))?;
            let option = state
                .spec()
                .resolution_option(chosen_path)
                .ok_or_else(|| io::Error::other("missing slice resolution option"))?;
            format!("proved the tool on the bench for {}", option.label)
        }
        SliceDemoCli::Clear => {
            state
                .clear_tool()
                .map_err(|error| io::Error::other(error.to_string()))?;
            String::from("cleared the tool through sealing and alignment")
        }
        SliceDemoCli::Deploy(resolution_path) => {
            let chosen_path = resolution_path
                .or(state.resolution_path())
                .unwrap_or(state.spec().default_resolution_path);
            state
                .deploy_tool_for(chosen_path)
                .map_err(|error| io::Error::other(error.to_string()))?;
            let option = state
                .spec()
                .resolution_option(chosen_path)
                .ok_or_else(|| io::Error::other("missing slice resolution option"))?;
            format!(
                "deployed the tool on the live field problem through {}",
                option.label
            )
        }
        SliceDemoCli::Recognize => {
            state
                .recognize_result()
                .map_err(|error| io::Error::other(error.to_string()))?;
            String::from("earned local Recognition through field success")
        }
        SliceDemoCli::Unlock => {
            state
                .unlock_first_current_form_node()
                .map_err(|error| io::Error::other(error.to_string()))?;
            format!(
                "unlocked the first {} node: `{}`",
                state.unlock().current_form,
                state.unlock().node_name
            )
        }
        _ => unreachable!("validated step command should not reach non-step arm"),
    };

    let (state_path, status_path) = sync_state_artifacts_at(root, &state)?;
    Ok(format!(
        "# Hueman Slice Demo\n\n\
         ## {heading}\n\n\
         - {action_summary}\n\
         - persisted state: `{}`\n\
         - status artifact: `{}`\n\n\
         {}\n",
        state_path.display(),
        status_path.display(),
        build_vertical_slice_progress_report(&state)
    ))
}

fn advance_state_to_completion(
    state: &mut VerticalSliceState,
    output: &mut String,
    resolution_path: Option<SliceResolutionPath>,
) -> io::Result<()> {
    loop {
        match state.phase() {
            SlicePhase::NeedObserved => {
                state
                    .survey_safe_seam()
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str("- survey the active seam through the slice Aura View\n");
            }
            SlicePhase::SeamSurveyed => {
                let spec = state.spec();
                state
                    .gather_inputs(
                        spec.required_regular_current_units,
                        spec.required_holographic_aura_units,
                    )
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str(&format!(
                    "- gather {} Regular Current units and {} Aura units\n",
                    spec.required_regular_current_units, spec.required_holographic_aura_units
                ));
            }
            SlicePhase::InputsGathered => {
                state
                    .refine_opal_oil()
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str(&format!(
                    "- refine 1 {} unit\n",
                    state.spec().signature_resource
                ));
            }
            SlicePhase::OpalOilRefined => {
                state
                    .name_tool(state.spec().crafted_object)
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str(&format!(
                    "- name the tool `{}`\n",
                    state.spec().crafted_object
                ));
            }
            SlicePhase::ToolNamed => {
                let chosen_path = resolution_path.unwrap_or(state.spec().default_resolution_path);
                let option = state
                    .spec()
                    .resolution_option(chosen_path)
                    .ok_or_else(|| io::Error::other("missing slice resolution option"))?;
                state
                    .prove_tool_for(chosen_path)
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str(&format!(
                    "- prove the tool on the bench for {}\n",
                    option.label
                ));
            }
            SlicePhase::ToolProven => {
                state
                    .clear_tool()
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str("- clear the tool through sealing and alignment\n");
            }
            SlicePhase::ToolCleared => {
                let chosen_path = resolution_path
                    .or(state.resolution_path())
                    .unwrap_or(state.spec().default_resolution_path);
                let option = state
                    .spec()
                    .resolution_option(chosen_path)
                    .ok_or_else(|| io::Error::other("missing slice resolution option"))?;
                state
                    .deploy_tool_for(chosen_path)
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str(&format!(
                    "- deploy the tool on the live field problem through {}\n",
                    option.label
                ));
            }
            SlicePhase::ToolDeployed => {
                state
                    .recognize_result()
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str("- earn local Recognition through field success\n");
            }
            SlicePhase::RecognitionEarned => {
                state
                    .unlock_first_current_form_node()
                    .map_err(|error| io::Error::other(error.to_string()))?;
                output.push_str(&format!(
                    "- unlock the first {} node: `{}`\n",
                    state.unlock().current_form,
                    state.unlock().node_name
                ));
            }
            SlicePhase::CurrentFormUnlocked => {
                output.push('\n');
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let cli = parse_slice_demo_cli(std::env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    match cli {
        SliceDemoCli::Help => {
            println!("{}", usage());
            Ok(())
        }
        SliceDemoCli::ScenarioList => {
            println!("{}", run_scenario_list_at()?);
            Ok(())
        }
        SliceDemoCli::ScenarioUse(slice_id) => {
            println!("{}", run_scenario_use_at(Path::new("."), &slice_id)?);
            Ok(())
        }
        SliceDemoCli::Status => {
            println!("{}", run_status_at(Path::new("."))?);
            Ok(())
        }
        SliceDemoCli::Next => {
            println!("{}", run_next_at(Path::new("."))?);
            Ok(())
        }
        SliceDemoCli::NextStart => {
            println!("{}", run_next_start_at(Path::new("."))?);
            Ok(())
        }
        SliceDemoCli::NextComplete => {
            println!("{}", run_next_complete_at(Path::new("."))?);
            Ok(())
        }
        SliceDemoCli::Walk(resolution_path) => {
            println!("{}", run_walk_at(Path::new("."), resolution_path)?);
            Ok(())
        }
        SliceDemoCli::Reset => {
            println!("{}", run_reset_at(Path::new("."))?);
            Ok(())
        }
        SliceDemoCli::Survey
        | SliceDemoCli::Gather
        | SliceDemoCli::Refine
        | SliceDemoCli::Name(_)
        | SliceDemoCli::Prove(_)
        | SliceDemoCli::Clear
        | SliceDemoCli::Deploy(_)
        | SliceDemoCli::Recognize
        | SliceDemoCli::Unlock => {
            println!("{}", run_step_at(Path::new("."), cli)?);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SliceDemoCli, parse_slice_demo_cli, run_next_at, run_next_complete_at, run_next_start_at,
        run_reset_at, run_scenario_list_at, run_scenario_use_at, run_status_at, run_step_at,
        run_walk_at, usage,
    };
    use hollow_grove::current_synthesis_engine::CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH;
    use hollow_grove::hueman_slice::SliceResolutionPath;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn cli_defaults_to_walk() {
        assert_eq!(
            parse_slice_demo_cli(std::iter::empty::<String>()).expect("cli should parse"),
            SliceDemoCli::Walk(None)
        );
    }

    #[test]
    fn cli_supports_help_step_commands_and_walk() {
        assert_eq!(
            parse_slice_demo_cli([String::from("scenario"), String::from("list")])
                .expect("scenario list should parse"),
            SliceDemoCli::ScenarioList
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("status")]).expect("status should parse"),
            SliceDemoCli::Status
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("next")]).expect("next should parse"),
            SliceDemoCli::Next
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("next-start")]).expect("next-start should parse"),
            SliceDemoCli::NextStart
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("next-complete")])
                .expect("next-complete should parse"),
            SliceDemoCli::NextComplete
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("walk")]).expect("walk should parse"),
            SliceDemoCli::Walk(None)
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("walk"), String::from("defense")])
                .expect("walk defense should parse"),
            SliceDemoCli::Walk(Some(SliceResolutionPath::FlockDefense))
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("--help")]).expect("help should parse"),
            SliceDemoCli::Help
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("reset")]).expect("reset should parse"),
            SliceDemoCli::Reset
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("survey")]).expect("survey should parse"),
            SliceDemoCli::Survey
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("gather")]).expect("gather should parse"),
            SliceDemoCli::Gather
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("refine")]).expect("refine should parse"),
            SliceDemoCli::Refine
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("name")]).expect("name should parse"),
            SliceDemoCli::Name(None)
        );
        assert_eq!(
            parse_slice_demo_cli([
                String::from("name"),
                String::from("Ridge"),
                String::from("Lantern"),
                String::from("Drill"),
            ])
            .expect("name with explicit words should parse"),
            SliceDemoCli::Name(Some(String::from("Ridge Lantern Drill")))
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("prove")]).expect("prove should parse"),
            SliceDemoCli::Prove(None)
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("prove"), String::from("defense")])
                .expect("prove defense should parse"),
            SliceDemoCli::Prove(Some(SliceResolutionPath::FlockDefense))
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("clear")]).expect("clear should parse"),
            SliceDemoCli::Clear
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("deploy")]).expect("deploy should parse"),
            SliceDemoCli::Deploy(None)
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("deploy"), String::from("route")])
                .expect("deploy route should parse"),
            SliceDemoCli::Deploy(Some(SliceResolutionPath::RouteStabilization))
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("recognize")]).expect("recognize should parse"),
            SliceDemoCli::Recognize
        );
        assert_eq!(
            parse_slice_demo_cli([String::from("unlock")]).expect("unlock should parse"),
            SliceDemoCli::Unlock
        );
    }

    #[test]
    fn usage_mentions_status_walk_and_reset() {
        let usage = usage();
        assert!(usage.contains("scenario list"));
        assert!(usage.contains("scenario use <slice-id>"));
        assert!(usage.contains("status"));
        assert!(usage.contains("next"));
        assert!(usage.contains("next-start"));
        assert!(usage.contains("next-complete"));
        assert!(usage.contains("walk"));
        assert!(usage.contains("reset"));
        assert!(usage.contains("survey"));
        assert!(usage.contains("name"));
        assert!(usage.contains("route"));
        assert!(usage.contains("defense"));
        assert!(usage.contains("recognize"));
        assert!(usage.contains("unlock"));
    }

    #[test]
    fn status_reports_initial_need_observed_phase() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-status");
        let output = run_status_at(&root).expect("status should succeed");
        assert!(output.starts_with("# Vertical Slice Progress"));
        assert!(output.contains("phase: NeedObserved"));
        assert!(output.contains("Aura Ridge Opal Oil Starter Loop"));
        assert!(root.join("artifacts/hueman_slice_state.txt").exists());
        assert!(root.join("artifacts/hueman_slice_status.md").exists());
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn status_does_not_rewrite_existing_status_artifact() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-status-stable");
        run_scenario_use_at(&root, "aura_ridge_opal_oil_gremlin")
            .expect("scenario use should create artifacts");
        let status_path = root.join("artifacts/hueman_slice_status.md");
        let first_modified = fs::metadata(&status_path)
            .expect("status artifact should exist")
            .modified()
            .expect("status artifact should have a modified time");

        std::thread::sleep(std::time::Duration::from_millis(5));
        let output = run_status_at(&root).expect("status should succeed");
        let second_modified = fs::metadata(&status_path)
            .expect("status artifact should still exist")
            .modified()
            .expect("status artifact should have a modified time");

        assert_eq!(first_modified, second_modified);
        assert!(output.contains("status artifact"));

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn scenario_list_reports_available_slices() {
        let output = run_scenario_list_at().expect("scenario list should succeed");
        assert!(output.contains("aura_ridge_opal_oil_gremlin"));
        assert!(output.contains("flooded_quarry_spillrail_latch"));
    }

    #[test]
    fn scenario_use_switches_the_hueman_slice() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-scenario-use");
        let output = run_scenario_use_at(&root, "flooded_quarry_spillrail_latch")
            .expect("scenario use should succeed");
        assert!(output.contains("Flooded Quarry Night Watch Loop"));
        let persisted = fs::read_to_string(root.join("artifacts/hueman_slice_state.txt"))
            .expect("state exists");
        assert!(persisted.contains("slice_id: flooded_quarry_spillrail_latch"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn bootstrap_follows_current_synthesis_scenario_when_present() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-bootstrap-scenario");
        fs::create_dir_all(root.join("artifacts")).expect("artifact dir should create");
        fs::write(
            root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH),
            "# Current Synthesis TUI State\nscenario_id: flooded_quarry_night_watch\nseed: 7\ncompleted_ticks: 1\nfocused_npc_id: quarry_foreman_01\n",
        )
        .expect("current synthesis state should write");

        let output = run_status_at(&root).expect("status should bootstrap from scenario");
        assert!(output.contains("Flooded Quarry Night Watch Loop"));
        assert!(output.contains("phase: NeedObserved"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn next_reports_locked_follow_up_before_recognition() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-next-pending");
        let output = run_next_at(&root).expect("next should succeed");

        assert!(output.starts_with("# Hueman Slice Follow-Up"));
        assert!(output.contains("no branch has been selected yet"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn step_commands_advance_the_slice_one_phase_at_a_time() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-steps");

        let output = run_step_at(&root, SliceDemoCli::Survey).expect("survey should succeed");
        assert!(output.contains("## Survey"));
        assert!(output.contains("phase: SeamSurveyed"));

        let output = run_step_at(&root, SliceDemoCli::Gather).expect("gather should succeed");
        assert!(output.contains("## Gather"));
        assert!(output.contains("phase: InputsGathered"));

        let output = run_step_at(&root, SliceDemoCli::Refine).expect("refine should succeed");
        assert!(output.contains("## Refine"));
        assert!(output.contains("phase: OpalOilRefined"));

        let output =
            run_step_at(&root, SliceDemoCli::Name(None)).expect("name should succeed by default");
        assert!(output.contains("## Name"));
        assert!(output.contains("tool: Ridge Lantern Drill"));
        assert!(output.contains("phase: ToolNamed"));

        let output = run_step_at(
            &root,
            SliceDemoCli::Prove(Some(SliceResolutionPath::FlockDefense)),
        )
        .expect("prove should succeed");
        assert!(output.contains("phase: ToolProven"));
        assert!(output.contains("resolution path: Flock Defense"));

        let output = run_step_at(&root, SliceDemoCli::Clear).expect("clear should succeed");
        assert!(output.contains("phase: ToolCleared"));

        let output = run_step_at(&root, SliceDemoCli::Deploy(None)).expect("deploy should succeed");
        assert!(output.contains("phase: ToolDeployed"));
        assert!(output.contains("resolution path: Flock Defense"));

        let output = run_step_at(&root, SliceDemoCli::Recognize).expect("recognize should succeed");
        assert!(output.contains("phase: RecognitionEarned"));

        let output = run_step_at(&root, SliceDemoCli::Unlock).expect("unlock should succeed");
        assert!(output.contains("phase: CurrentFormUnlocked"));
        assert!(output.contains("unlocked: true"));

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn name_command_rejects_non_canonical_name() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-name-error");
        run_step_at(&root, SliceDemoCli::Survey).expect("survey should succeed");
        run_step_at(&root, SliceDemoCli::Gather).expect("gather should succeed");
        run_step_at(&root, SliceDemoCli::Refine).expect("refine should succeed");

        let error = run_step_at(&root, SliceDemoCli::Name(Some(String::from("Wrong Drill"))))
            .expect_err("wrong tool name should fail");

        assert_eq!(
            error.to_string(),
            "expected tool name `Ridge Lantern Drill`, got `Wrong Drill`"
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn deploy_command_rejects_switching_branches_after_branch_proof() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-branch-mismatch");
        run_step_at(&root, SliceDemoCli::Survey).expect("survey should succeed");
        run_step_at(&root, SliceDemoCli::Gather).expect("gather should succeed");
        run_step_at(&root, SliceDemoCli::Refine).expect("refine should succeed");
        run_step_at(&root, SliceDemoCli::Name(None)).expect("name should succeed");
        run_step_at(
            &root,
            SliceDemoCli::Prove(Some(SliceResolutionPath::FlockDefense)),
        )
        .expect("defense prove should succeed");
        run_step_at(&root, SliceDemoCli::Clear).expect("clear should succeed");

        let error = run_step_at(
            &root,
            SliceDemoCli::Deploy(Some(SliceResolutionPath::RouteStabilization)),
        )
        .expect_err("deploy should reject switching branches");

        assert_eq!(
            error.to_string(),
            "expected resolution path `defense`, got `route`"
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn walk_reaches_the_first_gremlin_unlock() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-walk");
        let output = run_walk_at(&root, None).expect("walk should succeed");
        assert!(output.starts_with("# Hueman Slice Demo"));
        assert!(output.contains("unlock the first Gremlin node: `Load-Bearing Grip`"));
        assert!(output.contains("phase: CurrentFormUnlocked"));
        assert!(output.contains("unlocked: true"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn status_reads_the_persisted_completed_state() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-persisted");
        run_walk_at(&root, None).expect("walk should persist completion");

        let output = run_status_at(&root).expect("status should read persisted state");

        assert!(output.contains("phase: CurrentFormUnlocked"));
        assert!(output.contains("tool: Ridge Lantern Drill"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn reset_restores_the_initial_state_after_completion() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-reset");
        run_walk_at(&root, None).expect("walk should persist completion");

        let output = run_reset_at(&root).expect("reset should succeed");

        assert!(output.contains("restored the slice to the initial Hueman state"));
        assert!(output.contains("phase: NeedObserved"));
        assert!(output.contains("unlocked: false"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn defense_branch_walk_persists_the_selected_path() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-defense");
        let output = run_walk_at(&root, Some(SliceResolutionPath::FlockDefense))
            .expect("defense walk should succeed");

        assert!(output.contains("through Flock Defense"));
        assert!(output.contains("resolution path: Flock Defense"));
        assert!(output.contains("repelled from the hinge line"));

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn quarry_walk_uses_quarry_terms_and_unlocks_the_goblin_path() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-quarry-walk");
        run_scenario_use_at(&root, "flooded_quarry_spillrail_latch")
            .expect("scenario use should succeed");

        let output = run_walk_at(&root, None).expect("quarry walk should succeed");

        assert!(output.contains("refine 1 Mercury Mirror unit"));
        assert!(output.contains("name the tool `Spillrail Latch`"));
        assert!(output.contains("through Crane Route Hold"));
        assert!(output.contains("unlock the first Goblin node: `Loadline Grip`"));
        assert!(output.contains("branch credential: Quarry Rim Trust"));

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn next_reports_unlocked_follow_up_after_completed_branch() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-next-unlocked");
        run_walk_at(&root, Some(SliceResolutionPath::RouteStabilization))
            .expect("route walk should succeed");

        let output = run_next_at(&root).expect("next should read unlocked task");

        assert!(output.contains("task: Route Hinge Survey"));
        assert!(output.contains("uses reward: Hinge Seal Charge x1"));
        assert!(output.contains("trust credential: Ridge Hinge Trust"));
        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn next_follow_up_commands_advance_the_second_slice() {
        let root = unique_temp_dir("hollow-grove-hueman-slice-next-commands");
        run_walk_at(&root, Some(SliceResolutionPath::FlockDefense))
            .expect("defense walk should succeed");

        let output = run_next_start_at(&root).expect("next-start should succeed");
        assert!(output.contains("begin the shelterline night watch"));
        assert!(output.contains("phase: InProgress"));

        let output = run_next_complete_at(&root).expect("next-complete should succeed");
        assert!(output.contains("Complete the shelterline night watch"));
        assert!(output.contains("phase: Completed"));

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }
}
