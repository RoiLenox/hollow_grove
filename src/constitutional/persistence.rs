use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use crate::composition::ExternalRef;
use crate::hollow_grove_contract::House;
use crate::institution::{InstitutionId, OfficeId};

use super::*;

const MAGIC: &[u8; 8] = b"HGCONST\0";
pub const CONSTITUTIONAL_ARCHIVE_VERSION: u16 = 1;

#[derive(Debug)]
pub enum ConstitutionalArchiveError {
    Io(io::Error),
    InvalidMagic,
    UnsupportedVersion(u16),
    Truncated,
    InvalidUtf8,
    InvalidTag { kind: &'static str, tag: u8 },
    InvalidIdentifier(String),
    InvalidExternalReference(String),
    LengthOverflow,
    TrailingBytes,
    Runtime(ConstitutionalRuntimeError),
}

impl fmt::Display for ConstitutionalArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "constitutional archive error: {self:?}")
    }
}

impl std::error::Error for ConstitutionalArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Runtime(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ConstitutionalArchiveError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ConstitutionalRuntimeError> for ConstitutionalArchiveError {
    fn from(value: ConstitutionalRuntimeError) -> Self {
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

    fn u128(&mut self, value: u128) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn string(&mut self, value: &str) -> Result<(), ConstitutionalArchiveError> {
        let length =
            u64::try_from(value.len()).map_err(|_| ConstitutionalArchiveError::LengthOverflow)?;
        self.u64(length);
        self.bytes.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn list<T>(
        &mut self,
        values: &[T],
        mut write: impl FnMut(&mut Self, &T) -> Result<(), ConstitutionalArchiveError>,
    ) -> Result<(), ConstitutionalArchiveError> {
        self.u64(
            u64::try_from(values.len()).map_err(|_| ConstitutionalArchiveError::LengthOverflow)?,
        );
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

    fn take(&mut self, length: usize) -> Result<&'a [u8], ConstitutionalArchiveError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ConstitutionalArchiveError::LengthOverflow)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(ConstitutionalArchiveError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn u8(&mut self) -> Result<u8, ConstitutionalArchiveError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, ConstitutionalArchiveError> {
        Ok(u16::from_le_bytes(
            self.take(2)?.try_into().expect("two bytes"),
        ))
    }

    fn u64(&mut self) -> Result<u64, ConstitutionalArchiveError> {
        Ok(u64::from_le_bytes(
            self.take(8)?.try_into().expect("eight bytes"),
        ))
    }

    fn u128(&mut self) -> Result<u128, ConstitutionalArchiveError> {
        Ok(u128::from_le_bytes(
            self.take(16)?.try_into().expect("sixteen bytes"),
        ))
    }

    fn string(&mut self) -> Result<String, ConstitutionalArchiveError> {
        let length =
            usize::try_from(self.u64()?).map_err(|_| ConstitutionalArchiveError::LengthOverflow)?;
        String::from_utf8(self.take(length)?.to_vec())
            .map_err(|_| ConstitutionalArchiveError::InvalidUtf8)
    }

    fn list<T>(
        &mut self,
        mut read: impl FnMut(&mut Self) -> Result<T, ConstitutionalArchiveError>,
    ) -> Result<Vec<T>, ConstitutionalArchiveError> {
        let length =
            usize::try_from(self.u64()?).map_err(|_| ConstitutionalArchiveError::LengthOverflow)?;
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

macro_rules! constitutional_id_codec {
    ($write:ident, $read:ident, $ty:ty) => {
        fn $write(writer: &mut Writer, value: &$ty) -> Result<(), ConstitutionalArchiveError> {
            writer.string(value.as_str())
        }

        fn $read(reader: &mut Reader<'_>) -> Result<$ty, ConstitutionalArchiveError> {
            let value = reader.string()?;
            <$ty>::new(value.clone())
                .map_err(|_| ConstitutionalArchiveError::InvalidIdentifier(value))
        }
    };
}

constitutional_id_codec!(write_wave_id, read_wave_id, WaveId);
constitutional_id_codec!(write_bond_id, read_bond_id, BondId);
constitutional_id_codec!(write_event_id, read_event_id, ConstitutionalEventId);
constitutional_id_codec!(write_participant_id, read_participant_id, ParticipantId);
constitutional_id_codec!(write_role_id, read_role_id, RoleId);
constitutional_id_codec!(write_obligation_id, read_obligation_id, ObligationId);
constitutional_id_codec!(write_permission_id, read_permission_id, PermissionId);
constitutional_id_codec!(write_rule_set_id, read_rule_set_id, RuleSetId);
constitutional_id_codec!(write_unit_id, read_unit_id, UnitId);
constitutional_id_codec!(
    write_house_decision_id,
    read_house_decision_id,
    HouseDecisionId
);
constitutional_id_codec!(
    write_authority_actor_id,
    read_authority_actor_id,
    AuthorityActorId
);
constitutional_id_codec!(
    write_transaction_id,
    read_transaction_id,
    CurrentTransactionId
);
constitutional_id_codec!(write_observation_id, read_observation_id, AuraObservationId);
constitutional_id_codec!(write_evaluation_id, read_evaluation_id, EvaluationId);
constitutional_id_codec!(write_tombstone_id, read_tombstone_id, TombstoneId);
constitutional_id_codec!(write_toke_id, read_toke_id, TokeId);
constitutional_id_codec!(write_resolution_id, read_resolution_id, ResolutionId);
constitutional_id_codec!(write_challenge_id, read_challenge_id, ChallengeId);
constitutional_id_codec!(write_default_id, read_default_id, DefaultId);

fn write_institution_id(
    writer: &mut Writer,
    value: &InstitutionId,
) -> Result<(), ConstitutionalArchiveError> {
    writer.string(value.as_str())
}

fn read_institution_id(
    reader: &mut Reader<'_>,
) -> Result<InstitutionId, ConstitutionalArchiveError> {
    let value = reader.string()?;
    InstitutionId::new(value.clone())
        .map_err(|_| ConstitutionalArchiveError::InvalidIdentifier(value))
}

fn write_office_id(
    writer: &mut Writer,
    value: &OfficeId,
) -> Result<(), ConstitutionalArchiveError> {
    writer.string(value.as_str())
}

fn read_office_id(reader: &mut Reader<'_>) -> Result<OfficeId, ConstitutionalArchiveError> {
    let value = reader.string()?;
    OfficeId::new(value.clone()).map_err(|_| ConstitutionalArchiveError::InvalidIdentifier(value))
}

fn write_evidence(
    writer: &mut Writer,
    value: &EvidenceRef,
) -> Result<(), ConstitutionalArchiveError> {
    writer.string(&value.0.namespace)?;
    writer.string(&value.0.key)
}

fn read_evidence(reader: &mut Reader<'_>) -> Result<EvidenceRef, ConstitutionalArchiveError> {
    let namespace = reader.string()?;
    let key = reader.string()?;
    ExternalRef::new(namespace, key)
        .map(EvidenceRef)
        .map_err(|error| ConstitutionalArchiveError::InvalidExternalReference(error.to_string()))
}

fn write_house(writer: &mut Writer, value: House) {
    writer.u8(match value {
        House::Stonebend => 0,
        House::Sandmanor => 1,
        House::Glaushouse => 2,
        House::Flynt => 3,
    });
}

fn read_house(reader: &mut Reader<'_>) -> Result<House, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(House::Stonebend),
        1 => Ok(House::Sandmanor),
        2 => Ok(House::Glaushouse),
        3 => Ok(House::Flynt),
        tag => Err(ConstitutionalArchiveError::InvalidTag { kind: "House", tag }),
    }
}

fn write_sign(writer: &mut Writer, value: Sign) {
    writer.u8(match value {
        Sign::Positive => 0,
        Sign::Negative => 1,
    });
}

fn read_sign(reader: &mut Reader<'_>) -> Result<Sign, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(Sign::Positive),
        1 => Ok(Sign::Negative),
        tag => Err(ConstitutionalArchiveError::InvalidTag { kind: "Sign", tag }),
    }
}

fn write_position(writer: &mut Writer, value: CausalPosition) {
    writer.u64(value.get());
}

fn read_position(reader: &mut Reader<'_>) -> Result<CausalPosition, ConstitutionalArchiveError> {
    Ok(CausalPosition::new(reader.u64()?))
}

fn write_quantity(
    writer: &mut Writer,
    value: &SignedQuantity,
) -> Result<(), ConstitutionalArchiveError> {
    write_sign(writer, value.sign);
    writer.u128(value.magnitude);
    write_unit_id(writer, &value.unit)
}

fn read_quantity(reader: &mut Reader<'_>) -> Result<SignedQuantity, ConstitutionalArchiveError> {
    let sign = read_sign(reader)?;
    let magnitude = reader.u128()?;
    let unit = read_unit_id(reader)?;
    SignedQuantity::new(sign, magnitude, unit)
        .map_err(|error| ConstitutionalArchiveError::InvalidExternalReference(error.to_string()))
}

fn write_totals(
    writer: &mut Writer,
    value: &SignedTotals,
) -> Result<(), ConstitutionalArchiveError> {
    writer.u128(value.positive);
    writer.u128(value.negative);
    write_unit_id(writer, &value.unit)
}

fn read_totals(reader: &mut Reader<'_>) -> Result<SignedTotals, ConstitutionalArchiveError> {
    Ok(SignedTotals {
        positive: reader.u128()?,
        negative: reader.u128()?,
        unit: read_unit_id(reader)?,
    })
}

fn write_excess(writer: &mut Writer, value: &NetExcess) -> Result<(), ConstitutionalArchiveError> {
    match value.sign {
        None => writer.u8(0),
        Some(sign) => {
            writer.u8(1);
            write_sign(writer, sign);
        }
    }
    writer.u128(value.magnitude);
    write_unit_id(writer, &value.unit)
}

fn read_excess(reader: &mut Reader<'_>) -> Result<NetExcess, ConstitutionalArchiveError> {
    let sign = match reader.u8()? {
        0 => None,
        1 => Some(read_sign(reader)?),
        tag => {
            return Err(ConstitutionalArchiveError::InvalidTag {
                kind: "Option<Sign>",
                tag,
            });
        }
    };
    Ok(NetExcess {
        sign,
        magnitude: reader.u128()?,
        unit: read_unit_id(reader)?,
    })
}

fn write_term(writer: &mut Writer, value: &BondTerm) {
    match value {
        BondTerm::Finite { end } => {
            writer.u8(0);
            write_position(writer, *end);
        }
        BondTerm::Perpetual => writer.u8(1),
    }
}

fn read_term(reader: &mut Reader<'_>) -> Result<BondTerm, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(BondTerm::Finite {
            end: read_position(reader)?,
        }),
        1 => Ok(BondTerm::Perpetual),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "BondTerm",
            tag,
        }),
    }
}

