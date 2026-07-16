#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceKingdom {
    Stonebend,
    Sandmanor,
    Glaushouse,
    Flynt,
}

impl SliceKingdom {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stonebend => "Stonebend",
            Self::Sandmanor => "Sandmanor",
            Self::Glaushouse => "Glaushouse",
            Self::Flynt => "Flynt",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceLoopStage {
    pub kingdom: SliceKingdom,
    pub constitutional_action: &'static str,
    pub implementation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceResolutionPath {
    RouteStabilization,
    FlockDefense,
}

impl SliceResolutionPath {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RouteStabilization => "route",
            Self::FlockDefense => "defense",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "route" => Some(Self::RouteStabilization),
            "defense" => Some(Self::FlockDefense),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceResolutionOption {
    pub path: SliceResolutionPath,
    pub label: &'static str,
    pub proof_condition: &'static str,
    pub clearance_condition: &'static str,
    pub deployment: &'static str,
    pub recognition_result: &'static str,
    pub failure_condition: &'static str,
    pub produced_resource: &'static str,
    pub produced_resource_units: u8,
    pub recognition_credential: &'static str,
    pub follow_up_focus: &'static str,
    pub follow_up_task_title: &'static str,
    pub follow_up_task_summary: &'static str,
    pub follow_up_task_start: &'static str,
    pub follow_up_task_completion: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalSliceSpec {
    pub id: &'static str,
    pub title: &'static str,
    pub opening_location: &'static str,
    pub opening_need: &'static str,
    pub route_context: &'static str,
    pub current_state: &'static str,
    pub aura_state: &'static str,
    pub signature_resource: &'static str,
    pub required_regular_current_units: u8,
    pub required_holographic_aura_units: u8,
    pub required_opal_oil_units: u8,
    pub aura_view: &'static str,
    pub aura_glow: &'static str,
    pub ordinary_skill_track: &'static [&'static str],
    pub current_form: &'static str,
    pub unlock_node: &'static str,
    pub transformation_unlock: &'static str,
    pub crafted_object: &'static str,
    pub deployment_result: &'static str,
    pub default_resolution_path: SliceResolutionPath,
    pub resolution_options: &'static [SliceResolutionOption],
    pub tested_systems: &'static [&'static str],
    pub open_questions: &'static [&'static str],
    pub loop_stages: &'static [SliceLoopStage],
}

impl VerticalSliceSpec {
    pub fn resolution_option(
        &self,
        path: SliceResolutionPath,
    ) -> Option<&'static SliceResolutionOption> {
        self.resolution_options
            .iter()
            .find(|option| option.path == path)
    }
}

pub const FLOODED_QUARRY_VERTICAL_SLICE_LOOP: [SliceLoopStage; 4] = [
    SliceLoopStage {
        kingdom: SliceKingdom::Stonebend,
        constitutional_action: "Name it",
        implementation: "Name a quarry-stable load control tool as the `Spillrail Latch`, making its purpose explicit: hold the crane rim, route flood pressure, and keep cargo lanes legible under night watch.",
    },
    SliceLoopStage {
        kingdom: SliceKingdom::Sandmanor,
        constitutional_action: "Prove it",
        implementation: "Run a pressure proof that checks latch hold, pump timing, and route marker stability under a short flood pulse before field use.",
    },
    SliceLoopStage {
        kingdom: SliceKingdom::Glaushouse,
        constitutional_action: "Clear it",
        implementation: "Perform a sealing and recovery pass that confirms the latch body, lantern marks, and operator stance remain safe on a wet quarry rim.",
    },
    SliceLoopStage {
        kingdom: SliceKingdom::Flynt,
        constitutional_action: "Recognize it",
        implementation: "Use the latch on the live crane rim so workers, pumps, and cargo lanes all prove the tool under real night-watch pressure.",
    },
];

