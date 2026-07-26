use hollow_grove::gameplay::{
    CharacterCondition, DeepPressureAftermath, DeepPressureOutcomeId, DeepPressureOutcomeRecord,
    DeepPressurePersonId, DeepPressureSettlementChoice, DeepPressureState, MAX_PARTY_MEMBERS,
    PartyActionId, PartyActorId, PartyError, PartyMemberAvailability, PartyState,
    RecruitmentCandidateId, RecruitmentDecision, RecruitmentDecisionReason, RecruitmentPath,
    RelationshipMemory, WorldMapId,
};

fn outcome(choice: DeepPressureSettlementChoice) -> DeepPressureOutcomeRecord {
    DeepPressureOutcomeRecord {
        id: DeepPressureOutcomeId::for_choice(choice),
        choice,
        committed_by: "participant.boardwalk.deep-pressure-assembly".into(),
        player_support_is_nonbinding: true,
        four_house_acts: Vec::new(),
        recovery_bond: None,
        aftermath: DeepPressureAftermath {
            crew_care: 70,
            coast_recovery: 72,
            field_security: 74,
            basin_repair: 71,
            production_posture: "bounded test posture".into(),
            contested_certificate: false,
            unresolved_obligations: Vec::new(),
            visible_changes: Vec::new(),
        },
        refusal_and_limits: Vec::new(),
    }
}

fn relationship(person: DeepPressurePersonId, condition: CharacterCondition) -> RelationshipMemory {
    RelationshipMemory {
        person,
        affinity: 5,
        reliability: 5,
        condition,
        remembered_outcomes: vec!["Deep Pressure remembered".into()],
        unresolved_promises: Default::default(),
        boundaries: vec!["capable subject decides".into()],
        constitutional_bond: None,
    }
}

fn established_party() -> PartyState {
    let mut party = PartyState::new();
    party.establish_hueman("being-continuity.hueman").unwrap();
    party
}

#[test]
fn hueman_plus_five_is_the_hard_party_cap_and_duplicate_requests_cannot_badger() {
    let mut party = established_party();
    let settlement = outcome(DeepPressureSettlementChoice::SharedBurdenCompact);
    for candidate in RecruitmentCandidateId::ALL.into_iter().take(5) {
        let event = party
            .request_recruitment(
                candidate,
                candidate.accepted_paths()[0],
                &relationship(candidate.person(), CharacterCondition::Well),
                &settlement,
            )
            .unwrap();
        assert!(matches!(
            event,
            hollow_grove::gameplay::PartyEvent::RecruitmentDecided {
                member: Some(_),
                ..
            }
        ));
    }
    assert_eq!(party.member_count(), MAX_PARTY_MEMBERS);
    assert_eq!(
        party
            .request_recruitment(
                RecruitmentCandidateId::BreakwaterCurrentReader,
                RecruitmentPath::IndependentCompany,
                &relationship(
                    DeepPressurePersonId::TessBreakwater,
                    CharacterCondition::Well
                ),
                &settlement,
            )
            .unwrap_err(),
        PartyError::PartyAtCapacity
    );

    let existing = RecruitmentCandidateId::RiptidePressureKeeper;
    assert_eq!(
        party
            .request_recruitment(
                existing,
                RecruitmentPath::SharedWork,
                &relationship(existing.person(), CharacterCondition::Well),
                &settlement,
            )
            .unwrap_err(),
        PartyError::RecruitmentDecisionAlreadyRecorded(existing)
    );
}

#[test]
fn exposure_produces_a_persistent_debt_free_refusal() {
    let mut party = established_party();
    let candidate = RecruitmentCandidateId::RiptidePressureKeeper;
    let event = party
        .request_recruitment(
            candidate,
            RecruitmentPath::RecoveryFirst,
            &relationship(candidate.person(), CharacterCondition::Exposed),
            &outcome(DeepPressureSettlementChoice::CrewAndCoastRestitution),
        )
        .unwrap();
    let hollow_grove::gameplay::PartyEvent::RecruitmentDecided { record, member } = event else {
        panic!("recruitment decision event");
    };
    assert_eq!(record.decision, RecruitmentDecision::Declined);
    assert_eq!(
        record.reason,
        RecruitmentDecisionReason::ConditionRequiresCare
    );
    assert!(record.decision_is_persistent);
    assert!(!record.refusal_creates_debt);
    assert!(!record.may_be_asked_again);
    assert!(member.is_none());
    assert!(!party.is_recruited(candidate));
    assert_eq!(
        party
            .request_recruitment(
                candidate,
                RecruitmentPath::RecoveryFirst,
                &relationship(candidate.person(), CharacterCondition::Well),
                &outcome(DeepPressureSettlementChoice::SharedBurdenCompact),
            )
            .unwrap_err(),
        PartyError::RecruitmentDecisionAlreadyRecorded(candidate)
    );
}

