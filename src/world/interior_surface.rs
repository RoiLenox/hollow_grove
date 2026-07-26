//! Shared identity, geometry, and route-entry law for interior world surfaces.
//!
//! Surfaces are bounded fill regions, never routes and never substitute House
//! authority. Their display names are singular geographic identities.

use serde::{Deserialize, Serialize};

use super::geography::ConstitutionalRouteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InteriorSurfaceId {
    AuraField,
    AuraBeach,
    AuraBasin,
}

impl InteriorSurfaceId {
    pub const ALL: [Self; 3] = [Self::AuraField, Self::AuraBeach, Self::AuraBasin];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::AuraField => "aura-field",
            Self::AuraBeach => "aura-beach",
            Self::AuraBasin => "aura-basin",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::AuraField => "Aura Field",
            Self::AuraBeach => "Aura Beach",
            Self::AuraBasin => "Aura Basin",
        }
    }

    #[must_use]
    pub const fn access_routes(self) -> &'static [ConstitutionalRouteId] {
        match self {
            Self::AuraField => &[
                ConstitutionalRouteId::AuraRidge,
                ConstitutionalRouteId::AuraWay,
                ConstitutionalRouteId::MntAura,
            ],
            Self::AuraBeach => &[
                ConstitutionalRouteId::AuraRidge,
                ConstitutionalRouteId::CurrentSea,
                ConstitutionalRouteId::Glausbahn,
                ConstitutionalRouteId::CurrentSeanad,
            ],
            Self::AuraBasin => &[
                ConstitutionalRouteId::AuraRidge,
                ConstitutionalRouteId::CurrentSea,
                ConstitutionalRouteId::Boardwalk,
                ConstitutionalRouteId::Riptide,
                ConstitutionalRouteId::BasinMotorspeedway,
                ConstitutionalRouteId::StairwayToHeaven,
            ],
        }
    }

    #[must_use]
    pub fn permits_route_entry(self, route: ConstitutionalRouteId) -> bool {
        self.access_routes().contains(&route)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfacePoint {
    pub x_per_mille: u16,
    pub y_per_mille: u16,
}
