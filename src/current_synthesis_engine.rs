use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use crate::current_synthesis_scenario::{DEFAULT_SCENARIO_ID, ScenarioDefinition, load_scenario};
use crate::hollow_grove_contract::{
    build_hollow_grove_alignment_validation_report, build_hollow_grove_alignment_witness,
    build_world_context_output as build_contract_world_context_output,
};
use crate::hueman_progression::{
    HUEMAN_SLICE_STATE_ARTIFACT_PATH, SlicePhase, parse_vertical_slice_state,
};
use crate::{ArtifactSession, write_text_artifact};

pub const CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_tui_state.txt";
pub const CURRENT_SYNTHESIS_ENGINE_STATUS_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_engine_status.md";
pub const CURRENT_SYNTHESIS_WORLD_CONTEXT_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_world_context.md";
pub const CURRENT_SYNTHESIS_ALIGNMENT_WITNESS_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_alignment_witness.md";
pub const CURRENT_SYNTHESIS_ALIGNMENT_VALIDATION_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_alignment_validation.md";
pub const CURRENT_SYNTHESIS_BOND_INSPECTOR_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_bond_inspector.md";
pub const CURRENT_SYNTHESIS_RESOURCE_INSPECTOR_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_resource_inspector.md";
pub const CURRENT_SYNTHESIS_NPC_INSPECTOR_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_npc_inspector.md";
pub const CURRENT_SYNTHESIS_CLEOPATRA_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_cleopatra.md";
pub const CURRENT_SYNTHESIS_SNAPSHOT_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_snapshot.txt";
pub const CURRENT_SYNTHESIS_EVENT_LOG_ARTIFACT_PATH: &str =
    "artifacts/current_synthesis_events.txt";

const DEFAULT_SEED: u64 = 7;
const CURRENT_SYNTHESIS_CHECKPOINT_INTERVAL: usize = 4;
const PLAYER_ACTION_MOVE_PREFIX: &str = "move::";
const PLAYER_ACTION_DECIDE_PREFIX: &str = "decide::";
const PLAYER_ACTION_SUPPORT_PREFIX: &str = "support::";

pub type PropertyMap = BTreeMap<String, u16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticSide {
    Left,
    Right,
}