#[test]
fn protected_refusal_changes_the_lawful_way_to_ask_without_forcing_acceptance() {
    let mut party = established_party();
    let candidate = RecruitmentCandidateId::FieldEngagementSteward;
    let event = party
        .request_recruitment(
            candidate,
            RecruitmentPath::SharedWork,
            &relationship(candidate.person(), CharacterCondition::Well),
            &outcome(DeepPressureSettlementChoice::ProtectedRefusal),
        )
        .unwrap();
    let hollow_grove::gameplay::PartyEvent::RecruitmentDecided { record, member } = event else {
        panic!("recruitment decision event");
    };
    assert_eq!(record.decision, RecruitmentDecision::Declined);
    assert_eq!(
        record.reason,
        RecruitmentDecisionReason::ProtectedRefusalRequiresIndependentCompany
    );
    assert!(member.is_none());
}

#[test]
fn exhausted_recruit_rests_before_leading_and_recovers_on_a_world_shift() {
    let mut party = established_party();
    let candidate = RecruitmentCandidateId::BasinCareRunner;
    party
        .request_recruitment(
            candidate,
            RecruitmentPath::RecoveryFirst,
            &relationship(candidate.person(), CharacterCondition::Exhausted),
            &outcome(DeepPressureSettlementChoice::CrewAndCoastRestitution),
        )
        .unwrap();
    let actor = PartyActorId::Companion(candidate);
    assert_eq!(
        party.member(actor).unwrap().availability,
        PartyMemberAvailability::Resting
    );
    assert_eq!(
        party.switch_lead(actor).unwrap_err(),
        PartyError::ActorUnavailable
    );
    let recovery = party.advance_shift().unwrap().expect("rest recovery event");
    assert!(matches!(
        recovery,
        hollow_grove::gameplay::PartyEvent::ShiftRecoveryApplied { .. }
    ));
    assert_eq!(
        party.member(actor).unwrap().availability,
        PartyMemberAvailability::Ready
    );
    party.switch_lead(actor).unwrap();
    assert_eq!(party.lead, actor);
}

#[test]
fn engagement_steward_action_is_location_bound_evidence_and_replays() {
    let mut party = established_party();
    let initial = party.clone();
    let candidate = RecruitmentCandidateId::FieldEngagementSteward;
    let settlement = outcome(DeepPressureSettlementChoice::SharedBurdenCompact);
    let relationship = relationship(candidate.person(), CharacterCondition::Well);
    let recruitment = party
        .request_recruitment(
            candidate,
            RecruitmentPath::SharedWork,
            &relationship,
            &settlement,
        )
        .unwrap();
    let actor = PartyActorId::Companion(candidate);
    let lead = party.switch_lead(actor).unwrap();
    let mut campaign = DeepPressureState::new();
    campaign.outcome = Some(settlement);
    let before_wrong_map = party.clone();
    assert!(matches!(
        party
            .use_action(
                actor,
                PartyActionId::ReadEngagementWork,
                WorldMapId::AuraBeachCoastalCommons,
                None,
                &campaign,
            )
            .unwrap_err(),
        PartyError::ActionLocationRequired { .. }
    ));
    assert_eq!(party, before_wrong_map);
    let action = party
        .use_action(
            actor,
            PartyActionId::ReadEngagementWork,
            WorldMapId::AuraFieldWorkingLand,
            None,
            &campaign,
        )
        .unwrap();
    assert!(party.field_actions[0].finding.contains("Engagement Farm"));
    assert!(!party.field_actions[0].creates_constitutional_decision);

    let mut replayed = initial;
    replayed.apply(&recruitment).unwrap();
    replayed.apply(&lead).unwrap();
    replayed.apply(&action).unwrap();
    assert_eq!(replayed, party);
}
