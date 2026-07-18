//! Neutral affiliation mechanics shared by institutional domains.
//! These records hold state and validate authority; they do not choose tactics
//! or supply setting-specific language.

use crate::institution::{
    AccessPolicy, AccessRequirement, AccessRequirementMatch, ClearanceLevel, GroupId,
    InstitutionCatalog, InstitutionId, InstitutionalBeingId, InstitutionalEntityId,
    InstitutionalRelationship, MembershipId, RoleId, SiteId, Visibility, ZoneId,
};

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|b| {
                        b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'.' || b == b'-'
                    })
                {
                    return None;
                }
                Some(Self(value))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
id!(AffiliationId);
id!(SponsorshipId);
id!(ReputationId);
id!(ObligationId);
id!(ClaimId);
id!(EventId);
id!(AccessGrantId);

pub type WorldTimestamp = u64;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AffiliationState {
    None,
    Candidate,
    Associate,
    RecognizedAssociate,
    Initiated,
    Senior,
    Former,
    Suspended,
    Expelled,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MembershipRole {
    Candidate,
    Associate,
    FullMember,
    Officer,
    Sponsor,
    Elder,
    Leader,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineageStatus {
    None,
    AffiliatedFamily,
    Inherited,
    Founding,
    Disputed,
    Disowned,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecognitionLevel {
    None,
    Provisional,
    Internal,
    Established,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstitutionalVerb {
    Recruit,
    Sponsor,
    Initiate,
    Recognize,
    Represent,
    Protect,
    Shelter,
    Broker,
    Collect,
    Appraise,
    Commission,
    Smuggle,
    Launder,
    Mediate,
    Warn,
    Retaliate,
    Suspend,
    Expel,
    Restore,
    GrantAccess,
    RevokeAccess,
    RecordDebt,
    SettleDebt,
    ClaimAffiliation,
    VerifyAffiliation,
    DenyAffiliation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthoritySource {
    Membership,
    Role,
    Office,
    Sponsorship,
    ExplicitGrant,
    Emergency,
    Custom,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetScope {
    SelfOnly,
    Member,
    Subgroup,
    Institution,
    Public,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimTruth {
    Unknown,
    True,
    False,
    PartiallyTrue,
    FormerlyTrue,
    Disputed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationResult {
    Verified,
    Rejected,
    Uncertain,
    FormerlyVerified,
    Disputed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObligationKind {
    Favor,
    Financial,
    Protection,
    Silence,
    Service,
    Sponsorship,
    Restitution,
    BloodDebt,
    ArtisticCommission,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObligationStatus {
    Open,
    Called,
    PartiallyPaid,
    Settled,
    Forgiven,
    Defaulted,
    Inherited,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObligationWeight {
    Minor,
    Significant,
    Major,
    Severe,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecruitmentStatus {
    Proposed,
    Observed,
    AcceptedAsCandidate,
    ActiveCandidate,
    ApprovedForInitiation,
    Rejected,
    Withdrawn,
    Betrayed,
    Completed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpulsionReason {
    Betrayal,
    Theft,
    Cowardice,
    UnauthorizedViolence,
    RefusalOfDebt,
    ExposureOfSecrets,
    FalseClaim,
    PoliticalDecision,
    InternalConflict,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstitutionalEventKind {
    RecruitmentStarted,
    CandidateAccepted,
    SponsorshipCreated,
    SponsorshipEnded,
    InitiationApproved,
    InitiationCompleted,
    MembershipRecognized,
    AffiliationClaimed,
    AffiliationVerified,
    AffiliationDisputed,
    PrestigeChanged,
    AccessGranted,
    AccessRevoked,
    ObligationCreated,
    ObligationSettled,
    MemberSuspended,
    MemberExpelled,
    MemberRestored,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalMembership {
    pub id: MembershipId,
    pub being: InstitutionalBeingId,
    pub institution: InstitutionId,
    /// Stable institutional title. This is distinct from the neutral authority
    /// tier in `role`, so access policies never need a catalog-side membership.
    pub role_id: Option<RoleId>,
    pub role: MembershipRole,
    pub affiliation_state: AffiliationState,
    pub lineage: LineageStatus,
    pub sponsor: Option<InstitutionalBeingId>,
    pub subgroup: Option<GroupId>,
    pub joined_at: Option<WorldTimestamp>,
    pub initiated_at: Option<WorldTimestamp>,
    pub ended_at: Option<WorldTimestamp>,
    pub public_visibility: Visibility,
    pub internal_recognition: RecognitionLevel,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sponsorship {
    pub id: SponsorshipId,
    pub sponsor: InstitutionalBeingId,
    pub candidate: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub active: bool,
    pub started_at: WorldTimestamp,
    pub ended_at: Option<WorldTimestamp>,
    pub liability: SponsorshipLiability,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SponsorshipLiability {
    None,
    Social,
    Financial,
    Disciplinary,
    Blood,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recruitment {
    pub candidate: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub sponsor: InstitutionalBeingId,
    pub started_at: WorldTimestamp,
    pub status: RecruitmentStatus,
    pub trials_completed: u32,
    pub unresolved_betrayal: bool,
    pub obligations: Vec<ObligationId>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalObligation {
    pub id: ObligationId,
    pub debtor: InstitutionalEntityId,
    pub creditor: InstitutionalEntityId,
    pub institution: InstitutionId,
    pub kind: ObligationKind,
    pub status: ObligationStatus,
    pub weight: ObligationWeight,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffiliationClaim {
    pub id: ClaimId,
    pub claimant: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub claimed_state: AffiliationState,
    pub claimed_role: Option<MembershipRole>,
    pub truth_status: ClaimTruth,
    pub visibility: Visibility,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalCapability {
    pub institution: InstitutionId,
    pub role: Option<MembershipRole>,
    pub minimum_state: AffiliationState,
    pub action: InstitutionalVerb,
    pub target_scope: TargetScope,
    pub requires_witness: bool,
    pub requires_sponsor: bool,
    pub requires_vote: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalEvent {
    pub id: EventId,
    pub kind: InstitutionalEventKind,
    pub institution: InstitutionId,
    pub subject: InstitutionalBeingId,
    pub at: WorldTimestamp,
}
/// The single mutable home for cross-institution affiliation state.
#[derive(Debug, Clone, Default)]
pub struct InstitutionalWorldState {
    pub catalog: InstitutionCatalog,
    pub memberships: Vec<InstitutionalMembership>,
    pub sponsorships: Vec<Sponsorship>,
    pub obligations: Vec<InstitutionalObligation>,
    pub claims: Vec<AffiliationClaim>,
    pub events: Vec<InstitutionalEvent>,
    pub access_grants: Vec<AccessGrant>,
}

/// A scoped, revocable exception to ordinary institutional access policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessGrant {
    pub id: AccessGrantId,
    pub grantee: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub site: Option<SiteId>,
    pub zone: Option<ZoneId>,
    pub clearance: Option<ClearanceLevel>,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessDecision {
    Allowed,
    Denied,
}

/// A read-only traversal gate result. Consumers may present a denial, but this
/// result does not move the actor or select a response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneEntryResult {
    Allowed(ZoneEntryAllowed),
    Denied(AccessDeniedContext),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneEntryAllowed {
    pub being: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub site: SiteId,
    pub zone: ZoneId,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessDeniedContext {
    pub being: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub site: SiteId,
    pub zone: ZoneId,
    pub matching: AccessRequirementMatch,
    pub unmet_requirements: Vec<AccessRequirement>,
}
/// Read-only institutional facts available to a scene adapter.
///
/// This intentionally contains no objective, tactic, score, or selected
/// action. Decision systems may interpret these facts, but institutional state
/// never chooses on their behalf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalSceneContext {
    pub observer: InstitutionalBeingId,
    pub subject: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub observer_membership: Option<InstitutionalMembership>,
    pub subject_membership: Option<InstitutionalMembership>,
    pub relationships: Vec<InstitutionalRelationship>,
}
impl InstitutionalWorldState {
    pub fn from_catalog(catalog: InstitutionCatalog) -> Self {
        Self {
            catalog,
            ..Self::default()
        }
    }
    pub fn membership_of(
        &self,
        being: &InstitutionalBeingId,
        institution: &InstitutionId,
    ) -> Option<&InstitutionalMembership> {
        self.memberships
            .iter()
            .find(|entry| &entry.being == being && &entry.institution == institution)
    }
    #[must_use]
    pub fn members_of(&self, institution: &InstitutionId) -> Vec<&InstitutionalMembership> {
        self.memberships
            .iter()
            .filter(|entry| &entry.institution == institution)
            .collect()
    }
    #[must_use]
    pub fn institutions_for(
        &self,
        being: &InstitutionalBeingId,
    ) -> Vec<&crate::institution::Institution> {
        self.memberships
            .iter()
            .filter(|entry| &entry.being == being)
            .filter_map(|entry| self.catalog.institution(&entry.institution))
            .collect()
    }
    #[must_use]
    pub fn scene_context_for(
        &self,
        observer: &InstitutionalBeingId,
        subject: &InstitutionalBeingId,
        institution: &InstitutionId,
    ) -> InstitutionalSceneContext {
        let institution_entity = InstitutionalEntityId::Institution(institution.clone());
        let observer_entity = InstitutionalEntityId::Being(observer.clone());
        let subject_entity = InstitutionalEntityId::Being(subject.clone());
        let mut relationships = self
            .catalog
            .relationships_between(&observer_entity, &subject_entity)
            .into_iter()
            .chain(
                self.catalog
                    .relationships_between(&observer_entity, &institution_entity),
            )
            .chain(
                self.catalog
                    .relationships_between(&subject_entity, &institution_entity),
            )
            .cloned()
            .collect::<Vec<_>>();
        relationships.sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
        relationships.dedup_by(|left, right| left.id == right.id);
        InstitutionalSceneContext {
            observer: observer.clone(),
            subject: subject.clone(),
            institution: institution.clone(),
            observer_membership: self.membership_of(observer, institution).cloned(),
            subject_membership: self.membership_of(subject, institution).cloned(),
            relationships,
        }
    }
    /// Evaluates a neutral access policy against stable IDs, active membership,
    /// offices, relationships, and scoped grants. It never inspects a display
    /// name or chooses an action.
    #[must_use]
    pub fn evaluate_access(
        &self,
        being: &InstitutionalBeingId,
        institution: &InstitutionId,
        site: &SiteId,
        zone: &ZoneId,
        policy: &AccessPolicy,
    ) -> AccessDecision {
        let matches = |requirement: &AccessRequirement| match requirement {
            AccessRequirement::Public => true,
            AccessRequirement::InstitutionMembership(required) => {
                required == institution && self.has_active_membership(being, required)
            }
            AccessRequirement::Role(role) => self.memberships.iter().any(|membership| {
                membership.being == *being
                    && membership.role_id.as_ref() == Some(role)
                    && membership_is_active(membership)
            }),
            AccessRequirement::Office(office) => {
                self.catalog.office_holders.iter().any(|holder| {
                    holder.active && holder.office == *office && holder.being == *being
                })
            }
            AccessRequirement::MinimumStanding(_) => false,
            AccessRequirement::Clearance(required) => self.access_grants.iter().any(|grant| {
                grant_matches(grant, being, institution, site, zone)
                    && grant.clearance.is_some_and(|clearance| {
                        clearance_rank(clearance) >= clearance_rank(*required)
                    })
            }),
            AccessRequirement::Relationship(kind) => {
                self.catalog.relationships.iter().any(|entry| {
                    entry.kind == *kind
                        && (entry.source == InstitutionalEntityId::Being(being.clone())
                            || entry.target == InstitutionalEntityId::Being(being.clone()))
                })
            }
            AccessRequirement::ExplicitGrant => self
                .access_grants
                .iter()
                .any(|grant| grant_matches(grant, being, institution, site, zone)),
        };
        let allowed = match policy.matching {
            AccessRequirementMatch::Any => policy.requirements.iter().any(matches),
            AccessRequirementMatch::All => policy.requirements.iter().all(matches),
        };
        if allowed {
            AccessDecision::Allowed
        } else {
            AccessDecision::Denied
        }
    }
    /// Gates an attempted zone entry through an access policy without applying
    /// traversal or deciding how an actor responds to denial.
    #[must_use]
    pub fn request_zone_entry(
        &self,
        being: &InstitutionalBeingId,
        institution: &InstitutionId,
        site: &SiteId,
        zone: &ZoneId,
        policy: &AccessPolicy,
    ) -> ZoneEntryResult {
        match self.evaluate_access(being, institution, site, zone, policy) {
            AccessDecision::Allowed => ZoneEntryResult::Allowed(ZoneEntryAllowed {
                being: being.clone(),
                institution: institution.clone(),
                site: site.clone(),
                zone: zone.clone(),
            }),
            AccessDecision::Denied => ZoneEntryResult::Denied(AccessDeniedContext {
                being: being.clone(),
                institution: institution.clone(),
                site: site.clone(),
                zone: zone.clone(),
                matching: policy.matching,
                unmet_requirements: policy.requirements.clone(),
            }),
        }
    }
    fn has_active_membership(
        &self,
        being: &InstitutionalBeingId,
        institution: &InstitutionId,
    ) -> bool {
        self.memberships.iter().any(|membership| {
            membership.being == *being
                && membership.institution == *institution
                && membership_is_active(membership)
        })
    }
    pub fn validate(&self) -> Result<(), MembershipValidationError> {
        self.catalog
            .validate()
            .map_err(|_| MembershipValidationError::InvalidCatalog)?;
        for membership in &self.memberships {
            validate_membership(membership, &self.sponsorships, &self.memberships)?;
            if self.catalog.institution(&membership.institution).is_none() {
                return Err(MembershipValidationError::UnknownInstitution);
            }
            if let Some(role) = &membership.role_id
                && !self
                    .catalog
                    .roles
                    .iter()
                    .any(|entry| entry.id == *role && entry.institution == membership.institution)
            {
                return Err(MembershipValidationError::UnknownRole);
            }
            if let Some(group) = &membership.subgroup
                && !self
                    .catalog
                    .groups
                    .iter()
                    .any(|entry| entry.id == *group && entry.institution == membership.institution)
            {
                return Err(MembershipValidationError::UnknownGroup);
            }
        }
        Ok(())
    }
}

fn membership_is_active(membership: &InstitutionalMembership) -> bool {
    !matches!(
        membership.affiliation_state,
        AffiliationState::None
            | AffiliationState::Former
            | AffiliationState::Suspended
            | AffiliationState::Expelled
    )
}

fn grant_matches(
    grant: &AccessGrant,
    being: &InstitutionalBeingId,
    institution: &InstitutionId,
    site: &SiteId,
    zone: &ZoneId,
) -> bool {
    grant.active
        && grant.grantee == *being
        && grant.institution == *institution
        && grant.site.as_ref().is_none_or(|allowed| allowed == site)
        && grant.zone.as_ref().is_none_or(|allowed| allowed == zone)
}

const fn clearance_rank(clearance: ClearanceLevel) -> u8 {
    match clearance {
        ClearanceLevel::Basic => 0,
        ClearanceLevel::Restricted => 1,
        ClearanceLevel::Classified => 2,
        ClearanceLevel::Black => 3,
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalAction {
    pub actor: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub role: Option<MembershipRole>,
    pub verb: InstitutionalVerb,
    pub target: InstitutionalEntityId,
    pub authority: AuthoritySource,
    pub visibility: Visibility,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiateMemberCommand {
    pub candidate: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub approving_authority: InstitutionalBeingId,
    pub witnesses: Vec<InstitutionalBeingId>,
    pub sponsor: InstitutionalBeingId,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpelMemberCommand {
    pub member: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub authority: InstitutionalBeingId,
    pub reason: ExpulsionReason,
    pub protections_revoked: bool,
    pub access_revoked: bool,
    pub debts_preserved: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreMemberCommand {
    pub former_member: InstitutionalBeingId,
    pub institution: InstitutionId,
    pub approving_authority: InstitutionalBeingId,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MembershipValidationError {
    InvalidCatalog,
    UnknownInstitution,
    UnknownRole,
    UnknownGroup,
    CandidateHasFullAuthority,
    InitiatedMemberLacksRecognition,
    CandidateMissingSponsor,
    SponsorNotEligible,
    ExpelledStillActive,
    InvalidInitiation,
    CannotExpelNonMember,
}

pub fn validate_membership(
    membership: &InstitutionalMembership,
    sponsorships: &[Sponsorship],
    all_memberships: &[InstitutionalMembership],
) -> Result<(), MembershipValidationError> {
    if membership.affiliation_state == AffiliationState::Candidate
        && membership.role != MembershipRole::Candidate
    {
        return Err(MembershipValidationError::CandidateHasFullAuthority);
    }
    if matches!(
        membership.affiliation_state,
        AffiliationState::Initiated | AffiliationState::Senior
    ) && membership.internal_recognition < RecognitionLevel::Internal
    {
        return Err(MembershipValidationError::InitiatedMemberLacksRecognition);
    }
    if membership.affiliation_state == AffiliationState::Candidate
        && !sponsorships.iter().any(|entry| {
            entry.active
                && entry.candidate == membership.being
                && entry.institution == membership.institution
        })
    {
        return Err(MembershipValidationError::CandidateMissingSponsor);
    }
    if membership.affiliation_state == AffiliationState::Expelled && membership.ended_at.is_none() {
        return Err(MembershipValidationError::ExpelledStillActive);
    }
    for sponsorship in sponsorships
        .iter()
        .filter(|entry| entry.active && entry.institution == membership.institution)
    {
        let eligible = all_memberships.iter().any(|candidate| {
            candidate.being == sponsorship.sponsor
                && candidate.institution == sponsorship.institution
                && matches!(
                    candidate.affiliation_state,
                    AffiliationState::Initiated | AffiliationState::Senior
                )
                && !matches!(candidate.role, MembershipRole::Candidate)
        });
        if !eligible {
            return Err(MembershipValidationError::SponsorNotEligible);
        }
    }
    Ok(())
}

pub fn can_perform(
    membership: &InstitutionalMembership,
    verb: InstitutionalVerb,
    capabilities: &[InstitutionalCapability],
) -> bool {
    if matches!(
        membership.affiliation_state,
        AffiliationState::Expelled | AffiliationState::Former | AffiliationState::Suspended
    ) {
        return false;
    }
    capabilities.iter().any(|capability| {
        capability.institution == membership.institution
            && capability.action == verb
            && membership.affiliation_state >= capability.minimum_state
            && (capability.role.is_none() || capability.role == Some(membership.role))
    })
}

pub fn initiate_member(
    membership: &mut InstitutionalMembership,
    recruitment: &mut Recruitment,
    sponsor: &Sponsorship,
    witnesses: &[InstitutionalBeingId],
    at: WorldTimestamp,
) -> Result<InstitutionalEvent, MembershipValidationError> {
    if membership.affiliation_state != AffiliationState::Candidate
        || membership.role != MembershipRole::Candidate
        || !sponsor.active
        || recruitment.status != RecruitmentStatus::ApprovedForInitiation
        || recruitment.unresolved_betrayal
        || witnesses.is_empty()
    {
        return Err(MembershipValidationError::InvalidInitiation);
    }
    membership.affiliation_state = AffiliationState::Initiated;
    membership.role = MembershipRole::FullMember;
    membership.initiated_at = Some(at);
    membership.internal_recognition = RecognitionLevel::Internal;
    recruitment.status = RecruitmentStatus::Completed;
    Ok(InstitutionalEvent {
        id: EventId::new(format!("event.initiation.{}", membership.id.as_str()))
            .expect("derived from stable ID"),
        kind: InstitutionalEventKind::InitiationCompleted,
        institution: membership.institution.clone(),
        subject: membership.being.clone(),
        at,
    })
}

pub fn expel_member(
    membership: &mut InstitutionalMembership,
    at: WorldTimestamp,
) -> Result<InstitutionalEvent, MembershipValidationError> {
    if matches!(
        membership.affiliation_state,
        AffiliationState::None | AffiliationState::Candidate
    ) {
        return Err(MembershipValidationError::CannotExpelNonMember);
    }
    membership.affiliation_state = AffiliationState::Expelled;
    membership.ended_at = Some(at);
    Ok(InstitutionalEvent {
        id: EventId::new(format!("event.expulsion.{}", membership.id.as_str()))
            .expect("derived from stable ID"),
        kind: InstitutionalEventKind::MemberExpelled,
        institution: membership.institution.clone(),
        subject: membership.being.clone(),
        at,
    })
}

pub fn restore_member(
    membership: &mut InstitutionalMembership,
    at: WorldTimestamp,
) -> InstitutionalEvent {
    membership.affiliation_state = AffiliationState::Initiated;
    membership.role = MembershipRole::FullMember;
    membership.ended_at = None;
    InstitutionalEvent {
        id: EventId::new(format!("event.restoration.{}", membership.id.as_str()))
            .expect("derived from stable ID"),
        kind: InstitutionalEventKind::MemberRestored,
        institution: membership.institution.clone(),
        subject: membership.being.clone(),
        at,
    }
}
pub fn verify_affiliation_claim(claim: &AffiliationClaim) -> VerificationResult {
    match claim.truth_status {
        ClaimTruth::True => VerificationResult::Verified,
        ClaimTruth::False => VerificationResult::Rejected,
        ClaimTruth::FormerlyTrue => VerificationResult::FormerlyVerified,
        ClaimTruth::Disputed => VerificationResult::Disputed,
        ClaimTruth::Unknown | ClaimTruth::PartiallyTrue => VerificationResult::Uncertain,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::institution::Visibility;
    fn being(value: &str) -> InstitutionalBeingId {
        InstitutionalBeingId::new(value).unwrap()
    }
    fn membership() -> InstitutionalMembership {
        InstitutionalMembership {
            id: MembershipId::new("membership.test.candidate").unwrap(),
            being: being("being.test.candidate"),
            institution: InstitutionId::new("institution.test").unwrap(),
            role_id: None,
            role: MembershipRole::Candidate,
            affiliation_state: AffiliationState::Candidate,
            lineage: LineageStatus::Inherited,
            sponsor: Some(being("being.test.sponsor")),
            subgroup: None,
            joined_at: Some(1),
            initiated_at: None,
            ended_at: None,
            public_visibility: Visibility::Known,
            internal_recognition: RecognitionLevel::Provisional,
        }
    }
    #[test]
    fn sponsorship_and_initiation_preserve_the_required_transition() {
        let mut member = membership();
        let sponsor = Sponsorship {
            id: SponsorshipId::new("sponsorship.test").unwrap(),
            sponsor: being("being.test.sponsor"),
            candidate: member.being.clone(),
            institution: member.institution.clone(),
            active: true,
            started_at: 1,
            ended_at: None,
            liability: SponsorshipLiability::Social,
        };
        let sponsor_membership = InstitutionalMembership {
            id: MembershipId::new("membership.test.sponsor").unwrap(),
            being: sponsor.sponsor.clone(),
            institution: member.institution.clone(),
            role_id: None,
            role: MembershipRole::FullMember,
            affiliation_state: AffiliationState::Initiated,
            lineage: LineageStatus::None,
            sponsor: None,
            subgroup: None,
            joined_at: Some(1),
            initiated_at: Some(1),
            ended_at: None,
            public_visibility: Visibility::Known,
            internal_recognition: RecognitionLevel::Internal,
        };
        assert!(
            validate_membership(
                &member,
                std::slice::from_ref(&sponsor),
                &[member.clone(), sponsor_membership]
            )
            .is_ok()
        );
        let mut recruitment = Recruitment {
            candidate: member.being.clone(),
            institution: member.institution.clone(),
            sponsor: sponsor.sponsor.clone(),
            started_at: 1,
            status: RecruitmentStatus::ApprovedForInitiation,
            trials_completed: 1,
            unresolved_betrayal: false,
            obligations: vec![],
        };
        let event = initiate_member(
            &mut member,
            &mut recruitment,
            &sponsor,
            &[being("being.test.witness")],
            2,
        )
        .unwrap();
        assert_eq!(member.affiliation_state, AffiliationState::Initiated);
        assert_eq!(member.role, MembershipRole::FullMember);
        assert_eq!(event.kind, InstitutionalEventKind::InitiationCompleted);
    }
    #[test]
    fn expulsion_and_restoration_keep_history_and_access_state() {
        let mut member = membership();
        member.affiliation_state = AffiliationState::Initiated;
        member.role = MembershipRole::FullMember;
        member.internal_recognition = RecognitionLevel::Internal;
        let event = expel_member(&mut member, 3).unwrap();
        assert_eq!(event.kind, InstitutionalEventKind::MemberExpelled);
        assert_eq!(member.affiliation_state, AffiliationState::Expelled);
        assert_eq!(member.ended_at, Some(3));
        let restored = restore_member(&mut member, 4);
        assert_eq!(restored.kind, InstitutionalEventKind::MemberRestored);
        assert_eq!(member.affiliation_state, AffiliationState::Initiated);
        assert!(member.ended_at.is_none());
    }
    #[test]
    fn false_claim_is_not_verified() {
        let claim = AffiliationClaim {
            id: ClaimId::new("claim.test").unwrap(),
            claimant: being("being.test.claimant"),
            institution: InstitutionId::new("institution.test").unwrap(),
            claimed_state: AffiliationState::Initiated,
            claimed_role: Some(MembershipRole::FullMember),
            truth_status: ClaimTruth::False,
            visibility: Visibility::Known,
        };
        assert_eq!(
            verify_affiliation_claim(&claim),
            VerificationResult::Rejected
        );
    }
    #[test]
    fn scene_context_is_read_only_membership_and_relationship_data() {
        let member = membership();
        let state = InstitutionalWorldState {
            memberships: vec![member.clone()],
            ..Default::default()
        };
        let observer = being("being.test.observer");
        let context = state.scene_context_for(&observer, &member.being, &member.institution);
        assert_eq!(context.observer, observer);
        assert_eq!(context.subject_membership, Some(member));
        assert!(context.observer_membership.is_none());
        assert!(context.relationships.is_empty());
    }
    #[test]
    fn access_policy_uses_stable_membership_and_scoped_grants() {
        let member = membership();
        let site = SiteId::new("site.test").unwrap();
        let zone = ZoneId::new("zone.test").unwrap();
        let member_policy = AccessPolicy {
            matching: AccessRequirementMatch::Any,
            requirements: vec![AccessRequirement::InstitutionMembership(
                member.institution.clone(),
            )],
        };
        let state = InstitutionalWorldState {
            memberships: vec![member.clone()],
            ..Default::default()
        };
        assert_eq!(
            state.evaluate_access(
                &member.being,
                &member.institution,
                &site,
                &zone,
                &member_policy,
            ),
            AccessDecision::Allowed
        );
        let outsider = being("being.test.outsider");
        assert_eq!(
            state.evaluate_access(&outsider, &member.institution, &site, &zone, &member_policy,),
            AccessDecision::Denied
        );
        let grant_state = InstitutionalWorldState {
            access_grants: vec![AccessGrant {
                id: AccessGrantId::new("access-grant.test").unwrap(),
                grantee: outsider.clone(),
                institution: member.institution.clone(),
                site: Some(site.clone()),
                zone: Some(zone.clone()),
                clearance: None,
                active: true,
            }],
            ..Default::default()
        };
        let grant_policy = AccessPolicy {
            matching: AccessRequirementMatch::Any,
            requirements: vec![AccessRequirement::ExplicitGrant],
        };
        assert_eq!(
            grant_state.evaluate_access(
                &outsider,
                &member.institution,
                &site,
                &zone,
                &grant_policy,
            ),
            AccessDecision::Allowed
        );
    }
}
