//! The one canonical Aura Basin and its collision-and-return working places.
//!
//! Aura Basin serves Flynt first. It is not owned wholesale by Manticorp, the
//! Gallows, or any crew. Its injury and recovery loop remains constitutionally
//! linked to Glaüshouse care, while lawful Hollowing remains Stonebend work.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

use super::geography::ConstitutionalRouteId;
use super::interior_surface::{InteriorSurfaceId, SurfacePoint};

pub const AURA_BASIN_MAP_ID: &str = "aura-basin.collision-grounds";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraBasinFacilityId {
    HuntGround,
    DenSeam,
    RescueWatch,
    RaceSpur,
    WeaponsRange,
    TransformationTrial,
    ConflictBoundary,
    HollowRecoveryYard,
    LawfulHollowingStation,
    SalvageDepot,
    HideWorks,
    TrailShelter,
    TriageEvacuationPoint,
    FrameRecoveryGarage,
    CompetitionRing,
    FairyMenWaycamp,
}

impl AuraBasinFacilityId {
    pub const ALL: [Self; 16] = [
        Self::HuntGround,
        Self::DenSeam,
        Self::RescueWatch,
        Self::RaceSpur,
        Self::WeaponsRange,
        Self::TransformationTrial,
        Self::ConflictBoundary,
        Self::HollowRecoveryYard,
        Self::LawfulHollowingStation,
        Self::SalvageDepot,
        Self::HideWorks,
        Self::TrailShelter,
        Self::TriageEvacuationPoint,
        Self::FrameRecoveryGarage,
        Self::CompetitionRing,
        Self::FairyMenWaycamp,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::HuntGround => "aura-basin.facility.hunt-ground",
            Self::DenSeam => "aura-basin.facility.den-seam",
            Self::RescueWatch => "aura-basin.facility.rescue-watch",
            Self::RaceSpur => "aura-basin.facility.race-spur",
            Self::WeaponsRange => "aura-basin.facility.weapons-range",
            Self::TransformationTrial => "aura-basin.facility.transformation-trial",
            Self::ConflictBoundary => "aura-basin.facility.conflict-boundary",
            Self::HollowRecoveryYard => "aura-basin.facility.hollow-recovery-yard",
            Self::LawfulHollowingStation => "aura-basin.facility.lawful-hollowing-station",
            Self::SalvageDepot => "aura-basin.facility.salvage-depot",
            Self::HideWorks => "aura-basin.facility.hide-works",
            Self::TrailShelter => "aura-basin.facility.trail-shelter",
            Self::TriageEvacuationPoint => "aura-basin.facility.triage-evacuation-point",
            Self::FrameRecoveryGarage => "aura-basin.facility.frame-recovery-garage",
            Self::CompetitionRing => "aura-basin.facility.competition-ring",
            Self::FairyMenWaycamp => "aura-basin.facility.fairy-men-waycamp",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraBasinFacilityKind {
    Hunt,
    DenAndWildlife,
    Rescue,
    Racing,
    WeaponTrial,
    TransformationTrial,
    TerritorialBoundary,
    HollowRecovery,
    LawfulHollowing,
    Salvage,
    MaterialCraft,
    Shelter,
    TriageAndEvacuation,
    FrameRecovery,
    Competition,
    FolkWaycamp,
}

impl AuraBasinFacilityKind {
    pub const REQUIRED: [Self; 16] = [
        Self::Hunt,
        Self::DenAndWildlife,
        Self::Rescue,
        Self::Racing,
        Self::WeaponTrial,
        Self::TransformationTrial,
        Self::TerritorialBoundary,
        Self::HollowRecovery,
        Self::LawfulHollowing,
        Self::Salvage,
        Self::MaterialCraft,
        Self::Shelter,
        Self::TriageAndEvacuation,
        Self::FrameRecovery,
        Self::Competition,
        Self::FolkWaycamp,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hunt => "hunt ground",
            Self::DenAndWildlife => "den and wildlife seam",
            Self::Rescue => "rescue watch",
            Self::Racing => "race spur",
            Self::WeaponTrial => "weapons range",
            Self::TransformationTrial => "transformation trial",
            Self::TerritorialBoundary => "territorial boundary",
            Self::HollowRecovery => "Hollow recovery yard",
            Self::LawfulHollowing => "lawful Hollowing station",
            Self::Salvage => "salvage depot",
            Self::MaterialCraft => "hide and material works",
            Self::Shelter => "trail shelter",
            Self::TriageAndEvacuation => "triage and evacuation point",
            Self::FrameRecovery => "Frame recovery garage",
            Self::Competition => "competition ring",
            Self::FolkWaycamp => "We Fairy Men waycamp",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuraBasinFacility {
    pub id: AuraBasinFacilityId,
    pub name: &'static str,
    pub kind: AuraBasinFacilityKind,
    pub function: &'static str,
}

impl AuraBasinFacility {
    const fn new(
        id: AuraBasinFacilityId,
        name: &'static str,
        kind: AuraBasinFacilityKind,
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
pub struct AuraBasinHouseRole {
    pub house: House,
    pub responsibility: &'static str,
    pub limit: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraBasinSurface {
    pub id: InteriorSurfaceId,
    pub map_id: &'static str,
    pub dominant_house: House,
    pub regional_attribution: &'static str,
    pub boundary: [SurfacePoint; 3],
    pub access_routes: [ConstitutionalRouteId; 6],
    pub facilities: Vec<AuraBasinFacility>,
    pub house_roles: [AuraBasinHouseRole; 4],
}

impl AuraBasinSurface {
    #[must_use]
    pub fn facility(&self, id: AuraBasinFacilityId) -> Option<&AuraBasinFacility> {
        self.facilities.iter().find(|facility| facility.id == id)
    }

    #[must_use]
    pub fn permits_route_entry(&self, route: ConstitutionalRouteId) -> bool {
        self.id.permits_route_entry(route)
    }

    pub fn validate(&self) -> Result<(), AuraBasinError> {
        if self.id != InteriorSurfaceId::AuraBasin
            || self.map_id != AURA_BASIN_MAP_ID
            || self.dominant_house != House::Flynt
            || self.regional_attribution
                != "Flynt collision, hunt, rescue, competition, recovery, and return ground"
        {
            return Err(AuraBasinError::IdentityOrAttributionMismatch);
        }
        if self.boundary
            != [
                SurfacePoint {
                    x_per_mille: 500,
                    y_per_mille: 140,
                },
                SurfacePoint {
                    x_per_mille: 265,
                    y_per_mille: 500,
                },
                SurfacePoint {
                    x_per_mille: 500,
                    y_per_mille: 860,
                },
            ]
        {
            return Err(AuraBasinError::BoundaryMismatch);
        }
        if self.access_routes.as_slice() != self.id.access_routes() {
            return Err(AuraBasinError::AccessRouteMismatch);
        }
        validate_facilities(&self.facilities)?;
        validate_house_roles(&self.house_roles)?;
        Ok(())
    }
}

pub fn canonical_aura_basin() -> Result<AuraBasinSurface, AuraBasinError> {
    use AuraBasinFacilityId as Id;
    use AuraBasinFacilityKind as Kind;

    let surface = AuraBasinSurface {
        id: InteriorSurfaceId::AuraBasin,
        map_id: AURA_BASIN_MAP_ID,
        dominant_house: House::Flynt,
        regional_attribution: "Flynt collision, hunt, rescue, competition, recovery, and return ground",
        boundary: [
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 140,
            },
            SurfacePoint {
                x_per_mille: 265,
                y_per_mille: 500,
            },
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 860,
            },
        ],
        access_routes: [
            ConstitutionalRouteId::AuraRidge,
            ConstitutionalRouteId::CurrentSea,
            ConstitutionalRouteId::Boardwalk,
            ConstitutionalRouteId::Riptide,
            ConstitutionalRouteId::BasinMotorspeedway,
            ConstitutionalRouteId::StairwayToHeaven,
        ],
        facilities: vec![
            AuraBasinFacility::new(
                Id::HuntGround,
                "Common Hunt Ground",
                Kind::Hunt,
                "tracked quarry, declared purpose, restraint, recovery, and witnessed result",
            ),
            AuraBasinFacility::new(
                Id::DenSeam,
                "Den Seam",
                Kind::DenAndWildlife,
                "territory, nesting, migration, rare encounters, and protected retreat",
            ),
            AuraBasinFacility::new(
                Id::RescueWatch,
                "Basin Rescue Watch",
                Kind::Rescue,
                "distress watch, retrieval, stabilization, custody, and care transfer",
            ),
            AuraBasinFacility::new(
                Id::RaceSpur,
                "Basin Race Spur",
                Kind::Racing,
                "timed production trials linked to Basin Motor Speedway without replacing it",
            ),
            AuraBasinFacility::new(
                Id::WeaponsRange,
                "Basin Weapons Range",
                Kind::WeaponTrial,
                "bounded weapon testing, backstops, observers, failure records, and cease-fire",
            ),
            AuraBasinFacility::new(
                Id::TransformationTrial,
                "Transformation Trial Ground",
                Kind::TransformationTrial,
                "consented stress observation without granting Synthesis or clinical authority",
            ),
            AuraBasinFacility::new(
                Id::ConflictBoundary,
                "Territorial Conflict Boundary",
                Kind::TerritorialBoundary,
                "named territory, warning, challenge, disengagement, and protected passage",
            ),
            AuraBasinFacility::new(
                Id::HollowRecoveryYard,
                "Hollow Recovery Yard",
                Kind::HollowRecovery,
                "quarantine, provenance, custody, measurement, and transfer of recovered Hollow",
            ),
            AuraBasinFacility::new(
                Id::LawfulHollowingStation,
                "Stonebend Hollowing Station",
                Kind::LawfulHollowing,
                "authorized extraction that preserves exterior identity and records removed material",
            ),
            AuraBasinFacility::new(
                Id::SalvageDepot,
                "Basin Salvage Depot",
                Kind::Salvage,
                "recovered equipment, machine remains, claims, hazards, repair, and return",
            ),
            AuraBasinFacility::new(
                Id::HideWorks,
                "Hide and Material Works",
                Kind::MaterialCraft,
                "cleaning, curing, grading, provenance, waste, and conversion into goods",
            ),
            AuraBasinFacility::new(
                Id::TrailShelter,
                "Pressure Shelter",
                Kind::Shelter,
                "weather refuge, route register, supplies, communication, and protected rest",
            ),
            AuraBasinFacility::new(
                Id::TriageEvacuationPoint,
                "Basin Triage and Evacuation Point",
                Kind::TriageAndEvacuation,
                "narrow first response and transfer into qualified Glaüshouse care",
            ),
            AuraBasinFacility::new(
                Id::FrameRecoveryGarage,
                "Frame Recovery Garage",
                Kind::FrameRecovery,
                "stabilization, transport fit, damaged-part custody, and repair referral",
            ),
            AuraBasinFacility::new(
                Id::CompetitionRing,
                "Common Competition Ring",
                Kind::Competition,
                "declared rules, equal entry, witnesses, stoppage, outcome, and recognition request",
            ),
            AuraBasinFacility::new(
                Id::FairyMenWaycamp,
                "We Fairy Men Waycamp",
                Kind::FolkWaycamp,
                "traveling protection, music, rumor, guidance, and voluntary aid without sovereignty",
            ),
        ],
        house_roles: [
            AuraBasinHouseRole {
                house: House::Flynt,
                responsibility: "recognizes performed function, rescue, competition, resolved claims, and lawful return",
                limit: "Flynt recognition and rural protection do not make a crew sovereign over the Basin",
            },
            AuraBasinHouseRole {
                house: House::Glaushouse,
                responsibility: "clears triage, injury, exposure, contamination, recovery, and transport to care",
                limit: "care, custody, and survival do not prove combat or grant territorial standing",
            },
            AuraBasinHouseRole {
                house: House::Stonebend,
                responsibility: "names territory, quarry, recovered bodies, materials, claims, and lawful Hollowing",
                limit: "Hollowing requires authority and provenance and never becomes ownership of a Being",
            },
            AuraBasinHouseRole {
                house: House::Sandmanor,
                responsibility: "proves weapons, methods, routes, transformations, repair, and repeated performance",
                limit: "a successful trial does not create Clearance, recognition, office, or Title",
            },
        ],
    };
    surface.validate()?;
    Ok(surface)
}

fn validate_facilities(facilities: &[AuraBasinFacility]) -> Result<(), AuraBasinError> {
    let mut ids = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for facility in facilities {
        if !ids.insert(facility.id.stable_id()) {
            return Err(AuraBasinError::DuplicateFacility(facility.id));
        }
        kinds.insert(facility.kind);
    }
    if ids.len() != AuraBasinFacilityId::ALL.len()
        || AuraBasinFacilityKind::REQUIRED
            .into_iter()
            .any(|kind| !kinds.contains(&kind))
    {
        return Err(AuraBasinError::IncompleteFacilitySystem);
    }
    Ok(())
}

fn validate_house_roles(roles: &[AuraBasinHouseRole; 4]) -> Result<(), AuraBasinError> {
    if roles.iter().enumerate().any(|(index, role)| {
        roles[index + 1..]
            .iter()
            .any(|other| other.house == role.house)
    }) {
        return Err(AuraBasinError::HouseRoleMismatch);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuraBasinError {
    IdentityOrAttributionMismatch,
    BoundaryMismatch,
    AccessRouteMismatch,
    DuplicateFacility(AuraBasinFacilityId),
    IncompleteFacilitySystem,
    HouseRoleMismatch,
}

impl fmt::Display for AuraBasinError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Aura Basin contract rejected: {self:?}")
    }
}

impl std::error::Error for AuraBasinError {}