impl SemanticSide {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    pub fn family_name(self) -> &'static str {
        match self {
            Self::Left => "Aura",
            Self::Right => "Current",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidueDestination {
    Aether,
    Bathos,
}

impl ResidueDestination {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aether => "Aether",
            Self::Bathos => "Bathos",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineWay {
    pub id: String,
    pub source: String,
    pub direction: String,
    pub side: SemanticSide,
    pub properties: PropertyMap,
    pub availability: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondCandidate {
    pub id: String,
    pub participants: Vec<String>,
    pub selected_arms: Vec<String>,
    pub side: SemanticSide,
    pub properties: PropertyMap,
    pub viability: u16,
    pub cost: u16,
    pub source_need: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondResult {
    pub selected_bond: BondCandidate,
    pub resulting_link: String,
    pub resulting_moment: String,
    pub unused_bonds: Vec<BondCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Residue {
    pub source_bond: String,
    pub side: SemanticSide,
    pub properties: PropertyMap,
    pub destination: ResidueDestination,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResourceComposition {
    pub aura_total: u16,
    pub aura_properties: PropertyMap,
    pub current_total: u16,
    pub current_properties: PropertyMap,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorldState {
    pub route_stability: u16,
    pub shelter_integrity: u16,
    pub power_stability: u16,
    pub labor_availability: u16,
    pub faction_tension: u16,
    pub conflict_risk: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpcState {
    pub id: String,
    pub name: String,
    pub role: String,
    pub faction: String,
    pub location: String,
    pub needs: Vec<String>,
    pub memories: Vec<String>,
    pub relationships: Vec<String>,
    pub condition: String,
    pub perceived_world: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BLEPDecision {
    pub npc_id: String,
    pub world_inputs: Vec<String>,
    pub inferred_need: String,
    pub candidate_bonds: Vec<BondCandidate>,
    pub selected_bond: BondCandidate,
    pub resulting_action: String,
    pub unused_bonds: Vec<BondCandidate>,
    pub confidence: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleopatraState {
    pub active_npcs: Vec<String>,
    pub queued_blep_passes: Vec<String>,
    pub faction_conditions: Vec<String>,
    pub settlement_conditions: Vec<String>,
    pub war_conditions: Vec<String>,
    pub global_resource_composition: ResourceComposition,
    pub recent_blep_passes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardSynthesisPass {
    pub human_need: String,
    pub machine_complement: String,
    pub available_ways: Vec<EngineWay>,
    pub candidate_bonds: Vec<BondCandidate>,
    pub bond_result: BondResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedBlepRelay {
    pub npc_id: String,
    pub npc_moment_id: String,
    pub committed: bool,
    pub blep_decision: BLEPDecision,
    pub blep_residues: Vec<Residue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickRecord {
    pub tick_number: u32,
    pub player_moment_id: String,
    pub npc_moment_id: String,
    pub forward_pass: ForwardSynthesisPass,
    pub forward_residues: Vec<Residue>,
    pub blep_decision: BLEPDecision,
    pub blep_residues: Vec<Residue>,
    pub coordinated_blep_relays: Vec<CoordinatedBlepRelay>,
    pub resources_after_tick: ResourceComposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSynthesisState {
    scenario_id: String,
    focused_npc_id: String,
    seed: u64,
    completed_ticks: u32,
    player_need: String,
    planned_player_actions: Vec<String>,
    resources: ResourceComposition,
    world: WorldState,
    npcs: Vec<NpcState>,
    cleopatra: CleopatraState,
    history: Vec<TickRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedCurrentSynthesisState {
    pub scenario_id: String,
    pub seed: u64,
    pub completed_ticks: u32,
    pub focused_npc_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuemanFeedback {
    pub unlocked: bool,
    pub recognized_route_branch: bool,
    pub recognized_defense_branch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewArtifact {
    pub path: PathBuf,
    pub contents: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerActionKind {
    Move,
    Decide,
    Support,
    General,
}

impl PlayerActionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Decide => "decide",
            Self::Support => "support",
            Self::General => "plan",
        }
    }

    fn posture_label(self) -> &'static str {
        match self {
            Self::Move => "movement-first",
            Self::Decide => "decision-first",
            Self::Support => "support-first",
            Self::General => "general-pressure",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerActionDirective {
    kind: PlayerActionKind,
    detail: String,
    target: Option<String>,
    traits: BTreeMap<String, String>,
    schema: PlayerActionSchema,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerActionProfile {
    directives: Vec<PlayerActionDirective>,
    move_count: usize,
    decision_count: usize,
    support_count: usize,
    general_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PlayerActionSchema {
    Move(MoveActionSpec),
    Decide(DecideActionSpec),
    Support(SupportActionSpec),
    General,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteLane {
    Ring,
    Diamond,
    Spine,
    SandmanorBranch,
    StonebendGlaushouseLeg,
    QuarryRim,
    WesternRoad,
    IntakeLine,
    ShelterLane,
    General,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteLineName {
    AuraRidge,
    AuraWay,
    BasinMotorspeedway,
    Boardwalk,
    Glausbahn,
    StairwayToHeaven,
    Riptide,
    CurrentSea,
    MountAura,
    QuarryRim,
    WesternRoad,
    IntakeLine,
    Shelterline,
    General,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteShape {
    Curved,
    Straight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteSettlement {
    Flynt,
    Glaushouse,
    Stonebend,
    Sandmanor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RouteLineDefinition {
    line: RouteLineName,
    shape: RouteShape,
    lane: RouteLane,
    from: RouteSettlement,
    to: RouteSettlement,
    surface_custodian: RouteCustodian,
    inverse_custodian: Option<RouteCustodian>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResolvedMoveRoute {
    definition: RouteLineDefinition,
    from: RouteSettlement,
    to: RouteSettlement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedDecisionContext {
    site_slug: String,
    site_name: String,
    domain_slug: &'static str,
    domain_name: &'static str,
    route: Option<ResolvedMoveRoute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedSupportContext {
    site_slug: String,
    site_name: String,
    beneficiary_name: Option<String>,
    route: Option<ResolvedMoveRoute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteCustodian {
    Hal,
    Clouseau,
    Cleopatra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovePace {
    Careful,
    Balanced,
    Fast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveMethod {
    Traverse,
    Scout,
    Flank,
    Tunnel,
    Carry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveStance {
    Quiet,
    Steady,
    Forceful,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MoveActionSpec {
    from: Option<String>,
    to: Option<String>,
    lane: RouteLane,
    line: RouteLineName,
    pace: MovePace,
    method: MoveMethod,
    stance: MoveStance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecideFocus {
    Route,
    Power,
    Shelter,
    Alliance,
    Conflict,
    Labor,
    General,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecideCommitment {
    Hold,
    Shift,
    Commit,
    Withdraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecideAuthority {
    Solo,
    Stonebend,
    Glaushouse,
    Sandmanor,
    Shared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecideSignal {
    Quiet,
    Public,
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DecideActionSpec {
    focus: DecideFocus,
    commitment: DecideCommitment,
    authority: DecideAuthority,
    signal: DecideSignal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportAsset {
    Pump,
    Crane,
    Shelter,
    Route,
    Crew,
    Power,
    Intake,
    Bridge,
    General,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportFront {
    Route,
    Shelter,
    Power,
    Labor,
    General,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportIntensity {
    Light,
    Balanced,
    Heavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportDuration {
    Burst,
    Hold,
    Extended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SupportActionSpec {
    asset: SupportAsset,
    beneficiary: Option<String>,
    front: SupportFront,
    intensity: SupportIntensity,
    duration: SupportDuration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StateSnapshot {
    applied_event_count: usize,
    state: CurrentSynthesisState,
    hueman_feedback: HuemanFeedback,
}

#[derive(Debug, Default)]
struct SnapshotTickBuilder {
    player_moment_id: Option<String>,
    npc_moment_id: Option<String>,
    forward_human_need: Option<String>,
    forward_machine_complement: Option<String>,
    forward_available_ways: Vec<EngineWay>,
    forward_candidate_bonds: Vec<BondCandidate>,
    forward_selected_bond_id: Option<String>,
    forward_resulting_link: Option<String>,
    forward_resulting_moment: Option<String>,
    forward_unused_bonds: Vec<BondCandidate>,
    forward_residues: Vec<Residue>,
    blep_npc_id: Option<String>,
    blep_world_inputs: Vec<String>,
    blep_inferred_need: Option<String>,
    blep_candidate_bonds: Vec<BondCandidate>,
    blep_selected_bond_id: Option<String>,
    blep_resulting_action: Option<String>,
    blep_unused_bonds: Vec<BondCandidate>,
    blep_confidence: Option<u16>,
    blep_residues: Vec<Residue>,
    relay_order: Vec<String>,
    coordinated_relays: BTreeMap<String, SnapshotRelayBuilder>,
    resources_after_tick: ResourceComposition,
}

#[derive(Debug, Default)]
struct SnapshotRelayBuilder {
    npc_moment_id: Option<String>,
    committed: Option<bool>,
    world_inputs: Vec<String>,
    inferred_need: Option<String>,
    candidate_bonds: Vec<BondCandidate>,
    selected_bond_id: Option<String>,
    resulting_action: Option<String>,
    unused_bonds: Vec<BondCandidate>,
    confidence: Option<u16>,
    residues: Vec<Residue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentSynthesisEvent {
    ScenarioSelected {
        scenario_id: String,
        seed: u64,
        focused_npc_id: String,
    },
    FocusedNpcChanged {
        focused_npc_id: String,
    },
    PlayerActionPlanned {
        action_label: String,
    },
    HuemanFeedbackChanged {
        hueman_feedback: HuemanFeedback,
    },
    CleopatraTicked {
        focused_npc_id: String,
        hueman_feedback: HuemanFeedback,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineLens {
    Status,
    Pleb,
    Meta,
    Blep,
}

impl EngineLens {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "status" => Some(Self::Status),
            "pleb" => Some(Self::Pleb),
            "meta" => Some(Self::Meta),
            "blep" => Some(Self::Blep),
            _ => None,
        }
    }
}

impl ResourceComposition {
    pub fn empty() -> Self {
        Self {
            aura_total: 0,
            aura_properties: PropertyMap::new(),
            current_total: 0,
            current_properties: PropertyMap::new(),
        }
    }

    fn apply_residue(&mut self, residue: &Residue) {
        match residue.side {
            SemanticSide::Left => add_properties(&mut self.aura_properties, &residue.properties),
            SemanticSide::Right => {
                add_properties(&mut self.current_properties, &residue.properties)
            }
        }
        self.recalculate_totals();
    }

    fn recalculate_totals(&mut self) {
        self.aura_total = property_total(&self.aura_properties);
        self.current_total = property_total(&self.current_properties);
    }
}

impl WorldState {
    fn for_scenario(scenario_id: &str) -> Self {
        match scenario_id {
            "flooded_quarry_night_watch" => Self {
                route_stability: 34,
                shelter_integrity: 56,
                power_stability: 39,
                labor_availability: 45,
                faction_tension: 58,
                conflict_risk: 54,
            },
            "scout_valley_vertical_slice" => Self {
                route_stability: 41,
                shelter_integrity: 37,
                power_stability: 52,
                labor_availability: 43,
                faction_tension: 49,
                conflict_risk: 42,
            },
            _ => Self {
                route_stability: 50,
                shelter_integrity: 50,
                power_stability: 50,
                labor_availability: 50,
                faction_tension: 50,
                conflict_risk: 50,
            },
        }
    }

    fn active_consequences(&self) -> Vec<String> {
        let mut consequences = Vec::new();
        if self.route_stability <= 35 {
            consequences.push(String::from(
                "route stability is brittle enough to collapse if labor is pulled away",
            ));
        } else if self.route_stability >= 70 {
            consequences.push(String::from(
                "route continuity is holding long enough for coordinated movement",
            ));
        }
        if self.shelter_integrity <= 35 {
            consequences.push(String::from(
                "shelter integrity is close to failing under the next pressure wave",
            ));
        } else if self.shelter_integrity >= 70 {
            consequences.push(String::from(
                "shelter lanes are stabilized enough for sustained intake",
            ));
        }
        if self.power_stability <= 35 {
            consequences.push(String::from(
                "power stability is near blackout and may sever route support",
            ));
        } else if self.power_stability >= 70 {
            consequences.push(String::from(
                "power continuity is strong enough to support extended operations",
            ));
        }
        if self.labor_availability <= 35 {
            consequences.push(String::from(
                "labor bandwidth is strained and every reassignment now costs another front",
            ));
        }
        if self.faction_tension >= 65 {
            consequences.push(String::from(
                "faction tension is degrading negotiation and pushing actors toward force",
            ));
        } else if self.faction_tension <= 35 {
            consequences.push(String::from(
                "faction tension has eased enough to keep alliance channels open",
            ));
        }
        if self.conflict_risk >= 70 {
            consequences.push(String::from(
                "conflict risk is nearing release and could trigger a live grab or raid",
            ));
        }
        if consequences.is_empty() {
            consequences.push(String::from(
                "world pressure is unstable but still recoverable across all fronts",
            ));
        }
        consequences
    }

    fn summary_lines(&self) -> String {
        format!(
            "- route stability: {}\n- shelter integrity: {}\n- power stability: {}\n- labor availability: {}\n- faction tension: {}\n- conflict risk: {}\n",
            self.route_stability,
            self.shelter_integrity,
            self.power_stability,
            self.labor_availability,
            self.faction_tension,
            self.conflict_risk
        )
    }
}

impl RouteLane {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ring => "ring",
            Self::Diamond => "diamond",
            Self::Spine => "spine",
            Self::SandmanorBranch => "sandmanor-branch",
            Self::StonebendGlaushouseLeg => "stonebend-glaushouse-leg",
            Self::QuarryRim => "quarry-rim",
            Self::WesternRoad => "western-road",
            Self::IntakeLine => "intake-line",
            Self::ShelterLane => "shelter-lane",
            Self::General => "general",
        }
    }
}

impl RouteLineName {
    fn as_str(self) -> &'static str {
        match self {
            Self::AuraRidge => "aura-ridge",
            Self::AuraWay => "aura-way",
            Self::BasinMotorspeedway => "basin-motorspeedway",
            Self::Boardwalk => "boardwalk",
            Self::Glausbahn => "glausbahn",
            Self::StairwayToHeaven => "stairway-to-heaven",
            Self::Riptide => "riptide",
            Self::CurrentSea => "current-seanad",
            Self::MountAura => "mnt-aura",
            Self::QuarryRim => "quarry-rim",
            Self::WesternRoad => "western-road",
            Self::IntakeLine => "intake-line",
            Self::Shelterline => "shelterline",
            Self::General => "general",
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            Self::AuraRidge => "Aura Ridge",
            Self::AuraWay => "The Aura Way",
            Self::BasinMotorspeedway => "Basin Motorspeedway",
            Self::Boardwalk => "The Boardwalk",
            Self::Glausbahn => "The Glausbahn",
            Self::StairwayToHeaven => "The Stairway to Heaven",
            Self::Riptide => "The Riptide",
            Self::CurrentSea => "The Current Seanad",
            Self::MountAura => "Mnt. Aura",
            Self::QuarryRim => "Quarry Rim",
            Self::WesternRoad => "Western Road",
            Self::IntakeLine => "Intake Line",
            Self::Shelterline => "Shelterline",
            Self::General => "General",
        }
    }
}

impl MovePace {
    fn as_str(self) -> &'static str {
        match self {
            Self::Careful => "careful",
            Self::Balanced => "balanced",
            Self::Fast => "fast",
        }
    }
}

impl MoveMethod {
    fn as_str(self) -> &'static str {
        match self {
            Self::Traverse => "traverse",
            Self::Scout => "scout",
            Self::Flank => "flank",
            Self::Tunnel => "tunnel",
            Self::Carry => "carry",
        }
    }
}

impl MoveStance {
    fn as_str(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Steady => "steady",
            Self::Forceful => "forceful",
        }
    }
}

impl DecideFocus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Route => "route",
            Self::Power => "power",
            Self::Shelter => "shelter",
            Self::Alliance => "alliance",
            Self::Conflict => "conflict",
            Self::Labor => "labor",
            Self::General => "general",
        }
    }
}

impl DecideCommitment {
    fn as_str(self) -> &'static str {
        match self {
            Self::Hold => "hold",
            Self::Shift => "shift",
            Self::Commit => "commit",
            Self::Withdraw => "withdraw",
        }
    }
}

impl DecideAuthority {
    fn as_str(self) -> &'static str {
        match self {
            Self::Solo => "solo",
            Self::Stonebend => "stonebend",
            Self::Glaushouse => "glaushouse",
            Self::Sandmanor => "sandmanor",
            Self::Shared => "shared",
        }
    }
}

impl DecideSignal {
    fn as_str(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Public => "public",
            Self::Emergency => "emergency",
        }
    }
}

impl SupportAsset {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pump => "pump",
            Self::Crane => "crane",
            Self::Shelter => "shelter",
            Self::Route => "route",
            Self::Crew => "crew",
            Self::Power => "power",
            Self::Intake => "intake",
            Self::Bridge => "bridge",
            Self::General => "general",
        }
    }
}

impl SupportFront {
    fn as_str(self) -> &'static str {
        match self {
            Self::Route => "route",
            Self::Shelter => "shelter",
            Self::Power => "power",
            Self::Labor => "labor",
            Self::General => "general",
        }
    }
}

impl SupportIntensity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Balanced => "balanced",
            Self::Heavy => "heavy",
        }
    }
}

impl SupportDuration {
    fn as_str(self) -> &'static str {
        match self {
            Self::Burst => "burst",
            Self::Hold => "hold",
            Self::Extended => "extended",
        }
    }
}

impl RouteShape {
    fn as_str(self) -> &'static str {
        match self {
            Self::Curved => "curved",
            Self::Straight => "straight",
        }
    }
}

impl RouteSettlement {
    fn as_str(self) -> &'static str {
        match self {
            Self::Flynt => "flynt",
            Self::Glaushouse => "glaushouse",
            Self::Stonebend => "stonebend",
            Self::Sandmanor => "sandmanor",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        let normalized = value.to_ascii_lowercase().replace(['-', '_'], " ");
        if normalized.contains("flynt") {
            Some(Self::Flynt)
        } else if normalized.contains("glaushouse") {
            Some(Self::Glaushouse)
        } else if normalized.contains("stonebend") {
            Some(Self::Stonebend)
        } else if normalized.contains("sandmanor") {
            Some(Self::Sandmanor)
        } else {
            None
        }
    }
}

impl RouteCustodian {
    fn as_str(self) -> &'static str {
        match self {
            Self::Hal => "hal",
            Self::Clouseau => "clouseau",
            Self::Cleopatra => "cleopatra",
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            Self::Hal => "HAL",
            Self::Clouseau => "Clouseau",
            Self::Cleopatra => "Cleopatra",
        }
    }
}

impl ResolvedMoveRoute {
    fn line_slug(self) -> String {
        match self.definition.line {
            RouteLineName::AuraRidge => match (self.from, self.to) {
                (RouteSettlement::Stonebend, RouteSettlement::Glaushouse) => {
                    String::from("aura-ridge-south")
                }
                (RouteSettlement::Glaushouse, RouteSettlement::Stonebend) => {
                    String::from("aura-ridge-north")
                }
                (RouteSettlement::Sandmanor, _) | (_, RouteSettlement::Sandmanor) => {
                    String::from("aura-ridge-east")
                }
                _ => String::from("aura-ridge"),
            },
            _ => self.definition.line.as_str().to_owned(),
        }
    }

    fn line_display_name(self) -> String {
        match self.definition.line {
            RouteLineName::AuraRidge => match (self.from, self.to) {
                (RouteSettlement::Stonebend, RouteSettlement::Glaushouse) => {
                    String::from("Aura Ridge South")
                }
                (RouteSettlement::Glaushouse, RouteSettlement::Stonebend) => {
                    String::from("Aura Ridge North")
                }
                (RouteSettlement::Sandmanor, _) | (_, RouteSettlement::Sandmanor) => {
                    String::from("Aura Ridge East")
                }
                _ => String::from("Aura Ridge"),
            },
            _ => self.definition.line.display_name().to_owned(),
        }
    }
}

impl ResolvedDecisionContext {
    fn packet_label(&self, spec: &DecideActionSpec) -> String {
        format!(
            "site={} domain={} commitment={} authority={} signal={}",
            self.site_name,
            self.domain_name,
            spec.commitment.as_str(),
            spec.authority.as_str(),
            spec.signal.as_str()
        )
    }
}

impl ResolvedSupportContext {
    fn packet_label(&self, spec: &SupportActionSpec) -> String {
        let beneficiary = self
            .beneficiary_name
            .as_deref()
            .map(|name| format!(" beneficiary={name}"))
            .unwrap_or_default();
        format!(
            "site={} asset={} front={} intensity={} duration={}{}",
            self.site_name,
            spec.asset.as_str(),
            spec.front.as_str(),
            spec.intensity.as_str(),
            spec.duration.as_str(),
            beneficiary
        )
    }
}

fn route_line_definition(line: RouteLineName) -> Option<RouteLineDefinition> {
    match line {
        RouteLineName::StairwayToHeaven => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Curved,
            lane: RouteLane::Ring,
            from: RouteSettlement::Stonebend,
            to: RouteSettlement::Flynt,
            surface_custodian: RouteCustodian::Hal,
            inverse_custodian: Some(RouteCustodian::Cleopatra),
        }),
        RouteLineName::BasinMotorspeedway => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::Diamond,
            from: RouteSettlement::Stonebend,
            to: RouteSettlement::Flynt,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::Riptide => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Curved,
            lane: RouteLane::Ring,
            from: RouteSettlement::Flynt,
            to: RouteSettlement::Glaushouse,
            surface_custodian: RouteCustodian::Hal,
            inverse_custodian: Some(RouteCustodian::Cleopatra),
        }),
        RouteLineName::Boardwalk => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::Diamond,
            from: RouteSettlement::Flynt,
            to: RouteSettlement::Glaushouse,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::AuraRidge => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::StonebendGlaushouseLeg,
            from: RouteSettlement::Glaushouse,
            to: RouteSettlement::Stonebend,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::CurrentSea => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Curved,
            lane: RouteLane::Ring,
            from: RouteSettlement::Glaushouse,
            to: RouteSettlement::Sandmanor,
            surface_custodian: RouteCustodian::Hal,
            inverse_custodian: Some(RouteCustodian::Cleopatra),
        }),
        RouteLineName::Glausbahn => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::SandmanorBranch,
            from: RouteSettlement::Glaushouse,
            to: RouteSettlement::Sandmanor,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::MountAura => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Curved,
            lane: RouteLane::Ring,
            from: RouteSettlement::Sandmanor,
            to: RouteSettlement::Stonebend,
            surface_custodian: RouteCustodian::Hal,
            inverse_custodian: Some(RouteCustodian::Cleopatra),
        }),
        RouteLineName::AuraWay => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::SandmanorBranch,
            from: RouteSettlement::Sandmanor,
            to: RouteSettlement::Stonebend,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::QuarryRim => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::QuarryRim,
            from: RouteSettlement::Stonebend,
            to: RouteSettlement::Sandmanor,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::WesternRoad => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::WesternRoad,
            from: RouteSettlement::Flynt,
            to: RouteSettlement::Glaushouse,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::IntakeLine => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::IntakeLine,
            from: RouteSettlement::Sandmanor,
            to: RouteSettlement::Stonebend,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::Shelterline => Some(RouteLineDefinition {
            line,
            shape: RouteShape::Straight,
            lane: RouteLane::ShelterLane,
            from: RouteSettlement::Glaushouse,
            to: RouteSettlement::Flynt,
            surface_custodian: RouteCustodian::Clouseau,
            inverse_custodian: None,
        }),
        RouteLineName::General => None,
    }
}

fn move_forward_bias(spec: &MoveActionSpec, candidate_id: &str) -> i32 {
    let line_bias = match spec.line {
        RouteLineName::AuraWay
        | RouteLineName::BasinMotorspeedway
        | RouteLineName::Boardwalk
        | RouteLineName::Glausbahn => {
            if candidate_id.contains("enhanced-sight") {
                12
            } else if candidate_id.contains("flight") {
                8
            } else {
                0
            }
        }
        RouteLineName::AuraRidge => {
            if candidate_id.contains("enhanced-sight") {
                8
            } else if candidate_id.contains("anchor-armor") {
                6
            } else {
                0
            }
        }
        RouteLineName::StairwayToHeaven
        | RouteLineName::Riptide
        | RouteLineName::CurrentSea
        | RouteLineName::MountAura => {
            if candidate_id.contains("flight") {
                12
            } else if candidate_id.contains("camouflage") {
                6
            } else {
                0
            }
        }
        RouteLineName::QuarryRim | RouteLineName::WesternRoad => {
            if candidate_id.contains("burrowing") {
                12
            } else if candidate_id.contains("anchor-armor") {
                8
            } else {
                0
            }
        }
        RouteLineName::IntakeLine | RouteLineName::Shelterline => {
            if candidate_id.contains("anchor-armor") {
                10
            } else if candidate_id.contains("camouflage") {
                4
            } else {
                0
            }
        }
        RouteLineName::General => 0,
    };
    let method_bias = match spec.method {
        MoveMethod::Scout => {
            if candidate_id.contains("enhanced-sight") {
                14
            } else if candidate_id.contains("echo-hearing") {
                6
            } else {
                0
            }
        }
        MoveMethod::Flank => {
            if candidate_id.contains("flight") {
                10
            } else if candidate_id.contains("camouflage") {
                10
            } else {
                0
            }
        }
        MoveMethod::Tunnel => {
            if candidate_id.contains("burrowing") {
                18
            } else {
                0
            }
        }
        MoveMethod::Carry => {
            if candidate_id.contains("anchor-armor") {
                12
            } else {
                0
            }
        }
        MoveMethod::Traverse => 0,
    };
    let pace_bias = match spec.pace {
        MovePace::Fast if candidate_id.contains("flight") => 8,
        MovePace::Careful if candidate_id.contains("enhanced-sight") => 6,
        MovePace::Careful if candidate_id.contains("camouflage") => 4,
        _ => 0,
    };
    let stance_bias = match spec.stance {
        MoveStance::Quiet if candidate_id.contains("camouflage") => 10,
        MoveStance::Forceful if candidate_id.contains("burrowing") => 10,
        MoveStance::Steady if candidate_id.contains("anchor-armor") => 6,
        _ => 0,
    };
    line_bias + method_bias + pace_bias + stance_bias
}

fn decide_forward_bias(spec: &DecideActionSpec, candidate_id: &str) -> i32 {
    let focus_bias = match spec.focus {
        DecideFocus::Route if candidate_id.contains("enhanced-sight") => 8,
        DecideFocus::Power if candidate_id.contains("burrowing") => 4,
        DecideFocus::Shelter if candidate_id.contains("anchor-armor") => 8,
        DecideFocus::Conflict if candidate_id.contains("camouflage") => 5,
        _ => 0,
    };
    let commitment_bias = match spec.commitment {
        DecideCommitment::Commit if candidate_id.contains("anchor-armor") => 4,
        DecideCommitment::Withdraw if candidate_id.contains("camouflage") => 6,
        _ => 0,
    };
    focus_bias + commitment_bias
}

fn support_forward_bias(spec: &SupportActionSpec, candidate_id: &str) -> i32 {
    let asset_bias = match spec.asset {
        SupportAsset::Route | SupportAsset::Bridge if candidate_id.contains("anchor-armor") => 10,
        SupportAsset::Pump | SupportAsset::Power if candidate_id.contains("burrowing") => 5,
        SupportAsset::Shelter | SupportAsset::Intake if candidate_id.contains("anchor-armor") => 8,
        _ => 0,
    };
    let intensity_bias = match spec.intensity {
        SupportIntensity::Heavy if candidate_id.contains("anchor-armor") => 6,
        SupportIntensity::Light if candidate_id.contains("camouflage") => 4,
        _ => 0,
    };
    asset_bias + intensity_bias
}

fn move_blep_bias(spec: &MoveActionSpec, candidate_id: &str) -> i32 {
    let line_bias = match spec.line {
        RouteLineName::AuraWay
        | RouteLineName::BasinMotorspeedway
        | RouteLineName::Boardwalk
        | RouteLineName::Glausbahn => {
            if candidate_id.contains("survey-route-network") {
                10
            } else if candidate_id.contains("repair-bridge") {
                8
            } else {
                0
            }
        }
        RouteLineName::AuraRidge => {
            if candidate_id.contains("repair-bridge") {
                10
            } else if candidate_id.contains("hold-shelterline") {
                4
            } else {
                0
            }
        }
        RouteLineName::StairwayToHeaven
        | RouteLineName::Riptide
        | RouteLineName::CurrentSea
        | RouteLineName::MountAura => {
            if candidate_id.contains("survey-route-network") {
                12
            } else {
                0
            }
        }
        RouteLineName::QuarryRim | RouteLineName::WesternRoad => {
            if candidate_id.contains("open-tunnel") {
                18
            } else if candidate_id.contains("repair-bridge") {
                12
            } else {
                0
            }
        }
        RouteLineName::IntakeLine => {
            if candidate_id.contains("hold-shelterline") {
                8
            } else if candidate_id.contains("request-alliance") {
                4
            } else {
                0
            }
        }
        RouteLineName::Shelterline => {
            if candidate_id.contains("hold-shelterline") {
                12
            } else {
                0
            }
        }
        RouteLineName::General => 0,
    };
    let method_bias = match spec.method {
        MoveMethod::Scout if candidate_id.contains("survey-route-network") => 12,
        MoveMethod::Tunnel if candidate_id.contains("open-tunnel") => 20,
        MoveMethod::Carry if candidate_id.contains("repair-bridge") => 8,
        MoveMethod::Flank if candidate_id.contains("hide-and-wait") => 6,
        _ => 0,
    };
    let pace_bias = match spec.pace {
        MovePace::Fast if candidate_id.contains("open-tunnel") => 6,
        MovePace::Careful if candidate_id.contains("survey-route-network") => 6,
        _ => 0,
    };
    let stance_bias = match spec.stance {
        MoveStance::Quiet if candidate_id.contains("hide-and-wait") => 8,
        MoveStance::Forceful if candidate_id.contains("raid-convoy") => 6,
        MoveStance::Steady if candidate_id.contains("repair-bridge") => 6,
        _ => 0,
    };
    line_bias + method_bias + pace_bias + stance_bias
}

fn decide_blep_bias(spec: &DecideActionSpec, candidate_id: &str) -> i32 {
    let focus_bias = match spec.focus {
        DecideFocus::Alliance if candidate_id.contains("request-alliance") => 18,
        DecideFocus::Route if candidate_id.contains("survey-route-network") => 12,
        DecideFocus::Power if candidate_id.contains("open-tunnel") => 8,
        DecideFocus::Shelter if candidate_id.contains("hold-shelterline") => 18,
        DecideFocus::Conflict if candidate_id.contains("raid-convoy") => 14,
        DecideFocus::Labor if candidate_id.contains("request-alliance") => 8,
        _ => 0,
    };
    let commitment_bias = match spec.commitment {
        DecideCommitment::Hold if candidate_id.contains("hold-shelterline") => 14,
        DecideCommitment::Shift if candidate_id.contains("survey-route-network") => 10,
        DecideCommitment::Commit if candidate_id.contains("request-alliance") => 10,
        DecideCommitment::Withdraw if candidate_id.contains("hide-and-wait") => 14,
        _ => 0,
    };
    let authority_bias = match spec.authority {
        DecideAuthority::Shared if candidate_id.contains("request-alliance") => 12,
        DecideAuthority::Stonebend if candidate_id.contains("repair-bridge") => 8,
        DecideAuthority::Glaushouse if candidate_id.contains("hold-shelterline") => 8,
        DecideAuthority::Sandmanor if candidate_id.contains("survey-route-network") => 8,
        _ => 0,
    };
    let signal_bias = match spec.signal {
        DecideSignal::Emergency if candidate_id.contains("hold-shelterline") => 8,
        DecideSignal::Emergency if candidate_id.contains("open-tunnel") => 6,
        DecideSignal::Quiet if candidate_id.contains("hide-and-wait") => 8,
        DecideSignal::Public if candidate_id.contains("request-alliance") => 8,
        _ => 0,
    };
    focus_bias + commitment_bias + authority_bias + signal_bias
}

fn support_blep_bias(spec: &SupportActionSpec, candidate_id: &str) -> i32 {
    let asset_bias = match spec.asset {
        SupportAsset::Pump | SupportAsset::Power if candidate_id.contains("request-alliance") => 8,
        SupportAsset::Pump | SupportAsset::Intake if candidate_id.contains("hold-shelterline") => 8,
        SupportAsset::Crane | SupportAsset::Bridge if candidate_id.contains("repair-bridge") => 16,
        SupportAsset::Route if candidate_id.contains("survey-route-network") => 10,
        SupportAsset::Crew if candidate_id.contains("request-alliance") => 12,
        SupportAsset::Shelter if candidate_id.contains("hold-shelterline") => 16,
        _ => 0,
    };
    let front_bias = match spec.front {
        SupportFront::Route if candidate_id.contains("repair-bridge") => 12,
        SupportFront::Route if candidate_id.contains("open-tunnel") => 10,
        SupportFront::Shelter if candidate_id.contains("hold-shelterline") => 14,
        SupportFront::Power if candidate_id.contains("request-alliance") => 8,
        SupportFront::Labor if candidate_id.contains("request-alliance") => 10,
        _ => 0,
    };
    let intensity_bias = match spec.intensity {
        SupportIntensity::Heavy if candidate_id.contains("repair-bridge") => 8,
        SupportIntensity::Heavy if candidate_id.contains("hold-shelterline") => 8,
        SupportIntensity::Light if candidate_id.contains("hide-and-wait") => 4,
        _ => 0,
    };
    let duration_bias = match spec.duration {
        SupportDuration::Hold if candidate_id.contains("hold-shelterline") => 10,
        SupportDuration::Extended if candidate_id.contains("request-alliance") => 8,
        SupportDuration::Burst if candidate_id.contains("repair-bridge") => 6,
        _ => 0,
    };
    asset_bias + front_bias + intensity_bias + duration_bias
}

fn decide_context_bias(context: &ResolvedDecisionContext, candidate_id: &str) -> i32 {
    let route_bias = match context.domain_slug {
        "route-stability" => {
            if candidate_id.contains("enhanced-sight") {
                36
            } else if candidate_id.contains("echo-hearing") {
                20
            } else if candidate_id.contains("flight") {
                10
            } else if candidate_id.contains("burrowing") {
                -12
            } else {
                0
            }
        }
        "power-stability" => {
            if candidate_id.contains("burrowing") {
                10
            } else if candidate_id.contains("anchor-armor") {
                4
            } else {
                0
            }
        }
        "shelter-integrity" => {
            if candidate_id.contains("anchor-armor") {
                12
            } else if candidate_id.contains("camouflage") {
                5
            } else {
                0
            }
        }
        "alliance-channel" => {
            if candidate_id.contains("echo-hearing") {
                14
            } else if candidate_id.contains("camouflage") {
                4
            } else {
                0
            }
        }
        "conflict-risk" => {
            if candidate_id.contains("camouflage") {
                10
            } else if candidate_id.contains("anchor-armor") {
                5
            } else {
                0
            }
        }
        "labor-availability" => {
            if candidate_id.contains("burrowing") {
                8
            } else if candidate_id.contains("anchor-armor") {
                4
            } else {
                0
            }
        }
        _ => 0,
    };
    let site_bias = if context.site_slug.contains("aura-ridge") {
        if candidate_id.contains("enhanced-sight") {
            10
        } else if candidate_id.contains("anchor-armor") {
            4
        } else if candidate_id.contains("burrowing") {
            -8
        } else {
            0
        }
    } else if context.site_slug.contains("stonebend") {
        if candidate_id.contains("anchor-armor") {
            6
        } else {
            0
        }
    } else if context.site_slug.contains("sandmanor") {
        if candidate_id.contains("burrowing") {
            6
        } else {
            0
        }
    } else {
        0
    };
    route_bias + site_bias
}

fn support_context_bias(context: &ResolvedSupportContext, candidate_id: &str) -> i32 {
    let site_bias = if context.site_slug.contains("aura-ridge") {
        if candidate_id.contains("anchor-armor") {
            220
        } else if candidate_id.contains("enhanced-sight") {
            -120
        } else if candidate_id.contains("burrowing") {
            8
        } else if candidate_id.contains("camouflage") {
            4
        } else {
            0
        }
    } else if context.site_slug.contains("shelterline") {
        if candidate_id.contains("anchor-armor") {
            16
        } else if candidate_id.contains("camouflage") {
            4
        } else {
            0
        }
    } else if context.site_slug.contains("power") {
        if candidate_id.contains("burrowing") {
            8
        } else if candidate_id.contains("flight") {
            4
        } else {
            0
        }
    } else {
        0
    };
    let route_bias = if context.route.is_some() {
        if candidate_id.contains("anchor-armor") {
            20
        } else if candidate_id.contains("enhanced-sight") {
            -12
        } else {
            0
        }
    } else {
        0
    };
    site_bias + route_bias
}

fn slugify_identifier(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }
    slug.trim_matches('-').to_owned()
}

fn humanize_identifier(value: &str) -> String {
    let slug = slugify_identifier(value);
    if slug.is_empty() {
        return String::from("General");
    }
    slug.split('-')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn resolve_route_hint(value: &str) -> Option<ResolvedMoveRoute> {
    let line = parse_route_line_name(value)?;
    let definition = route_line_definition(line)?;
    let normalized = value.to_ascii_lowercase().replace(['-', '_'], " ");
    let (from, to) = match line {
        RouteLineName::AuraRidge if normalized.contains("south") => {
            (RouteSettlement::Stonebend, RouteSettlement::Glaushouse)
        }
        RouteLineName::AuraRidge if normalized.contains("north") => {
            (RouteSettlement::Glaushouse, RouteSettlement::Stonebend)
        }
        RouteLineName::AuraRidge if normalized.contains("east") => {
            (RouteSettlement::Sandmanor, RouteSettlement::Stonebend)
        }
        _ => (definition.from, definition.to),
    };
    Some(ResolvedMoveRoute {
        definition,
        from,
        to,
    })
}

fn decide_domain(focus: DecideFocus) -> (&'static str, &'static str) {
    match focus {
        DecideFocus::Route => ("route-stability", "Route Stability"),
        DecideFocus::Power => ("power-stability", "Power Stability"),
        DecideFocus::Shelter => ("shelter-integrity", "Shelter Integrity"),
        DecideFocus::Alliance => ("alliance-channel", "Alliance Channel"),
        DecideFocus::Conflict => ("conflict-risk", "Conflict Risk"),
        DecideFocus::Labor => ("labor-availability", "Labor Availability"),
        DecideFocus::General => ("field-balance", "Field Balance"),
    }
}

fn default_decide_site(spec: &DecideActionSpec) -> (String, String) {
    match spec.authority {
        DecideAuthority::Stonebend => (
            String::from("stonebend-council"),
            String::from("Stonebend Council"),
        ),
        DecideAuthority::Glaushouse => (
            String::from("glaushouse-exchange"),
            String::from("Glaushouse Exchange"),
        ),
        DecideAuthority::Sandmanor => (
            String::from("sandmanor-intake"),
            String::from("Sandmanor Intake"),
        ),
        DecideAuthority::Shared => match spec.focus {
            DecideFocus::Route => (
                String::from("aura-ridge-spine"),
                String::from("Aura Ridge Spine"),
            ),
            DecideFocus::Power => (String::from("basin-grid"), String::from("Basin Grid")),
            DecideFocus::Shelter => (
                String::from("shelterline-ring"),
                String::from("Shelterline Ring"),
            ),
            DecideFocus::Alliance => (String::from("shared-table"), String::from("Shared Table")),
            DecideFocus::Conflict => (
                String::from("current-boundary"),
                String::from("Current Boundary"),
            ),
            DecideFocus::Labor => (String::from("labor-yard"), String::from("Labor Yard")),
            DecideFocus::General => (String::from("shared-field"), String::from("Shared Field")),
        },
        DecideAuthority::Solo => match spec.focus {
            DecideFocus::Route => (String::from("route-call"), String::from("Route Call")),
            DecideFocus::Power => (String::from("power-call"), String::from("Power Call")),
            DecideFocus::Shelter => (String::from("shelter-call"), String::from("Shelter Call")),
            DecideFocus::Alliance => (String::from("alliance-call"), String::from("Alliance Call")),
            DecideFocus::Conflict => (String::from("conflict-edge"), String::from("Conflict Edge")),
            DecideFocus::Labor => (String::from("labor-call"), String::from("Labor Call")),
            DecideFocus::General => (String::from("field-call"), String::from("Field Call")),
        },
    }
}

fn default_support_site(spec: &SupportActionSpec) -> (String, String) {
    match spec.front {
        SupportFront::Route => (String::from("route-works"), String::from("Route Works")),
        SupportFront::Shelter => (
            String::from("shelterline-hold"),
            String::from("Shelterline Hold"),
        ),
        SupportFront::Power => (String::from("power-house"), String::from("Power House")),
        SupportFront::Labor => (String::from("labor-yard"), String::from("Labor Yard")),
        SupportFront::General => match spec.asset {
            SupportAsset::Bridge => (String::from("bridge-works"), String::from("Bridge Works")),
            SupportAsset::Intake => (String::from("intake-house"), String::from("Intake House")),
            SupportAsset::Pump | SupportAsset::Power => {
                (String::from("power-house"), String::from("Power House"))
            }
            SupportAsset::Shelter => (
                String::from("shelterline-hold"),
                String::from("Shelterline Hold"),
            ),
            SupportAsset::Crew => (String::from("labor-yard"), String::from("Labor Yard")),
            SupportAsset::Route | SupportAsset::Crane => {
                (String::from("route-works"), String::from("Route Works"))
            }
            SupportAsset::General => (String::from("field-support"), String::from("Field Support")),
        },
    }
}

fn resolve_move_route(spec: &MoveActionSpec) -> Option<ResolvedMoveRoute> {
    let definition = route_line_definition(spec.line)?;
    let from = spec
        .from
        .as_deref()
        .and_then(RouteSettlement::parse)
        .unwrap_or(definition.from);
    let to = spec
        .to
        .as_deref()
        .and_then(RouteSettlement::parse)
        .unwrap_or(definition.to);
    Some(ResolvedMoveRoute {
        definition,
        from,
        to,
    })
}

fn move_route_definition(profile: &PlayerActionProfile) -> Option<ResolvedMoveRoute> {
    profile.directives.iter().find_map(|directive| {
        if let PlayerActionSchema::Move(spec) = &directive.schema {
            resolve_move_route(spec)
        } else {
            None
        }
    })
}

fn resolve_decide_context(
    directive: &PlayerActionDirective,
    spec: &DecideActionSpec,
) -> ResolvedDecisionContext {
    let explicit_site = trait_value(&directive.traits, &["site", "zone"])
        .or(directive.target.as_deref())
        .filter(|value| !value.eq_ignore_ascii_case(spec.focus.as_str()));
    let route = trait_value(&directive.traits, &["line", "route"])
        .and_then(resolve_route_hint)
        .or_else(|| explicit_site.and_then(resolve_route_hint))
        .or_else(|| resolve_route_hint(&directive.detail));
    let (site_slug, site_name) = if let Some(route) = route {
        (route.line_slug(), route.line_display_name())
    } else if let Some(site) = explicit_site {
        (slugify_identifier(site), humanize_identifier(site))
    } else {
        default_decide_site(spec)
    };
    let (domain_slug, domain_name) = decide_domain(spec.focus);
    ResolvedDecisionContext {
        site_slug,
        site_name,
        domain_slug,
        domain_name,
        route,
    }
}

fn decide_context(
    profile: &PlayerActionProfile,
) -> Option<(DecideActionSpec, ResolvedDecisionContext)> {
    profile.directives.iter().find_map(|directive| {
        if let PlayerActionSchema::Decide(spec) = &directive.schema {
            Some((spec.clone(), resolve_decide_context(directive, spec)))
        } else {
            None
        }
    })
}

fn resolve_support_context(
    directive: &PlayerActionDirective,
    spec: &SupportActionSpec,
) -> ResolvedSupportContext {
    let explicit_site = trait_value(&directive.traits, &["site", "zone", "line", "route"])
        .or(spec.beneficiary.as_deref())
        .or(directive.target.as_deref());
    let route = trait_value(&directive.traits, &["line", "route"])
        .and_then(resolve_route_hint)
        .or_else(|| explicit_site.and_then(resolve_route_hint))
        .or_else(|| resolve_route_hint(&directive.detail));
    let (site_slug, site_name) = if let Some(route) = route {
        (route.line_slug(), route.line_display_name())
    } else if let Some(site) = explicit_site {
        (slugify_identifier(site), humanize_identifier(site))
    } else {
        default_support_site(spec)
    };
    let beneficiary_name = spec
        .beneficiary
        .as_deref()
        .map(humanize_identifier)
        .filter(|name| name != &site_name);
    ResolvedSupportContext {
        site_slug,
        site_name,
        beneficiary_name,
        route,
    }
}

fn support_context(
    profile: &PlayerActionProfile,
) -> Option<(SupportActionSpec, ResolvedSupportContext)> {
    profile.directives.iter().find_map(|directive| {
        if let PlayerActionSchema::Support(spec) = &directive.schema {
            Some((spec.clone(), resolve_support_context(directive, spec)))
        } else {
            None
        }
    })
}

impl PlayerActionProfile {
    fn from_actions(actions: &[String]) -> Self {
        let directives = actions
            .iter()
            .map(|action| decode_player_action(action))
            .collect::<Vec<_>>();
        let mut profile = Self {
            directives,
            move_count: 0,
            decision_count: 0,
            support_count: 0,
            general_count: 0,
        };
        for directive in &profile.directives {
            match directive.kind {
                PlayerActionKind::Move => profile.move_count += 1,
                PlayerActionKind::Decide => profile.decision_count += 1,
                PlayerActionKind::Support => profile.support_count += 1,
                PlayerActionKind::General => profile.general_count += 1,
            }
        }
        profile
    }

    fn is_empty(&self) -> bool {
        self.directives.is_empty()
    }

    fn dominant_kind(&self) -> PlayerActionKind {
        let ranked = [
            (self.move_count, PlayerActionKind::Move),
            (self.decision_count, PlayerActionKind::Decide),
            (self.support_count, PlayerActionKind::Support),
            (self.general_count, PlayerActionKind::General),
        ];
        ranked
            .into_iter()
            .max_by_key(|(count, kind)| {
                let tie_break = match kind {
                    PlayerActionKind::Move => 4usize,
                    PlayerActionKind::Decide => 3,
                    PlayerActionKind::Support => 2,
                    PlayerActionKind::General => 1,
                };
                (*count, tie_break)
            })
            .map(|(_, kind)| kind)
            .unwrap_or(PlayerActionKind::General)
    }

    fn movement_pressure(&self) -> i32 {
        self.move_count as i32 * 18
            + if self.dominant_kind() == PlayerActionKind::Move {
                10
            } else {
                0
            }
    }

    fn decision_pressure(&self) -> i32 {
        self.decision_count as i32 * 18
            + if self.dominant_kind() == PlayerActionKind::Decide {
                10
            } else {
                0
            }
    }

    fn support_pressure(&self) -> i32 {
        self.support_count as i32 * 18
            + if self.dominant_kind() == PlayerActionKind::Support {
                10
            } else {
                0
            }
    }

    fn general_pressure(&self) -> i32 {
        self.general_count as i32 * 10
    }

    fn display_label(&self) -> String {
        if self.is_empty() {
            String::from("none")
        } else {
            self.directives
                .iter()
                .map(render_player_action_directive)
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    fn render_pending_actions(&self) -> String {
        if self.is_empty() {
            return String::from("- none\n");
        }
        self.directives
            .iter()
            .map(|directive| format!("- {}\n", render_player_action_directive(directive)))
            .collect()
    }

    fn has_context_keyword(&self, keywords: &[&str]) -> bool {
        self.directives.iter().any(|directive| {
            let mut haystacks = vec![directive.detail.as_str()];
            if let Some(target) = directive.target.as_deref() {
                haystacks.push(target);
            }
            haystacks.extend(directive.traits.values().map(String::as_str));
            haystacks.iter().any(|value| {
                let lower = value.to_ascii_lowercase();
                keywords.iter().any(|needle| lower.contains(needle))
            })
        })
    }

    fn hal_machine_complement(&self) -> String {
        match self.dominant_kind() {
            PlayerActionKind::Move => {
                if let Some(definition) = move_route_definition(self) {
                    match definition.definition.surface_custodian {
                        RouteCustodian::Hal => format!(
                            "HAL holds the public curved route {} while keeping line-of-sight, lift, and pressure continuity synchronized with the player's movement. Cleopatra manages the inverse half-circle beneath it for downstream NPC continuity.",
                            definition.line_display_name()
                        ),
                        RouteCustodian::Clouseau => format!(
                            "HAL complements Clouseau's straight route {} by holding the surrounding curve pressure, distance, and lift map without taking over the player's line choice.",
                            definition.line_display_name()
                        ),
                        RouteCustodian::Cleopatra => String::from(
                            "HAL aligns a complementary counter-position around movement continuity.",
                        ),
                    }
                } else {
                    String::from(
                        "HAL aligns a complementary counter-position around movement continuity: line-of-sight, lift, and pressure mapping stay synchronized with the player's route changes without replacing the choice.",
                    )
                }
            }
            PlayerActionKind::Decide => String::from(
                "HAL aligns a complementary counter-position around decision continuity: line-of-sight, lift, and pressure mapping stay synchronized with the player's choice pressure without deciding in the player's place.",
            ),
            PlayerActionKind::Support => String::from(
                "HAL aligns a complementary counter-position around support continuity: line-of-sight, lift, and pressure mapping stay synchronized with the player's brace-and-stabilize posture without automating the route.",
            ),
            PlayerActionKind::General => String::from(
                "HAL aligns a complementary counter-position: line-of-sight, lift, and pressure mapping stay available but do not determine the outcome alone.",
            ),
        }
    }

    fn schema_focus_label(&self) -> String {
        for directive in &self.directives {
            match &directive.schema {
                PlayerActionSchema::Move(spec) => {
                    let line_label = resolve_move_route(spec)
                        .map(|route| route.line_display_name())
                        .unwrap_or_else(|| spec.line.display_name().to_owned());
                    return format!(
                        "line={} pace={} method={} stance={}",
                        line_label,
                        spec.pace.as_str(),
                        spec.method.as_str(),
                        spec.stance.as_str()
                    );
                }
                PlayerActionSchema::Decide(spec) => {
                    return resolve_decide_context(directive, spec).packet_label(spec);
                }
                PlayerActionSchema::Support(spec) => {
                    return resolve_support_context(directive, spec).packet_label(spec);
                }
                PlayerActionSchema::General => {}
            }
        }
        String::from("general")
    }

    fn clouseau_relay(&self, selected_bond: &BondCandidate) -> String {
        match self.dominant_kind() {
            PlayerActionKind::Move => {
                if let Some(definition) = move_route_definition(self) {
                    match definition.definition.surface_custodian {
                        RouteCustodian::Clouseau => format!(
                            "Clouseau handles the player's straight route {} on {} and keeps `{}` aligned with the usable chord through the map.",
                            definition.line_display_name(),
                            self.schema_focus_label(),
                            selected_bond.id,
                        ),
                        RouteCustodian::Hal => format!(
                            "Clouseau reads HAL's curved route {} as a live clue boundary on {} and keeps `{}` oriented toward the crossing seam the player can still exploit.",
                            definition.line_display_name(),
                            self.schema_focus_label(),
                            selected_bond.id,
                        ),
                        RouteCustodian::Cleopatra => format!(
                            "Clouseau tracks {} and keeps `{}` aligned with the player's usable seam.",
                            self.schema_focus_label(),
                            selected_bond.id,
                        ),
                    }
                } else {
                    format!(
                        "Clouseau reads the player's motion as live route pressure on {} and keeps `{}` oriented toward seams that can still be crossed.",
                        self.schema_focus_label(),
                        selected_bond.id,
                    )
                }
            }
            PlayerActionKind::Decide => format!(
                "Clouseau reads the player's decision pressure as a clue-selection problem around {} and keeps `{}` aligned with the most legible commitment.",
                decide_context(self)
                    .map(|(_, context)| format!(
                        "{} across {}",
                        context.site_name, context.domain_name
                    ))
                    .unwrap_or_else(|| self.schema_focus_label()),
                selected_bond.id,
            ),
            PlayerActionKind::Support => format!(
                "Clouseau reads the player's support posture as a hold-and-stabilize clue around {} and keeps `{}` aligned with what will keep the field from breaking.",
                support_context(self)
                    .map(|(_, context)| context.site_name)
                    .unwrap_or_else(|| self.schema_focus_label()),
                selected_bond.id,
            ),
            PlayerActionKind::General => format!(
                "Clouseau keeps `{}` tied to the player's unresolved route pressure without freezing the branch into a script.",
                selected_bond.id
            ),
        }
    }

    fn hal_relay(&self, selected_bond: &BondCandidate) -> String {
        match self.dominant_kind() {
            PlayerActionKind::Move => {
                if let Some(definition) = move_route_definition(self) {
                    match definition.definition.surface_custodian {
                        RouteCustodian::Hal => format!(
                            "HAL runs the curved route {} and counter-positions `{}` with ring pressure, distance, and lift continuity under {}.",
                            definition.line_display_name(),
                            selected_bond.id,
                            self.schema_focus_label(),
                        ),
                        RouteCustodian::Clouseau => format!(
                            "HAL shadows Clouseau's straight route {} and counter-positions `{}` with the surrounding curve pressure under {}.",
                            definition.line_display_name(),
                            selected_bond.id,
                            self.schema_focus_label(),
                        ),
                        RouteCustodian::Cleopatra => format!(
                            "HAL counter-positions `{}` with complementary mapping under {}.",
                            selected_bond.id,
                            self.schema_focus_label(),
                        ),
                    }
                } else {
                    format!(
                        "HAL counter-positions `{}` with distance, lift, and pressure continuity so motion can stay fluid under {}.",
                        selected_bond.id,
                        self.schema_focus_label(),
                    )
                }
            }
            PlayerActionKind::Decide => format!(
                "HAL counter-positions `{}` with clarity and reflection so the player's choice can stay coherent under {}.",
                selected_bond.id,
                decide_context(self)
                    .map(|(_, context)| format!(
                        "{} across {}",
                        context.site_name, context.domain_name
                    ))
                    .unwrap_or_else(|| self.schema_focus_label()),
            ),
            PlayerActionKind::Support => format!(
                "HAL counter-positions `{}` with structure and load mapping so the player's support action lands where {} is truly bearing.",
                selected_bond.id,
                support_context(self)
                    .map(|(_, context)| context.site_name)
                    .unwrap_or_else(|| self.schema_focus_label()),
            ),
            PlayerActionKind::General => format!(
                "HAL counter-positions `{}` with complementary mapping instead of direct control.",
                selected_bond.id
            ),
        }
    }

    fn joint_relay(&self, selected_bond: &BondCandidate, player_moment: &str) -> String {
        if self.dominant_kind() == PlayerActionKind::Move {
            if let Some(definition) = move_route_definition(self) {
                let inverse = definition
                    .definition
                    .inverse_custodian
                    .map(|custodian| custodian.display_name())
                    .unwrap_or("none");
                return format!(
                    "{} packet: surface route {} belongs to {}, inverse route belongs to {}, Clouseau carries the playable chord, HAL carries the pressure field, and Cleopatra receives `{}` for NPC coordination. Moment focus: {}",
                    self.dominant_kind().posture_label(),
                    definition.line_display_name(),
                    definition.definition.surface_custodian.display_name(),
                    inverse,
                    selected_bond.id,
                    player_moment
                );
            }
        }
        format!(
            "{} packet: Clouseau carries the player's {} posture through {}, HAL mirrors it with complementary pressure mapping, and Cleopatra receives `{}` as the live handoff before NPC selection. Moment focus: {}",
            self.dominant_kind().posture_label(),
            self.dominant_kind().as_str(),
            self.schema_focus_label(),
            selected_bond.id,
            player_moment
        )
    }

    fn world_inputs(&self) -> Vec<String> {
        let mut inputs = vec![format!(
            "player action mode {}",
            self.dominant_kind().posture_label()
        )];
        inputs.push(String::from(
            "map topology: upper-left Stairway to Heaven curve and Basin Motorspeedway straight enter Flynt; Flynt connects to Glaushouse through the Riptide curve and the Boardwalk straight; Aura Ridge runs as a north-south spine between Glaushouse and Stonebend with an Aura Ridge East cut toward Sandmanor; Glaushouse also branches to Sandmanor through the Current Seanad curve and the Glausbahn straight; Sandmanor closes the loop to Stonebend through Mnt. Aura curve and the Aura Way straight",
        ));
        inputs.push(String::from(
            "route custody: HAL holds Stairway to Heaven, the Riptide, the Current Seanad, and Mnt. Aura; Clouseau handles Basin Motorspeedway, the Boardwalk, the Glausbahn, the Aura Way, and Aura Ridge North, South, and East; Cleopatra manages HAL's inverse half-circle for NPC continuity and route understructure",
        ));
        for directive in &self.directives {
            inputs.push(format!(
                "player {} {}",
                directive.kind.as_str(),
                directive.detail
            ));
            match &directive.schema {
                PlayerActionSchema::Move(spec) => {
                    if let Some(from) = spec.from.as_deref() {
                        inputs.push(format!("player move from {from}"));
                    }
                    if let Some(to) = spec.to.as_deref() {
                        inputs.push(format!("player move to {to}"));
                    }
                    let line_context = resolve_move_route(spec);
                    let line_slug = line_context
                        .map(|route| route.line_slug())
                        .unwrap_or_else(|| spec.line.as_str().to_owned());
                    let line_name = line_context
                        .map(|route| route.line_display_name())
                        .unwrap_or_else(|| spec.line.display_name().to_owned());
                    inputs.push(format!("player move lane {}", spec.lane.as_str()));
                    inputs.push(format!("player move line {}", spec.line.as_str()));
                    inputs.push(format!("player move line-segment {line_slug}"));
                    inputs.push(format!("player move line-name {line_name}"));
                    if let Some(definition) = line_context {
                        inputs.push(format!(
                            "player move topology {} {} {} -> {}",
                            definition.line_display_name(),
                            definition.definition.shape.as_str(),
                            definition.from.as_str(),
                            definition.to.as_str()
                        ));
                        inputs.push(format!(
                            "player move line-family {}",
                            definition.definition.line.display_name()
                        ));
                        inputs.push(format!(
                            "player move surface-custodian {}",
                            definition.definition.surface_custodian.as_str()
                        ));
                        if let Some(inverse) = definition.definition.inverse_custodian {
                            inputs.push(format!(
                                "player move inverse-custodian {}",
                                inverse.as_str()
                            ));
                        }
                    } else if let Some(definition) = route_line_definition(spec.line) {
                        inputs.push(format!(
                            "player move topology {} {} {} -> {}",
                            definition.line.display_name(),
                            definition.shape.as_str(),
                            definition.from.as_str(),
                            definition.to.as_str()
                        ));
                        inputs.push(format!(
                            "player move surface-custodian {}",
                            definition.surface_custodian.as_str()
                        ));
                        if let Some(inverse) = definition.inverse_custodian {
                            inputs.push(format!(
                                "player move inverse-custodian {}",
                                inverse.as_str()
                            ));
                        }
                    }
                    inputs.push(format!("player move pace={}", spec.pace.as_str()));
                    inputs.push(format!("player move method={}", spec.method.as_str()));
                    inputs.push(format!("player move stance={}", spec.stance.as_str()));
                }
                PlayerActionSchema::Decide(spec) => {
                    let context = resolve_decide_context(directive, spec);
                    inputs.push(format!("player decide focus={}", spec.focus.as_str()));
                    inputs.push(format!(
                        "player decide commitment={}",
                        spec.commitment.as_str()
                    ));
                    inputs.push(format!(
                        "player decide authority={}",
                        spec.authority.as_str()
                    ));
                    inputs.push(format!("player decide signal={}", spec.signal.as_str()));
                    inputs.push(format!("player decide site {}", context.site_slug));
                    inputs.push(format!("player decide site-name {}", context.site_name));
                    inputs.push(format!("player decide domain {}", context.domain_slug));
                    inputs.push(format!("player decide domain-name {}", context.domain_name));
                    if let Some(route) = context.route {
                        inputs.push(format!("player decide line-segment {}", route.line_slug()));
                        inputs.push(format!(
                            "player decide line-family {}",
                            route.definition.line.display_name()
                        ));
                        inputs.push(format!(
                            "player decide surface-custodian {}",
                            route.definition.surface_custodian.as_str()
                        ));
                        if let Some(inverse) = route.definition.inverse_custodian {
                            inputs.push(format!(
                                "player decide inverse-custodian {}",
                                inverse.as_str()
                            ));
                        }
                    }
                }
                PlayerActionSchema::Support(spec) => {
                    let context = resolve_support_context(directive, spec);
                    inputs.push(format!("player support asset={}", spec.asset.as_str()));
                    if let Some(beneficiary) = spec.beneficiary.as_deref() {
                        inputs.push(format!("player support beneficiary {beneficiary}"));
                    }
                    if let Some(beneficiary_name) = context.beneficiary_name.as_deref() {
                        inputs.push(format!(
                            "player support beneficiary-name {beneficiary_name}"
                        ));
                    }
                    inputs.push(format!("player support front={}", spec.front.as_str()));
                    inputs.push(format!(
                        "player support intensity={}",
                        spec.intensity.as_str()
                    ));
                    inputs.push(format!(
                        "player support duration={}",
                        spec.duration.as_str()
                    ));
                    inputs.push(format!("player support site {}", context.site_slug));
                    inputs.push(format!("player support site-name {}", context.site_name));
                    if let Some(route) = context.route {
                        inputs.push(format!("player support line-segment {}", route.line_slug()));
                        inputs.push(format!(
                            "player support line-family {}",
                            route.definition.line.display_name()
                        ));
                        inputs.push(format!(
                            "player support surface-custodian {}",
                            route.definition.surface_custodian.as_str()
                        ));
                        if let Some(inverse) = route.definition.inverse_custodian {
                            inputs.push(format!(
                                "player support inverse-custodian {}",
                                inverse.as_str()
                            ));
                        }
                    }
                }
                PlayerActionSchema::General => {
                    if let Some(target) = directive.target.as_deref() {
                        inputs.push(format!(
                            "player {} target {}",
                            directive.kind.as_str(),
                            target
                        ));
                    }
                    for (key, value) in &directive.traits {
                        inputs.push(format!(
                            "player {} {}={}",
                            directive.kind.as_str(),
                            key,
                            value
                        ));
                    }
                }
            }
        }
        inputs
    }

    fn forward_bias(&self, candidate_id: &str) -> i32 {
        let decide_context = decide_context(self);
        let support_context = support_context(self);
        let movement = if candidate_id.contains("flight") {
            self.movement_pressure() * 2 + 8
        } else if candidate_id.contains("enhanced-sight") {
            self.movement_pressure() + self.decision_pressure() * 2 + 10
        } else if candidate_id.contains("burrowing") {
            self.movement_pressure() + 14
        } else if candidate_id.contains("camouflage") {
            self.support_pressure() + self.movement_pressure() / 2
        } else if candidate_id.contains("anchor-armor") {
            self.support_pressure() * 2 + 8
        } else if candidate_id.contains("echo-hearing") {
            self.decision_pressure() + self.general_pressure()
        } else {
            0
        };
        let schema_bias = self
            .directives
            .iter()
            .map(|directive| match &directive.schema {
                PlayerActionSchema::Move(spec) => move_forward_bias(spec, candidate_id),
                PlayerActionSchema::Decide(spec) => {
                    decide_forward_bias(spec, candidate_id)
                        + decide_context
                            .as_ref()
                            .map(|(_, context)| decide_context_bias(context, candidate_id))
                            .unwrap_or(0)
                }
                PlayerActionSchema::Support(spec) => {
                    support_forward_bias(spec, candidate_id)
                        + support_context
                            .as_ref()
                            .map(|(_, context)| support_context_bias(context, candidate_id))
                            .unwrap_or(0)
                }
                PlayerActionSchema::General => 0,
            })
            .sum::<i32>();
        movement + self.general_pressure() + schema_bias
    }

    fn blep_bias(&self, candidate_id: &str) -> i32 {
        let decide_context = decide_context(self);
        let support_context = support_context(self);
        let movement = if candidate_id.contains("open-tunnel") {
            self.movement_pressure() * 2 + 8
        } else if candidate_id.contains("survey-route-network") {
            self.movement_pressure() + self.decision_pressure() + 12
        } else if candidate_id.contains("repair-bridge") {
            self.movement_pressure() + self.support_pressure()
        } else {
            0
        };
        let decision = if candidate_id.contains("request-alliance") {
            self.decision_pressure() * 2 + 8
        } else if candidate_id.contains("survey-route-network") {
            self.decision_pressure() * 2 + 12
        } else {
            0
        };
        let support = if candidate_id.contains("hold-shelterline") {
            self.support_pressure() * 2 + 10
        } else if candidate_id.contains("repair-bridge") {
            self.support_pressure() * 2 + 8
        } else {
            0
        };
        let schema_bias = self
            .directives
            .iter()
            .map(|directive| match &directive.schema {
                PlayerActionSchema::Move(spec) => move_blep_bias(spec, candidate_id),
                PlayerActionSchema::Decide(spec) => {
                    decide_blep_bias(spec, candidate_id)
                        + decide_context
                            .as_ref()
                            .map(|(_, context)| decide_context_bias(context, candidate_id))
                            .unwrap_or(0)
                }
                PlayerActionSchema::Support(spec) => {
                    support_blep_bias(spec, candidate_id)
                        + support_context
                            .as_ref()
                            .map(|(_, context)| support_context_bias(context, candidate_id))
                            .unwrap_or(0)
                }
                PlayerActionSchema::General => 0,
            })
            .sum::<i32>();
        movement + decision + support + self.general_pressure() + schema_bias
    }
}

fn effective_hueman_feedback(hueman_feedback: Option<HuemanFeedback>) -> HuemanFeedback {
    hueman_feedback.unwrap_or(HuemanFeedback {
        unlocked: false,
        recognized_route_branch: false,
        recognized_defense_branch: false,
    })
}

fn persisted_from_state(state: &CurrentSynthesisState) -> PersistedCurrentSynthesisState {
    PersistedCurrentSynthesisState {
        scenario_id: state.scenario_id.clone(),
        seed: state.seed,
        completed_ticks: state.completed_ticks,
        focused_npc_id: state.focused_npc_id.clone(),
    }
}

impl PersistedCurrentSynthesisState {
    pub fn primary() -> Self {
        let scenario = load_scenario(DEFAULT_SCENARIO_ID)
            .expect("default current synthesis scenario must load");
        Self {
            scenario_id: scenario.id,
            seed: DEFAULT_SEED,
            completed_ticks: 1,
            focused_npc_id: scenario.default_focused_npc_id,
        }
    }
}

impl CurrentSynthesisState {
    pub fn replay(persisted: &PersistedCurrentSynthesisState) -> Self {
        Self::replay_with_hueman(persisted, None)
    }

    pub fn replay_with_hueman(
        persisted: &PersistedCurrentSynthesisState,
        hueman_feedback: Option<HuemanFeedback>,
    ) -> Self {
        let scenario = load_scenario(&persisted.scenario_id)
            .expect("persisted current synthesis scenario must load");
        let mut state = Self::initial(persisted, &scenario);
        for _ in 0..persisted.completed_ticks {
            state.tick(hueman_feedback);
        }
        state
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn focused_npc_id(&self) -> &str {
        &self.focused_npc_id
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn completed_ticks(&self) -> u32 {
        self.completed_ticks
    }

    pub fn player_need(&self) -> &str {
        &self.player_need
    }

    pub fn planned_player_actions(&self) -> &[String] {
        &self.planned_player_actions
    }

    pub fn resources(&self) -> &ResourceComposition {
        &self.resources
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn npc(&self) -> &NpcState {
        self.find_npc(&self.focused_npc_id)
            .expect("focused npc id must point to a known npc")
    }

    pub fn npcs(&self) -> &[NpcState] {
        &self.npcs
    }

    pub fn cleopatra(&self) -> &CleopatraState {
        &self.cleopatra
    }

    pub fn history(&self) -> &[TickRecord] {
        &self.history
    }

    pub fn last_tick(&self) -> Option<&TickRecord> {
        self.history.last()
    }

    pub fn find_npc(&self, npc_id: &str) -> Option<&NpcState> {
        self.npcs.iter().find(|npc| npc.id == npc_id)
    }

    fn initial(persisted: &PersistedCurrentSynthesisState, scenario: &ScenarioDefinition) -> Self {
        let npcs = scenario
            .npcs
            .iter()
            .map(|npc| NpcState {
                id: npc.id.clone(),
                name: npc.name.clone(),
                role: npc.role.clone(),
                faction: npc.faction.clone(),
                location: npc.location.clone(),
                needs: npc.needs.clone(),
                memories: npc.memories.clone(),
                relationships: npc.relationships.clone(),
                condition: npc.condition.clone(),
                perceived_world: npc.perceived_world.clone(),
            })
            .collect::<Vec<_>>();
        Self {
            scenario_id: scenario.id.clone(),
            focused_npc_id: persisted.focused_npc_id.clone(),
            seed: persisted.seed,
            completed_ticks: 0,
            player_need: scenario.player_need.clone(),
            planned_player_actions: Vec::new(),
            resources: ResourceComposition::empty(),
            world: WorldState::for_scenario(&scenario.id),
            npcs,
            cleopatra: CleopatraState {
                active_npcs: scenario.npcs.iter().map(|npc| npc.id.clone()).collect(),
                queued_blep_passes: vec![persisted.focused_npc_id.clone()],
                faction_conditions: scenario.faction_conditions.clone(),
                settlement_conditions: scenario.settlement_conditions.clone(),
                war_conditions: scenario.war_conditions.clone(),
                global_resource_composition: ResourceComposition::empty(),
                recent_blep_passes: Vec::new(),
            },
            history: Vec::new(),
        }
    }

    fn tick(&mut self, hueman_feedback: Option<HuemanFeedback>) {
        let tick_number = self.completed_ticks + 1;
        let action_profile = PlayerActionProfile::from_actions(&self.planned_player_actions);
        let forward_pass = build_forward_pass(
            self.seed,
            tick_number,
            &self.player_need,
            &self.resources,
            &action_profile,
            hueman_feedback,
        );
        let forward_residues = residues_from_unused_bonds(&forward_pass.bond_result.unused_bonds);
        for residue in &forward_residues {
            self.resources.apply_residue(residue);
        }

        let player_moment_id = format!("moment/player/{tick_number}");
        let relay_npcs = self
            .cleopatra
            .active_npcs
            .iter()
            .filter_map(|npc_id| self.find_npc(npc_id).cloned())
            .collect::<Vec<_>>();
        let blep_resources = self.resources.clone();
        let coordinated_blep_relays = relay_npcs
            .iter()
            .map(|npc| {
                let world_inputs = build_world_inputs(
                    self,
                    &forward_pass,
                    npc,
                    tick_number,
                    &action_profile,
                    hueman_feedback,
                );
                let blep_decision = build_blep_decision(
                    self.seed,
                    tick_number,
                    npc,
                    &blep_resources,
                    &action_profile,
                    &world_inputs,
                    &forward_pass.bond_result.resulting_moment,
                    hueman_feedback,
                );
                let blep_residues = residues_from_unused_bonds(&blep_decision.unused_bonds);
                CoordinatedBlepRelay {
                    npc_id: npc.id.clone(),
                    npc_moment_id: format!("moment/npc/{tick_number}/{}", npc.id),
                    committed: npc.id == self.focused_npc_id,
                    blep_decision,
                    blep_residues,
                }
            })
            .collect::<Vec<_>>();
        let committed_relay = coordinated_blep_relays
            .iter()
            .find(|relay| relay.committed)
            .cloned()
            .expect("focused npc must produce one committed relay");
        for residue in coordinated_blep_relays
            .iter()
            .flat_map(|relay| relay.blep_residues.iter())
        {
            self.resources.apply_residue(residue);
        }

        let npc_moment_id = committed_relay.npc_moment_id.clone();
        for relay in &coordinated_blep_relays {
            if let Some(npc) = self.npcs.iter_mut().find(|npc| npc.id == relay.npc_id) {
                npc.needs = vec![relay.blep_decision.inferred_need.clone()];
                npc.memories.push(format!(
                    "tick {tick_number}: {}",
                    relay.blep_decision.resulting_action
                ));
                npc.perceived_world = relay.blep_decision.world_inputs.clone();
            }
        }

        self.cleopatra.queued_blep_passes = self.cleopatra.active_npcs.clone();
        self.cleopatra.global_resource_composition = self.resources.clone();
        self.cleopatra.recent_blep_passes = coordinated_blep_relays
            .iter()
            .map(|relay| {
                let npc = relay_npcs
                    .iter()
                    .find(|npc| npc.id == relay.npc_id)
                    .expect("relay npc must exist");
                format!(
                    "tick {tick_number}: {} -> {}{}",
                    npc.name,
                    relay.blep_decision.selected_bond.id,
                    if relay.committed {
                        " [committed]"
                    } else {
                        " [coordinated]"
                    }
                )
            })
            .collect();
        apply_world_consequences(
            &mut self.world,
            &action_profile,
            &forward_pass,
            &coordinated_blep_relays,
            &self.resources,
        );
        self.planned_player_actions.clear();

        self.history.push(TickRecord {
            tick_number,
            player_moment_id,
            npc_moment_id,
            forward_pass,
            forward_residues,
            blep_decision: committed_relay.blep_decision.clone(),
            blep_residues: committed_relay.blep_residues.clone(),
            coordinated_blep_relays,
            resources_after_tick: self.resources.clone(),
        });
        self.completed_ticks = tick_number;
    }
}

pub fn build_persisted_state_output(state: &PersistedCurrentSynthesisState) -> String {
    format!(
        "# Current Synthesis TUI State\n\
         scenario_id: {}\n\
         seed: {}\n\
         completed_ticks: {}\n\
         focused_npc_id: {}\n",
        state.scenario_id, state.seed, state.completed_ticks, state.focused_npc_id
    )
}

pub fn parse_persisted_state(contents: &str) -> io::Result<PersistedCurrentSynthesisState> {
    let mut scenario_id = None;
    let mut seed = None;
    let mut completed_ticks = None;
    let mut focused_npc_id = None;
    let mut seen_unknown = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("current synthesis state line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "scenario_id" => scenario_id = Some(value.to_owned()),
            "seed" => {
                seed = Some(value.parse::<u64>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid u64 for seed: {value}"),
                    )
                })?)
            }
            "completed_ticks" => {
                completed_ticks = Some(value.parse::<u32>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid u32 for completed_ticks: {value}"),
                    )
                })?)
            }
            "focused_npc_id" => focused_npc_id = Some(value.to_owned()),
            other => seen_unknown.push(other.to_owned()),
        }
    }

    if let Some(unknown) = seen_unknown.first() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("current synthesis state contains unknown key: {unknown}"),
        ));
    }

    let scenario_id = scenario_id.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "current synthesis state missing scenario_id",
        )
    })?;
    let scenario = load_scenario(&scenario_id)?;

    let focused_npc_id = focused_npc_id.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "current synthesis state missing focused_npc_id",
        )
    })?;
    if !scenario.npcs.iter().any(|npc| npc.id == focused_npc_id) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsupported focused_npc_id for scenario {scenario_id}: {focused_npc_id}"),
        ));
    }

    Ok(PersistedCurrentSynthesisState {
        scenario_id,
        seed: seed.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "current synthesis state missing seed",
            )
        })?,
        completed_ticks: completed_ticks.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "current synthesis state missing completed_ticks",
            )
        })?,
        focused_npc_id,
    })
}

