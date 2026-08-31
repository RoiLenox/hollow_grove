//! Four canonical double-slotted midpoint hinges for adaptive world geometry.
//!
//! The hinges are structural projections. They do not create traversal,
//! constitutional authority, route identity, or a new House boundary.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::geography::HouseBoundary;
use super::spatial_inheritance::{
    CanonicalSpatialAnchor, HollowGroveSpatialScope, anchor_position,
};
use super::spatial_topology_v2::{
    HollowGroveSpatialTopologyV2, LEGACY_CURRENT_SEANAD_ROUTE_ID, LEGACY_GLAUSBAHN_ROUTE_ID,
    SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION, TopologyV2RegionId, TopologyV2RouteId,
    canonical_hollow_grove_spatial_topology_v2, canonical_spatial_topology_v2_template,
    topology_v2_route_id,
};
use crate::composition::{
    AdaptiveHingeProjection, DoubleSlottedSpatialHinge, NORMALIZED_UNIT, NormalizedCoordinate,
    NormalizedPoint, ResolvedAdaptiveHinge, SpatialHingeId, SpatialHingeLane, SpatialHingeSlot,
    SpatialInheritanceError, SpatialRuleSetId,
};

pub const ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION: u16 = 2;
pub const ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID: &str =
    "ruleset.hollow-grove.adaptive-midpoint-hinges-v2";
const LEGACY_ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION: u16 = 1;
const LEGACY_ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID: &str =
    "ruleset.hollow-grove.adaptive-midpoint-hinges-v1";
pub const RECTANGULAR_INNER_SLOT_INSET_PER_MILLE: u16 = 45;
pub const ADAPTIVE_MORPH_CONTRACT_SCHEMA_VERSION: u16 = 1;
pub const ADAPTIVE_MORPH_INTERPOLATION_ID: &str = "linear-fixed-point-v1";
pub const ADAPTIVE_MORPH_DEVELOPER_STEP_PARTS: u64 = NORMALIZED_UNIT / 20;
pub const ADAPTIVE_MORPH_AUDIT_INTERVALS: u64 = 20;

/// The four quadrant hinges around the one central Aura Ridge spine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CanonicalMidpointHinge {
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}

impl CanonicalMidpointHinge {
    pub const ALL: [Self; 4] = [
        Self::Northeast,
        Self::Southeast,
        Self::Southwest,
        Self::Northwest,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Northeast => "hinge.midpoint.northeast",
            Self::Southeast => "hinge.midpoint.southeast",
            Self::Southwest => "hinge.midpoint.southwest",
            Self::Northwest => "hinge.midpoint.northwest",
        }
    }

    #[must_use]
    pub const fn region_title(self) -> &'static str {
        match self {
            Self::Northeast => "Aura Field",
            Self::Southeast => "Aura Beach",
            Self::Southwest => "Flynt Basin",
            Self::Northwest => "Stonebend Rise",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Northeast => "Aura Field Midway Hinge",
            Self::Southeast => "Aura Beach Midway Hinge",
            Self::Southwest => "Flynt Basin Midway Hinge",
            Self::Northwest => "Stonebend Rise Midway Hinge",
        }
    }

    #[must_use]
    pub const fn boundary(self) -> HouseBoundary {
        match self {
            Self::Northeast => HouseBoundary::StonebendSandmanor,
            Self::Southeast => HouseBoundary::GlaushouseSandmanor,
            Self::Southwest => HouseBoundary::FlyntGlaushouse,
            Self::Northwest => HouseBoundary::StonebendFlynt,
        }
    }

    /// Stable Inner then Outer route assignment.
    #[must_use]
    pub const fn routes(self) -> [TopologyV2RouteId; 2] {
        match self {
            Self::Northeast => [TopologyV2RouteId::AuraWay, TopologyV2RouteId::MountAura],
            Self::Southeast => [
                TopologyV2RouteId::Glausbaughn,
                TopologyV2RouteId::CurrentSea,
            ],
            Self::Southwest => [TopologyV2RouteId::Boardwalk, TopologyV2RouteId::Riptide],
            Self::Northwest => [
                TopologyV2RouteId::BasinMotorSpeedway,
                TopologyV2RouteId::StairwayToHeaven,
            ],
        }
    }

    /// Stable Inner then Outer rectangular slot targets in per-mille units.
    #[must_use]
    pub const fn rectangular_targets_per_mille(self) -> [(u16, u16); 2] {
        match self {
            Self::Northeast => [(690, 185), (735, 140)],
            Self::Southeast => [(690, 815), (735, 860)],
            Self::Southwest => [(310, 815), (265, 860)],
            Self::Northwest => [(310, 185), (265, 140)],
        }
    }
}

/// Lore-facing title for a route's isolated slot in a Midway hinge.
///
/// Aura Ridge forms the central spine, so it has no quadrant Midway title.
#[must_use]
pub const fn canonical_midway_name(route: TopologyV2RouteId) -> Option<&'static str> {
    match route {
        TopologyV2RouteId::AuraWay => Some("The Aura Midway"),
        TopologyV2RouteId::MountAura => Some("Mount Aura Midway"),
        TopologyV2RouteId::Glausbaughn => Some("Glausbaughn Midway"),
        TopologyV2RouteId::CurrentSea => Some("Current Sea Midway"),
        TopologyV2RouteId::Boardwalk => Some("Boardwalk Midway"),
        TopologyV2RouteId::Riptide => Some("Riptide Midway"),
        TopologyV2RouteId::BasinMotorSpeedway => Some("Basin Motor Speedway Midway"),
        TopologyV2RouteId::StairwayToHeaven => Some("Stairway to Heaven Midway"),
        TopologyV2RouteId::AuraRidge => None,
    }
}

