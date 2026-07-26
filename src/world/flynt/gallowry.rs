//! World-facing access adapter for the Gallowry, the hidden headquarters and
//! cultural home of the Gallows.
//!
//! The Gallowry is a site, never an institution. Membership and authority are
//! always evaluated against the Gallows institution defined by the canonical
//! Flynt projection.

use crate::institution::{
    AccessPolicy, AccessRequirement, AccessRequirementMatch, InstitutionalBeingId, RoleId, SiteId,
    ZoneId,
};
use crate::institution_affiliation::{AccessDecision, AffiliationState, InstitutionalWorldState};

use super::{gallowry_site_id, gallows_id, gallows_member_role_id};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GallowryZone {
    MeetingPlace,
    CulturalCenter,
    Gallery,
    OperationalHub,
}

#[must_use]
pub fn zone_id(zone: GallowryZone) -> ZoneId {
    let value = match zone {
        GallowryZone::MeetingPlace => "zone.flynt.gallowry.meeting-place",
        GallowryZone::CulturalCenter => "zone.flynt.gallowry.cultural-center",
        GallowryZone::Gallery => "zone.flynt.gallowry.gallery",
        GallowryZone::OperationalHub => "zone.flynt.gallowry.operational-hub",
    };
    ZoneId::new(value).expect("canonical Gallowry zone ID")
}

#[must_use]
pub fn member_access_policy() -> AccessPolicy {
    AccessPolicy {
        matching: AccessRequirementMatch::Any,
        requirements: vec![
            AccessRequirement::Role(gallows_member_role_id()),
            AccessRequirement::ExplicitGrant,
        ],
    }
}

pub struct GallowryDomain<'a> {
    state: &'a InstitutionalWorldState,
}

impl<'a> GallowryDomain<'a> {
    #[must_use]
    pub const fn new(state: &'a InstitutionalWorldState) -> Self {
        Self { state }
    }

    #[must_use]
    pub fn is_gallows_member(&self, being: &InstitutionalBeingId) -> bool {
        self.state
            .membership_of(being, &gallows_id())
            .is_some_and(|membership| {
                !matches!(
                    membership.affiliation_state,
                    AffiliationState::None
                        | AffiliationState::Former
                        | AffiliationState::Suspended
                        | AffiliationState::Expelled
                ) && membership.role_id.as_ref() == Some(&gallows_member_role_id())
            })
    }

    #[must_use]
    pub fn access(&self, being: &InstitutionalBeingId, zone: GallowryZone) -> AccessDecision {
        self.state.evaluate_access(
            being,
            &gallows_id(),
            &gallowry_site_id(),
            &zone_id(zone),
            &member_access_policy(),
        )
    }

    #[must_use]
    pub fn can_access(&self, being: &InstitutionalBeingId, zone: GallowryZone) -> bool {
        self.access(being, zone) == AccessDecision::Allowed
    }
}

#[must_use]
pub fn is_gallowry_site(site: &SiteId) -> bool {
    site == &gallowry_site_id()
}

#[must_use]
pub fn is_gallows_role(role: &RoleId) -> bool {
    role == &gallows_member_role_id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::institution::{MembershipId, Visibility};
    use crate::institution_affiliation::{
        AffiliationState, InstitutionalMembership, LineageStatus, MembershipRole, RecognitionLevel,
    };
    use crate::world::canonical_institutional_world_state;

    fn being(value: &str) -> InstitutionalBeingId {
        InstitutionalBeingId::new(value).unwrap()
    }

    #[test]
    fn gallowry_zones_are_only_site_functions_from_canon() {
        assert_eq!(
            zone_id(GallowryZone::MeetingPlace).as_str(),
            "zone.flynt.gallowry.meeting-place"
        );
        assert_eq!(
            zone_id(GallowryZone::OperationalHub).as_str(),
            "zone.flynt.gallowry.operational-hub"
        );
    }

    #[test]
    fn gallows_membership_or_explicit_grant_controls_hidden_site_access() {
        let member = being("being.flynt.gallows-member-fixture");
        let mut state = canonical_institutional_world_state();
        state.memberships.push(InstitutionalMembership {
            id: MembershipId::new("membership.flynt.gallows-member-fixture").unwrap(),
            being: member.clone(),
            institution: gallows_id(),
            role_id: Some(gallows_member_role_id()),
            role: MembershipRole::FullMember,
            affiliation_state: AffiliationState::Initiated,
            lineage: LineageStatus::None,
            sponsor: None,
            subgroup: None,
            joined_at: Some(0),
            initiated_at: Some(0),
            ended_at: None,
            public_visibility: Visibility::Hidden,
            internal_recognition: RecognitionLevel::Internal,
        });

        let domain = GallowryDomain::new(&state);
        assert!(domain.is_gallows_member(&member));
        assert!(domain.can_access(&member, GallowryZone::MeetingPlace));
        assert!(!domain.can_access(
            &being("being.flynt.unaffiliated-fixture"),
            GallowryZone::MeetingPlace
        ));
    }
}
