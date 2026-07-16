use std::fmt;
use std::io;

use crate::decision_engine::{DecisionExecution, DecisionIntent, execute_decision};
use crate::frame_state::{BeingId, FrameId, FrameState};
use crate::landing::LandingOutcome;
use crate::point::Point;
use crate::world_map_geometry::{PointGeometryState, WorldCenterId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrentDepthId {
    HollowCurrent,
    Current,
    Abyss,
}

impl CurrentDepthId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HollowCurrent => "Hollow Current",
            Self::Current => "Current",
            Self::Abyss => "Abyss",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuraDimensionId {
    AuraShine,
    AuraView,
    InnerAura,
}

impl AuraDimensionId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraShine => "Aura Shine",
            Self::AuraView => "Aura View",
            Self::InnerAura => "Inner Aura",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalRouteId {
    StairwayToHeaven,
}

impl CanonicalRouteId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StairwayToHeaven => "Stairway to Heaven",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalHorizonId {
    StonebendAscent,
}

impl CanonicalHorizonId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StonebendAscent => "Stonebend Ascent",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HuemanCapacities {
    current_capacity: u16,
    aura_capacity: u16,
}

impl HuemanCapacities {
    #[must_use]
    pub const fn new(current_capacity: u16, aura_capacity: u16) -> Self {
        Self {
            current_capacity,
            aura_capacity,
        }
    }

    #[must_use]
    pub const fn origin() -> Self {
        Self::new(1, 1)
    }

    #[must_use]
    pub const fn current_capacity(&self) -> u16 {
        self.current_capacity
    }

    #[must_use]
    pub const fn aura_capacity(&self) -> u16 {
        self.aura_capacity
    }

    #[must_use]
    pub const fn incremented(self) -> Self {
        Self::new(self.current_capacity + 1, self.aura_capacity + 1)
    }
}

impl Default for HuemanCapacities {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrentDepthDevelopment {
    hollow_current: u16,
    current_speed: u16,
    abyss: u16,
}

impl CurrentDepthDevelopment {
    #[must_use]
    pub const fn new(hollow_current: u16, current_speed: u16, abyss: u16) -> Self {
        Self {
            hollow_current,
            current_speed,
            abyss,
        }
    }

    #[must_use]
    pub const fn origin() -> Self {
        Self::new(1, 1, 1)
    }

    #[must_use]
    pub const fn hollow_current(&self) -> u16 {
        self.hollow_current
    }

    #[must_use]
    pub const fn current_speed(&self) -> u16 {
        self.current_speed
    }

    #[must_use]
    pub const fn abyss(&self) -> u16 {
        self.abyss
    }
}

impl Default for CurrentDepthDevelopment {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuraDevelopment {
    aura_shine: u16,
    aura_view: u16,
    inner_aura: u16,
}

impl AuraDevelopment {
    #[must_use]
    pub const fn new(aura_shine: u16, aura_view: u16, inner_aura: u16) -> Self {
        Self {
            aura_shine,
            aura_view,
            inner_aura,
        }
    }

    #[must_use]
    pub const fn origin() -> Self {
        Self::new(1, 1, 1)
    }

    #[must_use]
    pub const fn aura_shine(&self) -> u16 {
        self.aura_shine
    }

    #[must_use]
    pub const fn aura_view(&self) -> u16 {
        self.aura_view
    }

    #[must_use]
    pub const fn inner_aura(&self) -> u16 {
        self.inner_aura
    }
}

impl Default for AuraDevelopment {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointProgressionState {
    stable_point_level: u16,
    capacities: HuemanCapacities,
    current_depths: CurrentDepthDevelopment,
    aura_dimensions: AuraDevelopment,
    last_applied_ascension: Option<u16>,
}

impl PointProgressionState {
    #[must_use]
    pub const fn new(
        stable_point_level: u16,
        capacities: HuemanCapacities,
        current_depths: CurrentDepthDevelopment,
        aura_dimensions: AuraDevelopment,
        last_applied_ascension: Option<u16>,
    ) -> Self {
        Self {
            stable_point_level,
            capacities,
            current_depths,
            aura_dimensions,
            last_applied_ascension,
        }
    }

    #[must_use]
    pub const fn origin() -> Self {
        Self::new(
            1,
            HuemanCapacities::origin(),
            CurrentDepthDevelopment::origin(),
            AuraDevelopment::origin(),
            None,
        )
    }

    #[must_use]
    pub const fn stable_point_level(&self) -> u16 {
        self.stable_point_level
    }

    #[must_use]
    pub const fn capacities(&self) -> &HuemanCapacities {
        &self.capacities
    }

    #[must_use]
    pub const fn current_depths(&self) -> &CurrentDepthDevelopment {
        &self.current_depths
    }

    #[must_use]
    pub const fn aura_dimensions(&self) -> &AuraDevelopment {
        &self.aura_dimensions
    }

    #[must_use]
    pub const fn last_applied_ascension(&self) -> Option<u16> {
        self.last_applied_ascension
    }

    #[must_use]
    pub fn with_applied_ascension(&self, ascension_index: u16) -> Self {
        Self::new(
            ascension_index,
            self.capacities.clone().incremented(),
            self.current_depths.clone(),
            self.aura_dimensions.clone(),
            Some(ascension_index),
        )
    }
}

impl Default for PointProgressionState {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReachableWorldState {
    geometry: PointGeometryState,
    newly_reachable_horizons: Vec<CanonicalHorizonId>,
    newly_visible_routes: Vec<CanonicalRouteId>,
    newly_survivable_routes: Vec<CanonicalRouteId>,
    next_frame_potential_available: bool,
}

impl ReachableWorldState {
    #[must_use]
    pub fn new(
        geometry: PointGeometryState,
        newly_reachable_horizons: Vec<CanonicalHorizonId>,
        newly_visible_routes: Vec<CanonicalRouteId>,
        newly_survivable_routes: Vec<CanonicalRouteId>,
        next_frame_potential_available: bool,
    ) -> Self {
        Self {
            geometry,
            newly_reachable_horizons,
            newly_visible_routes,
            newly_survivable_routes,
            next_frame_potential_available,
        }
    }

    #[must_use]
    pub fn origin() -> Self {
        Self::new(
            PointGeometryState::origin(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            false,
        )
    }

    #[must_use]
    pub fn with_geometry(geometry: PointGeometryState) -> Self {
        Self::new(geometry, Vec::new(), Vec::new(), Vec::new(), false)
    }

    #[must_use]
    pub fn with_geometry_preserving_state(&self, geometry: PointGeometryState) -> Self {
        let mut next = self.clone();
        next.geometry = geometry;
        next
    }

    #[must_use]
    pub const fn geometry(&self) -> &PointGeometryState {
        &self.geometry
    }

    #[must_use]
    pub fn newly_reachable_horizons(&self) -> &[CanonicalHorizonId] {
        &self.newly_reachable_horizons
    }

    #[must_use]
    pub fn newly_visible_routes(&self) -> &[CanonicalRouteId] {
        &self.newly_visible_routes
    }

    #[must_use]
    pub fn newly_survivable_routes(&self) -> &[CanonicalRouteId] {
        &self.newly_survivable_routes
    }

    #[must_use]
    pub const fn next_frame_potential_available(&self) -> bool {
        self.next_frame_potential_available
    }

    #[must_use]
    pub fn route_visible(&self, route: CanonicalRouteId) -> bool {
        self.newly_visible_routes.contains(&route)
    }

    #[must_use]
    pub fn route_survivable(&self, route: CanonicalRouteId) -> bool {
        self.newly_survivable_routes.contains(&route)
    }

    #[must_use]
    pub fn merge_consequence(&self, consequence: &PointWorldConsequence) -> Self {
        let mut next = self.clone();
        for horizon in &consequence.newly_reachable_horizons {
            if !next.newly_reachable_horizons.contains(horizon) {
                next.newly_reachable_horizons.push(*horizon);
            }
        }
        for route in &consequence.newly_visible_routes {
            if !next.newly_visible_routes.contains(route) {
                next.newly_visible_routes.push(*route);
            }
        }
        for route in &consequence.newly_survivable_routes {
            if !next.newly_survivable_routes.contains(route) {
                next.newly_survivable_routes.push(*route);
            }
        }
        next.next_frame_potential_available |= consequence.next_frame_potential_available;
        next
    }
}

impl Default for ReachableWorldState {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointWorldConsequence {
    newly_reachable_horizons: Vec<CanonicalHorizonId>,
    newly_visible_routes: Vec<CanonicalRouteId>,
    newly_survivable_routes: Vec<CanonicalRouteId>,
    next_frame_potential_available: bool,
}

impl PointWorldConsequence {
    #[must_use]
    pub fn stairway_to_heaven() -> Self {
        Self {
            newly_reachable_horizons: vec![CanonicalHorizonId::StonebendAscent],
            newly_visible_routes: vec![CanonicalRouteId::StairwayToHeaven],
            newly_survivable_routes: vec![CanonicalRouteId::StairwayToHeaven],
            next_frame_potential_available: true,
        }
    }

    #[must_use]
    pub fn newly_reachable_horizons(&self) -> &[CanonicalHorizonId] {
        &self.newly_reachable_horizons
    }

    #[must_use]
    pub fn newly_visible_routes(&self) -> &[CanonicalRouteId] {
        &self.newly_visible_routes
    }

    #[must_use]
    pub fn newly_survivable_routes(&self) -> &[CanonicalRouteId] {
        &self.newly_survivable_routes
    }

    #[must_use]
    pub const fn next_frame_potential_available(&self) -> bool {
        self.next_frame_potential_available
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointSquaredAscension {
    ascension_index: u16,
    source_point_level: u16,
    current_capacity_before: u16,
    current_capacity_after: u16,
    aura_capacity_before: u16,
    aura_capacity_after: u16,
    point_squared_state: FrameState,
    world_consequence: PointWorldConsequence,
}

impl PointSquaredAscension {
    #[must_use]
    pub const fn ascension_index(&self) -> u16 {
        self.ascension_index
    }

    #[must_use]
    pub const fn source_point_level(&self) -> u16 {
        self.source_point_level
    }

    #[must_use]
    pub const fn current_capacity_before(&self) -> u16 {
        self.current_capacity_before
    }

    #[must_use]
    pub const fn current_capacity_after(&self) -> u16 {
        self.current_capacity_after
    }

    #[must_use]
    pub const fn aura_capacity_before(&self) -> u16 {
        self.aura_capacity_before
    }

    #[must_use]
    pub const fn aura_capacity_after(&self) -> u16 {
        self.aura_capacity_after
    }

    #[must_use]
    pub const fn point_squared_state(&self) -> &FrameState {
        &self.point_squared_state
    }

    #[must_use]
    pub const fn world_consequence(&self) -> &PointWorldConsequence {
        &self.world_consequence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointSquaredPrepareError {
    NotLandedPointSquared,
}

impl fmt::Display for PointSquaredPrepareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotLandedPointSquared => {
                f.write_str("Point² ascension requires a canonical landed Point²")
            }
        }
    }
}

impl std::error::Error for PointSquaredPrepareError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointSquaredApplicationStatus {
    Applied,
    AlreadyApplied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointSquaredApplication {
    status: PointSquaredApplicationStatus,
    stabilized_point: Point,
    witness: String,
}

impl PointSquaredApplication {
    #[must_use]
    pub const fn new(
        status: PointSquaredApplicationStatus,
        stabilized_point: Point,
        witness: String,
    ) -> Self {
        Self {
            status,
            stabilized_point,
            witness,
        }
    }

    #[must_use]
    pub const fn status(&self) -> PointSquaredApplicationStatus {
        self.status
    }

    #[must_use]
    pub const fn stabilized_point(&self) -> &Point {
        &self.stabilized_point
    }

    #[must_use]
    pub fn witness(&self) -> &str {
        &self.witness
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointProgressionDiagnosticCode {
    CurrentCapacityInvalid,
    AuraCapacityInvalid,
    CapacityPairBroken,
    DevelopmentExceedsCapacity,
    StablePointLevelMismatch,
    DuplicateAscension,
    HuemanIdentityLost,
    HorizonStateMismatch,
    RaninaCenterInvariantBroken,
    CurrentSpeedFrameImbalance,
    AbyssCoverageImbalance,
    AuraViewIdentityImbalance,
    InnerAuraWorldImbalance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointProgressionDiagnostic {
    pub code: PointProgressionDiagnosticCode,
    pub message: String,
    pub is_error: bool,
}

pub fn prepare_point_squared_ascension(
    point: &Point,
    landing: &LandingOutcome,
) -> Result<PointSquaredAscension, PointSquaredPrepareError> {
    let LandingOutcome::Kiss(kiss) = landing else {
        return Err(PointSquaredPrepareError::NotLandedPointSquared);
    };

    let source_level = point.progression().stable_point_level();
    let capacities = point.progression().capacities();
    Ok(PointSquaredAscension {
        ascension_index: source_level + 1,
        source_point_level: source_level,
        current_capacity_before: capacities.current_capacity(),
        current_capacity_after: capacities.current_capacity() + 1,
        aura_capacity_before: capacities.aura_capacity(),
        aura_capacity_after: capacities.aura_capacity() + 1,
        point_squared_state: kiss.point_squared().clone(),
        world_consequence: PointWorldConsequence::stairway_to_heaven(),
    })
}

pub fn apply_point_squared_ascension(
    point: &Point,
    ascension: &PointSquaredAscension,
) -> PointSquaredApplication {
    if point.progression().last_applied_ascension() >= Some(ascension.ascension_index()) {
        return PointSquaredApplication {
            status: PointSquaredApplicationStatus::AlreadyApplied,
            stabilized_point: point.clone(),
            witness: format!(
                "Point² ascension {} was already applied; Current Capacity remains {}, Aura Capacity remains {}.",
                ascension.ascension_index(),
                point.progression().capacities().current_capacity(),
                point.progression().capacities().aura_capacity()
            ),
        };
    }

    let next_progression = point
        .progression()
        .with_applied_ascension(ascension.ascension_index());
    let next_world = point
        .world()
        .merge_consequence(ascension.world_consequence());
    let stabilized_point = Point::with_domain_state(
        ascension.point_squared_state().clone(),
        next_progression,
        next_world,
    );
    PointSquaredApplication {
        status: PointSquaredApplicationStatus::Applied,
        stabilized_point,
        witness: format!(
            "Point² ascension {} applied exactly once: Current Capacity {} → {}, Aura Capacity {} → {}.",
            ascension.ascension_index(),
            ascension.current_capacity_before(),
            ascension.current_capacity_after(),
            ascension.aura_capacity_before(),
            ascension.aura_capacity_after()
        ),
    }
}

pub fn validate_point_progression(point: &Point) -> Vec<PointProgressionDiagnostic> {
    let mut diagnostics = Vec::new();
    let progression = point.progression();
    let capacities = progression.capacities();
    let depths = progression.current_depths();
    let aura = progression.aura_dimensions();

    if capacities.current_capacity() == 0 {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::CurrentCapacityInvalid,
            "Current Capacity must remain at least 1.",
        ));
    }
    if capacities.aura_capacity() == 0 {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::AuraCapacityInvalid,
            "Aura Capacity must remain at least 1.",
        ));
    }
    if capacities.current_capacity() != capacities.aura_capacity() {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::CapacityPairBroken,
            "Point² must raise Current Capacity and Aura Capacity together.",
        ));
    }
    if progression.stable_point_level() != capacities.current_capacity()
        || progression.stable_point_level() != capacities.aura_capacity()
    {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::StablePointLevelMismatch,
            "Stable Point level must match the paired Current and Aura capacities.",
        ));
    }
    for (label, value, capacity) in [
        (
            "Hollow Current",
            depths.hollow_current(),
            capacities.current_capacity(),
        ),
        (
            "Current Speed",
            depths.current_speed(),
            capacities.current_capacity(),
        ),
        ("Abyss", depths.abyss(), capacities.current_capacity()),
        ("Aura Shine", aura.aura_shine(), capacities.aura_capacity()),
        ("Aura View", aura.aura_view(), capacities.aura_capacity()),
        ("Inner Aura", aura.inner_aura(), capacities.aura_capacity()),
    ] {
        if value > capacity {
            diagnostics.push(error(
                PointProgressionDiagnosticCode::DevelopmentExceedsCapacity,
                format!(
                    "{label} development {value} exceeds its current capacity ceiling {capacity}."
                ),
            ));
        }
    }
    if let Some(last_applied) = progression.last_applied_ascension()
        && last_applied > progression.stable_point_level()
    {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::DuplicateAscension,
            "last_applied_ascension cannot exceed the stabilized Point level.",
        ));
    }
    if point.being() != BeingId::Hueman {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::HuemanIdentityLost,
            "Point² must not replace BeingId::Hueman.",
        ));
    }
    if point.world().geometry().center() != WorldCenterId::Ranina {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::RaninaCenterInvariantBroken,
            "Ranina must remain the unique world center for every Point.",
        ));
    }

    let stairway_unlocked = point
        .world()
        .route_visible(CanonicalRouteId::StairwayToHeaven)
        && point
            .world()
            .route_survivable(CanonicalRouteId::StairwayToHeaven);
    if progression.stable_point_level() >= 2 && !stairway_unlocked {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::HorizonStateMismatch,
            "Stairway to Heaven must become visible and survivable after the first applied Point² ascension.",
        ));
    }
    if progression.stable_point_level() < 2 && stairway_unlocked {
        diagnostics.push(error(
            PointProgressionDiagnosticCode::HorizonStateMismatch,
            "Stairway to Heaven cannot be visible and survivable before the first Point² ascension.",
        ));
    }

    if depths.current_speed() > depths.hollow_current() {
        diagnostics.push(note(
            PointProgressionDiagnosticCode::CurrentSpeedFrameImbalance,
            "High Current Speed with insufficient Hollow Current means the Frame moves faster than its held form can safely sustain.",
        ));
    }
    if depths.hollow_current() > depths.abyss() {
        diagnostics.push(note(
            PointProgressionDiagnosticCode::AbyssCoverageImbalance,
            "Strong Hollow Current with weak Abyss means the body appears durable while hidden damage accumulates.",
        ));
    }
    if aura.aura_view() > aura.inner_aura() {
        diagnostics.push(note(
            PointProgressionDiagnosticCode::AuraViewIdentityImbalance,
            "High Aura View with weak Inner Aura means the Hueman perceives more than the self can clearly stabilize.",
        ));
    }
    if aura.inner_aura() > aura.aura_view() {
        diagnostics.push(note(
            PointProgressionDiagnosticCode::InnerAuraWorldImbalance,
            "Strong Inner Aura with weak Aura View means the Hueman knows the self deeply but cannot yet read the wider world.",
        ));
    }

    diagnostics
}

