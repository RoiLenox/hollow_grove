use std::cell::RefCell;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::thread::LocalKey;

use crate::hollow_grove_content::{
    build_elf_radiologist_role, build_flyntian_dagger_profile,
    build_glaushouse_medical_team_profile, build_gnome_emergency_physician_role,
    render_regional_item_profile, render_role_profile, validate_generated_content_batch,
    validate_medical_team_profile, validate_regional_item_profile,
};
use crate::hueman_slice::{VerticalSliceSpec, primary_vertical_slice};

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
pub const CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_collision_relay.md";
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
pub const HUEMAN_VERTICAL_SLICE_ARTIFACT_PATH: &str = "artifacts/hueman_vertical_slice.md";

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

pub fn hueman_vertical_slice_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_VERTICAL_SLICE_ARTIFACT_PATH)
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
    sections_markdown_cache: &'static OnceLock<String>,
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
    "public trade legs are declared from Stonebend -> Glaushouse and Glaushouse -> Sandmanor, with Sandmanor's straight continuation reaching Aura Fields at the Stonebend/Glaushouse junction",
    "the straight ridge still belongs to the same large circular map body that rounds onward through the rest of Hueman",
];

const AURA_RIDGE_INTENT_LINES: &[&str] = &[
    "keep free trade moving along the declared straight ridge legs without collapsing kingdom identity",
    "let Glaushouse serve as the visible turn where eastern and western trade pressure changes direction",
    "keep the straight ridge legible without breaking the larger circular world loop",
];

const GLAUSHOUSE_PRESENCE_LINES: &[&str] = &[
    "repair: Glaushouse is the repair and clearance function for both machine bodies and Hueman bodies",
    "style: Glaushouse presents as a mechanical-industrial medical capital with Berlin severity, Milan polish, chrome discipline, and clinic glamour",
    "Prima Donna: visible institutional authority and final grant of Clearance",
    "Persephone: triage lead, recovery guide, and right hand beside the Prima Donna",
    "Nightingales: Glaushouse white-blood-cell civic people studied through diagnosis, inflammation, drainage, and recovery rather than treated as public staff ranks",
    "Glaus Gel: the signature repair resource supporting sealing, bonding, cooling, and restoration",
    "Glausteel: the hard integrated alloy associated with cleared machine and civic work",
];

const GLAUSHOUSE_INTENT_LINES: &[&str] = &[
    "repair: restore function without erasing real damage or real risk",
    "style: run recovery through cold-lit wards, industrial bays, strict presentation, and commanding public poise rather than rustic comfort",
    "Prima Donna: grant Clearance without escaping challenge from the floor",
    "Persephone: manage descent, triage, recovery, and return",
    "Nightingales: remain the internal white-blood-cell witness when charts or appearances claim more recovery than the body can verify",
    "Glaus Gel: support machine and Hueman repair as a practical synthesis medium",
    "Glausteel: carry the cleared integrated hard branch after restoration and synthesis work",
];

const STONEBEND_PRESENCE_LINES: &[&str] = &[
    "craft: Stonebend names, titles, and makes material claims legible as buildable form",
    "Proletariat: collective labor pressure and shared leverage beneath any formal title",
    "Hypergiant: the visible holder of Title without ownership of the constitution itself",
    "Freemason: structural craft, standards, and whether a named thing can actually be built and kept standing",
    "Stonebend presence keeps Title, Labor, and Craft visibly braided",
    "diamonds: reflective mineral wealth aligned with title, structure, and witness",
    "Mercury Mirror: Stonebend's signature reflective resource refined through Hollowing and reflective craft",
    "Mercurite: Stonebend's hard material branch for tools, armor, and structural bearing",
];

const STONEBEND_INTENT_LINES: &[&str] = &[
    "craft: name the thing, declare its burden, and shape material into workable repeatable form",
    "Proletariat: challenge false authority from the side of lived work and borne burden",
    "Hypergiant: carry the public Title while remaining challengeable",
    "Freemason: test whether named structure can bear, transfer, and maintain load",
    "reallocate pressure: move burden, title, and structural responsibility into the right vessel or route",
    "Hollowing: refine carried pressure out of active current without making the remaining current meaningless",
    "Mercury Mirror: refine hollow current toward reflective craft rather than decorative glamour",
    "Stonebend intent keeps authority challengeable instead of absolute",
];

const FLYNT_PRESENCE_LINES: &[&str] = &[
    "engineering: Flynt is the recognition and field-engineering function that moves capability into lived operation",
    "style: Flynt presents as a boardwalk-casino hunting capital with neon vice, opal glamour, North African desert grounds, and hard modern swagger",
    "Tross: the recognizing head of Flynt and bearer of Contracore",
    "Delinquent: west guard pressure testing rejected routes and hard alternatives",
    "Juvenile: east guard pressure holding beginnings, apprenticeship, and undeclared potential",
    "White Dwarfs: four close guards holding Tross's personal ring without taking the north or south posts",
    "Wardens: the people of Flynt holding common infrastructure and route continuity",
    "opals: mined feedstock and recognition-rich gleam carried through Flynt engineering",
    "Opal Oil: Flynt's signature resource produced through regular current and holographic aura",
    "Flynt presence keeps hidden value and field usefulness legible before title catches up",
];

const FLYNT_INTENT_LINES: &[&str] = &[
    "engineering: turn practical capability into deployable routes, services, machines, and infrastructure",
    "style: run recognition through boardwalk temptation, casino risk, nocturnal glamour, and outer hunting pressure rather than polite institutional order",
    "Tross: recognize what is actually there even when institutions have denied or overlooked it",
    "Delinquent: pressure false certainty from the west through deviation and challenge",
    "Juvenile: protect beginnings and not-yet-recognized worth from the east",
    "White Dwarfs: keep the close guard without becoming substitute rulers",
    "Wardens: hold the common route body and public engineering substrate",
    "opals: support advanced engineering feedstock and field-ready refinement",
    "Opal Oil: turn regular current and holographic aura into a practical engineering medium",
    "helpers keep recognition challengeable instead of becoming sovereign rank",
];

const SANDMANOR_PRESENCE_LINES: &[&str] = &[
    "configuration: Sandmanor is the proof and design function, split between Minorian count and Minoan arrangement",
    "Sandmen: the people and witness body of Sandmanor beneath the contest",
    "Minoans: designers, arrangers, modelers, and builders of intentional composition",
    "Minorians: counters, record-keepers, measurers, and public witnesses of proof",
    "Aura Beach: the Minoan-facing coastal court where High Elf judgment, display, and arrangement stay socially visible",
    "Aura Fields: the Minorian-facing proof ground where count, measure, and public comparison stay exposed",
    "The Sandman: the singular leader of the Sandmen, chosen through visible improvement",
    "Prism Sand: Sandmanor's signature reflective resource for measurement, refraction, and records",
    "Prismiron: Sandmanor's hard branch for proof-ready structures and instruments",
    "Sandmanor presence favors witnessed comparison over inherited fixed rank",
];

const SANDMANOR_INTENT_LINES: &[&str] = &[
    "configuration: prove the claim, test the proof, and improve the arrangement",
    "Sandmen: carry the public witness that makes contest and improvement socially binding",
    "Minoans: teach modeled arrangement, design, and structure",
    "Minorians: teach count, measure, assay, and reproducible proof",
    "Aura Beach: run visible judgment through the High Elf court so arrangement, etiquette, and display stay publicly enforceable",
    "Aura Fields: run Minorian proof through open comparison so measure, tally, and market-facing verification can be contested",
    "The Sandman: emerge through the greatest witnessed improvement rather than inheritance",
    "Prism Sand: support counting, refraction, glass, and record logic",
    "Prismiron: support precise durable structures after proof survives inspection",
    "Sandmanor intent keeps rivalry productive instead of merely punitive",
];

const INVERSE_CIRCLE_PRESENCE_LINES: &[&str] = &[
    "The Stairway to Heaven: high border ascent above an underground inverse curve, with rung marks and lifted edge traffic",
    "The Riptide: outer-border pull currents above an underground inverse curve, with drag marks, return pressure, and Merman roaming along the rim",
    "The Current Seanad: exposed outer water band above an underground inverse curve, with counted channels and measured carry",
    "Mnt. Aura: outer border curve from Stonebend to Sandmanor above an underground inverse curve, with bright air and felt border pressure before sight",
];

const INVERSE_CIRCLE_INTENT_LINES: &[&str] = &[
    "The Stairway to Heaven: raise the border upward while its underground inverse curve stores hidden descent pressure",
    "The Riptide: pull travelers backward through edge pressure and memory while its underground inverse curve keeps hidden return pull and Merman range alive on the water rim",
    "The Current Seanad: measure, sustain, and carry motion around the outer water border while its underground inverse curve keeps hidden current structure",
    "Mnt. Aura: hold the curved outer border from Stonebend to Sandmanor while its underground inverse curve keeps the hidden under-arc",
];

const GREMLIN_LENS_LINES: &[&str] = &[
    "Aura Basin reads as hunt pressure, route opportunities, den seams, and drillable carry",
    "Aura Fields reads as infrastructure crossings, farm lanes, survey lanes, and deployable work",
    "Aura Beach reads as exposure, salvage, boardwalk threshold, and field-engineering release",
];

const GOBLIN_LENS_LINES: &[&str] = &[
    "Aura Basin reads as load paths, hidden supports, and structural bearing",
    "Aura Fields reads as named work sites, claim boundaries, and public craft pressure",
    "Aura Beach reads as threshold framing, recovery of material, and edge structure",
];

const GOBLIN_OVERLAY_LINES: &[&str] = &[
    "Stonebend carries Proletariat, Hypergiant, and Freemason as a constitutional balance",
    "Hypergiant is the public face seen first from outside the structure",
    "goblin reading notices whether title is still legitimate beneath the visible face",
    "Stonebend pressure logic asks who bears the claim and how the burden is reallocated",
];

const SPRITE_LENS_LINES: &[&str] = &[
    "Aura Basin reads as triage depth, hidden damage, and latent recovery need",
    "Aura Fields reads as treatment traffic, care signals, and integration risk",
    "Aura Beach reads as exposure, vulnerability, and urgent stabilization",
];

const PIXY_LENS_LINES: &[&str] = &[
    "Aura Basin reads as count density, hidden records, and unresolved variables",
    "Aura Fields reads as Minorian comparison ground, measurable change, and public proof pressure",
    "Aura Beach reads as Minoan exposure court, edge cases, and test conditions held near judgment and display",
];

const PIXY_OVERLAY_LINES: &[&str] = &[
    "Minoans make the pixy reading notice arrangement, composition, and configuration pressure.",
    "Minorians make the pixy reading notice tally, measure, and public proof.",
    "Aura Beach belongs to the Minoan side, where High Elf Court keeps visible arrangement under public judgment.",
    "Aura Fields belongs to the Minorian side, where proof is compared, counted, and made legible in public.",
    "the Sandman contest makes improvement itself visible as the basis of rule.",
    "Sandmanor keeps its canonical western place even when relational viewpoints read it from another side.",
];

const AURA_REGION_DEFINITIONS: [AuraRegionDefinition; 3] = [
    AuraRegionDefinition {
        name: "Aura Basin",
        movement: "movement reads as inward and narrowing",
        encounter: "encounter tone reads as close, muffled, and formative",
        world_description: "world description favors pressure, shelter, dens, and accumulation",
    },
    AuraRegionDefinition {
        name: "Aura Fields",
        movement: "movement reads as lateral and exposed",
        encounter: "encounter tone reads as social, visible, publicly witnessed, and negotiable",
        world_description: "world description favors weather, distance, farming, and traversal",
    },
    AuraRegionDefinition {
        name: "Aura Beach",
        movement: "movement reads as outward and threshold-facing",
        encounter: "encounter tone reads as reflective, exposed, judged, and releasing",
        world_description: "world description favors edge, court display, training, and departure",
    },
];

