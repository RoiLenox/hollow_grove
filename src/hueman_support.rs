use std::fmt::Write as _;
use std::path::PathBuf;

pub const CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_base.md";
pub const CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_activation_gate.md";
pub const CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_operational.md";
pub const CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_contract.md";
pub const CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_sequence.md";
pub const CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_topology.md";
pub const CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_selection.md";
pub const CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_consequence.md";
pub const CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_execution_spec.md";
pub const CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_behavior_rules.md";
pub const CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_transition_pm_to_le.md";
pub const HUEMAN_BOUNDARY_ARTIFACT_PATH: &str = "artifacts/hueman_boundary.md";
pub const HUEMAN_FOURWAY_ARTIFACT_PATH: &str = "artifacts/hueman_fourway.md";
pub const HUEMAN_AURA_TRIAD_ARTIFACT_PATH: &str = "artifacts/hueman_aura_triad.md";
pub const HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH: &str = "artifacts/hueman_aura_behavior.md";
pub const HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH: &str = "artifacts/hueman_stonebend_roles.md";
pub const HUEMAN_TROSS_HELPERS_ARTIFACT_PATH: &str = "artifacts/hueman_tross_helpers.md";
pub const HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH: &str = "artifacts/hueman_glaushouse_roles.md";
pub const HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH: &str = "artifacts/hueman_sandmanor_roles.md";
pub const HUEMAN_PROCEDURAL_UPLIFT_ARTIFACT_PATH: &str = "artifacts/hueman_procedural_uplift.md";
pub const HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH: &str = "artifacts/hueman_archetype_lens.md";
pub const HUEMAN_START_PATHS_ARTIFACT_PATH: &str = "artifacts/hueman_start_paths.md";
pub const HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH: &str = "artifacts/hueman_path_crossovers.md";
pub const HUEMAN_LINK_PHYSICS_ARTIFACT_PATH: &str = "artifacts/hueman_link_physics.md";
pub const HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH: &str = "artifacts/hueman_inverse_circle.md";
pub const HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH: &str = "artifacts/hueman_crossover_scenes.md";
pub const HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH: &str = "artifacts/hueman_scene_presence.md";
pub const HUEMAN_SCENE_INTENT_ARTIFACT_PATH: &str = "artifacts/hueman_scene_intent.md";
pub const HUEMAN_SCENE_DRIFT_ARTIFACT_PATH: &str = "artifacts/hueman_scene_drift.md";
pub const VERTICAL_INTEGRATION_STACK_ARTIFACT_PATH: &str =
    "artifacts/vertical_integration_stack.md";
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

pub fn hueman_stonebend_roles_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH)
}

pub fn hueman_tross_helpers_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH)
}

pub fn hueman_glaushouse_roles_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH)
}

pub fn hueman_sandmanor_roles_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH)
}

pub fn hueman_procedural_uplift_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_PROCEDURAL_UPLIFT_ARTIFACT_PATH)
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

pub fn hueman_link_physics_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH)
}

pub fn hueman_inverse_circle_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH)
}

pub fn hueman_crossover_scenes_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH)
}

pub fn hueman_scene_presence_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH)
}

pub fn hueman_scene_intent_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_SCENE_INTENT_ARTIFACT_PATH)
}

pub fn hueman_scene_drift_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_SCENE_DRIFT_ARTIFACT_PATH)
}

pub fn vertical_integration_stack_artifact_path() -> PathBuf {
    PathBuf::from(VERTICAL_INTEGRATION_STACK_ARTIFACT_PATH)
}

pub fn hueman_start_choices_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_START_CHOICES_ARTIFACT_PATH)
}

#[derive(Clone, Copy)]
struct HuemanSceneDefinition {
    name: &'static str,
    presence: &'static str,
    intent: &'static str,
}

#[derive(Clone, Copy)]
struct HuemanSectionDefinition {
    title: &'static str,
    lines: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct HuemanRoleArtifactDefinition {
    title: &'static str,
    structural_rule: &'static str,
    sections: &'static [HuemanSectionDefinition],
    boundary_reminder: &'static str,
}

#[derive(Clone, Copy)]
struct HuemanAnchorDefinition {
    name: &'static str,
    direction: &'static str,
    archetype: &'static str,
    primary_scene: &'static str,
    start_path: [&'static str; 3],
    presence_lines: &'static [&'static str],
    intent_lines: &'static [&'static str],
    role_artifact: &'static HuemanRoleArtifactDefinition,
    lens_lines: &'static [&'static str],
    lens_overlay_title: &'static str,
    lens_overlay_lines: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct AuraRegionDefinition {
    name: &'static str,
    movement: &'static str,
    encounter: &'static str,
    world_description: &'static str,
}

const HUEMAN_SCENE_DEFINITIONS: [HuemanSceneDefinition; 4] = [
    HuemanSceneDefinition {
        name: "Seam Market",
        presence: "rumor carriers, salvage brokers, exchangers, temporary stalls, signal cloths",
        intent: "exchange, rumor flow, salvage circulation, temporary trust",
    },
    HuemanSceneDefinition {
        name: "Threshold Weather",
        presence: "drifters, lookouts, spray traces, warning markers, bright debris",
        intent: "warning, drift, exposure, onward movement",
    },
    HuemanSceneDefinition {
        name: "Pressure Shelter",
        presence: "keepers, hoarders, wardens, bundled stores, inward fires",
        intent: "concealment, storage, guarded warmth, selective admission",
    },
    HuemanSceneDefinition {
        name: "Split Trace",
        presence: "echoes, doubles, uncertain witnesses, partial camps, contradictory clues",
        intent: "witness, ambiguity, coexistence, half-open routing",
    },
];

const AURA_RIDGE_PRESENCE_LINES: &[&str] = &[
    "straight-ridge caravans, free traders, hinge stalls, and visible right-angle turn traffic",
    "public trade legs are declared from Stonebend -> Glaushouse and Glaushouse -> Sandmanor",
];

const AURA_RIDGE_INTENT_LINES: &[&str] = &[
    "keep free trade moving along the declared straight ridge legs without collapsing kingdom identity",
    "let Glaushouse serve as the visible turn where eastern and western trade pressure changes direction",
];

const GLAUSHOUSE_PRESENCE_LINES: &[&str] = &[
    "Prima Donna: scene-facing lead and public center of Glaushouse presence",
    "Persephone: assistant, relay, and step-down continuity beside the lead",
    "Nightengales: nurses and common people carrying the lived body of Glaushouse",
    "jades: polished green thresholds, court stone, and care tokens mined in the South",
    "Jadomer: Glaushouse's outward export carried as current refined through jade yield",
];

const GLAUSHOUSE_INTENT_LINES: &[&str] = &[
    "Prima Donna: tone-setting, command, final say",
    "Persephone: delegated execution, continuity, relay, and step-down succession",
    "Nightengales: care, stabilization, bedside recovery, and public body without command",
    "jades: turn beauty, care, and sovereignty into a held southern resource",
    "Jadomer: turn southern beauty and material care into Glaushouse's outward trade body",
];

const STONEBEND_PRESENCE_LINES: &[&str] = &[
    "Proliteriate: collective labor pressure and shared leverage",
    "Hypergiant: public-facing speaker without superior rank",
    "Freemason: built order, sealed works, and hidden structure",
    "Hypergiant may appear first, but triad power stays equal",
    "Geralds: the common people of Stonebend holding its public mass",
    "diamonds: mined civic wealth held under Stonebend's equal-power structure",
    "mercury mirror: Stonebend's outward export refined from hollow current and diamond yield",
];

const STONEBEND_INTENT_LINES: &[&str] = &[
    "Proliteriate: shared leverage, labor continuity, public weight",
    "Hypergiant: legible representation, negotiation, public continuity",
    "Freemason: durable structure, enclosed coordination, civic continuity",
    "Geralds: keep the city's common pressure visible beneath the triad",
    "diamonds: compress public value into durable civic leverage",
    "mercury mirror: refine hollow current and diamond value into Stonebend's main outward-facing export",
    "equal power keeps Stonebend intent braided instead of sovereign",
];

const FLYNT_PRESENCE_LINES: &[&str] = &[
    "Tross: Flynt-anchored line presence running North -> South",
    "Juvenile: north head pressure held at the Flynt-facing side of the line",
    "Delinquent: south guard pressure carried downline toward Glaushouse-facing scenes",
    "White Dwarfs: four close guards holding Tross's personal ring without taking the north or south posts",
    "Wardens: the people of Flynt holding the line body around Tross",
    "opals: mined gleam carried through Flynt's guarded northern line",
    "Opal Oil: Flynt's outward export carried as hollow current refined through opal yield",
    "Tross helpers do not outrank scene or civic roles",
];

const FLYNT_INTENT_LINES: &[&str] = &[
    "Tross: keep the Flynt-anchored line running North -> South without sovereign rank",
    "Juvenile: hold the North head, spot early motion, keep Flynt-facing approach alert",
    "Delinquent: hold the South end, deter breach, harden Glaushouse-facing thresholds",
    "White Dwarfs: maintain the close personal guard around Tross without replacing the directional helpers",
    "Wardens: hold the common line body of Flynt around the Tross duty",
    "opals: move guarded brightness along the line without exposing the whole body",
    "Opal Oil: turn hollow current and guarded opal yield into Flynt's outward trade pressure",
    "helpers keep line duty without becoming sovereign roles",
];

const SANDMANOR_PRESENCE_LINES: &[&str] = &[
    "Sandmen: the people of Sandmanor holding the shared social body beneath the contest",
    "Minoans: southern room-makers, interior singers, draped thresholds, tuned chambers",
    "Minorians: northern counters, ledger-keepers, tally boards, visible judges",
    "Sandmanite: Minoan winner carrying the Sandman office through design-crossed improvement",
    "Sandmanorian: Minorian winner carrying the Sandman office through accounting-crossed improvement",
    "crystals: mined facets and stewarded witness stock beneath the rivalry",
    "Crystoleum: Sandmanor's outward glass-sand export carried through current and crystal proof",
    "Sandmanor presence favors visible comparison instead of inherited fixed rank",
];

