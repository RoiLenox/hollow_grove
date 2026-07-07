use std::path::PathBuf;

pub const CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_base.md";
pub const CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_activation_gate.md";
pub const CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_operational.md";
pub const HUEMAN_BOUNDARY_ARTIFACT_PATH: &str = "artifacts/hueman_boundary.md";
pub const HUEMAN_FOURWAY_ARTIFACT_PATH: &str = "artifacts/hueman_fourway.md";
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
         Hueman runs through the Fourway before resolving downward into Triway.\n\n\
         ## Stack\n\n\
         ```text\n\
         Hueman\n\
         ↓\n\
         Fourway\n\
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
         - Triway remains the lower recursive split.\n\
         - Fourway does not replace Triway.\n\
         - Fourway resolves downward into Triway.\n\n\
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
         Fourway is a Hueman/world structure above the kernel path. It must not redefine Triway.\n",
        hueman_boundary.len(),
        hueman_motion_map.len()
    )
}

pub fn build_hueman_start_choices_from_artifacts(
    hueman_fourway: &str,
    hueman_motion_map: &str,
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
         - world behavior is not active yet\n\
         - species mechanics are not active yet\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Fourway bytes: {}.\n\
         Hueman motion map bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         This is a Hueman-layer start declaration only. It does not change Current Synthesis or Hollow Grove.\n",
        hueman_fourway.len(),
        hueman_motion_map.len()
    )
}

#[cfg(test)]
mod tests {
    use super::{
        build_hueman_boundary_from_artifacts, build_hueman_fourway_from_artifacts,
        build_hueman_motion_map_from_artifacts,
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
            build_hueman_start_choices_from_artifacts("fourway", "motion"),
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
             - world behavior is not active yet\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Fourway bytes: 7.\n\
             Hueman motion map bytes: 6.\n\n\
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
             Hueman runs through the Fourway before resolving downward into Triway.\n\n\
             ## Stack\n\n\
             ```text\n\
             Hueman\n\
             ↓\n\
             Fourway\n\
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
             - Triway remains the lower recursive split.\n\
             - Fourway does not replace Triway.\n\
             - Fourway resolves downward into Triway.\n\n\
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
             Fourway is a Hueman/world structure above the kernel path. It must not redefine Triway.\n"
        );
    }
}
