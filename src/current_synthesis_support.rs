#![allow(dead_code)]

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use hollow_grove::SnapshotBoundary;

pub const SNAPSHOT_ARTIFACT_PATH: &str = hollow_grove::SNAPSHOT_ARTIFACT_PATH;
pub const PROMPT_ARTIFACT_PATH: &str = hollow_grove::PROMPT_ARTIFACT_PATH;
pub const DESKTOP_STATUS_ARTIFACT_PATH: &str = hollow_grove::DESKTOP_STATUS_ARTIFACT_PATH;
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
pub const CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_collision_relay.md";
pub const CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_activation_gate.md";
pub const DEFAULT_ARTIFACT_INDEX: &str = "# Artifact Index\n\n## Boundary\n\n`Point -> Triway -> Fourway -> HollowGrove -> CurrentSeam -> AuraBeam -> Point² (Landed Point) -> KernelPass -> Client Artifacts -> Current Synthesis -> Hueman`\n";
static ARTIFACT_INDEX_CACHE: OnceLock<Mutex<Option<CachedArtifactIndex>>> = OnceLock::new();
static CURRENT_SYNTHESIS_EXECUTION_SPEC_PREFIX: OnceLock<String> = OnceLock::new();

struct CachedArtifactIndex {
    path: PathBuf,
    modified: Option<SystemTime>,
    len: u64,
    contents: String,
}

pub fn read_artifact(path: &Path) -> io::Result<String> {
    hollow_grove::read_text_artifact(path)
}

pub fn write_artifact(path: &Path, contents: &str) -> io::Result<()> {
    hollow_grove::write_text_artifact(path, contents)
}

pub fn ensure_artifact_index(path: &Path) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }

    write_artifact(path, DEFAULT_ARTIFACT_INDEX)
}

fn artifact_index_metadata(path: &Path) -> io::Result<(Option<SystemTime>, u64)> {
    let metadata = fs::metadata(path)?;
    Ok((metadata.modified().ok(), metadata.len()))
}

pub fn load_artifact_index(path: &Path) -> io::Result<String> {
    let mut wrote_default = false;
    let (modified, len) = match artifact_index_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            write_artifact(path, DEFAULT_ARTIFACT_INDEX)?;
            wrote_default = true;
            artifact_index_metadata(path)?
        }
        Err(error) => return Err(error),
    };

    let cache = ARTIFACT_INDEX_CACHE.get_or_init(|| Mutex::new(None));
    {
        let guard = cache
            .lock()
            .expect("artifact index cache should not poison");
        if let Some(cached) = guard.as_ref() {
            if cached.path == path && cached.modified == modified && cached.len == len {
                return Ok(cached.contents.clone());
            }
        }
    }

    let contents = if wrote_default {
        String::from(DEFAULT_ARTIFACT_INDEX)
    } else {
        read_artifact(path)?
    };

    let mut guard = cache
        .lock()
        .expect("artifact index cache should not poison");
    *guard = Some(CachedArtifactIndex {
        path: path.to_path_buf(),
        modified,
        len,
        contents: contents.clone(),
    });
    Ok(contents)
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

pub fn extract_prompt_witness(prompt: &str) -> io::Result<&str> {
    let prefix = "## Canonical Witness\n\n```text\n";
    let suffix = "\n```\n\n## Structured Snapshot Reference";

    let start = prompt.find(prefix).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "prompt artifact missing canonical witness header",
        )
    })? + prefix.len();

    let end = prompt[start..]
        .find(suffix)
        .map(|offset| start + offset)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "prompt artifact missing canonical witness footer",
            )
        })?;

    Ok(&prompt[start..end])
}

fn build_frozen_boundary_section(
    heading: &str,
    snapshot_boundary: &SnapshotBoundary,
    note: &str,
) -> String {
    let mut output = String::with_capacity(320);
    let _ = write!(
        output,
        "## {heading}\n\n\
         - exterior ingress: `{}`\n\
         - complementary return: `{}`\n\
         - landed route: `{}`\n\
         - universal landed point: `{}`\n\
         - {note}\n\n",
        snapshot_boundary.grove_seam_route(),
        snapshot_boundary.hollow_beam_route(),
        snapshot_boundary.landing_route(),
        snapshot_boundary.landed_point(),
    );
    output
}