const SANDMANOR_INTENT_LINES: &[&str] = &[
    "Sandmen: carry the public witness that makes the contest socially binding",
    "Minoans: teach design as atmosphere, cadence, and room-song",
    "Minorians: teach accounting as measure, proof, and public count",
    "Sandmanite: take rule when a Minoan proves the strongest reciprocal improvement",
    "Sandmanorian: take rule when a Minorian proves the strongest reciprocal improvement",
    "crystals: expose stewardship, count, and designed atmosphere through visible mineral proof",
    "Crystoleum: turn stewarded crystal proof into Sandmanor's outward trade body",
    "Sandmanor intent keeps rivalry productive instead of purely destructive",
];

const INVERSE_CIRCLE_PRESENCE_LINES: &[&str] = &[
    "The Stairway to Heaven: concealed rise shafts, rung marks, hush traffic",
    "The Riptide: pull currents, drag marks, return pressure in the tunnel bends",
    "The Current Sea: underground flow chambers, counted channels, measured carry",
    "The Aura Way: soft-lit passages, atmospheric bleed, felt route pressure before sight",
];

const INVERSE_CIRCLE_INTENT_LINES: &[&str] = &[
    "The Stairway to Heaven: conceal ascent and reward those who can keep climbing in secret",
    "The Riptide: pull travelers backward through pressure and memory",
    "The Current Sea: measure, sustain, and carry hidden motion through the interior",
    "The Aura Way: saturate the tunnel route with felt atmosphere before visible event",
];

const GOBLIN_LENS_LINES: &[&str] = &[
    "Aura Basin reads as burrow, shelter, and kept stores",
    "Aura Fields reads as forage paths, routes, and workable ground",
    "Aura Beach reads as exposed salvage, tide risk, and thin cover",
];

const GREMLIN_LENS_LINES: &[&str] = &[
    "Aura Basin reads as stress seams, pressure joints, and hidden leverage",
    "Aura Fields reads as barter space, friction lines, and noisy crossings",
    "Aura Beach reads as scrap edge, discard flow, and threshold apparatus",
];

const GREMLIN_OVERLAY_LINES: &[&str] = &[
    "Stonebend carries Proliteriate, Hypergiant, and Freemason as an equal-power triad",
    "Hypergiant is the public face seen first from outside the structure",
    "gremlin reading notices equal leverage behind the public face rather than a single ruler",
    "civic order stays vertically integrated with the Fourway start instead of floating above it abstractly",
];

const PIXY_LENS_LINES: &[&str] = &[
    "Aura Basin reads as hush, glow, and suspended potential",
    "Aura Fields reads as shimmer, weather play, and visible drift",
    "Aura Beach reads as glint, spray, and bright dispersal",
];

const SPRITE_LENS_LINES: &[&str] = &[
    "Aura Basin reads as root echo, sleep, and soft enclosure",
    "Aura Fields reads as current, sway, and open circulation",
    "Aura Beach reads as horizon pull, release, and farward motion",
];

const SPRITE_OVERLAY_LINES: &[&str] = &[
    "Minoans make the sprite reading notice tuned interiors, cadence, and room-song pressure.",
    "Minorians make the sprite reading notice tallies, balance sheets, and public proof.",
    "the Sandman contest makes improvement itself visible as the basis of rule.",
    "Sandmanor keeps its canonical western place even when relational viewpoints read it from another side.",
];

const AURA_REGION_DEFINITIONS: [AuraRegionDefinition; 3] = [
    AuraRegionDefinition {
        name: "Aura Basin",
        movement: "movement reads as inward and narrowing",
        encounter: "encounter tone reads as close, muffled, and formative",
        world_description: "world description favors pressure, shelter, and accumulation",
    },
    AuraRegionDefinition {
        name: "Aura Fields",
        movement: "movement reads as lateral and exposed",
        encounter: "encounter tone reads as social, visible, and negotiable",
        world_description: "world description favors weather, distance, and traversal",
    },
    AuraRegionDefinition {
        name: "Aura Beach",
        movement: "movement reads as outward and threshold-facing",
        encounter: "encounter tone reads as reflective, sparse, and releasing",
        world_description: "world description favors edge, horizon, and departure",
    },
];

const STONEBEND_ROLE_SECTIONS: [HuemanSectionDefinition; 4] = [
    HuemanSectionDefinition {
        title: "Stonebend Power Triad",
        lines: &["Proliteriate", "Hypergiant", "Freemason"],
    },
    HuemanSectionDefinition {
        title: "Power Balance",
        lines: &[
            "Proliteriate, Hypergiant, and Freemason hold equal power inside Stonebend.",
            "Hypergiant is the public face of the triad, not a higher authority.",
            "public representation does not override equal internal standing.",
            "no single role may collapse the triad into a solo rule.",
        ],
    },
    HuemanSectionDefinition {
        title: "Vertical Integration",
        lines: &[
            "Stonebend remains the East-facing `gremlin` start on the Fourway.",
            "the civic triad is a Hueman/world governance layer attached to that start.",
            "Geralds are the people of Stonebend and carry the city's common civic body.",
            "Stonebend mines diamonds.",
            "Stonebend uses hollow current with diamond yield to produce mercury mirror as its main export.",
            "Current Synthesis remains the lower operating layer beneath this governance.",
            "Hollow Grove remains the recursive core beneath both.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "Stonebend roles are descriptive-only for now",
            "no command resolver or role AI is active",
            "no automatic power shifts are active",
            "no feedback into Current Synthesis",
            "no feedback into Hollow Grove",
        ],
    },
];

const TROSS_ROLE_SECTIONS: [HuemanSectionDefinition; 5] = [
    HuemanSectionDefinition {
        title: "Anchor",
        lines: &[
            "Tross is in Flynt.",
            "Flynt remains North = `goblin` on the Fourway roster.",
            "Wardens are the people of Flynt.",
            "Flynt mines opals.",
            "Flynt exports Opal Oil as its main outward trade good, formed from hollow current and opal yield.",
            "Tross runs North -> South rather than spanning the whole Fourway equally.",
        ],
    },
    HuemanSectionDefinition {
        title: "Helper Pair",
        lines: &["Delinquent", "Juvenile"],
    },
    HuemanSectionDefinition {
        title: "Personal Guard",
        lines: &[
            "The White Dwarfs are Tross's personal guard.",
            "there are four White Dwarfs.",
            "they keep close guard around Tross rather than taking directional posts from the helper pair.",
        ],
    },
    HuemanSectionDefinition {
        title: "North-South Guard",
        lines: &[
            "Juvenile guards North at the Flynt-facing head of the line.",
            "Delinquent guards South.",
            "South remains Glaushouse = `pixy` on the Fourway roster.",
            "helper duty runs down the line from Flynt instead of behaving like sovereign rule.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "Tross helpers are descriptive-only for now",
            "no helper AI or encounter resolver is active",
            "no automatic north or south event gate is active",
            "no feedback into Current Synthesis",
            "no feedback into Hollow Grove",
        ],
    },
];

const GLAUSHOUSE_ROLE_SECTIONS: [HuemanSectionDefinition; 4] = [
    HuemanSectionDefinition {
        title: "Canonical Anchor",
        lines: &[
            "Glaushouse remains the South-facing `pixy` start on the Fourway.",
            "Glaushouse holds the luminous southern threshold of the roster.",
            "Glaushouse mines jades.",
            "Glaushouse exports Jadomer as its main outward trade good, formed from current and jades.",
        ],
    },
    HuemanSectionDefinition {
        title: "Glaushouse Order",
        lines: &[
            "Prima Donna is the leader.",
            "Persephone is the assistant and step-down continuity.",
            "Nightengales are the nurses and the common people of Glaushouse.",
        ],
    },
    HuemanSectionDefinition {
        title: "Social Balance",
        lines: &[
            "Prima Donna sets tone, command, and public face.",
            "Persephone carries relay authority and may step down from the lead into continuity duty.",
            "Nightengales keep recovery, bedside care, and the lived body of the kingdom.",
            "leadership does not erase the people; the people remain visible through the Nightengales.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "Glaushouse roles are descriptive-only for now",
            "no court resolver or succession engine is active",
            "no nurse AI or care loop is active",
            "no feedback into Current Synthesis",
            "no feedback into Hollow Grove",
        ],
    },
];

const SANDMANOR_ROLE_SECTIONS: [HuemanSectionDefinition; 6] = [
    HuemanSectionDefinition {
        title: "Canonical Anchor",
        lines: &[
            "Sandmanor remains the West-facing `sprite` start on the Fourway.",
            "from Stonebend, Sandmanor sits on the far counter-arc.",
            "from Glaushouse, Sandmanor may read eastward across the relational arc without changing the canonical map.",
            "",
            "Sandmanor mines crystals.",
            "Sandmanor exports Crystoleum as its main outward trade good, formed from current and crystals.",
        ],
    },
    HuemanSectionDefinition {
        title: "Sandmanor Halves",
        lines: &[
            "Sandmen are the people of Sandmanor.",
            "Minoans hold the South.",
            "Minorians hold the North.",
        ],
    },
    HuemanSectionDefinition {
        title: "Native Crafts",
        lines: &[
            "Minoans design interiors, rooms, and atmospheres like a song.",
            "Minorians account, tally, and measure what Sandmanor can sustain.",
            "Minoans and Minorians are the rival houses inside the Sandmen.",
            "each side keeps its own people and its own craft pressure.",
        ],
    },
    HuemanSectionDefinition {
        title: "Rival Teaching Contract",
        lines: &[
            "a Minorian must teach a Minoan to account.",
            "a Minoan must teach a Minorian to design like a song.",
            "each rival has to improve at the other's native discipline rather than remain pure.",
        ],
    },
    HuemanSectionDefinition {
        title: "Sandman Rule",
        lines: &[
            "the crowd judges which rival is most improved by the opposing lesson.",
            "the office of rule is the Sandman.",
            "a Minoan winner is referred to as the Sandmanite.",
            "a Minorian winner is referred to as the Sandmanorian.",
            "the winning title-holder becomes ruler of Sandmanor until the contest turns again.",
            "rule is earned by witnessed improvement, not fixed inheritance.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "Sandmanor roles are descriptive-only for now",
            "no contest resolver or crowd AI is active",
            "no automatic succession cycle is active",
            "no feedback into Current Synthesis",
            "no feedback into Hollow Grove",
        ],
    },
];

