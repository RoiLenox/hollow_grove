//! Executable invariants shared by domain archives and presentation seams.
//!
//! This module deliberately audits boundaries; it does not reduce gameplay or
//! make House decisions. A passing report means the records are safe to hand
//! to the Rampart and to read from a presentation adapter.

use std::fmt;

use crate::ai_personas::{SharedInterfaceRecord, replay_advisory};
use crate::gameplay::{
    GAMEPLAY_ARCHIVE_FORMAT, GAMEPLAY_RAMPART_COMPONENT_ID, GameplayRampartBinding,
};
use crate::guide_identity::GuideContextV1;
use crate::runtime_rampart::{
    RUNTIME_RAMPART_ARCHIVE_FORMAT, RUNTIME_RAMPART_ARCHIVE_VERSION, RUNTIME_RAMPART_IDENTITY,
};

pub const KERNEL_INVARIANT_SCHEMA_VERSION: u16 = 1;
pub const CENTRAL_JUNCTION_COORDINATION_IDENTITY: &str = "region.central-junction";
pub const PLATINUM_PATH_ID: &str = "path.platinum.v1";
pub const PLATINUM_PATH_NAME: &str = "Platinum Path";
pub const PLATINUM_PATH_SHORT_NAME: &str = "the Plath";

const INVARIANTS: [&str; 7] = [
    "identity-continuity-preserved",
    "advisories-never-commit-world-state",
    "shared-records-have-provenance",
    "rampart-binding-is-replayable",
    "refusal-remains-lawful",
    "central-junction-is-coordination-only",
    "roi-and-roselina-oversee-guide-context",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelInvariantReport {
    pub schema_version: u16,
    pub checked_invariants: Vec<&'static str>,
    pub violations: Vec<String>,
}

impl KernelInvariantReport {
    #[must_use]
    pub fn passed(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn ensure_passed(&self) -> Result<(), KernelInvariantError> {
        self.violations
            .first()
            .cloned()
            .map_or(Ok(()), |violation| Err(KernelInvariantError(violation)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelInvariantError(pub String);

impl fmt::Display for KernelInvariantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "kernel invariant failed: {}", self.0)
    }
}

impl std::error::Error for KernelInvariantError {}

/// Audits the shared gameplay/Rampart boundary without changing state.
#[must_use]
pub fn audit_gameplay_rampart_binding(binding: &GameplayRampartBinding) -> KernelInvariantReport {
    let mut violations = Vec::new();
    if binding.rampart_identity != RUNTIME_RAMPART_IDENTITY
        && binding.rampart_identity != crate::runtime_rampart::LEGACY_RUNTIME_FEDERATION_IDENTITY
    {
        violations.push("Rampart identity is not canonical".into());
    }
    if binding.rampart_archive_format != RUNTIME_RAMPART_ARCHIVE_FORMAT
        || binding.rampart_archive_version != RUNTIME_RAMPART_ARCHIVE_VERSION
    {
        violations.push("Rampart archive format or version is not canonical".into());
    }
    if binding.component_id != GAMEPLAY_RAMPART_COMPONENT_ID {
        violations.push("gameplay binding is not the authoritative component".into());
    }
    for advisory in &binding.advisory_records {
        let Ok(bytes) = serde_json::to_vec(advisory) else {
            violations.push("advisory record is not serializable".into());
            continue;
        };
        if let Err(error) = replay_advisory(&bytes) {
            violations.push(error.to_string());
        }
    }
    for record in &binding.shared_interface_records {
        audit_shared_interface_record(record, &mut violations);
    }
    KernelInvariantReport {
        schema_version: KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: INVARIANTS.to_vec(),
        violations,
    }
}

#[deprecated(since = "0.1.0", note = "use audit_gameplay_rampart_binding")]
#[must_use]
pub fn audit_gameplay_federation_binding(
    binding: &GameplayRampartBinding,
) -> KernelInvariantReport {
    audit_gameplay_rampart_binding(binding)
}

/// Audits a Guide projection and requires both canonical Guide identities.
#[must_use]
pub fn audit_guide_context(context: &GuideContextV1) -> KernelInvariantReport {
    let mut violations = Vec::new();
    if let Err(error) = context.validate() {
        violations.push(error.to_string());
    }
    if context.guides.len() != 2 {
        violations.push("Guide oversight requires exactly Roi and Roselina".into());
    }
    let identities = context
        .guides
        .iter()
        .map(|guide| guide.identity)
        .collect::<std::collections::BTreeSet<_>>();
    if identities
        != crate::guide_identity::GuideIdentity::ALL
            .into_iter()
            .collect()
    {
        violations.push("Guide oversight identity set is not Roi and Roselina".into());
    }
    KernelInvariantReport {
        schema_version: KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: INVARIANTS.to_vec(),
        violations,
    }
}

fn audit_shared_interface_record(record: &SharedInterfaceRecord, violations: &mut Vec<String>) {
    for (label, value) in [
        ("record_id", &record.record_id),
        ("subject_id", &record.subject_id),
        ("authority_id", &record.authority_id),
        ("status", &record.status),
        ("provenance_id", &record.provenance_id),
    ] {
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            violations.push(format!("shared interface {label} is empty or unstable"));
        }
    }
}

/// Stable declaration used by audits and documentation: Central Junction is
/// the shared coordination district, not a fifth House or decision-maker.
#[must_use]
pub fn central_junction_is_coordination_only() -> bool {
    CENTRAL_JUNCTION_COORDINATION_IDENTITY == "region.central-junction"
        && GAMEPLAY_ARCHIVE_FORMAT == "hollow-grove-gameplay"
}

/// Boardwalk influence is domain state, never a new kernel sovereign or House.
#[must_use]
pub fn audit_boardwalk_territory(
    territory: Option<&crate::gameplay::BoardwalkTerritoryView>,
) -> KernelInvariantReport {
    use crate::gameplay::{BoardwalkFactionId, boardwalk_factions};
    use crate::hollow_grove_contract::House;
    use crate::world::geography::ConstitutionalRouteId;
    use crate::world::route_network::{RouteGeometryClass, RouteNetwork};

    let mut violations = Vec::new();
    let network = RouteNetwork::canonical();
    let geography_preserved = network.as_ref().is_ok_and(|network| {
        network.segments().len() == 10
            && network
                .segment(ConstitutionalRouteId::Boardwalk)
                .is_some_and(|route| {
                    route.geometry == RouteGeometryClass::Straight
                        && route.endpoints.contains(&House::Flynt)
                        && route.endpoints.contains(&House::Glaushouse)
                })
            && network
                .segment(ConstitutionalRouteId::Riptide)
                .is_some_and(|route| route.geometry == RouteGeometryClass::Round)
    });
    if !geography_preserved {
        violations.push("Boardwalk and Riptide route identities changed".into());
    }
    for faction in boardwalk_factions() {
        if faction.rival_faction != faction.faction_id.rival()
            || faction.sovereign_house_affiliation.is_some()
            || faction.syndicate_organization.is_some()
        {
            violations.push("Boardwalk faction acquired unauthorized affiliation".into());
        }
    }
    if let Some(territory) = territory {
        if !territory.formal_neutrality
            || territory.sovereign_faction.is_some()
            || !territory.public_access
            || !territory.contested
        {
            violations.push("Boardwalk influence displaced neutral public passage".into());
        }
        if territory.territory_id != territory.level.territory_id()
            || territory.route_id != ConstitutionalRouteId::Boardwalk.stable_id()
            || territory.dominant_faction != territory.influence.dominant()
            || territory.rival_faction
                != territory
                    .dominant_faction
                    .unwrap_or(territory.level.home_faction())
                    .rival()
        {
            violations.push("Boardwalk territory identity or influence projection diverged".into());
        }
        if !territory
            .local_presence
            .iter()
            .any(|p| p.faction_id == BoardwalkFactionId::Trooners)
            || !territory
                .local_presence
                .iter()
                .any(|p| p.faction_id == BoardwalkFactionId::Gooners)
        {
            violations.push("Boardwalk ambient rival presence is missing".into());
        }
    }
    KernelInvariantReport {
        schema_version: KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: vec![
            "boardwalk-straight-riptide-curved-network-preserved",
            "boardwalk-factions-separate-from-house-and-syndicate",
            "boardwalk-influence-never-grants-sovereignty-or-closes-passage",
        ],
        violations,
    }
}

/// Audits the live host-specific Form without granting any later Synthesis.
#[must_use]
pub fn audit_sympiote_foundation(
    state: &crate::world::sympiote_foundation::SympioteFoundationState,
) -> KernelInvariantReport {
    let mut violations: Vec<String> = state
        .validate()
        .err()
        .map(|error| error.to_string())
        .into_iter()
        .collect();
    if let Err(error) =
        crate::world::sympiote_foundation::sympiote_foundation_lore().operational_handoffs()
    {
        violations.push(error.to_string());
    }
    KernelInvariantReport {
        schema_version: KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: vec![
            "sympiote-host-and-form-continuity-preserved",
            "sympiote-clinical-consent-and-authority-preserved",
            "sympiote-rest-and-repeatable-use-preserved",
            "sympiote-practice-and-offers-never-grant-later-forms",
        ],
        violations,
    }
}

/// Audits the bounded object state and its separate operational handoffs.
#[must_use]
pub fn audit_survey_frame(
    state: &crate::world::survey_frame::SurveyFrameState,
) -> KernelInvariantReport {
    let mut violations: Vec<String> = state
        .validate()
        .err()
        .map(|e| e.to_string())
        .into_iter()
        .collect();
    if let Err(error) = crate::world::survey_frame::survey_frame_lore().operational_handoffs() {
        violations.push(error.to_string());
    }
    KernelInvariantReport {
        schema_version: KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: vec![
            "survey-frame-object-identity-and-revision-continuity",
            "survey-frame-measurement-is-not-support",
            "survey-frame-authority-resident-choice-and-delivery-separated",
            "survey-frame-negative-trial-and-repair-history-retained",
            "survey-frame-withdrawal-requires-recovery",
            "survey-frame-bounded-domain-records",
        ],
        violations,
    }
}

/// Audits authored lore contracts without treating them as executed cases.
#[must_use]
pub fn audit_functional_lore_definitions(
    definitions: &[crate::world::lived_lore::FunctionalLoreDefinition],
) -> KernelInvariantReport {
    KernelInvariantReport {
        schema_version: KERNEL_INVARIANT_SCHEMA_VERSION,
        checked_invariants: vec![
            "lore-identity-and-jurisdiction-preserved",
            "lore-role-handoffs-have-exact-endpoints",
            "lore-house-and-route-coverage-preserved",
        ],
        violations: crate::world::lived_lore::validate_definitions(definitions)
            .err()
            .map(|error| error.to_string())
            .into_iter()
            .collect(),
    }
}
