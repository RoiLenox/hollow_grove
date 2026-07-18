//! Read-only composition witnesses for existing Hollow Grove systems.
//!
//! These constructors prove that neutral composition can reference existing
//! domain mechanics without changing their authority or behavior.

use std::fmt;

use crate::composition::{
    CompositionCatalog, CompositionCatalogError, CompositionNode, CompositionNodeId,
    CompositionRecord, CompositionRecordId, Containment, ExternalRef, ExternalRefError, ScaleKey,
    StableKeyError,
};
use crate::hollow_grove_contract::House;
use crate::hueman_support::{
    build_hueman_boundary_from_artifacts, build_hueman_motion_map_from_artifacts,
};
use crate::lineage_contract::{SandmanorTransitionError, validate_sandmanor_transition};
use crate::point_progression::build_canonical_point_squared_fixture;
use crate::{FrameId, Symptom, run_kernel_cycle};

use super::fourway::{FourwayDirection, house_at};
use super::house_institutions::sandmen_id;

#[derive(Debug)]
pub enum CompositionWitnessError {
    Catalog(CompositionCatalogError),
    StableKey(StableKeyError),
    ExternalRef(ExternalRefError),
    PointProgression(std::io::Error),
    SandmanorTransition(SandmanorTransitionError),
}

impl fmt::Display for CompositionWitnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Catalog(error) => error.fmt(formatter),
            Self::StableKey(error) => error.fmt(formatter),
            Self::ExternalRef(error) => error.fmt(formatter),
            Self::PointProgression(error) => error.fmt(formatter),
            Self::SandmanorTransition(error) => write!(formatter, "{error:?}"),
        }
    }
}

impl std::error::Error for CompositionWitnessError {}

impl From<CompositionCatalogError> for CompositionWitnessError {
    fn from(error: CompositionCatalogError) -> Self {
        Self::Catalog(error)
    }
}
impl From<StableKeyError> for CompositionWitnessError {
    fn from(error: StableKeyError) -> Self {
        Self::StableKey(error)
    }
}
impl From<ExternalRefError> for CompositionWitnessError {
    fn from(error: ExternalRefError) -> Self {
        Self::ExternalRef(error)
    }
}
impl From<std::io::Error> for CompositionWitnessError {
    fn from(error: std::io::Error) -> Self {
        Self::PointProgression(error)
    }
}
impl From<SandmanorTransitionError> for CompositionWitnessError {
    fn from(error: SandmanorTransitionError) -> Self {
        Self::SandmanorTransition(error)
    }
}

fn node(
    id: &str,
    namespace: &str,
    key: impl Into<String>,
    scale: &str,
) -> Result<CompositionNode, CompositionWitnessError> {
    Ok(CompositionNode {
        id: CompositionNodeId::new(id)?,
        object: ExternalRef::new(namespace, key)?,
        scale: ScaleKey::new(scale)?,
    })
}

fn reference(
    namespace: &str,
    key: impl Into<String>,
) -> Result<ExternalRef, CompositionWitnessError> {
    Ok(ExternalRef::new(namespace, key)?)
}

fn record(
    id: &str,
    sources: Vec<CompositionNodeId>,
    result: CompositionNodeId,
    operation: ExternalRef,
    evidence: Option<ExternalRef>,
) -> Result<CompositionRecord, CompositionWitnessError> {
    Ok(CompositionRecord {
        id: CompositionRecordId::new(id)?,
        sources,
        result,
        operation,
        evidence,
    })
}

/// Builds the four agreed read-only witnesses in one neutral catalog.
pub fn canonical_composition_witness_catalog() -> Result<CompositionCatalog, CompositionWitnessError>
{
    let mut catalog = CompositionCatalog::new();
    point_progression_witness(&mut catalog)?;
    sandmanor_transformation_witness(&mut catalog)?;
    sandmanor_house_witness(&mut catalog)?;
    runtime_artifact_witness(&mut catalog)?;
    Ok(catalog)
}

fn point_progression_witness(
    catalog: &mut CompositionCatalog,
) -> Result<(), CompositionWitnessError> {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
    let fixture = build_canonical_point_squared_fixture()?;
    assert_eq!(kernel_pass.end_point(), fixture.point_before());
    assert_eq!(
        fixture
            .first_application()
            .stabilized_point()
            .progression()
            .stable_point_level(),
        2
    );

    let origin = node(
        "node.point.hueman.origin",
        "point",
        "hueman.origin",
        "scale.point",
    )?;
    let landed = node(
        "node.point.hueman.landed-2",
        "point",
        "hueman.landed-2",
        "scale.point",
    )?;
    let stabilized = node(
        "node.point.hueman.level-2",
        "point",
        "hueman.level-2",
        "scale.point",
    )?;
    for entry in [&origin, &landed, &stabilized] {
        catalog.insert_node(entry.clone())?;
    }
    catalog.insert_record(record(
        "record.point.kernel-pass",
        vec![origin.id.clone()],
        landed.id.clone(),
        reference("kernel-pass", "canonical-origin")?,
        Some(reference("kernel-pass", "canonical-origin-trace")?),
    )?)?;
    catalog.insert_record(record(
        "record.point.point-squared-ascension",
        vec![landed.id.clone()],
        stabilized.id,
        reference("point-squared-ascension", "canonical-first")?,
        Some(reference(
            "point-squared-ascension",
            "canonical-first-application",
        )?),
    )?)?;
    Ok(())
}

