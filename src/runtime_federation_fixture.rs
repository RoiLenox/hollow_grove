//! Canonical executable fixture for The Runtime Federation V1.

use crate::CANONICAL_WITNESS;
use crate::constitutional::{
    CONSTITUTIONAL_ARCHIVE_VERSION, ConstitutionalRuntime, GrovePhase, REGIONAL_ARCHIVE_VERSION,
    RegionalSynthesisRuntime, RuleSetId, V2_RULE_SET, encode_constitutional_archive,
    encode_regional_archive,
};
use crate::gameplay::{
    GAMEPLAY_ARCHIVE_FORMAT, GAMEPLAY_ARCHIVE_SCHEMA_VERSION, GameApplicationService,
};
use crate::runtime_federation::{
    CentralJunctionOperationProof, CompletedKernelPassEvidence, FederatedEventReference,
    FederationComponentArchive, FederationComponentDescriptor, FederationComponentKind,
    FederationPhaseRecord, RUNTIME_FEDERATION_ARCHIVE_IDENTITY, RUNTIME_FEDERATION_ARCHIVE_VERSION,
    RUNTIME_FEDERATION_CANONICAL_NAME, RUNTIME_FEDERATION_IDENTITY,
    RUNTIME_FEDERATION_INSTITUTIONAL_FORMAT, RUNTIME_FEDERATION_KERNEL_EVIDENCE_FORMAT,
    RuntimeFederationArchivePayload, RuntimeFederationError, RuntimeFederationManifest,
    component_digest,
};
use crate::world::function_junction_archive::{
    FUNCTION_JUNCTION_ARCHIVE_FORMAT, FUNCTION_JUNCTION_ARCHIVE_VERSION,
    encode_function_junction_archive,
};
use crate::world::function_junction_fixture::canonical_function_junction_archive_fixture;
use crate::world::seasonal_functions_archive::{
    CANONICAL_ANNUAL_CYCLE_ID, SEASONAL_ARCHIVE_FORMAT, SEASONAL_ARCHIVE_VERSION,
    encode_seasonal_archive,
};
use crate::world::seasonal_functions_fixture::canonical_seasonal_archive_fixture;
use crate::world::service_tournament_archive::{
    SERVICE_TOURNAMENT_ARCHIVE_FORMAT, SERVICE_TOURNAMENT_ARCHIVE_VERSION,
    encode_service_tournament_archive,
};
use crate::world::service_tournament_fixture::canonical_service_tournament_archive_fixture;
use crate::world::session::WorldSession;
use crate::world::world_point_archive::{
    WORLD_POINT_ARCHIVE_FORMAT, WORLD_POINT_ARCHIVE_VERSION, encode_world_point_archive,
};
use crate::world::world_point_fixture::canonical_world_point_archive_fixture;

pub const FEDERATION_KERNEL_COMPONENT_ID: &str = "component.kernel.completed-pass";
pub const FEDERATION_CONSTITUTIONAL_COMPONENT_ID: &str = "component.constitutional.runtime";
pub const FEDERATION_REGIONAL_COMPONENT_ID: &str = "component.regional.synthesis";
pub const FEDERATION_INSTITUTIONAL_COMPONENT_ID: &str = "component.institutional.authority";
pub const FEDERATION_WORLD_POINT_COMPONENT_ID: &str = "component.world.point";
pub const FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID: &str =
    "component.function-junction.canonical-year";
pub const FEDERATION_TOURNAMENT_COMPONENT_ID: &str = "component.service-tournament.canonical-year";
pub const FEDERATION_ROUTE_COMPONENT_ID: &str = "component.route-passage.canonical-year";
pub const FEDERATION_PERMANENCE_COMPONENT_ID: &str = "component.permanence.canonical-year";
pub const FEDERATION_GAMEPLAY_COMPONENT_ID: &str = "component.gameplay.authoritative";

