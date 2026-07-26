use std::collections::BTreeSet;

use hollow_grove::gameplay::{
    MAX_PARTY_COMPANIONS, MAX_PARTY_MEMBERS, PartyActionId, RecruitmentCandidateId,
};
use hollow_grove::world::aura_field::{
    AuraFieldFacilityId, AuraFieldFacilityKind, canonical_aura_field,
};

fn main() {
    let candidates = RecruitmentCandidateId::ALL;
    let candidate_ids = candidates
        .iter()
        .map(|candidate| candidate.stable_id())
        .collect::<BTreeSet<_>>();
    let continuities = candidates
        .iter()
        .map(|candidate| candidate.continuity_id())
        .collect::<BTreeSet<_>>();
    let people = candidates
        .iter()
        .map(|candidate| candidate.person())
        .collect::<BTreeSet<_>>();
    let actions = candidates
        .iter()
        .map(|candidate| candidate.action())
        .collect::<BTreeSet<_>>();
    assert_eq!(candidate_ids.len(), candidates.len());
    assert_eq!(continuities.len(), candidates.len());
    assert_eq!(people.len(), candidates.len());
    assert_eq!(actions.len(), PartyActionId::ALL.len());
    assert!(
        candidates
            .iter()
            .all(|candidate| candidate.accepted_paths().len() == 2)
    );

    let field = canonical_aura_field().expect("canonical Aura Field");
    let engagement = field
        .facility(AuraFieldFacilityId::EngagementFarm)
        .expect("Engagement Farm");
    assert_eq!(engagement.kind, AuraFieldFacilityKind::EngagementFarm);
    assert!(engagement.function.contains("leave-without-debt"));

    println!("party/recruitment audit: pass");
    println!("party members: {MAX_PARTY_MEMBERS} total");
    println!("companions: {MAX_PARTY_COMPANIONS} active slots");
    println!("authored candidates: {}", candidates.len());
    println!("field actions: {}", actions.len());
    println!("engagement farm: {}", engagement.id.stable_id());
}
