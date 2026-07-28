//! Constitutional solid mining and offshore Current-well operations.
//!
//! Extraction sites are working places attached to an existing constitutional
//! route. They are not new routes, House property by implication, or authority
//! shortcuts. `Highway to Hell` is the descending industrial gallery inside
//! the Stairway-to-Heaven mountain complex and therefore does not alter the
//! locked ten-route network.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::geography::ConstitutionalRouteId;

pub const MNT_AURA_MINE_MAP_ID: &str = "mnt-aura.high-mine";
pub const STAIRWAY_MINE_MAP_ID: &str = "stairway-to-heaven.burden-mine";
pub const HIGHWAY_TO_HELL_MAP_ID: &str = "highway-to-hell.deepworks";
pub const RIPTIDE_RIG_MAP_ID: &str = "riptide.current-recovery-rig";
pub const CURRENT_SEA_RIG_MAP_ID: &str = "current-sea.depth-production-rig";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExtractionSiteId {
    MntAuraHighMine,
    StairwayBurdenMine,
    HighwayToHellDeepworks,
    RiptideRecoveryRig,
    CurrentSeaDepthRig,
}

impl ExtractionSiteId {
    pub const ALL: [Self; 5] = [
        Self::MntAuraHighMine,
        Self::StairwayBurdenMine,
        Self::HighwayToHellDeepworks,
        Self::RiptideRecoveryRig,
        Self::CurrentSeaDepthRig,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::MntAuraHighMine => "extraction.mnt-aura.high-mine",
            Self::StairwayBurdenMine => "extraction.stairway.burden-mine",
            Self::HighwayToHellDeepworks => "extraction.highway-to-hell.deepworks",
            Self::RiptideRecoveryRig => "extraction.riptide.current-recovery-rig",
            Self::CurrentSeaDepthRig => "extraction.current-sea.depth-rig",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::MntAuraHighMine => "Mt. Aura High Mine",
            Self::StairwayBurdenMine => "Stairway Burden Mine",
            Self::HighwayToHellDeepworks => "Highway to Hell Deepworks",
            Self::RiptideRecoveryRig => "Riptide Current Recovery Rig",
            // Historical display name for the specialized offshore branch;
            // this worksite does not define the civic Current Sea route.
            Self::CurrentSeaDepthRig => "Current Sea Depth Production Rig",
        }
    }

    #[must_use]
    pub const fn map_id(self) -> &'static str {
        match self {
            Self::MntAuraHighMine => MNT_AURA_MINE_MAP_ID,
            Self::StairwayBurdenMine => STAIRWAY_MINE_MAP_ID,
            Self::HighwayToHellDeepworks => HIGHWAY_TO_HELL_MAP_ID,
            Self::RiptideRecoveryRig => RIPTIDE_RIG_MAP_ID,
            Self::CurrentSeaDepthRig => CURRENT_SEA_RIG_MAP_ID,
        }
    }

    #[must_use]
    pub const fn route(self) -> ConstitutionalRouteId {
        match self {
            Self::MntAuraHighMine => ConstitutionalRouteId::MntAura,
            Self::StairwayBurdenMine | Self::HighwayToHellDeepworks => {
                ConstitutionalRouteId::StairwayToHeaven
            }
            Self::RiptideRecoveryRig => ConstitutionalRouteId::Riptide,
            Self::CurrentSeaDepthRig => ConstitutionalRouteId::CurrentSea,
        }
    }

    #[must_use]
    pub const fn is_nested_descent(self) -> bool {
        matches!(self, Self::HighwayToHellDeepworks)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtractionMethod {
    SolidSeamMining,
    OffshoreCurrentWell,
}

impl ExtractionMethod {
    #[must_use]
    pub const fn analogy(self) -> &'static str {
        match self {
            Self::SolidSeamMining => {
                "coal-side: solid seams are cut, supported, hauled, and graded"
            }
            Self::OffshoreCurrentWell => {
                "petroleum-side: deep fluids are drilled, pressure-controlled, recovered, and certified"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtractedResource {
    AuraStone,
    BurdenOre,
    DeepIron,
    RecoveredCurrentBrine,
    CertifiedCurrentBrine,
}

impl ExtractedResource {
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::AuraStone => "Aura-bearing stone",
            Self::BurdenOre => "burden ore",
            Self::DeepIron => "deep iron",
            Self::RecoveredCurrentBrine => "recovered Current-bearing brine",
            Self::CertifiedCurrentBrine => "certified Current-bearing brine",
        }
    }

    #[must_use]
    pub const fn form(self) -> &'static str {
        match self {
            Self::AuraStone | Self::BurdenOre | Self::DeepIron => "solid",
            Self::RecoveredCurrentBrine | Self::CertifiedCurrentBrine => "fluid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExtractionFacilityId {
    SurveyOffice,
    Headframe,
    VentilationHouse,
    HoistAndCage,
    WorkingFace,
    RefugeChamber,
    PumpStation,
    GradeAndCustodyYard,
    DrillFloor,
    Derrick,
    PressureControl,
    CurrentSeparator,
    SpillBoomDepot,
    DiveAndRescueBay,
    CertificationLaboratory,
    TransferManifold,
}

impl ExtractionFacilityId {
    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::SurveyOffice => "facility.survey-office",
            Self::Headframe => "facility.headframe",
            Self::VentilationHouse => "facility.ventilation-house",
            Self::HoistAndCage => "facility.hoist-and-cage",
            Self::WorkingFace => "facility.working-face",
            Self::RefugeChamber => "facility.refuge-chamber",
            Self::PumpStation => "facility.pump-station",
            Self::GradeAndCustodyYard => "facility.grade-custody-yard",
            Self::DrillFloor => "facility.drill-floor",
            Self::Derrick => "facility.derrick",
            Self::PressureControl => "facility.pressure-control",
            Self::CurrentSeparator => "facility.current-separator",
            Self::SpillBoomDepot => "facility.spill-boom-depot",
            Self::DiveAndRescueBay => "facility.dive-rescue-bay",
            Self::CertificationLaboratory => "facility.certification-laboratory",
            Self::TransferManifold => "facility.transfer-manifold",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::SurveyOffice => "Survey and Name Office",
            Self::Headframe => "Headframe",
            Self::VentilationHouse => "Ventilation House",
            Self::HoistAndCage => "Hoist and Cage",
            Self::WorkingFace => "Working Face",
            Self::RefugeChamber => "Refuge Chamber",
            Self::PumpStation => "Mine Pump Station",
            Self::GradeAndCustodyYard => "Grade and Custody Yard",
            Self::DrillFloor => "Drill Floor",
            Self::Derrick => "Depth Derrick",
            Self::PressureControl => "Pressure-Control House",
            Self::CurrentSeparator => "Current Separator",
            Self::SpillBoomDepot => "Spill-Boom Depot",
            Self::DiveAndRescueBay => "Dive and Rescue Bay",
            Self::CertificationLaboratory => "Depth Certification Laboratory",
            Self::TransferManifold => "Custody Transfer Manifold",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionSiteDefinition {
    pub id: ExtractionSiteId,
    pub method: ExtractionMethod,
    pub resource: ExtractedResource,
    pub constitutional_function: &'static str,
    pub facilities: &'static [ExtractionFacilityId],
    pub principal_hazards: &'static [&'static str],
    pub route_limit: &'static str,
}

impl ExtractionSiteDefinition {
    pub fn validate(&self) -> Result<(), ExtractionContractError> {
        if self.id.route() == ConstitutionalRouteId::StairwayToHeaven
            && self.id.is_nested_descent()
            && self.id.display_name() != "Highway to Hell Deepworks"
        {
            return Err(ExtractionContractError::HighwayIdentityMismatch);
        }
        let solid = self.resource.form() == "solid";
        if solid != (self.method == ExtractionMethod::SolidSeamMining) {
            return Err(ExtractionContractError::MethodResourceMismatch(self.id));
        }
        let facility_set: BTreeSet<_> = self.facilities.iter().copied().collect();
        if facility_set.len() != self.facilities.len()
            || self.facilities.len() < 6
            || self.principal_hazards.len() < 3
        {
            return Err(ExtractionContractError::IncompleteSite(self.id));
        }
        let required = match self.method {
            ExtractionMethod::SolidSeamMining => [
                ExtractionFacilityId::SurveyOffice,
                ExtractionFacilityId::VentilationHouse,
                ExtractionFacilityId::WorkingFace,
                ExtractionFacilityId::RefugeChamber,
            ],
            ExtractionMethod::OffshoreCurrentWell => [
                ExtractionFacilityId::DrillFloor,
                ExtractionFacilityId::PressureControl,
                ExtractionFacilityId::DiveAndRescueBay,
                ExtractionFacilityId::TransferManifold,
            ],
        };
        if required
            .into_iter()
            .any(|facility| !facility_set.contains(&facility))
        {
            return Err(ExtractionContractError::IncompleteSite(self.id));
        }
        Ok(())
    }
}

pub fn canonical_extraction_sites() -> Result<Vec<ExtractionSiteDefinition>, ExtractionContractError>
{
    use ExtractedResource as Resource;
    use ExtractionFacilityId as Facility;
    use ExtractionMethod::{OffshoreCurrentWell, SolidSeamMining};
    use ExtractionSiteId as Site;

    const LAND_MINE: &[Facility] = &[
        Facility::SurveyOffice,
        Facility::Headframe,
        Facility::VentilationHouse,
        Facility::HoistAndCage,
        Facility::WorkingFace,
        Facility::RefugeChamber,
        Facility::PumpStation,
        Facility::GradeAndCustodyYard,
    ];
    const OFFSHORE_RIG: &[Facility] = &[
        Facility::DrillFloor,
        Facility::Derrick,
        Facility::PressureControl,
        Facility::CurrentSeparator,
        Facility::SpillBoomDepot,
        Facility::DiveAndRescueBay,
        Facility::CertificationLaboratory,
        Facility::TransferManifold,
    ];

    let sites = vec![
        ExtractionSiteDefinition {
            id: Site::MntAuraHighMine,
            method: SolidSeamMining,
            resource: Resource::AuraStone,
            constitutional_function: "aspiration made material through surveyed high-rock work, repeatable grades, and safe return",
            facilities: LAND_MINE,
            principal_hazards: &["rockfall", "Aura-pressure weather", "haul-road exposure"],
            route_limit: "Mt. Aura access supports work and aspiration; it grants no mineral title",
        },
        ExtractionSiteDefinition {
            id: Site::StairwayBurdenMine,
            method: SolidSeamMining,
            resource: Resource::BurdenOre,
            constitutional_function: "recognized capability accepts measured extraction burden without converting ascent into ownership",
            facilities: LAND_MINE,
            principal_hazards: &["ledge fall", "hoist overload", "altitude exposure"],
            route_limit: "Stairway ascent remains a constitutional passage and cannot be closed permanently for extraction",
        },
        ExtractionSiteDefinition {
            id: Site::HighwayToHellDeepworks,
            method: SolidSeamMining,
            resource: Resource::DeepIron,
            constitutional_function: "the Stairway complex's descending industrial gallery tests whether ascent can bear responsibility for what is cut below",
            facilities: LAND_MINE,
            principal_hazards: &["firedamp", "heat", "flooding", "roof convergence"],
            route_limit: "Highway to Hell is a mine road nested inside Stairway to Heaven, never an eleventh constitutional route",
        },
        ExtractionSiteDefinition {
            id: Site::RiptideRecoveryRig,
            method: OffshoreCurrentWell,
            resource: Resource::RecoveredCurrentBrine,
            constitutional_function: "retrieve leaking well fluid, endangered crews, and failed equipment under emergency containment",
            facilities: OFFSHORE_RIG,
            principal_hazards: &[
                "blowout",
                "Current spill",
                "violent sea state",
                "diver entrapment",
            ],
            route_limit: "Riptide retrieval authorizes narrow rescue and containment, not ordinary production certification",
        },
        ExtractionSiteDefinition {
            id: Site::CurrentSeaDepthRig,
            method: OffshoreCurrentWell,
            resource: Resource::CertifiedCurrentBrine,
            constitutional_function: "prove that a deep well survives pressure, separation, custody transfer, and sustained depth certification",
            facilities: OFFSHORE_RIG,
            principal_hazards: &[
                "well-control loss",
                "depth-pressure fatigue",
                "separator contamination",
            ],
            route_limit: "Current Sea certification does not name a claim, recognize a producer, or waive rescue duties",
        },
    ];
    let mut ids = BTreeSet::new();
    let mut maps = BTreeSet::new();
    for site in &sites {
        if !ids.insert(site.id) || !maps.insert(site.id.map_id()) {
            return Err(ExtractionContractError::DuplicateSite(site.id));
        }
        site.validate()?;
    }
    if sites.len() != ExtractionSiteId::ALL.len() {
        return Err(ExtractionContractError::MissingSite);
    }
    Ok(sites)
}

#[must_use]
pub fn extraction_site(
    id: ExtractionSiteId,
) -> Result<ExtractionSiteDefinition, ExtractionContractError> {
    canonical_extraction_sites()?
        .into_iter()
        .find(|site| site.id == id)
        .ok_or(ExtractionContractError::MissingSite)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractionContractError {
    DuplicateSite(ExtractionSiteId),
    MissingSite,
    IncompleteSite(ExtractionSiteId),
    MethodResourceMismatch(ExtractionSiteId),
    HighwayIdentityMismatch,
}

impl fmt::Display for ExtractionContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Hollow Grove extraction contract rejected: {self:?}"
        )
    }
}

impl std::error::Error for ExtractionContractError {}
