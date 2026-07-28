//! The Runtime Federation: deterministic continuity across accepted Hollow
//! Grove domain archives.
//!
//! The federation owns no domain decision. It binds independently replayable
//! component archives, validates their relationships, and carries accepted
//! consequence forward without reopening the universal kernel or flattening
//! domain history into one event enum.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    CONSTITUTIONAL_ARCHIVE_VERSION, GrovePhase, REGIONAL_ARCHIVE_VERSION,
    decode_constitutional_archive, decode_regional_archive, encode_constitutional_archive,
    encode_regional_archive,
};
use crate::gameplay::{
    GAMEPLAY_ARCHIVE_FORMAT, GAMEPLAY_ARCHIVE_SCHEMA_VERSION, GameApplicationService,
    decode_gameplay_archive_with_metadata,
};
use crate::world::function_junction_archive::{
    FUNCTION_JUNCTION_ARCHIVE_FORMAT, FUNCTION_JUNCTION_ARCHIVE_VERSION,
    decode_function_junction_archive, encode_function_junction_archive,
};
use crate::world::seasonal_functions_archive::{
    SEASONAL_ARCHIVE_FORMAT, SEASONAL_ARCHIVE_VERSION, decode_seasonal_archive,
    encode_seasonal_archive,
};
use crate::world::service_tournament_archive::{
    SERVICE_TOURNAMENT_ARCHIVE_FORMAT, SERVICE_TOURNAMENT_ARCHIVE_VERSION,
    decode_service_tournament_archive, encode_service_tournament_archive,
};
use crate::world::session::WorldSession;
use crate::world::world_point_archive::{
    WORLD_POINT_ARCHIVE_FORMAT, WORLD_POINT_ARCHIVE_VERSION, decode_world_point_archive,
    encode_world_point_archive,
};

pub const RUNTIME_FEDERATION_CANONICAL_NAME: &str = "The Runtime Federation";
pub const RUNTIME_FEDERATION_IDENTITY: &str = "runtime-federation.hollow-grove.v1";
pub const RUNTIME_FEDERATION_ARCHIVE_IDENTITY: &str = "archive.runtime-federation.hollow-grove.v1";
pub const RUNTIME_FEDERATION_ARCHIVE_FORMAT: &str = "HGRF";
pub const RUNTIME_FEDERATION_ARCHIVE_VERSION: u16 = 1;
pub const RUNTIME_FEDERATION_KERNEL_EVIDENCE_FORMAT: &str = "HGKPE";
pub const RUNTIME_FEDERATION_INSTITUTIONAL_FORMAT: &str = "HGINST";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FederationComponentKind {
    CompletedKernelPass,
    ConstitutionalRuntime,
    RegionalSynthesis,
    InstitutionalAuthority,
    WorldPoint,
    SeasonalFunctionJunction,
    ServiceTournament,
    RouteAndPassage,
    Permanence,
    AuthoritativeGameplay,
}

