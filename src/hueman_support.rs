use std::path::PathBuf;

pub const CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_base.md";
pub const CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_activation_gate.md";
pub const CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_operational.md";
pub const CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH: &str = "artifacts/current_synthesis_topology.md";
pub const HUEMAN_BOUNDARY_ARTIFACT_PATH: &str = "artifacts/hueman_boundary.md";
pub const HUEMAN_FOURWAY_ARTIFACT_PATH: &str = "artifacts/hueman_fourway.md";
pub const HUEMAN_AURA_TRIAD_ARTIFACT_PATH: &str = "artifacts/hueman_aura_triad.md";
pub const HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH: &str = "artifacts/hueman_aura_behavior.md";
pub const HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH: &str = "artifacts/hueman_archetype_lens.md";
pub const HUEMAN_START_PATHS_ARTIFACT_PATH: &str = "artifacts/hueman_start_paths.md";
pub const HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH: &str = "artifacts/hueman_path_crossovers.md";
pub const HUEMAN_MOTION_MAP_ARTIFACT_PATH: &str = "artifacts/hueman_motion_map.md";
pub const HUEMAN_START_CHOICES_ARTIFACT_PATH: &str = "artifacts/hueman_start_choices.md";

pub fn hueman_boundary_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_BOUNDARY_ARTIFACT_PATH)
}

pub fn hueman_motion_map_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_MOTION_MAP_ARTIFACT_PATH)
}

pub fn hueman_fourway_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_FOURWAY_ARTIFACT_PATH)
}

pub fn hueman_aura_triad_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_AURA_TRIAD_ARTIFACT_PATH)
}

pub fn hueman_aura_behavior_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH)
}

pub fn hueman_archetype_lens_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH)
}

pub fn hueman_start_paths_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_START_PATHS_ARTIFACT_PATH)
}

pub fn hueman_path_crossovers_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH)
}

pub fn hueman_start_choices_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_START_CHOICES_ARTIFACT_PATH)
}

pub fn build_hueman_boundary_from_artifacts(
    current_synthesis_base: &str,
    current_synthesis_activation_gate: &str,
) -> String {
    format!(
        "# Hueman Boundary\n\n\
         ## Stack\n\n\
         ```text\n\
         Hollow Grove\n\
         ↓\n\
         KernelPass\n\
         ↓\n\
         Artifacts\n\
         ↓\n\
         Current Synthesis\n\
         ↓\n\
         Hueman\n\
         ```\n\n\
         ## Layer Role\n\n\
         - Hueman is the later persistent 32-bit collaboration/world layer.\n\
         - Current Synthesis remains the operating-system layer beneath it.\n\
         - Hollow Grove remains the recursive core beneath both.\n\n\
         ## Movement Distinction\n\n\
         - Hollow Grove moves active context through the locked field.\n\
         - Hueman moves the character sprite through the same locked field.\n\
         - Human Core remains the operator anchor.\n\n\
         ## Deferred World Logic\n\n\
         - Stonebend is deferred.\n\
         - Sandmanor is deferred.\n\
         - Glaushouse is deferred.\n\
         - Flynt is deferred.\n\
         - species logic is deferred.\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Activation Status\n\n\
         - Current Synthesis activation remains denied.\n\
         - Hueman world activation is not enabled.\n\
         - collaborative persistence is not enabled.\n\
         - visual world mapping is not enabled.\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis base bytes: {}.\n\
         Current Synthesis activation gate bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Hueman may consume Current Synthesis. Current Synthesis does not know Hueman exists.\n",
        current_synthesis_base.len(),
        current_synthesis_activation_gate.len()
    )
}

