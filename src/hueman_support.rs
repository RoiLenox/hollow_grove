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
pub const HUEMAN_FLYNT_CONSTITUTION_ARTIFACT_PATH: &str = "artifacts/hueman_flynt_constitution.md";
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

pub fn hueman_flynt_constitution_artifact_path() -> PathBuf {
    PathBuf::from(HUEMAN_FLYNT_CONSTITUTION_ARTIFACT_PATH)
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
    "restored travelers, tourists, ceremonies, public gatherings, free traders, and open exchange terraces",
    "the elevated Glaushouse <-> Stonebend ridge presents restored beings through Equal Gaze and civic witness",
    "Current Sea certification remains a separate northern ordeal rather than another name for the visible ridge",
];

const AURA_RIDGE_INTENT_LINES: &[&str] = &[
    "Witness: return restored beings to public sight, presentation, exchange, and civic reintegration",
    "hold Equal Gaze as reciprocal public witness without turning it into legal certification",
    "keep the elevated open ridge legible without breaking the larger circular world loop",
];

const GLAUSHOUSE_PRESENCE_LINES: &[&str] = &[
    "constitutional domain: Glaushouse clears medicine, recovery, and lawful Synthesis for machine bodies and Hueman bodies",
    "style: Glaushouse presents as a mechanical-industrial medical capital with Berlin severity, Milan polish, chrome discipline, and clinic glamour",
    "Prima Donna: the singular highest clinical office, currently presented through Doctor Ratchet",
    "Persephone: the multiple balanced whole-course rank, with Nurse House as one current holder",
    "Matron and Marshal: equal Aura-forward and Current-forward branches",
    "The Nightingales: the universal clinical foundation, constitutional nursing and clinical-care institution, bedside check, and civic body aligned with white blood cells",
    "Glauspitals: clinical facilities for diagnosis, medicine, care, recovery, and lawful procedure",
    "Chromacord: clinical record and evidence infrastructure that cannot create consent or Clearance",
    "Glaus Gel: the signature repair resource supporting sealing, bonding, cooling, and restoration",
    "Glausteel: the hard integrated alloy associated with cleared machine and civic work",
];

const GLAUSHOUSE_INTENT_LINES: &[&str] = &[
    "Clear: permit only a named, scoped, consented, competent, time-bounded intervention with a recovery path",
    "style: run recovery through cold-lit wards, industrial bays, strict presentation, and commanding public poise rather than rustic comfort",
    "Prima Donna: preserve clinical law and high-risk Clearance without manufacturing consent, evidence, Title, or recognition",
    "Persephone: reconcile Matron and Marshal evidence across passage, stabilization, recovery, discharge, regression, and return",
    "Nightingales: protect bedside truth, patient advocacy, and immediate clinical stops with mandatory review",
    "consent and capacity: never infer participation from silence, custody, dependence, recognition, refusal, or Aura influence",
    "Synthesis: record actual outcomes, preserve prior identity, and carry recovery through completion",
    "Illegal Synthesis: stop coercive, unsafe, fraudulent, concealed, identity-erasing, or unrecorded transformation",
    "Glaus Gel: support machine and Hueman repair as a practical synthesis medium",
    "Glausteel: carry the cleared integrated hard branch after restoration and synthesis work",
];

const STONEBEND_PRESENCE_LINES: &[&str] = &[
    "constitutional domain: Stonebend establishes Name, Title, structure, boundary, continuity, and lawful Hollowing",
    "Geralds: Stonebend's civic people, whose standing supports petition, witness, stewardship, inheritance, and challenge",
    "Hypergiant: the singular highest Stonebend office and final custodian of constitutional integrity",
    "Proliteriate: the permanent distributed Yield network whose active witnesses carry bounded temporary mandates",
    "High Freemason: the singular office leading the Freemason institution's structural execution, survey, Seal, custody, and defense",
    "Stonebend presence keeps Claim, Title, and Yield distinct",
    "diamonds: reflective mineral wealth aligned with title, structure, and witness",
    "Mercury Mirror: Stonebend's signature reflective resource refined through Hollowing and reflective craft",
    "Mercurite: Stonebend's hard material branch for tools, armor, and structural bearing",
];

const STONEBEND_INTENT_LINES: &[&str] = &[
    "Name: attach a stable identifier to one subject in one scope without manufacturing truth",
    "Title: grant only recorded standing, authority, ownership, custody, stewardship, jurisdiction, obligation, or office",
    "Mirror: verify correspondence among subject, record, continuity, provenance, survey, and accession without becoming authority",
    "Seal: bind the correct authority, subject, scope, decision, and sequence",
    "Hollowing: require authority, purpose, consent where applicable, evidence, qualified operation, custody, continuity, restoration, and Seal",
    "Illegal Hollowing: stop unauthorized, coercive, fraudulent, concealed, excessive, destructive, or unrecorded interior alteration",
    "Tombstone: preserve ended Names, Titles, structures, and obligations without treating deletion as history",
    "Stonebend intent never treats recognition as Title, transformation as accession, clearance as consent, or custody as ownership",
];

const FLYNT_PRESENCE_LINES: &[&str] = &[
    "engineering: Flynt is the recognition and field-engineering function that moves capability into lived operation",
    "style: Flynt presents as a boardwalk-casino hunting capital with neon vice, opal glamour, North African desert grounds, and hard modern swagger",
    "Tross: the sovereign executive from whom all Flynt constitutional authority derives",
    "Chimera: the lower apex integration of Wolf, Bat, and Snake/Fish",
    "Manticorp: the maintained sovereign Synthesis Form held by Tross, and the lawful military institution formally led by Tross",
    "urban: Tross publicly commands Manticorp while the same man is Mystery Man/Mr. X, leader of the Gallows underground",
    "rural: The Gallows is Flynt's underground organized-crime body, with We Fairy Men and the Basin bands remaining distinct",
    "opals: mined feedstock and recognition-rich gleam carried through Flynt engineering",
    "Opal Oil: Flynt's signature resource produced through regular current and holographic aura",
    "Flynt presence keeps hidden value and field usefulness legible before title catches up",
];

const FLYNT_INTENT_LINES: &[&str] = &[
    "engineering: turn practical capability into deployable routes, services, machines, and infrastructure",
    "style: run recognition through boardwalk temptation, casino risk, nocturnal glamour, and outer hunting pressure rather than polite institutional order",
    "Tross: remain the single constitutional executive above both institutional expressions",
    "Chimera: refine the lower apex integration without becoming Manticorp",
    "Manticorp: remain both a distinct maintained Form and the institution named for that sovereign ideal",
    "Tross = Mystery Man = Mr. X: preserve one stable person across public and underground command",
    "The Gallows and We Fairy Men: preserve underground leverage, Basin distinction, and divided Recipe custodianship",
    "The Gallowry: remain the hidden headquarters of the Gallows rather than becoming a separate institution",
    "opals: support advanced engineering feedstock and field-ready refinement",
    "Opal Oil: turn regular current and holographic aura into a practical engineering medium",
    "presentation must preserve direct Tross command of both Manticorp and the Gallows",
];

const SANDMANOR_PRESENCE_LINES: &[&str] = &[
    "configuration: Sandmanor is the proof and design function, split between Minorian count and Minoan arrangement",
    "Sandmen: the stable compatibility name for Sandmanor's House-wide proof-body adapter, not a third people or second authority",
    "Minoans: the equal exterior Elf tradition of arrangement, navigation, access, and relationship to the world",
    "Minorians: the equal interior Gnome tradition of maintenance, cultivation, repeated function, and repairability",
    "Aura Beach and Current Sea: the Minoan regional proving grounds where outward design, mobility, and navigation remain visible",
    "Aura Field: the Minorian regional proving ground where cultivation, maintenance, repeated labor, and yield remain testable",
    "The Sandman: the singular highest office, selected only through the completed reciprocal Contest of Improvement and full accession",
    "Prism Sand: Sandmanor's signature reflective resource for measurement, refraction, and records",
    "Prismiron: Sandmanor's hard branch for proof-ready structures and instruments",
    "Sandmanor presence favors witnessed comparison over inherited fixed rank",
];

