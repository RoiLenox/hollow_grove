//! Corrected Hollow Grove spatial topology layered above frozen V1 replay.
//!
//! V1 remains byte-compatible. V2 removes the accidental Current Seanad
//! duplicate, makes Current Sea the curved Sandmanor–Glaüshouse route, makes
//! Glausbaughn its straight companion, and partitions Aura Farm into equal
//! Aura Field and Content Farm triangles above the complete Aura Beach.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{
    NormalizedCoordinate, NormalizedPoint, ResolvedAdaptiveHinge, ResolvedSpatialPath,
    ResolvedSpatialRegion, ScaleKey, SpatialAnchor, SpatialCatalogSnapshot, SpatialFrame,
    SpatialFrameId, SpatialInheritanceCatalog, SpatialInheritanceError, SpatialPath,
    SpatialPathGeometry, SpatialPathId, SpatialRegion, SpatialRegionId, SpatialRuleSetId,
    SpatialTemplate, SpatialTemplateId,
};
use crate::hollow_grove_contract::House;

use super::spatial_inheritance::{
    CanonicalSpatialAnchor, HollowGroveSpatialScope, anchor_id, anchor_position, frame_id,
    house_frame_bounds,
};

pub const SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION: u16 = 2;
pub const SPATIAL_TOPOLOGY_V2_TEMPLATE_ID: &str = "template.hollow-grove.main-map-v2";
pub const SPATIAL_TOPOLOGY_V2_RULE_SET_ID: &str = "ruleset.hollow-grove.spatial-topology-v2";
pub const SPATIAL_TOPOLOGY_V2_DIGEST_PREFIX: &str = "fnv1a64-hollow-grove-spatial-topology-v2";
pub const LEGACY_CURRENT_SEANAD_ROUTE_ID: &str = "geography.route.current-seanad";
pub const LEGACY_GLAUSBAHN_ROUTE_ID: &str = "geography.route.glausbahn";
pub const ROUTE_BAND_WIDTH_PER_MILLE: u16 = 45;

/// The corrected nine-route roster. Aura Ridge is the only central spine;
/// every other route occupies one isolated Midway slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TopologyV2RouteId {
    Boardwalk,
    Riptide,
    AuraRidge,
    Glausbaughn,
    CurrentSea,
    AuraWay,
    MountAura,
    BasinMotorSpeedway,
    StairwayToHeaven,
}