pub fn build_hueman_motion_map_from_artifacts(
    hueman_boundary: &str,
    current_synthesis_operational: &str,
) -> String {
    format!(
        "# Hueman Motion Map\n\n\
         ## Locked Field\n\n\
         ```text\n\
         7 Hollow Back     8 Hollow Grove    9 Hollow Bend\n\n\
         4 The Grove       5 Human Core      6 The Hollows\n\n\
         1 Grove Orchard   2 Grove Hollow    3 Grove Falls\n\
         ```\n\n\
         ## Node Classes\n\n\
         - META: `1`, `3`, `7`, `9`\n\
         - PLEB: `4`, `6`, `8`\n\
         - SYNTH: `2`\n\
         - CORE: `5`\n\n\
         ## Hueman Reading\n\n\
         - the sprite moves through the field\n\
         - Human Core remains the operator anchor\n\
         - named world logic remains deferred\n\n\
         ## Lower-Layer Reading Preserved\n\n\
         - Hollow Grove keeps active-context movement\n\
         - Current Synthesis keeps `PLEB`/`META` occupancy\n\
         - the field remains one locked map across layers\n\n\
         ## Artifact Inputs\n\n\
         Hueman boundary bytes: {}.\n\
         Current Synthesis operational bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Hueman reads the map as world-facing representation. Hollow Grove and Current Synthesis keep the lower-layer operating semantics.\n",
        hueman_boundary.len(),
        current_synthesis_operational.len()
    )
}

pub fn build_hueman_fourway_from_artifacts(
    hueman_boundary: &str,
    hueman_motion_map: &str,
) -> String {
    format!(
        "# Hueman Fourway\n\n\
         ## Structural Rule\n\n\
         Hueman runs through the Fourway before resolving downward into AuraTriad and then Triway.\n\n\
         ## Stack\n\n\
         ```text\n\
         Hueman\n\
         ↓\n\
         Fourway\n\
         ↓\n\
         AuraTriad\n\
         ↓\n\
         Triway\n\
         ↓\n\
         Hollow Grove\n\
         ```\n\n\
         ## Four Directions\n\n\
         - North\n\
         - East\n\
         - South\n\
         - West\n\n\
         ## Meaning\n\n\
         - Fourway is the world-facing directional map.\n\
         - Fourway includes straight lines and rounded corner bends.\n\
         - Fourway resolves downward into AuraTriad first.\n\
         - Triway remains the lower recursive split.\n\
         - Fourway does not replace Triway.\n\
         - Fourway does not own PLEB or META.\n\n\
         ## Initial World Roster\n\n\
         - North = Flynt = `goblin`\n\
         - East = Stonebend = `gremlin`\n\
         - South = Glaushouse = `pixy`\n\
         - West = Sandmanor = `sprite`\n\n\
         ## Boundary\n\n\
         - Fourway belongs to Hueman.\n\
         - Triway belongs to Hollow Grove.\n\
         - Current Synthesis does not own Fourway.\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman boundary bytes: {}.\n\
         Hueman motion map bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Fourway is a Hueman/world structure above AuraTriad and the kernel path. It must not redefine Triway.\n",
        hueman_boundary.len(),
        hueman_motion_map.len()
    )
}

pub fn build_hueman_aura_triad_from_artifacts(
    hueman_fourway: &str,
    current_synthesis_topology: &str,
) -> String {
    format!(
        "# Hueman Aura Triad\n\n\
         ## Structural Rule\n\n\
         AuraTriad is the three-region resolution beneath Fourway and above Triway.\n\n\
         ## Stack\n\n\
         ```text\n\
         Hueman\n\
         ↓\n\
         Fourway\n\
         ↓\n\
         AuraTriad\n\
         ↓\n\
         Triway\n\
         ↓\n\
         Hollow Grove\n\
         ```\n\n\
         ## Triad\n\n\
         ```text\n\
         Aura Basin\n\
         ↓\n\
         Aura Fields\n\
         ↓\n\
         Aura Beach\n\
         ```\n\n\
         ## Meaning\n\n\
         - AuraTriad is the world-facing three-region route body beneath Fourway.\n\
         - Current Synthesis already records these as inverse-route regions.\n\
         - Hueman reads them as the triadic resolution of the world map.\n\
         - Triway remains the lower recursive split after this layer.\n\n\
         ## PLEB and META\n\n\
         - `PLEB` and `META` remain Current Synthesis occupancy semantics.\n\
         - AuraTriad does not move `PLEB` or `META` into the kernel.\n\
         - AuraTriad does not redefine Triway.\n\n\
         ## Boundary\n\n\
         - AuraTriad belongs to Hueman as world reading.\n\
         - the source geography remains readable from Current Synthesis.\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Fourway bytes: {}.\n\
         Current Synthesis topology bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         AuraTriad is the bridge between Hueman Fourway and Hollow Grove Triway. It is not itself a kernel structure.\n",
        hueman_fourway.len(),
        current_synthesis_topology.len()
    )
}

