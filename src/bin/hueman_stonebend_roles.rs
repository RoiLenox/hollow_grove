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
        assert!(
            output.contains("Gremlin -> Goblin -> Ghoul -> Troll -> Ork -> Ogre -> Troglodyte")
        );
        assert!(
            output.contains("Hypergiant is the Stonebend office that possesses the Troglodyte")
        );
        assert!(output.contains("whoever holds the Hypergiant Crown is Hypergiant"));
        assert!(output.contains("alone or together"));
        assert!(output.contains("either office may take the Crown and become Hypergiant"));
        assert!(output.contains("the Stonebender is the public proving ground at Stonebend"));
        assert!(output.contains("Stonehenge-like arena at Stonebend"));
        assert!(output.contains("Proletariat enters as labor force"));
        assert!(output.contains("Freemason tests craft, structure"));
        assert!(output.contains("separates hollow current from regular current"));
        assert!(output.contains("Mercurite is the accepted hard branch"));
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
