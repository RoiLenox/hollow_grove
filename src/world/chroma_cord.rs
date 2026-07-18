//! Read-only `chroma_cord` projection for Glaüshouse clinical charting.
//!
//! This module is intentionally an adapter, not a second clinical engine. It
//! validates the four chart phases, gates presentation through the existing
//! institutional access model, and projects an authorized entry as a
//! `ClinicalFinding`. It cannot grant access, authorize care, or declare
//! Clearance.

use crate::being_object_ontology::{ObjectId, ObjectState, canonical_object_state};
use crate::institution::{
    AccessPolicy, AccessRequirement, AccessRequirementMatch, InstitutionalBeingId, MembershipId,
    OfficeId, RoleId, SiteId, Visibility, ZoneId,
};
use crate::institution_affiliation::{
    AccessDeniedContext, AffiliationState, InstitutionalMembership, InstitutionalWorldState,
    LineageStatus, MembershipRole, RecognitionLevel, ZoneEntryResult,
};

use super::{canonical_institutional_world_state, house_institutions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChromaCordHue {
    Red6,
    Orange5,
    Yellow4,
    Green3,
    Blue2,
    Violet1,
}

impl ChromaCordHue {
    #[must_use]
    pub const fn as_code(self) -> &'static str {
        match self {
            Self::Red6 => "R6",
            Self::Orange5 => "O5",
            Self::Yellow4 => "Y4",
            Self::Green3 => "G3",
            Self::Blue2 => "B2",
            Self::Violet1 => "V1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChromaCordPhase {
    Presence,
    Performance,
    Perception,
    Position,
}

impl ChromaCordPhase {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presence => "presence",
            Self::Performance => "performance",
            Self::Perception => "perception",
            Self::Position => "position",
        }
    }
}

/// One committed four-phase chart statement from the external application.
///
/// Hollow Grove receives the semantic record only. Storage format, append-only
/// persistence, patient identity, consent, and retention remain owned by the
/// clinical application and its institution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChromaCordEntry {
    pub id: String,
    pub subject: InstitutionalBeingId,
    pub author: InstitutionalBeingId,
    pub presence: String,
    pub performance: String,
    pub perception: String,
    pub position: String,
    pub hue: ChromaCordHue,
    pub committed_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChromaCordEntryError {
    EmptyId,
    EmptyPhase(ChromaCordPhase),
}

