//! Canonical institutional fixtures for the three non-Flynt Houses.
//! They reuse the neutral institutional domain; no new decision logic lives here.

use crate::hollow_grove_contract::House;
use crate::institution::*;

fn id<T>(value: &str, build: impl FnOnce(String) -> Result<T, IdError>) -> T {
    build(value.into()).expect("canonical stable ID")
}
fn institution(value: &str) -> InstitutionId {
    id(value, InstitutionId::new)
}
fn office(value: &str) -> OfficeId {
    id(value, OfficeId::new)
}
fn role(value: &str) -> RoleId {
    id(value, RoleId::new)
}
fn group(value: &str) -> GroupId {
    id(value, GroupId::new)
}
fn site(value: &str) -> SiteId {
    id(value, SiteId::new)
}
fn zone(value: &str) -> ZoneId {
    id(value, ZoneId::new)
}
fn relationship(value: &str) -> RelationshipId {
    id(value, RelationshipId::new)
}
fn rel(
    id_value: &str,
    source: InstitutionalEntityId,
    kind: RelationshipKind,
    target: InstitutionalEntityId,
) -> InstitutionalRelationship {
    InstitutionalRelationship {
        id: relationship(id_value),
        source,
        kind,
        target,
        authority: None,
        visibility: Visibility::Known,
    }
}

pub fn stonebend_constitution_id() -> InstitutionId {
    institution("institution.stonebend.constitution")
}
pub fn sandmen_id() -> InstitutionId {
    institution("institution.sandmanor.sandmen")
}
pub fn glaushouse_medical_civilization_id() -> InstitutionId {
    institution("institution.glaushouse.medical-civilization")
}