impl TopologyV2RouteId {
    pub const ALL: [Self; 9] = [
        Self::Boardwalk,
        Self::Riptide,
        Self::AuraRidge,
        Self::Glausbaughn,
        Self::CurrentSea,
        Self::AuraWay,
        Self::MountAura,
        Self::BasinMotorSpeedway,
        Self::StairwayToHeaven,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Boardwalk => "geography.route.boardwalk",
            Self::Riptide => "geography.route.riptide",
            Self::AuraRidge => "geography.route.aura-ridge",
            Self::Glausbaughn => "geography.route.glausbaughn",
            Self::CurrentSea => "geography.route.current-sea",
            Self::AuraWay => "geography.route.aura-way",
            Self::MountAura => "geography.route.mnt-aura",
            Self::BasinMotorSpeedway => "geography.route.basin-motorspeedway",
            Self::StairwayToHeaven => "geography.route.stairway-to-heaven",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Boardwalk => "Boardwalk",
            Self::Riptide => "Riptide",
            Self::AuraRidge => "Aura Ridge",
            Self::Glausbaughn => "Glausbaughn",
            Self::CurrentSea => "Current Sea",
            Self::AuraWay => "Aura Way",
            Self::MountAura => "Mount Aura",
            Self::BasinMotorSpeedway => "Basin Motor Speedway",
            Self::StairwayToHeaven => "Stairway to Heaven",
        }
    }

    #[must_use]
    pub const fn geometry(self) -> SpatialPathGeometry {
        match self {
            Self::Boardwalk
            | Self::AuraRidge
            | Self::Glausbaughn
            | Self::AuraWay
            | Self::BasinMotorSpeedway => SpatialPathGeometry::Straight,
            Self::Riptide | Self::CurrentSea | Self::MountAura | Self::StairwayToHeaven => {
                SpatialPathGeometry::Round
            }
        }
    }

    #[must_use]
    pub const fn endpoints(self) -> [House; 2] {
        match self {
            Self::Boardwalk | Self::Riptide => [House::Flynt, House::Glaushouse],
            Self::AuraRidge => [House::Glaushouse, House::Stonebend],
            Self::Glausbaughn | Self::CurrentSea => [House::Sandmanor, House::Glaushouse],
            Self::AuraWay | Self::MountAura => [House::Stonebend, House::Sandmanor],
            Self::BasinMotorSpeedway => [House::Stonebend, House::Flynt],
            Self::StairwayToHeaven => [House::Flynt, House::Stonebend],
        }
    }

    /// Accepts current V2 identities plus the two explicit V1 compatibility
    /// aliases. Alias parsing never changes the canonical V2 output identity.
    #[must_use]
    pub fn from_stable_or_legacy_id(value: &str) -> Option<Self> {
        match value {
            "geography.route.boardwalk" => Some(Self::Boardwalk),
            "geography.route.riptide" => Some(Self::Riptide),
            "geography.route.aura-ridge" => Some(Self::AuraRidge),
            "geography.route.glausbaughn" | LEGACY_GLAUSBAHN_ROUTE_ID => Some(Self::Glausbaughn),
            "geography.route.current-sea" | LEGACY_CURRENT_SEANAD_ROUTE_ID => {
                Some(Self::CurrentSea)
            }
            "geography.route.aura-way" => Some(Self::AuraWay),
            "geography.route.mnt-aura" => Some(Self::MountAura),
            "geography.route.basin-motorspeedway" => Some(Self::BasinMotorSpeedway),
            "geography.route.stairway-to-heaven" => Some(Self::StairwayToHeaven),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TopologyV2RegionId {
    AuraFarm,
    AuraField,
    ContentFarm,
    AuraBeach,
    AuraBasin,
}

impl TopologyV2RegionId {
    pub const ALL: [Self; 5] = [
        Self::AuraFarm,
        Self::AuraField,
        Self::ContentFarm,
        Self::AuraBeach,
        Self::AuraBasin,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::AuraFarm => "region.aura-farm",
            Self::AuraField => "region.aura-farm.aura-field",
            Self::ContentFarm => "region.aura-farm.content-farm",
            Self::AuraBeach => "region.aura-beach",
            Self::AuraBasin => "region.aura-basin",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::AuraFarm => "Aura Farm",
            Self::AuraField => "Aura Field",
            Self::ContentFarm => "Content Farm",
            Self::AuraBeach => "Aura Beach",
            Self::AuraBasin => "Aura Basin",
        }
    }

    /// Ordered per-mille vertices in the corrected screen topology.
    #[must_use]
    pub const fn polygon_per_mille(self) -> [(u16, u16); 3] {
        match self {
            Self::AuraFarm => [(500, 140), (735, 500), (500, 500)],
            Self::AuraField => [(500, 140), (735, 500), (500, 320)],
            Self::ContentFarm => [(500, 320), (735, 500), (500, 500)],
            Self::AuraBeach => [(500, 500), (735, 500), (500, 860)],
            Self::AuraBasin => [(500, 140), (265, 500), (500, 860)],
        }
    }
}

/// Corrected topology plus inherited world/House projections.
#[derive(Debug)]
pub struct HollowGroveSpatialTopologyV2 {
    catalog: SpatialInheritanceCatalog,
    template_id: SpatialTemplateId,
}

impl HollowGroveSpatialTopologyV2 {
    #[must_use]
    pub fn catalog(&self) -> &SpatialInheritanceCatalog {
        &self.catalog
    }

    #[must_use]
    pub fn template_id(&self) -> &SpatialTemplateId {
        &self.template_id
    }

    #[must_use]
    pub fn template(&self) -> &SpatialTemplate {
        self.catalog
            .template(&self.template_id)
            .expect("validated V2 topology retains its template")
    }

    pub fn resolve_route(
        &self,
        scope: HollowGroveSpatialScope,
        route: TopologyV2RouteId,
    ) -> Result<ResolvedSpatialPath, SpatialTopologyV2Error> {
        Ok(self
            .catalog
            .resolve_path(&frame_id(scope)?, &topology_v2_route_id(route)?)?)
    }

    pub fn resolve_region(
        &self,
        scope: HollowGroveSpatialScope,
        region: TopologyV2RegionId,
    ) -> Result<ResolvedSpatialRegion, SpatialTopologyV2Error> {
        Ok(self
            .catalog
            .resolve_region(&frame_id(scope)?, &topology_v2_region_id(region)?)?)
    }

    pub fn resolve_hinge(
        &self,
        scope: HollowGroveSpatialScope,
        hinge: &crate::composition::DoubleSlottedSpatialHinge,
        rectangularity: NormalizedCoordinate,
    ) -> Result<ResolvedAdaptiveHinge, SpatialTopologyV2Error> {
        Ok(self
            .catalog
            .resolve_double_slotted_hinge(&frame_id(scope)?, hinge, rectangularity)?)
    }

    pub fn validate(&self) -> Result<SpatialTopologyV2Audit, SpatialTopologyV2Error> {
        self.catalog.validate_integrity()?;
        let expected_rule_set = SpatialRuleSetId::new(SPATIAL_TOPOLOGY_V2_RULE_SET_ID)?;
        if self.catalog.rule_set() != &expected_rule_set {
            return Err(SpatialTopologyV2Error::RuleSetMismatch);
        }
        if self.catalog.template_count() != 1 || self.catalog.frame_count() != 5 {
            return Err(SpatialTopologyV2Error::CatalogShapeMismatch);
        }
        let expected_template = canonical_spatial_topology_v2_template()?;
        if self.template() != &expected_template {
            return Err(SpatialTopologyV2Error::TemplateDrift);
        }
        for expected in canonical_v2_frames(&self.template_id)? {
            if self.catalog.frame(expected.id()) != Some(&expected) {
                return Err(SpatialTopologyV2Error::FrameDrift(expected.id().clone()));
            }
        }
        validate_partition_law()?;

        Ok(SpatialTopologyV2Audit {
            schema_version: SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION,
            route_count: TopologyV2RouteId::ALL.len(),
            straight_route_count: TopologyV2RouteId::ALL
                .into_iter()
                .filter(|route| route.geometry() == SpatialPathGeometry::Straight)
                .count(),
            curved_route_count: TopologyV2RouteId::ALL
                .into_iter()
                .filter(|route| route.geometry() == SpatialPathGeometry::Round)
                .count(),
            region_count: TopologyV2RegionId::ALL.len(),
            aura_farm_partition_is_exact: true,
            content_farm_borders_aura_beach: true,
            current_seanad_is_legacy_only: self
                .template()
                .path(&SpatialPathId::new(LEGACY_CURRENT_SEANAD_ROUTE_ID)?)
                .is_none(),
            glausbahn_is_legacy_only: self
                .template()
                .path(&SpatialPathId::new(LEGACY_GLAUSBAHN_ROUTE_ID)?)
                .is_none(),
            catalog_digest: self.catalog.canonical_digest()?,
            template_digest: self.catalog.template_digest(&self.template_id)?,
        })
    }

    pub fn archive(&self) -> Result<SpatialTopologyV2Archive, SpatialTopologyV2Error> {
        self.validate()?;
        Ok(SpatialTopologyV2Archive {
            schema_version: SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION,
            catalog: self.catalog.snapshot(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialTopologyV2Audit {
    pub schema_version: u16,
    pub route_count: usize,
    pub straight_route_count: usize,
    pub curved_route_count: usize,
    pub region_count: usize,
    pub aura_farm_partition_is_exact: bool,
    pub content_farm_borders_aura_beach: bool,
    pub current_seanad_is_legacy_only: bool,
    pub glausbahn_is_legacy_only: bool,
    pub catalog_digest: String,
    pub template_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpatialTopologyV2Archive {
    pub schema_version: u16,
    pub catalog: SpatialCatalogSnapshot,
}

pub fn canonical_hollow_grove_spatial_topology_v2()
-> Result<HollowGroveSpatialTopologyV2, SpatialTopologyV2Error> {
    let template = canonical_spatial_topology_v2_template()?;
    let template_id = template.id().clone();
    let mut catalog = SpatialInheritanceCatalog::with_rule_set(SpatialRuleSetId::new(
        SPATIAL_TOPOLOGY_V2_RULE_SET_ID,
    )?);
    catalog.insert_template(template)?;
    for frame in canonical_v2_frames(&template_id)? {
        catalog.insert_frame(frame)?;
    }
    let topology = HollowGroveSpatialTopologyV2 {
        catalog,
        template_id,
    };
    topology.validate()?;
    Ok(topology)
}

pub fn canonical_spatial_topology_v2_template() -> Result<SpatialTemplate, SpatialTopologyV2Error> {
    let anchors = CanonicalSpatialAnchor::ALL
        .into_iter()
        .map(|anchor| {
            Ok(SpatialAnchor {
                id: anchor_id(anchor)?,
                position: anchor_position(anchor)?,
            })
        })
        .collect::<Result<Vec<_>, SpatialInheritanceError>>()?;

    let paths = TopologyV2RouteId::ALL
        .into_iter()
        .map(|route| {
            let [from, to] = route.endpoints();
            Ok(SpatialPath {
                id: topology_v2_route_id(route)?,
                from: anchor_id(CanonicalSpatialAnchor::for_house(from))?,
                to: anchor_id(CanonicalSpatialAnchor::for_house(to))?,
                geometry: route.geometry(),
                control_points: route_control_points(route)?,
                band_width: NormalizedCoordinate::from_ratio(
                    u64::from(ROUTE_BAND_WIDTH_PER_MILLE),
                    1_000,
                )?,
            })
        })
        .collect::<Result<Vec<_>, SpatialInheritanceError>>()?;

    let regions = TopologyV2RegionId::ALL
        .into_iter()
        .map(|region| {
            Ok(SpatialRegion {
                id: topology_v2_region_id(region)?,
                polygon: region
                    .polygon_per_mille()
                    .into_iter()
                    .map(|(x, y)| normalized_per_mille(x, y))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        })
        .collect::<Result<Vec<_>, SpatialInheritanceError>>()?;

    Ok(SpatialTemplate::new(
        SpatialTemplateId::new(SPATIAL_TOPOLOGY_V2_TEMPLATE_ID)?,
        anchors,
        paths,
        regions,
    )?)
}

pub fn topology_v2_route_id(
    route: TopologyV2RouteId,
) -> Result<SpatialPathId, SpatialInheritanceError> {
    Ok(SpatialPathId::new(route.stable_id())?)
}

pub fn topology_v2_region_id(
    region: TopologyV2RegionId,
) -> Result<SpatialRegionId, SpatialInheritanceError> {
    Ok(SpatialRegionId::new(region.stable_id())?)
}

pub fn encode_spatial_topology_v2(
    topology: &HollowGroveSpatialTopologyV2,
) -> Result<Vec<u8>, SpatialTopologyV2ArchiveError> {
    Ok(serde_json::to_vec(&topology.archive()?)?)
}

pub fn decode_spatial_topology_v2(
    bytes: &[u8],
) -> Result<HollowGroveSpatialTopologyV2, SpatialTopologyV2ArchiveError> {
    let archive: SpatialTopologyV2Archive = serde_json::from_slice(bytes)?;
    if archive.schema_version != SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION {
        return Err(SpatialTopologyV2Error::UnsupportedSchema {
            expected: SPATIAL_TOPOLOGY_V2_SCHEMA_VERSION,
            actual: archive.schema_version,
        }
        .into());
    }
    let catalog = SpatialInheritanceCatalog::replay_snapshot(archive.catalog)?;
    let topology = HollowGroveSpatialTopologyV2 {
        catalog,
        template_id: SpatialTemplateId::new(SPATIAL_TOPOLOGY_V2_TEMPLATE_ID)?,
    };
    topology.validate()?;
    Ok(topology)
}

/// Validates a frozen V1 archive, then emits the one corrected canonical V2
/// topology. The old Current Seanad curve becomes Current Sea; the obsolete
/// straight Current Sea projection and curved Glausbahn do not survive as
/// canonical V2 paths.
pub fn migrate_spatial_v1_archive_to_topology_v2(
    bytes: &[u8],
) -> Result<Vec<u8>, SpatialTopologyV2ArchiveError> {
    super::spatial_inheritance::decode_hollow_grove_spatial_archive(bytes)
        .map_err(|error| SpatialTopologyV2Error::LegacyV1(error.to_string()))?;
    encode_spatial_topology_v2(&canonical_hollow_grove_spatial_topology_v2()?)
}

pub fn spatial_topology_v2_digest(
    topology: &HollowGroveSpatialTopologyV2,
) -> Result<String, SpatialTopologyV2ArchiveError> {
    let bytes = encode_spatial_topology_v2(topology)?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("{SPATIAL_TOPOLOGY_V2_DIGEST_PREFIX}:{hash:016x}"))
}

pub fn build_spatial_topology_v2_witness(
    topology: &HollowGroveSpatialTopologyV2,
) -> Result<String, SpatialTopologyV2ArchiveError> {
    let audit = topology.validate()?;
    let mut output = format!(
        "# Hollow Grove Spatial Topology V2 Witness\n\n\
         - schema_version: {}\n\
         - route_count: {}\n\
         - straight_route_count: {}\n\
         - curved_route_count: {}\n\
         - region_count: {}\n\
         - aura_farm_partition_is_exact: {}\n\
         - content_farm_borders_aura_beach: {}\n\
         - current_seanad_is_legacy_only: {}\n\
         - glausbahn_is_legacy_only: {}\n\
         - catalog_digest: {}\n\
         - template_digest: {}\n\
         - replay_digest: {}\n\n\
         ## Corrected Routes\n\n",
        audit.schema_version,
        audit.route_count,
        audit.straight_route_count,
        audit.curved_route_count,
        audit.region_count,
        audit.aura_farm_partition_is_exact,
        audit.content_farm_borders_aura_beach,
        audit.current_seanad_is_legacy_only,
        audit.glausbahn_is_legacy_only,
        audit.catalog_digest,
        audit.template_digest,
        spatial_topology_v2_digest(topology)?,
    );
    for route in TopologyV2RouteId::ALL {
        let [from, to] = route.endpoints();
        output.push_str(&format!(
            "- {} (`{}`): {:?} / {:?} -> {:?}\n",
            route.display_name(),
            route.stable_id(),
            route.geometry(),
            from,
            to,
        ));
    }
    output.push_str("\n## Corrected Regions\n\n");
    for region in TopologyV2RegionId::ALL {
        let points = region
            .polygon_per_mille()
            .map(|(x, y)| format!("({x}, {y})"))
            .join(" -> ");
        output.push_str(&format!(
            "- {} (`{}`): {}\n",
            region.display_name(),
            region.stable_id(),
            points,
        ));
    }
    output.push_str(&format!(
        "\n## Legacy Decode Aliases\n\n\
         - `{LEGACY_CURRENT_SEANAD_ROUTE_ID}` -> `{}`\n\
         - `{LEGACY_GLAUSBAHN_ROUTE_ID}` -> `{}`\n",
        TopologyV2RouteId::CurrentSea.stable_id(),
        TopologyV2RouteId::Glausbaughn.stable_id(),
    ));
    Ok(output)
}

fn canonical_v2_frames(
    template_id: &SpatialTemplateId,
) -> Result<Vec<SpatialFrame>, SpatialInheritanceError> {
    let mut frames = vec![SpatialFrame::root(
        frame_id(HollowGroveSpatialScope::World)?,
        template_id.clone(),
        ScaleKey::new("scale.world")?,
    )];
    for house in [
        House::Stonebend,
        House::Sandmanor,
        House::Glaushouse,
        House::Flynt,
    ] {
        frames.push(SpatialFrame::child(
            frame_id(HollowGroveSpatialScope::House(house))?,
            frame_id(HollowGroveSpatialScope::World)?,
            template_id.clone(),
            ScaleKey::new("scale.city")?,
            house_frame_bounds(house)?,
        ));
    }
    Ok(frames)
}

fn route_control_points(
    route: TopologyV2RouteId,
) -> Result<Vec<NormalizedPoint>, SpatialInheritanceError> {
    let control = match route {
        TopologyV2RouteId::Riptide => Some((310, 800)),
        TopologyV2RouteId::CurrentSea => Some((730, 800)),
        TopologyV2RouteId::MountAura => Some((690, 200)),
        TopologyV2RouteId::StairwayToHeaven => Some((310, 200)),
        TopologyV2RouteId::Boardwalk
        | TopologyV2RouteId::AuraRidge
        | TopologyV2RouteId::Glausbaughn
        | TopologyV2RouteId::AuraWay
        | TopologyV2RouteId::BasinMotorSpeedway => None,
    };
    control
        .map(|(x, y)| normalized_per_mille(x, y).map(|point| vec![point]))
        .unwrap_or_else(|| Ok(vec![]))
}

fn normalized_per_mille(x: u16, y: u16) -> Result<NormalizedPoint, SpatialInheritanceError> {
    Ok(NormalizedPoint::new(
        NormalizedCoordinate::from_ratio(u64::from(x), 1_000)?,
        NormalizedCoordinate::from_ratio(u64::from(y), 1_000)?,
    ))
}

fn polygon_twice_area(points: [(u16, u16); 3]) -> u64 {
    points
        .into_iter()
        .zip(points.into_iter().cycle().skip(1))
        .take(3)
        .map(|((left_x, left_y), (right_x, right_y))| {
            i64::from(left_x) * i64::from(right_y) - i64::from(right_x) * i64::from(left_y)
        })
        .sum::<i64>()
        .unsigned_abs()
}

fn validate_partition_law() -> Result<(), SpatialTopologyV2Error> {
    let farm = TopologyV2RegionId::AuraFarm.polygon_per_mille();
    let field = TopologyV2RegionId::AuraField.polygon_per_mille();
    let content = TopologyV2RegionId::ContentFarm.polygon_per_mille();
    let beach = TopologyV2RegionId::AuraBeach.polygon_per_mille();
    if polygon_twice_area(field) != polygon_twice_area(content)
        || polygon_twice_area(field) + polygon_twice_area(content) != polygon_twice_area(farm)
        || content[1] != beach[1]
        || content[2] != beach[0]
    {
        return Err(SpatialTopologyV2Error::PartitionLawMismatch);
    }
    Ok(())
}

#[derive(Debug)]
pub enum SpatialTopologyV2Error {
    Kernel(SpatialInheritanceError),
    UnsupportedSchema { expected: u16, actual: u16 },
    RuleSetMismatch,
    CatalogShapeMismatch,
    TemplateDrift,
    FrameDrift(SpatialFrameId),
    PartitionLawMismatch,
    LegacyV1(String),
}

impl From<SpatialInheritanceError> for SpatialTopologyV2Error {
    fn from(value: SpatialInheritanceError) -> Self {
        Self::Kernel(value)
    }
}

impl From<crate::composition::StableKeyError> for SpatialTopologyV2Error {
    fn from(value: crate::composition::StableKeyError) -> Self {
        Self::Kernel(SpatialInheritanceError::StableKey(value))
    }
}

impl fmt::Display for SpatialTopologyV2Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Hollow Grove Spatial Topology V2 rejected: {self:?}"
        )
    }
}

impl std::error::Error for SpatialTopologyV2Error {}

#[derive(Debug)]
pub enum SpatialTopologyV2ArchiveError {
    Json(serde_json::Error),
    Contract(SpatialTopologyV2Error),
}

impl From<serde_json::Error> for SpatialTopologyV2ArchiveError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<SpatialTopologyV2Error> for SpatialTopologyV2ArchiveError {
    fn from(value: SpatialTopologyV2Error) -> Self {
        Self::Contract(value)
    }
}

impl From<SpatialInheritanceError> for SpatialTopologyV2ArchiveError {
    fn from(value: SpatialInheritanceError) -> Self {
        Self::Contract(SpatialTopologyV2Error::Kernel(value))
    }
}

impl From<crate::composition::StableKeyError> for SpatialTopologyV2ArchiveError {
    fn from(value: crate::composition::StableKeyError) -> Self {
        Self::Contract(SpatialTopologyV2Error::from(value))
    }
}

impl fmt::Display for SpatialTopologyV2ArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "Spatial Topology V2 JSON error: {error}"),
            Self::Contract(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SpatialTopologyV2ArchiveError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::spatial_inheritance::{
        canonical_hollow_grove_spatial_map, encode_hollow_grove_spatial_archive,
    };

    #[test]
    fn corrected_roster_has_nine_routes_and_only_aura_ridge_as_spine() {
        let topology = canonical_hollow_grove_spatial_topology_v2().unwrap();
        let audit = topology.validate().unwrap();
        assert_eq!(
            (
                audit.route_count,
                audit.straight_route_count,
                audit.curved_route_count
            ),
            (9, 5, 4)
        );
        assert!(audit.current_seanad_is_legacy_only);
        assert!(audit.glausbahn_is_legacy_only);
        assert_eq!(
            TopologyV2RouteId::CurrentSea.geometry(),
            SpatialPathGeometry::Round
        );
        assert_eq!(
            TopologyV2RouteId::Glausbaughn.geometry(),
            SpatialPathGeometry::Straight
        );
    }

    #[test]
    fn aura_farm_is_two_equal_triangles_and_content_farm_borders_full_beach() {
        validate_partition_law().unwrap();
        let field = TopologyV2RegionId::AuraField.polygon_per_mille();
        let content = TopologyV2RegionId::ContentFarm.polygon_per_mille();
        let beach = TopologyV2RegionId::AuraBeach.polygon_per_mille();
        assert_eq!(polygon_twice_area(field), polygon_twice_area(content));
        assert_eq!((content[1], content[2]), (beach[1], beach[0]));
    }

    #[test]
    fn legacy_typos_decode_to_canonical_v2_identities() {
        assert_eq!(
            TopologyV2RouteId::from_stable_or_legacy_id(LEGACY_CURRENT_SEANAD_ROUTE_ID),
            Some(TopologyV2RouteId::CurrentSea)
        );
        assert_eq!(
            TopologyV2RouteId::from_stable_or_legacy_id(LEGACY_GLAUSBAHN_ROUTE_ID),
            Some(TopologyV2RouteId::Glausbaughn)
        );
        assert_eq!(
            TopologyV2RouteId::from_stable_or_legacy_id(LEGACY_CURRENT_SEANAD_ROUTE_ID)
                .unwrap()
                .stable_id(),
            "geography.route.current-sea"
        );
    }

    #[test]
    fn v2_archive_round_trips_and_v1_migration_preserves_frozen_input() {
        let topology = canonical_hollow_grove_spatial_topology_v2().unwrap();
        let bytes = encode_spatial_topology_v2(&topology).unwrap();
        let replayed = decode_spatial_topology_v2(&bytes).unwrap();
        assert_eq!(
            spatial_topology_v2_digest(&replayed).unwrap(),
            spatial_topology_v2_digest(&topology).unwrap()
        );

        let v1 =
            encode_hollow_grove_spatial_archive(&canonical_hollow_grove_spatial_map().unwrap())
                .unwrap();
        let migrated = migrate_spatial_v1_archive_to_topology_v2(&v1).unwrap();
        assert_eq!(migrated, bytes);
        assert_eq!(
            encode_hollow_grove_spatial_archive(
                &super::super::spatial_inheritance::decode_hollow_grove_spatial_archive(&v1)
                    .unwrap()
            )
            .unwrap(),
            v1
        );
    }

    #[test]
    fn inherited_v2_routes_and_regions_keep_local_geometry() {
        let topology = canonical_hollow_grove_spatial_topology_v2().unwrap();
        let world = topology
            .resolve_route(
                HollowGroveSpatialScope::World,
                TopologyV2RouteId::CurrentSea,
            )
            .unwrap();
        let sandmanor = topology
            .resolve_route(
                HollowGroveSpatialScope::House(House::Sandmanor),
                TopologyV2RouteId::CurrentSea,
            )
            .unwrap();
        assert_eq!(world.geometry, SpatialPathGeometry::Round);
        assert_eq!(world.local_from, sandmanor.local_from);
        assert_ne!(world.root_from, sandmanor.root_from);
        assert_eq!(
            topology
                .resolve_region(
                    HollowGroveSpatialScope::World,
                    TopologyV2RegionId::ContentFarm,
                )
                .unwrap()
                .local_polygon
                .len(),
            3
        );
    }
}
