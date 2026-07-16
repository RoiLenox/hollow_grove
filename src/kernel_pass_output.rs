use std::fmt::Write as _;

use crate::{
    AURA_BEAM_WITNESS_LABEL, CURRENT_SEAM_WITNESS_LABEL, DecisionExecution, DecisionIntent,
    FOURWAY_WITNESS_LABEL, FireContext, FrameState, KernelPass, KissLanding,
    LANDED_WITNESS_DESCRIPTION, LANDED_WITNESS_LABEL, LandingOutcome, Point, RecipeIntent,
    START_WITNESS_LABEL, SynthesisExecution, SynthesisExecutionError, SynthesisRecipe,
    SynthesisRecipeCompileError, SynthesisScript, Way, execute_kernel_pass_decision,
    execute_synthesis_recipe, fire_with_context, gremlin_tinker_recipe, land_contact,
    pixy_confusion_recipe,
};

pub const SNAPSHOT_ARTIFACT_PATH: &str = "artifacts/kernel_pass_snapshot.json";
pub const PROMPT_ARTIFACT_PATH: &str = "artifacts/consumer_prompt.md";
pub const DESKTOP_STATUS_ARTIFACT_PATH: &str = "artifacts/desktop_status.txt";
pub const INVERSE_PATH_QUESTION: &str =
    "What does this completed pass reveal about the inverse path of the end use?";
pub const BOUNDARY_REMINDER: &str = "Do not mutate the kernel. Interpret only.";

fn way_name(way: Way) -> &'static str {
    match way {
        Way::One => "One",
        Way::Two => "Two",
        Way::Three => "Three",
    }
}

fn manager_name(manager: crate::Manager) -> &'static str {
    match manager {
        crate::Manager::Hal => "HAL",
        crate::Manager::Clouseau => "Clouseau",
        crate::Manager::Cleopatra => "Cleopatra",
    }
}

fn manager_domain_name(domain: crate::ManagerDomain) -> &'static str {
    match domain {
        crate::ManagerDomain::Meta => "META",
        crate::ManagerDomain::Pleb => "PLEB",
        crate::ManagerDomain::Blep => "BLEP",
    }
}

fn manager_relation_name(relation: crate::ManagerRelation) -> &'static str {
    match relation {
        crate::ManagerRelation::PlebMeta => "PLEB ↔ META",
        crate::ManagerRelation::PlebPleb => "PLEB ↔ PLEB",
        crate::ManagerRelation::PlebBlep => "PLEB ↔ BLEP",
    }
}

fn manager_geometry_name(geometry: crate::ManagerGeometry) -> &'static str {
    match geometry {
        crate::ManagerGeometry::Curved => "curved",
        crate::ManagerGeometry::Straight => "straight",
        crate::ManagerGeometry::Inverted => "inverted",
    }
}

fn manager_function_name(function: crate::ManagerFunction) -> &'static str {
    match function {
        crate::ManagerFunction::Locate => "Locates",
        crate::ManagerFunction::Connect => "Connects",
        crate::ManagerFunction::Reflect => "Reflects",
    }
}

fn push_escaped_canonical_witness(output: &mut String, kernel_pass: &KernelPass) {
    for ch in kernel_pass.canonical_witness().chars() {
        if ch == '\n' {
            output.push_str("\\n");
        } else {
            output.push(ch);
        }
    }
}

fn render_identifier_list<T: std::fmt::Debug>(items: &[T]) -> String {
    let mut output = String::with_capacity(32);
    push_identifier_list(&mut output, items);
    output
}

fn push_identifier_list<T: std::fmt::Debug>(output: &mut String, items: &[T]) {
    if items.is_empty() {
        output.push_str("none");
        return;
    }

    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        let _ = write!(output, "{item:?}");
    }
}

fn format_signed_delta(value: i16) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn script_name(script: &SynthesisScript) -> &'static str {
    match script {
        SynthesisScript::ApplyPrismDelta(_) => "ApplyPrismDelta",
        SynthesisScript::AddFlow(_) => "AddFlow",
        SynthesisScript::AddGlow(_) => "AddGlow",
        SynthesisScript::SetFrame(_) => "SetFrame",
    }
}

fn mismatched_relation(relation: crate::ManagerRelation) -> crate::ManagerRelation {
    match relation {
        crate::ManagerRelation::PlebMeta => crate::ManagerRelation::PlebPleb,
        crate::ManagerRelation::PlebPleb => crate::ManagerRelation::PlebMeta,
        crate::ManagerRelation::PlebBlep => crate::ManagerRelation::PlebMeta,
    }
}

#[derive(Debug)]
enum SynthesisFixtureBuildError {
    Execute(SynthesisExecutionError),
    Landing(crate::ScriptApplicationError),
}

struct SynthesisFixture {
    execution: SynthesisExecution,
    kiss: KissLanding,
    miss_contact: crate::ContactOutcome,
    miss_frame_state: FrameState,
}

