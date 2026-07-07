#![allow(dead_code)]

use std::fs;
use std::io;
use std::path::Path;

pub const SNAPSHOT_ARTIFACT_PATH: &str = "artifacts/kernel_pass_snapshot.json";
pub const PROMPT_ARTIFACT_PATH: &str = "artifacts/consumer_prompt.md";
pub const DESKTOP_STATUS_ARTIFACT_PATH: &str = "artifacts/desktop_status.txt";
pub const CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_base.md";
pub const ARTIFACT_INDEX_PATH: &str = "artifacts/index.md";
pub const CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_state.md";
pub const CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_sequence.md";
pub const CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_topology.md";
pub const CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH: &str = "artifacts/current_synthesis_clients.md";
pub const CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_choice.md";
pub const CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_contract.md";
pub const CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH: &str = "artifacts/current_synthesis_preview.md";
pub const CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_operational.md";
pub const CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_selection.md";
pub const CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_consequence.md";
pub const CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_readiness.md";
pub const CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_execution_spec.md";
pub const CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_behavior_rules.md";
pub const CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_transition_pm_to_le.md";
pub const CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_activation_gate.md";

pub fn read_artifact(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_artifact(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)
}

pub fn extract_canonical_witness(desktop_status: &str) -> io::Result<&str> {
    let prefix = "Canonical witness:\n";
    let suffix = "\n\nNote: read-only desktop artifact";

    let start = desktop_status.find(prefix).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "desktop status artifact missing canonical witness header",
        )
    })? + prefix.len();

    let end = desktop_status[start..]
        .find(suffix)
        .map(|offset| start + offset)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "desktop status artifact missing canonical witness footer",
            )
        })?;

    Ok(&desktop_status[start..end])
}

pub fn build_current_synthesis_base_from_artifacts(
    snapshot: &str,
    prompt: &str,
    desktop_status: &str,
) -> io::Result<String> {
    let canonical_witness = extract_canonical_witness(desktop_status)?;

    Ok(format!(
        "# Current Synthesis Base\n\n\
         ## Hollow Grove Status\n\n\
         Hollow Grove remains the stable recursive core.\n\n\
         ## KernelPass Status\n\n\
         `KernelPass` remains the canonical deterministic witness of one completed recursion.\n\n\
         ## Canonical Witness\n\n\
         ```text\n\
         {canonical_witness}\n\
         ```\n\n\
         ## Artifact Layer Status\n\n\
         - `artifacts/kernel_pass_snapshot.json`: present and read-only.\n\
         - `artifacts/consumer_prompt.md`: present and read-only.\n\
         - `artifacts/desktop_status.txt`: present and read-only.\n\n\
         Snapshot bytes: {}.\n\
         Prompt bytes: {}.\n\
         Desktop status bytes: {}.\n\n\
         ## Current Synthesis\n\n\
         Current Synthesis is the OS layer built on Hollow Grove.\n\n\
         ## Deferral\n\n\
         - `PLEB` and `META` are deferred.\n\
         - HAL is deferred.\n\
         - Clouseau is deferred.\n\
         - `niri`/`river` are untouched.\n",
        snapshot.len(),
        prompt.len(),
        desktop_status.len()
    ))
}