const STONEBEND_ROLE_SECTIONS: [HuemanSectionDefinition; 6] = [
    HuemanSectionDefinition {
        title: "Role Definitions",
        lines: &[
            "Proletariat bears labor witness, shared burden, and the right to challenge false Title from below.",
            "whoever holds the Hypergiant Crown is Hypergiant: visible Title, public naming, and outward negotiation without sovereign exemption.",
            "Hypergiant is the Stonebend office that possesses the Troglodyte when Title has to descend into the deepest burden form.",
            "Freemason tests craft, structure, and whether the named form can actually carry load.",
        ],
    },
    HuemanSectionDefinition {
        title: "Creature Rank Ladder",
        lines: &[
            "Stonebend's creature ladder rises through Gremlin -> Goblin -> Ghoul -> Troll -> Ork -> Ogre -> Troglodyte.",
            "Gremlin marks the apprentice pressure-reader who learns scrap cunning, hidden seams, and first burden claims.",
            "Goblin marks the named craft worker who can hold one public job, one claim, and one visible tool burden.",
            "Ghoul marks the rank where the worker can stay with ruin, salvage, and difficult remains without losing structural judgment.",
            "Troll marks stubborn endurance, bridge custody, and long-span structural holding.",
            "Ork marks the war-labor rank: direct force, organized push, and contested build defense under pressure.",
            "Ogre marks the heavy carrier who takes direct load transfer, breach work, and blunt material relocation.",
            "Troglodyte marks the deepest Stonebend rank: understructure memory, cavern authority, and last-burden custody.",
        ],
    },
    HuemanSectionDefinition {
        title: "Power Balance",
        lines: &[
            "Proletariat, Hypergiant, and Freemason hold Stonebend's constitutional balance as Title, Labor, and Craft.",
            "Hypergiant is the public face of the balance, not a higher authority than the others.",
            "the office of Hypergiant may not remain vacant; if the Crown is broken, opposed, or lost, another bearer must be raised.",
            "the Proletariat and the Freemason may each oppose the Hypergiant Crown, alone or together, if the Title is no longer legitimate.",
            "either office may take the Crown and become Hypergiant, and both may move against it at once.",
            "no single role may collapse the balance into solo rule.",
        ],
    },
    HuemanSectionDefinition {
        title: "Stonebender",
        lines: &[
            "the Stonebender is the public proving ground at Stonebend where Hypergiant, Proletariat, and Freemason may contest supremacy without pretending the constitution is peaceful.",
            "the Stonebender takes place in a Stonehenge-like arena at Stonebend: standing-stone rings, ritual lanes, weapon circles, burden relays, and last-standing contests under public witness.",
            "Hypergiant enters as visible title and reigning crown pressure.",
            "Proletariat enters as labor force, crowd will, and below-title challenge made bodily.",
            "Freemason enters as structural discipline, engineered force, and proof that craft can defeat mere mass or spectacle.",
            "the Stonebender does not replace the Crown, but it is the grand public pressure test that may justify opposition, succession pressure, or renewed legitimacy.",
        ],
    },
    HuemanSectionDefinition {
        title: "Constitutional Role",
        lines: &[
            "Stonebend carries Name It and Craft across both Hueman and Hollow Grove-facing interpretation.",
            "Goblin is Stonebend's confirmed Current-origin path, but Stonebend is not reduced to a form ladder.",
            "Stonebend mines diamonds and reflective mineral structures.",
            "Stonebend separates hollow current from regular current before reflective craft begins.",
            "Stonebend uses Hollowing and reflective craft to produce Mercury Mirror rather than treating Hollowing as ordinary synthesis.",
            "Mercurite is the accepted hard branch for Stonebend-made structural bearing.",
            "Stonebend's signature pressure action is reallocation.",
            "Current Synthesis remains a lower operating layer beneath this governance.",
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

const TROSS_ROLE_SECTIONS: [HuemanSectionDefinition; 7] = [
    HuemanSectionDefinition {
        title: "Anchor",
        lines: &[
            "Tross is in Flynt.",
            "Flynt remains West on the Fourway roster.",
            "Flynt carries Recognize It and Engineering across Hueman and Hollow Grove-facing interpretation.",
            "Flynt reads as a boardwalk-casino hunting capital with opal glamour, nocturnal vice, and outer desert hunt pressure.",
            "Aura Basin is Flynt's nearest hunt ground, where Gargoyles contest Werewolves before the outward climb into Aura Ridge circulation.",
            "The Riptide is the outer roaming water rim where Mermen are hunted and harvested.",
            "Wardens are the people of Flynt.",
            "Flynt mines opals.",
            "Flynt exports Opal Oil as its main outward trade good, formed from regular current and holographic aura.",
            "Gremlin is Flynt's confirmed Current-origin path.",
            "Tross runs West -> East rather than spanning the whole Fourway equally.",
        ],
    },
    HuemanSectionDefinition {
        title: "Hybrid Rank Ladder",
        lines: &[
            "Flynt is not a straight creature ladder; it is a mixing pit with convergent ascent.",
            "Gargoyle is Flynt's first mixed synthesis recipe: Gremlin pressure and Pixy finesse hardened together into air-and-stone watchfulness, route memory, and mineral endurance held in one body, and the standard body used for Aura Basin hunts.",
            "Merman is a sea-current form a Flyntian Gargoyle must hunt and harvest along the Riptide when opal flow, reflective depth, and Pixy fluidity turn seaward.",
            "Werewolf is a feral land-hunt form a Flyntian Gargoyle must hunt and harvest in Aura Basin when Gremlin aggression and night-pressure turn predatory and upright.",
            "Chimera is its own completed synthesis recipe and form: the successful synthesis of Gargoyle, Merman, and Werewolf into one composite war-body with multiple instincts held at once.",
            "Manticore is the later apex synthesis recipe mastered after Chimera: the face-bearing body with ranged deterrence, territorial memory, armored posture, difficult outward reach, and the final Flynt form that can oppose Stonebend's Troglodyte.",
            "these creature ranks do not replace Tross, Delinquent, Juvenile, Wardens, or the White Dwarfs; they describe growth inside Flynt's mixed engineering lineage.",
        ],
    },
    HuemanSectionDefinition {
        title: "Progression Contract",
        lines: &[
            "Flynt progression is recipe-gated rather than hereditary: office, hunt right, and creature form remain separate.",
            "a contender first gathers opal-bearing field materials in Flynt, uncovers recipe instruction through route puzzles and treasure-hunt clues, and then submits the ingredient body to Glaushouse synthesis before embodiment counts as stable public form.",
            "Gargoyle is the mandatory first embodiment for living and hunting as Flynt; no Werewolf or Merman hunt right opens before Gargoyle mastery is stable.",
            "Werewolf remains the Aura Basin branch: it must be hunted and harvested on land by a Gargoyle and brought back through the guarded Flynt line as verified proof.",
            "Merman remains the Riptide branch: it must be hunted and harvested on the water rim by a Gargoyle and brought back through the same line as verified proof.",
            "Chimera only counts after Gargoyle, Werewolf, and Merman have each been separately mastered and then recombined into one completed synthesis recipe.",
            "Manticore is not Chimera under another name; it is the later mastered escalation opened after Chimera through a harder Contracore-facing ordeal.",
            "only a mastered Manticore may challenge Tross for Contracore, because challenge body and ruling office must remain distinct.",
        ],
    },
    HuemanSectionDefinition {
        title: "Role Definitions",
        lines: &[
            "whoever holds Contracore is Tross: the line head and first recognizer of what is actually present in Flynt's field.",
            "the office of Tross may not remain vacant; if Contracore changes hands, the next holder is Tross immediately.",
            "only someone who has mastered the Manticore recipe and form may challenge Tross for Contracore.",
            "Delinquent guards West by challenging false certainty, rejected routes, and brittle plans.",
            "Juvenile guards East by protecting beginnings, apprenticeship, and not-yet-recognized worth.",
            "Wardens keep the common route body, infrastructure continuity, and public engineering substrate of Flynt.",
        ],
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
        title: "West-East Guard",
        lines: &[
            "Delinquent guards West at the Flynt-facing head of the line.",
            "Juvenile guards East.",
            "helper duty runs down the line from Flynt instead of behaving like sovereign rule.",
            "recognition remains field-facing and challengeable rather than becoming another naming office.",
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

const GLAUSHOUSE_ROLE_SECTIONS: [HuemanSectionDefinition; 5] = [
    HuemanSectionDefinition {
        title: "Canonical Anchor",
        lines: &[
            "Glaushouse remains East-facing on the Fourway.",
            "Glaushouse carries Clear It, Repair, and exclusive Synthesis authority for both machine and Hueman bodies.",
            "Glaushouse reads as a mechanical-industrial medical capital: Berlin severity, Milan polish, chrome discipline, and commanding clinic glamour.",
            "Sprite is Glaushouse's confirmed Aura-origin path.",
            "Glaushouse mines jades and refines Glaus Gel as its jade-colored repair and synthesis medium.",
            "Glausteel is the accepted hard branch for cleared integrated work.",
        ],
    },
    HuemanSectionDefinition {
        title: "Role Definitions",
        lines: &[
            "whoever holds final public Clearance is Prima Donna: visible command, named release authority, and final judgment answerability.",
            "Persephone runs triage, descent, recovery, and relay continuity beneath the Prima Donna.",
            "Glaushouse physicians, nurses, and rehabilitation staff hold bedside truth, common repair work, and the day-to-day restoration body of Glaushouse.",
            "Nightingales remain the white-blood-cell civic defense image Glaushouse carries through recognition, clearing, and recovery.",
            "Glaushouse keeps medicine, machinery, presentation, and discipline visibly braided instead of hiding one behind the others.",
        ],
    },
    HuemanSectionDefinition {
        title: "Creature Rank Ladder",
        lines: &[
            "Glaushouse's creature ladder rises through Pixy -> Sprite -> Farie -> Nymph -> Siren -> Muse.",
            "Pixy marks the first care rank: nimble bedside attention, tiny repairs, and quick perceptive response.",
            "Sprite marks active floor care, relay motion, and immediate restoration duty under visible pressure.",
            "Farie marks the rank where care becomes consciously enchanted: delicate repair, tonal guidance, and restorative charm used with discipline.",
            "Nymph marks the rank where healing presence, ward atmosphere, and sustained recovery guidance become stable.",
            "Siren marks the rank where voice, recall, and commanding recovery presence can call people back from drift, refusal, or dangerous surrender.",
            "Muse marks the highest Glaushouse rank: restorative inspiration, formal guidance, and the ability to set the tone of recovery without leaving care behind.",
            "Muse is the highest care form in Glaushouse, not a separate sovereign office above Prima Donna.",
        ],
    },
    HuemanSectionDefinition {
        title: "Social Balance",
        lines: &[
            "Prima Donna grants Clearance and public face.",
            "Persephone carries relay authority through triage, descent, and return.",
            "the recovery floor keeps repair, bedside care, and the lived body of the kingdom.",
            "the office of Prima Donna may not remain vacant; public Clearance must always have a final holder.",
            "Clearance may be challenged from the floor when restoration is not yet real.",
            "Persephone and the recovery floor may each oppose false Clearance, alone or together, until recovery is materially real.",
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

const SANDMANOR_ROLE_SECTIONS: [HuemanSectionDefinition; 7] = [
    HuemanSectionDefinition {
        title: "Canonical Anchor",
        lines: &[
            "Sandmanor remains South-facing on the Fourway.",
            "from Stonebend, Sandmanor sits on the far counter-arc.",
            "from Glaushouse, Sandmanor may read southward across the relational arc without changing the canonical map.",
            "",
            "Sandmanor carries Prove It and Configuration, split between Minorian count and Minoan arrangement.",
            "Pixy is Sandmanor's confirmed Aura-origin path.",
            "Sandmanor mines crystals and refines Prism Sand as its signature proof-and-record resource.",
            "Prismiron is the accepted hard branch for proof-ready structures and instruments.",
        ],
    },
    HuemanSectionDefinition {
        title: "Role Definitions",
        lines: &[
            "Sandmen are the people and witness body of Sandmanor.",
            "Minoans are the southern design-and-arrangement house.",
            "Minorians are the northern count-and-proof house.",
            "The Sandman is the singular office of rule, won by witnessed improvement rather than inherited bloodline.",
        ],
    },
    HuemanSectionDefinition {
        title: "Native Crafts",
        lines: &[
            "Minoans practice arrangement, composition, modeling, and intentional structure.",
            "Minorians practice account, tally, measure, assay, and public proof.",
            "Aura Beach is the Minoan court strand, where High Elf judgment runs visible arrangement near the shore.",
            "Aura Fields is the Minorian proof plain, where count, comparison, and witnessed verification are kept public.",
            "Minoans and Minorians are the rival houses inside the Sandmen.",
            "each side keeps its own people and its own craft pressure.",
        ],
    },
    HuemanSectionDefinition {
        title: "Hybrid House Ladders",
        lines: &[
            "Sandmanor is divided between Minorian Gnomes and Minoan Elves rather than one shared creature chain.",
            "Minorian Gnomes govern counting, measurement, grading, inventory, records, scheduling, allocation, and maintenance.",
            "Gnomes do not evolve through a formal ladder.",
            "A Gnome becomes whatever it keeps.",
            "Minoan Elves govern design, fashion, art, architecture, presentation, public culture, and visible interpretation.",
            "Gnomes internalize Aura into order.",
            "Elves externalize Aura into expression.",
            "the Sandman may emerge from either house after witnessed reciprocal improvement rather than from a fixed species ladder.",
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
            "the office of Sandman may not stand vacant; if one falls, the witnessed contest turns again until another is recognized.",
            "either house may produce The Sandman if the crowd recognizes the stronger reciprocal improvement.",
            "the winning contender becomes The Sandman.",
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

static STONEBEND_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static TROSS_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static GLAUSHOUSE_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static SANDMANOR_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();

const STONEBEND_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Stonebend Roles",
    structural_rule: "Stonebend carries Name It and Craft through a three-part civic balance that belongs to Hueman's world layer and remains vertically integrated above Current Synthesis and Hollow Grove.",
    sections: &STONEBEND_ROLE_SECTIONS,
    sections_markdown_cache: &STONEBEND_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Stonebend roles belong to Hueman's civic layer. They do not replace HAL, Clouseau, or any Current Synthesis client boundary.",
};

const TROSS_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Tross Helpers",
    structural_rule: "Tross is anchored in Flynt, where recognition and engineering run as a west-to-east helper line inside Hueman's world layer.",
    sections: &TROSS_ROLE_SECTIONS,
    sections_markdown_cache: &TROSS_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Tross helpers belong to Hueman's Flynt-anchored directional line. They do not replace Fourway placement, civic roles, or kernel ownership.",
};

const GLAUSHOUSE_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Glaushouse Roles",
    structural_rule: "Glaushouse carries Clear It, Repair, and exclusive Synthesis through a scene-facing court and care order inside Hueman's world layer, where leadership, triage, machine repair, and Hueman nursing remain socially visible without leaving the Fourway boundary.",
    sections: &GLAUSHOUSE_ROLE_SECTIONS,
    sections_markdown_cache: &GLAUSHOUSE_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Glaushouse roles belong to Hueman's kingdom layer. They do not replace scene logic, procedural care systems, or any Current Synthesis client boundary.",
};

const SANDMANOR_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Sandmanor Roles",
    structural_rule: "Sandmanor carries Prove It and Configuration through a rival two-house rule inside Hueman's world layer, where Minorian count and Minoan arrangement compete through reciprocal teaching rather than inherited fixed sovereignty.",
    sections: &SANDMANOR_ROLE_SECTIONS,
    sections_markdown_cache: &SANDMANOR_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Sandmanor roles belong to Hueman's kingdom layer. They do not replace Fourway placement, scene logic, or any Current Synthesis client boundary.",
};

const STONEBEND_PROCEDURE_LINES: &[&str] = &[
    "Name It and Craft remain the Stonebend procedure functions, turning civic pressure into named made form.",
    "Proletariat, Hypergiant, and Freemason enter any civic decision as a constitutional balance.",
    "Hypergiant may present first as the public face, but may not finalize alone.",
    "Stonebend still gathers materials and recipe knowledge for form ascent, but routes embodiment through Hollowing first.",
    "Stonebend also externalizes rivalry through the Stonebender: a Stonehenge-like public arena at Stonebend where crown pressure, labor force, and structural craft are tested before the crowd.",
    "diamonds and reflective mineral structures stay inside Stonebend's civic balance instead of becoming a private sovereign right.",
    "Hollowing separates hollow current from regular current, reallocates carried pressure, and supports Mercury Mirror under the same civic balance.",
    "Stonebend does not treat Hollowing as ordinary synthesis; Hollowing is its trademark civic gate before accepted form can be named and borne.",
    "Stonebend procedure asks what the thing is called, what burden it claims, and where that burden belongs.",
    "until activation changes, Stonebend procedure remains declared rather than executed.",
];

const FLYNT_PROCEDURE_LINES: &[&str] = &[
    "Recognize It and Engineering remain the Flynt procedure functions, turning capability into operation and field trust.",
    "Tross holds the Flynt line from West -> East as the procedural spine.",
    "Delinquent checks the West head before Juvenile checks the East end.",
    "the four White Dwarfs keep close guard around Tross while Wardens hold the broader line body.",
    "Flynt routes ascent through boardwalk risk, casino pressure, outer hunting expeditions, and public recognition instead of quiet technical certification.",
    "Flynt knowledge gates open through puzzle trails, treasure-hunt clues, and route memory rather than library inheritance or bloodline permission.",
    "a Flynt contender gathers ingredients in the field but still has to pass through Glaushouse synthesis before a claimed form counts as stable public embodiment.",
    "Gargoyle is the mandatory first embodiment and the legal first hunt body for Flynt progression.",
    "Flynt uses Aura Basin as the near land-hunt ground for Gargoyle versus Werewolf conflict, then lifts yield upward into Aura Ridge circulation rather than keeping the kill buried where it fell.",
    "Flynt uses the Riptide as the roaming water rim where Mermen are pursued, harvested, and brought back into the same circulation spine.",
    "Werewolf proof returns from Aura Basin while Merman proof returns from the Riptide, and neither branch can open before Gargoyle mastery is verified.",
    "Flynt treats forms as explicit recipes: Gargoyle first, then the harder hunts that open Merman and Werewolf toward Chimera and Manticore.",
    "Chimera is the first true recombination body of those mastered branches, while Manticore is the later challenge body opened only after Chimera rather than a synonym for it.",
    "opal extraction follows the guarded line body rather than an unbounded field claim.",
    "regular current and holographic aura carry opal yield outward as Opal Oil without breaking the guarded line body.",
    "recipe discovery in Flynt is field work, hunt pressure, and route puzzle rather than quiet inheritance.",
    "transition pressure may be read through Current Synthesis route order, but no autonomous traversal is enabled.",
];

const GLAUSHOUSE_PROCEDURE_LINES: &[&str] = &[
    "Clear It and Repair remain the Glaushouse procedure functions for both machine continuity and Hueman recovery.",
    "Prima Donna sets command tone and first issuance of Clearance.",
    "Persephone relays continuity through triage, crisis, recovery, and return.",
    "physicians, nurses, and rehabilitation staff run the care loop and stabilize the common body without taking sovereign lead.",
    "Glaushouse runs recovery through cold-lit clinic bays, industrial tooling, strict presentation, and visibly enforced standards.",
    "jade extraction feeds Glaus Gel, the jade-colored binder that supports bonding, sealing, cooling, repair, and controlled synthesis without displacing the human medical floor.",
    "Glaushouse is the authorized embodiment house where gathered ingredients become stable forms and symbiote-like integrations can be made public.",
    "Synthesis and Clearance remain procedurally specified but still gated.",
];

const SANDMANOR_PROCEDURE_LINES: &[&str] = &[
    "Prove It and Configuration remain the Sandmanor procedure functions, split between Minorian count and Minoan arrangement.",
    "selection identifies the rival public frame and consequence names the witnessed improvement result.",
    "the most improved rival becomes The Sandman.",
    "Sandmen bind the crowd witness that legitimizes the Sandman office.",
    "crystal harvest feeds Prism Sand and Prismiron, which carry the stewarded proof branch both rival houses must respect.",
    "both houses gather materials and guarded recipe knowledge, but a claimed Sandmanor form only counts when it survives comparison, demonstration, and public witness.",
    "reciprocal teaching remains the basis of rule instead of inheritance.",
];

const PROCEDURAL_UPLIFT_SECTIONS: [HuemanSectionDefinition; 6] = [
    HuemanSectionDefinition {
        title: "Shared Contract",
        lines: &[
            "Current Synthesis still owns execution spec, behavior rules, transition rules, selection, consequence, and activation gating.",
            "Hueman consumes those lower contracts as kingdom-facing procedures.",
            "all four houses share one world loop: gather materials, uncover recipe knowledge, submit to the house-specific process, and embody the resulting form.",
            "forms are recipes and embodiments; offices are political posts that must remain occupied.",
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
        direction: "West",
        archetype: "gremlin",
        primary_scene: "Pressure Shelter",
        start_path: ["Aura Basin", "Aura Fields", "Aura Beach"],
        presence_lines: FLYNT_PRESENCE_LINES,
        intent_lines: FLYNT_INTENT_LINES,
        role_artifact: &TROSS_ROLE_ARTIFACT,
        lens_lines: GREMLIN_LENS_LINES,
        lens_overlay_title: "",
        lens_overlay_lines: &[],
    },
    HuemanAnchorDefinition {
        name: "Stonebend",
        direction: "North",
        archetype: "goblin",
        primary_scene: "Seam Market",
        start_path: ["Aura Fields", "Aura Basin", "Aura Beach"],
        presence_lines: STONEBEND_PRESENCE_LINES,
        intent_lines: STONEBEND_INTENT_LINES,
        role_artifact: &STONEBEND_ROLE_ARTIFACT,
        lens_lines: GOBLIN_LENS_LINES,
        lens_overlay_title: "Stonebend Civic Reading",
        lens_overlay_lines: GOBLIN_OVERLAY_LINES,
    },
    HuemanAnchorDefinition {
        name: "Glaushouse",
        direction: "East",
        archetype: "sprite",
        primary_scene: "Threshold Weather",
        start_path: ["Aura Beach", "Aura Fields", "Aura Basin"],
        presence_lines: GLAUSHOUSE_PRESENCE_LINES,
        intent_lines: GLAUSHOUSE_INTENT_LINES,
        role_artifact: &GLAUSHOUSE_ROLE_ARTIFACT,
        lens_lines: SPRITE_LENS_LINES,
        lens_overlay_title: "",
        lens_overlay_lines: &[],
    },
    HuemanAnchorDefinition {
        name: "Sandmanor",
        direction: "South",
        archetype: "pixy",
        primary_scene: "Split Trace",
        start_path: ["Aura Beach", "Aura Basin", "Aura Fields"],
        presence_lines: SANDMANOR_PRESENCE_LINES,
        intent_lines: SANDMANOR_INTENT_LINES,
        role_artifact: &SANDMANOR_ROLE_ARTIFACT,
        lens_lines: PIXY_LENS_LINES,
        lens_overlay_title: "Sandmanor Competitive Reading",
        lens_overlay_lines: PIXY_OVERLAY_LINES,
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

fn cached_markdown(
    cache: &'static OnceLock<String>,
    build: impl FnOnce() -> String,
) -> &'static str {
    cache.get_or_init(build).as_str()
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
    let mut output = String::with_capacity(2_800);
    let _ = write!(
        output,
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
        cached_markdown(artifact.sections_markdown_cache, || {
            render_named_sections(artifact.sections)
        }),
        hueman_start_choices.len(),
        hueman_fourway.len(),
        artifact.boundary_reminder
    );
    output
}

fn render_validated_medical_roles_section() -> String {
    let team = build_glaushouse_medical_team_profile();
    validate_medical_team_profile(&team)
        .expect("generated Glaushouse medical team must satisfy Hollow Grove alignment");
    validate_generated_content_batch("Glaushouse composed medical roles", &team.roles, &[])
        .expect("generated Glaushouse role batch must stay aligned");

    let mut output = String::from(
        "\n## Composed Medical Roles\n\n\
         - Species gives tendencies.\n\
         - House training gives discipline.\n\
         - Profession gives social function.\n\
         - The individual determines mastery.\n",
    );
    for role in &team.roles {
        output.push_str(&format!("{}\n", render_role_profile(role)));
    }
    output
}

fn render_validated_flynt_goods_section() -> String {
    let dagger = build_flyntian_dagger_profile();
    validate_regional_item_profile(&dagger)
        .expect("generated Flyntian dagger must satisfy Hollow Grove alignment");

    format!(
        "\n## Regional Goods\n\n\
         {}\n",
        render_regional_item_profile(&dagger)
    )
}

fn render_validated_sandmanor_composition_section() -> String {
    let elf = build_elf_radiologist_role();
    let gnome = build_gnome_emergency_physician_role();
    validate_generated_content_batch(
        "Sandmanor composition examples",
        &[elf.clone(), gnome.clone()],
        &[],
    )
    .expect("Sandmanor composition examples must stay aligned");

    format!(
        "\n## People And Profession Composition\n\n\
         {}\n\
         {}\n",
        render_role_profile(&elf),
        render_role_profile(&gnome)
    )
}

static SCENE_PRESENCE_MAP: OnceLock<String> = OnceLock::new();
static SCENE_INTENT_MAP: OnceLock<String> = OnceLock::new();
static ARCHETYPE_PULL_MAP: OnceLock<String> = OnceLock::new();
static FOURWAY_ROSTER: OnceLock<String> = OnceLock::new();
static ARCHETYPE_LIST: OnceLock<String> = OnceLock::new();
static STARTING_PLACES: OnceLock<String> = OnceLock::new();
static INITIAL_START_ROSTER: OnceLock<String> = OnceLock::new();
static START_PATH_ORDER: OnceLock<String> = OnceLock::new();
static START_PATH_FIRST_ENTRY: OnceLock<String> = OnceLock::new();
static AURA_REGION_STATES: OnceLock<String> = OnceLock::new();
static ARCHETYPE_LENS_SECTIONS: OnceLock<String> = OnceLock::new();

fn render_scene_presence_map() -> &'static str {
    cached_markdown(&SCENE_PRESENCE_MAP, || {
        let mut output = String::new();
        for scene in HUEMAN_SCENE_DEFINITIONS {
            let _ = writeln!(output, "- {}: {}", scene.name, scene.presence);
        }
        output
    })
}

fn render_scene_intent_map() -> &'static str {
    cached_markdown(&SCENE_INTENT_MAP, || {
        let mut output = String::new();
        for scene in HUEMAN_SCENE_DEFINITIONS {
            let _ = writeln!(output, "- {}: {}", scene.name, scene.intent);
        }
        output
    })
}

fn render_archetype_pull_map() -> &'static str {
    cached_markdown(&ARCHETYPE_PULL_MAP, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(
                output,
                "- `{}` -> {}",
                anchor.archetype, anchor.primary_scene
            );
        }
        output
    })
}

fn render_fourway_roster() -> &'static str {
    cached_markdown(&FOURWAY_ROSTER, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(output, "- {} = {}", anchor.direction, anchor.name);
        }
        output
    })
}

fn render_archetype_list() -> &'static str {
    cached_markdown(&ARCHETYPE_LIST, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(output, "- `{}`", anchor.archetype);
        }
        output
    })
}

