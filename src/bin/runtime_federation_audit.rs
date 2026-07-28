use std::process::ExitCode;

use hollow_grove::constitutional::{GrovePhase, V2_RULE_SET};
use hollow_grove::runtime_federation::{
    FederationComponentKind, RUNTIME_FEDERATION_ARCHIVE_FORMAT, RUNTIME_FEDERATION_ARCHIVE_VERSION,
    RUNTIME_FEDERATION_CANONICAL_NAME, RUNTIME_FEDERATION_IDENTITY,
    decode_runtime_federation_archive, encode_runtime_federation_archive,
};
use hollow_grove::runtime_federation_fixture::canonical_runtime_federation_payload;

fn main() -> ExitCode {
    match audit() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("FAIL The Runtime Federation audit: {error}");
            ExitCode::FAILURE
        }
    }
}

fn require(condition: bool, label: impl AsRef<str>) -> Result<(), String> {
    if condition {
        println!("PASS {}", label.as_ref());
        Ok(())
    } else {
        Err(label.as_ref().into())
    }
}

fn audit() -> Result<(), String> {
    let payload = canonical_runtime_federation_payload().map_err(|error| error.to_string())?;
    let bytes = encode_runtime_federation_archive(&payload).map_err(|error| error.to_string())?;
    let (decoded, runtime) =
        decode_runtime_federation_archive(&bytes).map_err(|error| error.to_string())?;
    let manifest = runtime.manifest();

    println!("{RUNTIME_FEDERATION_CANONICAL_NAME} V1 Audit");
    require(
        manifest.canonical_name == RUNTIME_FEDERATION_CANONICAL_NAME
            && manifest.federation_identity == RUNTIME_FEDERATION_IDENTITY
            && manifest.archive_version == RUNTIME_FEDERATION_ARCHIVE_VERSION,
        format!(
            "formal identity={} archive={}/v{}",
            manifest.federation_identity,
            RUNTIME_FEDERATION_ARCHIVE_FORMAT,
            manifest.archive_version
        ),
    )?;
    require(
        manifest.ruleset_identity == V2_RULE_SET,
        format!("ruleset identity={}", manifest.ruleset_identity),
    )?;
    require(
        manifest.canonical_year_identity == "central-junction.canonical-year.2047.v1",
        format!(
            "canonical-year identity={}",
            manifest.canonical_year_identity
        ),
    )?;
    require(
        manifest.components.len() == FederationComponentKind::REQUIRED.len()
            && FederationComponentKind::REQUIRED.into_iter().all(|kind| {
                manifest
                    .components
                    .iter()
                    .any(|component| component.kind == kind)
            }),
        format!(
            "component archive identities and versions={}",
            manifest.components.len()
        ),
    )?;
    for component in &manifest.components {
        require(
            !component.digest.is_empty() && runtime.component(&component.component_id).is_some(),
            format!(
                "component={} kind={} format={}/v{} checksum={}",
                component.component_id,
                component.kind.display_name(),
                component.format,
                component.archive_version,
                component.digest
            ),
        )?;
    }
    require(
        manifest
            .components
            .iter()
            .find(|component| component.kind == FederationComponentKind::CompletedKernelPass)
            .is_some_and(|component| component.dependencies.is_empty()),
        "universal-kernel purity: completed bounded pass is a root component",
    )?;
    require(
        manifest.current_phase == GrovePhase::TheFestival
            && GrovePhase::ALL
                == [
                    GrovePhase::TheWayBack,
                    GrovePhase::TheInitiation,
                    GrovePhase::TheGathering,
                    GrovePhase::TheFestival,
                ]
            && GrovePhase::TheFestival.next() == GrovePhase::TheWayBack,
        "current Grove phase=The Festival; fixed four-phase loop closes at The Way Back",
    )?;
    require(
        manifest.physical_checkpoint_id != manifest.digital_checkpoint_id,
        format!(
            "physical checkpoint={} digital checkpoint={}",
            manifest.physical_checkpoint_id, manifest.digital_checkpoint_id
        ),
    )?;
    require(
        manifest
            .components
            .iter()
            .any(|component| component.kind == FederationComponentKind::WorldPoint),
        "world Point binding replayed through HGPNT",
    )?;
    require(
        manifest
            .components
            .iter()
            .any(|component| component.kind == FederationComponentKind::SeasonalFunctionJunction),
        "Function Junction and House Season state replayed through HGFJP",
    )?;
    require(
        manifest
            .components
            .iter()
            .any(|component| component.kind == FederationComponentKind::ServiceTournament)
            && manifest.first_playable_proof.nonlethal,
        "Service Tournament nesting and nonlethality",
    )?;
    require(
        manifest
            .components
            .iter()
            .any(|component| component.kind == FederationComponentKind::RouteAndPassage),
        "route direction and body/force/route distinctions replayed through HGSEA",
    )?;
    require(
        manifest
            .components
            .iter()
            .any(|component| component.kind == FederationComponentKind::Permanence)
            && manifest.first_playable_proof.permanence_proof_ids.len() == 4,
        "Permanence petition has four House proofs and Stonebend seal replay",
    )?;
    require(
        manifest
            .components
            .iter()
            .any(|component| component.kind == FederationComponentKind::AuthoritativeGameplay),
        "authoritative gameplay archive linkage=schema V3",
    )?;
    let rejected = runtime
        .event(&manifest.first_playable_proof.rejected_attempt_event_id)
        .ok_or_else(|| "rejected operation event missing".to_owned())?;
    require(
        !rejected.accepted
            && rejected.result_state_id.is_none()
            && !rejected.evidence_ids.is_empty(),
        "rejection history retains evidence without false state mutation",
    )?;
    let accepted = runtime
        .event(&manifest.accepted_result_event_id)
        .ok_or_else(|| "accepted result event missing".to_owned())?;
    require(
        accepted.accepted
            && accepted.result_state_id.as_deref() == Some(manifest.confirmed_state_id.as_str())
            && manifest.confirmed_state_id == manifest.next_way_back_state_id,
        "accepted Festival result becomes the next Way Back input",
    )?;
    require(
        runtime
            .event(&manifest.first_playable_proof.real_emergency_event_id)
            .is_some(),
        "real-emergency distinction",
    )?;
    require(
        runtime
            .event(&manifest.first_playable_proof.cross_domain_event_id)
            .is_some_and(|event| !event.caused_by.is_empty()),
        "cross-domain causal reference",
    )?;
    require(
        runtime
            .event(
                &manifest
                    .first_playable_proof
                    .constitutional_restraint_event_id,
            )
            .is_some(),
        "constitutional restraint decision",
    )?;
    require(
        encode_runtime_federation_archive(&decoded).map_err(|error| error.to_string())? == bytes,
        "deterministic exact aggregate replay",
    )?;
    let mut reversed = decoded;
    reversed.manifest.components.reverse();
    reversed.manifest.events.reverse();
    reversed.component_archives.reverse();
    require(
        encode_runtime_federation_archive(&reversed).map_err(|error| error.to_string())? == bytes,
        "insertion-order independence",
    )?;
    require(
        !manifest.presentation_authoritative
            && manifest.first_playable_proof.presentation_reads_only,
        "presentation non-authority",
    )?;
    require(!manifest.transfers_sovereignty, "no sovereignty transfer")?;
    println!("PASS aggregate checksum={}", manifest.aggregate_digest);
    println!("PASS archive bytes={}", bytes.len());
    Ok(())
}
