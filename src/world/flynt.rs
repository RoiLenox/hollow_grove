//! Canonical projection of the Flynt constitution into Hollow Grove's neutral
//! institutional records.
//!
//! `flynt_constitution` owns constitutional meaning. This module projects that
//! meaning into world-facing offices, institutions, groups, sites, roles, and
//! relationships without creating a second hierarchy.

pub mod gallowry;

use std::collections::HashSet;

use crate::hollow_grove_contract::House;
use crate::institution::*;
use crate::institution_affiliation::{
    AffiliationState, InstitutionalMembership, InstitutionalWorldState,
};
use flynt_constitution::{ConstitutionError, FlyntConstitution};

fn id<T>(value: &str, make: impl FnOnce(String) -> Result<T, IdError>) -> T {
    make(value.into()).expect("canonical Flynt stable ID")
}
fn institution_id(value: &str) -> InstitutionId {
    id(value, InstitutionId::new)
}
fn office_id(value: &str) -> OfficeId {
    id(value, OfficeId::new)
}
fn role_id(value: &str) -> RoleId {
    id(value, RoleId::new)
}
fn group_id(value: &str) -> GroupId {
    id(value, GroupId::new)
}
fn site_id(value: &str) -> SiteId {
    id(value, SiteId::new)
}
fn zone_id(value: &str) -> ZoneId {
    id(value, ZoneId::new)
}
fn being_id(value: &str) -> InstitutionalBeingId {
    id(value, InstitutionalBeingId::new)
}
fn relationship_id(value: &str) -> RelationshipId {
    id(value, RelationshipId::new)
}

pub fn tross_office_id() -> OfficeId {
    office_id("office.flynt.tross")
}
pub fn tross_being_id() -> InstitutionalBeingId {
    being_id("being.flynt.tross")
}
#[must_use]
pub fn manticorp_form_id() -> &'static str {
    flynt_constitution::FORM_MANTICORP
}
pub fn constitutional_chimera_id() -> InstitutionalBeingId {
    being_id("being.flynt.constitutional-chimera")
}
pub fn manticorp_id() -> InstitutionId {
    institution_id("institution.flynt.manticorp")
}
pub fn mystery_men_id() -> InstitutionId {
    institution_id("institution.flynt.mystery-men")
}
pub fn mystery_man_role_id() -> RoleId {
    role_id("role.flynt.mystery-man")
}
pub fn gallows_id() -> InstitutionId {
    institution_id("institution.flynt.gallows")
}
pub fn we_fairy_men_group_id() -> GroupId {
    group_id("group.flynt.we-fairy-men")
}
pub fn gallowry_site_id() -> SiteId {
    site_id("site.flynt.gallowry")
}
pub fn manticorp_member_role_id() -> RoleId {
    role_id("role.flynt.manticorp-member")
}
pub fn mystery_operative_role_id() -> RoleId {
    role_id("role.flynt.mystery-operative")
}
pub fn gallows_member_role_id() -> RoleId {
    role_id("role.flynt.gallows-member")
}