pub fn encode_current_synthesis_player_action(
    action_kind: &str,
    action_label: &str,
) -> io::Result<String> {
    let trimmed = action_label.trim();
    if trimmed.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "player action requires a non-empty action label",
        ));
    }
    let prefix = match action_kind {
        "move" => PLAYER_ACTION_MOVE_PREFIX,
        "decide" => PLAYER_ACTION_DECIDE_PREFIX,
        "support" => PLAYER_ACTION_SUPPORT_PREFIX,
        "plan" => "",
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported player action kind: {other}"),
            ));
        }
    };
    Ok(format!("{prefix}{trimmed}"))
}

pub fn read_or_create_persisted_state_at(
    root: &Path,
) -> io::Result<PersistedCurrentSynthesisState> {
    Ok(load_current_synthesis_at(root, None)?.0)
}

pub fn write_persisted_state_at(
    root: &Path,
    persisted: &PersistedCurrentSynthesisState,
) -> io::Result<PathBuf> {
    let path = root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH);
    write_text_artifact(&path, &build_persisted_state_output(persisted))?;
    Ok(path)
}

pub fn read_hueman_feedback_at(root: &Path) -> io::Result<Option<HuemanFeedback>> {
    match crate::read_text_artifact(&root.join(HUEMAN_SLICE_STATE_ARTIFACT_PATH)) {
        Ok(contents) => {
            let state = parse_vertical_slice_state(&contents)?;
            let unlocked = matches!(
                state.phase(),
                SlicePhase::RecognitionEarned | SlicePhase::CurrentFormUnlocked
            );
            let resolution_path = state.resolution_path();
            Ok(Some(HuemanFeedback {
                unlocked,
                recognized_route_branch: unlocked
                    && resolution_path
                        == Some(crate::hueman_slice::SliceResolutionPath::RouteStabilization),
                recognized_defense_branch: unlocked
                    && resolution_path
                        == Some(crate::hueman_slice::SliceResolutionPath::FlockDefense),
            }))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

pub fn load_current_synthesis_at(
    root: &Path,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let effective_feedback = effective_hueman_feedback(current_hueman_feedback);
    let mut events = read_or_create_events_at(root, effective_feedback)?;
    let checkpoint = read_state_snapshot_at(root).ok();
    let (mut state, mut last_feedback) = reconstruct_state_from_storage(&events, checkpoint)?;
    if last_feedback != effective_feedback {
        events.push(CurrentSynthesisEvent::HuemanFeedbackChanged {
            hueman_feedback: effective_feedback,
        });
        write_event_log_at(root, &events)?;
        let (next_state, next_feedback) = apply_current_synthesis_event(
            Some(state),
            last_feedback,
            &CurrentSynthesisEvent::HuemanFeedbackChanged {
                hueman_feedback: effective_feedback,
            },
        )?;
        state = next_state;
        last_feedback = next_feedback;
    }
    let persisted = persisted_from_state(&state);
    write_persisted_state_at(root, &persisted)?;

    let checkpoint_event_count = read_state_snapshot_at(root)
        .ok()
        .map(|snapshot| snapshot.applied_event_count)
        .unwrap_or(0);
    if checkpoint_event_count == 0
        || checkpoint_event_count > events.len()
        || events.len().saturating_sub(checkpoint_event_count)
            >= CURRENT_SYNTHESIS_CHECKPOINT_INTERVAL
    {
        let checkpoint_feedback = if state.completed_ticks == 0 {
            effective_feedback
        } else {
            last_feedback
        };
        let _ = write_state_snapshot_at(root, &state, Some(checkpoint_feedback), events.len())?;
    }

    Ok((persisted, state))
}

pub fn select_current_synthesis_scenario_at(
    root: &Path,
    scenario_id: &str,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let scenario = load_scenario(scenario_id)?;
    let mut events =
        read_or_create_events_at(root, effective_hueman_feedback(current_hueman_feedback))?;
    events.push(CurrentSynthesisEvent::ScenarioSelected {
        scenario_id: scenario.id,
        seed: DEFAULT_SEED,
        focused_npc_id: scenario.default_focused_npc_id,
    });
    write_event_log_at(root, &events)?;
    load_current_synthesis_at(root, current_hueman_feedback)
}

pub fn focus_current_synthesis_npc_at(
    root: &Path,
    focused_npc_id: &str,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let effective_feedback = effective_hueman_feedback(current_hueman_feedback);
    let (_persisted, state) = load_current_synthesis_at(root, Some(effective_feedback))?;
    if state.find_npc(focused_npc_id).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown npc id: {focused_npc_id}"),
        ));
    }
    let mut events = read_or_create_events_at(root, effective_feedback)?;
    events.push(CurrentSynthesisEvent::FocusedNpcChanged {
        focused_npc_id: focused_npc_id.to_owned(),
    });
    write_event_log_at(root, &events)?;
    load_current_synthesis_at(root, Some(effective_feedback))
}

pub fn plan_current_synthesis_player_action_at(
    root: &Path,
    action_label: &str,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let effective_feedback = effective_hueman_feedback(current_hueman_feedback);
    let trimmed = action_label.trim();
    if trimmed.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "player plan requires a non-empty action label",
        ));
    }
    let mut events = read_or_create_events_at(root, effective_feedback)?;
    events.push(CurrentSynthesisEvent::PlayerActionPlanned {
        action_label: trimmed.to_owned(),
    });
    write_event_log_at(root, &events)?;
    load_current_synthesis_at(root, Some(effective_feedback))
}

pub fn advance_current_synthesis_player_action_at(
    root: &Path,
    encoded_action_label: &str,
    focused_npc_id: Option<&str>,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    let effective_feedback = effective_hueman_feedback(current_hueman_feedback);
    let trimmed = encoded_action_label.trim();
    if trimmed.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "player action requires a non-empty action label",
        ));
    }
    let (persisted, state) = load_current_synthesis_at(root, Some(effective_feedback))?;
    let selected_npc_id = focused_npc_id.unwrap_or(&persisted.focused_npc_id);
    if state.find_npc(selected_npc_id).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown npc id: {selected_npc_id}"),
        ));
    }

    let mut events = read_or_create_events_at(root, effective_feedback)?;
    events.push(CurrentSynthesisEvent::PlayerActionPlanned {
        action_label: trimmed.to_owned(),
    });
    if selected_npc_id != persisted.focused_npc_id {
        events.push(CurrentSynthesisEvent::FocusedNpcChanged {
            focused_npc_id: selected_npc_id.to_owned(),
        });
    }
    events.push(CurrentSynthesisEvent::CleopatraTicked {
        focused_npc_id: selected_npc_id.to_owned(),
        hueman_feedback: effective_feedback,
    });
    write_event_log_at(root, &events)?;
    load_current_synthesis_at(root, Some(effective_feedback))
}

pub fn append_current_synthesis_tick_at(
    root: &Path,
    focused_npc_id: Option<&str>,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    append_current_synthesis_ticks_at(root, focused_npc_id, 1, current_hueman_feedback)
}

pub fn append_current_synthesis_ticks_at(
    root: &Path,
    focused_npc_id: Option<&str>,
    tick_count: usize,
    current_hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<(PersistedCurrentSynthesisState, CurrentSynthesisState)> {
    if tick_count == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "tick count must be greater than zero",
        ));
    }

    let effective_feedback = effective_hueman_feedback(current_hueman_feedback);
    let (persisted, state) = load_current_synthesis_at(root, Some(effective_feedback))?;
    let selected_npc_id = focused_npc_id.unwrap_or(&persisted.focused_npc_id);
    if state.find_npc(selected_npc_id).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown npc id: {selected_npc_id}"),
        ));
    }

    let mut events = read_or_create_events_at(root, effective_feedback)?;
    if selected_npc_id != persisted.focused_npc_id {
        events.push(CurrentSynthesisEvent::FocusedNpcChanged {
            focused_npc_id: selected_npc_id.to_owned(),
        });
    }
    for _ in 0..tick_count {
        events.push(CurrentSynthesisEvent::CleopatraTicked {
            focused_npc_id: selected_npc_id.to_owned(),
            hueman_feedback: effective_feedback,
        });
    }
    write_event_log_at(root, &events)?;
    load_current_synthesis_at(root, Some(effective_feedback))
}

pub fn read_or_replay_state_at(
    root: &Path,
    persisted: &PersistedCurrentSynthesisState,
    hueman_feedback: Option<HuemanFeedback>,
) -> io::Result<CurrentSynthesisState> {
    let (derived_persisted, state) = load_current_synthesis_at(root, hueman_feedback)?;
    if derived_persisted.scenario_id != persisted.scenario_id
        || derived_persisted.seed != persisted.seed
        || derived_persisted.completed_ticks != persisted.completed_ticks
        || derived_persisted.focused_npc_id != persisted.focused_npc_id
    {
        let _ = write_persisted_state_at(root, &derived_persisted);
    }
    Ok(state)
}

