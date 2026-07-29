//! Hollow Grove bindings for the neutral spatial-inheritance kernel.
//!
//! The main world and every nested House map share one canonical local
//! template. Proper names and constitutional route identities stay in this
//! world layer; the kernel receives only generic frames, anchors, paths, and
//! transforms.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::composition::{
    NormalizedCoordinate, NormalizedPoint, NormalizedRect, ResolvedSpatialAnchor,
    ResolvedSpatialPath, ResolvedSpatialRegion, ScaleKey, SpatialAnchor, SpatialAnchorId,
    SpatialAuthorityId, SpatialCatalogSnapshot, SpatialEvidenceId, SpatialFrame, SpatialFrameId,
    SpatialInheritanceCatalog, SpatialInheritanceError, SpatialPath, SpatialPathGeometry,
    SpatialPathId, SpatialRegion, SpatialRegionId, SpatialRuleSetId, SpatialTemplate,
    SpatialTemplateId, SpatialViewpoint,
};
use crate::hollow_grove_contract::House;

use super::geography::ConstitutionalRouteId;
use super::route_network::{RouteGeometryClass, RouteNetwork, house_anchor};

pub const CANONICAL_SPATIAL_TEMPLATE_ID: &str = "template.hollow-grove.main-map";
pub const WORLD_SPATIAL_FRAME_ID: &str = "frame.world.hollow-grove";
pub const HOLLOW_GROVE_SPATIAL_RULE_SET_ID: &str = "ruleset.hollow-grove.spatial-inheritance-v1";
pub const HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION: u16 = 1;
pub const HOUSE_FRAME_RADIUS_PER_MILLE: u16 = 90;
pub const ROUTE_BAND_WIDTH_PER_MILLE: u16 = 45;
pub const AURA_RIDGE_EAST_PROJECTION_ID: &str = "projection.route.aura-ridge-east";
pub const SHARED_MAP_AUTHORITY_ID: &str = "authority.hollow-grove.shared-map";
pub const SPATIAL_CANON_EVIDENCE_ID: &str = "evidence.hollow-grove.spatial-inheritance-v1";
pub const GEOGRAPHY_EVIDENCE_ID: &str = "evidence.hollow-grove.constitutional-geography-v2";
pub const ROUTE_NETWORK_EVIDENCE_ID: &str = "evidence.hollow-grove.route-network-v1";
pub const SCREEN_MAP_EVIDENCE_ID: &str = "evidence.hueman.screen-map-v0-1-0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CanonicalSpatialAnchor {
    Ranina,
    AuraRidgeJunction,
    Stonebend,
    Sandmanor,
    Glaushouse,
    Flynt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CanonicalSpatialRegion {
    AuraField,
    AuraBeach,
    AuraBasin,
}

impl CanonicalSpatialRegion {
    pub const ALL: [Self; 3] = [Self::AuraField, Self::AuraBeach, Self::AuraBasin];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::AuraField => "region.aura-field",
            Self::AuraBeach => "region.aura-beach",
            Self::AuraBasin => "region.aura-basin",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HollowGroveSpatialLevel {
    World,
    City,
    District,
    Site,
    Room,
    Object,
    Detail,
}

impl HollowGroveSpatialLevel {
    pub const ALL_NAMED: [Self; 7] = [
        Self::World,
        Self::City,
        Self::District,
        Self::Site,
        Self::Room,
        Self::Object,
        Self::Detail,
    ];

    #[must_use]
    pub const fn scale_key(self) -> &'static str {
        match self {
            Self::World => "scale.world",
            Self::City => "scale.city",
            Self::District => "scale.district",
            Self::Site => "scale.site",
            Self::Room => "scale.room",
            Self::Object => "scale.object",
            Self::Detail => "scale.detail",
        }
    }

    #[must_use]
    pub const fn accepts_child(self, child: Self) -> bool {
        matches!(
            (self, child),
            (Self::World, Self::City)
                | (Self::City, Self::District)
                | (Self::District, Self::Site)
                | (Self::Site, Self::Room)
                | (Self::Room, Self::Object)
                | (Self::Object, Self::Detail)
                | (Self::Detail, Self::Detail)
        )
    }

