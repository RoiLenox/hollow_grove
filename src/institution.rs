//! Neutral institutional records.  This module describes context and authority;
//! it deliberately does not select actions or present dialogue.

use std::fmt;

use crate::hollow_grove_contract::House;

macro_rules! stable_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || byte == b'.'
                            || byte == b'-'
                    })
                {
                    return Err(IdError::Invalid(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    Invalid(String),
}

impl fmt::Display for IdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(value) => write!(formatter, "invalid stable identifier: {value}"),
        }
    }
}

impl std::error::Error for IdError {}

stable_id!(InstitutionId);
stable_id!(OfficeId);
stable_id!(MembershipId);
stable_id!(RoleId);
stable_id!(GroupId);
stable_id!(SiteId);
stable_id!(ZoneId);
stable_id!(RelationshipId);
stable_id!(InstitutionalBeingId);
stable_id!(IdentityId);
stable_id!(RankId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstitutionKind {
    Government,
    Military,
    Investigative,
    Intelligence,
    Criminal,
    Cultural,
    Medical,
    Religious,
    Commercial,
    Fraternal,
    Hybrid,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OfficeScope {
    Institution,
    House,
    Regional,
    CrossInstitution,
    World,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationshipKind {
    Commands,
    AnswersTo,
    Coordinates,
    Recognizes,
    Investigates,
    RecruitsFrom,
    Infiltrates,
    Protects,
    Supplies,
    Rivals,
    BargainsWith,
    CooperatesWith,
    OperatesOutside,
    Overrides,
    OwesDebtTo,
    GrantsAccessTo,
    MemberOf,
    SubgroupOf,
    HeadquarteredAt,
    Represents,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    Public,
    Known,
    Restricted,
    Classified,
    Hidden,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecrecyLevel {
    Open,
    Internal,
    Initiated,
    Compartmentalized,
    Black,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthorityLevel {
    Advisory,
    Coordinating,
    Emergency,
    Command,
    Sovereign,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SiteKind {
    Gallery,
    ExhibitionStudio,
    PerformanceVenue,
    AuctionHouse,
    SocialClub,
    Headquarters,
    PrivateCourt,
    Archive,
    Workshop,
    MedicalFacility,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearanceLevel {
    Basic,
    Restricted,
    Classified,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Institution {
    pub id: InstitutionId,
    pub name: String,
    pub kinds: Vec<InstitutionKind>,
    pub house: Option<House>,
    pub domains: Vec<String>,
    pub headquarters: Option<SiteId>,
    pub public_visibility: Visibility,
    pub internal_secrecy: SecrecyLevel,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Office {
    pub id: OfficeId,
    pub name: String,
    pub scope: OfficeScope,
    pub institution: Option<InstitutionId>,
    pub house: Option<House>,
    pub singular: bool,
    pub authority: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub institution: InstitutionId,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub institution: InstitutionId,
    pub parent: Option<GroupId>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub house: House,
    pub site_kinds: Vec<SiteKind>,
    pub controlled_by: Option<InstitutionId>,
    pub zones: Vec<ZoneId>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfficeHolder {
    pub office: OfficeId,
    pub being: InstitutionalBeingId,
    pub active: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstitutionalEntityId {
    Institution(InstitutionId),
    Office(OfficeId),
    Group(GroupId),
    Site(SiteId),
    Being(InstitutionalBeingId),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalRelationship {
    pub id: RelationshipId,
    pub source: InstitutionalEntityId,
    pub kind: RelationshipKind,
    pub target: InstitutionalEntityId,
    pub authority: Option<AuthorityLevel>,
    pub visibility: Visibility,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessRequirement {
    Public,
    InstitutionMembership(InstitutionId),
    Role(RoleId),
    Office(OfficeId),
    MinimumStanding(String),
    Clearance(ClearanceLevel),
    Relationship(RelationshipKind),
    ExplicitGrant,
}
/// How a set of neutral access requirements is interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessRequirementMatch {
    Any,
    All,
}
/// Data-only access policy. World domains supply the requirements; a traversal
/// or scene layer may evaluate them against its available identity and grant
/// data without relying on display names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPolicy {
    pub matching: AccessRequirementMatch,
    pub requirements: Vec<AccessRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstitutionValidationError {
    DuplicateInstitution(InstitutionId),
    DuplicateOffice(OfficeId),
    DuplicateSite(SiteId),
    MissingInstitution(InstitutionId),
    MissingSite(SiteId),
    MissingRole(RoleId),
    MissingGroup(GroupId),
    InvalidMembership(MembershipId),
    SingularOfficeHasMultipleActiveHolders(OfficeId),
}

#[derive(Debug, Clone, Default)]
pub struct InstitutionCatalog {
    pub institutions: Vec<Institution>,
    pub offices: Vec<Office>,
    pub roles: Vec<Role>,
    pub groups: Vec<Group>,
    pub sites: Vec<Site>,
    pub office_holders: Vec<OfficeHolder>,
    pub relationships: Vec<InstitutionalRelationship>,
}

impl InstitutionCatalog {
    pub fn institution(&self, id: &InstitutionId) -> Option<&Institution> {
        self.institutions.iter().find(|entry| &entry.id == id)
    }
    pub fn offices_held_by(&self, being: &InstitutionalBeingId) -> Vec<&Office> {
        self.office_holders
            .iter()
            .filter(|holder| holder.active && &holder.being == being)
            .filter_map(|holder| {
                self.offices
                    .iter()
                    .find(|office| office.id == holder.office)
            })
            .collect()
    }
    pub fn relationships_from(
        &self,
        entity: &InstitutionalEntityId,
    ) -> Vec<&InstitutionalRelationship> {
        self.relationships
            .iter()
            .filter(|entry| &entry.source == entity)
            .collect()
    }
    pub fn relationships_between(
        &self,
        a: &InstitutionalEntityId,
        b: &InstitutionalEntityId,
    ) -> Vec<&InstitutionalRelationship> {
        self.relationships
            .iter()
            .filter(|entry| {
                (&entry.source == a && &entry.target == b)
                    || (&entry.source == b && &entry.target == a)
            })
            .collect()
    }
    pub fn can_coordinate(&self, office: &OfficeId, institution: &InstitutionId) -> bool {
        self.relationships.iter().any(|entry| {
            entry.source == InstitutionalEntityId::Office(office.clone())
                && entry.target == InstitutionalEntityId::Institution(institution.clone())
                && matches!(
                    entry.kind,
                    RelationshipKind::Coordinates | RelationshipKind::Overrides
                )
        })
    }
    pub fn validate(&self) -> Result<(), InstitutionValidationError> {
        validate_unique(
            self.institutions.iter().map(|entry| &entry.id),
            InstitutionValidationError::DuplicateInstitution,
        )?;
        validate_unique(
            self.offices.iter().map(|entry| &entry.id),
            InstitutionValidationError::DuplicateOffice,
        )?;
        validate_unique(
            self.sites.iter().map(|entry| &entry.id),
            InstitutionValidationError::DuplicateSite,
        )?;
        for institution in &self.institutions {
            if let Some(site) = &institution.headquarters
                && !self.sites.iter().any(|entry| &entry.id == site)
            {
                return Err(InstitutionValidationError::MissingSite(site.clone()));
            }
        }
        for role in &self.roles {
            if self.institution(&role.institution).is_none() {
                return Err(InstitutionValidationError::MissingInstitution(
                    role.institution.clone(),
                ));
            }
        }
        for group in &self.groups {
            if self.institution(&group.institution).is_none() {
                return Err(InstitutionValidationError::MissingInstitution(
                    group.institution.clone(),
                ));
            }
        }
        for office in self.offices.iter().filter(|office| office.singular) {
            if self
                .office_holders
                .iter()
                .filter(|holder| holder.active && holder.office == office.id)
                .count()
                > 1
            {
                return Err(
                    InstitutionValidationError::SingularOfficeHasMultipleActiveHolders(
                        office.id.clone(),
                    ),
                );
            }
        }
        Ok(())
    }
}

fn validate_unique<'a, T>(
    ids: impl Iterator<Item = &'a T>,
    error: impl Fn(T) -> InstitutionValidationError,
) -> Result<(), InstitutionValidationError>
where
    T: Eq + std::hash::Hash + Clone + 'a,
{
    let mut seen = std::collections::HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(error(id.clone()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable_ids_reject_display_names() {
        assert!(InstitutionId::new("institution.flynt.gallowry").is_ok());
        assert!(InstitutionId::new("The Gallowry").is_err());
    }
    #[test]
    fn singular_office_rejects_two_active_holders() {
        let office = Office {
            id: OfficeId::new("office.test.head").unwrap(),
            name: "Head".into(),
            scope: OfficeScope::House,
            institution: None,
            house: Some(House::Flynt),
            singular: true,
            authority: vec![],
        };
        let mut catalog = InstitutionCatalog {
            offices: vec![office.clone()],
            ..Default::default()
        };
        catalog.office_holders = vec![
            OfficeHolder {
                office: office.id.clone(),
                being: InstitutionalBeingId::new("being.one").unwrap(),
                active: true,
            },
            OfficeHolder {
                office: office.id.clone(),
                being: InstitutionalBeingId::new("being.two").unwrap(),
                active: true,
            },
        ];
        assert!(matches!(
            catalog.validate(),
            Err(InstitutionValidationError::SingularOfficeHasMultipleActiveHolders(_))
        ));
    }
}