impl FederationComponentKind {
    pub const REQUIRED: [Self; 10] = [
        Self::CompletedKernelPass,
        Self::ConstitutionalRuntime,
        Self::RegionalSynthesis,
        Self::InstitutionalAuthority,
        Self::WorldPoint,
        Self::SeasonalFunctionJunction,
        Self::ServiceTournament,
        Self::RouteAndPassage,
        Self::Permanence,
        Self::AuthoritativeGameplay,
    ];

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::CompletedKernelPass => "completed universal kernel pass",
            Self::ConstitutionalRuntime => "constitutional runtime",
            Self::RegionalSynthesis => "regional Synthesis",
            Self::InstitutionalAuthority => "institutional authority",
            Self::WorldPoint => "world Point",
            Self::SeasonalFunctionJunction => "seasonal and Function Junction",
            Self::ServiceTournament => "Service Tournament",
            Self::RouteAndPassage => "route and passage",
            Self::Permanence => "Permanence",
            Self::AuthoritativeGameplay => "authoritative gameplay",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederationComponentDescriptor {
    pub component_id: String,
    pub archive_identity: String,
    pub kind: FederationComponentKind,
    pub format: String,
    pub archive_version: u16,
    pub digest: String,
    pub dependencies: Vec<String>,
}

impl FederationComponentDescriptor {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.dependencies.sort();
        value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederationComponentArchive {
    pub component_id: String,
    pub archive_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederatedEventReference {
    pub federation_event_id: String,
    pub component_id: String,
    pub domain_event_id: String,
    pub subject_id: String,
    pub causal_position: u64,
    pub accepted: bool,
    pub result_state_id: Option<String>,
    pub evidence_ids: Vec<String>,
    pub authority_ids: Vec<String>,
    pub provenance_ids: Vec<String>,
    pub caused_by: Vec<String>,
}

impl FederatedEventReference {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.evidence_ids.sort();
        value.evidence_ids.dedup();
        value.authority_ids.sort();
        value.authority_ids.dedup();
        value.provenance_ids.sort();
        value.provenance_ids.dedup();
        value.caused_by.sort();
        value.caused_by.dedup();
        value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederationMigrationRecord {
    pub migration_id: String,
    pub from_version: u16,
    pub to_version: u16,
    pub provenance_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederationPhaseRecord {
    pub phase: GrovePhase,
    pub federation_event_id: String,
    pub state_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CentralJunctionOperationProof {
    pub operation_id: String,
    pub phase_records: Vec<FederationPhaseRecord>,
    pub rejected_attempt_event_id: String,
    pub accepted_result_event_id: String,
    pub real_emergency_event_id: String,
    pub cross_domain_event_id: String,
    pub constitutional_restraint_event_id: String,
    pub permanence_petition_id: String,
    pub permanence_proof_ids: Vec<String>,
    pub nonlethal: bool,
    pub presentation_reads_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFederationManifest {
    pub canonical_name: String,
    pub federation_identity: String,
    pub archive_identity: String,
    pub archive_version: u16,
    pub ruleset_identity: String,
    pub canonical_year_identity: String,
    pub components: Vec<FederationComponentDescriptor>,
    pub events: Vec<FederatedEventReference>,
    pub current_phase: GrovePhase,
    pub physical_checkpoint_id: String,
    pub digital_checkpoint_id: String,
    pub accepted_result_event_id: String,
    pub prior_confirmed_state_id: String,
    pub confirmed_state_id: String,
    pub next_way_back_state_id: String,
    pub migration_history: Vec<FederationMigrationRecord>,
    pub aggregate_digest: String,
    pub transfers_sovereignty: bool,
    pub presentation_authoritative: bool,
    pub first_playable_proof: CentralJunctionOperationProof,
}

impl RuntimeFederationManifest {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.components = value
            .components
            .iter()
            .map(FederationComponentDescriptor::canonicalized)
            .collect();
        value
            .components
            .sort_by(|left, right| left.component_id.cmp(&right.component_id));
        value.events = value
            .events
            .iter()
            .map(FederatedEventReference::canonicalized)
            .collect();
        value.events.sort_by(|left, right| {
            left.causal_position
                .cmp(&right.causal_position)
                .then_with(|| left.federation_event_id.cmp(&right.federation_event_id))
        });
        value
            .migration_history
            .sort_by(|left, right| left.migration_id.cmp(&right.migration_id));
        value.first_playable_proof.permanence_proof_ids.sort();
        value
    }

    pub fn refresh_aggregate_digest(&mut self) -> Result<(), RuntimeFederationError> {
        self.aggregate_digest.clear();
        *self = self.canonicalized();
        self.aggregate_digest = manifest_digest(self)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFederationArchivePayload {
    pub manifest: RuntimeFederationManifest,
    pub component_archives: Vec<FederationComponentArchive>,
}

impl RuntimeFederationArchivePayload {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.manifest = value.manifest.canonicalized();
        value
            .component_archives
            .sort_by(|left, right| left.component_id.cmp(&right.component_id));
        value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuntimeFederationArchiveEnvelope {
    format: String,
    archive_version: u16,
    checksum: String,
    payload: RuntimeFederationArchivePayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedKernelPassEvidence {
    pub completed_pass_id: String,
    pub kernel_identity: String,
    pub canonical_witness: String,
    pub bounded_pass_complete: bool,
    pub federation_aware: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFederation {
    manifest: RuntimeFederationManifest,
    components: BTreeMap<String, FederationComponentArchive>,
    event_index: BTreeMap<String, FederatedEventReference>,
    subject_index: BTreeMap<String, Vec<String>>,
    evidence_index: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CentralJunctionOperationView {
    pub operation_id: String,
    pub current_phase: GrovePhase,
    pub accepted_state_id: String,
    pub rejected_attempt_count: usize,
    pub real_emergency_recorded: bool,
    pub presentation_may_mutate: bool,
}

impl RuntimeFederation {
    pub fn replay(
        payload: &RuntimeFederationArchivePayload,
    ) -> Result<Self, RuntimeFederationError> {
        let payload = payload.canonicalized();
        validate_manifest_identity(&payload.manifest)?;
        validate_components(&payload)?;
        validate_dependencies(&payload.manifest.components)?;
        validate_events(&payload.manifest)?;
        validate_first_playable_proof(&payload.manifest)?;

        let expected_digest = manifest_digest(&payload.manifest)?;
        if payload.manifest.aggregate_digest != expected_digest {
            return Err(RuntimeFederationError::AggregateDigestMismatch {
                expected: expected_digest,
                actual: payload.manifest.aggregate_digest,
            });
        }

        let components = payload
            .component_archives
            .into_iter()
            .map(|archive| (archive.component_id.clone(), archive))
            .collect::<BTreeMap<_, _>>();
        let event_index = payload
            .manifest
            .events
            .iter()
            .cloned()
            .map(|event| (event.federation_event_id.clone(), event))
            .collect::<BTreeMap<_, _>>();
        let mut subject_index: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut evidence_index: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for event in event_index.values() {
            subject_index
                .entry(event.subject_id.clone())
                .or_default()
                .push(event.federation_event_id.clone());
            for evidence in &event.evidence_ids {
                evidence_index
                    .entry(evidence.clone())
                    .or_default()
                    .push(event.federation_event_id.clone());
            }
        }

        Ok(Self {
            manifest: payload.manifest,
            components,
            event_index,
            subject_index,
            evidence_index,
        })
    }

    #[must_use]
    pub const fn manifest(&self) -> &RuntimeFederationManifest {
        &self.manifest
    }

    #[must_use]
    pub fn component(&self, id: &str) -> Option<&FederationComponentArchive> {
        self.components.get(id)
    }

    #[must_use]
    pub fn event(&self, id: &str) -> Option<&FederatedEventReference> {
        self.event_index.get(id)
    }

    #[must_use]
    pub fn events_for_subject(&self, id: &str) -> &[String] {
        self.subject_index.get(id).map_or(&[], Vec::as_slice)
    }

    #[must_use]
    pub fn events_for_evidence(&self, id: &str) -> &[String] {
        self.evidence_index.get(id).map_or(&[], Vec::as_slice)
    }

    #[must_use]
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    #[must_use]
    pub fn first_playable_view(&self) -> CentralJunctionOperationView {
        let proof = &self.manifest.first_playable_proof;
        CentralJunctionOperationView {
            operation_id: proof.operation_id.clone(),
            current_phase: self.manifest.current_phase,
            accepted_state_id: self.manifest.confirmed_state_id.clone(),
            rejected_attempt_count: self
                .event_index
                .values()
                .filter(|event| !event.accepted)
                .count(),
            real_emergency_recorded: self
                .event_index
                .contains_key(&proof.real_emergency_event_id),
            presentation_may_mutate: false,
        }
    }
}

pub fn encode_runtime_federation_archive(
    payload: &RuntimeFederationArchivePayload,
) -> Result<Vec<u8>, RuntimeFederationError> {
    let payload = payload.canonicalized();
    RuntimeFederation::replay(&payload)?;
    let checksum = digest_serialized(&payload)?;
    serde_json::to_vec(&RuntimeFederationArchiveEnvelope {
        format: RUNTIME_FEDERATION_ARCHIVE_FORMAT.into(),
        archive_version: RUNTIME_FEDERATION_ARCHIVE_VERSION,
        checksum,
        payload,
    })
    .map_err(|error| RuntimeFederationError::Json(error.to_string()))
}

pub fn decode_runtime_federation_archive(
    bytes: &[u8],
) -> Result<(RuntimeFederationArchivePayload, RuntimeFederation), RuntimeFederationError> {
    let envelope: RuntimeFederationArchiveEnvelope = serde_json::from_slice(bytes)
        .map_err(|error| RuntimeFederationError::Json(error.to_string()))?;
    if envelope.format != RUNTIME_FEDERATION_ARCHIVE_FORMAT {
        return Err(RuntimeFederationError::UnsupportedFormat(envelope.format));
    }
    if envelope.archive_version != RUNTIME_FEDERATION_ARCHIVE_VERSION {
        return Err(RuntimeFederationError::UnsupportedVersion(
            envelope.archive_version,
        ));
    }
    let actual = digest_serialized(&envelope.payload)?;
    if envelope.checksum != actual {
        return Err(RuntimeFederationError::ChecksumMismatch {
            expected: envelope.checksum,
            actual,
        });
    }
    let payload = envelope.payload.canonicalized();
    let runtime = RuntimeFederation::replay(&payload)?;
    Ok((payload, runtime))
}

pub fn migrate_runtime_federation_archive(bytes: &[u8]) -> Result<Vec<u8>, RuntimeFederationError> {
    let (payload, _) = decode_runtime_federation_archive(bytes)?;
    encode_runtime_federation_archive(&payload)
}

#[must_use]
pub fn component_digest(bytes: &[u8]) -> String {
    digest_bytes(bytes)
}

fn validate_manifest_identity(
    manifest: &RuntimeFederationManifest,
) -> Result<(), RuntimeFederationError> {
    for (label, value) in [
        ("ruleset identity", manifest.ruleset_identity.as_str()),
        (
            "canonical-year identity",
            manifest.canonical_year_identity.as_str(),
        ),
        (
            "physical checkpoint",
            manifest.physical_checkpoint_id.as_str(),
        ),
        (
            "digital checkpoint",
            manifest.digital_checkpoint_id.as_str(),
        ),
        (
            "prior confirmed state",
            manifest.prior_confirmed_state_id.as_str(),
        ),
        ("confirmed state", manifest.confirmed_state_id.as_str()),
        (
            "next Way Back state",
            manifest.next_way_back_state_id.as_str(),
        ),
    ] {
        require_stable_id(value, label)?;
    }
    if manifest.canonical_name != RUNTIME_FEDERATION_CANONICAL_NAME
        || manifest.federation_identity != RUNTIME_FEDERATION_IDENTITY
        || manifest.archive_identity != RUNTIME_FEDERATION_ARCHIVE_IDENTITY
        || manifest.archive_version != RUNTIME_FEDERATION_ARCHIVE_VERSION
    {
        return Err(RuntimeFederationError::CanonicalIdentityMismatch);
    }
    if manifest.current_phase != GrovePhase::TheFestival
        || manifest.confirmed_state_id != manifest.next_way_back_state_id
        || manifest.confirmed_state_id == manifest.prior_confirmed_state_id
        || manifest.physical_checkpoint_id == manifest.digital_checkpoint_id
        || manifest.transfers_sovereignty
        || manifest.presentation_authoritative
    {
        return Err(RuntimeFederationError::InvalidContinuity);
    }
    let mut migration_ids = BTreeSet::new();
    for migration in &manifest.migration_history {
        require_stable_id(&migration.migration_id, "migration identity")?;
        require_stable_id(&migration.provenance_id, "migration provenance identity")?;
        if migration.from_version >= migration.to_version
            || !migration_ids.insert(migration.migration_id.as_str())
        {
            return Err(RuntimeFederationError::InvalidMigration(
                migration.migration_id.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_components(
    payload: &RuntimeFederationArchivePayload,
) -> Result<(), RuntimeFederationError> {
    let descriptors = payload
        .manifest
        .components
        .iter()
        .map(|component| (component.component_id.clone(), component))
        .collect::<BTreeMap<_, _>>();
    if descriptors.len() != payload.manifest.components.len() {
        return Err(RuntimeFederationError::DuplicateComponent);
    }
    let archives = payload
        .component_archives
        .iter()
        .map(|archive| (archive.component_id.clone(), archive))
        .collect::<BTreeMap<_, _>>();
    if archives.len() != payload.component_archives.len() {
        return Err(RuntimeFederationError::DuplicateComponentArchive);
    }
    if descriptors.keys().ne(archives.keys()) {
        return Err(RuntimeFederationError::ComponentArchiveSetMismatch);
    }
    let present_kinds = descriptors
        .values()
        .map(|component| component.kind)
        .collect::<BTreeSet<_>>();
    for required in FederationComponentKind::REQUIRED {
        if !present_kinds.contains(&required) {
            return Err(RuntimeFederationError::MissingRequiredComponent(required));
        }
    }
    for descriptor in descriptors.values() {
        require_stable_id(&descriptor.component_id, "component identity")?;
        require_stable_id(&descriptor.archive_identity, "component archive identity")?;
        if descriptor.archive_version == 0
            && descriptor.kind != FederationComponentKind::CompletedKernelPass
        {
            return Err(RuntimeFederationError::InvalidComponentVersion(
                descriptor.component_id.clone(),
            ));
        }
        let archive = archives
            .get(&descriptor.component_id)
            .expect("descriptor and archive key sets agree");
        let actual = component_digest(&archive.archive_bytes);
        if actual != descriptor.digest {
            return Err(RuntimeFederationError::ComponentDigestMismatch {
                component_id: descriptor.component_id.clone(),
                expected: descriptor.digest.clone(),
                actual,
            });
        }
        replay_component(descriptor, &archive.archive_bytes)?;
    }
    Ok(())
}

fn replay_component(
    descriptor: &FederationComponentDescriptor,
    bytes: &[u8],
) -> Result<(), RuntimeFederationError> {
    let mismatch = || RuntimeFederationError::ComponentCodecMismatch {
        component_id: descriptor.component_id.clone(),
        kind: descriptor.kind,
        format: descriptor.format.clone(),
        version: descriptor.archive_version,
    };
    let replay_error = |error: String| RuntimeFederationError::ComponentReplay {
        component_id: descriptor.component_id.clone(),
        error,
    };

    match descriptor.kind {
        FederationComponentKind::CompletedKernelPass
            if descriptor.format == RUNTIME_FEDERATION_KERNEL_EVIDENCE_FORMAT
                && descriptor.archive_version == 1 =>
        {
            let evidence: CompletedKernelPassEvidence =
                serde_json::from_slice(bytes).map_err(|error| replay_error(error.to_string()))?;
            if !evidence.bounded_pass_complete
                || evidence.federation_aware
                || evidence.canonical_witness.is_empty()
            {
                return Err(replay_error(
                    "kernel evidence does not prove a completed federation-unaware pass".into(),
                ));
            }
            Ok(())
        }
        FederationComponentKind::ConstitutionalRuntime
            if descriptor.format == "HGCONST"
                && descriptor.archive_version == CONSTITUTIONAL_ARCHIVE_VERSION =>
        {
            let runtime = decode_constitutional_archive(bytes)
                .map_err(|error| replay_error(error.to_string()))?;
            let replayed = encode_constitutional_archive(&runtime)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::RegionalSynthesis
            if descriptor.format == "HGREG"
                && descriptor.archive_version == REGIONAL_ARCHIVE_VERSION =>
        {
            let runtime =
                decode_regional_archive(bytes).map_err(|error| replay_error(error.to_string()))?;
            let replayed = encode_regional_archive(&runtime)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::InstitutionalAuthority
            if descriptor.format == RUNTIME_FEDERATION_INSTITUTIONAL_FORMAT
                && descriptor.archive_version == 2 =>
        {
            let text =
                std::str::from_utf8(bytes).map_err(|error| replay_error(error.to_string()))?;
            let session = WorldSession::from_persisted_output(text)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(
                bytes,
                session.persisted_state_output().as_bytes(),
                replay_error,
            )
        }
        FederationComponentKind::WorldPoint
            if descriptor.format == WORLD_POINT_ARCHIVE_FORMAT
                && descriptor.archive_version == WORLD_POINT_ARCHIVE_VERSION =>
        {
            let decoded = decode_world_point_archive(bytes)
                .map_err(|error| replay_error(error.to_string()))?;
            let replayed = encode_world_point_archive(&decoded.payload)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::SeasonalFunctionJunction
            if descriptor.format == FUNCTION_JUNCTION_ARCHIVE_FORMAT
                && descriptor.archive_version == FUNCTION_JUNCTION_ARCHIVE_VERSION =>
        {
            let decoded = decode_function_junction_archive(bytes)
                .map_err(|error| replay_error(error.to_string()))?;
            let replayed = encode_function_junction_archive(&decoded.payload)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::ServiceTournament
            if descriptor.format == SERVICE_TOURNAMENT_ARCHIVE_FORMAT
                && descriptor.archive_version == SERVICE_TOURNAMENT_ARCHIVE_VERSION =>
        {
            let decoded = decode_service_tournament_archive(bytes)
                .map_err(|error| replay_error(error.to_string()))?;
            let replayed = encode_service_tournament_archive(&decoded.payload)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::RouteAndPassage
            if descriptor.format == SEASONAL_ARCHIVE_FORMAT
                && descriptor.archive_version == SEASONAL_ARCHIVE_VERSION =>
        {
            let decoded =
                decode_seasonal_archive(bytes).map_err(|error| replay_error(error.to_string()))?;
            let replayed = encode_seasonal_archive(&decoded.payload)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::Permanence
            if descriptor.format == FUNCTION_JUNCTION_ARCHIVE_FORMAT
                && descriptor.archive_version == FUNCTION_JUNCTION_ARCHIVE_VERSION =>
        {
            let decoded = decode_function_junction_archive(bytes)
                .map_err(|error| replay_error(error.to_string()))?;
            if decoded
                .annual_states
                .values()
                .any(|state| state.permanence_runtime.petitions().is_empty())
            {
                return Err(replay_error("Permanence component has no petition".into()));
            }
            let replayed = encode_function_junction_archive(&decoded.payload)
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, &replayed, replay_error)
        }
        FederationComponentKind::AuthoritativeGameplay
            if descriptor.format == GAMEPLAY_ARCHIVE_FORMAT
                && descriptor.archive_version == GAMEPLAY_ARCHIVE_SCHEMA_VERSION =>
        {
            let text =
                std::str::from_utf8(bytes).map_err(|error| replay_error(error.to_string()))?;
            let decoded = decode_gameplay_archive_with_metadata(text)
                .map_err(|error| replay_error(error.to_string()))?;
            if decoded.federation_binding.federation_identity != RUNTIME_FEDERATION_IDENTITY
                || decoded.federation_binding.component_id != descriptor.component_id
            {
                return Err(replay_error(
                    "gameplay archive is not linked to this federation component".into(),
                ));
            }
            let replayed = GameApplicationService::from_archive(text)
                .map_err(|error| replay_error(error.to_string()))?;
            let replayed = replayed
                .encode_archive()
                .map_err(|error| replay_error(error.to_string()))?;
            exact_component_bytes(bytes, replayed.as_bytes(), replay_error)
        }
        _ => Err(mismatch()),
    }
}

fn exact_component_bytes(
    original: &[u8],
    replayed: &[u8],
    error: impl FnOnce(String) -> RuntimeFederationError,
) -> Result<(), RuntimeFederationError> {
    if original == replayed {
        Ok(())
    } else {
        Err(error(
            "production replay changed canonical archive bytes".into(),
        ))
    }
}

fn validate_dependencies(
    components: &[FederationComponentDescriptor],
) -> Result<(), RuntimeFederationError> {
    let graph = components
        .iter()
        .map(|component| {
            (
                component.component_id.as_str(),
                component
                    .dependencies
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for (component, dependencies) in &graph {
        if dependencies
            .windows(2)
            .any(|pair| pair.first() == pair.get(1))
        {
            return Err(RuntimeFederationError::DuplicateDependency(
                (*component).into(),
            ));
        }
        for dependency in dependencies {
            if component == dependency {
                return Err(RuntimeFederationError::SelfDependency((*component).into()));
            }
            if !graph.contains_key(dependency) {
                return Err(RuntimeFederationError::MissingDependency {
                    component_id: (*component).into(),
                    dependency_id: (*dependency).into(),
                });
            }
        }
    }

    fn visit<'a>(
        node: &'a str,
        graph: &BTreeMap<&'a str, Vec<&'a str>>,
        visiting: &mut BTreeSet<&'a str>,
        visited: &mut BTreeSet<&'a str>,
    ) -> Result<(), RuntimeFederationError> {
        if visited.contains(node) {
            return Ok(());
        }
        if !visiting.insert(node) {
            return Err(RuntimeFederationError::CyclicalDependency(node.into()));
        }
        for dependency in &graph[node] {
            visit(dependency, graph, visiting, visited)?;
        }
        visiting.remove(node);
        visited.insert(node);
        Ok(())
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for component in graph.keys() {
        visit(component, &graph, &mut visiting, &mut visited)?;
    }
    Ok(())
}

fn validate_events(manifest: &RuntimeFederationManifest) -> Result<(), RuntimeFederationError> {
    let component_ids = manifest
        .components
        .iter()
        .map(|component| component.component_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut event_ids = BTreeSet::new();
    let mut event_positions = BTreeMap::new();
    for event in &manifest.events {
        for (label, value) in [
            (
                "federation event identity",
                event.federation_event_id.as_str(),
            ),
            ("domain event identity", event.domain_event_id.as_str()),
            ("event subject identity", event.subject_id.as_str()),
        ] {
            require_stable_id(value, label)?;
        }
        if !component_ids.contains(event.component_id.as_str()) {
            return Err(RuntimeFederationError::UnknownEventComponent(
                event.component_id.clone(),
            ));
        }
        if !event_ids.insert(event.federation_event_id.as_str()) {
            return Err(RuntimeFederationError::DuplicateEvent(
                event.federation_event_id.clone(),
            ));
        }
        if event.evidence_ids.is_empty()
            || event.authority_ids.is_empty()
            || event.provenance_ids.is_empty()
            || (!event.accepted && event.result_state_id.is_some())
        {
            return Err(RuntimeFederationError::InvalidEvent(
                event.federation_event_id.clone(),
            ));
        }
        event_positions.insert(event.federation_event_id.as_str(), event.causal_position);
    }
    for event in &manifest.events {
        for cause in &event.caused_by {
            let Some(position) = event_positions.get(cause.as_str()) else {
                return Err(RuntimeFederationError::MissingCausalEvent(cause.clone()));
            };
            if *position >= event.causal_position {
                return Err(RuntimeFederationError::InvalidCausalOrder {
                    event_id: event.federation_event_id.clone(),
                    cause_id: cause.clone(),
                });
            }
        }
    }
    let accepted = manifest
        .events
        .iter()
        .find(|event| event.federation_event_id == manifest.accepted_result_event_id)
        .ok_or_else(|| {
            RuntimeFederationError::MissingAcceptedResult(manifest.accepted_result_event_id.clone())
        })?;
    if !accepted.accepted
        || accepted.result_state_id.as_deref() != Some(manifest.confirmed_state_id.as_str())
        || !manifest.events.iter().any(|event| !event.accepted)
    {
        return Err(RuntimeFederationError::InvalidAcceptedResult);
    }
    Ok(())
}

fn validate_first_playable_proof(
    manifest: &RuntimeFederationManifest,
) -> Result<(), RuntimeFederationError> {
    let proof = &manifest.first_playable_proof;
    require_stable_id(&proof.operation_id, "operation identity")?;
    if proof.phase_records.len() != GrovePhase::ALL.len()
        || proof
            .phase_records
            .iter()
            .map(|record| record.phase)
            .ne(GrovePhase::ALL)
        || proof.accepted_result_event_id != manifest.accepted_result_event_id
        || proof.permanence_proof_ids.len() != 4
        || !proof.nonlethal
        || !proof.presentation_reads_only
    {
        return Err(RuntimeFederationError::InvalidPlayableProof);
    }
    let events = manifest
        .events
        .iter()
        .map(|event| (event.federation_event_id.as_str(), event))
        .collect::<BTreeMap<_, _>>();
    for event_id in [
        proof.rejected_attempt_event_id.as_str(),
        proof.accepted_result_event_id.as_str(),
        proof.real_emergency_event_id.as_str(),
        proof.cross_domain_event_id.as_str(),
        proof.constitutional_restraint_event_id.as_str(),
    ] {
        if !events.contains_key(event_id) {
            return Err(RuntimeFederationError::InvalidPlayableProof);
        }
    }
    if events[proof.rejected_attempt_event_id.as_str()].accepted
        || !events[proof.accepted_result_event_id.as_str()].accepted
        || events[proof.cross_domain_event_id.as_str()]
            .caused_by
            .is_empty()
        || proof
            .phase_records
            .iter()
            .any(|record| !events.contains_key(record.federation_event_id.as_str()))
    {
        return Err(RuntimeFederationError::InvalidPlayableProof);
    }
    Ok(())
}

fn require_stable_id(value: &str, label: &'static str) -> Result<(), RuntimeFederationError> {
    if value.is_empty()
        || value.chars().any(char::is_whitespace)
        || value
            .chars()
            .any(|character| character.is_ascii_uppercase())
    {
        return Err(RuntimeFederationError::InvalidStableIdentity {
            label,
            value: value.into(),
        });
    }
    Ok(())
}

fn manifest_digest(manifest: &RuntimeFederationManifest) -> Result<String, RuntimeFederationError> {
    let mut canonical = manifest.canonicalized();
    canonical.aggregate_digest.clear();
    digest_serialized(&canonical)
}

fn digest_serialized(value: &impl Serialize) -> Result<String, RuntimeFederationError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| RuntimeFederationError::Json(error.to_string()))?;
    Ok(digest_bytes(&bytes))
}

fn digest_bytes(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeFederationError {
    Json(String),
    UnsupportedFormat(String),
    UnsupportedVersion(u16),
    ChecksumMismatch {
        expected: String,
        actual: String,
    },
    AggregateDigestMismatch {
        expected: String,
        actual: String,
    },
    CanonicalIdentityMismatch,
    InvalidStableIdentity {
        label: &'static str,
        value: String,
    },
    InvalidContinuity,
    InvalidMigration(String),
    DuplicateComponent,
    DuplicateComponentArchive,
    ComponentArchiveSetMismatch,
    MissingRequiredComponent(FederationComponentKind),
    InvalidComponentVersion(String),
    ComponentDigestMismatch {
        component_id: String,
        expected: String,
        actual: String,
    },
    ComponentCodecMismatch {
        component_id: String,
        kind: FederationComponentKind,
        format: String,
        version: u16,
    },
    ComponentReplay {
        component_id: String,
        error: String,
    },
    SelfDependency(String),
    DuplicateDependency(String),
    MissingDependency {
        component_id: String,
        dependency_id: String,
    },
    CyclicalDependency(String),
    UnknownEventComponent(String),
    DuplicateEvent(String),
    InvalidEvent(String),
    MissingCausalEvent(String),
    InvalidCausalOrder {
        event_id: String,
        cause_id: String,
    },
    MissingAcceptedResult(String),
    InvalidAcceptedResult,
    InvalidPlayableProof,
}

impl fmt::Display for RuntimeFederationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "The Runtime Federation rejected archive: {self:?}"
        )
    }
}

impl std::error::Error for RuntimeFederationError {}