const SANDMANOR_INTENT_LINES: &[&str] = &[
    "configuration: prove the claim, test the proof, and improve the arrangement",
    "Sandmen: preserve the stable institution projection used by common House proof decisions without becoming a people or sovereign",
    "Minoans: teach exterior relationship, navigation, access, and horizon-facing design",
    "Minorians: teach interior function, maintenance, cultivation, and repeated-system design",
    "Aura Beach and Current Sea: prove outward mobility, coastal relationship, and navigation within disclosed scope",
    "Aura Field: prove cultivation, maintenance, yield, sustainability, and repeated work within disclosed scope",
    "The Sandman: emerge through the greatest documented reciprocal improvement, then complete Title, recognition, learning statement, and Seal",
    "Prism Sand: support counting, refraction, glass, and record logic",
    "Prismiron: support precise durable structures after proof survives inspection",
    "Sandmanor intent keeps rivalry productive instead of merely punitive",
];

const INVERSE_CIRCLE_PRESENCE_LINES: &[&str] = &[
    "The Stairway to Heaven / Ascend: high border ascent above an underground inverse curve, with burden stations, rung marks, and lifted edge traffic",
    "The Riptide / Retrieve: emergency water corridor above an underground inverse curve, with rescue beacons, drag marks, intake traffic, and Merman roaming along the rim",
    "The Current Seanad / Deliberate: institutional water court above an underground inverse curve, with hearing chambers, evidence crossings, and measured assembly",
    "Mt. Aura / Aspire: outer border curve from Stonebend to Sandmanor above an underground inverse curve, with bright air, higher goals, and felt border pressure before sight",
];

const INVERSE_CIRCLE_INTENT_LINES: &[&str] = &[
    "The Stairway to Heaven / Ascend: accept higher burden through ascent while its underground inverse curve stores hidden descent pressure",
    "The Riptide / Retrieve: carry crises toward Glaushouse intake while its underground inverse curve keeps emergency undertow and Merman range alive on the water rim",
    "The Current Seanad / Deliberate: hold difficult design and repair questions for institutional judgment while its underground inverse curve keeps hidden current structure",
    "Mt. Aura / Aspire: hold the established aspirational curve from Stonebend to Sandmanor while its underground inverse curve keeps the hidden under-arc",
];

const GREMLIN_LENS_LINES: &[&str] = &[
    "Aura Basin reads as hunt pressure, route opportunities, den seams, and drillable carry",
    "Aura Field reads as infrastructure crossings, farm lanes, survey lanes, and deployable work",
    "Aura Beach reads as exposure, salvage, boardwalk threshold, and field-engineering release",
];

const GOBLIN_LENS_LINES: &[&str] = &[
    "Aura Basin reads as load paths, hidden supports, and structural bearing",
    "Aura Field reads as named work sites, claim boundaries, and public craft pressure",
    "Aura Beach reads as threshold framing, recovery of material, and edge structure",
];

const GOBLIN_OVERLAY_LINES: &[&str] = &[
    "Stonebend's people are the Geralds; civic membership never implies office",
    "the Hypergiant is the singular highest constitutional office, not a form or transformation tier",
    "the Proliteriate supplies civic legitimacy while the Freemason institution supplies structural execution",
    "goblin form reading may notice structure and burden, but it cannot grant Name, Title, office, or accession",
];

const SPRITE_LENS_LINES: &[&str] = &[
    "Aura Basin reads as triage depth, hidden damage, and latent recovery need",
    "Aura Field reads as treatment traffic, care signals, and integration risk",
    "Aura Beach reads as exposure, vulnerability, and urgent stabilization",
];

const PIXY_LENS_LINES: &[&str] = &[
    "Aura Basin reads as count density, hidden records, and unresolved variables",
    "Aura Field reads as Minorian comparison ground, measurable change, and public proof pressure",
    "Aura Beach reads as the Minoan coastal proving ground, with edge cases, access, navigation, and outward test conditions held in view",
];