fn build_synthesis_fixture(
    start: &Point,
    recipe: SynthesisRecipe,
) -> Result<SynthesisFixture, SynthesisFixtureBuildError> {
    let execution =
        execute_synthesis_recipe(start, &recipe).map_err(SynthesisFixtureBuildError::Execute)?;
    let LandingOutcome::Kiss(kiss) = execution.landing().clone() else {
        unreachable!("canonical fire should kiss");
    };
    let aim = execution.aim();

    let miss_context = FireContext::new(
        mismatched_relation(aim.manager_lock().relation()),
        aim.manager_lock().geometry(),
        aim.bond().linked_way(),
    );
    let miss_contact = fire_with_context(aim, &miss_context);
    let miss_landing = land_contact(start.frame_state(), aim, miss_contact)
        .map_err(SynthesisFixtureBuildError::Landing)?;
    let LandingOutcome::Miss {
        frame_state: miss_frame_state,
    } = miss_landing
    else {
        unreachable!("miss fixture should remain a miss");
    };

    Ok(SynthesisFixture {
        execution,
        kiss,
        miss_contact,
        miss_frame_state,
    })
}

fn push_recipe_intents(output: &mut String, recipe: &SynthesisRecipe) {
    for intent in recipe.intents() {
        match intent {
            RecipeIntent::ModifyPrism(delta) => {
                for (label, value) in [
                    ("Stonebend / Body", delta.body()),
                    ("Flynt / Spirit", delta.spirit()),
                    ("Glaüshouse / Mind", delta.mind()),
                    ("Minorian / Interior", delta.soul_interior()),
                    ("Minoan / Exterior", delta.soul_exterior()),
                ] {
                    if value != 0 {
                        let _ = writeln!(output, "    {label} {}", format_signed_delta(value));
                    }
                }
            }
            RecipeIntent::LearnFlow(flow_id) => {
                let _ = writeln!(output, "    Learn Flow: {flow_id:?}");
            }
            RecipeIntent::LearnGlow(glow_id) => {
                let _ = writeln!(output, "    Learn Glow: {glow_id:?}");
            }
            RecipeIntent::ChangeFrame(frame_id) => {
                let _ = writeln!(output, "    Change Frame: {frame_id:?}");
            }
        }
    }
    output.push('\n');
}

fn push_script_list(output: &mut String, scripts: &[SynthesisScript]) {
    for (index, script) in scripts.iter().enumerate() {
        match script {
            SynthesisScript::ApplyPrismDelta(delta) => {
                let _ = write!(
                    output,
                    "  {}. ApplyPrismDelta\n\
                     \x20\x20\x20\x20Stonebend / Body: {}\n\
                     \x20\x20\x20\x20Flynt / Spirit: {}\n\
                     \x20\x20\x20\x20Glaüshouse / Mind: {}\n\
                     \x20\x20\x20\x20Minorian / Interior: {}\n\
                     \x20\x20\x20\x20Minoan / Exterior: {}\n\n",
                    index + 1,
                    format_signed_delta(delta.body()),
                    format_signed_delta(delta.spirit()),
                    format_signed_delta(delta.mind()),
                    format_signed_delta(delta.soul_interior()),
                    format_signed_delta(delta.soul_exterior()),
                );
            }
            SynthesisScript::AddFlow(flow_id) => {
                let _ = write!(
                    output,
                    "  {}. AddFlow\n\
                     \x20\x20\x20\x20{flow_id:?}\n\n",
                    index + 1,
                );
            }
            SynthesisScript::AddGlow(glow_id) => {
                let _ = write!(
                    output,
                    "  {}. AddGlow\n\
                     \x20\x20\x20\x20{glow_id:?}\n\n",
                    index + 1,
                );
            }
            SynthesisScript::SetFrame(frame_id) => {
                let _ = write!(
                    output,
                    "  {}. SetFrame\n\
                     \x20\x20\x20\x20{frame_id:?}\n\n",
                    index + 1,
                );
            }
        }
    }
}

fn push_applied_script_names(output: &mut String, scripts: &[SynthesisScript]) {
    for (index, script) in scripts.iter().enumerate() {
        let _ = writeln!(output, "    {}. {}", index + 1, script_name(script));
    }
    output.push('\n');
}

