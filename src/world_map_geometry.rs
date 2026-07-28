use std::fmt;
use std::io;

use crate::decision_engine::{DecisionExecution, DecisionIntent, execute_decision};
use crate::frame_state::{BeingId, FrameState};
use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode, House};
use crate::manager_domain::{Manager, ManagerDomain, ManagerGeometry};
use crate::point::Point;
use crate::point_progression::{
    CanonicalRouteId, PointProgressionState, PointSquaredApplication,
    PointSquaredApplicationStatus, PointSquaredAscension, ReachableWorldState,
    apply_point_squared_ascension, build_canonical_point_squared_fixture,
    prepare_point_squared_ascension,
};
use crate::{CANONICAL_WITNESS, run_kernel_cycle, symptom::Symptom};

const CANONICAL_HOUSE_GRAMMAR: [House; 4] = [
    House::Stonebend,
    House::Sandmanor,
    House::Glaushouse,
    House::Flynt,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldCenterId {
    Ranina,
}

impl WorldCenterId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ranina => "Ranina",
        }
    }

    #[must_use]
    pub const fn meaning(self) -> &'static str {
        match self {
            Self::Ranina => "frog / frog-crab / living transformation hinge",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RotationPosition(u8);

impl RotationPosition {
    pub const COUNT: u8 = 12;
    pub const DEGREES_PER_POSITION: u16 = 30;

    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        if value >= 1 && value <= Self::COUNT {
            Some(Self(value))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn one() -> Self {
        Self(1)
    }

    #[must_use]
    pub const fn six() -> Self {
        Self(6)
    }

    #[must_use]
    pub const fn seven() -> Self {
        Self(7)
    }

    #[must_use]
    pub const fn twelve() -> Self {
        Self(12)
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn next(self) -> Self {
        if self.0 == Self::COUNT {
            Self(1)
        } else {
            Self(self.0 + 1)
        }
    }

    #[must_use]
    pub const fn previous(self) -> Self {
        if self.0 == 1 {
            Self(Self::COUNT)
        } else {
            Self(self.0 - 1)
        }
    }

    #[must_use]
    pub const fn opposite(self) -> Self {
        if self.0 <= 6 {
            Self(self.0 + 6)
        } else {
            Self(self.0 - 6)
        }
    }

    #[must_use]
    pub const fn angle_degrees(self) -> u16 {
        (self.0 as u16 - 1) * Self::DEGREES_PER_POSITION
    }
}

impl fmt::Display for RotationPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for RotationPosition {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("rotation position must be in 1..=12, got {value}"),
            )
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseNumber {
    One,
    Two,
    Three,
    Four,
}

impl HouseNumber {
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            4 => Some(Self::Four),
            _ => None,
        }
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    #[must_use]
    pub const fn house(self) -> House {
        match self {
            Self::One => House::Stonebend,
            Self::Two => House::Sandmanor,
            Self::Three => House::Glaushouse,
            Self::Four => House::Flynt,
        }
    }
}

impl fmt::Display for HouseNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<u8> for HouseNumber {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("house number must be in 1..=4, got {value}"),
            )
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RotationPass {
    One,
    Two,
    Three,
}

impl RotationPass {
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            _ => None,
        }
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::One => "Pass 1",
            Self::Two => "Pass 2",
            Self::Three => "Pass 3",
        }
    }
}

impl fmt::Display for RotationPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<u8> for RotationPass {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("rotation pass must be in 1..=3, got {value}"),
            )
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorldRing(u16);

impl WorldRing {
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationalCoordinate {
    ring: WorldRing,
    position: RotationPosition,
}

impl RotationalCoordinate {
    #[must_use]
    pub const fn new(ring: WorldRing, position: RotationPosition) -> Self {
        Self { ring, position }
    }

    #[must_use]
    pub const fn ring(self) -> WorldRing {
        self.ring
    }