    #[must_use]
    pub const fn accepts_depth(self, depth: usize) -> bool {
        match self {
            Self::World => depth == 0,
            Self::City => depth == 1,
            Self::District => depth == 2,
            Self::Site => depth == 3,
            Self::Room => depth == 4,
            Self::Object => depth == 5,
            Self::Detail => depth >= 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HollowGroveSpatialDeclaration {
    pub frame_id: SpatialFrameId,
    pub parent_frame_id: Option<SpatialFrameId>,
    pub template_id: SpatialTemplateId,
    pub level: HollowGroveSpatialLevel,
    pub bounds_in_parent: NormalizedRect,
    pub placement_authority: SpatialAuthorityId,
    pub evidence: Vec<SpatialEvidenceId>,
    pub source_anchor: Option<CanonicalSpatialAnchor>,
}

impl HollowGroveSpatialDeclaration {
    pub fn root(
        frame_id: SpatialFrameId,
        template_id: SpatialTemplateId,
        placement_authority: SpatialAuthorityId,
        evidence: Vec<SpatialEvidenceId>,
    ) -> Self {
        Self {
            frame_id,
            parent_frame_id: None,
            template_id,
            level: HollowGroveSpatialLevel::World,
            bounds_in_parent: NormalizedRect::full(),
            placement_authority,
            evidence,
            source_anchor: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn inherited(
        frame_id: SpatialFrameId,
        parent_frame_id: SpatialFrameId,
        template_id: SpatialTemplateId,
        level: HollowGroveSpatialLevel,
        bounds_in_parent: NormalizedRect,
        placement_authority: SpatialAuthorityId,
        evidence: Vec<SpatialEvidenceId>,
        source_anchor: Option<CanonicalSpatialAnchor>,
    ) -> Self {
        Self {
            frame_id,
            parent_frame_id: Some(parent_frame_id),
            template_id,
            level,
            bounds_in_parent,
            placement_authority,
            evidence,
            source_anchor,
        }
    }
}

impl CanonicalSpatialAnchor {
    pub const ALL: [Self; 6] = [
        Self::Ranina,
        Self::AuraRidgeJunction,
        Self::Stonebend,
        Self::Sandmanor,
        Self::Glaushouse,
        Self::Flynt,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Ranina => "anchor.ranina",
            Self::AuraRidgeJunction => "anchor.aura-ridge-junction",
            Self::Stonebend => "anchor.stonebend",
            Self::Sandmanor => "anchor.sandmanor",
            Self::Glaushouse => "anchor.glaushouse",
            Self::Flynt => "anchor.flynt",
        }
    }

    #[must_use]
    pub const fn house(self) -> Option<House> {
        match self {
            Self::Ranina | Self::AuraRidgeJunction => None,
            Self::Stonebend => Some(House::Stonebend),
            Self::Sandmanor => Some(House::Sandmanor),
            Self::Glaushouse => Some(House::Glaushouse),
            Self::Flynt => Some(House::Flynt),
        }
    }

    #[must_use]
    pub const fn for_house(house: House) -> Self {
        match house {
            House::Stonebend => Self::Stonebend,
            House::Sandmanor => Self::Sandmanor,
            House::Glaushouse => Self::Glaushouse,
            House::Flynt => Self::Flynt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HollowGroveSpatialScope {
    World,
    House(House),
}

impl HollowGroveSpatialScope {
    #[must_use]
    pub const fn scale_key(self) -> &'static str {
        self.level().scale_key()
    }

    #[must_use]
    pub const fn level(self) -> HollowGroveSpatialLevel {
        match self {
            Self::World => HollowGroveSpatialLevel::World,
            Self::House(_) => HollowGroveSpatialLevel::City,
        }
    }

    #[must_use]
    pub const fn frame_key(self) -> &'static str {
        match self {
            Self::World => WORLD_SPATIAL_FRAME_ID,
            Self::House(House::Stonebend) => "frame.city.stonebend",
            Self::House(House::Sandmanor) => "frame.city.sandmanor",
            Self::House(House::Glaushouse) => "frame.city.glaushouse",
            Self::House(House::Flynt) => "frame.city.flynt",
        }
    }
}

#[derive(Debug)]
pub struct HollowGroveSpatialMap {
    catalog: SpatialInheritanceCatalog,
    template_id: SpatialTemplateId,
    declarations: BTreeMap<SpatialFrameId, HollowGroveSpatialDeclaration>,
}

impl HollowGroveSpatialMap {
    #[must_use]
    pub fn catalog(&self) -> &SpatialInheritanceCatalog {
        &self.catalog
    }

    #[must_use]
    pub fn template_id(&self) -> &SpatialTemplateId {
        &self.template_id
    }

    #[must_use]
    pub fn declaration(&self, frame_id: &SpatialFrameId) -> Option<&HollowGroveSpatialDeclaration> {
        self.declarations.get(frame_id)
    }

    pub fn declarations(&self) -> impl ExactSizeIterator<Item = &HollowGroveSpatialDeclaration> {
        self.declarations.values()
    }

    pub fn resolve_anchor(
        &self,
        scope: HollowGroveSpatialScope,
        anchor: CanonicalSpatialAnchor,
    ) -> Result<ResolvedSpatialAnchor, SpatialInheritanceError> {
        self.catalog
            .resolve_anchor(&frame_id(scope)?, &anchor_id(anchor)?)
    }

    pub fn resolve_route(
        &self,
        scope: HollowGroveSpatialScope,
        route: ConstitutionalRouteId,
    ) -> Result<ResolvedSpatialPath, SpatialInheritanceError> {
        self.catalog
            .resolve_path(&frame_id(scope)?, &route_id(route)?)
    }

    pub fn resolve_region(
        &self,
        scope: HollowGroveSpatialScope,
        region: CanonicalSpatialRegion,
    ) -> Result<ResolvedSpatialRegion, SpatialInheritanceError> {
        self.catalog
            .resolve_region(&frame_id(scope)?, &region_id(region)?)
    }

    pub fn resolve_viewpoint(
        &self,
        scope: HollowGroveSpatialScope,
        local_position: NormalizedPoint,
    ) -> Result<SpatialViewpoint, SpatialInheritanceError> {
        self.catalog
            .resolve_viewpoint(&frame_id(scope)?, local_position)
    }

    #[must_use]
    pub fn deepest_viewpoint(&self, root_position: NormalizedPoint) -> Option<SpatialViewpoint> {
        self.catalog.deepest_viewpoint(root_position)
    }

    /// Atomically validates and publishes another inherited spatial frame.
    ///
    /// A failed declaration never becomes addressable. A correction after
    /// successful publication requires a new frame identity.
    pub fn publish_inheritance(
        &mut self,
        mut declaration: HollowGroveSpatialDeclaration,
    ) -> Result<(), HollowGroveSpatialError> {
        normalize_declaration(&mut declaration)?;
        self.validate_declaration(&declaration)?;

        let frame = match declaration.parent_frame_id.as_ref() {
            Some(parent_id) => SpatialFrame::child(
                declaration.frame_id.clone(),
                parent_id.clone(),
                declaration.template_id.clone(),
                ScaleKey::new(declaration.level.scale_key())?,
                declaration.bounds_in_parent,
            ),
            None => SpatialFrame::root(
                declaration.frame_id.clone(),
                declaration.template_id.clone(),
                ScaleKey::new(declaration.level.scale_key())?,
            ),
        };
        self.catalog.insert_frame(frame)?;
        self.declarations
            .insert(declaration.frame_id.clone(), declaration);
        Ok(())
    }

    pub fn inherit_nested_map(
        &mut self,
        declaration: HollowGroveSpatialDeclaration,
    ) -> Result<(), HollowGroveSpatialError> {
        self.publish_inheritance(declaration)
    }

    fn validate_declaration(
        &self,
        declaration: &HollowGroveSpatialDeclaration,
    ) -> Result<(), HollowGroveSpatialError> {
        if declaration.template_id != self.template_id {
            return Err(HollowGroveSpatialError::TemplateIdentityMismatch {
                expected: self.template_id.clone(),
                actual: declaration.template_id.clone(),
            });
        }
        if self.declarations.contains_key(&declaration.frame_id) {
            return Err(HollowGroveSpatialError::DuplicateDeclaration(
                declaration.frame_id.clone(),
            ));
        }
        if declaration.evidence.is_empty() {
            return Err(HollowGroveSpatialError::MissingPlacementEvidence(
                declaration.frame_id.clone(),
            ));
        }

        match declaration.parent_frame_id.as_ref() {
            None => {
                if declaration.level != HollowGroveSpatialLevel::World {
                    return Err(HollowGroveSpatialError::RootLevelRequired);
                }
                if declaration.bounds_in_parent != NormalizedRect::full() {
                    return Err(HollowGroveSpatialError::Inheritance(
                        SpatialInheritanceError::RootMustFillUnit,
                    ));
                }
                if declaration.source_anchor.is_some() {
                    return Err(HollowGroveSpatialError::RootCannotHaveSourceAnchor);
                }
            }
            Some(parent_id) => {
                if parent_id == &declaration.frame_id {
                    return Err(HollowGroveSpatialError::SelfParent(
                        declaration.frame_id.clone(),
                    ));
                }
                let parent = self.declarations.get(parent_id).ok_or_else(|| {
                    HollowGroveSpatialError::Inheritance(SpatialInheritanceError::MissingParent(
                        parent_id.clone(),
                    ))
                })?;
                if !parent.level.accepts_child(declaration.level) {
                    return Err(HollowGroveSpatialError::IllegalLevelTransition {
                        parent: parent.level,
                        child: declaration.level,
                    });
                }
                if let Some(source_anchor) = declaration.source_anchor
                    && declaration.bounds_in_parent.center() != anchor_position(source_anchor)?
                {
                    return Err(HollowGroveSpatialError::SourceAnchorPlacementMismatch {
                        frame: declaration.frame_id.clone(),
                        anchor: source_anchor,
                    });
                }
            }
        }
        Ok(())
    }
}

pub fn canonical_hollow_grove_spatial_map() -> Result<HollowGroveSpatialMap, HollowGroveSpatialError>
{
    let template = canonical_spatial_template()?;
    let template_id = template.id().clone();
    let mut catalog = SpatialInheritanceCatalog::with_rule_set(SpatialRuleSetId::new(
        HOLLOW_GROVE_SPATIAL_RULE_SET_ID,
    )?);
    catalog.insert_template(template)?;
    let mut map = HollowGroveSpatialMap {
        catalog,
        template_id: template_id.clone(),
        declarations: BTreeMap::new(),
    };
    map.publish_inheritance(canonical_frame_declaration(
        HollowGroveSpatialScope::World,
        &template_id,
    )?)?;

    for house in [
        House::Stonebend,
        House::Sandmanor,
        House::Glaushouse,
        House::Flynt,
    ] {
        map.publish_inheritance(canonical_frame_declaration(
            HollowGroveSpatialScope::House(house),
            &template_id,
        )?)?;
    }

    map.validate_conformance()?;
    Ok(map)
}

pub fn canonical_frame_declaration(
    scope: HollowGroveSpatialScope,
    template_id: &SpatialTemplateId,
) -> Result<HollowGroveSpatialDeclaration, HollowGroveSpatialError> {
    let authority = SpatialAuthorityId::new(SHARED_MAP_AUTHORITY_ID)?;
    let evidence = canonical_spatial_evidence()?;
    match scope {
        HollowGroveSpatialScope::World => Ok(HollowGroveSpatialDeclaration::root(
            frame_id(scope)?,
            template_id.clone(),
            authority,
            evidence,
        )),
        HollowGroveSpatialScope::House(house) => Ok(HollowGroveSpatialDeclaration::inherited(
            frame_id(scope)?,
            frame_id(HollowGroveSpatialScope::World)?,
            template_id.clone(),
            HollowGroveSpatialLevel::City,
            house_frame_bounds(house)?,
            authority,
            evidence,
            Some(CanonicalSpatialAnchor::for_house(house)),
        )),
    }
}

pub fn canonical_spatial_evidence() -> Result<Vec<SpatialEvidenceId>, HollowGroveSpatialError> {
    let mut evidence = [
        GEOGRAPHY_EVIDENCE_ID,
        ROUTE_NETWORK_EVIDENCE_ID,
        SCREEN_MAP_EVIDENCE_ID,
        SPATIAL_CANON_EVIDENCE_ID,
    ]
    .into_iter()
    .map(|id| SpatialEvidenceId::new(id).map_err(HollowGroveSpatialError::from))
    .collect::<Result<Vec<_>, _>>()?;
    evidence.sort();
    Ok(evidence)
}

fn normalize_declaration(
    declaration: &mut HollowGroveSpatialDeclaration,
) -> Result<(), HollowGroveSpatialError> {
    declaration.evidence.sort();
    if declaration.evidence.is_empty() {
        return Err(HollowGroveSpatialError::MissingPlacementEvidence(
            declaration.frame_id.clone(),
        ));
    }
    if declaration
        .evidence
        .windows(2)
        .any(|pair| pair[0] == pair[1])
    {
        return Err(HollowGroveSpatialError::DuplicatePlacementEvidence(
            declaration.frame_id.clone(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowGroveSpatialConformanceReport {
    pub schema_version: u16,
    pub rule_set: SpatialRuleSetId,
    pub catalog_digest: String,
    pub template_digest: String,
    pub anchor_count: usize,
    pub path_count: usize,
    pub region_count: usize,
    pub frame_count: usize,
    pub maximum_depth: usize,
}

impl HollowGroveSpatialMap {
    pub fn validate_conformance(
        &self,
    ) -> Result<HollowGroveSpatialConformanceReport, HollowGroveSpatialError> {
        self.catalog.validate_integrity()?;
        let expected_rule_set = SpatialRuleSetId::new(HOLLOW_GROVE_SPATIAL_RULE_SET_ID)?;
        if self.catalog.rule_set() != &expected_rule_set {
            return Err(HollowGroveSpatialError::RuleSetMismatch {
                expected: expected_rule_set,
                actual: self.catalog.rule_set().clone(),
            });
        }
        if self.catalog.template_count() != 1 {
            return Err(HollowGroveSpatialError::ExactlyOneTemplateRequired {
                actual: self.catalog.template_count(),
            });
        }
        if self.declarations.len() != self.catalog.frame_count() {
            return Err(HollowGroveSpatialError::DeclarationCatalogMismatch);
        }
        let canonical_template = canonical_spatial_template()?;
        let actual_template = self.catalog.template(&self.template_id).ok_or_else(|| {
            HollowGroveSpatialError::Inheritance(SpatialInheritanceError::MissingTemplate(
                self.template_id.clone(),
            ))
        })?;
        if actual_template != &canonical_template {
            return Err(HollowGroveSpatialError::CanonicalTemplateDrift);
        }

        let mut maximum_depth = 0usize;
        for declaration in self.declarations.values() {
            if declaration.evidence.is_empty() {
                return Err(HollowGroveSpatialError::MissingPlacementEvidence(
                    declaration.frame_id.clone(),
                ));
            }
            let unique_evidence = declaration
                .evidence
                .iter()
                .collect::<BTreeSet<&SpatialEvidenceId>>();
            if unique_evidence.len() != declaration.evidence.len() {
                return Err(HollowGroveSpatialError::DuplicatePlacementEvidence(
                    declaration.frame_id.clone(),
                ));
            }
            match declaration.parent_frame_id.as_ref() {
                None => {
                    if declaration.level != HollowGroveSpatialLevel::World {
                        return Err(HollowGroveSpatialError::RootLevelRequired);
                    }
                    if declaration.source_anchor.is_some() {
                        return Err(HollowGroveSpatialError::RootCannotHaveSourceAnchor);
                    }
                }
                Some(parent_id) => {
                    let parent = self.declarations.get(parent_id).ok_or_else(|| {
                        HollowGroveSpatialError::Inheritance(
                            SpatialInheritanceError::MissingParent(parent_id.clone()),
                        )
                    })?;
                    if !parent.level.accepts_child(declaration.level) {
                        return Err(HollowGroveSpatialError::IllegalLevelTransition {
                            parent: parent.level,
                            child: declaration.level,
                        });
                    }
                    if let Some(source_anchor) = declaration.source_anchor
                        && declaration.bounds_in_parent.center() != anchor_position(source_anchor)?
                    {
                        return Err(HollowGroveSpatialError::SourceAnchorPlacementMismatch {
                            frame: declaration.frame_id.clone(),
                            anchor: source_anchor,
                        });
                    }
                }
            }
            let frame = self.catalog.frame(&declaration.frame_id).ok_or_else(|| {
                HollowGroveSpatialError::Inheritance(SpatialInheritanceError::MissingFrame(
                    declaration.frame_id.clone(),
                ))
            })?;
            if frame.parent_id() != declaration.parent_frame_id.as_ref()
                || frame.template_id() != &declaration.template_id
                || frame.scale().as_str() != declaration.level.scale_key()
                || frame.bounds_in_parent() != declaration.bounds_in_parent
            {
                return Err(HollowGroveSpatialError::PublishedDeclarationDrift(
                    declaration.frame_id.clone(),
                ));
            }
            let depth = self.catalog.frame_depth(&declaration.frame_id)?;
            maximum_depth = maximum_depth.max(depth);
            if !declaration.level.accepts_depth(depth) {
                return Err(HollowGroveSpatialError::LevelDepthMismatch {
                    frame: declaration.frame_id.clone(),
                    level: declaration.level,
                    depth,
                });
            }
        }

        let world = canonical_frame_declaration(HollowGroveSpatialScope::World, &self.template_id)?;
        self.require_canonical_declaration(&world)?;
        for house in [
            House::Stonebend,
            House::Sandmanor,
            House::Glaushouse,
            House::Flynt,
        ] {
            let expected = canonical_frame_declaration(
                HollowGroveSpatialScope::House(house),
                &self.template_id,
            )?;
            self.require_canonical_declaration(&expected)?;
        }

        Ok(HollowGroveSpatialConformanceReport {
            schema_version: HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION,
            rule_set: self.catalog.rule_set().clone(),
            catalog_digest: self.catalog.canonical_digest()?,
            template_digest: self.catalog.template_digest(&self.template_id)?,
            anchor_count: actual_template.anchors().len(),
            path_count: actual_template.paths().len(),
            region_count: actual_template.regions().len(),
            frame_count: self.catalog.frame_count(),
            maximum_depth,
        })
    }

    fn require_canonical_declaration(
        &self,
        expected: &HollowGroveSpatialDeclaration,
    ) -> Result<(), HollowGroveSpatialError> {
        let actual = self.declarations.get(&expected.frame_id).ok_or_else(|| {
            HollowGroveSpatialError::MissingCanonicalFrame(expected.frame_id.clone())
        })?;
        if actual != expected {
            return Err(HollowGroveSpatialError::CanonicalFrameDrift(
                expected.frame_id.clone(),
            ));
        }
        Ok(())
    }

    pub fn archive(&self) -> Result<HollowGroveSpatialArchive, HollowGroveSpatialError> {
        self.validate_conformance()?;
        Ok(HollowGroveSpatialArchive {
            schema_version: HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION,
            catalog: self.catalog.snapshot(),
            declarations: self.declarations.values().cloned().collect(),
        })
    }

    pub fn replay_archive(
        mut archive: HollowGroveSpatialArchive,
    ) -> Result<Self, HollowGroveSpatialError> {
        if archive.schema_version != HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION {
            return Err(HollowGroveSpatialError::UnsupportedArchiveSchema {
                expected: HOLLOW_GROVE_SPATIAL_ARCHIVE_VERSION,
                actual: archive.schema_version,
            });
        }
        archive
            .declarations
            .sort_by(|left, right| left.frame_id.cmp(&right.frame_id));
        let catalog = SpatialInheritanceCatalog::replay_snapshot(archive.catalog)?;
        let template_id = SpatialTemplateId::new(CANONICAL_SPATIAL_TEMPLATE_ID)?;
        let mut declarations = BTreeMap::new();
        for mut declaration in archive.declarations {
            normalize_declaration(&mut declaration)?;
            let frame_id = declaration.frame_id.clone();
            if declarations.insert(frame_id.clone(), declaration).is_some() {
                return Err(HollowGroveSpatialError::DuplicateDeclaration(frame_id));
            }
        }
        let map = Self {
            catalog,
            template_id,
            declarations,
        };
        map.validate_conformance()?;
        Ok(map)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HollowGroveSpatialArchive {
    pub schema_version: u16,
    pub catalog: SpatialCatalogSnapshot,
    pub declarations: Vec<HollowGroveSpatialDeclaration>,
}

pub fn encode_hollow_grove_spatial_archive(
    map: &HollowGroveSpatialMap,
) -> Result<Vec<u8>, HollowGroveSpatialArchiveError> {
    let archive = map.archive()?;
    serde_json::to_vec(&archive).map_err(HollowGroveSpatialArchiveError::Json)
}

pub fn decode_hollow_grove_spatial_archive(
    bytes: &[u8],
) -> Result<HollowGroveSpatialMap, HollowGroveSpatialArchiveError> {
    let archive = serde_json::from_slice(bytes).map_err(HollowGroveSpatialArchiveError::Json)?;
    HollowGroveSpatialMap::replay_archive(archive).map_err(Into::into)
}

pub fn migrate_hollow_grove_spatial_archive(
    bytes: &[u8],
) -> Result<Vec<u8>, HollowGroveSpatialArchiveError> {
    encode_hollow_grove_spatial_archive(&decode_hollow_grove_spatial_archive(bytes)?)
}

pub fn hollow_grove_spatial_replay_digest(
    map: &HollowGroveSpatialMap,
) -> Result<String, HollowGroveSpatialArchiveError> {
    let bytes = encode_hollow_grove_spatial_archive(map)?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-hollow-grove-spatial-v1:{hash:016x}"))
}

pub fn build_hollow_grove_spatial_witness(
    map: &HollowGroveSpatialMap,
) -> Result<String, HollowGroveSpatialArchiveError> {
    let report = map.validate_conformance()?;
    let replay_digest = hollow_grove_spatial_replay_digest(map)?;
    let template = map
        .catalog()
        .template(map.template_id())
        .expect("conformant spatial map retains its canonical template");
    let mut output = format!(
        "# Hollow Grove Spatial Inheritance Witness\n\n\
         - schema_version: {}\n\
         - rule_set: {}\n\
         - selection_policy: {}\n\
         - catalog_digest: {}\n\
         - template_digest: {}\n\
         - replay_digest: {}\n\
         - anchor_count: {}\n\
         - path_count: {}\n\
         - region_count: {}\n\
         - frame_count: {}\n\
         - maximum_depth: {}\n\n\
         ## Canonical Local Anchors\n\n",
        report.schema_version,
        report.rule_set,
        crate::composition::SPATIAL_SELECTION_POLICY_ID,
        report.catalog_digest,
        report.template_digest,
        replay_digest,
        report.anchor_count,
        report.path_count,
        report.region_count,
        report.frame_count,
        report.maximum_depth,
    );
    for anchor in template.anchors() {
        output.push_str(&format!(
            "- {}: ({}, {})\n",
            anchor.id,
            anchor.position.x.parts(),
            anchor.position.y.parts()
        ));
    }
    output.push_str("\n## Canonical Local Paths\n\n");
    for path in template.paths() {
        let controls = path
            .control_points
            .iter()
            .map(|point| format!("({}, {})", point.x.parts(), point.y.parts()))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!(
            "- {}: {} -> {} [{:?}] band={} controls=[{}]\n",
            path.id,
            path.from,
            path.to,
            path.geometry,
            path.band_width.parts(),
            controls,
        ));
    }
    output.push_str("\n## Canonical Local Regions\n\n");
    for region in template.regions() {
        let polygon = region
            .polygon
            .iter()
            .map(|point| format!("({}, {})", point.x.parts(), point.y.parts()))
            .collect::<Vec<_>>()
            .join(" -> ");
        output.push_str(&format!("- {}: {}\n", region.id, polygon));
    }
    output.push_str("\n## Published Frames\n\n");
    for declaration in map.declarations() {
        let depth = map
            .catalog()
            .frame_depth(&declaration.frame_id)
            .map_err(HollowGroveSpatialError::from)?;
        let parent = declaration
            .parent_frame_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| String::from("(root)"));
        let evidence = declaration
            .evidence
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!(
            "- {}: parent={} level={:?} depth={} template={} authority={} evidence=[{}]\n",
            declaration.frame_id,
            parent,
            declaration.level,
            depth,
            declaration.template_id,
            declaration.placement_authority,
            evidence,
        ));
    }
    Ok(output)
}

pub fn write_hollow_grove_spatial_archive(
    path: impl AsRef<Path>,
    map: &HollowGroveSpatialMap,
) -> Result<(), HollowGroveSpatialArchiveError> {
    crate::artifact_io::write_bytes_atomically(
        path.as_ref(),
        &encode_hollow_grove_spatial_archive(map)?,
    )?;
    Ok(())
}

pub fn read_hollow_grove_spatial_archive(
    path: impl AsRef<Path>,
) -> Result<HollowGroveSpatialMap, HollowGroveSpatialArchiveError> {
    decode_hollow_grove_spatial_archive(&fs::read(path)?)
}

pub fn canonical_spatial_template() -> Result<SpatialTemplate, HollowGroveSpatialError> {
    let anchors = CanonicalSpatialAnchor::ALL
        .into_iter()
        .map(|anchor| {
            Ok(SpatialAnchor {
                id: anchor_id(anchor)?,
                position: anchor_position(anchor)?,
            })
        })
        .collect::<Result<Vec<_>, SpatialInheritanceError>>()?;

    let network = RouteNetwork::canonical()
        .map_err(|error| HollowGroveSpatialError::RouteNetwork(error.to_string()))?;
    let mut paths = network
        .segments()
        .iter()
        .map(|segment| {
            Ok(SpatialPath {
                id: route_id(segment.route)?,
                from: anchor_id(CanonicalSpatialAnchor::for_house(segment.endpoints[0]))?,
                to: anchor_id(CanonicalSpatialAnchor::for_house(segment.endpoints[1]))?,
                geometry: match segment.geometry {
                    RouteGeometryClass::Straight => SpatialPathGeometry::Straight,
                    RouteGeometryClass::Round => SpatialPathGeometry::Round,
                },
                control_points: canonical_route_controls(segment.route)?,
                band_width: NormalizedCoordinate::from_ratio(
                    u64::from(ROUTE_BAND_WIDTH_PER_MILLE),
                    1_000,
                )?,
            })
        })
        .collect::<Result<Vec<_>, SpatialInheritanceError>>()?;
    paths.push(SpatialPath {
        id: SpatialPathId::new(AURA_RIDGE_EAST_PROJECTION_ID)?,
        from: anchor_id(CanonicalSpatialAnchor::AuraRidgeJunction)?,
        to: anchor_id(CanonicalSpatialAnchor::Sandmanor)?,
        geometry: SpatialPathGeometry::Straight,
        control_points: vec![],
        band_width: NormalizedCoordinate::from_ratio(u64::from(ROUTE_BAND_WIDTH_PER_MILLE), 1_000)?,
    });

    let regions = CanonicalSpatialRegion::ALL
        .into_iter()
        .map(|region| {
            Ok(SpatialRegion {
                id: region_id(region)?,
                polygon: canonical_region_polygon(region)?,
            })
        })
        .collect::<Result<Vec<_>, SpatialInheritanceError>>()?;

    Ok(SpatialTemplate::new(
        SpatialTemplateId::new(CANONICAL_SPATIAL_TEMPLATE_ID)?,
        anchors,
        paths,
        regions,
    )?)
}

pub fn frame_id(scope: HollowGroveSpatialScope) -> Result<SpatialFrameId, SpatialInheritanceError> {
    Ok(SpatialFrameId::new(scope.frame_key())?)
}

pub fn anchor_id(
    anchor: CanonicalSpatialAnchor,
) -> Result<SpatialAnchorId, SpatialInheritanceError> {
    Ok(SpatialAnchorId::new(anchor.stable_id())?)
}

pub fn route_id(route: ConstitutionalRouteId) -> Result<SpatialPathId, SpatialInheritanceError> {
    Ok(SpatialPathId::new(route.stable_id())?)
}

pub fn region_id(
    region: CanonicalSpatialRegion,
) -> Result<SpatialRegionId, SpatialInheritanceError> {
    Ok(SpatialRegionId::new(region.stable_id())?)
}

fn canonical_route_controls(
    route: ConstitutionalRouteId,
) -> Result<Vec<NormalizedPoint>, SpatialInheritanceError> {
    use ConstitutionalRouteId::{
        AuraRidge, AuraWay, BasinMotorspeedway, Boardwalk, CurrentSea, CurrentSeanad, Glausbahn,
        MntAura, Riptide, StairwayToHeaven,
    };
    let control = match route {
        AuraRidge | AuraWay | CurrentSea | Boardwalk | BasinMotorspeedway => return Ok(vec![]),
        MntAura => (690, 200),
        CurrentSeanad => (730, 800),
        Riptide => (310, 800),
        StairwayToHeaven => (310, 200),
        // The inner reciprocal curve remains distinct from the outer Seanad
        // curve despite sharing its two House endpoints.
        Glausbahn => (610, 680),
    };
    Ok(vec![normalized_per_mille(control.0, control.1)?])
}

fn canonical_region_polygon(
    region: CanonicalSpatialRegion,
) -> Result<Vec<NormalizedPoint>, SpatialInheritanceError> {
    let anchors = match region {
        CanonicalSpatialRegion::AuraField => [
            CanonicalSpatialAnchor::Stonebend,
            CanonicalSpatialAnchor::Sandmanor,
            CanonicalSpatialAnchor::Ranina,
        ],
        CanonicalSpatialRegion::AuraBeach => [
            CanonicalSpatialAnchor::Sandmanor,
            CanonicalSpatialAnchor::Glaushouse,
            CanonicalSpatialAnchor::Ranina,
        ],
        CanonicalSpatialRegion::AuraBasin => [
            CanonicalSpatialAnchor::Stonebend,
            CanonicalSpatialAnchor::Flynt,
            CanonicalSpatialAnchor::Glaushouse,
        ],
    };
    anchors
        .into_iter()
        .map(anchor_position)
        .collect::<Result<Vec<_>, _>>()
}

pub fn anchor_position(
    anchor: CanonicalSpatialAnchor,
) -> Result<NormalizedPoint, SpatialInheritanceError> {
    match anchor.house() {
        None => Ok(NormalizedPoint::center()),
        Some(house) => {
            let position = house_anchor(house);
            normalized_per_mille(position.x_per_mille, position.y_per_mille)
        }
    }
}

pub fn house_frame_bounds(house: House) -> Result<NormalizedRect, SpatialInheritanceError> {
    let center = house_anchor(house);
    let radius = HOUSE_FRAME_RADIUS_PER_MILLE;
    NormalizedRect::new(
        normalized_per_mille(center.x_per_mille - radius, center.y_per_mille - radius)?,
        normalized_per_mille(center.x_per_mille + radius, center.y_per_mille + radius)?,
    )
}

fn normalized_per_mille(
    x_per_mille: u16,
    y_per_mille: u16,
) -> Result<NormalizedPoint, SpatialInheritanceError> {
    Ok(NormalizedPoint::new(
        NormalizedCoordinate::from_ratio(u64::from(x_per_mille), 1_000)?,
        NormalizedCoordinate::from_ratio(u64::from(y_per_mille), 1_000)?,
    ))
}

#[derive(Debug)]
pub enum HollowGroveSpatialError {
    Inheritance(SpatialInheritanceError),
    RouteNetwork(String),
    UnsupportedArchiveSchema {
        expected: u16,
        actual: u16,
    },
    RuleSetMismatch {
        expected: SpatialRuleSetId,
        actual: SpatialRuleSetId,
    },
    ExactlyOneTemplateRequired {
        actual: usize,
    },
    TemplateIdentityMismatch {
        expected: SpatialTemplateId,
        actual: SpatialTemplateId,
    },
    DuplicateDeclaration(SpatialFrameId),
    MissingPlacementEvidence(SpatialFrameId),
    DuplicatePlacementEvidence(SpatialFrameId),
    RootLevelRequired,
    RootCannotHaveSourceAnchor,
    SourceAnchorPlacementMismatch {
        frame: SpatialFrameId,
        anchor: CanonicalSpatialAnchor,
    },
    SelfParent(SpatialFrameId),
    IllegalLevelTransition {
        parent: HollowGroveSpatialLevel,
        child: HollowGroveSpatialLevel,
    },
    DeclarationCatalogMismatch,
    PublishedDeclarationDrift(SpatialFrameId),
    LevelDepthMismatch {
        frame: SpatialFrameId,
        level: HollowGroveSpatialLevel,
        depth: usize,
    },
    MissingCanonicalFrame(SpatialFrameId),
    CanonicalFrameDrift(SpatialFrameId),
    CanonicalTemplateDrift,
}

impl From<SpatialInheritanceError> for HollowGroveSpatialError {
    fn from(value: SpatialInheritanceError) -> Self {
        Self::Inheritance(value)
    }
}

impl From<crate::composition::StableKeyError> for HollowGroveSpatialError {
    fn from(value: crate::composition::StableKeyError) -> Self {
        Self::Inheritance(SpatialInheritanceError::StableKey(value))
    }
}

impl fmt::Display for HollowGroveSpatialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inheritance(error) => error.fmt(formatter),
            Self::RouteNetwork(error) => {
                write!(formatter, "canonical route network is invalid: {error}")
            }
            Self::UnsupportedArchiveSchema { expected, actual } => write!(
                formatter,
                "unsupported Hollow Grove spatial archive schema {actual}; expected {expected}"
            ),
            Self::RuleSetMismatch { expected, actual } => write!(
                formatter,
                "spatial rule-set mismatch: expected {expected}, found {actual}"
            ),
            Self::ExactlyOneTemplateRequired { actual } => write!(
                formatter,
                "Hollow Grove spatial canon requires exactly one template; found {actual}"
            ),
            Self::TemplateIdentityMismatch { expected, actual } => write!(
                formatter,
                "spatial declaration uses template {actual}; expected {expected}"
            ),
            Self::DuplicateDeclaration(frame) => {
                write!(formatter, "duplicate spatial declaration for {frame}")
            }
            Self::MissingPlacementEvidence(frame) => {
                write!(
                    formatter,
                    "spatial declaration {frame} has no placement evidence"
                )
            }
            Self::DuplicatePlacementEvidence(frame) => {
                write!(
                    formatter,
                    "spatial declaration {frame} repeats placement evidence"
                )
            }
            Self::RootLevelRequired => {
                formatter.write_str("a parentless spatial declaration must use the world level")
            }
            Self::RootCannotHaveSourceAnchor => {
                formatter.write_str("the root spatial declaration cannot inherit from an anchor")
            }
            Self::SourceAnchorPlacementMismatch { frame, anchor } => write!(
                formatter,
                "spatial frame {frame} is not centered on its declared source anchor {anchor:?}"
            ),
            Self::SelfParent(frame) => {
                write!(
                    formatter,
                    "spatial declaration {frame} cannot parent itself"
                )
            }
            Self::IllegalLevelTransition { parent, child } => write!(
                formatter,
                "illegal spatial level transition from {parent:?} to {child:?}"
            ),
            Self::DeclarationCatalogMismatch => formatter
                .write_str("published spatial declarations do not match the kernel frame catalog"),
            Self::PublishedDeclarationDrift(frame) => write!(
                formatter,
                "published spatial frame {frame} differs from its immutable declaration"
            ),
            Self::LevelDepthMismatch {
                frame,
                level,
                depth,
            } => write!(
                formatter,
                "spatial frame {frame} declares level {level:?} at incompatible depth {depth}"
            ),
            Self::MissingCanonicalFrame(frame) => {
                write!(formatter, "canonical spatial frame is missing: {frame}")
            }
            Self::CanonicalFrameDrift(frame) => {
                write!(
                    formatter,
                    "canonical spatial frame declaration drifted: {frame}"
                )
            }
            Self::CanonicalTemplateDrift => formatter
                .write_str("canonical spatial template differs from Route Network authority"),
        }
    }
}

impl std::error::Error for HollowGroveSpatialError {}

#[derive(Debug)]
pub enum HollowGroveSpatialArchiveError {
    Io(io::Error),
    Json(serde_json::Error),
    Contract(HollowGroveSpatialError),
}

impl From<io::Error> for HollowGroveSpatialArchiveError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<HollowGroveSpatialError> for HollowGroveSpatialArchiveError {
    fn from(value: HollowGroveSpatialError) -> Self {
        Self::Contract(value)
    }
}

impl fmt::Display for HollowGroveSpatialArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "spatial archive I/O error: {error}"),
            Self::Json(error) => write!(formatter, "spatial archive JSON error: {error}"),
            Self::Contract(error) => write!(formatter, "spatial archive contract error: {error}"),
        }
    }
}

impl std::error::Error for HollowGroveSpatialArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Contract(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_and_all_house_cities_share_one_map_template() {
        let map = canonical_hollow_grove_spatial_map().unwrap();
        assert_eq!(map.catalog().template_count(), 1);
        assert_eq!(map.catalog().frame_count(), 5);
        for scope in [
            HollowGroveSpatialScope::World,
            HollowGroveSpatialScope::House(House::Stonebend),
            HollowGroveSpatialScope::House(House::Sandmanor),
            HollowGroveSpatialScope::House(House::Glaushouse),
            HollowGroveSpatialScope::House(House::Flynt),
        ] {
            assert_eq!(
                map.catalog()
                    .frame(&frame_id(scope).unwrap())
                    .unwrap()
                    .template_id(),
                map.template_id()
            );
        }
    }

    #[test]
    fn every_scope_keeps_identical_local_anchors_but_distinct_root_views() {
        let map = canonical_hollow_grove_spatial_map().unwrap();
        let world = map
            .resolve_anchor(
                HollowGroveSpatialScope::World,
                CanonicalSpatialAnchor::Sandmanor,
            )
            .unwrap();
        let stonebend = map
            .resolve_anchor(
                HollowGroveSpatialScope::House(House::Stonebend),
                CanonicalSpatialAnchor::Sandmanor,
            )
            .unwrap();
        assert_eq!(world.local_position, stonebend.local_position);
        assert_ne!(world.root_position, stonebend.root_position);
        assert_eq!(world.depth, 0);
        assert_eq!(stonebend.depth, 1);
    }

    #[test]
    fn mt_aura_inherits_as_the_same_round_route_at_every_city_scale() {
        let map = canonical_hollow_grove_spatial_map().unwrap();
        let world = map
            .resolve_route(
                HollowGroveSpatialScope::World,
                ConstitutionalRouteId::MntAura,
            )
            .unwrap();
        assert_eq!(world.geometry, SpatialPathGeometry::Round);
        for house in [
            House::Stonebend,
            House::Sandmanor,
            House::Glaushouse,
            House::Flynt,
        ] {
            let inherited = map
                .resolve_route(
                    HollowGroveSpatialScope::House(house),
                    ConstitutionalRouteId::MntAura,
                )
                .unwrap();
            assert_eq!(inherited.geometry, world.geometry);
            assert_eq!(inherited.local_from, world.local_from);
            assert_eq!(inherited.local_to, world.local_to);
            assert_ne!(inherited.root_from, world.root_from);
            assert_ne!(inherited.root_to, world.root_to);
        }
    }

    #[test]
    fn arbitrary_deeper_maps_reuse_the_template_and_cached_root_projection() {
        let mut map = canonical_hollow_grove_spatial_map().unwrap();
        let nested_id = SpatialFrameId::new("frame.district.stonebend.fixture").unwrap();
        map.inherit_nested_map(HollowGroveSpatialDeclaration::inherited(
            nested_id.clone(),
            frame_id(HollowGroveSpatialScope::House(House::Stonebend)).unwrap(),
            map.template_id().clone(),
            HollowGroveSpatialLevel::District,
            NormalizedRect::new(
                normalized_per_mille(250, 250).unwrap(),
                normalized_per_mille(750, 750).unwrap(),
            )
            .unwrap(),
            SpatialAuthorityId::new(SHARED_MAP_AUTHORITY_ID).unwrap(),
            canonical_spatial_evidence().unwrap(),
            None,
        ))
        .unwrap();

        let center = map
            .catalog()
            .resolve_anchor(
                &nested_id,
                &anchor_id(CanonicalSpatialAnchor::Ranina).unwrap(),
            )
            .unwrap();
        assert_eq!(center.depth, 2);
        assert_eq!(map.catalog().lineage(&nested_id).unwrap().len(), 3);
        assert_eq!(
            map.deepest_viewpoint(center.root_position)
                .unwrap()
                .frame_id,
            nested_id
        );
        let report = map.validate_conformance().unwrap();
        assert_eq!(report.maximum_depth, 2);
        assert_eq!(report.frame_count, 6);
    }

    #[test]
    fn canonical_template_contains_all_routes_with_network_geometry() {
        let template = canonical_spatial_template().unwrap();
        assert_eq!(template.anchors().len(), 6);
        assert_eq!(template.paths().len(), 11);
        assert_eq!(template.regions().len(), 3);
        for route in ConstitutionalRouteId::ALL {
            let path = template.path(&route_id(route).unwrap()).unwrap();
            let expected = match super::super::route_network::canonical_route_geometry(route) {
                RouteGeometryClass::Straight => SpatialPathGeometry::Straight,
                RouteGeometryClass::Round => SpatialPathGeometry::Round,
            };
            assert_eq!(path.geometry, expected);
            assert_eq!(
                path.control_points.len(),
                usize::from(expected == SpatialPathGeometry::Round)
            );
            assert_eq!(
                path.band_width,
                NormalizedCoordinate::from_ratio(u64::from(ROUTE_BAND_WIDTH_PER_MILLE), 1_000)
                    .unwrap()
            );
        }
        assert!(
            template
                .path(&SpatialPathId::new(AURA_RIDGE_EAST_PROJECTION_ID).unwrap())
                .is_some()
        );
    }

    #[test]
    fn archive_replay_is_deterministic_and_preserves_provenance() {
        let map = canonical_hollow_grove_spatial_map().unwrap();
        let encoded = encode_hollow_grove_spatial_archive(&map).unwrap();
        let replayed = decode_hollow_grove_spatial_archive(&encoded).unwrap();
        assert_eq!(
            hollow_grove_spatial_replay_digest(&replayed).unwrap(),
            hollow_grove_spatial_replay_digest(&map).unwrap()
        );
        assert_eq!(
            encode_hollow_grove_spatial_archive(&replayed).unwrap(),
            encoded
        );
        let city = replayed
            .declaration(&frame_id(HollowGroveSpatialScope::House(House::Stonebend)).unwrap())
            .unwrap();
        assert_eq!(city.placement_authority.as_str(), SHARED_MAP_AUTHORITY_ID);
        assert_eq!(city.evidence, canonical_spatial_evidence().unwrap());
    }

    #[test]
    fn archive_file_restart_and_migration_preserve_the_canonical_digest() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!("hollow-grove-spatial-archive-{nonce}"));
        let path = directory.join("spatial.json");
        fs::create_dir_all(&directory).unwrap();

        let map = canonical_hollow_grove_spatial_map().unwrap();
        let digest = hollow_grove_spatial_replay_digest(&map).unwrap();
        write_hollow_grove_spatial_archive(&path, &map).unwrap();
        let restarted = read_hollow_grove_spatial_archive(&path).unwrap();
        assert_eq!(
            hollow_grove_spatial_replay_digest(&restarted).unwrap(),
            digest
        );
        let migrated = migrate_hollow_grove_spatial_archive(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            hollow_grove_spatial_replay_digest(
                &decode_hollow_grove_spatial_archive(&migrated).unwrap()
            )
            .unwrap(),
            digest
        );

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn spatial_witness_exposes_rule_digests_topology_and_provenance() {
        let map = canonical_hollow_grove_spatial_map().unwrap();
        let witness = build_hollow_grove_spatial_witness(&map).unwrap();
        for required in [
            "# Hollow Grove Spatial Inheritance Witness",
            HOLLOW_GROVE_SPATIAL_RULE_SET_ID,
            crate::composition::SPATIAL_SELECTION_POLICY_ID,
            "fnv1a64-spatial-v1:",
            "fnv1a64-spatial-template-v1:",
            "fnv1a64-hollow-grove-spatial-v1:",
            "geography.route.mnt-aura",
            "frame.city.stonebend",
            SHARED_MAP_AUTHORITY_ID,
            ROUTE_NETWORK_EVIDENCE_ID,
        ] {
            assert!(witness.contains(required), "witness missing {required}");
        }
    }

    #[test]
    fn publication_rejects_missing_evidence_skipped_levels_and_identity_reuse() {
        let mut map = canonical_hollow_grove_spatial_map().unwrap();
        let city = frame_id(HollowGroveSpatialScope::House(House::Stonebend)).unwrap();
        let template_id = map.template_id().clone();
        let declaration = |id: &str, level, evidence| {
            HollowGroveSpatialDeclaration::inherited(
                SpatialFrameId::new(id).unwrap(),
                city.clone(),
                template_id.clone(),
                level,
                NormalizedRect::new(
                    normalized_per_mille(250, 250).unwrap(),
                    normalized_per_mille(750, 750).unwrap(),
                )
                .unwrap(),
                SpatialAuthorityId::new(SHARED_MAP_AUTHORITY_ID).unwrap(),
                evidence,
                None,
            )
        };

        assert!(matches!(
            map.inherit_nested_map(declaration(
                "frame.district.missing-evidence",
                HollowGroveSpatialLevel::District,
                Vec::new(),
            )),
            Err(HollowGroveSpatialError::MissingPlacementEvidence(_))
        ));
        assert!(matches!(
            map.inherit_nested_map(declaration(
                "frame.site.skipped-district",
                HollowGroveSpatialLevel::Site,
                canonical_spatial_evidence().unwrap(),
            )),
            Err(HollowGroveSpatialError::IllegalLevelTransition { .. })
        ));
        let repeated = canonical_spatial_evidence().unwrap()[0].clone();
        assert!(matches!(
            map.inherit_nested_map(declaration(
                "frame.district.duplicate-evidence",
                HollowGroveSpatialLevel::District,
                vec![repeated.clone(), repeated],
            )),
            Err(HollowGroveSpatialError::DuplicatePlacementEvidence(_))
        ));
        let mismatched_anchor = HollowGroveSpatialDeclaration::inherited(
            SpatialFrameId::new("frame.district.wrong-source-anchor").unwrap(),
            city.clone(),
            template_id.clone(),
            HollowGroveSpatialLevel::District,
            NormalizedRect::new(
                normalized_per_mille(250, 250).unwrap(),
                normalized_per_mille(750, 750).unwrap(),
            )
            .unwrap(),
            SpatialAuthorityId::new(SHARED_MAP_AUTHORITY_ID).unwrap(),
            canonical_spatial_evidence().unwrap(),
            Some(CanonicalSpatialAnchor::Stonebend),
        );
        assert!(matches!(
            map.inherit_nested_map(mismatched_anchor),
            Err(HollowGroveSpatialError::SourceAnchorPlacementMismatch { .. })
        ));

        let valid = declaration(
            "frame.district.unique",
            HollowGroveSpatialLevel::District,
            canonical_spatial_evidence().unwrap(),
        );
        map.inherit_nested_map(valid.clone()).unwrap();
        assert!(matches!(
            map.inherit_nested_map(valid),
            Err(HollowGroveSpatialError::DuplicateDeclaration(_))
        ));
    }
}