pub fn canonical_runtime_federation_payload()
-> Result<RuntimeFederationArchivePayload, RuntimeFederationError> {
    let kernel_bytes = serde_json::to_vec(&CompletedKernelPassEvidence {
        completed_pass_id: "kernel-pass.hollow-grove.canonical".into(),
        kernel_identity: "kernel.hollow-grove.universal-recursion.v0.1.2".into(),
        canonical_witness: CANONICAL_WITNESS.into(),
        bounded_pass_complete: true,
        federation_aware: false,
    })
    .map_err(|error| RuntimeFederationError::Json(error.to_string()))?;
    let constitutional_bytes = encode_constitutional_archive(&ConstitutionalRuntime::new())
        .map_err(|error| RuntimeFederationError::ComponentReplay {
            component_id: FEDERATION_CONSTITUTIONAL_COMPONENT_ID.into(),
            error: error.to_string(),
        })?;
    let regional_bytes =
        encode_regional_archive(&RegionalSynthesisRuntime::new()).map_err(|error| {
            RuntimeFederationError::ComponentReplay {
                component_id: FEDERATION_REGIONAL_COMPONENT_ID.into(),
                error: error.to_string(),
            }
        })?;
    let institutional_bytes = WorldSession::canonical()
        .persisted_state_output()
        .into_bytes();
    let world_point_bytes = encode_world_point_archive(&canonical_world_point_archive_fixture())
        .map_err(|error| RuntimeFederationError::ComponentReplay {
            component_id: FEDERATION_WORLD_POINT_COMPONENT_ID.into(),
            error: error.to_string(),
        })?;
    let tournament_bytes =
        encode_service_tournament_archive(&canonical_service_tournament_archive_fixture())
            .map_err(|error| RuntimeFederationError::ComponentReplay {
                component_id: FEDERATION_TOURNAMENT_COMPONENT_ID.into(),
                error: error.to_string(),
            })?;
    let seasonal_bytes =
        encode_seasonal_archive(&canonical_seasonal_archive_fixture()).map_err(|error| {
            RuntimeFederationError::ComponentReplay {
                component_id: FEDERATION_ROUTE_COMPONENT_ID.into(),
                error: error.to_string(),
            }
        })?;
    let function_junction_bytes = encode_function_junction_archive(
        &canonical_function_junction_archive_fixture(),
    )
    .map_err(|error| RuntimeFederationError::ComponentReplay {
        component_id: FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID.into(),
        error: error.to_string(),
    })?;
    let gameplay = GameApplicationService::new(
        RuleSetId::new(V2_RULE_SET).expect("canonical V2 ruleset identity"),
    );
    let gameplay_bytes = gameplay
        .encode_archive()
        .map_err(|error| RuntimeFederationError::ComponentReplay {
            component_id: FEDERATION_GAMEPLAY_COMPONENT_ID.into(),
            error: error.to_string(),
        })?
        .into_bytes();

    let mut pairs = vec![
        component(
            FEDERATION_KERNEL_COMPONENT_ID,
            "archive.kernel-pass.hollow-grove.canonical",
            FederationComponentKind::CompletedKernelPass,
            RUNTIME_FEDERATION_KERNEL_EVIDENCE_FORMAT,
            1,
            &[],
            kernel_bytes,
        ),
        component(
            FEDERATION_CONSTITUTIONAL_COMPONENT_ID,
            "archive.constitutional.runtime.empty-v1",
            FederationComponentKind::ConstitutionalRuntime,
            "HGCONST",
            CONSTITUTIONAL_ARCHIVE_VERSION,
            &[FEDERATION_KERNEL_COMPONENT_ID],
            constitutional_bytes,
        ),
        component(
            FEDERATION_REGIONAL_COMPONENT_ID,
            "archive.regional.synthesis.empty-v1",
            FederationComponentKind::RegionalSynthesis,
            "HGREG",
            REGIONAL_ARCHIVE_VERSION,
            &[FEDERATION_CONSTITUTIONAL_COMPONENT_ID],
            regional_bytes,
        ),
        component(
            FEDERATION_INSTITUTIONAL_COMPONENT_ID,
            "archive.institutional.authority.canonical",
            FederationComponentKind::InstitutionalAuthority,
            RUNTIME_FEDERATION_INSTITUTIONAL_FORMAT,
            2,
            &[FEDERATION_CONSTITUTIONAL_COMPONENT_ID],
            institutional_bytes,
        ),
        component(
            FEDERATION_WORLD_POINT_COMPONENT_ID,
            "archive.world-point.hollow-grove.canonical",
            FederationComponentKind::WorldPoint,
            WORLD_POINT_ARCHIVE_FORMAT,
            WORLD_POINT_ARCHIVE_VERSION,
            &[FEDERATION_KERNEL_COMPONENT_ID],
            world_point_bytes,
        ),
        component(
            FEDERATION_TOURNAMENT_COMPONENT_ID,
            "archive.service-tournament.canonical-year",
            FederationComponentKind::ServiceTournament,
            SERVICE_TOURNAMENT_ARCHIVE_FORMAT,
            SERVICE_TOURNAMENT_ARCHIVE_VERSION,
            &[
                FEDERATION_CONSTITUTIONAL_COMPONENT_ID,
                FEDERATION_INSTITUTIONAL_COMPONENT_ID,
                FEDERATION_WORLD_POINT_COMPONENT_ID,
            ],
            tournament_bytes,
        ),
        component(
            FEDERATION_ROUTE_COMPONENT_ID,
            "archive.route-passage.canonical-year",
            FederationComponentKind::RouteAndPassage,
            SEASONAL_ARCHIVE_FORMAT,
            SEASONAL_ARCHIVE_VERSION,
            &[
                FEDERATION_CONSTITUTIONAL_COMPONENT_ID,
                FEDERATION_WORLD_POINT_COMPONENT_ID,
            ],
            seasonal_bytes,
        ),
        component(
            FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID,
            "archive.function-junction.canonical-year",
            FederationComponentKind::SeasonalFunctionJunction,
            FUNCTION_JUNCTION_ARCHIVE_FORMAT,
            FUNCTION_JUNCTION_ARCHIVE_VERSION,
            &[
                FEDERATION_CONSTITUTIONAL_COMPONENT_ID,
                FEDERATION_INSTITUTIONAL_COMPONENT_ID,
                FEDERATION_ROUTE_COMPONENT_ID,
                FEDERATION_TOURNAMENT_COMPONENT_ID,
                FEDERATION_WORLD_POINT_COMPONENT_ID,
            ],
            function_junction_bytes.clone(),
        ),
        component(
            FEDERATION_PERMANENCE_COMPONENT_ID,
            "archive.permanence.canonical-year",
            FederationComponentKind::Permanence,
            FUNCTION_JUNCTION_ARCHIVE_FORMAT,
            FUNCTION_JUNCTION_ARCHIVE_VERSION,
            &[FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID],
            function_junction_bytes,
        ),
        component(
            FEDERATION_GAMEPLAY_COMPONENT_ID,
            "archive.gameplay.authoritative.v3",
            FederationComponentKind::AuthoritativeGameplay,
            GAMEPLAY_ARCHIVE_FORMAT,
            GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
            &[
                FEDERATION_CONSTITUTIONAL_COMPONENT_ID,
                FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID,
                FEDERATION_INSTITUTIONAL_COMPONENT_ID,
                FEDERATION_PERMANENCE_COMPONENT_ID,
                FEDERATION_REGIONAL_COMPONENT_ID,
                FEDERATION_ROUTE_COMPONENT_ID,
                FEDERATION_TOURNAMENT_COMPONENT_ID,
                FEDERATION_WORLD_POINT_COMPONENT_ID,
            ],
            gameplay_bytes,
        ),
    ];
    pairs.sort_by(|left, right| left.0.component_id.cmp(&right.0.component_id));
    let (components, component_archives): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    let events = canonical_events();
    let proof = CentralJunctionOperationProof {
        operation_id: "operation.central-junction.four-house-bridge".into(),
        phase_records: vec![
            phase(
                GrovePhase::TheWayBack,
                "federation-event.operation.way-back",
                "state.bridge-control.v1",
            ),
            phase(
                GrovePhase::TheInitiation,
                "federation-event.operation.initiation",
                "state.bridge-control.candidate",
            ),
            phase(
                GrovePhase::TheGathering,
                "federation-event.operation.gathering",
                "state.bridge-control.related",
            ),
            phase(
                GrovePhase::TheFestival,
                "federation-event.operation.accepted",
                "state.bridge-control.v2",
            ),
        ],
        rejected_attempt_event_id: "federation-event.operation.rejected".into(),
        accepted_result_event_id: "federation-event.operation.accepted".into(),
        real_emergency_event_id: "federation-event.operation.real-emergency".into(),
        cross_domain_event_id: "federation-event.operation.accepted".into(),
        constitutional_restraint_event_id: "federation-event.operation.restraint".into(),
        permanence_petition_id: "petition.permanence.bridge".into(),
        permanence_proof_ids: vec![
            "attestation.permanence.bridge.identity".into(),
            "attestation.permanence.bridge.integrity".into(),
            "attestation.permanence.bridge.pattern".into(),
            "attestation.permanence.bridge.recognition".into(),
        ],
        nonlethal: true,
        presentation_reads_only: true,
    };
    let mut manifest = RuntimeFederationManifest {
        canonical_name: RUNTIME_FEDERATION_CANONICAL_NAME.into(),
        federation_identity: RUNTIME_FEDERATION_IDENTITY.into(),
        archive_identity: RUNTIME_FEDERATION_ARCHIVE_IDENTITY.into(),
        archive_version: RUNTIME_FEDERATION_ARCHIVE_VERSION,
        ruleset_identity: V2_RULE_SET.into(),
        canonical_year_identity: CANONICAL_ANNUAL_CYCLE_ID.into(),
        components,
        events,
        current_phase: GrovePhase::TheFestival,
        physical_checkpoint_id: "checkpoint.canonical-year.autumn-equinox.physical".into(),
        digital_checkpoint_id: "checkpoint.canonical-year.autumn-equinox.digital".into(),
        accepted_result_event_id: "federation-event.operation.accepted".into(),
        prior_confirmed_state_id: "state.bridge-control.v1".into(),
        confirmed_state_id: "state.bridge-control.v2".into(),
        next_way_back_state_id: "state.bridge-control.v2".into(),
        migration_history: Vec::new(),
        aggregate_digest: String::new(),
        transfers_sovereignty: false,
        presentation_authoritative: false,
        first_playable_proof: proof,
    };
    manifest.refresh_aggregate_digest()?;
    Ok(RuntimeFederationArchivePayload {
        manifest,
        component_archives,
    })
}