pub fn build_hueman_start_choices_from_artifacts(
    hueman_fourway: &str,
    hueman_aura_triad: &str,
) -> String {
    format!(
        "# Hueman Start Choices\n\n\
         ## End User Archetypes\n\n\
         - `goblin`\n\
         - `gremlin`\n\
         - `pixy`\n\
         - `sprite`\n\n\
         ## Starting Places\n\n\
         - Flynt\n\
         - Stonebend\n\
         - Glaushouse\n\
         - Sandmanor\n\n\
         ## Fourway Placement\n\n\
         - North = Flynt = `goblin`\n\
         - East = Stonebend = `gremlin`\n\
         - South = Glaushouse = `pixy`\n\
         - West = Sandmanor = `sprite`\n\n\
         ## Initial Start Roster\n\n\
         - `goblin` starts in Flynt\n\
         - `gremlin` starts in Stonebend\n\
         - `pixy` starts in Glaushouse\n\
         - `sprite` starts in Sandmanor\n\n\
         ## Status\n\n\
         - the end user may choose one archetype\n\
         - the starting place follows the initial Hueman roster\n\
         - the starting direction follows the Fourway roster\n\
         - the world resolves downward through AuraTriad after start choice\n\
         - AuraTriad behavior is descriptive-only after start choice\n\
         - species mechanics are not active yet\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Fourway bytes: {}.\n\
         Hueman AuraTriad bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         This is a Hueman-layer start declaration only. It does not change Current Synthesis or Hollow Grove.\n",
        hueman_fourway.len(),
        hueman_aura_triad.len()
    )
}

pub fn build_hueman_aura_behavior_from_artifacts(
    hueman_aura_triad: &str,
    hueman_start_choices: &str,
) -> String {
    format!(
        "# Hueman Aura Behavior\n\n\
         ## Structural Rule\n\n\
         After the start choice is placed on the Fourway, Hueman reads AuraTriad as three descriptive region states.\n\n\
         ## Entry Order\n\n\
         ```text\n\
         Start Choice\n\
         ↓\n\
         Aura Basin\n\
         ↓\n\
         Aura Fields\n\
         ↓\n\
         Aura Beach\n\
         ```\n\n\
         ## Region States\n\n\
         ### Aura Basin\n\n\
         - movement reads as inward and narrowing\n\
         - encounter tone reads as close, muffled, and formative\n\
         - world description favors pressure, shelter, and accumulation\n\n\
         ### Aura Fields\n\n\
         - movement reads as lateral and exposed\n\
         - encounter tone reads as social, visible, and negotiable\n\
         - world description favors weather, distance, and traversal\n\n\
         ### Aura Beach\n\n\
         - movement reads as outward and threshold-facing\n\
         - encounter tone reads as reflective, sparse, and releasing\n\
         - world description favors edge, horizon, and departure\n\n\
         ## Status\n\n\
         - AuraTriad behavior is descriptive-only for now\n\
         - movement pressure is declarative, not simulated\n\
         - encounter tone is declarative, not procedural\n\
         - the Fourway roster remains unchanged\n\
         - species mechanics are not active yet\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman AuraTriad bytes: {}.\n\
         Hueman Start Choices bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Aura behavior is a Hueman-facing reading of the world after start choice. It does not alter Current Synthesis occupancy or Hollow Grove recursion.\n",
        hueman_aura_triad.len(),
        hueman_start_choices.len()
    )
}