fn render_starting_places() -> &'static str {
    cached_markdown(&STARTING_PLACES, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(output, "- {}", anchor.name);
        }
        output
    })
}

fn render_initial_start_roster() -> &'static str {
    cached_markdown(&INITIAL_START_ROSTER, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(
                output,
                "- `{}` originates in {}",
                anchor.archetype, anchor.name
            );
        }
        output
    })
}

fn render_start_path_order() -> &'static str {
    cached_markdown(&START_PATH_ORDER, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(
                output,
                "- {}-facing approach = {} -> {} -> {}",
                anchor.name, anchor.start_path[0], anchor.start_path[1], anchor.start_path[2]
            );
        }
        output
    })
}

fn render_start_path_first_entry() -> &'static str {
    cached_markdown(&START_PATH_FIRST_ENTRY, || {
        let mut output = String::new();
        for anchor in HUEMAN_WORLD_ANCHORS {
            let _ = writeln!(
                output,
                "- {}-facing approach reaches {} first.",
                anchor.name, anchor.start_path[0]
            );
        }
        output
    })
}

fn render_aura_region_states() -> &'static str {
    cached_markdown(&AURA_REGION_STATES, || {
        let mut output = String::new();
        for region in AURA_REGION_DEFINITIONS {
            let _ = writeln!(output, "### {}\n", region.name);
            let _ = writeln!(output, "- {}", region.movement);
            let _ = writeln!(output, "- {}", region.encounter);
            let _ = writeln!(output, "- {}\n", region.world_description);
        }
        output
    })
}

