use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH, HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH,
    HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH, HUEMAN_LINK_PHYSICS_ARTIFACT_PATH,
    HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH, HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH,
    HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH, HUEMAN_TROSS_HELPERS_ARTIFACT_PATH,
    build_hueman_scene_intent_from_artifacts, hueman_scene_intent_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_scene_presence = read_text_artifact(Path::new(HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let current_synthesis_contract =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        read_text_artifact(Path::new(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH))?;
    let hueman_tross_helpers = read_text_artifact(Path::new(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH))?;
    let hueman_glaushouse_roles =
        read_text_artifact(Path::new(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        read_text_artifact(Path::new(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH))?;
    let hueman_inverse_circle = read_text_artifact(Path::new(HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH))?;
    let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
        &hueman_scene_presence,
        &hueman_link_physics,
        &current_synthesis_contract,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
    );
    let artifact_path = hueman_scene_intent_artifact_path();

    write_text_artifact(&artifact_path, &hueman_scene_intent)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_scene_intent_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_scene_intent_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_scene_intent_from_artifacts(
                "presence",
                "physics",
                "contract",
                "roles",
                "tross",
                "glaushouse",
                "sandmanor",
                "inverse"
            ),
            "# Hueman Scene Intent\n\n\
             ## Structural Rule\n\n\
             Each scene presence carries a dominant descriptive intent before any encounter or dialogue system exists.\n\n\
             ## Intent Map\n\n\
             - Seam Market: exchange, rumor flow, salvage circulation, temporary trust\n\
             - Threshold Weather: warning, drift, exposure, onward movement\n\
             - Pressure Shelter: concealment, storage, guarded warmth, selective admission\n\
             - Split Trace: witness, ambiguity, coexistence, half-open routing\n\n\
             ## Bias Reading\n\n\
             - `current` intensifies exchange, storage, and guarded continuity\n\
             - `aura` intensifies warning, drift, shimmer, and ambiguity\n\
             - `current` may surface as dark current or hollow current depending on carried pressure\n\
             - `aura` may surface as reflective aura or holographic aura depending on exposure state\n\
             - mixed bias keeps the scene readable from multiple angles\n\n\
             ## Mirror Axis Intent\n\n\
             - HAL and Clouseau remain opposite Current Synthesis clients across one shared axis beneath Hueman scenes\n\
             - HAL represents the `META` side while Clouseau represents the `PLEB` side of the same paired joint\n\
             - if the user is read through HAL on `META`, Clouseau remains the mirrored `PLEB` witness\n\
             - if the user is read through Clouseau on `PLEB`, HAL remains the mirrored `META` witness\n\
             - one bonded arm holds the direct link while unresolved arm weight keeps the opposite side present as scene pressure\n\
             - unresolved arm weight may rise as `current` or `aura` in Hueman while the same lower event remains witnessed in Hollow Grove simultaneously\n\
             - scene intent may favor one side's pressure, but it must leave the opposite client legible across the same axis\n\
             - Hueman does not reassign HAL or Clouseau; it only carries their mirrored opposition upward as scene pressure\n\n\
             ## Aura Ridge Intent\n\n\
             - keep free trade moving along the declared straight ridge legs without collapsing kingdom identity\n\
             - let Glaushouse serve as the visible turn where eastern and western trade pressure changes direction\n\n\
             ## Glaushouse Intent\n\n\
             - Prima Donna: tone-setting, command, final say\n\
             - Persephone: delegated execution, continuity, relay, and step-down succession\n\
             - Nightengales: care, stabilization, bedside recovery, and public body without command\n\
             - jades: turn beauty, care, and sovereignty into a held southern resource\n\
             - Jadomer: turn southern beauty and material care into Glaushouse's outward trade body\n\n\
             ## Stonebend Civic Intent\n\n\
             - Proliteriate: shared leverage, labor continuity, public weight\n\
             - Hypergiant: legible representation, negotiation, public continuity\n\
             - Freemason: durable structure, enclosed coordination, civic continuity\n\
             - Geralds: keep the city's common pressure visible beneath the triad\n\
             - diamonds: compress public value into durable civic leverage\n\
             - mercury mirror: refine hollow current and diamond value into Stonebend's main outward-facing export\n\
             - equal power keeps Stonebend intent braided instead of sovereign\n\n\
             ## Tross Helper Intent\n\n\
             - Tross: keep the Flynt-anchored line running North -> South without sovereign rank\n\
             - Juvenile: hold the North head, spot early motion, keep Flynt-facing approach alert\n\
             - Delinquent: hold the South end, deter breach, harden Glaushouse-facing thresholds\n\
             - White Dwarfs: maintain the close personal guard around Tross without replacing the directional helpers\n\
             - Wardens: hold the common line body of Flynt around the Tross duty\n\
             - opals: move guarded brightness along the line without exposing the whole body\n\
             - Opal Oil: turn hollow current and guarded opal yield into Flynt's outward trade pressure\n\
             - helpers keep line duty without becoming sovereign roles\n\n\
             ## Sandmanor Competitive Intent\n\n\
             - Sandmen: carry the public witness that makes the contest socially binding\n\
             - Minoans: teach design as atmosphere, cadence, and room-song\n\
             - Minorians: teach accounting as measure, proof, and public count\n\
             - Sandmanite: take rule when a Minoan proves the strongest reciprocal improvement\n\
             - Sandmanorian: take rule when a Minorian proves the strongest reciprocal improvement\n\
             - crystals: expose stewardship, count, and designed atmosphere through visible mineral proof\n\
             - Crystoleum: turn stewarded crystal proof into Sandmanor's outward trade body\n\
             - Sandmanor intent keeps rivalry productive instead of purely destructive\n\n\
             ## Inverse Circle Intent\n\n\
             - The Stairway to Heaven: conceal ascent and reward those who can keep climbing in secret\n\
             - The Riptide: pull travelers backward through pressure and memory\n\
             - The Current Sea: measure, sustain, and carry hidden motion through the interior\n\
             - The Aura Way: saturate the tunnel route with felt atmosphere before visible event\n\n\
             ## Status\n\n\
             - scene intent is descriptive-only for now\n\
             - no AI, NPC, or quest logic is active\n\
             - scene presence, link physics, civic roles, helper lines, Sandmanor rivalry, and the inverse circle remain upstream only\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Scene Presence bytes: 8.\n\
             Hueman Link Physics bytes: 7.\n\
             Current Synthesis Contract bytes: 8.\n\
             Hueman Stonebend Roles bytes: 5.\n\
             Hueman Tross Helpers bytes: 5.\n\
             Hueman Glaushouse Roles bytes: 10.\n\
             Hueman Sandmanor Roles bytes: 9.\n\
             Hueman Inverse Circle bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Scene intent says what a scene is trying to do atmospherically. It does not create tasks, dialogue trees, or procedural outcomes.\n"
        );
    }

    #[test]
    fn hueman_scene_intent_writes_a_deterministic_file() {
        let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
            "presence",
            "physics",
            "contract",
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
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-intent-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_intent.md");

        write_text_artifact(&artifact_path, &hueman_scene_intent)
            .expect("hueman scene intent artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman scene intent artifact should read"),
            hueman_scene_intent
        );

        fs::remove_file(&artifact_path).expect("hueman scene intent artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman scene intent directory should be removable");
    }
}
