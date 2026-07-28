use serde::{Deserialize, Serialize};

use crate::world::aura_basin::{AURA_BASIN_MAP_ID, AuraBasinFacilityId};
use crate::world::aura_beach::{AURA_BEACH_MAP_ID, AuraBeachFacilityId};
use crate::world::aura_field::{AURA_FIELD_MAP_ID, AuraFieldFacilityId};
use crate::world::extraction::{
    CURRENT_SEA_RIG_MAP_ID, ExtractionFacilityId, ExtractionSiteId, HIGHWAY_TO_HELL_MAP_ID,
    MNT_AURA_MINE_MAP_ID, RIPTIDE_RIG_MAP_ID, STAIRWAY_MINE_MAP_ID,
};
use crate::world::geography::ConstitutionalRouteId;
use crate::world::interior_surface::InteriorSurfaceId;

use super::CardinalDirection;

pub const STARTER_MAP_ID: &str = "aura-ridge.grove-approach";
pub const BOARDWALK_MAP_ID: &str = "boardwalk.return-vestibule";
/// Stable archive identifier. The current presentation is the civic
/// Many-Witness Concourse; the historical wire name must remain replayable.
pub const CURRENT_SEA_MAP_ID: &str = "current-sea.deep-certification-landing";
pub const MAP_WIDTH: u16 = 20;
pub const MAP_HEIGHT: u16 = 18;
pub const MAP_TILE_SIZE: u16 = 8;
pub const STARTER_MAP_WIDTH: u16 = MAP_WIDTH;
pub const STARTER_MAP_HEIGHT: u16 = MAP_HEIGHT;
pub const STARTER_MAP_TILE_SIZE: u16 = MAP_TILE_SIZE;

pub const STARTER_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T.....TT....TT.....T",
    "T.HHH.TT.==.TT.FF..T",
    "T.HHH....==.....F..T",
    "T........==........T",
    "T..CCCC..==..WWWW..T",
    "T..CCCC..==..WWWW..T",
    "T..CCDC..==..WWWW..T",
    "T........==........T",
    "T=====S========....T",
    "T........==...HHH..T",
    "T.FF.....==...HHH..T",
    "T........==...HHH..T",
    "T....N...==........T",
    "T........==........T",
    "T........==........T",
    "T........==........T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const BOARDWALK_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T..................T",
    "T..===........===..T",
    "T..===........===..T",
    "T...P..O...N..G....T",
    "T..====....======..T",
    "T..................T",
    "T========I=========T",
    "T..................T",
    "T..=====....=====..T",
    "T..................T",
    "T........R.........T",
    "T..................T",
    "T..A...............T",
    "T..................T",
    "T........=.........T",
    "T..................T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const CURRENT_SEA_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "Tcc==cc==cc==cc==ccT",
    "T..c....==....c....T",
    "T......c==c........T",
    "T..cG...M....V.c...T",
    "T.cc....==....cc...T",
    "T....c..==..c......T",
    "T..c...xxx.........T",
    "T==================T",
    "T..cL....=....Y.c..T",
    "T.c......=......c..T",
    "T...c....I....c....T",
    "T........=.........T",
    "T..cR....=....c....T",
    "T.c......=......c..T",
    "T........=.........T",
    "T==================T",
    "TTTTTTTTTTTTTTTTTTTT",
];

/// One farm-country map inside the one canonical Aura Field surface.
///
/// Lowercase runs are cultivated ground or buildings. Uppercase markers are
/// interactable working places from the Aura Field facility catalog.
pub const AURA_FIELD_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T.oooo.....pppp....T",
    "T.oooo.....pppp....T",
    "T.oOoo.....pPpA....T",
    "T======I===========T",
    "T.cccc....cVcc.....T",
    "T.cccc....cccc..W..T",
    "T.cFcc....cEcc.....T",
    "T==================T",
    "T....BBBB....GGGG..T",
    "T....B..B....G..G..T",
    "T....BN.B....GKGG..T",
    "T======C===========T",
    "T...hHhh...mMmm.Q..T",
    "T...hhhh...mmmm....T",
    "T...hhhh...mmmm....T",
    "T...L....=...S.....T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const AURA_BEACH_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "~~~~~~~~~~~~~~~~~~~~",
    "~~~==============~~~",
    "~~~=............=~~~",
    "~~~=B...W...T...=~~~",
    "~~~======V=======~~~",
    "~~~=....A.......=~~~",
    "~~~=............=~~~",
    "~~~=N.........R.=~~~",
    "~~~==============~~~",
    "~~~=.....P......=~~~",
    "~~~=............=~~~",
    "~~~=E...C...D...=~~~",
    "~~~==============~~~",
    "~~~=.....L......=~~~",
    "~~~=S...M...H...=~~~",
    "~~~=............=~~~",
    "~~~==============~~~",
    "~~~~~~~~~~~~~~~~~~~~",
];