fn component(
    component_id: &str,
    archive_identity: &str,
    kind: FederationComponentKind,
    format: &str,
    archive_version: u16,
    dependencies: &[&str],
    archive_bytes: Vec<u8>,
) -> (FederationComponentDescriptor, FederationComponentArchive) {
    (
        FederationComponentDescriptor {
            component_id: component_id.into(),
            archive_identity: archive_identity.into(),
            kind,
            format: format.into(),
            archive_version,
            digest: component_digest(&archive_bytes),
            dependencies: dependencies.iter().map(|value| (*value).into()).collect(),
        },
        FederationComponentArchive {
            component_id: component_id.into(),
            archive_bytes,
        },
    )
}

fn phase(phase: GrovePhase, event: &str, state: &str) -> FederationPhaseRecord {
    FederationPhaseRecord {
        phase,
        federation_event_id: event.into(),
        state_id: state.into(),
    }
}

#[allow(clippy::too_many_arguments)]
fn event(
    id: &str,
    component: &str,
    domain_event: &str,
    causal_position: u64,
    accepted: bool,
    result_state: Option<&str>,
    caused_by: &[&str],
) -> FederatedEventReference {
    FederatedEventReference {
        federation_event_id: id.into(),
        component_id: component.into(),
        domain_event_id: domain_event.into(),
        subject_id: "subject.central-junction.four-house-bridge".into(),
        causal_position,
        accepted,
        result_state_id: result_state.map(Into::into),
        evidence_ids: vec![format!("evidence.{domain_event}")],
        authority_ids: vec![format!("authority.{domain_event}")],
        provenance_ids: vec![format!("provenance.{domain_event}")],
        caused_by: caused_by.iter().map(|value| (*value).into()).collect(),
    }
}

