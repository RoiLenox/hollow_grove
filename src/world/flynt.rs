//! Flynt canonical institutional fixtures.  Flynt-specific terms stay here,
//! above the neutral `institution` domain.

pub mod gallowry;

use crate::hollow_grove_contract::House;
use crate::institution::*;
use crate::institution_affiliation::{
    AffiliationState, InstitutionalMembership, InstitutionalWorldState, MembershipRole,
};

fn id<T>(value: &str, make: impl FnOnce(String) -> Result<T, IdError>) -> T {
    make(value.into()).expect("canonical stable id")
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
fn relationship_id(value: &str) -> RelationshipId {
    id(value, RelationshipId::new)
}

pub fn tross_office_id() -> OfficeId {
    office_id("office.flynt.tross")
}
pub fn chimera_office_id() -> OfficeId {
    office_id("office.flynt.chimera")
}
pub fn manticorps_id() -> InstitutionId {
    institution_id("institution.flynt.manticorps")
}
pub fn mystery_men_id() -> InstitutionId {
    institution_id("institution.flynt.mystery-men")
}
pub fn gallowry_id() -> InstitutionId {
    institution_id("institution.flynt.gallowry")
}
pub fn gallowry_site_id() -> SiteId {
    site_id("site.flynt.gallowry")
}
pub fn mystery_man_role_id() -> RoleId {
    role_id("role.flynt.mystery-man")
}
pub fn manticorps_soldier_role_id() -> RoleId {
    role_id("role.flynt.manticorps-soldier")
}
pub fn gallow_role_id() -> RoleId {
    gallowry::gallow_role_id()
}

/// Creates a Rope owned by the Gallowry. The base fixture has no named Ropes.
pub fn gallowry_rope(id_suffix: &str, name: impl Into<String>) -> Group {
    Group {
        id: group_id(&format!("group.flynt.gallowry.rope.{id_suffix}")),
        name: name.into(),
        institution: gallowry_id(),
        parent: None,
    }
}

pub fn gallowry_rope_relationship(rope: &Group) -> InstitutionalRelationship {
    InstitutionalRelationship {
        id: relationship_id(&format!("relationship.flynt.{}-subgroup", rope.id.as_str())),
        source: InstitutionalEntityId::Group(rope.id.clone()),
        kind: RelationshipKind::SubgroupOf,
        target: InstitutionalEntityId::Institution(gallowry_id()),
        authority: None,
        visibility: Visibility::Restricted,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MysterySpecialty {
    Investigation,
    Undercover,
    Counterintelligence,
    Anomaly,
    Forensics,
    MedicalReconstruction,
    WitnessProtection,
    TransformationCrime,
    OrganizedCrime,
    RouteCrime,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MysteryManProfile {
    pub being: InstitutionalBeingId,
    pub codename: String,
    pub public_identity: Option<IdentityId>,
    pub operational_identity: Option<IdentityId>,
    pub cover_identities: Vec<IdentityId>,
    pub specialties: Vec<MysterySpecialty>,
    pub clearance: ClearanceLevel,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManticorpsSpecialty {
    Expedition,
    Amphibious,
    RapidDeployment,
    Siege,
    DisasterResponse,
    MonsterHunting,
    RouteDefense,
    HostileEnvironment,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeploymentStatus {
    Ready,
    Deployed,
    Recovering,
    Reserve,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManticorpsProfile {
    pub being: InstitutionalBeingId,
    pub rank: RankId,
    pub unit: Option<GroupId>,
    pub specialties: Vec<ManticorpsSpecialty>,
    pub deployment_status: DeploymentStatus,
}

#[derive(Debug, Clone)]
pub struct FlyntInstitutions {
    pub catalog: InstitutionCatalog,
    /// Active memberships live in the detailed neutral affiliation model.
    /// Catalogs define institutions, roles, sites, and relationships only.
    pub memberships: Vec<InstitutionalMembership>,
    pub mystery_men: Vec<MysteryManProfile>,
    pub manticorps: Vec<ManticorpsProfile>,
}

impl FlyntInstitutions {
    pub fn is_gallow(&self, being: &InstitutionalBeingId) -> bool {
        self.memberships.iter().any(|membership| {
            &membership.being == being
                && membership.institution == gallowry_id()
                && membership.role_id.as_ref() == Some(&gallow_role_id())
                && membership.role == MembershipRole::FullMember
                && matches!(
                    membership.affiliation_state,
                    AffiliationState::Initiated | AffiliationState::Senior
                )
        })
    }
    pub fn mystery_man_profile(&self, being: &InstitutionalBeingId) -> Option<&MysteryManProfile> {
        self.mystery_men.iter().find(|entry| &entry.being == being)
    }
    pub fn manticorps_profile(&self, being: &InstitutionalBeingId) -> Option<&ManticorpsProfile> {
        self.manticorps.iter().find(|entry| &entry.being == being)
    }
    pub fn validate(&self) -> Result<(), FlyntValidationError> {
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
        if manticorps_id() == mystery_men_id() {
            return Err(FlyntValidationError::CorruptInstitutionIdentity);
        }
        if self.catalog.institution(&gallowry_id()).is_none()
            || !self.catalog.sites.iter().any(|site| {
                site.id == gallowry_site_id() && site.controlled_by.as_ref() == Some(&gallowry_id())
            })
        {
            return Err(FlyntValidationError::GallowrySiteLink);
        }
        if self
            .catalog
            .offices
            .iter()
            .any(|office| office.id == tross_office_id() && office.institution.is_some())
            || self
                .catalog
                .offices
                .iter()
                .any(|office| office.id == chimera_office_id() && office.institution.is_some())
        {
            return Err(FlyntValidationError::OfficeIsInstitution);
        }
        for group in &self.catalog.groups {
            if group.id.as_str().starts_with("group.flynt.gallowry.rope.")
                && group.institution != gallowry_id()
            {
                return Err(FlyntValidationError::RopeOutsideGallowry);
            }
            if group.id.as_str().starts_with("group.flynt.gallowry.rope.")
                && !self.catalog.relationships.iter().any(|relationship| {
                    relationship.source == InstitutionalEntityId::Group(group.id.clone())
                        && relationship.kind == RelationshipKind::SubgroupOf
                        && relationship.target == InstitutionalEntityId::Institution(gallowry_id())
                })
            {
                return Err(FlyntValidationError::RopeOutsideGallowry);
            }
        }
        if self.catalog.relationships.iter().any(|relationship| {
            relationship.source == InstitutionalEntityId::Office(tross_office_id())
                && relationship.kind == RelationshipKind::Commands
                && relationship.target == InstitutionalEntityId::Institution(gallowry_id())
        }) {
            return Err(FlyntValidationError::TrossCommandsGallowry);
        }
        for holder in self
            .catalog
            .office_holders
            .iter()
            .filter(|holder| holder.active && holder.office == tross_office_id())
        {
            if self.memberships.iter().any(|membership| {
                membership.being == holder.being
                    && membership.role_id.as_ref() == Some(&mystery_man_role_id())
            }) {
                return Err(FlyntValidationError::TrossIsMysteryMan);
            }
        }
        let mut labels = self
            .catalog
            .institutions
            .iter()
            .map(|entry| entry.name.as_str())
            .chain(self.catalog.offices.iter().map(|entry| entry.name.as_str()))
            .chain(self.catalog.roles.iter().map(|entry| entry.name.as_str()))
            .chain(self.catalog.groups.iter().map(|entry| entry.name.as_str()))
            .chain(self.catalog.sites.iter().map(|entry| entry.name.as_str()));
        if labels.any(|label| label.to_ascii_lowercase().contains("persephone")) {
            return Err(FlyntValidationError::ForbiddenFixtureLore);
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlyntValidationError {
    Catalog(InstitutionValidationError),
    CorruptInstitutionIdentity,
    GallowrySiteLink,
    OfficeIsInstitution,
    RopeOutsideGallowry,
    TrossCommandsGallowry,
    TrossIsMysteryMan,
    InvalidMembership,
    ForbiddenFixtureLore,
}

pub fn canonical_flynt_institutions() -> FlyntInstitutions {
    let manticorps = manticorps_id();
    let mystery = mystery_men_id();
    let gallowry = gallowry_id();
    let site = gallowry_site_id();
    let catalog = InstitutionCatalog {
        institutions: vec![
            Institution {
                id: manticorps.clone(),
                name: "Manticorps".into(),
                kinds: vec![InstitutionKind::Military],
                house: Some(House::Flynt),
                domains: vec![
                    "military".into(),
                    "expedition".into(),
                    "amphibious operations".into(),
                    "rapid deployment".into(),
                    "hostile-environment combat".into(),
                    "disaster response".into(),
                    "external defense".into(),
                    "route seizure".into(),
                    "monster hunting".into(),
                ],
                headquarters: None,
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
            },
            Institution {
                id: mystery.clone(),
                name: "Mystery Men".into(),
                kinds: vec![
                    InstitutionKind::Investigative,
                    InstitutionKind::Intelligence,
                ],
                house: Some(House::Flynt),
                domains: vec![
                    "federal investigation".into(),
                    "undercover operations".into(),
                    "counterintelligence".into(),
                    "extraordinary cases".into(),
                ],
                headquarters: None,
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
            },
            Institution {
                id: gallowry.clone(),
                name: "The Gallowry".into(),
                kinds: vec![
                    InstitutionKind::Criminal,
                    InstitutionKind::Cultural,
                    InstitutionKind::Fraternal,
                ],
                house: Some(House::Flynt),
                domains: vec![
                    "ritualized criminal underworld".into(),
                    "patronage".into(),
                    "negotiation".into(),
                ],
                headquarters: Some(site.clone()),
                public_visibility: Visibility::Known,
                internal_secrecy: SecrecyLevel::Initiated,
            },
        ],
        offices: vec![
            Office {
                id: tross_office_id(),
                name: "The Tross".into(),
                scope: OfficeScope::House,
                institution: None,
                house: Some(House::Flynt),
                singular: true,
                authority: vec![
                    "FlyntSovereignty".into(),
                    "InstitutionalRecognition".into(),
                    "PublicLegitimacy".into(),
                    "Appointment".into(),
                    "StrategicCommand".into(),
                ],
            },
            Office {
                id: chimera_office_id(),
                name: "The Chimera".into(),
                scope: OfficeScope::CrossInstitution,
                institution: None,
                house: Some(House::Flynt),
                singular: true,
                authority: vec![
                    "CrossInstitutionCoordination".into(),
                    "EmergencyCommand".into(),
                    "OperationalOverride".into(),
                    "ChampionAuthority".into(),
                    "DirectTrossMandate".into(),
                ],
            },
        ],
        roles: vec![
            Role {
                id: manticorps_soldier_role_id(),
                name: "Manticorps soldier".into(),
                institution: manticorps.clone(),
            },
            Role {
                id: mystery_man_role_id(),
                name: "Mystery Man".into(),
                institution: mystery.clone(),
            },
            Role {
                id: gallowry::noose_role_id(),
                name: "Noose".into(),
                institution: gallowry.clone(),
            },
            Role {
                id: gallowry::gallow_role_id(),
                name: "Gallow".into(),
                institution: gallowry.clone(),
            },
            Role {
                id: gallowry::sponsor_role_id(),
                name: "Gallowry sponsor".into(),
                institution: gallowry.clone(),
            },
            Role {
                id: gallowry::curator_role_id(),
                name: "Gallowry curator".into(),
                institution: gallowry.clone(),
            },
            Role {
                id: gallowry::broker_role_id(),
                name: "Gallowry broker".into(),
                institution: gallowry.clone(),
            },
            Role {
                id: gallowry::enforcer_role_id(),
                name: "Gallowry enforcer".into(),
                institution: gallowry.clone(),
            },
        ],
        groups: vec![],
        sites: vec![Site {
            id: site.clone(),
            name: "The Gallowry".into(),
            house: House::Flynt,
            site_kinds: vec![
                SiteKind::Gallery,
                SiteKind::ExhibitionStudio,
                SiteKind::PerformanceVenue,
                SiteKind::AuctionHouse,
                SiteKind::SocialClub,
                SiteKind::Headquarters,
                SiteKind::PrivateCourt,
                SiteKind::Archive,
                SiteKind::Workshop,
            ],
            controlled_by: Some(gallowry.clone()),
            zones: vec![
                zone_id("zone.flynt.gallowry.exhibition-floor"),
                zone_id("zone.flynt.gallowry.salon"),
                zone_id("zone.flynt.gallowry.hanging-rooms"),
                zone_id("zone.flynt.gallowry.rope-archive"),
                zone_id("zone.flynt.gallowry.black-studio"),
                zone_id("zone.flynt.gallowry.scaffold"),
            ],
        }],
        office_holders: vec![],
        relationships: vec![
            rel(
                "relationship.flynt.tross-commands-manticorps",
                InstitutionalEntityId::Office(tross_office_id()),
                RelationshipKind::Commands,
                InstitutionalEntityId::Institution(manticorps.clone()),
            ),
            rel(
                "relationship.flynt.tross-commands-mystery-men",
                InstitutionalEntityId::Office(tross_office_id()),
                RelationshipKind::Commands,
                InstitutionalEntityId::Institution(mystery.clone()),
            ),
            rel(
                "relationship.flynt.tross-recognizes-chimera",
                InstitutionalEntityId::Office(tross_office_id()),
                RelationshipKind::Recognizes,
                InstitutionalEntityId::Office(chimera_office_id()),
            ),
            rel(
                "relationship.flynt.chimera-answers-to-tross",
                InstitutionalEntityId::Office(chimera_office_id()),
                RelationshipKind::AnswersTo,
                InstitutionalEntityId::Office(tross_office_id()),
            ),
            rel(
                "relationship.flynt.chimera-coordinates-manticorps",
                InstitutionalEntityId::Office(chimera_office_id()),
                RelationshipKind::Coordinates,
                InstitutionalEntityId::Institution(manticorps.clone()),
            ),
            rel(
                "relationship.flynt.chimera-coordinates-mystery-men",
                InstitutionalEntityId::Office(chimera_office_id()),
                RelationshipKind::Coordinates,
                InstitutionalEntityId::Institution(mystery.clone()),
            ),
            rel(
                "relationship.flynt.chimera-bargains-gallowry",
                InstitutionalEntityId::Office(chimera_office_id()),
                RelationshipKind::BargainsWith,
                InstitutionalEntityId::Institution(gallowry.clone()),
            ),
            rel(
                "relationship.flynt.manticorps-cooperates-mystery-men",
                InstitutionalEntityId::Institution(manticorps.clone()),
                RelationshipKind::CooperatesWith,
                InstitutionalEntityId::Institution(mystery.clone()),
            ),
            rel(
                "relationship.flynt.manticorps-rivals-gallowry",
                InstitutionalEntityId::Institution(manticorps.clone()),
                RelationshipKind::Rivals,
                InstitutionalEntityId::Institution(gallowry.clone()),
            ),
            rel(
                "relationship.flynt.mystery-men-investigates-gallowry",
                InstitutionalEntityId::Institution(mystery.clone()),
                RelationshipKind::Investigates,
                InstitutionalEntityId::Institution(gallowry.clone()),
            ),
            rel(
                "relationship.flynt.mystery-men-investigates-manticorps",
                InstitutionalEntityId::Institution(mystery.clone()),
                RelationshipKind::Investigates,
                InstitutionalEntityId::Institution(manticorps.clone()),
            ),
            rel(
                "relationship.flynt.gallowry-operates-outside",
                InstitutionalEntityId::Institution(gallowry.clone()),
                RelationshipKind::OperatesOutside,
                InstitutionalEntityId::Office(tross_office_id()),
            ),
            rel(
                "relationship.flynt.gallowry-bargains-tross",
                InstitutionalEntityId::Institution(gallowry.clone()),
                RelationshipKind::BargainsWith,
                InstitutionalEntityId::Office(tross_office_id()),
            ),
            rel(
                "relationship.flynt.gallowry-headquartered-at",
                InstitutionalEntityId::Institution(gallowry),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(site),
            ),
        ],
    };
    FlyntInstitutions {
        catalog,
        memberships: vec![],
        mystery_men: vec![],
        manticorps: vec![],
    }
}
fn rel(
    value: &str,
    source: InstitutionalEntityId,
    kind: RelationshipKind,
    target: InstitutionalEntityId,
) -> InstitutionalRelationship {
    InstitutionalRelationship {
        id: relationship_id(value),
        source,
        kind,
        target,
        authority: None,
        visibility: Visibility::Known,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::institution_affiliation::RecognitionLevel;
    #[test]
    fn canonical_fixture_locks_flynt_institutions_and_relationships() {
        let flynt = canonical_flynt_institutions();
        flynt.validate().unwrap();
        assert!(flynt.catalog.institution(&manticorps_id()).is_some());
        assert!(flynt.catalog.institution(&mystery_men_id()).is_some());
        assert!(flynt.catalog.institution(&gallowry_id()).is_some());
        assert!(
            flynt
                .catalog
                .offices
                .iter()
                .any(|office| office.id == tross_office_id())
        );
        assert!(
            flynt
                .catalog
                .can_coordinate(&chimera_office_id(), &manticorps_id())
        );
        assert!(
            flynt
                .catalog
                .relationships_between(
                    &InstitutionalEntityId::Institution(mystery_men_id()),
                    &InstitutionalEntityId::Institution(gallowry_id())
                )
                .iter()
                .any(|relationship| relationship.kind == RelationshipKind::Investigates)
        );
    }
    #[test]
    fn contradiction_tross_cannot_command_gallowry() {
        let mut flynt = canonical_flynt_institutions();
        flynt.catalog.relationships.push(rel(
            "relationship.flynt.invalid-command",
            InstitutionalEntityId::Office(tross_office_id()),
            RelationshipKind::Commands,
            InstitutionalEntityId::Institution(gallowry_id()),
        ));
        assert_eq!(
            flynt.validate(),
            Err(FlyntValidationError::TrossCommandsGallowry)
        );
    }
    #[test]
    fn contradiction_tross_cannot_be_a_mystery_man() {
        let mut flynt = canonical_flynt_institutions();
        let being = id("being.flynt.tross-test", InstitutionalBeingId::new);
        flynt.catalog.office_holders.push(OfficeHolder {
            office: tross_office_id(),
            being: being.clone(),
            active: true,
        });
        flynt.memberships.push(InstitutionalMembership {
            id: id("membership.flynt.invalid-tross", MembershipId::new),
            being,
            institution: mystery_men_id(),
            role_id: Some(mystery_man_role_id()),
            role: MembershipRole::FullMember,
            subgroup: None,
            affiliation_state: AffiliationState::Initiated,
            lineage: crate::institution_affiliation::LineageStatus::None,
            sponsor: None,
            joined_at: Some(0),
            initiated_at: Some(0),
            ended_at: None,
            public_visibility: Visibility::Known,
            internal_recognition: RecognitionLevel::Internal,
        });
        assert_eq!(
            flynt.validate(),
            Err(FlyntValidationError::TrossIsMysteryMan)
        );
    }

    #[test]
    fn contradiction_persephone_is_not_flynt_fixture_data() {
        let mut flynt = canonical_flynt_institutions();
        flynt.catalog.sites[0].name = "Persephone Annex".into();
        assert_eq!(
            flynt.validate(),
            Err(FlyntValidationError::ForbiddenFixtureLore)
        );
    }

    #[test]
    fn rope_is_queryable_as_a_gallowry_subgroup() {
        let mut flynt = canonical_flynt_institutions();
        let rope = gallowry_rope("fixture", "Fixture Rope");
        flynt
            .catalog
            .relationships
            .push(gallowry_rope_relationship(&rope));
        flynt.catalog.groups.push(rope.clone());
        flynt.validate().unwrap();
        assert!(
            flynt
                .catalog
                .relationships_from(&InstitutionalEntityId::Group(rope.id))
                .iter()
                .any(|relationship| relationship.kind == RelationshipKind::SubgroupOf)
        );
    }
}