fn write_wave(writer: &mut Writer, value: &WaveRecord) -> Result<(), ConstitutionalArchiveError> {
    write_wave_id(writer, &value.id)?;
    write_evidence(writer, &value.origin)?;
    write_position(writer, value.causal_position);
    Ok(())
}

fn read_wave(reader: &mut Reader<'_>) -> Result<WaveRecord, ConstitutionalArchiveError> {
    Ok(WaveRecord {
        id: read_wave_id(reader)?,
        origin: read_evidence(reader)?,
        causal_position: read_position(reader)?,
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

fn read_house_function(
    reader: &mut Reader<'_>,
) -> Result<HouseFunction, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(HouseFunction::Name),
        1 => Ok(HouseFunction::Prove),
        2 => Ok(HouseFunction::Clear),
        3 => Ok(HouseFunction::Recognize),
        4 => Ok(HouseFunction::Resolve),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "HouseFunction",
            tag,
        }),
    }
}

fn write_authority(
    writer: &mut Writer,
    value: &AuthoritySnapshot,
) -> Result<(), ConstitutionalArchiveError> {
    write_authority_actor_id(writer, &value.actor)?;
    write_office_id(writer, &value.office)?;
    match &value.institution {
        Some(institution) => {
            writer.u8(1);
            write_institution_id(writer, institution)?;
        }
        None => writer.u8(0),
    }
    write_house(writer, value.house);
    writer.list(&value.authorities, |writer, value| writer.string(value))?;
    write_position(writer, value.observed_at);
    Ok(())
}