fn build_state_snapshot_output(
    state: &CurrentSynthesisState,
    hueman_feedback: Option<HuemanFeedback>,
    applied_event_count: usize,
) -> String {
    let hueman_feedback = effective_hueman_feedback(hueman_feedback);
    let mut output = String::from("# Current Synthesis Snapshot\n");
    output.push_str(&format!("applied_event_count: {applied_event_count}\n"));
    push_snapshot_line(&mut output, "scenario_id", &state.scenario_id);
    push_snapshot_line(&mut output, "focused_npc_id", &state.focused_npc_id);
    output.push_str(&format!("seed: {}\n", state.seed));
    output.push_str(&format!("completed_ticks: {}\n", state.completed_ticks));
    push_snapshot_line(&mut output, "player_need", &state.player_need);
    for action in &state.planned_player_actions {
        push_snapshot_line(&mut output, "planned_player_action", action);
    }
    output.push_str(&format!("hueman_unlocked: {}\n", hueman_feedback.unlocked));
    output.push_str(&format!(
        "hueman_recognized_route_branch: {}\n",
        hueman_feedback.recognized_route_branch
    ));
    output.push_str(&format!(
        "hueman_recognized_defense_branch: {}\n",
        hueman_feedback.recognized_defense_branch
    ));
    output.push_str(&format!(
        "world_route_stability: {}\n",
        state.world.route_stability
    ));
    output.push_str(&format!(
        "world_shelter_integrity: {}\n",
        state.world.shelter_integrity
    ));
    output.push_str(&format!(
        "world_power_stability: {}\n",
        state.world.power_stability
    ));
    output.push_str(&format!(
        "world_labor_availability: {}\n",
        state.world.labor_availability
    ));
    output.push_str(&format!(
        "world_faction_tension: {}\n",
        state.world.faction_tension
    ));
    output.push_str(&format!(
        "world_conflict_risk: {}\n",
        state.world.conflict_risk
    ));
    push_resource_lines(
        &mut output,
        "resources_aura_total",
        "resources_aura_property",
        "resources_current_total",
        "resources_current_property",
        &state.resources,
    );
    for npc in &state.npcs {
        push_snapshot_fields(
            &mut output,
            "npc",
            &[
                &npc.id,
                &npc.name,
                &npc.role,
                &npc.faction,
                &npc.location,
                &npc.condition,
            ],
        );
        for need in &npc.needs {
            push_snapshot_fields(&mut output, "npc_need", &[&npc.id, need]);
        }
        for memory in &npc.memories {
            push_snapshot_fields(&mut output, "npc_memory", &[&npc.id, memory]);
        }
        for relationship in &npc.relationships {
            push_snapshot_fields(&mut output, "npc_relationship", &[&npc.id, relationship]);
        }
        for world in &npc.perceived_world {
            push_snapshot_fields(&mut output, "npc_world", &[&npc.id, world]);
        }
    }
    for active_npc in &state.cleopatra.active_npcs {
        push_snapshot_line(&mut output, "cleopatra_active_npc", active_npc);
    }
    for queued in &state.cleopatra.queued_blep_passes {
        push_snapshot_line(&mut output, "cleopatra_queue", queued);
    }
    for condition in &state.cleopatra.faction_conditions {
        push_snapshot_line(&mut output, "cleopatra_faction_condition", condition);
    }
    for condition in &state.cleopatra.settlement_conditions {
        push_snapshot_line(&mut output, "cleopatra_settlement_condition", condition);
    }
    for condition in &state.cleopatra.war_conditions {
        push_snapshot_line(&mut output, "cleopatra_war_condition", condition);
    }
    for recent in &state.cleopatra.recent_blep_passes {
        push_snapshot_line(&mut output, "cleopatra_recent_blep", recent);
    }
    push_resource_lines(
        &mut output,
        "cleopatra_global_aura_total",
        "cleopatra_global_aura_property",
        "cleopatra_global_current_total",
        "cleopatra_global_current_property",
        &state.cleopatra.global_resource_composition,
    );
    for tick in &state.history {
        let tick_number = tick.tick_number.to_string();
        push_snapshot_fields(
            &mut output,
            "tick",
            &[&tick_number, &tick.player_moment_id, &tick.npc_moment_id],
        );
        push_snapshot_fields(
            &mut output,
            "tick_forward",
            &[
                &tick_number,
                &tick.forward_pass.human_need,
                &tick.forward_pass.machine_complement,
                &tick.forward_pass.bond_result.selected_bond.id,
                &tick.forward_pass.bond_result.resulting_link,
                &tick.forward_pass.bond_result.resulting_moment,
            ],
        );
        for way in &tick.forward_pass.available_ways {
            push_snapshot_fields(
                &mut output,
                "tick_forward_way",
                &[&tick_number, &serialize_engine_way(way)],
            );
        }
        for bond in &tick.forward_pass.candidate_bonds {
            push_snapshot_fields(
                &mut output,
                "tick_forward_candidate",
                &[&tick_number, &serialize_bond_candidate(bond)],
            );
        }
        for bond in &tick.forward_pass.bond_result.unused_bonds {
            push_snapshot_fields(
                &mut output,
                "tick_forward_unused_bond",
                &[&tick_number, &serialize_bond_candidate(bond)],
            );
        }
        for residue in &tick.forward_residues {
            push_snapshot_fields(
                &mut output,
                "tick_forward_residue",
                &[&tick_number, &serialize_residue(residue)],
            );
        }
        push_snapshot_fields(
            &mut output,
            "tick_blep",
            &[
                &tick_number,
                &tick.blep_decision.npc_id,
                &tick.blep_decision.inferred_need,
                &tick.blep_decision.selected_bond.id,
                &tick.blep_decision.resulting_action,
                &tick.blep_decision.confidence.to_string(),
            ],
        );
        for world_input in &tick.blep_decision.world_inputs {
            push_snapshot_fields(
                &mut output,
                "tick_blep_world_input",
                &[&tick_number, world_input],
            );
        }
        for bond in &tick.blep_decision.candidate_bonds {
            push_snapshot_fields(
                &mut output,
                "tick_blep_candidate",
                &[&tick_number, &serialize_bond_candidate(bond)],
            );
        }
        for bond in &tick.blep_decision.unused_bonds {
            push_snapshot_fields(
                &mut output,
                "tick_blep_unused_bond",
                &[&tick_number, &serialize_bond_candidate(bond)],
            );
        }
        for residue in &tick.blep_residues {
            push_snapshot_fields(
                &mut output,
                "tick_blep_residue",
                &[&tick_number, &serialize_residue(residue)],
            );
        }
        for relay in &tick.coordinated_blep_relays {
            push_snapshot_fields(
                &mut output,
                "tick_blep_relay",
                &[
                    &tick_number,
                    &relay.npc_id,
                    &relay.npc_moment_id,
                    if relay.committed { "true" } else { "false" },
                    &relay.blep_decision.inferred_need,
                    &relay.blep_decision.selected_bond.id,
                    &relay.blep_decision.resulting_action,
                    &relay.blep_decision.confidence.to_string(),
                ],
            );
            for world_input in &relay.blep_decision.world_inputs {
                push_snapshot_fields(
                    &mut output,
                    "tick_blep_relay_world_input",
                    &[&tick_number, &relay.npc_id, world_input],
                );
            }
            for bond in &relay.blep_decision.candidate_bonds {
                push_snapshot_fields(
                    &mut output,
                    "tick_blep_relay_candidate",
                    &[&tick_number, &relay.npc_id, &serialize_bond_candidate(bond)],
                );
            }
            for bond in &relay.blep_decision.unused_bonds {
                push_snapshot_fields(
                    &mut output,
                    "tick_blep_relay_unused_bond",
                    &[&tick_number, &relay.npc_id, &serialize_bond_candidate(bond)],
                );
            }
            for residue in &relay.blep_residues {
                push_snapshot_fields(
                    &mut output,
                    "tick_blep_relay_residue",
                    &[&tick_number, &relay.npc_id, &serialize_residue(residue)],
                );
            }
        }
        push_resource_lines(
            &mut output,
            &format!("tick_resources_aura_total/{}", tick.tick_number),
            &format!("tick_resources_aura_property/{}", tick.tick_number),
            &format!("tick_resources_current_total/{}", tick.tick_number),
            &format!("tick_resources_current_property/{}", tick.tick_number),
            &tick.resources_after_tick,
        );
    }
    output
}

fn parse_state_snapshot(contents: &str) -> io::Result<StateSnapshot> {
    let mut applied_event_count = None;
    let mut scenario_id = None;
    let mut focused_npc_id = None;
    let mut seed = None;
    let mut completed_ticks = None;
    let mut player_need = None;
    let mut planned_player_actions = Vec::new();
    let mut hueman_unlocked = None;
    let mut hueman_route = None;
    let mut hueman_defense = None;
    let mut world = WorldState::default();
    let mut resources = ResourceComposition::empty();
    let mut npcs = Vec::<NpcState>::new();
    let mut cleopatra = CleopatraState {
        active_npcs: Vec::new(),
        queued_blep_passes: Vec::new(),
        faction_conditions: Vec::new(),
        settlement_conditions: Vec::new(),
        war_conditions: Vec::new(),
        global_resource_composition: ResourceComposition::empty(),
        recent_blep_passes: Vec::new(),
    };
    let mut tick_builders = BTreeMap::<u32, SnapshotTickBuilder>::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("snapshot line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "applied_event_count" => {
                applied_event_count = Some(value.parse::<usize>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid usize for applied_event_count: {value}"),
                    )
                })?)
            }
            "scenario_id" => scenario_id = Some(unescape_snapshot_value(value)?),
            "focused_npc_id" => focused_npc_id = Some(unescape_snapshot_value(value)?),
            "seed" => seed = Some(parse_snapshot_u64(value, key)?),
            "completed_ticks" => completed_ticks = Some(parse_snapshot_u32(value, key)?),
            "player_need" => player_need = Some(unescape_snapshot_value(value)?),
            "planned_player_action" => planned_player_actions.push(unescape_snapshot_value(value)?),
            "hueman_unlocked" => hueman_unlocked = Some(parse_snapshot_bool(value, key)?),
            "hueman_recognized_route_branch" => {
                hueman_route = Some(parse_snapshot_bool(value, key)?)
            }
            "hueman_recognized_defense_branch" => {
                hueman_defense = Some(parse_snapshot_bool(value, key)?)
            }
            "world_route_stability" => world.route_stability = parse_snapshot_u16(value, key)?,
            "world_shelter_integrity" => world.shelter_integrity = parse_snapshot_u16(value, key)?,
            "world_power_stability" => world.power_stability = parse_snapshot_u16(value, key)?,
            "world_labor_availability" => {
                world.labor_availability = parse_snapshot_u16(value, key)?
            }
            "world_faction_tension" => world.faction_tension = parse_snapshot_u16(value, key)?,
            "world_conflict_risk" => world.conflict_risk = parse_snapshot_u16(value, key)?,
            "resources_aura_total" => resources.aura_total = parse_snapshot_u16(value, key)?,
            "resources_current_total" => resources.current_total = parse_snapshot_u16(value, key)?,
            "resources_aura_property" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                resources
                    .aura_properties
                    .insert(fields[0].clone(), parse_snapshot_u16(&fields[1], key)?);
            }
            "resources_current_property" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                resources
                    .current_properties
                    .insert(fields[0].clone(), parse_snapshot_u16(&fields[1], key)?);
            }
            "npc" => {
                let fields = split_snapshot_fields(value, 6, key)?;
                npcs.push(NpcState {
                    id: fields[0].clone(),
                    name: fields[1].clone(),
                    role: fields[2].clone(),
                    faction: fields[3].clone(),
                    location: fields[4].clone(),
                    condition: fields[5].clone(),
                    needs: Vec::new(),
                    memories: Vec::new(),
                    relationships: Vec::new(),
                    perceived_world: Vec::new(),
                });
            }
            "npc_need" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                find_snapshot_npc_mut(&mut npcs, &fields[0], key)?
                    .needs
                    .push(fields[1].clone());
            }
            "npc_memory" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                find_snapshot_npc_mut(&mut npcs, &fields[0], key)?
                    .memories
                    .push(fields[1].clone());
            }
            "npc_relationship" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                find_snapshot_npc_mut(&mut npcs, &fields[0], key)?
                    .relationships
                    .push(fields[1].clone());
            }
            "npc_world" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                find_snapshot_npc_mut(&mut npcs, &fields[0], key)?
                    .perceived_world
                    .push(fields[1].clone());
            }
            "cleopatra_active_npc" => cleopatra.active_npcs.push(unescape_snapshot_value(value)?),
            "cleopatra_queue" => cleopatra
                .queued_blep_passes
                .push(unescape_snapshot_value(value)?),
            "cleopatra_faction_condition" => cleopatra
                .faction_conditions
                .push(unescape_snapshot_value(value)?),
            "cleopatra_settlement_condition" => cleopatra
                .settlement_conditions
                .push(unescape_snapshot_value(value)?),
            "cleopatra_war_condition" => cleopatra
                .war_conditions
                .push(unescape_snapshot_value(value)?),
            "cleopatra_recent_blep" => cleopatra
                .recent_blep_passes
                .push(unescape_snapshot_value(value)?),
            "cleopatra_global_aura_total" => {
                cleopatra.global_resource_composition.aura_total = parse_snapshot_u16(value, key)?
            }
            "cleopatra_global_current_total" => {
                cleopatra.global_resource_composition.current_total =
                    parse_snapshot_u16(value, key)?
            }
            "cleopatra_global_aura_property" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                cleopatra
                    .global_resource_composition
                    .aura_properties
                    .insert(fields[0].clone(), parse_snapshot_u16(&fields[1], key)?);
            }
            "cleopatra_global_current_property" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                cleopatra
                    .global_resource_composition
                    .current_properties
                    .insert(fields[0].clone(), parse_snapshot_u16(&fields[1], key)?);
            }
            "tick" => {
                let fields = split_snapshot_fields(value, 3, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                builder.player_moment_id = Some(fields[1].clone());
                builder.npc_moment_id = Some(fields[2].clone());
            }
            "tick_forward" => {
                let fields = split_snapshot_fields(value, 6, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                builder.forward_human_need = Some(fields[1].clone());
                builder.forward_machine_complement = Some(fields[2].clone());
                builder.forward_selected_bond_id = Some(fields[3].clone());
                builder.forward_resulting_link = Some(fields[4].clone());
                builder.forward_resulting_moment = Some(fields[5].clone());
            }
            "tick_forward_way" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .forward_available_ways
                    .push(parse_engine_way(&fields[1])?);
            }
            "tick_forward_candidate" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .forward_candidate_bonds
                    .push(parse_bond_candidate(&fields[1])?);
            }
            "tick_forward_unused_bond" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .forward_unused_bonds
                    .push(parse_bond_candidate(&fields[1])?);
            }
            "tick_forward_residue" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .forward_residues
                    .push(parse_residue(&fields[1])?);
            }
            "tick_blep" => {
                let fields = split_snapshot_fields(value, 6, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                builder.blep_npc_id = Some(fields[1].clone());
                builder.blep_inferred_need = Some(fields[2].clone());
                builder.blep_selected_bond_id = Some(fields[3].clone());
                builder.blep_resulting_action = Some(fields[4].clone());
                builder.blep_confidence = Some(parse_snapshot_u16(&fields[5], key)?);
            }
            "tick_blep_world_input" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .blep_world_inputs
                    .push(fields[1].clone());
            }
            "tick_blep_candidate" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .blep_candidate_bonds
                    .push(parse_bond_candidate(&fields[1])?);
            }
            "tick_blep_unused_bond" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .blep_unused_bonds
                    .push(parse_bond_candidate(&fields[1])?);
            }
            "tick_blep_residue" => {
                let fields = split_snapshot_fields(value, 2, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .blep_residues
                    .push(parse_residue(&fields[1])?);
            }
            "tick_blep_relay" => {
                let fields = split_snapshot_fields(value, 8, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                if !builder
                    .relay_order
                    .iter()
                    .any(|npc_id| npc_id == &fields[1])
                {
                    builder.relay_order.push(fields[1].clone());
                }
                let relay = builder
                    .coordinated_relays
                    .entry(fields[1].clone())
                    .or_default();
                relay.npc_moment_id = Some(fields[2].clone());
                relay.committed = Some(parse_snapshot_bool(&fields[3], key)?);
                relay.inferred_need = Some(fields[4].clone());
                relay.selected_bond_id = Some(fields[5].clone());
                relay.resulting_action = Some(fields[6].clone());
                relay.confidence = Some(parse_snapshot_u16(&fields[7], key)?);
            }
            "tick_blep_relay_world_input" => {
                let fields = split_snapshot_fields(value, 3, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                if !builder
                    .relay_order
                    .iter()
                    .any(|npc_id| npc_id == &fields[1])
                {
                    builder.relay_order.push(fields[1].clone());
                }
                builder
                    .coordinated_relays
                    .entry(fields[1].clone())
                    .or_default()
                    .world_inputs
                    .push(fields[2].clone());
            }
            "tick_blep_relay_candidate" => {
                let fields = split_snapshot_fields(value, 3, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                if !builder
                    .relay_order
                    .iter()
                    .any(|npc_id| npc_id == &fields[1])
                {
                    builder.relay_order.push(fields[1].clone());
                }
                builder
                    .coordinated_relays
                    .entry(fields[1].clone())
                    .or_default()
                    .candidate_bonds
                    .push(parse_bond_candidate(&fields[2])?);
            }
            "tick_blep_relay_unused_bond" => {
                let fields = split_snapshot_fields(value, 3, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                if !builder
                    .relay_order
                    .iter()
                    .any(|npc_id| npc_id == &fields[1])
                {
                    builder.relay_order.push(fields[1].clone());
                }
                builder
                    .coordinated_relays
                    .entry(fields[1].clone())
                    .or_default()
                    .unused_bonds
                    .push(parse_bond_candidate(&fields[2])?);
            }
            "tick_blep_relay_residue" => {
                let fields = split_snapshot_fields(value, 3, key)?;
                let tick_number = parse_snapshot_u32(&fields[0], key)?;
                let builder = tick_builders.entry(tick_number).or_default();
                if !builder
                    .relay_order
                    .iter()
                    .any(|npc_id| npc_id == &fields[1])
                {
                    builder.relay_order.push(fields[1].clone());
                }
                builder
                    .coordinated_relays
                    .entry(fields[1].clone())
                    .or_default()
                    .residues
                    .push(parse_residue(&fields[2])?);
            }
            other if other.starts_with("tick_resources_aura_total/") => {
                let tick_number =
                    parse_snapshot_u32(&other["tick_resources_aura_total/".len()..], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .resources_after_tick
                    .aura_total = parse_snapshot_u16(value, key)?;
            }
            other if other.starts_with("tick_resources_current_total/") => {
                let tick_number =
                    parse_snapshot_u32(&other["tick_resources_current_total/".len()..], key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .resources_after_tick
                    .current_total = parse_snapshot_u16(value, key)?;
            }
            other if other.starts_with("tick_resources_aura_property/") => {
                let tick_number =
                    parse_snapshot_u32(&other["tick_resources_aura_property/".len()..], key)?;
                let fields = split_snapshot_fields(value, 2, key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .resources_after_tick
                    .aura_properties
                    .insert(fields[0].clone(), parse_snapshot_u16(&fields[1], key)?);
            }
            other if other.starts_with("tick_resources_current_property/") => {
                let tick_number =
                    parse_snapshot_u32(&other["tick_resources_current_property/".len()..], key)?;
                let fields = split_snapshot_fields(value, 2, key)?;
                tick_builders
                    .entry(tick_number)
                    .or_default()
                    .resources_after_tick
                    .current_properties
                    .insert(fields[0].clone(), parse_snapshot_u16(&fields[1], key)?);
            }
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("snapshot contains unknown key: {other}"),
                ));
            }
        }
    }

    let history = tick_builders
        .into_iter()
        .map(|(tick_number, builder)| build_tick_record_from_snapshot_builder(tick_number, builder))
        .collect::<io::Result<Vec<_>>>()?;

    Ok(StateSnapshot {
        applied_event_count: applied_event_count.unwrap_or(0),
        state: CurrentSynthesisState {
            scenario_id: required_snapshot_field(scenario_id, "scenario_id")?,
            focused_npc_id: required_snapshot_field(focused_npc_id, "focused_npc_id")?,
            seed: required_snapshot_field(seed, "seed")?,
            completed_ticks: required_snapshot_field(completed_ticks, "completed_ticks")?,
            player_need: required_snapshot_field(player_need, "player_need")?,
            planned_player_actions,
            resources,
            world,
            npcs,
            cleopatra,
            history,
        },
        hueman_feedback: HuemanFeedback {
            unlocked: required_snapshot_field(hueman_unlocked, "hueman_unlocked")?,
            recognized_route_branch: required_snapshot_field(
                hueman_route,
                "hueman_recognized_route_branch",
            )?,
            recognized_defense_branch: required_snapshot_field(
                hueman_defense,
                "hueman_recognized_defense_branch",
            )?,
        },
    })
}

fn read_state_snapshot_at(root: &Path) -> io::Result<StateSnapshot> {
    let contents = crate::read_text_artifact(&root.join(CURRENT_SYNTHESIS_SNAPSHOT_ARTIFACT_PATH))?;
    parse_state_snapshot(&contents)
}

pub fn write_state_snapshot_at(
    root: &Path,
    state: &CurrentSynthesisState,
    hueman_feedback: Option<HuemanFeedback>,
    applied_event_count: usize,
) -> io::Result<PathBuf> {
    let path = root.join(CURRENT_SYNTHESIS_SNAPSHOT_ARTIFACT_PATH);
    write_text_artifact(
        &path,
        &build_state_snapshot_output(state, hueman_feedback, applied_event_count),
    )?;
    Ok(path)
}

fn build_event_log_output(events: &[CurrentSynthesisEvent]) -> String {
    let mut output = String::from("# Current Synthesis Event Log\n");
    for event in events {
        match event {
            CurrentSynthesisEvent::ScenarioSelected {
                scenario_id,
                seed,
                focused_npc_id,
            } => push_snapshot_fields(
                &mut output,
                "event",
                &[
                    "scenario_selected",
                    scenario_id,
                    &seed.to_string(),
                    focused_npc_id,
                ],
            ),
            CurrentSynthesisEvent::FocusedNpcChanged { focused_npc_id } => push_snapshot_fields(
                &mut output,
                "event",
                &["focused_npc_changed", focused_npc_id],
            ),
            CurrentSynthesisEvent::PlayerActionPlanned { action_label } => push_snapshot_fields(
                &mut output,
                "event",
                &["player_action_planned", action_label],
            ),
            CurrentSynthesisEvent::HuemanFeedbackChanged { hueman_feedback } => {
                push_snapshot_fields(
                    &mut output,
                    "event",
                    &[
                        "hueman_feedback_changed",
                        if hueman_feedback.unlocked {
                            "true"
                        } else {
                            "false"
                        },
                        if hueman_feedback.recognized_route_branch {
                            "true"
                        } else {
                            "false"
                        },
                        if hueman_feedback.recognized_defense_branch {
                            "true"
                        } else {
                            "false"
                        },
                    ],
                )
            }
            CurrentSynthesisEvent::CleopatraTicked {
                focused_npc_id,
                hueman_feedback,
            } => push_snapshot_fields(
                &mut output,
                "event",
                &[
                    "cleopatra_ticked",
                    focused_npc_id,
                    if hueman_feedback.unlocked {
                        "true"
                    } else {
                        "false"
                    },
                    if hueman_feedback.recognized_route_branch {
                        "true"
                    } else {
                        "false"
                    },
                    if hueman_feedback.recognized_defense_branch {
                        "true"
                    } else {
                        "false"
                    },
                ],
            ),
        }
    }
    output
}

fn parse_event_log(contents: &str) -> io::Result<Vec<CurrentSynthesisEvent>> {
    let mut events = Vec::new();
    let lines = contents.lines().collect::<Vec<_>>();
    for (idx, raw_line) in lines.iter().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            if idx + 1 == lines.len() {
                break;
            }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("event log line is missing ':' separator: {line}"),
            ));
        };
        if key.trim() != "event" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("event log contains unknown key: {}", key.trim()),
            ));
        }
        let kind = value
            .trim()
            .split_once('\t')
            .map(|(kind, _)| kind)
            .unwrap_or(value.trim());
        match kind {
            "scenario_selected" => {
                let fields = split_snapshot_fields(value.trim(), 4, "event")?;
                events.push(CurrentSynthesisEvent::ScenarioSelected {
                    scenario_id: fields[1].clone(),
                    seed: parse_snapshot_u64(&fields[2], "event seed")?,
                    focused_npc_id: fields[3].clone(),
                });
            }
            "focused_npc_changed" => {
                let fields = split_snapshot_fields(value.trim(), 2, "event")?;
                events.push(CurrentSynthesisEvent::FocusedNpcChanged {
                    focused_npc_id: fields[1].clone(),
                });
            }
            "player_action_planned" => {
                let fields = split_snapshot_fields(value.trim(), 2, "event")?;
                events.push(CurrentSynthesisEvent::PlayerActionPlanned {
                    action_label: fields[1].clone(),
                });
            }
            "hueman_feedback_changed" => {
                let fields = split_snapshot_fields(value.trim(), 4, "event")?;
                events.push(CurrentSynthesisEvent::HuemanFeedbackChanged {
                    hueman_feedback: HuemanFeedback {
                        unlocked: parse_snapshot_bool(&fields[1], "event hueman_unlocked")?,
                        recognized_route_branch: parse_snapshot_bool(
                            &fields[2],
                            "event recognized_route_branch",
                        )?,
                        recognized_defense_branch: parse_snapshot_bool(
                            &fields[3],
                            "event recognized_defense_branch",
                        )?,
                    },
                });
            }
            "cleopatra_ticked" => {
                let fields = split_snapshot_fields(value.trim(), 5, "event")?;
                events.push(CurrentSynthesisEvent::CleopatraTicked {
                    focused_npc_id: fields[1].clone(),
                    hueman_feedback: HuemanFeedback {
                        unlocked: parse_snapshot_bool(&fields[2], "event hueman_unlocked")?,
                        recognized_route_branch: parse_snapshot_bool(
                            &fields[3],
                            "event recognized_route_branch",
                        )?,
                        recognized_defense_branch: parse_snapshot_bool(
                            &fields[4],
                            "event recognized_defense_branch",
                        )?,
                    },
                });
            }
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unsupported current synthesis event: {other}"),
                ));
            }
        }
    }
    Ok(events)
}

fn read_events_at(root: &Path) -> io::Result<Vec<CurrentSynthesisEvent>> {
    let contents =
        crate::read_text_artifact(&root.join(CURRENT_SYNTHESIS_EVENT_LOG_ARTIFACT_PATH))?;
    parse_event_log(&contents)
}

fn synthesize_events_from_persisted_state(
    persisted: &PersistedCurrentSynthesisState,
    hueman_feedback: HuemanFeedback,
) -> Vec<CurrentSynthesisEvent> {
    let mut events = vec![CurrentSynthesisEvent::ScenarioSelected {
        scenario_id: persisted.scenario_id.clone(),
        seed: persisted.seed,
        focused_npc_id: persisted.focused_npc_id.clone(),
    }];
    for _ in 0..persisted.completed_ticks {
        events.push(CurrentSynthesisEvent::CleopatraTicked {
            focused_npc_id: persisted.focused_npc_id.clone(),
            hueman_feedback,
        });
    }
    events
}

fn read_or_create_events_at(
    root: &Path,
    hueman_feedback: HuemanFeedback,
) -> io::Result<Vec<CurrentSynthesisEvent>> {
    match read_events_at(root) {
        Ok(events) if !events.is_empty() => Ok(events),
        Ok(_) => {
            let events = synthesize_events_from_persisted_state(
                &PersistedCurrentSynthesisState::primary(),
                hueman_feedback,
            );
            write_event_log_at(root, &events)?;
            Ok(events)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let persisted = match crate::read_text_artifact(
                &root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH),
            ) {
                Ok(contents) => parse_persisted_state(&contents)?,
                Err(read_error) if read_error.kind() == io::ErrorKind::NotFound => {
                    PersistedCurrentSynthesisState::primary()
                }
                Err(read_error) => return Err(read_error),
            };
            let events = synthesize_events_from_persisted_state(&persisted, hueman_feedback);
            write_event_log_at(root, &events)?;
            Ok(events)
        }
        Err(error) => Err(error),
    }
}

fn write_event_log_at(root: &Path, events: &[CurrentSynthesisEvent]) -> io::Result<PathBuf> {
    let path = root.join(CURRENT_SYNTHESIS_EVENT_LOG_ARTIFACT_PATH);
    write_text_artifact(&path, &build_event_log_output(events))?;
    Ok(path)
}