const PIXY_OVERLAY_LINES: &[&str] = &[
    "Minoans make the pixy reading notice arrangement, composition, and configuration pressure.",
    "Minorians make the pixy reading notice tally, measure, and public proof.",
    "Aura Beach belongs to the Minoan exterior tradition, where coastal access, mobility, navigation, and environmental relationship remain testable.",
    "Aura Field belongs to the Minorian side, where proof is compared, counted, and made legible in public.",
    "the Contest of Improvement makes reciprocal teaching, baseline evidence, final evidence, criticism, and review visible as the lawful basis of Sandman accession.",
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
        name: "Aura Field",
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

const STONEBEND_ROLE_SECTIONS: [HuemanSectionDefinition; 9] = [
    HuemanSectionDefinition {
        title: "People and Principal Authorities",
        lines: &[
            "the Geralds are Stonebend's constitutional people; citizenship provides standing but never implies office.",
            "the Hypergiant is the singular highest Stonebend office and preserves the integrity of Names, Titles, succession, boundaries, and sealed constitutional records.",
            "the Proliteriate is Stonebend's permanent distributed public network; its active voices are temporary, bounded, recallable witnesses rather than permanent chamber seats or creature roles.",
            "the High Freemason is the singular office leading the wider Freemason institution for structure, defense, survey, Seals, custody, lawful execution, and Hollowing equipment.",
        ],
    },
    HuemanSectionDefinition {
        title: "Form Continuity",
        lines: &[
            "the existing Gremlin -> Goblin -> Ghoul -> Spectre -> Troll -> Ork -> Ogre -> Troglodyte form sequence remains the complete Presynce embodiment ladder and a creature-form projection, not a political rank ladder.",
            "a changed Frame may receive a transformed-form Name while its former Names, lineage, liabilities, and continuity remain recorded.",
            "no Current Form, Aura Frame, Synthesis, species, combat rank, wealth level, or lineage automatically grants eligibility for Hypergiant or High Freemason.",
            "transformation may never create office or Title automatically.",
        ],
    },
    HuemanSectionDefinition {
        title: "Constitutional Powers",
        lines: &[
            "Stonebend separates Claim, Title, and Yield without creating three equal sovereigns: the Freemason forges the Claim, the Hypergiant bears the Title, and the Proliteriate shields the Yield.",
            "the Hypergiant is the highest constitutional office, remains evidence-bound and challengeable, and may not occupy Proliteriate or Freemason office simultaneously.",
            "the Proliteriate preserves public witness and dissent through district, guild or workshop, labor crew or worksite, and inherited or commonwealth nodes; it raises temporary witnesses and cannot execute armed seizure or Hollowing.",
            "the Freemason forges and tests Claims, preserves records and boundaries, may refuse unlawful orders, and cannot create sovereignty or self-certify a sovereign Claim.",
            "one power may challenge; the other two distinct powers must concur to remove; the same power cannot count twice, and representation removal cannot abolish the network.",
            "Hypergiant accession requires a stable Claim, independent Freemason examination, Proliteriate Yield hearing, relinquishment of protected elevation, consequence descent, Flynt or Basin Proof of Persistence, The Lazerhorn, lawful vacancy, Seal, and public oath.",
        ],
    },
    HuemanSectionDefinition {
        title: "Name, Title, Mirror, and Seal",
        lines: &[
            "a Name identifies one subject in a defined scope and preserves continuity; it does not grant ownership, office, competence, clearance, recognition, or moral worth.",
            "a Title authorizes only the recorded standing, authority, ownership, custody, stewardship, jurisdiction, obligation, or office attached to a named subject.",
            "a Mercury Mirror verifies correspondence among subject, record, present, past, structure, plan, provenance, survey, boundary, and accession; it does not create truth or grant Title.",
            "a Seal binds the correct subject, authority, scope, decision, and sequence and makes later alteration detectable.",
            "ownership, custody, and stewardship remain distinct.",
            "renaming preserves former Names, dates, reasons, continuity, associated Titles, and Tombstoned claims.",
        ],
    },
    HuemanSectionDefinition {
        title: "Hollowing and Continuity",
        lines: &[
            "lawful Hollowing requires a named subject, authority, purpose, defined scope, consent when applicable, pre- and post-procedure evidence, qualified operator, safety and custody plans, continuity determination, restoration or disposition, and Seal.",
            "emergency Hollowing is narrow, necessary, recorded, and followed immediately by mandatory review.",
            "every extracted Hollow record preserves source provenance and custody.",
            "Illegal Hollowing is Stonebend's signature constitutional offense.",
            "Tombstone is durable constitutional history, never deletion, and a Tombstoned Name or Title cannot exercise active authority.",
            "Current Synthesis remains authoritative for Current, Aura, Bond, Synthesis, persistence, replay, and common House decisions beneath this Stonebend-specific law.",
        ],
    },
    HuemanSectionDefinition {
        title: "Aura Way, Aether, and Stone",
        lines: &[
            "Mt. Aura is Aether, the shared ideal and pinnacle; Riptide is Bathos, the downward pole of weight and consequence; neither is owned by Stonebend.",
            "Aether is weightless Current and Current is the heaviest Aether: distinct burden states of one medium whose stable lineage survives lawful refinement.",
            "Aura is Aether revealed through Form, while Current is Bathos embodied through weight.",
            "Aura Way is the standard known path through prerequisites, education, supervised practice, examination, demonstrated responsibility, and recognition eligibility.",
            "the Houses teach the work, Aura Way organizes the path, and Stonebend names completion without declaring metaphysical perfection.",
            "lawful material Hollowing removes only authorized removable burden, preserves essential fractions, and carries source Current, result Aether, proof, provenance, custody, and Seal.",
            "geographic stone Form refracts universal Aether into particular Aura: Opal varies, Diamond concentrates, and Quartz resonates without creating exclusive House ownership.",
            "ordinary Aura manifestation preserves the stone lattice and does not require melting.",
        ],
    },
    HuemanSectionDefinition {
        title: "Three Gates, Diamond, and Bounded Title Scope",
        lines: &[
            "Stonebend has exactly three bidirectional constitutional gates: Flynt-facing through Stairway to Heaven and Basin Motor Speedway, Central Junction-facing through the Craft Corridor, and Sandmanor-facing through Aura Way and Mt. Aura.",
            "Central Junction remains a district, Mt. Aura remains an ideal and route landmark, and Riptide remains outside Stonebend on its established route.",
            "one stable core Title may hold independent Formation Recognition, Public Circulation, and Operational Deployment scopes; rejection or limitation at one gate does not erase another scope.",
            "gate failures return typed evidence and distinguish honest failure, negligence, fraud, illegality, and constitutional hollowness.",
            "Diamond is Stonebend's continuing sovereign Title; the Hypergiant is its temporary active bearer; Diamond remains when vacant.",
            "temporary Proliteriate witnesses carry only a stable bounded mandate, may be recalled, and return authority to the permanent network when the matter ends.",
            "Spartacus is an archetype for a raised witness, not a permanent fourth office.",
            "removal and succession preserve durable Tombstones rather than deleting ended authority.",
            "no Hypergiant claims Diamond without The Lazerhorn; recommendation, lineage, former office, and self-certification cannot shorten the path.",
            "every gate crossing remains traceable to Claim evidence, Title or boundary disposition, and Yield accountability even when routine work is delegated.",
        ],
    },
    HuemanSectionDefinition {
        title: "Title Lifecycle and Constitutional Continuity",
        lines: &[
            "a Claim is not automatically a Title; Stonebend recognition establishes the bounded Title, while activation separately permits its exercise under explicit policy conditions.",
            "maintenance sustains an active term, while renewal is a formal decision to continue the same stable Title into another term.",
            "limitation, supervision, suspension, remediation, restoration, and removal target one identified scope or constitutional layer rather than silently destroying the core Title.",
            "honest failure, negligence, fraud, illegality, and constitutional hollowness remain distinct and receive proportionate recorded consequences.",
            "restoration repairs the Title without erasing the interruption, remediation evidence, or continuing limitations.",
            "every ended active term links to a Tombstone, and honorable completion remains distinct from punitive removal.",
            "during Diamond vacancy, bounded continuity mandates may preserve existing duties but cannot become Diamond, appoint a Hypergiant, create sovereign law, or grow permanent through use.",
            "High Freemason replacement requires independent Forge review; no candidate self-certifies and no outgoing bearer appoints the replacement unilaterally.",
            "the Proliteriate network survives node change, witness recall, and mandate completion without a permanent speaker or locked numerical threshold.",
            "Claim, Title, and Yield evidence remain traceable through every temporal review.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "STONEBEND_CONSTITUTION_V2.md is the canonical Stonebend law",
            "STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md is the canonical bounded first-pass foundation",
            "STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md is the canonical bounded second-pass constitutional supplement",
            "STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md is the canonical bounded third-pass lifecycle supplement",
            "src/world/stonebend.rs enforces Stonebend-specific authority and registry invariants above the frozen runtime",
            "src/world/stonebend/foundation.rs enforces Aura Way, material provenance, lawful refinement, and stone-refraction invariants without extending Stonebend government",
            "src/world/stonebend/second_pass.rs enforces the three gates, bounded Title scopes, Diamond tenure, distributed Proliteriate, removal, Tombstones, succession, and delegated accountability without changing gameplay",
            "src/world/stonebend/third_pass.rs enforces recognition, activation, maintenance, renewal, targeted intervention, restoration, vacancy continuity, independent Forge replacement, and temporal Claim/Title/Yield review without replacing the common Bond engine",
            "Body -> Presynce is Stonebend's Hueman faculty: embodied anticipation bounded by Frame, Flow, polarity, evidence, and uncertainty, never an automatic dodge or counter",
            "src/world/house_institutions.rs provides the neutral institution projection",
            "Godot and Hueman artifacts are presentation only",
            "the recursion kernel and Constitutional Runtime V2 remain untouched and authoritative in their frozen domains",
        ],
    },
];

const FLYNT_CONSTITUTION_SECTIONS: [HuemanSectionDefinition; 9] = [
    HuemanSectionDefinition {
        title: "Constitutional Executive",
        lines: &[
            "Tross is the sovereign executive of Flynt.",
            "Every Flynt institution and office ultimately derives constitutional authority from Tross.",
            "Manticorp and the Gallows answer directly to Tross through distinct public and underground authorities.",
        ],
    },
    HuemanSectionDefinition {
        title: "Constitutional Companion",
        lines: &[
            "There is exactly one constitutional Chimera.",
            "Chimera is the lower apex integration of Wolf, Bat, and Snake/Fish, with meaningful refinement before Manticorp.",
            "Manticorp is a distinct maintained Synthesis Form beyond Chimera and the institution named after that sovereign ideal.",
            "The Tross is the presently maintained living holder of Manticorp and the formal constitutional leader of Manticorp Institution.",
            "Stable identity invariant: Tross = Mystery Man = Mr. X.",
            "Manticorp Form stable ID: flynt.form.manticorp.",
            "Manticorp Recipe stable ID: flynt.recipe.divided-manticorp.",
            "Manticorp may continue throughout the Tross's life through bodily discipline, Recipe renewal, divided Basin knowledge, institutional recognition, and specialized Glaushouse care.",
            "Gremlincoin is the Gremlin Way: meaningful salvage, improvisation, risk, mobility, low-resource adaptation, frontier labor, and discovered abandoned value become lawful Synthesis evidence.",
            "Gargoyle is never automatic at a numeric threshold; Recipe viability must turn Gremlin opportunity into maintained structure, territory, responsibility, maintenance, and renewal.",
            "The Gremlin finds value. The Gargoyle makes that value stand.",
        ],
    },
    HuemanSectionDefinition {
        title: "Urban Expression",
        lines: &[
            "The public hierarchy is Tross -> Manticorp Institution; the same Tross is underground Mystery Man/Mr. X -> The Gallows.",
            "The public chain is Tross -> Manticorp Institution.",
            "The underground chain is Mystery Man/Mr. X -> The Gallows.",
            "Manticorp is Flynt's formal military institution for territorial defense, military command, constitutional protection, disciplined force, military training, and lawful deployment.",
            "Mystery Men is Flynt's unified publicly recognized, operationally classified federal investigative bureau; it is distinct from the Mystery Man identity of Tross.",
            "The Mystery Man and Mr. X are aliases or operational identities of Tross, not separate persons or subordinates.",
            "The Gallows is Flynt's underground Yakuza/mafia-like body governing illicit trade, debts, contraband, and deniable action.",
        ],
    },
    HuemanSectionDefinition {
        title: "Rural Expression",
        lines: &[
            "The Gallows is an underground command expression of Tross, while We Fairy Men is a coalition of distinct Basin traditions.",
            "Bro White, The Beauty, and Cinderellaman remain distinct leaders and custodians of divided Manticorp Recipe components.",
            "The Basin bands may negotiate with either face of Tross without being permanently absorbed into Manticorp or the Gallows.",
            "Music and roaming culture are expressions of We Fairy Men, not a replacement for its separate identities and territories.",
        ],
    },
    HuemanSectionDefinition {
        title: "The Gallowry",
        lines: &[
            "The Gallowry is the hidden headquarters and home of the Gallows; it is not the Gallows itself.",
            "The Gallowry is the Gallows meeting place, headquarters, cultural center, gallery, and operational hub.",
        ],
    },
    HuemanSectionDefinition {
        title: "Founding Leaders",
        lines: &[
            "Bro White is the enduring Gargoyle Founding Leader office; Bro White and the 7 Brothas is its crew.",
            "Cinderellaman is the enduring Merman Founding Leader office; Cinderellaman and His Midnight Crew is its crew.",
            "The Beauty is the enduring Werewolf Founding Leader office; The Beauty and His Beasts is its crew.",
            "Successors may inherit each Founding Leader office without creating a new constitutional office.",
        ],
    },
    HuemanSectionDefinition {
        title: "Constitutional Union",
        lines: &[
            "Bro White, Cinderellaman, and The Beauty unite as We Fairy Men.",
            "We Fairy Men is the constitutional folk expression of the completed Chimera, not a temporary alliance.",
            "The urban and rural expressions are complementary traditions of the same Chimera authority, not competing governments.",
        ],
    },
    HuemanSectionDefinition {
        title: "Spirit and Resynce",
        lines: &[
            "Spirit -> Resynce is Flynt's Hueman faculty: relational synchronization through Beings, groups, routes, encounters, Bonds, affiliation, and recognition, never mind reading or a replacement Current Form ladder.",
            "We Fairy Men remain a distinct Aura Ridge frontier culture of caravaners, scouts, guides, traders, performers, salvagers, escorts, and pathfinders expressing mobile communal Resynce without formal Flynt state authority.",
            "The Gallows remain the distinct Flynt civic institution of consequential Resynce through reputation, public challenge, accountability, honor, disgrace, and judgment.",
            "This faculty jurisdiction preserves the frozen rural hierarchy while never merging the We Fairy Men group identity into the Gallows institution identity.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "the Flynt hierarchy is constitutionally locked",
            "Tross succession is not defined by this architecture and may not be inferred from creature progression",
            "Current Synthesis remains authoritative over constitutional state and validation",
            "Hueman and Godot may present the hierarchy but may not reinterpret it",
        ],
    },
];

const GLAUSHOUSE_ROLE_SECTIONS: [HuemanSectionDefinition; 7] = [
    HuemanSectionDefinition {
        title: "Canonical Anchor",
        lines: &[
            "Glaushouse remains East-facing on the Fourway.",
            "Glaushouse clears medicine, recovery, and lawful Synthesis for both machine and Hueman bodies.",
            "Glaushouse retains exclusive Synthesis authority among Houses while Current Runtime V2 remains authoritative over common Bond resolution and causality.",
            "Glaushouse reads as a mechanical-industrial medical capital: Berlin severity, Milan polish, chrome discipline, and commanding clinic glamour.",
            "Sprite is Glaushouse's confirmed Aura-origin path.",
            "Glaushouse mines jades and refines Glaus Gel as its jade-colored repair and synthesis medium.",
            "Glausteel is the accepted hard branch for cleared integrated work.",
        ],
    },
    HuemanSectionDefinition {
        title: "Constitutional Authorities",
        lines: &[
            "Prima Donna is the singular highest clinical office; Doctor Ratchet is its frozen current holder identity.",
            "Persephone is a multiple balanced clinical rank; Nurse House is one frozen current Persephone identity.",
            "The Nightingales are the universal clinical foundation, constitutional nursing and clinical-care institution, patient advocates, and protected bedside check.",
            "a Nightingale may choose Matron or Marshal; Matron and Marshal are equal complementary branches, and cross-training remains open.",
            "mastery of both branches may earn Persephone; multiple Persephones may serve while only one Prima Donna is active.",
            "the generative ladder is Nightingale -> equal Matron or Marshal branches -> multiple Persephones -> one Prima Donna.",
            "Glauspitals operates clinical facilities, while Chromacord preserves clinical records and evidence.",
            "advancement Toke/Tokens preserve one stable person and prior mastery; neither office, institution, chart, gesture, species, transformation, nor technical ability manufactures another authority.",
        ],
    },
    HuemanSectionDefinition {
        title: "Clinical Law",
        lines: &[
            "Diagnosis identifies condition; consent authorizes participation; Clearance permits procedure.",
            "Care preserves the subject; Synthesis transforms; recovery completes the act.",
            "consent is explicit, scoped, informed, voluntary, current, capacity-based, and never inferred from silence, custody, dependence, recognition, or Aura influence.",
            "consent to Hollowing is not consent to Synthesis, and Synthesis consent is not research consent.",
            "every Clearance names subject, procedure, operator, facility, scope, risk, consent, capacity, stopping conditions, recovery, and expiration.",
            "proof is not Clearance; recognition is not Clearance; custody is not ownership; transformation is not accession.",
        ],
    },
    HuemanSectionDefinition {
        title: "Form and Practice Ladder",
        lines: &[
            "Glaushouse's separate Aura form-and-practice ladder remains Pixy -> Sprite -> Farie -> Nymph -> Siren -> Muse; it does not replace the clinical ladder.",
            "Pixy marks nimble bedside attention; Sprite marks active floor care and relay motion; Farie marks delicate restorative practice used with discipline.",
            "Nymph marks stable healing presence; Siren marks commanding recall and recovery presence; Muse marks the highest restorative inspiration and formal guidance.",
            "the Aura ladder records Frame, practice, capability, and mastery development; only Nightingale evidence, branch proofs, and lawful advancement create Matron, Marshal, Persephone, or Prima Donna candidacy.",
            "a technically capable form still requires training, current competence, institutional authorization, scope, recognition, and Title where applicable.",
        ],
    },
    HuemanSectionDefinition {
        title: "Synthesis and Recovery",
        lines: &[
            "Synthesis requires a valid subject, explicit Synthesis consent, active Clearance, qualified operator, lawful material provenance, actual-outcome recording, identity continuity, and recovery.",
            "Synthesis is Continuance through renewal: a real living Form depends on maintenance, Recipe practice, the Form's Ways, compatible conditions, the Hueman, and institutional care.",
            "Adjustment, Graft, Reconstruction, and Transfiguration are intended clinical depths; Overgrowth is an emergency failure state.",
            "host rejection and Sympiote rejection are distinct, and major Synthesis requires both technical and lived viability.",
            "high-risk Synthesis requires a recovery plan and Nightingale witness; emergency Synthesis requires post-event review; experimental Synthesis is openly marked.",
            "Gnome -> Minotaur at Aura Field and Elf -> Centaur at Aura Beach remain regional Synthesis canon.",
            "Glaushouse clears safety, Sandmanor proves design, Stonebend preserves identity and Title, and Flynt recognizes function.",
            "Synthesis never automatically creates Title, office, recognition, or political standing.",
            "failed, partial, unstable, unintended, or injurious Synthesis is recorded as the actual result rather than renamed as success.",
        ],
    },
    HuemanSectionDefinition {
        title: "Protected Checks",
        lines: &[
            "a Nightingale may halt immediate clinical danger, absent consent, wrong subject, wrong procedure, excess scope, identity mismatch, failed safeguards, or missing rescue capacity.",
            "every Nightingale stop triggers mandatory review and is protected from retaliation.",
            "the Matron reads lived and Aura continuity; the Marshal holds bodily and Current continuity; Persephone preserves the whole patient and may deny premature discharge.",
            "Persephones collectively govern Living Ledger viability; Prima Donna governs Recipe Ledger transformation.",
            "clinical custody never becomes ownership, and recovery duties survive operator or institutional succession.",
            "Illegal Synthesis is Glaushouse's signature constitutional offense.",
            "Grip must not become domination; Show must not become humiliation; Grit must not glorify avoidable suffering.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "Glaushouse constitutional law is ratified in GLAUSHOUSE_CONSTITUTION_V2.md",
            "the House-specific registry validates clinical law above frozen Current Runtime V2",
            "Mind -> Precog is Glaushouse's Hueman faculty: evidence-grounded probable continuation through Glow and disclosed uncertainty, never omniscience or guaranteed future",
            "Hueman and Godot may present records but may not create consent, Clearance, privilege, office, or Synthesis outcomes",
            "common Bond, Current, Aura, persistence, replay, and final Synthesis Resolution remain authoritative in Current Runtime V2",
            "the universal recursion kernel remains isolated from Glaushouse-specific law",
        ],
    },
];

const SANDMANOR_ROLE_SECTIONS: [HuemanSectionDefinition; 9] = [
    HuemanSectionDefinition {
        title: "Canonical Anchor",
        lines: &[
            "Sandmanor remains South-facing on the Fourway.",
            "from Stonebend, Sandmanor sits on the far counter-arc.",
            "from Glaushouse, Sandmanor may read southward across the relational arc without changing the canonical map.",
            "",
            "Sandmanor proves: design proposes, method orders, teaching transmits, demonstration reveals, evidence supports, criticism tests, reproduction confirms, failure teaches, revision improves, and reciprocity transforms.",
            "Pixy is Sandmanor's confirmed Aura-origin path.",
            "Sandmanor mines crystals and refines Prism Sand as its signature proof-and-record resource.",
            "Prismiron is the accepted hard branch for proof-ready structures and instruments.",
        ],
    },
    HuemanSectionDefinition {
        title: "Equal Civic Traditions",
        lines: &[
            "Minorians are Sandmanor's interior Gnome tradition and ask how a design works from within.",
            "Minoans are Sandmanor's exterior Elf tradition and ask how a design meets the world beyond itself.",
            "Minorians and Minoans have equal constitutional standing and remain distinct enough to correct one another.",
            "the Sandmen stable institution ID remains the House-wide proof-body adapter; it is not a third people, a second government, or the singular Sandman office.",
        ],
    },
    HuemanSectionDefinition {
        title: "Regional Design",
        lines: &[
            "Aura Field is the Minorian proving ground for cultivation, repeated labor, maintenance, yield, sustainability, Gnome practice, and Minotaur advancement.",
            "Aura Beach and the Current Sea are the Minoan proving grounds for coastal access, navigation, exploration, Elf practice, and Centaur mobility.",
            "Gnome -> Minotaur is the canonical Aura Field regional Synthesis design; Minotaurs perform advanced tending and field labor.",
            "Elf -> Centaur is the canonical Aura Beach and Current Sea regional Synthesis design; Centaurs roam Aura Beach and guard the Current Sea.",
            "Sandmanor proves regional role and recipe logic; Glaushouse clears Synthesis; Stonebend preserves identity; Flynt recognizes lawful role or institution.",
        ],
    },
    HuemanSectionDefinition {
        title: "Hueman Soul Faculties",
        lines: &[
            "Sandmanor is the Hueman Soul, divided equally into Minorian/Gnome Soul Interior -> Prefog and Minoan/Elf Soul Exterior -> Prefig.",
            "Prefog opens multiple legal possibilities; Prefig gives the selected legal possibility provisional or demonstrable form; neither is proof and neither independently executes Synthesis.",
            "the reciprocal cycle is Prefog -> Prefig -> Proof -> Evidence or Failure -> Revision -> Prefog, with all failed Prefig evidence preserved.",
            "only the existing proof lifecycle may advance a Prefig source through demonstration, reproduction, proof, credential, or standard status.",
            "Minotaur is cultivated Prefog at Aura Field; Centaur is embodied Prefig at Aura Beach and Current Sea; both are mature regional manifestations rather than new peoples or authorities.",
        ],
    },
    HuemanSectionDefinition {
        title: "Proof Law",
        lines: &[
            "every design names its author or lawful design body, purpose, assumptions, method, risks, failure states, measurement plan, version, history, and scope.",
            "every proof identifies a claim, scope, evidence, demonstration, criticism, reproduction where required, and the exact version proved.",
            "failure is evidence and remains recorded; a prototype is not production; simulated output is not direct physical performance.",
            "a material revision preserves the prior version and receives renewed proof rather than laundering an older result.",
            "good-faith criticism remains in the record, and emergency proof expires or enters ordinary review.",
            "proof is not Stonebend Title, Glaushouse clearance or consent, Flynt recognition, ownership, office, or universal truth.",
        ],
    },
    HuemanSectionDefinition {
        title: "Contest of Improvement",
        lines: &[
            "one Minorian and one Minoan teach each other meaningful practice, design principle, method, observation, criticism, and tradition-specific skill.",
            "both candidates preserve baseline and final evidence, disclose assistance and conflicts, and submit to public questions and independent review.",
            "the audience judges documented improvement rather than selecting a favorite; review includes both traditions, teachers, affected citizens, evidence stewards, and conflict reviewers.",
            "insufficient improvement produces no winner; a genuine tie leaves the Sandman unresolved until a further reciprocal trial or later Contest; Contest fraud or teaching sabotage voids affected evidence or the result.",
        ],
    },
    HuemanSectionDefinition {
        title: "Sandman Office",
        lines: &[
            "The Sandman is the singular highest constitutional office and asks whether a claim has earned the right to be relied upon as proven.",
            "the candidate most transformed by learning from the other may accede only after a completed lawful Contest, resolved challenges, Stonebend Title, Flynt recognition, a public learning statement, and Seal.",
            "combat, heredity, wealth, popularity, transformation, recognition alone, or legacy progression never creates the office.",
            "during vacancy, one Minorian steward and one Minoan steward govern jointly without becoming Sandman or permanently altering the Constitution.",
        ],
    },
    HuemanSectionDefinition {
        title: "Education and Civilization",
        lines: &[
            "teaching creates a genuine opportunity for understanding, questions, correction, fair assessment, and preserved intellectual lineage.",
            "every credential references demonstrated assessment; hidden criteria and purchased or fabricated qualifications are Fraudulent Design.",
            "an apprenticeship records teaching obligations, work expectations, support, safety, assessment, credit, termination, and complaints; labor may not replace teaching.",
            "standards identify purpose, scope, evidence, public review, both traditions where relevant, implementation, transition, revision, and appeal.",
            "successor institutions inherit correction, attribution, student, research, failure, and proof obligations together with assets and reputation.",
            "Fraudulent Design is Sandmanor's signature constitutional offense.",
        ],
    },
    HuemanSectionDefinition {
        title: "Status",
        lines: &[
            "SANDMANOR_CONSTITUTION_V2.md is the ratified canonical Sandmanor law",
            "src/world/sandmanor.rs validates proof, Contest, accession, education, revision, failure, and regional design above frozen Current Runtime V2",
            "the common runtime still owns Bond lifecycle, signed Current and Aura, shared House decisions, common Synthesis causality, persistence, and replay",
            "Hueman and Godot may present Sandmanor records but may not create proof, authorship, mastery, credential, office, clearance, Title, recognition, or Synthesis outcome",
            "the universal recursion kernel remains isolated from Sandmanor-specific law",
        ],
    },
];

static STONEBEND_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static FLYNT_CONSTITUTION_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static GLAUSHOUSE_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();
static SANDMANOR_ROLE_SECTIONS_MARKDOWN: OnceLock<String> = OnceLock::new();

const STONEBEND_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Stonebend Roles",
    structural_rule: "Stonebend carries Name It and Craft through a three-part civic balance that belongs to Hueman's world layer and remains vertically integrated above Current Synthesis and Hollow Grove.",
    sections: &STONEBEND_ROLE_SECTIONS,
    sections_markdown_cache: &STONEBEND_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Stonebend roles belong to Hueman's civic layer. They do not replace HAL, Clouseau, or any Current Synthesis client boundary.",
};