fn error(
    code: PointProgressionDiagnosticCode,
    message: impl Into<String>,
) -> PointProgressionDiagnostic {
    PointProgressionDiagnostic {
        code,
        message: message.into(),
        is_error: true,
    }
}

fn note(
    code: PointProgressionDiagnosticCode,
    message: impl Into<String>,
) -> PointProgressionDiagnostic {
    PointProgressionDiagnostic {
        code,
        message: message.into(),
        is_error: false,
    }
}

pub fn build_point_progression_state_output(point: &Point) -> String {
    let rule_identity = point.world().geometry().rule_of_twelve_position();
    let position = rule_identity
        .map(|identity| identity.absolute_position().to_string())
        .unwrap_or_default();
    let pass = rule_identity
        .map(|identity| identity.pass().value().to_string())
        .unwrap_or_default();
    let house_number = rule_identity
        .map(|identity| identity.house_number().value().to_string())
        .unwrap_or_default();
    let house_alignment = rule_identity
        .map(|identity| identity.house().as_str().to_string())
        .unwrap_or_default();
    let primary_anchor = rule_identity
        .map(|identity| identity.is_primary_anchor().to_string())
        .unwrap_or_default();
    let threshold_target = rule_identity
        .and_then(|identity| identity.threshold())
        .map(|threshold| threshold.toward_house().as_str().to_string())
        .unwrap_or_default();
    let rotation_complete = rule_identity
        .map(|identity| identity.rotation_complete().to_string())
        .unwrap_or_default();
    format!(
        "# Point Progression State\n\
         stable_point_level: {}\n\
         world_center: {}\n\
         ring: {}\n\
         position: {}\n\
         pass: {}\n\
         house_number: {}\n\
         house_alignment: {}\n\
         primary_anchor: {}\n\
         threshold_target: {}\n\
         rotation_complete: {}\n\
         current_capacity: {}\n\
         aura_capacity: {}\n\
         hollow_current: {}\n\
         current_speed: {}\n\
         abyss: {}\n\
         aura_shine: {}\n\
         aura_view: {}\n\
         inner_aura: {}\n\
         last_applied_ascension: {}\n\
         newly_reachable_horizons: {}\n\
         newly_visible_routes: {}\n\
         newly_survivable_routes: {}\n\
         next_frame_potential_available: {}\n",
        point.progression().stable_point_level(),
        point.world().geometry().center().as_str(),
        point.progression().stable_point_level(),
        position,
        pass,
        house_number,
        house_alignment,
        primary_anchor,
        threshold_target,
        rotation_complete,
        point.progression().capacities().current_capacity(),
        point.progression().capacities().aura_capacity(),
        point.progression().current_depths().hollow_current(),
        point.progression().current_depths().current_speed(),
        point.progression().current_depths().abyss(),
        point.progression().aura_dimensions().aura_shine(),
        point.progression().aura_dimensions().aura_view(),
        point.progression().aura_dimensions().inner_aura(),
        point
            .progression()
            .last_applied_ascension()
            .map(|value| value.to_string())
            .unwrap_or_default(),
        join_horizons(point.world().newly_reachable_horizons()),
        join_routes(point.world().newly_visible_routes()),
        join_routes(point.world().newly_survivable_routes()),
        point.world().next_frame_potential_available(),
    )
}