fn apply_current_synthesis_event(
    state: Option<CurrentSynthesisState>,
    last_feedback: HuemanFeedback,
    event: &CurrentSynthesisEvent,
) -> io::Result<(CurrentSynthesisState, HuemanFeedback)> {
    match event {
        CurrentSynthesisEvent::ScenarioSelected {
            scenario_id,
            seed,
            focused_npc_id,
        } => {
            let scenario = load_scenario(scenario_id)?;
            let persisted = PersistedCurrentSynthesisState {
                scenario_id: scenario.id.clone(),
                seed: *seed,
                completed_ticks: 0,
                focused_npc_id: focused_npc_id.clone(),
            };
            Ok((
                CurrentSynthesisState::initial(&persisted, &scenario),
                last_feedback,
            ))
        }
        CurrentSynthesisEvent::FocusedNpcChanged { focused_npc_id } => {
            let mut state = state.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "focus change event encountered before scenario selection",
                )
            })?;
            if state.find_npc(focused_npc_id).is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("focus change event references unknown npc id: {focused_npc_id}"),
                ));
            }
            state.focused_npc_id = focused_npc_id.clone();
            state.cleopatra.queued_blep_passes = vec![focused_npc_id.clone()];
            Ok((state, last_feedback))
        }
        CurrentSynthesisEvent::PlayerActionPlanned { action_label } => {
            let mut state = state.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "player action event encountered before scenario selection",
                )
            })?;
            state.planned_player_actions.push(action_label.clone());
            Ok((state, last_feedback))
        }
        CurrentSynthesisEvent::HuemanFeedbackChanged { hueman_feedback } => Ok((
            state.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "hueman feedback event encountered before scenario selection",
                )
            })?,
            *hueman_feedback,
        )),
        CurrentSynthesisEvent::CleopatraTicked {
            focused_npc_id,
            hueman_feedback,
        } => {
            let mut state = state.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "tick event encountered before scenario selection",
                )
            })?;
            if state.find_npc(focused_npc_id).is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("tick event references unknown npc id: {focused_npc_id}"),
                ));
            }
            state.focused_npc_id = focused_npc_id.clone();
            state.cleopatra.queued_blep_passes = vec![focused_npc_id.clone()];
            state.tick(Some(*hueman_feedback));
            Ok((state, *hueman_feedback))
        }
    }
}

fn reconstruct_state_from_storage(
    events: &[CurrentSynthesisEvent],
    checkpoint: Option<StateSnapshot>,
) -> io::Result<(CurrentSynthesisState, HuemanFeedback)> {
    let mut state = None;
    let mut last_feedback = effective_hueman_feedback(None);
    let start_index = if let Some(checkpoint) = checkpoint {
        if checkpoint.applied_event_count <= events.len() {
            state = Some(checkpoint.state);
            last_feedback = checkpoint.hueman_feedback;
            checkpoint.applied_event_count
        } else {
            0
        }
    } else {
        0
    };

    for event in &events[start_index..] {
        let (next_state, next_feedback) =
            apply_current_synthesis_event(state, last_feedback, event)?;
        state = Some(next_state);
        last_feedback = next_feedback;
    }

    state.map(|state| (state, last_feedback)).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "current synthesis event log did not produce a state",
        )
    })
}

pub fn build_view_artifacts(
    root: &Path,
    persisted: &PersistedCurrentSynthesisState,
    state: &CurrentSynthesisState,
) -> Vec<ViewArtifact> {
    let last_selected_bond_id = state
        .last_tick()
        .map(|tick| tick.forward_pass.bond_result.selected_bond.id.as_str())
        .unwrap_or("bond/player/enhanced-sight/1");
    vec![
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_ENGINE_STATUS_ARTIFACT_PATH),
            contents: build_engine_output(state, EngineLens::Status),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_WORLD_CONTEXT_ARTIFACT_PATH),
            contents: build_world_context_output(),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_ALIGNMENT_WITNESS_ARTIFACT_PATH),
            contents: build_hollow_grove_alignment_witness(),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_ALIGNMENT_VALIDATION_ARTIFACT_PATH),
            contents: build_hollow_grove_alignment_validation_report(),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_BOND_INSPECTOR_ARTIFACT_PATH),
            contents: build_bond_inspector_output(state, last_selected_bond_id)
                .unwrap_or_else(|| String::from("# Bond Inspector\n\nbond not found\n")),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_RESOURCE_INSPECTOR_ARTIFACT_PATH),
            contents: build_resource_history_output(state),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_NPC_INSPECTOR_ARTIFACT_PATH),
            contents: build_npc_inspector_output(state, &persisted.focused_npc_id)
                .unwrap_or_else(|| String::from("# NPC Inspector\n\nnpc not found\n")),
        },
        ViewArtifact {
            path: root.join(CURRENT_SYNTHESIS_CLEOPATRA_ARTIFACT_PATH),
            contents: build_cleopatra_status_output(state),
        },
    ]
}

pub fn write_view_artifacts(
    root: &Path,
    persisted: &PersistedCurrentSynthesisState,
    state: &CurrentSynthesisState,
) -> io::Result<Vec<PathBuf>> {
    let artifacts = build_view_artifacts(root, persisted, state);
    let mut written = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        write_text_artifact(&artifact.path, &artifact.contents)?;
        written.push(artifact.path);
    }
    Ok(written)
}

pub fn stage_view_artifacts(
    session: &mut ArtifactSession,
    root: &Path,
    persisted: &PersistedCurrentSynthesisState,
    state: &CurrentSynthesisState,
) {
    session.stage_text_artifact(
        &root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH),
        build_persisted_state_output(persisted),
    );
    for artifact in build_view_artifacts(root, persisted, state) {
        session.stage_text_artifact(&artifact.path, artifact.contents);
    }
}

pub fn build_engine_output(state: &CurrentSynthesisState, lens: EngineLens) -> String {
    let Some(last_tick) = state.last_tick() else {
        return String::from("# Current Synthesis Engine\n\nNo completed ticks.\n");
    };
    match lens {
        EngineLens::Status => format!(
            "# Current Synthesis Engine\n\n\
             ## Overview\n\n\
             - PLEB: forward human-Machine synthesis\n\
             - META: HAL supplies the complementary counter-position\n\
             - BLEP: reverse NPC synthesis under Cleopatra\n\
             - completed ticks: {}\n\
             - deterministic seed: {}\n\
             - scenario: `{}`\n\
             - focused npc: `{}`\n\
             - active npc count: {}\n\n\
             - planned player actions: {}\n\n\
             ## Kernel Boundary\n\n\
             - Hollow Grove still selects one Way into Link at kernel depth.\n\
             - Current Synthesis interprets the selected Bond as a Moment.\n\
             - Aura, Current, HAL, Clouseau, Cleopatra, and NPC meaning stay outside the kernel.\n\n\
             ## Live Relay\n\n\
             - player posture: {}\n\
             - Clouseau: {}\n\
             - HAL: {}\n\
             - shared packet: {}\n\
             - Cleopatra handoff: {}\n\n\
             ## Latest Cycle\n\n\
             - player need: {}\n\
             - player selected Bond: {}\n\
             - player Moment: {}\n\
             - npc selected Bond: {}\n\
             - npc Moment: {}\n\
             - Aura total: {}\n\
             - Current total: {}\n\n\
             ## World State\n\n\
             {}\
             ## Active Consequences\n\n\
             {}\n",
            state.completed_ticks(),
            state.seed(),
            state.scenario_id(),
            state.focused_npc_id(),
            state.npcs().len(),
            PlayerActionProfile::from_actions(state.planned_player_actions()).display_label(),
            extract_world_input_detail(
                &last_tick.blep_decision.world_inputs,
                "player action mode "
            )
            .unwrap_or("general-pressure"),
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "clouseau relay ")
                .unwrap_or("relay pending"),
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "hal relay ")
                .unwrap_or("relay pending"),
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "joint relay ")
                .unwrap_or("relay pending"),
            last_tick.blep_decision.resulting_action.as_str(),
            state.player_need(),
            last_tick.forward_pass.bond_result.selected_bond.id,
            last_tick.forward_pass.bond_result.resulting_moment,
            last_tick.blep_decision.selected_bond.id,
            last_tick.blep_decision.resulting_action,
            state.resources.aura_total,
            state.resources.current_total,
            state.world.summary_lines(),
            render_lines(&state.world.active_consequences())
        ),
        EngineLens::Pleb => format!(
            "# PLEB Forward Synthesis\n\n\
             ## Need\n\n\
             {}\n\n\
             ## META Complement\n\n\
             {}\n\n\
             ## Candidate Bonds\n\n\
             {}\n\n\
             ## Selected Bond\n\n\
             - id: {}\n\
             - link: {}\n\
             - Moment: {}\n",
            last_tick.forward_pass.human_need,
            last_tick.forward_pass.machine_complement,
            render_candidate_list(
                &last_tick.forward_pass.candidate_bonds,
                Some(&last_tick.forward_pass.bond_result.selected_bond.id)
            ),
            last_tick.forward_pass.bond_result.selected_bond.id,
            last_tick.forward_pass.bond_result.resulting_link,
            last_tick.forward_pass.bond_result.resulting_moment
        ),
        EngineLens::Meta => format!(
            "# META Complement View\n\n\
             ## HAL\n\n\
             HAL remains the META-side complement. It does not decide alone; it exposes counter-arms and viability pressure.\n\n\
             ## Live Counter-Position\n\n\
             - {}\n\
             - shared packet: {}\n\n\
             ## Available Ways\n\n\
             {}\n",
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "hal relay ")
                .unwrap_or("HAL relay pending"),
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "joint relay ")
                .unwrap_or("relay pending"),
            render_way_list(&last_tick.forward_pass.available_ways)
        ),
        EngineLens::Blep => format!(
            "# BLEP Reverse Synthesis\n\n\
             ## Cleopatra\n\n\
             Cleopatra schedules BLEP without scripting the outcome.\n\n\
             ## Relay Intake\n\n\
             - player posture: {}\n\
             - Clouseau: {}\n\
             - HAL: {}\n\n\
             ## Inferred Need\n\n\
             {}\n\n\
             ## Candidate Actions\n\n\
             {}\n\n\
             ## Selected NPC Bond\n\n\
             - id: {}\n\
             - action: {}\n\
             - confidence: {}\n",
            extract_world_input_detail(
                &last_tick.blep_decision.world_inputs,
                "player action mode "
            )
            .unwrap_or("general-pressure"),
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "clouseau relay ")
                .unwrap_or("relay pending"),
            extract_world_input_detail(&last_tick.blep_decision.world_inputs, "hal relay ")
                .unwrap_or("relay pending"),
            last_tick.blep_decision.inferred_need,
            render_candidate_list(
                &last_tick.blep_decision.candidate_bonds,
                Some(&last_tick.blep_decision.selected_bond.id)
            ),
            last_tick.blep_decision.selected_bond.id,
            last_tick.blep_decision.resulting_action,
            last_tick.blep_decision.confidence
        ),
    }
}

pub fn build_world_context_output() -> String {
    build_contract_world_context_output()
}

pub fn build_player_status_output(state: &CurrentSynthesisState) -> String {
    let action_profile = PlayerActionProfile::from_actions(state.planned_player_actions());
    let pending_actions = action_profile.render_pending_actions();
    let next_step = if state.planned_player_actions.is_empty() {
        "player move <action-text>"
    } else {
        "cleopatra tick"
    };
    let last_cycle = state
        .last_tick()
        .map(|tick| {
            format!(
                "- posture: {}\n- Clouseau: {}\n- HAL: {}\n- last selected bond: {}\n- last npc action: {}\n- last aura/current: {}/{}\n",
                extract_world_input_detail(&tick.blep_decision.world_inputs, "player action mode ")
                    .unwrap_or("general-pressure"),
                extract_world_input_detail(&tick.blep_decision.world_inputs, "clouseau relay ")
                    .unwrap_or("relay pending"),
                extract_world_input_detail(&tick.blep_decision.world_inputs, "hal relay ")
                    .unwrap_or("relay pending"),
                tick.forward_pass.bond_result.selected_bond.id,
                tick.blep_decision.resulting_action,
                state.resources.aura_total,
                state.resources.current_total
            )
        })
        .unwrap_or_else(|| String::from("- no completed cycle yet\n"));

    format!(
        "# Player Status\n\n\
         ## Snapshot\n\n\
         - scenario: `{}`\n\
         - focused npc: `{}`\n\
         - completed ticks: {}\n\
         - player need: {}\n\n\
         ## World State\n\n\
         {}\
         ## Active Consequences\n\n\
         {}\n\
         ## Pending Actions\n\n\
         {}\n\
         ## Recommended Next Step\n\n\
         - `{}`\n\n\
         ## Latest Cycle\n\n\
         {}",
        state.scenario_id(),
        state.focused_npc_id(),
        state.completed_ticks(),
        state.player_need(),
        state.world.summary_lines(),
        render_lines(&state.world.active_consequences()),
        pending_actions,
        next_step,
        last_cycle
    )
}

pub fn build_bond_list_output(state: &CurrentSynthesisState) -> String {
    let Some(last_tick) = state.last_tick() else {
        return String::from("# Bond List\n\nNo completed ticks.\n");
    };
    let mut lines = String::from("# Bond List\n\n");
    lines.push_str("## Player Candidates\n\n");
    lines.push_str(&render_candidate_list(
        &last_tick.forward_pass.candidate_bonds,
        Some(&last_tick.forward_pass.bond_result.selected_bond.id),
    ));
    lines.push_str("\n## NPC Candidates\n\n");
    lines.push_str(&render_candidate_list(
        &last_tick.blep_decision.candidate_bonds,
        Some(&last_tick.blep_decision.selected_bond.id),
    ));
    lines
}

pub fn build_bond_inspector_output(state: &CurrentSynthesisState, bond_id: &str) -> Option<String> {
    let tick = state.last_tick()?;
    let (candidate, selected, owner) = if let Some(candidate) = tick
        .forward_pass
        .candidate_bonds
        .iter()
        .find(|candidate| candidate.id == bond_id)
    {
        (
            candidate,
            candidate.id == tick.forward_pass.bond_result.selected_bond.id,
            "player",
        )
    } else if let Some(candidate) = tick
        .blep_decision
        .candidate_bonds
        .iter()
        .find(|candidate| candidate.id == bond_id)
    {
        (
            candidate,
            candidate.id == tick.blep_decision.selected_bond.id,
            "npc",
        )
    } else {
        return None;
    };

    let mut residue_summary = String::from("- selected: yes\n- residue destination: none\n");
    if !selected {
        residue_summary = format!(
            "- selected: no\n- residue destination: {}\n- resource family: {}\n",
            destination_for_side(candidate.side).as_str(),
            candidate.side.family_name()
        );
    }

    Some(format!(
        "# Bond Inspector\n\n\
         ## Bond\n\n\
         - id: {}\n\
         - owner: {}\n\
         - side: {}\n\
         - viability: {}\n\
         - cost: {}\n\
         - source need: {}\n\
         - participants: {}\n\
         - selected arms: {}\n\n\
         ## Properties\n\n\
         {}\n\
         ## Resolution\n\n\
         {}",
        candidate.id,
        owner,
        candidate.side.as_str(),
        candidate.viability,
        candidate.cost,
        candidate.source_need,
        candidate.participants.join(", "),
        candidate.selected_arms.join(", "),
        render_properties(&candidate.properties),
        residue_summary
    ))
}

pub fn build_bond_trace_output(state: &CurrentSynthesisState, moment_id: &str) -> Option<String> {
    let tick = state.history.iter().find(|record| {
        record.player_moment_id == moment_id
            || record.npc_moment_id == moment_id
            || record
                .coordinated_blep_relays
                .iter()
                .any(|relay| relay.npc_moment_id == moment_id)
    })?;
    if tick.player_moment_id == moment_id {
        return Some(format!(
            "# Bond Trace\n\n\
             ## Moment\n\n\
             - id: {}\n\
             - need: {}\n\
             - selected Bond: {}\n\
             - resulting Moment: {}\n\n\
             ## Unused Left Bonds\n\n\
             {}\n\
             ## Unused Right Bonds\n\n\
             {}\n",
            tick.player_moment_id,
            tick.forward_pass.human_need,
            tick.forward_pass.bond_result.selected_bond.id,
            tick.forward_pass.bond_result.resulting_moment,
            render_unused_bonds(
                &tick.forward_pass.bond_result.unused_bonds,
                SemanticSide::Left
            ),
            render_unused_bonds(
                &tick.forward_pass.bond_result.unused_bonds,
                SemanticSide::Right
            )
        ));
    }

    if let Some(relay) = tick
        .coordinated_blep_relays
        .iter()
        .find(|relay| relay.npc_moment_id == moment_id)
    {
        return Some(build_cleopatra_trace_output_from_relay(tick, relay));
    }

    Some(build_cleopatra_trace_output_from_tick(tick))
}

pub fn build_resource_aura_output(state: &CurrentSynthesisState) -> String {
    let sources = recent_residue_sources(state, SemanticSide::Left);
    format!(
        "# Aura Inspector\n\n\
         - total: {}\n\
         - recent left residue sources: {}\n\n\
         {}",
        state.resources.aura_total,
        if sources.is_empty() {
            String::from("none")
        } else {
            sources.join(", ")
        },
        render_properties(&state.resources.aura_properties)
    )
}

pub fn build_resource_current_output(state: &CurrentSynthesisState) -> String {
    let sources = recent_residue_sources(state, SemanticSide::Right);
    format!(
        "# Current Inspector\n\n\
         - total: {}\n\
         - recent right residue sources: {}\n\n\
         {}",
        state.resources.current_total,
        if sources.is_empty() {
            String::from("none")
        } else {
            sources.join(", ")
        },
        render_properties(&state.resources.current_properties)
    )
}

pub fn build_resource_residues_output(state: &CurrentSynthesisState) -> String {
    let mut output = String::from("# Residue Inspector\n\n");
    for tick in &state.history {
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!("## Tick {}\n\n", tick.tick_number),
        );
        for residue in tick.forward_residues.iter().chain(
            tick.coordinated_blep_relays
                .iter()
                .flat_map(|relay| relay.blep_residues.iter()),
        ) {
            let _ = fmt::Write::write_fmt(
                &mut output,
                format_args!(
                    "- {} -> {} -> {}\n",
                    residue.source_bond,
                    residue.destination.as_str(),
                    residue.side.family_name()
                ),
            );
        }
        output.push('\n');
    }
    output
}

pub fn build_resource_history_output(state: &CurrentSynthesisState) -> String {
    let mut output = String::from("# Resource History\n\n");
    for tick in &state.history {
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!(
                "- tick {}: Aura={} Current={} player={} npc={} coordinated={}\n",
                tick.tick_number,
                tick.resources_after_tick.aura_total,
                tick.resources_after_tick.current_total,
                tick.forward_pass.bond_result.selected_bond.id,
                tick.blep_decision.selected_bond.id,
                tick.coordinated_blep_relays.len()
            ),
        );
    }
    output.push_str("\n## Aura Composition\n\n");
    output.push_str(&render_properties(&state.resources.aura_properties));
    output.push_str("\n## Current Composition\n\n");
    output.push_str(&render_properties(&state.resources.current_properties));
    output.push_str("\n## World Pressure\n\n");
    output.push_str(&state.world.summary_lines());
    output.push_str("\n## Active Consequences\n\n");
    output.push_str(&render_lines(&state.world.active_consequences()));
    output
}

pub fn build_npc_list_output(state: &CurrentSynthesisState) -> String {
    let mut output = String::from("# NPC List\n\n");
    for npc in state.npcs() {
        let focus_marker = if npc.id == state.focused_npc_id() {
            " [focused]"
        } else {
            ""
        };
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!(
                "- {} (`{}`) role={} faction={} location={}{}\n",
                npc.name, npc.id, npc.role, npc.faction, npc.location, focus_marker
            ),
        );
    }
    output
}

pub fn build_npc_inspector_output(state: &CurrentSynthesisState, npc_id: &str) -> Option<String> {
    let npc = state.find_npc(npc_id)?;
    let relay = latest_relay_for_npc(state, npc_id);
    let inferred_need = relay
        .map(|(_tick, relay)| relay.blep_decision.inferred_need.as_str())
        .unwrap_or_else(|| {
            npc.needs
                .first()
                .map(String::as_str)
                .unwrap_or("unassigned")
        });
    let perceived_conditions = relay
        .map(|(_tick, relay)| render_lines(&relay.blep_decision.world_inputs))
        .unwrap_or_else(|| render_lines(&npc.perceived_world));
    let candidates = relay
        .map(|(_tick, relay)| {
            render_candidate_list(
                &relay.blep_decision.candidate_bonds,
                Some(&relay.blep_decision.selected_bond.id),
            )
        })
        .unwrap_or_else(|| String::from("- no completed BLEP pass for this npc yet\n"));
    let selected_action_bond = relay
        .map(|(_tick, relay)| relay.blep_decision.selected_bond.id.as_str())
        .unwrap_or("pending");
    let selected_action = relay
        .map(|(_tick, relay)| relay.blep_decision.resulting_action.as_str())
        .unwrap_or("pending");
    let residue_count = relay
        .map(|(_tick, relay)| relay.blep_residues.len())
        .unwrap_or(0);
    let relay_mode = relay
        .map(|(_tick, relay)| {
            if relay.committed {
                "committed"
            } else {
                "coordinated"
            }
        })
        .unwrap_or("pending");

    Some(format!(
        "# NPC Inspector\n\n\
         ## NPC\n\n\
         - id: {}\n\
         - name: {}\n\
         - role: {}\n\
         - faction: {}\n\
         - location: {}\n\
         - need: {}\n\
         - condition: {}\n\
         - relay mode: {}\n\n\
         ## Memory\n\n\
         {}\n\
         ## Perceived Conditions\n\n\
         {}\n\
         ## BLEP Candidates\n\n\
         {}\n\
         ## Selected Action Bond\n\n\
         - id: {}\n\
         - action: {}\n\
         - residues from rejected actions: {}\n",
        npc.id,
        npc.name,
        npc.role,
        npc.faction,
        npc.location,
        inferred_need,
        npc.condition,
        relay_mode,
        render_lines(&npc.memories),
        perceived_conditions,
        candidates,
        selected_action_bond,
        selected_action,
        residue_count
    ))
}

pub fn build_npc_history_output(state: &CurrentSynthesisState, npc_id: &str) -> Option<String> {
    if state.find_npc(npc_id).is_none() {
        return None;
    }
    let mut output = String::from("# NPC History\n\n");
    for tick in &state.history {
        if let Some(relay) = tick
            .coordinated_blep_relays
            .iter()
            .find(|relay| relay.npc_id == npc_id)
        {
            let _ = fmt::Write::write_fmt(
                &mut output,
                format_args!(
                    "- tick {}: {} -> {} [{}]\n",
                    tick.tick_number,
                    relay.blep_decision.selected_bond.id,
                    relay.blep_decision.resulting_action,
                    if relay.committed {
                        "committed"
                    } else {
                        "coordinated"
                    }
                ),
            );
        }
    }
    Some(output)
}

pub fn build_cleopatra_status_output(state: &CurrentSynthesisState) -> String {
    let tick = state.last_tick();
    format!(
        "# Cleopatra\n\n\
         ## Status\n\n\
         - active NPCs: {}\n\
         - queued BLEP passes: {}\n\
         - recent BLEP passes: {}\n\
         - coordinated relay count: {}\n\
         - live relay packet: {}\n\
         - Aura total: {}\n\
         - Current total: {}\n\
         - possible conflict escalation: {}\n\n\
         ## World State\n\n\
         {}\
         ## Active Consequences\n\n\
         {}\n\
         ## Faction Pressure\n\n\
         {}\n\
         ## Route Pressure\n\n\
         {}\n",
        state.cleopatra.active_npcs.join(", "),
        state.cleopatra.queued_blep_passes.join(", "),
        if state.cleopatra.recent_blep_passes.is_empty() {
            String::from("none")
        } else {
            state.cleopatra.recent_blep_passes.join(", ")
        },
        tick.map(|record| record.coordinated_blep_relays.len())
            .unwrap_or(0),
        tick.and_then(|record| {
            extract_world_input_detail(&record.blep_decision.world_inputs, "joint relay ")
        })
        .unwrap_or("relay pending"),
        state.resources.aura_total,
        state.resources.current_total,
        tick.map(|_record| {
            if state.resources.current_total > state.resources.aura_total {
                "rising Current pressure may trigger raids or forced excavations"
            } else {
                "Aura remains competitive enough to preserve alliance or coordination options"
            }
        })
        .unwrap_or("no completed cycle"),
        state.world.summary_lines(),
        render_lines(&state.world.active_consequences()),
        render_lines(&state.cleopatra.faction_conditions),
        render_lines(&state.cleopatra.settlement_conditions)
    )
}

pub fn build_cleopatra_trace_output(state: &CurrentSynthesisState, npc_id: &str) -> Option<String> {
    let (tick, relay) = latest_relay_for_npc(state, npc_id)?;
    Some(build_cleopatra_trace_output_from_relay(tick, relay))
}

fn build_cleopatra_trace_output_from_tick(tick: &TickRecord) -> String {
    let relay = tick
        .coordinated_blep_relays
        .iter()
        .find(|relay| relay.committed)
        .unwrap_or_else(|| {
            tick.coordinated_blep_relays
                .first()
                .expect("tick relay set should not be empty")
        });
    build_cleopatra_trace_output_from_relay(tick, relay)
}

fn build_cleopatra_trace_output_from_relay(
    _tick: &TickRecord,
    relay: &CoordinatedBlepRelay,
) -> String {
    format!(
        "# Cleopatra BLEP Trace\n\n\
         NPC: {}\n\
         Relay Mode: {}\n\
         Need: {}\n\
         Pressure: {}\n\
         Candidates:\n\
         {}\n\
         Selected Bond:\n\
         {}\n\
         Resulting Moment:\n\
         {}\n\
         Unused left Bonds:\n\
         {}\n\
         Unused right Bonds:\n\
         {}",
        relay.blep_decision.npc_id,
        if relay.committed {
            "committed"
        } else {
            "coordinated"
        },
        relay.blep_decision.inferred_need,
        relay.blep_decision.world_inputs.join(", "),
        render_numbered_candidates(&relay.blep_decision.candidate_bonds),
        relay.blep_decision.selected_bond.id,
        relay.blep_decision.resulting_action,
        render_unused_bonds(&relay.blep_decision.unused_bonds, SemanticSide::Left),
        render_unused_bonds(&relay.blep_decision.unused_bonds, SemanticSide::Right)
    )
}

fn latest_tick_for_npc<'a>(
    state: &'a CurrentSynthesisState,
    npc_id: &str,
) -> Option<&'a TickRecord> {
    state.history.iter().rev().find(|tick| {
        tick.coordinated_blep_relays
            .iter()
            .any(|relay| relay.npc_id == npc_id)
    })
}

fn latest_relay_for_npc<'a>(
    state: &'a CurrentSynthesisState,
    npc_id: &str,
) -> Option<(&'a TickRecord, &'a CoordinatedBlepRelay)> {
    let tick = latest_tick_for_npc(state, npc_id)?;
    let relay = tick
        .coordinated_blep_relays
        .iter()
        .find(|relay| relay.npc_id == npc_id)?;
    Some((tick, relay))
}

pub fn build_cleopatra_pressures_output(state: &CurrentSynthesisState) -> String {
    let mut combined = state.cleopatra.faction_conditions.clone();
    combined.extend(state.cleopatra.settlement_conditions.clone());
    combined.extend(state.cleopatra.war_conditions.clone());
    format!("# Cleopatra Pressures\n\n{}\n", render_lines(&combined))
}