const FLYNT_CONSTITUTION_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Flynt Constitution",
    structural_rule: "Tross is Flynt's sovereign executive; the unique Chimera is First Companion; urban and rural traditions remain complementary expressions of that one constitutional authority.",
    sections: &FLYNT_CONSTITUTION_SECTIONS,
    sections_markdown_cache: &FLYNT_CONSTITUTION_SECTIONS_MARKDOWN,
    boundary_reminder: "Current Synthesis owns canonical Flynt authority and validation. Hueman renders the hierarchy but does not reinterpret or duplicate it.",
};

const GLAUSHOUSE_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Glaushouse Roles",
    structural_rule: "Glaushouse clears lawful medicine and maintained Synthesis through a universal Nightingale foundation, equal Matron and Marshal branches, multiple Persephones, and one singular Prima Donna office while Glauspitals and Chromacord support care and evidence and Current Runtime V2 remains authoritative over common causality.",
    sections: &GLAUSHOUSE_ROLE_SECTIONS,
    sections_markdown_cache: &GLAUSHOUSE_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Glaushouse roles belong to Hueman's kingdom layer. They do not replace scene logic, procedural care systems, or any Current Synthesis client boundary.",
};

const SANDMANOR_ROLE_ARTIFACT: HuemanRoleArtifactDefinition = HuemanRoleArtifactDefinition {
    title: "Hueman Sandmanor Roles",
    structural_rule: "Sandmanor proves through equal Minorian interior and Minoan exterior traditions, preserves failure and criticism, and selects the singular Sandman through documented reciprocal improvement rather than force, popularity, inheritance, or transformation.",
    sections: &SANDMANOR_ROLE_SECTIONS,
    sections_markdown_cache: &SANDMANOR_ROLE_SECTIONS_MARKDOWN,
    boundary_reminder: "Sandmanor roles belong to Hueman's kingdom layer. They do not replace Fourway placement, scene logic, or any Current Synthesis client boundary.",
};