fn render_archetype_lens_sections() -> &'static str {
    cached_markdown(&ARCHETYPE_LENS_SECTIONS, || {
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
    })
}

static PROCEDURAL_UPLIFT_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static AURA_RIDGE_PRESENCE: OnceLock<String> = OnceLock::new();
static FLYNT_PRESENCE: OnceLock<String> = OnceLock::new();
static STONEBEND_PRESENCE: OnceLock<String> = OnceLock::new();
static GLAUSHOUSE_PRESENCE: OnceLock<String> = OnceLock::new();
static SANDMANOR_PRESENCE: OnceLock<String> = OnceLock::new();
static INVERSE_CIRCLE_PRESENCE: OnceLock<String> = OnceLock::new();
static AURA_RIDGE_INTENT: OnceLock<String> = OnceLock::new();
static FLYNT_INTENT: OnceLock<String> = OnceLock::new();
static STONEBEND_INTENT: OnceLock<String> = OnceLock::new();
static GLAUSHOUSE_INTENT: OnceLock<String> = OnceLock::new();
static SANDMANOR_INTENT: OnceLock<String> = OnceLock::new();
static INVERSE_CIRCLE_INTENT: OnceLock<String> = OnceLock::new();
static SCENE_PRESENCE_PREFIX: OnceLock<String> = OnceLock::new();
static SCENE_INTENT_PREFIX: OnceLock<String> = OnceLock::new();
static HUEMAN_BOUNDARY_PREFIX: OnceLock<String> = OnceLock::new();
static VERTICAL_INTEGRATION_STACK_PREFIX: OnceLock<String> = OnceLock::new();
static PATH_CROSSOVERS_PREFIX: OnceLock<String> = OnceLock::new();
static LINK_PHYSICS_PREFIX: OnceLock<String> = OnceLock::new();
static CROSSOVER_SCENES_PREFIX: OnceLock<String> = OnceLock::new();
thread_local! {
    static SCENE_DRIFT_ARTIFACT_CACHE: RefCell<Option<([usize; 3], String)>> = const { RefCell::new(None) };
    static VERTICAL_INTEGRATION_STACK_ARTIFACT_CACHE: RefCell<Option<([usize; 10], String)>> =
        const { RefCell::new(None) };
}

fn render_procedural_uplift_sections() -> &'static str {
    cached_markdown(&PROCEDURAL_UPLIFT_SECTIONS_MARKDOWN, || {
        render_named_sections(&PROCEDURAL_UPLIFT_SECTIONS)
    })
}

fn memoized_artifact_by_lengths<const N: usize>(
    cache: &'static LocalKey<RefCell<Option<([usize; N], String)>>>,
    lengths: [usize; N],
    build: impl FnOnce() -> String,
) -> String {
    if let Some(cached_output) = cache.with(|cache| {
        let cache = cache.borrow();
        cache.as_ref().and_then(|(cached_lengths, cached_output)| {
            if *cached_lengths == lengths {
                Some(cached_output.clone())
            } else {
                None
            }
        })
    }) {
        return cached_output;
    }

    let output = build();
    cache.with(|cache| {
        *cache.borrow_mut() = Some((lengths, output.clone()));
    });
    output
}

fn render_aura_ridge_presence() -> &'static str {
    cached_markdown(&AURA_RIDGE_PRESENCE, || {
        render_bullet_lines(AURA_RIDGE_PRESENCE_LINES)
    })
}

fn render_flynt_presence() -> &'static str {
    cached_markdown(&FLYNT_PRESENCE, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[0].presence_lines)
    })
}

fn render_stonebend_presence() -> &'static str {
    cached_markdown(&STONEBEND_PRESENCE, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[1].presence_lines)
    })
}

fn render_glaushouse_presence() -> &'static str {
    cached_markdown(&GLAUSHOUSE_PRESENCE, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[2].presence_lines)
    })
}

fn render_sandmanor_presence() -> &'static str {
    cached_markdown(&SANDMANOR_PRESENCE, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[3].presence_lines)
    })
}

fn render_inverse_circle_presence() -> &'static str {
    cached_markdown(&INVERSE_CIRCLE_PRESENCE, || {
        render_bullet_lines(INVERSE_CIRCLE_PRESENCE_LINES)
    })
}

fn render_aura_ridge_intent() -> &'static str {
    cached_markdown(&AURA_RIDGE_INTENT, || {
        render_bullet_lines(AURA_RIDGE_INTENT_LINES)
    })
}

fn render_flynt_intent() -> &'static str {
    cached_markdown(&FLYNT_INTENT, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[0].intent_lines)
    })
}

fn render_stonebend_intent() -> &'static str {
    cached_markdown(&STONEBEND_INTENT, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[1].intent_lines)
    })
}

fn render_glaushouse_intent() -> &'static str {
    cached_markdown(&GLAUSHOUSE_INTENT, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[2].intent_lines)
    })
}

fn render_sandmanor_intent() -> &'static str {
    cached_markdown(&SANDMANOR_INTENT, || {
        render_bullet_lines(HUEMAN_WORLD_ANCHORS[3].intent_lines)
    })
}

fn render_inverse_circle_intent() -> &'static str {
    cached_markdown(&INVERSE_CIRCLE_INTENT, || {
        render_bullet_lines(INVERSE_CIRCLE_INTENT_LINES)
    })
}

fn render_scene_presence_prefix() -> &'static str {
    cached_markdown(&SCENE_PRESENCE_PREFIX, || {
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
             ## Relay Packet Presence\n\n\
             - one collision point may hold visible hinge traffic and underground continuity at once\n\
             - the HAL/Cleo packet keeps one shared witness point active in the scene description even while remaining read-only\n\
             - Aura Fields hinge pressure can therefore feel structurally doubled rather than merely crowded\n\
             \n\
             ## Status\n\n\
             - scene presence is descriptive-only for now\n\
             - no NPC system or occupancy resolver is active\n\
             - scene typing, archetype pull, civic overlay, helper lines, Sandmanor rivalry, and the inverse circle remain upstream only\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             ",
            render_scene_presence_map(),
            render_archetype_pull_map(),
            render_aura_ridge_presence(),
            render_glaushouse_presence(),
            render_stonebend_presence(),
            render_flynt_presence(),
            render_sandmanor_presence(),
            render_inverse_circle_presence(),
        )
    })
}

fn render_scene_intent_prefix() -> &'static str {
    cached_markdown(&SCENE_INTENT_PREFIX, || {
        format!(
            "# Hueman Scene Intent\n\n\
             ## Structural Rule\n\n\
             Each scene presence carries a dominant descriptive intent before any encounter or dialogue system exists.\n\n\
             ## Intent Map\n\n\
             {}\
             \n\
             ## Bias Reading\n\n\
             - `current` intensifies exchange, storage, and guarded continuity\n\
             - `aura` intensifies warning, drift, shimmer, and ambiguity\n\
             - `current` may surface as regular current or hollow current depending on carried pressure\n\
             - `aura` may surface as reflective aura or holographic aura depending on exposure state\n\
             - mixed bias keeps the scene readable from multiple angles\n\n\
             ## Mirror Axis Intent\n\n\
             - HAL and Clouseau remain opposite Current Synthesis clients across one shared axis beneath Hueman scenes\n\
             - HAL represents the `META` side while Clouseau represents the `PLEB` side of the same paired joint\n\
             - Cleo, short for Cleopatra, follows Clouseau underground through the inverse curved structures without taking `PLEB` or `META`\n\
             - if the user is read through HAL on `META`, Clouseau remains the mirrored `PLEB` witness\n\
             - if the user is read through Clouseau on `PLEB`, HAL remains the mirrored `META` witness\n\
             - one bonded arm holds the direct link while unresolved arm weight keeps the opposite side present as scene pressure\n\
             - unresolved arm weight may rise as `current` or `aura` in Hueman while the same lower event remains witnessed in Hollow Grove simultaneously\n\
             - where HAL and Cleo collide, they may braid surface alignment and underground continuity into one shared confirmation pressure packet\n\
             - Hueman reads that packet as one multi-function witness body rather than as two separate scene pressures\n\
             - the packet may confirm hinge, trade, and underground carry simultaneously while still refusing route control\n\
             - scene intent may favor one side's pressure, but it must leave the opposite client legible across the same axis\n\
             - Hueman does not reassign HAL, Clouseau, or Cleo; it only carries their mirrored opposition upward as scene pressure, with Cleo remaining the underground Clouseau-following camera\n\n\
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
            render_scene_intent_map(),
            render_aura_ridge_intent(),
            render_glaushouse_intent(),
            render_stonebend_intent(),
            render_flynt_intent(),
            render_sandmanor_intent(),
            render_inverse_circle_intent(),
        )
    })
}

