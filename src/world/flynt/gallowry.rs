//! Gallowry cultural interpretation of neutral institutional affiliation data.
//! It provides vocabulary, permissions, and context; it never chooses a tactic.

use std::collections::HashMap;

use super::{gallowry_id, gallowry_site_id};
use crate::falloutman::ResponsePresentationKind;
use crate::institution::*;
use crate::institution_affiliation::*;

fn stable<T>(value: &str, create: impl FnOnce(String) -> Option<T>) -> T {
    create(value.into()).expect("canonical stable id")
}
fn role(value: &str) -> RoleId {
    stable(value, |value| RoleId::new(value).ok())
}
fn zone(value: &str) -> ZoneId {
    stable(value, |value| ZoneId::new(value).ok())
}

pub fn noose_role_id() -> RoleId {
    role("role.flynt.gallowry.noose")
}
pub fn gallow_role_id() -> RoleId {
    role("role.flynt.gallowry.gallow")
}
pub fn sponsor_role_id() -> RoleId {
    role("role.flynt.gallowry.sponsor")
}
pub fn curator_role_id() -> RoleId {
    role("role.flynt.gallowry.curator")
}
pub fn broker_role_id() -> RoleId {
    role("role.flynt.gallowry.broker")
}
pub fn enforcer_role_id() -> RoleId {
    role("role.flynt.gallowry.enforcer")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GallowryStanding {
    Outsider,
    Noose,
    Associate,
    HalfHung,
    Hung,
    CutLoose,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GallowryPrestige {
    Unknown,
    Noted,
    Respected,
    WellHung,
    Celebrated,
    Feared,
    Legendary,
    Disgraced,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BraggadocioLevel {
    None,
    Dry,
    Playful,
    Bold,
    Theatrical,
    Excessive,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GallowryReputation {
    pub influence: i32,
    pub fear: i32,
    pub trust: i32,
    pub wealth: i32,
    pub style: i32,
    pub desirability: i32,
    pub artistic_status: i32,
    pub loyalty: i32,
    pub notoriety: i32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GallowryProfile {
    pub being: InstitutionalBeingId,
    pub membership: MembershipId,
    pub standing: GallowryStanding,
    pub prestige: GallowryPrestige,
    pub sponsor: Option<InstitutionalBeingId>,
    pub rope: Option<GroupId>,
    pub obligations: Vec<ObligationId>,
    pub protections: Vec<String>,
    pub public_reputation: GallowryReputation,
    pub internal_reputation: GallowryReputation,
    pub sexual_braggadocio: BraggadocioLevel,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CulturalConnotation {
    Institutional,
    Criminal,
    Prestigious,
    SexualInnuendo,
    Humorous,
    Threatening,
    Flirtatious,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstitutionalDialogueAct {
    ClaimAffiliation,
    DenyAffiliation,
    ChallengeAffiliation,
    VerifyAffiliation,
    RecognizeMember,
    RequestSponsor,
    OfferSponsorship,
    InvokeProtection,
    ThreatenRetaliation,
    OfferDebt,
    CollectDebt,
    FlirtThroughStatus,
    MockStatus,
    PraisePrestige,
    ExposeImpostor,
    DeclareExpulsion,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GallowryZone {
    ExhibitionFloor,
    Salon,
    HangingRooms,
    RopeArchive,
    BlackStudio,
    Scaffold,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexiconTerm {
    pub text: &'static str,
    pub connotations: Vec<CulturalConnotation>,
}
#[derive(Debug, Clone)]
pub struct InstitutionalLexicon {
    pub institution: InstitutionId,
    pub affiliation_terms: HashMap<AffiliationState, LexiconTerm>,
    pub role_terms: HashMap<MembershipRole, LexiconTerm>,
    pub lineage_terms: HashMap<LineageStatus, LexiconTerm>,
    pub action_terms: HashMap<InstitutionalVerb, LexiconTerm>,
    pub prestige_terms: HashMap<GallowryPrestige, LexiconTerm>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SocialReaction {
    pub fear: i32,
    pub respect: i32,
    pub attraction: i32,
    pub distrust: i32,
    pub hostility: i32,
    pub amusement: i32,
    pub deference: i32,
    pub curiosity: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GallowryDecisionContext {
    pub speaker_is_noose: bool,
    pub target_is_gallow: bool,
    pub claim_verification: Option<VerificationResult>,
    pub target_is_well_hung: bool,
    pub target_is_cut_loose: bool,
    pub sponsor_known: bool,
    pub obligation_open: bool,
}
/// A Flynt-facing projection of neutral scene facts. It supplies cultural
/// context and zone access, but contains no selected tactic or action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GallowrySceneContext {
    pub institutional: InstitutionalSceneContext,
    pub cultural: GallowryDecisionContext,
    pub subject_access: Vec<GallowryZoneAccess>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GallowryZoneAccess {
    pub zone: GallowryZone,
    pub allowed: bool,
}
/// A culturally rendered option for Falloutman. The caller supplies the already
/// selected typed act; this adapter only supplies player-facing wording.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GallowryFalloutmanOption {
    pub act: InstitutionalDialogueAct,
    pub visible_text: &'static str,
    pub presentation_kind: ResponsePresentationKind,
    pub enabled: bool,
}

pub fn present_dialogue_act(
    act: InstitutionalDialogueAct,
    context: &GallowryDecisionContext,
) -> GallowryFalloutmanOption {
    let (visible_text, enabled) = match act {
        InstitutionalDialogueAct::ClaimAffiliation => ("[Claim] \"I'm Hung.\"", true),
        InstitutionalDialogueAct::ChallengeAffiliation
        | InstitutionalDialogueAct::VerifyAffiliation => (
            "[Challenge] \"Then name your sponsor.\"",
            context.claim_verification != Some(VerificationResult::Verified),
        ),
        InstitutionalDialogueAct::FlirtThroughStatus => ("[Flirt] \"How well Hung?\"", true),
        InstitutionalDialogueAct::ExposeImpostor => (
            "[Expose] \"You're no Gallow.\"",
            context.claim_verification == Some(VerificationResult::Rejected),
        ),
        InstitutionalDialogueAct::DeclareExpulsion => (
            "[Recognize] \"They cut him loose.\"",
            context.target_is_cut_loose,
        ),
        InstitutionalDialogueAct::RecognizeMember => (
            "[Recognize] \"I know you're Hung.\"",
            context.target_is_gallow,
        ),
        InstitutionalDialogueAct::InvokeProtection => (
            "[Threaten] \"Protection cuts both ways.\"",
            context.target_is_gallow,
        ),
        InstitutionalDialogueAct::DenyAffiliation => {
            ("[Deny] \"That claim carries no weight here.\"", true)
        }
        InstitutionalDialogueAct::RequestSponsor => ("[Verify] \"Name your sponsor.\"", true),
        InstitutionalDialogueAct::OfferSponsorship => ("[Offer] \"I can sponsor you.\"", true),
        InstitutionalDialogueAct::ThreatenRetaliation => (
            "[Threaten] \"The Gallowry will hear of this.\"",
            context.target_is_gallow,
        ),
        InstitutionalDialogueAct::OfferDebt => ("[Bargain] \"Put it on my debt.\"", true),
        InstitutionalDialogueAct::CollectDebt => {
            ("[Collect] \"Your debt is due.\"", context.obligation_open)
        }
        InstitutionalDialogueAct::MockStatus => {
            ("[Mock] \"That's all the hanging you can manage?\"", true)
        }
        InstitutionalDialogueAct::PraisePrestige => (
            "[Praise] \"You're well Hung.\"",
            context.target_is_well_hung,
        ),
    };
    GallowryFalloutmanOption {
        act,
        visible_text,
        presentation_kind: ResponsePresentationKind::Spoken,
        enabled,
    }
}

pub fn canonical_lexicon() -> InstitutionalLexicon {
    let mut affiliation_terms = HashMap::new();
    affiliation_terms.insert(
        AffiliationState::Candidate,
        term("Noose", &[CulturalConnotation::Institutional]),
    );
    affiliation_terms.insert(
        AffiliationState::RecognizedAssociate,
        term(
            "Half-Hung",
            &[
                CulturalConnotation::Institutional,
                CulturalConnotation::SexualInnuendo,
            ],
        ),
    );
    affiliation_terms.insert(
        AffiliationState::Initiated,
        term(
            "Hung",
            &[
                CulturalConnotation::Institutional,
                CulturalConnotation::Criminal,
                CulturalConnotation::SexualInnuendo,
            ],
        ),
    );
    affiliation_terms.insert(
        AffiliationState::Expelled,
        term(
            "Cut Loose",
            &[
                CulturalConnotation::Institutional,
                CulturalConnotation::Threatening,
            ],
        ),
    );
    let mut role_terms = HashMap::new();
    role_terms.insert(
        MembershipRole::Candidate,
        term("Noose", &[CulturalConnotation::Institutional]),
    );
    role_terms.insert(
        MembershipRole::FullMember,
        term("Gallow", &[CulturalConnotation::Institutional]),
    );
    let mut lineage_terms = HashMap::new();
    lineage_terms.insert(
        LineageStatus::Inherited,
        term(
            "Born Hung",
            &[
                CulturalConnotation::Institutional,
                CulturalConnotation::SexualInnuendo,
            ],
        ),
    );
    let mut action_terms = HashMap::new();
    action_terms.insert(
        InstitutionalVerb::Initiate,
        term("Hang", &[CulturalConnotation::Institutional]),
    );
    action_terms.insert(
        InstitutionalVerb::Expel,
        term("Cut Loose", &[CulturalConnotation::Institutional]),
    );
    action_terms.insert(
        InstitutionalVerb::ClaimAffiliation,
        term(
            "I'm Hung.",
            &[
                CulturalConnotation::Institutional,
                CulturalConnotation::SexualInnuendo,
            ],
        ),
    );
    let mut prestige_terms = HashMap::new();
    prestige_terms.insert(
        GallowryPrestige::WellHung,
        term(
            "Well Hung",
            &[
                CulturalConnotation::Prestigious,
                CulturalConnotation::SexualInnuendo,
                CulturalConnotation::Flirtatious,
            ],
        ),
    );
    InstitutionalLexicon {
        institution: gallowry_id(),
        affiliation_terms,
        role_terms,
        lineage_terms,
        action_terms,
        prestige_terms,
    }
}
fn term(text: &'static str, connotations: &[CulturalConnotation]) -> LexiconTerm {
    LexiconTerm {
        text,
        connotations: connotations.to_vec(),
    }
}
pub fn derive_prestige(reputation: &GallowryReputation) -> GallowryPrestige {
    if reputation.influence >= 8 && reputation.trust >= 6
        || reputation.notoriety >= 8 && reputation.style >= 6
        || reputation.desirability >= 8 && reputation.artistic_status >= 6
        || reputation.fear >= 8 && reputation.wealth >= 6
    {
        GallowryPrestige::WellHung
    } else if reputation.fear >= 9 {
        GallowryPrestige::Feared
    } else if reputation.influence >= 5 || reputation.style >= 5 {
        GallowryPrestige::Respected
    } else {
        GallowryPrestige::Unknown
    }
}

#[derive(Debug, Clone, Default)]
pub struct GallowryDomain {
    pub state: InstitutionalWorldState,
    pub recruitments: Vec<Recruitment>,
    pub profiles: Vec<GallowryProfile>,
    pub claims: Vec<AffiliationClaim>,
    pub capabilities: Vec<InstitutionalCapability>,
}
impl GallowryDomain {
    pub fn membership_of(&self, being: &InstitutionalBeingId) -> Option<&InstitutionalMembership> {
        self.state.membership_of(being, &gallowry_id())
    }
    pub fn gallowry_profile(&self, being: &InstitutionalBeingId) -> Option<&GallowryProfile> {
        self.profiles.iter().find(|entry| &entry.being == being)
    }
    pub fn is_noose(&self, being: &InstitutionalBeingId) -> bool {
        self.membership_of(being)
            .is_some_and(|entry| entry.affiliation_state == AffiliationState::Candidate)
    }
    pub fn is_gallow(&self, being: &InstitutionalBeingId) -> bool {
        self.membership_of(being).is_some_and(|entry| {
            entry.affiliation_state == AffiliationState::Initiated
                && entry.role == MembershipRole::FullMember
        })
    }
    pub fn is_hung(&self, being: &InstitutionalBeingId) -> bool {
        self.membership_of(being).is_some_and(|entry| {
            matches!(
                entry.affiliation_state,
                AffiliationState::Initiated | AffiliationState::Senior
            )
        })
    }
    pub fn is_cut_loose(&self, being: &InstitutionalBeingId) -> bool {
        self.membership_of(being)
            .is_some_and(|entry| entry.affiliation_state == AffiliationState::Expelled)
    }
    pub fn is_born_hung(&self, being: &InstitutionalBeingId) -> bool {
        self.membership_of(being)
            .is_some_and(|entry| entry.lineage == LineageStatus::Inherited)
    }
    pub fn is_well_hung(&self, being: &InstitutionalBeingId) -> bool {
        self.gallowry_profile(being)
            .is_some_and(|entry| entry.prestige == GallowryPrestige::WellHung)
    }
    pub fn sponsor_of(&self, candidate: &InstitutionalBeingId) -> Option<&InstitutionalBeingId> {
        self.state
            .sponsorships
            .iter()
            .find(|entry| entry.active && &entry.candidate == candidate)
            .map(|entry| &entry.sponsor)
    }
    pub fn can_perform(&self, being: &InstitutionalBeingId, verb: InstitutionalVerb) -> bool {
        self.membership_of(being).is_some_and(|entry| {
            crate::institution_affiliation::can_perform(entry, verb, &self.capabilities)
        })
    }
    pub fn can_access(&self, being: &InstitutionalBeingId, site: &SiteId, zone: &ZoneId) -> bool {
        if site != &gallowry_site_id() {
            return false;
        }
        let Some(member) = self.membership_of(being) else {
            return zone == &zone_for(GallowryZone::ExhibitionFloor);
        };
        if matches!(
            member.affiliation_state,
            AffiliationState::Expelled | AffiliationState::Former | AffiliationState::Suspended
        ) {
            return zone == &zone_for(GallowryZone::ExhibitionFloor);
        }
        match zone.as_str() {
            "zone.flynt.gallowry.exhibition-floor" => true,
            "zone.flynt.gallowry.salon" => {
                member.affiliation_state >= AffiliationState::RecognizedAssociate
            }
            "zone.flynt.gallowry.hanging-rooms" => {
                member.affiliation_state >= AffiliationState::Initiated
            }
            "zone.flynt.gallowry.rope-archive" => {
                member.role != MembershipRole::Candidate
                    && member.affiliation_state >= AffiliationState::Initiated
            }
            "zone.flynt.gallowry.black-studio" => matches!(
                member.role,
                MembershipRole::Officer | MembershipRole::Elder | MembershipRole::Leader
            ),
            "zone.flynt.gallowry.scaffold" => {
                matches!(member.role, MembershipRole::Elder | MembershipRole::Leader)
            }
            _ => false,
        }
    }
    pub fn verify_affiliation_claim(
        &self,
        _observer: &InstitutionalBeingId,
        claim: &AffiliationClaim,
    ) -> VerificationResult {
        crate::institution_affiliation::verify_affiliation_claim(claim)
    }
    pub fn social_reaction_to_claim(&self, claim: &AffiliationClaim) -> SocialReaction {
        match self.verify_affiliation_claim(&claim.claimant, claim) {
            VerificationResult::Verified => SocialReaction {
                respect: 2,
                deference: 1,
                ..Default::default()
            },
            VerificationResult::Rejected => SocialReaction {
                distrust: 3,
                hostility: 1,
                ..Default::default()
            },
            VerificationResult::FormerlyVerified => SocialReaction {
                distrust: 2,
                curiosity: 1,
                ..Default::default()
            },
            _ => SocialReaction {
                curiosity: 1,
                ..Default::default()
            },
        }
    }
    pub fn decision_context_for(
        &self,
        speaker: &InstitutionalBeingId,
        target: &InstitutionalBeingId,
        claim: Option<&AffiliationClaim>,
    ) -> GallowryDecisionContext {
        GallowryDecisionContext {
            speaker_is_noose: self.is_noose(speaker),
            target_is_gallow: self.is_gallow(target),
            claim_verification: claim.map(|entry| self.verify_affiliation_claim(speaker, entry)),
            target_is_well_hung: self.is_well_hung(target),
            target_is_cut_loose: self.is_cut_loose(target),
            sponsor_known: self.sponsor_of(speaker).is_some(),
            obligation_open: self.state.obligations.iter().any(|entry| {
                entry.status != ObligationStatus::Settled
                    && entry.debtor == InstitutionalEntityId::Being(speaker.clone())
            }),
        }
    }
    /// Builds read-only scene input for Stanislavski and presentation adapters.
    /// Callers remain responsible for choosing objectives and tactics.
    pub fn scene_context_for(
        &self,
        speaker: &InstitutionalBeingId,
        target: &InstitutionalBeingId,
        claim: Option<&AffiliationClaim>,
    ) -> GallowrySceneContext {
        let subject_access = [
            GallowryZone::ExhibitionFloor,
            GallowryZone::Salon,
            GallowryZone::HangingRooms,
            GallowryZone::RopeArchive,
            GallowryZone::BlackStudio,
            GallowryZone::Scaffold,
        ]
        .into_iter()
        .map(|zone| GallowryZoneAccess {
            allowed: self.can_access(target, &gallowry_site_id(), &zone_for(zone)),
            zone,
        })
        .collect();
        GallowrySceneContext {
            institutional: self
                .state
                .scene_context_for(speaker, target, &gallowry_id()),
            cultural: self.decision_context_for(speaker, target, claim),
            subject_access,
        }
    }
}
pub fn zone_for(zone_kind: GallowryZone) -> ZoneId {
    zone(match zone_kind {
        GallowryZone::ExhibitionFloor => "zone.flynt.gallowry.exhibition-floor",
        GallowryZone::Salon => "zone.flynt.gallowry.salon",
        GallowryZone::HangingRooms => "zone.flynt.gallowry.hanging-rooms",
        GallowryZone::RopeArchive => "zone.flynt.gallowry.rope-archive",
        GallowryZone::BlackStudio => "zone.flynt.gallowry.black-studio",
        GallowryZone::Scaffold => "zone.flynt.gallowry.scaffold",
    })
}
pub fn canonical_capabilities() -> Vec<InstitutionalCapability> {
    let institution = gallowry_id();
    vec![
        cap(
            institution.clone(),
            MembershipRole::Candidate,
            AffiliationState::Candidate,
            InstitutionalVerb::Recognize,
        ),
        cap(
            institution.clone(),
            MembershipRole::Candidate,
            AffiliationState::Candidate,
            InstitutionalVerb::Recruit,
        ),
        cap(
            institution.clone(),
            MembershipRole::FullMember,
            AffiliationState::Initiated,
            InstitutionalVerb::ClaimAffiliation,
        ),
        cap(
            institution.clone(),
            MembershipRole::FullMember,
            AffiliationState::Initiated,
            InstitutionalVerb::Sponsor,
        ),
        cap(
            institution.clone(),
            MembershipRole::FullMember,
            AffiliationState::Initiated,
            InstitutionalVerb::Protect,
        ),
        cap(
            institution.clone(),
            MembershipRole::FullMember,
            AffiliationState::Initiated,
            InstitutionalVerb::Broker,
        ),
        cap(
            institution.clone(),
            MembershipRole::Elder,
            AffiliationState::Senior,
            InstitutionalVerb::Initiate,
        ),
        cap(
            institution.clone(),
            MembershipRole::Leader,
            AffiliationState::Senior,
            InstitutionalVerb::Expel,
        ),
        cap(
            institution,
            MembershipRole::Leader,
            AffiliationState::Senior,
            InstitutionalVerb::Restore,
        ),
    ]
}
fn cap(
    institution: InstitutionId,
    role: MembershipRole,
    minimum_state: AffiliationState,
    action: InstitutionalVerb,
) -> InstitutionalCapability {
    InstitutionalCapability {
        institution,
        role: Some(role),
        minimum_state,
        action,
        target_scope: TargetScope::Member,
        requires_witness: false,
        requires_sponsor: false,
        requires_vote: false,
    }
}
pub fn dialogue_act_for(line: &str) -> Option<InstitutionalDialogueAct> {
    match line {
        "I'm Hung." => Some(InstitutionalDialogueAct::ClaimAffiliation),
        "How well Hung?" => Some(InstitutionalDialogueAct::FlirtThroughStatus),
        "You're no Gallow." => Some(InstitutionalDialogueAct::ExposeImpostor),
        "Name your sponsor." => Some(InstitutionalDialogueAct::VerifyAffiliation),
        "They cut him loose." => Some(InstitutionalDialogueAct::DeclareExpulsion),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn being(value: &str) -> InstitutionalBeingId {
        InstitutionalBeingId::new(value).unwrap()
    }
    fn membership(
        state: AffiliationState,
        role: MembershipRole,
        lineage: LineageStatus,
    ) -> InstitutionalMembership {
        InstitutionalMembership {
            id: MembershipId::new("membership.flynt.test").unwrap(),
            being: being("being.flynt.test"),
            institution: gallowry_id(),
            role_id: None,
            role,
            affiliation_state: state,
            lineage,
            sponsor: None,
            subgroup: None,
            joined_at: Some(1),
            initiated_at: None,
            ended_at: None,
            public_visibility: Visibility::Known,
            internal_recognition: RecognitionLevel::Internal,
        }
    }
    #[test]
    fn lexicon_maps_canon_without_leaking_words_to_core() {
        let lexicon = canonical_lexicon();
        assert_eq!(
            lexicon.affiliation_terms[&AffiliationState::Candidate].text,
            "Noose"
        );
        assert_eq!(
            lexicon.role_terms[&MembershipRole::FullMember].text,
            "Gallow"
        );
        assert_eq!(
            lexicon.prestige_terms[&GallowryPrestige::WellHung].text,
            "Well Hung"
        );
    }
    #[test]
    fn noose_is_not_a_gallow_and_cannot_initiate() {
        let mut domain = GallowryDomain {
            state: InstitutionalWorldState {
                memberships: vec![membership(
                    AffiliationState::Candidate,
                    MembershipRole::Candidate,
                    LineageStatus::None,
                )],
                ..Default::default()
            },
            capabilities: canonical_capabilities(),
            ..Default::default()
        };
        let candidate = domain.state.memberships[0].being.clone();
        assert!(domain.is_noose(&candidate));
        assert!(!domain.is_gallow(&candidate));
        assert!(!domain.can_perform(&candidate, InstitutionalVerb::Initiate));
        domain.state.memberships[0].lineage = LineageStatus::Inherited;
        assert!(domain.is_born_hung(&candidate));
        assert!(!domain.is_gallow(&candidate));
    }
    #[test]
    fn prestige_is_reputation_not_authority() {
        let rep = GallowryReputation {
            influence: 8,
            trust: 6,
            ..Default::default()
        };
        assert_eq!(derive_prestige(&rep), GallowryPrestige::WellHung);
        let associate = membership(
            AffiliationState::RecognizedAssociate,
            MembershipRole::Associate,
            LineageStatus::None,
        );
        let domain = GallowryDomain {
            state: InstitutionalWorldState {
                memberships: vec![associate.clone()],
                ..Default::default()
            },
            profiles: vec![GallowryProfile {
                being: associate.being.clone(),
                membership: associate.id.clone(),
                standing: GallowryStanding::HalfHung,
                prestige: GallowryPrestige::WellHung,
                sponsor: None,
                rope: None,
                obligations: vec![],
                protections: vec![],
                public_reputation: rep.clone(),
                internal_reputation: rep,
                sexual_braggadocio: BraggadocioLevel::Bold,
            }],
            capabilities: canonical_capabilities(),
            ..Default::default()
        };
        assert!(domain.is_well_hung(&associate.being));
        assert!(!domain.can_perform(&associate.being, InstitutionalVerb::Expel));
    }
    #[test]
    fn claims_access_and_dialogue_are_structured() {
        let member = membership(
            AffiliationState::Initiated,
            MembershipRole::FullMember,
            LineageStatus::None,
        );
        let claim = AffiliationClaim {
            id: ClaimId::new("claim.flynt.test").unwrap(),
            claimant: member.being.clone(),
            institution: gallowry_id(),
            claimed_state: AffiliationState::Initiated,
            claimed_role: Some(MembershipRole::FullMember),
            truth_status: ClaimTruth::True,
            visibility: Visibility::Known,
        };
        let domain = GallowryDomain {
            state: InstitutionalWorldState {
                memberships: vec![member.clone()],
                ..Default::default()
            },
            capabilities: canonical_capabilities(),
            ..Default::default()
        };
        assert!(domain.can_perform(&member.being, InstitutionalVerb::ClaimAffiliation));
        assert!(domain.can_access(
            &member.being,
            &gallowry_site_id(),
            &zone_for(GallowryZone::HangingRooms)
        ));
        assert_eq!(
            domain.verify_affiliation_claim(&member.being, &claim),
            VerificationResult::Verified
        );
        assert_eq!(
            dialogue_act_for("I'm Hung."),
            Some(InstitutionalDialogueAct::ClaimAffiliation)
        );
    }
    #[test]
    fn falloutman_adapter_renders_but_does_not_choose_the_act() {
        let context = GallowryDecisionContext {
            claim_verification: Some(VerificationResult::Rejected),
            ..Default::default()
        };
        let option = present_dialogue_act(InstitutionalDialogueAct::ExposeImpostor, &context);
        assert_eq!(option.visible_text, "[Expose] \"You're no Gallow.\"");
        assert!(option.enabled);
        assert_eq!(option.presentation_kind, ResponsePresentationKind::Spoken);
    }
    #[test]
    fn scene_context_projects_gallowry_access_without_selecting_a_tactic() {
        let member = membership(
            AffiliationState::Initiated,
            MembershipRole::FullMember,
            LineageStatus::None,
        );
        let domain = GallowryDomain {
            state: InstitutionalWorldState {
                memberships: vec![member.clone()],
                ..Default::default()
            },
            ..Default::default()
        };
        let context = domain.scene_context_for(&being("being.flynt.observer"), &member.being, None);
        assert_eq!(context.institutional.subject_membership, Some(member));
        assert!(
            context
                .subject_access
                .iter()
                .any(|entry| entry.zone == GallowryZone::HangingRooms && entry.allowed)
        );
        assert!(
            context
                .subject_access
                .iter()
                .any(|entry| entry.zone == GallowryZone::Scaffold && !entry.allowed)
        );
    }
}
