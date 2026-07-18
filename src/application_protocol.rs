//! Hollow Grove's control contract for native applications presented in Hueman.
//!
//! Hollow Grove owns lifecycle, world attachment, access, and projection policy.
//! A managed application keeps authority over its own domain grammar and storage.
//! Window geometry is presentation evidence only; it never establishes world
//! meaning by itself.

use std::fmt;

use crate::institution::{
    AccessPolicy, AccessRequirement, InstitutionCatalog, InstitutionId, InstitutionalBeingId,
    SiteId, ZoneId,
};
use crate::institution_affiliation::{AccessDecision, InstitutionalWorldState};
use crate::world::chroma_cord::{
    chroma_cord_access_policy, chroma_cord_site_id, chroma_cord_zone_id,
};
use crate::world::house_institutions::glaushouse_medical_civilization_id;

pub const APPLICATION_REGISTRY_SCHEMA_VERSION: &str = "0.1.0";
pub const APPLICATION_REGISTRY_ARTIFACT_PATH: &str =
    "artifacts/hollow_grove_application_registry.json";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationId(String);

impl ApplicationId {
    pub fn new(value: impl Into<String>) -> Result<Self, ApplicationRegistryError> {
        let value = value.into();
        if value.is_empty()
            || !value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'.' || byte == b'-'
            })
        {
            return Err(ApplicationRegistryError::InvalidApplicationId(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ApplicationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationKind {
    ClinicalCharting,
}

impl ApplicationKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClinicalCharting => "clinical_charting",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationAuthority {
    HollowGrove,
    ChromaCord,
}

impl ApplicationAuthority {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HollowGrove => "hollow_grove",
            Self::ChromaCord => "chroma_cord",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationAuthorityBoundary {
    pub lifecycle: ApplicationAuthority,
    pub world_attachment: ApplicationAuthority,
    pub access: ApplicationAuthority,
    pub projection: ApplicationAuthority,
    pub domain_grammar: ApplicationAuthority,
    pub storage: ApplicationAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationProjection {
    SemanticOnly,
}

impl ApplicationProjection {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticOnly => "semantic_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationPrivacy {
    pub capture_allowed: bool,
    pub projection: ApplicationProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationCapability {
    LifecycleControl,
    NativeWindowIdentity,
    WorldAttachment,
    InstitutionalAccess,
    SemanticProjection,
    PrivacyEnforcement,
    HealthAndVersion,
    ProtocolMigration,
    BackupCoordination,
}

impl ApplicationCapability {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleControl => "lifecycle_control",
            Self::NativeWindowIdentity => "native_window_identity",
            Self::WorldAttachment => "world_attachment",
            Self::InstitutionalAccess => "institutional_access",
            Self::SemanticProjection => "semantic_projection",
            Self::PrivacyEnforcement => "privacy_enforcement",
            Self::HealthAndVersion => "health_and_version",
            Self::ProtocolMigration => "protocol_migration",
            Self::BackupCoordination => "backup_coordination",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationProhibition {
    RewriteDomainGrammar,
    MutateCommittedRecords,
    ManufactureClearance,
    BypassIdentityOrConsent,
    CaptureClinicalWindow,
    CopyClinicalStoreIntoArtifacts,
}

impl ApplicationProhibition {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RewriteDomainGrammar => "rewrite_domain_grammar",
            Self::MutateCommittedRecords => "mutate_committed_records",
            Self::ManufactureClearance => "manufacture_clearance",
            Self::BypassIdentityOrConsent => "bypass_identity_or_consent",
            Self::CaptureClinicalWindow => "capture_clinical_window",
            Self::CopyClinicalStoreIntoArtifacts => "copy_clinical_store_into_artifacts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationScopeEnvelope {
    pub minimum_required: Vec<ApplicationCapability>,
    pub maximum_allowed: Vec<ApplicationCapability>,
    pub prohibited: Vec<ApplicationProhibition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationScreenAnchor {
    pub x_thousandths: u16,
    pub y_thousandths: u16,
}

impl ApplicationScreenAnchor {
    #[must_use]
    pub fn x(self) -> f32 {
        f32::from(self.x_thousandths) / 1_000.0
    }

    #[must_use]
    pub fn y(self) -> f32 {
        f32::from(self.y_thousandths) / 1_000.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationWorldAnchor {
    pub node_id: String,
    pub node_name: String,
    pub node_kind: String,
    pub institution: InstitutionId,
    pub site: SiteId,
    pub zone: ZoneId,
    pub screen: ApplicationScreenAnchor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationDefinition {
    pub id: ApplicationId,
    pub canonical_name: String,
    pub kind: ApplicationKind,
    pub window_app_id: String,
    pub launch_entrypoint: String,
    pub world_anchor: ApplicationWorldAnchor,
    pub privacy: ApplicationPrivacy,
    pub authority: ApplicationAuthorityBoundary,
    pub scope: ApplicationScopeEnvelope,
    pub access_policy: AccessPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowGroveApplicationRegistry {
    pub schema_version: &'static str,
    pub applications: Vec<ApplicationDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationRegistryError {
    InvalidApplicationId(String),
    DuplicateApplication(ApplicationId),
    DuplicateCanonicalName(String),
    MissingInstitution(InstitutionId),
    MissingSite(SiteId),
    MissingZone(ZoneId),
    InvalidRoleRequirement,
    InvalidOfficeRequirement,
    MinimumScopeExceedsMaximum,
    PrivacyContradictsScope,
}

impl HollowGroveApplicationRegistry {
    pub fn validate(&self, catalog: &InstitutionCatalog) -> Result<(), ApplicationRegistryError> {
        for (index, application) in self.applications.iter().enumerate() {
            if self.applications[..index]
                .iter()
                .any(|candidate| candidate.id == application.id)
            {
                return Err(ApplicationRegistryError::DuplicateApplication(
                    application.id.clone(),
                ));
            }
            if self.applications[..index]
                .iter()
                .any(|candidate| candidate.canonical_name == application.canonical_name)
            {
                return Err(ApplicationRegistryError::DuplicateCanonicalName(
                    application.canonical_name.clone(),
                ));
            }
            if catalog
                .institution(&application.world_anchor.institution)
                .is_none()
            {
                return Err(ApplicationRegistryError::MissingInstitution(
                    application.world_anchor.institution.clone(),
                ));
            }
            let Some(site) = catalog
                .sites
                .iter()
                .find(|site| site.id == application.world_anchor.site)
            else {
                return Err(ApplicationRegistryError::MissingSite(
                    application.world_anchor.site.clone(),
                ));
            };
            if site.controlled_by.as_ref() != Some(&application.world_anchor.institution)
                || !site.zones.contains(&application.world_anchor.zone)
            {
                return Err(ApplicationRegistryError::MissingZone(
                    application.world_anchor.zone.clone(),
                ));
            }
            for requirement in &application.access_policy.requirements {
                match requirement {
                    AccessRequirement::Role(role) => {
                        if !catalog.roles.iter().any(|candidate| {
                            candidate.id == *role
                                && candidate.institution == application.world_anchor.institution
                        }) {
                            return Err(ApplicationRegistryError::InvalidRoleRequirement);
                        }
                    }
                    AccessRequirement::Office(office) => {
                        if !catalog.offices.iter().any(|candidate| {
                            candidate.id == *office
                                && candidate.institution.as_ref()
                                    == Some(&application.world_anchor.institution)
                        }) {
                            return Err(ApplicationRegistryError::InvalidOfficeRequirement);
                        }
                    }
                    _ => {}
                }
            }
            if application
                .scope
                .minimum_required
                .iter()
                .any(|capability| !application.scope.maximum_allowed.contains(capability))
            {
                return Err(ApplicationRegistryError::MinimumScopeExceedsMaximum);
            }
            if application.privacy.capture_allowed
                && application
                    .scope
                    .prohibited
                    .contains(&ApplicationProhibition::CaptureClinicalWindow)
            {
                return Err(ApplicationRegistryError::PrivacyContradictsScope);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn application(&self, name_or_id: &str) -> Option<&ApplicationDefinition> {
        self.applications.iter().find(|application| {
            application.canonical_name == name_or_id || application.id.as_str() == name_or_id
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationLifecycle {
    Attached,
}

impl ApplicationLifecycle {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attached => "attached",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationAttachment {
    pub application: ApplicationDefinition,
    pub lifecycle: ApplicationLifecycle,
    pub window_id: u64,
    pub observed_window_app_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationAttachmentError {
    UnknownApplication(String),
    WindowIdentityMismatch { expected: String, observed: String },
}

#[must_use]
pub fn canonical_hollow_grove_application_registry() -> HollowGroveApplicationRegistry {
    HollowGroveApplicationRegistry {
        schema_version: APPLICATION_REGISTRY_SCHEMA_VERSION,
        applications: vec![ApplicationDefinition {
            id: ApplicationId::new("application.glaushouse.chroma-cord")
                .expect("canonical application ID"),
            canonical_name: "chroma_cord".into(),
            kind: ApplicationKind::ClinicalCharting,
            window_app_id: "hollow-grove.chroma-cord".into(),
            launch_entrypoint: "launch-chroma-cord.sh".into(),
            world_anchor: ApplicationWorldAnchor {
                node_id: "glaushouse".into(),
                node_name: "Glaüshouse".into(),
                node_kind: "kingdom".into(),
                institution: glaushouse_medical_civilization_id(),
                site: chroma_cord_site_id(),
                zone: chroma_cord_zone_id(),
                screen: ApplicationScreenAnchor {
                    x_thousandths: 500,
                    y_thousandths: 860,
                },
            },
            privacy: ApplicationPrivacy {
                capture_allowed: false,
                projection: ApplicationProjection::SemanticOnly,
            },
            authority: ApplicationAuthorityBoundary {
                lifecycle: ApplicationAuthority::HollowGrove,
                world_attachment: ApplicationAuthority::HollowGrove,
                access: ApplicationAuthority::HollowGrove,
                projection: ApplicationAuthority::HollowGrove,
                domain_grammar: ApplicationAuthority::ChromaCord,
                storage: ApplicationAuthority::ChromaCord,
            },
            scope: ApplicationScopeEnvelope {
                minimum_required: vec![
                    ApplicationCapability::LifecycleControl,
                    ApplicationCapability::NativeWindowIdentity,
                    ApplicationCapability::WorldAttachment,
                    ApplicationCapability::InstitutionalAccess,
                    ApplicationCapability::SemanticProjection,
                    ApplicationCapability::PrivacyEnforcement,
                ],
                maximum_allowed: vec![
                    ApplicationCapability::LifecycleControl,
                    ApplicationCapability::NativeWindowIdentity,
                    ApplicationCapability::WorldAttachment,
                    ApplicationCapability::InstitutionalAccess,
                    ApplicationCapability::SemanticProjection,
                    ApplicationCapability::PrivacyEnforcement,
                    ApplicationCapability::HealthAndVersion,
                    ApplicationCapability::ProtocolMigration,
                    ApplicationCapability::BackupCoordination,
                ],
                prohibited: vec![
                    ApplicationProhibition::RewriteDomainGrammar,
                    ApplicationProhibition::MutateCommittedRecords,
                    ApplicationProhibition::ManufactureClearance,
                    ApplicationProhibition::BypassIdentityOrConsent,
                    ApplicationProhibition::CaptureClinicalWindow,
                    ApplicationProhibition::CopyClinicalStoreIntoArtifacts,
                ],
            },
            access_policy: chroma_cord_access_policy(),
        }],
    }
}

pub fn attach_managed_application_window(
    registry: &HollowGroveApplicationRegistry,
    name_or_id: &str,
    observed_window_app_id: &str,
    window_id: u64,
) -> Result<ApplicationAttachment, ApplicationAttachmentError> {
    let application = registry
        .application(name_or_id)
        .ok_or_else(|| ApplicationAttachmentError::UnknownApplication(name_or_id.to_owned()))?;
    if application.window_app_id != observed_window_app_id {
        return Err(ApplicationAttachmentError::WindowIdentityMismatch {
            expected: application.window_app_id.clone(),
            observed: observed_window_app_id.to_owned(),
        });
    }
    Ok(ApplicationAttachment {
        application: application.clone(),
        lifecycle: ApplicationLifecycle::Attached,
        window_id,
        observed_window_app_id: observed_window_app_id.to_owned(),
    })
}

#[must_use]
pub fn authorize_managed_application_view(
    state: &InstitutionalWorldState,
    observer: &InstitutionalBeingId,
    application: &ApplicationDefinition,
) -> AccessDecision {
    state.evaluate_access(
        observer,
        &application.world_anchor.institution,
        &application.world_anchor.site,
        &application.world_anchor.zone,
        &application.access_policy,
    )
}

#[must_use]
pub fn build_hollow_grove_application_witness() -> String {
    let registry = canonical_hollow_grove_application_registry();
    let application = registry
        .application("chroma_cord")
        .expect("canonical chroma_cord application");
    format!(
        "Hollow Grove managed application\n\
         application: {}\n\
         application_id: {}\n\
         window_app_id: {}\n\
         lifecycle_owner: {}\n\
         world_attachment_owner: {}\n\
         access_owner: {}\n\
         domain_grammar_owner: {}\n\
         storage_owner: {}\n\
         world_anchor: {} ({}, {})\n\
         projection: {}\n\
         capture_allowed: {}\n\
         minimum_scope: lifecycle + identity + attachment + access + semantic projection + privacy\n\
         maximum_scope: orchestration + health + versioned migration + backup coordination\n\
         scope_ceiling: chart grammar and committed records remain outside Hollow Grove mutation authority\n",
        application.canonical_name,
        application.id,
        application.window_app_id,
        application.authority.lifecycle.as_str(),
        application.authority.world_attachment.as_str(),
        application.authority.access.as_str(),
        application.authority.domain_grammar.as_str(),
        application.authority.storage.as_str(),
        application.world_anchor.node_name,
        application.world_anchor.screen.x(),
        application.world_anchor.screen.y(),
        application.privacy.projection.as_str(),
        application.privacy.capture_allowed,
    )
}

#[must_use]
pub fn build_hollow_grove_application_registry_json() -> String {
    let registry = canonical_hollow_grove_application_registry();
    let application = registry
        .application("chroma_cord")
        .expect("canonical chroma_cord application");
    format!(
        "{{\n\
           \"schema_version\": \"{}\",\n\
           \"control_plane\": \"hollow_grove\",\n\
           \"applications\": [\n\
             {{\n\
               \"id\": \"{}\",\n\
               \"canonical_name\": \"{}\",\n\
               \"kind\": \"{}\",\n\
               \"window_app_id\": \"{}\",\n\
               \"launch_entrypoint\": \"{}\",\n\
               \"world_anchor\": {{\n\
                 \"id\": \"{}\",\n\
                 \"name\": \"{}\",\n\
                 \"kind\": \"{}\",\n\
                 \"institution_id\": \"{}\",\n\
                 \"site_id\": \"{}\",\n\
                 \"zone_id\": \"{}\",\n\
                 \"normalized\": {{\"x\": {}, \"y\": {}}}\n\
               }},\n\
               \"privacy\": {{\n\
                 \"capture_allowed\": {},\n\
                 \"projection\": \"{}\"\n\
               }},\n\
               \"authority\": {{\n\
                 \"lifecycle\": \"{}\",\n\
                 \"world_attachment\": \"{}\",\n\
                 \"access\": \"{}\",\n\
                 \"projection\": \"{}\",\n\
                 \"domain_grammar\": \"{}\",\n\
                 \"storage\": \"{}\"\n\
               }},\n\
               \"scope\": {{\n\
                 \"minimum_required\": [\"lifecycle_control\", \"native_window_identity\", \"world_attachment\", \"institutional_access\", \"semantic_projection\", \"privacy_enforcement\"],\n\
                 \"maximum_allowed\": [\"lifecycle_control\", \"native_window_identity\", \"world_attachment\", \"institutional_access\", \"semantic_projection\", \"privacy_enforcement\", \"health_and_version\", \"protocol_migration\", \"backup_coordination\"],\n\
                 \"prohibited\": [\"rewrite_domain_grammar\", \"mutate_committed_records\", \"manufacture_clearance\", \"bypass_identity_or_consent\", \"capture_clinical_window\", \"copy_clinical_store_into_artifacts\"]\n\
               }}\n\
             }}\n\
           ]\n\
         }}\n",
        registry.schema_version,
        application.id,
        application.canonical_name,
        application.kind.as_str(),
        application.window_app_id,
        application.launch_entrypoint,
        application.world_anchor.node_id,
        application.world_anchor.node_name,
        application.world_anchor.node_kind,
        application.world_anchor.institution,
        application.world_anchor.site,
        application.world_anchor.zone,
        application.world_anchor.screen.x(),
        application.world_anchor.screen.y(),
        application.privacy.capture_allowed,
        application.privacy.projection.as_str(),
        application.authority.lifecycle.as_str(),
        application.authority.world_attachment.as_str(),
        application.authority.access.as_str(),
        application.authority.projection.as_str(),
        application.authority.domain_grammar.as_str(),
        application.authority.storage.as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::institution::AccessRequirement;
    use crate::world::{canonical_institutional_world_state, institutional_access_fixture};

    #[test]
    fn canonical_registry_validates_against_the_institutional_catalog() {
        let state = canonical_institutional_world_state();
        let registry = canonical_hollow_grove_application_registry();
        registry.validate(&state.catalog).unwrap();
        assert_eq!(registry.applications.len(), 1);
    }

    #[test]
    fn chroma_cord_is_explicitly_attached_to_glaushouse() {
        let registry = canonical_hollow_grove_application_registry();
        let application = registry.application("chroma_cord").unwrap();
        assert_eq!(application.world_anchor.node_id, "glaushouse");
        assert_eq!(application.world_anchor.screen.x_thousandths, 500);
        assert_eq!(application.world_anchor.screen.y_thousandths, 860);
        assert_eq!(
            application.authority.world_attachment,
            ApplicationAuthority::HollowGrove
        );
    }

    #[test]
    fn native_window_identity_is_required_for_attachment() {
        let registry = canonical_hollow_grove_application_registry();
        let attachment = attach_managed_application_window(
            &registry,
            "chroma_cord",
            "hollow-grove.chroma-cord",
            41,
        )
        .unwrap();
        assert_eq!(attachment.lifecycle, ApplicationLifecycle::Attached);
        assert!(matches!(
            attach_managed_application_window(&registry, "chroma_cord", "kitty", 41),
            Err(ApplicationAttachmentError::WindowIdentityMismatch { .. })
        ));
    }

    #[test]
    fn clinical_surface_is_semantic_only_and_uses_existing_access_policy() {
        let registry = canonical_hollow_grove_application_registry();
        let application = registry.application("chroma_cord").unwrap();
        assert!(!application.privacy.capture_allowed);
        assert_eq!(
            application.privacy.projection,
            ApplicationProjection::SemanticOnly
        );
        assert!(
            application
                .access_policy
                .requirements
                .contains(&AccessRequirement::ExplicitGrant)
        );
        assert!(
            application
                .scope
                .maximum_allowed
                .contains(&ApplicationCapability::ProtocolMigration)
        );
        assert!(
            application
                .scope
                .prohibited
                .contains(&ApplicationProhibition::MutateCommittedRecords)
        );
    }

    #[test]
    fn hollow_grove_authorizes_the_managed_view_from_institutional_state() {
        let state = institutional_access_fixture();
        let registry = canonical_hollow_grove_application_registry();
        let application = registry.application("chroma_cord").unwrap();
        let authorized = InstitutionalBeingId::new("being.glaushouse.fixture-member").unwrap();
        let outsider = InstitutionalBeingId::new("being.glaushouse.application-outsider").unwrap();
        assert_eq!(
            authorize_managed_application_view(&state, &authorized, application),
            AccessDecision::Allowed
        );
        assert_eq!(
            authorize_managed_application_view(&state, &outsider, application),
            AccessDecision::Denied
        );
    }

    #[test]
    fn checked_registry_artifact_matches_the_typed_builder() {
        assert_eq!(
            build_hollow_grove_application_registry_json(),
            include_str!("../artifacts/hollow_grove_application_registry.json")
        );
    }

    #[test]
    fn witness_states_the_split_authority_and_privacy_boundary() {
        let witness = build_hollow_grove_application_witness();
        assert!(witness.contains("lifecycle_owner: hollow_grove"));
        assert!(witness.contains("domain_grammar_owner: chroma_cord"));
        assert!(witness.contains("world_anchor: Glaüshouse (0.5, 0.86)"));
        assert!(witness.contains("capture_allowed: false"));
    }
}
