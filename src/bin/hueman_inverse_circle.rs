use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_LINK_PHYSICS_ARTIFACT_PATH,
    build_hueman_inverse_circle_from_artifacts, hueman_inverse_circle_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let hueman_inverse_circle =
        build_hueman_inverse_circle_from_artifacts(&hueman_fourway, &hueman_link_physics);
    let artifact_path = hueman_inverse_circle_artifact_path();

    write_text_artifact(&artifact_path, &hueman_inverse_circle)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_inverse_circle_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_inverse_circle_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_inverse_circle_from_artifacts("fourway", "physics"),
            "# Hueman Inverse Circle\n\n\
             ## Structural Rule\n\n\
             The inverse circle is an interior underground ring of secret tunnels inside Hueman's world layer, mirroring the visible route system without replacing the canonical Fourway map.\n\n\
             ## Interior Tunnel Ring\n\n\
             - the inverse circle stays underground\n\
             - the inverse circle stays on the interior\n\
             - the inverse circle is secret rather than public-facing\n\
             - the inverse circle mirrors upper travel without becoming the upper travel itself\n\n\
             ## Tunnel Sequence\n\n\
             - The Stairway to Heaven\n\
             - The Riptide\n\
             - The Current Sea\n\
             - The Aura Way\n\n\
             ## Mirror Reading\n\n\
             - The Stairway to Heaven reads as the hidden ascent tunnel inside the circle.\n\
             - The Riptide reads as the pull that drags movement back through interior force.\n\
             - The Current Sea reads as the underground flow of counted, sustained motion.\n\
             - The Aura Way reads as the interior atmospheric passage where pressure becomes felt before seen.\n\n\
             ## Boundary\n\n\
             - the inverse circle belongs to Hueman as subterranean world structure\n\
             - it does not replace Fourway, AuraTriad, or kernel routing\n\
             - it remains readable from link physics without feeding back into lower layers\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Fourway bytes: 7.\n\
             Hueman Link Physics bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             The inverse circle is an underground interior mirror path. It does not overwrite the visible world map, Current Synthesis geography, or Hollow Grove recursion.\n"
        );
    }

    #[test]
    fn hueman_inverse_circle_writes_a_deterministic_file() {
        let hueman_inverse_circle =
            build_hueman_inverse_circle_from_artifacts("fourway", "physics");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-inverse-circle-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_inverse_circle.md");

        write_text_artifact(&artifact_path, &hueman_inverse_circle)
            .expect("hueman inverse circle artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman inverse circle artifact should read"),
            hueman_inverse_circle
        );

        fs::remove_file(&artifact_path)
            .expect("hueman inverse circle artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman inverse circle directory should be removable");
    }
}