pub fn build_current_synthesis_state_from_artifacts(
    current_synthesis_base: &str,
    artifact_index: &str,
) -> String {
    format!(
        "# Current Synthesis State\n\n\
         ## Current Mode\n\n\
         Current Synthesis Base\n\n\
         ## Available Artifacts\n\n\
         - `artifacts/kernel_pass_snapshot.json`\n\
         - `artifacts/consumer_prompt.md`\n\
         - `artifacts/desktop_status.txt`\n\
         - `artifacts/current_synthesis_base.md`\n\
         - `artifacts/current_synthesis_state.md`\n\
         - `artifacts/current_synthesis_sequence.md`\n\
         - `artifacts/current_synthesis_topology.md`\n\
         - `artifacts/current_synthesis_clients.md`\n\
         - `artifacts/current_synthesis_choice.md`\n\
         - `artifacts/current_synthesis_contract.md`\n\
         - `artifacts/current_synthesis_preview.md`\n\
         - `artifacts/current_synthesis_operational.md`\n\
         - `artifacts/current_synthesis_selection.md`\n\
         - `artifacts/current_synthesis_consequence.md`\n\
         - `artifacts/current_synthesis_readiness.md`\n\
         - `artifacts/current_synthesis_execution_spec.md`\n\
         - `artifacts/current_synthesis_behavior_rules.md`\n\
         - `artifacts/current_synthesis_transition_pm_to_le.md`\n\
         - `artifacts/current_synthesis_activation_gate.md`\n\
         - `artifacts/index.md`\n\n\
         ## Active Clients\n\n\
         - `current_synthesis_base`\n\
         - `current_synthesis_state`\n\
         - `current_synthesis_sequence`\n\
         - `current_synthesis_topology`\n\
         - `current_synthesis_clients`\n\
         - `current_synthesis_choice`\n\
         - `current_synthesis_contract`\n\
         - `current_synthesis_preview`\n\
         - `current_synthesis_operational`\n\
         - `current_synthesis_selection`\n\
         - `current_synthesis_consequence`\n\
         - `current_synthesis_readiness`\n\
         - `current_synthesis_execution_spec`\n\
         - `current_synthesis_behavior_rules`\n\
         - `current_synthesis_transition_pm_to_le`\n\
         - `current_synthesis_activation_gate`\n\
         - `current_synthesis`\n\n\
         ## Next Possible Action\n\n\
         Keep the route preview, operational view, selection, consequence, readiness, execution spec, behavior rules, transition rule, and activation gate read-only until Current Synthesis is explicitly activated for behavior.\n\n\
         ## Deferred Status\n\n\
         - HAL status: deferred\n\
         - Clouseau status: deferred\n\
         - `PLEB`/`META` execution: deferred\n\
         - `niri`/`river` integration: deferred\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis base bytes: {}.\n\
         Artifact index bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Current Synthesis consumes Hollow Grove; Hollow Grove does not know Current Synthesis exists.\n",
        current_synthesis_base.len(),
        artifact_index.len()
    )
}

pub fn build_current_synthesis_sequence_from_artifacts(
    current_synthesis_base: &str,
    current_synthesis_state: &str,
) -> String {
    format!(
        "# Current Synthesis Sequence\n\n\
         ## Sequence Lock\n\n\
         ```text\n\
         P/M\n\
         ↓\n\
         L/E\n\
         ↓\n\
         E/T\n\
         ↓\n\
         B/A\n\
         ```\n\n\
         ## Joint Model\n\n\
         Each paired joint has a `PLEB` side, a `META` side, three possible arms of movement on each side, one bonded arm, and unused arms that remain as clue context, environmental residue, or route material.\n\n\
         ## Client Sides\n\n\
         - HAL belongs to `META`.\n\
         - Clouseau belongs to `PLEB`.\n\n\
         ## Topology Status\n\n\
         Topology is downstream from this sequence.\n\n\
         ## Deferral\n\n\
         - `PLEB`/`META` execution deferred\n\
         - HAL behavior deferred\n\
         - Clouseau behavior deferred\n\
         - `niri`/`river` integration deferred\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis base bytes: {}.\n\
         Current Synthesis state bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         This sequence belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_base.len(),
        current_synthesis_state.len()
    )
}

pub fn build_current_synthesis_topology_from_artifacts(
    current_synthesis_sequence: &str,
    current_synthesis_state: &str,
) -> String {
    format!(
        "# Current Synthesis Topology\n\n\
         ## Joint Order\n\n\
         ```text\n\
         P/M\n\
         ↓\n\
         L/E\n\
         ↓\n\
         E/T\n\
         ↓\n\
         B/A\n\
         ```\n\n\
         ## Adjacency\n\n\
         - `P/M` connects to `L/E`.\n\
         - `L/E` connects to `E/T`.\n\
         - `E/T` connects to `B/A`.\n\n\
         ## Side Assignment\n\n\
         - HAL is assigned to the `META` side of each joint.\n\
         - Clouseau is assigned to the `PLEB` side of each joint.\n\n\
         ## Inverse Curved Route\n\n\
         The inverse curved route runs beneath the plains as downstream Current Synthesis geography.\n\n\
         ```text\n\
         Aura Basin\n\
         ↓\n\
         Aura Fields\n\
         ↓\n\
         Aura Beach\n\
         ```\n\n\
         These remain route regions and route stations, not Hollow Grove layers.\n\n\
         ## Deferral\n\n\
         - traversal deferred\n\
         - `PLEB`/`META` execution deferred\n\
         - HAL behavior deferred\n\
         - Clouseau behavior deferred\n\
         - `niri`/`river` integration deferred\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis sequence bytes: {}.\n\
         Current Synthesis state bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Topology belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_sequence.len(),
        current_synthesis_state.len()
    )
}