fn read_authority(
    reader: &mut Reader<'_>,
) -> Result<AuthoritySnapshot, ConstitutionalArchiveError> {
    let actor = read_authority_actor_id(reader)?;
    let office = read_office_id(reader)?;
    let institution = match reader.u8()? {
        0 => None,
        1 => Some(read_institution_id(reader)?),
        tag => {
            return Err(ConstitutionalArchiveError::InvalidTag {
                kind: "Option<InstitutionId>",
                tag,
            });
        }
    };
    Ok(AuthoritySnapshot {
        actor,
        office,
        institution,
        house: read_house(reader)?,
        authorities: reader.list(|reader| reader.string())?,
        observed_at: read_position(reader)?,
    })
}

fn write_decision(
    writer: &mut Writer,
    value: &HouseDecision,
) -> Result<(), ConstitutionalArchiveError> {
    write_house_decision_id(writer, &value.id)?;
    write_house_function(writer, value.function);
    write_authority(writer, &value.authority)?;
    writer.u8(match value.outcome {
        HouseDecisionOutcome::Accepted => 0,
        HouseDecisionOutcome::Rejected => 1,
        HouseDecisionOutcome::Inconclusive => 2,
    });
    writer.list(&value.evidence, write_evidence)?;
    write_position(writer, value.causal_position);
    Ok(())
}

fn read_decision(reader: &mut Reader<'_>) -> Result<HouseDecision, ConstitutionalArchiveError> {
    let id = read_house_decision_id(reader)?;
    let function = read_house_function(reader)?;
    let authority = read_authority(reader)?;
    let outcome = match reader.u8()? {
        0 => HouseDecisionOutcome::Accepted,
        1 => HouseDecisionOutcome::Rejected,
        2 => HouseDecisionOutcome::Inconclusive,
        tag => {
            return Err(ConstitutionalArchiveError::InvalidTag {
                kind: "HouseDecisionOutcome",
                tag,
            });
        }
    };
    Ok(HouseDecision {
        id,
        function,
        authority,
        outcome,
        evidence: reader.list(read_evidence)?,
        causal_position: read_position(reader)?,
    })
}

fn write_participant(
    writer: &mut Writer,
    value: &BondParticipant,
) -> Result<(), ConstitutionalArchiveError> {
    write_participant_id(writer, &value.id)?;
    writer.u8(match value.kind {
        ParticipantKind::Huemen => 0,
        ParticipantKind::Npc => 1,
        ParticipantKind::House => 2,
        ParticipantKind::Institution => 3,
        ParticipantKind::Material => 4,
        ParticipantKind::Recipe => 5,
        ParticipantKind::Object => 6,
        ParticipantKind::Transformation => 7,
        ParticipantKind::Process => 8,
    });
    writer.list(&value.roles, write_role_id)
}

fn read_participant(
    reader: &mut Reader<'_>,
) -> Result<BondParticipant, ConstitutionalArchiveError> {
    let id = read_participant_id(reader)?;
    let kind = match reader.u8()? {
        0 => ParticipantKind::Huemen,
        1 => ParticipantKind::Npc,
        2 => ParticipantKind::House,
        3 => ParticipantKind::Institution,
        4 => ParticipantKind::Material,
        5 => ParticipantKind::Recipe,
        6 => ParticipantKind::Object,
        7 => ParticipantKind::Transformation,
        8 => ParticipantKind::Process,
        tag => {
            return Err(ConstitutionalArchiveError::InvalidTag {
                kind: "ParticipantKind",
                tag,
            });
        }
    };
    Ok(BondParticipant {
        id,
        kind,
        roles: reader.list(read_role_id)?,
    })
}

fn write_jurisdiction(
    writer: &mut Writer,
    value: &InstitutionalJurisdictionSnapshot,
) -> Result<(), ConstitutionalArchiveError> {
    write_institution_id(writer, &value.institution)?;
    write_house(writer, value.house);
    write_position(writer, value.observed_at);
    writer.list(&value.evidence, write_evidence)
}

