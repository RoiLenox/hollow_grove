//! Traversable constitutional route skeleton above route law and below maps.
//!
//! Geometry answers how a route is shaped. The constitutional geography
//! module remains authoritative over why the route exists. The implemented
//! Aura Field, Aura Beach, and Aura Basin contracts live in sibling modules
//! and are deliberately absent from the route-segment count.
//!
//! The Aura Way Design corridor, Current Sea civic interface, and Riptide
//! intake segment are archive-compatible map projections. Their screen
//! topology cannot redefine permanent Aura Way direction, the Current Sea
//! body, Riptide/Undertow force identity, or the Boardwalk traveler route.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hollow_grove_contract::House;

use super::geography::{
    ConstitutionalFlowDirection, ConstitutionalGeography, ConstitutionalRouteId,
    canonical_constitutional_geography,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RouteGeometryClass {
    Straight,
    /// The world-facing Round class rendered as a curved route.
    Round,
    SeaOrdeal,
}

impl RouteGeometryClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Straight => "Straight",
            Self::Round => "Round",
            Self::SeaOrdeal => "Sea Ordeal",
        }
    }

    #[must_use]
    pub const fn presentation_term(self) -> &'static str {
        match self {
            Self::Straight => "straight",
            Self::Round => "curved",
            Self::SeaOrdeal => "crossing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HouseMapAnchor {
    pub house: House,
    pub x_per_mille: u16,
    pub y_per_mille: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteNetworkSegment {
    pub route: ConstitutionalRouteId,
    pub geometry: RouteGeometryClass,
    pub endpoints: [House; 2],
    pub start: HouseMapAnchor,
    pub end: HouseMapAnchor,
}

impl RouteNetworkSegment {
    #[must_use]
    pub const fn contains_house(&self, house: House) -> bool {
        house_eq(self.endpoints[0], house) || house_eq(self.endpoints[1], house)
    }

    #[must_use]
    pub const fn shares_endpoint(&self, other: &Self) -> bool {
        self.contains_house(other.endpoints[0]) || self.contains_house(other.endpoints[1])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteNetwork {
    segments: Vec<RouteNetworkSegment>,
}

impl RouteNetwork {
    pub fn canonical() -> Result<Self, RouteNetworkError> {
        let geography = canonical_constitutional_geography()
            .map_err(|error| RouteNetworkError::Geography(error.to_string()))?;
        Self::from_geography(&geography)
    }

    pub fn from_geography(geography: &ConstitutionalGeography) -> Result<Self, RouteNetworkError> {
        let segments = geography
            .routes()
            .iter()
            .map(|definition| {
                let endpoints = match definition.direction {
                    ConstitutionalFlowDirection::Directed { from, to } => [from, to],
                    ConstitutionalFlowDirection::Reciprocal
                    | ConstitutionalFlowDirection::SharedDeliberation => {
                        definition.boundary.houses()
                    }
                };
                Ok(RouteNetworkSegment {
                    route: definition.id,
                    geometry: canonical_route_geometry(definition.id),
                    endpoints,
                    start: house_anchor(endpoints[0]),
                    end: house_anchor(endpoints[1]),
                })
            })
            .collect::<Result<Vec<_>, RouteNetworkError>>()?;
        let network = Self { segments };
        network.validate(geography)?;
        Ok(network)
    }

    #[must_use]
    pub fn segments(&self) -> &[RouteNetworkSegment] {
        &self.segments
    }

    #[must_use]
    pub fn segment(&self, route: ConstitutionalRouteId) -> Option<&RouteNetworkSegment> {
        self.segments.iter().find(|segment| segment.route == route)
    }

    #[must_use]
    pub fn routes_by_geometry(&self, geometry: RouteGeometryClass) -> Vec<ConstitutionalRouteId> {
        self.segments
            .iter()
            .filter(|segment| segment.geometry == geometry)
            .map(|segment| segment.route)
            .collect()
    }

    #[must_use]
    pub fn can_transfer(&self, from: ConstitutionalRouteId, to: ConstitutionalRouteId) -> bool {
        let (Some(from), Some(to)) = (self.segment(from), self.segment(to)) else {
            return false;
        };
        from.route == to.route || from.shares_endpoint(to)
    }

    #[must_use]
    pub fn transfer_house(
        &self,
        from: ConstitutionalRouteId,
        to: ConstitutionalRouteId,
    ) -> Option<House> {
        let from = self.segment(from)?;
        let to = self.segment(to)?;
        from.endpoints
            .into_iter()
            .find(|house| to.contains_house(*house))
    }

    pub fn validate(&self, geography: &ConstitutionalGeography) -> Result<(), RouteNetworkError> {
        let mut routes = BTreeSet::new();
        let mut geometry_counts = BTreeMap::new();
        for segment in &self.segments {
            if !routes.insert(segment.route) {
                return Err(RouteNetworkError::DuplicateRoute(segment.route));
            }
            let definition = geography
                .route(segment.route)
                .ok_or(RouteNetworkError::MissingRoute(segment.route))?;
            if !definition.boundary.contains(segment.endpoints[0])
                || !definition.boundary.contains(segment.endpoints[1])
                || house_eq(segment.endpoints[0], segment.endpoints[1])
                || segment.start != house_anchor(segment.endpoints[0])
                || segment.end != house_anchor(segment.endpoints[1])
                || segment.geometry != canonical_route_geometry(segment.route)
            {
                return Err(RouteNetworkError::SegmentMismatch(segment.route));
            }
            *geometry_counts.entry(segment.geometry).or_insert(0_usize) += 1;
        }
        for route in ConstitutionalRouteId::ALL {
            if !routes.contains(&route) {
                return Err(RouteNetworkError::MissingRoute(route));
            }
        }
        for (geometry, expected) in [
            (RouteGeometryClass::Straight, 5),
            (RouteGeometryClass::Round, 4),
            (RouteGeometryClass::SeaOrdeal, 1),
        ] {
            let actual = geometry_counts.get(&geometry).copied().unwrap_or_default();
            if actual != expected {
                return Err(RouteNetworkError::GeometryCount {
                    geometry,
                    expected,
                    actual,
                });
            }
        }
        Ok(())
    }
}

#[must_use]
pub const fn canonical_route_geometry(route: ConstitutionalRouteId) -> RouteGeometryClass {
    use ConstitutionalRouteId::{
        AuraRidge, AuraWay, BasinMotorspeedway, Boardwalk, CurrentSea, CurrentSeanad, Glausbahn,
        MntAura, Riptide, StairwayToHeaven,
    };
    match route {
        AuraRidge | AuraWay | Glausbahn | Boardwalk | BasinMotorspeedway => {
            RouteGeometryClass::Straight
        }
        MntAura | CurrentSeanad | Riptide | StairwayToHeaven => RouteGeometryClass::Round,
        CurrentSea => RouteGeometryClass::SeaOrdeal,
    }
}

#[must_use]
pub const fn house_anchor(house: House) -> HouseMapAnchor {
    match house {
        House::Stonebend => HouseMapAnchor {
            house,
            x_per_mille: 500,
            y_per_mille: 140,
        },
        House::Flynt => HouseMapAnchor {
            house,
            x_per_mille: 265,
            y_per_mille: 500,
        },
        House::Glaushouse => HouseMapAnchor {
            house,
            x_per_mille: 500,
            y_per_mille: 860,
        },
        House::Sandmanor => HouseMapAnchor {
            house,
            x_per_mille: 735,
            y_per_mille: 500,
        },
    }
}

const fn house_eq(left: House, right: House) -> bool {
    matches!(
        (left, right),
        (House::Stonebend, House::Stonebend)
            | (House::Flynt, House::Flynt)
            | (House::Glaushouse, House::Glaushouse)
            | (House::Sandmanor, House::Sandmanor)
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteNetworkError {
    Geography(String),
    DuplicateRoute(ConstitutionalRouteId),
    MissingRoute(ConstitutionalRouteId),
    SegmentMismatch(ConstitutionalRouteId),
    GeometryCount {
        geometry: RouteGeometryClass,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for RouteNetworkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Hollow Grove route network rejected: {self:?}")
    }
}

impl std::error::Error for RouteNetworkError {}
