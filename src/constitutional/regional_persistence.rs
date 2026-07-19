//! Versioned persistence for the region-bound Synthesis aggregate.
//!
//! Archives persist accepted commands and their constitutional inputs. Loading
//! re-submits those inputs to the production reducer, so assignments and
//! lineage are reconstructed by law rather than trusted as mutable snapshots.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use crate::composition::ExternalRef;
use crate::hollow_grove_contract::House;
use crate::institution::{InstitutionId, OfficeId, SiteId};
use crate::lineage_contract::SandmanorForm;

use super::*;

const REGIONAL_MAGIC: &[u8; 8] = b"HGREGV2\0";
const MAX_ARCHIVE_ITEMS: usize = 1_000_000;
pub const REGIONAL_ARCHIVE_VERSION: u16 = 1;
pub const REGIONAL_LEGACY_ARCHIVE_VERSION: u16 = 0;

#[derive(Debug)]
pub enum RegionalArchiveError {
    Io(io::Error),
    InvalidMagic,
    UnsupportedVersion(u16),
    Truncated,
    InvalidUtf8,
    InvalidTag { kind: &'static str, tag: u8 },
    InvalidIdentifier(String),
    InvalidExternalReference(String),
    LengthOverflow,
    ItemLimitExceeded(u64),
    SequenceMismatch { expected: u64, actual: u64 },
    TrailingBytes,
    Runtime(RegionalSynthesisError),
}

impl fmt::Display for RegionalArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "regional archive error: {self:?}")
    }
}