pub const FLOODED_QUARRY_VERTICAL_SLICE_RESOLUTIONS: [SliceResolutionOption; 2] = [
    SliceResolutionOption {
        path: SliceResolutionPath::RouteStabilization,
        label: "Crane Route Hold",
        proof_condition: "Bench proof must verify cable catch timing, spillrail hold, and route marker visibility through a full quarry surge cycle.",
        clearance_condition: "Clearance must confirm wet-rim footing, latch seal integrity, and safe load transfer from the drowned shelf back to the crane path.",
        deployment: "Lock the Spillrail Latch into the crane rim, route flood pressure into the spillrail, and reopen the cargo lane before the next ore wagon reaches the quarry edge.",
        recognition_result: "The crane path holds through midnight, the ore lane reopens, and the watch recognizes the player for restoring industrial flow through Stonebend/Flynt infrastructure discipline.",
        failure_condition: "Route failure occurs if the latch slips, the cable timing breaks, or the reopened lane collapses back into flood pressure before the wagon crosses.",
        produced_resource: "Spillrail Route Seal",
        produced_resource_units: 1,
        recognition_credential: "Quarry Rim Trust",
        follow_up_focus: "Biases later Stonebend and Flynt work toward heavy-route custody, cable timing, and industrial load redistribution.",
        follow_up_task_title: "Crane Marker Survey",
        follow_up_task_summary: "Carry the Spillrail Route Seal along the reopened rim, identify which cargo markers still drift under pressure, and prepare the first permanent quarry routing correction.",
        follow_up_task_start: "Carry the Spillrail Route Seal onto the crane rim and begin the crane marker survey.",
        follow_up_task_completion: "Complete the crane marker survey, identify the unstable cargo markers, and unlock the first permanent quarry routing correction.",
    },
    SliceResolutionOption {
        path: SliceResolutionPath::FlockDefense,
        label: "Pump Intake Hold",
        proof_condition: "Bench proof must verify intake bracing, lantern signal clarity, and operator endurance during a short blackout-pressure burst.",
        clearance_condition: "Clearance must confirm tower footing, brace heat spread, and enough recovery space to keep workers moving between the pumps and the rim without panic.",
        deployment: "Use the Spillrail Latch to brace the intake tower edge, keep the pumps online, and maintain a lit evacuation lane while the flood pulse peaks.",
        recognition_result: "The pumps survive the night, the intake tower stays online, and the watch recognizes the player for preserving lives and power under direct quarry pressure.",
        failure_condition: "Defense failure occurs if the tower brace gives way, the signal lane goes dark, or pump loss forces the night watch to abandon the intake floor.",
        produced_resource: "Intake Brace Charge",
        produced_resource_units: 1,
        recognition_credential: "Tower Watch Trust",
        follow_up_focus: "Biases later Glaushouse and Sandmanor work toward intake resilience, emergency signaling, and blackout recovery.",
        follow_up_task_title: "Pump Relay Audit",
        follow_up_task_summary: "Carry the Intake Brace Charge through the tower relay path, identify the weakest blackout handoff, and prepare the first permanent quarry emergency circuit.",
        follow_up_task_start: "Carry the Intake Brace Charge into the tower relay path and begin the pump relay audit.",
        follow_up_task_completion: "Complete the pump relay audit, identify the weakest blackout handoff, and unlock the first permanent quarry emergency circuit.",
    },
];