pub fn build_current_synthesis_base_from_boundary(
    snapshot_boundary: &SnapshotBoundary,
    snapshot_len: usize,
    prompt: &str,
    desktop_status: &str,
) -> io::Result<String> {
    let prompt_witness = extract_prompt_witness(prompt)?;
    let desktop_witness = extract_canonical_witness(desktop_status)?;

    if prompt_witness != snapshot_boundary.canonical_witness() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "prompt artifact canonical witness does not match snapshot boundary",
        ));
    }

    if desktop_witness != snapshot_boundary.canonical_witness() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "desktop status canonical witness does not match snapshot boundary",
        ));
    }

    let frozen_boundary_section = build_frozen_boundary_section(
        "Frozen Snapshot Boundary",
        snapshot_boundary,
        "Current Synthesis consumes this boundary as its primary Hollow Grove interface.",
    );

    let canonical_witness = snapshot_boundary.canonical_witness();
    let mut output =
        String::with_capacity(1_500 + canonical_witness.len() + frozen_boundary_section.len());
    output.push_str(
        "# Current Synthesis Base\n\n\
         ## Hollow Grove Status\n\n\
         Hollow Grove remains the stable recursive core.\n\n\
         ## KernelPass Status\n\n\
         `KernelPass` remains the canonical deterministic witness of one completed recursion.\n\n\
         ## Canonical Witness\n\n\
         ```text\n",
    );
    output.push_str(canonical_witness);
    output.push_str(
        "\n\
         ```\n\n",
    );
    output.push_str(&frozen_boundary_section);
    output.push_str(
        "## Artifact Layer Status\n\n\
         - `artifacts/kernel_pass_snapshot.json`: present and read-only.\n\
         - `artifacts/consumer_prompt.md`: present and read-only.\n\
         - `artifacts/desktop_status.txt`: present and read-only.\n\n",
    );
    let _ = writeln!(output, "Snapshot bytes: {snapshot_len}.");
    let _ = writeln!(output, "Prompt bytes: {}.", prompt.len());
    let _ = writeln!(output, "Desktop status bytes: {}.", desktop_status.len());
    output.push_str(
        "\n\
         ## Current Synthesis\n\n\
         Current Synthesis is the operating layer built on Hollow Grove and consumed later by Hueman.\n\
         At this layer, unresolved route material may later divide into `dark current` or `hollow current`, and into `reflective aura` or `holographic aura`.\n\n\
         ## Vertical Position\n\n\
         - Current Synthesis consumes the frozen snapshot boundary as its primary Hollow Grove interface.\n\
         - Prompt and desktop artifacts remain mirrored read-only witnesses of that same boundary.\n\
         - Hueman remains the world layer above Current Synthesis.\n\
         - no feedback into Hollow Grove\n\n\
         ## Deferral\n\n\
         - `PLEB` and `META` are deferred.\n\
         - HAL is deferred.\n\
         - Clouseau is deferred.\n\
         - `niri`/`river` are untouched.\n",
    );
    Ok(output)
}