pub fn build_hueman_archetype_lens_from_artifacts(
    hueman_start_choices: &str,
    hueman_aura_behavior: &str,
) -> String {
    format!(
        "# Hueman Archetype Lens\n\n\
         ## Structural Rule\n\n\
         After start choice and AuraTriad behavior are declared, each archetype reads the same regions through a different descriptive lens.\n\n\
         ## Archetype Readings\n\n\
         ### `goblin`\n\n\
         - Aura Basin reads as burrow, shelter, and kept stores\n\
         - Aura Fields reads as forage paths, routes, and workable ground\n\
         - Aura Beach reads as exposed salvage, tide risk, and thin cover\n\n\
         ### `gremlin`\n\n\
         - Aura Basin reads as stress seams, pressure joints, and hidden leverage\n\
         - Aura Fields reads as barter space, friction lines, and noisy crossings\n\
         - Aura Beach reads as scrap edge, discard flow, and threshold apparatus\n\n\
         ### `pixy`\n\n\
         - Aura Basin reads as hush, glow, and suspended potential\n\
         - Aura Fields reads as shimmer, weather play, and visible drift\n\
         - Aura Beach reads as glint, spray, and bright dispersal\n\n\
         ### `sprite`\n\n\
         - Aura Basin reads as root echo, sleep, and soft enclosure\n\
         - Aura Fields reads as current, sway, and open circulation\n\
         - Aura Beach reads as horizon pull, release, and farward motion\n\n\
         ## Status\n\n\
         - archetype lens is descriptive-only for now\n\
         - no procedural bonuses or penalties are active\n\
         - the Fourway start roster remains unchanged\n\
         - AuraTriad region behavior remains shared underneath the lens\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Aura Behavior bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         The archetype lens changes interpretation, not rules. It is a Hueman-facing difference in reading the world after placement.\n",
        hueman_start_choices.len(),
        hueman_aura_behavior.len()
    )
}

pub fn build_hueman_start_paths_from_artifacts(
    hueman_start_choices: &str,
    hueman_archetype_lens: &str,
) -> String {
    format!(
        "# Hueman Start Paths\n\n\
         ## Structural Rule\n\n\
         Each Fourway start enters AuraTriad through a first descriptive region before any procedural mechanics exist.\n\n\
         ## Route Order\n\n\
         - Flynt = `goblin` = Aura Basin -> Aura Fields -> Aura Beach\n\
         - Stonebend = `gremlin` = Aura Fields -> Aura Basin -> Aura Beach\n\
         - Glaushouse = `pixy` = Aura Beach -> Aura Fields -> Aura Basin\n\
         - Sandmanor = `sprite` = Aura Beach -> Aura Basin -> Aura Fields\n\n\
         ## First Entry\n\n\
         - Flynt enters Aura Basin first.\n\
         - Stonebend enters Aura Fields first.\n\
         - Glaushouse enters Aura Beach first.\n\
         - Sandmanor enters Aura Beach first.\n\n\
         ## Status\n\n\
         - start-path order is descriptive-only for now\n\
         - the first region is declared but not procedurally enforced\n\
         - archetype lens remains interpretive above the route order\n\
         - species mechanics are not active yet\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Archetype Lens bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Start paths declare which AuraTriad region a start naturally enters first. They do not add movement rules or alter lower-layer topology.\n",
        hueman_start_choices.len(),
        hueman_archetype_lens.len()
    )
}