fn push_synthesis_fixture_body(
    output: &mut String,
    fixture: &SynthesisFixture,
    recipe_heading: &str,
) {
    let recipe = fixture.execution.recipe();
    let _ = write!(
        output,
        "{recipe_heading}\n\n\
         \x20\x20ID:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Name:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Intended changes:\n",
        recipe.recipe_id(),
        recipe.display_name(),
    );
    push_recipe_intents(output, recipe);

    output.push_str(
        "RECIPE COMPILER\n\n\
         \x20\x20status:\n\
         \x20\x20\x20\x20compiled\n\n\
         SYNTHESIS SCRIPTS\n\n\
         \x20\x20status:\n\
         \x20\x20\x20\x20ready\n\n",
    );
    push_script_list(output, fixture.execution.scripts());

    let aim = fixture.execution.aim();
    let manager_lock = aim.manager_lock();
    let named_route = aim.named_route().unwrap_or("unset");
    let _ = write!(
        output,
        "SYNTHESIS AIM\n\n\
         \x20\x20ID:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Source Recipe:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Manager:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Domain:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Relation:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Geometry:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Function:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Bond:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Named Route:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Scripts:\n\n",
        aim.aim_id(),
        aim.source_recipe_id(),
        manager_name(manager_lock.manager()),
        manager_domain_name(manager_lock.domain()),
        manager_relation_name(manager_lock.relation()),
        manager_geometry_name(manager_lock.geometry()),
        manager_function_name(manager_lock.function()),
        way_name(aim.bond().linked_way()),
        named_route,
    );
    push_script_list(output, aim.scripts());
    let _ = write!(
        output,
        "  Status:\n\
         \x20\x20\x20\x20{}\n\n",
        aim.status_label(),
    );

    let current_prism_delta = fixture
        .kiss
        .applied_scripts()
        .iter()
        .find_map(|script| match script {
            SynthesisScript::ApplyPrismDelta(delta) => Some(*delta),
            _ => None,
        })
        .unwrap_or(crate::PrismDelta::zero());
    let glow_learned = render_identifier_list(
        &fixture
            .kiss
            .applied_scripts()
            .iter()
            .filter_map(|script| match script {
                SynthesisScript::AddGlow(glow_id) => Some(*glow_id),
                _ => None,
            })
            .collect::<Vec<_>>(),
    );
    let flow_learned = render_identifier_list(
        &fixture
            .kiss
            .applied_scripts()
            .iter()
            .filter_map(|script| match script {
                SynthesisScript::AddFlow(flow_id) => Some(*flow_id),
                _ => None,
            })
            .collect::<Vec<_>>(),
    );

    let _ = write!(
        output,
        "FIRE\n\n\
         \x20\x20status:\n\
         \x20\x20\x20\x20committed\n\n\
         \x20\x20Contact:\n\
         \x20\x20\x20\x20{}\n\n\
         KISS LANDING\n\n\
         \x20\x20Scripts applied:\n\
         \x20\x20\x20\x20yes\n\n\
         \x20\x20Applied Scripts:\n",
        fixture.execution.contact().as_str(),
    );
    push_applied_script_names(output, fixture.kiss.applied_scripts());
    let _ = write!(
        output,
        "  Starting Frame:\n\
         \x20\x20\x20\x20{:?}\n\n\
         \x20\x20Point² Frame:\n\
         \x20\x20\x20\x20{:?}\n\n\
         \x20\x20CURRENT PRISM DELTA:\n\
         \x20\x20\x20\x20Stonebend / Body: {}\n\
         \x20\x20\x20\x20Flynt / Spirit: {}\n\
         \x20\x20\x20\x20Glaüshouse / Mind: {}\n\
         \x20\x20\x20\x20Minorian / Interior: {}\n\
         \x20\x20\x20\x20Minoan / Exterior: {}\n\n\
         \x20\x20GLOW LEARNED:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20FLOW LEARNED:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20FrameState changed:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Point² produced:\n\
         \x20\x20\x20\x20true\n\n\
         {}\n\
         MISS LANDING\n\n\
         \x20\x20Contact:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Scripts applied:\n\
         \x20\x20\x20\x20no\n\n\
         \x20\x20FrameState changed:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Point² produced:\n\
         \x20\x20\x20\x20no\n\n",
        fixture.kiss.before().frame(),
        fixture.kiss.point_squared().frame(),
        format_signed_delta(current_prism_delta.body()),
        format_signed_delta(current_prism_delta.spirit()),
        format_signed_delta(current_prism_delta.mind()),
        format_signed_delta(current_prism_delta.soul_interior()),
        format_signed_delta(current_prism_delta.soul_exterior()),
        glow_learned,
        flow_learned,
        fixture.kiss.before() != fixture.kiss.point_squared(),
        build_frame_state_section(
            "POINT²",
            Some(LANDED_WITNESS_DESCRIPTION),
            fixture.kiss.point_squared(),
        ),
        fixture.miss_contact.as_str(),
        fixture.miss_frame_state != *fixture.kiss.before(),
    );
}

fn build_synthesis_fixture_section(
    start: &Point,
    recipe_heading: &str,
    recipe: SynthesisRecipe,
) -> String {
    let mut output = String::with_capacity(1600);
    match build_synthesis_fixture(start, recipe) {
        Ok(fixture) => push_synthesis_fixture_body(&mut output, &fixture, recipe_heading),
        Err(SynthesisFixtureBuildError::Execute(SynthesisExecutionError::Compile(error))) => {
            let _ = write!(
                output,
                "{recipe_heading}\n\n\
                 RECIPE COMPILER\n\n\
                 \x20\x20status:\n\
                 \x20\x20\x20\x20error\n\n\
                 \x20\x20error:\n\
                 \x20\x20\x20\x20{:?}\n\n\
                 SYNTHESIS SCRIPTS\n\n\
                 \x20\x20none\n\n",
                error
            );
        }
        Err(SynthesisFixtureBuildError::Execute(SynthesisExecutionError::Aim(error))) => {
            let mapped_error = match error {
                crate::AimBuildError::EmptyAimId | crate::AimBuildError::EmptyRecipeId => {
                    SynthesisRecipeCompileError::EmptyRecipeId
                }
                crate::AimBuildError::NoScripts => SynthesisRecipeCompileError::NoIntents,
            };
            let _ = write!(
                output,
                "{recipe_heading}\n\n\
                 SYNTHESIS AIM\n\n\
                 \x20\x20status:\n\
                 \x20\x20\x20\x20error\n\n\
                 \x20\x20error:\n\
                 \x20\x20\x20\x20{:?}\n\n",
                mapped_error
            );
        }
        Err(SynthesisFixtureBuildError::Execute(SynthesisExecutionError::UnknownRecipe)) => {
            let _ = write!(
                output,
                "{recipe_heading}\n\n\
                 RECIPE COMPILER\n\n\
                 \x20\x20status:\n\
                 \x20\x20\x20\x20error\n\n\
                 \x20\x20error:\n\
                 \x20\x20\x20\x20UnknownRecipe\n\n",
            );
        }
        Err(SynthesisFixtureBuildError::Execute(SynthesisExecutionError::Landing(error)))
        | Err(SynthesisFixtureBuildError::Landing(error)) => {
            let _ = write!(
                output,
                "{recipe_heading}\n\n\
                 KISS LANDING\n\n\
                 \x20\x20status:\n\
                 \x20\x20\x20\x20error\n\n\
                 \x20\x20error:\n\
                 \x20\x20\x20\x20{:?}\n\n",
                error
            );
        }
    }

    output
}

