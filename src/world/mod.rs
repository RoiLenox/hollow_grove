pub mod aura_basin;
pub mod aura_beach;
pub mod aura_field;
pub mod central_junction;
pub mod central_junction_seasonal_functions;
pub mod chroma_cord;
pub mod composition_witnesses;
pub mod constitutional_interfaces;
pub mod current_sea_passage;
pub mod extraction;
pub mod flynt;
pub mod fourway;
pub mod function_junction;
pub mod function_junction_archive;
pub mod function_junction_fixture;
pub mod geography;
pub mod glaushouse;
pub mod house_institutions;
pub mod house_scene_context;
pub mod hueman_faculties;
pub mod interior_surface;
pub mod lived_lore;
pub mod minoan_court;
pub mod permanence;
pub mod persistence;
pub mod power_recipes;
pub mod route_network;
pub mod sandmanor;
pub mod seasonal_functions_archive;
pub mod seasonal_functions_fixture;
pub mod service_tournament;
pub mod service_tournament_archive;
pub mod service_tournament_fixture;
pub mod session;
pub mod stonebend;
pub mod sympiote;
pub mod way_back;
pub mod world_point;
pub mod world_point_archive;
pub mod world_point_fixture;

use crate::institution::{
    InstitutionCatalog, InstitutionalBeingId, MembershipId, OfficeHolder, OfficeId, RoleId, SiteId,
    Visibility, ZoneId,
};
use crate::institution_affiliation::{
    AccessGrant, AccessGrantId, AffiliationState, InstitutionalMembership, InstitutionalWorldState,
    LineageStatus, MembershipRole, RecognitionLevel,
};

/// Joins the canonical House fixture catalogs into the one state container that
/// future runtime, traversal, and dialogue layers should receive.
pub fn canonical_institutional_world_state() -> InstitutionalWorldState {
    let mut catalog = house_institutions::canonical_house_institutions();
    let flynt = flynt::canonical_flynt_institutions();
    flynt
        .validate()
        .expect("the Flynt constitutional projection must validate before merge");
    merge_catalog(&mut catalog, flynt.catalog);
    catalog
        .validate()
        .expect("all canonical institutional fixtures must compose");
    InstitutionalWorldState::from_catalog(catalog)
}

/// Anonymous, non-canonical runtime examples for access, traversal, and scene
/// integration tests. These records do not establish a named NPC roster.
#[must_use]
pub fn institutional_access_fixture() -> InstitutionalWorldState {
    let mut state = canonical_institutional_world_state();
    let stonebend_member = being("being.stonebend.fixture-member");
    let sandmanor_member = being("being.sandmanor.fixture-member");
    let glaushouse_member = being("being.glaushouse.fixture-member");
    let flynt_member = being("being.flynt.fixture-member");
    state.memberships = vec![
        member(
            "membership.stonebend.fixture-member",
            stonebend_member.clone(),
            house_institutions::stonebend_constitution_id(),
            "role.stonebend.gerald",
            MembershipRole::Associate,
            AffiliationState::Associate,
        ),
        member(
            "membership.sandmanor.fixture-member",
            sandmanor_member.clone(),
            house_institutions::sandmen_id(),
            "role.sandmanor.minorian",
            MembershipRole::Associate,
            AffiliationState::Associate,
        ),
        member(
            "membership.glaushouse.fixture-member",
            glaushouse_member.clone(),
            glaushouse::glauspitals_id(),
            "role.glaushouse.recovery-staff",
            MembershipRole::Associate,
            AffiliationState::Associate,
        ),
        member(
            "membership.flynt.fixture-member",
            flynt_member.clone(),
            flynt::gallows_id(),
            "role.flynt.gallows-member",
            MembershipRole::FullMember,
            AffiliationState::Initiated,
        ),
    ];
    state.catalog.office_holders.extend([
        OfficeHolder {
            office: office("office.stonebend.hypergiant"),
            being: stonebend_member.clone(),
            active: true,
        },
        OfficeHolder {
            office: office("office.sandmanor.sandman"),
            being: sandmanor_member.clone(),
            active: true,
        },
        OfficeHolder {
            office: office("office.glaushouse.prima-donna"),
            being: glaushouse_member.clone(),
            active: true,
        },
    ]);
    state.access_grants = vec![
        grant(
            "access-grant.stonebend.fixture-guest",
            stonebend_member,
            house_institutions::stonebend_constitution_id(),
            "site.stonebend.stonebender",
            "zone.stonebend.stonebender.burden-relay",
        ),
        grant(
            "access-grant.sandmanor.fixture-guest",
            sandmanor_member,
            house_institutions::sandmen_id(),
            "site.sandmanor.aura-beach",
            "zone.sandmanor.aura-beach.court-strand",
        ),
        grant(
            "access-grant.glaushouse.fixture-guest",
            glaushouse_member,
            glaushouse::glauspitals_id(),
            "site.glaushouse.central-medical-district",
            "zone.glaushouse.medical-district.recovery-chambers",
        ),
        grant(
            "access-grant.flynt.fixture-guest",
            flynt_member,
            flynt::gallows_id(),
            "site.flynt.gallowry",
            "zone.flynt.gallowry.meeting-place",
        ),
    ];
    state
}