pub fn build_current_synthesis_base_from_artifacts(
    snapshot: &str,
    prompt: &str,
    desktop_status: &str,
) -> io::Result<String> {
    let snapshot_boundary = SnapshotBoundary::parse(snapshot)?;
    build_current_synthesis_base_from_boundary(
        &snapshot_boundary,
        snapshot.len(),
        prompt,
        desktop_status,
    )
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
         - `artifacts/current_synthesis_collision_relay.md`\n\
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
         - `current_synthesis_collision_relay`\n\
         - `current_synthesis_activation_gate`\n\
         - `current_synthesis`\n\n\
         ## Next Possible Action\n\n\
         Keep the route preview, operational view, selection, consequence, readiness, execution spec, behavior rules, transition rule, collision relay, and activation gate read-only until Current Synthesis is explicitly activated for behavior.\n\n\
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
         Each paired joint has a `PLEB` side, a `META` side, three possible arms of movement on each side, one bonded arm, and unused arms that remain as clue context, environmental residue, or route material.\n\
         Each `META` letter faces its `PLEB` counterpart across the same joint.\n\
         Each side carries three available arms toward that counterpart.\n\
         One arm per side bonds into the selected link while the remaining arm weight stays available for later downstream reading.\n\n\
         ## Unbonded Resolution\n\n\
         - bonded arms remain the selected route through the joint\n\
         - unbonded arms do not disappear after bond selection\n\
         - unbonded arms may later resolve into `current` or `aura`\n\
         - `current` later divides into `dark current` or `hollow current`\n\
         - `aura` later divides into `reflective aura` or `holographic aura`\n\
         - that later resolution depends on downstream physics rather than kernel bond selection alone\n\n\
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

pub fn build_current_synthesis_topology_from_boundary(
    snapshot_boundary: &SnapshotBoundary,
    snapshot_len: usize,
    current_synthesis_sequence: &str,
    current_synthesis_state: &str,
) -> String {
    let frozen_boundary_section = build_frozen_boundary_section(
        "Frozen Kernel Entry Boundary",
        snapshot_boundary,
        "Current Synthesis begins downstream from this frozen Hollow Grove boundary rather than recomputing it.",
    );

    format!(
        "# Current Synthesis Topology\n\n\
         {}\
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
         - Clouseau is assigned to the `PLEB` side of each joint.\n\
         - Cleo (short for Cleopatra) is assigned beneath Clouseau's side of the joint axis to follow Clouseau through the underground inverse curved structures.\n\n\
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
         ## Underground Inverse Structures\n\n\
         - four underground inverse curved lines run on the `PLEB` side\n\
         - four underground inverse curved lines run on the `META` side\n\
         - each side mirrors the visible Hueman border names: Stairway to Heaven, Riptide, Current Seanad, and Mount Aura\n\
         - Cleo follows Clouseau underground while watching these curved lines without taking `PLEB` or `META` occupancy\n\n\
         ## Route Material Families\n\n\
         - route material may present as `dark current` or `hollow current`\n\
         - route material may present as `reflective aura` or `holographic aura`\n\
         - subtype presence does not change joint order or side assignment\n\n\
         ## Deferral\n\n\
         - traversal deferred\n\
         - `PLEB`/`META` execution deferred\n\
         - HAL behavior deferred\n\
         - Clouseau behavior deferred\n\
         - Cleo behavior deferred\n\
         - `niri`/`river` integration deferred\n\n\
         ## Artifact Inputs\n\n\
         Snapshot bytes: {}.\n\
         Current Synthesis sequence bytes: {}.\n\
         Current Synthesis state bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Topology belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        frozen_boundary_section,
        snapshot_len,
        current_synthesis_sequence.len(),
        current_synthesis_state.len()
    )
}

pub fn build_current_synthesis_topology_from_artifacts(
    snapshot: &str,
    current_synthesis_sequence: &str,
    current_synthesis_state: &str,
) -> String {
    let snapshot_boundary =
        SnapshotBoundary::parse(snapshot).expect("snapshot boundary should parse");

    build_current_synthesis_topology_from_boundary(
        &snapshot_boundary,
        snapshot.len(),
        current_synthesis_sequence,
        current_synthesis_state,
    )
}

pub fn build_current_synthesis_clients_from_boundary(
    snapshot_boundary: &SnapshotBoundary,
    snapshot_len: usize,
    current_synthesis_topology: &str,
    current_synthesis_sequence: &str,
) -> String {
    let frozen_boundary_section = build_frozen_boundary_section(
        "Frozen Kernel Placement Boundary",
        snapshot_boundary,
        "Client placement begins downstream from this frozen Hollow Grove boundary and does not recompute it.",
    );

    format!(
        "# Current Synthesis Clients\n\n\
         {}\
         ## Placement Lock\n\n\
         - HAL is placed on the `META` side of each joint.\n\
         - Clouseau is placed on the `PLEB` side of each joint.\n\
         - Cleo is placed beneath Clouseau's route to follow underground inverse curved structures.\n\n\
         ## Joint Placement\n\n\
         - `P/M`: HAL on `META`, Clouseau on `PLEB`.\n\
         - `L/E`: HAL on `META`, Clouseau on `PLEB`.\n\
         - `E/T`: HAL on `META`, Clouseau on `PLEB`.\n\
         - `B/A`: HAL on `META`, Clouseau on `PLEB`.\n\n\
         ## Underground Placement\n\n\
         - `P/M`, `L/E`, `E/T`, and `B/A`: Cleo remains beneath Clouseau's route rather than occupying either side.\n\
         - Cleo follows Clouseau underground while watching four underground inverse curved lines on each side.\n\n\
         ## Behavior Status\n\n\
         - no movement\n\
         - no traversal\n\
         - no automation\n\
         - no `PLEB`/`META` execution\n\
         - no Cleo live observation\n\
         - no runtime state\n\n\
         ## Artifact Inputs\n\n\
         Snapshot bytes: {}.\n\
         Current Synthesis topology bytes: {}.\n\
         Current Synthesis sequence bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Client placement belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        frozen_boundary_section,
        snapshot_len,
        current_synthesis_topology.len(),
        current_synthesis_sequence.len()
    )
}

pub fn build_current_synthesis_clients_from_artifacts(
    snapshot: &str,
    current_synthesis_topology: &str,
    current_synthesis_sequence: &str,
) -> io::Result<String> {
    let snapshot_boundary = SnapshotBoundary::parse(snapshot)?;

    Ok(build_current_synthesis_clients_from_boundary(
        &snapshot_boundary,
        snapshot.len(),
        current_synthesis_topology,
        current_synthesis_sequence,
    ))
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
         ## Mirror Axis\n\n\
         - if the user is read through HAL on `META`, Clouseau remains the opposite `PLEB` witness\n\
         - if the user is read through Clouseau on `PLEB`, HAL remains the opposite `META` witness\n\
         - the user-facing side and the opposing side stay mirrored across one axis\n\
         - the axis may not collapse both clients into one side\n\n\
         ## HAL/Cleo Collision Relay\n\n\
         - when HAL and Cleo collide at one joint crossing, HAL may contribute surface-side complementary alignment\n\
         - when HAL and Cleo collide at one joint crossing, Cleo may contribute underground route continuity from below Clouseau's path\n\
         - the useful relay function is to confirm that one visible route condition and one underground inverse condition belong to the same event body\n\
         - no live relay executes yet\n\n\
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
         ## HAL/Cleo Shared Function\n\n\
         - when HAL and Cleo intersect, HAL supplies surface-side alignment while Cleo supplies underground continuity beneath Clouseau's route\n\
         - the shared function is route confirmation across above-ground and underground readings at the same crossing\n\
         - no live relay or packet exchange occurs yet\n\n\
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
         ## Collision Relay Consequence\n\n\
         - HAL and Cleo may be declared as a paired relay where they collide.\n\
         - the relay would share surface alignment and underground continuity as one confirmation body.\n\
         - the relay remains descriptive only.\n\n\
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
         - Cleo underground observation is not enabled\n\
         - HAL/Cleo collision relay is not enabled\n\
         - runtime state is not introduced\n\n\
         ## Current Readiness\n\n\
         - `PLEB` cannot act yet\n\
         - `META` cannot act yet\n\
         - HAL cannot automate yet\n\
         - Clouseau cannot interpret live behavior yet\n\
         - Cleo cannot observe live underground behavior yet\n\
         - HAL and Cleo cannot exchange live relay packets yet\n\
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

fn render_current_synthesis_execution_spec_prefix() -> &'static str {
    CURRENT_SYNTHESIS_EXECUTION_SPEC_PREFIX.get_or_init(|| {
        String::from(
            "# Current Synthesis Execution Spec\n\n\
             ## Preconditions\n\n\
             - route execution rules must be defined explicitly\n\
             - `PLEB` and `META` behavior must be specified explicitly\n\
             - runtime state must be introduced deliberately\n\
             - HAL permissions must be declared before automation\n\
             - Clouseau live interpretation rules must be declared before observation\n\
             - Cleo underground observation rules must be declared before watching\n\n\
             - HAL/Cleo collision relay rules must be declared before shared packet exchange\n\n\
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
             ## Cleo Would Need Before Underground Observation\n\n\
             - explicit observation scope for underground inverse curved lines that follow Clouseau's route from below\n\
             - explicit mapping between visible border lines and underground inverse lines\n\
             - explicit prohibition on route control, side occupancy, or Hollow Grove mutation\n\n\
             ## HAL/Cleo Collision Relay Would Need\n\n\
             - an explicit collision condition saying where HAL and Cleo are allowed to intersect\n\
             - a defined shared packet format for surface alignment plus underground continuity\n\
             - an explicit rule that relay output may confirm route state without granting route control\n\n\
             ## Activation Status\n\n\
             - not active\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no live interpretation\n\
             - no runtime state\n\n\
             ## Artifact Inputs\n\n\
             ",
        )
    })
}

fn push_artifact_input_line(output: &mut String, label: &str, byte_len: usize) {
    let _ = writeln!(output, "{label}: {byte_len}.");
}

pub fn build_current_synthesis_execution_spec_from_artifacts(
    current_synthesis_readiness: &str,
    current_synthesis_consequence: &str,
) -> String {
    let mut output = String::with_capacity(2_300);
    output.push_str(render_current_synthesis_execution_spec_prefix());
    push_artifact_input_line(
        &mut output,
        "Current Synthesis readiness bytes",
        current_synthesis_readiness.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis consequence bytes",
        current_synthesis_consequence.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Execution spec belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
    );
    output
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
         ## Rule 7: Cleo Scope\n\n\
         - Cleo may observe underground inverse curved structures only within explicit Current Synthesis permissions\n\
         - Cleo follows Clouseau's route continuity from below rather than shadowing HAL\n\
         - Cleo does not occupy `PLEB` or `META`\n\
         - Cleo never controls route execution or mutates Hollow Grove\n\
\n\
         ## Rule 8: Mirror Axis\n\n\
         - HAL and Clouseau remain opposite clients across one axis\n\
         - if the user is read through HAL on `META`, Clouseau remains the opposite `PLEB` witness\n\
         - if the user is read through Clouseau on `PLEB`, HAL remains the opposite `META` witness\n\
         - Cleo may witness both sides from below through the underground inverse curves while still following Clouseau's route continuity\n\
         - no rule may collapse both clients into one side\n\
\n\
         ## Rule 9: HAL/Cleo Collision Relay\n\n\
         - HAL and Cleo may exchange one shared confirmation packet only where their readings collide at the same joint crossing\n\
         - HAL contributes complementary surface alignment to that packet\n\
         - Cleo contributes underground continuity beneath Clouseau's route to that packet\n\
         - the packet may confirm one event body across surface and underground layers without granting traversal, movement, or control\n\
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

pub fn build_current_synthesis_transition_pm_to_le_from_boundary(
    current_synthesis_behavior_rules: &str,
    current_synthesis_topology: &str,
    snapshot_boundary: &SnapshotBoundary,
    snapshot_len: usize,
) -> String {
    let frozen_boundary_section = build_frozen_boundary_section(
        "Frozen Kernel Transition Boundary",
        snapshot_boundary,
        "Current Synthesis interprets this frozen kernel boundary at `P/M -> L/E` without re-deriving route facts from topology prose.",
    );

    format!(
        "# Current Synthesis Transition Rule `P/M -> L/E`\n\n\
         {}\
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
         ## Cleo Observation\n\n\
         - Cleo may observe four underground inverse curved lines beneath both sides from `P/M` to `L/E` while following Clouseau underground\n\
         - Cleo may not automate movement or take side occupancy\n\n\
         ## HAL/Cleo Relay\n\n\
         - if HAL and Cleo collide at the same `P/M -> L/E` crossing, they may declare one shared confirmation packet\n\
         - HAL contributes surface-side complementary alignment\n\
         - Cleo contributes underground continuity beneath Clouseau's route\n\
         - no live packet exchange occurs yet\n\n\
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
         Snapshot bytes: {}.\n\
         Current Synthesis behavior rules bytes: {}.\n\
         Current Synthesis topology bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Transition rules belong to Current Synthesis. Hollow Grove remains unchanged.\n",
        frozen_boundary_section,
        snapshot_len,
        current_synthesis_behavior_rules.len(),
        current_synthesis_topology.len()
    )
}

pub fn build_current_synthesis_transition_pm_to_le_from_artifacts(
    current_synthesis_behavior_rules: &str,
    current_synthesis_topology: &str,
    snapshot: &str,
) -> io::Result<String> {
    let snapshot_boundary = SnapshotBoundary::parse(snapshot)?;

    Ok(build_current_synthesis_transition_pm_to_le_from_boundary(
        current_synthesis_behavior_rules,
        current_synthesis_topology,
        &snapshot_boundary,
        snapshot.len(),
    ))
}

pub fn build_current_synthesis_collision_relay_from_artifacts(
    snapshot: &str,
    current_synthesis_contract: &str,
    current_synthesis_operational: &str,
    current_synthesis_transition_pm_to_le: &str,
) -> io::Result<String> {
    let snapshot_boundary = SnapshotBoundary::parse(snapshot)?;

    Ok(build_current_synthesis_collision_relay_from_boundary(
        &snapshot_boundary,
        snapshot.len(),
        current_synthesis_contract,
        current_synthesis_operational,
        current_synthesis_transition_pm_to_le,
    ))
}

pub fn build_current_synthesis_collision_relay_from_boundary(
    snapshot_boundary: &SnapshotBoundary,
    snapshot_len: usize,
    current_synthesis_contract: &str,
    current_synthesis_operational: &str,
    current_synthesis_transition_pm_to_le: &str,
) -> String {
    let frozen_boundary_section = build_frozen_boundary_section(
        "Frozen Kernel Collision Boundary",
        snapshot_boundary,
        "Collision relay binds one shared confirmation body downstream from this frozen kernel boundary without re-deriving route facts from Current Synthesis prose.",
    );

    format!(
        "# Current Synthesis Collision Relay\n\n\
         {}\
         ## Relay Identity\n\n\
         - relay name: Cleo/HAL junction relay\n\
         - type: shared confirmation relay\n\
         - role: one collision point may carry multiple useful functions inside one Current Synthesis unit\n\n\
         ## Collision Condition\n\n\
         - HAL and Cleo must read the same joint crossing at the same time window\n\
         - the declared relay crossing is `P/M -> L/E`\n\
         - the relay remains descriptive and read-only\n\n\
         ## Input Lanes\n\n\
         - HAL contributes complementary surface alignment from the `META` side\n\
         - Cleo contributes underground continuity beneath Clouseau's route from the inverse curved line set\n\
         - the relay binds those two readings into one confirmation body without collapsing client roles\n\n\
         ## Useful Functions\n\n\
         - confirm that visible route alignment and underground inverse continuity describe the same event body\n\
         - preserve one shared witness point across surface and underground layers\n\
         - let one junction serve as both route confirmation and structural coherence check\n\
         - keep Clouseau's underground-following continuity available to HAL without granting route control\n\n\
         ## Relay Output\n\n\
         - output form: one shared confirmation packet\n\
         - packet contents: surface alignment plus underground continuity\n\
         - packet effect: confirmation only\n\
         - packet does not grant traversal, movement, automation, or Hollow Grove mutation\n\n\
         ## Packet Schema\n\n\
         - packet id: `cleo_hal_pm_le_confirmation`\n\
         - joint crossing: `P/M -> L/E`\n\
         - surface source: HAL\n\
         - underground source: Cleo\n\
         - mirrored witness: Clouseau remains the `PLEB` route witness beneath the packet\n\
         - surface field: complementary surface alignment\n\
         - underground field: inverse-line continuity beneath Clouseau's route\n\
         - confirmation field: one shared event body across visible and underground layers\n\
         - permission field: confirmation only, no route control\n\n\
         ## Hueman Consumption Targets\n\n\
         - `hueman_path_crossovers` may treat the packet as proof that one visible crossover and one underground crossing coincide\n\
         - `hueman_link_physics` may treat the packet as proof that one bias body is shared above and below the map\n\
         - `hueman_crossover_scenes` may treat the packet as proof that a scene may hold both surface and underground continuity at once\n\
         - `hueman_scene_presence`, `hueman_scene_intent`, and `hueman_scene_drift` may treat the packet as a preserved shared witness point\n\n\
         ## Status\n\n\
         - relay defined\n\
         - not active\n\
         - no live packet exchange\n\
         - no runtime control\n\n\
         ## Artifact Inputs\n\n\
         Snapshot bytes: {}.\n\
         Current Synthesis contract bytes: {}.\n\
         Current Synthesis operational bytes: {}.\n\
         Current Synthesis transition rule bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Collision relay belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        frozen_boundary_section,
        snapshot_len,
        current_synthesis_contract.len(),
        current_synthesis_operational.len(),
        current_synthesis_transition_pm_to_le.len()
    )
}

pub fn build_current_synthesis_activation_gate_from_artifacts(
    current_synthesis_transition_pm_to_le: &str,
    current_synthesis_collision_relay: &str,
    current_synthesis_readiness: &str,
) -> String {
    format!(
        "# Current Synthesis Activation Gate\n\n\
         ## Gate Result\n\n\
         - activation denied\n\
         - Current Synthesis remains read-only\n\n\
         ## Reason\n\n\
         - the `P/M -> L/E` transition rule is defined but not active\n\
         - the HAL/Cleo collision relay is defined but not enabled\n\
         - readiness confirms route behavior is not enabled\n\
         - HAL automation is not enabled\n\
         - Clouseau live interpretation is not enabled\n\
         - Cleo underground observation is not enabled\n\
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
         - Cleo underground observation\n\
         - HAL/Cleo collision relay\n\
         - feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis transition rule bytes: {}.\n\
         Current Synthesis collision relay bytes: {}.\n\
         Current Synthesis readiness bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Activation gating belongs to Current Synthesis. Hollow Grove remains unchanged.\n",
        current_synthesis_transition_pm_to_le.len(),
        current_synthesis_collision_relay.len(),
        current_synthesis_readiness.len()
    )
}