fn build_version_two_decision_section(heading: &str, result: &DecisionExecution) -> String {
    let mut output = String::with_capacity(1_600);
    let observation = result.observation();
    let trace = result.trace();
    let _ = writeln!(output, "{heading}\n");
    output.push_str("Decision Trace\n\n");
    let _ = writeln!(
        output,
        "Observe:\n  {:?}, Intent {}",
        observation.frame_state().frame(),
        observation.intent().as_str()
    );
    let _ = writeln!(
        output,
        "  Flow: {}",
        render_identifier_list(trace.observation().flows())
    );
    let _ = writeln!(
        output,
        "  Glow: {}",
        render_identifier_list(trace.observation().glows())
    );
    if let Some(route_geometry) = observation.route_geometry() {
        let _ = writeln!(
            output,
            "  Route Geometry: {}",
            manager_geometry_name(route_geometry)
        );
    }
    output.push('\n');
    output.push_str("State checks:\n");
    for check in trace.observation().state_checks() {
        let _ = writeln!(
            output,
            "  {}: frame={}, flow={}, glow={}",
            check.candidate_id().as_str(),
            check.already_canonical_frame(),
            check.already_knows_canonical_flow(),
            check.already_knows_canonical_glow(),
        );
    }
    output.push('\n');
    output.push_str("Generate:\n");
    for candidate in trace.generation() {
        let _ = writeln!(output, "  {}", candidate.candidate_id().as_str());
        let _ = writeln!(output, "    Manager: {}", manager_name(candidate.manager()));
        let _ = writeln!(
            output,
            "    Geometry: {}",
            manager_geometry_name(candidate.manager_geometry())
        );
        let _ = writeln!(
            output,
            "    Orientation: {}",
            candidate.orientation().as_str()
        );
    }
    output.push('\n');
    output.push_str("Evaluate:\n");
    for (evaluation, trace_evaluation) in result.evaluations().iter().zip(trace.evaluations()) {
        let _ = writeln!(
            output,
            "  {} = {}, {}",
            evaluation.candidate_id().as_str(),
            evaluation.score(),
            evaluation.reason().as_str()
        );
        let _ = writeln!(
            output,
            "    Intent score: {}",
            trace_evaluation.intent_score()
        );
        let _ = writeln!(
            output,
            "    Realized penalty: {}",
            trace_evaluation.realized_state_penalty()
        );
        let _ = writeln!(output, "    Final: {}", trace_evaluation.final_score());
        output.push_str("    Reasons: ");
        for (index, reason_code) in trace_evaluation.reason_codes().iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            output.push_str(reason_code.as_str());
        }
        output.push('\n');
    }
    output.push('\n');
    let _ = writeln!(
        output,
        "Choose:\n  Highest score: {}\n  Tie: {}\n  {}\n",
        trace.choice().highest_score(),
        trace.choice().tie_occurred(),
        result.chosen().candidate().candidate_id().as_str()
    );
    let tie_break_label = result
        .chosen()
        .tie_break()
        .map(crate::DecisionTieBreak::as_str)
        .unwrap_or("no tie");
    let _ = writeln!(
        output,
        "Tie-break:\n  {}\n  Reason: {}\n  Geometry matched: {}\n  Generate order resolved: {}",
        tie_break_label,
        trace.choice().tie_break_reason().as_str(),
        trace.choice().manager_geometry_matched(),
        trace.choice().generate_order_resolved(),
    );
    if let Some(candidate_id) = trace.choice().geometry_matching_candidate() {
        let _ = writeln!(
            output,
            "  Geometry match candidate: {}",
            candidate_id.as_str()
        );
    }
    if !trace.choice().tied_candidates().is_empty() {
        output.push_str("  Tied candidates: ");
        for (index, candidate_id) in trace.choice().tied_candidates().iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            output.push_str(candidate_id.as_str());
        }
        output.push('\n');
    }
    output.push('\n');
    let _ = writeln!(
        output,
        "Recipe:\n  {}\n  ID: {}\n  Handed to Version 1.1: {}\n",
        result.recipe().display_name(),
        trace.recipe_bridge().recipe_id(),
        trace.recipe_bridge().handed_to_execution_facade(),
    );
    let execution_trace = trace.execution();
    let landed_frame = execution_trace
        .landed_frame()
        .map(|frame| format!("{frame:?} Point²"))
        .unwrap_or_else(|| "unchanged Point".to_string());
    let _ = writeln!(
        output,
        "Version 1.1 Execution:\n    {:?} → {} → {}",
        result.execution().start_frame_state().frame(),
        execution_trace.contact().as_str(),
        landed_frame,
    );
    let _ = writeln!(output, "  Contact: {}", execution_trace.contact().as_str());
    let _ = writeln!(
        output,
        "  Point² produced: {}",
        execution_trace.point_squared_produced()
    );
    let delta = execution_trace.prism_delta();
    if delta.body() != 0 {
        let _ = writeln!(
            output,
            "  Body {} → {}",
            result.execution().start_frame_state().prism().body(),
            i32::from(result.execution().start_frame_state().prism().body())
                + i32::from(delta.body())
        );
    }
    if delta.mind() != 0 {
        let _ = writeln!(
            output,
            "  Mind {} → {}",
            result.execution().start_frame_state().prism().mind(),
            i32::from(result.execution().start_frame_state().prism().mind())
                + i32::from(delta.mind())
        );
    }
    if !execution_trace.added_flow().is_empty() {
        output.push_str("  Flow +");
        push_identifier_list(&mut output, execution_trace.added_flow());
        output.push('\n');
    }
    if !execution_trace.added_glow().is_empty() {
        output.push_str("  Glow +");
        push_identifier_list(&mut output, execution_trace.added_glow());
        output.push('\n');
    }
    output.push('\n');
    output
}