fn being(value: &str) -> InstitutionalBeingId {
    InstitutionalBeingId::new(value).expect("canonical fixture being ID")
}

fn member(
    id: &str,
    being: InstitutionalBeingId,
    institution: crate::institution::InstitutionId,
    role_id: &str,
    role: MembershipRole,
    affiliation_state: AffiliationState,
) -> InstitutionalMembership {
    InstitutionalMembership {
        id: MembershipId::new(id).expect("canonical fixture membership ID"),
        being,
        institution,
        role_id: Some(RoleId::new(role_id).expect("canonical fixture role ID")),
        role,
        affiliation_state,
        lineage: LineageStatus::None,
        sponsor: None,
        subgroup: None,
        joined_at: Some(0),
        initiated_at: (affiliation_state == AffiliationState::Initiated).then_some(0),
        ended_at: None,
        public_visibility: Visibility::Known,
        internal_recognition: RecognitionLevel::Internal,
    }
}

fn grant(
    id: &str,
    grantee: InstitutionalBeingId,
    institution: crate::institution::InstitutionId,
    site: &str,
    zone: &str,
) -> AccessGrant {
    AccessGrant {
        id: AccessGrantId::new(id).expect("canonical fixture grant ID"),
        grantee,
        institution,
        site: Some(SiteId::new(site).expect("canonical fixture site ID")),
        zone: Some(ZoneId::new(zone).expect("canonical fixture zone ID")),
        clearance: None,
        active: true,
    }
}

fn office(value: &str) -> OfficeId {
    OfficeId::new(value).expect("canonical fixture office ID")
}

fn merge_catalog(destination: &mut InstitutionCatalog, source: InstitutionCatalog) {
    destination.institutions.extend(source.institutions);
    destination.offices.extend(source.offices);
    destination.roles.extend(source.roles);
    destination.groups.extend(source.groups);
    destination.sites.extend(source.sites);
    destination.office_holders.extend(source.office_holders);
    destination.relationships.extend(source.relationships);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::institution::{AccessPolicy, AccessRequirement, AccessRequirementMatch, RoleId};
    use crate::world::flynt::{gallows_id, manticorp_id};
    #[test]
    fn aggregate_state_contains_all_four_house_fixture_sets() {
        let state = canonical_institutional_world_state();
        state.validate().unwrap();
        assert!(state.catalog.institution(&manticorp_id()).is_some());
        assert!(state.catalog.institution(&gallows_id()).is_some());
        assert!(
            state
                .catalog
                .institution(&house_institutions::stonebend_constitution_id())
                .is_some()
        );
        assert!(
            state
                .catalog
                .institution(&house_institutions::sandmen_id())
                .is_some()
        );
        assert!(
            state
                .catalog
                .institution(&house_institutions::glaushouse_medical_civilization_id())
                .is_some()
        );
    }
    #[test]
    fn anonymous_access_fixture_has_one_member_holder_and_grant_per_house() {
        let state = institutional_access_fixture();
        state.validate().unwrap();
        assert_eq!(state.memberships.len(), 4);
        assert_eq!(state.catalog.office_holders.len(), 4);
        assert_eq!(state.access_grants.len(), 4);
        let policy = AccessPolicy {
            matching: AccessRequirementMatch::Any,
            requirements: vec![AccessRequirement::ExplicitGrant],
        };
        for grant in &state.access_grants {
            assert_eq!(
                state.evaluate_access(
                    &grant.grantee,
                    &grant.institution,
                    grant.site.as_ref().unwrap(),
                    grant.zone.as_ref().unwrap(),
                    &policy,
                ),
                crate::institution_affiliation::AccessDecision::Allowed
            );
        }
    }
    #[test]
    fn detailed_membership_role_ids_drive_role_access() {
        let state = institutional_access_fixture();
        let member = being("being.flynt.fixture-member");
        let policy = AccessPolicy {
            matching: AccessRequirementMatch::Any,
            requirements: vec![AccessRequirement::Role(
                RoleId::new("role.flynt.gallows-member").unwrap(),
            )],
        };
        assert_eq!(
            state.evaluate_access(
                &member,
                &flynt::gallows_id(),
                &flynt::gallowry_site_id(),
                &ZoneId::new("zone.flynt.gallowry.meeting-place").unwrap(),
                &policy,
            ),
            crate::institution_affiliation::AccessDecision::Allowed
        );
    }
}