fn read_jurisdiction(
    reader: &mut Reader<'_>,
) -> Result<InstitutionalJurisdictionSnapshot, ConstitutionalArchiveError> {
    Ok(InstitutionalJurisdictionSnapshot {
        institution: read_institution_id(reader)?,
        house: read_house(reader)?,
        observed_at: read_position(reader)?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_initial_current(
    writer: &mut Writer,
    value: &InitialCurrent,
) -> Result<(), ConstitutionalArchiveError> {
    write_participant_id(writer, &value.owner)?;
    write_participant_id(writer, &value.custodian)?;
    write_quantity(writer, &value.quantity)?;
    writer.list(&value.evidence, write_evidence)
}

fn read_initial_current(
    reader: &mut Reader<'_>,
) -> Result<InitialCurrent, ConstitutionalArchiveError> {
    Ok(InitialCurrent {
        owner: read_participant_id(reader)?,
        custodian: read_participant_id(reader)?,
        quantity: read_quantity(reader)?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_formation(
    writer: &mut Writer,
    value: &BondFormation,
) -> Result<(), ConstitutionalArchiveError> {
    write_bond_id(writer, &value.id)?;
    write_wave_id(writer, &value.initiating_wave)?;
    write_house(writer, value.governing_house);
    write_institution_id(writer, &value.governing_institution)?;
    write_jurisdiction(writer, &value.jurisdiction)?;
    writer.list(&value.parent_bonds, write_bond_id)?;
    writer.list(&value.inheritance_evidence, write_evidence)?;
    writer.list(&value.participants, write_participant)?;
    writer.list(&value.obligations, write_obligation_id)?;
    writer.list(&value.permissions, write_permission_id)?;
    write_term(writer, &value.term);
    write_unit_id(writer, &value.current_unit)?;
    write_unit_id(writer, &value.aura_unit)?;
    writer.list(&value.starting_current, write_initial_current)?;
    writer.list(&value.initial_aura, write_quantity)?;
    writer.list(&value.evidence, write_evidence)?;
    write_decision(writer, &value.stonebend_naming)
}

fn read_formation(reader: &mut Reader<'_>) -> Result<BondFormation, ConstitutionalArchiveError> {
    Ok(BondFormation {
        id: read_bond_id(reader)?,
        initiating_wave: read_wave_id(reader)?,
        governing_house: read_house(reader)?,
        governing_institution: read_institution_id(reader)?,
        jurisdiction: read_jurisdiction(reader)?,
        parent_bonds: reader.list(read_bond_id)?,
        inheritance_evidence: reader.list(read_evidence)?,
        participants: reader.list(read_participant)?,
        obligations: reader.list(read_obligation_id)?,
        permissions: reader.list(read_permission_id)?,
        term: read_term(reader)?,
        current_unit: read_unit_id(reader)?,
        aura_unit: read_unit_id(reader)?,
        starting_current: reader.list(read_initial_current)?,
        initial_aura: reader.list(read_quantity)?,
        evidence: reader.list(read_evidence)?,
        stonebend_naming: read_decision(reader)?,
    })
}

fn write_account(
    writer: &mut Writer,
    value: &CurrentAccount,
) -> Result<(), ConstitutionalArchiveError> {
    match value {
        CurrentAccount::Participant(id) => {
            writer.u8(0);
            write_participant_id(writer, id)
        }
        CurrentAccount::External(value) => {
            writer.u8(1);
            writer.string(value)
        }
        CurrentAccount::Sink(value) => {
            writer.u8(2);
            writer.string(value)
        }
    }
}

fn read_account(reader: &mut Reader<'_>) -> Result<CurrentAccount, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(CurrentAccount::Participant(read_participant_id(reader)?)),
        1 => Ok(CurrentAccount::External(reader.string()?)),
        2 => Ok(CurrentAccount::Sink(reader.string()?)),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "CurrentAccount",
            tag,
        }),
    }
}

fn write_operation(writer: &mut Writer, value: CurrentOperation) {
    writer.u8(match value {
        CurrentOperation::Enter => 0,
        CurrentOperation::Leave => 1,
        CurrentOperation::Transfer => 2,
        CurrentOperation::Split => 3,
        CurrentOperation::Merge => 4,
        CurrentOperation::Branch => 5,
        CurrentOperation::Reverse => 6,
        CurrentOperation::Circulate => 7,
        CurrentOperation::Consume => 8,
        CurrentOperation::Decay => 9,
        CurrentOperation::Expire => 10,
    });
}

fn read_operation(reader: &mut Reader<'_>) -> Result<CurrentOperation, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(CurrentOperation::Enter),
        1 => Ok(CurrentOperation::Leave),
        2 => Ok(CurrentOperation::Transfer),
        3 => Ok(CurrentOperation::Split),
        4 => Ok(CurrentOperation::Merge),
        5 => Ok(CurrentOperation::Branch),
        6 => Ok(CurrentOperation::Reverse),
        7 => Ok(CurrentOperation::Circulate),
        8 => Ok(CurrentOperation::Consume),
        9 => Ok(CurrentOperation::Decay),
        10 => Ok(CurrentOperation::Expire),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "CurrentOperation",
            tag,
        }),
    }
}

fn write_edge(writer: &mut Writer, value: &CurrentEdge) -> Result<(), ConstitutionalArchiveError> {
    write_account(writer, &value.source)?;
    write_account(writer, &value.destination)?;
    write_quantity(writer, &value.quantity)
}

fn read_edge(reader: &mut Reader<'_>) -> Result<CurrentEdge, ConstitutionalArchiveError> {
    Ok(CurrentEdge {
        source: read_account(reader)?,
        destination: read_account(reader)?,
        quantity: read_quantity(reader)?,
    })
}

fn write_transaction(
    writer: &mut Writer,
    value: &CurrentTransaction,
) -> Result<(), ConstitutionalArchiveError> {
    write_transaction_id(writer, &value.id)?;
    write_wave_id(writer, &value.wave)?;
    write_operation(writer, value.operation);
    writer.list(&value.edges, write_edge)?;
    writer.list(&value.evidence, write_evidence)
}