fn build_version_two_witness_section(kernel_pass: &KernelPass) -> String {
    let current = execute_kernel_pass_decision(kernel_pass, DecisionIntent::FavorCurrent);
    let aura = execute_kernel_pass_decision(kernel_pass, DecisionIntent::FavorAura);
    let neutral = execute_kernel_pass_decision(kernel_pass, DecisionIntent::Neutral);
    let mut output = String::with_capacity(2_200);
    output.push_str("VERSION 2 DECISION TRACE\n\n");

    match current {
        Ok(result) => output.push_str(&build_version_two_decision_section(
            "CURRENT-FAVORED",
            &result,
        )),
        Err(error) => {
            let _ = writeln!(output, "CURRENT-FAVORED\n\nerror:\n  {:?}\n", error);
        }
    }

    match aura {
        Ok(result) => output.push_str(&build_version_two_decision_section("AURA-FAVORED", &result)),
        Err(error) => {
            let _ = writeln!(output, "AURA-FAVORED\n\nerror:\n  {:?}\n", error);
        }
    }

    match neutral {
        Ok(result) => output.push_str(&build_version_two_decision_section("NEUTRAL", &result)),
        Err(error) => {
            let _ = writeln!(output, "NEUTRAL\n\nerror:\n  {:?}\n", error);
        }
    }

    output
}

fn build_frame_state_section(
    heading: &str,
    label: Option<&str>,
    frame_state: &crate::FrameState,
) -> String {
    let prism = frame_state.prism();
    let flow = render_identifier_list(frame_state.flow_learnset());
    let glow = render_identifier_list(frame_state.glow_learnset());
    let mut output = String::with_capacity(256);
    output.push_str(heading);
    output.push('\n');
    if let Some(label) = label {
        output.push_str(label);
        output.push_str("\n\n");
    } else {
        output.push('\n');
    }
    let _ = write!(
        output,
        "Frame:\n\
         \x20\x20{:?}\n\n\
         CURRENT PRISM\n\n\
         \x20\x20Stonebend / Body:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Flynt / Spirit:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Glaüshouse / Mind:\n\
         \x20\x20\x20\x20{}\n\n\
         \x20\x20Sandmanor / Soul:\n\
         \x20\x20\x20\x20Minorian / Interior:\n\
         \x20\x20\x20\x20\x20\x20{}\n\
         \x20\x20\x20\x20Minoan / Exterior:\n\
         \x20\x20\x20\x20\x20\x20{}\n\n\
         FLOW\n\n\
         \x20\x20{}\n\n\
         GLOW\n\n\
         \x20\x20{}\n",
        frame_state.frame(),
        prism.body(),
        prism.spirit(),
        prism.mind(),
        prism.soul_interior(),
        prism.soul_exterior(),
        flow,
        glow,
    );
    output
}

fn build_frame_state_witness_section(kernel_pass: &KernelPass) -> String {
    let start = kernel_pass.start_frame_state();
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();
    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "{}\n\
         TRIWAY\n\n\
         \x20\x20ways: [{}, {}, {}]\n\n\
         FOURWAY\n\n\
         \x20\x20label: {}\n\n\
         HOLLOW GROVE\n\n\
         \x20\x20bond: {}\n\
         \x20\x20atmosphere: [{}, {}]\n\n\
         CURRENT SEAM\n\n\
         \x20\x20route: {}\n\n\
         AURA BEAM\n\n\
         \x20\x20route: {}\n\n",
        build_frame_state_section("STARTING POINT", None, start),
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        FOURWAY_WITNESS_LABEL,
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        kernel_pass.grove_seam().route(),
        kernel_pass.hollow_beam().route(),
    )
}