pub const AURA_BASIN_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T....rrrr....dddd..T",
    "T.H..rrrr....Dddd..T",
    "T....rrrr....dddd..T",
    "T==================T",
    "T.W....X...........T",
    "T..................T",
    "T.R......C.........T",
    "T==================T",
    "T..tttt....ssss....T",
    "T.T.ttt....Ssss....T",
    "T..tttt....ssss....T",
    "T==================T",
    "T.F..G..E..O.......T",
    "T..................T",
    "T.L..V..Y..Q.......T",
    "T........=.........T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const LAND_MINE_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T........=.........T",
    "T..S.....=....H....T",
    "T........=.........T",
    "T==================T",
    "T..V..........C....T",
    "T........=.........T",
    "T........=.........T",
    "T========P=========T",
    "T........=.........T",
    "T..W..........R....T",
    "T........=.........T",
    "T==================T",
    "T..G..........Y....T",
    "T........=.........T",
    "T........=.........T",
    "T........=.........T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const OFFSHORE_RIG_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "~~~~~~~~~~~~~~~~~~~~",
    "~~~==============~~~",
    "~~~=....D.......=~~~",
    "~~~=............=~~~",
    "~~~======P=======~~~",
    "~~~=............=~~~",
    "~~~=.F........C.=~~~",
    "~~~=............=~~~",
    "~~~==============~~~",
    "~~~=............=~~~",
    "~~~=.B........R.=~~~",
    "~~~=............=~~~",
    "~~~======L=======~~~",
    "~~~=............=~~~",
    "~~~=.S........T.=~~~",
    "~~~=............=~~~",
    "~~~==============~~~",
    "~~~~~~~~~~~~~~~~~~~~",
];

pub const STRAIGHT_ROUTE_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T........=.........T",
    "T..===...=...===...T",
    "T........=.........T",
    "T........K.........T",
    "T........=.........T",
    "T........=.........T",
    "T==================T",
    "T........=.........T",
    "T..===...=...===...T",
    "T........=.........T",
    "T........=.........T",
    "T........=.........T",
    "T..===...=...===...T",
    "T........=.........T",
    "T........=.........T",
    "T........=.........T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const ROUND_ROUTE_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T..................T",
    "T....=========.....T",
    "T...==.......==....T",
    "T...=.....K...=....T",
    "T...=.....=...=....T",
    "T...==....=..==....T",
    "T....==...=.==.....T",
    "T.....==..===......T",
    "T......==.=........T",
    "T.......===........T",
    "T........=.........T",
    "T........=.........T",
    "T........=.........T",
    "T........=.........T",
    "T........=.........T",
    "T..................T",
    "TTTTTTTTTTTTTTTTTTTT",
];

pub const RIPTIDE_ROUTE_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "~~~~~~~~~~~~~~~~~~~~",
    "~~~~~~========~~~~~~",
    "~~~~~==......==~~~~~",
    "~~~~==........==~~~~",
    "~~~~=....K.....=~~~~",
    "~~~~=....=.....=~~~~",
    "~~~~==...=....==~~~~",
    "~~~~~==..=...==~~~~~",
    "~~~~~~=..=..==~~~~~~",
    "~~~~~~=..=.==~~~~~~~",
    "~~~~~~=..===~~~~~~~~",
    "~~~~~~=...=~~~~~~~~~",
    "~~~~~~=...=~~~~~~~~~",
    "~~~~~~=...=~~~~~~~~~",
    "~~~~~~=...=~~~~~~~~~",
    "~~~~~~=...=~~~~~~~~~",
    "~~~~~~=====~~~~~~~~~",
    "~~~~~~~~~~~~~~~~~~~~",
];

