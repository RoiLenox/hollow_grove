//! Read-only cultural scene projections for the non-Flynt Houses.
//!
//! These adapters expose institutional facts and cultural signals to scene and
//! presentation layers. They never select a tactic, resolve succession, or
//! mutate Current Synthesis or the Hollow Grove kernel.

use crate::institution::{
    AccessPolicy, AccessRequirement, AccessRequirementMatch, InstitutionId, InstitutionalBeingId,
    RoleId, SiteId, ZoneId,
};
use crate::institution_affiliation::{InstitutionalSceneContext, InstitutionalWorldState};

use super::glaushouse;
use super::house_institutions::{sandmen_id, stonebend_constitution_id};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseZoneContext {
    pub institution: InstitutionId,
    pub site: SiteId,
    pub zone: ZoneId,
    pub policy: AccessPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StonebendSceneSignal {
    PublicTitle,
    LaborWitness,
    StructuralCraft,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendSceneContext {
    pub institutional: InstitutionalSceneContext,
    pub signals: Vec<StonebendSceneSignal>,
    pub zones: Vec<HouseZoneContext>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandmanorSceneSignal {
    WitnessedImprovement,
    PublicProof,
    IntentionalArrangement,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanorSceneContext {
    pub institutional: InstitutionalSceneContext,
    pub signals: Vec<SandmanorSceneSignal>,
    pub zones: Vec<HouseZoneContext>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlaushouseSceneSignal {
    PublicClearance,
    TriageAndRelay,
    RecoveryFloorWitness,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlaushouseSceneContext {
    pub institutional: InstitutionalSceneContext,
    pub signals: Vec<GlaushouseSceneSignal>,
    pub zones: Vec<HouseZoneContext>,
}

/// Stonebend's public proving grounds are public; no contest or succession
/// resolver is implied by the context.
#[must_use]
pub fn stonebend_scene_context(
    state: &InstitutionalWorldState,
    observer: &InstitutionalBeingId,
    subject: &InstitutionalBeingId,
) -> StonebendSceneContext {
    let institution = stonebend_constitution_id();
    StonebendSceneContext {
        institutional: state.scene_context_for(observer, subject, &institution),
        signals: vec![
            StonebendSceneSignal::PublicTitle,
            StonebendSceneSignal::LaborWitness,
            StonebendSceneSignal::StructuralCraft,
        ],
        zones: zones_for(state, &institution, public_policy()),
    }
}

/// Sandmanor's proof spaces are public. This read-only projection does not
/// resolve reciprocal teaching, the Contest of Improvement, or Sandman
/// accession; `world::sandmanor` owns that executable House-specific law.
#[must_use]
pub fn sandmanor_scene_context(
    state: &InstitutionalWorldState,
    observer: &InstitutionalBeingId,
    subject: &InstitutionalBeingId,
) -> SandmanorSceneContext {
    let institution = sandmen_id();
    SandmanorSceneContext {
        institutional: state.scene_context_for(observer, subject, &institution),
        signals: vec![
            SandmanorSceneSignal::WitnessedImprovement,
            SandmanorSceneSignal::PublicProof,
            SandmanorSceneSignal::IntentionalArrangement,
        ],
        zones: zones_for(state, &institution, public_policy()),
    }
}

/// Glaüshouse clinical zones use explicit neutral access policies.
#[must_use]
pub fn glaushouse_scene_context(
    state: &InstitutionalWorldState,
    observer: &InstitutionalBeingId,
    subject: &InstitutionalBeingId,
) -> GlaushouseSceneContext {
    let institution = glaushouse::glauspitals_id();
    let mut zones = zones_for(state, &institution, glaushouse_member_policy());
    for context in &mut zones {
        context.policy = glaushouse_clinical_access_policy(&context.zone);
    }
    GlaushouseSceneContext {
        institutional: state.scene_context_for(observer, subject, &institution),
        signals: vec![
            GlaushouseSceneSignal::PublicClearance,
            GlaushouseSceneSignal::TriageAndRelay,
            GlaushouseSceneSignal::RecoveryFloorWitness,
        ],
        zones,
    }
}

fn zones_for(
    state: &InstitutionalWorldState,
    institution: &InstitutionId,
    policy: AccessPolicy,
) -> Vec<HouseZoneContext> {
    state
        .catalog
        .sites
        .iter()
        .filter(|site| site.controlled_by.as_ref() == Some(institution))
        .flat_map(|site| {
            let site_policy = policy.clone();
            site.zones
                .iter()
                .cloned()
                .map(move |zone| HouseZoneContext {
                    institution: institution.clone(),
                    site: site.id.clone(),
                    zone,
                    policy: site_policy.clone(),
                })
        })
        .collect()
}

fn public_policy() -> AccessPolicy {
    AccessPolicy {
        matching: AccessRequirementMatch::Any,
        requirements: vec![AccessRequirement::Public],
    }
}

fn glaushouse_member_policy() -> AccessPolicy {
    AccessPolicy {
        matching: AccessRequirementMatch::Any,
        requirements: vec![
            AccessRequirement::InstitutionMembership(glaushouse::glauspitals_id()),
            AccessRequirement::ExplicitGrant,
        ],
    }
}

/// Canonical clinical access policy, expressed only through neutral IDs and
/// requirements. It does not infer access from a display name.
#[must_use]
pub fn glaushouse_clinical_access_policy(zone: &ZoneId) -> AccessPolicy {
    match zone.as_str() {
        "zone.glaushouse.medical-district.diagnostic-halls" => public_policy(),
        "zone.glaushouse.medical-district.surgical-theaters"
        | "zone.glaushouse.medical-district.rehabilitation-pools" => glaushouse_member_policy(),
        "zone.glaushouse.medical-district.recovery-chambers" => AccessPolicy {
            matching: AccessRequirementMatch::Any,
            requirements: vec![
                AccessRequirement::Role(role_id("role.glaushouse.recovery-staff")),
                AccessRequirement::Role(glaushouse::persephone_rank_id()),
                AccessRequirement::Office(glaushouse::prima_donna_office_id()),
                AccessRequirement::ExplicitGrant,
            ],
        },
        _ => glaushouse_member_policy(),
    }
}

fn role_id(value: &str) -> RoleId {
    RoleId::new(value).expect("canonical stable role ID")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::institution::InstitutionalBeingId;
    use crate::institution_affiliation::ZoneEntryResult;
    use crate::world::{canonical_institutional_world_state, institutional_access_fixture};

    fn being(value: &str) -> InstitutionalBeingId {
        InstitutionalBeingId::new(value).unwrap()
    }

    #[test]
    fn house_adapters_project_context_without_a_resolver() {
        let state = canonical_institutional_world_state();
        let observer = being("being.test.observer");
        let subject = being("being.test.subject");
        let stonebend = stonebend_scene_context(&state, &observer, &subject);
        let sandmanor = sandmanor_scene_context(&state, &observer, &subject);
        let glaushouse = glaushouse_scene_context(&state, &observer, &subject);
        assert_eq!(stonebend.signals.len(), 3);
        assert!(
            stonebend
                .zones
                .iter()
                .all(|entry| entry.policy == public_policy())
        );
        assert_eq!(sandmanor.signals.len(), 3);
        assert!(
            sandmanor
                .zones
                .iter()
                .all(|entry| entry.policy == public_policy())
        );
        assert!(glaushouse.zones.iter().any(|entry| {
            entry.zone.as_str() == "zone.glaushouse.medical-district.diagnostic-halls"
                && entry.policy == public_policy()
        }));
        assert!(
            glaushouse
                .zones
                .iter()
                .any(|entry| entry.policy == glaushouse_member_policy())
        );
        let recovery = glaushouse
            .zones
            .iter()
            .find(|entry| {
                entry.zone.as_str() == "zone.glaushouse.medical-district.recovery-chambers"
            })
            .unwrap();
        assert!(
            recovery
                .policy
                .requirements
                .contains(&AccessRequirement::Office(
                    glaushouse::prima_donna_office_id()
                ))
        );
    }
    #[test]
    fn clinical_zone_entry_returns_presentable_denial_or_allowance() {
        let state = institutional_access_fixture();
        let holder = being("being.glaushouse.fixture-member");
        let outsider = being("being.test.outsider");
        let context = glaushouse_scene_context(&state, &outsider, &holder);
        let recovery = context
            .zones
            .iter()
            .find(|entry| {
                entry.zone.as_str() == "zone.glaushouse.medical-district.recovery-chambers"
            })
            .unwrap();
        assert!(matches!(
            state.request_zone_entry(
                &holder,
                &recovery.institution,
                &recovery.site,
                &recovery.zone,
                &recovery.policy,
            ),
            ZoneEntryResult::Allowed(_)
        ));
        match state.request_zone_entry(
            &outsider,
            &recovery.institution,
            &recovery.site,
            &recovery.zone,
            &recovery.policy,
        ) {
            ZoneEntryResult::Denied(denial) => {
                assert_eq!(denial.zone, recovery.zone);
                assert!(!denial.unmet_requirements.is_empty());
            }
            ZoneEntryResult::Allowed(_) => panic!("outsider must not enter recovery chambers"),
        }
    }
}
