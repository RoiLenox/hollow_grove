use std::fmt;
use std::io;

use crate::frame_state::BeingId;
use crate::hollow_grove_contract::{AlignmentDiagnostic, AlignmentDiagnosticCode, House};
use crate::point_progression::{
    CanonicalRouteId, PointSquaredApplicationStatus, build_canonical_point_squared_fixture,
};

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
}

impl Default for PointGeometryState {
    fn default() -> Self {
        Self::origin()
    }
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

pub fn canonical_rotation_contract_fixture() -> HollowGroveRotationContractInput {
    HollowGroveRotationContractInput::default()
}

pub fn validate_hollow_grove_rotation_contract(
    input: &HollowGroveRotationContractInput,
) -> Vec<AlignmentDiagnostic> {
    let mut diagnostics = Vec::new();

    if !input.ranina_is_exact_center {
        diagnostics.push(geometry_error("Ranina must remain the exact center."));
    }
    if let Some(position) = input.ranina_numbered_position {
        diagnostics.push(geometry_error(format!(
            "Ranina is the unnumbered center and cannot be assigned numbered position {position}."
        )));
    }
    if input.ranina_is_house {
        diagnostics.push(geometry_error(
            "Ranina is the center and cannot be promoted into a fifth House.",
        ));
    }
    if input.rotation_positions != RotationPosition::COUNT {
        diagnostics.push(geometry_error(format!(
            "A full Hollow Grove rotation must contain exactly 12 positions, got {}.",
            input.rotation_positions
        )));
    }
    if input.stonebend_position != 1 {
        diagnostics.push(geometry_error(format!(
            "Stonebend must anchor Position 1, got Position {}.",
            input.stonebend_position
        )));
    }
    if input.glaushouse_threshold_position != 6 {
        diagnostics.push(geometry_error(format!(
            "Position 6 is the Glaüshouse threshold, got Position {}.",
            input.glaushouse_threshold_position
        )));
    }
    if input.glaushouse_position != 7 {
        diagnostics.push(geometry_error(format!(
            "Glaüshouse must anchor Position 7, got Position {}.",
            input.glaushouse_position
        )));
    }

    match (
        RotationPosition::new(input.stonebend_position),
        RotationPosition::new(input.glaushouse_position),
    ) {
        (Some(stonebend), Some(glaushouse)) if stonebend.opposite() != glaushouse => {
            diagnostics.push(geometry_error(format!(
                "Stonebend Position {} and Glaüshouse Position {} must be opposites across Ranina.",
                stonebend, glaushouse
            )));
        }
        (Some(_), Some(_)) => {}
        _ => diagnostics.push(geometry_error(
            "Stonebend and Glaüshouse positions must both be valid positions inside 1..=12.",
        )),
    }

    if !input.point_squared_opens_next_ring {
        diagnostics.push(geometry_error(
            "Point² must open the next ring around the same center.",
        ));
    }
    if input.point_squared_rotates_automatically {
        diagnostics.push(geometry_error(
            "Point² cannot rotate the Point automatically; position and ring remain distinct.",
        ));
    }
    if input.moving_one_position_grants_capacity {
        diagnostics.push(geometry_error(
            "Moving from one position to another cannot grant Point² capacity advancement.",
        ));
    }
    if input.point_squared_creates_new_center {
        diagnostics.push(geometry_error(
            "Point² cannot create a new center; Ranina remains fixed.",
        ));
    }
    if input.multiple_centers_exist {
        diagnostics.push(geometry_error(
            "Each ring must rotate around the same Ranina center; multiple centers are forbidden.",
        ));
    }
    if input.point_squared_creates_horizon_squared {
        diagnostics.push(geometry_error(
            "Point² already names the opened horizon and cannot create a separate Horizon² state.",
        ));
    }
    if !input.distinct_ring_and_position {
        diagnostics.push(geometry_error(
            "Ring and position must remain distinct so radial progression does not collapse into angular movement.",
        ));
    }

    diagnostics
}

pub fn build_map_witness() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let point_before = fixture.point_before();
    let point_after = fixture.first_application().stabilized_point();
    let before_position = point_before
        .world()
        .geometry()
        .current_position()
        .map(|position| position.to_string())
        .unwrap_or_else(|| String::from("none"));
    let after_position = point_after
        .world()
        .geometry()
        .current_position()
        .map(|position| position.to_string())
        .unwrap_or_else(|| String::from("none"));

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
         Capricorn\n\
         Sea-Goat\n\
         Hollow Current\n\
         Life held in form\n\n\
         Position 6:\n\
         Glaüshouse Threshold\n\
         Descent toward Abyss\n\n\
         Position 7:\n\
         Glaüshouse\n\
         Cancer\n\
         Crab\n\
         Abyss\n\
         Life felt in depth\n\n\
         Opposition:\n\
         Stonebend 1 ↔ Glaüshouse 7\n\
         Separation: 6 positions\n\
         Angle: 180 degrees\n\n\
         Point Fixture:\n\
         Before Point²: ring {} / position {}\n\
         After Point²: ring {} / position {}\n\
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
        before_position,
        point_after.progression().stable_point_level(),
        after_position,
        point_before.world().geometry().current_position()
            == point_after.world().geometry().current_position(),
        point_after.being() == BeingId::Hueman,
    ))
}