    #[must_use]
    pub const fn position(self) -> RotationPosition {
        self.position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointLocation {
    RaninaCenter,
    RingPosition(RotationPosition),
}

impl PointLocation {
    #[must_use]
    pub const fn center() -> Self {
        Self::RaninaCenter
    }

    #[must_use]
    pub const fn at_position(position: RotationPosition) -> Self {
        Self::RingPosition(position)
    }

    #[must_use]
    pub const fn glaushouse_pole() -> Self {
        Self::RingPosition(RotationPosition::seven())
    }

    #[must_use]
    pub const fn current_position(self) -> Option<RotationPosition> {
        match self {
            Self::RaninaCenter => None,
            Self::RingPosition(position) => Some(position),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointGeometryState {
    center: WorldCenterId,
    location: PointLocation,
}

impl PointGeometryState {
    #[must_use]
    pub const fn new(center: WorldCenterId, location: PointLocation) -> Self {
        Self { center, location }
    }

    #[must_use]
    pub const fn ranina_center() -> Self {
        Self::new(WorldCenterId::Ranina, PointLocation::RaninaCenter)
    }

    #[must_use]
    pub const fn at_position(position: RotationPosition) -> Self {
        Self::new(WorldCenterId::Ranina, PointLocation::at_position(position))
    }

    #[must_use]
    pub const fn origin() -> Self {
        Self::new(WorldCenterId::Ranina, PointLocation::glaushouse_pole())
    }

    #[must_use]
    pub const fn center(&self) -> WorldCenterId {
        self.center
    }

    #[must_use]
    pub const fn location(&self) -> PointLocation {
        self.location
    }

    #[must_use]
    pub const fn current_position(&self) -> Option<RotationPosition> {
        self.location.current_position()
    }

    #[must_use]
    pub fn coordinate_for_ring(&self, ring: u16) -> Option<RotationalCoordinate> {
        self.current_position()
            .map(|position| RotationalCoordinate::new(WorldRing::new(ring), position))
    }

    #[must_use]
    pub fn rule_of_twelve_position(&self) -> Option<RuleOfTwelvePosition> {
        self.current_position().map(position_identity)
    }
}

impl Default for PointGeometryState {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl RelativeDirection {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::North => "North",
            Self::Northeast => "Northeast",
            Self::East => "East",
            Self::Southeast => "Southeast",
            Self::South => "South",
            Self::Southwest => "Southwest",
            Self::West => "West",
            Self::Northwest => "Northwest",
        }
    }

    #[must_use]
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::Northeast => "northeast",
            Self::East => "east",
            Self::Southeast => "southeast",
            Self::South => "south",
            Self::Southwest => "southwest",
            Self::West => "west",
            Self::Northwest => "northwest",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Proximity {
    Proximal,
    Distal,
}

impl Proximity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proximal => "Proximal",
            Self::Distal => "Distal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialGeometry {
    Flat,
    Round,
    Inverted,
}

impl SpatialGeometry {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flat => "Flat",
            Self::Round => "Round",
            Self::Inverted => "Inverted",
        }
    }

    #[must_use]
    pub const fn manager_geometry(self) -> ManagerGeometry {
        match self {
            Self::Flat => ManagerGeometry::Straight,
            Self::Round => ManagerGeometry::Curved,
            Self::Inverted => ManagerGeometry::Inverted,
        }
    }

    #[must_use]
    pub const fn manager(self) -> Manager {
        match self {
            Self::Flat => Manager::Clouseau,
            Self::Round => Manager::Hal,
            Self::Inverted => Manager::Cleopatra,
        }
    }
}

impl From<ManagerGeometry> for SpatialGeometry {
    fn from(value: ManagerGeometry) -> Self {
        match value {
            ManagerGeometry::Straight => Self::Flat,
            ManagerGeometry::Curved => Self::Round,
            ManagerGeometry::Inverted => Self::Inverted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Proxy {
    anchor: House,
    direction: RelativeDirection,
    geometry: SpatialGeometry,
    proximity: Proximity,
    route: Option<CanonicalRouteId>,
    coordinate: Option<RotationalCoordinate>,
}

impl Proxy {
    #[must_use]
    pub const fn new(
        anchor: House,
        direction: RelativeDirection,
        geometry: SpatialGeometry,
        proximity: Proximity,
        route: Option<CanonicalRouteId>,
        coordinate: Option<RotationalCoordinate>,
    ) -> Self {
        Self {
            anchor,
            direction,
            geometry,
            proximity,
            route,
            coordinate,
        }
    }

    #[must_use]
    pub const fn anchor(&self) -> House {
        self.anchor
    }

    #[must_use]
    pub const fn direction(&self) -> RelativeDirection {
        self.direction
    }

    #[must_use]
    pub const fn geometry(&self) -> SpatialGeometry {
        self.geometry
    }

    #[must_use]
    pub const fn proximity(&self) -> Proximity {
        self.proximity
    }

    #[must_use]
    pub const fn route(&self) -> Option<CanonicalRouteId> {
        self.route
    }

    #[must_use]
    pub const fn coordinate(&self) -> Option<RotationalCoordinate> {
        self.coordinate
    }

    #[must_use]
    pub const fn domain(&self) -> ManagerDomain {
        ManagerDomain::Pleb
    }

    #[must_use]
    pub const fn manager(&self) -> Manager {
        Manager::Clouseau
    }

    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "{} {} {} of {}",
            self.proximity.as_str(),
            self.geometry.as_str(),
            self.direction.phrase(),
            house_display(self.anchor)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoxyRelation {
    Bond,
}

impl MoxyRelation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bond => "Bond",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Moxy {
    origin: Proxy,
    destination: Option<House>,
    relation: MoxyRelation,
    route: Option<CanonicalRouteId>,
    manager: Manager,
}

impl Moxy {
    #[must_use]
    pub const fn new(
        origin: Proxy,
        destination: Option<House>,
        relation: MoxyRelation,
        route: Option<CanonicalRouteId>,
        manager: Manager,
    ) -> Self {
        Self {
            origin,
            destination,
            relation,
            route,
            manager,
        }
    }

    #[must_use]
    pub const fn origin(&self) -> &Proxy {
        &self.origin
    }

    #[must_use]
    pub const fn destination(&self) -> Option<House> {
        self.destination
    }

    #[must_use]
    pub const fn relation(&self) -> MoxyRelation {
        self.relation
    }

    #[must_use]
    pub const fn route(&self) -> Option<CanonicalRouteId> {
        self.route
    }

    #[must_use]
    pub const fn manager(&self) -> Manager {
        self.manager
    }

    #[must_use]
    pub const fn domain(&self) -> ManagerDomain {
        ManagerDomain::Meta
    }

    #[must_use]
    pub fn render(&self) -> String {
        match (self.destination, self.route) {
            (Some(destination), Some(route)) => format!(
                "{} toward {} through {}",
                self.relation.as_str(),
                house_display(destination),
                route.as_str()
            ),
            (Some(destination), None) => {
                format!(
                    "{} toward {}",
                    self.relation.as_str(),
                    house_display(destination)
                )
            }
            _ => self.relation.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxySourceKind {
    Proxy,
    Moxy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FoxySource {
    Proxy(Proxy),
    Moxy(Moxy),
}

impl FoxySource {
    #[must_use]
    pub const fn kind(&self) -> FoxySourceKind {
        match self {
            Self::Proxy(_) => FoxySourceKind::Proxy,
            Self::Moxy(_) => FoxySourceKind::Moxy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReflectionKind {
    InvertedReturn,
}

impl ReflectionKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvertedReturn => "InvertedReturn",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Foxy {
    source: FoxySource,
    reflection_kind: ReflectionKind,
    manager: Manager,
}

impl Foxy {
    #[must_use]
    pub const fn new(
        source: FoxySource,
        reflection_kind: ReflectionKind,
        manager: Manager,
    ) -> Self {
        Self {
            source,
            reflection_kind,
            manager,
        }
    }

    #[must_use]
    pub const fn source(&self) -> &FoxySource {
        &self.source
    }

    #[must_use]
    pub const fn reflection_kind(&self) -> ReflectionKind {
        self.reflection_kind
    }

    #[must_use]
    pub const fn manager(&self) -> Manager {
        self.manager
    }

    #[must_use]
    pub const fn domain(&self) -> ManagerDomain {
        ManagerDomain::Blep
    }

    #[must_use]
    pub fn render(&self) -> String {
        match &self.source {
            FoxySource::Proxy(proxy) => format!("Inverted reflection of {}", proxy.render()),
            FoxySource::Moxy(moxy) => {
                if let Some(destination) = moxy.destination() {
                    format!(
                        "Inverted reflection of the {}-{} bond",
                        house_display(moxy.origin().anchor()),
                        house_display(destination)
                    )
                } else {
                    format!("Inverted reflection of {}", moxy.render())
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSpatialInterpretation {
    proxy: Option<Proxy>,
    moxy: Option<Moxy>,
    foxy: Option<Foxy>,
}

impl PlayerSpatialInterpretation {
    #[must_use]
    pub const fn new(proxy: Option<Proxy>, moxy: Option<Moxy>, foxy: Option<Foxy>) -> Self {
        Self { proxy, moxy, foxy }
    }

    #[must_use]
    pub const fn proxy(&self) -> Option<&Proxy> {
        self.proxy.as_ref()
    }

    #[must_use]
    pub const fn moxy(&self) -> Option<&Moxy> {
        self.moxy.as_ref()
    }

    #[must_use]
    pub const fn foxy(&self) -> Option<&Foxy> {
        self.foxy.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProxyClaim {
    pub anchor: Option<House>,
    pub direction: Option<RelativeDirection>,
    pub geometry: Option<SpatialGeometry>,
    pub proximity: Option<Proximity>,
    pub route: Option<CanonicalRouteId>,
    pub coordinate: Option<RotationalCoordinate>,
    pub domain: Option<ManagerDomain>,
    pub manager: Option<Manager>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoxyClaim {
    pub destination: Option<House>,
    pub relation: Option<MoxyRelation>,
    pub route: Option<CanonicalRouteId>,
    pub manager: Option<Manager>,
    pub domain: Option<ManagerDomain>,
    pub velocity_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoxyClaim {
    pub source_kind: Option<FoxySourceKind>,
    pub reflection_kind: Option<ReflectionKind>,
    pub manager: Option<Manager>,
    pub domain: Option<ManagerDomain>,
    pub automatically_evil: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSpatialContractInput {
    pub proxy: ProxyClaim,
    pub moxy: MoxyClaim,
    pub foxy: FoxyClaim,
    pub proxy_replaces_coordinate: bool,
    pub moxy_as_proximity: bool,
    pub proxy_as_ring: bool,
    pub automatic_point_squared_from_proxy: bool,
    pub automatic_movement_from_moxy: bool,
    pub automatic_legality_from_foxy: bool,
    pub round_proxy_forbidden: bool,
    pub coordinate_alignment_claim: Option<(RotationPosition, House)>,
}

impl Default for PlayerSpatialContractInput {
    fn default() -> Self {
        Self {
            proxy: ProxyClaim {
                anchor: Some(House::Stonebend),
                direction: Some(RelativeDirection::Northwest),
                geometry: Some(SpatialGeometry::Round),
                proximity: Some(Proximity::Distal),
                route: Some(CanonicalRouteId::StairwayToHeaven),
                coordinate: Some(RotationalCoordinate::new(
                    WorldRing::new(2),
                    RotationPosition::twelve(),
                )),
                domain: Some(ManagerDomain::Pleb),
                manager: Some(Manager::Clouseau),
            },
            moxy: MoxyClaim {
                destination: Some(House::Flynt),
                relation: Some(MoxyRelation::Bond),
                route: Some(CanonicalRouteId::StairwayToHeaven),
                manager: Some(Manager::Hal),
                domain: Some(ManagerDomain::Meta),
                velocity_only: false,
            },
            foxy: FoxyClaim {
                source_kind: Some(FoxySourceKind::Moxy),
                reflection_kind: Some(ReflectionKind::InvertedReturn),
                manager: Some(Manager::Cleopatra),
                domain: Some(ManagerDomain::Blep),
                automatically_evil: false,
            },
            proxy_replaces_coordinate: false,
            moxy_as_proximity: false,
            proxy_as_ring: false,
            automatic_point_squared_from_proxy: false,
            automatic_movement_from_moxy: false,
            automatic_legality_from_foxy: false,
            round_proxy_forbidden: false,
            coordinate_alignment_claim: Some((RotationPosition::twelve(), House::Flynt)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSpatialFixture {
    point: Point,
    rotation_context: RotationObservationContext,
    interpretation: PlayerSpatialInterpretation,
}

impl PlayerSpatialFixture {
    #[must_use]
    pub const fn point(&self) -> &Point {
        &self.point
    }

    #[must_use]
    pub const fn rotation_context(&self) -> RotationObservationContext {
        self.rotation_context
    }

    #[must_use]
    pub const fn interpretation(&self) -> &PlayerSpatialInterpretation {
        &self.interpretation
    }
}

#[must_use]
pub fn canonical_player_spatial_contract_fixture() -> PlayerSpatialContractInput {
    PlayerSpatialContractInput::default()
}

#[must_use]
pub fn derive_player_spatial_interpretation(point: &Point) -> PlayerSpatialInterpretation {
    let Some(coordinate) = point
        .world()
        .geometry()
        .coordinate_for_ring(point.progression().stable_point_level())
    else {
        return PlayerSpatialInterpretation::new(None, None, None);
    };

    if coordinate.position() == RotationPosition::twelve()
        && point
            .world()
            .route_visible(CanonicalRouteId::StairwayToHeaven)
        && point
            .world()
            .route_survivable(CanonicalRouteId::StairwayToHeaven)
    {
        let proxy = Proxy::new(
            House::Stonebend,
            RelativeDirection::Northwest,
            SpatialGeometry::Round,
            Proximity::Distal,
            Some(CanonicalRouteId::StairwayToHeaven),
            Some(coordinate),
        );
        let moxy = Moxy::new(
            proxy.clone(),
            Some(House::Flynt),
            MoxyRelation::Bond,
            Some(CanonicalRouteId::StairwayToHeaven),
            Manager::Hal,
        );
        return PlayerSpatialInterpretation::new(Some(proxy), Some(moxy), None);
    }

    PlayerSpatialInterpretation::new(None, None, None)
}

pub fn build_canonical_player_spatial_fixture() -> io::Result<PlayerSpatialFixture> {
    let point_squared_fixture = build_canonical_point_squared_fixture()?;
    let stabilized = point_squared_fixture.first_application().stabilized_point();
    let world = stabilized
        .world()
        .with_geometry_preserving_state(
            PointGeometryState::at_position(RotationPosition::twelve()),
        );
    let point = Point::with_physical_domain_state(
        stabilized.frame_state().clone(),
        stabilized.progression().clone(),
        world,
        stabilized.physical().clone(),
    );
    let rotation_context = observation_context_for_point(&point)
        .expect("canonical player spatial fixture requires a numbered world position");
    let interpretation = derive_player_spatial_interpretation(&point);

    Ok(PlayerSpatialFixture {
        point,
        rotation_context,
        interpretation,
    })
}

pub fn validate_player_spatial_contract(
    input: &PlayerSpatialContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.proxy.anchor.is_none() {
        diagnostics.push(player_spatial_error("Proxy must include an anchor."));
    }
    if input.proxy.direction.is_none() {
        diagnostics.push(player_spatial_error("Proxy must include a direction."));
    }
    if input.proxy.geometry.is_none() {
        diagnostics.push(player_spatial_error("Proxy must include geometry."));
    }
    if input.proxy.proximity.is_none() {
        diagnostics.push(player_spatial_error("Proxy must include proximity."));
    }
    if input.proxy.domain != Some(ManagerDomain::Pleb) {
        diagnostics.push(player_spatial_error(
            "Proxy must remain in the PLEB / Proxy domain.",
        ));
    }
    if input.proxy.manager != Some(Manager::Clouseau) {
        diagnostics.push(player_spatial_error("Clouseau must handle Proxy."));
    }
    if input.round_proxy_forbidden {
        diagnostics.push(player_spatial_error(
            "Round locations cannot be excluded from Proxy; Proxy is not Flat-only.",
        ));
    }

    if input.moxy_as_proximity {
        diagnostics.push(player_spatial_error(
            "Moxy cannot be used as a Proximity value.",
        ));
    }
    if input.proxy_as_ring {
        diagnostics.push(player_spatial_error("Proxy cannot replace Ring."));
    }
    if input.moxy.velocity_only || input.moxy.relation.is_none() {
        diagnostics.push(player_spatial_error(
            "Moxy must describe a relation, bond, destination, or beyond-context; it cannot be velocity only.",
        ));
    }
    if input.moxy.domain != Some(ManagerDomain::Meta) {
        diagnostics.push(player_spatial_error(
            "Moxy must remain in the META / Moxy domain.",
        ));
    }
    if input.moxy.manager != Some(Manager::Hal) {
        diagnostics.push(player_spatial_error("HAL must handle Moxy."));
    }

    if input.foxy.source_kind.is_none() || input.foxy.reflection_kind.is_none() {
        diagnostics.push(player_spatial_error(
            "Foxy must identify a source and reflection kind.",
        ));
    }
    if input.foxy.domain != Some(ManagerDomain::Blep) {
        diagnostics.push(player_spatial_error(
            "Foxy must remain in the BLEP / Foxy domain.",
        ));
    }
    if input.foxy.manager != Some(Manager::Cleopatra) {
        diagnostics.push(player_spatial_error("Cleopatra must handle Foxy."));
    }
    if input.foxy.automatically_evil {
        diagnostics.push(player_spatial_error(
            "Foxy cannot automatically mean evil; its root meaning is reflection and inversion.",
        ));
    }

    if input.proxy_replaces_coordinate {
        diagnostics.push(player_spatial_error(
            "Proxy cannot replace world coordinates; Ring + Absolute Position remain authoritative.",
        ));
    }
    if input.automatic_point_squared_from_proxy {
        diagnostics.push(player_spatial_error(
            "Proxy creation cannot automatically grant Point².",
        ));
    }
    if input.automatic_movement_from_moxy {
        diagnostics.push(player_spatial_error(
            "Moxy cannot automatically execute movement.",
        ));
    }
    if input.automatic_legality_from_foxy {
        diagnostics.push(player_spatial_error(
            "Foxy reflection cannot automatically mark a Recipe legal.",
        ));
    }

    if let Some((position, claimed_house)) = input.coordinate_alignment_claim {
        if house_for_position(position) != claimed_house {
            diagnostics.push(player_spatial_error(format!(
                "Position {} derives {} from the Rule of Twelve and cannot be stored as {}.",
                position,
                house_display(house_for_position(position)),
                house_display(claimed_house)
            )));
        }
    }

    diagnostics
}

pub fn build_player_location_witness() -> io::Result<String> {
    let fixture = build_canonical_player_spatial_fixture()?;
    let point = fixture.point();
    let rotation = fixture.rotation_context();
    let interpretation = fixture.interpretation();
    let proxy = interpretation
        .proxy()
        .expect("canonical player spatial fixture requires Proxy");
    let moxy = interpretation
        .moxy()
        .expect("canonical player spatial fixture requires Moxy");

    Ok(format!(
        "PLAYER SPATIAL INTERPRETATION\n\n\
         World Coordinate:\n\
         Ring {}\n\
         Position {}\n\n\
         Rule-of-Twelve:\n\
         Pass {}\n\
         House Number {}\n\
         House Alignment: {}\n\
         Rotation Complete: {}\n\n\
         Proxy:\n\
         {}\n\n\
         Proxy Details:\n\
         Anchor: {}\n\
         Direction: {}\n\
         Geometry: {}\n\
         Proximity: {}\n\
         Route: {}\n\n\
         Moxy:\n\
         {}\n\n\
         Foxy:\n\
         Inactive\n\n\
         Managers:\n\
         Clouseau locates.\n\
         HAL connects.\n\
         Cleopatra has no active reflection.\n",
        point.progression().stable_point_level(),
        rotation.absolute_position(),
        rotation.pass().value(),
        rotation.house_number(),
        house_display(rotation.house()),
        yes_no(rotation.rotation_complete()),
        proxy.render(),
        house_display(proxy.anchor()),
        proxy.direction().as_str(),
        proxy.geometry().as_str(),
        proxy.proximity().as_str(),
        proxy
            .route()
            .map(CanonicalRouteId::as_str)
            .unwrap_or("unset"),
        moxy.render(),
    ))
}

fn player_spatial_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::PlayerSpatialMismatch,
        message: message.into(),
    }
}

#[must_use]
fn yes_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HousePositionKind {
    PrimaryAnchor,
    Recurrence,
}

impl HousePositionKind {
    #[must_use]
    pub const fn is_primary_anchor(self) -> bool {
        matches!(self, Self::PrimaryAnchor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThresholdKind {
    DescentIntoAbyss,
}

impl ThresholdKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescentIntoAbyss => "DescentIntoAbyss",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionThreshold {
    from_position: RotationPosition,
    toward_house: House,
    kind: ThresholdKind,
}

impl PositionThreshold {
    #[must_use]
    pub const fn new(
        from_position: RotationPosition,
        toward_house: House,
        kind: ThresholdKind,
    ) -> Self {
        Self {
            from_position,
            toward_house,
            kind,
        }
    }

    #[must_use]
    pub const fn from_position(self) -> RotationPosition {
        self.from_position
    }

    #[must_use]
    pub const fn toward_house(self) -> House {
        self.toward_house
    }

    #[must_use]
    pub const fn kind(self) -> ThresholdKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleOfTwelvePosition {
    absolute_position: RotationPosition,
    pass: RotationPass,
    house_number: HouseNumber,
    house: House,
    position_kind: HousePositionKind,
    threshold: Option<PositionThreshold>,
    rotation_complete: bool,
}

impl RuleOfTwelvePosition {
    #[must_use]
    pub const fn new(
        absolute_position: RotationPosition,
        pass: RotationPass,
        house_number: HouseNumber,
        house: House,
        position_kind: HousePositionKind,
        threshold: Option<PositionThreshold>,
        rotation_complete: bool,
    ) -> Self {
        Self {
            absolute_position,
            pass,
            house_number,
            house,
            position_kind,
            threshold,
            rotation_complete,
        }
    }

    #[must_use]
    pub const fn absolute_position(self) -> RotationPosition {
        self.absolute_position
    }

    #[must_use]
    pub const fn pass(self) -> RotationPass {
        self.pass
    }

    #[must_use]
    pub const fn house_number(self) -> HouseNumber {
        self.house_number
    }

    #[must_use]
    pub const fn local_step(self) -> HouseNumber {
        self.house_number
    }

    #[must_use]
    pub const fn house(self) -> House {
        self.house
    }

    #[must_use]
    pub const fn position_kind(self) -> HousePositionKind {
        self.position_kind
    }

    #[must_use]
    pub const fn is_primary_anchor(self) -> bool {
        self.position_kind.is_primary_anchor()
    }

    #[must_use]
    pub const fn threshold(self) -> Option<PositionThreshold> {
        self.threshold
    }

    #[must_use]
    pub const fn rotation_complete(self) -> bool {
        self.rotation_complete
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationObservationContext {
    ring: WorldRing,
    absolute_position: RotationPosition,
    pass: RotationPass,
    house_number: HouseNumber,
    house: House,
    primary_anchor: bool,
    threshold_to: Option<House>,
    rotation_complete: bool,
}

impl RotationObservationContext {
    #[must_use]
    pub const fn ring(self) -> WorldRing {
        self.ring
    }

    #[must_use]
    pub const fn absolute_position(self) -> RotationPosition {
        self.absolute_position
    }

    #[must_use]
    pub const fn pass(self) -> RotationPass {
        self.pass
    }

    #[must_use]
    pub const fn house_number(self) -> HouseNumber {
        self.house_number
    }

    #[must_use]
    pub const fn house(self) -> House {
        self.house
    }

    #[must_use]
    pub const fn primary_anchor(self) -> bool {
        self.primary_anchor
    }

    #[must_use]
    pub const fn threshold_to(self) -> Option<House> {
        self.threshold_to
    }

    #[must_use]
    pub const fn rotation_complete(self) -> bool {
        self.rotation_complete
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotationCompletion {
    ring: WorldRing,
    completed_position: RotationPosition,
    next_position: RotationPosition,
    full_cycle_complete: bool,
}

impl RotationCompletion {
    #[must_use]
    pub const fn ring(self) -> WorldRing {
        self.ring
    }

    #[must_use]
    pub const fn completed_position(self) -> RotationPosition {
        self.completed_position
    }

    #[must_use]
    pub const fn next_position(self) -> RotationPosition {
        self.next_position
    }

    #[must_use]
    pub const fn full_cycle_complete(self) -> bool {
        self.full_cycle_complete
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpiralTransition {
    from_ring: WorldRing,
    from_position: RotationPosition,
    to_ring: WorldRing,
    to_position: RotationPosition,
    point_squared_ascension: u16,
}

impl SpiralTransition {
    #[must_use]
    pub const fn from_ring(self) -> WorldRing {
        self.from_ring
    }

    #[must_use]
    pub const fn from_position(self) -> RotationPosition {
        self.from_position
    }

    #[must_use]
    pub const fn to_ring(self) -> WorldRing {
        self.to_ring
    }

    #[must_use]
    pub const fn to_position(self) -> RotationPosition {
        self.to_position
    }

    #[must_use]
    pub const fn point_squared_ascension(self) -> u16 {
        self.point_squared_ascension
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiralTransitionError {
    MissingNumberedPosition,
    NotAtRotationCompletion,
}

impl fmt::Display for SpiralTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNumberedPosition => {
                f.write_str("spiral transition requires a numbered world position")
            }
            Self::NotAtRotationCompletion => {
                f.write_str("spiral transition requires rotation completion at Position 12")
            }
        }
    }
}

impl std::error::Error for SpiralTransitionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOfTwelvePositionClaim {
    pub absolute_position: u8,
    pub pass_number: u8,
    pub house_number: u8,
    pub house: House,
    pub primary_anchor: bool,
    pub threshold_to: Option<House>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOfTwelveContractInput {
    pub ranina_numbered_position: Option<u8>,
    pub local_house_grammar: Vec<House>,
    pub passes: Vec<Vec<House>>,
    pub position_claims: Vec<RuleOfTwelvePositionClaim>,
    pub ordinary_wrap_increases_ring: bool,
    pub ordinary_wrap_increases_capacity: bool,
    pub entering_position_twelve_grants_point_squared: bool,
    pub point_squared_forces_position_change: bool,
    pub pass_four_inside_rotation: bool,
    pub position_thirteen_exists: bool,
    pub point_squared_becomes_position_thirteen: bool,
    pub point_squared_creates_horizon_squared: bool,
    pub new_center_for_each_pass: bool,
    pub new_center_for_each_ring: bool,
}

impl Default for RuleOfTwelveContractInput {
    fn default() -> Self {
        Self {
            ranina_numbered_position: None,
            local_house_grammar: CANONICAL_HOUSE_GRAMMAR.to_vec(),
            passes: vec![
                CANONICAL_HOUSE_GRAMMAR.to_vec(),
                CANONICAL_HOUSE_GRAMMAR.to_vec(),
                CANONICAL_HOUSE_GRAMMAR.to_vec(),
            ],
            position_claims: canonical_rule_of_twelve_positions()
                .into_iter()
                .map(|identity| RuleOfTwelvePositionClaim {
                    absolute_position: identity.absolute_position().value(),
                    pass_number: identity.pass().value(),
                    house_number: identity.house_number().value(),
                    house: identity.house(),
                    primary_anchor: identity.is_primary_anchor(),
                    threshold_to: identity.threshold().map(PositionThreshold::toward_house),
                })
                .collect(),
            ordinary_wrap_increases_ring: false,
            ordinary_wrap_increases_capacity: false,
            entering_position_twelve_grants_point_squared: false,
            point_squared_forces_position_change: false,
            pass_four_inside_rotation: false,
            position_thirteen_exists: false,
            point_squared_becomes_position_thirteen: false,
            point_squared_creates_horizon_squared: false,
            new_center_for_each_pass: false,
            new_center_for_each_ring: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOfTwelveSpiralFixture {
    point_before: Point,
    ordinary_wrap_point: Point,
    decision: DecisionExecution,
    ascension: PointSquaredAscension,
    spiral_transition: SpiralTransition,
    first_application: PointSquaredApplication,
    second_application: PointSquaredApplication,
}

impl RuleOfTwelveSpiralFixture {
    #[must_use]
    pub const fn point_before(&self) -> &Point {
        &self.point_before
    }

    #[must_use]
    pub const fn ordinary_wrap_point(&self) -> &Point {
        &self.ordinary_wrap_point
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
    pub const fn spiral_transition(&self) -> SpiralTransition {
        self.spiral_transition
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

#[must_use]
pub fn house_display(house: House) -> &'static str {
    match house {
        House::Stonebend => "Stonebend",
        House::Sandmanor => "Sandmanor",
        House::Glaushouse => "Glaüshouse",
        House::Flynt => "Flynt",
    }
}

#[must_use]
pub const fn material_verb_for_house(house: House) -> &'static str {
    match house {
        House::Stonebend => "Diamond claims",
        House::Sandmanor => "Crystal measures",
        House::Glaushouse => "Jade clears",
        House::Flynt => "Opal shimmers",
    }
}

#[must_use]
pub const fn operational_function_for_house(house: House) -> &'static str {
    match house {
        House::Stonebend => "establish, hold, embody, stabilize",
        House::Sandmanor => "measure, compare, model, diagnose",
        House::Glaushouse => "clear, repair, treat, restore",
        House::Flynt => "move, test, execute, transmit",
    }
}

#[must_use]
pub const fn house_number_for_position(position: RotationPosition) -> HouseNumber {
    match ((position.value() - 1) % 4) + 1 {
        1 => HouseNumber::One,
        2 => HouseNumber::Two,
        3 => HouseNumber::Three,
        _ => HouseNumber::Four,
    }
}

#[must_use]
pub const fn local_step_for_position(position: RotationPosition) -> HouseNumber {
    house_number_for_position(position)
}

#[must_use]
pub const fn house_for_position(position: RotationPosition) -> House {
    house_number_for_position(position).house()
}

#[must_use]
pub const fn pass_for_position(position: RotationPosition) -> RotationPass {
    match ((position.value() - 1) / 4) + 1 {
        1 => RotationPass::One,
        2 => RotationPass::Two,
        _ => RotationPass::Three,
    }
}

#[must_use]
pub const fn next_position(position: RotationPosition) -> RotationPosition {
    position.next()
}

#[must_use]
pub const fn previous_position(position: RotationPosition) -> RotationPosition {
    position.previous()
}

#[must_use]
pub const fn opposite_position(position: RotationPosition) -> RotationPosition {
    position.opposite()
}

#[must_use]
pub const fn is_primary_house_anchor(position: RotationPosition) -> bool {
    matches!(position.value(), 1 | 7)
}

#[must_use]
pub const fn threshold_at_position(position: RotationPosition) -> Option<PositionThreshold> {
    if position.value() == 6 {
        Some(PositionThreshold::new(
            position,
            House::Glaushouse,
            ThresholdKind::DescentIntoAbyss,
        ))
    } else {
        None
    }
}

#[must_use]
pub const fn is_rotation_complete(position: RotationPosition) -> bool {
    position.value() == 12
}

#[must_use]
pub const fn position_identity(position: RotationPosition) -> RuleOfTwelvePosition {
    RuleOfTwelvePosition::new(
        position,
        pass_for_position(position),
        house_number_for_position(position),
        house_for_position(position),
        if is_primary_house_anchor(position) {
            HousePositionKind::PrimaryAnchor
        } else {
            HousePositionKind::Recurrence
        },
        threshold_at_position(position),
        is_rotation_complete(position),
    )
}

#[must_use]
pub fn canonical_rule_of_twelve_positions() -> [RuleOfTwelvePosition; 12] {
    [
        position_identity(RotationPosition::one()),
        position_identity(RotationPosition::new(2).expect("2 is valid")),
        position_identity(RotationPosition::new(3).expect("3 is valid")),
        position_identity(RotationPosition::new(4).expect("4 is valid")),
        position_identity(RotationPosition::new(5).expect("5 is valid")),
        position_identity(RotationPosition::six()),
        position_identity(RotationPosition::seven()),
        position_identity(RotationPosition::new(8).expect("8 is valid")),
        position_identity(RotationPosition::new(9).expect("9 is valid")),
        position_identity(RotationPosition::new(10).expect("10 is valid")),
        position_identity(RotationPosition::new(11).expect("11 is valid")),
        position_identity(RotationPosition::twelve()),
    ]
}

#[must_use]
pub fn rotation_completion_for(ring: WorldRing, position: RotationPosition) -> RotationCompletion {
    RotationCompletion {
        ring,
        completed_position: position,
        next_position: next_position(position),
        full_cycle_complete: is_rotation_complete(position),
    }
}

#[must_use]
pub fn observation_context_for_point(point: &Point) -> Option<RotationObservationContext> {
    let position = point.world().geometry().current_position()?;
    let identity = position_identity(position);
    Some(RotationObservationContext {
        ring: WorldRing::new(point.progression().stable_point_level()),
        absolute_position: identity.absolute_position(),
        pass: identity.pass(),
        house_number: identity.house_number(),
        house: identity.house(),
        primary_anchor: identity.is_primary_anchor(),
        threshold_to: identity.threshold().map(PositionThreshold::toward_house),
        rotation_complete: identity.rotation_complete(),
    })
}

pub fn advance_angular_position(point: &Point) -> Result<Point, SpiralTransitionError> {
    let Some(position) = point.world().geometry().current_position() else {
        return Err(SpiralTransitionError::MissingNumberedPosition);
    };
    let next_world = point
        .world()
        .with_geometry_preserving_state(PointGeometryState::at_position(next_position(position)));
    Ok(Point::with_physical_domain_state(
        point.frame_state().clone(),
        point.progression().clone(),
        next_world,
        point.physical().clone(),
    ))
}

pub fn select_canonical_spiral_transition(
    point: &Point,
    ascension: &PointSquaredAscension,
) -> Result<SpiralTransition, SpiralTransitionError> {
    let Some(position) = point.world().geometry().current_position() else {
        return Err(SpiralTransitionError::MissingNumberedPosition);
    };
    if !is_rotation_complete(position) {
        return Err(SpiralTransitionError::NotAtRotationCompletion);
    }
    Ok(SpiralTransition {
        from_ring: WorldRing::new(point.progression().stable_point_level()),
        from_position: position,
        to_ring: WorldRing::new(ascension.ascension_index()),
        to_position: RotationPosition::one(),
        point_squared_ascension: ascension.ascension_index(),
    })
}

pub fn apply_point_squared_spiral_transition(
    point: &Point,
    ascension: &PointSquaredAscension,
    transition: SpiralTransition,
) -> PointSquaredApplication {
    let application = apply_point_squared_ascension(point, ascension);
    if application.status() != PointSquaredApplicationStatus::Applied {
        return application;
    }

    let stabilized = application.stabilized_point();
    let next_world = stabilized
        .world()
        .with_geometry_preserving_state(PointGeometryState::at_position(transition.to_position()));
    let stabilized_point = Point::with_physical_domain_state(
        stabilized.frame_state().clone(),
        stabilized.progression().clone(),
        next_world,
        stabilized.physical().clone(),
    );
    PointSquaredApplication::new(
        PointSquaredApplicationStatus::Applied,
        stabilized_point,
        format!(
            "{} Spiral transition selected: Ring {} / Position {} → Ring {} / Position {}.",
            application.witness(),
            transition.from_ring().value(),
            transition.from_position(),
            transition.to_ring().value(),
            transition.to_position()
        ),
    )
}

pub fn canonical_rotation_contract_fixture() -> HollowGroveRotationContractInput {
    HollowGroveRotationContractInput::default()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowGroveRotationContractInput {
    pub ranina_is_exact_center: bool,
    pub ranina_numbered_position: Option<u8>,
    pub ranina_is_house: bool,
    pub rotation_positions: u8,
    pub stonebend_position: u8,
    pub glaushouse_threshold_position: u8,
    pub glaushouse_position: u8,
    pub point_squared_opens_next_ring: bool,
    pub point_squared_rotates_automatically: bool,
    pub moving_one_position_grants_capacity: bool,
    pub point_squared_creates_new_center: bool,
    pub multiple_centers_exist: bool,
    pub point_squared_creates_horizon_squared: bool,
    pub distinct_ring_and_position: bool,
}

impl Default for HollowGroveRotationContractInput {
    fn default() -> Self {
        Self {
            ranina_is_exact_center: true,
            ranina_numbered_position: None,
            ranina_is_house: false,
            rotation_positions: RotationPosition::COUNT,
            stonebend_position: 1,
            glaushouse_threshold_position: 6,
            glaushouse_position: 7,
            point_squared_opens_next_ring: true,
            point_squared_rotates_automatically: false,
            moving_one_position_grants_capacity: false,
            point_squared_creates_new_center: false,
            multiple_centers_exist: false,
            point_squared_creates_horizon_squared: false,
            distinct_ring_and_position: true,
        }
    }
}

pub fn canonical_rule_of_twelve_contract_fixture() -> RuleOfTwelveContractInput {
    RuleOfTwelveContractInput::default()
}

pub fn validate_hollow_grove_rotation_contract(
    input: &HollowGroveRotationContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if !input.ranina_is_exact_center {
        diagnostics.push(rotation_error("Ranina must remain the exact center."));
    }
    if let Some(position) = input.ranina_numbered_position {
        diagnostics.push(rotation_error(format!(
            "Ranina is the unnumbered center and cannot be assigned numbered position {position}."
        )));
    }
    if input.ranina_is_house {
        diagnostics.push(rotation_error(
            "Ranina is the center and cannot be promoted into a fifth House.",
        ));
    }
    if input.rotation_positions != RotationPosition::COUNT {
        diagnostics.push(rotation_error(format!(
            "A full Hollow Grove rotation must contain exactly 12 positions, got {}.",
            input.rotation_positions
        )));
    }
    if input.stonebend_position != 1 {
        diagnostics.push(rotation_error(format!(
            "Stonebend must anchor Position 1, got Position {}.",
            input.stonebend_position
        )));
    }
    if input.glaushouse_threshold_position != 6 {
        diagnostics.push(rotation_error(format!(
            "Position 6 is the Glaüshouse threshold, got Position {}.",
            input.glaushouse_threshold_position
        )));
    }
    if input.glaushouse_position != 7 {
        diagnostics.push(rotation_error(format!(
            "Glaüshouse must anchor Position 7, got Position {}.",
            input.glaushouse_position
        )));
    }

    match (
        RotationPosition::new(input.stonebend_position),
        RotationPosition::new(input.glaushouse_position),
    ) {
        (Some(stonebend), Some(glaushouse)) if stonebend.opposite() != glaushouse => {
            diagnostics.push(rotation_error(format!(
                "Stonebend Position {} and Glaüshouse Position {} must be opposites across Ranina.",
                stonebend, glaushouse
            )));
        }
        (Some(_), Some(_)) => {}
        _ => diagnostics.push(rotation_error(
            "Stonebend and Glaüshouse positions must both be valid positions inside 1..=12.",
        )),
    }

    if !input.point_squared_opens_next_ring {
        diagnostics.push(rotation_error(
            "Point² must open the next ring around the same center.",
        ));
    }
    if input.point_squared_rotates_automatically {
        diagnostics.push(rotation_error(
            "Point² cannot rotate the Point automatically; position and ring remain distinct.",
        ));
    }
    if input.moving_one_position_grants_capacity {
        diagnostics.push(rotation_error(
            "Moving from one position to another cannot grant Point² capacity advancement.",
        ));
    }
    if input.point_squared_creates_new_center {
        diagnostics.push(rotation_error(
            "Point² cannot create a new center; Ranina remains fixed.",
        ));
    }
    if input.multiple_centers_exist {
        diagnostics.push(rotation_error(
            "Each ring must rotate around the same Ranina center; multiple centers are forbidden.",
        ));
    }
    if input.point_squared_creates_horizon_squared {
        diagnostics.push(rotation_error(
            "Point² already names the opened horizon and cannot create a separate Horizon² state.",
        ));
    }
    if !input.distinct_ring_and_position {
        diagnostics.push(rotation_error(
            "Ring and position must remain distinct so radial progression does not collapse into angular movement.",
        ));
    }

    diagnostics
}

pub fn validate_rule_of_twelve_contract(
    input: &RuleOfTwelveContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(position) = input.ranina_numbered_position {
        diagnostics.push(rule_error(format!(
            "Ranina remains outside the numbered wheel and cannot take position {position}."
        )));
    }
    if input.local_house_grammar.len() != 4 {
        diagnostics.push(rule_error(format!(
            "The local House grammar must contain exactly four Houses, got {}.",
            input.local_house_grammar.len()
        )));
    } else if input.local_house_grammar != CANONICAL_HOUSE_GRAMMAR {
        diagnostics.push(rule_error(
            "The local House grammar must remain Stonebend, Sandmanor, Glaüshouse, Flynt.",
        ));
    }

    if input.passes.len() != 3 {
        diagnostics.push(rule_error(format!(
            "A complete rotation must contain exactly three passes, got {}.",
            input.passes.len()
        )));
    }
    for (index, pass) in input.passes.iter().enumerate() {
        if pass.len() != 4 {
            diagnostics.push(rule_error(format!(
                "Pass {} must contain exactly four House steps, got {}.",
                index + 1,
                pass.len()
            )));
            continue;
        }
        if pass.as_slice() != CANONICAL_HOUSE_GRAMMAR {
            diagnostics.push(rule_error(format!(
                "Pass {} must preserve the House order Stonebend, Sandmanor, Glaüshouse, Flynt.",
                index + 1
            )));
        }
    }

    if input.position_claims.len() != 12 {
        diagnostics.push(rule_error(format!(
            "The Rule of Twelve must define exactly 12 absolute positions, got {}.",
            input.position_claims.len()
        )));
    }
    let mut seen_positions = [false; 12];
    for claim in &input.position_claims {
        let Some(position) = RotationPosition::new(claim.absolute_position) else {
            diagnostics.push(rule_error(format!(
                "Absolute position {} is invalid; positions must stay inside 1..=12.",
                claim.absolute_position
            )));
            continue;
        };
        if seen_positions[(position.value() - 1) as usize] {
            diagnostics.push(rule_error(format!(
                "Absolute position {} was claimed more than once.",
                position
            )));
        } else {
            seen_positions[(position.value() - 1) as usize] = true;
        }

        let Some(pass) = RotationPass::new(claim.pass_number) else {
            diagnostics.push(rule_error(format!(
                "Pass {} is invalid; one rotation only contains Pass 1 through Pass 3.",
                claim.pass_number
            )));
            continue;
        };
        let Some(house_number) = HouseNumber::new(claim.house_number) else {
            diagnostics.push(rule_error(format!(
                "House number {} is invalid; the local grammar only contains House numbers 1 through 4.",
                claim.house_number
            )));
            continue;
        };
        let derived = position_identity(position);
        if pass != derived.pass() {
            diagnostics.push(rule_error(format!(
                "Position {} must belong to {}, got Pass {}.",
                position,
                derived.pass(),
                pass.value()
            )));
        }
        if house_number != derived.house_number() {
            diagnostics.push(rule_error(format!(
                "Position {} must carry House number {}, got House number {}.",
                position,
                derived.house_number(),
                house_number
            )));
        }
        if claim.house != derived.house() {
            diagnostics.push(rule_error(format!(
                "Position {} must align with {}, got {}.",
                position,
                house_display(derived.house()),
                house_display(claim.house)
            )));
        }
        if claim.primary_anchor != derived.is_primary_anchor() {
            diagnostics.push(rule_error(format!(
                "Position {} primary-anchor status drifted from the canonical Rule of Twelve.",
                position
            )));
        }
        if claim.threshold_to != derived.threshold().map(PositionThreshold::toward_house) {
            diagnostics.push(rule_error(format!(
                "Position {} threshold metadata drifted from the canonical Rule of Twelve.",
                position
            )));
        }
    }
    if seen_positions.iter().any(|seen| !seen) {
        diagnostics.push(rule_error(
            "The Rule of Twelve must cover every absolute position from 1 through 12 exactly once.",
        ));
    }

    if input.ordinary_wrap_increases_ring {
        diagnostics.push(rule_error(
            "Angular wrap from Position 12 to Position 1 cannot increase the ring without Point².",
        ));
    }
    if input.ordinary_wrap_increases_capacity {
        diagnostics.push(rule_error(
            "Ordinary angular movement cannot increase Current Capacity or Aura Capacity.",
        ));
    }
    if input.entering_position_twelve_grants_point_squared {
        diagnostics.push(rule_error(
            "Entering Position 12 cannot automatically grant Point².",
        ));
    }
    if input.point_squared_forces_position_change {
        diagnostics.push(rule_error(
            "Point² cannot force an angular position change regardless of scenario.",
        ));
    }
    if input.pass_four_inside_rotation {
        diagnostics.push(rule_error(
            "Pass 4 cannot exist inside one twelve-position rotation.",
        ));
    }
    if input.position_thirteen_exists {
        diagnostics.push(rule_error(
            "No Position 13 exists inside the Rule of Twelve; higher progression must use another ring.",
        ));
    }
    if input.point_squared_becomes_position_thirteen {
        diagnostics.push(rule_error(
            "Point² cannot become Position 13 because no Position 13 exists.",
        ));
    }
    if input.point_squared_creates_horizon_squared {
        diagnostics.push(rule_error(
            "Point² remains Point² and cannot be replaced by Horizon².",
        ));
    }
    if input.new_center_for_each_pass || input.new_center_for_each_ring {
        diagnostics.push(rule_error(
            "Ranina remains the same center across every pass and every ring.",
        ));
    }

    diagnostics
}

pub fn build_canonical_spiral_fixture() -> io::Result<RuleOfTwelveSpiralFixture> {
    let point_before = Point::with_domain_state(
        FrameState::origin(),
        PointProgressionState::origin(),
        ReachableWorldState::with_geometry(PointGeometryState::at_position(
            RotationPosition::twelve(),
        )),
    );
    let ordinary_wrap_point = advance_angular_position(&point_before)
        .map_err(|error| io::Error::other(error.to_string()))?;
    let decision = execute_decision(&point_before, DecisionIntent::FavorCurrent)
        .map_err(|error| io::Error::other(format!("spiral fixture execution failed: {error:?}")))?;
    let ascension = prepare_point_squared_ascension(&point_before, decision.execution().landing())
        .map_err(|error| io::Error::other(error.to_string()))?;
    let spiral_transition = select_canonical_spiral_transition(&point_before, &ascension)
        .map_err(|error| io::Error::other(error.to_string()))?;
    let first_application =
        apply_point_squared_spiral_transition(&point_before, &ascension, spiral_transition);
    let second_application = apply_point_squared_spiral_transition(
        first_application.stabilized_point(),
        &ascension,
        spiral_transition,
    );

    Ok(RuleOfTwelveSpiralFixture {
        point_before,
        ordinary_wrap_point,
        decision,
        ascension,
        spiral_transition,
        first_application,
        second_application,
    })
}

pub fn build_map_witness() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let point_before = fixture.point_before();
    let point_after = fixture.first_application().stabilized_point();
    let before_identity = point_before
        .world()
        .geometry()
        .rule_of_twelve_position()
        .expect("canonical point has numbered position");
    let after_identity = point_after
        .world()
        .geometry()
        .rule_of_twelve_position()
        .expect("canonical point has numbered position");

    Ok(format!(
        "HOLLOW GROVE ROTATIONAL MAP WITNESS\n\n\
         Center:\n\
         {}\n\
         Numbered Position: none\n\
         Meaning: {}\n\n\
         Rotation:\n\
         Positions: {}\n\
         Degrees per Position: {}\n\n\
         Position 1:\n\
         Stonebend\n\
         House Number: 1\n\
         Pass: 1\n\
         Capricorn\n\
         Sea-Goat\n\
         Hollow Current\n\
         Life held in form\n\n\
         Position 6:\n\
         Sandmanor\n\
         House Number: 2\n\
         Pass: 2\n\
         Glaüshouse Threshold\n\
         Descent toward Abyss\n\n\
         Position 7:\n\
         Glaüshouse\n\
         House Number: 3\n\
         Pass: 2\n\
         Cancer\n\
         Crab\n\
         Abyss\n\
         Life felt in depth\n\n\
         Opposition:\n\
         Stonebend 1 ↔ Glaüshouse 7\n\
         Separation: 6 positions\n\
         Angle: 180 degrees\n\n\
         Point Fixture:\n\
         Before Point²: ring {} / position {} / pass {} / house number {} / {}\n\
         After Point²: ring {} / position {} / pass {} / house number {} / {}\n\
         Position preserved without movement: {}\n\
         Being remains Hueman: {}\n\n\
         Progression:\n\
         Point = Hueman + current reachable ring\n\
         Point² = Current Capacity +1 + Aura Capacity +1\n\
         Point² opens next ring\n\
         Ranina remains centered\n",
        WorldCenterId::Ranina.as_str(),
        WorldCenterId::Ranina.meaning(),
        RotationPosition::COUNT,
        RotationPosition::DEGREES_PER_POSITION,
        point_before.progression().stable_point_level(),
        before_identity.absolute_position(),
        before_identity.pass().value(),
        before_identity.house_number(),
        house_display(before_identity.house()),
        point_after.progression().stable_point_level(),
        after_identity.absolute_position(),
        after_identity.pass().value(),
        after_identity.house_number(),
        house_display(after_identity.house()),
        point_before.world().geometry().current_position()
            == point_after.world().geometry().current_position(),
        point_after.being() == BeingId::Hueman,
    ))
}

pub fn build_map_validation_report() -> io::Result<String> {
    let contract_diagnostics =
        validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture());
    let rule_diagnostics =
        validate_rule_of_twelve_contract(&canonical_rule_of_twelve_contract_fixture());
    let fixture = build_canonical_point_squared_fixture()?;
    let before = fixture.point_before();
    let after = fixture.first_application().stabilized_point();
    let mut errors = Vec::new();
    errors.extend(contract_diagnostics);
    errors.extend(rule_diagnostics);

    if before.world().geometry().center() != WorldCenterId::Ranina
        || after.world().geometry().center() != WorldCenterId::Ranina
    {
        errors.push(rotation_error(
            "Ranina must remain the unique center before and after Point².",
        ));
    }
    if before.world().geometry().current_position() != after.world().geometry().current_position() {
        errors.push(rotation_error(
            "Point² cannot rotate the Point automatically when no movement occurs.",
        ));
    }
    if after.progression().stable_point_level() != before.progression().stable_point_level() + 1 {
        errors.push(rotation_error(
            "Point² must open the next ring by raising the stabilized Point level by one.",
        ));
    }
    if fixture.first_application().status() != PointSquaredApplicationStatus::Applied {
        errors.push(rotation_error(
            "The canonical map fixture requires one legally applied Point² ascension.",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Rotational Map Validation\n\n\
             - status: pass\n\
             - unique Ranina center: pass\n\
             - twelve-position rotation: pass\n\
             - four-House grammar: pass\n\
             - three-pass repetition: pass\n\
             - Stonebend position 1: pass\n\
             - Glaüshouse threshold 6: pass\n\
             - Glaüshouse position 7: pass\n\
             - opposition geometry: pass\n\
             - wraparound logic: pass\n\
             - ring and position distinction: pass\n\
             - Point² radial expansion: pass\n\
             - Ranina center invariance: pass\n",
        ))
    } else {
        let mut output =
            String::from("# Hollow Grove Rotational Map Validation\n\n- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        Ok(output)
    }
}

pub fn build_rule_of_twelve_witness() -> io::Result<String> {
    let spiral_fixture = build_canonical_spiral_fixture()?;
    let completion = rotation_completion_for(WorldRing::new(1), RotationPosition::twelve());
    Ok(format!(
        "HOLLOW GROVE RULE OF TWELVE\n\n\
         Center:\n\
         Ranina\n\
         Numbered: No\n\
         Unique Center: Yes\n\n\
         Local House Grammar:\n\
         1 Stonebend   — {}\n\
         2 Sandmanor   — {}\n\
         3 Glaüshouse  — {}\n\
         4 Flynt       — {}\n\n\
         Pass 1:\n\
         1 Stonebend\n\
         2 Sandmanor\n\
         3 Glaüshouse\n\
         4 Flynt\n\n\
         Pass 2:\n\
         5 Stonebend\n\
         6 Sandmanor — Glaüshouse threshold\n\
         7 Glaüshouse — primary Cancer/Crab pole\n\
         8 Flynt\n\n\
         Pass 3:\n\
         9 Stonebend\n\
         10 Sandmanor\n\
         11 Glaüshouse\n\
         12 Flynt — rotational completion\n\n\
         Formula:\n\
         4 Houses × 3 Passes = 12 Positions\n\n\
         Primary Axis:\n\
         Stonebend 1 ↔ Ranina ↔ Glaüshouse 7\n\n\
         Position 12:\n\
         Pass 3\n\
         House Number 4\n\
         Flynt\n\
         Next Angular Position: {}\n\
         Automatic Point²: No\n\n\
         Spiral Rule:\n\
         Position 12, Ring {}\n\
         + legal Point²\n\
         + stabilization\n\
         → Position {}, Ring {}\n\n\
         V1.1 topology unchanged.\n",
        material_verb_for_house(House::Stonebend),
        material_verb_for_house(House::Sandmanor),
        material_verb_for_house(House::Glaushouse),
        material_verb_for_house(House::Flynt),
        completion.next_position(),
        spiral_fixture.spiral_transition().from_ring().value(),
        spiral_fixture.spiral_transition().to_position(),
        spiral_fixture.spiral_transition().to_ring().value(),
    ))
}

pub fn build_rule_of_twelve_validation_report() -> io::Result<String> {
    let rotation_diagnostics =
        validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture());
    let rule_diagnostics =
        validate_rule_of_twelve_contract(&canonical_rule_of_twelve_contract_fixture());
    let spiral_fixture = build_canonical_spiral_fixture()?;
    let ordinary_wrap = spiral_fixture.ordinary_wrap_point();
    let stabilized = spiral_fixture.first_application().stabilized_point();
    let mut errors = Vec::new();
    errors.extend(rotation_diagnostics);
    errors.extend(rule_diagnostics);

    if ordinary_wrap.progression().stable_point_level() != 1
        || ordinary_wrap.progression().capacities().current_capacity() != 1
        || ordinary_wrap.progression().capacities().aura_capacity() != 1
        || ordinary_wrap.world().geometry().current_position() != Some(RotationPosition::one())
    {
        errors.push(rule_error(
            "Ordinary angular wrap from Position 12 to Position 1 must keep ring and capacities unchanged.",
        ));
    }
    if spiral_fixture.first_application().status() != PointSquaredApplicationStatus::Applied
        || stabilized.progression().stable_point_level() != 2
        || stabilized.progression().capacities().current_capacity() != 2
        || stabilized.progression().capacities().aura_capacity() != 2
        || stabilized.world().geometry().current_position() != Some(RotationPosition::one())
    {
        errors.push(rule_error(
            "The canonical spiral transition must land at Ring 2 / Position 1 with paired 2/2 capacities.",
        ));
    }
    if spiral_fixture.second_application().status() != PointSquaredApplicationStatus::AlreadyApplied
    {
        errors.push(rule_error(
            "Replaying the same spiral ascension must remain exactly-once and report already applied.",
        ));
    }
    if run_kernel_cycle(Symptom::origin()).to_string() != CANONICAL_WITNESS {
        errors.push(rule_error(
            "The Rule of Twelve cannot alter the frozen V1.1 witness topology.",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Rule of Twelve Validation\n\n\
             - status: pass\n\
             - four-House grammar: pass\n\
             - three-pass repetition: pass\n\
             - twelve-position derivation: pass\n\
             - Position 6 Sandmanor threshold: pass\n\
             - Position 7 Glaüshouse pole: pass\n\
             - Position 12 Flynt completion: pass\n\
             - angular/radial distinction: pass\n\
             - automatic Point² prevention: pass\n\
             - V1.1 unchanged: pass\n",
        ))
    } else {
        let mut output =
            String::from("# Hollow Grove Rule of Twelve Validation\n\n- status: fail\n");
        for diagnostic in errors {
            output.push_str(&format!(
                "- {:?}: {}\n",
                diagnostic.code, diagnostic.message
            ));
        }
        Ok(output)
    }
}

pub fn build_map_artifact() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let point_after = fixture.first_application().stabilized_point();
    let identity = point_after
        .world()
        .geometry()
        .rule_of_twelve_position()
        .expect("canonical point has numbered position");
    Ok(format!(
        "HOLLOW GROVE RANINA / TWELVE-POSITION MAP ARTIFACT\n\n\
         Canonical Doctrine:\n\
         Ranina is the center.\n\
         Stonebend is the first form.\n\
         Glaüshouse is the opposite depth.\n\
         Four Houses form one grammar.\n\
         The grammar repeats three times.\n\
         Point² opens the next ring around the same center.\n\n\
         Center:\n\
         - Ranina = exact center = frog / frog-crab / living transformation hinge\n\n\
         Rotation:\n\
         - positions: 12\n\
         - degrees per position: 30\n\
         - opposite(1) = 7\n\
         - next(12) = 1\n\
         - previous(1) = 12\n\n\
         Anchors:\n\
         - Position 1 = Stonebend = House Number 1 = Pass 1\n\
         - Position 6 = Sandmanor = House Number 2 = Pass 2 = Glaüshouse threshold\n\
         - Position 7 = Glaüshouse = House Number 3 = Pass 2 = primary opposite pole\n\
         - Position 12 = Flynt = House Number 4 = Pass 3 = angular completion\n\n\
         Canonical Point² Fixture:\n\
         - after: ring {} / position {} / pass {} / house number {} / {}\n\
         - current capacity: {}\n\
         - aura capacity: {}\n\
         - Stairway to Heaven visible: {}\n\
         - Stairway to Heaven survivable: {}\n\
         - destination: Stonebend / Position 1\n",
        point_after.progression().stable_point_level(),
        identity.absolute_position(),
        identity.pass().value(),
        identity.house_number(),
        house_display(identity.house()),
        point_after.progression().capacities().current_capacity(),
        point_after.progression().capacities().aura_capacity(),
        point_after
            .world()
            .route_visible(CanonicalRouteId::StairwayToHeaven),
        point_after
            .world()
            .route_survivable(CanonicalRouteId::StairwayToHeaven),
    ))
}

fn rotation_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::RotationalMapMismatch,
        message: message.into(),
    }
}

fn rule_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::RuleOfTwelveMismatch,
        message: message.into(),
    }
}

#[must_use]
pub fn stonebend_anchor_position() -> RotationPosition {
    RotationPosition::one()
}

#[must_use]
pub fn glaushouse_threshold_position() -> RotationPosition {
    RotationPosition::six()
}

#[must_use]
pub fn glaushouse_anchor_position() -> RotationPosition {
    RotationPosition::seven()
}

#[must_use]
pub const fn house_anchor_for_position(position: RotationPosition) -> Option<House> {
    match position.value() {
        1 => Some(House::Stonebend),
        7 => Some(House::Glaushouse),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CANONICAL_HOUSE_GRAMMAR, Foxy, FoxySource, FoxySourceKind, HouseNumber, Moxy, MoxyRelation,
        PlayerSpatialContractInput, PointGeometryState, Proximity, Proxy, ReflectionKind,
        RelativeDirection, RotationPass, RotationPosition, RuleOfTwelveContractInput,
        RuleOfTwelvePositionClaim, SpatialGeometry, WorldCenterId,
        build_canonical_player_spatial_fixture, build_canonical_spiral_fixture, build_map_artifact,
        build_map_validation_report, build_map_witness, build_player_location_witness,
        build_rule_of_twelve_validation_report, build_rule_of_twelve_witness,
        canonical_player_spatial_contract_fixture, canonical_rotation_contract_fixture,
        canonical_rule_of_twelve_contract_fixture, canonical_rule_of_twelve_positions,
        derive_player_spatial_interpretation, glaushouse_anchor_position,
        glaushouse_threshold_position, house_anchor_for_position, house_for_position,
        is_rotation_complete, local_step_for_position, next_position, opposite_position,
        pass_for_position, position_identity, previous_position,
        select_canonical_spiral_transition, stonebend_anchor_position,
        validate_hollow_grove_rotation_contract, validate_player_spatial_contract,
        validate_rule_of_twelve_contract,
    };
    use crate::hollow_grove_contract::House;
    use crate::point::Point;
    use crate::point_progression::{
        PointSquaredApplicationStatus, build_canonical_point_squared_fixture,
    };

    #[test]
    fn canonical_center_fixture_passes() {
        let center = PointGeometryState::ranina_center();
        assert_eq!(center.center(), WorldCenterId::Ranina);
        assert_eq!(center.current_position(), None);
    }

    #[test]
    fn full_rotation_wraps_and_opposes_correctly() {
        let one = RotationPosition::one();
        let six = RotationPosition::six();
        let seven = RotationPosition::seven();
        assert_eq!(RotationPosition::COUNT, 12);
        assert_eq!(RotationPosition::DEGREES_PER_POSITION, 30);
        assert_eq!(next_position(one).value(), 2);
        assert_eq!(next_position(six).value(), 7);
        assert_eq!(next_position(RotationPosition::twelve()), one);
        assert_eq!(previous_position(one).value(), 12);
        assert_eq!(opposite_position(one), seven);
        assert_eq!(opposite_position(seven), one);
        assert_eq!(seven.angle_degrees() - one.angle_degrees(), 180);
    }

    #[test]
    fn canonical_house_and_threshold_anchors_are_locked() {
        assert_eq!(stonebend_anchor_position().value(), 1);
        assert_eq!(glaushouse_threshold_position().value(), 6);
        assert_eq!(glaushouse_anchor_position().value(), 7);
        assert_eq!(
            house_anchor_for_position(stonebend_anchor_position()),
            Some(House::Stonebend)
        );
        assert_eq!(
            house_anchor_for_position(glaushouse_anchor_position()),
            Some(House::Glaushouse)
        );
        assert_eq!(
            house_anchor_for_position(glaushouse_threshold_position()),
            None
        );
    }

    #[test]
    fn canonical_four_house_grammar_and_three_pass_repetition_hold() {
        assert_eq!(CANONICAL_HOUSE_GRAMMAR.len(), 4);
        assert_eq!(
            house_for_position(RotationPosition::one()),
            House::Stonebend
        );
        assert_eq!(
            house_for_position(RotationPosition::new(2).expect("2 is valid")),
            House::Sandmanor
        );
        assert_eq!(
            house_for_position(RotationPosition::new(3).expect("3 is valid")),
            House::Glaushouse
        );
        assert_eq!(
            house_for_position(RotationPosition::new(4).expect("4 is valid")),
            House::Flynt
        );
        assert_eq!(
            house_for_position(RotationPosition::new(5).expect("5 is valid")),
            House::Stonebend
        );
        assert_eq!(
            house_for_position(RotationPosition::six()),
            House::Sandmanor
        );
        assert_eq!(
            house_for_position(RotationPosition::seven()),
            House::Glaushouse
        );
        assert_eq!(
            house_for_position(RotationPosition::new(8).expect("8 is valid")),
            House::Flynt
        );
        assert_eq!(
            house_for_position(RotationPosition::new(9).expect("9 is valid")),
            House::Stonebend
        );
        assert_eq!(
            house_for_position(RotationPosition::new(10).expect("10 is valid")),
            House::Sandmanor
        );
        assert_eq!(
            house_for_position(RotationPosition::new(11).expect("11 is valid")),
            House::Glaushouse
        );
        assert_eq!(house_for_position(RotationPosition::twelve()), House::Flynt);
        assert_eq!(
            pass_for_position(RotationPosition::one()),
            RotationPass::One
        );
        assert_eq!(
            pass_for_position(RotationPosition::new(4).expect("4 is valid")),
            RotationPass::One
        );
        assert_eq!(
            pass_for_position(RotationPosition::new(5).expect("5 is valid")),
            RotationPass::Two
        );
        assert_eq!(
            pass_for_position(RotationPosition::new(8).expect("8 is valid")),
            RotationPass::Two
        );
        assert_eq!(
            pass_for_position(RotationPosition::new(9).expect("9 is valid")),
            RotationPass::Three
        );
        assert_eq!(
            pass_for_position(RotationPosition::twelve()),
            RotationPass::Three
        );
    }

    #[test]
    fn canonical_position_identities_match_the_table() {
        let positions = canonical_rule_of_twelve_positions();
        let expected = [
            (1, 1, 1, House::Stonebend, true),
            (2, 1, 2, House::Sandmanor, false),
            (3, 1, 3, House::Glaushouse, false),
            (4, 1, 4, House::Flynt, false),
            (5, 2, 1, House::Stonebend, false),
            (6, 2, 2, House::Sandmanor, false),
            (7, 2, 3, House::Glaushouse, true),
            (8, 2, 4, House::Flynt, false),
            (9, 3, 1, House::Stonebend, false),
            (10, 3, 2, House::Sandmanor, false),
            (11, 3, 3, House::Glaushouse, false),
            (12, 3, 4, House::Flynt, false),
        ];

        for (identity, (position, pass, house_number, house, primary)) in
            positions.into_iter().zip(expected)
        {
            assert_eq!(identity.absolute_position().value(), position);
            assert_eq!(identity.pass().value(), pass);
            assert_eq!(identity.house_number().value(), house_number);
            assert_eq!(identity.house(), house);
            assert_eq!(identity.is_primary_anchor(), primary);
        }
    }

    #[test]
    fn position_six_and_seven_keep_the_threshold_and_pole_distinction() {
        let six = position_identity(RotationPosition::six());
        assert_eq!(six.pass(), RotationPass::Two);
        assert_eq!(six.house_number(), HouseNumber::Two);
        assert_eq!(six.house(), House::Sandmanor);
        assert_eq!(
            six.threshold().map(|threshold| threshold.toward_house()),
            Some(House::Glaushouse)
        );
        assert!(!six.is_primary_anchor());

        let seven_identity = position_identity(RotationPosition::seven());
        assert_eq!(seven_identity.pass(), RotationPass::Two);
        assert_eq!(seven_identity.house_number(), HouseNumber::Three);
        assert_eq!(seven_identity.house(), House::Glaushouse);
        assert_eq!(seven_identity.threshold(), None);
        assert!(seven_identity.is_primary_anchor());
        assert_eq!(
            opposite_position(RotationPosition::seven()),
            RotationPosition::one()
        );
    }

    #[test]
    fn position_twelve_marks_rotation_completion_without_automatic_ascension() {
        let twelve = position_identity(RotationPosition::twelve());
        assert_eq!(twelve.pass(), RotationPass::Three);
        assert_eq!(twelve.house_number(), HouseNumber::Four);
        assert_eq!(twelve.house(), House::Flynt);
        assert!(is_rotation_complete(RotationPosition::twelve()));
        assert_eq!(
            next_position(RotationPosition::twelve()),
            RotationPosition::one()
        );
    }

    #[test]
    fn canonical_rotation_and_rule_contract_fixtures_pass() {
        assert!(
            validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture())
                .is_empty()
        );
        assert!(
            validate_rule_of_twelve_contract(&canonical_rule_of_twelve_contract_fixture())
                .is_empty()
        );
    }

    #[test]
    fn contradiction_fixtures_fail_with_explicit_messages() {
        let contradictions = [
            (
                "Ranina position",
                RuleOfTwelveContractInput {
                    ranina_numbered_position: Some(1),
                    ..RuleOfTwelveContractInput::default()
                },
                "outside the numbered wheel",
            ),
            (
                "pass two order drift",
                RuleOfTwelveContractInput {
                    passes: vec![
                        CANONICAL_HOUSE_GRAMMAR.to_vec(),
                        vec![
                            House::Stonebend,
                            House::Glaushouse,
                            House::Sandmanor,
                            House::Flynt,
                        ],
                        CANONICAL_HOUSE_GRAMMAR.to_vec(),
                    ],
                    ..RuleOfTwelveContractInput::default()
                },
                "Pass 2 must preserve the House order",
            ),
            (
                "position six wrong house",
                RuleOfTwelveContractInput {
                    position_claims: {
                        let mut claims =
                            canonical_rule_of_twelve_contract_fixture().position_claims;
                        claims[5] = RuleOfTwelvePositionClaim {
                            absolute_position: 6,
                            pass_number: 2,
                            house_number: 3,
                            house: House::Glaushouse,
                            primary_anchor: false,
                            threshold_to: None,
                        };
                        claims
                    },
                    ..RuleOfTwelveContractInput::default()
                },
                "Position 6 must carry House number 2",
            ),
            (
                "position seven wrong house",
                RuleOfTwelveContractInput {
                    position_claims: {
                        let mut claims =
                            canonical_rule_of_twelve_contract_fixture().position_claims;
                        claims[6] = RuleOfTwelvePositionClaim {
                            absolute_position: 7,
                            pass_number: 2,
                            house_number: 2,
                            house: House::Sandmanor,
                            primary_anchor: false,
                            threshold_to: None,
                        };
                        claims
                    },
                    ..RuleOfTwelveContractInput::default()
                },
                "Position 7 must carry House number 3",
            ),
            (
                "position twelve stonebend",
                RuleOfTwelveContractInput {
                    position_claims: {
                        let mut claims =
                            canonical_rule_of_twelve_contract_fixture().position_claims;
                        claims[11] = RuleOfTwelvePositionClaim {
                            absolute_position: 12,
                            pass_number: 3,
                            house_number: 1,
                            house: House::Stonebend,
                            primary_anchor: false,
                            threshold_to: None,
                        };
                        claims
                    },
                    ..RuleOfTwelveContractInput::default()
                },
                "Position 12 must carry House number 4",
            ),
            (
                "ordinary wrap grants capacity",
                RuleOfTwelveContractInput {
                    ordinary_wrap_increases_capacity: true,
                    ..RuleOfTwelveContractInput::default()
                },
                "Ordinary angular movement cannot increase",
            ),
            (
                "entering twelve grants point squared",
                RuleOfTwelveContractInput {
                    entering_position_twelve_grants_point_squared: true,
                    ..RuleOfTwelveContractInput::default()
                },
                "Entering Position 12 cannot automatically grant Point²",
            ),
            (
                "pass four exists",
                RuleOfTwelveContractInput {
                    pass_four_inside_rotation: true,
                    ..RuleOfTwelveContractInput::default()
                },
                "Pass 4 cannot exist",
            ),
            (
                "position thirteen exists",
                RuleOfTwelveContractInput {
                    position_thirteen_exists: true,
                    ..RuleOfTwelveContractInput::default()
                },
                "No Position 13 exists",
            ),
        ];

        for (label, input, expected) in contradictions {
            let diagnostics = validate_rule_of_twelve_contract(&input);
            assert!(!diagnostics.is_empty(), "{label} should fail");
            assert!(
                diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.message.contains(expected)),
                "{label} should mention `{expected}`"
            );
        }
    }

    #[test]
    fn canonical_point_fixture_uses_glaushouse_position_and_preserves_it_across_point_squared() {
        let fixture = build_canonical_point_squared_fixture().expect("fixture should build");
        let before = fixture.point_before();
        let after = fixture.first_application().stabilized_point();

        assert_eq!(
            before.world().geometry().current_position(),
            Some(RotationPosition::seven())
        );
        assert_eq!(
            local_step_for_position(RotationPosition::seven()),
            HouseNumber::Three
        );
        assert_eq!(before.progression().stable_point_level(), 1);
        assert_eq!(after.progression().stable_point_level(), 2);
        assert_eq!(
            after.world().geometry().current_position(),
            Some(RotationPosition::seven())
        );
        assert_eq!(
            fixture.first_application().status(),
            PointSquaredApplicationStatus::Applied
        );
    }

    #[test]
    fn ordinary_wrap_preserves_ring_and_capacities() {
        let fixture = build_canonical_spiral_fixture().expect("fixture should build");
        let before = fixture.point_before();
        let wrapped = fixture.ordinary_wrap_point();
        assert_eq!(
            before.world().geometry().current_position(),
            Some(RotationPosition::twelve())
        );
        assert_eq!(
            wrapped.world().geometry().current_position(),
            Some(RotationPosition::one())
        );
        assert_eq!(
            wrapped.progression().stable_point_level(),
            before.progression().stable_point_level()
        );
        assert_eq!(
            wrapped.progression().capacities(),
            before.progression().capacities()
        );
    }

    #[test]
    fn canonical_spiral_transition_requires_position_twelve_and_applies_exactly_once() {
        let fixture = build_canonical_spiral_fixture().expect("fixture should build");
        let before = fixture.point_before();
        let after = fixture.first_application().stabilized_point();

        assert_eq!(
            before.world().geometry().current_position(),
            Some(RotationPosition::twelve())
        );
        assert_eq!(
            after.world().geometry().current_position(),
            Some(RotationPosition::one())
        );
        assert_eq!(after.progression().stable_point_level(), 2);
        assert_eq!(after.progression().capacities().current_capacity(), 2);
        assert_eq!(after.progression().capacities().aura_capacity(), 2);
        assert_eq!(
            fixture.second_application().status(),
            PointSquaredApplicationStatus::AlreadyApplied
        );
    }

    #[test]
    fn point_can_locate_at_center_or_rotational_coordinate() {
        let center_point = Point::with_domain_state(
            Point::origin().frame_state().clone(),
            Point::origin().progression().clone(),
            crate::point_progression::ReachableWorldState::with_geometry(
                PointGeometryState::ranina_center(),
            ),
        );
        assert_eq!(center_point.world().geometry().current_position(), None);

        let location = PointGeometryState::at_position(RotationPosition::seven());
        assert_eq!(location.current_position(), Some(RotationPosition::seven()));
    }

    #[test]
    fn witness_and_validation_surfaces_render() {
        let witness = build_map_witness().expect("map witness should build");
        assert!(witness.contains("HOLLOW GROVE ROTATIONAL MAP WITNESS"));
        assert!(witness.contains("House Number: 3"));

        let report = build_map_validation_report().expect("map validation should build");
        assert!(report.contains("status: pass"));

        let rule_witness = build_rule_of_twelve_witness().expect("rule witness should build");
        assert!(rule_witness.contains("HOLLOW GROVE RULE OF TWELVE"));
        assert!(rule_witness.contains("4 Houses × 3 Passes = 12 Positions"));

        let rule_report =
            build_rule_of_twelve_validation_report().expect("rule validation should build");
        assert!(rule_report.contains("status: pass"));

        let artifact = build_map_artifact().expect("map artifact should build");
        assert!(artifact.contains("Four Houses form one grammar."));
    }

    #[test]
    fn proxy_only_fixture_renders_the_canonical_local_address() {
        let proxy = Proxy::new(
            House::Sandmanor,
            RelativeDirection::South,
            SpatialGeometry::Flat,
            Proximity::Proximal,
            None,
            None,
        );

        assert_eq!(proxy.render(), "Proximal Flat south of Sandmanor");
        assert_eq!(proxy.manager(), crate::Manager::Clouseau);
        assert_eq!(proxy.domain(), crate::ManagerDomain::Pleb);
    }

    #[test]
    fn round_proxy_with_moxy_renders_bond_without_losing_grounded_location() {
        let proxy = Proxy::new(
            House::Stonebend,
            RelativeDirection::Northwest,
            SpatialGeometry::Round,
            Proximity::Distal,
            Some(crate::point_progression::CanonicalRouteId::StairwayToHeaven),
            None,
        );
        let moxy = Moxy::new(
            proxy.clone(),
            Some(House::Flynt),
            MoxyRelation::Bond,
            Some(crate::point_progression::CanonicalRouteId::StairwayToHeaven),
            crate::Manager::Hal,
        );

        assert_eq!(proxy.render(), "Distal Round northwest of Stonebend");
        assert_eq!(
            moxy.render(),
            "Bond toward Flynt through Stairway to Heaven"
        );
        assert_eq!(moxy.manager(), crate::Manager::Hal);
        assert_eq!(moxy.domain(), crate::ManagerDomain::Meta);
    }

    #[test]
    fn inverted_proxy_with_foxy_keeps_proxy_grounded_and_foxy_reflective() {
        let proxy = Proxy::new(
            House::Flynt,
            RelativeDirection::South,
            SpatialGeometry::Inverted,
            Proximity::Proximal,
            None,
            None,
        );
        let foxy = Foxy::new(
            FoxySource::Proxy(proxy.clone()),
            ReflectionKind::InvertedReturn,
            crate::Manager::Cleopatra,
        );

        assert_eq!(proxy.render(), "Proximal Inverted south of Flynt");
        assert_eq!(
            foxy.render(),
            "Inverted reflection of Proximal Inverted south of Flynt"
        );
        assert_eq!(foxy.manager(), crate::Manager::Cleopatra);
        assert_eq!(foxy.domain(), crate::ManagerDomain::Blep);
    }

    #[test]
    fn canonical_player_spatial_fixture_proves_proxy_anchor_and_alignment_can_differ() {
        let fixture =
            build_canonical_player_spatial_fixture().expect("player spatial fixture should build");
        let interpretation = fixture.interpretation();
        let proxy = interpretation.proxy().expect("Proxy should exist");
        let moxy = interpretation.moxy().expect("Moxy should exist");

        assert_eq!(fixture.point().progression().stable_point_level(), 2);
        assert_eq!(
            fixture.rotation_context().absolute_position(),
            RotationPosition::twelve()
        );
        assert_eq!(fixture.rotation_context().pass(), RotationPass::Three);
        assert_eq!(fixture.rotation_context().house_number(), HouseNumber::Four);
        assert_eq!(fixture.rotation_context().house(), House::Flynt);
        assert!(fixture.rotation_context().rotation_complete());
        assert_eq!(proxy.anchor(), House::Stonebend);
        assert_ne!(fixture.rotation_context().house(), proxy.anchor());
        assert_eq!(proxy.render(), "Distal Round northwest of Stonebend");
        assert_eq!(
            moxy.render(),
            "Bond toward Flynt through Stairway to Heaven"
        );
        assert!(interpretation.foxy().is_none());
    }

    #[test]
    fn foxy_of_moxy_forms_a_valid_reflected_bond() {
        let proxy = Proxy::new(
            House::Stonebend,
            RelativeDirection::Northwest,
            SpatialGeometry::Round,
            Proximity::Distal,
            Some(crate::point_progression::CanonicalRouteId::StairwayToHeaven),
            None,
        );
        let moxy = Moxy::new(
            proxy,
            Some(House::Flynt),
            MoxyRelation::Bond,
            Some(crate::point_progression::CanonicalRouteId::StairwayToHeaven),
            crate::Manager::Hal,
        );
        let foxy = Foxy::new(
            FoxySource::Moxy(moxy),
            ReflectionKind::InvertedReturn,
            crate::Manager::Cleopatra,
        );

        assert_eq!(
            foxy.render(),
            "Inverted reflection of the Stonebend-Flynt bond"
        );
        assert_eq!(foxy.source().kind(), FoxySourceKind::Moxy);
    }

    #[test]
    fn player_spatial_contract_and_rule_of_twelve_authority_hold_together() {
        assert!(
            validate_player_spatial_contract(&canonical_player_spatial_contract_fixture())
                .is_empty()
        );

        let witness =
            build_player_location_witness().expect("player location witness should build");
        assert!(witness.contains("House Alignment: Flynt"));
        assert!(witness.contains("Distal Round northwest of Stonebend"));

        let fixture =
            build_canonical_player_spatial_fixture().expect("player spatial fixture should build");
        let derived = derive_player_spatial_interpretation(fixture.point());
        assert_eq!(derived.proxy().expect("Proxy").anchor(), House::Stonebend);
        assert_eq!(fixture.rotation_context().house(), House::Flynt);
    }

    #[test]
    fn player_spatial_contradictions_fail_with_explicit_messages() {
        let contradictions = [
            (
                "proxy anchor missing",
                PlayerSpatialContractInput {
                    proxy: super::ProxyClaim {
                        anchor: None,
                        ..canonical_player_spatial_contract_fixture().proxy
                    },
                    ..canonical_player_spatial_contract_fixture()
                },
                "Proxy must include an anchor",
            ),
            (
                "moxy as proximity",
                PlayerSpatialContractInput {
                    moxy_as_proximity: true,
                    ..canonical_player_spatial_contract_fixture()
                },
                "Moxy cannot be used as a Proximity value",
            ),
            (
                "moxy velocity only",
                PlayerSpatialContractInput {
                    moxy: super::MoxyClaim {
                        relation: None,
                        velocity_only: true,
                        ..canonical_player_spatial_contract_fixture().moxy
                    },
                    ..canonical_player_spatial_contract_fixture()
                },
                "cannot be velocity only",
            ),
            (
                "foxy wrong manager",
                PlayerSpatialContractInput {
                    foxy: super::FoxyClaim {
                        manager: Some(crate::Manager::Hal),
                        ..canonical_player_spatial_contract_fixture().foxy
                    },
                    ..canonical_player_spatial_contract_fixture()
                },
                "Cleopatra must handle Foxy",
            ),
            (
                "proxy wrong domain",
                PlayerSpatialContractInput {
                    proxy: super::ProxyClaim {
                        domain: Some(crate::ManagerDomain::Blep),
                        ..canonical_player_spatial_contract_fixture().proxy
                    },
                    ..canonical_player_spatial_contract_fixture()
                },
                "Proxy must remain in the PLEB / Proxy domain",
            ),
            (
                "moxy wrong domain",
                PlayerSpatialContractInput {
                    moxy: super::MoxyClaim {
                        domain: Some(crate::ManagerDomain::Pleb),
                        ..canonical_player_spatial_contract_fixture().moxy
                    },
                    ..canonical_player_spatial_contract_fixture()
                },
                "Moxy must remain in the META / Moxy domain",
            ),
            (
                "foxy wrong domain",
                PlayerSpatialContractInput {
                    foxy: super::FoxyClaim {
                        domain: Some(crate::ManagerDomain::Meta),
                        ..canonical_player_spatial_contract_fixture().foxy
                    },
                    ..canonical_player_spatial_contract_fixture()
                },
                "Foxy must remain in the BLEP / Foxy domain",
            ),
            (
                "position twelve wrong house",
                PlayerSpatialContractInput {
                    coordinate_alignment_claim: Some((
                        RotationPosition::twelve(),
                        House::Stonebend,
                    )),
                    ..canonical_player_spatial_contract_fixture()
                },
                "Position 12 derives Flynt",
            ),
            (
                "proxy grants point squared",
                PlayerSpatialContractInput {
                    automatic_point_squared_from_proxy: true,
                    ..canonical_player_spatial_contract_fixture()
                },
                "Proxy creation cannot automatically grant Point²",
            ),
            (
                "moxy auto moves",
                PlayerSpatialContractInput {
                    automatic_movement_from_moxy: true,
                    ..canonical_player_spatial_contract_fixture()
                },
                "Moxy cannot automatically execute movement",
            ),
            (
                "foxy auto legalizes",
                PlayerSpatialContractInput {
                    automatic_legality_from_foxy: true,
                    ..canonical_player_spatial_contract_fixture()
                },
                "cannot automatically mark a Recipe legal",
            ),
            (
                "round proxy forbidden",
                PlayerSpatialContractInput {
                    round_proxy_forbidden: true,
                    ..canonical_player_spatial_contract_fixture()
                },
                "Round locations cannot be excluded from Proxy",
            ),
        ];

        for (label, input, expected) in contradictions {
            let diagnostics = validate_player_spatial_contract(&input);
            assert!(!diagnostics.is_empty(), "{label} should fail");
            assert!(
                diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.message.contains(expected)),
                "{label} should mention `{expected}`"
            );
        }
    }

    #[test]
    fn non_completion_position_cannot_select_spiral_transition() {
        let fixture = build_canonical_point_squared_fixture().expect("fixture should build");
        let error = select_canonical_spiral_transition(fixture.point_before(), fixture.ascension())
            .expect_err("position 7 should not spiral");
        assert!(error.to_string().contains("Position 12"));
    }
}
