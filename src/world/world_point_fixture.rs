//! Golden deterministic fixture for the world-scale Hollow Grove Point.

use std::collections::BTreeSet;

use crate::composition::{
    AxisHandedness, CompositionRecordId, ExpandedPointField, FieldId, OrientedPoint,
    PhysicalExtent, PhysicalPosition, PointCenterId, PointId, PointInversion, PointScaling,
    PolarityAxis, PoleId, PositiveScaleFactor, ScaleKey, SpatialAuthorityId, SpatialEvidenceId,
    SpatialRegionId, lawfully_scale_point,
};

use super::world_point::{
    CENTRAL_JUNCTION_REGION_ID, DARK_AURA_REGION_ID, HollowGroveWorldPointBinding,
    LIGHT_AURA_REGION_ID, WORLD_FIELD_ID, WORLD_NEGATIVE_POLE_ID, WORLD_POINT_ID,
    WORLD_POSITIVE_POLE_ID, WorldCardinalOrientation, WorldFieldRelation, WorldFieldRelationship,
    WorldFieldRelationshipId, WorldFieldSubjectId,
};
use super::world_point_archive::WorldPointArchivePayload;

fn evidence(value: &str) -> SpatialEvidenceId {
    SpatialEvidenceId::new(value).expect("canonical world Point evidence ID")
}

fn region(value: &str) -> SpatialRegionId {
    SpatialRegionId::new(value).expect("canonical world region ID")
}

fn relationship(
    id: &str,
    subject_id: &str,
    relation: WorldFieldRelation,
    region_id: Option<&str>,
) -> WorldFieldRelationship {
    WorldFieldRelationship {
        relationship_id: WorldFieldRelationshipId::new(id)
            .expect("canonical world field relationship ID"),
        subject_id: WorldFieldSubjectId::new(subject_id).expect("canonical world field subject ID"),
        relation,
        region_id: region_id.map(region),
        evidence_ids: [evidence(&format!("evidence.{id}"))].into_iter().collect(),
    }
}

