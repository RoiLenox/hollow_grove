use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH, DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH,
    SNAPSHOT_ARTIFACT_PATH, build_current_synthesis_base_from_artifacts, read_artifact,
    write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let snapshot = read_artifact(Path::new(SNAPSHOT_ARTIFACT_PATH))?;
    let prompt = read_artifact(Path::new(PROMPT_ARTIFACT_PATH))?;
    let desktop_status = read_artifact(Path::new(DESKTOP_STATUS_ARTIFACT_PATH))?;
    let current_synthesis_base =
        build_current_synthesis_base_from_artifacts(&snapshot, &prompt, &desktop_status)?;
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_base)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_base_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_base_reads_existing_hollow_grove_artifacts() {
        let snapshot = "{\n  \"start\": \"Point\",\n  \"triway\": {\n    \"ways\": [\"One\", \"Two\", \"Three\"]\n  },\n  \"hollow_grove\": {\n    \"bond\": \"One\",\n    \"atmosphere\": [\"Two\", \"Three\"]\n  },\n  \"current_seam\": \"CurrentSeam\",\n  \"aura_beam\": \"AuraBeam\",\n  \"landed\": \"Point\",\n  \"canonical_witness\": \"start Point\\n↓\\nTriway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam\\n↓\\nAuraBeam\\n↓\\nlanded Point\"\n}";
        let prompt = "# Consumer Prompt\n\n## Canonical Witness\n\n```text\nstart Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point\n```\n\n## Structured Snapshot Reference\n\n`artifacts/kernel_pass_snapshot.json`\n\n## Inverse-Path Question\n\nWhat does this completed pass reveal about the inverse path of the end use?\n\n## Boundary Reminder\n\nDo not mutate the kernel. Interpret only.\n";
        let desktop_status = "Hollow Grove status: one completed witnessed recursion\n\nCanonical witness:\nstart Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point\n\nNote: read-only desktop artifact\nNote: niri/river configs untouched\n";

        assert_eq!(
            build_current_synthesis_base_from_artifacts(snapshot, prompt, desktop_status)
                .expect("current synthesis base should build"),
            "# Current Synthesis Base\n\n\
             ## Hollow Grove Status\n\n\
             Hollow Grove remains the stable recursive core.\n\n\
             ## KernelPass Status\n\n\
             `KernelPass` remains the canonical deterministic witness of one completed recursion.\n\n\
             ## Canonical Witness\n\n\
             ```text\n\
             start Point\n\
             ↓\n\
             Triway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             CurrentSeam\n\
             ↓\n\
             AuraBeam\n\
             ↓\n\
             landed Point\n\
             ```\n\n\
             ## Artifact Layer Status\n\n\
             - `artifacts/kernel_pass_snapshot.json`: present and read-only.\n\
             - `artifacts/consumer_prompt.md`: present and read-only.\n\
             - `artifacts/desktop_status.txt`: present and read-only.\n\n\
             Snapshot bytes: 358.\n\
             Prompt bytes: 379.\n\
             Desktop status bytes: 229.\n\n\
             ## Current Synthesis\n\n\
             Current Synthesis is the OS layer built on Hollow Grove.\n\n\
             ## Deferral\n\n\
             - `PLEB` and `META` are deferred.\n\
             - HAL is deferred.\n\
             - Clouseau is deferred.\n\
             - `niri`/`river` are untouched.\n"
        );
    }

    #[test]
    fn current_synthesis_base_writes_a_deterministic_file() {
        let snapshot = "{}";
        let prompt = "prompt";
        let desktop_status = "Hollow Grove status: one completed witnessed recursion\n\nCanonical witness:\nstart Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point\n\nNote: read-only desktop artifact\nNote: niri/river configs untouched\n";
        let current_synthesis_base =
            build_current_synthesis_base_from_artifacts(snapshot, prompt, desktop_status)
                .expect("current synthesis base should build");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-base-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_base.md");

        write_artifact(&artifact_path, &current_synthesis_base)
            .expect("current synthesis base artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis base artifact should be readable"),
            current_synthesis_base
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis base artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis base directory should be removable");
    }
}
