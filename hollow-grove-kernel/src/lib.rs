//! Neutral Hollow Grove composition and provenance kernel.
//!
//! This module records which addressable nodes participated in a completed
//! composition. It does not execute operations or interpret domain references.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

mod oriented_point;

pub use oriented_point::{
    AxisHandedness, ExpandedPointField, FieldId, OrientedPoint, OrientedPointError, PhysicalExtent,
    PhysicalPosition, PointCenterId, PointId, PointInversion, PointScaling, PolarityAxis,
    PolarityTendency, PoleId, PositiveScaleFactor, RelativePolarity, SpatialAuthorityId,
    SpatialEvidenceId, SpatialRegionId, invert_point, lawfully_scale_point,
};

fn is_stable_key(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableKeyError {
    Invalid(String),
}

impl fmt::Display for StableKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(value) => write!(formatter, "invalid stable key: {value}"),
        }
    }
}

impl std::error::Error for StableKeyError {}

macro_rules! stable_key {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, StableKeyError> {
                let value = value.into();
                if is_stable_key(&value) {
                    Ok(Self(value))
                } else {
                    Err(StableKeyError::Invalid(value))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl TryFrom<String> for $name {
            type Error = StableKeyError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

stable_key!(CompositionNodeId);
stable_key!(CompositionRecordId);
stable_key!(ScaleKey);

/// Opaque reference to a domain-owned object, operation, or evidence trace.
/// The composition layer does not parse either field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalRef {
    pub namespace: String,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalRefError {
    InvalidNamespace(String),
    EmptyKey,
    MultilineKey,
}

impl fmt::Display for ExternalRefError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNamespace(value) => {
                write!(formatter, "invalid external namespace: {value}")
            }
            Self::EmptyKey => formatter.write_str("external reference key must not be empty"),
            Self::MultilineKey => formatter.write_str("external reference key must be one line"),
        }
    }
}

impl std::error::Error for ExternalRefError {}

impl ExternalRef {
    pub fn new(
        namespace: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Self, ExternalRefError> {
        let namespace = namespace.into();
        let key = key.into();
        if !is_stable_key(&namespace) {
            return Err(ExternalRefError::InvalidNamespace(namespace));
        }
        if key.is_empty() {
            return Err(ExternalRefError::EmptyKey);
        }
        if key.contains(['\n', '\r']) {
            return Err(ExternalRefError::MultilineKey);
        }
        Ok(Self { namespace, key })
    }
}

/// An addressable domain projection. Every composition node is eligible to be
/// a source in a later composition record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionNode {
    pub id: CompositionNodeId,
    pub object: ExternalRef,
    pub scale: ScaleKey,
}

/// Causal provenance. Source order is retained for operations that need it,
/// but carries no interpretation in this neutral layer.
#[derive(Debug, PartialEq, Eq)]
pub struct CompositionRecord {
    pub id: CompositionRecordId,
    pub sources: Vec<CompositionNodeId>,
    pub result: CompositionNodeId,
    pub operation: ExternalRef,
    pub evidence: Option<ExternalRef>,
}

/// Direct structural membership, deliberately distinct from causal provenance.
#[derive(Debug, PartialEq, Eq)]
pub struct Containment {
    pub container: CompositionNodeId,
    pub member: CompositionNodeId,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompositionCatalogError {
    DuplicateNodeId(CompositionNodeId),
    DuplicateRecordId(CompositionRecordId),
    MissingSourceNode(CompositionNodeId),
    MissingResultNode(CompositionNodeId),
    MissingContainmentContainer(CompositionNodeId),
    MissingContainmentMember(CompositionNodeId),
    DuplicateContainment {
        container: CompositionNodeId,
        member: CompositionNodeId,
    },
    EmptySources(CompositionRecordId),
    SelfContainment(CompositionNodeId),
    ContainmentCycle {
        container: CompositionNodeId,
        member: CompositionNodeId,
    },
}

impl fmt::Display for CompositionCatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNodeId(id) => write!(formatter, "duplicate composition node ID: {id}"),
            Self::DuplicateRecordId(id) => {
                write!(formatter, "duplicate composition record ID: {id}")
            }
            Self::MissingSourceNode(id) => {
                write!(formatter, "missing composition source node: {id}")
            }
            Self::MissingResultNode(id) => {
                write!(formatter, "missing composition result node: {id}")
            }
            Self::MissingContainmentContainer(id) => {
                write!(formatter, "missing composition containment container: {id}")
            }
            Self::MissingContainmentMember(id) => {
                write!(formatter, "missing composition containment member: {id}")
            }
            Self::DuplicateContainment { container, member } => {
                write!(
                    formatter,
                    "duplicate composition containment: {container} contains {member}"
                )
            }
            Self::EmptySources(id) => write!(formatter, "composition record has no sources: {id}"),
            Self::SelfContainment(id) => {
                write!(formatter, "composition node contains itself: {id}")
            }
            Self::ContainmentCycle { container, member } => {
                write!(
                    formatter,
                    "composition containment cycle: {container} contains {member}"
                )
            }
        }
    }
}