fn build_forward_pass(
    seed: u64,
    tick_number: u32,
    player_need: &str,
    resources: &ResourceComposition,
    action_profile: &PlayerActionProfile,
    hueman_feedback: Option<HuemanFeedback>,
) -> ForwardSynthesisPass {
    let available_ways = vec![
        engine_way(
            "way/player/ridge-scan",
            "Clouseau",
            "survey the upper valley seam",
            SemanticSide::Left,
            &[("perception", 3), ("visibility", 2)],
            9,
        ),
        engine_way(
            "way/player/crosswind-step",
            "Clouseau",
            "push through unstable air and brush",
            SemanticSide::Left,
            &[("speed", 2), ("expression", 1)],
            7,
        ),
        engine_way(
            "way/player/anchor-stance",
            "Clouseau",
            "hold ground against collapse",
            SemanticSide::Right,
            &[("endurance", 3), ("structure", 2)],
            8,
        ),
        engine_way(
            "way/meta/overwatch",
            "HAL",
            "resolve line-of-sight and distance",
            SemanticSide::Left,
            &[("clarity", 3), ("reflection", 2)],
            10,
        ),
        engine_way(
            "way/meta/echo-lift",
            "HAL",
            "convert contour into lift and motion",
            SemanticSide::Left,
            &[("lift", 3), ("speed", 2)],
            8,
        ),
        engine_way(
            "way/meta/pressure-map",
            "HAL",
            "resolve pressure-bearing stone and void",
            SemanticSide::Right,
            &[("pressure", 3), ("structure", 3)],
            9,
        ),
    ];
    let machine_complement = action_profile.hal_machine_complement();
    let candidate_bonds = vec![
        bond_candidate(
            tick_number,
            "player",
            "flight",
            SemanticSide::Left,
            &[("lift", 4), ("visibility", 3), ("speed", 2)],
            67,
            29,
            player_need,
            &["Clouseau", "HAL"],
            &["way/player/ridge-scan", "way/meta/echo-lift"],
        ),
        bond_candidate(
            tick_number,
            "player",
            "enhanced-sight",
            SemanticSide::Left,
            &[("perception", 5), ("clarity", 4), ("reflection", 3)],
            78,
            22,
            player_need,
            &["Clouseau", "HAL"],
            &["way/player/ridge-scan", "way/meta/overwatch"],
        ),
        bond_candidate(
            tick_number,
            "player",
            "echo-hearing",
            SemanticSide::Left,
            &[("perception", 4), ("expression", 2), ("clarity", 2)],
            63,
            18,
            player_need,
            &["Clouseau", "HAL"],
            &["way/player/crosswind-step", "way/meta/overwatch"],
        ),
        bond_candidate(
            tick_number,
            "player",
            "camouflage",
            SemanticSide::Right,
            &[("concealment", 4), ("persistence", 3), ("structure", 1)],
            64,
            17,
            player_need,
            &["Clouseau", "HAL"],
            &["way/player/anchor-stance", "way/meta/pressure-map"],
        ),
        bond_candidate(
            tick_number,
            "player",
            "anchor-armor",
            SemanticSide::Right,
            &[("mass", 4), ("endurance", 4), ("physical_force", 2)],
            61,
            24,
            player_need,
            &["Clouseau", "HAL"],
            &["way/player/anchor-stance", "way/meta/pressure-map"],
        ),
        bond_candidate(
            tick_number,
            "player",
            "burrowing",
            SemanticSide::Right,
            &[("pressure", 4), ("structure", 4), ("momentum", 3)],
            58,
            21,
            player_need,
            &["Clouseau", "HAL"],
            &["way/player/anchor-stance", "way/meta/pressure-map"],
        ),
    ];
    let selected_index = select_forward_candidate(
        seed,
        tick_number,
        &candidate_bonds,
        resources,
        action_profile,
        hueman_feedback,
    );
    let selected_bond = candidate_bonds[selected_index].clone();
    let unused_bonds = candidate_bonds
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != selected_index)
        .map(|(_, candidate)| candidate.clone())
        .collect::<Vec<_>>();
    let resulting_link = format!(
        "{} ↔ {}",
        selected_bond.selected_arms[0], selected_bond.selected_arms[1]
    );
    let resulting_moment = if selected_bond.id.contains("enhanced-sight") {
        String::from(
            "The scout resolves the hostile valley sightline and exposes the safest moving seam before dusk.",
        )
    } else if selected_bond.id.contains("flight") {
        String::from(
            "The scout lifts above the valley break and maps the route from the air before dusk.",
        )
    } else if selected_bond.id.contains("burrowing") {
        String::from(
            "The scout reads the valley from beneath the surface pressure and discovers a buried route.",
        )
    } else {
        format!(
            "The scout commits to {} and changes the valley approach.",
            selected_bond.id
        )
    };
    ForwardSynthesisPass {
        human_need: player_need.to_owned(),
        machine_complement,
        available_ways,
        candidate_bonds,
        bond_result: BondResult {
            selected_bond,
            resulting_link,
            resulting_moment,
            unused_bonds,
        },
    }
}

fn build_blep_decision(
    seed: u64,
    tick_number: u32,
    npc: &NpcState,
    resources: &ResourceComposition,
    action_profile: &PlayerActionProfile,
    world_inputs: &[String],
    player_moment: &str,
    hueman_feedback: Option<HuemanFeedback>,
) -> BLEPDecision {
    let inferred_need = if player_moment.contains("sightline") {
        String::from("Restore access to the western road the scout just exposed")
    } else if player_moment.contains("air") {
        String::from("Secure the ridge route before the exposed high path collapses")
    } else {
        String::from("Convert the new route clue into a viable evacuation path")
    };
    let mut candidate_bonds = vec![
        bond_candidate(
            tick_number,
            "npc",
            "repair-bridge",
            SemanticSide::Right,
            &[("structure", 3), ("endurance", 3), ("mass", 2)],
            70,
            23,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["route bracing", "load redistribution"],
        ),
        bond_candidate(
            tick_number,
            "npc",
            "open-tunnel",
            SemanticSide::Right,
            &[("pressure", 4), ("structure", 4), ("momentum", 3)],
            73,
            26,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["subsurface seam", "labor redirect"],
        ),
        bond_candidate(
            tick_number,
            "npc",
            "raid-convoy",
            SemanticSide::Right,
            &[("fuel", 4), ("physical_force", 4), ("pressure", 3)],
            59,
            37,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["convoy strike", "forced extraction"],
        ),
        bond_candidate(
            tick_number,
            "npc",
            "request-alliance",
            SemanticSide::Left,
            &[("social_presence", 4), ("expression", 3), ("clarity", 2)],
            57,
            16,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["signal tower", "shared labor request"],
        ),
        bond_candidate(
            tick_number,
            "npc",
            "hide-and-wait",
            SemanticSide::Right,
            &[("concealment", 4), ("persistence", 3)],
            52,
            11,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["cover", "delay"],
        ),
    ];
    if hueman_feedback.is_some_and(|feedback| feedback.recognized_defense_branch) {
        candidate_bonds.push(bond_candidate(
            tick_number,
            "npc",
            "hold-shelterline",
            SemanticSide::Right,
            &[("structure", 3), ("endurance", 4), ("clarity", 2)],
            71,
            20,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["fence anchor", "shelterline hold"],
        ));
    }
    if hueman_feedback.is_some_and(|feedback| feedback.recognized_route_branch) {
        candidate_bonds.push(bond_candidate(
            tick_number,
            "npc",
            "survey-route-network",
            SemanticSide::Left,
            &[("clarity", 4), ("pressure", 2), ("social_presence", 2)],
            69,
            18,
            &inferred_need,
            &[&npc.name, "Cleopatra"],
            &["route audit", "load witness"],
        ));
    }
    let selected_index = select_blep_candidate(
        seed,
        tick_number,
        &candidate_bonds,
        resources,
        action_profile,
        world_inputs,
        npc,
        hueman_feedback,
    );
    let selected_bond = candidate_bonds[selected_index].clone();
    let unused_bonds = candidate_bonds
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != selected_index)
        .map(|(_, candidate)| candidate.clone())
        .collect::<Vec<_>>();
    let resulting_action = if selected_bond.id.contains("open-tunnel") {
        format!(
            "{} redirects labor into a new underground route and stabilizes a tunnel under the western road.",
            npc.name
        )
    } else if selected_bond.id.contains("request-alliance") {
        format!(
            "{} signals for outside labor and holds the road long enough for help to arrive.",
            npc.name
        )
    } else if selected_bond.id.contains("hold-shelterline") {
        format!(
            "{} hardens the shelter edge into a lit holding line and keeps intake open under pressure.",
            npc.name
        )
    } else if selected_bond.id.contains("survey-route-network") {
        format!(
            "{} turns the route trust into a live survey packet and reassigns labor toward the true bearing path.",
            npc.name
        )
    } else {
        format!(
            "{} commits to {} under Cleopatra's field coordination.",
            npc.name, selected_bond.id
        )
    };
    let confidence = (selected_bond.viability
        + supporting_resource_for_side(resources, selected_bond.side))
    .min(100);
    BLEPDecision {
        npc_id: npc.id.clone(),
        world_inputs: world_inputs.to_vec(),
        inferred_need,
        candidate_bonds,
        selected_bond,
        resulting_action,
        unused_bonds,
        confidence,
    }
}

fn build_world_inputs(
    state: &CurrentSynthesisState,
    forward_pass: &ForwardSynthesisPass,
    npc: &NpcState,
    tick_number: u32,
    action_profile: &PlayerActionProfile,
    hueman_feedback: Option<HuemanFeedback>,
) -> Vec<String> {
    let mut inputs = vec![
        String::from("fuel shortage"),
        String::from("road blockade"),
        String::from("injured workers need passage"),
        format!("focused npc {} at {}", npc.name, npc.location),
        format!("npc condition {}", npc.condition),
        format!("planned player actions {}", action_profile.display_label()),
        format!("Aura total now {}", state.resources.aura_total),
        format!("Current total now {}", state.resources.current_total),
        format!(
            "player moment {}",
            forward_pass.bond_result.resulting_moment.to_lowercase()
        ),
        format!(
            "clouseau relay {}",
            action_profile.clouseau_relay(&forward_pass.bond_result.selected_bond)
        ),
        format!(
            "hal relay {}",
            action_profile.hal_relay(&forward_pass.bond_result.selected_bond)
        ),
        format!(
            "joint relay {}",
            action_profile.joint_relay(
                &forward_pass.bond_result.selected_bond,
                &forward_pass.bond_result.resulting_moment
            )
        ),
        format!("world route stability {}", state.world.route_stability),
        format!("world shelter integrity {}", state.world.shelter_integrity),
        format!("world power stability {}", state.world.power_stability),
        format!(
            "world labor availability {}",
            state.world.labor_availability
        ),
        format!("world faction tension {}", state.world.faction_tension),
        format!("world conflict risk {}", state.world.conflict_risk),
    ];
    inputs.extend(
        state
            .world
            .active_consequences()
            .into_iter()
            .map(|line| format!("world consequence {line}")),
    );
    inputs.extend(action_profile.world_inputs());
    if tick_number > 1 {
        inputs.push(String::from(
            "previous residues are altering route viability",
        ));
    }
    if let Some(feedback) = hueman_feedback {
        if feedback.unlocked {
            inputs.push(String::from("hueman gremlin unlock is active"));
        }
        if feedback.recognized_route_branch {
            inputs.push(String::from("hueman route hinge trust is active"));
        }
        if feedback.recognized_defense_branch {
            inputs.push(String::from("hueman flockline trust is active"));
        }
    }
    inputs
}

fn apply_world_consequences(
    world: &mut WorldState,
    action_profile: &PlayerActionProfile,
    forward_pass: &ForwardSynthesisPass,
    coordinated_blep_relays: &[CoordinatedBlepRelay],
    resources: &ResourceComposition,
) {
    for directive in &action_profile.directives {
        match &directive.schema {
            PlayerActionSchema::Move(spec) => {
                adjust_world_value(&mut world.route_stability, 4);
                adjust_world_value(&mut world.labor_availability, -1);
                match spec.line {
                    RouteLineName::AuraRidge
                    | RouteLineName::AuraWay
                    | RouteLineName::BasinMotorspeedway
                    | RouteLineName::Boardwalk
                    | RouteLineName::Glausbahn => adjust_world_value(&mut world.route_stability, 3),
                    RouteLineName::QuarryRim | RouteLineName::WesternRoad => {
                        adjust_world_value(&mut world.route_stability, 4);
                        adjust_world_value(&mut world.power_stability, -1);
                    }
                    RouteLineName::IntakeLine => {
                        adjust_world_value(&mut world.power_stability, 2);
                    }
                    RouteLineName::Shelterline => {
                        adjust_world_value(&mut world.shelter_integrity, 2);
                    }
                    RouteLineName::StairwayToHeaven
                    | RouteLineName::Riptide
                    | RouteLineName::CurrentSea
                    | RouteLineName::MountAura
                    | RouteLineName::General => {}
                }
                match spec.method {
                    MoveMethod::Scout => {
                        adjust_world_value(&mut world.route_stability, 2);
                        adjust_world_value(&mut world.conflict_risk, -1);
                    }
                    MoveMethod::Tunnel => {
                        adjust_world_value(&mut world.route_stability, 3);
                        adjust_world_value(&mut world.labor_availability, -2);
                    }
                    MoveMethod::Carry => {
                        adjust_world_value(&mut world.shelter_integrity, 1);
                        adjust_world_value(&mut world.labor_availability, -1);
                    }
                    MoveMethod::Flank => adjust_world_value(&mut world.conflict_risk, -1),
                    MoveMethod::Traverse => {}
                }
                match spec.pace {
                    MovePace::Fast => {
                        adjust_world_value(&mut world.power_stability, -1);
                        adjust_world_value(&mut world.conflict_risk, 1);
                    }
                    MovePace::Careful => adjust_world_value(&mut world.conflict_risk, -1),
                    MovePace::Balanced => {}
                }
                match spec.stance {
                    MoveStance::Quiet => adjust_world_value(&mut world.conflict_risk, -1),
                    MoveStance::Forceful => adjust_world_value(&mut world.faction_tension, 2),
                    MoveStance::Steady => {}
                }
            }
            PlayerActionSchema::Decide(spec) => {
                let context = resolve_decide_context(directive, spec);
                adjust_world_value(&mut world.faction_tension, -2);
                adjust_world_value(&mut world.conflict_risk, -1);
                match spec.focus {
                    DecideFocus::Alliance => {
                        adjust_world_value(&mut world.faction_tension, -4);
                        adjust_world_value(&mut world.conflict_risk, -2);
                    }
                    DecideFocus::Power => adjust_world_value(&mut world.power_stability, 2),
                    DecideFocus::Shelter => adjust_world_value(&mut world.shelter_integrity, 2),
                    DecideFocus::Route => adjust_world_value(&mut world.route_stability, 2),
                    DecideFocus::Labor => adjust_world_value(&mut world.labor_availability, 2),
                    DecideFocus::Conflict => {
                        adjust_world_value(&mut world.faction_tension, 3);
                        adjust_world_value(&mut world.conflict_risk, 4);
                    }
                    DecideFocus::General => {}
                }
                match spec.commitment {
                    DecideCommitment::Hold => adjust_world_value(&mut world.shelter_integrity, 1),
                    DecideCommitment::Shift => adjust_world_value(&mut world.route_stability, 1),
                    DecideCommitment::Withdraw => adjust_world_value(&mut world.conflict_risk, -1),
                    DecideCommitment::Commit => {}
                }
                match spec.authority {
                    DecideAuthority::Shared => adjust_world_value(&mut world.faction_tension, -2),
                    DecideAuthority::Stonebend
                    | DecideAuthority::Glaushouse
                    | DecideAuthority::Sandmanor => {
                        adjust_world_value(&mut world.faction_tension, -1)
                    }
                    DecideAuthority::Solo => {}
                }
                if spec.signal == DecideSignal::Emergency {
                    adjust_world_value(&mut world.conflict_risk, 1);
                }
                if context.domain_slug == "route-stability" || context.route.is_some() {
                    adjust_world_value(&mut world.route_stability, 1);
                }
                if context.site_slug.contains("aura-ridge") {
                    adjust_world_value(&mut world.route_stability, 1);
                }
                if spec.authority == DecideAuthority::Shared {
                    adjust_world_value(&mut world.faction_tension, -1);
                }
                if spec.signal == DecideSignal::Public {
                    adjust_world_value(&mut world.faction_tension, -1);
                }
            }
            PlayerActionSchema::Support(spec) => {
                let context = resolve_support_context(directive, spec);
                adjust_world_value(&mut world.shelter_integrity, 3);
                adjust_world_value(&mut world.power_stability, 1);
                adjust_world_value(&mut world.labor_availability, -1);
                match spec.asset {
                    SupportAsset::Pump | SupportAsset::Power => {
                        adjust_world_value(&mut world.power_stability, 4);
                    }
                    SupportAsset::Crane | SupportAsset::Bridge | SupportAsset::Route => {
                        adjust_world_value(&mut world.route_stability, 3);
                    }
                    SupportAsset::Shelter | SupportAsset::Intake => {
                        adjust_world_value(&mut world.shelter_integrity, 4);
                    }
                    SupportAsset::Crew => {
                        adjust_world_value(&mut world.labor_availability, 2);
                    }
                    SupportAsset::General => {}
                }
                match spec.front {
                    SupportFront::Route => adjust_world_value(&mut world.route_stability, 2),
                    SupportFront::Shelter => adjust_world_value(&mut world.shelter_integrity, 2),
                    SupportFront::Power => adjust_world_value(&mut world.power_stability, 2),
                    SupportFront::Labor => adjust_world_value(&mut world.labor_availability, 2),
                    SupportFront::General => {}
                }
                match spec.intensity {
                    SupportIntensity::Heavy => {
                        adjust_world_value(&mut world.labor_availability, -1)
                    }
                    SupportIntensity::Light => adjust_world_value(&mut world.conflict_risk, -1),
                    SupportIntensity::Balanced => {}
                }
                if spec.duration == SupportDuration::Extended {
                    adjust_world_value(&mut world.power_stability, 1);
                }
                if context.route.is_some() || context.site_slug.contains("aura-ridge") {
                    adjust_world_value(&mut world.route_stability, 2);
                }
                if spec.asset == SupportAsset::Bridge || spec.asset == SupportAsset::Route {
                    adjust_world_value(&mut world.route_stability, 1);
                }
                if spec.front == SupportFront::Route {
                    adjust_world_value(&mut world.route_stability, 1);
                }
            }
            PlayerActionSchema::General => {
                if directive_has_keyword(directive, &["route", "road", "scout"]) {
                    adjust_world_value(&mut world.route_stability, 1);
                }
                if directive_has_keyword(directive, &["brace", "stabilize", "repair"]) {
                    adjust_world_value(&mut world.shelter_integrity, 1);
                }
            }
        }
    }

    match forward_pass.bond_result.selected_bond.id.as_str() {
        id if id.contains("enhanced-sight") => {
            adjust_world_value(&mut world.route_stability, 4);
            adjust_world_value(&mut world.conflict_risk, -1);
        }
        id if id.contains("flight") => {
            adjust_world_value(&mut world.route_stability, 5);
            adjust_world_value(&mut world.power_stability, -1);
        }
        id if id.contains("burrowing") => {
            adjust_world_value(&mut world.route_stability, 3);
            adjust_world_value(&mut world.labor_availability, -2);
        }
        id if id.contains("anchor-armor") => {
            adjust_world_value(&mut world.shelter_integrity, 3);
        }
        id if id.contains("camouflage") => {
            adjust_world_value(&mut world.conflict_risk, -2);
        }
        id if id.contains("echo-hearing") => {
            adjust_world_value(&mut world.faction_tension, -1);
        }
        _ => {}
    }

    for relay in coordinated_blep_relays {
        let bond_id = relay.blep_decision.selected_bond.id.as_str();
        if bond_id.contains("open-tunnel") {
            adjust_world_value(&mut world.route_stability, 5);
            adjust_world_value(&mut world.labor_availability, -2);
            adjust_world_value(&mut world.power_stability, -1);
        } else if bond_id.contains("repair-bridge") {
            adjust_world_value(&mut world.route_stability, 4);
            adjust_world_value(&mut world.shelter_integrity, 2);
            adjust_world_value(&mut world.labor_availability, -2);
        } else if bond_id.contains("request-alliance") {
            adjust_world_value(&mut world.labor_availability, 3);
            adjust_world_value(&mut world.faction_tension, -4);
            adjust_world_value(&mut world.conflict_risk, -3);
        } else if bond_id.contains("hold-shelterline") {
            adjust_world_value(&mut world.shelter_integrity, 5);
            adjust_world_value(&mut world.conflict_risk, -2);
        } else if bond_id.contains("raid-convoy") {
            adjust_world_value(&mut world.power_stability, 4);
            adjust_world_value(&mut world.faction_tension, 5);
            adjust_world_value(&mut world.conflict_risk, 6);
            adjust_world_value(&mut world.labor_availability, -1);
        } else if bond_id.contains("survey-route-network") {
            adjust_world_value(&mut world.route_stability, 4);
            adjust_world_value(&mut world.conflict_risk, -2);
        } else if bond_id.contains("hide-and-wait") {
            adjust_world_value(&mut world.conflict_risk, -1);
            adjust_world_value(&mut world.route_stability, -1);
        }
    }

    if resources.current_total > resources.aura_total.saturating_add(20) {
        adjust_world_value(&mut world.conflict_risk, 2);
    }
    if resources.aura_total > resources.current_total.saturating_add(20) {
        adjust_world_value(&mut world.faction_tension, -2);
    }
    if action_profile.has_context_keyword(&["power", "grid", "pump", "generator"]) {
        adjust_world_value(&mut world.power_stability, 2);
    }
    if action_profile.has_context_keyword(&["shelter", "intake", "hold", "brace"]) {
        adjust_world_value(&mut world.shelter_integrity, 2);
    }
    if action_profile.has_context_keyword(&["route", "road", "rim", "ridge"]) {
        adjust_world_value(&mut world.route_stability, 2);
    }
}

fn residues_from_unused_bonds(unused_bonds: &[BondCandidate]) -> Vec<Residue> {
    unused_bonds
        .iter()
        .map(|bond| Residue {
            source_bond: bond.id.clone(),
            side: bond.side,
            properties: bond.properties.clone(),
            destination: destination_for_side(bond.side),
        })
        .collect()
}

fn select_forward_candidate(
    seed: u64,
    tick_number: u32,
    candidates: &[BondCandidate],
    resources: &ResourceComposition,
    action_profile: &PlayerActionProfile,
    hueman_feedback: Option<HuemanFeedback>,
) -> usize {
    let mut best_index = 0usize;
    let mut best_score = i32::MIN;
    for (index, candidate) in candidates.iter().enumerate() {
        let left_bias = match candidate.id.as_str() {
            id if id.contains("enhanced-sight") => 120,
            id if id.contains("flight") => 95,
            id if id.contains("burrowing") => 60,
            _ => 0,
        };
        let hueman_bias = match hueman_feedback {
            Some(feedback)
                if feedback.recognized_route_branch && candidate.id.contains("burrowing") =>
            {
                55
            }
            Some(feedback)
                if feedback.recognized_route_branch && candidate.id.contains("enhanced-sight") =>
            {
                40
            }
            Some(feedback)
                if feedback.recognized_defense_branch && candidate.id.contains("anchor-armor") =>
            {
                55
            }
            Some(feedback)
                if feedback.recognized_defense_branch && candidate.id.contains("camouflage") =>
            {
                28
            }
            Some(feedback) if feedback.unlocked && candidate.side == SemanticSide::Right => 18,
            _ => 0,
        };
        let resource_bias = supporting_resource_for_side(resources, candidate.side) as i32 * 3;
        let action_bias = action_profile.forward_bias(&candidate.id);
        let score = candidate.viability as i32 * 10 - candidate.cost as i32
            + left_bias
            + hueman_bias
            + resource_bias
            + action_bias
            + stable_tiebreak(seed, tick_number, &candidate.id);
        if score > best_score {
            best_score = score;
            best_index = index;
        }
    }
    best_index
}

fn select_blep_candidate(
    seed: u64,
    tick_number: u32,
    candidates: &[BondCandidate],
    resources: &ResourceComposition,
    action_profile: &PlayerActionProfile,
    world_inputs: &[String],
    npc: &NpcState,
    hueman_feedback: Option<HuemanFeedback>,
) -> usize {
    let world_pressure = world_inputs
        .iter()
        .filter(|line| line.contains("road") || line.contains("injured") || line.contains("fuel"))
        .count() as i32
        * 9;
    let route_fragile = world_inputs
        .iter()
        .any(|line| line.contains("route stability is brittle"));
    let shelter_fragile = world_inputs
        .iter()
        .any(|line| line.contains("shelter integrity is close to failing"));
    let power_fragile = world_inputs
        .iter()
        .any(|line| line.contains("power stability is near blackout"));
    let tension_high = world_inputs
        .iter()
        .any(|line| line.contains("faction tension is degrading negotiation"));
    let alliance_window = world_inputs
        .iter()
        .any(|line| line.contains("alliance channels open"));
    let conflict_high = world_inputs
        .iter()
        .any(|line| line.contains("conflict risk is nearing release"));
    let mut best_index = 0usize;
    let mut best_score = i32::MIN;
    for (index, candidate) in candidates.iter().enumerate() {
        let contextual_bias = match candidate.id.as_str() {
            id if id.contains("open-tunnel") => 75,
            id if id.contains("repair-bridge") => 50,
            id if id.contains("request-alliance") => 30,
            id if id.contains("raid-convoy") => -20,
            _ => 0,
        };
        let role_bias = if npc.role.contains("triage") {
            match candidate.id.as_str() {
                id if id.contains("hold-shelterline") => 60,
                id if id.contains("request-alliance") => 25,
                _ => 0,
            }
        } else {
            0
        };
        let world_bias = match candidate.id.as_str() {
            id if id.contains("open-tunnel") => {
                (if route_fragile { 45 } else { 0 }) + if power_fragile { 12 } else { 0 }
            }
            id if id.contains("repair-bridge") => {
                (if route_fragile { 32 } else { 0 }) + if shelter_fragile { 18 } else { 0 }
            }
            id if id.contains("hold-shelterline") => {
                (if shelter_fragile { 50 } else { 0 }) + if conflict_high { 14 } else { 0 }
            }
            id if id.contains("request-alliance") => {
                (if tension_high { -10 } else { 0 }) + if alliance_window { 28 } else { 0 }
            }
            id if id.contains("raid-convoy") => {
                (if conflict_high { 18 } else { -10 }) + if tension_high { 12 } else { 0 }
            }
            id if id.contains("survey-route-network") => {
                (if route_fragile { 30 } else { 0 }) + if alliance_window { 10 } else { 0 }
            }
            _ => 0,
        };
        let hueman_bias = match hueman_feedback {
            Some(feedback)
                if feedback.recognized_route_branch && candidate.id.contains("open-tunnel") =>
            {
                45
            }
            Some(feedback)
                if feedback.recognized_route_branch
                    && candidate.id.contains("survey-route-network") =>
            {
                60
            }
            Some(feedback)
                if feedback.recognized_route_branch && candidate.id.contains("repair-bridge") =>
            {
                20
            }
            Some(feedback)
                if feedback.recognized_defense_branch
                    && candidate.id.contains("hold-shelterline") =>
            {
                70
            }
            Some(feedback)
                if feedback.recognized_defense_branch && candidate.id.contains("repair-bridge") =>
            {
                25
            }
            Some(feedback)
                if feedback.recognized_defense_branch && candidate.id.contains("hide-and-wait") =>
            {
                10
            }
            _ => 0,
        };
        let orchestration_bias = action_profile.blep_bias(&candidate.id);
        let score = candidate.viability as i32 * 10 - candidate.cost as i32
            + world_pressure
            + contextual_bias
            + role_bias
            + world_bias
            + hueman_bias
            + orchestration_bias
            + supporting_resource_for_side(resources, candidate.side) as i32 * 4
            + stable_tiebreak(seed ^ 0xC1E0, tick_number, &candidate.id);
        if score > best_score {
            best_score = score;
            best_index = index;
        }
    }
    best_index
}

fn supporting_resource_for_side(resources: &ResourceComposition, side: SemanticSide) -> u16 {
    match side {
        SemanticSide::Left => resource_support(&resources.aura_properties),
        SemanticSide::Right => resource_support(&resources.current_properties),
    }
}

fn resource_support(properties: &PropertyMap) -> u16 {
    properties.values().copied().sum::<u16>() / 4
}

fn stable_tiebreak(seed: u64, tick_number: u32, candidate_id: &str) -> i32 {
    let mut hash = 1469598103934665603u64 ^ seed ^ tick_number as u64;
    for byte in candidate_id.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    (hash % 11) as i32
}

fn engine_way(
    id: &str,
    source: &str,
    direction: &str,
    side: SemanticSide,
    properties: &[(&str, u16)],
    availability: u16,
) -> EngineWay {
    EngineWay {
        id: id.to_owned(),
        source: source.to_owned(),
        direction: direction.to_owned(),
        side,
        properties: property_map(properties),
        availability,
    }
}

fn bond_candidate(
    tick_number: u32,
    owner: &str,
    slug: &str,
    side: SemanticSide,
    properties: &[(&str, u16)],
    viability: u16,
    cost: u16,
    source_need: &str,
    participants: &[&str],
    selected_arms: &[&str],
) -> BondCandidate {
    BondCandidate {
        id: format!("bond/{owner}/{slug}/{tick_number}"),
        participants: participants
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        selected_arms: selected_arms
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        side,
        properties: property_map(properties),
        viability,
        cost,
        source_need: source_need.to_owned(),
    }
}

fn property_map(entries: &[(&str, u16)]) -> PropertyMap {
    let mut map = PropertyMap::new();
    for (name, value) in entries {
        map.insert((*name).to_owned(), *value);
    }
    map
}

fn property_total(properties: &PropertyMap) -> u16 {
    properties.values().copied().sum()
}

fn add_properties(target: &mut PropertyMap, source: &PropertyMap) {
    for (name, value) in source {
        let entry = target.entry(name.clone()).or_insert(0);
        *entry = entry.saturating_add(*value);
    }
}

fn destination_for_side(side: SemanticSide) -> ResidueDestination {
    match side {
        SemanticSide::Left => ResidueDestination::Aether,
        SemanticSide::Right => ResidueDestination::Bathos,
    }
}