pub fn build_snapshot_output(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();
    let mut output = String::with_capacity(600);
    let _ = write!(
        output,
        "{{\n\
         \x20\x20\"start\": \"{}\",\n\
         \x20\x20\"triway\": {{\n\
         \x20\x20\x20\x20\"ways\": [\"{}\", \"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"fourway\": \"{}\",\n\
         \x20\x20\"hollow_grove\": {{\n\
         \x20\x20\x20\x20\"bond\": \"{}\",\n\
         \x20\x20\x20\x20\"atmosphere\": [\"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"grove_seam\": \"{}\",\n\
         \x20\x20\"grove_seam_route\": \"{}\",\n\
         \x20\x20\"hollow_beam\": \"{}\",\n\
         \x20\x20\"hollow_beam_route\": \"{}\",\n\
         \x20\x20\"landed\": \"{}\",\n\
         \x20\x20\"landing_route\": \"{}\",\n\
         \x20\x20\"landed_point\": \"{}\",\n\
         \x20\x20\"canonical_witness\": \"",
        START_WITNESS_LABEL,
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        FOURWAY_WITNESS_LABEL,
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        CURRENT_SEAM_WITNESS_LABEL,
        kernel_pass.grove_seam().route(),
        AURA_BEAM_WITNESS_LABEL,
        kernel_pass.hollow_beam().route(),
        LANDED_WITNESS_DESCRIPTION,
        kernel_pass.landed().route(),
        LANDED_WITNESS_LABEL,
    );
    push_escaped_canonical_witness(&mut output, kernel_pass);
    output.push_str("\"\n}");
    output
}

pub fn build_prompt_artifact_output(kernel_pass: &KernelPass) -> String {
    format!(
        "# Consumer Prompt\n\n\
         ## Canonical Witness\n\n\
         ```text\n\
         {}\n\
         ```\n\n\
         ## Structured Snapshot Reference\n\n\
         `{SNAPSHOT_ARTIFACT_PATH}`\n\n\
         ## Inverse-Path Question\n\n\
         {INVERSE_PATH_QUESTION}\n\n\
         ## Boundary Reminder\n\n\
         {BOUNDARY_REMINDER}\n",
        kernel_pass
    )
}

pub fn build_desktop_status_output(kernel_pass: &KernelPass) -> String {
    let mut output = String::with_capacity(6_000);
    let _ = write!(
        output,
        "Hollow Grove status: one completed witnessed recursion\n\n\
         Canonical witness:\n\
         {}\n\n\
         Note: read-only desktop artifact\n\
         Note: niri/river configs untouched\n\n\
         {}\n\
         {}\
         {}\
         {}",
        kernel_pass,
        build_frame_state_witness_section(kernel_pass),
        build_synthesis_fixture_section(
            kernel_pass.start_point(),
            "SYNTHESIS RECIPE",
            pixy_confusion_recipe(),
        ),
        build_synthesis_fixture_section(
            kernel_pass.start_point(),
            "GREMLIN TINKER RECIPE",
            gremlin_tinker_recipe(),
        ),
        build_version_two_witness_section(kernel_pass),
    );
    output
}

pub fn build_tree_output(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "KernelPass\n\
         ├─ start: {}\n\
         ├─ triway\n\
         │  ├─ ways: [{}, {}, {}]\n\
         ├─ fourway: {}\n\
         ├─ hollow_grove\n\
         │  ├─ bond: {}\n\
         │  └─ atmosphere: [{}, {}]\n\
         ├─ current_seam: {} [{}]\n\
         ├─ aura_beam: {} [{}]\n\
         └─ point_squared: {} ({}) [{}]",
        START_WITNESS_LABEL,
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        FOURWAY_WITNESS_LABEL,
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        CURRENT_SEAM_WITNESS_LABEL,
        kernel_pass.grove_seam().route(),
        AURA_BEAM_WITNESS_LABEL,
        kernel_pass.hollow_beam().route(),
        LANDED_WITNESS_LABEL,
        LANDED_WITNESS_DESCRIPTION,
        kernel_pass.landed().route()
    )
}

pub fn build_inverse_path_prompt(witness: &str) -> String {
    format!("{witness}\n\n{INVERSE_PATH_QUESTION}")
}

#[cfg(test)]
mod tests {
    use crate::{
        CANONICAL_WITNESS, ExteriorShape, KernelInput, Mode, Symptom, run_kernel_cycle,
        run_kernel_cycle_with_input,
    };

    use super::{
        build_desktop_status_output, build_inverse_path_prompt, build_prompt_artifact_output,
        build_snapshot_output, build_tree_output,
    };