/// Exact integer point used at the Rust-to-presentation boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveMorphPoint {
    pub x_parts: u64,
    pub y_parts: u64,
}

impl AdaptiveMorphPoint {
    #[must_use]
    pub const fn from_normalized(point: NormalizedPoint) -> Self {
        Self {
            x_parts: point.x.parts(),
            y_parts: point.y.parts(),
        }
    }
}

/// A fixed anchor that is never displaced by rectangularity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveMorphAnchorContract {
    pub anchor_id: String,
    pub position: AdaptiveMorphPoint,
}

/// One route lane's authoritative circular midpoint and rectangular target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveMorphSlotContract {
    pub lane: SpatialHingeLane,
    pub route_id: String,
    pub route_name: String,
    pub midway_name: String,
    pub circular_midpoint: AdaptiveMorphPoint,
    pub rectangular_position: AdaptiveMorphPoint,
}

/// One of the four symmetric double-slotted Midway hinges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveMorphHingeContract {
    pub hinge_id: String,
    pub display_name: String,
    pub region_title: String,
    pub circular_pivot: AdaptiveMorphPoint,
    pub rectangular_pivot: AdaptiveMorphPoint,
    pub slots: [AdaptiveMorphSlotContract; 2],
}

/// Bottom-up presentation contract derived only from the kernel projection,
/// canonical V2 topology, and canonical Midway definitions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HollowGroveAdaptiveMorphContract {
    pub schema_version: u16,
    pub topology_schema_version: u16,
    pub normalization_unit: u64,
    pub interpolation: String,
    pub default_rectangularity_parts: u64,
    pub developer_step_parts: u64,
    pub anchor_policy: String,
    pub region_policy: String,
    pub fixed_anchors: Vec<AdaptiveMorphAnchorContract>,
    pub fixed_route_ids: Vec<String>,
    pub locked_region_ids: Vec<String>,
    pub hinges: Vec<AdaptiveMorphHingeContract>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveMorphAudit {
    pub sample_count: usize,
    pub fixed_anchor_count: usize,
    pub locked_region_count: usize,
    pub hinge_count: usize,
    pub slot_count: usize,
    pub endpoints_remain_fixed: bool,
    pub slots_remain_isolated: bool,
    pub canonical_circle_is_preserved: bool,
    pub rectangular_symmetry_is_exact: bool,
    pub central_spine_remains_fixed: bool,
}

/// Immutable, serializable adaptive-hinge definition kept separate from the
/// frozen Spatial Inheritance V1 archive and corrected V2 topology archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HollowGroveAdaptiveHingeStructure {
    schema_version: u16,
    rule_set: SpatialRuleSetId,
    hinges: Vec<DoubleSlottedSpatialHinge>,
}