fn canonical_events() -> Vec<FederatedEventReference> {
    vec![
        event(
            "federation-event.operation.way-back",
            FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID,
            "grove-cycle.bridge-control.rejected",
            10,
            true,
            Some("state.bridge-control.v1"),
            &[],
        ),
        event(
            "federation-event.operation.initiation",
            FEDERATION_TOURNAMENT_COMPONENT_ID,
            "scenario.canonical-year.west-bridge-breach",
            20,
            true,
            Some("state.bridge-control.candidate"),
            &["federation-event.operation.way-back"],
        ),
        event(
            "federation-event.operation.gathering",
            FEDERATION_TOURNAMENT_COMPONENT_ID,
            "event.canonical-year.gathering.service-tournament",
            30,
            true,
            Some("state.bridge-control.related"),
            &["federation-event.operation.initiation"],
        ),
        event(
            "federation-event.operation.rejected",
            FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID,
            "grove-cycle.bridge-control.rejected",
            40,
            false,
            None,
            &["federation-event.operation.gathering"],
        ),
        event(
            "federation-event.operation.real-emergency",
            FEDERATION_TOURNAMENT_COMPONENT_ID,
            "scenario.canonical-year.real-emergency-response",
            41,
            true,
            None,
            &["federation-event.operation.gathering"],
        ),
        event(
            "federation-event.operation.restraint",
            FEDERATION_TOURNAMENT_COMPONENT_ID,
            "objective.canonical-year.bridge-restraint",
            42,
            true,
            None,
            &["federation-event.operation.gathering"],
        ),
        event(
            "federation-event.operation.permanence-petition",
            FEDERATION_PERMANENCE_COMPONENT_ID,
            "petition.permanence.bridge",
            43,
            true,
            None,
            &[
                "federation-event.operation.real-emergency",
                "federation-event.operation.restraint",
            ],
        ),
        event(
            "federation-event.operation.accepted",
            FEDERATION_FUNCTION_JUNCTION_COMPONENT_ID,
            "grove-cycle.bridge-control.accepted",
            50,
            true,
            Some("state.bridge-control.v2"),
            &[
                "federation-event.operation.permanence-petition",
                "federation-event.operation.rejected",
            ],
        ),
    ]
}