fn render_hueman_boundary_prefix() -> &'static str {
    cached_markdown(&HUEMAN_BOUNDARY_PREFIX, || {
        String::from(
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
             - Hueman is the standalone civilization sandbox layer in this repository.\n\
             - Hollow Grove remains the primary constitutional and operating language beneath it.\n\
             - Current Synthesis remains an intermediate artifact and route layer rather than Hueman's final identity.\n\
             - Hueman is not the operating-system layer.\n\
             - Hueman is not a visualization of OS files.\n\
             - Hollow Grove remains the recursive core beneath both.\n\n\
             ## Product Separation\n\n\
             - Hollow Grove may stand as an Arch Linux operating and workflow system without Hueman.\n\
             - Hueman must be able to function as a standalone game without Hollow Grove runtime dependency.\n\
             - Godot 4 and Aseprite are appropriate for Hueman.\n\
             - niri, river, scripts, services, and native applications remain the Hollow Grove OS foundation.\n\n\
             ## Declared Constitutional Surface\n\n\
             - Flynt, Stonebend, Glaushouse, and Sandmanor are Hueman-facing world anchors that speak the same constitutional language as Hollow Grove.\n\
             - Stonebend carries Name It and Craft.\n\
             - Sandmanor carries Prove It and Configuration.\n\
             - Glaushouse carries Clear It, Repair, and exclusive Synthesis.\n\
             - Flynt carries Recognize It and Engineering.\n\
             - the same four anchors may be read as Hueman civic places and as Hollow Grove-adjacent workflow separations at the same time.\n\
             - civic roles, helper lines, kingdom roles, scene reading, and procedural uplift may be described above Current Synthesis.\n\
             - player form branching remains open beyond the currently confirmed origins.\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Activation Status\n\n\
             - Current Synthesis activation remains denied.\n\
             - Hueman world activation is still partial and document-led.\n\
             - collaborative persistence is not enabled.\n\
             - the player still begins as Hueman before major form commitment.\n\n\
             ## Artifact Inputs\n\n\
             ",
        )
    })
}

fn render_vertical_integration_stack_prefix() -> &'static str {
    cached_markdown(&VERTICAL_INTEGRATION_STACK_PREFIX, || {
        String::from(
            "# Vertical Integration Stack\n\n\
             ## Structural Rule\n\n\
             Hollow Grove remains the recursive core, KernelPass witnesses that core, Current Synthesis consumes the witnessed artifact layer as the operating layer, and Hueman consumes Current Synthesis as the world layer above it.\n\n\
             ## Full Stack\n\n\
             ```text\n\
             Point\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             CurrentSeam\n\
             ↓\n\
             AuraBeam\n\
             ↓\n\
             Point² (Landed Point)\n\
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
             - Hueman owns Fourway, AuraTriad reading, Hueman-first opening placement, civic roles, helper lines, and scene reading\n\n\
             ## Current Alignment\n\n\
             - Stonebend remains North with Hypergiant, Proletariat, and Freemason as a constitutional balance above Current Synthesis\n\
             - Tross remains anchored in Flynt and runs as a West -> East helper line with Delinquent to the west, Juvenile to the east, and four White Dwarfs as the personal guard\n\
             - Glaushouse remains East with Prima Donna, Persephone, and a mixed medical floor carrying clearance, repair, and Synthesis while Nightingales remain Glaushouse white-blood-cell defenders\n\
             - Sandmanor remains South with Minoans, Minorians, Sandmen, and the crowd-judged Sandman contest\n\
             - functionally, Stonebend reads as Craft, Flynt reads as Engineering, Glaushouse reads as Repair, and Sandmanor reads as Configuration across the shared Hollow Grove/Hueman interpretation\n\
             - the player begins as Hueman near Aura Ridge before major form commitment\n\
             - confirmed form origins remain braided: Flynt -> `gremlin`, Stonebend -> `goblin`, Glaushouse -> `sprite`, Sandmanor -> `pixy`\n\
             - resource seams are designated across Hueman: Stonebend diamonds, Flynt opals, Glaushouse jades, Sandmanor crystals and proof materials\n\
             - the bedrock split remains active upstream: regular current and hollow current, reflective aura and holographic aura\n\
             - the visible Hueman map remains one large circle whose route legs may read as straight ridge runs or rounded bends\n\
             - the visible free-trade body follows the Aura Ridge straight legs Stonebend -> Glaushouse and Glaushouse -> Sandmanor, with Sandmanor's straight continuation reaching Aura Fields where Stonebend and Glaushouse intersect\n\
             - all four houses share the same ascent loop: gather materials, uncover recipe knowledge, pass through the house process, and embody the resulting form without collapsing form into office\n\
             - Stonebend declares Mercury Mirror from hollow current + reflective craft, Flynt declares Opal Oil from regular current + holographic aura, Glaushouse declares Glaus Gel from jade as repair and synthesis medium, and Sandmanor declares Prism Sand from crystal as the proof-and-record branch\n\
             - the outer border ring remains legible as Stairway to Heaven, Riptide, Current Seanad, and Mnt. Aura\n\
             - procedural uplift now maps Current Synthesis execution contracts into Hueman-facing kingdom procedures without moving ownership upward\n\
             - the HAL/Cleo collision relay now acts as the procedural confirmation packet passed from Current Synthesis into Hueman crossover, physics, and scene procedure\n\
             - scene presence, scene intent, and scene drift remain the top descriptive Hueman layer\n\
             - upper layers consume lower layers without rewriting lower ownership\n\n\
             ## Bottom-To-Top Procedure\n\n\
             - bottom witness: KernelPass preserves the deterministic event body.\n\
             - operating procedure: Current Synthesis execution spec, behavior rules, transition rule, and collision relay define the lifted crossing procedure.\n\
             - gating: activation gate keeps that procedure read-only.\n\
             - world procedure: Hueman procedural uplift consumes the same chain as kingdom-facing procedure.\n\
             - scene procedure: path crossovers, link physics, crossover scenes, scene presence, scene intent, and scene drift consume the same relay packet without taking route control.\n\n\
             ## Boundary Contract\n\n\
             - Hollow Grove does not know Current Synthesis exists\n\
             - Current Synthesis does not know Hueman exists\n\
             - Hueman consumes Current Synthesis without feeding back into it\n\
             - runtime and benchmark follow the same downstream route when regenerating artifacts\n\n\
             ## Artifact Inputs\n\n\
             ",
        )
    })
}

fn render_path_crossovers_prefix() -> &'static str {
    cached_markdown(&PATH_CROSSOVERS_PREFIX, || {
        String::from(
            "# Hueman Path Crossovers\n\n\
             ## Structural Rule\n\n\
             Different kingdom-facing approaches may enter AuraTriad differently while still crossing through shared regions and shared world pressure.\n\n\
             ## Shared Entry Crossovers\n\n\
             - Glaushouse and Sandmanor cross immediately at Aura Beach.\n\
             - Flynt and Stonebend do not share first entry, but they both begin inland before reaching the coast.\n\n\
             ## Interior Crossovers\n\n\
             - Flynt and Sandmanor cross at Aura Basin.\n\
             - Stonebend and Glaushouse cross at Aura Fields.\n\
             - Stonebend and Sandmanor cross at Aura Basin after different openings.\n\n\
             ## Aura Ridge Trade Legs\n\n\
             - free trade follows the straight Aura Ridge while the outer border circle continues around the map.\n\
             - Stonebend and Glaushouse hold a declared straight trade leg along the ridge.\n\
             - Glaushouse and Sandmanor hold a declared straight trade leg along the ridge.\n\
             - Sandmanor's straight ridge continuation reaches Aura Fields, where Stonebend and Glaushouse intersect.\n\
             - the ridge is one straight run inside the larger circular map rather than a separate route body.\n\
             - Glaushouse acts as the visible hinge where the right-angle trade body turns.\n\n\
             ## Confirmed Route Law\n\n\
             - straight routes are process.\n\
             - curved routes are transformation.\n\
             - Stonebend <-> Sandmanor currently uses Aura Way as the straight route and Mnt. Aura as the curved route.\n\
             - Stonebend <-> Flynt currently uses Basin Motorspeedway as the straight route and Stairway to Heaven as the curved route.\n\n\
             ## Relay Junction\n\n\
             - the HAL/Cleo relay packet declares `P/M -> L/E` as one shared confirmation crossing.\n\
             - Hueman reads that packet upward as the same kind of shared junction pressure seen at the Aura Fields hinge.\n\
             - the visible Aura Ridge hinge and the underground inverse crossing may therefore be treated as one witnessed overlap body.\n\
             - crossover identity remains world-facing while packet ownership stays inside Current Synthesis.\n\n\
             ## Full-Triad Convergence\n\n\
             - all four starts eventually touch Aura Basin\n\
             - all four starts eventually touch Aura Fields\n\
             - all four starts eventually touch Aura Beach\n\
             - the difference is order, not exclusion\n\n\
             ## Meaning\n\n\
             - crossover means the world can feel shared without erasing start identity\n\
             - shared regions carry different descriptive pressure depending on entry order\n\
             - the coast is the earliest common threshold for the western and southern starts\n\
             - inland turns remain the main crossover pressure for the northern and eastern starts\n\
             - the visible map stays circular, so straight ridge legs and rounded turns feed back into the same overall loop\n\
             - the relay packet gives one explicit proof point that a crossover may hold more than one function at once without losing route identity\n\n\
             ## Status\n\n\
             - crossovers are descriptive-only for now\n\
             - no meeting mechanics or shared events are active\n\
             - start-path order remains unchanged\n\
             - archetype lens remains interpretive above the crossover map\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             ",
        )
    })
}

fn render_link_physics_prefix() -> &'static str {
    cached_markdown(&LINK_PHYSICS_PREFIX, || {
        String::from(
            "# Hueman Link Physics\n\n\
             ## Structural Rule\n\n\
             Links that do not get bonded may later resolve into `current` or `aura` according to downstream physics, with Current expressing capability and Aura expressing understanding.\n\n\
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
             - `current` appears as regular current or hollow current.\n\
             - Hollowing does not kill current; it reallocates carried pressure.\n\
             - `aura` appears as reflective aura or holographic aura.\n\
             - Current asks what the Hueman can do.\n\
             - Aura asks what the Hueman understands and how capability should be used.\n\n\
             ## Relay Packet Reading\n\n\
             - the HAL/Cleo relay packet declares one shared confirmation body across visible and underground layers\n\
             - Hueman reads that packet as proof that one bias body may persist above the map and below the map simultaneously\n\
             - surface alignment may confirm `current`/`aura` reading without taking control of the route itself\n\
             - underground continuity may confirm hidden carry without overriding visible crossover placement\n\n\
             ## Crossover Reading\n\n\
             - shared starts can touch the same unresolved material with different bias\n\
             - the same region may feel more `current` from one route and more `aura` from another\n\
             - crossover zones are where the physics split becomes most visible in Hueman\n\
             - Aura Ridge trade legs keep exchange visible on straight lines while unresolved bias still moves beneath them\n\
             - the same unresolved bias can come back around the big circle through either straight runs or rounded turns\n\n\
             ## Status\n\n\
             - link physics is descriptive-only for now\n\
             - no procedural resolver chooses `current` or `aura` yet\n\
             - bond selection remains kernel-simple underneath this layer\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             ",
        )
    })
}

