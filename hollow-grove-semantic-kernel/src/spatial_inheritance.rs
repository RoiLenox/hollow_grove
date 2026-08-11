//! Generic spatial templates and parent/child frame inheritance.
//!
//! The kernel owns normalized coordinates, topology, and cached transforms.
//! Domain layers remain responsible for attaching names and meaning.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

use serde::{Deserialize, Serialize};

use super::{
    ScaleKey, SpatialAnchorId, SpatialFrameId, SpatialPathId, SpatialRegionId, SpatialRuleSetId,
    SpatialTemplateId, StableKeyError,
};

/// Fixed-point precision used by every normalized spatial coordinate.
///
/// Integer coordinates keep inherited projections deterministic and make a
/// cached frame transform inexpensive to apply.
pub const NORMALIZED_UNIT: u64 = 1_000_000_000;
pub const SPATIAL_CATALOG_SCHEMA_VERSION: u16 = 1;
pub const SPATIAL_SELECTION_POLICY_ID: &str = "deepest-then-stable-frame-id-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct NormalizedCoordinate(u64);

impl NormalizedCoordinate {
    pub fn new(parts: u64) -> Result<Self, SpatialInheritanceError> {
        if parts <= NORMALIZED_UNIT {
            Ok(Self(parts))
        } else {
            Err(SpatialInheritanceError::CoordinateOutOfRange(parts))
        }
    }

    pub fn from_ratio(numerator: u64, denominator: u64) -> Result<Self, SpatialInheritanceError> {
        if denominator == 0 || numerator > denominator {
            return Err(SpatialInheritanceError::InvalidCoordinateRatio {
                numerator,
                denominator,
            });
        }
        let parts = (u128::from(numerator) * u128::from(NORMALIZED_UNIT)
            + u128::from(denominator / 2))
            / u128::from(denominator);
        Self::new(parts as u64)
    }

    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn midpoint() -> Self {
        Self(NORMALIZED_UNIT / 2)
    }

    #[must_use]
    pub const fn one() -> Self {
        Self(NORMALIZED_UNIT)
    }