pub fn build_hueman_path_crossovers_from_artifacts(
    hueman_start_paths: &str,
    hueman_aura_behavior: &str,
) -> String {
    format!(
        "# Hueman Path Crossovers\n\n\
         ## Structural Rule\n\n\
         Different starts may enter AuraTriad differently while still crossing through shared regions and shared world pressure.\n\n\
         ## Shared Entry Crossovers\n\n\
         - Glaushouse and Sandmanor cross immediately at Aura Beach.\n\
         - Flynt and Stonebend do not share first entry, but they both begin inland before reaching the coast.\n\n\
         ## Interior Crossovers\n\n\
         - Flynt and Sandmanor cross at Aura Basin.\n\
         - Stonebend and Glaushouse cross at Aura Fields.\n\
         - Stonebend and Sandmanor cross at Aura Basin after different openings.\n\n\
         ## Full-Triad Convergence\n\n\
         - all four starts eventually touch Aura Basin\n\
         - all four starts eventually touch Aura Fields\n\
         - all four starts eventually touch Aura Beach\n\
         - the difference is order, not exclusion\n\n\
         ## Meaning\n\n\
         - crossover means the world can feel shared without erasing start identity\n\
         - shared regions carry different descriptive pressure depending on entry order\n\
         - the coast is the earliest common threshold for the western and southern starts\n\
         - inland turns remain the main crossover pressure for the northern and eastern starts\n\n\
         ## Status\n\n\
         - crossovers are descriptive-only for now\n\
         - no meeting mechanics or shared events are active\n\
         - start-path order remains unchanged\n\
         - archetype lens remains interpretive above the crossover map\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Start Paths bytes: {}.\n\
         Hueman Aura Behavior bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Path crossovers declare where starts can meaningfully overlap in AuraTriad. They do not create procedural encounters or alter lower-layer routing.\n",
        hueman_start_paths.len(),
        hueman_aura_behavior.len()
    )
}

#[cfg(test)]
mod tests {
    use super::{
        build_hueman_archetype_lens_from_artifacts,
        build_hueman_aura_behavior_from_artifacts,
        build_hueman_aura_triad_from_artifacts,
        build_hueman_boundary_from_artifacts, build_hueman_fourway_from_artifacts,
        build_hueman_motion_map_from_artifacts,
        build_hueman_path_crossovers_from_artifacts,
        build_hueman_start_paths_from_artifacts,
        build_hueman_start_choices_from_artifacts,
    };

    #[test]
    fn hueman_boundary_builder_is_deterministic() {
        assert_eq!(
            build_hueman_boundary_from_artifacts("base", "gate"),
            "# Hueman Boundary\n\n\
             ## Stack\n\n\
             ```text\n\
             Hollow Grove\n\
             ↓\n\
             KernelPass\n\
             ↓\n\
             Artifacts\n\
             ↓\n\
             Current Synthesis\n\
             ↓\n\
             Hueman\n\
             ```\n\n\
             ## Layer Role\n\n\
             - Hueman is the later persistent 32-bit collaboration/world layer.\n\
             - Current Synthesis remains the operating-system layer beneath it.\n\
             - Hollow Grove remains the recursive core beneath both.\n\n\
             ## Movement Distinction\n\n\
             - Hollow Grove moves active context through the locked field.\n\
             - Hueman moves the character sprite through the same locked field.\n\
             - Human Core remains the operator anchor.\n\n\
             ## Deferred World Logic\n\n\
             - Stonebend is deferred.\n\
             - Sandmanor is deferred.\n\
             - Glaushouse is deferred.\n\
             - Flynt is deferred.\n\
             - species logic is deferred.\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Activation Status\n\n\
             - Current Synthesis activation remains denied.\n\
             - Hueman world activation is not enabled.\n\
             - collaborative persistence is not enabled.\n\
             - visual world mapping is not enabled.\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis base bytes: 4.\n\
             Current Synthesis activation gate bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Hueman may consume Current Synthesis. Current Synthesis does not know Hueman exists.\n"
        );
    }

    #[test]
    fn hueman_motion_map_builder_is_deterministic() {
        assert_eq!(
            build_hueman_motion_map_from_artifacts("boundary", "ops"),
            "# Hueman Motion Map\n\n\
             ## Locked Field\n\n\
             ```text\n\
             7 Hollow Back     8 Hollow Grove    9 Hollow Bend\n\n\
             4 The Grove       5 Human Core      6 The Hollows\n\n\
             1 Grove Orchard   2 Grove Hollow    3 Grove Falls\n\
             ```\n\n\
             ## Node Classes\n\n\
             - META: `1`, `3`, `7`, `9`\n\
             - PLEB: `4`, `6`, `8`\n\
             - SYNTH: `2`\n\
             - CORE: `5`\n\n\
             ## Hueman Reading\n\n\
             - the sprite moves through the field\n\
             - Human Core remains the operator anchor\n\
             - named world logic remains deferred\n\n\
             ## Lower-Layer Reading Preserved\n\n\
             - Hollow Grove keeps active-context movement\n\
             - Current Synthesis keeps `PLEB`/`META` occupancy\n\
             - the field remains one locked map across layers\n\n\
             ## Artifact Inputs\n\n\
             Hueman boundary bytes: 8.\n\
             Current Synthesis operational bytes: 3.\n\n\
             ## Boundary Reminder\n\n\
             Hueman reads the map as world-facing representation. Hollow Grove and Current Synthesis keep the lower-layer operating semantics.\n"
        );
    }