pub const PRIMARY_VERTICAL_SLICE_LOOP: [SliceLoopStage; 4] = [
    SliceLoopStage {
        kingdom: SliceKingdom::Stonebend,
        constitutional_action: "Name it",
        implementation: "Name a small field tool as the `Ridge Lantern Drill`, making its purpose explicit: stabilize a route hinge, light a threatened fence line, and carry pressure safely.",
    },
    SliceLoopStage {
        kingdom: SliceKingdom::Sandmanor,
        constitutional_action: "Prove it",
        implementation: "Run a short bench test that checks pressure stability, burn duration, and safe current draw before the tool is allowed into the field.",
    },
    SliceLoopStage {
        kingdom: SliceKingdom::Glaushouse,
        constitutional_action: "Clear it",
        implementation: "Perform a small sealing and alignment pass that verifies the drill housing, lantern feed, and Hueman handling posture are safe enough to use.",
    },
    SliceLoopStage {
        kingdom: SliceKingdom::Flynt,
        constitutional_action: "Recognize it",
        implementation: "Use the tool on the actual ridge problem at night so the route, flock, and fence line all demonstrate that the design works under lived pressure.",
    },
];

pub const PRIMARY_VERTICAL_SLICE_RESOLUTIONS: [SliceResolutionOption; 2] = [
    SliceResolutionOption {
        path: SliceResolutionPath::RouteStabilization,
        label: "Route Stabilization",
        proof_condition: "Bench proof must verify hinge-post stability, lantern burn duration, and safe current draw across a full route-night cycle.",
        clearance_condition: "Clearance must confirm hinge alignment, lantern sealing, and pressure reallocation into the route edge without overloading the pasture fence.",
        deployment: "Drive the Ridge Lantern Drill into the failing hinge post, relight the fence lantern, and relocate pressure so the route edge and pasture boundary hold without escalating into a fight.",
        recognition_result: "The ridge hinge stays open, the fence line glows through the night, and the settlement recognizes the player for solving the pressure problem through infrastructure-first Flynt engineering.",
        failure_condition: "Route failure occurs if the hinge still buckles, the lantern gutters out, or the pressure relocation shifts the problem farther down the fence line.",
        produced_resource: "Hinge Seal Charge",
        produced_resource_units: 1,
        recognition_credential: "Ridge Hinge Trust",
        follow_up_focus: "Biases later Flynt and Stonebend route work toward infrastructure hold, hinge tuning, and pressure relocation.",
        follow_up_task_title: "Route Hinge Survey",
        follow_up_task_summary: "Carry the Hinge Seal Charge to two weaker posts along the trade leg, survey which one is truly bearing the redirected pressure, and prepare the first named route maintenance plan.",
        follow_up_task_start: "Carry the Hinge Seal Charge down the trade leg and begin the route hinge survey at the first weak post.",
        follow_up_task_completion: "Complete the route hinge survey, identify the true bearing post, and unlock the first named route maintenance plan.",
    },
    SliceResolutionOption {
        path: SliceResolutionPath::FlockDefense,
        label: "Flock Defense",
        proof_condition: "Bench proof must verify anchor hold, glare spread, and operator recovery under a short high-pressure defense burst.",
        clearance_condition: "Clearance must confirm grip safety, heat shielding, and enough protected movement space to keep the flock separated from the wolf line.",
        deployment: "Use the Ridge Lantern Drill as a pressure-safe anchor on the threatened fence line, creating a lit work zone that lets the player hold wolves off and protect the flock during a direct defense pass.",
        recognition_result: "The flock survives the night, the wolves are repelled from the hinge line, and the settlement recognizes the player for field-ready defense work without reducing the slice to combat alone.",
        failure_condition: "Defense failure occurs if the anchor slips, the work zone collapses, or the flock line breaks under direct pressure before dawn.",
        produced_resource: "Ward Flare Charge",
        produced_resource_units: 1,
        recognition_credential: "Flockline Trust",
        follow_up_focus: "Biases later Flynt and Glaushouse work toward emergency holding lines, rescue cover, and live-pressure response.",
        follow_up_task_title: "Shelterline Night Watch",
        follow_up_task_summary: "Carry the Ward Flare Charge to the recovery shelter edge, map the safest fallback lane for livestock and patients, and prepare the first live-pressure rescue hold.",
        follow_up_task_start: "Carry the Ward Flare Charge to the recovery shelter edge and begin the shelterline night watch.",
        follow_up_task_completion: "Complete the shelterline night watch, map the fallback lane, and unlock the first live-pressure rescue hold.",
    },
];