    #[test]
    fn snapshot_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_snapshot_output(&kernel_pass),
            "{\n\
             \x20\x20\"start\": \"Point\",\n\
             \x20\x20\"triway\": {\n\
             \x20\x20\x20\x20\"ways\": [\"One\", \"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"fourway\": \"Fourway\",\n\
             \x20\x20\"hollow_grove\": {\n\
             \x20\x20\x20\x20\"bond\": \"One\",\n\
             \x20\x20\x20\x20\"atmosphere\": [\"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"grove_seam\": \"CurrentSeam\",\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam\": \"AuraBeam\",\n\
             \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
             \x20\x20\"landed\": \"Landed Point\",\n\
             \x20\x20\"landing_route\": \"BlepArrival\",\n\
             \x20\x20\"landed_point\": \"Point²\",\n\
             \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
             }"
        );
    }

    #[test]
    fn prompt_artifact_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_prompt_artifact_output(&kernel_pass),
            "# Consumer Prompt\n\n\
             ## Canonical Witness\n\n\
             ```text\n\
             Point\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             CurrentSeam [PlebExterior]\n\
             ↓\n\
             AuraBeam [BlepReturn]\n\
             ↓\n\
             Point² (Landed Point) [BlepArrival]\n\
             ```\n\n\
             ## Structured Snapshot Reference\n\n\
             `artifacts/kernel_pass_snapshot.json`\n\n\
             ## Inverse-Path Question\n\n\
             What does this completed pass reveal about the inverse path of the end use?\n\n\
             ## Boundary Reminder\n\n\
             Do not mutate the kernel. Interpret only.\n"
        );
    }

    #[test]
    fn desktop_status_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let output = build_desktop_status_output(&kernel_pass);

        assert!(output.starts_with("Hollow Grove status: one completed witnessed recursion\n\n"));
        assert!(output.contains("Canonical witness:\nPoint\n↓\nTriway\n↓\nFourway"));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
        assert!(output.contains("SYNTHESIS RECIPE\n\n  ID:\n    pixy_confusion"));
        assert!(output.contains("GREMLIN TINKER RECIPE\n\n  ID:\n    gremlin_tinker"));
        assert!(output.contains("Pixy Confusion Recipe"));
        assert!(output.contains("Gremlin Tinker Recipe"));
        assert!(output.contains("VERSION 2 DECISION"));
        assert!(output.contains("CURRENT-FAVORED"));
        assert!(output.contains("AURA-FAVORED"));
        assert!(output.contains("NEUTRAL"));
        assert!(output.contains("Source Recipe:\n    pixy_confusion"));
        assert!(output.contains("Source Recipe:\n    gremlin_tinker"));
        assert!(output.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy"));
        assert!(output.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin"));
    }

    #[test]
    fn desktop_status_displays_start_and_landed_frame_ids() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert!(output.contains("STARTING POINT\n\nFrame:\n  Hueman"));
        assert!(output.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy"));
        assert!(output.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin"));
    }

    #[test]
    fn desktop_status_displays_all_five_prism_values() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert!(output.contains("Stonebend / Body:\n    1"));
        assert!(output.contains("Flynt / Spirit:\n    1"));
        assert!(output.contains("Glaüshouse / Mind:\n    1"));
        assert!(output.contains("Sandmanor / Soul:"));
        assert!(output.contains("Minorian / Interior:\n      1"));
        assert!(output.contains("Minoan / Exterior:\n      1"));
    }

    #[test]
    fn desktop_status_renders_empty_flow_and_glow_as_none() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert_eq!(output.matches("FLOW\n\n  none").count(), 2);
        assert!(output.contains("FLOW\n\n  TinkerGrip"));
        assert_eq!(output.matches("GLOW\n\n  none").count(), 2);
        assert_eq!(output.matches("GLOW\n\n  Confusion").count(), 1);
    }

    #[test]
    fn desktop_status_reports_kiss_landing_state_change() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert!(output.contains("KISS LANDING\n\n  Scripts applied:\n    yes"));
        assert_eq!(
            output
                .matches("KISS LANDING\n\n  Scripts applied:\n    yes")
                .count(),
            2
        );
        assert!(output.contains("FrameState changed:\n    true"));
        assert!(output.contains("Point² produced:\n    true"));
        assert!(output.contains("MISS LANDING\n\n  Contact:\n    Miss"));
        assert!(output.contains("Scripts applied:\n    no"));
    }

    #[test]
    fn straight_and_curved_routes_both_display_preserved_frame_state() {
        let straight = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));
        let curved = build_desktop_status_output(&run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: crate::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        ));

        assert!(straight.contains("STARTING POINT\n\nFrame:\n  Hueman"));
        assert!(straight.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy"));
        assert!(straight.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin"));
        assert!(straight.contains("STARTING POINT\n\nFrame:\n  Hueman\n\nCURRENT PRISM"));
        assert!(straight.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy\n\nCURRENT PRISM"));
        assert!(straight.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin\n\nCURRENT PRISM"));
        assert!(straight.contains("Glaüshouse / Mind:\n    3"));
        assert!(straight.contains("Stonebend / Body:\n    3"));
        assert!(curved.contains("STARTING POINT\n\nFrame:\n  Hueman"));
        assert!(curved.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy"));
        assert!(curved.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin"));
        assert!(curved.contains("STARTING POINT\n\nFrame:\n  Hueman\n\nCURRENT PRISM"));
        assert!(curved.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy\n\nCURRENT PRISM"));
        assert!(curved.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin\n\nCURRENT PRISM"));
        assert!(curved.contains("Glaüshouse / Mind:\n    3"));
        assert!(curved.contains("Stonebend / Body:\n    3"));
    }

    #[test]
    fn rendering_frame_state_is_read_only() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let start_before = kernel_pass.start_frame_state().clone();
        let end_before = kernel_pass.end_frame_state().clone();

        let _output = build_desktop_status_output(&kernel_pass);

        assert_eq!(kernel_pass.start_frame_state(), &start_before);
        assert_eq!(kernel_pass.end_frame_state(), &end_before);
    }

    #[test]
    fn point_squared_and_landed_point_render_as_one_final_stage() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert_eq!(output.matches("POINT²\nLanded Point").count(), 2);
        assert!(!output.contains("Landed Point\n→ Point²"));
        assert!(!output.contains("AuraBeam\n→ Landed Point\n→ Point²"));
    }

    #[test]
    fn desktop_status_displays_the_canonical_recipe_and_compiled_scripts() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert!(output.contains("SYNTHESIS RECIPE\n\n  ID:\n    pixy_confusion"));
        assert!(output.contains("Name:\n    Pixy Confusion Recipe"));
        assert!(output.contains("Intended changes:\n    Glaüshouse / Mind +2"));
        assert!(output.contains("Learn Glow: Confusion"));
        assert!(output.contains("Change Frame: Pixy"));
        assert!(output.contains("1. ApplyPrismDelta"));
        assert!(output.contains("Stonebend / Body: 0"));
        assert!(output.contains("Flynt / Spirit: 0"));
        assert!(output.contains("Glaüshouse / Mind: +2"));
        assert!(output.contains("Minorian / Interior: 0"));
        assert!(output.contains("Minoan / Exterior: 0"));
        assert!(output.contains("SYNTHESIS SCRIPTS\n\n  status:\n    ready"));
        assert!(output.contains("2. AddGlow\n    Confusion"));
        assert!(output.contains("3. SetFrame\n    Pixy"));
        assert!(output.contains("GREMLIN TINKER RECIPE\n\n  ID:\n    gremlin_tinker"));
        assert!(output.contains("Name:\n    Gremlin Tinker Recipe"));
        assert!(output.contains("Intended changes:\n    Stonebend / Body +2"));
        assert!(output.contains("Learn Flow: TinkerGrip"));
        assert!(output.contains("Change Frame: Gremlin"));
        assert!(output.contains("2. AddFlow\n    TinkerGrip"));
        assert!(output.contains("3. SetFrame\n    Gremlin"));
    }

    #[test]
    fn desktop_status_displays_the_constructed_synthesis_aim() {
        let output = build_desktop_status_output(&run_kernel_cycle(Symptom::origin()));

        assert!(output.contains("SYNTHESIS AIM\n\n  ID:\n    pixy_confusion_aim"));
        assert!(output.contains("Source Recipe:\n    pixy_confusion"));
        assert!(output.contains("Manager:\n    HAL"));
        assert!(output.contains("Domain:\n    META"));
        assert!(output.contains("Relation:\n    PLEB ↔ META"));
        assert!(output.contains("Geometry:\n    curved"));
        assert!(output.contains("Function:\n    Connects"));
        assert!(output.contains("Bond:\n    One"));
        assert!(output.contains("Named Route:\n    unset"));
        assert!(output.contains("1. ApplyPrismDelta"));
        assert!(output.contains("2. AddGlow\n    Confusion"));
        assert!(output.contains("3. SetFrame\n    Pixy"));
        assert!(output.contains("Status:\n    prepared"));
        assert!(output.contains("FIRE\n\n  status:\n    committed"));
        assert!(output.contains("Contact:\n    Kiss"));
        assert!(output.contains("KISS LANDING\n\n  Scripts applied:\n    yes"));
        assert!(output.contains("Starting Frame:\n    Hueman"));
        assert!(output.contains("Point² Frame:\n    Pixy"));
        assert!(output.contains("CURRENT PRISM DELTA:\n    Stonebend / Body: 0"));
        assert!(output.contains("GLOW LEARNED:\n    Confusion"));
        assert!(output.contains("FLOW LEARNED:\n    none"));
        assert!(output.contains("FrameState changed:\n    true"));
        assert!(output.contains("Point² produced:\n    true"));
        assert!(output.contains("MISS LANDING\n\n  Contact:\n    Miss"));
        assert!(output.contains("SYNTHESIS AIM\n\n  ID:\n    gremlin_tinker_aim"));
        assert!(output.contains("Source Recipe:\n    gremlin_tinker"));
        assert!(output.contains("Manager:\n    Clouseau"));
        assert!(output.contains("Domain:\n    PLEB"));
        assert!(output.contains("Relation:\n    PLEB ↔ PLEB"));
        assert!(output.contains("Geometry:\n    straight"));
        assert!(output.contains("Function:\n    Locates"));
        assert!(output.contains("2. AddFlow\n    TinkerGrip"));
        assert!(output.contains("3. SetFrame\n    Gremlin"));
        assert!(output.contains("Point² Frame:\n    Gremlin"));
        assert!(output.contains("CURRENT PRISM DELTA:\n    Stonebend / Body: +2"));
        assert!(output.contains("GLOW LEARNED:\n    none"));
        assert!(output.contains("FLOW LEARNED:\n    TinkerGrip"));
        assert!(!output.contains("SYNTHESIS BEAM"));
        assert!(!output.contains("Transmission:\n"));
        assert!(output.contains("Version 1.1 Execution:\n    Hueman → Kiss → Gremlin Point²"));
        assert!(output.contains("Version 1.1 Execution:\n    Hueman → Kiss → Pixy Point²"));
        assert!(output.contains("Route Geometry: straight"));
        assert!(output.contains("Tie-break:\n  observed straight geometry"));
    }

    #[test]
    fn tree_output_remains_canonical() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            build_tree_output(&kernel_pass),
            "KernelPass\n\
             ├─ start: Point\n\
             ├─ triway\n\
             │  ├─ ways: [One, Two, Three]\n\
             ├─ fourway: Fourway\n\
             ├─ hollow_grove\n\
             │  ├─ bond: One\n\
             │  └─ atmosphere: [Two, Three]\n\
             ├─ current_seam: CurrentSeam [PlebExterior]\n\
             ├─ aura_beam: AuraBeam [BlepReturn]\n\
             └─ point_squared: Point² (Landed Point) [BlepArrival]"
        );
    }

    #[test]
    fn inverse_path_prompt_preserves_the_given_witness_exactly() {
        assert_eq!(
            build_inverse_path_prompt(CANONICAL_WITNESS),
            format!(
                "{CANONICAL_WITNESS}\n\nWhat does this completed pass reveal about the inverse path of the end use?"
            )
        );
    }
}