    #[test]
    fn hueman_start_choices_builder_is_deterministic() {
        assert_eq!(
            build_hueman_start_choices_from_artifacts("fourway", "triad"),
            "# Hueman Start Choices\n\n\
             ## End User Archetypes\n\n\
             - `goblin`\n\
             - `gremlin`\n\
             - `pixy`\n\
             - `sprite`\n\n\
             ## Starting Places\n\n\
             - Flynt\n\
             - Stonebend\n\
             - Glaushouse\n\
             - Sandmanor\n\n\
             ## Fourway Placement\n\n\
             - North = Flynt = `goblin`\n\
             - East = Stonebend = `gremlin`\n\
             - South = Glaushouse = `pixy`\n\
             - West = Sandmanor = `sprite`\n\n\
             ## Initial Start Roster\n\n\
             - `goblin` starts in Flynt\n\
             - `gremlin` starts in Stonebend\n\
             - `pixy` starts in Glaushouse\n\
             - `sprite` starts in Sandmanor\n\n\
             ## Status\n\n\
             - the end user may choose one archetype\n\
             - the starting place follows the initial Hueman roster\n\
             - the starting direction follows the Fourway roster\n\
             - the world resolves downward through AuraTriad after start choice\n\
             - AuraTriad behavior is descriptive-only after start choice\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Fourway bytes: 7.\n\
             Hueman AuraTriad bytes: 5.\n\n\
             ## Boundary Reminder\n\n\
             This is a Hueman-layer start declaration only. It does not change Current Synthesis or Hollow Grove.\n"
        );
    }

    #[test]
    fn hueman_fourway_builder_is_deterministic() {
        assert_eq!(
            build_hueman_fourway_from_artifacts("boundary", "motion"),
            "# Hueman Fourway\n\n\
             ## Structural Rule\n\n\
             Hueman runs through the Fourway before resolving downward into AuraTriad and then Triway.\n\n\
             ## Stack\n\n\
             ```text\n\
             Hueman\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             AuraTriad\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Hollow Grove\n\
             ```\n\n\
             ## Four Directions\n\n\
             - North\n\
             - East\n\
             - South\n\
             - West\n\n\
             ## Meaning\n\n\
             - Fourway is the world-facing directional map.\n\
             - Fourway includes straight lines and rounded corner bends.\n\
             - Fourway resolves downward into AuraTriad first.\n\
             - Triway remains the lower recursive split.\n\
             - Fourway does not replace Triway.\n\
             - Fourway does not own PLEB or META.\n\n\
             ## Initial World Roster\n\n\
             - North = Flynt = `goblin`\n\
             - East = Stonebend = `gremlin`\n\
             - South = Glaushouse = `pixy`\n\
             - West = Sandmanor = `sprite`\n\n\
             ## Boundary\n\n\
             - Fourway belongs to Hueman.\n\
             - Triway belongs to Hollow Grove.\n\
             - Current Synthesis does not own Fourway.\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman boundary bytes: 8.\n\
             Hueman motion map bytes: 6.\n\n\
             ## Boundary Reminder\n\n\
             Fourway is a Hueman/world structure above AuraTriad and the kernel path. It must not redefine Triway.\n"
        );
    }