fn recent_residue_sources(state: &CurrentSynthesisState, side: SemanticSide) -> Vec<String> {
    state
        .last_tick()
        .map(|tick| {
            tick.forward_residues
                .iter()
                .chain(tick.blep_residues.iter())
                .filter(|residue| residue.side == side)
                .map(|residue| residue.source_bond.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn infer_player_action_kind(detail: &str) -> PlayerActionKind {
    let lower = detail.to_ascii_lowercase();
    if [
        "move", "cross", "advance", "retreat", "climb", "descend", "route", "flank", "carry",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        PlayerActionKind::Move
    } else if [
        "decide",
        "choose",
        "signal",
        "assess",
        "survey",
        "judge",
        "commit",
        "recognize",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        PlayerActionKind::Decide
    } else if [
        "brace",
        "hold",
        "stabilize",
        "repair",
        "defend",
        "anchor",
        "support",
        "shield",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        PlayerActionKind::Support
    } else {
        PlayerActionKind::General
    }
}

fn decode_player_action(action: &str) -> PlayerActionDirective {
    if let Some(detail) = action.strip_prefix(PLAYER_ACTION_MOVE_PREFIX) {
        return decode_player_action_detail(PlayerActionKind::Move, detail);
    }
    if let Some(detail) = action.strip_prefix(PLAYER_ACTION_DECIDE_PREFIX) {
        return decode_player_action_detail(PlayerActionKind::Decide, detail);
    }
    if let Some(detail) = action.strip_prefix(PLAYER_ACTION_SUPPORT_PREFIX) {
        return decode_player_action_detail(PlayerActionKind::Support, detail);
    }
    decode_player_action_detail(infer_player_action_kind(action), action)
}

fn render_player_action_directive(directive: &PlayerActionDirective) -> String {
    let mut parts = Vec::new();
    if !directive.detail.is_empty() {
        parts.push(directive.detail.clone());
    }
    match &directive.schema {
        PlayerActionSchema::Move(spec) => {
            if let Some(from) = spec.from.as_deref() {
                parts.push(format!("from={from}"));
            }
            if let Some(to) = spec.to.as_deref() {
                parts.push(format!("to={to}"));
            }
            parts.push(format!("line={}", spec.line.as_str()));
            parts.push(format!("pace={}", spec.pace.as_str()));
            parts.push(format!("method={}", spec.method.as_str()));
            parts.push(format!("stance={}", spec.stance.as_str()));
        }
        PlayerActionSchema::Decide(spec) => {
            parts.push(format!("focus={}", spec.focus.as_str()));
            parts.push(format!("commitment={}", spec.commitment.as_str()));
            parts.push(format!("authority={}", spec.authority.as_str()));
            parts.push(format!("signal={}", spec.signal.as_str()));
        }
        PlayerActionSchema::Support(spec) => {
            parts.push(format!("asset={}", spec.asset.as_str()));
            if let Some(beneficiary) = spec.beneficiary.as_deref() {
                parts.push(format!("beneficiary={beneficiary}"));
            }
            parts.push(format!("front={}", spec.front.as_str()));
            parts.push(format!("intensity={}", spec.intensity.as_str()));
            parts.push(format!("duration={}", spec.duration.as_str()));
        }
        PlayerActionSchema::General => {
            if let Some(target) = directive.target.as_deref() {
                parts.push(format!("target={target}"));
            }
            for (key, value) in &directive.traits {
                parts.push(format!("{key}={value}"));
            }
        }
    }
    if parts.is_empty() {
        format!("[{}] configured", directive.kind.as_str())
    } else {
        format!("[{}] {}", directive.kind.as_str(), parts.join(" "))
    }
}

fn decode_player_action_detail(kind: PlayerActionKind, detail: &str) -> PlayerActionDirective {
    let trimmed = detail.trim();
    let mut free_text = Vec::new();
    let mut raw_target = None;
    let mut traits = BTreeMap::new();

    for token in trimmed.split_whitespace() {
        if let Some((raw_key, raw_value)) = token.split_once('=') {
            let key = raw_key.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if key.is_empty() || value.is_empty() {
                free_text.push(token.to_owned());
                continue;
            }
            if key == "target" {
                raw_target = Some(value.to_owned());
            } else {
                traits.insert(key, value.to_owned());
            }
        } else {
            free_text.push(token.to_owned());
        }
    }

    let detail = if free_text.is_empty() {
        if let Some(target) = raw_target.as_deref() {
            format!("toward {target}")
        } else if traits.is_empty() {
            trimmed.to_owned()
        } else {
            traits
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>()
                .join(" ")
        }
    } else {
        free_text.join(" ")
    };

    let schema = match kind {
        PlayerActionKind::Move => PlayerActionSchema::Move(build_move_action_spec(
            &detail,
            raw_target.as_deref(),
            &traits,
        )),
        PlayerActionKind::Decide => PlayerActionSchema::Decide(build_decide_action_spec(
            &detail,
            raw_target.as_deref(),
            &traits,
        )),
        PlayerActionKind::Support => PlayerActionSchema::Support(build_support_action_spec(
            &detail,
            raw_target.as_deref(),
            &traits,
        )),
        PlayerActionKind::General => PlayerActionSchema::General,
    };
    let target = match &schema {
        PlayerActionSchema::Move(spec) => spec.to.clone().or(raw_target.clone()),
        PlayerActionSchema::Decide(spec) => raw_target
            .clone()
            .or_else(|| Some(spec.focus.as_str().to_owned())),
        PlayerActionSchema::Support(spec) => spec.beneficiary.clone().or(raw_target.clone()),
        PlayerActionSchema::General => raw_target.clone(),
    };

    PlayerActionDirective {
        kind,
        detail,
        target,
        traits,
        schema,
    }
}

fn build_move_action_spec(
    detail: &str,
    raw_target: Option<&str>,
    traits: &BTreeMap<String, String>,
) -> MoveActionSpec {
    let from = trait_value(traits, &["from", "origin", "start"]).map(str::to_owned);
    let to = trait_value(traits, &["to", "destination"])
        .or(raw_target)
        .map(str::to_owned);
    let line = infer_route_line(detail, raw_target, traits, from.as_deref(), to.as_deref());
    let lane = infer_route_lane(detail, traits, from.as_deref(), to.as_deref(), line);
    let pace = infer_move_pace(detail, traits);
    let method = infer_move_method(detail, traits);
    let stance = infer_move_stance(detail, traits);
    MoveActionSpec {
        from,
        to,
        lane,
        line,
        pace,
        method,
        stance,
    }
}

fn build_decide_action_spec(
    detail: &str,
    raw_target: Option<&str>,
    traits: &BTreeMap<String, String>,
) -> DecideActionSpec {
    DecideActionSpec {
        focus: infer_decide_focus(detail, raw_target, traits),
        commitment: infer_decide_commitment(detail, traits),
        authority: infer_decide_authority(detail, traits),
        signal: infer_decide_signal(detail, traits),
    }
}

fn build_support_action_spec(
    detail: &str,
    raw_target: Option<&str>,
    traits: &BTreeMap<String, String>,
) -> SupportActionSpec {
    SupportActionSpec {
        asset: infer_support_asset(detail, traits),
        beneficiary: trait_value(traits, &["beneficiary", "target"])
            .or(raw_target)
            .map(str::to_owned),
        front: infer_support_front(detail, raw_target, traits),
        intensity: infer_support_intensity(detail, traits),
        duration: infer_support_duration(detail, traits),
    }
}

fn trait_value<'a>(traits: &'a BTreeMap<String, String>, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| traits.get(*key).map(String::as_str))
}

fn infer_route_line(
    detail: &str,
    raw_target: Option<&str>,
    traits: &BTreeMap<String, String>,
    from: Option<&str>,
    to: Option<&str>,
) -> RouteLineName {
    if let Some(line) = trait_value(traits, &["line", "route"]) {
        if let Some(named) = parse_route_line_name(line) {
            return named;
        }
    }
    let context = format!(
        "{} {} {} {}",
        detail,
        raw_target.unwrap_or(""),
        from.unwrap_or(""),
        to.unwrap_or("")
    )
    .to_ascii_lowercase();
    let normalized = context.replace(['-', '_'], " ");
    let curved_hint = normalized.contains("curve")
        || normalized.contains("curved")
        || normalized.contains("ring")
        || matches!(trait_value(traits, &["shape", "lane"]), Some(value) if value.eq_ignore_ascii_case("curve") || value.eq_ignore_ascii_case("curved") || value.eq_ignore_ascii_case("ring"));
    let straight_hint = normalized.contains("straight")
        || normalized.contains("direct")
        || normalized.contains("ridge")
        || normalized.contains("boardwalk")
        || normalized.contains("bahn")
        || matches!(trait_value(traits, &["shape", "lane"]), Some(value) if value.eq_ignore_ascii_case("straight") || value.eq_ignore_ascii_case("direct"));
    if normalized.contains("stairway") {
        RouteLineName::StairwayToHeaven
    } else if normalized.contains("riptide") {
        RouteLineName::Riptide
    } else if normalized.contains("current seanad") || normalized.contains("current sea") {
        RouteLineName::CurrentSea
    } else if normalized.contains("boardwalk") {
        RouteLineName::Boardwalk
    } else if normalized.contains("glausbahn") {
        RouteLineName::Glausbahn
    } else if normalized.contains("aura ridge") {
        RouteLineName::AuraRidge
    } else if normalized.contains("mnt aura") || normalized.contains("mount aura") {
        RouteLineName::MountAura
    } else if normalized.contains("aura way") || pair_matches(from, to, "stonebend", "sandmanor") {
        if pair_matches(from, to, "stonebend", "sandmanor") && curved_hint {
            RouteLineName::MountAura
        } else {
            RouteLineName::AuraWay
        }
    } else if normalized.contains("basin motorspeedway")
        || pair_matches(from, to, "stonebend", "flynt")
    {
        if pair_matches(from, to, "stonebend", "flynt") && curved_hint {
            RouteLineName::StairwayToHeaven
        } else {
            RouteLineName::BasinMotorspeedway
        }
    } else if pair_matches(from, to, "flynt", "glaushouse") {
        if curved_hint {
            RouteLineName::Riptide
        } else {
            RouteLineName::Boardwalk
        }
    } else if pair_matches(from, to, "glaushouse", "sandmanor") {
        if curved_hint {
            RouteLineName::CurrentSea
        } else {
            RouteLineName::Glausbahn
        }
    } else if pair_matches(from, to, "stonebend", "glaushouse") {
        RouteLineName::AuraRidge
    } else if normalized.contains("quarry") || normalized.contains("rim") {
        RouteLineName::QuarryRim
    } else if normalized.contains("western road") {
        RouteLineName::WesternRoad
    } else if normalized.contains("intake") || normalized.contains("tower") {
        RouteLineName::IntakeLine
    } else if normalized.contains("shelterline") || normalized.contains("shelter line") {
        RouteLineName::Shelterline
    } else if straight_hint && normalized.contains("sandmanor") && normalized.contains("glaushouse")
    {
        RouteLineName::Glausbahn
    } else {
        RouteLineName::General
    }
}

fn infer_route_lane(
    detail: &str,
    traits: &BTreeMap<String, String>,
    from: Option<&str>,
    to: Option<&str>,
    line: RouteLineName,
) -> RouteLane {
    if let Some(lane) = trait_value(traits, &["lane"]) {
        let lower = lane.to_ascii_lowercase();
        if lower.contains("ring") {
            return RouteLane::Ring;
        }
        if lower.contains("diamond") {
            return RouteLane::Diamond;
        }
        if lower.contains("spine") || lower.contains("center") {
            return RouteLane::Spine;
        }
        if lower.contains("sandmanor") {
            return RouteLane::SandmanorBranch;
        }
        if lower.contains("stonebend") || lower.contains("glaushouse") {
            return RouteLane::StonebendGlaushouseLeg;
        }
    }
    match line {
        RouteLineName::StairwayToHeaven
        | RouteLineName::Riptide
        | RouteLineName::CurrentSea
        | RouteLineName::MountAura => RouteLane::Ring,
        RouteLineName::AuraWay | RouteLineName::Glausbahn => RouteLane::SandmanorBranch,
        RouteLineName::AuraRidge => RouteLane::StonebendGlaushouseLeg,
        RouteLineName::Boardwalk => RouteLane::Diamond,
        RouteLineName::BasinMotorspeedway => RouteLane::Diamond,
        RouteLineName::QuarryRim => RouteLane::QuarryRim,
        RouteLineName::WesternRoad => RouteLane::WesternRoad,
        RouteLineName::IntakeLine => RouteLane::IntakeLine,
        RouteLineName::Shelterline => RouteLane::ShelterLane,
        RouteLineName::General => {
            let context = format!("{} {} {}", detail, from.unwrap_or(""), to.unwrap_or(""))
                .to_ascii_lowercase();
            if context.contains("sandmanor") {
                RouteLane::SandmanorBranch
            } else if context.contains("stonebend") && context.contains("glaushouse") {
                RouteLane::StonebendGlaushouseLeg
            } else if context.contains("flynt") && context.contains("glaushouse") {
                RouteLane::Diamond
            } else {
                RouteLane::General
            }
        }
    }
}

fn parse_route_line_name(value: &str) -> Option<RouteLineName> {
    let lower = value.to_ascii_lowercase();
    let normalized = lower.replace(['-', '_'], " ");
    if normalized.contains("aura ridge") {
        Some(RouteLineName::AuraRidge)
    } else if normalized.contains("aura way") {
        Some(RouteLineName::AuraWay)
    } else if normalized.contains("basin motorspeedway") {
        Some(RouteLineName::BasinMotorspeedway)
    } else if normalized.contains("boardwalk") {
        Some(RouteLineName::Boardwalk)
    } else if normalized.contains("glausbahn") {
        Some(RouteLineName::Glausbahn)
    } else if normalized.contains("stairway") {
        Some(RouteLineName::StairwayToHeaven)
    } else if normalized.contains("riptide") {
        Some(RouteLineName::Riptide)
    } else if normalized.contains("current seanad") || normalized.contains("current sea") {
        Some(RouteLineName::CurrentSea)
    } else if normalized.contains("mnt aura") || normalized.contains("mount aura") {
        Some(RouteLineName::MountAura)
    } else if normalized.contains("quarry rim") {
        Some(RouteLineName::QuarryRim)
    } else if normalized.contains("western road") {
        Some(RouteLineName::WesternRoad)
    } else if normalized.contains("intake") {
        Some(RouteLineName::IntakeLine)
    } else if normalized.contains("shelterline") || normalized.contains("shelter line") {
        Some(RouteLineName::Shelterline)
    } else {
        None
    }
}

fn infer_move_pace(detail: &str, traits: &BTreeMap<String, String>) -> MovePace {
    match trait_value(traits, &["pace", "speed"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("fast") || value.contains("rush") || value.contains("sprint") => {
            MovePace::Fast
        }
        value if value.contains("careful") || value.contains("slow") || value.contains("quiet") => {
            MovePace::Careful
        }
        _ => MovePace::Balanced,
    }
}

fn infer_move_method(detail: &str, traits: &BTreeMap<String, String>) -> MoveMethod {
    match trait_value(traits, &["method"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("scout") || value.contains("survey") => MoveMethod::Scout,
        value if value.contains("flank") => MoveMethod::Flank,
        value if value.contains("tunnel") || value.contains("burrow") => MoveMethod::Tunnel,
        value if value.contains("carry") || value.contains("haul") => MoveMethod::Carry,
        _ => MoveMethod::Traverse,
    }
}

fn infer_move_stance(detail: &str, traits: &BTreeMap<String, String>) -> MoveStance {
    match trait_value(traits, &["stance"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("quiet") || value.contains("stealth") => MoveStance::Quiet,
        value if value.contains("force") || value.contains("hard") => MoveStance::Forceful,
        _ => MoveStance::Steady,
    }
}

fn infer_decide_focus(
    detail: &str,
    raw_target: Option<&str>,
    traits: &BTreeMap<String, String>,
) -> DecideFocus {
    match trait_value(traits, &["focus"])
        .or(raw_target)
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("route") || value.contains("road") => DecideFocus::Route,
        value if value.contains("power") || value.contains("pump") => DecideFocus::Power,
        value if value.contains("shelter") || value.contains("intake") => DecideFocus::Shelter,
        value if value.contains("alliance") || value.contains("truce") => DecideFocus::Alliance,
        value if value.contains("conflict") || value.contains("raid") => DecideFocus::Conflict,
        value if value.contains("labor") || value.contains("crew") => DecideFocus::Labor,
        _ => DecideFocus::General,
    }
}

fn infer_decide_commitment(detail: &str, traits: &BTreeMap<String, String>) -> DecideCommitment {
    match trait_value(traits, &["commitment"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("hold") => DecideCommitment::Hold,
        value if value.contains("shift") || value.contains("redirect") => DecideCommitment::Shift,
        value if value.contains("withdraw") || value.contains("retreat") => {
            DecideCommitment::Withdraw
        }
        _ => DecideCommitment::Commit,
    }
}

fn infer_decide_authority(detail: &str, traits: &BTreeMap<String, String>) -> DecideAuthority {
    match trait_value(traits, &["authority"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("stonebend") => DecideAuthority::Stonebend,
        value if value.contains("glaushouse") => DecideAuthority::Glaushouse,
        value if value.contains("sandmanor") => DecideAuthority::Sandmanor,
        value if value.contains("shared") || value.contains("joint") => DecideAuthority::Shared,
        _ => DecideAuthority::Solo,
    }
}

fn infer_decide_signal(detail: &str, traits: &BTreeMap<String, String>) -> DecideSignal {
    match trait_value(traits, &["signal"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("emergency") || value.contains("alarm") => DecideSignal::Emergency,
        value if value.contains("public") || value.contains("open") => DecideSignal::Public,
        _ => DecideSignal::Quiet,
    }
}

fn infer_support_asset(detail: &str, traits: &BTreeMap<String, String>) -> SupportAsset {
    match trait_value(traits, &["asset"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("pump") => SupportAsset::Pump,
        value if value.contains("crane") => SupportAsset::Crane,
        value if value.contains("shelter") => SupportAsset::Shelter,
        value if value.contains("route") || value.contains("road") => SupportAsset::Route,
        value if value.contains("crew") || value.contains("worker") => SupportAsset::Crew,
        value
            if value.contains("power") || value.contains("grid") || value.contains("generator") =>
        {
            SupportAsset::Power
        }
        value if value.contains("intake") || value.contains("tower") => SupportAsset::Intake,
        value if value.contains("bridge") => SupportAsset::Bridge,
        _ => SupportAsset::General,
    }
}

fn infer_support_front(
    detail: &str,
    raw_target: Option<&str>,
    traits: &BTreeMap<String, String>,
) -> SupportFront {
    match trait_value(traits, &["front"])
        .or(raw_target)
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("route") || value.contains("road") => SupportFront::Route,
        value if value.contains("shelter") || value.contains("intake") => SupportFront::Shelter,
        value if value.contains("power") || value.contains("pump") => SupportFront::Power,
        value if value.contains("labor") || value.contains("crew") => SupportFront::Labor,
        _ => SupportFront::General,
    }
}

fn infer_support_intensity(detail: &str, traits: &BTreeMap<String, String>) -> SupportIntensity {
    match trait_value(traits, &["intensity"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("light") => SupportIntensity::Light,
        value if value.contains("heavy") || value.contains("hard") => SupportIntensity::Heavy,
        _ => SupportIntensity::Balanced,
    }
}

fn infer_support_duration(detail: &str, traits: &BTreeMap<String, String>) -> SupportDuration {
    match trait_value(traits, &["duration"])
        .unwrap_or(detail)
        .to_ascii_lowercase()
        .as_str()
    {
        value if value.contains("burst") || value.contains("short") => SupportDuration::Burst,
        value if value.contains("extended") || value.contains("long") => SupportDuration::Extended,
        _ => SupportDuration::Hold,
    }
}

fn pair_matches(from: Option<&str>, to: Option<&str>, left: &str, right: &str) -> bool {
    let from = from.unwrap_or("").to_ascii_lowercase();
    let to = to.unwrap_or("").to_ascii_lowercase();
    (from.contains(left) && to.contains(right)) || (from.contains(right) && to.contains(left))
}

fn directive_has_keyword(directive: &PlayerActionDirective, keywords: &[&str]) -> bool {
    let mut haystacks = vec![directive.detail.as_str()];
    if let Some(target) = directive.target.as_deref() {
        haystacks.push(target);
    }
    haystacks.extend(directive.traits.values().map(String::as_str));
    haystacks.iter().any(|value| {
        let lower = value.to_ascii_lowercase();
        keywords.iter().any(|needle| lower.contains(needle))
    })
}

fn adjust_world_value(value: &mut u16, delta: i32) {
    let adjusted = (*value as i32 + delta).clamp(0, 100);
    *value = adjusted as u16;
}

fn extract_world_input_detail<'a>(world_inputs: &'a [String], prefix: &str) -> Option<&'a str> {
    world_inputs
        .iter()
        .find_map(|line| line.strip_prefix(prefix))
}

fn render_properties(properties: &PropertyMap) -> String {
    if properties.is_empty() {
        return String::from("- none\n");
    }
    let mut output = String::new();
    for (name, value) in properties {
        let _ = fmt::Write::write_fmt(&mut output, format_args!("- {}: {}\n", name, value));
    }
    output
}

fn render_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        return String::from("- none\n");
    }
    let mut output = String::new();
    for line in lines {
        let _ = fmt::Write::write_fmt(&mut output, format_args!("- {}\n", line));
    }
    output
}

fn render_candidate_list(candidates: &[BondCandidate], selected_id: Option<&str>) -> String {
    let mut output = String::new();
    for candidate in candidates {
        let marker = if selected_id == Some(candidate.id.as_str()) {
            "selected"
        } else {
            "unused"
        };
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!(
                "- {} [{}] side={} viability={} cost={}\n",
                candidate.id,
                marker,
                candidate.side.as_str(),
                candidate.viability,
                candidate.cost
            ),
        );
    }
    output
}

fn render_numbered_candidates(candidates: &[BondCandidate]) -> String {
    let mut output = String::new();
    for (index, candidate) in candidates.iter().enumerate() {
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!("{}. {}\n", index + 1, candidate.id),
        );
    }
    output
}

fn render_unused_bonds(candidates: &[BondCandidate], side: SemanticSide) -> String {
    let mut output = String::new();
    for candidate in candidates.iter().filter(|candidate| candidate.side == side) {
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!(
                "{} -> {} -> {}\n",
                candidate.id,
                destination_for_side(side).as_str(),
                side.family_name()
            ),
        );
    }
    if output.is_empty() {
        output.push_str("none\n");
    }
    output
}

fn render_way_list(ways: &[EngineWay]) -> String {
    let mut output = String::new();
    for way in ways {
        let _ = fmt::Write::write_fmt(
            &mut output,
            format_args!(
                "- {} source={} direction={} side={} availability={}\n",
                way.id,
                way.source,
                way.direction,
                way.side.as_str(),
                way.availability
            ),
        );
    }
    output
}

fn push_snapshot_line(output: &mut String, key: &str, value: &str) {
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&escape_snapshot_value(value));
    output.push('\n');
}

fn push_snapshot_fields(output: &mut String, key: &str, fields: &[&str]) {
    output.push_str(key);
    output.push_str(": ");
    output.push_str(
        &fields
            .iter()
            .map(|field| escape_snapshot_value(field))
            .collect::<Vec<_>>()
            .join("\t"),
    );
    output.push('\n');
}

fn push_resource_lines(
    output: &mut String,
    aura_total_key: &str,
    aura_property_key: &str,
    current_total_key: &str,
    current_property_key: &str,
    resource: &ResourceComposition,
) {
    output.push_str(&format!("{aura_total_key}: {}\n", resource.aura_total));
    for (key, value) in &resource.aura_properties {
        push_snapshot_fields(output, aura_property_key, &[key, &value.to_string()]);
    }
    output.push_str(&format!(
        "{current_total_key}: {}\n",
        resource.current_total
    ));
    for (key, value) in &resource.current_properties {
        push_snapshot_fields(output, current_property_key, &[key, &value.to_string()]);
    }
}

fn escape_snapshot_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn unescape_snapshot_value(value: &str) -> io::Result<String> {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        let escaped = chars.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "snapshot value ended with an incomplete escape sequence",
            )
        })?;
        match escaped {
            '\\' => output.push('\\'),
            'n' => output.push('\n'),
            't' => output.push('\t'),
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unsupported snapshot escape sequence: \\{other}"),
                ));
            }
        }
    }
    Ok(output)
}

fn split_snapshot_fields(value: &str, expected: usize, key: &str) -> io::Result<Vec<String>> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                let escaped = chars.next().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{key} contains an incomplete escape sequence"),
                    )
                })?;
                match escaped {
                    '\\' => current.push('\\'),
                    'n' => current.push('\n'),
                    't' => current.push('\t'),
                    other => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("{key} contains unsupported escape sequence: \\{other}"),
                        ));
                    }
                }
            }
            '\t' => {
                fields.push(current);
                current = String::new();
            }
            other => current.push(other),
        }
    }
    fields.push(current);
    if fields.len() != expected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{key} expected {expected} fields, got {}", fields.len()),
        ));
    }
    Ok(fields)
}

fn parse_snapshot_bool(value: &str, key: &str) -> io::Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid bool for {key}: {value}"),
        )),
    }
}

fn parse_snapshot_u16(value: &str, key: &str) -> io::Result<u16> {
    value.parse::<u16>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid u16 for {key}: {value}"),
        )
    })
}

fn parse_snapshot_u32(value: &str, key: &str) -> io::Result<u32> {
    value.parse::<u32>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid u32 for {key}: {value}"),
        )
    })
}

fn parse_snapshot_u64(value: &str, key: &str) -> io::Result<u64> {
    value.parse::<u64>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid u64 for {key}: {value}"),
        )
    })
}

fn parse_semantic_side(value: &str) -> io::Result<SemanticSide> {
    match value {
        "left" => Ok(SemanticSide::Left),
        "right" => Ok(SemanticSide::Right),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid semantic side: {value}"),
        )),
    }
}

fn parse_residue_destination(value: &str) -> io::Result<ResidueDestination> {
    match value {
        "Aether" => Ok(ResidueDestination::Aether),
        "Bathos" => Ok(ResidueDestination::Bathos),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid residue destination: {value}"),
        )),
    }
}

fn serialize_property_map(map: &PropertyMap) -> String {
    map.iter()
        .map(|(key, value)| format!("{}={value}", escape_snapshot_value(key)))
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_property_map(value: &str) -> io::Result<PropertyMap> {
    let mut map = PropertyMap::new();
    if value.is_empty() {
        return Ok(map);
    }
    for entry in value.split(',') {
        let (key, raw_value) = entry.split_once('=').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid property map entry: {entry}"),
            )
        })?;
        map.insert(
            unescape_snapshot_value(key)?,
            parse_snapshot_u16(raw_value, "property_value")?,
        );
    }
    Ok(map)
}

fn serialize_engine_way(way: &EngineWay) -> String {
    [
        way.id.as_str(),
        way.source.as_str(),
        way.direction.as_str(),
        way.side.as_str(),
        &serialize_property_map(&way.properties),
        &way.availability.to_string(),
    ]
    .iter()
    .map(|field| escape_snapshot_value(field))
    .collect::<Vec<_>>()
    .join("\t")
}

fn parse_engine_way(value: &str) -> io::Result<EngineWay> {
    let fields = split_snapshot_fields(value, 6, "engine_way")?;
    Ok(EngineWay {
        id: fields[0].clone(),
        source: fields[1].clone(),
        direction: fields[2].clone(),
        side: parse_semantic_side(&fields[3])?,
        properties: parse_property_map(&fields[4])?,
        availability: parse_snapshot_u16(&fields[5], "engine_way availability")?,
    })
}

fn serialize_bond_candidate(candidate: &BondCandidate) -> String {
    [
        candidate.id.as_str(),
        &candidate.participants.join("\u{1f}"),
        &candidate.selected_arms.join("\u{1f}"),
        candidate.side.as_str(),
        &serialize_property_map(&candidate.properties),
        &candidate.viability.to_string(),
        &candidate.cost.to_string(),
        candidate.source_need.as_str(),
    ]
    .iter()
    .map(|field| escape_snapshot_value(field))
    .collect::<Vec<_>>()
    .join("\t")
}

fn parse_bond_candidate(value: &str) -> io::Result<BondCandidate> {
    let fields = split_snapshot_fields(value, 8, "bond_candidate")?;
    Ok(BondCandidate {
        id: fields[0].clone(),
        participants: if fields[1].is_empty() {
            Vec::new()
        } else {
            fields[1].split('\u{1f}').map(str::to_owned).collect()
        },
        selected_arms: if fields[2].is_empty() {
            Vec::new()
        } else {
            fields[2].split('\u{1f}').map(str::to_owned).collect()
        },
        side: parse_semantic_side(&fields[3])?,
        properties: parse_property_map(&fields[4])?,
        viability: parse_snapshot_u16(&fields[5], "bond_candidate viability")?,
        cost: parse_snapshot_u16(&fields[6], "bond_candidate cost")?,
        source_need: fields[7].clone(),
    })
}

fn serialize_residue(residue: &Residue) -> String {
    [
        residue.source_bond.as_str(),
        residue.side.as_str(),
        &serialize_property_map(&residue.properties),
        residue.destination.as_str(),
    ]
    .iter()
    .map(|field| escape_snapshot_value(field))
    .collect::<Vec<_>>()
    .join("\t")
}

fn parse_residue(value: &str) -> io::Result<Residue> {
    let fields = split_snapshot_fields(value, 4, "residue")?;
    Ok(Residue {
        source_bond: fields[0].clone(),
        side: parse_semantic_side(&fields[1])?,
        properties: parse_property_map(&fields[2])?,
        destination: parse_residue_destination(&fields[3])?,
    })
}

