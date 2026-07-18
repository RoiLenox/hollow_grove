//! Runtime-facing institutional session seam.
//!
//! A session owns the mutable institutional state used by traversal and scene
//! adapters. It exposes facts and presentation-ready access results only; it
//! does not feed an institutional decision back into Stanislavski or the
//! Hollow Grove kernel.

use crate::falloutman::{InstitutionalAccessPresentation, present_institutional_access};
use crate::institution::{AccessPolicy, InstitutionId, InstitutionalBeingId, SiteId, ZoneId};
use crate::institution_affiliation::{
    InstitutionalSceneContext, InstitutionalWorldState, MembershipValidationError, ZoneEntryResult,
};

use super::canonical_institutional_world_state;
use super::persistence::{
    INSTITUTIONAL_STATE_ARTIFACT_PATH, build_persisted_state_output, parse_persisted_state,
};

#[derive(Debug, Clone)]
pub struct WorldSession {
    institutional: InstitutionalWorldState,
}

impl WorldSession {
    /// Starts a runtime session from the canonical four-House institutional
    /// catalog. Dynamic memberships and events can be loaded into this seam
    /// once institutional persistence is introduced.
    #[must_use]
    pub fn canonical() -> Self {
        Self::from_institutional_state(canonical_institutional_world_state())
            .expect("canonical institutional world state must validate")
    }

    /// Accepts already-loaded state only when its neutral institutional
    /// invariants hold.
    pub fn from_institutional_state(
        institutional: InstitutionalWorldState,
    ) -> Result<Self, MembershipValidationError> {
        institutional.validate()?;
        Ok(Self { institutional })
    }

    /// Loads dynamic institutional records over the canonical catalog. A
    /// missing artifact starts a fresh canonical session; malformed persisted
    /// state fails closed instead of being silently discarded.
    pub fn load_or_canonical_at(root: &std::path::Path) -> std::io::Result<Self> {
        let canonical = canonical_institutional_world_state();
        match crate::artifact_io::read_text_artifact(&root.join(INSTITUTIONAL_STATE_ARTIFACT_PATH))
        {
            Ok(contents) => {
                let state = parse_persisted_state(&contents, canonical.catalog)?;
                Self::from_institutional_state(state).map_err(|error| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("invalid institutional session: {error:?}"),
                    )
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::canonical()),
            Err(error) => Err(error),
        }
    }

    #[must_use]
    pub const fn institutional(&self) -> &InstitutionalWorldState {
        &self.institutional
    }

    #[must_use]
    pub fn scene_context_for(
        &self,
        observer: &InstitutionalBeingId,
        subject: &InstitutionalBeingId,
        institution: &InstitutionId,
    ) -> InstitutionalSceneContext {
        self.institutional
            .scene_context_for(observer, subject, institution)
    }

    /// Evaluates a requested entry without changing position or selecting a
    /// response. The caller decides whether to present or act on the result.
    #[must_use]
    pub fn request_zone_entry(
        &self,
        being: &InstitutionalBeingId,
        institution: &InstitutionId,
        site: &SiteId,
        zone: &ZoneId,
        policy: &AccessPolicy,
    ) -> ZoneEntryResult {
        self.institutional
            .request_zone_entry(being, institution, site, zone, policy)
    }

    /// Converts an already-evaluated access result into generic Falloutman
    /// presentation. This remains separate from candidate generation.
    #[must_use]
    pub fn present_zone_entry(&self, result: &ZoneEntryResult) -> InstitutionalAccessPresentation {
        present_institutional_access(result)
    }

    /// Stable runtime artifact content proving the active pipeline has loaded
    /// the typed institutional catalog. It intentionally reports no lore-only
    /// dynamic event or chosen action.
    #[must_use]
    pub fn runtime_context_output(&self) -> String {
        let catalog = &self.institutional.catalog;
        format!(
            "# Hollow Grove Institutional Runtime Context\n\n\
             ## Loaded State\n\n\
             - institutions: {}\n\
             - offices: {}\n\
             - roles: {}\n\
             - sites: {}\n\
             - relationships: {}\n\
             - active_memberships: {}\n\n\
             ## Boundary\n\n\
             - institutional facts may inform scene and traversal presentation\n\
             - zone-entry evaluation is read-only\n\
             - Stanislavski still chooses tactics\n\
             - Hollow Grove kernel mechanics receive no institutional action selection\n",
            catalog.institutions.len(),
            catalog.offices.len(),
            catalog.roles.len(),
            catalog.sites.len(),
            catalog.relationships.len(),
            self.institutional.memberships.len(),
        )
    }

    #[must_use]
    pub fn persisted_state_output(&self) -> String {
        build_persisted_state_output(&self.institutional)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::WorldSession;
    use crate::institution::InstitutionalBeingId;
    use crate::institution_affiliation::ZoneEntryResult;
    use crate::world::house_scene_context::glaushouse_scene_context;
    use crate::world::institutional_access_fixture;
    use crate::world::persistence::INSTITUTIONAL_STATE_ARTIFACT_PATH;

    fn being(value: &str) -> InstitutionalBeingId {
        InstitutionalBeingId::new(value).expect("test being ID")
    }

    #[test]
    fn session_adapts_denied_traversal_to_falloutman_without_selecting_a_tactic() {
        let session = WorldSession::from_institutional_state(institutional_access_fixture())
            .expect("access fixture must validate");
        let holder = being("being.glaushouse.fixture-member");
        let outsider = being("being.test.outsider");
        let context = glaushouse_scene_context(session.institutional(), &outsider, &holder);
        let recovery = context
            .zones
            .iter()
            .find(|entry| {
                entry.zone.as_str() == "zone.glaushouse.medical-district.recovery-chambers"
            })
            .expect("recovery zone");

        let result = session.request_zone_entry(
            &outsider,
            &recovery.institution,
            &recovery.site,
            &recovery.zone,
            &recovery.policy,
        );
        assert!(matches!(result, ZoneEntryResult::Denied(_)));

        let presentation = session.present_zone_entry(&result);
        assert_eq!(presentation.heading, "Access restricted");
        assert!(presentation.detail.contains("recovery-chambers"));
    }

    #[test]
    fn canonical_session_reports_a_valid_read_only_runtime_context() {
        let output = WorldSession::canonical().runtime_context_output();
        assert!(output.contains("# Hollow Grove Institutional Runtime Context"));
        assert!(output.contains("Stanislavski still chooses tactics"));
    }

    #[test]
    fn session_loads_dynamic_state_against_the_canonical_catalog() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("hollow-grove-institutional-session-{nonce}"));
        let session = WorldSession::from_institutional_state(institutional_access_fixture())
            .expect("fixture state");
        crate::artifact_io::write_text_artifact(
            &root.join(INSTITUTIONAL_STATE_ARTIFACT_PATH),
            &session.persisted_state_output(),
        )
        .expect("persisted state");

        let restored = WorldSession::load_or_canonical_at(&root).expect("restored session");
        assert_eq!(
            restored.institutional().memberships,
            session.institutional().memberships
        );
        fs::remove_dir_all(root).expect("temporary session cleanup");
    }
}