const STONEBEND_PROCEDURE_LINES: &[&str] = &[
    "Stonebend names: every constitutional act begins with a stable subject and scope.",
    "the Geralds provide civic standing; the Hypergiant preserves integrity; the Proliteriate supplies legitimacy; the Freemason institution executes lawful structural acts.",
    "Name identifies, Title authorizes, Mercury Mirror verifies, and Seal endures.",
    "naming does not prove a design, clear medicine or Synthesis, recognize competence, or grant public popularity.",
    "Title may never arise from transformation, recognition, clearance, legacy progression, or custody alone.",
    "Hollowing and Synthesis remain distinct constitutional acts.",
    "Illegal Hollowing is stopped, recorded, repaired where possible, and referred across House boundaries only for the acts those Houses own.",
    "renaming, succession, inheritance, and transformation preserve history, liabilities, and continuity.",
    "Tombstone preserves ended authority and prevents it from acting in replay.",
    "the Stonebend registry validates these rules above the frozen Current Synthesis and recursion layers.",
];

const FLYNT_PROCEDURE_LINES: &[&str] = &[
    "Recognize It and Engineering remain the Flynt procedure functions, turning capability into operation and field trust.",
    "Tross is the sovereign executive and the root of all Flynt constitutional authority.",
    "the same stable person is Tross = Mystery Man = Mr. X = presently maintained holder of Manticorp.",
    "the public chain is Tross -> Manticorp Institution, while the underground chain is Mystery Man/Mr. X -> The Gallows; both resolve to Tross.",
    "the Gallowry is the hidden headquarters of the Gallows, not an institution or alternate government.",
    "Flynt routes ascent through boardwalk risk, casino pressure, outer hunting expeditions, and public recognition instead of quiet technical certification.",
    "Flynt knowledge gates open through puzzle trails, treasure-hunt clues, and route memory rather than library inheritance or bloodline permission.",
    "Flynt's unique constitutional synthesis combines Gargoyle, Merman, and Werewolf into the one Chimera.",
    "Manticorp is both the formal military institution and a distinct maintained Synthesis Form beyond Chimera; ordinary personnel do not hold the Form.",
    "Bro White, The Beauty, and Cinderellaman remain distinct Basin leaders and divided Recipe custodians whose union is cooperation, not fusion.",
    "opal extraction follows the guarded line body rather than an unbounded field claim.",
    "regular current and holographic aura carry opal yield outward as Opal Oil without breaking the guarded line body.",
    "recipe discovery in Flynt is field work, hunt pressure, and route puzzle rather than quiet inheritance.",
    "transition pressure may be read through Current Synthesis route order, but no autonomous traversal is enabled.",
];