pub fn build_current_synthesis_clients_from_artifacts(
    current_synthesis_topology: &str,
    current_synthesis_sequence: &str,
) -> String {
    format!(
        "# Current Synthesis Clients\n\n\
         ## Placement Lock\n\n\
         - HAL is placed on the `META` side of each joint.\n\
         - Clouseau is placed on the `PLEB` side of each joint.\n\n\
         ## Joint Placement\n\n\
         - `P/M`: HAL on `META`, Clouseau on `PLEB`.\n\
         - `L/E`: HAL on `META`, Clouseau on `PLEB`.\n\
         - `E/T`: HAL on `META`, Clouseau on `PLEB`.\n\
         - `B/A`: HAL on `META`, Clouseau on `PLEB`.\n\n\
         ## Behavior Status\n\n\
         - no movement\n\
         - no traversal\n\
         - no automation\n\
         - no `PLEB`/`META` execution\n\
         - no runtime state\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis topology bytes: {}.\n\
         Current Synthesis sequence bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Client placement belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_topology.len(),
        current_synthesis_sequence.len()
    )
}

pub fn build_current_synthesis_choice_from_artifacts(
    current_synthesis_clients: &str,
    current_synthesis_topology: &str,
) -> String {
    format!(
        "# Current Synthesis Choice\n\n\
         ## Available Sides\n\n\
         - `PLEB` is available.\n\
         - `META` is available.\n\n\
         ## Placement Lock\n\n\
         - HAL remains assigned to `META`.\n\
         - Clouseau remains assigned to `PLEB`.\n\n\
         ## Choice Status\n\n\
         - user path choice is not executing yet\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no runtime state\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis clients bytes: {}.\n\
         Current Synthesis topology bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Path choice belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_clients.len(),
        current_synthesis_topology.len()
    )
}

pub fn build_current_synthesis_contract_from_artifacts(
    current_synthesis_choice: &str,
    current_synthesis_clients: &str,
) -> String {
    format!(
        "# Current Synthesis Contract\n\n\
         ## Choice Meaning\n\n\
         - If `PLEB` is chosen, HAL remains on `META` and Clouseau occupies `PLEB`.\n\
         - If `META` is chosen, HAL remains on `META` and Clouseau occupies `PLEB`.\n\n\
         ## Complementary Relation\n\n\
         HAL remains aligned with `META`.\n\n\
         Clouseau remains aligned with `PLEB`.\n\n\
         The chosen side and the complementary side remain distinct without execution.\n\n\
         ## Contract Status\n\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no runtime state\n\
         - no execution yet\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis choice bytes: {}.\n\
         Current Synthesis clients bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Route contract belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_choice.len(),
        current_synthesis_clients.len()
    )
}

pub fn build_current_synthesis_preview_from_artifacts(
    current_synthesis_contract: &str,
    current_synthesis_sequence: &str,
) -> String {
    format!(
        "# Current Synthesis Preview\n\n\
         ## `PLEB` Chosen\n\n\
         - HAL remains on `META`.\n\
         - Clouseau occupies `PLEB`.\n\
         - Joint order remains `P/M -> L/E -> E/T -> B/A`.\n\n\
         ## `META` Chosen\n\n\
         - HAL remains on `META`.\n\
         - Clouseau remains on `PLEB`.\n\
         - Joint order remains `P/M -> L/E -> E/T -> B/A`.\n\n\
         ## Preview Status\n\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no runtime state\n\
         - no execution\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis contract bytes: {}.\n\
         Current Synthesis sequence bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Route preview belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_contract.len(),
        current_synthesis_sequence.len()
    )
}

pub fn build_current_synthesis_operational_from_artifacts(
    current_synthesis_preview: &str,
    current_synthesis_contract: &str,
) -> String {
    format!(
        "# Current Synthesis Operational View\n\n\
         ## `PLEB` Side\n\n\
         - Clouseau belongs to `PLEB` as the clue-side client.\n\
         - `PLEB` remains the straight-side occupancy described by Current Synthesis.\n\
         - HAL does not occupy `PLEB`.\n\
         - no traversal or execution occurs here yet\n\n\
         ## `META` Side\n\n\
         - HAL belongs to `META` as the watch-side client.\n\
         - `META` remains the bent-side occupancy described by Current Synthesis.\n\
         - Clouseau does not occupy `META`.\n\
         - no traversal or execution occurs here yet\n\n\
         ## Shared Limits\n\n\
         - no automation\n\
         - no movement\n\
         - no runtime state\n\
         - no path execution\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis preview bytes: {}.\n\
         Current Synthesis contract bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Operational meaning belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_preview.len(),
        current_synthesis_contract.len()
    )
}