pub const PRIMARY_VERTICAL_SLICE_SPEC: VerticalSliceSpec = VerticalSliceSpec {
    id: "aura_ridge_opal_oil_gremlin",
    title: "Aura Ridge Opal Oil Starter Loop",
    opening_location: "A compact Aura Ridge field settlement at the meeting pressure of Stonebend, Sandmanor, and Glaushouse, with Flynt present through route hardware and field engineering.",
    opening_need: "Night pressure from wolves and failing fence lights is breaking the ridge hinge between pasture, trade path, and recovery shelter.",
    route_context: "The opening tests Aura Ridge as a real crossover body instead of a vague holding pen, while keeping Flynt present through infrastructure and recognition work.",
    current_state: "Regular Current",
    aura_state: "Holographic Aura",
    signature_resource: "Opal Oil",
    required_regular_current_units: 2,
    required_holographic_aura_units: 2,
    required_opal_oil_units: 1,
    aura_view: "Flynt Aura View reveals safe drilling seams, route load, current veins, and unstable fence nodes.",
    aura_glow: "Aura Glow intensifies near the safest current seam first, then retunes toward the damaged fence lantern that most needs field recognition.",
    ordinary_skill_track: &[
        "route survey",
        "pressure safety",
        "field drilling",
        "basic seal inspection",
    ],
    current_form: "Gremlin",
    unlock_node: "Load-Bearing Grip",
    transformation_unlock: "Unlock the first Gremlin node, `Load-Bearing Grip`, after the Ridge Lantern Drill is recognized in live use.",
    crafted_object: "Ridge Lantern Drill",
    deployment_result: "The player restores one threatened route edge, protects livestock without forcing a pure combat solution, and earns the first local Recognition that expands Flynt-oriented work capacity.",
    default_resolution_path: SliceResolutionPath::RouteStabilization,
    resolution_options: &PRIMARY_VERTICAL_SLICE_RESOLUTIONS,
    tested_systems: &[
        "ordinary Hueman traversal",
        "Aura Glow target search",
        "Flynt Aura View reading",
        "resource gathering",
        "Opal Oil refinement",
        "small craft naming",
        "bench proof",
        "repair / clearance interaction",
        "field deployment",
        "first Gremlin progression gate",
    ],
    open_questions: &[
        "exact opening micro-map geometry around Aura Ridge",
        "final UI presentation of Flynt Aura View",
        "final balance between the route-stabilization and flock-defense recognition paths",
        "how much of the proof step is minigame versus checklist versus simulation",
    ],
    loop_stages: &PRIMARY_VERTICAL_SLICE_LOOP,
};