const GLAUSHOUSE_PROCEDURE_LINES: &[&str] = &[
    "Glaushouse clears: every intervention names subject, diagnosis, consent, capacity, procedure, operator, scope, time, risk, and recovery.",
    "Doctor Ratchet holds the singular Prima Donna office; Nurse House is one Persephone among a multiple balanced rank; Nightingale is the universal clinical foundation.",
    "Matron reads lived and Aura continuity; Marshal holds bodily and Current continuity; Persephone preserves the whole patient.",
    "the sitting Prima Donna must keep Nightingale education, equal branches, cross-training, Ledger access, Recipe education, research, and candidacy open.",
    "Nightingales protect patient wishes and may issue an immediate stop that triggers mandatory review.",
    "Glauspitals operates clinical facilities and Chromacord preserves evidence; neither application nor chart declares Clearance.",
    "Glaushouse runs recovery through cold-lit clinic bays, industrial tooling, strict presentation, and visibly enforced standards.",
    "jade extraction feeds Glaus Gel, the jade-colored binder that supports bonding, sealing, cooling, repair, and controlled synthesis without displacing the human medical floor.",
    "lawful Synthesis requires explicit Synthesis consent, active Clearance, privilege, lawful provenance, accurate outcome, preserved identity, and completed recovery.",
    "Synthesis is Continuance through renewal; major Synthesis also requires technical and lived viability.",
    "Illegal Synthesis is stopped, reviewed, repaired where possible, and referred across House boundaries only for the acts each House owns.",
    "no clinical record, gesture, transformation, recognition, Title, proof, legacy state, or technical ability may silently substitute for another constitutional act.",
    "the Glaushouse registry enforces House-specific law without rewriting Current Runtime V2 or the recursion kernel.",
];