const STONEBEND_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Stonebend Roles",
    structural_rule: "Stonebend carries a three-part civic power that belongs to Hueman's world layer and remains vertically integrated above Current Synthesis and Hollow Grove.",
    sections: &STONEBEND_ROLE_SECTIONS,
    boundary_reminder: "Stonebend roles belong to Hueman's civic layer. They do not replace HAL, Clouseau, or any Current Synthesis client boundary.",
};

const TROSS_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Tross Helpers",
    structural_rule: "Tross is anchored in Flynt and runs as a north-to-south helper line inside Hueman's world layer.",
    sections: &TROSS_ROLE_SECTIONS,
    boundary_reminder: "Tross helpers belong to Hueman's Flynt-anchored directional line. They do not replace Fourway placement, civic roles, or kernel ownership.",
};

const GLAUSHOUSE_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Glaushouse Roles",
    structural_rule: "Glaushouse carries a scene-facing court and care order inside Hueman's world layer, where leadership, succession, and nursing remain socially visible without leaving the Fourway boundary.",
    sections: &GLAUSHOUSE_ROLE_SECTIONS,
    boundary_reminder: "Glaushouse roles belong to Hueman's kingdom layer. They do not replace scene logic, procedural care systems, or any Current Synthesis client boundary.",
};

const SANDMANOR_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Sandmanor Roles",
    structural_rule: "Sandmanor carries a rival two-house rule inside Hueman's world layer, where northern accountancy and southern interior-song design compete through reciprocal teaching rather than inherited fixed sovereignty.",
    sections: &SANDMANOR_ROLE_SECTIONS,
    boundary_reminder: "Sandmanor roles belong to Hueman's kingdom layer. They do not replace Fourway placement, scene logic, or any Current Synthesis client boundary.",
};

const STONEBEND_PROCEDURE_LINES: &[&str] = &[
    "Proliteriate, Hypergiant, and Freemason enter any civic decision as an equal-power triad.",
    "Hypergiant may present first as the public face, but may not finalize alone.",
    "Geralds provide the witnessed public body that confirms a civic shift without taking triad power.",
    "diamond extraction stays inside Stonebend's civic balance instead of becoming a private sovereign right.",
    "hollow current combines with diamond yield into mercury mirror under the same civic balance.",
    "until activation changes, Stonebend procedure remains declared rather than executed.",
];

const FLYNT_PROCEDURE_LINES: &[&str] = &[
    "Tross holds the Flynt line from North -> South as the procedural spine.",
    "Juvenile checks the North head before Delinquent checks the South end.",
    "the four White Dwarfs keep close guard around Tross while Wardens hold the broader line body.",
    "opal extraction follows the guarded line body rather than an unbounded field claim.",
    "hollow current carries opal yield outward as Opal Oil without breaking the guarded line body.",
    "transition pressure may be read through Current Synthesis route order, but no autonomous traversal is enabled.",
];

const GLAUSHOUSE_PROCEDURE_LINES: &[&str] = &[
    "Prima Donna sets command tone and first issuance.",
    "Persephone relays or inherits continuity when command steps down.",
    "Nightengales run the care loop and stabilize the common body without taking sovereign lead.",
    "current combines with jade extraction into Jadomer without displacing Nightengales care duty.",
    "succession and care remain procedurally specified but still gated.",
];

const SANDMANOR_PROCEDURE_LINES: &[&str] = &[
    "selection identifies the rival public frame and consequence names the witnessed improvement result.",
    "a Minoan winner takes Sandmanite; a Minorian winner takes Sandmanorian.",
    "Sandmen bind the crowd witness that legitimizes the Sandman office.",
    "current combines with crystal extraction into Crystoleum as part of the stewarded public export both rival houses must carry.",
    "reciprocal teaching remains the basis of rule instead of inheritance.",
];

const PROCEDURAL_UPLIFT_SECTIONS: [HuemanSectionDefinition; 6] = [
    HuemanSectionDefinition {
        title: "Shared Contract",
        lines: &[
            "Current Synthesis still owns execution spec, behavior rules, transition rules, selection, consequence, and activation gating.",
            "Hueman consumes those lower contracts as kingdom-facing procedures.",
            "no uplifted procedure may mutate Hollow Grove or rewrite Current Synthesis ownership.",
        ],
    },
    HuemanSectionDefinition {
        title: "Stonebend Procedure",
        lines: STONEBEND_PROCEDURE_LINES,
    },
    HuemanSectionDefinition {
        title: "Flynt Procedure",
        lines: FLYNT_PROCEDURE_LINES,
    },
    HuemanSectionDefinition {
        title: "Glaushouse Procedure",
        lines: GLAUSHOUSE_PROCEDURE_LINES,
    },
    HuemanSectionDefinition {
        title: "Sandmanor Procedure",
        lines: SANDMANOR_PROCEDURE_LINES,
    },
    HuemanSectionDefinition {
        title: "Activation Status",
        lines: &[
            "procedural uplift is defined",
            "Current Synthesis activation still denies live execution",
            "no autonomous NPC state, contest loop, care loop, or guard traversal is active",
            "no feedback into Current Synthesis",
            "no feedback into Hollow Grove",
        ],
    },
];

const HUEMAN_WORLD_ANCHORS: [HuemanAnchorDefinition; 4] = [
    HuemanAnchorDefinition {
        name: "Flynt",
        direction: "North",
        archetype: "goblin",
        primary_scene: "Pressure Shelter",
        start_path: ["Aura Basin", "Aura Fields", "Aura Beach"],
        presence_lines: FLYNT_PRESENCE_LINES,
        intent_lines: FLYNT_INTENT_LINES,
        role_artifact: &TROSS_ROLE_ARTIFACT,
        lens_lines: GOBLIN_LENS_LINES,
        lens_overlay_title: "",
        lens_overlay_lines: &[],
    },
    HuemanAnchorDefinition {
        name: "Stonebend",
        direction: "East",
        archetype: "gremlin",
        primary_scene: "Seam Market",
        start_path: ["Aura Fields", "Aura Basin", "Aura Beach"],
        presence_lines: STONEBEND_PRESENCE_LINES,
        intent_lines: STONEBEND_INTENT_LINES,
        role_artifact: &STONEBEND_ROLE_ARTIFACT,
        lens_lines: GREMLIN_LENS_LINES,
        lens_overlay_title: "Stonebend Civic Reading",
        lens_overlay_lines: GREMLIN_OVERLAY_LINES,
    },
    HuemanAnchorDefinition {
        name: "Glaushouse",
        direction: "South",
        archetype: "pixy",
        primary_scene: "Threshold Weather",
        start_path: ["Aura Beach", "Aura Fields", "Aura Basin"],
        presence_lines: GLAUSHOUSE_PRESENCE_LINES,
        intent_lines: GLAUSHOUSE_INTENT_LINES,
        role_artifact: &GLAUSHOUSE_ROLE_ARTIFACT,
        lens_lines: PIXY_LENS_LINES,
        lens_overlay_title: "",
        lens_overlay_lines: &[],
    },
    HuemanAnchorDefinition {
        name: "Sandmanor",
        direction: "West",
        archetype: "sprite",
        primary_scene: "Split Trace",
        start_path: ["Aura Beach", "Aura Basin", "Aura Fields"],
        presence_lines: SANDMANOR_PRESENCE_LINES,
        intent_lines: SANDMANOR_INTENT_LINES,
        role_artifact: &SANDMANOR_ROLE_ARTIFACT,
        lens_lines: SPRITE_LENS_LINES,
        lens_overlay_title: "Sandmanor Competitive Reading",
        lens_overlay_lines: SPRITE_OVERLAY_LINES,
    },
];

fn render_bullet_lines(lines: &[&str]) -> String {
    let mut output = String::new();
    for line in lines {
        if line.is_empty() {
            output.push('\n');
            continue;
        }
        output.push_str("- ");
        output.push_str(line);
        output.push('\n');
    }
    output
}

fn render_named_sections(sections: &[HuemanSectionDefinition]) -> String {
    let mut output = String::new();
    for section in sections {
        let _ = writeln!(output, "## {}\n", section.title);
        output.push_str(&render_bullet_lines(section.lines));
        output.push('\n');
    }
    output
}

fn build_hueman_role_artifact(
    artifact: &HuemanRoleArtifactDefinition,
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    format!(
        "# {}\n\n\
         ## Structural Rule\n\n\
         {}\n\n\
         {}\
         ## Artifact Inputs\n\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Fourway bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         {}\n",
        artifact.title,
        artifact.structural_rule,
        render_named_sections(artifact.sections),
        hueman_start_choices.len(),
        hueman_fourway.len(),
        artifact.boundary_reminder
    )
}

fn render_scene_presence_map() -> String {
    let mut output = String::new();
    for scene in HUEMAN_SCENE_DEFINITIONS {
        let _ = writeln!(output, "- {}: {}", scene.name, scene.presence);
    }
    output
}

fn render_scene_intent_map() -> String {
    let mut output = String::new();
    for scene in HUEMAN_SCENE_DEFINITIONS {
        let _ = writeln!(output, "- {}: {}", scene.name, scene.intent);
    }
    output
}