    #[test]
    fn hueman_aura_triad_builder_is_deterministic() {
        assert_eq!(
            build_hueman_aura_triad_from_artifacts("fourway", "topology"),
            "# Hueman Aura Triad\n\n\
             ## Structural Rule\n\n\
             AuraTriad is the three-region resolution beneath Fourway and above Triway.\n\n\
             ## Stack\n\n\
             ```text\n\
             Hueman\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             AuraTriad\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Hollow Grove\n\
             ```\n\n\
             ## Triad\n\n\
             ```text\n\
             Aura Basin\n\
             ↓\n\
             Aura Fields\n\
             ↓\n\
             Aura Beach\n\
             ```\n\n\
             ## Meaning\n\n\
             - AuraTriad is the world-facing three-region route body beneath Fourway.\n\
             - Current Synthesis already records these as inverse-route regions.\n\
             - Hueman reads them as the triadic resolution of the world map.\n\
             - Triway remains the lower recursive split after this layer.\n\n\
             ## PLEB and META\n\n\
             - `PLEB` and `META` remain Current Synthesis occupancy semantics.\n\
             - AuraTriad does not move `PLEB` or `META` into the kernel.\n\
             - AuraTriad does not redefine Triway.\n\n\
             ## Boundary\n\n\
             - AuraTriad belongs to Hueman as world reading.\n\
             - the source geography remains readable from Current Synthesis.\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Fourway bytes: 7.\n\
             Current Synthesis topology bytes: 8.\n\n\
             ## Boundary Reminder\n\n\
             AuraTriad is the bridge between Hueman Fourway and Hollow Grove Triway. It is not itself a kernel structure.\n"
        );
    }

    #[test]
    fn hueman_aura_behavior_builder_is_deterministic() {
        assert_eq!(
            build_hueman_aura_behavior_from_artifacts("triad", "start"),
            "# Hueman Aura Behavior\n\n\
             ## Structural Rule\n\n\
             After the start choice is placed on the Fourway, Hueman reads AuraTriad as three descriptive region states.\n\n\
             ## Entry Order\n\n\
             ```text\n\
             Start Choice\n\
             ↓\n\
             Aura Basin\n\
             ↓\n\
             Aura Fields\n\
             ↓\n\
             Aura Beach\n\
             ```\n\n\
             ## Region States\n\n\
             ### Aura Basin\n\n\
             - movement reads as inward and narrowing\n\
             - encounter tone reads as close, muffled, and formative\n\
             - world description favors pressure, shelter, and accumulation\n\n\
             ### Aura Fields\n\n\
             - movement reads as lateral and exposed\n\
             - encounter tone reads as social, visible, and negotiable\n\
             - world description favors weather, distance, and traversal\n\n\
             ### Aura Beach\n\n\
             - movement reads as outward and threshold-facing\n\
             - encounter tone reads as reflective, sparse, and releasing\n\
             - world description favors edge, horizon, and departure\n\n\
             ## Status\n\n\
             - AuraTriad behavior is descriptive-only for now\n\
             - movement pressure is declarative, not simulated\n\
             - encounter tone is declarative, not procedural\n\
             - the Fourway roster remains unchanged\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman AuraTriad bytes: 5.\n\
             Hueman Start Choices bytes: 5.\n\n\
             ## Boundary Reminder\n\n\
             Aura behavior is a Hueman-facing reading of the world after start choice. It does not alter Current Synthesis occupancy or Hollow Grove recursion.\n"
        );
    }

    #[test]
    fn hueman_archetype_lens_builder_is_deterministic() {
        assert_eq!(
            build_hueman_archetype_lens_from_artifacts("start", "aura"),
            "# Hueman Archetype Lens\n\n\
             ## Structural Rule\n\n\
             After start choice and AuraTriad behavior are declared, each archetype reads the same regions through a different descriptive lens.\n\n\
             ## Archetype Readings\n\n\
             ### `goblin`\n\n\
             - Aura Basin reads as burrow, shelter, and kept stores\n\
             - Aura Fields reads as forage paths, routes, and workable ground\n\
             - Aura Beach reads as exposed salvage, tide risk, and thin cover\n\n\
             ### `gremlin`\n\n\
             - Aura Basin reads as stress seams, pressure joints, and hidden leverage\n\
             - Aura Fields reads as barter space, friction lines, and noisy crossings\n\
             - Aura Beach reads as scrap edge, discard flow, and threshold apparatus\n\n\
             ### `pixy`\n\n\
             - Aura Basin reads as hush, glow, and suspended potential\n\
             - Aura Fields reads as shimmer, weather play, and visible drift\n\
             - Aura Beach reads as glint, spray, and bright dispersal\n\n\
             ### `sprite`\n\n\
             - Aura Basin reads as root echo, sleep, and soft enclosure\n\
             - Aura Fields reads as current, sway, and open circulation\n\
             - Aura Beach reads as horizon pull, release, and farward motion\n\n\
             ## Status\n\n\
             - archetype lens is descriptive-only for now\n\
             - no procedural bonuses or penalties are active\n\
             - the Fourway start roster remains unchanged\n\
             - AuraTriad region behavior remains shared underneath the lens\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Aura Behavior bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             The archetype lens changes interpretation, not rules. It is a Hueman-facing difference in reading the world after placement.\n"
        );
    }