pub fn build_current_synthesis_selection_from_artifacts(
    current_synthesis_choice: &str,
    current_synthesis_operational: &str,
) -> String {
    format!(
        "# Current Synthesis Selection\n\n\
         ## Selected Side\n\n\
         - `PLEB`\n\n\
         ## Complementary Side\n\n\
         - `META`\n\n\
         ## Placement Lock\n\n\
         - HAL remains on `META`.\n\
         - Clouseau remains on `PLEB`.\n\n\
         ## Selection Status\n\n\
         - deterministic read-only selection\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no runtime state\n\
         - no execution\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis choice bytes: {}.\n\
         Current Synthesis operational bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Selection belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_choice.len(),
        current_synthesis_operational.len()
    )
}

pub fn build_current_synthesis_consequence_from_artifacts(
    current_synthesis_selection: &str,
    current_synthesis_operational: &str,
) -> String {
    format!(
        "# Current Synthesis Consequence\n\n\
         ## Selected Side Consequence\n\n\
         - `PLEB` remains the occupied selected side.\n\
         - Clouseau remains on `PLEB`.\n\
         - `PLEB` remains descriptive only.\n\n\
         ## Complementary Side Consequence\n\n\
         - `META` remains the complementary side.\n\
         - HAL remains on `META`.\n\
         - `META` remains descriptive only.\n\n\
         ## Still Deferred\n\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no runtime state\n\
         - no path execution\n\
         - no feedback into Hollow Grove\n\n\
         ## Cannot Happen Yet\n\n\
         - HAL does not automate\n\
         - Clouseau does not interpret live behavior\n\
         - `PLEB`/`META` do not execute as routes\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis selection bytes: {}.\n\
         Current Synthesis operational bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Consequence belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_selection.len(),
        current_synthesis_operational.len()
    )
}

pub fn build_current_synthesis_readiness_from_artifacts(
    current_synthesis_consequence: &str,
    current_synthesis_selection: &str,
) -> String {
    format!(
        "# Current Synthesis Readiness\n\n\
         ## Locked\n\n\
         - sequence locked\n\
         - topology locked\n\
         - client placement locked\n\
         - choice locked\n\
         - contract locked\n\
         - preview locked\n\
         - operational view locked\n\
         - selection locked\n\
         - consequence locked\n\n\
         ## Missing Before Execution\n\n\
         - route execution rules are not defined\n\
         - `PLEB`/`META` behavior is not active\n\
         - HAL automation is not enabled\n\
         - Clouseau live interpretation is not enabled\n\
         - runtime state is not introduced\n\n\
         ## Current Readiness\n\n\
         - `PLEB` cannot act yet\n\
         - `META` cannot act yet\n\
         - HAL cannot automate yet\n\
         - Clouseau cannot interpret live behavior yet\n\
         - Current Synthesis remains read-only\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis consequence bytes: {}.\n\
         Current Synthesis selection bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Readiness belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_consequence.len(),
        current_synthesis_selection.len()
    )
}

pub fn build_current_synthesis_execution_spec_from_artifacts(
    current_synthesis_readiness: &str,
    current_synthesis_consequence: &str,
) -> String {
    format!(
        "# Current Synthesis Execution Spec\n\n\
         ## Preconditions\n\n\
         - route execution rules must be defined explicitly\n\
         - `PLEB` and `META` behavior must be specified explicitly\n\
         - runtime state must be introduced deliberately\n\
         - HAL permissions must be declared before automation\n\
         - Clouseau live interpretation rules must be declared before observation\n\n\
         ## `PLEB` Execution Would Need\n\n\
         - a defined `PLEB` route step model\n\
         - a defined transition rule between joints\n\
         - a defined boundary for clue production\n\n\
         ## `META` Execution Would Need\n\n\
         - a defined `META` route step model\n\
         - a defined watch or traversal rule between joints\n\
         - a defined boundary for complementary occupancy\n\n\
         ## HAL Would Need Before Automation\n\n\
         - explicit automation scope\n\
         - explicit allowed actions\n\
         - explicit prohibition on Hollow Grove mutation\n\n\
         ## Clouseau Would Need Before Live Interpretation\n\n\
         - explicit observation scope\n\
         - explicit clue or residue inputs\n\
         - explicit prohibition on control or automation\n\n\
         ## Activation Status\n\n\
         - not active\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no live interpretation\n\
         - no runtime state\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis readiness bytes: {}.\n\
         Current Synthesis consequence bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Execution spec belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_readiness.len(),
        current_synthesis_consequence.len()
    )
}

