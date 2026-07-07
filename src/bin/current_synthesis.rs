use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    ARTIFACT_INDEX_PATH, CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH, CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH, CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
    build_current_synthesis_activation_gate_from_artifacts,
    build_current_synthesis_base_from_artifacts,
    build_current_synthesis_behavior_rules_from_artifacts,
    build_current_synthesis_choice_from_artifacts, build_current_synthesis_clients_from_artifacts,
    build_current_synthesis_consequence_from_artifacts,
    build_current_synthesis_contract_from_artifacts,
    build_current_synthesis_execution_spec_from_artifacts,
    build_current_synthesis_operational_from_artifacts,
    build_current_synthesis_preview_from_artifacts,
    build_current_synthesis_readiness_from_artifacts,
    build_current_synthesis_selection_from_artifacts,
    build_current_synthesis_sequence_from_artifacts, build_current_synthesis_state_from_artifacts,
    build_current_synthesis_transition_pm_to_le_from_artifacts,
    build_current_synthesis_topology_from_artifacts, read_artifact, write_artifact,
};

fn run_current_synthesis_at(root: &Path) -> io::Result<[PathBuf; 16]> {
    let snapshot = read_artifact(&root.join(SNAPSHOT_ARTIFACT_PATH))?;
    let prompt = read_artifact(&root.join(PROMPT_ARTIFACT_PATH))?;
    let desktop_status = read_artifact(&root.join(DESKTOP_STATUS_ARTIFACT_PATH))?;
    let current_synthesis_base =
        build_current_synthesis_base_from_artifacts(&snapshot, &prompt, &desktop_status)?;
    let base_path = root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH);
    write_artifact(&base_path, &current_synthesis_base)?;

    let artifact_index = read_artifact(&root.join(ARTIFACT_INDEX_PATH))?;
    let current_synthesis_state =
        build_current_synthesis_state_from_artifacts(&current_synthesis_base, &artifact_index);
    let state_path = root.join(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH);
    write_artifact(&state_path, &current_synthesis_state)?;

    let current_synthesis_sequence = build_current_synthesis_sequence_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_state,
    );
    let sequence_path = root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH);
    write_artifact(&sequence_path, &current_synthesis_sequence)?;

    let current_synthesis_topology = build_current_synthesis_topology_from_artifacts(
        &current_synthesis_sequence,
        &current_synthesis_state,
    );
    let topology_path = root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH);
    write_artifact(&topology_path, &current_synthesis_topology)?;

    let current_synthesis_clients = build_current_synthesis_clients_from_artifacts(
        &current_synthesis_topology,
        &current_synthesis_sequence,
    );
    let clients_path = root.join(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH);
    write_artifact(&clients_path, &current_synthesis_clients)?;

    let current_synthesis_choice = build_current_synthesis_choice_from_artifacts(
        &current_synthesis_clients,
        &current_synthesis_topology,
    );
    let choice_path = root.join(CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH);
    write_artifact(&choice_path, &current_synthesis_choice)?;

    let current_synthesis_contract = build_current_synthesis_contract_from_artifacts(
        &current_synthesis_choice,
        &current_synthesis_clients,
    );
    let contract_path = root.join(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH);
    write_artifact(&contract_path, &current_synthesis_contract)?;

    let current_synthesis_preview = build_current_synthesis_preview_from_artifacts(
        &current_synthesis_contract,
        &current_synthesis_sequence,
    );
    let preview_path = root.join(CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH);
    write_artifact(&preview_path, &current_synthesis_preview)?;

    let current_synthesis_operational = build_current_synthesis_operational_from_artifacts(
        &current_synthesis_preview,
        &current_synthesis_contract,
    );
    let operational_path = root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH);
    write_artifact(&operational_path, &current_synthesis_operational)?;

    let current_synthesis_selection = build_current_synthesis_selection_from_artifacts(
        &current_synthesis_choice,
        &current_synthesis_operational,
    );
    let selection_path = root.join(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH);
    write_artifact(&selection_path, &current_synthesis_selection)?;

    let current_synthesis_consequence = build_current_synthesis_consequence_from_artifacts(
        &current_synthesis_selection,
        &current_synthesis_operational,
    );
    let consequence_path = root.join(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH);
    write_artifact(&consequence_path, &current_synthesis_consequence)?;

    let current_synthesis_readiness = build_current_synthesis_readiness_from_artifacts(
        &current_synthesis_consequence,
        &current_synthesis_selection,
    );
    let readiness_path = root.join(CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH);
    write_artifact(&readiness_path, &current_synthesis_readiness)?;

    let current_synthesis_execution_spec = build_current_synthesis_execution_spec_from_artifacts(
        &current_synthesis_readiness,
        &current_synthesis_consequence,
    );
    let execution_spec_path = root.join(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH);
    write_artifact(&execution_spec_path, &current_synthesis_execution_spec)?;

    let current_synthesis_behavior_rules = build_current_synthesis_behavior_rules_from_artifacts(
        &current_synthesis_execution_spec,
        &current_synthesis_selection,
    );
    let behavior_rules_path = root.join(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH);
    write_artifact(&behavior_rules_path, &current_synthesis_behavior_rules)?;

    let current_synthesis_transition_pm_to_le =
        build_current_synthesis_transition_pm_to_le_from_artifacts(
            &current_synthesis_behavior_rules,
            &current_synthesis_topology,
        );
    let transition_pm_to_le_path = root.join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH);
    write_artifact(
        &transition_pm_to_le_path,
        &current_synthesis_transition_pm_to_le,
    )?;

    let current_synthesis_activation_gate = build_current_synthesis_activation_gate_from_artifacts(
        &current_synthesis_transition_pm_to_le,
        &current_synthesis_readiness,
    );
    let activation_gate_path = root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH);
    write_artifact(&activation_gate_path, &current_synthesis_activation_gate)?;

    Ok([
        base_path,
        state_path,
        sequence_path,
        topology_path,
        clients_path,
        choice_path,
        contract_path,
        preview_path,
        operational_path,
        selection_path,
        consequence_path,
        readiness_path,
        execution_spec_path,
        behavior_rules_path,
        transition_pm_to_le_path,
        activation_gate_path,
    ])
}