pub const CURRENT_SEANAD_MAP_ROWS: [&str; MAP_HEIGHT as usize] = [
    "TTTTTTTTTTTTTTTTTTTT",
    "T..................T",
    "T..==============..T",
    "T..=............=..T",
    "T..=.....K......=..T",
    "T..=............=..T",
    "T..=..========..=..T",
    "T..=..=......=..=..T",
    "T..=..=......=..=..T",
    "T..=..=......=..=..T",
    "T..=..========..=..T",
    "T..=............=..T",
    "T..======..======..T",
    "T.........=........T",
    "T.........=........T",
    "T.........=........T",
    "T..................T",
    "TTTTTTTTTTTTTTTTTTTT",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldMapId {
    AuraRidgeGroveApproach,
    BoardwalkReturnVestibule,
    CurrentSeaDeepCertificationLanding,
    AuraFieldWorkingLand,
    AuraBeachCoastalCommons,
    AuraBasinCollisionGrounds,
    ExtractionSite(ExtractionSiteId),
    RoutePassage(ConstitutionalRouteId),
}

impl WorldMapId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuraRidgeGroveApproach => STARTER_MAP_ID,
            Self::BoardwalkReturnVestibule => BOARDWALK_MAP_ID,
            Self::CurrentSeaDeepCertificationLanding => CURRENT_SEA_MAP_ID,
            Self::AuraFieldWorkingLand => AURA_FIELD_MAP_ID,
            Self::AuraBeachCoastalCommons => AURA_BEACH_MAP_ID,
            Self::AuraBasinCollisionGrounds => AURA_BASIN_MAP_ID,
            Self::ExtractionSite(site) => site.map_id(),
            Self::RoutePassage(route) => route_passage_map_id(route),
        }
    }

    pub fn from_wire(value: &str) -> Result<Self, WorldMapError> {
        match value {
            STARTER_MAP_ID => Ok(Self::AuraRidgeGroveApproach),
            BOARDWALK_MAP_ID => Ok(Self::BoardwalkReturnVestibule),
            CURRENT_SEA_MAP_ID => Ok(Self::CurrentSeaDeepCertificationLanding),
            AURA_FIELD_MAP_ID | "aura-field" => Ok(Self::AuraFieldWorkingLand),
            AURA_BEACH_MAP_ID | "aura-beach" => Ok(Self::AuraBeachCoastalCommons),
            AURA_BASIN_MAP_ID | "aura-basin" => Ok(Self::AuraBasinCollisionGrounds),
            MNT_AURA_MINE_MAP_ID => Ok(Self::ExtractionSite(ExtractionSiteId::MntAuraHighMine)),
            STAIRWAY_MINE_MAP_ID => Ok(Self::ExtractionSite(ExtractionSiteId::StairwayBurdenMine)),
            HIGHWAY_TO_HELL_MAP_ID => Ok(Self::ExtractionSite(
                ExtractionSiteId::HighwayToHellDeepworks,
            )),
            RIPTIDE_RIG_MAP_ID => Ok(Self::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig)),
            CURRENT_SEA_RIG_MAP_ID => {
                Ok(Self::ExtractionSite(ExtractionSiteId::CurrentSeaDepthRig))
            }
            "riptide.emergency-intake" => Ok(Self::RoutePassage(ConstitutionalRouteId::Riptide)),
            "glausbahn.refinement-span" => Ok(Self::RoutePassage(ConstitutionalRouteId::Glausbahn)),
            "current-seanad.deliberation-chamber" => {
                Ok(Self::RoutePassage(ConstitutionalRouteId::CurrentSeanad))
            }
            "aura-way.design-corridor" => Ok(Self::RoutePassage(ConstitutionalRouteId::AuraWay)),
            "mnt-aura.aspiration-path" => Ok(Self::RoutePassage(ConstitutionalRouteId::MntAura)),
            "basin-motor-speedway.production-circuit" => Ok(Self::RoutePassage(
                ConstitutionalRouteId::BasinMotorspeedway,
            )),
            "stairway-to-heaven.ascent-path" => {
                Ok(Self::RoutePassage(ConstitutionalRouteId::StairwayToHeaven))
            }
            _ => Err(WorldMapError::UnknownMap(value.into())),
        }
    }

    #[must_use]
    pub const fn for_route(route: ConstitutionalRouteId) -> Self {
        match route {
            ConstitutionalRouteId::AuraRidge => Self::AuraRidgeGroveApproach,
            ConstitutionalRouteId::Boardwalk => Self::BoardwalkReturnVestibule,
            ConstitutionalRouteId::CurrentSea => Self::CurrentSeaDeepCertificationLanding,
            other => Self::RoutePassage(other),
        }
    }

    #[must_use]
    pub const fn route(self) -> Option<ConstitutionalRouteId> {
        match self {
            Self::AuraRidgeGroveApproach => Some(ConstitutionalRouteId::AuraRidge),
            Self::BoardwalkReturnVestibule => Some(ConstitutionalRouteId::Boardwalk),
            Self::CurrentSeaDeepCertificationLanding => Some(ConstitutionalRouteId::CurrentSea),
            Self::AuraFieldWorkingLand
            | Self::AuraBeachCoastalCommons
            | Self::AuraBasinCollisionGrounds
            | Self::ExtractionSite(_) => None,
            Self::RoutePassage(route) => Some(route),
        }
    }

    #[must_use]
    pub const fn surface(self) -> Option<InteriorSurfaceId> {
        match self {
            Self::AuraFieldWorkingLand => Some(InteriorSurfaceId::AuraField),
            Self::AuraBeachCoastalCommons => Some(InteriorSurfaceId::AuraBeach),
            Self::AuraBasinCollisionGrounds => Some(InteriorSurfaceId::AuraBasin),
            _ => None,
        }
    }

    #[must_use]
    pub const fn extraction(self) -> Option<ExtractionSiteId> {
        match self {
            Self::ExtractionSite(site) => Some(site),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_canonical(self) -> bool {
        match self.route() {
            Some(route) => map_id_eq(self, Self::for_route(route)),
            None => matches!(
                self,
                Self::AuraFieldWorkingLand
                    | Self::AuraBeachCoastalCommons
                    | Self::AuraBasinCollisionGrounds
                    | Self::ExtractionSite(_)
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionId {
    DeepPressurePerson(super::DeepPressurePersonId),
    AuraRidgeWitnessMarker,
    RidgefolkGuide,
    BoardwalkDischargeAdvocate,
    BoardwalkPimp,
    BoardwalkHoeWitness,
    BoardwalkGimp,
    BoardwalkGoonWitness,
    BoardwalkFacultyStation,
    BoardwalkReturningGoon,
    CurrentSeaGeraldRegistrar,
    CurrentSeaMercyDeep,
    /// Legacy archive name for the Current Sea crowd witness.
    CurrentSeaDepthWitness,
    CurrentSeaNameLedger,
    CurrentSeaMercuryMirror,
    CurrentSeaRestorationArchive,
    CurrentSeaFacultyStation,
    AuraFieldFacility(AuraFieldFacilityId),
    AuraBeachFacility(AuraBeachFacilityId),
    AuraBasinFacility(AuraBasinFacilityId),
    ExtractionFacility {
        site: ExtractionSiteId,
        facility: ExtractionFacilityId,
    },
    ConstitutionalRouteWitness(ConstitutionalRouteId),
}

/// Backwards-compatible name for the original Aura Ridge interaction type.
pub type StarterInteractionId = InteractionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: u16,
    pub y: u16,
    pub facing: CardinalDirection,
}

#[derive(Debug, Clone, Copy)]
pub struct WorldMapDefinition {
    pub id: WorldMapId,
    pub rows: &'static [&'static str; MAP_HEIGHT as usize],
    pub spawn: TilePosition,
}

impl WorldMapDefinition {
    #[must_use]
    pub fn move_actor(
        self,
        from: TilePosition,
        direction: CardinalDirection,
        resolved_choice: Option<super::BoardwalkChoice>,
    ) -> TilePosition {
        self.move_actor_with_cases(from, direction, resolved_choice, None)
    }

    #[must_use]
    pub fn move_actor_with_cases(
        self,
        from: TilePosition,
        direction: CardinalDirection,
        resolved_boardwalk_choice: Option<super::BoardwalkChoice>,
        resolved_stonebend_choice: Option<super::StonebendContinuityChoice>,
    ) -> TilePosition {
        let (offset_x, offset_y) = direction.offset();
        let target_x = i32::from(from.x) + offset_x;
        let target_y = i32::from(from.y) + offset_y;
        let mut resolved = TilePosition {
            x: from.x,
            y: from.y,
            facing: direction,
        };
        if self.tile_is_walkable(
            target_x,
            target_y,
            resolved_boardwalk_choice,
            resolved_stonebend_choice,
        ) {
            resolved.x = u16::try_from(target_x).expect("walkable map x is nonnegative");
            resolved.y = u16::try_from(target_y).expect("walkable map y is nonnegative");
        }
        resolved
    }

    #[must_use]
    pub fn interaction_in_front(
        self,
        from: TilePosition,
        resolved_choice: Option<super::BoardwalkChoice>,
    ) -> Option<InteractionId> {
        self.interaction_in_front_with_cases(from, resolved_choice, None)
    }

    #[must_use]
    pub fn interaction_in_front_with_cases(
        self,
        from: TilePosition,
        resolved_boardwalk_choice: Option<super::BoardwalkChoice>,
        _resolved_stonebend_choice: Option<super::StonebendContinuityChoice>,
    ) -> Option<InteractionId> {
        let (offset_x, offset_y) = from.facing.offset();
        let target = (i32::from(from.x) + offset_x, i32::from(from.y) + offset_y);
        let returning_goon_position = match resolved_boardwalk_choice {
            None => (9, 11),
            Some(super::BoardwalkChoice::PimpPatronage) => (8, 5),
            Some(super::BoardwalkChoice::GoonBond) => (12, 5),
            Some(super::BoardwalkChoice::LimitedCooperation) => (9, 9),
            Some(super::BoardwalkChoice::IndependentReturn) => (9, 7),
        };
        if self.id == WorldMapId::BoardwalkReturnVestibule && target == returning_goon_position {
            return Some(InteractionId::BoardwalkReturningGoon);
        }
        if let WorldMapId::RoutePassage(route) = self.id
            && target == (9, 4)
        {
            return Some(InteractionId::ConstitutionalRouteWitness(route));
        }
        if self.id == WorldMapId::AuraFieldWorkingLand {
            let facility = match target {
                (3, 3) => AuraFieldFacilityId::Orchard,
                (12, 3) => AuraFieldFacilityId::Paddock,
                (14, 3) => AuraFieldFacilityId::Apiary,
                (7, 4) => AuraFieldFacilityId::IrrigationWorks,
                (11, 5) => AuraFieldFacilityId::ProvingPlots,
                (3, 7) => AuraFieldFacilityId::UpperAuraFarm,
                (11, 7) => AuraFieldFacilityId::EastAuraFarm,
                (16, 13) => AuraFieldFacilityId::EngagementFarm,
                (16, 6) => AuraFieldFacilityId::Windbreak,
                (6, 11) => AuraFieldFacilityId::Barn,
                (14, 11) => AuraFieldFacilityId::Granary,
                (7, 12) => AuraFieldFacilityId::CompostYard,
                (5, 13) => AuraFieldFacilityId::Farmstead,
                (12, 13) => AuraFieldFacilityId::ProduceMarket,
                (4, 16) => AuraFieldFacilityId::ToolShed,
                (13, 16) => AuraFieldFacilityId::SeedHouse,
                _ => return None,
            };
            return Some(InteractionId::AuraFieldFacility(facility));
        }
        if self.id == WorldMapId::AuraBeachCoastalCommons {
            let facility = match target {
                (4, 3) => AuraBeachFacilityId::Beacon,
                (8, 3) => AuraBeachFacilityId::WeatherStation,
                (12, 3) => AuraBeachFacilityId::TideStation,
                (9, 4) => AuraBeachFacilityId::CoastalProvingStrand,
                (8, 5) => AuraBeachFacilityId::PublicApproach,
                (4, 7) => AuraBeachFacilityId::NavigationSchool,
                (14, 7) => AuraBeachFacilityId::RescuePost,
                (9, 9) => AuraBeachFacilityId::PublicPier,
                (4, 11) => AuraBeachFacilityId::ElfDesignYard,
                (8, 11) => AuraBeachFacilityId::CentaurRun,
                (12, 11) => AuraBeachFacilityId::DuneWard,
                (9, 13) => AuraBeachFacilityId::BoatLanding,
                (4, 14) => AuraBeachFacilityId::SalvageYard,
                (8, 14) => AuraBeachFacilityId::FishMarket,
                (12, 14) => AuraBeachFacilityId::RecoveryPavilion,
                _ => return None,
            };
            return Some(InteractionId::AuraBeachFacility(facility));
        }
        if self.id == WorldMapId::AuraBasinCollisionGrounds {
            let facility = match target {
                (2, 2) => AuraBasinFacilityId::HuntGround,
                (13, 2) => AuraBasinFacilityId::DenSeam,
                (2, 5) => AuraBasinFacilityId::WeaponsRange,
                (7, 5) => AuraBasinFacilityId::TransformationTrial,
                (2, 7) => AuraBasinFacilityId::RescueWatch,
                (9, 7) => AuraBasinFacilityId::ConflictBoundary,
                (2, 10) => AuraBasinFacilityId::TrailShelter,
                (11, 10) => AuraBasinFacilityId::HollowRecoveryYard,
                (2, 13) => AuraBasinFacilityId::FairyMenWaycamp,
                (5, 13) => AuraBasinFacilityId::FrameRecoveryGarage,
                (8, 13) => AuraBasinFacilityId::TriageEvacuationPoint,
                (11, 13) => AuraBasinFacilityId::CompetitionRing,
                (2, 15) => AuraBasinFacilityId::LawfulHollowingStation,
                (5, 15) => AuraBasinFacilityId::SalvageDepot,
                (8, 15) => AuraBasinFacilityId::HideWorks,
                (11, 15) => AuraBasinFacilityId::RaceSpur,
                _ => return None,
            };
            return Some(InteractionId::AuraBasinFacility(facility));
        }
        if let WorldMapId::ExtractionSite(site) = self.id {
            let facility = match target {
                (3, 2) => match site {
                    ExtractionSiteId::MntAuraHighMine
                    | ExtractionSiteId::StairwayBurdenMine
                    | ExtractionSiteId::HighwayToHellDeepworks => {
                        ExtractionFacilityId::SurveyOffice
                    }
                    _ => return None,
                },
                (8, 2) => match site {
                    ExtractionSiteId::RiptideRecoveryRig | ExtractionSiteId::CurrentSeaDepthRig => {
                        ExtractionFacilityId::Derrick
                    }
                    _ => return None,
                },
                (14, 2) => match site {
                    ExtractionSiteId::MntAuraHighMine
                    | ExtractionSiteId::StairwayBurdenMine
                    | ExtractionSiteId::HighwayToHellDeepworks => ExtractionFacilityId::Headframe,
                    _ => return None,
                },
                (9, 4) => match site {
                    ExtractionSiteId::RiptideRecoveryRig | ExtractionSiteId::CurrentSeaDepthRig => {
                        ExtractionFacilityId::PressureControl
                    }
                    _ => return None,
                },
                (3, 5) => ExtractionFacilityId::VentilationHouse,
                (14, 5) => ExtractionFacilityId::HoistAndCage,
                (5, 6) => ExtractionFacilityId::DrillFloor,
                (14, 6) => ExtractionFacilityId::CurrentSeparator,
                (9, 8) => match site {
                    ExtractionSiteId::MntAuraHighMine
                    | ExtractionSiteId::StairwayBurdenMine
                    | ExtractionSiteId::HighwayToHellDeepworks => ExtractionFacilityId::PumpStation,
                    _ => return None,
                },
                (3, 10) => match site {
                    ExtractionSiteId::MntAuraHighMine
                    | ExtractionSiteId::StairwayBurdenMine
                    | ExtractionSiteId::HighwayToHellDeepworks => ExtractionFacilityId::WorkingFace,
                    _ => return None,
                },
                (5, 10) => match site {
                    ExtractionSiteId::RiptideRecoveryRig | ExtractionSiteId::CurrentSeaDepthRig => {
                        ExtractionFacilityId::SpillBoomDepot
                    }
                    _ => return None,
                },
                (14, 10) => match site {
                    ExtractionSiteId::MntAuraHighMine
                    | ExtractionSiteId::StairwayBurdenMine
                    | ExtractionSiteId::HighwayToHellDeepworks => {
                        ExtractionFacilityId::RefugeChamber
                    }
                    _ => ExtractionFacilityId::DiveAndRescueBay,
                },
                (9, 12) => match site {
                    ExtractionSiteId::RiptideRecoveryRig | ExtractionSiteId::CurrentSeaDepthRig => {
                        ExtractionFacilityId::CertificationLaboratory
                    }
                    _ => return None,
                },
                (3, 13) => match site {
                    ExtractionSiteId::MntAuraHighMine
                    | ExtractionSiteId::StairwayBurdenMine
                    | ExtractionSiteId::HighwayToHellDeepworks => {
                        ExtractionFacilityId::GradeAndCustodyYard
                    }
                    _ => return None,
                },
                (14, 14) => ExtractionFacilityId::TransferManifold,
                _ => return None,
            };
            return Some(InteractionId::ExtractionFacility { site, facility });
        }
        match (self.id, target) {
            (WorldMapId::AuraRidgeGroveApproach, (6, 9)) => {
                Some(InteractionId::AuraRidgeWitnessMarker)
            }
            (WorldMapId::AuraRidgeGroveApproach, (5, 13)) => Some(InteractionId::RidgefolkGuide),
            (WorldMapId::BoardwalkReturnVestibule, (3, 13)) => {
                Some(InteractionId::BoardwalkDischargeAdvocate)
            }
            (WorldMapId::BoardwalkReturnVestibule, (4, 4)) => Some(InteractionId::BoardwalkPimp),
            (WorldMapId::BoardwalkReturnVestibule, (7, 4)) => {
                Some(InteractionId::BoardwalkHoeWitness)
            }
            (WorldMapId::BoardwalkReturnVestibule, (14, 4)) => Some(InteractionId::BoardwalkGimp),
            (WorldMapId::BoardwalkReturnVestibule, (11, 4)) => {
                Some(InteractionId::BoardwalkGoonWitness)
            }
            (WorldMapId::BoardwalkReturnVestibule, (9, 7)) => {
                Some(InteractionId::BoardwalkFacultyStation)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (4, 4)) => {
                Some(InteractionId::CurrentSeaGeraldRegistrar)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (8, 4)) => {
                Some(InteractionId::CurrentSeaMercyDeep)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (13, 4)) => {
                Some(InteractionId::CurrentSeaDepthWitness)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (4, 9)) => {
                Some(InteractionId::CurrentSeaNameLedger)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (14, 9)) => {
                Some(InteractionId::CurrentSeaMercuryMirror)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (9, 11)) => {
                Some(InteractionId::CurrentSeaFacultyStation)
            }
            (WorldMapId::CurrentSeaDeepCertificationLanding, (4, 13)) => {
                Some(InteractionId::CurrentSeaRestorationArchive)
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn projected_rows(self, resolved_choice: Option<super::BoardwalkChoice>) -> Vec<String> {
        self.projected_rows_with_cases(resolved_choice, None)
    }

    #[must_use]
    pub fn projected_rows_with_cases(
        self,
        resolved_boardwalk_choice: Option<super::BoardwalkChoice>,
        resolved_stonebend_choice: Option<super::StonebendContinuityChoice>,
    ) -> Vec<String> {
        let mut rows: Vec<String> = self.rows.iter().map(|row| (*row).into()).collect();
        if self.id == WorldMapId::CurrentSeaDeepCertificationLanding {
            if let Some(choice) = resolved_stonebend_choice {
                let gate_x = match choice {
                    super::StonebendContinuityChoice::AffirmExistingName => 7,
                    super::StonebendContinuityChoice::ProvisionalTransformedFormName => 8,
                    super::StonebendContinuityChoice::ReferIdentityConflict => 9,
                };
                set_tile(&mut rows, gate_x, 7, b'=');
                match choice {
                    super::StonebendContinuityChoice::AffirmExistingName => {
                        set_tile(&mut rows, 9, 4, b'S');
                    }
                    super::StonebendContinuityChoice::ProvisionalTransformedFormName => {
                        set_tile(&mut rows, 9, 4, b'm');
                    }
                    super::StonebendContinuityChoice::ReferIdentityConflict => {
                        set_tile(&mut rows, 9, 4, b'?');
                    }
                }
            }
            return rows;
        }
        if self.id != WorldMapId::BoardwalkReturnVestibule {
            return rows;
        }
        let Some(choice) = resolved_boardwalk_choice else {
            return rows;
        };
        // The Returning Goon changes position after deciding. This projection
        // is presentation-only; the committed choice remains the authority.
        set_tile(&mut rows, 9, 11, b'.');
        match choice {
            super::BoardwalkChoice::PimpPatronage => {
                set_tile(&mut rows, 8, 5, b'R');
            }
            super::BoardwalkChoice::GoonBond => {
                set_tile(&mut rows, 12, 5, b'R');
                set_tile(&mut rows, 13, 5, b'N');
            }
            super::BoardwalkChoice::LimitedCooperation => {
                set_tile(&mut rows, 9, 9, b'R');
            }
            super::BoardwalkChoice::IndependentReturn => {
                set_tile(&mut rows, 9, 7, b'R');
            }
        }
        rows
    }

    fn tile_is_walkable(
        self,
        x: i32,
        y: i32,
        resolved_boardwalk_choice: Option<super::BoardwalkChoice>,
        resolved_stonebend_choice: Option<super::StonebendContinuityChoice>,
    ) -> bool {
        if x < 0 || y < 0 || x >= i32::from(MAP_WIDTH) || y >= i32::from(MAP_HEIGHT) {
            return false;
        }
        if self.id == WorldMapId::BoardwalkReturnVestibule
            && let Some(choice) = resolved_boardwalk_choice
        {
            if (x, y) == (9, 11) {
                return true;
            }
            let occupied = match choice {
                super::BoardwalkChoice::PimpPatronage => [(8, 5), (-1, -1)],
                super::BoardwalkChoice::GoonBond => [(12, 5), (13, 5)],
                super::BoardwalkChoice::LimitedCooperation => [(9, 9), (-1, -1)],
                super::BoardwalkChoice::IndependentReturn => [(9, 7), (-1, -1)],
            };
            if occupied.contains(&(x, y)) {
                return false;
            }
        }
        if self.id == WorldMapId::CurrentSeaDeepCertificationLanding
            && let Some(choice) = resolved_stonebend_choice
        {
            let open_gate = match choice {
                super::StonebendContinuityChoice::AffirmExistingName => (7, 7),
                super::StonebendContinuityChoice::ProvisionalTransformedFormName => (8, 7),
                super::StonebendContinuityChoice::ReferIdentityConflict => (9, 7),
            };
            if (x, y) == open_gate {
                return true;
            }
        }
        if matches!(
            self.id,
            WorldMapId::AuraFieldWorkingLand
                | WorldMapId::AuraBeachCoastalCommons
                | WorldMapId::AuraBasinCollisionGrounds
                | WorldMapId::ExtractionSite(_)
        ) {
            return matches!(self.rows[y as usize].as_bytes()[x as usize], b'.' | b'=');
        }
        matches!(
            self.rows[y as usize].as_bytes()[x as usize],
            b'.' | b'=' | b'H' | b'D'
        )
    }
}

#[must_use]
pub const fn map_definition(id: WorldMapId) -> WorldMapDefinition {
    match id {
        WorldMapId::AuraRidgeGroveApproach => WorldMapDefinition {
            id,
            rows: &STARTER_MAP_ROWS,
            spawn: TilePosition {
                x: 9,
                y: 15,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::BoardwalkReturnVestibule => WorldMapDefinition {
            id,
            rows: &BOARDWALK_MAP_ROWS,
            spawn: TilePosition {
                x: 9,
                y: 15,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::CurrentSeaDeepCertificationLanding => WorldMapDefinition {
            id,
            rows: &CURRENT_SEA_MAP_ROWS,
            spawn: TilePosition {
                x: 9,
                y: 15,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::AuraFieldWorkingLand => WorldMapDefinition {
            id,
            rows: &AURA_FIELD_MAP_ROWS,
            spawn: TilePosition {
                x: 9,
                y: 16,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::AuraBeachCoastalCommons => WorldMapDefinition {
            id,
            rows: &AURA_BEACH_MAP_ROWS,
            spawn: TilePosition {
                x: 9,
                y: 15,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::AuraBasinCollisionGrounds => WorldMapDefinition {
            id,
            rows: &AURA_BASIN_MAP_ROWS,
            spawn: TilePosition {
                x: 9,
                y: 16,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::ExtractionSite(site) => WorldMapDefinition {
            id,
            rows: match site {
                ExtractionSiteId::MntAuraHighMine
                | ExtractionSiteId::StairwayBurdenMine
                | ExtractionSiteId::HighwayToHellDeepworks => &LAND_MINE_MAP_ROWS,
                ExtractionSiteId::RiptideRecoveryRig | ExtractionSiteId::CurrentSeaDepthRig => {
                    &OFFSHORE_RIG_MAP_ROWS
                }
            },
            spawn: TilePosition {
                x: 9,
                y: 15,
                facing: CardinalDirection::North,
            },
        },
        WorldMapId::RoutePassage(route) => WorldMapDefinition {
            id,
            rows: route_passage_rows(route),
            spawn: TilePosition {
                x: 9,
                y: 15,
                facing: CardinalDirection::North,
            },
        },
    }
}

const fn route_passage_map_id(route: ConstitutionalRouteId) -> &'static str {
    match route {
        ConstitutionalRouteId::AuraRidge => STARTER_MAP_ID,
        ConstitutionalRouteId::Boardwalk => BOARDWALK_MAP_ID,
        ConstitutionalRouteId::CurrentSea => CURRENT_SEA_MAP_ID,
        ConstitutionalRouteId::Riptide => "riptide.emergency-intake",
        ConstitutionalRouteId::Glausbahn => "glausbahn.refinement-span",
        ConstitutionalRouteId::CurrentSeanad => "current-seanad.deliberation-chamber",
        ConstitutionalRouteId::AuraWay => "aura-way.design-corridor",
        ConstitutionalRouteId::MntAura => "mnt-aura.aspiration-path",
        ConstitutionalRouteId::BasinMotorspeedway => "basin-motor-speedway.production-circuit",
        ConstitutionalRouteId::StairwayToHeaven => "stairway-to-heaven.ascent-path",
    }
}

const fn route_passage_rows(
    route: ConstitutionalRouteId,
) -> &'static [&'static str; MAP_HEIGHT as usize] {
    match route {
        ConstitutionalRouteId::Riptide => &RIPTIDE_ROUTE_MAP_ROWS,
        ConstitutionalRouteId::CurrentSeanad => &CURRENT_SEANAD_MAP_ROWS,
        ConstitutionalRouteId::MntAura
        | ConstitutionalRouteId::StairwayToHeaven
        | ConstitutionalRouteId::Glausbahn => &ROUND_ROUTE_MAP_ROWS,
        _ => &STRAIGHT_ROUTE_MAP_ROWS,
    }
}

const fn map_id_eq(left: WorldMapId, right: WorldMapId) -> bool {
    match (left, right) {
        (WorldMapId::AuraRidgeGroveApproach, WorldMapId::AuraRidgeGroveApproach)
        | (WorldMapId::BoardwalkReturnVestibule, WorldMapId::BoardwalkReturnVestibule)
        | (
            WorldMapId::CurrentSeaDeepCertificationLanding,
            WorldMapId::CurrentSeaDeepCertificationLanding,
        )
        | (WorldMapId::AuraFieldWorkingLand, WorldMapId::AuraFieldWorkingLand) => true,
        (WorldMapId::AuraBeachCoastalCommons, WorldMapId::AuraBeachCoastalCommons)
        | (WorldMapId::AuraBasinCollisionGrounds, WorldMapId::AuraBasinCollisionGrounds) => true,
        (WorldMapId::ExtractionSite(left), WorldMapId::ExtractionSite(right)) => {
            extraction_site_id_eq(left, right)
        }
        (WorldMapId::RoutePassage(left), WorldMapId::RoutePassage(right)) => {
            route_id_eq(left, right)
        }
        _ => false,
    }
}

const fn extraction_site_id_eq(left: ExtractionSiteId, right: ExtractionSiteId) -> bool {
    matches!(
        (left, right),
        (
            ExtractionSiteId::MntAuraHighMine,
            ExtractionSiteId::MntAuraHighMine
        ) | (
            ExtractionSiteId::StairwayBurdenMine,
            ExtractionSiteId::StairwayBurdenMine
        ) | (
            ExtractionSiteId::HighwayToHellDeepworks,
            ExtractionSiteId::HighwayToHellDeepworks
        ) | (
            ExtractionSiteId::RiptideRecoveryRig,
            ExtractionSiteId::RiptideRecoveryRig
        ) | (
            ExtractionSiteId::CurrentSeaDepthRig,
            ExtractionSiteId::CurrentSeaDepthRig
        )
    )
}

const fn route_id_eq(left: ConstitutionalRouteId, right: ConstitutionalRouteId) -> bool {
    matches!(
        (left, right),
        (
            ConstitutionalRouteId::Boardwalk,
            ConstitutionalRouteId::Boardwalk
        ) | (
            ConstitutionalRouteId::Riptide,
            ConstitutionalRouteId::Riptide
        ) | (
            ConstitutionalRouteId::CurrentSea,
            ConstitutionalRouteId::CurrentSea
        ) | (
            ConstitutionalRouteId::AuraRidge,
            ConstitutionalRouteId::AuraRidge
        ) | (
            ConstitutionalRouteId::Glausbahn,
            ConstitutionalRouteId::Glausbahn
        ) | (
            ConstitutionalRouteId::CurrentSeanad,
            ConstitutionalRouteId::CurrentSeanad
        ) | (
            ConstitutionalRouteId::AuraWay,
            ConstitutionalRouteId::AuraWay
        ) | (
            ConstitutionalRouteId::MntAura,
            ConstitutionalRouteId::MntAura
        ) | (
            ConstitutionalRouteId::BasinMotorspeedway,
            ConstitutionalRouteId::BasinMotorspeedway,
        ) | (
            ConstitutionalRouteId::StairwayToHeaven,
            ConstitutionalRouteId::StairwayToHeaven
        )
    )
}

fn set_tile(rows: &mut [String], x: usize, y: usize, tile: u8) {
    // Every authored map is validated by tests as fixed-width ASCII.
    let replacement = char::from(tile).to_string();
    rows[y].replace_range(x..x + 1, &replacement);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldMapError {
    UnknownMap(String),
    NonCanonicalMap(String),
}

impl std::fmt::Display for WorldMapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownMap(value) => write!(formatter, "unknown Hollow Grove map ID: {value}"),
            Self::NonCanonicalMap(value) => {
                write!(formatter, "noncanonical Hollow Grove map ID: {value}")
            }
        }
    }
}

impl std::error::Error for WorldMapError {}
