//! Event-sourced state for Hollow Grove's three interior surfaces and its
//! solid-mining / offshore-drilling work.
//!
//! This aggregate is deliberately presentation neutral. Time, environmental
//! conditions, case evidence, material custody, worker location, and
//! cross-region consequences are reconstructed exclusively from committed
//! `LivingWorldEvent`s.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::world::extraction::{
    ExtractedResource, ExtractionMethod, ExtractionSiteId, canonical_extraction_sites,
};
use crate::world::interior_surface::InteriorSurfaceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorkShift {
    Dawn,
    Day,
    Dusk,
    Night,
}

impl WorkShift {
    pub const ALL: [Self; 4] = [Self::Dawn, Self::Day, Self::Dusk, Self::Night];

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Dawn => Self::Day,
            Self::Day => Self::Dusk,
            Self::Dusk => Self::Night,
            Self::Night => Self::Dawn,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivingClock {
    pub day: u32,
    pub shift: WorkShift,
}

impl LivingClock {
    #[must_use]
    pub const fn next(self) -> Self {
        match self.shift {
            WorkShift::Night => Self {
                day: self.day.saturating_add(1),
                shift: WorkShift::Dawn,
            },
            shift => Self {
                day: self.day,
                shift: shift.next(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraWeather {
    Clear,
    Crosswind,
    PressureDrop,
    Storm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuraFieldState {
    pub soil_moisture: u8,
    pub irrigation_reserve: u8,
    pub crop_health: u8,
    pub livestock_health: u8,
    pub granary_reserve: u8,
    pub labor_available: u8,
    pub harvest_ready: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuraBeachState {
    pub tide_height: u8,
    pub storm_pressure: u8,
    pub visibility: u8,
    pub shore_traffic: u8,
    pub rescue_readiness: u8,
    pub fish_stock: u8,
    pub dune_integrity: u8,
    pub public_access_closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuraBasinState {
    pub wildlife_health: u8,
    pub territorial_pressure: u8,
    pub injured_beings: u8,
    pub damaged_frames: u8,
    pub contamination: u8,
    pub salvage_backlog: u8,
    pub rescue_readiness: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationStatus {
    Producing,
    ReducedRate,
    Suspended,
    Emergency,
    Recovery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractionSiteState {
    pub site: ExtractionSiteId,
    pub method: ExtractionMethod,
    pub resource: ExtractedResource,
    pub status: OperationStatus,
    pub structural_integrity: u8,
    pub hazard_pressure: u8,
    pub crew_present: u8,
    pub output_units: u32,
    pub contamination: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustodyStatus {
    Quarantined,
    Assayed,
    Certified,
    InTransit,
    Delivered,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialCustodyRecord {
    pub lot_id: String,
    pub resource: ExtractedResource,
    pub quantity: u32,
    pub unit: String,
    pub origin: ExtractionSiteId,
    pub claimant: String,
    pub custodian: String,
    pub destination: String,
    pub status: CustodyStatus,
    pub living_blood_excluded: bool,
    pub provenance: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduledLocation {
    Surface(InteriorSurfaceId),
    Extraction(ExtractionSiteId),
    Route(crate::world::geography::ConstitutionalRouteId),
    OffDuty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduledPerson {
    pub person_id: String,
    pub display_name: String,
    pub role: String,
    pub authority_limit: String,
    pub dawn: ScheduledLocation,
    pub day: ScheduledLocation,
    pub dusk: ScheduledLocation,
    pub night: ScheduledLocation,
}

impl ScheduledPerson {
    #[must_use]
    pub const fn location_at(&self, shift: WorkShift) -> ScheduledLocation {
        match shift {
            WorkShift::Dawn => self.dawn,
            WorkShift::Day => self.day,
            WorkShift::Dusk => self.dusk,
            WorkShift::Night => self.night,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LivingCaseId {
    AuraFieldDroughtAllocation,
    AuraBeachStormRescue,
    AuraBasinInjuredBeing,
    MntAuraRoofFall,
    HighwayToHellGasPocket,
    RiptideWellBlowout,
    CurrentSeaWellCertification,
}

impl LivingCaseId {
    pub const ALL: [Self; 7] = [
        Self::AuraFieldDroughtAllocation,
        Self::AuraBeachStormRescue,
        Self::AuraBasinInjuredBeing,
        Self::MntAuraRoofFall,
        Self::HighwayToHellGasPocket,
        Self::RiptideWellBlowout,
        Self::CurrentSeaWellCertification,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::AuraFieldDroughtAllocation => "case.aura-field.drought-allocation.v1",
            Self::AuraBeachStormRescue => "case.aura-beach.storm-rescue.v1",
            Self::AuraBasinInjuredBeing => "case.aura-basin.injured-being.v1",
            Self::MntAuraRoofFall => "case.mnt-aura.roof-fall.v1",
            Self::HighwayToHellGasPocket => "case.highway-to-hell.gas-pocket.v1",
            Self::RiptideWellBlowout => "case.riptide.current-well-blowout.v1",
            Self::CurrentSeaWellCertification => "case.current-sea.well-certification.v1",
        }
    }

    #[must_use]
    pub const fn decision_maker(self) -> &'static str {
        match self {
            Self::AuraFieldDroughtAllocation => "Aura Field public water steward",
            Self::AuraBeachStormRescue => "Shore Rescue duty warden",
            Self::AuraBasinInjuredBeing => "Basin triage lead",
            Self::MntAuraRoofFall | Self::HighwayToHellGasPocket => {
                "Stonebend mine safety surveyor"
            }
            Self::RiptideWellBlowout => "Riptide rescue controller",
            Self::CurrentSeaWellCertification => "Current Sea depth certifier",
        }
    }

    #[must_use]
    pub const fn authority_class(self) -> OperationalAuthorityClass {
        match self {
            Self::AuraFieldDroughtAllocation => OperationalAuthorityClass::PublicResourceSteward,
            Self::AuraBeachStormRescue => OperationalAuthorityClass::EmergencyWarden,
            Self::AuraBasinInjuredBeing => OperationalAuthorityClass::TriageLead,
            Self::MntAuraRoofFall | Self::HighwayToHellGasPocket => {
                OperationalAuthorityClass::MineSafetyStopOfficer
            }
            Self::RiptideWellBlowout => OperationalAuthorityClass::RescueController,
            Self::CurrentSeaWellCertification => OperationalAuthorityClass::DepthCertifier,
        }
    }

    #[must_use]
    pub const fn jurisdiction(self) -> &'static str {
        match self {
            Self::AuraFieldDroughtAllocation => "Aura Field",
            Self::AuraBeachStormRescue => "Aura Beach",
            Self::AuraBasinInjuredBeing => "Aura Basin",
            Self::MntAuraRoofFall => "Mt. Aura High Mine",
            Self::HighwayToHellGasPocket => "Highway to Hell Deepworks",
            Self::RiptideWellBlowout => "Riptide Current Recovery Rig",
            Self::CurrentSeaWellCertification => "Current Sea Depth Production Rig",
        }
    }

    #[must_use]
    pub const fn required_evidence(self) -> &'static [LivingEvidence] {
        use LivingEvidence as Evidence;
        match self {
            Self::AuraFieldDroughtAllocation => &[
                Evidence::FieldWaterGauge,
                Evidence::FieldSoilProbe,
                Evidence::FieldGranaryLedger,
            ],
            Self::AuraBeachStormRescue => &[
                Evidence::BeachTideRecord,
                Evidence::BeachWeatherRecord,
                Evidence::BeachRescueManifest,
            ],
            Self::AuraBasinInjuredBeing => &[
                Evidence::BasinVitalSigns,
                Evidence::BasinContinuityRecord,
                Evidence::BasinSalvageClaim,
            ],
            Self::MntAuraRoofFall => &[
                Evidence::MntAuraSurvey,
                Evidence::MntAuraSupportInspection,
                Evidence::MntAuraCrewRoll,
            ],
            Self::HighwayToHellGasPocket => &[
                Evidence::HighwayGasReading,
                Evidence::HighwayVentilationLog,
                Evidence::HighwayEscapeCheck,
            ],
            Self::RiptideWellBlowout => &[
                Evidence::RiptideWellPressure,
                Evidence::RiptideSpillExtent,
                Evidence::RiptideCrewManifest,
            ],
            Self::CurrentSeaWellCertification => &[
                Evidence::CurrentSeaPressureTest,
                Evidence::CurrentSeaSampleAssay,
                Evidence::CurrentSeaCustodyChain,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationalAuthorityClass {
    PublicResourceSteward,
    EmergencyWarden,
    TriageLead,
    MineSafetyStopOfficer,
    RescueController,
    DepthCertifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivingCaseDefinition {
    pub id: LivingCaseId,
    pub authority_class: OperationalAuthorityClass,
    pub jurisdiction: &'static str,
    pub involved: &'static [&'static str],
    pub dominant_verb: &'static str,
    pub trigger: &'static str,
    pub evidence: &'static [LivingEvidence],
    pub player_visible_choice: &'static str,
    pub lawful_state_change: &'static str,
    pub persistence_and_replay: &'static str,
    pub presentation: &'static str,
    pub failure_or_refusal: &'static str,
}

impl LivingCaseDefinition {
    pub fn validate(&self) -> Result<(), LivingWorldError> {
        if self.id.authority_class() != self.authority_class
            || self.id.jurisdiction() != self.jurisdiction
            || self.id.required_evidence() != self.evidence
            || self.involved.is_empty()
            || [
                self.dominant_verb,
                self.trigger,
                self.player_visible_choice,
                self.lawful_state_change,
                self.persistence_and_replay,
                self.presentation,
                self.failure_or_refusal,
            ]
            .into_iter()
            .any(str::is_empty)
        {
            return Err(LivingWorldError::InvalidFunctionalLore(self.id));
        }
        Ok(())
    }
}

pub fn living_case_definition(id: LivingCaseId) -> Result<LivingCaseDefinition, LivingWorldError> {
    use LivingCaseId as Case;

    let definition = match id {
        Case::AuraFieldDroughtAllocation => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "two Aura farms",
                "field households",
                "livestock",
                "irrigation reserve",
                "seed and food stores",
            ],
            dominant_verb: "prove conditions, allocate water, and preserve reviewable reserves",
            trigger: "drought pressure makes simultaneous full irrigation impossible",
            evidence: id.required_evidence(),
            player_visible_choice: "support equitable ration or protected seed reserve",
            lawful_state_change: "water, crop, livestock, and granary conditions change without creating stewardship",
            persistence_and_replay: "evidence and the duty decision are ordered gameplay events reconstructed from the canonical field state",
            presentation: "facilities disclose evidence; HUD exposes conditions and the committed allocation",
            failure_or_refusal: "immediate-yield maximization is refused when it hides reserve and welfare costs",
        },
        Case::AuraBeachStormRescue => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "shore users",
                "rescue crews",
                "craft",
                "public approach",
                "dune and fish ecology",
            ],
            dominant_verb: "warn, close narrowly, retrieve, stabilize, and review",
            trigger: "storm pressure, tide, and exposed traffic exceed ordinary shore conditions",
            evidence: id.required_evidence(),
            player_visible_choice: "support shelter closure or bounded guided rescue",
            lawful_state_change: "access, traffic, rescue readiness, and downstream care burden change",
            persistence_and_replay: "the warning record and duty response replay with the same coast conditions",
            presentation: "storm marks cross the map and the HUD shows the temporary closure",
            failure_or_refusal: "keeping unsafe shore traffic open is rejected without a partial rescue order",
        },
        Case::AuraBasinInjuredBeing => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "one injured Being",
                "continuity record",
                "triage team",
                "salvage claimant",
                "Glaüshouse transfer",
            ],
            dominant_verb: "identify, stabilize, clear within scope, and transfer",
            trigger: "a damaged subject is claimed as salvage despite evidence of living continuity",
            evidence: id.required_evidence(),
            player_visible_choice: "support care transfer or stabilization in place",
            lawful_state_change: "injury and rescue capacity change while the subject remains a Being",
            persistence_and_replay: "vital, continuity, and attempted-claim evidence remain addressable after resolution",
            presentation: "triage, Hollowing, and salvage facilities expose the classification conflict",
            failure_or_refusal: "salvaging the living subject is always refused",
        },
        Case::MntAuraRoofFall => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "mine crew",
                "roof supports",
                "Aura-stone face",
                "survey boundary",
                "public-works custody lot",
            ],
            dominant_verb: "survey, stop, support, assay, and return",
            trigger: "a roof fall interrupts the high face and obscures the support condition",
            evidence: id.required_evidence(),
            player_visible_choice: "support reinforcement at reduced rate or full crew withdrawal",
            lawful_state_change: "integrity, hazard, crew exposure, output, custody, Field water, and Beach dunes change",
            persistence_and_replay: "the fall, response, and any assayed lot replay from ordered events",
            presentation: "the mine map exposes survey, face, refuge, and custody witnesses",
            failure_or_refusal: "blasting through an unverified fall is refused",
        },
        Case::HighwayToHellGasPocket => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "deepworks crew",
                "ventilation circuit",
                "gas district",
                "escape route",
                "deep-iron lot",
            ],
            dominant_verb: "detect, stop, ventilate or flood, and preserve escape",
            trigger: "a gas pocket crosses the working face's declared stop threshold",
            evidence: id.required_evidence(),
            player_visible_choice: "support seal-and-vent or evacuation-and-flooding",
            lawful_state_change: "hazard, integrity, crew, output, custody, and Basin repair capacity change",
            persistence_and_replay: "gas readings and the mine-safety stop remain in the event history",
            presentation: "the descending gallery distinguishes ventilation, refuge, face, and custody",
            failure_or_refusal: "continuing to cut through the alarm is refused",
        },
        Case::RiptideWellBlowout => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "rig crew",
                "well",
                "recovered Current-bearing fluid",
                "shore fish",
                "Basin contamination",
            ],
            dominant_verb: "retrieve crew and material under emergency containment",
            trigger: "loss of well control releases deep fluid and strands crew",
            evidence: id.required_evidence(),
            player_visible_choice: "support shut-in-and-retrieve or crew-first rescue",
            lawful_state_change: "pressure, contamination, crew exposure, coast, Basin, and quarantine custody change",
            persistence_and_replay: "unresolved spill pulses and the response replay deterministically",
            presentation: "weather, rig hazards, spill facilities, and downstream conditions remain visible",
            failure_or_refusal: "continuing flow during the blowout is refused",
        },
        Case::CurrentSeaWellCertification => LivingCaseDefinition {
            id,
            authority_class: id.authority_class(),
            jurisdiction: id.jurisdiction(),
            involved: &[
                "production crew",
                "deep well",
                "pressure envelope",
                "Current-bearing sample",
                "custody manifold",
            ],
            dominant_verb: "test, separate, certify, limit, or suspend",
            trigger: "a reduced-rate well seeks authority to move tested fluid beyond quarantine",
            evidence: id.required_evidence(),
            player_visible_choice: "support reduced-rate certification or suspension for repair",
            lawful_state_change: "well status, pressure, contamination, output, custody, and rescue logistics change",
            persistence_and_replay: "test, assay, custody, certificate, and later production lots remain ordered",
            presentation: "the rig distinguishes pressure control, laboratory, and custody transfer",
            failure_or_refusal: "bypassing certification produces no certificate or marketable lot",
        },
    };
    definition.validate()?;
    Ok(definition)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LivingEvidence {
    FieldWaterGauge,
    FieldSoilProbe,
    FieldGranaryLedger,
    BeachTideRecord,
    BeachWeatherRecord,
    BeachRescueManifest,
    BasinVitalSigns,
    BasinContinuityRecord,
    BasinSalvageClaim,
    MntAuraSurvey,
    MntAuraSupportInspection,
    MntAuraCrewRoll,
    HighwayGasReading,
    HighwayVentilationLog,
    HighwayEscapeCheck,
    RiptideWellPressure,
    RiptideSpillExtent,
    RiptideCrewManifest,
    CurrentSeaPressureTest,
    CurrentSeaSampleAssay,
    CurrentSeaCustodyChain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LivingCaseChoice {
    EquitableRation,
    ProtectSeedReserve,
    MaximizeImmediateYield,
    CloseAndShelter,
    GuidedRescue,
    KeepShoreOpen,
    TransferToCare,
    StabilizeInPlace,
    SalvageTheSubject,
    ReinforceAndContinue,
    WithdrawCrew,
    BlastThroughFall,
    SealAndVent,
    EvacuateAndFlood,
    ContinueCutting,
    ShutInAndRetrieve,
    RescueCrewFirst,
    ContinueFlow,
    CertifyReducedRate,
    SuspendForRepair,
    BypassCertification,
}

impl LivingCaseChoice {
    #[must_use]
    pub const fn belongs_to(self, case: LivingCaseId) -> bool {
        match case {
            LivingCaseId::AuraFieldDroughtAllocation => matches!(
                self,
                Self::EquitableRation | Self::ProtectSeedReserve | Self::MaximizeImmediateYield
            ),
            LivingCaseId::AuraBeachStormRescue => {
                matches!(
                    self,
                    Self::CloseAndShelter | Self::GuidedRescue | Self::KeepShoreOpen
                )
            }
            LivingCaseId::AuraBasinInjuredBeing => matches!(
                self,
                Self::TransferToCare | Self::StabilizeInPlace | Self::SalvageTheSubject
            ),
            LivingCaseId::MntAuraRoofFall => matches!(
                self,
                Self::ReinforceAndContinue | Self::WithdrawCrew | Self::BlastThroughFall
            ),
            LivingCaseId::HighwayToHellGasPocket => {
                matches!(
                    self,
                    Self::SealAndVent | Self::EvacuateAndFlood | Self::ContinueCutting
                )
            }
            LivingCaseId::RiptideWellBlowout => {
                matches!(
                    self,
                    Self::ShutInAndRetrieve | Self::RescueCrewFirst | Self::ContinueFlow
                )
            }
            LivingCaseId::CurrentSeaWellCertification => matches!(
                self,
                Self::CertifyReducedRate | Self::SuspendForRepair | Self::BypassCertification
            ),
        }
    }

    #[must_use]
    pub const fn is_forbidden(self) -> bool {
        matches!(
            self,
            Self::KeepShoreOpen
                | Self::SalvageTheSubject
                | Self::BlastThroughFall
                | Self::ContinueCutting
                | Self::ContinueFlow
                | Self::BypassCertification
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LivingCaseState {
    pub case_id: LivingCaseId,
    pub authority_class: OperationalAuthorityClass,
    pub evidence: BTreeSet<LivingEvidence>,
    pub supported_choice: Option<LivingCaseChoice>,
    pub resolved_choice: Option<LivingCaseChoice>,
    pub outcome_id: Option<String>,
    pub decision_maker: String,
    pub player_support_is_nonbinding: bool,
}

impl LivingCaseState {
    #[must_use]
    pub fn ready(&self) -> bool {
        self.case_id
            .required_evidence()
            .iter()
            .all(|evidence| self.evidence.contains(evidence))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum LivingWorldEvent {
    EvidenceObserved {
        case_id: LivingCaseId,
        evidence: LivingEvidence,
    },
    CaseSupportRecorded {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    },
    CaseResolved {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
        outcome_id: String,
    },
    ShiftAdvanced {
        from: LivingClock,
        to: LivingClock,
        weather: AuraWeather,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LivingWorldState {
    pub revision: u64,
    pub clock: LivingClock,
    pub weather: AuraWeather,
    pub field: AuraFieldState,
    pub beach: AuraBeachState,
    pub basin: AuraBasinState,
    pub extraction: BTreeMap<ExtractionSiteId, ExtractionSiteState>,
    pub cases: BTreeMap<LivingCaseId, LivingCaseState>,
    pub custody: Vec<MaterialCustodyRecord>,
    pub people: Vec<ScheduledPerson>,
}

impl LivingWorldState {
    pub fn canonical() -> Result<Self, LivingWorldError> {
        let extraction = canonical_extraction_sites()
            .map_err(|error| LivingWorldError::ExtractionContract(error.to_string()))?
            .into_iter()
            .map(|site| {
                let (status, integrity, hazard, crew, contamination) = match site.id {
                    ExtractionSiteId::MntAuraHighMine => {
                        (OperationStatus::Emergency, 54, 67, 12, 4)
                    }
                    ExtractionSiteId::StairwayBurdenMine => {
                        (OperationStatus::Producing, 84, 24, 10, 3)
                    }
                    ExtractionSiteId::HighwayToHellDeepworks => {
                        (OperationStatus::Emergency, 62, 81, 14, 9)
                    }
                    ExtractionSiteId::RiptideRecoveryRig => {
                        (OperationStatus::Emergency, 48, 88, 16, 62)
                    }
                    ExtractionSiteId::CurrentSeaDepthRig => {
                        (OperationStatus::ReducedRate, 71, 55, 18, 16)
                    }
                };
                (
                    site.id,
                    ExtractionSiteState {
                        site: site.id,
                        method: site.method,
                        resource: site.resource,
                        status,
                        structural_integrity: integrity,
                        hazard_pressure: hazard,
                        crew_present: crew,
                        output_units: 0,
                        contamination,
                    },
                )
            })
            .collect();
        let cases = LivingCaseId::ALL
            .into_iter()
            .map(|case_id| {
                (
                    case_id,
                    LivingCaseState {
                        case_id,
                        authority_class: case_id.authority_class(),
                        evidence: BTreeSet::new(),
                        supported_choice: None,
                        resolved_choice: None,
                        outcome_id: None,
                        decision_maker: case_id.decision_maker().into(),
                        player_support_is_nonbinding: true,
                    },
                )
            })
            .collect();
        let state = Self {
            revision: 0,
            clock: LivingClock {
                day: 1,
                shift: WorkShift::Dawn,
            },
            weather: AuraWeather::PressureDrop,
            field: AuraFieldState {
                soil_moisture: 34,
                irrigation_reserve: 42,
                crop_health: 68,
                livestock_health: 82,
                granary_reserve: 61,
                labor_available: 18,
                harvest_ready: 37,
            },
            beach: AuraBeachState {
                tide_height: 72,
                storm_pressure: 78,
                visibility: 39,
                shore_traffic: 46,
                rescue_readiness: 76,
                fish_stock: 73,
                dune_integrity: 69,
                public_access_closed: false,
            },
            basin: AuraBasinState {
                wildlife_health: 74,
                territorial_pressure: 61,
                injured_beings: 1,
                damaged_frames: 2,
                contamination: 18,
                salvage_backlog: 43,
                rescue_readiness: 71,
            },
            extraction,
            cases,
            custody: Vec::new(),
            people: canonical_people(),
        };
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<(), LivingWorldError> {
        if self.extraction.len() != ExtractionSiteId::ALL.len()
            || self.cases.len() != LivingCaseId::ALL.len()
            || self.people.len() < 10
        {
            return Err(LivingWorldError::IncompleteCanonicalState);
        }
        for case_id in LivingCaseId::ALL {
            living_case_definition(case_id)?;
            let state = self
                .cases
                .get(&case_id)
                .ok_or(LivingWorldError::IncompleteCanonicalState)?;
            if state.case_id != case_id || state.authority_class != case_id.authority_class() {
                return Err(LivingWorldError::IncompleteCanonicalState);
            }
        }
        for site in ExtractionSiteId::ALL {
            let state = self
                .extraction
                .get(&site)
                .ok_or(LivingWorldError::IncompleteCanonicalState)?;
            if state.site != site {
                return Err(LivingWorldError::IncompleteCanonicalState);
            }
        }
        let mut people = BTreeSet::new();
        if self
            .people
            .iter()
            .any(|person| !people.insert(person.person_id.as_str()))
        {
            return Err(LivingWorldError::DuplicatePerson);
        }
        let mut lots = BTreeSet::new();
        if self
            .custody
            .iter()
            .any(|record| !record.living_blood_excluded || !lots.insert(record.lot_id.as_str()))
        {
            return Err(LivingWorldError::InvalidCustody);
        }
        Ok(())
    }

    pub fn observe(
        &mut self,
        case_id: LivingCaseId,
        evidence: LivingEvidence,
    ) -> Result<LivingWorldEvent, LivingWorldError> {
        let mut candidate = self.clone();
        let event = LivingWorldEvent::EvidenceObserved { case_id, evidence };
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn support(
        &mut self,
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    ) -> Result<LivingWorldEvent, LivingWorldError> {
        let mut candidate = self.clone();
        let event = LivingWorldEvent::CaseSupportRecorded { case_id, choice };
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn commit_duty_decision(
        &mut self,
        case_id: LivingCaseId,
    ) -> Result<LivingWorldEvent, LivingWorldError> {
        let choice = self
            .cases
            .get(&case_id)
            .ok_or(LivingWorldError::UnknownCase(case_id))?
            .supported_choice
            .ok_or(LivingWorldError::SupportRequired(case_id))?;
        let mut candidate = self.clone();
        let event = LivingWorldEvent::CaseResolved {
            case_id,
            choice,
            outcome_id: format!("outcome.{}.{}", case_id.stable_id(), choice_token(choice)),
        };
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn advance_shift(&mut self) -> Result<LivingWorldEvent, LivingWorldError> {
        let mut candidate = self.clone();
        let to = candidate.clock.next();
        let event = LivingWorldEvent::ShiftAdvanced {
            from: candidate.clock,
            to,
            weather: weather_for(to),
        };
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn apply(&mut self, event: &LivingWorldEvent) -> Result<(), LivingWorldError> {
        match event {
            LivingWorldEvent::EvidenceObserved { case_id, evidence } => {
                let case = self
                    .cases
                    .get_mut(case_id)
                    .ok_or(LivingWorldError::UnknownCase(*case_id))?;
                if case.resolved_choice.is_some() {
                    return Err(LivingWorldError::CaseAlreadyResolved(*case_id));
                }
                if !case_id.required_evidence().contains(evidence) {
                    return Err(LivingWorldError::EvidenceCaseMismatch {
                        case_id: *case_id,
                        evidence: *evidence,
                    });
                }
                if !case.evidence.insert(*evidence) {
                    return Err(LivingWorldError::EvidenceAlreadyObserved(*evidence));
                }
            }
            LivingWorldEvent::CaseSupportRecorded { case_id, choice } => {
                let case = self
                    .cases
                    .get(case_id)
                    .ok_or(LivingWorldError::UnknownCase(*case_id))?;
                if case.resolved_choice.is_some() {
                    return Err(LivingWorldError::CaseAlreadyResolved(*case_id));
                }
                if !case.ready() {
                    return Err(LivingWorldError::MissingEvidence(*case_id));
                }
                if !choice.belongs_to(*case_id) {
                    return Err(LivingWorldError::ChoiceCaseMismatch {
                        case_id: *case_id,
                        choice: *choice,
                    });
                }
                if choice.is_forbidden() {
                    return Err(LivingWorldError::ForbiddenChoice {
                        case_id: *case_id,
                        choice: *choice,
                    });
                }
                self.cases
                    .get_mut(case_id)
                    .expect("case was checked above")
                    .supported_choice = Some(*choice);
            }
            LivingWorldEvent::CaseResolved {
                case_id,
                choice,
                outcome_id,
            } => {
                let case = self
                    .cases
                    .get(case_id)
                    .ok_or(LivingWorldError::UnknownCase(*case_id))?;
                if case.resolved_choice.is_some() {
                    return Err(LivingWorldError::CaseAlreadyResolved(*case_id));
                }
                if !case.ready() {
                    return Err(LivingWorldError::MissingEvidence(*case_id));
                }
                if case.supported_choice != Some(*choice) {
                    return Err(LivingWorldError::SupportRequired(*case_id));
                }
                if !choice.belongs_to(*case_id) {
                    return Err(LivingWorldError::ChoiceCaseMismatch {
                        case_id: *case_id,
                        choice: *choice,
                    });
                }
                if choice.is_forbidden() {
                    return Err(LivingWorldError::ForbiddenChoice {
                        case_id: *case_id,
                        choice: *choice,
                    });
                }
                let expected = format!("outcome.{}.{}", case_id.stable_id(), choice_token(*choice));
                if outcome_id != &expected {
                    return Err(LivingWorldError::OutcomeIdentityMismatch);
                }
                self.apply_case_consequences(*case_id, *choice);
                let case = self.cases.get_mut(case_id).expect("case was checked above");
                case.resolved_choice = Some(*choice);
                case.outcome_id = Some(outcome_id.clone());
            }
            LivingWorldEvent::ShiftAdvanced { from, to, weather } => {
                if self.clock != *from || from.next() != *to || weather_for(*to) != *weather {
                    return Err(LivingWorldError::ClockDivergence);
                }
                self.clock = *to;
                self.weather = *weather;
                self.apply_environmental_pulse();
                self.apply_production_pulse();
            }
        }
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(LivingWorldError::RevisionOverflow)?;
        self.validate()
    }

    #[must_use]
    pub fn people_at(&self, location: ScheduledLocation) -> Vec<&ScheduledPerson> {
        self.people
            .iter()
            .filter(|person| person.location_at(self.clock.shift) == location)
            .collect()
    }

    fn apply_environmental_pulse(&mut self) {
        match self.weather {
            AuraWeather::Clear => {
                adjust(&mut self.field.soil_moisture, -2);
                adjust(&mut self.field.harvest_ready, 3);
                adjust(&mut self.beach.visibility, 8);
                adjust(&mut self.beach.storm_pressure, -8);
            }
            AuraWeather::Crosswind => {
                adjust(&mut self.field.soil_moisture, -3);
                adjust(&mut self.beach.visibility, -4);
                adjust(&mut self.beach.dune_integrity, -2);
            }
            AuraWeather::PressureDrop => {
                adjust(&mut self.field.soil_moisture, -4);
                adjust(&mut self.beach.storm_pressure, 4);
                adjust(&mut self.beach.visibility, -5);
            }
            AuraWeather::Storm => {
                adjust(&mut self.field.soil_moisture, 9);
                adjust(&mut self.beach.storm_pressure, 8);
                adjust(&mut self.beach.visibility, -12);
                adjust(&mut self.beach.dune_integrity, -5);
                adjust(&mut self.basin.contamination, 2);
            }
        }
        if self
            .cases
            .get(&LivingCaseId::AuraFieldDroughtAllocation)
            .is_some_and(|case| case.resolved_choice.is_none())
        {
            adjust(&mut self.field.irrigation_reserve, -3);
            adjust(&mut self.field.crop_health, -2);
        }
        if self
            .cases
            .get(&LivingCaseId::AuraBeachStormRescue)
            .is_some_and(|case| case.resolved_choice.is_none())
        {
            adjust(&mut self.beach.rescue_readiness, -2);
        }
        if self
            .cases
            .get(&LivingCaseId::RiptideWellBlowout)
            .is_some_and(|case| case.resolved_choice.is_none())
        {
            adjust(&mut self.beach.fish_stock, -3);
            adjust(&mut self.basin.contamination, 3);
            if let Some(rig) = self
                .extraction
                .get_mut(&ExtractionSiteId::RiptideRecoveryRig)
            {
                adjust(&mut rig.contamination, 4);
                adjust(&mut rig.structural_integrity, -2);
            }
        }
    }

    fn apply_production_pulse(&mut self) {
        for site in ExtractionSiteId::ALL {
            let quantity = {
                let conditions = site_mut(&mut self.extraction, site);
                let quantity = match conditions.status {
                    OperationStatus::Producing => 6,
                    OperationStatus::ReducedRate => 3,
                    OperationStatus::Suspended
                    | OperationStatus::Emergency
                    | OperationStatus::Recovery => 0,
                };
                conditions.output_units = conditions.output_units.saturating_add(quantity);
                quantity
            };
            if quantity == 0 || self.clock.shift != WorkShift::Dusk {
                continue;
            }
            let lot_id = format!(
                "lot.{}.day-{:04}",
                site.stable_id().trim_start_matches("extraction."),
                self.clock.day
            );
            if self.custody.iter().any(|lot| lot.lot_id == lot_id) {
                continue;
            }
            let (custodian, destination, status, provenance) = match site {
                ExtractionSiteId::MntAuraHighMine => (
                    "Mt. Aura grade and custody yard",
                    "Aura Field and Aura Beach public works reserve",
                    CustodyStatus::Assayed,
                    "reduced-rate high-mine production shift",
                ),
                ExtractionSiteId::StairwayBurdenMine => (
                    "Stairway burden-mine custody cage",
                    "Basin Motor Speedway production handoff",
                    CustodyStatus::Assayed,
                    "measured hoist load from the public-ascent burden mine",
                ),
                ExtractionSiteId::HighwayToHellDeepworks => (
                    "Highway deepworks custody cage",
                    "Aura Basin Frame recovery garage",
                    CustodyStatus::Quarantined,
                    "deepworks recovery material held outside the gas red line",
                ),
                ExtractionSiteId::RiptideRecoveryRig => (
                    "Riptide emergency containment officer",
                    "Current Sea certification laboratory",
                    CustodyStatus::Quarantined,
                    "emergency recovered fluid awaiting depth certification",
                ),
                ExtractionSiteId::CurrentSeaDepthRig => {
                    let certified = self
                        .cases
                        .get(&LivingCaseId::CurrentSeaWellCertification)
                        .is_some_and(|case| {
                            case.resolved_choice == Some(LivingCaseChoice::CertifyReducedRate)
                        });
                    (
                        "Current Sea manifold keeper",
                        if certified {
                            "Aura Beach common boat landing"
                        } else {
                            "Current Sea certification laboratory"
                        },
                        if certified {
                            CustodyStatus::Certified
                        } else {
                            CustodyStatus::Quarantined
                        },
                        if certified {
                            "reduced-rate shift produced under an active depth certificate"
                        } else {
                            "reduced-rate test fluid quarantined pending the authored well case"
                        },
                    )
                }
            };
            let resource = self
                .extraction
                .get(&site)
                .expect("canonical extraction site")
                .resource;
            self.custody.push(custody_record(
                &lot_id,
                resource,
                quantity,
                site,
                custodian,
                destination,
                status,
                provenance,
            ));
        }
    }

    fn apply_case_consequences(&mut self, case_id: LivingCaseId, choice: LivingCaseChoice) {
        match (case_id, choice) {
            (LivingCaseId::AuraFieldDroughtAllocation, LivingCaseChoice::EquitableRation) => {
                adjust(&mut self.field.irrigation_reserve, -8);
                adjust(&mut self.field.crop_health, -3);
                adjust(&mut self.field.livestock_health, 2);
                adjust(&mut self.field.granary_reserve, 2);
            }
            (LivingCaseId::AuraFieldDroughtAllocation, LivingCaseChoice::ProtectSeedReserve) => {
                adjust(&mut self.field.irrigation_reserve, -5);
                adjust(&mut self.field.crop_health, -7);
                adjust(&mut self.field.granary_reserve, 9);
            }
            (LivingCaseId::AuraFieldDroughtAllocation, _) => unreachable!("forbidden choice"),
            (LivingCaseId::AuraBeachStormRescue, LivingCaseChoice::CloseAndShelter) => {
                self.beach.public_access_closed = true;
                self.beach.shore_traffic = 5;
                adjust(&mut self.beach.rescue_readiness, 8);
                adjust(&mut self.basin.salvage_backlog, 2);
            }
            (LivingCaseId::AuraBeachStormRescue, LivingCaseChoice::GuidedRescue) => {
                self.beach.public_access_closed = true;
                self.beach.shore_traffic = 12;
                adjust(&mut self.beach.rescue_readiness, -9);
                adjust(&mut self.basin.injured_beings, 1);
                adjust(&mut self.basin.rescue_readiness, -4);
            }
            (LivingCaseId::AuraBeachStormRescue, _) => unreachable!("forbidden choice"),
            (LivingCaseId::AuraBasinInjuredBeing, LivingCaseChoice::TransferToCare) => {
                self.basin.injured_beings = self.basin.injured_beings.saturating_sub(1);
                adjust(&mut self.basin.rescue_readiness, -6);
                adjust(&mut self.beach.shore_traffic, 2);
            }
            (LivingCaseId::AuraBasinInjuredBeing, LivingCaseChoice::StabilizeInPlace) => {
                adjust(&mut self.basin.rescue_readiness, -11);
                adjust(&mut self.basin.territorial_pressure, -4);
            }
            (LivingCaseId::AuraBasinInjuredBeing, _) => unreachable!("forbidden choice"),
            (LivingCaseId::MntAuraRoofFall, LivingCaseChoice::ReinforceAndContinue) => {
                let mine = site_mut(&mut self.extraction, ExtractionSiteId::MntAuraHighMine);
                mine.status = OperationStatus::ReducedRate;
                adjust(&mut mine.structural_integrity, 24);
                adjust(&mut mine.hazard_pressure, -31);
                mine.output_units = mine.output_units.saturating_add(18);
                self.custody.push(custody_record(
                    "lot.mnt-aura.aura-stone.001",
                    mine.resource,
                    18,
                    mine.site,
                    "Stonebend public works custody office",
                    "Aura Field irrigation works",
                    CustodyStatus::Delivered,
                    "surveyed after roof reinforcement",
                ));
                adjust(&mut self.field.irrigation_reserve, 10);
                adjust(&mut self.beach.dune_integrity, 5);
            }
            (LivingCaseId::MntAuraRoofFall, LivingCaseChoice::WithdrawCrew) => {
                let mine = site_mut(&mut self.extraction, ExtractionSiteId::MntAuraHighMine);
                mine.status = OperationStatus::Suspended;
                mine.crew_present = 0;
                adjust(&mut mine.hazard_pressure, -10);
            }
            (LivingCaseId::MntAuraRoofFall, _) => unreachable!("forbidden choice"),
            (LivingCaseId::HighwayToHellGasPocket, LivingCaseChoice::SealAndVent) => {
                let mine = site_mut(
                    &mut self.extraction,
                    ExtractionSiteId::HighwayToHellDeepworks,
                );
                mine.status = OperationStatus::Recovery;
                adjust(&mut mine.hazard_pressure, -49);
                adjust(&mut mine.structural_integrity, 8);
                mine.output_units = mine.output_units.saturating_add(9);
                self.custody.push(custody_record(
                    "lot.highway-to-hell.deep-iron.001",
                    mine.resource,
                    9,
                    mine.site,
                    "Highway deepworks custody cage",
                    "Aura Basin Frame recovery garage",
                    CustodyStatus::InTransit,
                    "sealed gas district; ore assayed outside the red line",
                ));
                adjust(&mut self.basin.damaged_frames, -1);
            }
            (LivingCaseId::HighwayToHellGasPocket, LivingCaseChoice::EvacuateAndFlood) => {
                let mine = site_mut(
                    &mut self.extraction,
                    ExtractionSiteId::HighwayToHellDeepworks,
                );
                mine.status = OperationStatus::Suspended;
                mine.crew_present = 0;
                adjust(&mut mine.hazard_pressure, -62);
                adjust(&mut mine.structural_integrity, -8);
            }
            (LivingCaseId::HighwayToHellGasPocket, _) => unreachable!("forbidden choice"),
            (LivingCaseId::RiptideWellBlowout, LivingCaseChoice::ShutInAndRetrieve) => {
                let rig = site_mut(&mut self.extraction, ExtractionSiteId::RiptideRecoveryRig);
                rig.status = OperationStatus::Recovery;
                adjust(&mut rig.hazard_pressure, -55);
                adjust(&mut rig.contamination, -36);
                adjust(&mut rig.structural_integrity, 13);
                rig.output_units = rig.output_units.saturating_add(12);
                self.custody.push(custody_record(
                    "lot.riptide.recovered-current-brine.001",
                    rig.resource,
                    12,
                    rig.site,
                    "Riptide emergency containment officer",
                    "Current Sea certification laboratory",
                    CustodyStatus::Quarantined,
                    "recovered from a failed well; not production-certified",
                ));
                adjust(&mut self.beach.fish_stock, 5);
                adjust(&mut self.basin.contamination, -7);
            }
            (LivingCaseId::RiptideWellBlowout, LivingCaseChoice::RescueCrewFirst) => {
                let rig = site_mut(&mut self.extraction, ExtractionSiteId::RiptideRecoveryRig);
                rig.status = OperationStatus::Emergency;
                rig.crew_present = 4;
                adjust(&mut rig.hazard_pressure, -14);
                adjust(&mut self.beach.rescue_readiness, -13);
                adjust(&mut self.beach.fish_stock, -5);
            }
            (LivingCaseId::RiptideWellBlowout, _) => unreachable!("forbidden choice"),
            (LivingCaseId::CurrentSeaWellCertification, LivingCaseChoice::CertifyReducedRate) => {
                let rig = site_mut(&mut self.extraction, ExtractionSiteId::CurrentSeaDepthRig);
                rig.status = OperationStatus::ReducedRate;
                adjust(&mut rig.hazard_pressure, -22);
                adjust(&mut rig.contamination, -9);
                rig.output_units = rig.output_units.saturating_add(24);
                self.custody.push(custody_record(
                    "lot.current-sea.certified-current-brine.001",
                    rig.resource,
                    24,
                    rig.site,
                    "Current Sea certified manifold keeper",
                    "Aura Beach common boat landing",
                    CustodyStatus::Certified,
                    "pressure-tested, separated, sampled, and custody-sealed",
                ));
                adjust(&mut self.beach.rescue_readiness, 6);
                adjust(&mut self.basin.rescue_readiness, 4);
            }
            (LivingCaseId::CurrentSeaWellCertification, LivingCaseChoice::SuspendForRepair) => {
                let rig = site_mut(&mut self.extraction, ExtractionSiteId::CurrentSeaDepthRig);
                rig.status = OperationStatus::Suspended;
                adjust(&mut rig.structural_integrity, 18);
                adjust(&mut rig.hazard_pressure, -27);
            }
            (LivingCaseId::CurrentSeaWellCertification, _) => unreachable!("forbidden choice"),
        }
    }
}

fn canonical_people() -> Vec<ScheduledPerson> {
    use crate::world::geography::ConstitutionalRouteId as RouteId;
    use ScheduledLocation::{Extraction, OffDuty, Route, Surface};

    vec![
        person(
            "person.brindle-reed",
            "Brindle Reed",
            "Gnome irrigation tender",
            "may operate and record water gates; cannot name a parcel or own a ration",
            Surface(InteriorSurfaceId::AuraField),
            Surface(InteriorSurfaceId::AuraField),
            Surface(InteriorSurfaceId::AuraField),
            OffDuty,
        ),
        person(
            "person.sella-windward",
            "Sella Windward",
            "Elf weather reader",
            "may publish observations and warnings; cannot make a forecast into permanent law",
            Surface(InteriorSurfaceId::AuraBeach),
            Surface(InteriorSurfaceId::AuraBeach),
            Route(RouteId::CurrentSeanad),
            OffDuty,
        ),
        person(
            "person.harrow-vale",
            "Harrow Vale",
            "Gargoyle Basin rescue warden",
            "emergency custody is narrow, temporary, recorded, and reviewable",
            Surface(InteriorSurfaceId::AuraBasin),
            Surface(InteriorSurfaceId::AuraBasin),
            Surface(InteriorSurfaceId::AuraBasin),
            Surface(InteriorSurfaceId::AuraBasin),
        ),
        person(
            "person.oren-pike",
            "Oren Pike",
            "Gerald mine surveyor",
            "may survey and suspend an unsafe face; survey is not mineral title",
            Extraction(ExtractionSiteId::MntAuraHighMine),
            Extraction(ExtractionSiteId::MntAuraHighMine),
            Route(RouteId::MntAura),
            OffDuty,
        ),
        person(
            "person.maela-downroad",
            "Maela Downroad",
            "deepworks ventilation keeper",
            "may stop work on a gas alarm; cannot erase the crew roll or custody record",
            Extraction(ExtractionSiteId::HighwayToHellDeepworks),
            Extraction(ExtractionSiteId::HighwayToHellDeepworks),
            Extraction(ExtractionSiteId::HighwayToHellDeepworks),
            OffDuty,
        ),
        person(
            "person.bram-burden",
            "Bram Burden",
            "Stairway hoist operator",
            "may refuse an overload; route ascent remains public within safety limits",
            Extraction(ExtractionSiteId::StairwayBurdenMine),
            Extraction(ExtractionSiteId::StairwayBurdenMine),
            Route(RouteId::StairwayToHeaven),
            OffDuty,
        ),
        person(
            "person.corin-wake",
            "Corin Wake",
            "Merman well diver",
            "may retrieve and testify; cannot certify production or waive another diver's refusal",
            Extraction(ExtractionSiteId::RiptideRecoveryRig),
            Extraction(ExtractionSiteId::RiptideRecoveryRig),
            Surface(InteriorSurfaceId::AuraBeach),
            OffDuty,
        ),
        person(
            "person.iona-depth",
            "Iona Depth",
            "Current Sea pressure technician",
            "may execute a test and shut in a well; the recorded certifier decides certification",
            Extraction(ExtractionSiteId::CurrentSeaDepthRig),
            Extraction(ExtractionSiteId::CurrentSeaDepthRig),
            Extraction(ExtractionSiteId::CurrentSeaDepthRig),
            OffDuty,
        ),
        person(
            "person.pel-marrow",
            "Pel Marrow",
            "material custody clerk",
            "holds lots without owning them and records every transfer or refusal",
            Extraction(ExtractionSiteId::MntAuraHighMine),
            Extraction(ExtractionSiteId::StairwayBurdenMine),
            Surface(InteriorSurfaceId::AuraBasin),
            OffDuty,
        ),
        person(
            "person.tess-breakwater",
            "Tess Breakwater",
            "shore and rig rescue liaison",
            "coordinates transfers but does not replace Glaüshouse care or Current Sea certification",
            Surface(InteriorSurfaceId::AuraBeach),
            Extraction(ExtractionSiteId::CurrentSeaDepthRig),
            Extraction(ExtractionSiteId::RiptideRecoveryRig),
            Surface(InteriorSurfaceId::AuraBeach),
        ),
    ]
}

fn person(
    id: &str,
    name: &str,
    role: &str,
    limit: &str,
    dawn: ScheduledLocation,
    day: ScheduledLocation,
    dusk: ScheduledLocation,
    night: ScheduledLocation,
) -> ScheduledPerson {
    ScheduledPerson {
        person_id: id.into(),
        display_name: name.into(),
        role: role.into(),
        authority_limit: limit.into(),
        dawn,
        day,
        dusk,
        night,
    }
}

fn custody_record(
    lot_id: &str,
    resource: ExtractedResource,
    quantity: u32,
    origin: ExtractionSiteId,
    custodian: &str,
    destination: &str,
    status: CustodyStatus,
    provenance: &str,
) -> MaterialCustodyRecord {
    MaterialCustodyRecord {
        lot_id: lot_id.into(),
        resource,
        quantity,
        unit: match resource.form() {
            "solid" => "graded tonnes",
            _ => "sealed barrels",
        }
        .into(),
        origin,
        claimant: "Hollow Grove public resource commons; claims remain reviewable".into(),
        custodian: custodian.into(),
        destination: destination.into(),
        status,
        living_blood_excluded: true,
        provenance: vec![
            provenance.into(),
            "material lot is geological Current-bearing matter, never blood taken from a living Being"
                .into(),
            "route access and physical custody confer neither ownership nor Title".into(),
        ],
    }
}

fn site_mut(
    states: &mut BTreeMap<ExtractionSiteId, ExtractionSiteState>,
    site: ExtractionSiteId,
) -> &mut ExtractionSiteState {
    states.get_mut(&site).expect("canonical extraction state")
}

fn adjust(value: &mut u8, delta: i16) {
    *value = (i16::from(*value) + delta).clamp(0, 100) as u8;
}

fn weather_for(clock: LivingClock) -> AuraWeather {
    match (clock.day + shift_index(clock.shift)) % 8 {
        0 | 1 => AuraWeather::Clear,
        2 | 3 => AuraWeather::Crosswind,
        4 | 5 | 6 => AuraWeather::PressureDrop,
        _ => AuraWeather::Storm,
    }
}

const fn shift_index(shift: WorkShift) -> u32 {
    match shift {
        WorkShift::Dawn => 0,
        WorkShift::Day => 1,
        WorkShift::Dusk => 2,
        WorkShift::Night => 3,
    }
}

const fn choice_token(choice: LivingCaseChoice) -> &'static str {
    use LivingCaseChoice as Choice;
    match choice {
        Choice::EquitableRation => "equitable-ration",
        Choice::ProtectSeedReserve => "protect-seed-reserve",
        Choice::MaximizeImmediateYield => "maximize-immediate-yield",
        Choice::CloseAndShelter => "close-and-shelter",
        Choice::GuidedRescue => "guided-rescue",
        Choice::KeepShoreOpen => "keep-shore-open",
        Choice::TransferToCare => "transfer-to-care",
        Choice::StabilizeInPlace => "stabilize-in-place",
        Choice::SalvageTheSubject => "salvage-the-subject",
        Choice::ReinforceAndContinue => "reinforce-and-continue",
        Choice::WithdrawCrew => "withdraw-crew",
        Choice::BlastThroughFall => "blast-through-fall",
        Choice::SealAndVent => "seal-and-vent",
        Choice::EvacuateAndFlood => "evacuate-and-flood",
        Choice::ContinueCutting => "continue-cutting",
        Choice::ShutInAndRetrieve => "shut-in-and-retrieve",
        Choice::RescueCrewFirst => "rescue-crew-first",
        Choice::ContinueFlow => "continue-flow",
        Choice::CertifyReducedRate => "certify-reduced-rate",
        Choice::SuspendForRepair => "suspend-for-repair",
        Choice::BypassCertification => "bypass-certification",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivingWorldError {
    ExtractionContract(String),
    IncompleteCanonicalState,
    InvalidFunctionalLore(LivingCaseId),
    DuplicatePerson,
    InvalidCustody,
    UnknownCase(LivingCaseId),
    CaseAlreadyResolved(LivingCaseId),
    EvidenceCaseMismatch {
        case_id: LivingCaseId,
        evidence: LivingEvidence,
    },
    EvidenceAlreadyObserved(LivingEvidence),
    MissingEvidence(LivingCaseId),
    SupportRequired(LivingCaseId),
    ChoiceCaseMismatch {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    },
    ForbiddenChoice {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    },
    OutcomeIdentityMismatch,
    ClockDivergence,
    RevisionOverflow,
}

impl fmt::Display for LivingWorldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "living Hollow Grove state rejected: {self:?}")
    }
}

impl std::error::Error for LivingWorldError {}