pub fn bro_white_office_id() -> OfficeId {
    office_id("office.flynt.bro-white")
}
pub fn cinderellaman_office_id() -> OfficeId {
    office_id("office.flynt.cinderellaman")
}
pub fn the_beauty_office_id() -> OfficeId {
    office_id("office.flynt.the-beauty")
}
pub fn bro_white_crew_id() -> GroupId {
    group_id("group.flynt.bro-white-and-the-7-brothas")
}
pub fn cinderellaman_crew_id() -> GroupId {
    group_id("group.flynt.cinderellaman-and-his-midnight-crew")
}
pub fn the_beauty_crew_id() -> GroupId {
    group_id("group.flynt.the-beauty-and-his-beasts")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MysterySpecialty {
    Investigation,
    Intelligence,
    Counterintelligence,
    CovertOperations,
    OrganizedCrime,
    Contraband,
    Espionage,
    ConstitutionalSecurity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MysteryOperativeProfile {
    pub being: InstitutionalBeingId,
    pub codename: String,
    pub specialties: Vec<MysterySpecialty>,
    pub clearance: ClearanceLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeploymentStatus {
    Ready,
    Deployed,
    Recovering,
    Reserve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManticorpPersonnelProfile {
    pub being: InstitutionalBeingId,
    pub rank: RankId,
    pub unit: Option<GroupId>,
    pub deployment_status: DeploymentStatus,
}

#[derive(Debug)]
pub struct FlyntInstitutions {
    pub catalog: InstitutionCatalog,
    pub memberships: Vec<InstitutionalMembership>,
    pub mystery_operatives: Vec<MysteryOperativeProfile>,
    pub manticorp_personnel: Vec<ManticorpPersonnelProfile>,
    constitution: FlyntConstitution,
}

impl FlyntInstitutions {
    #[must_use]
    pub fn constitution(&self) -> &FlyntConstitution {
        &self.constitution
    }

    #[must_use]
    pub fn mystery_operative_profile(
        &self,
        being: &InstitutionalBeingId,
    ) -> Option<&MysteryOperativeProfile> {
        self.mystery_operatives
            .iter()
            .find(|entry| &entry.being == being)
    }

    #[must_use]
    pub fn manticorp_personnel_profile(
        &self,
        being: &InstitutionalBeingId,
    ) -> Option<&ManticorpPersonnelProfile> {
        self.manticorp_personnel
            .iter()
            .find(|entry| &entry.being == being)
    }

    #[must_use]
    pub fn is_gallows_member(&self, being: &InstitutionalBeingId) -> bool {
        self.memberships.iter().any(|membership| {
            &membership.being == being
                && membership.institution == gallows_id()
                && membership.role_id.as_ref() == Some(&gallows_member_role_id())
                && !matches!(
                    membership.affiliation_state,
                    AffiliationState::None
                        | AffiliationState::Former
                        | AffiliationState::Suspended
                        | AffiliationState::Expelled
                )
        })
    }

    pub fn validate(&self) -> Result<(), FlyntValidationError> {
        self.constitution
            .validate()
            .map_err(FlyntValidationError::Constitution)?;
        self.catalog
            .validate()
            .map_err(FlyntValidationError::Catalog)?;
        InstitutionalWorldState {
            catalog: self.catalog.clone(),
            memberships: self.memberships.clone(),
            ..Default::default()
        }
        .validate()
        .map_err(|_| FlyntValidationError::InvalidMembership)?;

        validate_exact_ids(
            self.catalog
                .institutions
                .iter()
                .map(|entry| entry.id.as_str()),
            &[
                manticorp_id().as_str(),
                mystery_men_id().as_str(),
                gallows_id().as_str(),
            ],
            FlyntValidationError::InstitutionRoster,
        )?;
        validate_exact_ids(
            self.catalog.offices.iter().map(|entry| entry.id.as_str()),
            &[
                tross_office_id().as_str(),
                bro_white_office_id().as_str(),
                cinderellaman_office_id().as_str(),
                the_beauty_office_id().as_str(),
            ],
            FlyntValidationError::OfficeRoster,
        )?;
        validate_exact_ids(
            self.catalog.groups.iter().map(|entry| entry.id.as_str()),
            &[
                we_fairy_men_group_id().as_str(),
                bro_white_crew_id().as_str(),
                cinderellaman_crew_id().as_str(),
                the_beauty_crew_id().as_str(),
            ],
            FlyntValidationError::GroupRoster,
        )?;
        validate_exact_ids(
            self.catalog.roles.iter().map(|entry| entry.id.as_str()),
            &[
                manticorp_member_role_id().as_str(),
                mystery_operative_role_id().as_str(),
                mystery_man_role_id().as_str(),
                gallows_member_role_id().as_str(),
            ],
            FlyntValidationError::RoleRoster,
        )?;
        validate_exact_ids(
            self.catalog.sites.iter().map(|entry| entry.id.as_str()),
            &[gallowry_site_id().as_str()],
            FlyntValidationError::SiteRoster,
        )?;

        if self
            .catalog
            .institutions
            .iter()
            .any(|institution| institution.name == "The Gallowry")
        {
            return Err(FlyntValidationError::GallowryIsInstitution);
        }
        let gallowry = self
            .catalog
            .sites
            .iter()
            .find(|site| site.id == gallowry_site_id())
            .ok_or(FlyntValidationError::GallowrySite)?;
        if gallowry.controlled_by.as_ref() != Some(&gallows_id()) {
            return Err(FlyntValidationError::GallowrySite);
        }
        if self
            .catalog
            .offices
            .iter()
            .any(|office| office.name == "Chimera")
        {
            return Err(FlyntValidationError::ChimeraIsOffice);
        }
        let active_tross_holders: Vec<_> = self
            .catalog
            .office_holders
            .iter()
            .filter(|holder| holder.active && holder.office == tross_office_id())
            .collect();
        if active_tross_holders.len() != 1 || active_tross_holders[0].being != tross_being_id() {
            return Err(FlyntValidationError::InvalidTrossHolder);
        }
        validate_authority_relationships(&self.catalog)?;
        if self.catalog != canonical_catalog() {
            return Err(FlyntValidationError::ProjectionMismatch);
        }
        Ok(())
    }
}

fn validate_exact_ids<'a>(
    actual: impl Iterator<Item = &'a str>,
    expected: &[&str],
    error: FlyntValidationError,
) -> Result<(), FlyntValidationError> {
    let actual: HashSet<_> = actual.collect();
    let expected: HashSet<_> = expected.iter().copied().collect();
    if actual == expected {
        Ok(())
    } else {
        Err(error)
    }
}

fn required_relationships() -> Vec<(
    InstitutionalEntityId,
    RelationshipKind,
    InstitutionalEntityId,
)> {
    vec![
        (
            InstitutionalEntityId::Office(tross_office_id()),
            RelationshipKind::Commands,
            InstitutionalEntityId::Being(constitutional_chimera_id()),
        ),
        (
            InstitutionalEntityId::Being(constitutional_chimera_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Office(tross_office_id()),
        ),
        (
            InstitutionalEntityId::Office(tross_office_id()),
            RelationshipKind::Commands,
            InstitutionalEntityId::Institution(manticorp_id()),
        ),
        (
            InstitutionalEntityId::Office(tross_office_id()),
            RelationshipKind::Commands,
            InstitutionalEntityId::Institution(gallows_id()),
        ),
        (
            InstitutionalEntityId::Institution(manticorp_id()),
            RelationshipKind::Commands,
            InstitutionalEntityId::Institution(mystery_men_id()),
        ),
        (
            InstitutionalEntityId::Institution(gallows_id()),
            RelationshipKind::HeadquarteredAt,
            InstitutionalEntityId::Site(gallowry_site_id()),
        ),
        (
            InstitutionalEntityId::Institution(gallows_id()),
            RelationshipKind::Commands,
            InstitutionalEntityId::Group(we_fairy_men_group_id()),
        ),
        (
            InstitutionalEntityId::Group(we_fairy_men_group_id()),
            RelationshipKind::Represents,
            InstitutionalEntityId::Being(constitutional_chimera_id()),
        ),
        (
            InstitutionalEntityId::Office(bro_white_office_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Group(we_fairy_men_group_id()),
        ),
        (
            InstitutionalEntityId::Office(cinderellaman_office_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Group(we_fairy_men_group_id()),
        ),
        (
            InstitutionalEntityId::Office(the_beauty_office_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Group(we_fairy_men_group_id()),
        ),
        (
            InstitutionalEntityId::Group(bro_white_crew_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Office(bro_white_office_id()),
        ),
        (
            InstitutionalEntityId::Group(cinderellaman_crew_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Office(cinderellaman_office_id()),
        ),
        (
            InstitutionalEntityId::Group(the_beauty_crew_id()),
            RelationshipKind::AnswersTo,
            InstitutionalEntityId::Office(the_beauty_office_id()),
        ),
    ]
}

fn validate_authority_relationships(
    catalog: &InstitutionCatalog,
) -> Result<(), FlyntValidationError> {
    let expected = required_relationships();
    if catalog.relationships.len() != expected.len()
        || expected.iter().any(|(source, kind, target)| {
            catalog
                .relationships
                .iter()
                .filter(|relationship| {
                    &relationship.source == source
                        && relationship.kind == *kind
                        && &relationship.target == target
                })
                .count()
                != 1
        })
    {
        return Err(FlyntValidationError::AuthorityRelationships);
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum FlyntValidationError {
    Constitution(ConstitutionError),
    Catalog(InstitutionValidationError),
    InvalidMembership,
    InstitutionRoster,
    OfficeRoster,
    GroupRoster,
    RoleRoster,
    SiteRoster,
    AuthorityRelationships,
    GallowryIsInstitution,
    GallowrySite,
    ChimeraIsOffice,
    InvalidTrossHolder,
    ProjectionMismatch,
}

pub fn canonical_flynt_institutions() -> FlyntInstitutions {
    let constitution = flynt_constitution::canonical_constitution()
        .expect("canonical Flynt constitution must validate");
    let institutions = FlyntInstitutions {
        catalog: canonical_catalog(),
        memberships: vec![],
        mystery_operatives: vec![],
        manticorp_personnel: vec![],
        constitution,
    };
    institutions
        .validate()
        .expect("canonical Flynt institutional projection must validate");
    institutions
}

fn canonical_catalog() -> InstitutionCatalog {
    let manticorp = manticorp_id();
    let mystery_men = mystery_men_id();
    let gallows = gallows_id();
    let gallowry = gallowry_site_id();
    let we_fairy_men = we_fairy_men_group_id();
    InstitutionCatalog {
        institutions: vec![
            Institution {
                id: manticorp.clone(),
                name: "Manticorp".into(),
                kinds: vec![InstitutionKind::Military],
                house: Some(House::Flynt),
                domains: vec![
                    "territorial defense".into(),
                    "military command".into(),
                    "constitutional protection".into(),
                    "disciplined force".into(),
                    "military training".into(),
                    "lawful deployment".into(),
                ],
                headquarters: None,
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
            },
            Institution {
                id: mystery_men.clone(),
                name: "Mystery Men".into(),
                kinds: vec![
                    InstitutionKind::Investigative,
                    InstitutionKind::Intelligence,
                ],
                house: Some(House::Flynt),
                domains: vec![
                    "investigation".into(),
                    "intelligence".into(),
                    "counterintelligence".into(),
                    "covert operations".into(),
                    "organized crime".into(),
                    "contraband".into(),
                    "espionage".into(),
                    "constitutional security".into(),
                ],
                headquarters: None,
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Black,
            },
            Institution {
                id: gallows.clone(),
                name: "The Gallows".into(),
                kinds: vec![
                    InstitutionKind::Criminal,
                    InstitutionKind::Cultural,
                    InstitutionKind::Fraternal,
                ],
                house: Some(House::Flynt),
                domains: vec![
                    "organized crime".into(),
                    "regional crews".into(),
                    "loyalty".into(),
                    "territory".into(),
                    "favors".into(),
                    "obligation".into(),
                    "cultural identity".into(),
                ],
                headquarters: Some(gallowry.clone()),
                public_visibility: Visibility::Hidden,
                internal_secrecy: SecrecyLevel::Black,
            },
        ],
        offices: vec![
            Office {
                id: tross_office_id(),
                name: "Tross".into(),
                scope: OfficeScope::House,
                institution: None,
                house: Some(House::Flynt),
                singular: true,
                authority: vec!["FlyntSovereignty".into(), "InstitutionalRecognition".into()],
            },
            founding_office(bro_white_office_id(), "Bro White", &gallows),
            founding_office(cinderellaman_office_id(), "Cinderellaman", &gallows),
            founding_office(the_beauty_office_id(), "The Beauty", &gallows),
        ],
        roles: vec![
            Role {
                id: manticorp_member_role_id(),
                name: "Manticorp member".into(),
                institution: manticorp.clone(),
            },
            Role {
                id: mystery_operative_role_id(),
                name: "Mystery Men operative".into(),
                institution: mystery_men.clone(),
            },
            Role {
                id: mystery_man_role_id(),
                name: "The Mystery Man".into(),
                institution: mystery_men.clone(),
            },
            Role {
                id: gallows_member_role_id(),
                name: "Gallows member".into(),
                institution: gallows.clone(),
            },
        ],
        groups: vec![
            Group {
                id: we_fairy_men.clone(),
                name: "We Fairy Men".into(),
                institution: gallows.clone(),
                parent: None,
            },
            founding_crew(
                bro_white_crew_id(),
                "Bro White and the 7 Brothas",
                &gallows,
                &we_fairy_men,
            ),
            founding_crew(
                cinderellaman_crew_id(),
                "Cinderellaman and His Midnight Crew",
                &gallows,
                &we_fairy_men,
            ),
            founding_crew(
                the_beauty_crew_id(),
                "The Beauty and His Beasts",
                &gallows,
                &we_fairy_men,
            ),
        ],
        sites: vec![Site {
            id: gallowry.clone(),
            name: "The Gallowry".into(),
            house: House::Flynt,
            site_kinds: vec![
                SiteKind::Headquarters,
                SiteKind::Gallery,
                SiteKind::SocialClub,
                SiteKind::Workshop,
            ],
            controlled_by: Some(gallows.clone()),
            zones: vec![
                zone_id("zone.flynt.gallowry.meeting-place"),
                zone_id("zone.flynt.gallowry.cultural-center"),
                zone_id("zone.flynt.gallowry.gallery"),
                zone_id("zone.flynt.gallowry.operational-hub"),
            ],
        }],
        office_holders: vec![OfficeHolder {
            office: tross_office_id(),
            being: tross_being_id(),
            active: true,
        }],
        relationships: required_relationships()
            .into_iter()
            .enumerate()
            .map(
                |(index, (source, kind, target))| InstitutionalRelationship {
                    id: relationship_id(&format!("relationship.flynt.canonical.{index}")),
                    source,
                    kind,
                    target,
                    authority: (kind == RelationshipKind::Commands)
                        .then_some(AuthorityLevel::Command),
                    visibility: Visibility::Restricted,
                },
            )
            .collect(),
    }
}

fn founding_office(id: OfficeId, name: &str, gallows: &InstitutionId) -> Office {
    Office {
        id,
        name: name.into(),
        scope: OfficeScope::Institution,
        institution: Some(gallows.clone()),
        house: Some(House::Flynt),
        singular: true,
        authority: vec!["FoundingLeaderLineage".into()],
    }
}

fn founding_crew(
    id: GroupId,
    name: &str,
    gallows: &InstitutionId,
    we_fairy_men: &GroupId,
) -> Group {
    Group {
        id,
        name: name.into(),
        institution: gallows.clone(),
        parent: Some(we_fairy_men.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_projection_validates_and_contains_exact_institutions() {
        let flynt = canonical_flynt_institutions();
        flynt.validate().unwrap();
        assert!(flynt.catalog.institution(&manticorp_id()).is_some());
        assert!(flynt.catalog.institution(&mystery_men_id()).is_some());
        assert!(flynt.catalog.institution(&gallows_id()).is_some());
        assert_eq!(flynt.catalog.institutions.len(), 3);
    }

    #[test]
    fn chimera_is_a_unique_being_projection_not_an_office() {
        let flynt = canonical_flynt_institutions();
        assert!(
            flynt
                .catalog
                .offices
                .iter()
                .all(|office| office.name != "Chimera")
        );
        assert_eq!(
            flynt
                .catalog
                .relationships
                .iter()
                .filter(|relationship| {
                    relationship.source == InstitutionalEntityId::Being(constitutional_chimera_id())
                })
                .count(),
            1
        );
    }

    #[test]
    fn gallowry_is_a_site_owned_by_the_gallows() {
        let flynt = canonical_flynt_institutions();
        assert!(
            flynt
                .catalog
                .institutions
                .iter()
                .all(|institution| institution.name != "The Gallowry")
        );
        let site = flynt
            .catalog
            .sites
            .iter()
            .find(|site| site.id == gallowry_site_id())
            .unwrap();
        assert_eq!(site.controlled_by.as_ref(), Some(&gallows_id()));
    }

    #[test]
    fn tross_directly_commands_public_and_underground_institutions() {
        let flynt = canonical_flynt_institutions();
        assert!(flynt.catalog.relationships.iter().any(|relationship| {
            relationship.source == InstitutionalEntityId::Office(tross_office_id())
                && relationship.target == InstitutionalEntityId::Institution(manticorp_id())
        }));
        assert!(flynt.catalog.relationships.iter().any(|relationship| {
            relationship.source == InstitutionalEntityId::Office(tross_office_id())
                && relationship.target == InstitutionalEntityId::Institution(gallows_id())
        }));
    }

    #[test]
    fn manticorp_form_is_a_distinct_flynt_form_identifier() {
        assert_eq!(manticorp_form_id(), flynt_constitution::FORM_MANTICORP);
        assert_ne!(manticorp_form_id(), manticorp_id().as_str());
    }

    #[test]
    fn an_invented_tross_holder_is_rejected() {
        let mut flynt = canonical_flynt_institutions();
        flynt.catalog.office_holders[0].being = being_id("being.flynt.unspecified-successor");
        assert_eq!(
            flynt.validate(),
            Err(FlyntValidationError::InvalidTrossHolder)
        );
    }
}