fn main() -> io::Result<()> {
    for artifact_path in run_current_synthesis_at(Path::new("."))? {
        println!("{}", artifact_path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        ARTIFACT_INDEX_PATH, DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH,
        SNAPSHOT_ARTIFACT_PATH, build_current_synthesis_activation_gate_from_artifacts,
        build_current_synthesis_base_from_artifacts,
        build_current_synthesis_behavior_rules_from_artifacts,
        build_current_synthesis_choice_from_artifacts,
        build_current_synthesis_clients_from_artifacts,
        build_current_synthesis_consequence_from_artifacts,
        build_current_synthesis_contract_from_artifacts,
        build_current_synthesis_execution_spec_from_artifacts,
        build_current_synthesis_operational_from_artifacts,
        build_current_synthesis_preview_from_artifacts,
        build_current_synthesis_readiness_from_artifacts,
        build_current_synthesis_selection_from_artifacts,
        build_current_synthesis_sequence_from_artifacts,
        build_current_synthesis_state_from_artifacts, read_artifact, write_artifact,
    };
    use super::run_current_synthesis_at;

    fn write_fixture(root: &Path, relative_path: &str, contents: &str) {
        let path = root.join(relative_path);
        write_artifact(&path, contents).expect("fixture should write");
    }

    #[test]
    fn current_synthesis_runner_regenerates_base_state_sequence_topology_clients_choice_contract_preview_operational_selection_consequence_readiness_execution_spec_behavior_rules_transition_and_activation_gate_in_order()
     {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_root = std::env::temp_dir().join(format!("current-synthesis-runner-{nonce}"));
        let snapshot = "{\n  \"start\": \"Point\",\n  \"triway\": {\n    \"ways\": [\"One\", \"Two\", \"Three\"]\n  },\n  \"hollow_grove\": {\n    \"bond\": \"One\",\n    \"atmosphere\": [\"Two\", \"Three\"]\n  },\n  \"current_seam\": \"CurrentSeam\",\n  \"aura_beam\": \"AuraBeam\",\n  \"landed\": \"Point\",\n  \"canonical_witness\": \"start Point\\n↓\\nTriway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam\\n↓\\nAuraBeam\\n↓\\nlanded Point\"\n}";
        let prompt = "# Consumer Prompt\n\n## Canonical Witness\n\n```text\nstart Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point\n```\n\n## Structured Snapshot Reference\n\n`artifacts/kernel_pass_snapshot.json`\n\n## Inverse-Path Question\n\nWhat does this completed pass reveal about the inverse path of the end use?\n\n## Boundary Reminder\n\nDo not mutate the kernel. Interpret only.\n";
        let desktop_status = "Hollow Grove status: one completed witnessed recursion\n\nCanonical witness:\nstart Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point\n\nNote: read-only desktop artifact\nNote: niri/river configs untouched\n";
        let artifact_index = "# Artifact Index\n\nindex";

        write_fixture(&artifact_root, SNAPSHOT_ARTIFACT_PATH, snapshot);
        write_fixture(&artifact_root, PROMPT_ARTIFACT_PATH, prompt);
        write_fixture(&artifact_root, DESKTOP_STATUS_ARTIFACT_PATH, desktop_status);
        write_fixture(&artifact_root, ARTIFACT_INDEX_PATH, artifact_index);

        let [
            base_path,
            state_path,
            sequence_path,
            topology_path,
            clients_path,
            choice_path,
            contract_path,
            preview_path,
            operational_path,
            selection_path,
            consequence_path,
            readiness_path,
            execution_spec_path,
            behavior_rules_path,
            transition_pm_to_le_path,
            activation_gate_path,
        ] = run_current_synthesis_at(&artifact_root).expect("current synthesis should run");
        let current_synthesis_base =
            build_current_synthesis_base_from_artifacts(snapshot, prompt, desktop_status)
                .expect("base should build");
        let current_synthesis_state =
            build_current_synthesis_state_from_artifacts(&current_synthesis_base, artifact_index);
        let current_synthesis_sequence = build_current_synthesis_sequence_from_artifacts(
            &current_synthesis_base,
            &current_synthesis_state,
        );
        let current_synthesis_topology =
            super::current_synthesis_support::build_current_synthesis_topology_from_artifacts(
                &current_synthesis_sequence,
                &current_synthesis_state,
            );
        let current_synthesis_clients = build_current_synthesis_clients_from_artifacts(
            &current_synthesis_topology,
            &current_synthesis_sequence,
        );
        let current_synthesis_choice = build_current_synthesis_choice_from_artifacts(
            &current_synthesis_clients,
            &current_synthesis_topology,
        );
        let current_synthesis_contract = build_current_synthesis_contract_from_artifacts(
            &current_synthesis_choice,
            &current_synthesis_clients,
        );
        let current_synthesis_preview = build_current_synthesis_preview_from_artifacts(
            &current_synthesis_contract,
            &current_synthesis_sequence,
        );
        let current_synthesis_operational = build_current_synthesis_operational_from_artifacts(
            &current_synthesis_preview,
            &current_synthesis_contract,
        );
        let current_synthesis_selection = build_current_synthesis_selection_from_artifacts(
            &current_synthesis_choice,
            &current_synthesis_operational,
        );
        let current_synthesis_consequence = build_current_synthesis_consequence_from_artifacts(
            &current_synthesis_selection,
            &current_synthesis_operational,
        );
        let current_synthesis_readiness = build_current_synthesis_readiness_from_artifacts(
            &current_synthesis_consequence,
            &current_synthesis_selection,
        );
        let current_synthesis_execution_spec =
            build_current_synthesis_execution_spec_from_artifacts(
                &current_synthesis_readiness,
                &current_synthesis_consequence,
            );
        let current_synthesis_behavior_rules =
            build_current_synthesis_behavior_rules_from_artifacts(
                &current_synthesis_execution_spec,
                &current_synthesis_selection,
            );
        let current_synthesis_transition_pm_to_le =
            super::current_synthesis_support::build_current_synthesis_transition_pm_to_le_from_artifacts(
                &current_synthesis_behavior_rules,
                &current_synthesis_topology,
            );
        let current_synthesis_activation_gate =
            build_current_synthesis_activation_gate_from_artifacts(
                &current_synthesis_transition_pm_to_le,
                &current_synthesis_readiness,
            );

        assert_eq!(
            read_artifact(&base_path).expect("base artifact should read"),
            current_synthesis_base
        );
        assert_eq!(
            read_artifact(&state_path).expect("state artifact should read"),
            current_synthesis_state
        );
        assert_eq!(
            read_artifact(&sequence_path).expect("sequence artifact should read"),
            current_synthesis_sequence
        );
        assert_eq!(
            read_artifact(&topology_path).expect("topology artifact should read"),
            current_synthesis_topology
        );
        assert_eq!(
            read_artifact(&clients_path).expect("clients artifact should read"),
            current_synthesis_clients
        );
        assert_eq!(
            read_artifact(&choice_path).expect("choice artifact should read"),
            current_synthesis_choice
        );
        assert_eq!(
            read_artifact(&contract_path).expect("contract artifact should read"),
            current_synthesis_contract
        );
        assert_eq!(
            read_artifact(&preview_path).expect("preview artifact should read"),
            current_synthesis_preview
        );
        assert_eq!(
            read_artifact(&operational_path).expect("operational artifact should read"),
            current_synthesis_operational
        );
        assert_eq!(
            read_artifact(&selection_path).expect("selection artifact should read"),
            current_synthesis_selection
        );
        assert_eq!(
            read_artifact(&consequence_path).expect("consequence artifact should read"),
            current_synthesis_consequence
        );
        assert_eq!(
            read_artifact(&readiness_path).expect("readiness artifact should read"),
            current_synthesis_readiness
        );
        assert_eq!(
            read_artifact(&execution_spec_path).expect("execution spec artifact should read"),
            current_synthesis_execution_spec
        );
        assert_eq!(
            read_artifact(&behavior_rules_path).expect("behavior rules artifact should read"),
            current_synthesis_behavior_rules
        );
        assert_eq!(
            read_artifact(&transition_pm_to_le_path)
                .expect("transition rule artifact should read"),
            current_synthesis_transition_pm_to_le
        );
        assert_eq!(
            read_artifact(&activation_gate_path).expect("activation gate artifact should read"),
            current_synthesis_activation_gate
        );

        fs::remove_file(&artifact_root.join(SNAPSHOT_ARTIFACT_PATH))
            .expect("snapshot fixture should be removable");
        fs::remove_file(&artifact_root.join(PROMPT_ARTIFACT_PATH))
            .expect("prompt fixture should be removable");
        fs::remove_file(&artifact_root.join(DESKTOP_STATUS_ARTIFACT_PATH))
            .expect("desktop fixture should be removable");
        fs::remove_file(&artifact_root.join(ARTIFACT_INDEX_PATH))
            .expect("index fixture should be removable");
        fs::remove_file(&base_path).expect("base artifact should be removable");
        fs::remove_file(&state_path).expect("state artifact should be removable");
        fs::remove_file(&sequence_path).expect("sequence artifact should be removable");
        fs::remove_file(&topology_path).expect("topology artifact should be removable");
        fs::remove_file(&clients_path).expect("clients artifact should be removable");
        fs::remove_file(&choice_path).expect("choice artifact should be removable");
        fs::remove_file(&contract_path).expect("contract artifact should be removable");
        fs::remove_file(&preview_path).expect("preview artifact should be removable");
        fs::remove_file(&operational_path).expect("operational artifact should be removable");
        fs::remove_file(&selection_path).expect("selection artifact should be removable");
        fs::remove_file(&consequence_path).expect("consequence artifact should be removable");
        fs::remove_file(&readiness_path).expect("readiness artifact should be removable");
        fs::remove_file(&execution_spec_path).expect("execution spec artifact should be removable");
        fs::remove_file(&behavior_rules_path).expect("behavior rules artifact should be removable");
        fs::remove_file(&transition_pm_to_le_path)
            .expect("transition rule artifact should be removable");
        fs::remove_file(&activation_gate_path)
            .expect("activation gate artifact should be removable");
        fs::remove_dir_all(artifact_root.join("artifacts"))
            .expect("artifact fixture directory should be removable");
        fs::remove_dir(&artifact_root).expect("artifact root should be removable");
    }
}