    #[must_use]
    pub const fn parts(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for NormalizedCoordinate {
    type Error = SpatialInheritanceError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<NormalizedCoordinate> for u64 {
    fn from(value: NormalizedCoordinate) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NormalizedPoint {
    pub x: NormalizedCoordinate,
    pub y: NormalizedCoordinate,
}

impl NormalizedPoint {
    #[must_use]
    pub const fn new(x: NormalizedCoordinate, y: NormalizedCoordinate) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn center() -> Self {
        Self::new(
            NormalizedCoordinate::midpoint(),
            NormalizedCoordinate::midpoint(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NormalizedRect {
    pub min: NormalizedPoint,
    pub max: NormalizedPoint,
}

impl NormalizedRect {
    pub fn new(
        min: NormalizedPoint,
        max: NormalizedPoint,
    ) -> Result<Self, SpatialInheritanceError> {
        if min.x.parts() >= max.x.parts() || min.y.parts() >= max.y.parts() {
            return Err(SpatialInheritanceError::InvalidFrameBounds);
        }
        Ok(Self { min, max })
    }

    #[must_use]
    pub const fn full() -> Self {
        Self {
            min: NormalizedPoint::new(NormalizedCoordinate::zero(), NormalizedCoordinate::zero()),
            max: NormalizedPoint::new(NormalizedCoordinate::one(), NormalizedCoordinate::one()),
        }
    }

    #[must_use]
    pub const fn contains(self, point: NormalizedPoint) -> bool {
        point.x.parts() >= self.min.x.parts()
            && point.x.parts() <= self.max.x.parts()
            && point.y.parts() >= self.min.y.parts()
            && point.y.parts() <= self.max.y.parts()
    }

    #[must_use]
    pub const fn width(self) -> u64 {
        self.max.x.parts() - self.min.x.parts()
    }

    #[must_use]
    pub const fn height(self) -> u64 {
        self.max.y.parts() - self.min.y.parts()
    }

    #[must_use]
    pub const fn area(self) -> u128 {
        self.width() as u128 * self.height() as u128
    }

    #[must_use]
    pub const fn center(self) -> NormalizedPoint {
        NormalizedPoint::new(
            NormalizedCoordinate(self.min.x.parts() + self.width() / 2),
            NormalizedCoordinate(self.min.y.parts() + self.height() / 2),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpatialPathGeometry {
    Straight,
    Round,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialAnchor {
    pub id: SpatialAnchorId,
    pub position: NormalizedPoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialPath {
    pub id: SpatialPathId,
    pub from: SpatialAnchorId,
    pub to: SpatialAnchorId,
    pub geometry: SpatialPathGeometry,
    /// Ordered control points in template-local coordinates.
    ///
    /// Straight paths have none. Round paths currently use one quadratic
    /// control point so every renderer receives exactly the same curve.
    pub control_points: Vec<NormalizedPoint>,
    pub band_width: NormalizedCoordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialRegion {
    pub id: SpatialRegionId,
    pub polygon: Vec<NormalizedPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialTemplate {
    id: SpatialTemplateId,
    anchors: Vec<SpatialAnchor>,
    paths: Vec<SpatialPath>,
    regions: Vec<SpatialRegion>,
}

impl SpatialTemplate {
    pub fn new(
        id: SpatialTemplateId,
        anchors: Vec<SpatialAnchor>,
        paths: Vec<SpatialPath>,
        regions: Vec<SpatialRegion>,
    ) -> Result<Self, SpatialInheritanceError> {
        let mut template = Self {
            id,
            anchors,
            paths,
            regions,
        };
        template.normalize_and_validate()?;
        Ok(template)
    }

    fn normalize_and_validate(&mut self) -> Result<(), SpatialInheritanceError> {
        let mut anchor_ids = BTreeSet::new();
        for anchor in &self.anchors {
            if !anchor_ids.insert(anchor.id.clone()) {
                return Err(SpatialInheritanceError::DuplicateAnchorId(
                    anchor.id.clone(),
                ));
            }
        }

        let mut path_ids = BTreeSet::new();
        for path in &self.paths {
            if !path_ids.insert(path.id.clone()) {
                return Err(SpatialInheritanceError::DuplicatePathId(path.id.clone()));
            }
            if path.from == path.to {
                return Err(SpatialInheritanceError::PathHasSameEndpoint(
                    path.id.clone(),
                ));
            }
            if path.band_width.parts() == 0 {
                return Err(SpatialInheritanceError::PathHasZeroBandWidth(
                    path.id.clone(),
                ));
            }
            let expected_controls = match path.geometry {
                SpatialPathGeometry::Straight => 0,
                SpatialPathGeometry::Round => 1,
            };
            if path.control_points.len() != expected_controls {
                return Err(SpatialInheritanceError::InvalidPathControlCount {
                    path: path.id.clone(),
                    geometry: path.geometry,
                    expected: expected_controls,
                    actual: path.control_points.len(),
                });
            }
            for endpoint in [&path.from, &path.to] {
                if !anchor_ids.contains(endpoint) {
                    return Err(SpatialInheritanceError::MissingPathAnchor {
                        path: path.id.clone(),
                        anchor: endpoint.clone(),
                    });
                }
            }
        }

        let mut region_ids = BTreeSet::new();
        for region in &self.regions {
            if !region_ids.insert(region.id.clone()) {
                return Err(SpatialInheritanceError::DuplicateRegionId(
                    region.id.clone(),
                ));
            }
            if region.polygon.len() < 3 {
                return Err(SpatialInheritanceError::InvalidRegionPolygon(
                    region.id.clone(),
                ));
            }
            let unique_vertices = region.polygon.iter().copied().collect::<BTreeSet<_>>();
            if unique_vertices.len() != region.polygon.len() || polygon_twice_area(region) == 0 {
                return Err(SpatialInheritanceError::InvalidRegionPolygon(
                    region.id.clone(),
                ));
            }
        }

        self.anchors.sort_by(|left, right| left.id.cmp(&right.id));
        self.paths.sort_by(|left, right| left.id.cmp(&right.id));
        self.regions.sort_by(|left, right| left.id.cmp(&right.id));
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> &SpatialTemplateId {
        &self.id
    }

    #[must_use]
    pub fn anchors(&self) -> &[SpatialAnchor] {
        &self.anchors
    }

    #[must_use]
    pub fn paths(&self) -> &[SpatialPath] {
        &self.paths
    }

    #[must_use]
    pub fn regions(&self) -> &[SpatialRegion] {
        &self.regions
    }

    #[must_use]
    pub fn anchor(&self, id: &SpatialAnchorId) -> Option<&SpatialAnchor> {
        self.anchors
            .binary_search_by(|anchor| anchor.id.cmp(id))
            .ok()
            .and_then(|position| self.anchors.get(position))
    }

    #[must_use]
    pub fn path(&self, id: &SpatialPathId) -> Option<&SpatialPath> {
        self.paths
            .binary_search_by(|path| path.id.cmp(id))
            .ok()
            .and_then(|position| self.paths.get(position))
    }

    #[must_use]
    pub fn region(&self, id: &SpatialRegionId) -> Option<&SpatialRegion> {
        self.regions
            .binary_search_by(|region| region.id.cmp(id))
            .ok()
            .and_then(|position| self.regions.get(position))
    }
}

fn polygon_twice_area(region: &SpatialRegion) -> i128 {
    region
        .polygon
        .iter()
        .zip(region.polygon.iter().cycle().skip(1))
        .take(region.polygon.len())
        .map(|(left, right)| {
            i128::from(left.x.parts()) * i128::from(right.y.parts())
                - i128::from(right.x.parts()) * i128::from(left.y.parts())
        })
        .sum::<i128>()
        .abs()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialFrame {
    id: SpatialFrameId,
    parent_id: Option<SpatialFrameId>,
    template_id: SpatialTemplateId,
    scale: ScaleKey,
    bounds_in_parent: NormalizedRect,
}

impl SpatialFrame {
    #[must_use]
    pub fn root(id: SpatialFrameId, template_id: SpatialTemplateId, scale: ScaleKey) -> Self {
        Self {
            id,
            parent_id: None,
            template_id,
            scale,
            bounds_in_parent: NormalizedRect::full(),
        }
    }

    #[must_use]
    pub fn child(
        id: SpatialFrameId,
        parent_id: SpatialFrameId,
        template_id: SpatialTemplateId,
        scale: ScaleKey,
        bounds_in_parent: NormalizedRect,
    ) -> Self {
        Self {
            id,
            parent_id: Some(parent_id),
            template_id,
            scale,
            bounds_in_parent,
        }
    }

    #[must_use]
    pub fn id(&self) -> &SpatialFrameId {
        &self.id
    }

    #[must_use]
    pub fn parent_id(&self) -> Option<&SpatialFrameId> {
        self.parent_id.as_ref()
    }

    #[must_use]
    pub fn template_id(&self) -> &SpatialTemplateId {
        &self.template_id
    }

    #[must_use]
    pub fn scale(&self) -> &ScaleKey {
        &self.scale
    }

    #[must_use]
    pub const fn bounds_in_parent(&self) -> NormalizedRect {
        self.bounds_in_parent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialTransform {
    bounds_in_root: NormalizedRect,
}

impl SpatialTransform {
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            bounds_in_root: NormalizedRect::full(),
        }
    }

    #[must_use]
    pub const fn bounds_in_root(self) -> NormalizedRect {
        self.bounds_in_root
    }

    #[must_use]
    pub fn map_to_root(self, local: NormalizedPoint) -> NormalizedPoint {
        NormalizedPoint::new(
            map_axis(
                self.bounds_in_root.min.x,
                self.bounds_in_root.max.x,
                local.x,
            ),
            map_axis(
                self.bounds_in_root.min.y,
                self.bounds_in_root.max.y,
                local.y,
            ),
        )
    }

    #[must_use]
    pub fn map_from_root(self, root: NormalizedPoint) -> Option<NormalizedPoint> {
        if !self.bounds_in_root.contains(root) {
            return None;
        }
        Some(NormalizedPoint::new(
            unmap_axis(self.bounds_in_root.min.x, self.bounds_in_root.max.x, root.x),
            unmap_axis(self.bounds_in_root.min.y, self.bounds_in_root.max.y, root.y),
        ))
    }

    fn compose(self, child_bounds: NormalizedRect) -> Result<Self, SpatialInheritanceError> {
        let min = self.map_to_root(child_bounds.min);
        let max = self.map_to_root(child_bounds.max);
        Ok(Self {
            bounds_in_root: NormalizedRect::new(min, max)?,
        })
    }
}

fn map_axis(
    minimum: NormalizedCoordinate,
    maximum: NormalizedCoordinate,
    local: NormalizedCoordinate,
) -> NormalizedCoordinate {
    let span = maximum.parts() - minimum.parts();
    let projected = (u128::from(span) * u128::from(local.parts())
        + u128::from(NORMALIZED_UNIT / 2))
        / u128::from(NORMALIZED_UNIT);
    NormalizedCoordinate(minimum.parts() + projected as u64)
}

fn unmap_axis(
    minimum: NormalizedCoordinate,
    maximum: NormalizedCoordinate,
    root: NormalizedCoordinate,
) -> NormalizedCoordinate {
    let span = maximum.parts() - minimum.parts();
    let offset = root.parts() - minimum.parts();
    let projected = (u128::from(offset) * u128::from(NORMALIZED_UNIT) + u128::from(span / 2))
        / u128::from(span);
    NormalizedCoordinate(projected.min(u128::from(NORMALIZED_UNIT)) as u64)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSpatialAnchor {
    pub frame_id: SpatialFrameId,
    pub anchor_id: SpatialAnchorId,
    pub local_position: NormalizedPoint,
    pub root_position: NormalizedPoint,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSpatialPath {
    pub frame_id: SpatialFrameId,
    pub path_id: SpatialPathId,
    pub geometry: SpatialPathGeometry,
    pub band_width: NormalizedCoordinate,
    pub local_from: NormalizedPoint,
    pub local_to: NormalizedPoint,
    pub local_control_points: Vec<NormalizedPoint>,
    pub root_from: NormalizedPoint,
    pub root_to: NormalizedPoint,
    pub root_control_points: Vec<NormalizedPoint>,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSpatialRegion {
    pub frame_id: SpatialFrameId,
    pub region_id: SpatialRegionId,
    pub local_polygon: Vec<NormalizedPoint>,
    pub root_polygon: Vec<NormalizedPoint>,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialViewpoint {
    pub frame_id: SpatialFrameId,
    pub template_id: SpatialTemplateId,
    pub scale: ScaleKey,
    pub local_position: NormalizedPoint,
    pub root_position: NormalizedPoint,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialViewpointSelection {
    pub policy_id: &'static str,
    pub selected: SpatialViewpoint,
    pub candidates: Vec<SpatialViewpoint>,
}

/// Versioned immutable representation used for deterministic replay.
///
/// Vectors are canonicalized by stable identity before the snapshot is
/// returned or replayed. Derived transforms are deliberately omitted and are
/// always recomputed from declarations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpatialCatalogSnapshot {
    pub schema_version: u16,
    pub rule_set: SpatialRuleSetId,
    pub templates: Vec<SpatialTemplate>,
    pub frames: Vec<SpatialFrame>,
}

#[derive(Debug, Clone, Copy)]
struct CachedFrame {
    transform: SpatialTransform,
    depth: usize,
}

/// Validated frame tree with precomposed root transforms.
///
/// Each viewpoint or inherited anchor resolves with one frame lookup and one
/// fixed-point affine transform, regardless of nesting depth.
#[derive(Debug)]
pub struct SpatialInheritanceCatalog {
    rule_set: SpatialRuleSetId,
    templates: HashMap<SpatialTemplateId, SpatialTemplate>,
    frames: HashMap<SpatialFrameId, SpatialFrame>,
    cached_frames: HashMap<SpatialFrameId, CachedFrame>,
    root_frame_id: Option<SpatialFrameId>,
}

impl Default for SpatialInheritanceCatalog {
    fn default() -> Self {
        Self::with_rule_set(
            SpatialRuleSetId::new("ruleset.spatial.default-v1")
                .expect("default spatial rule-set ID is stable"),
        )
    }
}

impl SpatialInheritanceCatalog {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_rule_set(rule_set: SpatialRuleSetId) -> Self {
        Self {
            rule_set,
            templates: HashMap::new(),
            frames: HashMap::new(),
            cached_frames: HashMap::new(),
            root_frame_id: None,
        }
    }

    #[must_use]
    pub fn rule_set(&self) -> &SpatialRuleSetId {
        &self.rule_set
    }

    pub fn insert_template(
        &mut self,
        mut template: SpatialTemplate,
    ) -> Result<(), SpatialInheritanceError> {
        if self.templates.contains_key(template.id()) {
            return Err(SpatialInheritanceError::DuplicateTemplateId(
                template.id().clone(),
            ));
        }
        template.normalize_and_validate()?;
        self.templates.insert(template.id().clone(), template);
        Ok(())
    }

    pub fn insert_frame(&mut self, frame: SpatialFrame) -> Result<(), SpatialInheritanceError> {
        if self.frames.contains_key(frame.id()) {
            return Err(SpatialInheritanceError::DuplicateFrameId(
                frame.id().clone(),
            ));
        }
        if !self.templates.contains_key(frame.template_id()) {
            return Err(SpatialInheritanceError::MissingTemplate(
                frame.template_id().clone(),
            ));
        }

        let cached = match frame.parent_id() {
            Some(parent_id) => {
                let parent = self
                    .cached_frames
                    .get(parent_id)
                    .ok_or_else(|| SpatialInheritanceError::MissingParent(parent_id.clone()))?;
                CachedFrame {
                    transform: parent.transform.compose(frame.bounds_in_parent())?,
                    depth: parent.depth + 1,
                }
            }
            None => {
                if let Some(root) = &self.root_frame_id {
                    return Err(SpatialInheritanceError::MultipleRootFrames {
                        existing: root.clone(),
                        proposed: frame.id().clone(),
                    });
                }
                if frame.bounds_in_parent() != NormalizedRect::full() {
                    return Err(SpatialInheritanceError::RootMustFillUnit);
                }
                CachedFrame {
                    transform: SpatialTransform::identity(),
                    depth: 0,
                }
            }
        };

        if frame.parent_id().is_none() {
            self.root_frame_id = Some(frame.id().clone());
        }
        self.cached_frames.insert(frame.id().clone(), cached);
        self.frames.insert(frame.id().clone(), frame);
        Ok(())
    }

    #[must_use]
    pub fn root_frame_id(&self) -> Option<&SpatialFrameId> {
        self.root_frame_id.as_ref()
    }

    #[must_use]
    pub fn template(&self, id: &SpatialTemplateId) -> Option<&SpatialTemplate> {
        self.templates.get(id)
    }

    #[must_use]
    pub fn frame(&self, id: &SpatialFrameId) -> Option<&SpatialFrame> {
        self.frames.get(id)
    }

    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    #[must_use]
    pub fn template_count(&self) -> usize {
        self.templates.len()
    }

    pub fn resolve_viewpoint(
        &self,
        frame_id: &SpatialFrameId,
        local_position: NormalizedPoint,
    ) -> Result<SpatialViewpoint, SpatialInheritanceError> {
        let frame = self
            .frame(frame_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))?;
        let cached = self
            .cached_frames
            .get(frame_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))?;
        Ok(SpatialViewpoint {
            frame_id: frame_id.clone(),
            template_id: frame.template_id().clone(),
            scale: frame.scale().clone(),
            local_position,
            root_position: cached.transform.map_to_root(local_position),
            depth: cached.depth,
        })
    }

    pub fn resolve_anchor(
        &self,
        frame_id: &SpatialFrameId,
        anchor_id: &SpatialAnchorId,
    ) -> Result<ResolvedSpatialAnchor, SpatialInheritanceError> {
        let frame = self
            .frame(frame_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))?;
        let template = self
            .template(frame.template_id())
            .ok_or_else(|| SpatialInheritanceError::MissingTemplate(frame.template_id().clone()))?;
        let anchor =
            template
                .anchor(anchor_id)
                .ok_or_else(|| SpatialInheritanceError::MissingAnchor {
                    template: template.id().clone(),
                    anchor: anchor_id.clone(),
                })?;
        let viewpoint = self.resolve_viewpoint(frame_id, anchor.position)?;
        Ok(ResolvedSpatialAnchor {
            frame_id: frame_id.clone(),
            anchor_id: anchor_id.clone(),
            local_position: anchor.position,
            root_position: viewpoint.root_position,
            depth: viewpoint.depth,
        })
    }

    pub fn resolve_path(
        &self,
        frame_id: &SpatialFrameId,
        path_id: &SpatialPathId,
    ) -> Result<ResolvedSpatialPath, SpatialInheritanceError> {
        let frame = self
            .frame(frame_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))?;
        let template = self
            .template(frame.template_id())
            .ok_or_else(|| SpatialInheritanceError::MissingTemplate(frame.template_id().clone()))?;
        let path = template
            .path(path_id)
            .ok_or_else(|| SpatialInheritanceError::MissingPath {
                template: template.id().clone(),
                path: path_id.clone(),
            })?;
        let from = template
            .anchor(&path.from)
            .expect("validated templates retain path endpoints");
        let to = template
            .anchor(&path.to)
            .expect("validated templates retain path endpoints");
        let cached = self
            .cached_frames
            .get(frame_id)
            .expect("validated frames retain cached transforms");
        Ok(ResolvedSpatialPath {
            frame_id: frame_id.clone(),
            path_id: path_id.clone(),
            geometry: path.geometry,
            band_width: path.band_width,
            local_from: from.position,
            local_to: to.position,
            local_control_points: path.control_points.clone(),
            root_from: cached.transform.map_to_root(from.position),
            root_to: cached.transform.map_to_root(to.position),
            root_control_points: path
                .control_points
                .iter()
                .copied()
                .map(|point| cached.transform.map_to_root(point))
                .collect(),
            depth: cached.depth,
        })
    }

    pub fn resolve_region(
        &self,
        frame_id: &SpatialFrameId,
        region_id: &SpatialRegionId,
    ) -> Result<ResolvedSpatialRegion, SpatialInheritanceError> {
        let frame = self
            .frame(frame_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))?;
        let template = self
            .template(frame.template_id())
            .ok_or_else(|| SpatialInheritanceError::MissingTemplate(frame.template_id().clone()))?;
        let region =
            template
                .region(region_id)
                .ok_or_else(|| SpatialInheritanceError::MissingRegion {
                    template: template.id().clone(),
                    region: region_id.clone(),
                })?;
        let cached = self
            .cached_frames
            .get(frame_id)
            .expect("validated frames retain cached transforms");
        Ok(ResolvedSpatialRegion {
            frame_id: frame_id.clone(),
            region_id: region_id.clone(),
            local_polygon: region.polygon.clone(),
            root_polygon: region
                .polygon
                .iter()
                .copied()
                .map(|point| cached.transform.map_to_root(point))
                .collect(),
            depth: cached.depth,
        })
    }

    pub fn select_viewpoint(
        &self,
        root_position: NormalizedPoint,
    ) -> Option<SpatialViewpointSelection> {
        let mut candidates = self
            .cached_frames
            .iter()
            .filter_map(|(frame_id, cached)| {
                let frame = self
                    .frames
                    .get(frame_id)
                    .expect("cached spatial frames retain declarations");
                cached
                    .transform
                    .map_from_root(root_position)
                    .map(|local_position| SpatialViewpoint {
                        frame_id: frame_id.clone(),
                        template_id: frame.template_id().clone(),
                        scale: frame.scale().clone(),
                        local_position,
                        root_position,
                        depth: cached.depth,
                    })
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| {
            right
                .depth
                .cmp(&left.depth)
                .then_with(|| left.frame_id.cmp(&right.frame_id))
        });
        let selected = candidates.first()?.clone();
        Some(SpatialViewpointSelection {
            policy_id: SPATIAL_SELECTION_POLICY_ID,
            selected,
            candidates,
        })
    }

    #[must_use]
    pub fn deepest_viewpoint(&self, root_position: NormalizedPoint) -> Option<SpatialViewpoint> {
        self.select_viewpoint(root_position)
            .map(|selection| selection.selected)
    }

    pub fn lineage(
        &self,
        frame_id: &SpatialFrameId,
    ) -> Result<Vec<&SpatialFrame>, SpatialInheritanceError> {
        let mut lineage = Vec::new();
        let mut current = self
            .frame(frame_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))?;
        lineage.push(current);
        while let Some(parent_id) = current.parent_id() {
            current = self
                .frame(parent_id)
                .expect("validated frame parents remain present");
            lineage.push(current);
        }
        lineage.reverse();
        Ok(lineage)
    }

    pub fn frame_transform(
        &self,
        frame_id: &SpatialFrameId,
    ) -> Result<SpatialTransform, SpatialInheritanceError> {
        self.cached_frames
            .get(frame_id)
            .map(|cached| cached.transform)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))
    }

    pub fn frame_depth(&self, frame_id: &SpatialFrameId) -> Result<usize, SpatialInheritanceError> {
        self.cached_frames
            .get(frame_id)
            .map(|cached| cached.depth)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(frame_id.clone()))
    }

    #[must_use]
    pub fn snapshot(&self) -> SpatialCatalogSnapshot {
        let mut templates = self.templates.values().cloned().collect::<Vec<_>>();
        templates.sort_by(|left, right| left.id().cmp(right.id()));
        let mut frames = self.frames.values().cloned().collect::<Vec<_>>();
        frames.sort_by(|left, right| left.id().cmp(right.id()));
        SpatialCatalogSnapshot {
            schema_version: SPATIAL_CATALOG_SCHEMA_VERSION,
            rule_set: self.rule_set.clone(),
            templates,
            frames,
        }
    }

    pub fn replay_snapshot(
        mut snapshot: SpatialCatalogSnapshot,
    ) -> Result<Self, SpatialInheritanceError> {
        if snapshot.schema_version != SPATIAL_CATALOG_SCHEMA_VERSION {
            return Err(SpatialInheritanceError::UnsupportedCatalogSchema {
                expected: SPATIAL_CATALOG_SCHEMA_VERSION,
                actual: snapshot.schema_version,
            });
        }
        if snapshot.templates.is_empty() {
            return Err(SpatialInheritanceError::TemplateRequired);
        }
        if snapshot.frames.is_empty() {
            return Err(SpatialInheritanceError::RootFrameRequired);
        }

        snapshot
            .templates
            .sort_by(|left, right| left.id().cmp(right.id()));
        snapshot
            .frames
            .sort_by(|left, right| left.id().cmp(right.id()));

        let mut catalog = Self::with_rule_set(snapshot.rule_set);
        for template in snapshot.templates {
            catalog.insert_template(template)?;
        }

        let mut pending = BTreeMap::new();
        for frame in snapshot.frames {
            let frame_id = frame.id().clone();
            if pending.insert(frame_id.clone(), frame).is_some() {
                return Err(SpatialInheritanceError::DuplicateFrameId(frame_id));
            }
        }
        let declared_frame_ids = pending.keys().cloned().collect::<BTreeSet<_>>();
        for frame in pending.values() {
            if let Some(parent_id) = frame.parent_id()
                && !declared_frame_ids.contains(parent_id)
            {
                return Err(SpatialInheritanceError::MissingParent(parent_id.clone()));
            }
        }

        let roots = pending
            .values()
            .filter(|frame| frame.parent_id().is_none())
            .map(|frame| frame.id().clone())
            .collect::<Vec<_>>();
        match roots.as_slice() {
            [] => return Err(SpatialInheritanceError::RootFrameRequired),
            [_] => {}
            [existing, proposed, ..] => {
                return Err(SpatialInheritanceError::MultipleRootFrames {
                    existing: existing.clone(),
                    proposed: proposed.clone(),
                });
            }
        }

        let root = pending
            .remove(&roots[0])
            .expect("root ID was collected from pending frames");
        catalog.insert_frame(root)?;

        while !pending.is_empty() {
            let ready = pending
                .iter()
                .filter(|(_, frame)| {
                    frame
                        .parent_id()
                        .is_some_and(|parent_id| catalog.frames.contains_key(parent_id))
                })
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>();
            if ready.is_empty() {
                return Err(SpatialInheritanceError::FrameCycle(
                    pending.keys().cloned().collect(),
                ));
            }
            for id in ready {
                let frame = pending
                    .remove(&id)
                    .expect("ready frame remains pending until insertion");
                catalog.insert_frame(frame)?;
            }
        }

        catalog.validate_integrity()?;
        Ok(catalog)
    }

    pub fn validate_integrity(&self) -> Result<(), SpatialInheritanceError> {
        let Some(root_id) = self.root_frame_id.as_ref() else {
            return Err(SpatialInheritanceError::RootFrameRequired);
        };
        if self.templates.is_empty() {
            return Err(SpatialInheritanceError::TemplateRequired);
        }
        if self.frames.len() != self.cached_frames.len() {
            return Err(SpatialInheritanceError::DerivedFrameCacheMismatch);
        }
        let root = self
            .frames
            .get(root_id)
            .ok_or_else(|| SpatialInheritanceError::MissingFrame(root_id.clone()))?;
        if root.parent_id().is_some() || root.bounds_in_parent() != NormalizedRect::full() {
            return Err(SpatialInheritanceError::RootMustFillUnit);
        }
        for frame in self.frames.values() {
            if !self.templates.contains_key(frame.template_id()) {
                return Err(SpatialInheritanceError::MissingTemplate(
                    frame.template_id().clone(),
                ));
            }
            if let Some(parent_id) = frame.parent_id()
                && !self.frames.contains_key(parent_id)
            {
                return Err(SpatialInheritanceError::MissingParent(parent_id.clone()));
            }
            let expected_depth = self.lineage(frame.id())?.len() - 1;
            let actual_depth = self.frame_depth(frame.id())?;
            if expected_depth != actual_depth {
                return Err(SpatialInheritanceError::DerivedFrameDepthMismatch {
                    frame: frame.id().clone(),
                    expected: expected_depth,
                    actual: actual_depth,
                });
            }
        }
        Ok(())
    }

    pub fn canonical_digest(&self) -> Result<String, SpatialInheritanceError> {
        self.validate_integrity()?;
        let snapshot = self.snapshot();
        let mut digest = SpatialDigest::new();
        digest.string("hollow-grove-spatial-catalog");
        digest.u64(u64::from(snapshot.schema_version));
        digest.string(snapshot.rule_set.as_str());
        digest.u64(snapshot.templates.len() as u64);
        for template in &snapshot.templates {
            digest.template(template);
        }
        digest.u64(snapshot.frames.len() as u64);
        for frame in &snapshot.frames {
            digest.frame(frame);
        }
        Ok(format!("fnv1a64-spatial-v1:{:016x}", digest.finish()))
    }

    pub fn template_digest(
        &self,
        template_id: &SpatialTemplateId,
    ) -> Result<String, SpatialInheritanceError> {
        let template = self
            .template(template_id)
            .ok_or_else(|| SpatialInheritanceError::MissingTemplate(template_id.clone()))?;
        let mut digest = SpatialDigest::new();
        digest.string("hollow-grove-spatial-template");
        digest.template(template);
        Ok(format!(
            "fnv1a64-spatial-template-v1:{:016x}",
            digest.finish()
        ))
    }
}

struct SpatialDigest(u64);

impl SpatialDigest {
    const fn new() -> Self {
        Self(0xcbf29ce484222325)
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }

    fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes());
    }

    fn string(&mut self, value: &str) {
        self.u64(value.len() as u64);
        self.bytes(value.as_bytes());
    }

    fn point(&mut self, point: NormalizedPoint) {
        self.u64(point.x.parts());
        self.u64(point.y.parts());
    }

    fn template(&mut self, template: &SpatialTemplate) {
        self.string(template.id().as_str());
        self.u64(template.anchors().len() as u64);
        for anchor in template.anchors() {
            self.string(anchor.id.as_str());
            self.point(anchor.position);
        }
        self.u64(template.paths().len() as u64);
        for path in template.paths() {
            self.string(path.id.as_str());
            self.string(path.from.as_str());
            self.string(path.to.as_str());
            self.u64(match path.geometry {
                SpatialPathGeometry::Straight => 0,
                SpatialPathGeometry::Round => 1,
            });
            self.u64(path.band_width.parts());
            self.u64(path.control_points.len() as u64);
            for point in &path.control_points {
                self.point(*point);
            }
        }
        self.u64(template.regions().len() as u64);
        for region in template.regions() {
            self.string(region.id.as_str());
            self.u64(region.polygon.len() as u64);
            for point in &region.polygon {
                self.point(*point);
            }
        }
    }

    fn frame(&mut self, frame: &SpatialFrame) {
        self.string(frame.id().as_str());
        match frame.parent_id() {
            Some(parent) => {
                self.u64(1);
                self.string(parent.as_str());
            }
            None => self.u64(0),
        }
        self.string(frame.template_id().as_str());
        self.string(frame.scale().as_str());
        self.point(frame.bounds_in_parent().min);
        self.point(frame.bounds_in_parent().max);
    }

    const fn finish(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpatialInheritanceError {
    StableKey(StableKeyError),
    UnsupportedCatalogSchema {
        expected: u16,
        actual: u16,
    },
    CoordinateOutOfRange(u64),
    InvalidCoordinateRatio {
        numerator: u64,
        denominator: u64,
    },
    InvalidFrameBounds,
    TemplateRequired,
    RootFrameRequired,
    DuplicateTemplateId(SpatialTemplateId),
    DuplicateAnchorId(SpatialAnchorId),
    DuplicatePathId(SpatialPathId),
    PathHasSameEndpoint(SpatialPathId),
    PathHasZeroBandWidth(SpatialPathId),
    InvalidPathControlCount {
        path: SpatialPathId,
        geometry: SpatialPathGeometry,
        expected: usize,
        actual: usize,
    },
    MissingPathAnchor {
        path: SpatialPathId,
        anchor: SpatialAnchorId,
    },
    DuplicateRegionId(SpatialRegionId),
    InvalidRegionPolygon(SpatialRegionId),
    DuplicateFrameId(SpatialFrameId),
    MissingTemplate(SpatialTemplateId),
    MissingParent(SpatialFrameId),
    FrameCycle(Vec<SpatialFrameId>),
    MultipleRootFrames {
        existing: SpatialFrameId,
        proposed: SpatialFrameId,
    },
    RootMustFillUnit,
    DerivedFrameCacheMismatch,
    DerivedFrameDepthMismatch {
        frame: SpatialFrameId,
        expected: usize,
        actual: usize,
    },
    MissingFrame(SpatialFrameId),
    MissingAnchor {
        template: SpatialTemplateId,
        anchor: SpatialAnchorId,
    },
    MissingPath {
        template: SpatialTemplateId,
        path: SpatialPathId,
    },
    MissingRegion {
        template: SpatialTemplateId,
        region: SpatialRegionId,
    },
}

impl From<StableKeyError> for SpatialInheritanceError {
    fn from(value: StableKeyError) -> Self {
        Self::StableKey(value)
    }
}

impl fmt::Display for SpatialInheritanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StableKey(error) => error.fmt(formatter),
            Self::UnsupportedCatalogSchema { expected, actual } => write!(
                formatter,
                "unsupported spatial catalog schema {actual}; expected {expected}"
            ),
            Self::CoordinateOutOfRange(parts) => {
                write!(
                    formatter,
                    "normalized coordinate exceeds {NORMALIZED_UNIT}: {parts}"
                )
            }
            Self::InvalidCoordinateRatio {
                numerator,
                denominator,
            } => write!(
                formatter,
                "invalid normalized coordinate ratio: {numerator}/{denominator}"
            ),
            Self::InvalidFrameBounds => {
                formatter.write_str("spatial frame bounds must have positive width and height")
            }
            Self::TemplateRequired => {
                formatter.write_str("spatial catalog requires at least one template")
            }
            Self::RootFrameRequired => {
                formatter.write_str("spatial catalog requires exactly one root frame")
            }
            Self::DuplicateTemplateId(id) => write!(formatter, "duplicate spatial template: {id}"),
            Self::DuplicateAnchorId(id) => write!(formatter, "duplicate spatial anchor: {id}"),
            Self::DuplicatePathId(id) => write!(formatter, "duplicate spatial path: {id}"),
            Self::PathHasSameEndpoint(id) => {
                write!(formatter, "spatial path has the same endpoint twice: {id}")
            }
            Self::PathHasZeroBandWidth(id) => {
                write!(formatter, "spatial path has zero band width: {id}")
            }
            Self::InvalidPathControlCount {
                path,
                geometry,
                expected,
                actual,
            } => write!(
                formatter,
                "spatial path {path} has {actual} controls for {geometry:?}; expected {expected}"
            ),
            Self::MissingPathAnchor { path, anchor } => {
                write!(
                    formatter,
                    "spatial path {path} references missing anchor {anchor}"
                )
            }
            Self::DuplicateRegionId(id) => write!(formatter, "duplicate spatial region: {id}"),
            Self::InvalidRegionPolygon(id) => {
                write!(formatter, "spatial region has an invalid polygon: {id}")
            }
            Self::DuplicateFrameId(id) => write!(formatter, "duplicate spatial frame: {id}"),
            Self::MissingTemplate(id) => write!(formatter, "missing spatial template: {id}"),
            Self::MissingParent(id) => write!(formatter, "missing spatial parent frame: {id}"),
            Self::FrameCycle(ids) => write!(
                formatter,
                "spatial frame ancestry contains a cycle involving: {}",
                ids.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::MultipleRootFrames { existing, proposed } => write!(
                formatter,
                "spatial catalog already has root {existing}; cannot add {proposed}"
            ),
            Self::RootMustFillUnit => {
                formatter.write_str("root spatial frame must fill the normalized unit")
            }
            Self::DerivedFrameCacheMismatch => {
                formatter.write_str("spatial derived-frame cache does not match declarations")
            }
            Self::DerivedFrameDepthMismatch {
                frame,
                expected,
                actual,
            } => write!(
                formatter,
                "spatial frame {frame} has derived depth {actual}; expected {expected}"
            ),
            Self::MissingFrame(id) => write!(formatter, "missing spatial frame: {id}"),
            Self::MissingAnchor { template, anchor } => {
                write!(formatter, "template {template} has no anchor {anchor}")
            }
            Self::MissingPath { template, path } => {
                write!(formatter, "template {template} has no path {path}")
            }
            Self::MissingRegion { template, region } => {
                write!(formatter, "template {template} has no region {region}")
            }
        }
    }
}

impl std::error::Error for SpatialInheritanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn key<T>(value: &str) -> T
    where
        T: TryFrom<String, Error = StableKeyError>,
    {
        T::try_from(value.to_owned()).unwrap()
    }

    fn coordinate(numerator: u64, denominator: u64) -> NormalizedCoordinate {
        NormalizedCoordinate::from_ratio(numerator, denominator).unwrap()
    }

    fn point(x: u64, y: u64) -> NormalizedPoint {
        NormalizedPoint::new(coordinate(x, 100), coordinate(y, 100))
    }

    fn template() -> SpatialTemplate {
        SpatialTemplate::new(
            key("template.test.cross"),
            vec![
                SpatialAnchor {
                    id: key("anchor.center"),
                    position: point(50, 50),
                },
                SpatialAnchor {
                    id: key("anchor.north"),
                    position: point(50, 10),
                },
                SpatialAnchor {
                    id: key("anchor.east"),
                    position: point(90, 50),
                },
            ],
            vec![
                SpatialPath {
                    id: key("path.straight"),
                    from: key("anchor.north"),
                    to: key("anchor.center"),
                    geometry: SpatialPathGeometry::Straight,
                    control_points: vec![],
                    band_width: coordinate(45, 1_000),
                },
                SpatialPath {
                    id: key("path.round"),
                    from: key("anchor.north"),
                    to: key("anchor.east"),
                    geometry: SpatialPathGeometry::Round,
                    control_points: vec![point(80, 20)],
                    band_width: coordinate(45, 1_000),
                },
            ],
            vec![SpatialRegion {
                id: key("region.test"),
                polygon: vec![point(50, 10), point(90, 50), point(50, 50)],
            }],
        )
        .unwrap()
    }

    fn catalog() -> SpatialInheritanceCatalog {
        let template = template();
        let template_id = template.id().clone();
        let root_id: SpatialFrameId = key("frame.test.root");
        let child_id: SpatialFrameId = key("frame.test.child");
        let mut catalog = SpatialInheritanceCatalog::new();
        catalog.insert_template(template).unwrap();
        catalog
            .insert_frame(SpatialFrame::root(
                root_id.clone(),
                template_id.clone(),
                key("scale.world"),
            ))
            .unwrap();
        catalog
            .insert_frame(SpatialFrame::child(
                child_id.clone(),
                root_id,
                template_id.clone(),
                key("scale.city"),
                NormalizedRect::new(point(20, 20), point(60, 60)).unwrap(),
            ))
            .unwrap();
        catalog
            .insert_frame(SpatialFrame::child(
                key("frame.test.grandchild"),
                child_id,
                template_id,
                key("scale.district"),
                NormalizedRect::new(point(25, 25), point(75, 75)).unwrap(),
            ))
            .unwrap();
        catalog
    }

    #[test]
    fn every_depth_reuses_one_template_without_copying_topology() {
        let catalog = catalog();
        assert_eq!(catalog.template_count(), 1);
        assert_eq!(catalog.frame_count(), 3);
        for frame_id in [
            key("frame.test.root"),
            key("frame.test.child"),
            key("frame.test.grandchild"),
        ] {
            assert_eq!(
                catalog.frame(&frame_id).unwrap().template_id().as_str(),
                "template.test.cross"
            );
        }
    }

    #[test]
    fn cached_transform_resolves_grandchild_anchor_directly_to_root() {
        let catalog = catalog();
        let resolved = catalog
            .resolve_anchor(&key("frame.test.grandchild"), &key("anchor.center"))
            .unwrap();
        assert_eq!(resolved.depth, 2);
        assert_eq!(resolved.local_position, point(50, 50));
        assert_eq!(resolved.root_position, point(40, 40));
        assert_eq!(catalog.lineage(&resolved.frame_id).unwrap().len(), 3);
    }

    #[test]
    fn deepest_viewpoint_reverses_the_cached_transform() {
        let catalog = catalog();
        let root_position = point(40, 40);
        let selection = catalog.select_viewpoint(root_position).unwrap();
        let viewpoint = selection.selected;
        assert_eq!(selection.policy_id, SPATIAL_SELECTION_POLICY_ID);
        assert_eq!(selection.candidates.len(), 3);
        assert_eq!(viewpoint.frame_id.as_str(), "frame.test.grandchild");
        assert_eq!(viewpoint.template_id.as_str(), "template.test.cross");
        assert_eq!(viewpoint.scale.as_str(), "scale.district");
        assert_eq!(viewpoint.local_position, point(50, 50));
        assert_eq!(viewpoint.depth, 2);
    }

    #[test]
    fn inherited_paths_keep_local_shape_and_receive_root_coordinates() {
        let catalog = catalog();
        let child = catalog
            .resolve_path(&key("frame.test.child"), &key("path.round"))
            .unwrap();
        assert_eq!(child.geometry, SpatialPathGeometry::Round);
        assert_eq!(child.local_from, point(50, 10));
        assert_eq!(child.local_to, point(90, 50));
        assert_eq!(child.local_control_points, vec![point(80, 20)]);
        assert_eq!(child.root_from, point(40, 24));
        assert_eq!(child.root_to, point(56, 40));
        assert_eq!(child.root_control_points, vec![point(52, 28)]);

        let region = catalog
            .resolve_region(&key("frame.test.child"), &key("region.test"))
            .unwrap();
        assert_eq!(
            region.local_polygon,
            vec![point(50, 10), point(90, 50), point(50, 50)]
        );
        assert_eq!(
            region.root_polygon,
            vec![point(40, 24), point(56, 40), point(40, 40)]
        );
    }

    #[test]
    fn catalog_rejects_missing_parents_and_invalid_templates() {
        let mut catalog = SpatialInheritanceCatalog::new();
        let template = template();
        let template_id = template.id().clone();
        catalog.insert_template(template).unwrap();
        assert!(matches!(
            catalog.insert_frame(SpatialFrame::child(
                key("frame.test.orphan"),
                key("frame.test.missing"),
                template_id,
                key("scale.city"),
                NormalizedRect::new(point(10, 10), point(20, 20)).unwrap(),
            )),
            Err(SpatialInheritanceError::MissingParent(_))
        ));

        assert!(matches!(
            SpatialTemplate::new(
                key("template.test.invalid"),
                vec![SpatialAnchor {
                    id: key("anchor.center"),
                    position: point(50, 50),
                }],
                vec![SpatialPath {
                    id: key("path.invalid"),
                    from: key("anchor.center"),
                    to: key("anchor.missing"),
                    geometry: SpatialPathGeometry::Straight,
                    control_points: vec![],
                    band_width: coordinate(45, 1_000),
                }],
                vec![],
            ),
            Err(SpatialInheritanceError::MissingPathAnchor { .. })
        ));

        let invalid_deserialized_shape = SpatialTemplate {
            id: key("template.test.unvalidated"),
            anchors: vec![SpatialAnchor {
                id: key("anchor.center"),
                position: point(50, 50),
            }],
            paths: vec![SpatialPath {
                id: key("path.invalid"),
                from: key("anchor.center"),
                to: key("anchor.missing"),
                geometry: SpatialPathGeometry::Straight,
                control_points: vec![],
                band_width: coordinate(45, 1_000),
            }],
            regions: vec![],
        };
        assert!(matches!(
            catalog.insert_template(invalid_deserialized_shape),
            Err(SpatialInheritanceError::MissingPathAnchor { .. })
        ));
    }

    #[test]
    fn snapshot_replay_is_order_independent_and_digest_stable() {
        let catalog = catalog();
        let digest = catalog.canonical_digest().unwrap();
        let mut snapshot = catalog.snapshot();
        snapshot.frames.reverse();
        snapshot.templates.reverse();

        let replayed = SpatialInheritanceCatalog::replay_snapshot(snapshot).unwrap();
        assert_eq!(replayed.canonical_digest().unwrap(), digest);
        assert_eq!(
            replayed
                .resolve_anchor(&key("frame.test.grandchild"), &key("anchor.center"))
                .unwrap()
                .root_position,
            point(40, 40)
        );
    }

    #[test]
    fn snapshot_replay_rejects_schema_drift_cycles_and_missing_roots() {
        let mut unsupported = catalog().snapshot();
        unsupported.schema_version += 1;
        assert!(matches!(
            SpatialInheritanceCatalog::replay_snapshot(unsupported),
            Err(SpatialInheritanceError::UnsupportedCatalogSchema { .. })
        ));

        let mut missing_root = catalog().snapshot();
        missing_root
            .frames
            .retain(|frame| frame.parent_id().is_some());
        assert!(matches!(
            SpatialInheritanceCatalog::replay_snapshot(missing_root),
            Err(SpatialInheritanceError::MissingParent(_))
                | Err(SpatialInheritanceError::RootFrameRequired)
        ));

        let mut cycle = catalog().snapshot();
        let template_id = cycle.templates[0].id().clone();
        cycle.frames.extend([
            SpatialFrame::child(
                key("frame.test.cycle-a"),
                key("frame.test.cycle-b"),
                template_id.clone(),
                key("scale.site"),
                NormalizedRect::new(point(10, 10), point(20, 20)).unwrap(),
            ),
            SpatialFrame::child(
                key("frame.test.cycle-b"),
                key("frame.test.cycle-a"),
                template_id,
                key("scale.room"),
                NormalizedRect::new(point(10, 10), point(20, 20)).unwrap(),
            ),
        ]);
        assert!(matches!(
            SpatialInheritanceCatalog::replay_snapshot(cycle),
            Err(SpatialInheritanceError::FrameCycle(_))
        ));
    }

    #[test]
    fn overlapping_equal_depth_frames_use_stable_identity_not_insertion_order() {
        let template = template();
        let template_id = template.id().clone();
        let mut catalog = SpatialInheritanceCatalog::new();
        catalog.insert_template(template).unwrap();
        catalog
            .insert_frame(SpatialFrame::root(
                key("frame.test.root"),
                template_id.clone(),
                key("scale.world"),
            ))
            .unwrap();
        for id in ["frame.test.zeta", "frame.test.alpha"] {
            catalog
                .insert_frame(SpatialFrame::child(
                    key(id),
                    key("frame.test.root"),
                    template_id.clone(),
                    key("scale.city"),
                    NormalizedRect::new(point(20, 20), point(80, 80)).unwrap(),
                ))
                .unwrap();
        }

        let selection = catalog.select_viewpoint(point(50, 50)).unwrap();
        assert_eq!(selection.selected.frame_id.as_str(), "frame.test.alpha");
        assert_eq!(selection.candidates.len(), 3);
    }
}
