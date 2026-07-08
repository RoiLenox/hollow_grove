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
        assert_eq!(
            build_hueman_stonebend_roles_from_artifacts("start", "fourway"),
            "# Hueman Stonebend Roles\n\n\
             ## Structural Rule\n\n\
             Stonebend carries a three-part civic power that belongs to Hueman's world layer and remains vertically integrated above Current Synthesis and Hollow Grove.\n\n\
             ## Stonebend Power Triad\n\n\
             - Proliteriate\n\
             - Hypergiant\n\
             - Freemason\n\n\
             ## Power Balance\n\n\
             - Proliteriate, Hypergiant, and Freemason hold equal power inside Stonebend.\n\
             - Hypergiant is the public face of the triad, not a higher authority.\n\
             - public representation does not override equal internal standing.\n\
             - no single role may collapse the triad into a solo rule.\n\n\
             ## Vertical Integration\n\n\
             - Stonebend remains the East-facing `gremlin` start on the Fourway.\n\
             - the civic triad is a Hueman/world governance layer attached to that start.\n\
             - Geralds are the people of Stonebend and carry the city's common civic body.\n\
             - Stonebend mines diamonds.\n\
             - Stonebend uses hollow current with diamond yield to produce mercury mirror as its main export.\n\
             - Current Synthesis remains the lower operating layer beneath this governance.\n\
             - Hollow Grove remains the recursive core beneath both.\n\n\
             ## Status\n\n\
             - Stonebend roles are descriptive-only for now\n\
             - no command resolver or role AI is active\n\
             - no automatic power shifts are active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Stonebend roles belong to Hueman's civic layer. They do not replace HAL, Clouseau, or any Current Synthesis client boundary.\n"
        );
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