pub fn build_map_validation_report() -> io::Result<String> {
    let contract_diagnostics =
        validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture());
    let fixture = build_canonical_point_squared_fixture()?;
    let before = fixture.point_before();
    let after = fixture.first_application().stabilized_point();
    let mut errors = contract_diagnostics;

    if before.world().geometry().center() != WorldCenterId::Ranina
        || after.world().geometry().center() != WorldCenterId::Ranina
    {
        errors.push(geometry_error(
            "Ranina must remain the unique center before and after Point².",
        ));
    }
    if before.world().geometry().current_position() != after.world().geometry().current_position() {
        errors.push(geometry_error(
            "Point² cannot rotate the Point automatically when no movement occurs.",
        ));
    }
    if after.progression().stable_point_level() != before.progression().stable_point_level() + 1 {
        errors.push(geometry_error(
            "Point² must open the next ring by raising the stabilized Point level by one.",
        ));
    }
    if fixture.first_application().status() != PointSquaredApplicationStatus::Applied {
        errors.push(geometry_error(
            "The canonical map fixture requires one legally applied Point² ascension.",
        ));
    }

    if errors.is_empty() {
        Ok(String::from(
            "# Hollow Grove Rotational Map Validation\n\n\
             - status: pass\n\
             - unique Ranina center: pass\n\
             - twelve-position rotation: pass\n\
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

pub fn build_map_artifact() -> io::Result<String> {
    let fixture = build_canonical_point_squared_fixture()?;
    let point_after = fixture.first_application().stabilized_point();
    Ok(format!(
        "HOLLOW GROVE RANINA / TWELVE-POSITION MAP ARTIFACT\n\n\
         Canonical Doctrine:\n\
         Ranina is the center.\n\
         Stonebend is the first form.\n\
         Glaüshouse is the opposite depth.\n\
         Twelve positions complete the rotation.\n\
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
         - Position 1 = Stonebend = Capricorn = Sea-Goat = Hollow Current\n\
         - Position 6 = Glaüshouse threshold\n\
         - Position 7 = Glaüshouse = Cancer = Crab = Abyss\n\n\
         Canonical Point² Fixture:\n\
         - before: ring {} / position {}\n\
         - after: ring {} / position {}\n\
         - current capacity: {}\n\
         - aura capacity: {}\n\
         - Stairway to Heaven visible: {}\n\
         - Stairway to Heaven survivable: {}\n\
         - destination: Stonebend / Position 1\n",
        fixture.point_before().progression().stable_point_level(),
        fixture
            .point_before()
            .world()
            .geometry()
            .current_position()
            .map(|position| position.to_string())
            .unwrap_or_else(|| String::from("none")),
        point_after.progression().stable_point_level(),
        point_after
            .world()
            .geometry()
            .current_position()
            .map(|position| position.to_string())
            .unwrap_or_else(|| String::from("none")),
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

fn geometry_error(message: impl Into<String>) -> AlignmentDiagnostic {
    AlignmentDiagnostic {
        code: AlignmentDiagnosticCode::RotationalMapMismatch,
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
        HollowGroveRotationContractInput, PointGeometryState, PointLocation, RotationPosition,
        WorldCenterId, build_map_artifact, build_map_validation_report, build_map_witness,
        canonical_rotation_contract_fixture, glaushouse_anchor_position,
        glaushouse_threshold_position, house_anchor_for_position, stonebend_anchor_position,
        validate_hollow_grove_rotation_contract,
    };
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
        assert_eq!(one.next().value(), 2);
        assert_eq!(six.next().value(), 7);
        assert_eq!(RotationPosition::new(12).expect("12 is valid").next(), one);
        assert_eq!(one.previous().value(), 12);
        assert_eq!(one.opposite(), seven);
        assert_eq!(seven.opposite(), one);
        assert_eq!(seven.angle_degrees() - one.angle_degrees(), 180);
    }

    #[test]
    fn canonical_house_and_threshold_anchors_are_locked() {
        assert_eq!(stonebend_anchor_position().value(), 1);
        assert_eq!(glaushouse_threshold_position().value(), 6);
        assert_eq!(glaushouse_anchor_position().value(), 7);
        assert_eq!(
            house_anchor_for_position(stonebend_anchor_position()),
            Some(crate::hollow_grove_contract::House::Stonebend)
        );
        assert_eq!(
            house_anchor_for_position(glaushouse_anchor_position()),
            Some(crate::hollow_grove_contract::House::Glaushouse)
        );
        assert_eq!(
            house_anchor_for_position(glaushouse_threshold_position()),
            None
        );
    }

    #[test]
    fn canonical_rotation_contract_fixture_passes() {
        assert!(
            validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture())
                .is_empty()
        );
    }

    #[test]
    fn contradiction_fixtures_fail_with_explicit_messages() {
        let contradictions = [
            (
                "Ranina position",
                HollowGroveRotationContractInput {
                    ranina_numbered_position: Some(1),
                    ..HollowGroveRotationContractInput::default()
                },
                "unnumbered center",
            ),
            (
                "bad rotation count",
                HollowGroveRotationContractInput {
                    rotation_positions: 10,
                    ..HollowGroveRotationContractInput::default()
                },
                "exactly 12 positions",
            ),
            (
                "stonebend drift",
                HollowGroveRotationContractInput {
                    stonebend_position: 2,
                    ..HollowGroveRotationContractInput::default()
                },
                "Stonebend must anchor Position 1",
            ),
            (
                "glaushouse pole drift",
                HollowGroveRotationContractInput {
                    glaushouse_position: 6,
                    ..HollowGroveRotationContractInput::default()
                },
                "Glaüshouse must anchor Position 7",
            ),
            (
                "automatic rotation",
                HollowGroveRotationContractInput {
                    point_squared_rotates_automatically: true,
                    ..HollowGroveRotationContractInput::default()
                },
                "cannot rotate the Point automatically",
            ),
            (
                "position walk ascension",
                HollowGroveRotationContractInput {
                    moving_one_position_grants_capacity: true,
                    ..HollowGroveRotationContractInput::default()
                },
                "cannot grant Point² capacity advancement",
            ),
            (
                "new center",
                HollowGroveRotationContractInput {
                    point_squared_creates_new_center: true,
                    ..HollowGroveRotationContractInput::default()
                },
                "cannot create a new center",
            ),
            (
                "horizon squared",
                HollowGroveRotationContractInput {
                    point_squared_creates_horizon_squared: true,
                    ..HollowGroveRotationContractInput::default()
                },
                "Horizon²",
            ),
        ];

        for (label, input, expected) in contradictions {
            let diagnostics = validate_hollow_grove_rotation_contract(&input);
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
    fn point_can_locate_at_center_or_rotational_coordinate() {
        let center_point = Point::with_domain_state(
            Point::origin().frame_state().clone(),
            Point::origin().progression().clone(),
            crate::point_progression::ReachableWorldState::with_geometry(
                PointGeometryState::ranina_center(),
            ),
        );
        assert_eq!(center_point.world().geometry().current_position(), None);

        let location = PointGeometryState::new(
            WorldCenterId::Ranina,
            PointLocation::RingPosition(RotationPosition::seven()),
        );
        assert_eq!(location.current_position(), Some(RotationPosition::seven()));
    }

    #[test]
    fn witness_and_validation_surfaces_render() {
        let witness = build_map_witness().expect("map witness should build");
        assert!(witness.contains("HOLLOW GROVE ROTATIONAL MAP WITNESS"));
        assert!(witness.contains("Ranina"));
        assert!(witness.contains("Stonebend 1 ↔ Glaüshouse 7"));

        let report = build_map_validation_report().expect("map validation should build");
        assert!(report.contains("status: pass"));

        let artifact = build_map_artifact().expect("map artifact should build");
        assert!(artifact.contains("HOLLOW GROVE RANINA / TWELVE-POSITION MAP ARTIFACT"));
    }
}