pub const FLOODED_QUARRY_VERTICAL_SLICE_SPEC: VerticalSliceSpec = VerticalSliceSpec {
    id: "flooded_quarry_spillrail_latch",
    title: "Flooded Quarry Night Watch Loop",
    opening_location: "A flooded quarry rim where Stonebend load paths and Sandmanor pressure-keeping meet under a wet Glaushouse recovery watch, with Flynt present through industrial routing and field engineering.",
    opening_need: "Night flood pressure is breaking the crane path and threatening the intake tower before the next ore wagon arrives.",
    route_context: "The opening tests a heavy industrial crossover body instead of a pastoral ridge body, while keeping recognition tied to working infrastructure and live watch pressure.",
    current_state: "Regular Current",
    aura_state: "Reflective Aura",
    signature_resource: "Mercury Mirror",
    required_regular_current_units: 2,
    required_holographic_aura_units: 1,
    required_opal_oil_units: 1,
    aura_view: "Stonebend/Flynt quarry sight reveals cable timing, spillrail stress, and flood transfer seams across the drowned shelf.",
    aura_glow: "Aura Glow hardens first around the safest rim latch point, then shifts toward the intake tower edge that will fail next without active routing.",
    ordinary_skill_track: &[
        "load reading",
        "wet-rim footing",
        "cable timing",
        "tower brace inspection",
    ],
    current_form: "Goblin",
    unlock_node: "Loadline Grip",
    transformation_unlock: "Unlock the first Goblin node, `Loadline Grip`, after the Spillrail Latch is recognized in live quarry use.",
    crafted_object: "Spillrail Latch",
    deployment_result: "The player restores one heavy route or holds one critical intake lane through the night watch, earning the first quarry-grade Recognition that expands Stonebend/Flynt industrial work.",
    default_resolution_path: SliceResolutionPath::RouteStabilization,
    resolution_options: &FLOODED_QUARRY_VERTICAL_SLICE_RESOLUTIONS,
    tested_systems: &[
        "ordinary Hueman traversal",
        "quarry seam reading",
        "cable timing observation",
        "resource gathering",
        "Mercury Mirror handling",
        "small craft naming",
        "bench proof",
        "repair / clearance interaction",
        "field deployment",
        "first Goblin progression gate",
    ],
    open_questions: &[
        "exact quarry verticality and drowned shelf geometry",
        "how quarry seam reading differs visually from Aura Ridge",
        "final balance between the crane-route and intake-hold recognition paths",
        "how much blackout pressure is systemic versus scripted in the first quarry watch",
    ],
    loop_stages: &FLOODED_QUARRY_VERTICAL_SLICE_LOOP,
};

pub fn primary_vertical_slice() -> &'static VerticalSliceSpec {
    &PRIMARY_VERTICAL_SLICE_SPEC
}

pub fn vertical_slice_by_id(id: &str) -> Option<&'static VerticalSliceSpec> {
    match id {
        "aura_ridge_opal_oil_gremlin" => Some(&PRIMARY_VERTICAL_SLICE_SPEC),
        "flooded_quarry_spillrail_latch" => Some(&FLOODED_QUARRY_VERTICAL_SLICE_SPEC),
        _ => None,
    }
}

pub fn vertical_slice_for_current_synthesis_scenario(
    scenario_id: &str,
) -> Option<&'static VerticalSliceSpec> {
    match scenario_id {
        "scout_valley_vertical_slice" => Some(&PRIMARY_VERTICAL_SLICE_SPEC),
        "flooded_quarry_night_watch" => Some(&FLOODED_QUARRY_VERTICAL_SLICE_SPEC),
        _ => None,
    }
}

pub fn list_vertical_slices() -> [&'static VerticalSliceSpec; 2] {
    [
        &PRIMARY_VERTICAL_SLICE_SPEC,
        &FLOODED_QUARRY_VERTICAL_SLICE_SPEC,
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        SliceKingdom, SliceResolutionPath, list_vertical_slices, primary_vertical_slice,
        vertical_slice_for_current_synthesis_scenario,
    };

    #[test]
    fn primary_slice_keeps_hueman_first_and_flynt_progression() {
        let slice = primary_vertical_slice();

        assert_eq!(slice.signature_resource, "Opal Oil");
        assert_eq!(slice.current_form, "Gremlin");
        assert!(slice.opening_location.contains("Aura Ridge"));
        assert_eq!(slice.loop_stages[3].kingdom, SliceKingdom::Flynt);
        assert_eq!(
            slice.default_resolution_path,
            SliceResolutionPath::RouteStabilization
        );
        assert_eq!(slice.resolution_options.len(), 2);
    }

    #[test]
    fn slice_catalog_maps_quarry_scenario() {
        let quarry = vertical_slice_for_current_synthesis_scenario("flooded_quarry_night_watch")
            .expect("quarry scenario should map to a slice");
        assert_eq!(quarry.current_form, "Goblin");
        assert!(
            list_vertical_slices()
                .iter()
                .any(|slice| slice.id == quarry.id)
        );
    }
}