fn read_transaction(
    reader: &mut Reader<'_>,
) -> Result<CurrentTransaction, ConstitutionalArchiveError> {
    Ok(CurrentTransaction {
        id: read_transaction_id(reader)?,
        wave: read_wave_id(reader)?,
        operation: read_operation(reader)?,
        edges: reader.list(read_edge)?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_accounting(
    writer: &mut Writer,
    value: &CurrentAccounting,
) -> Result<(), ConstitutionalArchiveError> {
    write_totals(writer, &value.historical)?;
    write_totals(writer, &value.incoming)?;
    write_totals(writer, &value.outgoing)?;
    write_totals(writer, &value.transferred)?;
    write_totals(writer, &value.retained)?;
    write_totals(writer, &value.unresolved)
}

fn read_accounting(
    reader: &mut Reader<'_>,
) -> Result<CurrentAccounting, ConstitutionalArchiveError> {
    Ok(CurrentAccounting {
        historical: read_totals(reader)?,
        incoming: read_totals(reader)?,
        outgoing: read_totals(reader)?,
        transferred: read_totals(reader)?,
        retained: read_totals(reader)?,
        unresolved: read_totals(reader)?,
    })
}

fn write_accumulation(
    writer: &mut Writer,
    value: &CurrentAccumulation,
) -> Result<(), ConstitutionalArchiveError> {
    write_accounting(writer, &value.accounting)?;
    match &value.through_transaction {
        Some(id) => {
            writer.u8(1);
            write_transaction_id(writer, id)
        }
        None => {
            writer.u8(0);
            Ok(())
        }
    }
}

fn read_accumulation(
    reader: &mut Reader<'_>,
) -> Result<CurrentAccumulation, ConstitutionalArchiveError> {
    let accounting = read_accounting(reader)?;
    let through_transaction = match reader.u8()? {
        0 => None,
        1 => Some(read_transaction_id(reader)?),
        tag => {
            return Err(ConstitutionalArchiveError::InvalidTag {
                kind: "Option<CurrentTransactionId>",
                tag,
            });
        }
    };
    Ok(CurrentAccumulation {
        accounting,
        through_transaction,
    })
}

fn write_observation(
    writer: &mut Writer,
    value: &AuraObservation,
) -> Result<(), ConstitutionalArchiveError> {
    write_observation_id(writer, &value.id)?;
    write_participant_id(writer, &value.observer)?;
    write_quantity(writer, &value.quantity)?;
    write_evidence(writer, &value.subject)?;
    writer.list(&value.evidence, write_evidence)
}

fn read_observation(
    reader: &mut Reader<'_>,
) -> Result<AuraObservation, ConstitutionalArchiveError> {
    Ok(AuraObservation {
        id: read_observation_id(reader)?,
        observer: read_participant_id(reader)?,
        quantity: read_quantity(reader)?,
        subject: read_evidence(reader)?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_polarity(writer: &mut Writer, value: ConstitutionalPolarity) {
    writer.u8(match value {
        ConstitutionalPolarity::PositiveCurrentPositiveAura => 0,
        ConstitutionalPolarity::PositiveCurrentNegativeAura => 1,
        ConstitutionalPolarity::NegativeCurrentPositiveAura => 2,
        ConstitutionalPolarity::NegativeCurrentNegativeAura => 3,
    });
}

fn read_polarity(
    reader: &mut Reader<'_>,
) -> Result<ConstitutionalPolarity, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(ConstitutionalPolarity::PositiveCurrentPositiveAura),
        1 => Ok(ConstitutionalPolarity::PositiveCurrentNegativeAura),
        2 => Ok(ConstitutionalPolarity::NegativeCurrentPositiveAura),
        3 => Ok(ConstitutionalPolarity::NegativeCurrentNegativeAura),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "ConstitutionalPolarity",
            tag,
        }),
    }
}

fn write_evaluation(
    writer: &mut Writer,
    value: &CurrentAuraEvaluation,
) -> Result<(), ConstitutionalArchiveError> {
    write_evaluation_id(writer, &value.id)?;
    write_totals(writer, &value.current)?;
    write_totals(writer, &value.aura)?;
    write_polarity(writer, value.polarity);
    writer.list(&value.evidence, write_evidence)
}

fn read_evaluation(
    reader: &mut Reader<'_>,
) -> Result<CurrentAuraEvaluation, ConstitutionalArchiveError> {
    Ok(CurrentAuraEvaluation {
        id: read_evaluation_id(reader)?,
        current: read_totals(reader)?,
        aura: read_totals(reader)?,
        polarity: read_polarity(reader)?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_ineligibility(writer: &mut Writer, value: IneligibilityReason) {
    writer.u8(match value {
        IneligibilityReason::NoNetExcess => 0,
        IneligibilityReason::ClearanceRejected => 1,
        IneligibilityReason::ClearanceInconclusive => 2,
    });
}

fn read_ineligibility(
    reader: &mut Reader<'_>,
) -> Result<IneligibilityReason, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(IneligibilityReason::NoNetExcess),
        1 => Ok(IneligibilityReason::ClearanceRejected),
        2 => Ok(IneligibilityReason::ClearanceInconclusive),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "IneligibilityReason",
            tag,
        }),
    }
}

fn write_tombstone(
    writer: &mut Writer,
    value: &Tombstone,
) -> Result<(), ConstitutionalArchiveError> {
    write_tombstone_id(writer, &value.id)?;
    write_bond_id(writer, &value.source_bond)?;
    write_house(writer, value.governing_house);
    write_institution_id(writer, &value.governing_institution)?;
    writer.list(&value.participants, write_participant)?;
    write_excess(writer, &value.constitutional_excess)?;
    write_polarity(writer, value.polarity);
    writer.list(&value.completed_obligations, write_obligation_id)?;
    writer.list(&value.remaining_obligations, write_obligation_id)?;
    writer.list(&value.evidence, write_evidence)
}

fn read_tombstone(reader: &mut Reader<'_>) -> Result<Tombstone, ConstitutionalArchiveError> {
    Ok(Tombstone {
        id: read_tombstone_id(reader)?,
        source_bond: read_bond_id(reader)?,
        governing_house: read_house(reader)?,
        governing_institution: read_institution_id(reader)?,
        participants: reader.list(read_participant)?,
        constitutional_excess: read_excess(reader)?,
        polarity: read_polarity(reader)?,
        completed_obligations: reader.list(read_obligation_id)?,
        remaining_obligations: reader.list(read_obligation_id)?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_toke(writer: &mut Writer, value: &Toke) -> Result<(), ConstitutionalArchiveError> {
    write_toke_id(writer, &value.id)?;
    write_tombstone_id(writer, &value.tombstone)?;
    writer.string(&value.index_key)?;
    writer.list(&value.evidence, write_evidence)
}

fn read_toke(reader: &mut Reader<'_>) -> Result<Toke, ConstitutionalArchiveError> {
    Ok(Toke {
        id: read_toke_id(reader)?,
        tombstone: read_tombstone_id(reader)?,
        index_key: reader.string()?,
        evidence: reader.list(read_evidence)?,
    })
}

fn write_disposition(writer: &mut Writer, value: ResolutionDisposition) {
    writer.u8(match value {
        ResolutionDisposition::Complete => 0,
        ResolutionDisposition::Renew => 1,
        ResolutionDisposition::Merge => 2,
        ResolutionDisposition::Branch => 3,
        ResolutionDisposition::Split => 4,
        ResolutionDisposition::Transfer => 5,
        ResolutionDisposition::Dissolve => 6,
    });
}

fn read_disposition(
    reader: &mut Reader<'_>,
) -> Result<ResolutionDisposition, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(ResolutionDisposition::Complete),
        1 => Ok(ResolutionDisposition::Renew),
        2 => Ok(ResolutionDisposition::Merge),
        3 => Ok(ResolutionDisposition::Branch),
        4 => Ok(ResolutionDisposition::Split),
        5 => Ok(ResolutionDisposition::Transfer),
        6 => Ok(ResolutionDisposition::Dissolve),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "ResolutionDisposition",
            tag,
        }),
    }
}