pub fn build_current_synthesis_behavior_rules_from_artifacts(
    current_synthesis_execution_spec: &str,
    current_synthesis_selection: &str,
) -> String {
    format!(
        "# Current Synthesis Behavior Rules\n\n\
         ## Rule 1: Occupancy\n\n\
         - the selected side remains occupied by its assigned client\n\
         - the complementary side remains occupied by its assigned client\n\n\
         ## Rule 2: Joint Order\n\n\
         - any future route behavior must follow `P/M -> L/E -> E/T -> B/A`\n\
         - no joint may be skipped\n\n\
         ## Rule 3: `PLEB`\n\n\
         - `PLEB` remains the straight-side route context\n\
         - Clouseau remains the `PLEB` client\n\
         - clue production stays within Current Synthesis\n\n\
         ## Rule 4: `META`\n\n\
         - `META` remains the bent-side route context\n\
         - HAL remains the `META` client\n\
         - complementary occupancy stays within Current Synthesis\n\n\
         ## Rule 5: HAL Scope\n\n\
         - HAL may act only within explicit Current Synthesis permissions\n\
         - HAL never mutates Hollow Grove\n\
\n\
         ## Rule 6: Clouseau Scope\n\n\
         - Clouseau may interpret only within explicit Current Synthesis permissions\n\
         - Clouseau never controls route execution\n\
\n\
         ## Activation Status\n\n\
         - rules defined\n\
         - not active\n\
         - no traversal\n\
         - no movement\n\
         - no automation\n\
         - no live interpretation\n\
         - no runtime state\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis execution spec bytes: {}.\n\
         Current Synthesis selection bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Behavior rules belong to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_execution_spec.len(),
        current_synthesis_selection.len()
    )
}

pub fn build_current_synthesis_transition_pm_to_le_from_artifacts(
    current_synthesis_behavior_rules: &str,
    current_synthesis_topology: &str,
) -> String {
    format!(
        "# Current Synthesis Transition Rule `P/M -> L/E`\n\n\
         ## Transition Condition\n\n\
         - the joint order must remain `P/M -> L/E -> E/T -> B/A`\n\
         - `PLEB` and `META` occupancy must remain locked\n\
         - this rule remains declarative only\n\n\
         ## `PLEB` Occupancy\n\n\
         - Clouseau remains on `PLEB`\n\
         - straight-side occupancy carries from `P/M` to `L/E`\n\n\
         ## `META` Occupancy\n\n\
         - HAL remains on `META`\n\
         - bent-side occupancy carries from `P/M` to `L/E`\n\n\
         ## HAL Observation\n\n\
         - HAL may observe complementary alignment at `P/M` and `L/E`\n\
         - HAL may not automate movement\n\n\
         ## Clouseau Observation\n\n\
         - Clouseau may observe clue continuity at `P/M` and `L/E`\n\
         - Clouseau may not control movement\n\n\
         ## Still Forbidden\n\n\
         - route traversal\n\
         - route movement\n\
         - automation\n\
         - live interpretation\n\
         - runtime state\n\
         - feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis behavior rules bytes: {}.\n\
         Current Synthesis topology bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Transition rules belong to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_behavior_rules.len(),
        current_synthesis_topology.len()
    )
}

pub fn build_current_synthesis_activation_gate_from_artifacts(
    current_synthesis_transition_pm_to_le: &str,
    current_synthesis_readiness: &str,
) -> String {
    format!(
        "# Current Synthesis Activation Gate\n\n\
         ## Gate Result\n\n\
         - activation denied\n\
         - Current Synthesis remains read-only\n\n\
         ## Reason\n\n\
         - the `P/M -> L/E` transition rule is defined but not active\n\
         - readiness confirms route behavior is not enabled\n\
         - HAL automation is not enabled\n\
         - Clouseau live interpretation is not enabled\n\
         - runtime state has not been introduced\n\n\
         ## Allowed Now\n\n\
         - deterministic artifact generation\n\
         - boundary documentation\n\
         - read-only evaluation of Current Synthesis structure\n\n\
         ## Not Allowed Now\n\n\
         - route traversal\n\
         - route movement\n\
         - HAL automation\n\
         - Clouseau live interpretation\n\
         - feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis transition rule bytes: {}.\n\
         Current Synthesis readiness bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Activation gating belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_transition_pm_to_le.len(),
        current_synthesis_readiness.len()
    )
}
