use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH, build_current_synthesis_contract_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_choice =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH))?;
    let current_synthesis_clients =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH))?;
    let current_synthesis_contract = build_current_synthesis_contract_from_artifacts(
        &current_synthesis_choice,
        &current_synthesis_clients,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_contract)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_contract_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_contract_reads_existing_artifacts() {
        let current_synthesis_choice = "# Current Synthesis Choice\n\nchoice";
        let current_synthesis_clients = "# Current Synthesis Clients\n\nclients";

        assert_eq!(
            build_current_synthesis_contract_from_artifacts(
                current_synthesis_choice,
                current_synthesis_clients
            ),
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
             Current Synthesis choice bytes: 34.\n\
             Current Synthesis clients bytes: 36.\n\n\
             ## Boundary Reminder\n\n\
             Route contract belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_contract_writes_a_deterministic_file() {
        let current_synthesis_contract =
            build_current_synthesis_contract_from_artifacts("choice", "clients");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-contract-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_contract.md");

        write_artifact(&artifact_path, &current_synthesis_contract)
            .expect("current synthesis contract artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis contract artifact should be readable"),
            current_synthesis_contract
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis contract artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis contract directory should be removable");
    }
}