    #[test]
    fn hueman_start_paths_builder_is_deterministic() {
        assert_eq!(
            build_hueman_start_paths_from_artifacts("start", "lens"),
            "# Hueman Start Paths\n\n\
             ## Structural Rule\n\n\
             Each Fourway start enters AuraTriad through a first descriptive region before any procedural mechanics exist.\n\n\
             ## Route Order\n\n\
             - Flynt = `goblin` = Aura Basin -> Aura Fields -> Aura Beach\n\
             - Stonebend = `gremlin` = Aura Fields -> Aura Basin -> Aura Beach\n\
             - Glaushouse = `pixy` = Aura Beach -> Aura Fields -> Aura Basin\n\
             - Sandmanor = `sprite` = Aura Beach -> Aura Basin -> Aura Fields\n\n\
             ## First Entry\n\n\
             - Flynt enters Aura Basin first.\n\
             - Stonebend enters Aura Fields first.\n\
             - Glaushouse enters Aura Beach first.\n\
             - Sandmanor enters Aura Beach first.\n\n\
             ## Status\n\n\
             - start-path order is descriptive-only for now\n\
             - the first region is declared but not procedurally enforced\n\
             - archetype lens remains interpretive above the route order\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Archetype Lens bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Start paths declare which AuraTriad region a start naturally enters first. They do not add movement rules or alter lower-layer topology.\n"
        );
    }

    #[test]
    fn hueman_path_crossovers_builder_is_deterministic() {
        assert_eq!(
            build_hueman_path_crossovers_from_artifacts("paths", "aura"),
            "# Hueman Path Crossovers\n\n\
             ## Structural Rule\n\n\
             Different starts may enter AuraTriad differently while still crossing through shared regions and shared world pressure.\n\n\
             ## Shared Entry Crossovers\n\n\
             - Glaushouse and Sandmanor cross immediately at Aura Beach.\n\
             - Flynt and Stonebend do not share first entry, but they both begin inland before reaching the coast.\n\n\
             ## Interior Crossovers\n\n\
             - Flynt and Sandmanor cross at Aura Basin.\n\
             - Stonebend and Glaushouse cross at Aura Fields.\n\
             - Stonebend and Sandmanor cross at Aura Basin after different openings.\n\n\
             ## Full-Triad Convergence\n\n\
             - all four starts eventually touch Aura Basin\n\
             - all four starts eventually touch Aura Fields\n\
             - all four starts eventually touch Aura Beach\n\
             - the difference is order, not exclusion\n\n\
             ## Meaning\n\n\
             - crossover means the world can feel shared without erasing start identity\n\
             - shared regions carry different descriptive pressure depending on entry order\n\
             - the coast is the earliest common threshold for the western and southern starts\n\
             - inland turns remain the main crossover pressure for the northern and eastern starts\n\n\
             ## Status\n\n\
             - crossovers are descriptive-only for now\n\
             - no meeting mechanics or shared events are active\n\
             - start-path order remains unchanged\n\
             - archetype lens remains interpretive above the crossover map\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Paths bytes: 5.\n\
             Hueman Aura Behavior bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Path crossovers declare where starts can meaningfully overlap in AuraTriad. They do not create procedural encounters or alter lower-layer routing.\n"
        );
    }
}
