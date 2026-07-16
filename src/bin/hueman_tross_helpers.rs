use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_tross_helpers_from_artifacts, hueman_tross_helpers_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_tross_helpers =
        build_hueman_tross_helpers_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_tross_helpers_artifact_path();

    write_text_artifact(&artifact_path, &hueman_tross_helpers)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_tross_helpers_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_tross_helpers_reads_existing_artifacts() {
        let output = build_hueman_tross_helpers_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Tross Helpers"));
        assert!(output.contains("Gremlin is Flynt's confirmed Current-origin path."));
        assert!(output.contains("boardwalk-casino hunting capital"));
        assert!(output.contains(
            "Aura Basin is Flynt's nearest hunt ground, where Gargoyles contest Werewolves"
        ));
        assert!(
            output.contains("The Riptide is the outer roaming water rim where Mermen are hunted")
        );
        assert!(output.contains("it is a mixing pit with convergent ascent"));
        assert!(output.contains("Gargoyle is Flynt's first mixed synthesis recipe"));
        assert!(output.contains("Flynt progression is recipe-gated rather than hereditary"));
        assert!(output.contains(
            "Gargoyle is the mandatory first embodiment for living and hunting as Flynt"
        ));
        assert!(output.contains("Merman is a sea-current form a Flyntian Gargoyle must hunt and harvest along the Riptide"));
        assert!(output.contains("Werewolf is a feral land-hunt form a Flyntian Gargoyle must hunt and harvest in Aura Basin"));
        assert!(output.contains(
            "Chimera only counts after Gargoyle, Werewolf, and Merman have each been separately mastered"
        ));
        assert!(output.contains("Manticore is not Chimera under another name"));
        assert!(output.contains("Chimera is its own completed synthesis recipe and form"));
        assert!(
            output.contains("Manticore is the later apex synthesis recipe mastered after Chimera")
        );
        assert!(output.contains("## Regional Goods"));
        assert!(output.contains("Flyntian Dagger = Flynt Opal"));
        assert!(output.contains("whoever holds Contracore is Tross"));
        assert!(output.contains(
            "only someone who has mastered the Manticore recipe and form may challenge Tross"
        ));
        assert!(output.contains("can oppose Stonebend's Troglodyte"));
        assert!(output.contains("office of Tross may not remain vacant"));
        assert!(output.contains("Delinquent guards West"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_tross_helpers_writes_a_deterministic_file() {
        let hueman_tross_helpers = build_hueman_tross_helpers_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-tross-helpers-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_tross_helpers.md");

        write_text_artifact(&artifact_path, &hueman_tross_helpers)
            .expect("hueman tross helpers artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman tross helpers artifact should read"),
            hueman_tross_helpers
        );

        fs::remove_file(&artifact_path).expect("hueman tross helpers artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman tross helpers directory should be removable");
    }
}