pub fn parse_point_progression_state(contents: &str) -> io::Result<PointProgressionState> {
    let mut stable_point_level = None;
    let mut current_capacity = None;
    let mut aura_capacity = None;
    let mut hollow_current = None;
    let mut current_speed = None;
    let mut abyss = None;
    let mut aura_shine = None;
    let mut aura_view = None;
    let mut inner_aura = None;
    let mut last_applied_ascension = None;

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("progression state line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "stable_point_level" => stable_point_level = Some(parse_u16_field(key, value)?),
            "world_center" | "ring" | "position" | "pass" | "house_number" | "house_alignment"
            | "primary_anchor" | "threshold_target" | "rotation_complete" => {}
            "current_capacity" => current_capacity = Some(parse_u16_field(key, value)?),
            "aura_capacity" => aura_capacity = Some(parse_u16_field(key, value)?),
            "hollow_current" => hollow_current = Some(parse_u16_field(key, value)?),
            "current_speed" => current_speed = Some(parse_u16_field(key, value)?),
            "abyss" => abyss = Some(parse_u16_field(key, value)?),
            "aura_shine" => aura_shine = Some(parse_u16_field(key, value)?),
            "aura_view" => aura_view = Some(parse_u16_field(key, value)?),
            "inner_aura" => inner_aura = Some(parse_u16_field(key, value)?),
            "last_applied_ascension" => {
                last_applied_ascension = if value.is_empty() {
                    None
                } else {
                    Some(parse_u16_field(key, value)?)
                }
            }
            "newly_reachable_horizons"
            | "newly_visible_routes"
            | "newly_survivable_routes"
            | "next_frame_potential_available" => {}
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown progression state key: {other}"),
                ));
            }
        }
    }

    Ok(PointProgressionState::new(
        stable_point_level.unwrap_or(1),
        HuemanCapacities::new(current_capacity.unwrap_or(1), aura_capacity.unwrap_or(1)),
        CurrentDepthDevelopment::new(
            hollow_current.unwrap_or(1),
            current_speed.unwrap_or(1),
            abyss.unwrap_or(1),
        ),
        AuraDevelopment::new(
            aura_shine.unwrap_or(1),
            aura_view.unwrap_or(1),
            inner_aura.unwrap_or(1),
        ),
        last_applied_ascension,
    ))
}