fn render_archetype_pull_map() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(
            output,
            "- `{}` -> {}",
            anchor.archetype, anchor.primary_scene
        );
    }
    output
}

fn render_fourway_roster() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(
            output,
            "- {} = {} = `{}`",
            anchor.direction, anchor.name, anchor.archetype
        );
    }
    output
}

fn render_archetype_list() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(output, "- `{}`", anchor.archetype);
    }
    output
}

fn render_starting_places() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(output, "- {}", anchor.name);
    }
    output
}

fn render_initial_start_roster() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(output, "- `{}` starts in {}", anchor.archetype, anchor.name);
    }
    output
}

fn render_start_path_order() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(
            output,
            "- {} = `{}` = {} -> {} -> {}",
            anchor.name,
            anchor.archetype,
            anchor.start_path[0],
            anchor.start_path[1],
            anchor.start_path[2]
        );
    }
    output
}

fn render_start_path_first_entry() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(
            output,
            "- {} enters {} first.",
            anchor.name, anchor.start_path[0]
        );
    }
    output
}

fn render_aura_region_states() -> String {
    let mut output = String::new();
    for region in AURA_REGION_DEFINITIONS {
        let _ = writeln!(output, "### {}\n", region.name);
        let _ = writeln!(output, "- {}", region.movement);
        let _ = writeln!(output, "- {}", region.encounter);
        let _ = writeln!(output, "- {}\n", region.world_description);
    }
    output
}

fn render_archetype_lens_sections() -> String {
    let mut output = String::new();
    for anchor in HUEMAN_WORLD_ANCHORS {
        let _ = writeln!(output, "### `{}`\n", anchor.archetype);
        output.push_str(&render_bullet_lines(anchor.lens_lines));
        output.push('\n');

        if !anchor.lens_overlay_title.is_empty() {
            let _ = writeln!(output, "## {}\n", anchor.lens_overlay_title);
            output.push_str(&render_bullet_lines(anchor.lens_overlay_lines));
            output.push('\n');
        }
    }
    output
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
         ## Declared World Surface\n\n\
         - Flynt, Stonebend, Glaushouse, and Sandmanor are declared as Hueman-facing world anchors.\n\
         - civic roles, helper lines, kingdom roles, scene reading, and procedural uplift may be described above Current Synthesis.\n\
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
         {}\
         \n\
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
        render_fourway_roster(),
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
         {}\
         \n\
         ## Starting Places\n\n\
         {}\
         \n\
         ## Fourway Placement\n\n\
         {}\
         \n\
         ## Initial Start Roster\n\n\
         {}\
         \n\
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
        render_archetype_list(),
        render_starting_places(),
        render_fourway_roster(),
        render_initial_start_roster(),
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
         {}\
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
        render_aura_region_states(),
        hueman_aura_triad.len(),
        hueman_start_choices.len()
    )
}

pub fn build_hueman_stonebend_roles_from_artifacts(
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[1].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    )
}

pub fn build_hueman_tross_helpers_from_artifacts(
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[0].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    )
}

pub fn build_hueman_glaushouse_roles_from_artifacts(
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[2].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    )
}

pub fn build_hueman_sandmanor_roles_from_artifacts(
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[3].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    )
}

pub fn build_hueman_procedural_uplift_from_artifacts(
    current_synthesis_execution_spec: &str,
    current_synthesis_behavior_rules: &str,
    current_synthesis_transition_pm_to_le: &str,
    current_synthesis_selection: &str,
    current_synthesis_consequence: &str,
    current_synthesis_activation_gate: &str,
    hueman_stonebend_roles: &str,
    hueman_tross_helpers: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
) -> String {
    format!(
        "# Hueman Procedural Uplift\n\n\
         ## Structural Rule\n\n\
         Hueman may lift procedural contracts from Current Synthesis into world-facing behavior surfaces without moving lower-layer ownership upward.\n\n\
         {}\
         ## Artifact Inputs\n\n\
         Current Synthesis execution spec bytes: {}.\n\
         Current Synthesis behavior rules bytes: {}.\n\
         Current Synthesis transition bytes: {}.\n\
         Current Synthesis selection bytes: {}.\n\
         Current Synthesis consequence bytes: {}.\n\
         Current Synthesis activation gate bytes: {}.\n\
         Hueman Stonebend Roles bytes: {}.\n\
         Hueman Tross Helpers bytes: {}.\n\
         Hueman Glaushouse Roles bytes: {}.\n\
         Hueman Sandmanor Roles bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Procedural uplift makes Hueman ready to consume lower-layer procedures. It does not activate those procedures or grant Hueman authority over Current Synthesis.\n",
        render_named_sections(&PROCEDURAL_UPLIFT_SECTIONS),
        current_synthesis_execution_spec.len(),
        current_synthesis_behavior_rules.len(),
        current_synthesis_transition_pm_to_le.len(),
        current_synthesis_selection.len(),
        current_synthesis_consequence.len(),
        current_synthesis_activation_gate.len(),
        hueman_stonebend_roles.len(),
        hueman_tross_helpers.len(),
        hueman_glaushouse_roles.len(),
        hueman_sandmanor_roles.len()
    )
}

