//! Bounded exact-canon and nearby-position retrieval for semantic prompts.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{
    CompositionCatalog, CompositionRecordId, ExternalRef, Position3, SemanticScope,
    squared_distance,
};

pub const MAX_RETRIEVED_CANON_FRAGMENTS: usize = 32;
pub const MAX_NEARBY_SEMANTIC_EXAMPLES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonStatus {
    Constitutional,
    Ratified,
    Historical,
    Interpretive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetrievedCanonFragment {
    pub reference: ExternalRef,
    pub source_document: String,
    pub section: String,
    pub status: CanonStatus,
    pub version: Option<String>,
    pub content: String,
}

impl RetrievedCanonFragment {
    fn validate(&self) -> Result<(), RetrievalError> {
        if self.source_document.trim().is_empty()
            || self.section.trim().is_empty()
            || self.content.trim().is_empty()
        {
            return Err(RetrievalError::MalformedCanonFragment(
                self.reference.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NearbySemanticExample {
    pub record_ref: ExternalRef,
    pub scope: SemanticScope,
    pub position: Position3,
    pub squared_distance: i64,
    pub semantic_tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievalRequest {
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub coordinate: Option<Position3>,
    pub constitutional_refs: Vec<ExternalRef>,
    pub maximum_canon_fragments: usize,
    pub maximum_nearby_examples: usize,
    pub semantic_tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrievalBundle {
    pub canon: Vec<RetrievedCanonFragment>,
    pub nearby_examples: Vec<NearbySemanticExample>,
}

pub trait CanonRetriever {
    fn retrieve(&self, request: &RetrievalRequest) -> Result<RetrievalBundle, RetrievalError>;
}

pub struct CatalogCanonRetriever<'a> {
    catalog: &'a CompositionCatalog,
    canon_by_reference: BTreeMap<ExternalRef, RetrievedCanonFragment>,
}

impl<'a> CatalogCanonRetriever<'a> {
    pub fn new(
        catalog: &'a CompositionCatalog,
        canon: impl IntoIterator<Item = RetrievedCanonFragment>,
    ) -> Result<Self, RetrievalError> {
        let mut canon_by_reference = BTreeMap::new();
        for fragment in canon {
            fragment.validate()?;
            if canon_by_reference
                .insert(fragment.reference.clone(), fragment.clone())
                .is_some()
            {
                return Err(RetrievalError::DuplicateCanonReference(fragment.reference));
            }
        }
        Ok(Self {
            catalog,
            canon_by_reference,
        })
    }
}

impl CanonRetriever for CatalogCanonRetriever<'_> {
    fn retrieve(&self, request: &RetrievalRequest) -> Result<RetrievalBundle, RetrievalError> {
        validate_limits(request)?;
        let mut references = request.constitutional_refs.clone();
        references.sort();
        references.dedup();
        let canon = references
            .into_iter()
            .take(request.maximum_canon_fragments)
            .map(|reference| {
                self.canon_by_reference
                    .get(&reference)
                    .cloned()
                    .ok_or(RetrievalError::UnknownCanonReference(reference))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(RetrievalBundle {
            canon,
            nearby_examples: nearby_semantic_examples(self.catalog, request)?,
        })
    }
}

pub fn nearby_semantic_examples(
    catalog: &CompositionCatalog,
    request: &RetrievalRequest,
) -> Result<Vec<NearbySemanticExample>, RetrievalError> {
    validate_limits(request)?;
    let center = request.coordinate.or_else(|| {
        catalog
            .current_semantic_position(&request.point_ref, &request.scope)
            .map(|record| record.position)
    });
    let Some(center) = center else {
        return Ok(Vec::new());
    };
    let requested_tags: BTreeSet<_> = request.semantic_tags.iter().cloned().collect();
    let mut examples = Vec::new();
    for record in catalog.semantic_position_records() {
        let tags = catalog
            .semantic_witness(&record.witness_ref)
            .map(|witness| witness.semantic_tags.clone())
            .unwrap_or_default();
        if !requested_tags.is_empty() && !tags.iter().any(|tag| requested_tags.contains(tag)) {
            continue;
        }
        let record_ref = ExternalRef::new("semantic-position", record.record_id.as_str())
            .map_err(|error| RetrievalError::ExternalReference(error.to_string()))?;
        examples.push((
            record.scope.scale != request.scope.scale,
            record.record_id.clone(),
            NearbySemanticExample {
                record_ref,
                scope: record.scope.clone(),
                position: record.position,
                squared_distance: squared_distance(center, record.position),
                semantic_tags: tags,
            },
        ));
    }
    examples.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.2.squared_distance.cmp(&right.2.squared_distance))
            .then_with(|| left.1.cmp(&right.1))
    });
    Ok(examples
        .into_iter()
        .take(request.maximum_nearby_examples)
        .map(|(_, _, example)| example)
        .collect())
}

fn validate_limits(request: &RetrievalRequest) -> Result<(), RetrievalError> {
    if request.maximum_canon_fragments > MAX_RETRIEVED_CANON_FRAGMENTS {
        return Err(RetrievalError::CanonLimitExceeded {
            supplied: request.maximum_canon_fragments,
            maximum: MAX_RETRIEVED_CANON_FRAGMENTS,
        });
    }
    if request.maximum_nearby_examples > MAX_NEARBY_SEMANTIC_EXAMPLES {
        return Err(RetrievalError::NearbyLimitExceeded {
            supplied: request.maximum_nearby_examples,
            maximum: MAX_NEARBY_SEMANTIC_EXAMPLES,
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrievalError {
    DuplicateCanonReference(ExternalRef),
    UnknownCanonReference(ExternalRef),
    MalformedCanonFragment(ExternalRef),
    CanonLimitExceeded { supplied: usize, maximum: usize },
    NearbyLimitExceeded { supplied: usize, maximum: usize },
    ExternalReference(String),
    ReplayRecordMissing(CompositionRecordId),
}

impl fmt::Display for RetrievalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic retrieval rejected request: {self:?}")
    }
}

impl std::error::Error for RetrievalError {}