#[must_use]
pub fn canonical_world_point_archive_fixture() -> WorldPointArchivePayload {
    let axis = PolarityAxis::new([0, 1, 0], AxisHandedness::RightHanded)
        .expect("canonical Hollow Grove world axis");
    let source_point = OrientedPoint {
        point_id: PointId::new("point.seed.hollow-grove").expect("canonical source Point ID"),
        center_id: PointCenterId::new("center.world.hollow-grove")
            .expect("canonical world center ID"),
        center: PhysicalPosition::origin(),
        orientation: axis,
        positive_pole_id: PoleId::new(WORLD_POSITIVE_POLE_ID)
            .expect("canonical world positive pole ID"),
        negative_pole_id: PoleId::new(WORLD_NEGATIVE_POLE_ID)
            .expect("canonical world negative pole ID"),
        scale: ScaleKey::new("scale.object").expect("canonical source Point scale"),
        extent: PhysicalExtent::new(1).expect("canonical source Point extent"),
        evidence_ids: [evidence("evidence.point.seed.hollow-grove")]
            .into_iter()
            .collect(),
        provenance_ids: BTreeSet::new(),
    };
    let scaling = PointScaling {
        scaling_id: CompositionRecordId::new("record.scale.hollow-grove-world-point")
            .expect("canonical world scaling record ID"),
        source_point_id: source_point.point_id.clone(),
        result_point_id: PointId::new(WORLD_POINT_ID).expect("canonical world Point ID"),
        source_scale: source_point.scale.clone(),
        result_scale: ScaleKey::new("scale.world").expect("canonical world scale"),
        factor: PositiveScaleFactor::new(1_000_000).expect("canonical positive world scale factor"),
        authority_ids: [
            SpatialAuthorityId::new("authority.hollow-grove.world-point-binding")
                .expect("canonical world scaling authority ID"),
        ]
        .into_iter()
        .collect(),
        evidence_ids: [evidence("evidence.record.scale.hollow-grove-world-point")]
            .into_iter()
            .collect(),
    };
    let world_point = lawfully_scale_point(&source_point, &scaling)
        .expect("canonical world Point must scale lawfully");
    let field = ExpandedPointField {
        field_id: FieldId::new(WORLD_FIELD_ID).expect("canonical world field ID"),
        source_point_id: world_point.point_id.clone(),
        scale: world_point.scale.clone(),
        center_region_id: region(CENTRAL_JUNCTION_REGION_ID),
        positive_region_id: region(LIGHT_AURA_REGION_ID),
        negative_region_id: region(DARK_AURA_REGION_ID),
        axis,
        evidence_ids: [evidence("evidence.field.world.hollow-grove")]
            .into_iter()
            .collect(),
        provenance_ids: [scaling.scaling_id.clone()].into_iter().collect(),
    };
    let binding = HollowGroveWorldPointBinding {
        point: world_point.clone(),
        field,
        cardinal_orientation: WorldCardinalOrientation::PositiveNorthTopNegativeSouthBottom,
        light_aura_region_id: region(LIGHT_AURA_REGION_ID),
        central_junction_region_id: region(CENTRAL_JUNCTION_REGION_ID),
        dark_aura_region_id: region(DARK_AURA_REGION_ID),
        lawfulness_requires_separate_determination: true,
        presentation_may_change_constitutional_polarity: false,
        evidence_ids: [evidence(
            "evidence.binding.world.hollow-grove-oriented-point",
        )]
        .into_iter()
        .collect(),
    };
    let relationships = vec![
        relationship(
            "relationship.world-field.central-junction",
            "venue.central-junction",
            WorldFieldRelation::CentralRegionPlacement,
            Some(CENTRAL_JUNCTION_REGION_ID),
        ),
        relationship(
            "relationship.world-field.aura-field",
            "venue.aura-field",
            WorldFieldRelation::PositiveRegionPlacement,
            Some(LIGHT_AURA_REGION_ID),
        ),
        relationship(
            "relationship.world-field.aura-beach",
            "venue.aura-beach",
            WorldFieldRelation::CentralRegionPlacement,
            Some(CENTRAL_JUNCTION_REGION_ID),
        ),
        relationship(
            "relationship.world-field.aura-basin",
            "venue.aura-basin",
            WorldFieldRelation::NegativeRegionPlacement,
            Some(DARK_AURA_REGION_ID),
        ),
        relationship(
            "relationship.world-field.way-back",
            "passage.the-way-back",
            WorldFieldRelation::TransverseOrCircumferential,
            None,
        ),
        relationship(
            "relationship.world-field.current-sea",
            "region.current-sea",
            WorldFieldRelation::TransverseOrCircumferential,
            None,
        ),
        relationship(
            "relationship.world-field.boardwalk",
            "route.boardwalk",
            WorldFieldRelation::TransverseOrCircumferential,
            None,
        ),
        relationship(
            "relationship.world-field.central-junction-functions",
            "function.central-junction.seasonal-cycle",
            WorldFieldRelation::CentralRegionPlacement,
            Some(CENTRAL_JUNCTION_REGION_ID),
        ),
    ];
    let explicit_inversion_probe = PointInversion {
        inversion_id: CompositionRecordId::new("record.inversion.hollow-grove-world-point-probe")
            .expect("canonical inversion probe record ID"),
        source_point_id: world_point.point_id.clone(),
        result_point_id: PointId::new("point.world.hollow-grove.explicitly-inverted-probe")
            .expect("canonical inverted probe Point ID"),
        authority_ids: [
            SpatialAuthorityId::new("authority.audit.explicit-point-inversion")
                .expect("canonical inversion probe authority ID"),
        ]
        .into_iter()
        .collect(),
        evidence_ids: [evidence(
            "evidence.record.inversion.hollow-grove-world-point-probe",
        )]
        .into_iter()
        .collect(),
    };
    WorldPointArchivePayload {
        source_point,
        scaling,
        binding,
        relationships,
        explicit_inversion_probe,
    }
}
