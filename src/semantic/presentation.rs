//! Read-only human and JSON projections of witnessed semantic data.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{Position3, squared_distance, vertical_origin_relation};

use super::{KernelAnchor, SemanticDatasetDecision, SemanticWitnessDatasetRecord, house_distances};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNearbyDatasetReading {
    pub witness_ref: String,
    pub point_ref: String,
    pub scale_key: String,
    pub position: Position3,
    pub squared_distance: i64,
    pub semantic_tags: Vec<String>,
}

pub fn inspect_semantic_dataset<'a>(
    records: &'a [SemanticWitnessDatasetRecord],
    point_ref: &str,
    scale_key: &str,
) -> Result<&'a SemanticWitnessDatasetRecord, SemanticPresentationError> {
    records
        .iter()
        .filter(|record| record.point_ref == point_ref && record.scope.scale_key == scale_key)
        .max_by(|left, right| left.witness_ref.cmp(&right.witness_ref))
        .ok_or_else(|| SemanticPresentationError::RecordNotFound {
            point_ref: point_ref.into(),
            scale_key: scale_key.into(),
        })
}

#[must_use]
pub fn semantic_dataset_history<'a>(
    records: &'a [SemanticWitnessDatasetRecord],
    point_ref: &str,
    scale_key: &str,
) -> Vec<&'a SemanticWitnessDatasetRecord> {
    let mut selected = records
        .iter()
        .filter(|record| record.point_ref == point_ref && record.scope.scale_key == scale_key)
        .collect::<Vec<_>>();
    selected.sort_by(|left, right| left.witness_ref.cmp(&right.witness_ref));
    selected
}

#[must_use]
pub fn semantic_dataset_nearby(
    records: &[SemanticWitnessDatasetRecord],
    scale_key: &str,
    center: Position3,
    maximum_results: usize,
) -> Vec<SemanticNearbyDatasetReading> {
    let mut selected = records
        .iter()
        .filter(|record| record.decision == SemanticDatasetDecision::Confirmed)
        .filter_map(|record| {
            record
                .confirmed_position
                .map(|position| SemanticNearbyDatasetReading {
                    witness_ref: record.witness_ref.clone(),
                    point_ref: record.point_ref.clone(),
                    scale_key: record.scope.scale_key.clone(),
                    position,
                    squared_distance: squared_distance(center, position),
                    semantic_tags: record.semantic_tags.clone(),
                })
        })
        .collect::<Vec<_>>();
    selected.sort_by(|left, right| {
        (left.scale_key != scale_key)
            .cmp(&(right.scale_key != scale_key))
            .then_with(|| left.squared_distance.cmp(&right.squared_distance))
            .then_with(|| left.witness_ref.cmp(&right.witness_ref))
    });
    selected.truncate(maximum_results);
    selected
}

#[must_use]
pub fn format_semantic_dataset_record(record: &SemanticWitnessDatasetRecord) -> String {
    let mut output = String::new();
    output.push_str("Point:\n");
    output.push_str(&record.point_ref);
    output.push_str("\n\nScale:\n");
    output.push_str(&record.scope.scale_key);
    output.push_str("\n\nCurrent position:\n");
    output.push_str(
        &record
            .prior_position
            .map_or_else(|| "None".into(), format_position),
    );
    output.push_str("\n\nNearest relationships:\n");
    for distance in house_distances(record.proposed_position) {
        output.push_str(KernelAnchor::House(distance.house).canonical_name());
        output.push_str(" (");
        output.push_str(&distance.squared_distance.to_string());
        output.push_str(")\n");
    }
    output.push_str("\nProposed position:\n");
    output.push_str(&format_position(record.proposed_position));
    output.push_str("\n\nMovement:\n");
    output.push_str(&record.translation.map_or_else(
        || "Position assertion".into(),
        |translation| {
            format!(
                "DX {:+}\nDY {:+}\nDZ {:+}",
                translation.dx().get(),
                translation.dy().get(),
                translation.dz().get()
            )
        },
    ));
    output.push_str("\n\nOrigin relation:\n");
    output.push_str(&record.prior_position.map_or_else(
        || "Not applicable".into(),
        |prior| {
            format!(
                "{:?}",
                vertical_origin_relation(prior, record.proposed_position)
            )
        },
    ));
    output.push_str("\n\nStatus:\n");
    match record.decision {
        SemanticDatasetDecision::Confirmed => {
            output.push_str("Confirmed\n\nConfirmed position:\n");
            output.push_str(
                &record
                    .confirmed_position
                    .map_or_else(|| "Missing".into(), format_position),
            );
            output.push_str("\n\nWitness:\n");
            output.push_str(&record.witness_ref);
        }
        SemanticDatasetDecision::Rejected => {
            output.push_str("Rejected\n\nReasons:\n");
            for reason in &record.rejection_reasons {
                output.push_str("- ");
                output.push_str(reason);
                output.push('\n');
            }
            output.push_str("\nWitness:\n");
            output.push_str(&record.witness_ref);
        }
    }
    output
}

fn format_position(position: Position3) -> String {
    format!(
        "X {:+}\nY {:+}\nZ {:+}",
        position.x().get(),
        position.y().get(),
        position.z().get()
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticPresentationError {
    RecordNotFound {
        point_ref: String,
        scale_key: String,
    },
}

impl fmt::Display for SemanticPresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic presentation failed: {self:?}")
    }
}

impl std::error::Error for SemanticPresentationError {}