impl HollowGroveAdaptiveHingeStructure {
    pub fn canonical() -> Result<Self, AdaptiveMidpointHingeError> {
        let mut structure = Self {
            schema_version: ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION,
            rule_set: SpatialRuleSetId::new(ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID)
                .map_err(SpatialInheritanceError::from)?,
            hinges: CanonicalMidpointHinge::ALL
                .into_iter()
                .map(canonical_hinge_definition)
                .collect::<Result<Vec<_>, _>>()?,
        };
        structure.normalize_and_validate()?;
        Ok(structure)
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    #[must_use]
    pub fn rule_set(&self) -> &SpatialRuleSetId {
        &self.rule_set
    }

    #[must_use]
    pub fn hinges(&self) -> &[DoubleSlottedSpatialHinge] {
        &self.hinges
    }

    #[must_use]
    pub fn hinge(&self, id: CanonicalMidpointHinge) -> Option<&DoubleSlottedSpatialHinge> {
        let stable_id = id.stable_id();
        self.hinges
            .iter()
            .find(|hinge| hinge.id.as_str() == stable_id)
    }

    pub fn resolve(
        &self,
        map: &HollowGroveSpatialTopologyV2,
        scope: HollowGroveSpatialScope,
        id: CanonicalMidpointHinge,
        rectangularity: NormalizedCoordinate,
    ) -> Result<ResolvedAdaptiveHinge, AdaptiveMidpointHingeError> {
        let hinge = self
            .hinge(id)
            .ok_or(AdaptiveMidpointHingeError::MissingCanonicalHinge(id))?;
        map.resolve_hinge(scope, hinge, rectangularity)
            .map_err(|error| AdaptiveMidpointHingeError::SpatialMap(error.to_string()))
    }

    pub fn audit(&self) -> Result<AdaptiveMidpointHingeAudit, AdaptiveMidpointHingeError> {
        self.validate()?;
        let route_count = self
            .hinges
            .iter()
            .flat_map(|hinge| hinge.slots.iter().map(|slot| slot.path_id.clone()))
            .collect::<BTreeSet<_>>()
            .len();
        Ok(AdaptiveMidpointHingeAudit {
            schema_version: self.schema_version,
            hinge_count: self.hinges.len(),
            slot_count: self.hinges.len() * 2,
            route_count,
            central_spine_remains_unhinged: self.hinges.iter().all(|hinge| {
                hinge
                    .slots
                    .iter()
                    .all(|slot| slot.path_id.as_str() != TopologyV2RouteId::AuraRidge.stable_id())
            }),
            legacy_route_names_are_absent: self.hinges.iter().all(|hinge| {
                hinge.slots.iter().all(|slot| {
                    slot.path_id.as_str() != LEGACY_CURRENT_SEANAD_ROUTE_ID
                        && slot.path_id.as_str() != LEGACY_GLAUSBAHN_ROUTE_ID
                })
            }),
            digest: adaptive_midpoint_hinge_digest(self)?,
        })
    }

    pub fn validate(&self) -> Result<(), AdaptiveMidpointHingeError> {
        let mut clone = self.clone();
        clone.normalize_and_validate()
    }

    fn migrate_legacy_v1_aliases(&mut self) -> Result<(), AdaptiveMidpointHingeError> {
        if self.schema_version != LEGACY_ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION
            || self.rule_set.as_str() != LEGACY_ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID
        {
            return Ok(());
        }
        self.schema_version = ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION;
        self.rule_set = SpatialRuleSetId::new(ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID)
            .map_err(SpatialInheritanceError::from)?;
        for slot in self
            .hinges
            .iter_mut()
            .flat_map(|hinge| hinge.slots.iter_mut())
        {
            let migrated = match slot.path_id.as_str() {
                LEGACY_CURRENT_SEANAD_ROUTE_ID => Some(TopologyV2RouteId::CurrentSea),
                LEGACY_GLAUSBAHN_ROUTE_ID => Some(TopologyV2RouteId::Glausbaughn),
                _ => None,
            };
            if let Some(route) = migrated {
                slot.path_id = topology_v2_route_id(route)?;
            }
        }
        Ok(())
    }

    fn normalize_and_validate(&mut self) -> Result<(), AdaptiveMidpointHingeError> {
        if self.schema_version != ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION {
            return Err(AdaptiveMidpointHingeError::UnsupportedSchema {
                expected: ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        let expected_rule_set = SpatialRuleSetId::new(ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID)
            .map_err(SpatialInheritanceError::from)?;
        if self.rule_set != expected_rule_set {
            return Err(AdaptiveMidpointHingeError::RuleSetMismatch);
        }

        self.hinges.sort_by(|left, right| left.id.cmp(&right.id));
        if self.hinges.windows(2).any(|pair| pair[0].id == pair[1].id) {
            return Err(AdaptiveMidpointHingeError::DuplicateHingeIdentity);
        }
        if self.hinges.len() != CanonicalMidpointHinge::ALL.len() {
            return Err(AdaptiveMidpointHingeError::HingeCount {
                expected: CanonicalMidpointHinge::ALL.len(),
                actual: self.hinges.len(),
            });
        }

        let template = canonical_spatial_topology_v2_template()
            .map_err(|error| AdaptiveMidpointHingeError::SpatialMap(error.to_string()))?;
        let mut expected = CanonicalMidpointHinge::ALL
            .into_iter()
            .map(canonical_hinge_definition)
            .collect::<Result<Vec<_>, _>>()?;
        expected.sort_by(|left, right| left.id.cmp(&right.id));
        if self.hinges != expected {
            return Err(AdaptiveMidpointHingeError::CanonicalDefinitionDrift);
        }

        let mut path_ids = BTreeSet::new();
        for hinge in &self.hinges {
            let circular =
                template.project_double_slotted_hinge(hinge, NormalizedCoordinate::zero())?;
            let rectangular =
                template.project_double_slotted_hinge(hinge, NormalizedCoordinate::one())?;
            if circular.slots[0].adaptive_midpoint == circular.slots[1].adaptive_midpoint
                || rectangular.slots[0].adaptive_midpoint == rectangular.slots[1].adaptive_midpoint
            {
                return Err(AdaptiveMidpointHingeError::SlotIsolationLost(
                    hinge.id.clone(),
                ));
            }
            for slot in &hinge.slots {
                if !path_ids.insert(slot.path_id.clone()) {
                    return Err(AdaptiveMidpointHingeError::RouteAssignedTwice(
                        slot.path_id.clone(),
                    ));
                }
            }
        }
        if path_ids.len() != 8 {
            return Err(AdaptiveMidpointHingeError::RouteCount {
                expected: 8,
                actual: path_ids.len(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveMidpointHingeAudit {
    pub schema_version: u16,
    pub hinge_count: usize,
    pub slot_count: usize,
    pub route_count: usize,
    pub central_spine_remains_unhinged: bool,
    pub legacy_route_names_are_absent: bool,
    pub digest: String,
}

pub fn canonical_adaptive_midpoint_hinges()
-> Result<HollowGroveAdaptiveHingeStructure, AdaptiveMidpointHingeError> {
    HollowGroveAdaptiveHingeStructure::canonical()
}

/// Builds the one checked presentation contract from the bottom-up spatial
/// authorities. No display-layer coordinate is accepted as input.
pub fn canonical_adaptive_morph_contract()
-> Result<HollowGroveAdaptiveMorphContract, AdaptiveMidpointHingeError> {
    let structure = canonical_adaptive_midpoint_hinges()?;
    let map = canonical_hollow_grove_spatial_topology_v2()
        .map_err(|error| AdaptiveMidpointHingeError::SpatialMap(error.to_string()))?;

    let fixed_anchors = CanonicalSpatialAnchor::ALL
        .into_iter()
        .map(|anchor| {
            Ok(AdaptiveMorphAnchorContract {
                anchor_id: anchor.stable_id().to_owned(),
                position: AdaptiveMorphPoint::from_normalized(anchor_position(anchor)?),
            })
        })
        .collect::<Result<Vec<_>, AdaptiveMidpointHingeError>>()?;

    let hinges = CanonicalMidpointHinge::ALL
        .into_iter()
        .map(|id| {
            let circular = structure.resolve(
                &map,
                HollowGroveSpatialScope::World,
                id,
                NormalizedCoordinate::zero(),
            )?;
            let rectangular = structure.resolve(
                &map,
                HollowGroveSpatialScope::World,
                id,
                NormalizedCoordinate::one(),
            )?;
            let routes = id.routes();
            let slots = std::array::from_fn(|index| {
                let route = routes[index];
                AdaptiveMorphSlotContract {
                    lane: circular.local.slots[index].lane,
                    route_id: route.stable_id().to_owned(),
                    route_name: route.display_name().to_owned(),
                    midway_name: canonical_midway_name(route)
                        .expect("every hinged route has one canonical Midway name")
                        .to_owned(),
                    circular_midpoint: AdaptiveMorphPoint::from_normalized(
                        circular.local.slots[index].circular_midpoint,
                    ),
                    rectangular_position: AdaptiveMorphPoint::from_normalized(
                        rectangular.local.slots[index].rectangular_position,
                    ),
                }
            });
            Ok(AdaptiveMorphHingeContract {
                hinge_id: id.stable_id().to_owned(),
                display_name: id.display_name().to_owned(),
                region_title: id.region_title().to_owned(),
                circular_pivot: AdaptiveMorphPoint::from_normalized(circular.local.circular_pivot),
                rectangular_pivot: AdaptiveMorphPoint::from_normalized(
                    rectangular.local.rectangular_pivot,
                ),
                slots,
            })
        })
        .collect::<Result<Vec<_>, AdaptiveMidpointHingeError>>()?;

    Ok(HollowGroveAdaptiveMorphContract {
        schema_version: ADAPTIVE_MORPH_CONTRACT_SCHEMA_VERSION,
        topology_schema_version: SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION,
        normalization_unit: NORMALIZED_UNIT,
        interpolation: ADAPTIVE_MORPH_INTERPOLATION_ID.to_owned(),
        default_rectangularity_parts: 0,
        developer_step_parts: ADAPTIVE_MORPH_DEVELOPER_STEP_PARTS,
        anchor_policy: "fixed-topology-v2".to_owned(),
        region_policy: "locked-topology-v2".to_owned(),
        fixed_anchors,
        fixed_route_ids: vec![TopologyV2RouteId::AuraRidge.stable_id().to_owned()],
        locked_region_ids: TopologyV2RegionId::ALL
            .into_iter()
            .map(|region| region.stable_id().to_owned())
            .collect(),
        hinges,
    })
}

pub fn canonical_adaptive_morph_contract_json() -> Result<String, AdaptiveMidpointHingeError> {
    let mut json = serde_json::to_string_pretty(&canonical_adaptive_morph_contract()?)
        .map_err(|error| AdaptiveMidpointHingeError::Serialization(error.to_string()))?;
    json.push('\n');
    Ok(json)
}

/// Samples the whole morph at the developer step interval and proves the
/// positional invariants that the presentation and later gameplay layers may
/// rely on.
pub fn audit_adaptive_topology_morph() -> Result<AdaptiveMorphAudit, AdaptiveMidpointHingeError> {
    let structure = canonical_adaptive_midpoint_hinges()?;
    let map = canonical_hollow_grove_spatial_topology_v2()
        .map_err(|error| AdaptiveMidpointHingeError::SpatialMap(error.to_string()))?;
    let structure_audit = structure.audit()?;

    for step in 0..=ADAPTIVE_MORPH_AUDIT_INTERVALS {
        let rectangularity =
            NormalizedCoordinate::from_ratio(step, ADAPTIVE_MORPH_AUDIT_INTERVALS)?;
        let poses = CanonicalMidpointHinge::ALL
            .into_iter()
            .map(|id| structure.resolve(&map, HollowGroveSpatialScope::World, id, rectangularity))
            .collect::<Result<Vec<_>, _>>()?;

        for pose in &poses {
            if pose.local.slots[0].adaptive_midpoint == pose.local.slots[1].adaptive_midpoint {
                return Err(AdaptiveMidpointHingeError::MorphInvariant(
                    "double-slotted Midway lanes intersect",
                ));
            }
            for slot in &pose.local.slots {
                let route = TopologyV2RouteId::from_stable_or_legacy_id(slot.path_id.as_str())
                    .ok_or(AdaptiveMidpointHingeError::MorphInvariant(
                        "morph slot references a non-V2 route",
                    ))?;
                let resolved = map
                    .resolve_route(HollowGroveSpatialScope::World, route)
                    .map_err(|error| AdaptiveMidpointHingeError::SpatialMap(error.to_string()))?;
                if slot.segments[0].from != resolved.local_from
                    || slot.segments[0].to != slot.adaptive_midpoint
                    || slot.segments[1].from != slot.adaptive_midpoint
                    || slot.segments[1].to != resolved.local_to
                {
                    return Err(AdaptiveMidpointHingeError::MorphInvariant(
                        "adaptive route endpoint continuity changed",
                    ));
                }
            }
        }

        if step == 0
            && poses.iter().any(|pose| {
                pose.local
                    .slots
                    .iter()
                    .any(|slot| slot.adaptive_midpoint != slot.circular_midpoint)
            })
        {
            return Err(AdaptiveMidpointHingeError::MorphInvariant(
                "zero rectangularity changed canonical circular geometry",
            ));
        }
        if step == ADAPTIVE_MORPH_AUDIT_INTERVALS
            && (poses.iter().any(|pose| {
                pose.local
                    .slots
                    .iter()
                    .any(|slot| slot.adaptive_midpoint != slot.rectangular_position)
            }) || !projections_are_x_mirrors(&poses[0].local, &poses[3].local)
                || !projections_are_x_mirrors(&poses[1].local, &poses[2].local)
                || !projections_are_y_mirrors(&poses[0].local, &poses[1].local)
                || !projections_are_y_mirrors(&poses[3].local, &poses[2].local))
        {
            return Err(AdaptiveMidpointHingeError::MorphInvariant(
                "rectangular Midway target symmetry changed",
            ));
        }
    }

    Ok(AdaptiveMorphAudit {
        sample_count: (ADAPTIVE_MORPH_AUDIT_INTERVALS + 1) as usize,
        fixed_anchor_count: CanonicalSpatialAnchor::ALL.len(),
        locked_region_count: TopologyV2RegionId::ALL.len(),
        hinge_count: structure_audit.hinge_count,
        slot_count: structure_audit.slot_count,
        endpoints_remain_fixed: true,
        slots_remain_isolated: true,
        canonical_circle_is_preserved: true,
        rectangular_symmetry_is_exact: true,
        central_spine_remains_fixed: structure_audit.central_spine_remains_unhinged,
    })
}

fn projections_are_x_mirrors(
    right: &AdaptiveHingeProjection,
    left: &AdaptiveHingeProjection,
) -> bool {
    points_are_x_mirrors(right.adaptive_pivot, left.adaptive_pivot)
        && right.slots.iter().zip(&left.slots).all(|(right, left)| {
            points_are_x_mirrors(right.adaptive_midpoint, left.adaptive_midpoint)
        })
}

fn projections_are_y_mirrors(
    upper: &AdaptiveHingeProjection,
    lower: &AdaptiveHingeProjection,
) -> bool {
    points_are_y_mirrors(upper.adaptive_pivot, lower.adaptive_pivot)
        && upper.slots.iter().zip(&lower.slots).all(|(upper, lower)| {
            points_are_y_mirrors(upper.adaptive_midpoint, lower.adaptive_midpoint)
        })
}

fn points_are_x_mirrors(right: NormalizedPoint, left: NormalizedPoint) -> bool {
    right.x.parts() + left.x.parts() == NORMALIZED_UNIT && right.y == left.y
}

fn points_are_y_mirrors(upper: NormalizedPoint, lower: NormalizedPoint) -> bool {
    upper.x == lower.x && upper.y.parts() + lower.y.parts() == NORMALIZED_UNIT
}

pub fn encode_adaptive_midpoint_hinges(
    structure: &HollowGroveAdaptiveHingeStructure,
) -> Result<Vec<u8>, AdaptiveMidpointHingeArchiveError> {
    structure.validate()?;
    serde_json::to_vec(structure).map_err(AdaptiveMidpointHingeArchiveError::Json)
}

pub fn decode_adaptive_midpoint_hinges(
    bytes: &[u8],
) -> Result<HollowGroveAdaptiveHingeStructure, AdaptiveMidpointHingeArchiveError> {
    let mut structure: HollowGroveAdaptiveHingeStructure =
        serde_json::from_slice(bytes).map_err(AdaptiveMidpointHingeArchiveError::Json)?;
    structure.migrate_legacy_v1_aliases()?;
    structure.normalize_and_validate()?;
    Ok(structure)
}

pub fn adaptive_midpoint_hinge_digest(
    structure: &HollowGroveAdaptiveHingeStructure,
) -> Result<String, AdaptiveMidpointHingeError> {
    structure.validate()?;
    let bytes = serde_json::to_vec(structure)
        .map_err(|error| AdaptiveMidpointHingeError::Serialization(error.to_string()))?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-adaptive-midpoint-hinges-v2:{hash:016x}"))
}

pub fn build_adaptive_midpoint_hinge_witness(
    structure: &HollowGroveAdaptiveHingeStructure,
) -> Result<String, AdaptiveMidpointHingeError> {
    let audit = structure.audit()?;
    let mut output = format!(
        "# Hollow Grove Adaptive Midpoint Hinge Witness\n\n\
         - schema_version: {}\n\
         - rule_set: {}\n\
         - digest: {}\n\
         - hinge_count: {}\n\
         - slot_count: {}\n\
         - route_count: {}\n\
         - central_spine_remains_unhinged: {}\n\
         - legacy_route_names_are_absent: {}\n\n\
         ## Double-Slotted Hinges\n\n",
        audit.schema_version,
        structure.rule_set(),
        audit.digest,
        audit.hinge_count,
        audit.slot_count,
        audit.route_count,
        audit.central_spine_remains_unhinged,
        audit.legacy_route_names_are_absent,
    );
    for id in CanonicalMidpointHinge::ALL {
        let hinge = structure
            .hinge(id)
            .ok_or(AdaptiveMidpointHingeError::MissingCanonicalHinge(id))?;
        let routes = id.routes();
        let inner_midway = canonical_midway_name(routes[0])
            .ok_or(AdaptiveMidpointHingeError::MissingMidwayName(routes[0]))?;
        let outer_midway = canonical_midway_name(routes[1])
            .ok_or(AdaptiveMidpointHingeError::MissingMidwayName(routes[1]))?;
        output.push_str(&format!(
            "- {} [{}; `{}`; {:?}]: Inner={} ({}) -> ({}, {}), Outer={} ({}) -> ({}, {})\n",
            id.display_name(),
            id.region_title(),
            id.stable_id(),
            id.boundary(),
            inner_midway,
            hinge.slots[0].path_id,
            hinge.slots[0].rectangular_position.x.parts(),
            hinge.slots[0].rectangular_position.y.parts(),
            outer_midway,
            hinge.slots[1].path_id,
            hinge.slots[1].rectangular_position.x.parts(),
            hinge.slots[1].rectangular_position.y.parts(),
        ));
    }
    Ok(output)
}

fn canonical_hinge_definition(
    id: CanonicalMidpointHinge,
) -> Result<DoubleSlottedSpatialHinge, AdaptiveMidpointHingeError> {
    let routes = id.routes();
    let targets = id.rectangular_targets_per_mille();
    Ok(DoubleSlottedSpatialHinge::new(
        SpatialHingeId::new(id.stable_id()).map_err(SpatialInheritanceError::from)?,
        SpatialHingeSlot {
            lane: SpatialHingeLane::Inner,
            path_id: topology_v2_route_id(routes[0])?,
            rectangular_position: per_mille_point(targets[0].0, targets[0].1)?,
        },
        SpatialHingeSlot {
            lane: SpatialHingeLane::Outer,
            path_id: topology_v2_route_id(routes[1])?,
            rectangular_position: per_mille_point(targets[1].0, targets[1].1)?,
        },
    ))
}

fn per_mille_point(x: u16, y: u16) -> Result<NormalizedPoint, SpatialInheritanceError> {
    Ok(NormalizedPoint::new(
        NormalizedCoordinate::from_ratio(u64::from(x), 1_000)?,
        NormalizedCoordinate::from_ratio(u64::from(y), 1_000)?,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaptiveMidpointHingeError {
    Kernel(SpatialInheritanceError),
    UnsupportedSchema { expected: u16, actual: u16 },
    RuleSetMismatch,
    DuplicateHingeIdentity,
    HingeCount { expected: usize, actual: usize },
    RouteCount { expected: usize, actual: usize },
    RouteAssignedTwice(crate::composition::SpatialPathId),
    SlotIsolationLost(SpatialHingeId),
    MissingCanonicalHinge(CanonicalMidpointHinge),
    MissingMidwayName(TopologyV2RouteId),
    CanonicalDefinitionDrift,
    MorphInvariant(&'static str),
    SpatialMap(String),
    Serialization(String),
}

impl From<SpatialInheritanceError> for AdaptiveMidpointHingeError {
    fn from(value: SpatialInheritanceError) -> Self {
        Self::Kernel(value)
    }
}

impl fmt::Display for AdaptiveMidpointHingeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kernel(error) => error.fmt(formatter),
            Self::UnsupportedSchema { expected, actual } => write!(
                formatter,
                "unsupported adaptive hinge schema {actual}; expected {expected}"
            ),
            Self::RuleSetMismatch => formatter.write_str("adaptive hinge rule-set mismatch"),
            Self::DuplicateHingeIdentity => {
                formatter.write_str("adaptive hinge identity is duplicated")
            }
            Self::HingeCount { expected, actual } => write!(
                formatter,
                "adaptive structure has {actual} hinges; expected {expected}"
            ),
            Self::RouteCount { expected, actual } => write!(
                formatter,
                "adaptive structure covers {actual} routes; expected {expected}"
            ),
            Self::RouteAssignedTwice(path) => {
                write!(formatter, "adaptive route {path} is assigned to two hinges")
            }
            Self::SlotIsolationLost(hinge) => {
                write!(formatter, "adaptive hinge {hinge} lost slot isolation")
            }
            Self::MissingCanonicalHinge(hinge) => {
                write!(formatter, "canonical adaptive hinge {hinge:?} is missing")
            }
            Self::MissingMidwayName(route) => {
                write!(
                    formatter,
                    "canonical adaptive route {route:?} has no Midway name"
                )
            }
            Self::CanonicalDefinitionDrift => {
                formatter.write_str("adaptive hinge definitions differ from canonical structure")
            }
            Self::MorphInvariant(message) => {
                write!(
                    formatter,
                    "adaptive topology morph invariant failed: {message}"
                )
            }
            Self::SpatialMap(error) => write!(formatter, "adaptive spatial map failed: {error}"),
            Self::Serialization(error) => {
                write!(formatter, "adaptive hinge serialization failed: {error}")
            }
        }
    }
}

impl std::error::Error for AdaptiveMidpointHingeError {}

#[derive(Debug)]
pub enum AdaptiveMidpointHingeArchiveError {
    Json(serde_json::Error),
    Contract(AdaptiveMidpointHingeError),
}

impl From<AdaptiveMidpointHingeError> for AdaptiveMidpointHingeArchiveError {
    fn from(value: AdaptiveMidpointHingeError) -> Self {
        Self::Contract(value)
    }
}

impl fmt::Display for AdaptiveMidpointHingeArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "adaptive hinge JSON error: {error}"),
            Self::Contract(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for AdaptiveMidpointHingeArchiveError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hollow_grove_contract::House;

    fn parts_point(x: u64, y: u64) -> NormalizedPoint {
        NormalizedPoint::new(
            NormalizedCoordinate::new(x).unwrap(),
            NormalizedCoordinate::new(y).unwrap(),
        )
    }

    #[test]
    fn canonical_structure_has_four_hinges_eight_isolated_slots_and_no_spine_route() {
        let structure = canonical_adaptive_midpoint_hinges().unwrap();
        let audit = structure.audit().unwrap();
        assert_eq!(
            (audit.hinge_count, audit.slot_count, audit.route_count),
            (4, 8, 8)
        );
        assert!(audit.central_spine_remains_unhinged);
        assert!(audit.legacy_route_names_are_absent);
        assert_eq!(
            CanonicalMidpointHinge::Southeast.routes(),
            [
                TopologyV2RouteId::Glausbaughn,
                TopologyV2RouteId::CurrentSea,
            ]
        );
    }

    #[test]
    fn circular_pose_reproduces_route_midpoints_and_rectangle_pose_is_double_slotted() {
        let structure = canonical_adaptive_midpoint_hinges().unwrap();
        let map = super::super::spatial_topology_v2::canonical_hollow_grove_spatial_topology_v2()
            .unwrap();
        let northeast_circle = structure
            .resolve(
                &map,
                HollowGroveSpatialScope::World,
                CanonicalMidpointHinge::Northeast,
                NormalizedCoordinate::zero(),
            )
            .unwrap();
        assert_eq!(
            northeast_circle.local.slots[0].adaptive_midpoint,
            parts_point(617_500_000, 320_000_000)
        );
        assert_eq!(
            northeast_circle.local.slots[1].adaptive_midpoint,
            parts_point(653_750_000, 260_000_000)
        );

        let northeast_rectangle = structure
            .resolve(
                &map,
                HollowGroveSpatialScope::World,
                CanonicalMidpointHinge::Northeast,
                NormalizedCoordinate::one(),
            )
            .unwrap();
        assert_eq!(
            northeast_rectangle.local.slots[0].adaptive_midpoint,
            per_mille_point(690, 185).unwrap()
        );
        assert_eq!(
            northeast_rectangle.local.slots[1].adaptive_midpoint,
            per_mille_point(735, 140).unwrap()
        );
        assert_ne!(
            northeast_rectangle.local.slots[0].adaptive_midpoint,
            northeast_rectangle.local.slots[1].adaptive_midpoint
        );
    }

    #[test]
    fn halfway_pose_inherits_through_house_frames_without_local_drift() {
        let structure = canonical_adaptive_midpoint_hinges().unwrap();
        let map = super::super::spatial_topology_v2::canonical_hollow_grove_spatial_topology_v2()
            .unwrap();
        let rectangularity = NormalizedCoordinate::midpoint();
        let world = structure
            .resolve(
                &map,
                HollowGroveSpatialScope::World,
                CanonicalMidpointHinge::Southwest,
                rectangularity,
            )
            .unwrap();
        let flynt = structure
            .resolve(
                &map,
                HollowGroveSpatialScope::House(House::Flynt),
                CanonicalMidpointHinge::Southwest,
                rectangularity,
            )
            .unwrap();
        assert_eq!(world.local, flynt.local);
        assert_ne!(world.root, flynt.root);
        assert_eq!((world.depth, flynt.depth), (0, 1));
    }

    #[test]
    fn bottom_up_morph_contract_locks_every_anchor_region_spine_and_midway() {
        let contract = canonical_adaptive_morph_contract().unwrap();
        assert_eq!(contract.schema_version, 1);
        assert_eq!(contract.topology_schema_version, 2);
        assert_eq!(contract.normalization_unit, 1_000_000_000);
        assert_eq!(contract.interpolation, "linear-fixed-point-v1");
        assert_eq!(contract.fixed_anchors.len(), 6);
        assert_eq!(contract.locked_region_ids.len(), 5);
        assert_eq!(contract.hinges.len(), 4);
        assert_eq!(contract.fixed_route_ids, ["geography.route.aura-ridge"]);

        let northeast = &contract.hinges[0];
        assert_eq!(northeast.hinge_id, "hinge.midpoint.northeast");
        assert_eq!(northeast.circular_pivot.x_parts, 635_625_000);
        assert_eq!(northeast.circular_pivot.y_parts, 290_000_000);
        assert_eq!(northeast.rectangular_pivot.x_parts, 712_500_000);
        assert_eq!(northeast.rectangular_pivot.y_parts, 162_500_000);
        assert_eq!(northeast.slots[0].midway_name, "The Aura Midway");
        assert_eq!(northeast.slots[1].midway_name, "Mount Aura Midway");
    }

    #[test]
    fn sampled_morph_keeps_bottom_up_positional_invariants() {
        let audit = audit_adaptive_topology_morph().unwrap();
        assert_eq!(audit.sample_count, 21);
        assert_eq!(audit.fixed_anchor_count, 6);
        assert_eq!(audit.locked_region_count, 5);
        assert_eq!((audit.hinge_count, audit.slot_count), (4, 8));
        assert!(audit.endpoints_remain_fixed);
        assert!(audit.slots_remain_isolated);
        assert!(audit.canonical_circle_is_preserved);
        assert!(audit.rectangular_symmetry_is_exact);
        assert!(audit.central_spine_remains_fixed);
    }

    #[test]
    fn standalone_archive_and_digest_are_deterministic_and_fail_closed() {
        let structure = canonical_adaptive_midpoint_hinges().unwrap();
        let bytes = encode_adaptive_midpoint_hinges(&structure).unwrap();
        let replayed = decode_adaptive_midpoint_hinges(&bytes).unwrap();
        assert_eq!(replayed, structure);
        assert_eq!(
            adaptive_midpoint_hinge_digest(&replayed).unwrap(),
            adaptive_midpoint_hinge_digest(&structure).unwrap()
        );

        let mut json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        json["schema_version"] = serde_json::json!(ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION + 1);
        assert!(decode_adaptive_midpoint_hinges(&serde_json::to_vec(&json).unwrap()).is_err());
    }

    #[test]
    fn legacy_v1_hinge_aliases_migrate_to_current_sea_and_glausbaughn() {
        let mut legacy = canonical_adaptive_midpoint_hinges().unwrap();
        legacy.schema_version = LEGACY_ADAPTIVE_MIDPOINT_HINGE_SCHEMA_VERSION;
        legacy.rule_set =
            SpatialRuleSetId::new(LEGACY_ADAPTIVE_MIDPOINT_HINGE_RULE_SET_ID).unwrap();
        let southeast = legacy
            .hinges
            .iter_mut()
            .find(|hinge| hinge.id.as_str() == CanonicalMidpointHinge::Southeast.stable_id())
            .unwrap();
        southeast.slots[0].path_id =
            crate::composition::SpatialPathId::new(LEGACY_GLAUSBAHN_ROUTE_ID).unwrap();
        southeast.slots[1].path_id =
            crate::composition::SpatialPathId::new(LEGACY_CURRENT_SEANAD_ROUTE_ID).unwrap();

        let bytes = serde_json::to_vec(&legacy).unwrap();
        let migrated = decode_adaptive_midpoint_hinges(&bytes).unwrap();
        let routes = migrated
            .hinge(CanonicalMidpointHinge::Southeast)
            .unwrap()
            .slots
            .each_ref()
            .map(|slot| slot.path_id.as_str());
        assert_eq!(
            routes,
            [
                TopologyV2RouteId::Glausbaughn.stable_id(),
                TopologyV2RouteId::CurrentSea.stable_id(),
            ]
        );
    }

    #[test]
    fn witness_names_every_hinge_lane_and_preserves_non_authority_boundary() {
        let structure = canonical_adaptive_midpoint_hinges().unwrap();
        let witness = build_adaptive_midpoint_hinge_witness(&structure).unwrap();
        for required in [
            "# Hollow Grove Adaptive Midpoint Hinge Witness",
            "Aura Field Midway Hinge",
            "Aura Beach Midway Hinge",
            "Flynt Basin Midway Hinge",
            "Stonebend Rise Midway Hinge",
            "The Aura Midway",
            "Mount Aura Midway",
            "Glausbaughn Midway",
            "Current Sea Midway",
            "Boardwalk Midway",
            "Riptide Midway",
            "Basin Motor Speedway Midway",
            "Stairway to Heaven Midway",
            "geography.route.aura-way",
            "geography.route.current-sea",
            "geography.route.boardwalk",
            "geography.route.stairway-to-heaven",
        ] {
            assert!(witness.contains(required), "witness missing {required}");
        }
    }

    #[test]
    fn midway_titles_cover_only_the_eight_quadrant_routes() {
        let titled = TopologyV2RouteId::ALL
            .into_iter()
            .filter(|route| canonical_midway_name(*route).is_some())
            .collect::<Vec<_>>();
        assert_eq!(titled.len(), 8);
        assert_eq!(
            canonical_midway_name(TopologyV2RouteId::AuraWay),
            Some("The Aura Midway")
        );
        assert_eq!(
            canonical_midway_name(TopologyV2RouteId::MountAura),
            Some("Mount Aura Midway")
        );
        assert_eq!(canonical_midway_name(TopologyV2RouteId::AuraRidge), None);
    }
}