impl ChromaCordEntry {
    pub fn validate(&self) -> Result<(), ChromaCordEntryError> {
        if self.id.trim().is_empty() {
            return Err(ChromaCordEntryError::EmptyId);
        }
        for (phase, value) in [
            (ChromaCordPhase::Presence, self.presence.as_str()),
            (ChromaCordPhase::Performance, self.performance.as_str()),
            (ChromaCordPhase::Perception, self.perception.as_str()),
            (ChromaCordPhase::Position, self.position.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(ChromaCordEntryError::EmptyPhase(phase));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChromaCordChartView {
    pub entry: ChromaCordEntry,
    pub finding: ObjectState,
    pub site: SiteId,
    pub zone: ZoneId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChromaCordViewError {
    InvalidEntry(ChromaCordEntryError),
    AccessDenied(Box<AccessDeniedContext>),
}

#[must_use]
pub fn chroma_cord_site_id() -> SiteId {
    SiteId::new("site.glaushouse.central-medical-district").expect("canonical chroma_cord site ID")
}

#[must_use]
pub fn chroma_cord_zone_id() -> ZoneId {
    ZoneId::new("zone.glaushouse.medical-district.recovery-chambers")
        .expect("canonical chroma_cord zone ID")
}

/// Access to a clinical chart is stricter than entry to a public diagnostic
/// hall. Role, office, or an existing scoped grant may authorize the view; the
/// application itself never creates any of those facts.
#[must_use]
pub fn chroma_cord_access_policy() -> AccessPolicy {
    AccessPolicy {
        matching: AccessRequirementMatch::Any,
        requirements: vec![
            AccessRequirement::Role(role_id("role.glaushouse.nightingale")),
            AccessRequirement::Role(role_id("role.glaushouse.recovery-staff")),
            AccessRequirement::Role(role_id("role.glaushouse.persephone")),
            AccessRequirement::Office(office_id("office.glaushouse.prima-donna")),
            AccessRequirement::ExplicitGrant,
        ],
    }
}

/// Produces a presentation-safe clinical finding only after the existing
/// Glaüshouse access gate allows the observer to read the chart.
pub fn view_chroma_cord_entry(
    state: &InstitutionalWorldState,
    observer: &InstitutionalBeingId,
    entry: &ChromaCordEntry,
) -> Result<ChromaCordChartView, ChromaCordViewError> {
    entry
        .validate()
        .map_err(ChromaCordViewError::InvalidEntry)?;
    let institution = house_institutions::glaushouse_medical_civilization_id();
    let site = chroma_cord_site_id();
    let zone = chroma_cord_zone_id();
    match state.request_zone_entry(
        observer,
        &institution,
        &site,
        &zone,
        &chroma_cord_access_policy(),
    ) {
        ZoneEntryResult::Allowed(_) => Ok(ChromaCordChartView {
            entry: entry.clone(),
            finding: canonical_object_state(ObjectId::ClinicalFinding),
            site,
            zone,
        }),
        ZoneEntryResult::Denied(denial) => Err(ChromaCordViewError::AccessDenied(Box::new(denial))),
    }
}

/// Canonical bounded witness for the external charting integration.
#[must_use]
pub fn build_chroma_cord_glaushouse_witness() -> String {
    let (state, nightingale) = nightingale_fixture();
    let outsider = being("being.glaushouse.chroma-cord-outsider");
    let entry = fixture_entry(&nightingale);
    let authorized = view_chroma_cord_entry(&state, &nightingale, &entry).is_ok();
    let outsider_denied = matches!(
        view_chroma_cord_entry(&state, &outsider, &entry),
        Err(ChromaCordViewError::AccessDenied(_))
    );
    format!(
        "chroma_cord Glaüshouse witness\n\
         author_role: Nightingale\n\
         phases: presence -> performance -> perception -> position\n\
         hue: {}\n\
         authorized_chart_view: {authorized}\n\
         outsider_denied: {outsider_denied}\n\
         authority_boundary: chart evidence does not declare Clearance\n",
        entry.hue.as_code()
    )
}

fn role_id(value: &str) -> RoleId {
    RoleId::new(value).expect("canonical chroma_cord role ID")
}

fn office_id(value: &str) -> OfficeId {
    OfficeId::new(value).expect("canonical chroma_cord office ID")
}

fn being(value: &str) -> InstitutionalBeingId {
    InstitutionalBeingId::new(value).expect("canonical chroma_cord being ID")
}

fn nightingale_fixture() -> (InstitutionalWorldState, InstitutionalBeingId) {
    let mut state = canonical_institutional_world_state();
    let nightingale = being("being.glaushouse.chroma-cord-nightingale");
    state.memberships.push(InstitutionalMembership {
        id: MembershipId::new("membership.glaushouse.chroma-cord-nightingale")
            .expect("canonical chroma_cord membership ID"),
        being: nightingale.clone(),
        institution: house_institutions::glaushouse_medical_civilization_id(),
        role_id: Some(role_id("role.glaushouse.nightingale")),
        role: MembershipRole::FullMember,
        affiliation_state: AffiliationState::Initiated,
        lineage: LineageStatus::None,
        sponsor: None,
        subgroup: None,
        joined_at: Some(0),
        initiated_at: Some(0),
        ended_at: None,
        public_visibility: Visibility::Known,
        internal_recognition: RecognitionLevel::Internal,
    });
    state
        .validate()
        .expect("canonical chroma_cord witness state");
    (state, nightingale)
}

fn fixture_entry(nightingale: &InstitutionalBeingId) -> ChromaCordEntry {
    ChromaCordEntry {
        id: "chart.glaushouse.witness-001".into(),
        subject: being("being.glaushouse.chroma-cord-patient"),
        author: nightingale.clone(),
        presence: "pressure arrives as an observable clinical signal".into(),
        performance: "the Nightingale records the coordinated intervention".into(),
        perception: "the signal remains unresolved and reviewable".into(),
        position: "continue observation without declaring Clearance".into(),
        hue: ChromaCordHue::Orange5,
        committed_at: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_entry_projects_as_a_clinical_finding() {
        let (state, nightingale) = nightingale_fixture();
        let entry = fixture_entry(&nightingale);
        let view = view_chroma_cord_entry(&state, &nightingale, &entry).unwrap();
        assert_eq!(view.finding.identity(), ObjectId::ClinicalFinding);
        assert_eq!(view.entry, entry);
        assert_eq!(view.zone, chroma_cord_zone_id());
    }

    #[test]
    fn all_four_phases_are_required() {
        let (_, nightingale) = nightingale_fixture();
        let mut entry = fixture_entry(&nightingale);
        entry.perception.clear();
        assert_eq!(
            entry.validate(),
            Err(ChromaCordEntryError::EmptyPhase(
                ChromaCordPhase::Perception
            ))
        );
    }

    #[test]
    fn nightingale_role_allows_view_without_an_app_created_grant() {
        let (state, nightingale) = nightingale_fixture();
        let entry = fixture_entry(&nightingale);
        assert!(state.access_grants.is_empty());
        assert!(view_chroma_cord_entry(&state, &nightingale, &entry).is_ok());
    }

    #[test]
    fn outsider_receives_the_existing_presentable_denial() {
        let (state, nightingale) = nightingale_fixture();
        let outsider = being("being.glaushouse.test-outsider");
        let entry = fixture_entry(&nightingale);
        match view_chroma_cord_entry(&state, &outsider, &entry) {
            Err(ChromaCordViewError::AccessDenied(denial)) => {
                assert_eq!(denial.zone, chroma_cord_zone_id());
                assert!(!denial.unmet_requirements.is_empty());
            }
            result => panic!("outsider must receive an access denial: {result:?}"),
        }
    }

    #[test]
    fn witness_keeps_chart_hue_and_clearance_authority_separate() {
        let witness = build_chroma_cord_glaushouse_witness();
        assert!(witness.contains("hue: O5"));
        assert!(witness.contains("authorized_chart_view: true"));
        assert!(witness.contains("outsider_denied: true"));
        assert!(witness.contains("does not declare Clearance"));
    }
}
