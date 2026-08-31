//! Compatibility facade for the former Runtime Federation API.
//!
//! New code belongs in [`crate::runtime_rampart`]. HGRF wire identities remain
//! unchanged so existing archives and integrations continue to replay.

pub use crate::runtime_rampart::*;

pub type RuntimeFederation = RuntimeRampart;
pub type RuntimeFederationManifest = RuntimeRampartManifest;
pub type RuntimeFederationArchivePayload = RuntimeRampartArchivePayload;
pub type RuntimeFederationError = RuntimeRampartError;
pub type FederationComponentKind = RampartComponentKind;
pub type FederationComponentDescriptor = RampartComponentDescriptor;
pub type FederationComponentArchive = RampartComponentArchive;
pub type FederatedEventReference = RampartEventReference;
pub type FederationMigrationRecord = RampartMigrationRecord;
pub type FederationPhaseRecord = RampartPhaseRecord;
pub type FederationResult = RampartResult;
pub type FederationRequest = RampartRequest;
pub type FederationOrchestrator<R, S, I, C> = RampartOrchestrator<R, S, I, C>;
pub type FederationError = RampartError;
pub type FederationApiCycleRequest = RampartApiCycleRequest;
pub type FederationApiCycleResponse = RampartApiCycleResponse;
pub type FederationApiDecision = RampartApiDecision;
pub type FederationHttpApi<I> = RampartHttpApi<I>;
pub type FederationHttpApiError = RampartHttpApiError;
pub type LocalFederationCycleInput = LocalRampartCycleInput;
pub type LocalFederationCycleError = LocalRampartCycleError;

pub const RUNTIME_FEDERATION_CANONICAL_NAME: &str = LEGACY_RUNTIME_FEDERATION_CANONICAL_NAME;
pub const RUNTIME_FEDERATION_IDENTITY: &str = LEGACY_RUNTIME_FEDERATION_IDENTITY;
pub const RUNTIME_FEDERATION_ARCHIVE_IDENTITY: &str = LEGACY_RUNTIME_FEDERATION_ARCHIVE_IDENTITY;
pub const RUNTIME_FEDERATION_ARCHIVE_FORMAT: &str = RUNTIME_RAMPART_ARCHIVE_FORMAT;
pub const RUNTIME_FEDERATION_ARCHIVE_VERSION: u16 = RUNTIME_RAMPART_ARCHIVE_VERSION;
pub const RUNTIME_FEDERATION_KERNEL_EVIDENCE_FORMAT: &str = RUNTIME_RAMPART_KERNEL_EVIDENCE_FORMAT;
pub const RUNTIME_FEDERATION_INSTITUTIONAL_FORMAT: &str = RUNTIME_RAMPART_INSTITUTIONAL_FORMAT;
pub const FEDERATION_HTTP_API_SCHEMA_VERSION: &str = RAMPART_HTTP_API_SCHEMA_VERSION;
pub const DEFAULT_FEDERATION_HTTP_ADDRESS: &str = DEFAULT_RAMPART_HTTP_ADDRESS;
pub const MAX_FEDERATION_HTTP_REQUEST_BODY_BYTES: usize = MAX_RAMPART_HTTP_REQUEST_BODY_BYTES;
pub const LOCAL_FEDERATION_ACTOR_REF: &str = LOCAL_RAMPART_ACTOR_REF;
pub const LOCAL_FEDERATION_AUTHORITY_REF: &str = "authority.local-operator.semantic-federation-v1";
pub const LOCAL_FEDERATION_GUIDE_SESSION_ID: &str = "session.semantic-federation-cli";
pub const MAX_LOCAL_FEDERATION_INTENT_BYTES: usize = MAX_LOCAL_RAMPART_INTENT_BYTES;

pub fn encode_runtime_federation_archive(
    payload: &RuntimeFederationArchivePayload,
) -> Result<Vec<u8>, RuntimeFederationError> {
    crate::runtime_rampart::encode_runtime_rampart_archive(payload)
}

pub fn decode_runtime_federation_archive(
    bytes: &[u8],
) -> Result<(RuntimeFederationArchivePayload, RuntimeFederation), RuntimeFederationError> {
    crate::runtime_rampart::decode_runtime_rampart_archive(bytes)
}

pub fn migrate_runtime_federation_archive(bytes: &[u8]) -> Result<Vec<u8>, RuntimeFederationError> {
    crate::runtime_rampart::migrate_runtime_rampart_archive(bytes)
}

pub async fn run_local_federation_cycle<I>(
    interpreter: I,
    input: &LocalFederationCycleInput,
    catalog: &mut crate::composition::CompositionCatalog,
) -> Result<FederationResult, LocalFederationCycleError>
where
    I: SemanticInterpreter,
{
    crate::runtime_rampart::run_local_rampart_cycle(interpreter, input, catalog).await
}

pub fn load_local_federation_catalog(
    path: &std::path::Path,
) -> Result<crate::composition::CompositionCatalog, FederationHttpApiError> {
    crate::runtime_rampart::load_local_rampart_catalog(path)
}
