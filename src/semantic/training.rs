//! Deterministic preparation of witnessed data for future shared-adapter work.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;

use crate::artifact_io::write_bytes_atomically;

use super::{SemanticDatasetError, SemanticWitnessDatasetRecord, decode_semantic_witness_jsonl};

pub const VALIDATION_MILLIS_MAX: u16 = 1000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDatasetSplit {
    pub train_bytes: Vec<u8>,
    pub validation_bytes: Vec<u8>,
    pub train_records: usize,
    pub validation_records: usize,
    pub seed: u64,
    pub validation_millis: u16,
}

pub fn split_semantic_witness_jsonl(
    bytes: &[u8],
    seed: u64,
    validation_millis: u16,
) -> Result<SemanticDatasetSplit, SemanticTrainingError> {
    if validation_millis > VALIDATION_MILLIS_MAX {
        return Err(SemanticTrainingError::InvalidValidationShare {
            supplied: validation_millis,
            maximum: VALIDATION_MILLIS_MAX,
        });
    }
    let mut records = decode_semantic_witness_jsonl(bytes)?;
    records.sort_by(|left, right| left.witness_ref.cmp(&right.witness_ref));
    ensure_unique_witnesses(&records)?;

    let mut train = Vec::new();
    let mut validation = Vec::new();
    for record in records {
        if split_bucket(seed, &record.witness_ref) < u64::from(validation_millis) {
            validation.push(record);
        } else {
            train.push(record);
        }
    }

    Ok(SemanticDatasetSplit {
        train_bytes: encode_records(&train)?,
        validation_bytes: encode_records(&validation)?,
        train_records: train.len(),
        validation_records: validation.len(),
        seed,
        validation_millis,
    })
}

pub fn write_semantic_dataset_split(
    input_path: &Path,
    train_path: &Path,
    validation_path: &Path,
    seed: u64,
    validation_millis: u16,
) -> Result<SemanticDatasetSplit, SemanticTrainingError> {
    if train_path == validation_path {
        return Err(SemanticTrainingError::DuplicateOutputPath);
    }
    let bytes =
        fs::read(input_path).map_err(|error| SemanticTrainingError::Read(error.to_string()))?;
    let split = split_semantic_witness_jsonl(&bytes, seed, validation_millis)?;
    write_bytes_atomically(train_path, &split.train_bytes)
        .map_err(|error| SemanticTrainingError::Write(error.to_string()))?;
    write_bytes_atomically(validation_path, &split.validation_bytes)
        .map_err(|error| SemanticTrainingError::Write(error.to_string()))?;
    Ok(split)
}

fn ensure_unique_witnesses(
    records: &[SemanticWitnessDatasetRecord],
) -> Result<(), SemanticTrainingError> {
    let mut witnesses = BTreeSet::new();
    for record in records {
        if !witnesses.insert(record.witness_ref.clone()) {
            return Err(SemanticTrainingError::DuplicateWitnessRef(
                record.witness_ref.clone(),
            ));
        }
    }
    Ok(())
}

fn encode_records(
    records: &[SemanticWitnessDatasetRecord],
) -> Result<Vec<u8>, SemanticTrainingError> {
    let mut output = Vec::new();
    for record in records {
        output.extend(
            serde_json::to_vec(record)
                .map_err(|error| SemanticTrainingError::Json(error.to_string()))?,
        );
        output.push(b'\n');
    }
    Ok(output)
}

fn split_bucket(seed: u64, witness_ref: &str) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = FNV_OFFSET;
    for byte in seed.to_le_bytes().into_iter().chain(witness_ref.bytes()) {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash % 1000
}

#[derive(Debug)]
pub enum SemanticTrainingError {
    Dataset(SemanticDatasetError),
    InvalidValidationShare { supplied: u16, maximum: u16 },
    DuplicateWitnessRef(String),
    DuplicateOutputPath,
    Json(String),
    Read(String),
    Write(String),
}

impl From<SemanticDatasetError> for SemanticTrainingError {
    fn from(value: SemanticDatasetError) -> Self {
        Self::Dataset(value)
    }
}

impl fmt::Display for SemanticTrainingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "semantic training preparation rejected state: {self:?}"
        )
    }
}

impl std::error::Error for SemanticTrainingError {}