pub fn build_hueman_archetype_lens_from_artifacts(
    hueman_start_choices: &str,
    hueman_aura_behavior: &str,
    hueman_stonebend_roles: &str,
    hueman_sandmanor_roles: &str,
) -> String {
    format!(
        "# Hueman Archetype Lens\n\n\
         ## Structural Rule\n\n\
         After start choice and AuraTriad behavior are declared, each archetype reads the same regions through a different descriptive lens.\n\n\
         ## Archetype Readings\n\n\
         {}\
         ## Status\n\n\
         - archetype lens is descriptive-only for now\n\
         - no procedural bonuses or penalties are active\n\
         - the Fourway start roster remains unchanged\n\
         - AuraTriad region behavior remains shared underneath the lens\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Aura Behavior bytes: {}.\n\
         Hueman Stonebend Roles bytes: {}.\n\
         Hueman Sandmanor Roles bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         The archetype lens changes interpretation, not rules. It is a Hueman-facing difference in reading the world after placement.\n",
        render_archetype_lens_sections(),
        hueman_start_choices.len(),
        hueman_aura_behavior.len(),
        hueman_stonebend_roles.len(),
        hueman_sandmanor_roles.len()
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
         {}\
         \n\
         ## First Entry\n\n\
         {}\
         \n\
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
        render_start_path_order(),
        render_start_path_first_entry(),
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
         ## Aura Ridge Trade Legs\n\n\
         - free trade follows the straight Aura Ridge rather than the underground inverse circle.\n\
         - Stonebend and Glaushouse hold a declared straight trade leg along the ridge.\n\
         - Glaushouse and Sandmanor hold a declared straight trade leg along the ridge.\n\
         - Glaushouse acts as the visible hinge where the right-angle trade body turns.\n\n\
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

pub fn build_hueman_link_physics_from_artifacts(
    current_synthesis_sequence: &str,
    hueman_path_crossovers: &str,
) -> String {
    format!(
        "# Hueman Link Physics\n\n\
         ## Structural Rule\n\n\
         Links that do not get bonded may later resolve into `current` or `aura` according to downstream physics.\n\n\
         ## Bond Split\n\n\
         - bonded link stays the selected route\n\
         - unbonded links remain available as unresolved world material\n\
         - unresolved material is not empty; it carries later directional bias\n\n\
         ## Arm Weight Reading\n\n\
         - each `META` letter and its `PLEB` counterpart carry three arms across the same joint\n\
         - one arm per side bonds into the selected link while the remaining arm weight stays unresolved\n\
         - retained heavier continuity pressure tends to read as `current`\n\
         - lighter exposed spill tends to read as `aura`\n\
         - Hueman reads that unresolved weight upward as `current` or `aura` while Hollow Grove keeps the same event as the lower witness simultaneously\n\
         - simultaneous reading does not grant Hueman authority to rewrite the kernel witness\n\n\
         ## Current Bias Physics\n\n\
         - continuity pressure favors `current`\n\
         - occupancy load favors `current`\n\
         - inland persistence favors `current`\n\
         - repeat traversal favors `current`\n\n\
         ## Aura Bias Physics\n\n\
         - exposure pressure favors `aura`\n\
         - threshold bleed favors `aura`\n\
         - atmospheric spill favors `aura`\n\
         - edge drift favors `aura`\n\n\
         ## Element Names\n\n\
         - `current` may also be called Bathos or dark water.\n\
         - `current` appears as dark current or hollow current.\n\
         - `aura` may also be called Aether or air.\n\
         - `aura` appears as reflective aura or holographic aura.\n\n\
         ## Crossover Reading\n\n\
         - shared starts can touch the same unresolved material with different bias\n\
         - the same region may feel more `current` from one route and more `aura` from another\n\
         - crossover zones are where the physics split becomes most visible in Hueman\n\
         - Aura Ridge trade legs keep exchange visible on straight lines while unresolved bias still moves beneath them\n\n\
         ## Status\n\n\
         - link physics is descriptive-only for now\n\
         - no procedural resolver chooses `current` or `aura` yet\n\
         - bond selection remains kernel-simple underneath this layer\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis sequence bytes: {}.\n\
         Hueman Path Crossovers bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Link physics explains how unbonded links may later read as `current` or `aura`. It does not rewrite Bond, HollowGrove, or Current Synthesis sequence ownership.\n",
        current_synthesis_sequence.len(),
        hueman_path_crossovers.len()
    )
}

pub fn build_hueman_inverse_circle_from_artifacts(
    hueman_fourway: &str,
    hueman_link_physics: &str,
) -> String {
    format!(
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
         Hueman Fourway bytes: {}.\n\
         Hueman Link Physics bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         The inverse circle is an underground interior mirror path. It does not overwrite the visible world map, Current Synthesis geography, or Hollow Grove recursion.\n",
        hueman_fourway.len(),
        hueman_link_physics.len()
    )
}

pub fn build_hueman_crossover_scenes_from_artifacts(
    hueman_path_crossovers: &str,
    hueman_link_physics: &str,
) -> String {
    format!(
        "# Hueman Crossover Scenes\n\n\
         ## Structural Rule\n\n\
         When `current`-biased and `aura`-biased unresolved links appear at the same crossover, the world produces a named descriptive scene type.\n\n\
         ## Scene Types\n\n\
         ### Seam Market\n\n\
         - appears where `current` continuity and `aura` spill remain in balance\n\
         - feels like trade, rumor, salvage, and temporary arrangement\n\
         - fits shared Aura Fields crossings best\n\
         - commonly appears along the Stonebend -> Glaushouse Aura Ridge leg\n\n\
         ### Threshold Weather\n\n\
         - appears where `aura` exposure outruns `current` continuity\n\
         - feels like spray, drift, shimmer, and unstable edges\n\
         - fits shared Aura Beach crossings best\n\n\
         ### Pressure Shelter\n\n\
         - appears where `current` persistence contains `aura` residue\n\
         - feels like storage, burrow heat, muffled exchange, and held tension\n\
         - fits shared Aura Basin crossings best\n\n\
         ### Split Trace\n\n\
         - appears where both biases are present but neither settles the scene\n\
         - feels like afterimage, contradictory clues, and route ambiguity\n\
         - fits delayed or secondary crossovers after different openings\n\n\
         ## Placement\n\n\
         - Aura Beach tends toward Threshold Weather first.\n\
         - Aura Fields tends toward Seam Market first.\n\
         - Aura Basin tends toward Pressure Shelter first.\n\
         - Split Trace can appear in any crossover zone where the bias remains unresolved.\n\n\
         ## Status\n\n\
         - crossover scenes are descriptive-only for now\n\
         - no encounter tables or event resolvers are active\n\
         - link physics remains the upstream explanation for the scene type\n\
         - path crossovers remain the upstream overlap map\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Path Crossovers bytes: {}.\n\
         Hueman Link Physics bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Crossover scenes name what a shared biased overlap feels like. They do not create procedural meetings, rewards, or movement rules.\n",
        hueman_path_crossovers.len(),
        hueman_link_physics.len()
    )
}

pub fn build_hueman_scene_presence_from_artifacts(
    hueman_crossover_scenes: &str,
    hueman_archetype_lens: &str,
    hueman_stonebend_roles: &str,
    hueman_tross_helpers: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
    hueman_inverse_circle: &str,
) -> String {
    let presence_map = render_scene_presence_map();
    let archetype_pull = render_archetype_pull_map();
    let aura_ridge_presence = render_bullet_lines(AURA_RIDGE_PRESENCE_LINES);
    let stonebend_presence = render_bullet_lines(HUEMAN_WORLD_ANCHORS[1].presence_lines);
    let flynt_presence = render_bullet_lines(HUEMAN_WORLD_ANCHORS[0].presence_lines);
    let glaushouse_presence = render_bullet_lines(HUEMAN_WORLD_ANCHORS[2].presence_lines);
    let sandmanor_presence = render_bullet_lines(HUEMAN_WORLD_ANCHORS[3].presence_lines);
    let inverse_circle_presence = render_bullet_lines(INVERSE_CIRCLE_PRESENCE_LINES);

    format!(
        "# Hueman Scene Presence\n\n\
         ## Structural Rule\n\n\
         Each crossover scene carries a characteristic kind of presence before any encounter mechanics exist.\n\n\
         ## Presence Map\n\n\
         {}\
         \n\
         ## Archetype Pull\n\n\
         {}\
         \n\
         ## Aura Ridge Presence\n\n\
         {}\
         \n\
         ## Glaushouse Presence\n\n\
         {}\
         \n\
         ## Stonebend Civic Presence\n\n\
         {}\
         \n\
         ## Tross Helper Presence\n\n\
         {}\
         \n\
         ## Sandmanor Competitive Presence\n\n\
         {}\
         \n\
         ## Inverse Circle Presence\n\n\
         {}\
         \n\
         ## Status\n\n\
         - scene presence is descriptive-only for now\n\
         - no NPC system or occupancy resolver is active\n\
         - scene typing, archetype pull, civic overlay, helper lines, Sandmanor rivalry, and the inverse circle remain upstream only\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Crossover Scenes bytes: {}.\n\
         Hueman Archetype Lens bytes: {}.\n\
         Hueman Stonebend Roles bytes: {}.\n\
         Hueman Tross Helpers bytes: {}.\n\
         Hueman Glaushouse Roles bytes: {}.\n\
         Hueman Sandmanor Roles bytes: {}.\n\
         Hueman Inverse Circle bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Scene presence says what kind of occupant or trace belongs in a scene. It does not create procedural actors, dialogue, or rewards.\n",
        presence_map,
        archetype_pull,
        aura_ridge_presence,
        glaushouse_presence,
        stonebend_presence,
        flynt_presence,
        sandmanor_presence,
        inverse_circle_presence,
        hueman_crossover_scenes.len(),
        hueman_archetype_lens.len(),
        hueman_stonebend_roles.len(),
        hueman_tross_helpers.len(),
        hueman_glaushouse_roles.len(),
        hueman_sandmanor_roles.len(),
        hueman_inverse_circle.len()
    )
}

pub fn build_hueman_scene_intent_from_artifacts(
    hueman_scene_presence: &str,
    hueman_link_physics: &str,
    current_synthesis_contract: &str,
    hueman_stonebend_roles: &str,
    hueman_tross_helpers: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
    hueman_inverse_circle: &str,
) -> String {
    let mut output = String::with_capacity(5_600);
    let intent_map = render_scene_intent_map();
    let aura_ridge_intent = render_bullet_lines(AURA_RIDGE_INTENT_LINES);
    let stonebend_intent = render_bullet_lines(HUEMAN_WORLD_ANCHORS[1].intent_lines);
    let flynt_intent = render_bullet_lines(HUEMAN_WORLD_ANCHORS[0].intent_lines);
    let glaushouse_intent = render_bullet_lines(HUEMAN_WORLD_ANCHORS[2].intent_lines);
    let sandmanor_intent = render_bullet_lines(HUEMAN_WORLD_ANCHORS[3].intent_lines);
    let inverse_circle_intent = render_bullet_lines(INVERSE_CIRCLE_INTENT_LINES);

    output.push_str(
        &format!(
            "# Hueman Scene Intent\n\n\
         ## Structural Rule\n\n\
         Each scene presence carries a dominant descriptive intent before any encounter or dialogue system exists.\n\n\
         ## Intent Map\n\n\
         {}\
         \n\
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
         {}\
         \n\
         ## Glaushouse Intent\n\n\
         {}\
         \n\
         ## Stonebend Civic Intent\n\n\
         {}\
         \n\
         ## Tross Helper Intent\n\n\
         {}\
         \n\
         ## Sandmanor Competitive Intent\n\n\
         {}\
         \n\
         ## Inverse Circle Intent\n\n\
         {}\
         \n\
         ## Status\n\n\
         - scene intent is descriptive-only for now\n\
         - no AI, NPC, or quest logic is active\n\
         - scene presence, link physics, civic roles, helper lines, Sandmanor rivalry, and the inverse circle remain upstream only\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         ",
            intent_map,
            aura_ridge_intent,
            glaushouse_intent,
            stonebend_intent,
            flynt_intent,
            sandmanor_intent,
            inverse_circle_intent,
        ),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Scene Presence bytes",
        hueman_scene_presence.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Link Physics bytes",
        hueman_link_physics.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis Contract bytes",
        current_synthesis_contract.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Stonebend Roles bytes",
        hueman_stonebend_roles.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Tross Helpers bytes",
        hueman_tross_helpers.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Glaushouse Roles bytes",
        hueman_glaushouse_roles.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Sandmanor Roles bytes",
        hueman_sandmanor_roles.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Inverse Circle bytes",
        hueman_inverse_circle.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Scene intent says what a scene is trying to do atmospherically. It does not create tasks, dialogue trees, or procedural outcomes.\n",
    );
    output
}

pub fn build_hueman_scene_drift_from_artifacts(
    hueman_scene_intent: &str,
    hueman_link_physics: &str,
) -> String {
    let mut output = String::with_capacity(2_100);
    output.push_str(
        "# Hueman Scene Drift\n\n\
         ## Structural Rule\n\n\
         Scene intent can drift into a different scene type when bias pressure persists over time without a full system resolving it.\n\n\
         ## Drift Vectors\n\n\
         ### Seam Market\n\n\
         - drifts toward Pressure Shelter when exchange slows and stored continuity takes over\n\
         - drifts toward Threshold Weather when structures fail and exposure outruns arrangement\n\n\
         ### Threshold Weather\n\n\
         - drifts toward Split Trace when warning persists without settlement\n\
         - drifts toward Seam Market when repeated crossings stabilize the edge into exchange\n\n\
         ### Pressure Shelter\n\n\
         - drifts toward Seam Market when guarded stores reopen into circulation\n\
         - drifts toward Split Trace when shelter empties and only residue remains\n\n\
         ### Split Trace\n\n\
         - drifts toward Threshold Weather when ambiguity spills outward into exposure\n\
         - drifts toward Pressure Shelter when traces are hoarded, muffled, or enclosed\n\n\
         ## Drift Drivers\n\n\
         - sustained `current` accumulation pulls scenes toward storage, continuity, and reopened exchange\n\
         - sustained `aura` accumulation pulls scenes toward exposure, drift, shimmer, and unstable edges\n\
         - mixed unresolved pressure preserves Split Trace longer instead of forcing a clean resolution\n\
         - repeated crossings can stabilize a scene back into exchange even after warning or ambiguity\n\n\
         ## Status\n\n\
         - scene drift is descriptive-only for now\n\
         - no time simulation or procedural resolver is active\n\
         - scene intent remains the upstream atmospheric layer\n\
         - link physics remains the upstream bias layer\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         ",
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Scene Intent bytes",
        hueman_scene_intent.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Link Physics bytes",
        hueman_link_physics.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Scene drift says how a scene may change if its pressure persists. It does not activate clocks, AI routines, or procedural world updates.\n",
    );
    output
}

fn push_artifact_input_line(output: &mut String, label: &str, byte_len: usize) {
    output.push_str(label);
    output.push_str(": ");
    output.push_str(&byte_len.to_string());
    output.push_str(".\n");
}

pub fn build_vertical_integration_stack_from_artifacts(
    current_synthesis_base: &str,
    hueman_boundary: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
    hueman_inverse_circle: &str,
    hueman_procedural_uplift: &str,
    hueman_scene_presence: &str,
    hueman_scene_intent: &str,
    hueman_scene_drift: &str,
) -> String {
    let mut output = String::with_capacity(3_400);
    output.push_str(
        "# Vertical Integration Stack\n\n\
         ## Structural Rule\n\n\
         Hollow Grove remains the recursive core, KernelPass witnesses that core, Current Synthesis consumes the witnessed artifact layer as the operating layer, and Hueman consumes Current Synthesis as the world layer above it.\n\n\
         ## Full Stack\n\n\
         ```text\n\
         Symptom 1\n\
         ↓\n\
         Triway\n\
         ↓\n\
         HollowGrove\n\
         ↓\n\
         GroveSeam\n\
         ↓\n\
         HollowBeam\n\
         ↓\n\
         landed Symptom 2\n\
         ↓\n\
         KernelPass\n\
         ↓\n\
         Client Artifacts\n\
         ↓\n\
         Current Synthesis\n\
         ↓\n\
         Hueman\n\
         ↓\n\
         World Roles and Scenes\n\
         ```\n\n\
         ## Layer Ownership\n\n\
         - kernel owns recursion, bond selection, and the canonical witness\n\
         - client artifacts mirror the witness without mutating kernel state\n\
         - Current Synthesis owns route semantics, activation gating, and client boundaries like HAL and Clouseau\n\
         - Hueman owns Fourway, AuraTriad reading, start placement, civic roles, helper lines, and scene reading\n\n\
         ## Current Alignment\n\n\
         - Stonebend remains East = `gremlin` with an equal-power civic triad above Current Synthesis\n\
         - Tross remains anchored in Flynt and runs as a North -> South helper line with four White Dwarfs as the personal guard\n\
         - Glaushouse remains South = `pixy` with Prima Donna over Persephone and Nightengales as the common people\n\
         - Sandmanor remains West = `sprite` with southern Minoans, northern Minorians, and a crowd-judged Sandman contest\n\
         - resource seams are designated across Hueman: Stonebend diamonds, Flynt opals, Glaushouse jades, Sandmanor crystals\n\
         - `current` reads as Bathos or dark water while `aura` reads as Aether or air at the Hueman layer\n\
         - the bedrock split remains active upstream: dark current and hollow current, reflective aura and holographic aura\n\
         - the visible free-trade body follows the Aura Ridge straight legs Stonebend -> Glaushouse and Glaushouse -> Sandmanor\n\
         - Stonebend declares Mercury Mirror from hollow current + diamonds, Flynt declares Opal Oil from hollow current + opal, Glaushouse declares Jadomer from current + jades, and Sandmanor declares Crystoleum from current + crystals\n\
         - the inverse circle remains an underground interior tunnel ring: Stairway to Heaven, Riptide, Current Sea, Aura Way\n\
         - procedural uplift now maps Current Synthesis execution contracts into Hueman-facing kingdom procedures without moving ownership upward\n\
         - scene presence, scene intent, and scene drift remain the top descriptive Hueman layer\n\
         - upper layers consume lower layers without rewriting lower ownership\n\n\
         ## Boundary Contract\n\n\
         - Hollow Grove does not know Current Synthesis exists\n\
         - Current Synthesis does not know Hueman exists\n\
         - Hueman consumes Current Synthesis without feeding back into it\n\
         - runtime and benchmark follow the same downstream route when regenerating artifacts\n\n\
         ## Artifact Inputs\n\n\
         ",
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis Base bytes",
        current_synthesis_base.len(),
    );
    push_artifact_input_line(&mut output, "Hueman Boundary bytes", hueman_boundary.len());
    push_artifact_input_line(
        &mut output,
        "Hueman Glaushouse Roles bytes",
        hueman_glaushouse_roles.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Sandmanor Roles bytes",
        hueman_sandmanor_roles.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Inverse Circle bytes",
        hueman_inverse_circle.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Procedural Uplift bytes",
        hueman_procedural_uplift.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Scene Presence bytes",
        hueman_scene_presence.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Scene Intent bytes",
        hueman_scene_intent.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Scene Drift bytes",
        hueman_scene_drift.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         This stack artifact documents vertical alignment only. It does not grant any upper layer authority to mutate the kernel or bypass existing layer boundaries.\n",
    );
    output
}

#[cfg(test)]
mod tests {
    use super::{
        build_hueman_archetype_lens_from_artifacts, build_hueman_aura_behavior_from_artifacts,
        build_hueman_aura_triad_from_artifacts, build_hueman_boundary_from_artifacts,
        build_hueman_crossover_scenes_from_artifacts, build_hueman_fourway_from_artifacts,
        build_hueman_glaushouse_roles_from_artifacts, build_hueman_inverse_circle_from_artifacts,
        build_hueman_link_physics_from_artifacts, build_hueman_motion_map_from_artifacts,
        build_hueman_path_crossovers_from_artifacts, build_hueman_procedural_uplift_from_artifacts,
        build_hueman_sandmanor_roles_from_artifacts, build_hueman_scene_drift_from_artifacts,
        build_hueman_scene_intent_from_artifacts, build_hueman_scene_presence_from_artifacts,
        build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
        build_hueman_stonebend_roles_from_artifacts, build_hueman_tross_helpers_from_artifacts,
        build_vertical_integration_stack_from_artifacts,
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
             ## Declared World Surface\n\n\
             - Flynt, Stonebend, Glaushouse, and Sandmanor are declared as Hueman-facing world anchors.\n\
             - civic roles, helper lines, kingdom roles, scene reading, and procedural uplift may be described above Current Synthesis.\n\
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
    fn hueman_stonebend_roles_builder_is_deterministic() {
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
    fn hueman_tross_helpers_builder_is_deterministic() {
        assert_eq!(
            build_hueman_tross_helpers_from_artifacts("start", "fourway"),
            "# Hueman Tross Helpers\n\n\
             ## Structural Rule\n\n\
             Tross is anchored in Flynt and runs as a north-to-south helper line inside Hueman's world layer.\n\n\
             ## Anchor\n\n\
             - Tross is in Flynt.\n\
         - Flynt remains North = `goblin` on the Fourway roster.\n\
         - Wardens are the people of Flynt.\n\
         - Flynt mines opals.\n\
         - Flynt exports Opal Oil as its main outward trade good, formed from hollow current and opal yield.\n\
         - Tross runs North -> South rather than spanning the whole Fourway equally.\n\n\
             ## Helper Pair\n\n\
             - Delinquent\n\
             - Juvenile\n\n\
             ## Personal Guard\n\n\
             - The White Dwarfs are Tross's personal guard.\n\
             - there are four White Dwarfs.\n\
             - they keep close guard around Tross rather than taking directional posts from the helper pair.\n\n\
             ## North-South Guard\n\n\
             - Juvenile guards North at the Flynt-facing head of the line.\n\
             - Delinquent guards South.\n\
             - South remains Glaushouse = `pixy` on the Fourway roster.\n\
             - helper duty runs down the line from Flynt instead of behaving like sovereign rule.\n\n\
             ## Status\n\n\
             - Tross helpers are descriptive-only for now\n\
             - no helper AI or encounter resolver is active\n\
             - no automatic north or south event gate is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Tross helpers belong to Hueman's Flynt-anchored directional line. They do not replace Fourway placement, civic roles, or kernel ownership.\n"
        );
    }

    #[test]
    fn hueman_glaushouse_roles_builder_is_deterministic() {
        assert_eq!(
            build_hueman_glaushouse_roles_from_artifacts("start", "fourway"),
            "# Hueman Glaushouse Roles\n\n\
             ## Structural Rule\n\n\
             Glaushouse carries a scene-facing court and care order inside Hueman's world layer, where leadership, succession, and nursing remain socially visible without leaving the Fourway boundary.\n\n\
             ## Canonical Anchor\n\n\
             - Glaushouse remains the South-facing `pixy` start on the Fourway.\n\
             - Glaushouse holds the luminous southern threshold of the roster.\n\
             - Glaushouse mines jades.\n\
             - Glaushouse exports Jadomer as its main outward trade good, formed from current and jades.\n\
\n\
             ## Glaushouse Order\n\n\
             - Prima Donna is the leader.\n\
             - Persephone is the assistant and step-down continuity.\n\
             - Nightengales are the nurses and the common people of Glaushouse.\n\
\n\
             ## Social Balance\n\n\
             - Prima Donna sets tone, command, and public face.\n\
             - Persephone carries relay authority and may step down from the lead into continuity duty.\n\
             - Nightengales keep recovery, bedside care, and the lived body of the kingdom.\n\
             - leadership does not erase the people; the people remain visible through the Nightengales.\n\n\
             ## Status\n\n\
             - Glaushouse roles are descriptive-only for now\n\
             - no court resolver or succession engine is active\n\
             - no nurse AI or care loop is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Glaushouse roles belong to Hueman's kingdom layer. They do not replace scene logic, procedural care systems, or any Current Synthesis client boundary.\n"
        );
    }

    #[test]
    fn hueman_sandmanor_roles_builder_is_deterministic() {
        assert_eq!(
            build_hueman_sandmanor_roles_from_artifacts("start", "fourway"),
            "# Hueman Sandmanor Roles\n\n\
             ## Structural Rule\n\n\
             Sandmanor carries a rival two-house rule inside Hueman's world layer, where northern accountancy and southern interior-song design compete through reciprocal teaching rather than inherited fixed sovereignty.\n\n\
             ## Canonical Anchor\n\n\
             - Sandmanor remains the West-facing `sprite` start on the Fourway.\n\
             - from Stonebend, Sandmanor sits on the far counter-arc.\n\
             - from Glaushouse, Sandmanor may read eastward across the relational arc without changing the canonical map.\n\n\
             - Sandmanor mines crystals.\n\
             - Sandmanor exports Crystoleum as its main outward trade good, formed from current and crystals.\n\n\
             ## Sandmanor Halves\n\n\
             - Sandmen are the people of Sandmanor.\n\
             - Minoans hold the South.\n\
             - Minorians hold the North.\n\n\
             ## Native Crafts\n\n\
             - Minoans design interiors, rooms, and atmospheres like a song.\n\
             - Minorians account, tally, and measure what Sandmanor can sustain.\n\
             - Minoans and Minorians are the rival houses inside the Sandmen.\n\
             - each side keeps its own people and its own craft pressure.\n\n\
             ## Rival Teaching Contract\n\n\
             - a Minorian must teach a Minoan to account.\n\
             - a Minoan must teach a Minorian to design like a song.\n\
             - each rival has to improve at the other's native discipline rather than remain pure.\n\n\
             ## Sandman Rule\n\n\
             - the crowd judges which rival is most improved by the opposing lesson.\n\
             - the office of rule is the Sandman.\n\
             - a Minoan winner is referred to as the Sandmanite.\n\
             - a Minorian winner is referred to as the Sandmanorian.\n\
             - the winning title-holder becomes ruler of Sandmanor until the contest turns again.\n\
             - rule is earned by witnessed improvement, not fixed inheritance.\n\n\
             ## Status\n\n\
             - Sandmanor roles are descriptive-only for now\n\
             - no contest resolver or crowd AI is active\n\
             - no automatic succession cycle is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Sandmanor roles belong to Hueman's kingdom layer. They do not replace Fourway placement, scene logic, or any Current Synthesis client boundary.\n"
        );
    }

    #[test]
    fn hueman_procedural_uplift_builder_is_deterministic() {
        assert_eq!(
            build_hueman_procedural_uplift_from_artifacts(
                "execution",
                "rules",
                "transition",
                "selection",
                "consequence",
                "gate",
                "stonebend",
                "tross",
                "glaushouse",
                "sandmanor"
            ),
            "# Hueman Procedural Uplift\n\n\
             ## Structural Rule\n\n\
             Hueman may lift procedural contracts from Current Synthesis into world-facing behavior surfaces without moving lower-layer ownership upward.\n\n\
             ## Shared Contract\n\n\
             - Current Synthesis still owns execution spec, behavior rules, transition rules, selection, consequence, and activation gating.\n\
             - Hueman consumes those lower contracts as kingdom-facing procedures.\n\
             - no uplifted procedure may mutate Hollow Grove or rewrite Current Synthesis ownership.\n\n\
             ## Stonebend Procedure\n\n\
             - Proliteriate, Hypergiant, and Freemason enter any civic decision as an equal-power triad.\n\
             - Hypergiant may present first as the public face, but may not finalize alone.\n\
             - Geralds provide the witnessed public body that confirms a civic shift without taking triad power.\n\
             - diamond extraction stays inside Stonebend's civic balance instead of becoming a private sovereign right.\n\
             - hollow current combines with diamond yield into mercury mirror under the same civic balance.\n\
             - until activation changes, Stonebend procedure remains declared rather than executed.\n\n\
             ## Flynt Procedure\n\n\
             - Tross holds the Flynt line from North -> South as the procedural spine.\n\
         - Juvenile checks the North head before Delinquent checks the South end.\n\
         - the four White Dwarfs keep close guard around Tross while Wardens hold the broader line body.\n\
         - opal extraction follows the guarded line body rather than an unbounded field claim.\n\
         - hollow current carries opal yield outward as Opal Oil without breaking the guarded line body.\n\
         - transition pressure may be read through Current Synthesis route order, but no autonomous traversal is enabled.\n\n\
             ## Glaushouse Procedure\n\n\
             - Prima Donna sets command tone and first issuance.\n\
             - Persephone relays or inherits continuity when command steps down.\n\
             - Nightengales run the care loop and stabilize the common body without taking sovereign lead.\n\
             - current combines with jade extraction into Jadomer without displacing Nightengales care duty.\n\
             - succession and care remain procedurally specified but still gated.\n\n\
             ## Sandmanor Procedure\n\n\
             - selection identifies the rival public frame and consequence names the witnessed improvement result.\n\
             - a Minoan winner takes Sandmanite; a Minorian winner takes Sandmanorian.\n\
             - Sandmen bind the crowd witness that legitimizes the Sandman office.\n\
             - current combines with crystal extraction into Crystoleum as part of the stewarded public export both rival houses must carry.\n\
             - reciprocal teaching remains the basis of rule instead of inheritance.\n\n\
             ## Activation Status\n\n\
             - procedural uplift is defined\n\
             - Current Synthesis activation still denies live execution\n\
             - no autonomous NPC state, contest loop, care loop, or guard traversal is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis execution spec bytes: 9.\n\
             Current Synthesis behavior rules bytes: 5.\n\
             Current Synthesis transition bytes: 10.\n\
             Current Synthesis selection bytes: 9.\n\
             Current Synthesis consequence bytes: 11.\n\
             Current Synthesis activation gate bytes: 4.\n\
             Hueman Stonebend Roles bytes: 9.\n\
             Hueman Tross Helpers bytes: 5.\n\
             Hueman Glaushouse Roles bytes: 10.\n\
             Hueman Sandmanor Roles bytes: 9.\n\n\
             ## Boundary Reminder\n\n\
             Procedural uplift makes Hueman ready to consume lower-layer procedures. It does not activate those procedures or grant Hueman authority over Current Synthesis.\n"
        );
    }

    #[test]
    fn hueman_archetype_lens_builder_is_deterministic() {
        assert_eq!(
            build_hueman_archetype_lens_from_artifacts("start", "aura", "roles", "sandmanor"),
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
             ## Stonebend Civic Reading\n\n\
             - Stonebend carries Proliteriate, Hypergiant, and Freemason as an equal-power triad\n\
             - Hypergiant is the public face seen first from outside the structure\n\
             - gremlin reading notices equal leverage behind the public face rather than a single ruler\n\
             - civic order stays vertically integrated with the Fourway start instead of floating above it abstractly\n\n\
             ### `pixy`\n\n\
             - Aura Basin reads as hush, glow, and suspended potential\n\
             - Aura Fields reads as shimmer, weather play, and visible drift\n\
             - Aura Beach reads as glint, spray, and bright dispersal\n\n\
             ### `sprite`\n\n\
             - Aura Basin reads as root echo, sleep, and soft enclosure\n\
             - Aura Fields reads as current, sway, and open circulation\n\
             - Aura Beach reads as horizon pull, release, and farward motion\n\n\
             ## Sandmanor Competitive Reading\n\n\
             - Minoans make the sprite reading notice tuned interiors, cadence, and room-song pressure.\n\
             - Minorians make the sprite reading notice tallies, balance sheets, and public proof.\n\
             - the Sandman contest makes improvement itself visible as the basis of rule.\n\
             - Sandmanor keeps its canonical western place even when relational viewpoints read it from another side.\n\n\
             ## Status\n\n\
             - archetype lens is descriptive-only for now\n\
             - no procedural bonuses or penalties are active\n\
             - the Fourway start roster remains unchanged\n\
             - AuraTriad region behavior remains shared underneath the lens\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Aura Behavior bytes: 4.\n\
             Hueman Stonebend Roles bytes: 5.\n\
             Hueman Sandmanor Roles bytes: 9.\n\n\
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
             ## Aura Ridge Trade Legs\n\n\
             - free trade follows the straight Aura Ridge rather than the underground inverse circle.\n\
             - Stonebend and Glaushouse hold a declared straight trade leg along the ridge.\n\
             - Glaushouse and Sandmanor hold a declared straight trade leg along the ridge.\n\
             - Glaushouse acts as the visible hinge where the right-angle trade body turns.\n\n\
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

    #[test]
    fn hueman_link_physics_builder_is_deterministic() {
        assert_eq!(
            build_hueman_link_physics_from_artifacts("sequence", "cross"),
            "# Hueman Link Physics\n\n\
             ## Structural Rule\n\n\
             Links that do not get bonded may later resolve into `current` or `aura` according to downstream physics.\n\n\
             ## Bond Split\n\n\
             - bonded link stays the selected route\n\
             - unbonded links remain available as unresolved world material\n\
             - unresolved material is not empty; it carries later directional bias\n\n\
             ## Arm Weight Reading\n\n\
             - each `META` letter and its `PLEB` counterpart carry three arms across the same joint\n\
             - one arm per side bonds into the selected link while the remaining arm weight stays unresolved\n\
             - retained heavier continuity pressure tends to read as `current`\n\
             - lighter exposed spill tends to read as `aura`\n\
             - Hueman reads that unresolved weight upward as `current` or `aura` while Hollow Grove keeps the same event as the lower witness simultaneously\n\
             - simultaneous reading does not grant Hueman authority to rewrite the kernel witness\n\n\
             ## Current Bias Physics\n\n\
             - continuity pressure favors `current`\n\
             - occupancy load favors `current`\n\
             - inland persistence favors `current`\n\
             - repeat traversal favors `current`\n\n\
             ## Aura Bias Physics\n\n\
             - exposure pressure favors `aura`\n\
             - threshold bleed favors `aura`\n\
             - atmospheric spill favors `aura`\n\
             - edge drift favors `aura`\n\n\
         ## Element Names\n\n\
         - `current` may also be called Bathos or dark water.\n\
         - `current` appears as dark current or hollow current.\n\
         - `aura` may also be called Aether or air.\n\
         - `aura` appears as reflective aura or holographic aura.\n\n\
             ## Crossover Reading\n\n\
             - shared starts can touch the same unresolved material with different bias\n\
             - the same region may feel more `current` from one route and more `aura` from another\n\
             - crossover zones are where the physics split becomes most visible in Hueman\n\
             - Aura Ridge trade legs keep exchange visible on straight lines while unresolved bias still moves beneath them\n\n\
             ## Status\n\n\
             - link physics is descriptive-only for now\n\
             - no procedural resolver chooses `current` or `aura` yet\n\
             - bond selection remains kernel-simple underneath this layer\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis sequence bytes: 8.\n\
             Hueman Path Crossovers bytes: 5.\n\n\
             ## Boundary Reminder\n\n\
             Link physics explains how unbonded links may later read as `current` or `aura`. It does not rewrite Bond, HollowGrove, or Current Synthesis sequence ownership.\n"
        );
    }

    #[test]
    fn hueman_inverse_circle_builder_is_deterministic() {
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
    fn hueman_crossover_scenes_builder_is_deterministic() {
        assert_eq!(
            build_hueman_crossover_scenes_from_artifacts("cross", "physics"),
            "# Hueman Crossover Scenes\n\n\
             ## Structural Rule\n\n\
             When `current`-biased and `aura`-biased unresolved links appear at the same crossover, the world produces a named descriptive scene type.\n\n\
             ## Scene Types\n\n\
             ### Seam Market\n\n\
             - appears where `current` continuity and `aura` spill remain in balance\n\
             - feels like trade, rumor, salvage, and temporary arrangement\n\
             - fits shared Aura Fields crossings best\n\
             - commonly appears along the Stonebend -> Glaushouse Aura Ridge leg\n\n\
             ### Threshold Weather\n\n\
             - appears where `aura` exposure outruns `current` continuity\n\
             - feels like spray, drift, shimmer, and unstable edges\n\
             - fits shared Aura Beach crossings best\n\n\
             ### Pressure Shelter\n\n\
             - appears where `current` persistence contains `aura` residue\n\
             - feels like storage, burrow heat, muffled exchange, and held tension\n\
             - fits shared Aura Basin crossings best\n\n\
             ### Split Trace\n\n\
             - appears where both biases are present but neither settles the scene\n\
             - feels like afterimage, contradictory clues, and route ambiguity\n\
             - fits delayed or secondary crossovers after different openings\n\n\
             ## Placement\n\n\
             - Aura Beach tends toward Threshold Weather first.\n\
             - Aura Fields tends toward Seam Market first.\n\
             - Aura Basin tends toward Pressure Shelter first.\n\
             - Split Trace can appear in any crossover zone where the bias remains unresolved.\n\n\
             ## Status\n\n\
             - crossover scenes are descriptive-only for now\n\
             - no encounter tables or event resolvers are active\n\
             - link physics remains the upstream explanation for the scene type\n\
             - path crossovers remain the upstream overlap map\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Path Crossovers bytes: 5.\n\
             Hueman Link Physics bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Crossover scenes name what a shared biased overlap feels like. They do not create procedural meetings, rewards, or movement rules.\n"
        );
    }

    #[test]
    fn hueman_scene_presence_builder_is_deterministic() {
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
    fn hueman_scene_intent_builder_is_deterministic() {
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
    fn hueman_scene_drift_builder_is_deterministic() {
        assert_eq!(
            build_hueman_scene_drift_from_artifacts("intent", "physics"),
            "# Hueman Scene Drift\n\n\
             ## Structural Rule\n\n\
             Scene intent can drift into a different scene type when bias pressure persists over time without a full system resolving it.\n\n\
             ## Drift Vectors\n\n\
             ### Seam Market\n\n\
             - drifts toward Pressure Shelter when exchange slows and stored continuity takes over\n\
             - drifts toward Threshold Weather when structures fail and exposure outruns arrangement\n\n\
             ### Threshold Weather\n\n\
             - drifts toward Split Trace when warning persists without settlement\n\
             - drifts toward Seam Market when repeated crossings stabilize the edge into exchange\n\n\
             ### Pressure Shelter\n\n\
             - drifts toward Seam Market when guarded stores reopen into circulation\n\
             - drifts toward Split Trace when shelter empties and only residue remains\n\n\
             ### Split Trace\n\n\
             - drifts toward Threshold Weather when ambiguity spills outward into exposure\n\
             - drifts toward Pressure Shelter when traces are hoarded, muffled, or enclosed\n\n\
             ## Drift Drivers\n\n\
             - sustained `current` accumulation pulls scenes toward storage, continuity, and reopened exchange\n\
             - sustained `aura` accumulation pulls scenes toward exposure, drift, shimmer, and unstable edges\n\
             - mixed unresolved pressure preserves Split Trace longer instead of forcing a clean resolution\n\
             - repeated crossings can stabilize a scene back into exchange even after warning or ambiguity\n\n\
             ## Status\n\n\
             - scene drift is descriptive-only for now\n\
             - no time simulation or procedural resolver is active\n\
             - scene intent remains the upstream atmospheric layer\n\
             - link physics remains the upstream bias layer\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Scene Intent bytes: 6.\n\
             Hueman Link Physics bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Scene drift says how a scene may change if its pressure persists. It does not activate clocks, AI routines, or procedural world updates.\n"
        );
    }

    #[test]
    fn vertical_integration_stack_builder_is_deterministic() {
        assert_eq!(
            build_vertical_integration_stack_from_artifacts(
                "base",
                "boundary",
                "glaushouse",
                "sandmanor",
                "inverse",
                "procedures",
                "presence",
                "intent",
                "drift"
            ),
            "# Vertical Integration Stack\n\n\
             ## Structural Rule\n\n\
             Hollow Grove remains the recursive core, KernelPass witnesses that core, Current Synthesis consumes the witnessed artifact layer as the operating layer, and Hueman consumes Current Synthesis as the world layer above it.\n\n\
             ## Full Stack\n\n\
             ```text\n\
             Symptom 1\n\
             ↓\n\
             Triway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             GroveSeam\n\
             ↓\n\
             HollowBeam\n\
             ↓\n\
             landed Symptom 2\n\
             ↓\n\
             KernelPass\n\
             ↓\n\
             Client Artifacts\n\
             ↓\n\
             Current Synthesis\n\
             ↓\n\
             Hueman\n\
             ↓\n\
             World Roles and Scenes\n\
             ```\n\n\
             ## Layer Ownership\n\n\
             - kernel owns recursion, bond selection, and the canonical witness\n\
             - client artifacts mirror the witness without mutating kernel state\n\
             - Current Synthesis owns route semantics, activation gating, and client boundaries like HAL and Clouseau\n\
             - Hueman owns Fourway, AuraTriad reading, start placement, civic roles, helper lines, and scene reading\n\n\
             ## Current Alignment\n\n\
             - Stonebend remains East = `gremlin` with an equal-power civic triad above Current Synthesis\n\
             - Tross remains anchored in Flynt and runs as a North -> South helper line with four White Dwarfs as the personal guard\n\
             - Glaushouse remains South = `pixy` with Prima Donna over Persephone and Nightengales as the common people\n\
             - Sandmanor remains West = `sprite` with southern Minoans, northern Minorians, and a crowd-judged Sandman contest\n\
         - resource seams are designated across Hueman: Stonebend diamonds, Flynt opals, Glaushouse jades, Sandmanor crystals\n\
         - `current` reads as Bathos or dark water while `aura` reads as Aether or air at the Hueman layer\n\
         - the bedrock split remains active upstream: dark current and hollow current, reflective aura and holographic aura\n\
         - the visible free-trade body follows the Aura Ridge straight legs Stonebend -> Glaushouse and Glaushouse -> Sandmanor\n\
             - Stonebend declares Mercury Mirror from hollow current + diamonds, Flynt declares Opal Oil from hollow current + opal, Glaushouse declares Jadomer from current + jades, and Sandmanor declares Crystoleum from current + crystals\n\
         - the inverse circle remains an underground interior tunnel ring: Stairway to Heaven, Riptide, Current Sea, Aura Way\n\
             - procedural uplift now maps Current Synthesis execution contracts into Hueman-facing kingdom procedures without moving ownership upward\n\
             - scene presence, scene intent, and scene drift remain the top descriptive Hueman layer\n\
             - upper layers consume lower layers without rewriting lower ownership\n\n\
             ## Boundary Contract\n\n\
             - Hollow Grove does not know Current Synthesis exists\n\
             - Current Synthesis does not know Hueman exists\n\
             - Hueman consumes Current Synthesis without feeding back into it\n\
             - runtime and benchmark follow the same downstream route when regenerating artifacts\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis Base bytes: 4.\n\
             Hueman Boundary bytes: 8.\n\
             Hueman Glaushouse Roles bytes: 10.\n\
             Hueman Sandmanor Roles bytes: 9.\n\
             Hueman Inverse Circle bytes: 7.\n\
             Hueman Procedural Uplift bytes: 10.\n\
             Hueman Scene Presence bytes: 8.\n\
             Hueman Scene Intent bytes: 6.\n\
             Hueman Scene Drift bytes: 5.\n\n\
             ## Boundary Reminder\n\n\
             This stack artifact documents vertical alignment only. It does not grant any upper layer authority to mutate the kernel or bypass existing layer boundaries.\n"
        );
    }
}
