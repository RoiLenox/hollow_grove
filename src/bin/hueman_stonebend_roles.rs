use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_stonebend_roles_from_artifacts, hueman_stonebend_roles_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        build_hueman_stonebend_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_stonebend_roles_artifact_path();

    write_text_artifact(&artifact_path, &hueman_stonebend_roles)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_stonebend_roles_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_stonebend_roles_reads_existing_artifacts() {
        let output = build_hueman_stonebend_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Stonebend Roles"));
        assert!(output.contains(
            "Gremlin -> Goblin -> Ghoul -> Spectre -> Troll -> Ork -> Ogre -> Troglodyte"
        ));
        assert!(output.contains("the Geralds are Stonebend's constitutional people"));
        assert!(output.contains("the Hypergiant is the singular highest Stonebend office"));
        assert!(
            output.contains("the Proliteriate is Stonebend's permanent distributed public network")
        );
        assert!(output.contains("the High Freemason is the singular office"));
        assert!(output.contains("transformation may never create office or Title automatically"));
        assert!(
            output.contains("Illegal Hollowing is Stonebend's signature constitutional offense")
        );
        assert!(output.contains("Mt. Aura is Aether"));
        assert!(output.contains("Riptide is Bathos"));
        assert!(output.contains("Aura Way is the standard known path"));
        assert!(output.contains("Opal varies, Diamond concentrates, and Quartz resonates"));
        assert!(output.contains("Stonebend has exactly three bidirectional constitutional gates"));
        assert!(output.contains("Diamond is Stonebend's continuing sovereign Title"));
        assert!(output.contains("no Hypergiant claims Diamond without The Lazerhorn"));
        assert!(output.contains("a Claim is not automatically a Title"));
        assert!(output.contains("High Freemason replacement requires independent Forge review"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_stonebend_roles_writes_a_deterministic_file() {
        let hueman_stonebend_roles =
            build_hueman_stonebend_roles_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-stonebend-roles-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_stonebend_roles.md");

        write_text_artifact(&artifact_path, &hueman_stonebend_roles)
            .expect("hueman stonebend roles artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman stonebend roles artifact should read"),
            hueman_stonebend_roles
        );

        fs::remove_file(&artifact_path)
            .expect("hueman stonebend roles artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman stonebend roles directory should be removable");
    }
}