fn sandmanor_transformation_witness(
    catalog: &mut CompositionCatalog,
) -> Result<(), CompositionWitnessError> {
    validate_sandmanor_transition(FrameId::Gnome, FrameId::Minotaur)?;
    validate_sandmanor_transition(FrameId::Minotaur, FrameId::Hecaton)?;

    let being = node(
        "node.being.hueman.sandmanor-witness",
        "being",
        "hueman.sandmanor-witness",
        "scale.being",
    )?;
    let gnome = node("node.frame.gnome", "frame", "gnome", "scale.frame")?;
    let minotaur = node("node.frame.minotaur", "frame", "minotaur", "scale.frame")?;
    let hecaton = node("node.frame.hecaton", "frame", "hecaton", "scale.frame")?;
    let sandmanor = node("node.house.sandmanor", "house", "sandmanor", "scale.house")?;
    for entry in [&being, &gnome, &minotaur, &hecaton, &sandmanor] {
        catalog.insert_node(entry.clone())?;
    }
    catalog.add_containment(Containment {
        container: sandmanor.id.clone(),
        member: being.id.clone(),
    })?;
    catalog.insert_record(record(
        "record.sandmanor.gnome-to-minotaur",
        vec![being.id.clone(), gnome.id],
        being.id.clone(),
        reference("sandmanor-transition", "gnome-to-minotaur")?,
        Some(reference("sandmanor-lineage", "gnome-minotaur-legal")?),
    )?)?;
    catalog.insert_record(record(
        "record.sandmanor.minotaur-to-hecaton",
        vec![being.id.clone(), minotaur.id],
        being.id,
        reference("sandmanor-transition", "minotaur-to-hecaton")?,
        Some(reference("sandmanor-lineage", "minotaur-hecaton-legal")?),
    )?)?;
    Ok(())
}

fn sandmanor_house_witness(
    catalog: &mut CompositionCatalog,
) -> Result<(), CompositionWitnessError> {
    assert_eq!(house_at(FourwayDirection::South), House::Sandmanor);
    let house = CompositionNodeId::new("node.house.sandmanor")?;
    let identity = node(
        "node.house-definition.sandmanor",
        "house-definition",
        "sandmanor",
        "scale.house-component",
    )?;
    let lineage = node(
        "node.lineage.sandmanor.minorian",
        "sandmanor-lineage",
        "minorian",
        "scale.lineage",
    )?;
    let institution = node(
        "node.institution.sandmanor.sandmen",
        "institution",
        sandmen_id().as_str(),
        "scale.institution",
    )?;
    let topology = node(
        "node.topology.fourway.sandmanor",
        "fourway",
        "south-sandmanor",
        "scale.topology",
    )?;
    for entry in [&identity, &lineage, &institution, &topology] {
        catalog.insert_node(entry.clone())?;
        catalog.add_containment(Containment {
            container: house.clone(),
            member: entry.id.clone(),
        })?;
    }
    catalog.insert_record(record(
        "record.house.sandmanor",
        vec![identity.id, lineage.id, institution.id, topology.id],
        house.clone(),
        reference("house-composition", "sandmanor-canonical")?,
        Some(reference("fourway", "south-sandmanor")?),
    )?)?;

    let grove = node("node.grove.hollow", "grove", "hollow", "scale.grove")?;
    catalog.insert_node(grove.clone())?;
    catalog.add_containment(Containment {
        container: grove.id.clone(),
        member: house.clone(),
    })?;
    catalog.insert_record(record(
        "record.grove.sandmanor-witness",
        vec![house],
        grove.id,
        reference("grove-composition", "sandmanor-witness")?,
        Some(reference("fourway", "canonical-roster")?),
    )?)?;
    Ok(())
}