fn write_bond_event(
    writer: &mut Writer,
    event: &BondEvent,
) -> Result<(), ConstitutionalArchiveError> {
    match event {
        BondEvent::Formed(value) => {
            writer.u8(0);
            write_formation(writer, value)
        }
        BondEvent::Validated(value) => {
            writer.u8(1);
            write_decision(writer, &value.sandmanor_proof)?;
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::Activated(value) => {
            writer.u8(2);
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::CurrentMoved(value) => {
            writer.u8(3);
            write_transaction(writer, value)
        }
        BondEvent::CurrentAccumulated(value) => {
            writer.u8(4);
            write_accumulation(writer, value)
        }
        BondEvent::AuraObserved(value) => {
            writer.u8(5);
            write_observation(writer, value)
        }
        BondEvent::Evaluated(value) => {
            writer.u8(6);
            write_evaluation(writer, value)
        }
        BondEvent::Matured(value) => {
            writer.u8(7);
            writer.u8(match value.trigger {
                MaturityTrigger::FiniteTermCompleted => 0,
                MaturityTrigger::PerpetualTermTerminated => 1,
            });
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::ExcessCalculated(value) => {
            writer.u8(8);
            write_excess(writer, value)
        }
        BondEvent::CondensationDecided(value) => {
            writer.u8(9);
            match value.status {
                CondensationStatus::Eligible => writer.u8(0),
                CondensationStatus::Ineligible(reason) => {
                    writer.u8(1);
                    write_ineligibility(writer, reason);
                }
            }
            match &value.glaushouse_clearance {
                Some(decision) => {
                    writer.u8(1);
                    write_decision(writer, decision)?;
                }
                None => writer.u8(0),
            }
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::TombstoneFormed(value) => {
            writer.u8(10);
            write_tombstone(writer, value)
        }
        BondEvent::TombstoneOmitted(value) => {
            writer.u8(11);
            write_ineligibility(writer, value.reason);
            Ok(())
        }
        BondEvent::TombstoneValidated(value) => {
            writer.u8(12);
            write_authority_actor_id(writer, &value.validator)?;
            write_evidence(writer, &value.validation_basis)?;
            writer.string(&value.replay_digest)?;
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::TombstoneValidationOmitted(value) => {
            writer.u8(13);
            write_ineligibility(writer, value.reason);
            Ok(())
        }
        BondEvent::FlyntRecognized(value) => {
            writer.u8(14);
            write_decision(writer, value)
        }
        BondEvent::TokeRecorded(value) => {
            writer.u8(15);
            write_toke(writer, value)
        }
        BondEvent::TokeOmitted(value) => {
            writer.u8(16);
            write_ineligibility(writer, value.reason);
            Ok(())
        }
        BondEvent::ChallengeFiled(value) => {
            writer.u8(18);
            write_challenge_id(writer, &value.id)?;
            write_participant_id(writer, &value.challenger)?;
            write_evidence(writer, &value.challenged_evidence)?;
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::ChallengeResolved(value) => {
            writer.u8(19);
            write_challenge_id(writer, &value.challenge)?;
            writer.u8(match value.outcome {
                ChallengeOutcome::Sustained => 0,
                ChallengeOutcome::Rejected => 1,
                ChallengeOutcome::Clarified => 2,
                ChallengeOutcome::Corrected => 3,
            });
            write_decision(writer, &value.sandmanor_proof)?;
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::DefaultDeclared(value) => {
            writer.u8(20);
            write_default_id(writer, &value.id)?;
            write_participant_id(writer, &value.participant)?;
            write_obligation_id(writer, &value.obligation)?;
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::DefaultResolved(value) => {
            writer.u8(21);
            write_default_id(writer, &value.default)?;
            writer.u8(match value.outcome {
                DefaultOutcome::Cured => 0,
                DefaultOutcome::Confirmed => 1,
            });
            writer.list(&value.evidence, write_evidence)
        }
        BondEvent::Resolved(value) => {
            writer.u8(17);
            write_resolution_id(writer, &value.id)?;
            write_disposition(writer, value.disposition);
            writer.list(&value.successor_bonds, write_bond_id)?;
            write_decision(writer, &value.glaushouse_resolution)?;
            writer.list(&value.evidence, write_evidence)
        }
    }
}

fn read_bond_event(reader: &mut Reader<'_>) -> Result<BondEvent, ConstitutionalArchiveError> {
    match reader.u8()? {
        0 => Ok(BondEvent::Formed(read_formation(reader)?)),
        1 => Ok(BondEvent::Validated(BondValidation {
            sandmanor_proof: read_decision(reader)?,
            evidence: reader.list(read_evidence)?,
        })),
        2 => Ok(BondEvent::Activated(BondActivation {
            evidence: reader.list(read_evidence)?,
        })),
        3 => Ok(BondEvent::CurrentMoved(read_transaction(reader)?)),
        4 => Ok(BondEvent::CurrentAccumulated(read_accumulation(reader)?)),
        5 => Ok(BondEvent::AuraObserved(read_observation(reader)?)),
        6 => Ok(BondEvent::Evaluated(read_evaluation(reader)?)),
        7 => {
            let trigger = match reader.u8()? {
                0 => MaturityTrigger::FiniteTermCompleted,
                1 => MaturityTrigger::PerpetualTermTerminated,
                tag => {
                    return Err(ConstitutionalArchiveError::InvalidTag {
                        kind: "MaturityTrigger",
                        tag,
                    });
                }
            };
            Ok(BondEvent::Matured(BondMaturity {
                trigger,
                evidence: reader.list(read_evidence)?,
            }))
        }
        8 => Ok(BondEvent::ExcessCalculated(read_excess(reader)?)),
        9 => {
            let status = match reader.u8()? {
                0 => CondensationStatus::Eligible,
                1 => CondensationStatus::Ineligible(read_ineligibility(reader)?),
                tag => {
                    return Err(ConstitutionalArchiveError::InvalidTag {
                        kind: "CondensationStatus",
                        tag,
                    });
                }
            };
            let glaushouse_clearance = match reader.u8()? {
                0 => None,
                1 => Some(read_decision(reader)?),
                tag => {
                    return Err(ConstitutionalArchiveError::InvalidTag {
                        kind: "Option<HouseDecision>",
                        tag,
                    });
                }
            };
            Ok(BondEvent::CondensationDecided(CondensationDecision {
                status,
                glaushouse_clearance,
                evidence: reader.list(read_evidence)?,
            }))
        }
        10 => Ok(BondEvent::TombstoneFormed(read_tombstone(reader)?)),
        11 => Ok(BondEvent::TombstoneOmitted(TombstoneOmission {
            reason: read_ineligibility(reader)?,
        })),
        12 => Ok(BondEvent::TombstoneValidated(TombstoneValidation {
            validator: read_authority_actor_id(reader)?,
            validation_basis: read_evidence(reader)?,
            replay_digest: reader.string()?,
            evidence: reader.list(read_evidence)?,
        })),
        13 => Ok(BondEvent::TombstoneValidationOmitted(TombstoneOmission {
            reason: read_ineligibility(reader)?,
        })),
        14 => Ok(BondEvent::FlyntRecognized(read_decision(reader)?)),
        15 => Ok(BondEvent::TokeRecorded(read_toke(reader)?)),
        16 => Ok(BondEvent::TokeOmitted(TombstoneOmission {
            reason: read_ineligibility(reader)?,
        })),
        18 => Ok(BondEvent::ChallengeFiled(BondChallenge {
            id: read_challenge_id(reader)?,
            challenger: read_participant_id(reader)?,
            challenged_evidence: read_evidence(reader)?,
            evidence: reader.list(read_evidence)?,
        })),
        19 => {
            let challenge = read_challenge_id(reader)?;
            let outcome = match reader.u8()? {
                0 => ChallengeOutcome::Sustained,
                1 => ChallengeOutcome::Rejected,
                2 => ChallengeOutcome::Clarified,
                3 => ChallengeOutcome::Corrected,
                tag => {
                    return Err(ConstitutionalArchiveError::InvalidTag {
                        kind: "ChallengeOutcome",
                        tag,
                    });
                }
            };
            Ok(BondEvent::ChallengeResolved(BondChallengeResolution {
                challenge,
                outcome,
                sandmanor_proof: read_decision(reader)?,
                evidence: reader.list(read_evidence)?,
            }))
        }
        20 => Ok(BondEvent::DefaultDeclared(BondDefault {
            id: read_default_id(reader)?,
            participant: read_participant_id(reader)?,
            obligation: read_obligation_id(reader)?,
            evidence: reader.list(read_evidence)?,
        })),
        21 => {
            let default = read_default_id(reader)?;
            let outcome = match reader.u8()? {
                0 => DefaultOutcome::Cured,
                1 => DefaultOutcome::Confirmed,
                tag => {
                    return Err(ConstitutionalArchiveError::InvalidTag {
                        kind: "DefaultOutcome",
                        tag,
                    });
                }
            };
            Ok(BondEvent::DefaultResolved(BondDefaultResolution {
                default,
                outcome,
                evidence: reader.list(read_evidence)?,
            }))
        }
        17 => Ok(BondEvent::Resolved(BondResolution {
            id: read_resolution_id(reader)?,
            disposition: read_disposition(reader)?,
            successor_bonds: reader.list(read_bond_id)?,
            glaushouse_resolution: read_decision(reader)?,
            evidence: reader.list(read_evidence)?,
        })),
        tag => Err(ConstitutionalArchiveError::InvalidTag {
            kind: "BondEvent",
            tag,
        }),
    }
}

fn write_event(
    writer: &mut Writer,
    value: &ConstitutionalEvent,
) -> Result<(), ConstitutionalArchiveError> {
    write_event_id(writer, &value.id)?;
    write_bond_id(writer, &value.bond)?;
    writer.u64(value.sequence);
    write_position(writer, value.causal_position);
    write_rule_set_id(writer, &value.rule_set)?;
    write_bond_event(writer, &value.payload)
}

fn read_event(reader: &mut Reader<'_>) -> Result<ConstitutionalEvent, ConstitutionalArchiveError> {
    Ok(ConstitutionalEvent {
        id: read_event_id(reader)?,
        bond: read_bond_id(reader)?,
        sequence: reader.u64()?,
        causal_position: read_position(reader)?,
        rule_set: read_rule_set_id(reader)?,
        payload: read_bond_event(reader)?,
    })
}

pub fn encode_constitutional_archive(
    runtime: &ConstitutionalRuntime,
) -> Result<Vec<u8>, ConstitutionalArchiveError> {
    let mut writer = Writer::new();
    writer.bytes.extend_from_slice(MAGIC);
    writer.u16(CONSTITUTIONAL_ARCHIVE_VERSION);
    let waves: Vec<_> = runtime.waves().collect();
    writer.list(&waves, |writer, wave| write_wave(writer, wave))?;
    writer.list(runtime.events(), write_event)?;
    Ok(writer.bytes)
}

pub fn decode_constitutional_archive(
    bytes: &[u8],
) -> Result<ConstitutionalRuntime, ConstitutionalArchiveError> {
    let mut reader = Reader::new(bytes);
    if reader.take(MAGIC.len())? != MAGIC {
        return Err(ConstitutionalArchiveError::InvalidMagic);
    }
    let version = reader.u16()?;
    if version != CONSTITUTIONAL_ARCHIVE_VERSION {
        return Err(ConstitutionalArchiveError::UnsupportedVersion(version));
    }
    let waves = reader.list(read_wave)?;
    let events = reader.list(read_event)?;
    if !reader.is_finished() {
        return Err(ConstitutionalArchiveError::TrailingBytes);
    }
    ConstitutionalRuntime::replay(waves, events).map_err(Into::into)
}

pub fn write_constitutional_archive(
    path: impl AsRef<Path>,
    runtime: &ConstitutionalRuntime,
) -> Result<(), ConstitutionalArchiveError> {
    fs::write(path, encode_constitutional_archive(runtime)?)?;
    Ok(())
}

pub fn read_constitutional_archive(
    path: impl AsRef<Path>,
) -> Result<ConstitutionalRuntime, ConstitutionalArchiveError> {
    decode_constitutional_archive(&fs::read(path)?)
}

/// Rewrites any supported archive into the current canonical representation.
/// Unsupported historical schemas are rejected rather than guessed.
pub fn migrate_constitutional_archive(bytes: &[u8]) -> Result<Vec<u8>, ConstitutionalArchiveError> {
    encode_constitutional_archive(&decode_constitutional_archive(bytes)?)
}

/// Versioned, deterministic digest used for replay comparison and independent
/// Tombstone validation. FNV-1a is an integrity checksum, not a signature.
pub fn constitutional_replay_digest(
    runtime: &ConstitutionalRuntime,
) -> Result<String, ConstitutionalArchiveError> {
    let bytes = encode_constitutional_archive(runtime)?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-v1:{hash:016x}"))
}

/// Digest of exactly one Bond's replay prefix plus every Wave it directly
/// cites. Tombstone validation compares against this value before appending
/// the validation event, so the digest has no self-reference.
pub fn constitutional_bond_replay_digest(
    runtime: &ConstitutionalRuntime,
    bond: &BondId,
) -> Result<String, ConstitutionalArchiveError> {
    let events: Vec<_> = runtime.events_for(bond).collect();
    if events.is_empty() {
        return Err(ConstitutionalArchiveError::Runtime(
            ConstitutionalRuntimeError::UnknownBond(bond.clone()),
        ));
    }
    let mut wave_ids = std::collections::BTreeSet::new();
    for event in &events {
        match &event.payload {
            BondEvent::Formed(formation) => {
                wave_ids.insert(formation.initiating_wave.clone());
            }
            BondEvent::CurrentMoved(transaction) => {
                wave_ids.insert(transaction.wave.clone());
            }
            _ => {}
        }
    }
    let waves: Vec<_> = wave_ids
        .iter()
        .map(|id| {
            runtime.wave(id).ok_or_else(|| {
                ConstitutionalArchiveError::Runtime(ConstitutionalRuntimeError::UnknownWave(
                    id.clone(),
                ))
            })
        })
        .collect::<Result<_, _>>()?;
    let mut writer = Writer::new();
    writer.bytes.extend_from_slice(b"HGBOND\0\0");
    writer.u16(CONSTITUTIONAL_ARCHIVE_VERSION);
    write_bond_id(&mut writer, bond)?;
    writer.list(&waves, |writer, wave| write_wave(writer, wave))?;
    writer.list(&events, |writer, event| write_event(writer, event))?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in writer.bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-bond-v1:{hash:016x}"))
}