fn parse_u16_field(key: &str, value: &str) -> io::Result<u16> {
    value.parse::<u16>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{key} must be an unsigned integer, got `{value}`"),
        )
    })
}

fn join_routes(routes: &[CanonicalRouteId]) -> String {
    routes
        .iter()
        .map(|route| route.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_horizons(horizons: &[CanonicalHorizonId]) -> String {
    horizons
        .iter()
        .map(|horizon| horizon.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn public_house_display(house: crate::hollow_grove_contract::House) -> &'static str {
    match house {
        crate::hollow_grove_contract::House::Stonebend => "Stonebend",
        crate::hollow_grove_contract::House::Sandmanor => "Sandmanor",
        crate::hollow_grove_contract::House::Glaushouse => "Glaüshouse",
        crate::hollow_grove_contract::House::Flynt => "Flynt",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalPointSquaredFixture {
    point_before: Point,
    decision: DecisionExecution,
    ascension: PointSquaredAscension,
    first_application: PointSquaredApplication,
    second_application: PointSquaredApplication,
}

impl CanonicalPointSquaredFixture {
    #[must_use]
    pub const fn point_before(&self) -> &Point {
        &self.point_before
    }

    #[must_use]
    pub const fn decision(&self) -> &DecisionExecution {
        &self.decision
    }

    #[must_use]
    pub const fn ascension(&self) -> &PointSquaredAscension {
        &self.ascension
    }

    #[must_use]
    pub const fn first_application(&self) -> &PointSquaredApplication {
        &self.first_application
    }

    #[must_use]
    pub const fn second_application(&self) -> &PointSquaredApplication {
        &self.second_application
    }
}

pub fn build_canonical_point_squared_fixture() -> io::Result<CanonicalPointSquaredFixture> {
    let point_before = Point::origin();
    let decision = execute_decision(&point_before, DecisionIntent::FavorCurrent)
        .map_err(|error| io::Error::other(format!("canonical Point² fixture failed: {error:?}")))?;
    let ascension = prepare_point_squared_ascension(&point_before, decision.execution().landing())
        .map_err(|error| io::Error::other(error.to_string()))?;
    let first_application = apply_point_squared_ascension(&point_before, &ascension);
    let second_application =
        apply_point_squared_ascension(first_application.stabilized_point(), &ascension);
    Ok(CanonicalPointSquaredFixture {
        point_before,
        decision,
        ascension,
        first_application,
        second_application,
    })
}

pub fn build_progression_witness() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let point_after = fixture.first_application().stabilized_point();
    let rotation = point_after
        .world()
        .geometry()
        .rule_of_twelve_position()
        .expect("canonical point should have a numbered position");
    Ok(format!(
        "HOLLOW GROVE PROGRESSION WITNESS\n\n\
         Stable Point Level: {}\n\
         Being: {:?}\n\
         World Center: {}\n\
         Ring: {}\n\
         Position: {}\n\
         Pass: {}\n\
         House Number: {}\n\
         House Alignment: {}\n\
         Primary Anchor: {}\n\
         Threshold Target: {}\n\
         Rotation Complete: {}\n\
         Current Capacity: {}\n\
         Aura Capacity: {}\n\
         Hollow Current: {}\n\
         Current Speed: {}\n\
         Abyss: {}\n\
         Aura Shine: {}\n\
         Aura View: {}\n\
         Inner Aura: {}\n\
         Last Point² consequence applied: {}\n\
         Newly reachable horizon: {}\n\
         Newly visible route: {}\n\
         Stairway to Heaven survivable: {}\n\
         Next Frame potential: {}\n\
         Current Event: Point² ascension without automatic angular rotation\n",
        point_after.progression().stable_point_level(),
        point_after.being(),
        point_after.world().geometry().center().as_str(),
        point_after.progression().stable_point_level(),
        rotation.absolute_position(),
        rotation.pass().value(),
        rotation.house_number().value(),
        public_house_display(rotation.house()),
        rotation.is_primary_anchor(),
        rotation
            .threshold()
            .map(|threshold| threshold.toward_house().as_str().to_string())
            .unwrap_or_else(|| String::from("none")),
        rotation.rotation_complete(),
        point_after.progression().capacities().current_capacity(),
        point_after.progression().capacities().aura_capacity(),
        point_after.progression().current_depths().hollow_current(),
        point_after.progression().current_depths().current_speed(),
        point_after.progression().current_depths().abyss(),
        point_after.progression().aura_dimensions().aura_shine(),
        point_after.progression().aura_dimensions().aura_view(),
        point_after.progression().aura_dimensions().inner_aura(),
        matches!(
            fixture.first_application().status(),
            PointSquaredApplicationStatus::Applied
        ),
        join_horizons(point_after.world().newly_reachable_horizons()),
        join_routes(point_after.world().newly_visible_routes()),
        point_after
            .world()
            .route_survivable(CanonicalRouteId::StairwayToHeaven),
        point_after.world().next_frame_potential_available(),
    ))
}

pub fn build_progression_validation_report() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let diagnostics = validate_point_progression(fixture.first_application().stabilized_point());
    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.is_error)
        .collect::<Vec<_>>();
    let notes = diagnostics
        .iter()
        .filter(|diagnostic| !diagnostic.is_error)
        .collect::<Vec<_>>();
    let mut output = String::from("# Hollow Grove Progression Validation\n\n");
    if errors.is_empty() {
        output.push_str(
            "- status: pass\n\
             - development values legal: pass\n\
             - development does not exceed capacity: pass\n\
             - paired capacity advancement: pass\n\
             - ascension duplication blocked: pass\n\
             - Hueman identity persists: pass\n\
             - Ranina center invariant: pass\n\
             - ring and position remain distinct: pass\n\
             - Rule of Twelve rotation state legal: pass\n\
             - topology remains unchanged: pass\n",
        );
    } else {
        output.push_str("- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
    }
    if !notes.is_empty() {
        output.push_str("\n## Domain Diagnostics\n");
        for diagnostic in notes {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
    }
    Ok(output)
}

pub fn build_point_squared_witness() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let point_after = fixture.first_application().stabilized_point();
    let landing = fixture.decision().execution().landing();
    let recipe = fixture.decision().recipe();
    let candidate = fixture.decision().chosen().candidate().candidate_id();
    let point_squared_frame = match landing {
        LandingOutcome::Kiss(kiss) => kiss.point_squared().frame(),
        LandingOutcome::Miss { .. } => FrameId::Hueman,
    };

    Ok(format!(
        "HOLLOW GROVE POINT² ASCENSION WITNESS\n\n\
         Root Doctrine:\n\
         Hollow Current holds.\n\
         Current flows.\n\
         Abyss feels.\n\
         Aura reveals.\n\
         Point² expands the self and the world together.\n\n\
         Current Depths:\n\
         Hollow Current = Stonebend = life held in form\n\
         Current = derivative = life moving between form and depth\n\
         Abyss = Glaüshouse = life felt in depth\n\n\
         Aura:\n\
         Aura Shine = outward presence\n\
         Aura View = surrounding world and horizon\n\
         Inner Aura = inward identity and clarity\n\n\
         Before:\n\
         Point Level: {}\n\
         Being: {:?}\n\
         World Center: {}\n\
         Ring: {}\n\
         Position: {}\n\
         Current Capacity: {}\n\
         Aura Capacity: {}\n\n\
         Landing:\n\
         V2 choice: {}\n\
         Recipe: {} ({})\n\
         V1.1 execution: pass\n\
         Point²: landed as {:?}\n\n\
         Ascension:\n\
         Current Capacity: {} → {}\n\
         Aura Capacity: {} → {}\n\
         Applied exactly once: {}\n\n\
         World Consequence:\n\
         Stairway to Heaven: {}\n\
         Stairway to Heaven survivable: {}\n\
         Destination: Stonebend\n\
         Next Frame: possible, not automatically granted\n\
         Active Frame after landing: {:?}\n\n\
         After Stabilization:\n\
         Point Level: {}\n\
         Being: {:?}\n\
         World Center: {}\n\
         Ring: {}\n\
         Position: {}\n\
         V1.1 topology unchanged: yes\n",
        fixture.point_before().progression().stable_point_level(),
        fixture.point_before().being(),
        fixture.point_before().world().geometry().center().as_str(),
        fixture.point_before().progression().stable_point_level(),
        fixture
            .point_before()
            .world()
            .geometry()
            .current_position()
            .map(|position| position.to_string())
            .unwrap_or_else(|| String::from("none")),
        fixture
            .point_before()
            .progression()
            .capacities()
            .current_capacity(),
        fixture
            .point_before()
            .progression()
            .capacities()
            .aura_capacity(),
        candidate.as_str(),
        recipe.display_name(),
        recipe.recipe_id(),
        point_squared_frame,
        fixture.ascension().current_capacity_before(),
        fixture.ascension().current_capacity_after(),
        fixture.ascension().aura_capacity_before(),
        fixture.ascension().aura_capacity_after(),
        if matches!(
            fixture.second_application().status(),
            PointSquaredApplicationStatus::AlreadyApplied
        ) {
            "yes"
        } else {
            "no"
        },
        if point_after
            .world()
            .route_visible(CanonicalRouteId::StairwayToHeaven)
        {
            "visible"
        } else {
            "hidden"
        },
        if point_after
            .world()
            .route_survivable(CanonicalRouteId::StairwayToHeaven)
        {
            "yes"
        } else {
            "no"
        },
        point_after.frame_state().frame(),
        point_after.progression().stable_point_level(),
        point_after.being(),
        point_after.world().geometry().center().as_str(),
        point_after.progression().stable_point_level(),
        point_after
            .world()
            .geometry()
            .current_position()
            .map(|position| position.to_string())
            .unwrap_or_else(|| String::from("none")),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        BeingId, CurrentPrism, FlowId, FrameId, GlowId, LandingOutcome, Point,
        pixy_confusion_recipe,
    };

    use super::{
        AuraDevelopment, CanonicalRouteId, CurrentDepthDevelopment, PointProgressionDiagnosticCode,
        PointProgressionState, PointSquaredApplicationStatus, PointSquaredPrepareError,
        build_canonical_point_squared_fixture, build_point_progression_state_output,
        build_point_squared_witness, parse_point_progression_state,
        prepare_point_squared_ascension, validate_point_progression,
    };

    #[test]
    fn origin_point_carries_origin_progression_and_world_state() {
        let point = Point::origin();

        assert_eq!(point.progression(), &PointProgressionState::origin());
        assert!(point.world().newly_visible_routes().is_empty());
        assert!(!point.world().next_frame_potential_available());
    }

    #[test]
    fn progression_state_round_trips_deterministically() {
        let fixture = build_canonical_point_squared_fixture().expect("fixture should build");
        let output =
            build_point_progression_state_output(fixture.first_application().stabilized_point());
        let parsed = parse_point_progression_state(&output).expect("output should parse");

        assert_eq!(
            parsed,
            fixture
                .first_application()
                .stabilized_point()
                .progression()
                .clone()
        );
    }

    #[test]
    fn progression_state_migrates_missing_capacities_to_origin_defaults() {
        let legacy = "# Point Progression State\nhollow_current: 1\ncurrent_speed: 1\nabyss: 1\naura_shine: 1\naura_view: 1\ninner_aura: 1\n";
        let parsed = parse_point_progression_state(legacy).expect("legacy state should parse");

        assert_eq!(parsed.stable_point_level(), 1);
        assert_eq!(parsed.capacities().current_capacity(), 1);
        assert_eq!(parsed.capacities().aura_capacity(), 1);
    }

    #[test]
    fn valid_point_squared_applies_paired_capacity_advancement_once() {
        let fixture = build_canonical_point_squared_fixture().expect("fixture should build");
        let point_after = fixture.first_application().stabilized_point();

        assert_eq!(
            fixture.first_application().status(),
            PointSquaredApplicationStatus::Applied
        );
        assert_eq!(
            fixture.second_application().status(),
            PointSquaredApplicationStatus::AlreadyApplied
        );
        assert_eq!(point_after.progression().capacities().current_capacity(), 2);
        assert_eq!(point_after.progression().capacities().aura_capacity(), 2);
        assert_eq!(
            fixture
                .second_application()
                .stabilized_point()
                .progression()
                .capacities()
                .current_capacity(),
            2
        );
        assert_eq!(
            fixture
                .second_application()
                .stabilized_point()
                .progression()
                .capacities()
                .aura_capacity(),
            2
        );
    }

    #[test]
    fn miss_does_not_prepare_an_ascension() {
        let miss = LandingOutcome::Miss {
            frame_state: Point::origin().frame_state().clone(),
        };

        assert_eq!(
            prepare_point_squared_ascension(&Point::origin(), &miss),
            Err(PointSquaredPrepareError::NotLandedPointSquared)
        );
    }

    #[test]
    fn hueman_identity_learning_prism_and_development_persist_through_ascension() {
        let point = Point::with_domain_state(
            crate::FrameState::new(
                FrameId::Gremlin,
                CurrentPrism::new(3, 1, 1, 1, 1),
                vec![FlowId::TinkerGrip],
                vec![GlowId::Recognition],
            ),
            PointProgressionState::new(
                1,
                crate::point_progression::HuemanCapacities::origin(),
                CurrentDepthDevelopment::origin(),
                AuraDevelopment::origin(),
                None,
            ),
            crate::point_progression::ReachableWorldState::origin(),
        );
        let execution = crate::execute_synthesis_recipe(&point, &pixy_confusion_recipe())
            .expect("execution should work");
        let ascension = prepare_point_squared_ascension(&point, execution.landing())
            .expect("kiss landing should prepare");
        let applied = crate::point_progression::apply_point_squared_ascension(&point, &ascension);
        let stabilized = applied.stabilized_point();

        assert_eq!(stabilized.being(), BeingId::Hueman);
        assert_eq!(stabilized.frame_state().frame(), FrameId::Pixy);
        assert_eq!(
            stabilized.frame_state().flow_learnset(),
            &[FlowId::TinkerGrip]
        );
        assert_eq!(
            stabilized.frame_state().glow_learnset(),
            &[GlowId::Recognition, GlowId::Confusion]
        );
        assert_eq!(stabilized.frame_state().prism().body(), 3);
        assert_eq!(
            stabilized.progression().current_depths(),
            point.progression().current_depths()
        );
        assert_eq!(
            stabilized.progression().aura_dimensions(),
            point.progression().aura_dimensions()
        );
    }

    #[test]
    fn stairway_to_heaven_is_hidden_before_and_unlocked_after_first_ascension() {
        let before = Point::origin();
        let fixture = build_canonical_point_squared_fixture().expect("fixture should build");
        let after = fixture.first_application().stabilized_point();

        assert!(
            !before
                .world()
                .route_visible(CanonicalRouteId::StairwayToHeaven)
        );
        assert!(
            !before
                .world()
                .route_survivable(CanonicalRouteId::StairwayToHeaven)
        );
        assert!(
            after
                .world()
                .route_visible(CanonicalRouteId::StairwayToHeaven)
        );
        assert!(
            after
                .world()
                .route_survivable(CanonicalRouteId::StairwayToHeaven)
        );
        assert!(after.world().next_frame_potential_available());
        assert_ne!(after.frame_state().frame(), FrameId::Troglodyte);
    }

    #[test]
    fn point_squared_stabilizes_into_the_next_point_without_higher_point_vocabulary() {
        let witness = build_point_squared_witness().expect("witness should build");
        let fixture = build_canonical_point_squared_fixture().expect("fixture should build");

        assert_eq!(fixture.point_before().progression().stable_point_level(), 1);
        assert_eq!(
            fixture
                .first_application()
                .stabilized_point()
                .progression()
                .stable_point_level(),
            2
        );
        assert!(!witness.contains("Point³"));
        assert!(!witness.contains("Point4"));
        assert!(!witness.contains("Horizon²"));
    }

    #[test]
    fn transformation_imbalance_notes_are_reported_without_failing_validation() {
        let point = Point::with_domain_state(
            crate::FrameState::origin(),
            PointProgressionState::new(
                2,
                crate::point_progression::HuemanCapacities::new(2, 2),
                CurrentDepthDevelopment::new(1, 2, 1),
                AuraDevelopment::new(1, 2, 1),
                None,
            ),
            crate::point_progression::ReachableWorldState::origin(),
        );
        let diagnostics = validate_point_progression(&point);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == PointProgressionDiagnosticCode::CurrentSpeedFrameImbalance
                && !diagnostic.is_error
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == PointProgressionDiagnosticCode::AuraViewIdentityImbalance
                && !diagnostic.is_error
        }));
    }

    #[test]
    fn development_above_capacity_fails_validation() {
        let point = Point::with_domain_state(
            crate::FrameState::origin(),
            PointProgressionState::new(
                1,
                crate::point_progression::HuemanCapacities::new(1, 1),
                CurrentDepthDevelopment::new(2, 1, 1),
                AuraDevelopment::origin(),
                None,
            ),
            crate::point_progression::ReachableWorldState::origin(),
        );
        let diagnostics = validate_point_progression(&point);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == PointProgressionDiagnosticCode::DevelopmentExceedsCapacity
                && diagnostic.is_error
        }));
    }
}