fn find_snapshot_npc_mut<'a>(
    npcs: &'a mut [NpcState],
    npc_id: &str,
    key: &str,
) -> io::Result<&'a mut NpcState> {
    npcs.iter_mut().find(|npc| npc.id == npc_id).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{key} references unknown npc id: {npc_id}"),
        )
    })
}

fn required_snapshot_field<T>(value: Option<T>, key: &str) -> io::Result<T> {
    value.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("snapshot missing {key}"),
        )
    })
}

fn select_snapshot_bond(
    bonds: &[BondCandidate],
    selected_id: &str,
    key: &str,
) -> io::Result<BondCandidate> {
    bonds
        .iter()
        .find(|bond| bond.id == selected_id)
        .cloned()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{key} references unknown selected bond id: {selected_id}"),
            )
        })
}

fn build_tick_record_from_snapshot_builder(
    tick_number: u32,
    builder: SnapshotTickBuilder,
) -> io::Result<TickRecord> {
    let forward_selected_bond_id = required_snapshot_field(
        builder.forward_selected_bond_id,
        "tick_forward selected bond",
    )?;
    let blep_selected_bond_id =
        required_snapshot_field(builder.blep_selected_bond_id, "tick_blep selected bond")?;
    let blep_decision = BLEPDecision {
        npc_id: required_snapshot_field(builder.blep_npc_id, "tick_blep npc_id")?,
        world_inputs: builder.blep_world_inputs,
        inferred_need: required_snapshot_field(
            builder.blep_inferred_need,
            "tick_blep inferred_need",
        )?,
        candidate_bonds: builder.blep_candidate_bonds.clone(),
        selected_bond: select_snapshot_bond(
            &builder.blep_candidate_bonds,
            &blep_selected_bond_id,
            "tick_blep",
        )?,
        resulting_action: required_snapshot_field(
            builder.blep_resulting_action,
            "tick_blep resulting_action",
        )?,
        unused_bonds: builder.blep_unused_bonds,
        confidence: required_snapshot_field(builder.blep_confidence, "tick_blep confidence")?,
    };
    let coordinated_blep_relays = if builder.coordinated_relays.is_empty() {
        vec![CoordinatedBlepRelay {
            npc_id: blep_decision.npc_id.clone(),
            npc_moment_id: required_snapshot_field(
                builder.npc_moment_id.clone(),
                "tick npc_moment_id",
            )?,
            committed: true,
            blep_decision: blep_decision.clone(),
            blep_residues: builder.blep_residues.clone(),
        }]
    } else {
        builder
            .relay_order
            .iter()
            .map(|npc_id| {
                let relay = builder.coordinated_relays.get(npc_id).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("snapshot missing relay builder for npc id: {npc_id}"),
                    )
                })?;
                let selected_bond_id = required_snapshot_field(
                    relay.selected_bond_id.clone(),
                    "tick_blep_relay selected bond",
                )?;
                Ok(CoordinatedBlepRelay {
                    npc_id: npc_id.clone(),
                    npc_moment_id: required_snapshot_field(
                        relay.npc_moment_id.clone(),
                        "tick_blep_relay npc_moment_id",
                    )?,
                    committed: required_snapshot_field(
                        relay.committed,
                        "tick_blep_relay committed",
                    )?,
                    blep_decision: BLEPDecision {
                        npc_id: npc_id.clone(),
                        world_inputs: relay.world_inputs.clone(),
                        inferred_need: required_snapshot_field(
                            relay.inferred_need.clone(),
                            "tick_blep_relay inferred_need",
                        )?,
                        candidate_bonds: relay.candidate_bonds.clone(),
                        selected_bond: select_snapshot_bond(
                            &relay.candidate_bonds,
                            &selected_bond_id,
                            "tick_blep_relay",
                        )?,
                        resulting_action: required_snapshot_field(
                            relay.resulting_action.clone(),
                            "tick_blep_relay resulting_action",
                        )?,
                        unused_bonds: relay.unused_bonds.clone(),
                        confidence: required_snapshot_field(
                            relay.confidence,
                            "tick_blep_relay confidence",
                        )?,
                    },
                    blep_residues: relay.residues.clone(),
                })
            })
            .collect::<io::Result<Vec<_>>>()?
    };
    Ok(TickRecord {
        tick_number,
        player_moment_id: required_snapshot_field(
            builder.player_moment_id,
            "tick player_moment_id",
        )?,
        npc_moment_id: required_snapshot_field(builder.npc_moment_id, "tick npc_moment_id")?,
        forward_pass: ForwardSynthesisPass {
            human_need: required_snapshot_field(
                builder.forward_human_need,
                "tick_forward human_need",
            )?,
            machine_complement: required_snapshot_field(
                builder.forward_machine_complement,
                "tick_forward machine_complement",
            )?,
            available_ways: builder.forward_available_ways,
            candidate_bonds: builder.forward_candidate_bonds.clone(),
            bond_result: BondResult {
                selected_bond: select_snapshot_bond(
                    &builder.forward_candidate_bonds,
                    &forward_selected_bond_id,
                    "tick_forward",
                )?,
                resulting_link: required_snapshot_field(
                    builder.forward_resulting_link,
                    "tick_forward resulting_link",
                )?,
                resulting_moment: required_snapshot_field(
                    builder.forward_resulting_moment,
                    "tick_forward resulting_moment",
                )?,
                unused_bonds: builder.forward_unused_bonds,
            },
        },
        forward_residues: builder.forward_residues,
        blep_decision,
        blep_residues: builder.blep_residues,
        coordinated_blep_relays,
        resources_after_tick: builder.resources_after_tick,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{Bond, Symptom, Way, run_kernel_cycle};

    use super::{
        CURRENT_SYNTHESIS_EVENT_LOG_ARTIFACT_PATH, CURRENT_SYNTHESIS_SNAPSHOT_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH, CurrentSynthesisEvent, CurrentSynthesisState,
        EngineLens, MoveMethod, MovePace, MoveStance, PersistedCurrentSynthesisState,
        PlayerActionKind, PlayerActionSchema, ResidueDestination, RouteLineName, SemanticSide,
        WorldState, advance_current_synthesis_player_action_at, append_current_synthesis_tick_at,
        append_current_synthesis_ticks_at, build_bond_inspector_output, build_bond_trace_output,
        build_cleopatra_trace_output, build_engine_output, build_persisted_state_output,
        build_resource_history_output, build_state_snapshot_output, decode_player_action,
        effective_hueman_feedback, encode_current_synthesis_player_action,
        load_current_synthesis_at, parse_event_log, parse_persisted_state, parse_state_snapshot,
        read_or_create_persisted_state_at, route_line_definition, write_persisted_state_at,
    };
    use crate::current_synthesis_scenario::DEFAULT_SCENARIO_ID;

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn pleb_meta_pass_selects_exactly_one_active_bond() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        let selected_id = &tick.forward_pass.bond_result.selected_bond.id;
        let selected_count = tick
            .forward_pass
            .candidate_bonds
            .iter()
            .filter(|bond| &bond.id == selected_id)
            .count();
        assert_eq!(selected_count, 1);
    }

    #[test]
    fn selected_bond_becomes_link_and_moment() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert!(!tick.forward_pass.bond_result.resulting_link.is_empty());
        assert!(
            tick.forward_pass
                .bond_result
                .resulting_moment
                .contains("valley")
        );
    }

    #[test]
    fn unselected_left_bonds_become_aether_and_aura() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert!(
            tick.forward_residues
                .iter()
                .any(|residue| residue.side == SemanticSide::Left
                    && residue.destination == ResidueDestination::Aether)
        );
        assert!(state.resources().aura_total > 0);
    }

    #[test]
    fn unselected_right_bonds_become_bathos_and_current() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert!(
            tick.forward_residues
                .iter()
                .any(|residue| residue.side == SemanticSide::Right
                    && residue.destination == ResidueDestination::Bathos)
        );
        assert!(state.resources().current_total > 0);
    }

    #[test]
    fn bond_properties_survive_inside_resource_composition() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        assert!(state.resources().aura_properties.contains_key("perception"));
        assert!(
            state
                .resources()
                .current_properties
                .contains_key("structure")
        );
    }

    #[test]
    fn blep_reads_world_conditions_and_generates_npc_candidates() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line.contains("road blockade"))
        );
        assert!(tick.blep_decision.candidate_bonds.len() >= 4);
    }

    #[test]
    fn cleopatra_selects_a_viable_action_without_fixed_scripting() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert!(tick.blep_decision.selected_bond.viability > 0);
        assert!(!tick.blep_decision.resulting_action.is_empty());
    }

    #[test]
    fn cleopatra_coordinates_the_full_active_npc_set_each_cycle() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert_eq!(
            tick.coordinated_blep_relays.len(),
            state.cleopatra().active_npcs.len()
        );
        assert_eq!(
            tick.coordinated_blep_relays
                .iter()
                .filter(|relay| relay.committed)
                .count(),
            1
        );
        assert!(tick.coordinated_blep_relays.iter().all(|relay| {
            relay
                .blep_decision
                .world_inputs
                .iter()
                .any(|line| line.starts_with("joint relay "))
        }));
    }

    #[test]
    fn npc_unused_bonds_feed_the_same_resource_environment() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        assert!(!tick.blep_residues.is_empty());
        assert!(state.resources().current_total >= property_sum(&tick.blep_residues));
    }

    #[test]
    fn repeated_passes_alter_future_possibilities_through_accumulated_resources() {
        let first_state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let second_state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState {
            completed_ticks: 2,
            ..PersistedCurrentSynthesisState::primary()
        });
        let first_selected = &first_state
            .last_tick()
            .expect("tick")
            .forward_pass
            .bond_result
            .selected_bond
            .id;
        let second_selected = &second_state
            .last_tick()
            .expect("tick")
            .forward_pass
            .bond_result
            .selected_bond
            .id;
        assert_ne!(
            first_state.resources().current_total,
            second_state.resources().current_total
        );
        assert_ne!(first_selected, second_selected);
    }

    #[test]
    fn kernel_remains_unaware_of_current_synthesis_semantics() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        assert_eq!(
            Bond::select([Way::One, Way::Two, Way::Three]).linked_way(),
            Way::One
        );
        let witness = kernel_pass.to_string();
        assert!(!witness.contains("PLEB"));
        assert!(!witness.contains("META"));
        assert!(!witness.contains("BLEP"));
        assert!(!witness.contains("Cleopatra"));
        assert!(witness.contains("AuraBeam"));
        assert!(witness.contains("CurrentSeam"));
        assert!(!witness.contains("Current Synthesis"));
    }

    #[test]
    fn persisted_state_round_trips() {
        let persisted = PersistedCurrentSynthesisState::primary();
        let output = build_persisted_state_output(&persisted);
        let parsed = parse_persisted_state(&output).expect("state should parse");
        assert_eq!(parsed, persisted);
    }

    #[test]
    fn persisted_state_bootstraps_on_first_read() {
        let root = unique_temp_dir("current-synthesis-state");
        let persisted = read_or_create_persisted_state_at(&root).expect("state should bootstrap");
        assert_eq!(persisted, PersistedCurrentSynthesisState::primary());
        assert!(
            root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH)
                .exists()
        );
        assert!(
            root.join(CURRENT_SYNTHESIS_EVENT_LOG_ARTIFACT_PATH)
                .exists()
        );
        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn persisted_state_writes_explicit_tick_count() {
        let root = unique_temp_dir("current-synthesis-state-write");
        let persisted = PersistedCurrentSynthesisState {
            completed_ticks: 3,
            ..PersistedCurrentSynthesisState::primary()
        };
        write_persisted_state_at(&root, &persisted).expect("state should write");
        let contents = fs::read_to_string(root.join(CURRENT_SYNTHESIS_TUI_STATE_ARTIFACT_PATH))
            .expect("state should exist");
        assert!(contents.contains("completed_ticks: 3"));
        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn event_log_and_checkpoint_round_trip_the_replayed_state() {
        let root = unique_temp_dir("current-synthesis-checkpoint-roundtrip");
        let persisted = PersistedCurrentSynthesisState {
            completed_ticks: 3,
            ..PersistedCurrentSynthesisState::primary()
        };
        write_persisted_state_at(&root, &persisted).expect("legacy persisted state should write");
        let (loaded_persisted, resumed) =
            load_current_synthesis_at(&root, None).expect("checkpointed load should succeed");

        assert_eq!(loaded_persisted, persisted);
        assert_eq!(resumed, CurrentSynthesisState::replay(&persisted));
        assert!(root.join(CURRENT_SYNTHESIS_SNAPSHOT_ARTIFACT_PATH).exists());
        assert!(
            root.join(CURRENT_SYNTHESIS_EVENT_LOG_ARTIFACT_PATH)
                .exists()
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn event_log_can_resume_one_additional_tick_from_checkpoint() {
        let root = unique_temp_dir("current-synthesis-checkpoint-resume");
        let first = PersistedCurrentSynthesisState {
            completed_ticks: 2,
            ..PersistedCurrentSynthesisState::primary()
        };
        let second = PersistedCurrentSynthesisState {
            completed_ticks: 3,
            ..PersistedCurrentSynthesisState::primary()
        };

        write_persisted_state_at(&root, &first).expect("legacy persisted state should write");
        let (_first_persisted, _first_state) =
            load_current_synthesis_at(&root, None).expect("checkpoint should build");
        let (resumed_persisted, resumed) = append_current_synthesis_tick_at(&root, None, None)
            .expect("tick append should succeed");

        assert_eq!(resumed_persisted, second);
        assert_eq!(resumed, CurrentSynthesisState::replay(&second));

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn event_log_ignores_a_torn_final_line() {
        let contents = "# Current Synthesis Event Log\n\
event: scenario_selected\tscout_valley_vertical_slice\t7\troute_warden_04\n\
se";

        let events = parse_event_log(contents).expect("torn final line should be ignored");

        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            CurrentSynthesisEvent::ScenarioSelected { scenario_id, .. }
                if scenario_id == "scout_valley_vertical_slice"
        ));
    }

    #[test]
    fn batched_tick_append_advances_multiple_cycles_in_one_write() {
        let root = unique_temp_dir("current-synthesis-checkpoint-batch");
        let first = PersistedCurrentSynthesisState {
            completed_ticks: 2,
            ..PersistedCurrentSynthesisState::primary()
        };
        let second = PersistedCurrentSynthesisState {
            completed_ticks: 5,
            ..PersistedCurrentSynthesisState::primary()
        };

        write_persisted_state_at(&root, &first).expect("legacy persisted state should write");
        let (_persisted, _state) =
            load_current_synthesis_at(&root, None).expect("checkpoint should build");
        let (resumed_persisted, resumed) = append_current_synthesis_ticks_at(&root, None, 3, None)
            .expect("batched tick append should succeed");

        assert_eq!(resumed_persisted, second);
        assert_eq!(resumed, CurrentSynthesisState::replay(&second));

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn renderers_surface_the_vertical_slice() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let tick = state.last_tick().expect("tick should exist");
        let engine = build_engine_output(&state, EngineLens::Status);
        let bond =
            build_bond_inspector_output(&state, &tick.forward_pass.bond_result.selected_bond.id)
                .expect("bond should render");
        let trace =
            build_bond_trace_output(&state, &tick.player_moment_id).expect("trace should render");
        let cleopatra = build_cleopatra_trace_output(&state, "route_warden_04")
            .expect("cleopatra trace should render");
        let resources = build_resource_history_output(&state);
        assert!(engine.contains("PLEB"));
        assert!(bond.contains("selected arms"));
        assert!(trace.contains("Unused Left Bonds"));
        assert!(cleopatra.contains("Selected Bond"));
        assert!(resources.contains("Aura Composition"));
    }

    #[test]
    fn advancing_a_player_move_executes_the_full_triad_in_one_pass() {
        let root = unique_temp_dir("current-synthesis-player-move");
        let encoded = encode_current_synthesis_player_action("move", "cross the flooded rim")
            .expect("move action should encode");
        let (_persisted, state) =
            advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                .expect("move advance should succeed");
        let tick = state.last_tick().expect("tick should exist");

        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line.contains("player move cross the flooded rim"))
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line.starts_with("clouseau relay "))
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line.starts_with("hal relay "))
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line.starts_with("joint relay "))
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn structured_player_action_decoding_extracts_target_and_traits() {
        let directive = decode_player_action(
            "move::cross the flooded rim target=upper-rim pace=fast stance=quiet",
        );

        assert_eq!(directive.kind, PlayerActionKind::Move);
        assert_eq!(directive.detail, "cross the flooded rim");
        assert_eq!(directive.target.as_deref(), Some("upper-rim"));
        assert_eq!(
            directive.traits.get("pace").map(String::as_str),
            Some("fast")
        );
        assert_eq!(
            directive.traits.get("stance").map(String::as_str),
            Some("quiet")
        );
        match directive.schema {
            PlayerActionSchema::Move(spec) => {
                assert_eq!(spec.to.as_deref(), Some("upper-rim"));
                assert_eq!(spec.line, RouteLineName::QuarryRim);
                assert_eq!(spec.pace, MovePace::Fast);
                assert_eq!(spec.stance, MoveStance::Quiet);
            }
            other => panic!("expected move schema, got {other:?}"),
        }
    }

    #[test]
    fn structured_player_actions_feed_world_inputs_and_shift_world_state() {
        let root = unique_temp_dir("current-synthesis-player-structured");
        let baseline = WorldState::for_scenario(DEFAULT_SCENARIO_ID);
        let encoded = encode_current_synthesis_player_action(
            "move",
            "cross the flooded rim target=upper-rim pace=fast",
        )
        .expect("move action should encode");
        let (_persisted, state) =
            advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                .expect("move advance should succeed");
        let tick = state.last_tick().expect("tick should exist");

        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move to upper-rim")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move line quarry-rim")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move pace=fast")
        );
        assert!(state.world().route_stability > baseline.route_stability);
        assert_ne!(state.world().power_stability, baseline.power_stability);

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn named_fourway_lines_infer_from_route_pairs() {
        let flynt_to_glaushouse =
            decode_player_action("move::from=flynt to=glaushouse method=scout");
        let stonebend_to_glaushouse =
            decode_player_action("move::from=stonebend to=glaushouse pace=balanced");
        let glaushouse_to_sandmanor =
            decode_player_action("move::from=glaushouse to=sandmanor straight");

        match flynt_to_glaushouse.schema {
            PlayerActionSchema::Move(spec) => {
                assert_eq!(spec.line, RouteLineName::Boardwalk);
                assert_eq!(spec.method, MoveMethod::Scout);
            }
            other => panic!("expected move schema, got {other:?}"),
        }
        match stonebend_to_glaushouse.schema {
            PlayerActionSchema::Move(spec) => {
                assert_eq!(spec.line, RouteLineName::AuraRidge);
            }
            other => panic!("expected move schema, got {other:?}"),
        }
        match glaushouse_to_sandmanor.schema {
            PlayerActionSchema::Move(spec) => {
                assert_eq!(spec.line, RouteLineName::Glausbahn);
            }
            other => panic!("expected move schema, got {other:?}"),
        }
    }

    #[test]
    fn aura_ridge_segment_aliases_fold_into_canonical_line() {
        let directive =
            decode_player_action("move::from=stonebend to=sandmanor line=aura-ridge-east");

        match directive.schema {
            PlayerActionSchema::Move(spec) => {
                assert_eq!(spec.line, RouteLineName::AuraRidge);
            }
            other => panic!("expected move schema, got {other:?}"),
        }
    }

    #[test]
    fn decide_actions_emit_site_and_domain_context() {
        let root = unique_temp_dir("current-synthesis-player-decide-context");
        let encoded = encode_current_synthesis_player_action(
            "decide",
            "target=aura-ridge-east focus=route commitment=shift authority=shared signal=public",
        )
        .expect("decide action should encode");
        let (_persisted, state) =
            advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                .expect("decide advance should succeed");
        let tick = state.last_tick().expect("tick should exist");

        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player decide site aura-ridge-east")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player decide site-name Aura Ridge East")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player decide domain route-stability")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player decide line-family Aura Ridge")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player decide surface-custodian clouseau")
        );
        assert!(
            build_engine_output(&state, EngineLens::Status)
                .contains("Aura Ridge East across Route Stability")
        );
        assert!(
            build_engine_output(&state, EngineLens::Status).contains("bond/player/enhanced-sight/")
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn support_actions_emit_site_and_route_context() {
        let root = unique_temp_dir("current-synthesis-player-support-context");
        let encoded = encode_current_synthesis_player_action(
            "support",
            "target=aura-ridge-east asset=bridge front=route intensity=heavy duration=burst",
        )
        .expect("support action should encode");
        let (_persisted, state) =
            advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                .expect("support advance should succeed");
        let tick = state.last_tick().expect("tick should exist");

        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player support site aura-ridge-east")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player support site-name Aura Ridge East")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player support line-family Aura Ridge")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player support surface-custodian clouseau")
        );
        assert!(build_engine_output(&state, EngineLens::Status).contains("Aura Ridge East"));
        assert!(
            build_engine_output(&state, EngineLens::Status).contains("bond/player/anchor-armor/")
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn mixed_move_decide_support_sequence_stays_route_aware() {
        let root = unique_temp_dir("current-synthesis-player-mixed-stress");
        let initial_completed_ticks = load_current_synthesis_at(&root, None)
            .expect("initial state should load")
            .1
            .completed_ticks();
        let sequence = [
            (
                "move",
                "from=stonebend to=sandmanor line=aura-ridge-east pace=careful method=scout stance=quiet",
                "movement-first",
                "player move line-segment aura-ridge-east",
                "player move line-name Aura Ridge East",
            ),
            (
                "decide",
                "target=aura-ridge-east focus=route commitment=shift authority=shared signal=public",
                "decision-first",
                "player decide site aura-ridge-east",
                "player decide domain route-stability",
            ),
            (
                "support",
                "target=aura-ridge-east asset=bridge front=route intensity=heavy duration=burst",
                "support-first",
                "player support site aura-ridge-east",
                "player support line-family Aura Ridge",
            ),
        ];

        let mut prior_completed_ticks = None;
        let mut final_state = None;

        for _ in 0..10 {
            for (kind, action_text, posture, world_input_a, world_input_b) in sequence {
                let encoded = encode_current_synthesis_player_action(kind, action_text)
                    .expect("mixed action should encode");
                let (_persisted, state) =
                    advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                        .expect("mixed action should advance");
                let tick = state.last_tick().expect("tick should exist");
                let status = build_engine_output(&state, EngineLens::Status);
                let completed_ticks = state.completed_ticks();

                assert!(
                    tick.blep_decision
                        .world_inputs
                        .iter()
                        .any(|line| line == world_input_a)
                );
                assert!(
                    tick.blep_decision
                        .world_inputs
                        .iter()
                        .any(|line| line == world_input_b)
                );
                assert!(
                    tick.blep_decision
                        .world_inputs
                        .iter()
                        .any(|line| line == "player move surface-custodian clouseau")
                        || tick
                            .blep_decision
                            .world_inputs
                            .iter()
                            .any(|line| line == "player decide surface-custodian clouseau")
                        || tick
                            .blep_decision
                            .world_inputs
                            .iter()
                            .any(|line| line == "player support surface-custodian clouseau")
                );
                assert!(status.contains("Aura Ridge East"));
                assert!(status.contains("bond/player/"));
                assert!(status.contains(posture));

                if let Some(previous) = prior_completed_ticks {
                    assert_eq!(completed_ticks, previous + 1);
                }
                prior_completed_ticks = Some(completed_ticks);
                final_state = Some(state);
            }
        }

        let final_state = final_state.expect("final state should exist");
        assert_eq!(final_state.completed_ticks(), initial_completed_ticks + 30);

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn named_route_topology_is_emitted_into_world_inputs() {
        let root = unique_temp_dir("current-synthesis-player-topology");
        let encoded = encode_current_synthesis_player_action(
            "move",
            "from=stonebend to=glaushouse line=aura-ridge pace=fast method=traverse stance=steady",
        )
        .expect("move action should encode");
        let (_persisted, state) =
            advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                .expect("move advance should succeed");
        let tick = state.last_tick().expect("tick should exist");

        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move line aura-ridge")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move line-segment aura-ridge-south")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move line-name Aura Ridge South")
        );
        assert!(
            tick.blep_decision.world_inputs.iter().any(|line| line
                == "player move topology Aura Ridge South straight stonebend -> glaushouse")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move line-family Aura Ridge")
        );
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line.contains("Aura Ridge North, South, and East"))
        );
        assert!(build_engine_output(&state, EngineLens::Status).contains("Aura Ridge South"));
        assert!(
            tick.blep_decision
                .world_inputs
                .iter()
                .any(|line| line == "player move surface-custodian clouseau")
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn curved_hal_routes_emit_cleopatra_inverse_custody() {
        let root = unique_temp_dir("current-synthesis-player-curved-custody");
        let encoded = encode_current_synthesis_player_action(
            "move",
            "from=sandmanor to=stonebend line=mnt-aura pace=careful method=scout stance=quiet",
        )
        .expect("move action should encode");
        let (_persisted, state) =
            advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                .expect("move advance should succeed");
        let tick = state.last_tick().expect("tick should exist");
        let joined_inputs = tick.blep_decision.world_inputs.join("\n");

        assert!(joined_inputs.contains("player move surface-custodian hal"));
        assert!(joined_inputs.contains("player move inverse-custodian cleopatra"));
        assert!(joined_inputs.contains("route custody: HAL holds Stairway to Heaven"));
        assert!(
            build_engine_output(&state, EngineLens::Status)
                .contains("inverse route belongs to Cleopatra")
        );

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn remaining_named_routes_emit_consistent_topology_and_custody() {
        let root = unique_temp_dir("current-synthesis-player-route-table");
        let route_lines = [
            RouteLineName::StairwayToHeaven,
            RouteLineName::BasinMotorspeedway,
            RouteLineName::Riptide,
            RouteLineName::Boardwalk,
            RouteLineName::CurrentSea,
            RouteLineName::Glausbahn,
            RouteLineName::MountAura,
            RouteLineName::AuraWay,
        ];

        let initial_completed_ticks = load_current_synthesis_at(&root, None)
            .expect("initial state should load")
            .1
            .completed_ticks();
        let mut completed_ticks = initial_completed_ticks;

        for _ in 0..2 {
            for line in route_lines {
                let definition = route_line_definition(line).expect("route should be defined");
                let encoded = encode_current_synthesis_player_action(
                    "move",
                    &format!(
                        "from={} to={} line={} pace=careful method=scout stance=quiet",
                        definition.from.as_str(),
                        definition.to.as_str(),
                        definition.line.as_str()
                    ),
                )
                .expect("route move should encode");
                let (_persisted, state) =
                    advance_current_synthesis_player_action_at(&root, &encoded, None, None)
                        .expect("route move should advance");
                let tick = state.last_tick().expect("tick should exist");
                let joined_inputs = tick.blep_decision.world_inputs.join("\n");
                let status = build_engine_output(&state, EngineLens::Status);

                assert!(
                    joined_inputs
                        .contains(&format!("player move line {}", definition.line.as_str()))
                );
                assert!(joined_inputs.contains(&format!(
                    "player move line-segment {}",
                    definition.line.as_str()
                )));
                assert!(joined_inputs.contains(&format!(
                    "player move line-name {}",
                    definition.line.display_name()
                )));
                assert!(joined_inputs.contains(&format!(
                    "player move topology {} {} {} -> {}",
                    definition.line.display_name(),
                    definition.shape.as_str(),
                    definition.from.as_str(),
                    definition.to.as_str()
                )));
                assert!(joined_inputs.contains(&format!(
                    "player move line-family {}",
                    definition.line.display_name()
                )));
                assert!(joined_inputs.contains(&format!(
                    "player move surface-custodian {}",
                    definition.surface_custodian.as_str()
                )));
                if let Some(inverse) = definition.inverse_custodian {
                    assert!(joined_inputs.contains(&format!(
                        "player move inverse-custodian {}",
                        inverse.as_str()
                    )));
                } else {
                    assert!(!joined_inputs.contains("player move inverse-custodian "));
                }
                assert!(status.contains(definition.line.display_name()));
                assert!(status.contains("bond/player/"));
                completed_ticks += 1;
                assert_eq!(state.completed_ticks(), completed_ticks);
            }
        }

        assert_eq!(completed_ticks, initial_completed_ticks + 16);

        fs::remove_dir_all(root).expect("cleanup should succeed");
    }

    #[test]
    fn state_snapshot_round_trips_world_metrics() {
        let state = CurrentSynthesisState::replay(&PersistedCurrentSynthesisState::primary());
        let feedback = effective_hueman_feedback(None);
        let snapshot = build_state_snapshot_output(&state, Some(feedback), 4);
        let parsed = parse_state_snapshot(&snapshot).expect("snapshot should parse");

        assert_eq!(parsed.state.world, state.world);
        assert_eq!(parsed.hueman_feedback, feedback);
        assert_eq!(parsed.applied_event_count, 4);
    }

    fn property_sum(residues: &[super::Residue]) -> u16 {
        residues
            .iter()
            .flat_map(|residue| residue.properties.values())
            .copied()
            .sum()
    }
}