pub fn canonical_house_institutions() -> InstitutionCatalog {
    let stonebend = stonebend_constitution_id();
    let sandmen = sandmen_id();
    let glaushouse = glaushouse_medical_civilization_id();
    let stonebender = site("site.stonebend.stonebender");
    let aura_beach = site("site.sandmanor.aura-beach");
    let aura_fields = site("site.sandmanor.aura-fields");
    let medical_district = site("site.glaushouse.central-medical-district");
    InstitutionCatalog {
        institutions: vec![
            Institution {
                id: stonebend.clone(),
                name: "Stonebend constitutional body".into(),
                kinds: vec![
                    InstitutionKind::Government,
                    InstitutionKind::Commercial,
                    InstitutionKind::Hybrid,
                ],
                house: Some(House::Stonebend),
                domains: vec![
                    "title".into(),
                    "labor".into(),
                    "craft".into(),
                    "construction".into(),
                    "hollowing".into(),
                ],
                headquarters: Some(stonebender.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Internal,
            },
            Institution {
                id: sandmen.clone(),
                name: "Sandmen".into(),
                kinds: vec![
                    InstitutionKind::Government,
                    InstitutionKind::Cultural,
                    InstitutionKind::Hybrid,
                ],
                house: Some(House::Sandmanor),
                domains: vec![
                    "witnessed improvement".into(),
                    "measurement".into(),
                    "design".into(),
                    "allocation".into(),
                    "public expression".into(),
                ],
                headquarters: None,
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Open,
            },
            Institution {
                id: glaushouse.clone(),
                name: "Glaüshouse medical civilization".into(),
                kinds: vec![
                    InstitutionKind::Medical,
                    InstitutionKind::Cultural,
                    InstitutionKind::Hybrid,
                ],
                house: Some(House::Glaushouse),
                domains: vec![
                    "diagnosis".into(),
                    "surgery".into(),
                    "rehabilitation".into(),
                    "synthesis".into(),
                    "medical education".into(),
                ],
                headquarters: Some(medical_district.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
            },
        ],
        offices: vec![
            Office {
                id: office("office.stonebend.hypergiant"),
                name: "Hypergiant".into(),
                scope: OfficeScope::House,
                institution: Some(stonebend.clone()),
                house: Some(House::Stonebend),
                singular: true,
                authority: vec![
                    "PublicTitle".into(),
                    "ConstitutionalIdentity".into(),
                    "OutwardNegotiation".into(),
                ],
            },
            Office {
                id: office("office.sandmanor.sandman"),
                name: "The Sandman".into(),
                scope: OfficeScope::House,
                institution: Some(sandmen.clone()),
                house: Some(House::Sandmanor),
                singular: true,
                authority: vec![
                    "WitnessedImprovement".into(),
                    "CrowdRecognition".into(),
                    "ConfigurationRule".into(),
                ],
            },
            Office {
                id: office("office.glaushouse.prima-donna"),
                name: "Prima Donna".into(),
                scope: OfficeScope::House,
                institution: Some(glaushouse.clone()),
                house: Some(House::Glaushouse),
                singular: true,
                authority: vec![
                    "PublicClearance".into(),
                    "ReleaseAuthority".into(),
                    "FinalJudgmentAnswerability".into(),
                ],
            },
        ],
        roles: vec![
            Role {
                id: role("role.stonebend.proletariat"),
                name: "Proletariat".into(),
                institution: stonebend.clone(),
            },
            Role {
                id: role("role.stonebend.freemason"),
                name: "Freemason".into(),
                institution: stonebend.clone(),
            },
            Role {
                id: role("role.stonebend.gerald"),
                name: "Gerald".into(),
                institution: stonebend.clone(),
            },
            Role {
                id: role("role.sandmanor.minorian"),
                name: "Minorian".into(),
                institution: sandmen.clone(),
            },
            Role {
                id: role("role.sandmanor.minoan"),
                name: "Minoan".into(),
                institution: sandmen.clone(),
            },
            Role {
                id: role("role.glaushouse.persephone"),
                name: "Persephone".into(),
                institution: glaushouse.clone(),
            },
            Role {
                id: role("role.glaushouse.nightingale"),
                name: "Nightingale".into(),
                institution: glaushouse.clone(),
            },
            Role {
                id: role("role.glaushouse.recovery-staff"),
                name: "Recovery staff".into(),
                institution: glaushouse.clone(),
            },
        ],
        groups: vec![
            Group {
                id: group("group.stonebend.proletariat"),
                name: "Proletariat".into(),
                institution: stonebend.clone(),
                parent: None,
            },
            Group {
                id: group("group.stonebend.freemason"),
                name: "Freemason".into(),
                institution: stonebend.clone(),
                parent: None,
            },
            Group {
                id: group("group.sandmanor.minorians"),
                name: "Minorians".into(),
                institution: sandmen.clone(),
                parent: None,
            },
            Group {
                id: group("group.sandmanor.minoans"),
                name: "Minoans".into(),
                institution: sandmen.clone(),
                parent: None,
            },
            Group {
                id: group("group.glaushouse.recovery-floor"),
                name: "Recovery floor".into(),
                institution: glaushouse.clone(),
                parent: None,
            },
        ],
        sites: vec![
            Site {
                id: stonebender.clone(),
                name: "The Stonebender".into(),
                house: House::Stonebend,
                site_kinds: vec![SiteKind::Headquarters, SiteKind::PerformanceVenue],
                controlled_by: Some(stonebend.clone()),
                zones: vec![
                    zone("zone.stonebend.stonebender.standing-rings"),
                    zone("zone.stonebend.stonebender.ritual-lanes"),
                    zone("zone.stonebend.stonebender.burden-relay"),
                ],
            },
            Site {
                id: aura_beach.clone(),
                name: "Aura Beach".into(),
                house: House::Sandmanor,
                site_kinds: vec![SiteKind::SocialClub, SiteKind::PerformanceVenue],
                controlled_by: Some(sandmen.clone()),
                zones: vec![zone("zone.sandmanor.aura-beach.court-strand")],
            },
            Site {
                id: aura_fields.clone(),
                name: "Aura Fields".into(),
                house: House::Sandmanor,
                site_kinds: vec![SiteKind::Workshop, SiteKind::Archive],
                controlled_by: Some(sandmen.clone()),
                zones: vec![zone("zone.sandmanor.aura-fields.proof-plain")],
            },
            Site {
                id: medical_district.clone(),
                name: "Central Glaüshouse medical district".into(),
                house: House::Glaushouse,
                site_kinds: vec![
                    SiteKind::Headquarters,
                    SiteKind::MedicalFacility,
                    SiteKind::Archive,
                    SiteKind::Workshop,
                    SiteKind::SocialClub,
                ],
                controlled_by: Some(glaushouse.clone()),
                zones: vec![
                    zone("zone.glaushouse.medical-district.diagnostic-halls"),
                    zone("zone.glaushouse.medical-district.surgical-theaters"),
                    zone("zone.glaushouse.medical-district.rehabilitation-pools"),
                    zone("zone.glaushouse.medical-district.recovery-chambers"),
                ],
            },
        ],
        office_holders: vec![],
        relationships: vec![
            rel(
                "relationship.stonebend.proletariat-subgroup",
                InstitutionalEntityId::Group(group("group.stonebend.proletariat")),
                RelationshipKind::SubgroupOf,
                InstitutionalEntityId::Institution(stonebend.clone()),
            ),
            rel(
                "relationship.stonebend.freemason-subgroup",
                InstitutionalEntityId::Group(group("group.stonebend.freemason")),
                RelationshipKind::SubgroupOf,
                InstitutionalEntityId::Institution(stonebend.clone()),
            ),
            rel(
                "relationship.sandmanor.minorians-subgroup",
                InstitutionalEntityId::Group(group("group.sandmanor.minorians")),
                RelationshipKind::SubgroupOf,
                InstitutionalEntityId::Institution(sandmen.clone()),
            ),
            rel(
                "relationship.sandmanor.minoans-subgroup",
                InstitutionalEntityId::Group(group("group.sandmanor.minoans")),
                RelationshipKind::SubgroupOf,
                InstitutionalEntityId::Institution(sandmen.clone()),
            ),
            rel(
                "relationship.glaushouse.recovery-floor-subgroup",
                InstitutionalEntityId::Group(group("group.glaushouse.recovery-floor")),
                RelationshipKind::SubgroupOf,
                InstitutionalEntityId::Institution(glaushouse.clone()),
            ),
            rel(
                "relationship.stonebend.headquartered-at-stonebender",
                InstitutionalEntityId::Institution(stonebend.clone()),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(stonebender),
            ),
            rel(
                "relationship.sandmen-operate-aura-beach",
                InstitutionalEntityId::Institution(sandmen.clone()),
                RelationshipKind::Represents,
                InstitutionalEntityId::Site(aura_beach),
            ),
            rel(
                "relationship.sandmen-operate-aura-fields",
                InstitutionalEntityId::Institution(sandmen.clone()),
                RelationshipKind::Represents,
                InstitutionalEntityId::Site(aura_fields),
            ),
            rel(
                "relationship.glaushouse.headquartered-at-medical-district",
                InstitutionalEntityId::Institution(glaushouse.clone()),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(medical_district),
            ),
            rel(
                "relationship.stonebend-geralds-supply-glaushouse",
                InstitutionalEntityId::Institution(stonebend),
                RelationshipKind::Supplies,
                InstitutionalEntityId::Institution(glaushouse.clone()),
            ),
            rel(
                "relationship.sandmanor-cooperates-glaushouse",
                InstitutionalEntityId::Institution(sandmen),
                RelationshipKind::CooperatesWith,
                InstitutionalEntityId::Institution(glaushouse),
            ),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn house_fixtures_lock_canonical_offices_people_and_sites() {
        let houses = canonical_house_institutions();
        houses.validate().unwrap();
        assert!(houses.institution(&stonebend_constitution_id()).is_some());
        assert!(houses.institution(&sandmen_id()).is_some());
        assert!(
            houses
                .institution(&glaushouse_medical_civilization_id())
                .is_some()
        );
        assert!(
            houses
                .offices
                .iter()
                .any(|entry| entry.id == office("office.stonebend.hypergiant"))
        );
        assert!(
            houses
                .offices
                .iter()
                .any(|entry| entry.id == office("office.sandmanor.sandman"))
        );
        assert!(
            houses
                .offices
                .iter()
                .any(|entry| entry.id == office("office.glaushouse.prima-donna"))
        );
        assert!(
            houses
                .roles
                .iter()
                .any(|entry| entry.id == role("role.sandmanor.minorian"))
        );
        assert!(
            houses
                .roles
                .iter()
                .any(|entry| entry.id == role("role.sandmanor.minoan"))
        );
    }
}
