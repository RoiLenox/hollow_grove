//! The one canonical Aura Beach and its coastal working places.
//!
//! Aura Beach is attributed first to Sandmanor's Minoan exterior tradition.
//! Current Sea is its linked civic-circulation jurisdiction, not a second
//! beach or a maritime body.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

use super::geography::ConstitutionalRouteId;
use super::interior_surface::{InteriorSurfaceId, SurfacePoint};

pub const AURA_BEACH_MAP_ID: &str = "aura-beach.coastal-commons";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraBeachFacilityId {
    CoastalProvingStrand,
    PublicApproach,
    NavigationSchool,
    Beacon,
    TideStation,
    WeatherStation,
    RescuePost,
    CentaurRun,
    ElfDesignYard,
    BoatLanding,
    PublicPier,
    RecoveryPavilion,
    DuneWard,
    FishMarket,
    SalvageYard,
}

impl AuraBeachFacilityId {
    pub const ALL: [Self; 15] = [
        Self::CoastalProvingStrand,
        Self::PublicApproach,
        Self::NavigationSchool,
        Self::Beacon,
        Self::TideStation,
        Self::WeatherStation,
        Self::RescuePost,
        Self::CentaurRun,
        Self::ElfDesignYard,
        Self::BoatLanding,
        Self::PublicPier,
        Self::RecoveryPavilion,
        Self::DuneWard,
        Self::FishMarket,
        Self::SalvageYard,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::CoastalProvingStrand => "aura-beach.facility.coastal-proving-strand",
            Self::PublicApproach => "aura-beach.facility.public-approach",
            Self::NavigationSchool => "aura-beach.facility.navigation-school",
            Self::Beacon => "aura-beach.facility.beacon",
            Self::TideStation => "aura-beach.facility.tide-station",
            Self::WeatherStation => "aura-beach.facility.weather-station",
            Self::RescuePost => "aura-beach.facility.rescue-post",
            Self::CentaurRun => "aura-beach.facility.centaur-run",
            Self::ElfDesignYard => "aura-beach.facility.elf-design-yard",
            Self::BoatLanding => "aura-beach.facility.boat-landing",
            Self::PublicPier => "aura-beach.facility.public-pier",
            Self::RecoveryPavilion => "aura-beach.facility.recovery-pavilion",
            Self::DuneWard => "aura-beach.facility.dune-ward",
            Self::FishMarket => "aura-beach.facility.fish-market",
            Self::SalvageYard => "aura-beach.facility.salvage-yard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraBeachFacilityKind {
    ProvingStrand,
    PublicApproach,
    Navigation,
    Beacon,
    TideObservation,
    WeatherObservation,
    Rescue,
    CentaurMobility,
    ElfDesign,
    Landing,
    Pier,
    Recovery,
    DuneProtection,
    CoastalMarket,
    Salvage,
}

impl AuraBeachFacilityKind {
    pub const REQUIRED: [Self; 15] = [
        Self::ProvingStrand,
        Self::PublicApproach,
        Self::Navigation,
        Self::Beacon,
        Self::TideObservation,
        Self::WeatherObservation,
        Self::Rescue,
        Self::CentaurMobility,
        Self::ElfDesign,
        Self::Landing,
        Self::Pier,
        Self::Recovery,
        Self::DuneProtection,
        Self::CoastalMarket,
        Self::Salvage,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvingStrand => "coastal proving strand",
            Self::PublicApproach => "public approach",
            Self::Navigation => "navigation school",
            Self::Beacon => "beacon",
            Self::TideObservation => "tide observation",
            Self::WeatherObservation => "weather observation",
            Self::Rescue => "shore rescue",
            Self::CentaurMobility => "Centaur mobility run",
            Self::ElfDesign => "Elf exterior-design yard",
            Self::Landing => "boat landing",
            Self::Pier => "public pier",
            Self::Recovery => "recovery pavilion",
            Self::DuneProtection => "dune ward",
            Self::CoastalMarket => "coastal market",
            Self::Salvage => "salvage yard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuraBeachFacility {
    pub id: AuraBeachFacilityId,
    pub name: &'static str,
    pub kind: AuraBeachFacilityKind,
    pub function: &'static str,
}

impl AuraBeachFacility {
    const fn new(
        id: AuraBeachFacilityId,
        name: &'static str,
        kind: AuraBeachFacilityKind,
        function: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            kind,
            function,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuraBeachHouseRole {
    pub house: House,
    pub responsibility: &'static str,
    pub limit: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraBeachSurface {
    pub id: InteriorSurfaceId,
    pub map_id: &'static str,
    pub dominant_house: House,
    pub regional_attribution: &'static str,
    pub boundary: [SurfacePoint; 3],
    pub access_routes: [ConstitutionalRouteId; 4],
    pub facilities: Vec<AuraBeachFacility>,
    pub house_roles: [AuraBeachHouseRole; 4],
}

impl AuraBeachSurface {
    #[must_use]
    pub fn facility(&self, id: AuraBeachFacilityId) -> Option<&AuraBeachFacility> {
        self.facilities.iter().find(|facility| facility.id == id)
    }

    #[must_use]
    pub fn permits_route_entry(&self, route: ConstitutionalRouteId) -> bool {
        self.id.permits_route_entry(route)
    }

    pub fn validate(&self) -> Result<(), AuraBeachError> {
        if self.id != InteriorSurfaceId::AuraBeach
            || self.map_id != AURA_BEACH_MAP_ID
            || self.dominant_house != House::Sandmanor
            || self.regional_attribution
                != "Sandmanor Minoan exterior coastal design and mobility proving ground"
        {
            return Err(AuraBeachError::IdentityOrAttributionMismatch);
        }
        if self.boundary
            != [
                SurfacePoint {
                    x_per_mille: 735,
                    y_per_mille: 500,
                },
                SurfacePoint {
                    x_per_mille: 500,
                    y_per_mille: 860,
                },
                SurfacePoint {
                    x_per_mille: 500,
                    y_per_mille: 500,
                },
            ]
        {
            return Err(AuraBeachError::BoundaryMismatch);
        }
        if self.access_routes.as_slice() != self.id.access_routes() {
            return Err(AuraBeachError::AccessRouteMismatch);
        }
        validate_facilities(&self.facilities)?;
        validate_house_roles(&self.house_roles)?;
        Ok(())
    }
}

pub fn canonical_aura_beach() -> Result<AuraBeachSurface, AuraBeachError> {
    use AuraBeachFacilityId as Id;
    use AuraBeachFacilityKind as Kind;

    let surface = AuraBeachSurface {
        id: InteriorSurfaceId::AuraBeach,
        map_id: AURA_BEACH_MAP_ID,
        dominant_house: House::Sandmanor,
        regional_attribution: "Sandmanor Minoan exterior coastal design and mobility proving ground",
        boundary: [
            SurfacePoint {
                x_per_mille: 735,
                y_per_mille: 500,
            },
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 860,
            },
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 500,
            },
        ],
        access_routes: [
            ConstitutionalRouteId::AuraRidge,
            ConstitutionalRouteId::CurrentSea,
            ConstitutionalRouteId::Glausbahn,
            ConstitutionalRouteId::CurrentSeanad,
        ],
        facilities: vec![
            AuraBeachFacility::new(
                Id::CoastalProvingStrand,
                "Coastal Proving Strand",
                Kind::ProvingStrand,
                "repeatable trials of coastal access, visibility, movement, and shoreline impact",
            ),
            AuraBeachFacility::new(
                Id::PublicApproach,
                "Common Shore Approach",
                Kind::PublicApproach,
                "accessible arrival, public meeting, orientation, and safe departure",
            ),
            AuraBeachFacility::new(
                Id::NavigationSchool,
                "Minoan Navigation School",
                Kind::Navigation,
                "charts, bearings, current, weather, communication, and return planning",
            ),
            AuraBeachFacility::new(
                Id::Beacon,
                "Aura Beach Beacon",
                Kind::Beacon,
                "public position, hazard, and return signal without recognition authority",
            ),
            AuraBeachFacility::new(
                Id::TideStation,
                "Tide and Current Station",
                Kind::TideObservation,
                "tide, current, depth, erosion, and shoreline-change records",
            ),
            AuraBeachFacility::new(
                Id::WeatherStation,
                "Coastal Weather Station",
                Kind::WeatherObservation,
                "wind, pressure, light, storm, visibility, and Aura exposure",
            ),
            AuraBeachFacility::new(
                Id::RescuePost,
                "Shore Rescue Post",
                Kind::Rescue,
                "watch, retrieval, stabilization, consent, and transfer to qualified care",
            ),
            AuraBeachFacility::new(
                Id::CentaurRun,
                "Centaur Mobility Run",
                Kind::CentaurMobility,
                "roaming, escort, patrol, changing terrain, and public mobility proof",
            ),
            AuraBeachFacility::new(
                Id::ElfDesignYard,
                "Elf Exterior-Design Yard",
                Kind::ElfDesign,
                "provisional approaches, shelters, signals, craft, and transport forms",
            ),
            AuraBeachFacility::new(
                Id::BoatLanding,
                "Common Boat Landing",
                Kind::Landing,
                "launch, landing, passenger transfer, cargo custody, and route continuity",
            ),
            AuraBeachFacility::new(
                Id::PublicPier,
                "Public Pier",
                Kind::Pier,
                "shared shore access, inspection, loading, fishing, and observation",
            ),
            AuraBeachFacility::new(
                Id::RecoveryPavilion,
                "Recovery Pavilion",
                Kind::Recovery,
                "shade, water, warming, rest, first response, and protected discharge",
            ),
            AuraBeachFacility::new(
                Id::DuneWard,
                "Living Dune Ward",
                Kind::DuneProtection,
                "erosion control, habitat, storm buffering, and marked foot access",
            ),
            AuraBeachFacility::new(
                Id::FishMarket,
                "Shore Fish Market",
                Kind::CoastalMarket,
                "landing records, weights, temperature, provenance, prices, and reserves",
            ),
            AuraBeachFacility::new(
                Id::SalvageYard,
                "Shore Salvage Yard",
                Kind::Salvage,
                "recovered craft, cargo, material custody, hazards, claims, and repair triage",
            ),
        ],
        house_roles: [
            AuraBeachHouseRole {
                house: House::Sandmanor,
                responsibility: "proves Minoan exterior design, access, navigation, mobility, and shoreline relationship",
                limit: "proof never creates Synthesis, Clearance, recognition, office, or Title",
            },
            AuraBeachHouseRole {
                house: House::Stonebend,
                responsibility: "names shoreline boundaries, vessels, lots, persons, continuities, and claims",
                limit: "a Name does not prove seaworthiness, safety, ownership, or competence",
            },
            AuraBeachHouseRole {
                house: House::Glaushouse,
                responsibility: "clears food, water, rescue, exposure, injury, and recovery hazards",
                limit: "first response and Clearance do not establish coastal proof or standing",
            },
            AuraBeachHouseRole {
                house: House::Flynt,
                responsibility: "recognizes public service, exchanges, completed functions, and resolved claims",
                limit: "recognition does not manufacture design proof, consent, or ownership",
            },
        ],
    };
    surface.validate()?;
    Ok(surface)
}

fn validate_facilities(facilities: &[AuraBeachFacility]) -> Result<(), AuraBeachError> {
    let mut ids = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for facility in facilities {
        if !ids.insert(facility.id.stable_id()) {
            return Err(AuraBeachError::DuplicateFacility(facility.id));
        }
        kinds.insert(facility.kind);
    }
    if ids.len() != AuraBeachFacilityId::ALL.len()
        || AuraBeachFacilityKind::REQUIRED
            .into_iter()
            .any(|kind| !kinds.contains(&kind))
    {
        return Err(AuraBeachError::IncompleteFacilitySystem);
    }
    Ok(())
}

fn validate_house_roles(roles: &[AuraBeachHouseRole; 4]) -> Result<(), AuraBeachError> {
    if roles.iter().enumerate().any(|(index, role)| {
        roles[index + 1..]
            .iter()
            .any(|other| other.house == role.house)
    }) {
        return Err(AuraBeachError::HouseRoleMismatch);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuraBeachError {
    IdentityOrAttributionMismatch,
    BoundaryMismatch,
    AccessRouteMismatch,
    DuplicateFacility(AuraBeachFacilityId),
    IncompleteFacilitySystem,
    HouseRoleMismatch,
}

impl fmt::Display for AuraBeachError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Aura Beach contract rejected: {self:?}")
    }
}

impl std::error::Error for AuraBeachError {}