impl std::error::Error for RegionalArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Runtime(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for RegionalArchiveError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<RegionalSynthesisError> for RegionalArchiveError {
    fn from(value: RegionalSynthesisError) -> Self {
        Self::Runtime(value)
    }
}

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn string(&mut self, value: &str) -> Result<(), RegionalArchiveError> {
        self.u64(u64::try_from(value.len()).map_err(|_| RegionalArchiveError::LengthOverflow)?);
        self.bytes.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn list<T>(
        &mut self,
        values: &[T],
        mut write: impl FnMut(&mut Self, &T) -> Result<(), RegionalArchiveError>,
    ) -> Result<(), RegionalArchiveError> {
        self.u64(u64::try_from(values.len()).map_err(|_| RegionalArchiveError::LengthOverflow)?);
        for value in values {
            write(self, value)?;
        }
        Ok(())
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], RegionalArchiveError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(RegionalArchiveError::LengthOverflow)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(RegionalArchiveError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn u8(&mut self) -> Result<u8, RegionalArchiveError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, RegionalArchiveError> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().expect("two bytes"),
        ))
    }

    fn u64(&mut self) -> Result<u64, RegionalArchiveError> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().expect("eight bytes"),
        ))
    }

    fn length(&mut self) -> Result<usize, RegionalArchiveError> {
        let declared = self.u64()?;
        let length = usize::try_from(declared).map_err(|_| RegionalArchiveError::LengthOverflow)?;
        if length > MAX_ARCHIVE_ITEMS {
            return Err(RegionalArchiveError::ItemLimitExceeded(declared));
        }
        Ok(length)
    }

    fn string(&mut self) -> Result<String, RegionalArchiveError> {
        let length = self.length()?;
        String::from_utf8(self.take(length)?.to_vec())
            .map_err(|_| RegionalArchiveError::InvalidUtf8)
    }

    fn list<T>(
        &mut self,
        mut read: impl FnMut(&mut Self) -> Result<T, RegionalArchiveError>,
    ) -> Result<Vec<T>, RegionalArchiveError> {
        let length = self.length()?;
        let mut values = Vec::with_capacity(length);
        for _ in 0..length {
            values.push(read(self)?);
        }
        Ok(values)
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

macro_rules! regional_id_codec {
    ($write:ident, $read:ident, $ty:ty) => {
        fn $write(writer: &mut Writer, value: &$ty) -> Result<(), RegionalArchiveError> {
            writer.string(value.as_str())
        }

        fn $read(reader: &mut Reader<'_>) -> Result<$ty, RegionalArchiveError> {
            let value = reader.string()?;
            <$ty>::new(value.clone()).map_err(|_| RegionalArchiveError::InvalidIdentifier(value))
        }
    };
}

regional_id_codec!(write_being_id, read_being_id, RegionalBeingId);
regional_id_codec!(
    write_regional_event_id,
    read_regional_event_id,
    RegionalEventId
);
regional_id_codec!(write_synthesis_id, read_synthesis_id, RegionalSynthesisId);
regional_id_codec!(write_rule_set_id, read_rule_set_id, RuleSetId);
regional_id_codec!(write_decision_id, read_decision_id, HouseDecisionId);
regional_id_codec!(write_actor_id, read_actor_id, AuthorityActorId);
regional_id_codec!(write_tombstone_id, read_tombstone_id, TombstoneId);

fn write_position(writer: &mut Writer, value: CausalPosition) {
    writer.u64(value.get());
}

fn read_position(reader: &mut Reader<'_>) -> Result<CausalPosition, RegionalArchiveError> {
    Ok(CausalPosition::new(reader.u64()?))
}

fn write_external(writer: &mut Writer, value: &EvidenceRef) -> Result<(), RegionalArchiveError> {
    writer.string(&value.0.namespace)?;
    writer.string(&value.0.key)
}

fn read_external(reader: &mut Reader<'_>) -> Result<EvidenceRef, RegionalArchiveError> {
    let namespace = reader.string()?;
    let key = reader.string()?;
    ExternalRef::new(namespace, key)
        .map(EvidenceRef)
        .map_err(|error| RegionalArchiveError::InvalidExternalReference(error.to_string()))
}

fn write_institution(
    writer: &mut Writer,
    value: &InstitutionId,
) -> Result<(), RegionalArchiveError> {
    writer.string(value.as_str())
}

fn read_institution(reader: &mut Reader<'_>) -> Result<InstitutionId, RegionalArchiveError> {
    let value = reader.string()?;
    InstitutionId::new(value.clone()).map_err(|_| RegionalArchiveError::InvalidIdentifier(value))
}

fn write_office(writer: &mut Writer, value: &OfficeId) -> Result<(), RegionalArchiveError> {
    writer.string(value.as_str())
}

fn read_office(reader: &mut Reader<'_>) -> Result<OfficeId, RegionalArchiveError> {
    let value = reader.string()?;
    OfficeId::new(value.clone()).map_err(|_| RegionalArchiveError::InvalidIdentifier(value))
}

fn write_site(writer: &mut Writer, value: &SiteId) -> Result<(), RegionalArchiveError> {
    writer.string(value.as_str())
}

fn read_site(reader: &mut Reader<'_>) -> Result<SiteId, RegionalArchiveError> {
    let value = reader.string()?;
    SiteId::new(value.clone()).map_err(|_| RegionalArchiveError::InvalidIdentifier(value))
}

fn write_house(writer: &mut Writer, value: House) {
    writer.u8(match value {
        House::Stonebend => 0,
        House::Sandmanor => 1,
        House::Glaushouse => 2,
        House::Flynt => 3,
    });
}

fn read_house(reader: &mut Reader<'_>) -> Result<House, RegionalArchiveError> {
    match reader.u8()? {
        0 => Ok(House::Stonebend),
        1 => Ok(House::Sandmanor),
        2 => Ok(House::Glaushouse),
        3 => Ok(House::Flynt),
        tag => Err(RegionalArchiveError::InvalidTag { kind: "House", tag }),
    }
}

fn write_region(writer: &mut Writer, value: ConstitutionalRegion) {
    writer.u8(match value {
        ConstitutionalRegion::AuraFields => 0,
        ConstitutionalRegion::AuraBeach => 1,
        ConstitutionalRegion::AuraSea => 2,
    });
}

fn read_region(reader: &mut Reader<'_>) -> Result<ConstitutionalRegion, RegionalArchiveError> {
    match reader.u8()? {
        0 => Ok(ConstitutionalRegion::AuraFields),
        1 => Ok(ConstitutionalRegion::AuraBeach),
        2 => Ok(ConstitutionalRegion::AuraSea),
        tag => Err(RegionalArchiveError::InvalidTag {
            kind: "ConstitutionalRegion",
            tag,
        }),
    }
}

fn write_form(writer: &mut Writer, value: SandmanorForm) {
    writer.u8(match value {
        SandmanorForm::Gnome => 0,
        SandmanorForm::Minotaur => 1,
        SandmanorForm::Hecaton => 2,
        SandmanorForm::Elf => 3,
        SandmanorForm::Centaur => 4,
        SandmanorForm::Pegasus => 5,
    });
}

fn read_form(reader: &mut Reader<'_>) -> Result<SandmanorForm, RegionalArchiveError> {
    match reader.u8()? {
        0 => Ok(SandmanorForm::Gnome),
        1 => Ok(SandmanorForm::Minotaur),
        2 => Ok(SandmanorForm::Hecaton),
        3 => Ok(SandmanorForm::Elf),
        4 => Ok(SandmanorForm::Centaur),
        5 => Ok(SandmanorForm::Pegasus),
        tag => Err(RegionalArchiveError::InvalidTag {
            kind: "SandmanorForm",
            tag,
        }),
    }
}

fn write_function(writer: &mut Writer, value: RegionalFunction) {
    writer.u8(match value {
        RegionalFunction::AuraFieldsStewardshipAndDefense => 0,
        RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship => 1,
    });
}

fn read_function(reader: &mut Reader<'_>) -> Result<RegionalFunction, RegionalArchiveError> {
    match reader.u8()? {
        0 => Ok(RegionalFunction::AuraFieldsStewardshipAndDefense),
        1 => Ok(RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship),
        tag => Err(RegionalArchiveError::InvalidTag {
            kind: "RegionalFunction",
            tag,
        }),
    }
}

fn write_subject_evidence(
    writer: &mut Writer,
    value: &SubjectEvidence,
) -> Result<(), RegionalArchiveError> {
    write_being_id(writer, &value.subject)?;
    write_external(writer, &value.reference)
}

fn read_subject_evidence(reader: &mut Reader<'_>) -> Result<SubjectEvidence, RegionalArchiveError> {
    Ok(SubjectEvidence {
        subject: read_being_id(reader)?,
        reference: read_external(reader)?,
    })
}

fn write_jurisdiction(
    writer: &mut Writer,
    value: &RegionalJurisdictionSnapshot,
) -> Result<(), RegionalArchiveError> {
    write_region(writer, value.region);
    write_site(writer, &value.site)?;
    write_institution(writer, &value.institution)?;
    write_house(writer, value.house);
    write_position(writer, value.observed_at);
    writer.list(&value.evidence, write_external)
}

fn read_jurisdiction(
    reader: &mut Reader<'_>,
) -> Result<RegionalJurisdictionSnapshot, RegionalArchiveError> {
    Ok(RegionalJurisdictionSnapshot {
        region: read_region(reader)?,
        site: read_site(reader)?,
        institution: read_institution(reader)?,
        house: read_house(reader)?,
        observed_at: read_position(reader)?,
        evidence: reader.list(read_external)?,
    })
}

fn write_standing(
    writer: &mut Writer,
    value: &RegionalStanding,
) -> Result<(), RegionalArchiveError> {
    write_region(writer, value.region);
    writer.u8(match value.kind {
        RegionalStandingKind::Established => 0,
        RegionalStandingKind::Visitor => 1,
    });
    write_jurisdiction(writer, &value.jurisdiction)?;
    writer.list(&value.evidence, write_subject_evidence)
}

fn read_standing(reader: &mut Reader<'_>) -> Result<RegionalStanding, RegionalArchiveError> {
    let region = read_region(reader)?;
    let kind = match reader.u8()? {
        0 => RegionalStandingKind::Established,
        1 => RegionalStandingKind::Visitor,
        tag => {
            return Err(RegionalArchiveError::InvalidTag {
                kind: "RegionalStandingKind",
                tag,
            });
        }
    };
    Ok(RegionalStanding {
        region,
        kind,
        jurisdiction: read_jurisdiction(reader)?,
        evidence: reader.list(read_subject_evidence)?,
    })
}

fn write_registration(
    writer: &mut Writer,
    value: &RegionalBeingRegistration,
) -> Result<(), RegionalArchiveError> {
    write_being_id(writer, &value.id)?;
    write_form(writer, value.form);
    write_standing(writer, &value.standing)?;
    writer.list(&value.evidence, write_subject_evidence)
}

fn read_registration(
    reader: &mut Reader<'_>,
) -> Result<RegionalBeingRegistration, RegionalArchiveError> {
    Ok(RegionalBeingRegistration {
        id: read_being_id(reader)?,
        form: read_form(reader)?,
        standing: read_standing(reader)?,
        evidence: reader.list(read_subject_evidence)?,
    })
}

fn write_house_function(writer: &mut Writer, value: HouseFunction) {
    writer.u8(match value {
        HouseFunction::Name => 0,
        HouseFunction::Prove => 1,
        HouseFunction::Clear => 2,
        HouseFunction::Recognize => 3,
        HouseFunction::Resolve => 4,
    });
}

fn read_house_function(reader: &mut Reader<'_>) -> Result<HouseFunction, RegionalArchiveError> {
    match reader.u8()? {
        0 => Ok(HouseFunction::Name),
        1 => Ok(HouseFunction::Prove),
        2 => Ok(HouseFunction::Clear),
        3 => Ok(HouseFunction::Recognize),
        4 => Ok(HouseFunction::Resolve),
        tag => Err(RegionalArchiveError::InvalidTag {
            kind: "HouseFunction",
            tag,
        }),
    }
}

fn write_decision(writer: &mut Writer, value: &HouseDecision) -> Result<(), RegionalArchiveError> {
    write_decision_id(writer, &value.id)?;
    write_house_function(writer, value.function);
    write_actor_id(writer, &value.authority.actor)?;
    write_office(writer, &value.authority.office)?;
    match &value.authority.institution {
        Some(institution) => {
            writer.u8(1);
            write_institution(writer, institution)?;
        }
        None => writer.u8(0),
    }
    write_house(writer, value.authority.house);
    writer.list(&value.authority.authorities, |writer, authority| {
        writer.string(authority)
    })?;
    write_position(writer, value.authority.observed_at);
    writer.u8(match value.outcome {
        HouseDecisionOutcome::Accepted => 0,
        HouseDecisionOutcome::Rejected => 1,
        HouseDecisionOutcome::Inconclusive => 2,
    });
    writer.list(&value.evidence, write_external)?;
    write_position(writer, value.causal_position);
    Ok(())
}

fn read_decision(reader: &mut Reader<'_>) -> Result<HouseDecision, RegionalArchiveError> {
    let id = read_decision_id(reader)?;
    let function = read_house_function(reader)?;
    let actor = read_actor_id(reader)?;
    let office = read_office(reader)?;
    let institution = match reader.u8()? {
        0 => None,
        1 => Some(read_institution(reader)?),
        tag => {
            return Err(RegionalArchiveError::InvalidTag {
                kind: "Option<InstitutionId>",
                tag,
            });
        }
    };
    let house = read_house(reader)?;
    let authorities = reader.list(|reader| reader.string())?;
    let observed_at = read_position(reader)?;
    let outcome = match reader.u8()? {
        0 => HouseDecisionOutcome::Accepted,
        1 => HouseDecisionOutcome::Rejected,
        2 => HouseDecisionOutcome::Inconclusive,
        tag => {
            return Err(RegionalArchiveError::InvalidTag {
                kind: "HouseDecisionOutcome",
                tag,
            });
        }
    };
    Ok(HouseDecision {
        id,
        function,
        authority: AuthoritySnapshot {
            actor,
            office,
            institution,
            house,
            authorities,
            observed_at,
        },
        outcome,
        evidence: reader.list(read_external)?,
        causal_position: read_position(reader)?,
    })
}

fn write_prerequisites(
    writer: &mut Writer,
    value: &RegionalSynthesisPrerequisites,
) -> Result<(), RegionalArchiveError> {
    write_subject_evidence(writer, &value.standing)?;
    write_subject_evidence(writer, &value.lineage)?;
    write_subject_evidence(writer, &value.readiness)?;
    write_subject_evidence(writer, &value.constitutional_rule)?;
    writer.list(&value.supporting, write_subject_evidence)
}

fn read_prerequisites(
    reader: &mut Reader<'_>,
) -> Result<RegionalSynthesisPrerequisites, RegionalArchiveError> {
    Ok(RegionalSynthesisPrerequisites {
        standing: read_subject_evidence(reader)?,
        lineage: read_subject_evidence(reader)?,
        readiness: read_subject_evidence(reader)?,
        constitutional_rule: read_subject_evidence(reader)?,
        supporting: reader.list(read_subject_evidence)?,
    })
}

fn write_command(
    writer: &mut Writer,
    value: &RegionalSynthesisCommand,
) -> Result<(), RegionalArchiveError> {
    write_synthesis_id(writer, &value.id)?;
    write_being_id(writer, &value.predecessor)?;
    write_being_id(writer, &value.result)?;
    write_form(writer, value.expected_predecessor_form);
    write_form(writer, value.requested_result_form);
    write_region(writer, value.requested_region);
    write_function(writer, value.requested_function);
    write_prerequisites(writer, &value.prerequisites)?;
    write_decision(writer, &value.authority.sandmanor_proof)?;
    write_decision(writer, &value.authority.glaushouse_resolution)?;
    writer.list(&value.evidence, write_subject_evidence)
}

fn read_command(reader: &mut Reader<'_>) -> Result<RegionalSynthesisCommand, RegionalArchiveError> {
    Ok(RegionalSynthesisCommand {
        id: read_synthesis_id(reader)?,
        predecessor: read_being_id(reader)?,
        result: read_being_id(reader)?,
        expected_predecessor_form: read_form(reader)?,
        requested_result_form: read_form(reader)?,
        requested_region: read_region(reader)?,
        requested_function: read_function(reader)?,
        prerequisites: read_prerequisites(reader)?,
        authority: RegionalSynthesisAuthority {
            sandmanor_proof: read_decision(reader)?,
            glaushouse_resolution: read_decision(reader)?,
        },
        evidence: reader.list(read_subject_evidence)?,
    })
}

fn write_tombstone(
    writer: &mut Writer,
    value: &RegionalTombstoneRecord,
) -> Result<(), RegionalArchiveError> {
    write_being_id(writer, &value.being)?;
    write_tombstone_id(writer, &value.tombstone)?;
    writer.list(&value.evidence, write_subject_evidence)
}

fn read_tombstone(
    reader: &mut Reader<'_>,
) -> Result<RegionalTombstoneRecord, RegionalArchiveError> {
    Ok(RegionalTombstoneRecord {
        being: read_being_id(reader)?,
        tombstone: read_tombstone_id(reader)?,
        evidence: reader.list(read_subject_evidence)?,
    })
}

fn encode_with_version(
    runtime: &RegionalSynthesisRuntime,
    version: u16,
) -> Result<Vec<u8>, RegionalArchiveError> {
    let mut writer = Writer::new();
    writer.bytes.extend_from_slice(REGIONAL_MAGIC);
    writer.u16(version);
    writer.u64(
        u64::try_from(runtime.events().len()).map_err(|_| RegionalArchiveError::LengthOverflow)?,
    );
    for event in runtime.events() {
        write_regional_event_id(&mut writer, &event.id)?;
        writer.u64(event.sequence);
        write_position(&mut writer, event.causal_position);
        write_rule_set_id(&mut writer, &event.rule_set)?;
        match &event.payload {
            RegionalEvent::BeingRegistered(registration) => {
                writer.u8(0);
                write_registration(&mut writer, registration)?;
            }
            RegionalEvent::SynthesisCompleted(record) => {
                writer.u8(1);
                write_command(&mut writer, &record.command)?;
            }
            RegionalEvent::BeingTombstoned(record) => {
                writer.u8(2);
                write_tombstone(&mut writer, record)?;
            }
        }
    }
    Ok(writer.bytes)
}

/// Encodes the current canonical regional archive format.
pub fn encode_regional_archive(
    runtime: &RegionalSynthesisRuntime,
) -> Result<Vec<u8>, RegionalArchiveError> {
    encode_with_version(runtime, REGIONAL_ARCHIVE_VERSION)
}

/// Produces the accepted V0 fixture format for migration conformance tests.
pub fn encode_legacy_regional_archive_v0(
    runtime: &RegionalSynthesisRuntime,
) -> Result<Vec<u8>, RegionalArchiveError> {
    encode_with_version(runtime, REGIONAL_LEGACY_ARCHIVE_VERSION)
}

/// Loads an archive by reducing every persisted command through production law.
pub fn decode_regional_archive(
    bytes: &[u8],
) -> Result<RegionalSynthesisRuntime, RegionalArchiveError> {
    let mut reader = Reader::new(bytes);
    if reader.take(REGIONAL_MAGIC.len())? != REGIONAL_MAGIC {
        return Err(RegionalArchiveError::InvalidMagic);
    }
    let version = reader.u16()?;
    if !matches!(
        version,
        REGIONAL_LEGACY_ARCHIVE_VERSION | REGIONAL_ARCHIVE_VERSION
    ) {
        return Err(RegionalArchiveError::UnsupportedVersion(version));
    }
    let event_count = reader.length()?;
    let mut runtime = RegionalSynthesisRuntime::new();
    for expected_sequence in 0..event_count {
        let metadata = RegionalEventMetadata {
            id: read_regional_event_id(&mut reader)?,
            causal_position: {
                let sequence = reader.u64()?;
                let actual = u64::try_from(expected_sequence)
                    .map_err(|_| RegionalArchiveError::LengthOverflow)?;
                if sequence != actual {
                    return Err(RegionalArchiveError::SequenceMismatch {
                        expected: actual,
                        actual: sequence,
                    });
                }
                read_position(&mut reader)?
            },
            rule_set: read_rule_set_id(&mut reader)?,
        };
        match reader.u8()? {
            0 => {
                runtime.register_being(metadata, read_registration(&mut reader)?)?;
            }
            1 => {
                runtime.synthesize(metadata, read_command(&mut reader)?)?;
            }
            2 => {
                runtime.tombstone_being(metadata, read_tombstone(&mut reader)?)?;
            }
            tag => {
                return Err(RegionalArchiveError::InvalidTag {
                    kind: "RegionalEvent",
                    tag,
                });
            }
        }
    }
    if !reader.is_finished() {
        return Err(RegionalArchiveError::TrailingBytes);
    }
    Ok(runtime)
}

/// Migrates any supported regional archive to the current canonical version.
pub fn migrate_regional_archive(bytes: &[u8]) -> Result<Vec<u8>, RegionalArchiveError> {
    encode_regional_archive(&decode_regional_archive(bytes)?)
}

pub fn write_regional_archive(
    path: impl AsRef<Path>,
    runtime: &RegionalSynthesisRuntime,
) -> Result<(), RegionalArchiveError> {
    fs::write(path, encode_regional_archive(runtime)?)?;
    Ok(())
}

pub fn read_regional_archive(
    path: impl AsRef<Path>,
) -> Result<RegionalSynthesisRuntime, RegionalArchiveError> {
    decode_regional_archive(&fs::read(path)?)
}

/// Stable FNV-1a digest of the current canonical archive bytes.
pub fn regional_archive_digest(
    runtime: &RegionalSynthesisRuntime,
) -> Result<u64, RegionalArchiveError> {
    let bytes = encode_regional_archive(runtime)?;
    let mut digest = 0xcbf29ce484222325_u64;
    for byte in bytes {
        digest ^= u64::from(byte);
        digest = digest.wrapping_mul(0x100000001b3);
    }
    Ok(digest)
}