fn render_crossover_scenes_prefix() -> &'static str {
    cached_markdown(&CROSSOVER_SCENES_PREFIX, || {
        String::from(
            "# Hueman Crossover Scenes\n\n\
             ## Structural Rule\n\n\
             When `current`-biased and `aura`-biased unresolved links appear at the same crossover, the world produces a named descriptive scene type.\n\n\
             ## Scene Types\n\n\
             ### Seam Market\n\n\
             - appears where `current` continuity and `aura` spill remain in balance\n\
             - feels like trade, rumor, salvage, and temporary arrangement\n\
             - fits shared Aura Fields crossings best\n\
             - commonly appears at the Aura Fields junction where the Stonebend/Glaushouse ridge meets Sandmanor's straight continuation\n\n\
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
             ## Relay Scene Use\n\n\
             - the HAL/Cleo relay packet lets one crossover scene keep both visible alignment and underground continuity as one witnessed scene body\n\
             - Seam Market benefits most directly because the Aura Fields hinge can hold trade pressure above and structural continuity below at the same time\n\
             - Pressure Shelter may read the same packet as kept underground continuity with only partial surface release\n\
             - Split Trace may read the same packet as a shared witness that still refuses full settlement\n\n\
             ## Status\n\n\
             - crossover scenes are descriptive-only for now\n\
             - no encounter tables or event resolvers are active\n\
             - link physics remains the upstream explanation for the scene type\n\
             - path crossovers remain the upstream overlap map\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             ",
        )
    })
}