impl std::error::Error for CompositionCatalogError {}

/// In-memory access boundary for neutral nodes, causal records, and direct
/// containment. It is intentionally not a general graph database.
#[derive(Debug)]
pub struct CompositionCatalog {
    nodes: Vec<CompositionNode>,
    records: Vec<CompositionRecord>,
    containments: Vec<Containment>,
    node_positions: HashMap<CompositionNodeId, usize>,
    record_positions: HashMap<CompositionRecordId, usize>,
    records_by_source: HashMap<CompositionNodeId, Vec<usize>>,
    records_by_result: HashMap<CompositionNodeId, Vec<usize>>,
}

impl Default for CompositionCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositionCatalog {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            records: Vec::new(),
            containments: Vec::new(),
            node_positions: HashMap::new(),
            record_positions: HashMap::new(),
            records_by_source: HashMap::new(),
            records_by_result: HashMap::new(),
        }
    }

    pub fn insert_node(&mut self, node: CompositionNode) -> Result<(), CompositionCatalogError> {
        if self.node_positions.contains_key(&node.id) {
            return Err(CompositionCatalogError::DuplicateNodeId(node.id));
        }
        let position = self.nodes.len();
        self.node_positions.insert(node.id.clone(), position);
        self.nodes.push(node);
        Ok(())
    }

    #[must_use]
    pub fn node(&self, id: &CompositionNodeId) -> Option<&CompositionNode> {
        self.node_positions
            .get(id)
            .and_then(|position| self.nodes.get(*position))
    }

    pub fn insert_record(
        &mut self,
        record: CompositionRecord,
    ) -> Result<(), CompositionCatalogError> {
        if self.record_positions.contains_key(&record.id) {
            return Err(CompositionCatalogError::DuplicateRecordId(record.id));
        }
        if record.sources.is_empty() {
            return Err(CompositionCatalogError::EmptySources(record.id));
        }
        for source in &record.sources {
            if self.node(source).is_none() {
                return Err(CompositionCatalogError::MissingSourceNode(source.clone()));
            }
        }
        if self.node(&record.result).is_none() {
            return Err(CompositionCatalogError::MissingResultNode(record.result));
        }

        let position = self.records.len();
        let mut indexed_sources = std::collections::HashSet::new();
        for source in &record.sources {
            if !indexed_sources.insert(source.clone()) {
                continue;
            }
            self.records_by_source
                .entry(source.clone())
                .or_default()
                .push(position);
        }
        self.records_by_result
            .entry(record.result.clone())
            .or_default()
            .push(position);
        self.record_positions.insert(record.id.clone(), position);
        self.records.push(record);
        Ok(())
    }

    #[must_use]
    pub fn record(&self, id: &CompositionRecordId) -> Option<&CompositionRecord> {
        self.record_positions
            .get(id)
            .and_then(|position| self.records.get(*position))
    }

    #[must_use]
    pub fn records_using_source(&self, source: &CompositionNodeId) -> Vec<&CompositionRecord> {
        self.records_by_source
            .get(source)
            .into_iter()
            .flatten()
            .filter_map(|position| self.records.get(*position))
            .collect()
    }

    #[must_use]
    pub fn records_producing_result(&self, result: &CompositionNodeId) -> Vec<&CompositionRecord> {
        self.records_by_result
            .get(result)
            .into_iter()
            .flatten()
            .filter_map(|position| self.records.get(*position))
            .collect()
    }

    pub fn add_containment(
        &mut self,
        containment: Containment,
    ) -> Result<(), CompositionCatalogError> {
        if self.node(&containment.container).is_none() {
            return Err(CompositionCatalogError::MissingContainmentContainer(
                containment.container,
            ));
        }
        if self.node(&containment.member).is_none() {
            return Err(CompositionCatalogError::MissingContainmentMember(
                containment.member,
            ));
        }
        if containment.container == containment.member {
            return Err(CompositionCatalogError::SelfContainment(
                containment.container,
            ));
        }
        if self.containments.iter().any(|entry| entry == &containment) {
            return Err(CompositionCatalogError::DuplicateContainment {
                container: containment.container,
                member: containment.member,
            });
        }
        if self.reaches(&containment.member, &containment.container) {
            return Err(CompositionCatalogError::ContainmentCycle {
                container: containment.container,
                member: containment.member,
            });
        }
        self.containments.push(containment);
        Ok(())
    }

    #[must_use]
    pub fn direct_containers(&self, member: &CompositionNodeId) -> Vec<&CompositionNode> {
        self.containments
            .iter()
            .filter(|entry| &entry.member == member)
            .filter_map(|entry| self.node(&entry.container))
            .collect()
    }

    #[must_use]
    pub fn direct_members(&self, container: &CompositionNodeId) -> Vec<&CompositionNode> {
        self.containments
            .iter()
            .filter(|entry| &entry.container == container)
            .filter_map(|entry| self.node(&entry.member))
            .collect()
    }

    fn reaches(&self, start: &CompositionNodeId, target: &CompositionNodeId) -> bool {
        let mut pending = vec![start.clone()];
        let mut visited = std::collections::HashSet::new();
        while let Some(current) = pending.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            if &current == target {
                return true;
            }
            pending.extend(
                self.containments
                    .iter()
                    .filter(|entry| entry.container == current)
                    .map(|entry| entry.member.clone()),
            );
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str) -> CompositionNode {
        CompositionNode {
            id: CompositionNodeId::new(id).unwrap(),
            object: ExternalRef::new("test", "same-object").unwrap(),
            scale: ScaleKey::new("scale.test").unwrap(),
        }
    }

    #[test]
    fn equal_domain_state_can_have_distinct_node_identities() {
        let first = node("node.test.first");
        let second = node("node.test.second");
        assert_eq!(first.object, second.object);
        assert_eq!(first.scale, second.scale);
        assert_ne!(first.id, second.id);
    }

    #[test]
    fn caller_supplied_identity_is_independent_of_insertion_order() {
        let first = node("node.test.first");
        let second = node("node.test.second");
        let mut forward = CompositionCatalog::new();
        forward.insert_node(first.clone()).unwrap();
        forward.insert_node(second.clone()).unwrap();
        let mut reverse = CompositionCatalog::new();
        reverse.insert_node(second.clone()).unwrap();
        reverse.insert_node(first.clone()).unwrap();
        assert_eq!(
            forward.node(&first.id).unwrap().id,
            reverse.node(&first.id).unwrap().id
        );
        assert_eq!(
            forward.node(&second.id).unwrap().id,
            reverse.node(&second.id).unwrap().id
        );
    }

    #[test]
    fn catalog_indexes_source_and_result_records() {
        let source = node("node.test.source");
        let result = node("node.test.result");
        let next = node("node.test.next");
        let mut catalog = CompositionCatalog::new();
        catalog.insert_node(source.clone()).unwrap();
        catalog.insert_node(result.clone()).unwrap();
        catalog.insert_node(next.clone()).unwrap();
        catalog
            .insert_record(CompositionRecord {
                id: CompositionRecordId::new("record.test.first").unwrap(),
                sources: vec![source.id.clone()],
                result: result.id.clone(),
                operation: ExternalRef::new("test-operation", "first").unwrap(),
                evidence: None,
            })
            .unwrap();
        catalog
            .insert_record(CompositionRecord {
                id: CompositionRecordId::new("record.test.second").unwrap(),
                sources: vec![source.id.clone(), result.id.clone()],
                result: next.id.clone(),
                operation: ExternalRef::new("test-operation", "second").unwrap(),
                evidence: None,
            })
            .unwrap();
        assert_eq!(catalog.records_using_source(&source.id).len(), 2);
        assert_eq!(catalog.records_producing_result(&result.id).len(), 1);
    }

    #[test]
    fn source_index_is_not_duplicated_when_an_operation_repeats_a_source() {
        let source = node("node.test.source");
        let result = node("node.test.result");
        let mut catalog = CompositionCatalog::new();
        catalog.insert_node(source.clone()).unwrap();
        catalog.insert_node(result.clone()).unwrap();
        catalog
            .insert_record(CompositionRecord {
                id: CompositionRecordId::new("record.test.repeated-source").unwrap(),
                sources: vec![source.id.clone(), source.id.clone()],
                result: result.id,
                operation: ExternalRef::new("test-operation", "repeated-source").unwrap(),
                evidence: None,
            })
            .unwrap();
        assert_eq!(catalog.records_using_source(&source.id).len(), 1);
    }

    #[test]
    fn catalog_rejects_missing_record_references_and_invalid_containment() {
        let source = node("node.test.source");
        let missing = CompositionNodeId::new("node.test.missing").unwrap();
        let mut catalog = CompositionCatalog::new();
        catalog.insert_node(source.clone()).unwrap();
        assert!(matches!(
            catalog.insert_record(CompositionRecord {
                id: CompositionRecordId::new("record.test.missing-source").unwrap(),
                sources: vec![missing.clone()],
                result: source.id.clone(),
                operation: ExternalRef::new("test-operation", "missing-source").unwrap(),
                evidence: None,
            }),
            Err(CompositionCatalogError::MissingSourceNode(_))
        ));
        assert!(matches!(
            catalog.insert_record(CompositionRecord {
                id: CompositionRecordId::new("record.test.missing-result").unwrap(),
                sources: vec![source.id.clone()],
                result: missing,
                operation: ExternalRef::new("test-operation", "missing-result").unwrap(),
                evidence: None,
            }),
            Err(CompositionCatalogError::MissingResultNode(_))
        ));
        assert!(matches!(
            catalog.add_containment(Containment {
                container: source.id.clone(),
                member: source.id.clone(),
            }),
            Err(CompositionCatalogError::SelfContainment(_))
        ));
    }

    #[test]
    fn external_references_are_opaque_and_future_scales_are_accepted() {
        let reference = ExternalRef::new("artifact-builder", "paths/Any Value_v2").unwrap();
        assert_eq!(reference.key, "paths/Any Value_v2");
        assert_eq!(
            ScaleKey::new("scale.federated-environment")
                .unwrap()
                .as_str(),
            "scale.federated-environment"
        );
    }

    #[test]
    fn duplicate_errors_identify_the_corrupt_identity_or_containment() {
        let first = node("node.test.first");
        let second = node("node.test.second");
        let mut catalog = CompositionCatalog::new();
        catalog.insert_node(first.clone()).unwrap();
        assert!(matches!(
            catalog.insert_node(first.clone()),
            Err(CompositionCatalogError::DuplicateNodeId(id)) if id == first.id
        ));
        catalog.insert_node(second.clone()).unwrap();
        let duplicate_record = CompositionRecord {
            id: CompositionRecordId::new("record.test.duplicate").unwrap(),
            sources: vec![first.id.clone()],
            result: second.id.clone(),
            operation: ExternalRef::new("test-operation", "duplicate").unwrap(),
            evidence: None,
        };
        catalog.insert_record(duplicate_record).unwrap();
        assert!(matches!(
            catalog.insert_record(CompositionRecord {
                id: CompositionRecordId::new("record.test.duplicate").unwrap(),
                sources: vec![first.id.clone()],
                result: second.id.clone(),
                operation: ExternalRef::new("test-operation", "duplicate").unwrap(),
                evidence: None,
            }),
            Err(CompositionCatalogError::DuplicateRecordId(id)) if id.as_str() == "record.test.duplicate"
        ));
        catalog
            .add_containment(Containment {
                container: first.id.clone(),
                member: second.id.clone(),
            })
            .unwrap();
        assert!(matches!(
            catalog.add_containment(Containment {
                container: first.id.clone(),
                member: second.id.clone(),
            }),
            Err(CompositionCatalogError::DuplicateContainment { container, member })
                if container == first.id && member == second.id
        ));
    }
}
