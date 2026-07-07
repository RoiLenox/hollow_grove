use std::path::PathBuf;

pub const CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_base.md";
pub const CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_activation_gate.md";
pub const CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_operational.md";
pub const HUEMAN_BOUNDARY_ARTIFACT_PATH: &str = "artifacts/hueman_boundary.md";
pub const HUEMAN_MOTION_MAP_ARTIFACT_PATH: &str = "artifacts/hueman_motion_map.md";

pub fn hueman_boundary_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_BOUNDARY_ARTIFACT_PATH)
}

pub fn hueman_motion_map_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_MOTION_MAP_ARTIFACT_PATH)
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

#[cfg(test)]
mod tests {
    use super::{
        build_hueman_boundary_from_artifacts, build_hueman_motion_map_from_artifacts,
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
}