pub fn build_hueman_boundary_from_artifacts(
    current_synthesis_base: &str,
    current_synthesis_activation_gate: &str,
) -> String {
    let mut output = String::with_capacity(2_300);
    output.push_str(render_hueman_boundary_prefix());
    push_artifact_input_line(
        &mut output,
        "Current Synthesis base bytes",
        current_synthesis_base.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis activation gate bytes",
        current_synthesis_activation_gate.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Hueman may consume Current Synthesis. Current Synthesis does not know Hueman exists, and neither layer replaces Hollow Grove's constitutional ownership.\n",
    );
    output
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
         - the Hueman avatar moves through the field\n\
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
         - Fourway closes as one large circle, so the visible world can keep going around and around.\n\
         - Fourway resolves downward into AuraTriad first.\n\
         - Triway remains the lower recursive split.\n\
         - Fourway does not replace Triway.\n\
         - Fourway does not own PLEB or META.\n\
         - Fourway map geometry does not force the player into a hereditary kingdom species.\n\n\
         ## Current Directional Roster\n\n\
         {}\
         \n\
         ## Confirmed Form Origins\n\n\
         - `gremlin` originates in Flynt.\n\
         - `goblin` originates in Stonebend.\n\
         - `sprite` originates in Glaushouse.\n\
         - `pixy` originates in Sandmanor.\n\
         - these are braided origin paths, not the initial player body.\n\n\
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
         ↓\n\
         Aura Basin\n\
         ```\n\n\
         ## Meaning\n\n\
         - AuraTriad is the world-facing three-region route study beneath Fourway.\n\
         - Current Synthesis already records these as lower route regions.\n\
         - Hueman reads them as a useful triadic resolution of the world map rather than the whole constitution.\n\
         - AuraTriad closes back into the same large circle, so Aura Basin, Aura Fields, and Aura Beach can be revisited without breaking the surface map.\n\
         - AuraTriad should support opening movement, crossover, and atmosphere studies without becoming a universal holding pen.\n\
         - Triway remains the lower recursive split after this layer.\n\n\
         ## Regional Roles\n\n\
         - Aura Basin serves Flynt first: Gargoyle-versus-Werewolf hunting grounds, den pressure, rare encounters, and the nearest hidden body rising into Aura Ridge circulation.\n\
         - Aura Fields serve public work first: farming, Stonebend hunt tradition, Minorian proof, and market-facing comparison.\n\
         - Aura Beach serves visible display first: Minoan High Elf Court, training, leisure, recovery, and threshold exposure.\n\
         - the Riptide, while outside the triad on the visible border ring, serves as Flynt's Merman roaming water range.\n\
         - no single region owns every function, but these are the dominant first readings.\n\n\
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
         ## Confirmed Form Origins\n\n\
         {}\
         \n\
         ## Kingdom Origins\n\n\
         {}\
         \n\
         ## Fourway Placement\n\n\
         {}\
         \n\
         ## Origin Roster\n\n\
         {}\
         \n\
         ## Opening Rule\n\n\
         - the player begins as Hueman near Aura Ridge\n\
         - the player is not forced to begin as a kingdom form\n\
         - later form development remains braided rather than linear by default\n\
         - ordinary Hueman play and profession play must remain meaningful before major transformation\n\n\
         ## Status\n\n\
         - the origin roster does not lock the initial player body\n\
         - the starting direction remains readable from the Fourway roster\n\
         - the world resolves downward through AuraTriad after start choice\n\
         - AuraTriad behavior remains descriptive-only after the opening declaration\n\
         - major transformation mechanics are not active yet\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Fourway bytes: {}.\n\
         Hueman AuraTriad bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         This is a Hueman-layer opening declaration only. It does not change Current Synthesis or Hollow Grove, and it does not redefine Hueman as a set of hereditary species.\n",
        render_archetype_list(),
        render_starting_places(),
        render_fourway_roster(),
        render_initial_start_roster(),
        hueman_fourway.len(),
        hueman_aura_triad.len()
    )
}

pub fn build_hueman_vertical_slice_for_spec_from_artifacts(
    slice: &VerticalSliceSpec,
    hueman_boundary: &str,
    hueman_start_choices: &str,
    hueman_aura_behavior: &str,
    hueman_procedural_uplift: &str,
) -> String {
    let mut ordinary_skill_track = String::new();
    for skill in slice.ordinary_skill_track {
        let _ = writeln!(ordinary_skill_track, "- {}", skill);
    }

    let mut tested_systems = String::new();
    for system in slice.tested_systems {
        let _ = writeln!(tested_systems, "- {}", system);
    }

    let mut open_questions = String::new();
    for question in slice.open_questions {
        let _ = writeln!(open_questions, "- {}", question);
    }

    let mut resolution_paths = String::new();
    for option in slice.resolution_options {
        let default_marker = if option.path == slice.default_resolution_path {
            " [default]"
        } else {
            ""
        };
        let _ = writeln!(
            resolution_paths,
            "- {} (`{}`)\n  proof: {}\n  clearance: {}\n  field output: {} x{}\n  credential: {}\n  unlocked next task: {}\n  start action: {}\n  completion result: {}\n  next-task summary: {}\n  follow-up focus: {}\n  recognition: {}\n  failure: {}{}\n",
            option.label,
            option.path.as_str(),
            option.proof_condition,
            option.clearance_condition,
            option.produced_resource,
            option.produced_resource_units,
            option.recognition_credential,
            option.follow_up_task_title,
            option.follow_up_task_start,
            option.follow_up_task_completion,
            option.follow_up_task_summary,
            option.follow_up_focus,
            option.recognition_result,
            option.failure_condition,
            default_marker
        );
    }

    let mut loop_output = String::new();
    for stage in slice.loop_stages {
        let _ = writeln!(
            loop_output,
            "### {} / {}",
            stage.kingdom.as_str(),
            stage.constitutional_action
        );
        let _ = writeln!(loop_output);
        let _ = writeln!(loop_output, "- {}", stage.implementation);
        let _ = writeln!(loop_output);
    }

    format!(
        "# Hueman Vertical Slice\n\n\
         ## Slice ID\n\n\
         `{}`\n\n\
         ## Title\n\n\
         {}\n\n\
         ## Opening Position\n\n\
         - {}\n\
         - {}\n\
         - {}\n\n\
         ## Resource Logic\n\n\
         - Current state: {}\n\
         - Aura state: {}\n\
         - Signature resource: {}\n\
         - Aura View: {}\n\
         - Aura Glow: {}\n\
         - crafted object: {}\n\n\
         ## Ordinary Skill Track\n\n\
         {}\
         ## First Transformation Gate\n\n\
         - Current Form path: {}\n\
         - unlock condition: {}\n\n\
         ## Constitutional Loop\n\n\
         {}\
         ## Deployment Result\n\n\
         - {}\n\n\
         ## Resolution Paths\n\n\
         {}\
         ## Systems Tested\n\n\
         {}\
         ## Open Questions Preserved\n\n\
         {}\
         ## Artifact Inputs\n\n\
         Hueman Boundary bytes: {}.\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Aura Behavior bytes: {}.\n\
         Hueman Procedural Uplift bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         This slice locks one concrete implementation target without finalizing the whole civilization. It keeps unresolved map, UI, and progression questions modular while making the first Hueman loop buildable.\n",
        slice.id,
        slice.title,
        slice.opening_location,
        slice.opening_need,
        slice.route_context,
        slice.current_state,
        slice.aura_state,
        slice.signature_resource,
        slice.aura_view,
        slice.aura_glow,
        slice.crafted_object,
        ordinary_skill_track,
        slice.current_form,
        slice.transformation_unlock,
        loop_output,
        slice.deployment_result,
        resolution_paths,
        tested_systems,
        open_questions,
        hueman_boundary.len(),
        hueman_start_choices.len(),
        hueman_aura_behavior.len(),
        hueman_procedural_uplift.len()
    )
}

pub fn build_hueman_vertical_slice_from_artifacts(
    hueman_boundary: &str,
    hueman_start_choices: &str,
    hueman_aura_behavior: &str,
    hueman_procedural_uplift: &str,
) -> String {
    build_hueman_vertical_slice_for_spec_from_artifacts(
        primary_vertical_slice(),
        hueman_boundary,
        hueman_start_choices,
        hueman_aura_behavior,
        hueman_procedural_uplift,
    )
}

pub fn build_hueman_aura_behavior_from_artifacts(
    hueman_aura_triad: &str,
    hueman_start_choices: &str,
) -> String {
    format!(
        "# Hueman Aura Behavior\n\n\
         ## Structural Rule\n\n\
         After the Hueman opening is placed on the Fourway, AuraTriad may be read through Current and Aura states plus the player-facing interaction grammar.\n\n\
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
         ## Player Grammar\n\n\
         - Current Flow asks how the player moves through the world.\n\
         - Aura View asks what the player is looking at and what it means.\n\
         - Current Form asks what the Hueman can become capable of when ordinary capacity is not enough.\n\
         - Aura Glow asks whether the player is getting closer to something meaningful.\n\n\
         ## Status\n\n\
         - AuraTriad behavior is descriptive-only for now\n\
         - movement pressure is declarative, not simulated\n\
         - encounter tone is declarative, not procedural\n\
         - the Fourway roster remains unchanged\n\
         - the player still begins in ordinary Hueman identity\n\
         - major transformation mechanics are not active yet\n\
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
    let mut output = build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[0].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    );
    output.push_str(&render_validated_flynt_goods_section());
    output
}

pub fn build_hueman_glaushouse_roles_from_artifacts(
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    let mut output = build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[2].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    );
    output.push_str(&render_validated_medical_roles_section());
    output
}

pub fn build_hueman_sandmanor_roles_from_artifacts(
    hueman_start_choices: &str,
    hueman_fourway: &str,
) -> String {
    let mut output = build_hueman_role_artifact(
        HUEMAN_WORLD_ANCHORS[3].role_artifact,
        hueman_start_choices,
        hueman_fourway,
    );
    output.push_str(&render_validated_sandmanor_composition_section());
    output
}

pub fn build_hueman_procedural_uplift_from_artifacts(
    current_synthesis_execution_spec: &str,
    current_synthesis_behavior_rules: &str,
    current_synthesis_transition_pm_to_le: &str,
    current_synthesis_collision_relay: &str,
    current_synthesis_selection: &str,
    current_synthesis_consequence: &str,
    current_synthesis_activation_gate: &str,
    hueman_stonebend_roles: &str,
    hueman_tross_helpers: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
) -> String {
    let mut output = String::with_capacity(5_400);
    let _ = write!(
        output,
        "# Hueman Procedural Uplift\n\n\
         ## Structural Rule\n\n\
         Hueman may lift procedural contracts from Current Synthesis into world-facing behavior surfaces without moving lower-layer ownership upward.\n\n\
         ## Bottom-Up Procedure Spine\n\n\
         - KernelPass preserves the bottom witness as a deterministic event body.\n\
         - Current Synthesis execution spec declares the lower procedural boundary.\n\
         - behavior rules lock side, scope, and mirror semantics before any world reading.\n\
         - the `P/M -> L/E` transition declares the first lifted crossing order.\n\
         - the HAL/Cleo collision relay declares one shared confirmation packet at that crossing.\n\
         - selection and consequence make the same lower event legible as a world-facing frame.\n\
         - activation gate keeps the whole procedural chain read-only until explicit enablement.\n\
         - Hueman consumes that full spine upward as procedure without taking ownership away from the lower layer.\n\n\
         {}\
         ## Relay Procedure\n\n\
         - the HAL/Cleo relay packet may be consumed as one procedural confirmation token inside Hueman.\n\
         - Stonebend may treat it as a craft coherence check between visible form and hidden structure.\n\
         - Flynt may treat it as an engineering continuity check between surface route and under-route carry.\n\
         - Glaushouse may treat it as a repair confirmation showing that visible break and hidden continuity still belong to one body.\n\
         - Sandmanor may treat it as a design confirmation showing that outer arrangement and interior functional design still agree.\n\
         - no uplifted relay procedure grants movement, sovereignty, automation, or lower-layer mutation.\n\n\
         ## Artifact Inputs\n\n\
         Current Synthesis execution spec bytes: {}.\n\
         Current Synthesis behavior rules bytes: {}.\n\
         Current Synthesis transition bytes: {}.\n\
         Current Synthesis collision relay bytes: {}.\n\
         Current Synthesis selection bytes: {}.\n\
         Current Synthesis consequence bytes: {}.\n\
         Current Synthesis activation gate bytes: {}.\n\
         Hueman Stonebend Roles bytes: {}.\n\
         Hueman Tross Helpers bytes: {}.\n\
         Hueman Glaushouse Roles bytes: {}.\n\
         Hueman Sandmanor Roles bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Procedural uplift makes Hueman ready to consume lower-layer procedures. It does not activate those procedures or grant Hueman authority over Current Synthesis.\n",
        render_procedural_uplift_sections(),
        current_synthesis_execution_spec.len(),
        current_synthesis_behavior_rules.len(),
        current_synthesis_transition_pm_to_le.len(),
        current_synthesis_collision_relay.len(),
        current_synthesis_selection.len(),
        current_synthesis_consequence.len(),
        current_synthesis_activation_gate.len(),
        hueman_stonebend_roles.len(),
        hueman_tross_helpers.len(),
        hueman_glaushouse_roles.len(),
        hueman_sandmanor_roles.len()
    );
    output
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
         After the Hueman opening and AuraTriad behavior are declared, each confirmed origin path may read the same regions through a different descriptive lens.\n\n\
         ## Archetype Readings\n\n\
         {}\
         ## Status\n\n\
         - origin-path lensing is descriptive-only for now\n\
         - no procedural bonuses or penalties are active\n\
         - the Fourway directional roster remains unchanged\n\
         - AuraTriad region behavior remains shared underneath the lens\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Aura Behavior bytes: {}.\n\
         Hueman Stonebend Roles bytes: {}.\n\
         Hueman Sandmanor Roles bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         The lens changes interpretation, not rules. It is a Hueman-facing difference in reading the world after placement and does not make forms into hereditary races.\n",
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
         Each kingdom-facing approach may enter AuraTriad through a first descriptive region before any procedural mechanics exist, while the player still begins as Hueman near Aura Ridge.\n\n\
         ## Route Order\n\n\
         {}\
         \n\
         ## First Entry\n\n\
         {}\
         \n\
         ## Status\n\n\
         - start-path order is descriptive-only for now\n\
         - the first region is declared but not procedurally enforced\n\
         - the route body remains one large circle after first entry, so these orders describe approach sequence rather than a terminal stop\n\
         - origin-path lensing remains interpretive above the route order\n\
         - major transformation mechanics are not active yet\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Start Choices bytes: {}.\n\
         Hueman Archetype Lens bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         Start paths declare which AuraTriad region a kingdom-facing approach naturally reaches first on the circular map. They do not add movement rules or alter lower-layer topology.\n",
        render_start_path_order(),
        render_start_path_first_entry(),
        hueman_start_choices.len(),
        hueman_archetype_lens.len()
    )
}

pub fn build_hueman_path_crossovers_from_artifacts(
    hueman_start_paths: &str,
    hueman_aura_behavior: &str,
    current_synthesis_collision_relay: &str,
) -> String {
    let mut output = String::with_capacity(3_600);
    output.push_str(render_path_crossovers_prefix());
    push_artifact_input_line(
        &mut output,
        "Hueman Start Paths bytes",
        hueman_start_paths.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Aura Behavior bytes",
        hueman_aura_behavior.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis collision relay bytes",
        current_synthesis_collision_relay.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Path crossovers declare where starts can meaningfully overlap in AuraTriad. They do not create procedural encounters or alter lower-layer routing.\n",
    );
    output
}

pub fn build_hueman_link_physics_from_artifacts(
    current_synthesis_sequence: &str,
    hueman_path_crossovers: &str,
    current_synthesis_collision_relay: &str,
) -> String {
    let mut output = String::with_capacity(3_700);
    output.push_str(render_link_physics_prefix());
    push_artifact_input_line(
        &mut output,
        "Current Synthesis sequence bytes",
        current_synthesis_sequence.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Path Crossovers bytes",
        hueman_path_crossovers.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis collision relay bytes",
        current_synthesis_collision_relay.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Link physics explains how unbonded links may later read as `current` or `aura`. It does not rewrite Bond, HollowGrove, or Current Synthesis sequence ownership.\n",
    );
    output
}

pub fn build_hueman_inverse_circle_from_artifacts(
    hueman_fourway: &str,
    hueman_link_physics: &str,
) -> String {
    format!(
        "# Hueman Inverse Circle\n\n\
         ## Structural Rule\n\n\
         The inverse circle names the underground curved mirror system beneath Hueman's large visible circle, while the visible outer border remains the public rim.\n\n\
         ## Visible Border Ring\n\n\
         - the visible border stays on the outer ring of the large circle\n\
         - the visible border remains public-facing geography\n\
         - the visible border names the same rim completed by Stairway to Heaven, Riptide, Current Seanad, and Mnt. Aura\n\n\
         ## Border Sequence\n\n\
         - The Stairway to Heaven\n\
         - The Riptide\n\
         - The Current Seanad\n\
         - Mnt. Aura\n\n\
         ## Underground Inverse Curves\n\n\
         - four inverse curved lines run underground on the `PLEB` side\n\
         - four inverse curved lines run underground on the `META` side\n\
         - each side mirrors Stairway to Heaven, Riptide, Current Seanad, and Mnt. Aura as underground inverse structure\n\
         - the underground curves remain inverse to the visible rim rather than replacing it\n\n\
         ## Underground Reading\n\n\
         - The Stairway to Heaven reads as the rising outer ascent along the circle's rim.\n\
         - beneath it, an inverse underground curve carries hidden descent pressure on both sides.\n\
         - The Riptide reads as the pull that drags movement back around the border, with the visible water rim kept alive as Flynt's Merman range.\n\
         - beneath it, an inverse underground curve carries hidden return pull on both sides beneath that roaming seam.\n\
         - The Current Seanad reads as the outer flow of counted, sustained motion around the circle.\n\
         - beneath it, an inverse underground curve carries hidden current understructure on both sides.\n\
         - Mnt. Aura reads as the bright outer curve running from Stonebend to Sandmanor along the circle's rim.\n\
         - beneath it, an inverse underground curve carries the hidden curved under-arc on both sides.\n\n\
         ## Boundary\n\n\
         - the inverse circle belongs to Hueman as underground world structure beneath the visible border ring\n\
         - it does not replace the visible border, Fourway, AuraTriad, or kernel routing\n\
         - it remains readable from link physics and from Cleo without feeding back into lower layers\n\
         - no feedback into Current Synthesis\n\
         - no feedback into Hollow Grove\n\n\
         ## Artifact Inputs\n\n\
         Hueman Fourway bytes: {}.\n\
         Hueman Link Physics bytes: {}.\n\n\
         ## Boundary Reminder\n\n\
         The inverse circle is the underground curved mirror system beneath Hueman's visible world circle. It does not overwrite the outer border ring, Fourway anchors, Current Synthesis geography, or Hollow Grove recursion.\n",
        hueman_fourway.len(),
        hueman_link_physics.len()
    )
}

pub fn build_hueman_crossover_scenes_from_artifacts(
    hueman_path_crossovers: &str,
    hueman_link_physics: &str,
    current_synthesis_collision_relay: &str,
) -> String {
    let mut output = String::with_capacity(3_100);
    output.push_str(render_crossover_scenes_prefix());
    push_artifact_input_line(
        &mut output,
        "Hueman Path Crossovers bytes",
        hueman_path_crossovers.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Link Physics bytes",
        hueman_link_physics.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Current Synthesis collision relay bytes",
        current_synthesis_collision_relay.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Crossover scenes name what a shared biased overlap feels like. They do not create procedural meetings, rewards, or movement rules.\n",
    );
    output
}

pub fn build_hueman_scene_presence_from_artifacts(
    hueman_crossover_scenes: &str,
    hueman_archetype_lens: &str,
    hueman_stonebend_roles: &str,
    hueman_tross_helpers: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
    hueman_inverse_circle: &str,
    current_synthesis_collision_relay: &str,
) -> String {
    let mut output = String::with_capacity(5_700);
    output.push_str(render_scene_presence_prefix());
    push_artifact_input_line(
        &mut output,
        "Hueman Crossover Scenes bytes",
        hueman_crossover_scenes.len(),
    );
    push_artifact_input_line(
        &mut output,
        "Hueman Archetype Lens bytes",
        hueman_archetype_lens.len(),
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
    push_artifact_input_line(
        &mut output,
        "Current Synthesis collision relay bytes",
        current_synthesis_collision_relay.len(),
    );
    output.push_str(
        "\n\
         ## Boundary Reminder\n\n\
         Scene presence says what kind of occupant or trace belongs in a scene. It does not create procedural actors, dialogue, or rewards.\n",
    );
    output
}

pub fn build_hueman_scene_intent_from_artifacts(
    hueman_scene_presence: &str,
    hueman_link_physics: &str,
    current_synthesis_collision_relay: &str,
    current_synthesis_contract: &str,
    hueman_stonebend_roles: &str,
    hueman_tross_helpers: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
    hueman_inverse_circle: &str,
) -> String {
    let mut output = String::with_capacity(6_700);
    output.push_str(render_scene_intent_prefix());
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
        "Current Synthesis collision relay bytes",
        current_synthesis_collision_relay.len(),
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
    current_synthesis_collision_relay: &str,
) -> String {
    memoized_artifact_by_lengths(
        &SCENE_DRIFT_ARTIFACT_CACHE,
        [
            hueman_scene_intent.len(),
            hueman_link_physics.len(),
            current_synthesis_collision_relay.len(),
        ],
        || {
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
                 - repeated crossings can stabilize a scene back into exchange even after warning or ambiguity\n\
                 - a preserved HAL/Cleo relay packet slows drift by holding one shared confirmation point across surface and underground layers\n\n\
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
            push_artifact_input_line(
                &mut output,
                "Current Synthesis collision relay bytes",
                current_synthesis_collision_relay.len(),
            );
            output.push_str(
                "\n\
                 ## Boundary Reminder\n\n\
                 Scene drift says how a scene may change if its pressure persists. It does not activate clocks, AI routines, or procedural world updates.\n",
            );
            output
        },
    )
}

fn push_artifact_input_line(output: &mut String, label: &str, byte_len: usize) {
    let _ = writeln!(output, "{label}: {byte_len}.");
}

pub fn build_vertical_integration_stack_from_artifacts(
    current_synthesis_base: &str,
    current_synthesis_collision_relay: &str,
    hueman_boundary: &str,
    hueman_glaushouse_roles: &str,
    hueman_sandmanor_roles: &str,
    hueman_inverse_circle: &str,
    hueman_procedural_uplift: &str,
    hueman_scene_presence: &str,
    hueman_scene_intent: &str,
    hueman_scene_drift: &str,
) -> String {
    memoized_artifact_by_lengths(
        &VERTICAL_INTEGRATION_STACK_ARTIFACT_CACHE,
        [
            current_synthesis_base.len(),
            current_synthesis_collision_relay.len(),
            hueman_boundary.len(),
            hueman_glaushouse_roles.len(),
            hueman_sandmanor_roles.len(),
            hueman_inverse_circle.len(),
            hueman_procedural_uplift.len(),
            hueman_scene_presence.len(),
            hueman_scene_intent.len(),
            hueman_scene_drift.len(),
        ],
        || {
            let mut output = String::with_capacity(4_800);
            output.push_str(render_vertical_integration_stack_prefix());
            push_artifact_input_line(
                &mut output,
                "Current Synthesis Base bytes",
                current_synthesis_base.len(),
            );
            push_artifact_input_line(
                &mut output,
                "Current Synthesis collision relay bytes",
                current_synthesis_collision_relay.len(),
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
        },
    )
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
        build_hueman_vertical_slice_for_spec_from_artifacts,
        build_hueman_vertical_slice_from_artifacts,
        build_vertical_integration_stack_from_artifacts,
    };

    #[test]
    fn hueman_boundary_builder_is_deterministic() {
        let output = build_hueman_boundary_from_artifacts("base", "gate");
        assert!(output.starts_with("# Hueman Boundary"));
        assert!(output.contains("Hueman is the standalone civilization sandbox layer"));
        assert!(
            output
                .contains("Hollow Grove remains the primary constitutional and operating language")
        );
        assert!(output.contains("Godot 4 and Aseprite are appropriate for Hueman."));
        assert!(output.contains("Current Synthesis activation gate bytes: 4."));
    }

    #[test]
    fn hueman_motion_map_builder_is_deterministic() {
        let output = build_hueman_motion_map_from_artifacts("boundary", "ops");
        assert!(output.starts_with("# Hueman Motion Map"));
        assert!(output.contains("the Hueman avatar moves through the field"));
        assert!(output.contains("Current Synthesis keeps `PLEB`/`META` occupancy"));
        assert!(output.contains("Current Synthesis operational bytes: 3."));
    }

    #[test]
    fn hueman_start_choices_builder_is_deterministic() {
        let output = build_hueman_start_choices_from_artifacts("fourway", "triad");
        assert!(output.starts_with("# Hueman Start Choices"));
        assert!(output.contains("the player begins as Hueman near Aura Ridge"));
        assert!(output.contains("`gremlin` originates in Flynt"));
        assert!(output.contains("`pixy` originates in Sandmanor"));
        assert!(output.contains("Hueman AuraTriad bytes: 5."));
    }

    #[test]
    fn hueman_fourway_builder_is_deterministic() {
        let output = build_hueman_fourway_from_artifacts("boundary", "motion");
        assert!(output.starts_with("# Hueman Fourway"));
        assert!(output.contains("West = Flynt"));
        assert!(output.contains("`goblin` originates in Stonebend."));
        assert!(output.contains("braided origin paths, not the initial player body"));
        assert!(output.contains("Hueman motion map bytes: 6."));
    }

    #[test]
    fn hueman_aura_triad_builder_is_deterministic() {
        let output = build_hueman_aura_triad_from_artifacts("fourway", "topology");
        assert!(output.starts_with("# Hueman Aura Triad"));
        assert!(output.contains("route study beneath Fourway"));
        assert!(output.contains("without becoming a universal holding pen"));
        assert!(output.contains("Current Synthesis topology bytes: 8."));
    }

    #[test]
    fn hueman_aura_behavior_builder_is_deterministic() {
        let output = build_hueman_aura_behavior_from_artifacts("triad", "start");
        assert!(output.starts_with("# Hueman Aura Behavior"));
        assert!(output.contains("## Player Grammar"));
        assert!(output.contains("Current Flow asks how the player moves through the world."));
        assert!(output.contains("the player still begins in ordinary Hueman identity"));
        assert!(output.contains("Hueman Start Choices bytes: 5."));
    }

    #[test]
    fn hueman_stonebend_roles_builder_is_deterministic() {
        let output = build_hueman_stonebend_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Stonebend Roles"));
        assert!(output.contains("whoever holds the Hypergiant Crown is Hypergiant"));
        assert!(output.contains("the Stonebender is the public proving ground at Stonebend"));
        assert!(output.contains("Goblin is Stonebend's confirmed Current-origin path"));
        assert!(output.contains("Stonebend's signature pressure action is reallocation."));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_tross_helpers_builder_is_deterministic() {
        let output = build_hueman_tross_helpers_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Tross Helpers"));
        assert!(output.contains("recognition and engineering run as a west-to-east helper line"));
        assert!(output.contains("Gremlin is Flynt's confirmed Current-origin path."));
        assert!(output.contains("Flynt progression is recipe-gated rather than hereditary"));
        assert!(output.contains("Manticore is not Chimera under another name"));
        assert!(output.contains("whoever holds Contracore is Tross"));
        assert!(output.contains("Delinquent guards West"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_glaushouse_roles_builder_is_deterministic() {
        let output = build_hueman_glaushouse_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Glaushouse Roles"));
        assert!(output.contains("exclusive Synthesis"));
        assert!(output.contains("Sprite is Glaushouse's confirmed Aura-origin path."));
        assert!(output.contains("whoever holds final public Clearance is Prima Donna"));
        assert!(output.contains("Clearance may be challenged from the floor"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_sandmanor_roles_builder_is_deterministic() {
        let output = build_hueman_sandmanor_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Sandmanor Roles"));
        assert!(output.contains("Prove It and Configuration"));
        assert!(output.contains("Pixy is Sandmanor's confirmed Aura-origin path."));
        assert!(output.contains("The Sandman is the singular office of rule"));
        assert!(output.contains("the winning contender becomes The Sandman."));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_procedural_uplift_builder_is_deterministic() {
        let output = build_hueman_procedural_uplift_from_artifacts(
            "execution",
            "rules",
            "transition",
            "relay",
            "selection",
            "consequence",
            "gate",
            "stonebend",
            "tross",
            "glaushouse",
            "sandmanor",
        );
        assert!(output.starts_with("# Hueman Procedural Uplift"));
        assert!(output.contains("## Bottom-Up Procedure Spine"));
        assert!(output.contains("## Relay Procedure"));
        assert!(output.contains(
            "a Flynt contender gathers ingredients in the field but still has to pass through Glaushouse synthesis"
        ));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_archetype_lens_builder_is_deterministic() {
        let output =
            build_hueman_archetype_lens_from_artifacts("start", "aura", "roles", "sandmanor");
        assert!(output.starts_with("# Hueman Archetype Lens"));
        assert!(output.contains("each confirmed origin path may read the same regions"));
        assert!(output.contains("### `gremlin`"));
        assert!(output.contains("goblin reading notices whether title is still legitimate"));
        assert!(output.contains("does not make forms into hereditary races"));
    }

    #[test]
    fn hueman_start_paths_builder_is_deterministic() {
        let output = build_hueman_start_paths_from_artifacts("start", "lens");
        assert!(output.starts_with("# Hueman Start Paths"));
        assert!(output.contains("while the player still begins as Hueman near Aura Ridge"));
        assert!(output.contains("Flynt-facing approach = Aura Basin -> Aura Fields -> Aura Beach"));
        assert!(output.contains("major transformation mechanics are not active yet"));
        assert!(output.contains("Hueman Archetype Lens bytes: 4."));
    }

    #[test]
    fn hueman_vertical_slice_builder_is_deterministic() {
        let output =
            build_hueman_vertical_slice_from_artifacts("boundary", "choices", "behavior", "uplift");
        assert!(output.starts_with("# Hueman Vertical Slice"));
        assert!(output.contains("Aura Ridge Opal Oil Starter Loop"));
        assert!(output.contains("Signature resource: Opal Oil"));
        assert!(output.contains("Current Form path: Gremlin"));
        assert!(output.contains("### Flynt / Recognize it"));
        assert!(output.contains("Route Stabilization (`route`)"));
        assert!(output.contains("Flock Defense (`defense`)"));
        assert!(output.contains("proof: Bench proof must verify hinge-post stability"));
        assert!(output.contains("field output: Hinge Seal Charge x1"));
        assert!(output.contains("credential: Flockline Trust"));
        assert!(output.contains("unlocked next task: Route Hinge Survey"));
        assert!(output.contains("unlocked next task: Shelterline Night Watch"));
        assert!(output.contains("failure: Defense failure occurs if the anchor slips"));
    }

    #[test]
    fn quarry_vertical_slice_builder_uses_the_selected_slice_spec() {
        let output = build_hueman_vertical_slice_for_spec_from_artifacts(
            &crate::hueman_slice::FLOODED_QUARRY_VERTICAL_SLICE_SPEC,
            "boundary",
            "choices",
            "behavior",
            "uplift",
        );

        assert!(output.contains("Flooded Quarry Night Watch Loop"));
        assert!(output.contains("Signature resource: Mercury Mirror"));
        assert!(output.contains("Current Form path: Goblin"));
        assert!(output.contains("Crane Route Hold (`route`)"));
        assert!(output.contains("Pump Intake Hold (`defense`)"));
        assert!(output.contains("credential: Quarry Rim Trust"));
        assert!(output.contains("credential: Tower Watch Trust"));
        assert!(output.contains("unlocked next task: Crane Marker Survey"));
        assert!(output.contains("unlocked next task: Pump Relay Audit"));
    }

    #[test]
    fn hueman_path_crossovers_builder_is_deterministic() {
        let output = build_hueman_path_crossovers_from_artifacts("paths", "aura", "relay");
        assert!(output.starts_with("# Hueman Path Crossovers"));
        assert!(output.contains("## Relay Junction"));
        assert!(output.contains("`P/M -> L/E`"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_link_physics_builder_is_deterministic() {
        let output = build_hueman_link_physics_from_artifacts("sequence", "cross", "relay");
        assert!(output.starts_with("# Hueman Link Physics"));
        assert!(output.contains("## Relay Packet Reading"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
        assert!(output.contains("one bias body may persist above the map and below the map"));
    }

    #[test]
    fn hueman_inverse_circle_builder_is_deterministic() {
        let output = build_hueman_inverse_circle_from_artifacts("fourway", "physics");
        assert!(output.starts_with("# Hueman Inverse Circle"));
        assert!(output.contains("Mnt. Aura"));
        assert!(output.contains(
            "each side mirrors Stairway to Heaven, Riptide, Current Seanad, and Mnt. Aura"
        ));
        assert!(output.contains("Hueman Link Physics bytes: 7."));
    }

    #[test]
    fn hueman_crossover_scenes_builder_is_deterministic() {
        let output = build_hueman_crossover_scenes_from_artifacts("cross", "physics", "relay");
        assert!(output.starts_with("# Hueman Crossover Scenes"));
        assert!(output.contains("## Relay Scene Use"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
        assert!(output.contains("Seam Market benefits most directly"));
    }

    #[test]
    fn hueman_scene_presence_builder_is_deterministic() {
        let output = build_hueman_scene_presence_from_artifacts(
            "scene",
            "lens",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
            "relay",
        );
        assert!(output.starts_with("# Hueman Scene Presence"));
        assert!(output.contains("## Relay Packet Presence"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
        assert!(
            output.contains("Aura Fields hinge pressure can therefore feel structurally doubled")
        );
    }

    #[test]
    fn hueman_scene_intent_builder_is_deterministic() {
        let output = build_hueman_scene_intent_from_artifacts(
            "presence",
            "physics",
            "relay",
            "contract",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
        );
        assert!(output.starts_with("# Hueman Scene Intent"));
        assert!(output.contains("shared confirmation pressure packet"));
        assert!(output.contains("one multi-function witness body"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_scene_drift_builder_is_deterministic() {
        let output = build_hueman_scene_drift_from_artifacts("intent", "physics", "relay");
        assert!(output.starts_with("# Hueman Scene Drift"));
        assert!(output.contains("slows drift by holding one shared confirmation point"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn vertical_integration_stack_builder_is_deterministic() {
        let output = build_vertical_integration_stack_from_artifacts(
            "base",
            "relay",
            "boundary",
            "glaushouse",
            "sandmanor",
            "inverse",
            "procedures",
            "presence",
            "intent",
            "drift",
        );
        assert!(output.starts_with("# Vertical Integration Stack"));
        assert!(output.contains("## Bottom-To-Top Procedure"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
        assert!(output.contains("scene procedure: path crossovers, link physics"));
    }
}
