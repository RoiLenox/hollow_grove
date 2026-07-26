//! Canonical institutional fixtures for the three non-Flynt Houses.
//! They reuse the neutral institutional domain; no new decision logic lives here.

use crate::hollow_grove_contract::House;
use crate::institution::*;
use crate::world::{glaushouse, sandmanor, stonebend};

fn id<T>(value: &str, build: impl FnOnce(String) -> Result<T, IdError>) -> T {
    build(value.into()).expect("canonical stable ID")
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
    stonebend::stonebend_constitution_id()
}
pub fn sandmen_id() -> InstitutionId {
    sandmanor::proof_civilization_id()
}
pub fn glaushouse_medical_civilization_id() -> InstitutionId {
    glaushouse::medical_civilization_id()
}

pub fn canonical_house_institutions() -> InstitutionCatalog {
    let stonebend = stonebend_constitution_id();
    let proliteriate = stonebend::proliteriate_id();
    let freemason = stonebend::freemason_institution_id();
    let sandmen = sandmen_id();
    let minoan_courthouse = sandmanor::milestone::minoan_county_courthouse_id();
    let glaushouse = glaushouse_medical_civilization_id();
    let glauspitals = glaushouse::glauspitals_id();
    let chromacord = glaushouse::chromacord_id();
    let nightingales = glaushouse::nightingales_id();
    let stonebender = site("site.stonebend.stonebender");
    let aura_beach = site("site.sandmanor.aura-beach");
    let minoan_courthouse_site = sandmanor::milestone::minoan_county_courthouse_site_id();
    // Save-compatible legacy ID for the one site displayed as Aura Field.
    let aura_fields = site("site.sandmanor.aura-fields");
    let medical_district = site("site.glaushouse.central-medical-district");
    InstitutionCatalog {
        institutions: vec![
            Institution {
                id: stonebend.clone(),
                name: "Stonebend Constitution".into(),
                kinds: vec![InstitutionKind::Government],
                house: Some(House::Stonebend),
                domains: vec![
                    "identity".into(),
                    "Name".into(),
                    "Title".into(),
                    "structure".into(),
                    "boundary".into(),
                    "continuity".into(),
                    "record".into(),
                    "lineage".into(),
                    "custody".into(),
                    "inheritance".into(),
                    "lawful Hollowing".into(),
                ],
                headquarters: Some(stonebender.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Internal,
            },
            Institution {
                id: proliteriate.clone(),
                name: "Proliteriate".into(),
                kinds: vec![InstitutionKind::Government],
                house: Some(House::Stonebend),
                domains: vec![
                    "public witness".into(),
                    "petition".into(),
                    "challenge".into(),
                    "Yield accountability".into(),
                    "distributed network mandates".into(),
                    "temporary witnesses".into(),
                    "succession hearing".into(),
                    "restitution".into(),
                ],
                headquarters: Some(stonebender.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Open,
            },
            Institution {
                id: freemason.clone(),
                name: "Freemason".into(),
                kinds: vec![InstitutionKind::Government],
                house: Some(House::Stonebend),
                domains: vec![
                    "architecture".into(),
                    "construction".into(),
                    "structural verification".into(),
                    "survey".into(),
                    "Seal".into(),
                    "custody".into(),
                    "defense".into(),
                    "lawful Hollowing execution".into(),
                    "restoration".into(),
                ],
                headquarters: Some(stonebender.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
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
                    "honest design".into(),
                    "proof".into(),
                    "method".into(),
                    "evidence".into(),
                    "reproduction".into(),
                    "criticism".into(),
                    "failure preservation".into(),
                    "revision".into(),
                    "reciprocal teaching".into(),
                    "witnessed improvement".into(),
                    "measurement".into(),
                    "standards".into(),
                ],
                headquarters: None,
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Open,
            },
            Institution {
                id: minoan_courthouse.clone(),
                name: "Minoan County Courthouse".into(),
                kinds: vec![InstitutionKind::Government],
                house: Some(House::Sandmanor),
                domains: vec![
                    "Southern Law".into(),
                    "marine law".into(),
                    "restricted-water violations".into(),
                    "Current Break permits and violations".into(),
                    "coastal public order".into(),
                    "booking and arraignment".into(),
                    "temporary lawful detention".into(),
                    "lawful transfer into Glaüshouse".into(),
                    "shared Hollow Grove judiciary".into(),
                    "five-stage judicial and Restitution cycle".into(),
                    "cross-House evidence without House-authority transfer".into(),
                ],
                headquarters: Some(minoan_courthouse_site.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Internal,
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
                    "constitutional medicine".into(),
                    "clearance law".into(),
                    "clinical ethics".into(),
                    "institutional coordination".into(),
                ],
                headquarters: Some(medical_district.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Internal,
            },
            Institution {
                id: glauspitals.clone(),
                name: "Glauspitals".into(),
                kinds: vec![InstitutionKind::Medical],
                house: Some(House::Glaushouse),
                domains: vec![
                    "diagnosis".into(),
                    "medicine".into(),
                    "care".into(),
                    "rehabilitation".into(),
                    "recovery".into(),
                    "Synthesis facilities".into(),
                ],
                headquarters: Some(medical_district.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
            },
            Institution {
                id: chromacord.clone(),
                name: "Chromacord".into(),
                kinds: vec![InstitutionKind::Medical],
                house: Some(House::Glaushouse),
                domains: vec![
                    "clinical records".into(),
                    "diagnostic evidence".into(),
                    "adverse-event records".into(),
                    "privacy".into(),
                ],
                headquarters: Some(medical_district.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Compartmentalized,
            },
            Institution {
                id: nightingales.clone(),
                name: "The Nightingales".into(),
                kinds: vec![InstitutionKind::Medical, InstitutionKind::Cultural],
                house: Some(House::Glaushouse),
                domains: vec![
                    "nursing".into(),
                    "clinical care".into(),
                    "patient advocacy".into(),
                    "bedside protection".into(),
                    "immediate clinical stop".into(),
                ],
                headquarters: Some(medical_district.clone()),
                public_visibility: Visibility::Public,
                internal_secrecy: SecrecyLevel::Internal,
            },
        ],
        offices: vec![
            Office {
                id: stonebend::hypergiant_office_id(),
                name: "Hypergiant".into(),
                scope: OfficeScope::House,
                institution: Some(stonebend.clone()),
                house: Some(House::Stonebend),
                singular: true,
                authority: vec![
                    "ConstitutionalIdentity".into(),
                    "HighNameConfirmation".into(),
                    "HighTitleConfirmation".into(),
                    "ConstitutionalIntegrity".into(),
                    "HighContinuityDispute".into(),
                ],
            },
            Office {
                id: stonebend::high_freemason_office_id(),
                name: "High Freemason".into(),
                scope: OfficeScope::Institution,
                institution: Some(freemason.clone()),
                house: Some(House::Stonebend),
                singular: true,
                authority: vec![
                    "StructuralCertification".into(),
                    "SurveyCertification".into(),
                    "SealIssuance".into(),
                    "CustodyCertification".into(),
                    "LawfulExecution".into(),
                ],
            },
            Office {
                id: sandmanor::sandman_office_id(),
                name: "The Sandman".into(),
                scope: OfficeScope::House,
                institution: Some(sandmen.clone()),
                house: Some(House::Sandmanor),
                singular: true,
                authority: vec![
                    "WitnessedImprovement".into(),
                    "ProofDetermination".into(),
                    "ReciprocalTeaching".into(),
                    "ReproductionOrder".into(),
                    "DesignIntegrity".into(),
                    "ContestIntegrity".into(),
                    "StandardsOfEvidence".into(),
                    "ConfigurationRule".into(),
                ],
            },
            Office {
                id: glaushouse::prima_donna_office_id(),
                name: "Prima Donna".into(),
                scope: OfficeScope::House,
                institution: Some(glaushouse.clone()),
                house: Some(House::Glaushouse),
                singular: true,
                authority: vec![
                    // Frozen Constitutional Runtime V2 adapter capabilities.
                    // These do not replace the typed Glaushouse clearance law;
                    // they let common HouseDecision projections address it.
                    "PublicClearance".into(),
                    "FinalJudgmentAnswerability".into(),
                    "ClinicalSovereignty".into(),
                    "HighRiskClearance".into(),
                    "SynthesisLaw".into(),
                    "ClinicalEthics".into(),
                    "ConstitutionalInterpretation".into(),
                ],
            },
        ],
        roles: vec![
            Role {
                id: role("role.stonebend.proliteriate-representative"),
                name: "Proliteriate representative".into(),
                institution: proliteriate.clone(),
            },
            Role {
                id: role("role.stonebend.freemason-member"),
                name: "Freemason member".into(),
                institution: freemason.clone(),
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
                id: glaushouse::nightingale_rank_id(),
                name: "Nightingale".into(),
                institution: nightingales.clone(),
            },
            Role {
                id: glaushouse::matron_rank_id(),
                name: "Matron".into(),
                institution: nightingales.clone(),
            },
            Role {
                id: glaushouse::marshal_rank_id(),
                name: "Marshal".into(),
                institution: nightingales.clone(),
            },
            Role {
                id: glaushouse::persephone_rank_id(),
                name: "Persephone".into(),
                institution: nightingales.clone(),
            },
            Role {
                id: role("role.glaushouse.recovery-staff"),
                name: "Recovery staff".into(),
                institution: glauspitals.clone(),
            },
        ],
        groups: vec![
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
                institution: glauspitals.clone(),
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
                zones: vec![
                    zone("zone.sandmanor.aura-beach.free-aura-beach"),
                    zone("zone.sandmanor.aura-beach.southern-coast"),
                    zone("zone.sandmanor.aura-beach.current-break"),
                    zone("zone.sandmanor.aura-beach.courthouse-approach"),
                    zone("zone.sandmanor.aura-beach.court-strand"),
                ],
            },
            Site {
                id: minoan_courthouse_site.clone(),
                name: "Minoan County Courthouse".into(),
                house: House::Sandmanor,
                site_kinds: vec![SiteKind::PrivateCourt, SiteKind::Headquarters],
                controlled_by: Some(minoan_courthouse.clone()),
                zones: vec![
                    zone("zone.sandmanor.minoan-county-courthouse.public-desk"),
                    zone("zone.sandmanor.minoan-county-courthouse.holding"),
                    zone("zone.sandmanor.minoan-county-courthouse.glaushouse-transfer"),
                ],
            },
            Site {
                id: aura_fields.clone(),
                name: "Aura Field".into(),
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
                controlled_by: Some(glauspitals.clone()),
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
                "relationship.stonebend.proliteriate-represents-geralds",
                InstitutionalEntityId::Institution(proliteriate.clone()),
                RelationshipKind::Represents,
                InstitutionalEntityId::Institution(stonebend.clone()),
            ),
            rel(
                "relationship.stonebend.hypergiant-coordinates-proliteriate",
                InstitutionalEntityId::Office(stonebend::hypergiant_office_id()),
                RelationshipKind::Coordinates,
                InstitutionalEntityId::Institution(proliteriate.clone()),
            ),
            rel(
                "relationship.stonebend.high-freemason-coordinates-hypergiant",
                InstitutionalEntityId::Office(stonebend::high_freemason_office_id()),
                RelationshipKind::Coordinates,
                InstitutionalEntityId::Office(stonebend::hypergiant_office_id()),
            ),
            rel(
                "relationship.stonebend.high-freemason-leads-freemason",
                InstitutionalEntityId::Office(stonebend::high_freemason_office_id()),
                RelationshipKind::Commands,
                InstitutionalEntityId::Institution(freemason.clone()),
            ),
            rel(
                "relationship.stonebend.freemason-cooperates-with-proliteriate",
                InstitutionalEntityId::Institution(freemason.clone()),
                RelationshipKind::CooperatesWith,
                InstitutionalEntityId::Institution(proliteriate.clone()),
            ),
            rel(
                "relationship.sandmanor.sandman-coordinates-proof-body",
                InstitutionalEntityId::Office(sandmanor::sandman_office_id()),
                RelationshipKind::Coordinates,
                InstitutionalEntityId::Institution(sandmen.clone()),
            ),
            rel(
                "relationship.sandmanor.minoan-courthouse-hosted-by-minoans",
                InstitutionalEntityId::Institution(minoan_courthouse),
                RelationshipKind::HostedBy,
                InstitutionalEntityId::Group(group("group.sandmanor.minoans")),
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
                InstitutionalEntityId::Institution(glauspitals.clone()),
            ),
            rel(
                "relationship.glaushouse.prima-donna-coordinates-glauspitals",
                InstitutionalEntityId::Office(glaushouse::prima_donna_office_id()),
                RelationshipKind::Coordinates,
                InstitutionalEntityId::Institution(glauspitals.clone()),
            ),
            rel(
                "relationship.glaushouse.chromacord-cooperates-glauspitals",
                InstitutionalEntityId::Institution(chromacord.clone()),
                RelationshipKind::CooperatesWith,
                InstitutionalEntityId::Institution(glauspitals.clone()),
            ),
            rel(
                "relationship.glaushouse.glauspitals-grants-chromacord-site-access",
                InstitutionalEntityId::Institution(glauspitals.clone()),
                RelationshipKind::GrantsAccessTo,
                InstitutionalEntityId::Institution(chromacord.clone()),
            ),
            rel(
                "relationship.stonebend.headquartered-at-stonebender",
                InstitutionalEntityId::Institution(stonebend.clone()),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(stonebender),
            ),
            rel(
                "relationship.stonebend.proliteriate-headquartered-at-stonebender",
                InstitutionalEntityId::Institution(proliteriate),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(site("site.stonebend.stonebender")),
            ),
            rel(
                "relationship.stonebend.freemason-headquartered-at-stonebender",
                InstitutionalEntityId::Institution(freemason),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(site("site.stonebend.stonebender")),
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
                InstitutionalEntityId::Site(medical_district.clone()),
            ),
            rel(
                "relationship.glaushouse.glauspitals-headquartered-at-medical-district",
                InstitutionalEntityId::Institution(glauspitals.clone()),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(medical_district.clone()),
            ),
            rel(
                "relationship.glaushouse.chromacord-headquartered-at-medical-district",
                InstitutionalEntityId::Institution(chromacord),
                RelationshipKind::HeadquarteredAt,
                InstitutionalEntityId::Site(medical_district.clone()),
            ),
            rel(
                "relationship.glaushouse.nightingales-headquartered-at-medical-district",
                InstitutionalEntityId::Institution(nightingales),
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
                .any(|entry| entry.id == stonebend::hypergiant_office_id())
        );
        assert!(houses.institution(&stonebend::proliteriate_id()).is_some());
        assert!(
            houses
                .institution(&stonebend::freemason_institution_id())
                .is_some()
        );
        assert!(
            houses
                .offices
                .iter()
                .any(|entry| entry.id == stonebend::high_freemason_office_id())
        );
        assert!(
            houses
                .offices
                .iter()
                .any(|entry| entry.id == sandmanor::sandman_office_id())
        );
        assert!(
            houses
                .offices
                .iter()
                .any(|entry| entry.id == glaushouse::prima_donna_office_id())
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