fn runtime_artifact_witness(
    catalog: &mut CompositionCatalog,
) -> Result<(), CompositionWitnessError> {
    let base = node(
        "node.artifact.current-synthesis.base",
        "artifact",
        "artifacts/current_synthesis_base.md",
        "scale.artifact",
    )?;
    let gate = node(
        "node.artifact.current-synthesis.activation-gate",
        "artifact",
        "artifacts/current_synthesis_activation_gate.md",
        "scale.artifact",
    )?;
    let boundary = node(
        "node.artifact.hueman.boundary",
        "artifact",
        "artifacts/hueman_boundary.md",
        "scale.artifact",
    )?;
    let motion_map = node(
        "node.artifact.hueman.motion-map",
        "artifact",
        "artifacts/hueman_motion_map.md",
        "scale.artifact",
    )?;
    let runtime = node(
        "node.runtime.hollow-grove",
        "runtime-context",
        "hollow-grove",
        "scale.runtime-context",
    )?;
    for entry in [&base, &gate, &boundary, &motion_map, &runtime] {
        catalog.insert_node(entry.clone())?;
    }
    let built_boundary = build_hueman_boundary_from_artifacts("canonical-base", "canonical-gate");
    assert!(built_boundary.contains("# Hueman Boundary"));
    catalog.add_containment(Containment {
        container: runtime.id.clone(),
        member: boundary.id.clone(),
    })?;
    catalog.insert_record(record(
        "record.artifact.hueman-boundary",
        vec![base.id, gate.id],
        boundary.id.clone(),
        reference("artifact-builder", "build-hueman-boundary-from-artifacts")?,
        Some(reference("artifact", "artifacts/hueman_boundary.md")?),
    )?)?;

    let built_motion_map = build_hueman_motion_map_from_artifacts(&built_boundary, "operational");
    assert!(built_motion_map.contains("# Hueman Motion Map"));
    catalog.add_containment(Containment {
        container: runtime.id,
        member: motion_map.id.clone(),
    })?;
    catalog.insert_record(record(
        "record.artifact.hueman-motion-map",
        vec![boundary.id],
        motion_map.id,
        reference("artifact-builder", "build-hueman-motion-map-from-artifacts")?,
        Some(reference("artifact", "artifacts/hueman_motion_map.md")?),
    )?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::CompositionCatalogError;

    #[test]
    fn point_witness_reuses_landed_point_as_a_source() {
        let catalog = canonical_composition_witness_catalog().unwrap();
        let landed = CompositionNodeId::new("node.point.hueman.landed-2").unwrap();
        assert_eq!(catalog.records_producing_result(&landed).len(), 1);
        assert_eq!(catalog.records_using_source(&landed).len(), 1);
        let ascent = catalog
            .record(&CompositionRecordId::new("record.point.point-squared-ascension").unwrap())
            .unwrap();
        assert_eq!(
            ascent.evidence.as_ref().unwrap().namespace,
            "point-squared-ascension"
        );
    }

    #[test]
    fn sandmanor_witness_preserves_being_identity_and_existing_legality_boundary() {
        let catalog = canonical_composition_witness_catalog().unwrap();
        let being = CompositionNodeId::new("node.being.hueman.sandmanor-witness").unwrap();
        assert_eq!(catalog.records_producing_result(&being).len(), 2);
        assert_eq!(catalog.records_using_source(&being).len(), 2);
        assert!(validate_sandmanor_transition(FrameId::Gnome, FrameId::Pegasus).is_err());
    }

    #[test]
    fn house_containment_is_distinct_from_house_composition_sources() {
        let catalog = canonical_composition_witness_catalog().unwrap();
        let house = CompositionNodeId::new("node.house.sandmanor").unwrap();
        let being = CompositionNodeId::new("node.being.hueman.sandmanor-witness").unwrap();
        assert!(
            catalog
                .direct_members(&house)
                .iter()
                .any(|node| node.id == being)
        );
        let house_record = catalog
            .record(&CompositionRecordId::new("record.house.sandmanor").unwrap())
            .unwrap();
        assert!(!house_record.sources.contains(&being));
        assert_eq!(catalog.records_using_source(&house).len(), 1);
    }

    #[test]
    fn runtime_artifact_output_becomes_next_builder_source() {
        let catalog = canonical_composition_witness_catalog().unwrap();
        let boundary = CompositionNodeId::new("node.artifact.hueman.boundary").unwrap();
        assert_eq!(catalog.records_producing_result(&boundary).len(), 1);
        assert_eq!(catalog.records_using_source(&boundary).len(), 1);
        let containers = catalog.direct_containers(&boundary);
        assert_eq!(containers.len(), 1);
        assert_eq!(containers[0].id.as_str(), "node.runtime.hollow-grove");
    }

    #[test]
    fn containment_cycles_are_rejected_without_domain_interpretation() {
        let mut catalog = CompositionCatalog::new();
        let first = node("node.test.first", "test", "first", "scale.test").unwrap();
        let second = node("node.test.second", "test", "second", "scale.test").unwrap();
        let third = node("node.test.third", "test", "third", "scale.test").unwrap();
        let disconnected = node(
            "node.test.disconnected",
            "test",
            "disconnected",
            "scale.test",
        )
        .unwrap();
        catalog.insert_node(first.clone()).unwrap();
        catalog.insert_node(second.clone()).unwrap();
        catalog.insert_node(third.clone()).unwrap();
        catalog.insert_node(disconnected.clone()).unwrap();
        catalog
            .add_containment(Containment {
                container: first.id.clone(),
                member: second.id.clone(),
            })
            .unwrap();
        catalog
            .add_containment(Containment {
                container: second.id.clone(),
                member: third.id.clone(),
            })
            .unwrap();
        catalog
            .add_containment(Containment {
                container: disconnected.id,
                member: first.id.clone(),
            })
            .unwrap();
        assert!(matches!(
            catalog.add_containment(Containment {
                container: third.id,
                member: first.id,
            }),
            Err(CompositionCatalogError::ContainmentCycle { .. })
        ));
    }
}