const SANDMANOR_PROCEDURE_LINES: &[&str] = &[
    "Prove It and Configuration remain the Sandmanor procedure functions, split between Minorian count and Minoan arrangement.",
    "selection identifies the rival public frame and consequence names the witnessed improvement result.",
    "the most improved rival becomes The Sandman.",
    "the Sandman accedes only through the completed evidence-judged Contest of Improvement, resolved challenges, Stonebend Title, Flynt recognition, public learning statement, and Seal.",
    "crystal harvest feeds Prism Sand and Prismiron, which carry the stewarded proof branch both equal civic traditions must respect.",
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
        start_path: ["Aura Basin", "Aura Field", "Aura Beach"],
        presence_lines: FLYNT_PRESENCE_LINES,
        intent_lines: FLYNT_INTENT_LINES,
        role_artifact: &FLYNT_CONSTITUTION_ARTIFACT,
        lens_lines: GREMLIN_LENS_LINES,
        lens_overlay_title: "",
        lens_overlay_lines: &[],
    },
    HuemanAnchorDefinition {
        name: "Stonebend",
        direction: "North",
        archetype: "goblin",
        primary_scene: "Seam Market",
        start_path: ["Aura Field", "Aura Basin", "Aura Beach"],
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
        start_path: ["Aura Beach", "Aura Field", "Aura Basin"],
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
        start_path: ["Aura Beach", "Aura Basin", "Aura Field"],
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

fn render_sandmanor_guardian_and_succession_section() -> &'static str {
    "
## Guardian, Coast, and Succession Constitution

- The Minorian asks how a place is lived within; the Minoan asks how a place lives within the world.
- Aura Farm contains Aura Fields and Content Farm as physical and cultural cultivation inside one Minorian system.
- healthy Content Farms educate, preserve memory, cultivate skill, and nourish attention; exploitative practices may exhaust or deceive an audience without making all content production corrupt.
- the Minoan coast runs Free Aura Beach -> Southern Coast -> Current Break -> Minoan County Courthouse -> Glaushouse, with regulation increasing gradually southward.
- Current Break is Sandmanor territory that hosts Flynt-authorized Manticorp training; Flynt retains Manticorp command and Minoan guardians teach coastal survival.
- the Minoan County Courthouse remains a Sandmanor institution when it transfers a patient or detainee into Glaushouse clinical care.
- Gnome -> Minotaur -> Hecaton and Elf -> Centaur -> Pegasus require service proof, Recipe authorization, Glaushouse compatibility, maintained Synthesis, investiture, and renewal.
- Minotaur carries Guardian of the Fields; Hecaton carries Guardian of the Whole Farm; Centaur carries Guardian of the Beach; Pegasus carries Guardian of the Horizon.
- the Form remains a body and the mantle remains a trust; suspension or removal ends authority without automatically erasing the maintained Form.
- the Contest final pair is one lawful Hecaton and one lawful Pegasus who sincerely teach each other and complete Aura Field, Content Farm, Liberty and Hospitality, Rescue and Horizon, and Reciprocal Integration trials.
- the audited crowd measures improvement from each candidate's own baseline; candidate order, insertion order, original mastery, duplicate ballots, and conflicts cannot choose the winner.
- a valid tie leaves the Sandman unresolved; teaching sabotage voids affected evidence or the responsible candidate.
- the winner remains one stable person, undergoes lawful maintained Sandman convergence, and may receive the one active sovereign mantle; the losing guardian keeps their Form and mantle unless separately removed.
- Long ago, the sovereign was called Aegon. Today, the same sovereign identity and tradition is known as the Sandman.
- Sandmanor remains one Design pole and one official Design Index; interior/cultivated and exterior/coastal design are complementary subdisciplines, not separate currencies or indexes.
"
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
             ## Flynt Constitutional Presence\n\n\
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
             - Aura Field hinge pressure can therefore feel structurally doubled rather than merely crowded\n\
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
             ## Flynt Constitutional Intent\n\n\
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
             - Current Synthesis owns its frozen route execution semantics, activation gating, and client boundaries like HAL and Clouseau\n\
             - constitutional geography owns the world-facing route-purpose roster above Current Synthesis without feeding meaning downward\n\
             - Hueman owns Fourway, AuraTriad reading, Hueman-first opening placement, civic projections, and scene reading\n\n\
             ## Current Alignment\n\n\
             - Stonebend remains North: Geralds are its people, Hypergiant temporarily bears Diamond, Proliteriate is its distributed Yield network, and High Freemason bears Claim authority above Current Synthesis\n\
             - Tross = Mystery Man = Mr. X is one stable sovereign person; Tross publicly leads Manticorp Institution, while the same man leads the Gallows underground; Manticorp Form is his presently maintained body\n\
             - Glaushouse remains East: Nightingale is the universal clinical foundation, Matron and Marshal are equal branches, multiple Persephones preserve whole-course care, Doctor Ratchet holds the one singular Prima Donna office, Glauspitals operates care, and Chromacord preserves evidence without becoming authority\n\
             - Sandmanor remains South with equal Minorian Gnome and Minoan Elf traditions, regional Gnome-to-Minotaur and Elf-to-Centaur designs, and the evidence-judged Contest of Improvement\n\
             - Hueman faculties remain an additive typed layer: Stonebend/Body/Presynce, Flynt/Spirit/Resynce, Glaushouse/Mind/Precog, and equal Sandmanor Soul halves Minorian/Prefog and Minoan/Prefig\n\
             - faculties describe Observe, Generate, Evaluate, and Execute-or-demonstrate; the existing deterministic runtime alone Chooses, and the existing Sandmanor proof lifecycle alone proves\n\
             - functionally, Stonebend reads as Craft, Flynt reads as Engineering, Glaushouse reads as Repair, and Sandmanor reads as Configuration across the shared Hollow Grove/Hueman interpretation\n\
             - the player begins as Hueman near Aura Ridge before major form commitment\n\
             - confirmed form origins remain braided: Flynt -> `gremlin`, Stonebend -> `goblin`, Glaushouse -> `sprite`, Sandmanor -> `pixy`\n\
             - resource seams are designated across Hueman: Stonebend diamonds, Flynt opals, Glaushouse jades, Sandmanor crystals and proof materials\n\
             - the bedrock split remains active upstream: regular current and hollow current, reflective aura and holographic aura\n\
             - the visible Hueman map remains one large circle whose route legs may read as straight ridge runs or rounded bends\n\
             - Aura Ridge / Witness is the visible civic-reintegration body between Glaushouse and Stonebend, while Aura Ridge East remains only a local screen-map segment toward Sandmanor\n\
             - Current Sea / Certify remains the separate Glaushouse-to-Stonebend depth ordeal and has no frozen Current Synthesis route projection\n\
             - all four houses share the same ascent loop: gather materials, uncover recipe knowledge, pass through the house process, and embody the resulting form without collapsing form into office\n\
             - Stonebend declares Mercury Mirror from hollow current + reflective craft, Flynt declares Opal Oil from regular current + holographic aura, Glaushouse declares Glaus Gel from jade as repair and synthesis medium, and Sandmanor declares Prism Sand from crystal as the proof-and-record branch\n\
             - the outer border ring remains legible as Stairway to Heaven, Riptide, Current Seanad, and Mt. Aura\n\
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
             - Stonebend and Glaushouse cross at Aura Field.\n\
             - Stonebend and Sandmanor cross at Aura Basin after different openings.\n\n\
             ## Aura Ridge Trade Legs\n\n\
             - Aura Ridge / Witness is the elevated public route between restored Glaushouse life and the Stonebend-facing Central Junction district.\n\
             - Central Junction contains the South Ridge Exchange, Junction Board, Clearing House, and Junction Wire; it is not merely a stock exchange.\n\
             - the Craft Corridor leads toward Stonebend, the Repair Corridor toward Glaushouse, the Design Corridor toward Sandmanor, and Flynt powers the district through its Engineering Ring and infrastructure network.\n\
             - Sandmanor designs Form, Flynt engineers Function, Stonebend makes Form endure, and Glaushouse keeps Function alive.\n\
             - one unnamed standard currency measures ordinary exchange; the four public indexes are noncurrency measurements, Toke/Tokens remain earned evidence, and Gremlincoin remains the Gremlin Way.\n\
             - House Sector Halls verify professional facts without setting prices; the Junction Board governs shared market standards, the Clearing House settles recognized contracts, and the Junction Wire publishes results.\n\
             - Current Haze remains unresolved possibility, Equal Gaze remains reconciled perspective, and Aura Beam remains revelation or transmission of a visible shared future rather than a market ticker.\n\
             - Equal Gaze, ceremony, tourism, trade, and public reintegration remain visible along the ridge.\n\
             - Aura Ridge East remains a local screen-map segment toward Sandmanor rather than another major constitutional route.\n\
             - Current Sea / Certify is the separate northern ordeal and may not be collapsed into Aura Ridge.\n\
             - the ridge remains part of the same large circular map body rather than a universal holding route.\n\n\
             ## Confirmed Route Law\n\n\
             - routes are constitutional verbs rather than mere connections.\n\
             - straight routes remain process geometry and curved routes remain transformation geometry.\n\
             - Flynt <-> Glaushouse uses Boardwalk / Return and Riptide / Retrieve.\n\
             - Glaushouse <-> Stonebend keeps Current Sea / Certify separate from Aura Ridge / Witness.\n\
             - Glaushouse <-> Sandmanor uses Glausbahn / Refine and Current Seanad / Deliberate.\n\
             - Stonebend <-> Sandmanor uses Aura Way / Design and Mt. Aura / Aspire.\n\
             - Stonebend <-> Flynt uses Basin Motor Speedway / Produce and Stairway to Heaven / Ascend.\n\n\
             ## Relay Junction\n\n\
             - the HAL/Cleo relay packet declares `P/M -> L/E` as one shared confirmation crossing.\n\
             - Hueman reads that packet upward as the same kind of shared junction pressure seen at the Aura Field hinge.\n\
             - the visible Aura Ridge hinge and the underground inverse crossing may therefore be treated as one witnessed overlap body.\n\
             - crossover identity remains world-facing while packet ownership stays inside Current Synthesis.\n\n\
             ## Full-Triad Convergence\n\n\
             - all four starts eventually touch Aura Basin\n\
             - all four starts eventually touch Aura Field\n\
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
             - fits shared Aura Field crossings best\n\
             - commonly appears at the Aura Field junction where the Stonebend/Glaushouse ridge meets Sandmanor's straight continuation\n\n\
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
             - Aura Field tends toward Seam Market first.\n\
             - Aura Basin tends toward Pressure Shelter first.\n\
             - Split Trace can appear in any crossover zone where the bias remains unresolved.\n\n\
             ## Relay Scene Use\n\n\
             - the HAL/Cleo relay packet lets one crossover scene keep both visible alignment and underground continuity as one witnessed scene body\n\
             - Seam Market benefits most directly because the Aura Field hinge can hold trade pressure above and structural continuity below at the same time\n\
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
         Aura Field\n\
         ↓\n\
         Aura Beach\n\
         ↓\n\
         Aura Basin\n\
         ```\n\n\
         ## Meaning\n\n\
         - AuraTriad is the world-facing three-region route study beneath Fourway.\n\
         - Current Synthesis already records these as lower route regions.\n\
         - Hueman reads them as a useful triadic resolution of the world map rather than the whole constitution.\n\
         - AuraTriad closes back into the same large circle, so Aura Basin, Aura Field, and Aura Beach can be revisited without breaking the surface map.\n\
         - AuraTriad should support opening movement, crossover, and atmosphere studies without becoming a universal holding pen.\n\
         - Triway remains the lower recursive split after this layer.\n\n\
         ## Regional Roles\n\n\
         - Aura Basin serves Flynt first: Gargoyle-versus-Werewolf hunting grounds, den pressure, rare encounters, and the nearest hidden body rising into Aura Ridge circulation.\n\
         - Aura Field serves public work first: farming, Stonebend hunt tradition, Minorian proof, and market-facing comparison.\n\
         - Aura Beach serves exterior proof first: Minoan coastal access, navigation, public approach, training, recovery, and threshold exposure.\n\
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
         Aura Field\n\
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

pub fn build_hueman_flynt_constitution_from_artifacts(
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
    output.push_str(render_sandmanor_guardian_and_succession_section());
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
    hueman_flynt_constitution: &str,
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
         Hueman Flynt Constitution bytes: {}.\n\
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
        hueman_flynt_constitution.len(),
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
         - the visible border names the same rim completed by Stairway to Heaven, Riptide, Current Seanad, and Mt. Aura\n\n\
         ## Border Sequence\n\n\
         - The Stairway to Heaven\n\
         - The Riptide\n\
         - The Current Seanad\n\
         - Mt. Aura\n\n\
         ## Underground Inverse Curves\n\n\
         - four inverse curved lines run underground on the `PLEB` side\n\
         - four inverse curved lines run underground on the `META` side\n\
         - each side mirrors Stairway to Heaven, Riptide, Current Seanad, and Mt. Aura as underground inverse structure\n\
         - the underground curves remain inverse to the visible rim rather than replacing it\n\n\
         ## Underground Reading\n\n\
         - The Stairway to Heaven reads as Ascend: the rising outer acceptance of higher burden along the circle's rim.\n\
         - beneath it, an inverse underground curve carries hidden descent pressure on both sides.\n\
         - The Riptide reads as Retrieve: emergency undertow carries crises toward Glaushouse intake, with the visible water rim kept alive as Flynt's Merman range.\n\
         - beneath it, an inverse underground curve carries hidden return pull on both sides beneath that roaming seam.\n\
         - The Current Seanad reads as Deliberate: the outer court holds evidence and difficult design or repair questions for judgment.\n\
         - beneath it, an inverse underground curve carries hidden current understructure on both sides.\n\
         - Mt. Aura reads as Aspire: the bright outer curve carries higher aims from Stonebend to Sandmanor along the circle's rim.\n\
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
    hueman_flynt_constitution: &str,
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
        "Hueman Flynt Constitution bytes",
        hueman_flynt_constitution.len(),
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
    hueman_flynt_constitution: &str,
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
        "Hueman Flynt Constitution bytes",
        hueman_flynt_constitution.len(),
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
        build_hueman_crossover_scenes_from_artifacts,
        build_hueman_flynt_constitution_from_artifacts, build_hueman_fourway_from_artifacts,
        build_hueman_glaushouse_roles_from_artifacts, build_hueman_inverse_circle_from_artifacts,
        build_hueman_link_physics_from_artifacts, build_hueman_motion_map_from_artifacts,
        build_hueman_path_crossovers_from_artifacts, build_hueman_procedural_uplift_from_artifacts,
        build_hueman_sandmanor_roles_from_artifacts, build_hueman_scene_drift_from_artifacts,
        build_hueman_scene_intent_from_artifacts, build_hueman_scene_presence_from_artifacts,
        build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
        build_hueman_stonebend_roles_from_artifacts,
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
        assert!(output.contains("the Geralds are Stonebend's constitutional people"));
        assert!(output.contains("the Hypergiant is the singular highest Stonebend office"));
        assert!(
            output.contains("the Proliteriate is Stonebend's permanent distributed public network")
        );
        assert!(
            output.contains("Illegal Hollowing is Stonebend's signature constitutional offense")
        );
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_flynt_constitution_builder_is_deterministic() {
        let output = build_hueman_flynt_constitution_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Flynt Constitution"));
        assert!(output.contains("Tross is the sovereign executive of Flynt"));
        assert!(output.contains("There is exactly one constitutional Chimera"));
        assert!(output.contains("The public hierarchy is Tross -> Manticorp Institution"));
        assert!(output.contains("The Gallowry is the hidden headquarters"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_glaushouse_roles_builder_is_deterministic() {
        let output = build_hueman_glaushouse_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Glaushouse Roles"));
        assert!(output.contains("Glaushouse clears medicine, recovery, and lawful Synthesis"));
        assert!(output.contains("Sprite is Glaushouse's confirmed Aura-origin path."));
        assert!(output.contains("Doctor Ratchet is its frozen current holder identity"));
        assert!(output.contains("every Nightingale stop triggers mandatory review"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_sandmanor_roles_builder_is_deterministic() {
        let output = build_hueman_sandmanor_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Sandmanor Roles"));
        assert!(output.contains("Sandmanor proves: design proposes"));
        assert!(output.contains("Pixy is Sandmanor's confirmed Aura-origin path."));
        assert!(output.contains("The Sandman is the singular highest constitutional office"));
        assert!(output.contains("Gnome -> Minotaur is the canonical Aura Field"));
        assert!(output.contains("Fraudulent Design is Sandmanor's signature"));
        assert!(output.contains("Gnome -> Minotaur -> Hecaton"));
        assert!(output.contains("called Aegon"));
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
            "flynt constitution",
            "glaushouse",
            "sandmanor",
        );
        assert!(output.starts_with("# Hueman Procedural Uplift"));
        assert!(output.contains("## Bottom-Up Procedure Spine"));
        assert!(output.contains("## Relay Procedure"));
        assert!(output.contains("Manticorp is both the formal military institution and a distinct maintained Synthesis Form"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_archetype_lens_builder_is_deterministic() {
        let output =
            build_hueman_archetype_lens_from_artifacts("start", "aura", "roles", "sandmanor");
        assert!(output.starts_with("# Hueman Archetype Lens"));
        assert!(output.contains("each confirmed origin path may read the same regions"));
        assert!(output.contains("### `gremlin`"));
        assert!(output.contains("goblin form reading may notice structure and burden"));
        assert!(output.contains("does not make forms into hereditary races"));
    }

    #[test]
    fn hueman_start_paths_builder_is_deterministic() {
        let output = build_hueman_start_paths_from_artifacts("start", "lens");
        assert!(output.starts_with("# Hueman Start Paths"));
        assert!(output.contains("while the player still begins as Hueman near Aura Ridge"));
        assert!(output.contains("Flynt-facing approach = Aura Basin -> Aura Field -> Aura Beach"));
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
        assert!(output.contains("Mt. Aura"));
        assert!(output.contains(
            "each side mirrors Stairway to Heaven, Riptide, Current Seanad, and Mt. Aura"
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
            output.contains("Aura Field hinge pressure can therefore feel structurally doubled")
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
