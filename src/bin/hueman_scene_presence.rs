use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH, HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH,
    HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH, HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH,
    HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH, HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH,
    HUEMAN_TROSS_HELPERS_ARTIFACT_PATH, build_hueman_scene_presence_from_artifacts,
    hueman_scene_presence_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_crossover_scenes =
        read_text_artifact(Path::new(HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH))?;
    let hueman_archetype_lens = read_text_artifact(Path::new(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        read_text_artifact(Path::new(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH))?;
    let hueman_tross_helpers = read_text_artifact(Path::new(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH))?;
    let hueman_glaushouse_roles =
        read_text_artifact(Path::new(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        read_text_artifact(Path::new(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH))?;
    let hueman_inverse_circle = read_text_artifact(Path::new(HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH))?;
    let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
        &hueman_crossover_scenes,
        &hueman_archetype_lens,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
    );
    let artifact_path = hueman_scene_presence_artifact_path();

    write_text_artifact(&artifact_path, &hueman_scene_presence)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_scene_presence_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_scene_presence_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_scene_presence_from_artifacts(
                "scene",
                "lens",
                "roles",
                "tross",
                "glaushouse",
                "sandmanor",
                "inverse"
            ),
            "# Hueman Scene Presence\n\n\
             ## Structural Rule\n\n\
             Each crossover scene carries a characteristic kind of presence before any encounter mechanics exist.\n\n\
             ## Presence Map\n\n\
             - Seam Market: rumor carriers, salvage brokers, exchangers, temporary stalls, signal cloths\n\
             - Threshold Weather: drifters, lookouts, spray traces, warning markers, bright debris\n\
             - Pressure Shelter: keepers, hoarders, wardens, bundled stores, inward fires\n\
             - Split Trace: echoes, doubles, uncertain witnesses, partial camps, contradictory clues\n\n\
             ## Archetype Pull\n\n\
             - `goblin` -> Pressure Shelter\n\
             - `gremlin` -> Seam Market\n\
             - `pixy` -> Threshold Weather\n\
             - `sprite` -> Split Trace\n\n\
             ## Aura Ridge Presence\n\n\
             - straight-ridge caravans, free traders, hinge stalls, and visible right-angle turn traffic\n\
             - public trade legs are declared from Stonebend -> Glaushouse and Glaushouse -> Sandmanor\n\n\
             ## Glaushouse Presence\n\n\
             - Prima Donna: scene-facing lead and public center of Glaushouse presence\n\
             - Persephone: assistant, relay, and step-down continuity beside the lead\n\
             - Nightengales: nurses and common people carrying the lived body of Glaushouse\n\
             - jades: polished green thresholds, court stone, and care tokens mined in the South\n\
             - Jadomer: Glaushouse's outward export carried as current refined through jade yield\n\n\
             ## Stonebend Civic Presence\n\n\
             - Proliteriate: collective labor pressure and shared leverage\n\
             - Hypergiant: public-facing speaker without superior rank\n\
             - Freemason: built order, sealed works, and hidden structure\n\
             - Hypergiant may appear first, but triad power stays equal\n\
             - Geralds: the common people of Stonebend holding its public mass\n\
             - diamonds: mined civic wealth held under Stonebend's equal-power structure\n\
             - mercury mirror: Stonebend's outward export refined from hollow current and diamond yield\n\n\
             ## Tross Helper Presence\n\n\
             - Tross: Flynt-anchored line presence running North -> South\n\
             - Juvenile: north head pressure held at the Flynt-facing side of the line\n\
             - Delinquent: south guard pressure carried downline toward Glaushouse-facing scenes\n\
             - White Dwarfs: four close guards holding Tross's personal ring without taking the north or south posts\n\
             - Wardens: the people of Flynt holding the line body around Tross\n\
             - opals: mined gleam carried through Flynt's guarded northern line\n\
             - Opal Oil: Flynt's outward export carried as hollow current refined through opal yield\n\
             - Tross helpers do not outrank scene or civic roles\n\n\
             ## Sandmanor Competitive Presence\n\n\
             - Sandmen: the people of Sandmanor holding the shared social body beneath the contest\n\
             - Minoans: southern room-makers, interior singers, draped thresholds, tuned chambers\n\
             - Minorians: northern counters, ledger-keepers, tally boards, visible judges\n\
             - Sandmanite: Minoan winner carrying the Sandman office through design-crossed improvement\n\
             - Sandmanorian: Minorian winner carrying the Sandman office through accounting-crossed improvement\n\
             - crystals: mined facets and stewarded witness stock beneath the rivalry\n\
             - Crystoleum: Sandmanor's outward glass-sand export carried through current and crystal proof\n\
             - Sandmanor presence favors visible comparison instead of inherited fixed rank\n\n\
             ## Inverse Circle Presence\n\n\
             - The Stairway to Heaven: concealed rise shafts, rung marks, hush traffic\n\
             - The Riptide: pull currents, drag marks, return pressure in the tunnel bends\n\
             - The Current Sea: underground flow chambers, counted channels, measured carry\n\
             - The Aura Way: soft-lit passages, atmospheric bleed, felt route pressure before sight\n\n\
             ## Status\n\n\
             - scene presence is descriptive-only for now\n\
             - no NPC system or occupancy resolver is active\n\
             - scene typing, archetype pull, civic overlay, helper lines, Sandmanor rivalry, and the inverse circle remain upstream only\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Crossover Scenes bytes: 5.\n\
             Hueman Archetype Lens bytes: 4.\n\
             Hueman Stonebend Roles bytes: 5.\n\
             Hueman Tross Helpers bytes: 5.\n\
             Hueman Glaushouse Roles bytes: 10.\n\
             Hueman Sandmanor Roles bytes: 9.\n\
             Hueman Inverse Circle bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Scene presence says what kind of occupant or trace belongs in a scene. It does not create procedural actors, dialogue, or rewards.\n"
        );
    }

    #[test]
    fn hueman_scene_presence_writes_a_deterministic_file() {
        let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
            "scene",
            "lens",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
        );
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-presence-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_presence.md");

        write_text_artifact(&artifact_path, &hueman_scene_presence)
            .expect("hueman scene presence artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman scene presence artifact should read"),
            hueman_scene_presence
        );

        fs::remove_file(&artifact_path)
            .expect("hueman scene presence artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman scene presence directory should be removable");
    }
}
