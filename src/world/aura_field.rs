//! The one canonical Aura Field and the working places contained within it.
//!
//! `Aura Field` is a singular geographic surface. Farms, plots, paddocks, and
//! facilities are constituents of that surface; none is a second Aura Field.
//! The plural spelling survives only in older persistence identifiers owned by
//! other modules.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

use super::geography::ConstitutionalRouteId;
use super::interior_surface::{InteriorSurfaceId, SurfacePoint};

pub const AURA_FIELD_MAP_ID: &str = "aura-field.working-land";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraFieldFacilityId {
    UpperAuraFarm,
    EastAuraFarm,
    EngagementFarm,
    ProvingPlots,
    Orchard,
    Paddock,
    IrrigationWorks,
    Barn,
    Granary,
    SeedHouse,
    Farmstead,
    ToolShed,
    ProduceMarket,
    Windbreak,
    Apiary,
    CompostYard,
}

impl AuraFieldFacilityId {
    pub const ALL: [Self; 16] = [
        Self::UpperAuraFarm,
        Self::EastAuraFarm,
        Self::EngagementFarm,
        Self::ProvingPlots,
        Self::Orchard,
        Self::Paddock,
        Self::IrrigationWorks,
        Self::Barn,
        Self::Granary,
        Self::SeedHouse,
        Self::Farmstead,
        Self::ToolShed,
        Self::ProduceMarket,
        Self::Windbreak,
        Self::Apiary,
        Self::CompostYard,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::UpperAuraFarm => "aura-field.facility.upper-aura-farm",
            Self::EastAuraFarm => "aura-field.facility.east-aura-farm",
            Self::EngagementFarm => "aura-field.facility.engagement-farm",
            Self::ProvingPlots => "aura-field.facility.proving-plots",
            Self::Orchard => "aura-field.facility.orchard",
            Self::Paddock => "aura-field.facility.paddock",
            Self::IrrigationWorks => "aura-field.facility.irrigation-works",
            Self::Barn => "aura-field.facility.barn",
            Self::Granary => "aura-field.facility.granary",
            Self::SeedHouse => "aura-field.facility.seed-house",
            Self::Farmstead => "aura-field.facility.farmstead",
            Self::ToolShed => "aura-field.facility.tool-shed",
            Self::ProduceMarket => "aura-field.facility.produce-market",
            Self::Windbreak => "aura-field.facility.windbreak",
            Self::Apiary => "aura-field.facility.apiary",
            Self::CompostYard => "aura-field.facility.compost-yard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AuraFieldFacilityKind {
    AuraFarm,
    EngagementFarm,
    ProvingPlot,
    Orchard,
    Paddock,
    Irrigation,
    Barn,
    Granary,
    SeedHouse,
    Farmstead,
    ToolWorks,
    Market,
    Windbreak,
    Apiary,
    Compost,
}

impl AuraFieldFacilityKind {
    pub const REQUIRED: [Self; 15] = [
        Self::AuraFarm,
        Self::EngagementFarm,
        Self::ProvingPlot,
        Self::Orchard,
        Self::Paddock,
        Self::Irrigation,
        Self::Barn,
        Self::Granary,
        Self::SeedHouse,
        Self::Farmstead,
        Self::ToolWorks,
        Self::Market,
        Self::Windbreak,
        Self::Apiary,
        Self::Compost,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraFarm => "Aura farm",
            Self::EngagementFarm => "engagement farm",
            Self::ProvingPlot => "proving plot",
            Self::Orchard => "orchard",
            Self::Paddock => "paddock",
            Self::Irrigation => "irrigation",
            Self::Barn => "barn",
            Self::Granary => "granary",
            Self::SeedHouse => "seed house",
            Self::Farmstead => "farmstead",
            Self::ToolWorks => "tool works",
            Self::Market => "produce market",
            Self::Windbreak => "windbreak",
            Self::Apiary => "apiary",
            Self::Compost => "compost yard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuraFieldFacility {
    pub id: AuraFieldFacilityId,
    pub name: &'static str,
    pub kind: AuraFieldFacilityKind,
    pub function: &'static str,
}

impl AuraFieldFacility {
    #[must_use]
    pub const fn new(
        id: AuraFieldFacilityId,
        name: &'static str,
        kind: AuraFieldFacilityKind,
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
pub struct AuraFieldHouseRole {
    pub house: House,
    pub responsibility: &'static str,
    pub limit: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraFieldSurface {
    pub id: InteriorSurfaceId,
    pub map_id: &'static str,
    pub dominant_house: House,
    pub regional_attribution: &'static str,
    pub boundary: [SurfacePoint; 3],
    pub access_routes: [ConstitutionalRouteId; 3],
    pub facilities: Vec<AuraFieldFacility>,
    pub house_roles: [AuraFieldHouseRole; 4],
}

impl AuraFieldSurface {
    #[must_use]
    pub fn facility(&self, id: AuraFieldFacilityId) -> Option<&AuraFieldFacility> {
        self.facilities.iter().find(|facility| facility.id == id)
    }

    #[must_use]
    pub fn permits_route_entry(&self, route: ConstitutionalRouteId) -> bool {
        self.id.permits_route_entry(route)
    }

    pub fn validate(&self) -> Result<(), AuraFieldError> {
        if self.id != InteriorSurfaceId::AuraField
            || self.id.display_name() != "Aura Field"
            || self.map_id != AURA_FIELD_MAP_ID
            || self.dominant_house != House::Sandmanor
            || self.regional_attribution
                != "Sandmanor Minorian interior cultivation and repeated-work proving ground"
        {
            return Err(AuraFieldError::IdentityMismatch);
        }
        let expected_boundary = [
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 140,
            },
            SurfacePoint {
                x_per_mille: 735,
                y_per_mille: 500,
            },
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 500,
            },
        ];
        if self.boundary != expected_boundary {
            return Err(AuraFieldError::BoundaryMismatch);
        }
        if self.access_routes.as_slice() != self.id.access_routes() {
            return Err(AuraFieldError::AccessRouteMismatch);
        }

        let mut ids = BTreeSet::new();
        let mut kinds = BTreeSet::new();
        let mut farm_count = 0;
        for facility in &self.facilities {
            if !ids.insert(facility.id.stable_id()) {
                return Err(AuraFieldError::DuplicateFacility(facility.id));
            }
            kinds.insert(facility.kind);
            if facility.kind == AuraFieldFacilityKind::AuraFarm {
                farm_count += 1;
            }
        }
        if ids.len() != AuraFieldFacilityId::ALL.len() {
            return Err(AuraFieldError::MissingFacility);
        }
        if farm_count < 2 {
            return Err(AuraFieldError::MultipleAuraFarmsRequired);
        }
        if AuraFieldFacilityKind::REQUIRED
            .into_iter()
            .any(|kind| !kinds.contains(&kind))
        {
            return Err(AuraFieldError::MissingFacilityKind);
        }
        if self.house_roles.iter().enumerate().any(|(index, role)| {
            self.house_roles[index + 1..]
                .iter()
                .any(|other| other.house == role.house)
        }) {
            return Err(AuraFieldError::HouseRoleMismatch);
        }
        Ok(())
    }
}

pub fn canonical_aura_field() -> Result<AuraFieldSurface, AuraFieldError> {
    use AuraFieldFacilityId as Id;
    use AuraFieldFacilityKind as Kind;

    let surface = AuraFieldSurface {
        id: InteriorSurfaceId::AuraField,
        map_id: AURA_FIELD_MAP_ID,
        dominant_house: House::Sandmanor,
        regional_attribution: "Sandmanor Minorian interior cultivation and repeated-work proving ground",
        boundary: [
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 140,
            },
            SurfacePoint {
                x_per_mille: 735,
                y_per_mille: 500,
            },
            SurfacePoint {
                x_per_mille: 500,
                y_per_mille: 500,
            },
        ],
        access_routes: [
            ConstitutionalRouteId::AuraRidge,
            ConstitutionalRouteId::AuraWay,
            ConstitutionalRouteId::MntAura,
        ],
        facilities: vec![
            AuraFieldFacility::new(
                Id::UpperAuraFarm,
                "Upper Aura Farm",
                Kind::AuraFarm,
                "grain, pulse, and rotation crops",
            ),
            AuraFieldFacility::new(
                Id::EastAuraFarm,
                "East Aura Farm",
                Kind::AuraFarm,
                "vegetables, fodder, and seasonal trials",
            ),
            AuraFieldFacility::new(
                Id::EngagementFarm,
                "Engagement Farm",
                Kind::EngagementFarm,
                "voluntary finite joint work, apprenticeship, disclosed roles, and leave-without-debt practice",
            ),
            AuraFieldFacility::new(
                Id::ProvingPlots,
                "Sandmanor Proving Plots",
                Kind::ProvingPlot,
                "repeatable cultivation proof without title or recognition",
            ),
            AuraFieldFacility::new(
                Id::Orchard,
                "Ridge Orchard",
                Kind::Orchard,
                "fruit, nut, graft, and pollination cycles",
            ),
            AuraFieldFacility::new(
                Id::Paddock,
                "Common Paddock",
                Kind::Paddock,
                "grazing, draft-animal rest, and manure recovery",
            ),
            AuraFieldFacility::new(
                Id::IrrigationWorks,
                "Aura Irrigation Works",
                Kind::Irrigation,
                "wells, channels, gates, drainage, and drought rationing",
            ),
            AuraFieldFacility::new(
                Id::Barn,
                "Common Barn",
                Kind::Barn,
                "livestock shelter, hay storage, and shared field labor",
            ),
            AuraFieldFacility::new(
                Id::Granary,
                "Field Granary",
                Kind::Granary,
                "drying, grading, storage, reserves, and spoilage records",
            ),
            AuraFieldFacility::new(
                Id::SeedHouse,
                "Seed House",
                Kind::SeedHouse,
                "seed selection, provenance, exchange, and seasonal memory",
            ),
            AuraFieldFacility::new(
                Id::Farmstead,
                "Field Farmstead",
                Kind::Farmstead,
                "dwelling, kitchen, wash, first aid, and work coordination",
            ),
            AuraFieldFacility::new(
                Id::ToolShed,
                "Tool Shed and Smithy",
                Kind::ToolWorks,
                "tool issue, sharpening, repair, harness, and spare parts",
            ),
            AuraFieldFacility::new(
                Id::ProduceMarket,
                "Field Produce Market",
                Kind::Market,
                "weighing, exchange, public prices, and Flynt recognition records",
            ),
            AuraFieldFacility::new(
                Id::Windbreak,
                "Living Windbreak",
                Kind::Windbreak,
                "shelter from Aura pressure, soil retention, and habitat",
            ),
            AuraFieldFacility::new(
                Id::Apiary,
                "Field Apiary",
                Kind::Apiary,
                "pollination, honey, wax, and seasonal health observation",
            ),
            AuraFieldFacility::new(
                Id::CompostYard,
                "Compost Yard",
                Kind::Compost,
                "manure, crop residue, heat, decay, and soil return",
            ),
        ],
        house_roles: [
            AuraFieldHouseRole {
                house: House::Stonebend,
                responsibility: "names parcels, boundaries, continuities, and disputes",
                limit: "naming a crop or parcel does not prove its yield",
            },
            AuraFieldHouseRole {
                house: House::Sandmanor,
                responsibility: "proves methods through cultivation, repetition, maintenance, and yield",
                limit: "proof does not grant consent, Clearance, recognition, or Title",
            },
            AuraFieldHouseRole {
                house: House::Glaushouse,
                responsibility: "clears water, food, animal, worker, and soil hazards",
                limit: "Clearance does not rename a parcel or recognize a claim",
            },
            AuraFieldHouseRole {
                house: House::Flynt,
                responsibility: "recognizes public records, exchanges, achievements, and resolved obligations",
                limit: "recognition does not manufacture proof or ownership",
            },
        ],
    };
    surface.validate()?;
    Ok(surface)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuraFieldError {
    IdentityMismatch,
    BoundaryMismatch,
    AccessRouteMismatch,
    DuplicateFacility(AuraFieldFacilityId),
    MissingFacility,
    MissingFacilityKind,
    MultipleAuraFarmsRequired,
    HouseRoleMismatch,
}

impl fmt::Display for AuraFieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Aura Field contract rejected: {self:?}")
    }
}

impl std::error::Error for AuraFieldError {}
