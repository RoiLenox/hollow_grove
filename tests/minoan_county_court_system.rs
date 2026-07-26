use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::IdentityId;
use hollow_grove::world::minoan_court::*;
use hollow_grove::world::stonebend::second_pass::{
    DomainEvidenceAuthority, GateScope, GateScopeRecognition, StonebendConstitutionalPower,
    StonebendGateFacing, StonebendTitleCore, TitleScopeDisposition,
};
use hollow_grove::world::stonebend::{EvidenceRecordId, NameRecordId, SealRecordId, TitleRecordId};

fn id(value: impl Into<String>) -> IdentityId {
    IdentityId::new(value).unwrap()
}

fn evidence(value: impl Into<String>) -> EvidenceRecordId {
    EvidenceRecordId::new(value).unwrap()
}

fn title(value: impl Into<String>) -> TitleRecordId {
    TitleRecordId::new(value).unwrap()
}

fn seal(value: impl Into<String>) -> SealRecordId {
    SealRecordId::new(value).unwrap()
}

fn stage_slug(stage: JudicialStage) -> &'static str {
    match stage {
        JudicialStage::Conciliation => "conciliation",
        JudicialStage::FirstHearing => "first-hearing",
        JudicialStage::Appeal => "appeal",
        JudicialStage::ConstitutionalReview => "constitutional-review",
        JudicialStage::Restitution => "restitution",
    }
}

fn stage(case: &IdentityId, stage: JudicialStage, cycle: u32, suffix: &str) -> JudicialStageRecord {
    JudicialStageRecord {
        identity: id(format!(
            "stage.{}.{}.{}.{}",
            case.as_str(),
            cycle,
            stage_slug(stage),
            suffix
        )),
        case: case.clone(),
        stage,
        cycle,
        evidence: vec![evidence(format!(
            "evidence.{}.{}.{}",
            stage_slug(stage),
            cycle,
            suffix
        ))],
    }
}

fn party(value: &str, standing: StandingGround) -> CaseParty {
    CaseParty {
        identity: id(value),
        standing: BTreeSet::from([standing]),
        represented_by: None,
    }
}

fn case_with(
    case_id: &str,
    jurisdictions: BTreeSet<CourtJurisdiction>,
    policy: CourtCasePolicy,
) -> CourtCase {
    CourtCase::new(
        id(case_id),
        vec![
            party("being.case.claimant", StandingGround::DirectInjury),
            party(
                "being.case.respondent",
                StandingGround::TitleOrScopeInterest,
            ),
        ],
        jurisdictions,
        BTreeSet::from([id("claim.case.primary")]),
        BTreeSet::from([title("title.case.subject")]),
        policy,
    )
    .unwrap()
}

fn submit(
    case: &mut CourtCase,
    submission_id: &str,
    source: DomainEvidenceSource,
    jurisdiction: CourtJurisdiction,
) {
    case.submit_evidence(EvidenceSubmission {
        identity: id(submission_id),
        source,
        jurisdiction,
        records: vec![evidence(format!("evidence.{submission_id}"))],
        authenticated_by: id(format!("authority.{submission_id}")),
        description: format!("authenticated {submission_id}"),
    })
    .unwrap();
}

fn basic_conciliation(case: &IdentityId, suffix: &str) -> ConciliationRecord {
    ConciliationRecord {
        identity: id(format!("conciliation.{}.{}", case.as_str(), suffix)),
        case: case.clone(),
        relationships: vec!["continuing lawful relation".into()],
        agreed_facts: vec!["a boundary exists".into()],
        disputed_facts: vec!["whether it was crossed".into()],
        immediate_risks: vec!["continued burden".into()],
        affected_titles: BTreeSet::from([title("title.case.subject")]),
        affected_yield: vec![evidence("evidence.yield.conciliation")],
        voluntary_repair_possible: false,
        settlement: None,
    }
}

fn basic_remedy(case: &IdentityId, suffix: &str, kind: RemedyKind) -> Remedy {
    Remedy {
        identity: id(format!("remedy.{}.{}", case.as_str(), suffix)),
        case: case.clone(),
        kind,
        target: JudgmentTarget::PublicRecord(id(format!("record.{suffix}"))),
        responsible_institution: ResponsibleInstitution::Stonebend,
        harmed_party_or_community: id("being.case.claimant"),
        ordered_action: "perform the bounded correction".into(),
        completion_condition: "verified delivery reaches the harmed party".into(),
        evidence: vec![evidence(format!("evidence.remedy.{suffix}"))],
    }
}

fn basic_hearing(
    case: &IdentityId,
    suffix: &str,
    submission: &str,
    remedy: &IdentityId,
) -> FirstHearingRecord {
    let finding = FirstHearingFinding {
        identity: id(format!("finding.{}.{}", case.as_str(), suffix)),
        kind: FindingKind::Fact,
        statement: "the authenticated boundary was crossed".into(),
        evidence: vec![evidence(format!("evidence.finding.{suffix}"))],
    };
    let judgment = Judgment {
        identity: id(format!("judgment.{}.{}", case.as_str(), suffix)),
        case: case.clone(),
        findings: BTreeSet::from([finding.identity.clone()]),
        legal_conclusions: vec!["a targeted correction is required".into()],
        targets: BTreeSet::from([JudgmentTarget::PublicRecord(id(format!("record.{suffix}")))]),
        remedies: BTreeSet::from([remedy.clone()]),
        constitutional_effect: JudgmentConstitutionalEffect::TargetedRemedy,
        evidence: vec![evidence(format!("evidence.judgment.{suffix}"))],
        court_executes_remedy: false,
        court_removes_principal_power: false,
    };
    FirstHearingRecord {
        identity: id(format!("hearing.{}.{}", case.as_str(), suffix)),
        case: case.clone(),
        standing_confirmed: BTreeSet::from([
            id("being.case.claimant"),
            id("being.case.respondent"),
        ]),
        jurisdictions_reviewed: BTreeSet::from([CourtJurisdiction::Stonebend]),
        evidence_considered: BTreeSet::from([id(submission)]),
        testimony_considered: BTreeSet::new(),
        findings: vec![finding],
        judgment: Some(judgment),
        dismissed: false,
        referral: None,
    }
}

fn basic_appeal(case: &IdentityId, suffix: &str) -> AppealRecord {
    AppealRecord {
        identity: id(format!("appeal.{}.{}", case.as_str(), suffix)),
        case: case.clone(),
        challenged_judgment: id(format!("judgment.{}.{}", case.as_str(), suffix)),
        grounds: BTreeSet::from([AppealGround::LegalError]),
        record_evidence: vec![evidence(format!("evidence.appeal.{suffix}"))],
        review_standard: "bounded legal error review".into(),
        disposition: AppealDisposition::Affirmed,
        effect_on_judgment: "targeted judgment remains effective".into(),
        retries_all_facts: false,
        stay: None,
    }
}

fn basic_review(case: &IdentityId, suffix: &str) -> ConstitutionalReviewRecord {
    ConstitutionalReviewRecord {
        identity: id(format!("review.{}.{}", case.as_str(), suffix)),
        case: case.clone(),
        grounds: BTreeSet::from([ConstitutionalReviewGround::HouseBoundary]),
        evidence: vec![evidence(format!("evidence.review.{suffix}"))],
        disposition: ConstitutionalReviewDisposition::ConstitutionallyValid,
        retries_settled_facts: false,
        ratifies_amendment: false,
        amends_constitution: false,
        bears_diamond: false,
        forges_stonebend_claim: false,
        replaces_proliteriate: false,
        transfers_house_authority: false,
    }
}

fn equilibrium(holds: bool) -> EquilibriumAssessment {
    EquilibriumAssessment {
        lawful_boundary_restored_or_clarified: holds,
        remedy_reached_intended_subject: holds,
        continuing_burden_lawfully_assigned: holds,
        historical_record_accurate: true,
        hidden_constitutional_violation: false,
        immediate_same_case_harm_unresolved: !holds,
        remaining_burden: if holds {
            "lawfully assigned continuing burden"
        } else {
            "unresolved immediate burden"
        }
        .into(),
    }
}

fn basic_restitution(
    case: &IdentityId,
    suffix: &str,
    remedy: &IdentityId,
    disposition: RestitutionDisposition,
    cycle: u32,
) -> RestitutionRecord {
    RestitutionRecord {
        identity: id(format!(
            "restitution.{}.{}.{}",
            case.as_str(),
            cycle,
            suffix
        )),
        case: case.clone(),
        judgment: id(format!("judgment.{}.{}", case.as_str(), suffix)),
        responsible_institutions: BTreeSet::from([ResponsibleInstitution::Stonebend]),
        harmed_parties_or_communities: BTreeSet::from([id("being.case.claimant")]),
        remedies: BTreeSet::from([remedy.clone()]),
        delivery_evidence: vec![evidence(format!("evidence.delivery.{suffix}.{cycle}"))],
        completion_evidence: disposition
            .closes_case()
            .then(|| evidence(format!("evidence.completion.{suffix}.{cycle}")))
            .into_iter()
            .collect(),
        remaining_burden: "historically recorded burden".into(),
        unintended_effects: Vec::new(),
        yield_evidence: vec![evidence(format!("evidence.yield.{suffix}.{cycle}"))],
        equilibrium: equilibrium(disposition.closes_case()),
        disposition,
        cycle,
    }
}

fn complete_case(case_id: &str) -> CourtCase {
    let mut case = case_with(
        case_id,
        BTreeSet::from([
            CourtJurisdiction::Stonebend,
            CourtJurisdiction::Constitutional,
        ]),
        CourtCasePolicy::default(),
    );
    let case_identity = case.identity.clone();
    submit(
        &mut case,
        "submission.stonebend.primary",
        DomainEvidenceSource::House(House::Stonebend),
        CourtJurisdiction::Stonebend,
    );
    case.record_conciliation(
        stage(&case_identity, JudicialStage::Conciliation, 0, "primary"),
        basic_conciliation(&case_identity, "primary"),
    )
    .unwrap();
    let remedy = basic_remedy(
        &case_identity,
        "primary",
        RemedyKind::PublicRecordCorrection,
    );
    let remedy_id = remedy.identity.clone();
    case.add_remedy(remedy).unwrap();
    case.record_first_hearing(
        stage(&case_identity, JudicialStage::FirstHearing, 0, "primary"),
        basic_hearing(
            &case_identity,
            "primary",
            "submission.stonebend.primary",
            &remedy_id,
        ),
    )
    .unwrap();
    case.record_appeal(
        stage(&case_identity, JudicialStage::Appeal, 0, "primary"),
        basic_appeal(&case_identity, "primary"),
    )
    .unwrap();
    case.record_constitutional_review(
        stage(
            &case_identity,
            JudicialStage::ConstitutionalReview,
            0,
            "primary",
        ),
        basic_review(&case_identity, "primary"),
    )
    .unwrap();
    case.record_restitution(
        stage(&case_identity, JudicialStage::Restitution, 0, "primary"),
        basic_restitution(
            &case_identity,
            "primary",
            &remedy_id,
            RestitutionDisposition::EquilibriumConfirmed,
            0,
        ),
        None,
    )
    .unwrap();
    case
}

#[test]
fn one_minoan_system_hosts_all_jurisdictions_without_becoming_a_house() {
    let system = MinoanCountyCourtSystem::default();
    system.validate().unwrap();
    assert_eq!(
        system.identity.as_str(),
        "institution.sandmanor.minoan-county-courthouse"
    );
    assert!(system.hosted_by_minoans);
    assert!(!system.owns_house_law);
    assert!(!system.is_house);
    assert_eq!(system.jurisdictions.len(), 8);
    assert_eq!(House::as_str(House::Sandmanor), "Sandmanor");
}

#[test]
fn central_junction_is_a_jurisdiction_but_not_a_house() {
    assert_eq!(CourtJurisdiction::CentralJunction.domain_house(), None);
    assert_eq!(House::Stonebend.as_str(), "Stonebend");
    assert_eq!(House::Sandmanor.as_str(), "Sandmanor");
    assert_eq!(House::Glaushouse.as_str(), "Glaushouse");
    assert_eq!(House::Flynt.as_str(), "Flynt");
}

#[test]
fn one_case_supports_multiple_jurisdictions_without_duplicate_identity() {
    let mut system = MinoanCountyCourtSystem::default();
    let case = case_with(
        "case.cross-house.one",
        BTreeSet::from([
            CourtJurisdiction::Stonebend,
            CourtJurisdiction::Flynt,
            CourtJurisdiction::Glaushouse,
            CourtJurisdiction::CrossHouse,
        ]),
        CourtCasePolicy::default(),
    );
    let case_id = case.identity.clone();
    system.open_case(case).unwrap();
    assert_eq!(system.case_count(), 1);
    assert_eq!(system.case(&case_id).unwrap().jurisdictions.len(), 4);
}

#[test]
fn house_evidence_retains_source_and_the_court_cannot_fabricate_it() {
    let mut case = case_with(
        "case.evidence.source",
        BTreeSet::from([CourtJurisdiction::Flynt]),
        CourtCasePolicy::default(),
    );
    submit(
        &mut case,
        "submission.flynt.persistence",
        DomainEvidenceSource::House(House::Flynt),
        CourtJurisdiction::Flynt,
    );
    let stored = case
        .evidence()
        .get(&id("submission.flynt.persistence"))
        .unwrap();
    assert_eq!(stored.source, DomainEvidenceSource::House(House::Flynt));
    assert!(!stored.source.supplied_by_court());
    let invalid = EvidenceSubmission {
        identity: id("submission.invalid.house"),
        source: DomainEvidenceSource::House(House::Glaushouse),
        jurisdiction: CourtJurisdiction::Flynt,
        records: vec![evidence("evidence.invalid.house")],
        authenticated_by: id("authority.invalid.house"),
        description: "wrong House evidence".into(),
    };
    assert!(matches!(
        case.submit_evidence(invalid),
        Err(CourtValidationError::InvalidDomainEvidence(_))
    ));
}

#[test]
fn five_stage_case_closes_only_after_restitution_equilibrium() {
    let case = complete_case("case.full-cycle");
    assert_eq!(case.closure, CaseClosure::EquilibriumConfirmed);
    assert_eq!(
        case.semantic_stage_history()
            .iter()
            .map(|record| record.stage)
            .collect::<Vec<_>>(),
        JudicialStage::ALL
    );
    assert_eq!(case.restitutions().len(), 1);
}

#[test]
fn judicial_stage_order_is_semantic_not_insertion_order() {
    let case = id("case.semantic-order");
    let forward = JudicialStage::ALL
        .into_iter()
        .map(|judicial_stage| stage(&case, judicial_stage, 0, stage_slug(judicial_stage)))
        .collect::<Vec<_>>();
    let mut reversed = forward.clone();
    reversed.reverse();
    let forward_order = semantic_judicial_history(&forward)
        .iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    let reverse_order = semantic_judicial_history(&reversed)
        .iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    assert_eq!(forward_order, JudicialStage::ALL);
    assert_eq!(forward_order, reverse_order);
}

#[test]
fn first_hearing_cannot_bypass_required_conciliation() {
    let mut case = case_with(
        "case.no-conciliation",
        BTreeSet::from([CourtJurisdiction::Stonebend]),
        CourtCasePolicy::default(),
    );
    submit(
        &mut case,
        "submission.stonebend.no-conciliation",
        DomainEvidenceSource::House(House::Stonebend),
        CourtJurisdiction::Stonebend,
    );
    let case_id = case.identity.clone();
    let remedy = basic_remedy(
        &case_id,
        "no-conciliation",
        RemedyKind::PublicRecordCorrection,
    );
    let remedy_id = remedy.identity.clone();
    case.add_remedy(remedy).unwrap();
    assert!(matches!(
        case.record_first_hearing(
            stage(&case_id, JudicialStage::FirstHearing, 0, "no-conciliation"),
            basic_hearing(
                &case_id,
                "no-conciliation",
                "submission.stonebend.no-conciliation",
                &remedy_id,
            )
        ),
        Err(CourtValidationError::MissingRequiredStage(
            JudicialStage::Conciliation
        ))
    ));
}

#[test]
fn emergency_protection_is_bounded_and_does_not_require_completed_conciliation() {
    let mut case = case_with(
        "case.emergency-protection",
        BTreeSet::from([CourtJurisdiction::Glaushouse]),
        CourtCasePolicy::default(),
    );
    let case_id = case.identity.clone();
    let order = ProtectiveOrder {
        identity: id("order.emergency.clinical-transfer"),
        case: case_id,
        authority: id("authority.court.duty"),
        evidence: vec![evidence("evidence.emergency.injury")],
        protected_subject: id("being.case.claimant"),
        action: ProtectiveAction::TemporaryClinicalTransfer,
        exact_scope: "transfer only; no medical viability finding".into(),
        reason: "credible immediate bodily harm".into(),
        start_condition: "on delivery to the courthouse".into(),
        termination_condition: "clinical intake or judicial review".into(),
        review_required: true,
        affected_titles: BTreeSet::new(),
        permanent: false,
    };
    case.issue_protective_order(order).unwrap();
    assert_eq!(case.protective_orders().len(), 1);
}

#[test]
fn temporary_protection_cannot_become_permanent_by_delay() {
    let mut case = case_with(
        "case.permanent-order",
        BTreeSet::from([CourtJurisdiction::Flynt]),
        CourtCasePolicy::default(),
    );
    let order = ProtectiveOrder {
        identity: id("order.invalid.permanent"),
        case: case.identity.clone(),
        authority: id("authority.court.duty"),
        evidence: vec![evidence("evidence.order")],
        protected_subject: id("being.case.claimant"),
        action: ProtectiveAction::RestrictedDeployment,
        exact_scope: "one machine".into(),
        reason: "immediate operational risk".into(),
        start_condition: "now".into(),
        termination_condition: "review".into(),
        review_required: true,
        affected_titles: BTreeSet::new(),
        permanent: true,
    };
    assert!(matches!(
        case.issue_protective_order(order),
        Err(CourtValidationError::UnboundedProtectiveOrder(_))
    ));
}

#[test]
fn conciliation_cannot_force_surrender_of_a_lawful_right() {
    let settlement = ConciliationSettlement {
        identity: id("settlement.coerced"),
        case: id("case.coerced"),
        parties: BTreeSet::from([id("being.case.claimant")]),
        agreed_remedies: BTreeSet::from([id("remedy.coerced")]),
        authority: id("authority.conciliator"),
        completion_condition: "claimant gives up protected standing".into(),
        voluntary: true,
        coerced: false,
        surrenders_lawful_right: true,
    };
    assert!(matches!(
        settlement.validate(),
        Err(CourtValidationError::InvalidConciliationSettlement(_))
    ));
}

#[test]
fn settlement_requires_restitution_and_failed_performance_keeps_case_open() {
    let mut case = case_with(
        "case.settlement",
        BTreeSet::from([CourtJurisdiction::MinoanCoastal]),
        CourtCasePolicy {
            conciliation_required: true,
            full_review_cycle_required: false,
        },
    );
    let case_id = case.identity.clone();
    let remedy = basic_remedy(&case_id, "settlement", RemedyKind::AccessRestoration);
    let remedy_id = remedy.identity.clone();
    case.add_remedy(remedy).unwrap();
    let mut conciliation = basic_conciliation(&case_id, "settlement");
    conciliation.voluntary_repair_possible = true;
    conciliation.settlement = Some(ConciliationSettlement {
        identity: id("settlement.voluntary"),
        case: case_id.clone(),
        parties: BTreeSet::from([id("being.case.claimant"), id("being.case.respondent")]),
        agreed_remedies: BTreeSet::from([remedy_id.clone()]),
        authority: id("authority.minoan.conciliator"),
        completion_condition: "access is actually restored".into(),
        voluntary: true,
        coerced: false,
        surrenders_lawful_right: false,
    });
    case.record_conciliation(
        stage(&case_id, JudicialStage::Conciliation, 0, "settlement"),
        conciliation,
    )
    .unwrap();
    assert_eq!(case.closure, CaseClosure::Open);
    let failed = RestitutionRecord {
        identity: id("restitution.settlement.failed"),
        case: case_id.clone(),
        judgment: id("settlement.voluntary"),
        responsible_institutions: BTreeSet::from([ResponsibleInstitution::MinoanCoastal]),
        harmed_parties_or_communities: BTreeSet::from([id("being.case.claimant")]),
        remedies: BTreeSet::from([remedy_id.clone()]),
        delivery_evidence: vec![evidence("evidence.settlement.attempt")],
        completion_evidence: vec![],
        remaining_burden: "access remains blocked".into(),
        unintended_effects: vec![],
        yield_evidence: vec![evidence("evidence.settlement.yield")],
        equilibrium: equilibrium(false),
        disposition: RestitutionDisposition::PartiallySatisfied,
        cycle: 0,
    };
    case.record_restitution(
        stage(&case_id, JudicialStage::Restitution, 0, "settlement-failed"),
        failed,
        None,
    )
    .unwrap();
    assert_eq!(case.closure, CaseClosure::Open);
    let complete = RestitutionRecord {
        identity: id("restitution.settlement.complete"),
        case: case_id.clone(),
        judgment: id("settlement.voluntary"),
        responsible_institutions: BTreeSet::from([ResponsibleInstitution::MinoanCoastal]),
        harmed_parties_or_communities: BTreeSet::from([id("being.case.claimant")]),
        remedies: BTreeSet::from([remedy_id]),
        delivery_evidence: vec![evidence("evidence.settlement.delivery")],
        completion_evidence: vec![evidence("evidence.settlement.completion")],
        remaining_burden: "ordinary public access conditions".into(),
        unintended_effects: vec![],
        yield_evidence: vec![evidence("evidence.settlement.final-yield")],
        equilibrium: equilibrium(true),
        disposition: RestitutionDisposition::EquilibriumConfirmed,
        cycle: 0,
    };
    case.record_restitution(
        stage(
            &case_id,
            JudicialStage::Restitution,
            0,
            "settlement-complete",
        ),
        complete,
        None,
    )
    .unwrap();
    assert_eq!(case.closure, CaseClosure::EquilibriumConfirmed);
}

#[test]
fn appeal_is_bounded_review_and_does_not_automatically_stay_or_retry() {
    let appeal = basic_appeal(&id("case.appeal"), "appeal");
    appeal.validate().unwrap();
    assert!(!appeal.retries_all_facts);
    assert!(appeal.stay.is_none());
    assert_eq!(appeal.disposition, AppealDisposition::Affirmed);
}

#[test]
fn appeal_cannot_hide_a_complete_retrial() {
    let mut appeal = basic_appeal(&id("case.appeal-invalid"), "appeal-invalid");
    appeal.retries_all_facts = true;
    assert!(matches!(
        appeal.validate(),
        Err(CourtValidationError::InvalidAppeal(_))
    ));
}

#[test]
fn constitutional_review_cannot_amend_ratify_or_assume_house_power() {
    let mut review = basic_review(&id("case.review-limits"), "review-limits");
    review.amends_constitution = true;
    assert!(matches!(
        review.validate(),
        Err(CourtValidationError::InvalidConstitutionalReview(_))
    ));
    review.amends_constitution = false;
    review.ratifies_amendment = true;
    assert!(review.validate().is_err());
    review.ratifies_amendment = false;
    review.bears_diamond = true;
    assert!(review.validate().is_err());
    review.bears_diamond = false;
    review.forges_stonebend_claim = true;
    assert!(review.validate().is_err());
    review.forges_stonebend_claim = false;
    review.replaces_proliteriate = true;
    assert!(review.validate().is_err());
}

#[test]
fn judgment_routes_enforcement_and_cannot_create_a_universal_court_arm() {
    let judgment = Judgment {
        identity: id("judgment.routing"),
        case: id("case.routing"),
        findings: BTreeSet::from([id("finding.routing")]),
        legal_conclusions: vec!["Flynt performs the technical restriction".into()],
        targets: BTreeSet::from([JudgmentTarget::SpecificAction(id("action.flynt.shutdown"))]),
        remedies: BTreeSet::from([id("remedy.flynt.shutdown")]),
        constitutional_effect: JudgmentConstitutionalEffect::TargetedRemedy,
        evidence: vec![evidence("evidence.routing")],
        court_executes_remedy: false,
        court_removes_principal_power: false,
    };
    judgment.validate().unwrap();
    assert!(!judgment.court_executes_remedy);
}

#[test]
fn hypergiant_ruling_is_challenge_evidence_not_judicial_removal() {
    let judgment = Judgment {
        identity: id("judgment.hypergiant.scope"),
        case: id("case.hypergiant.scope"),
        findings: BTreeSet::from([id("finding.hypergiant.scope")]),
        legal_conclusions: vec!["the bearer exceeded a bounded scope".into()],
        targets: BTreeSet::from([JudgmentTarget::BearerTenure(id(
            "tenure.hypergiant.current",
        ))]),
        remedies: BTreeSet::new(),
        constitutional_effect: JudgmentConstitutionalEffect::EvidenceForStonebendChallenge(
            StonebendConstitutionalPower::Hypergiant,
        ),
        evidence: vec![evidence("evidence.hypergiant.scope")],
        court_executes_remedy: false,
        court_removes_principal_power: false,
    };
    judgment.validate().unwrap();
    assert_eq!(
        judgment.constitutional_effect,
        JudgmentConstitutionalEffect::EvidenceForStonebendChallenge(
            StonebendConstitutionalPower::Hypergiant
        )
    );
    assert!(!judgment.court_removes_principal_power);
}

#[test]
fn failed_restitution_recurs_under_same_case_and_preserves_failed_history() {
    let mut case = case_with(
        "case.recurrence",
        BTreeSet::from([
            CourtJurisdiction::Stonebend,
            CourtJurisdiction::Constitutional,
        ]),
        CourtCasePolicy::default(),
    );
    let case_id = case.identity.clone();
    submit(
        &mut case,
        "submission.stonebend.recurrence",
        DomainEvidenceSource::House(House::Stonebend),
        CourtJurisdiction::Stonebend,
    );
    case.record_conciliation(
        stage(&case_id, JudicialStage::Conciliation, 0, "recurrence"),
        basic_conciliation(&case_id, "recurrence"),
    )
    .unwrap();
    let remedy = basic_remedy(&case_id, "recurrence", RemedyKind::ProvenanceRestoration);
    let remedy_id = remedy.identity.clone();
    case.add_remedy(remedy).unwrap();
    case.record_first_hearing(
        stage(&case_id, JudicialStage::FirstHearing, 0, "recurrence"),
        basic_hearing(
            &case_id,
            "recurrence",
            "submission.stonebend.recurrence",
            &remedy_id,
        ),
    )
    .unwrap();
    case.record_appeal(
        stage(&case_id, JudicialStage::Appeal, 0, "recurrence"),
        basic_appeal(&case_id, "recurrence"),
    )
    .unwrap();
    case.record_constitutional_review(
        stage(
            &case_id,
            JudicialStage::ConstitutionalReview,
            0,
            "recurrence",
        ),
        basic_review(&case_id, "recurrence"),
    )
    .unwrap();
    let failed = basic_restitution(
        &case_id,
        "recurrence",
        &remedy_id,
        RestitutionDisposition::ReturnedToFirstHearing,
        0,
    );
    let failed_id = failed.identity.clone();
    case.record_restitution(
        stage(&case_id, JudicialStage::Restitution, 0, "recurrence-failed"),
        failed,
        Some(CaseRecurrence {
            identity: id("recurrence.case.recurrence.first-hearing"),
            case: case_id.clone(),
            failed_restitution: failed_id.clone(),
            from_cycle: 0,
            to_cycle: 1,
            return_stage: JudicialStage::FirstHearing,
            lawful_reason: "execution exposed a materially incomplete factual record".into(),
            evidence: vec![evidence("evidence.recurrence.new-facts")],
        }),
    )
    .unwrap();
    assert_eq!(case.closure, CaseClosure::Open);
    assert!(case.restitutions().contains_key(&failed_id));
    assert_eq!(case.recurrences().len(), 1);

    let second_remedy = basic_remedy(
        &case_id,
        "recurrence-second",
        RemedyKind::ProvenanceRestoration,
    );
    let second_remedy_id = second_remedy.identity.clone();
    case.add_remedy(second_remedy).unwrap();
    case.record_first_hearing(
        stage(
            &case_id,
            JudicialStage::FirstHearing,
            1,
            "recurrence-second",
        ),
        basic_hearing(
            &case_id,
            "recurrence-second",
            "submission.stonebend.recurrence",
            &second_remedy_id,
        ),
    )
    .unwrap();
    case.record_restitution(
        stage(
            &case_id,
            JudicialStage::Restitution,
            1,
            "recurrence-complete",
        ),
        basic_restitution(
            &case_id,
            "recurrence-second",
            &second_remedy_id,
            RestitutionDisposition::EquilibriumConfirmed,
            1,
        ),
        None,
    )
    .unwrap();
    assert_eq!(case.identity, case_id);
    assert_eq!(case.closure, CaseClosure::EquilibriumConfirmed);
    assert_eq!(case.restitutions().len(), 2);
}

#[test]
fn recurrence_requires_a_lawful_reason_and_matching_return_stage() {
    let recurrence = CaseRecurrence {
        identity: id("recurrence.invalid"),
        case: id("case.invalid-recurrence"),
        failed_restitution: id("restitution.invalid"),
        from_cycle: 0,
        to_cycle: 1,
        return_stage: JudicialStage::Appeal,
        lawful_reason: String::new(),
        evidence: vec![evidence("evidence.invalid-recurrence")],
    };
    assert!(recurrence.lawful_reason.is_empty());
}

fn gate_recognition(title: &TitleRecordId, facing: StonebendGateFacing) -> GateScopeRecognition {
    let authority = match facing {
        StonebendGateFacing::Flynt => DomainEvidenceAuthority::FlyntProofOfPersistence,
        StonebendGateFacing::CentralJunction => {
            DomainEvidenceAuthority::CentralJunctionPublicStandard
        }
        StonebendGateFacing::Sandmanor => DomainEvidenceAuthority::SandmanorDesignAndFormation,
    };
    GateScopeRecognition {
        identity: id(format!("recognition.{}", facing.stable_id())),
        title: title.clone(),
        facing,
        scope: facing.scope(),
        disposition: TitleScopeDisposition::Recognized,
        domain_authority: authority,
        evidence: vec![evidence(format!("evidence.{}", facing.stable_id()))],
        boundary: "bounded gate authority".into(),
        returned_evidence: vec![],
    }
}

#[test]
fn targeted_scope_remedy_preserves_core_title_and_unrelated_scopes() {
    let title_id = title("title.targeted.machine");
    let mut core = StonebendTitleCore::new(
        title_id.clone(),
        id("being.targeted.machine"),
        NameRecordId::new("name.targeted.machine").unwrap(),
        id("claim.targeted.machine"),
        vec![evidence("evidence.targeted.core")],
        "lawful machine identity",
    );
    for facing in StonebendGateFacing::ALL {
        core.record_scope(gate_recognition(&title_id, facing))
            .unwrap();
    }
    let target = JudgmentTarget::GateScope {
        title: title_id.clone(),
        scope: GateScope::OperationalDeployment,
    };
    assert_eq!(
        target.title_intervention_target(),
        Some(
            hollow_grove::world::stonebend::third_pass::TitleInterventionTarget::GateScope(
                GateScope::OperationalDeployment
            )
        )
    );
    core.update_scope_disposition(
        GateScope::OperationalDeployment,
        TitleScopeDisposition::Suspended,
    )
    .unwrap();
    assert!(!core.authorizes(GateScope::OperationalDeployment));
    assert!(core.authorizes(GateScope::FormationRecognition));
    assert!(core.authorizes(GateScope::PublicCirculation));
    assert_eq!(core.title, title_id);
    core.update_scope_disposition(
        GateScope::OperationalDeployment,
        TitleScopeDisposition::Recognized,
    )
    .unwrap();
    assert!(core.authorizes(GateScope::OperationalDeployment));
}

#[test]
fn mandate_remedy_does_not_abolish_proliteriate() {
    let target = JudgmentTarget::Mandate(id("mandate.proliteriate.corrupt"));
    assert_eq!(
        target.title_intervention_target(),
        Some(
            hollow_grove::world::stonebend::third_pass::TitleInterventionTarget::Mandate(id(
                "mandate.proliteriate.corrupt"
            ))
        )
    );
    assert_ne!(
        StonebendConstitutionalPower::Proliteriate.institution(),
        minoan_county_court_system_id()
    );
}

#[test]
fn cross_house_evidence_routes_remedies_without_absorbing_authority() {
    let mut case = case_with(
        "case.cross-house.evidence",
        BTreeSet::from([
            CourtJurisdiction::Stonebend,
            CourtJurisdiction::Flynt,
            CourtJurisdiction::Glaushouse,
            CourtJurisdiction::CrossHouse,
        ]),
        CourtCasePolicy::default(),
    );
    submit(
        &mut case,
        "submission.cross.stonebend",
        DomainEvidenceSource::House(House::Stonebend),
        CourtJurisdiction::Stonebend,
    );
    submit(
        &mut case,
        "submission.cross.flynt",
        DomainEvidenceSource::House(House::Flynt),
        CourtJurisdiction::Flynt,
    );
    submit(
        &mut case,
        "submission.cross.glaushouse",
        DomainEvidenceSource::House(House::Glaushouse),
        CourtJurisdiction::Glaushouse,
    );
    let sources = case
        .evidence()
        .values()
        .map(|entry| entry.source)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        sources,
        BTreeSet::from([
            DomainEvidenceSource::House(House::Stonebend),
            DomainEvidenceSource::House(House::Flynt),
            DomainEvidenceSource::House(House::Glaushouse),
        ])
    );
    assert_eq!(case.identity.as_str(), "case.cross-house.evidence");
}

#[test]
fn restitution_is_broader_than_money_and_distinct_from_punishment() {
    let represented = BTreeSet::from([
        RemedyKind::PublicRecordCorrection,
        RemedyKind::PropertyReturn,
        RemedyKind::CustodyRestoration,
        RemedyKind::TitleRestoration,
        RemedyKind::GateScopeRestoration,
        RemedyKind::CareDelivery,
        RemedyKind::FormRepair,
        RemedyKind::LaborOrBurdenCompensation,
        RemedyKind::MaterialReplacement,
        RemedyKind::MonetaryCompensation,
    ]);
    assert!(represented.len() > 1);
    assert!(represented.contains(&RemedyKind::CareDelivery));
    assert!(represented.contains(&RemedyKind::PublicRecordCorrection));
    assert!(represented.contains(&RemedyKind::MonetaryCompensation));
}

#[test]
fn equilibrium_preserves_history_and_may_leave_lawfully_assigned_burden() {
    let assessment = equilibrium(true);
    assert!(assessment.can_hold());
    assert!(assessment.historical_record_accurate);
    assert!(!assessment.remaining_burden.is_empty());
}

fn proposal(scope: AmendmentScope, suffix: &str) -> AmendmentProposal {
    AmendmentProposal {
        identity: id(format!("amendment.{suffix}")),
        exact_text: format!("Ratified constitutional text for {suffix}."),
        superseded_text: vec![format!("Prior text for {suffix}.")],
        purpose: "clarify a bounded constitutional responsibility".into(),
        affected_houses: scope.required_houses(),
        affected_offices: BTreeSet::new(),
        affected_titles: BTreeSet::new(),
        affected_communities: BTreeSet::from([id(format!("community.{suffix}"))]),
        altered_authority: "only the authority stated in the exact text".into(),
        expected_yield: vec![evidence(format!("evidence.amendment.{suffix}.yield"))],
        scope,
        public_notice: vec![evidence(format!("evidence.amendment.{suffix}.notice"))],
        affected_party_testimony: vec![evidence(format!("evidence.amendment.{suffix}.testimony"))],
        bundled_unrelated_changes: false,
    }
}

fn amendment_stage(
    proposal: &IdentityId,
    stage: AmendmentStage,
    suffix: &str,
) -> AmendmentStageRecord {
    AmendmentStageRecord {
        identity: id(format!(
            "amendment-stage.{}.{}.{}",
            proposal.as_str(),
            stage.semantic_order(),
            suffix
        )),
        proposal: proposal.clone(),
        stage,
        evidence: vec![evidence(format!(
            "evidence.amendment-stage.{}.{}",
            stage.semantic_order(),
            suffix
        ))],
    }
}

fn prepare_certified_process(proposal: AmendmentProposal, suffix: &str) -> AmendmentProcess {
    let proposal_id = proposal.identity.clone();
    let mut process = AmendmentProcess::new(proposal).unwrap();
    for stage in [
        AmendmentStage::PublicNotice,
        AmendmentStage::AffectedPartyTestimony,
        AmendmentStage::ProceduralHearing,
        AmendmentStage::ConstitutionalReview,
    ] {
        process
            .add_stage(amendment_stage(&proposal_id, stage, suffix))
            .unwrap();
    }
    process
        .certify(AmendmentProcessCertification {
            identity: id(format!("certification.{suffix}")),
            proposal: proposal_id,
            court_system: minoan_county_court_system_id(),
            disposition: ConstitutionalReviewDisposition::EligibleForRatification,
            evidence: vec![evidence(format!("evidence.certification.{suffix}"))],
            court_ratified: false,
            path_lawful: true,
        })
        .unwrap();
    process
}

fn ratification(
    process: &AmendmentProcess,
    suffix: &str,
    house_assents: BTreeMap<House, IdentityId>,
) -> RatificationRecord {
    RatificationRecord {
        identity: id(format!("ratification.{suffix}")),
        proposal: process.proposal.identity.clone(),
        scope: process.proposal.scope.clone(),
        house_assents,
        evidence: vec![evidence(format!("evidence.ratification.{suffix}"))],
        court_enacted: false,
        central_junction_counted_as_house: false,
    }
}

fn seal_record(
    process: &AmendmentProcess,
    ratification: &RatificationRecord,
    suffix: &str,
) -> AmendmentSealRecord {
    AmendmentSealRecord {
        identity: seal(format!("seal.amendment.{suffix}")),
        proposal: process.proposal.identity.clone(),
        ratification: ratification.identity.clone(),
        final_text: process.proposal.exact_text.clone(),
        stonebend_record_authority: id("authority.stonebend.recording"),
        evidence: vec![evidence(format!("evidence.seal.{suffix}"))],
    }
}

fn implementation_review(
    process: &AmendmentProcess,
    suffix: &str,
) -> AmendmentImplementationReview {
    AmendmentImplementationReview {
        identity: id(format!("implementation-review.{suffix}")),
        proposal: process.proposal.identity.clone(),
        ratified_text: process.proposal.exact_text.clone(),
        implemented_text: process.proposal.exact_text.clone(),
        affected_title_updates: BTreeSet::new(),
        affected_house_evidence: process
            .proposal
            .affected_houses
            .iter()
            .map(|house| {
                (
                    *house,
                    evidence(format!(
                        "evidence.implementation.{}.{}",
                        house.as_str().to_lowercase(),
                        suffix
                    )),
                )
            })
            .collect(),
        notice_and_transition_evidence: vec![evidence(format!(
            "evidence.implementation.{suffix}.notice"
        ))],
        yield_evidence: vec![evidence(format!("evidence.implementation.{suffix}.yield"))],
        restitution_disposition: RestitutionDisposition::EquilibriumConfirmed,
        equilibrium: equilibrium(true),
    }
}

#[test]
fn house_local_amendment_requires_review_then_house_ratification_then_seal() {
    let mut process = prepare_certified_process(
        proposal(AmendmentScope::HouseLocal(House::Sandmanor), "local"),
        "local",
    );
    assert!(!process.certification.as_ref().unwrap().court_ratified);
    let record = ratification(
        &process,
        "local",
        BTreeMap::from([(House::Sandmanor, id("authority.sandmanor.ratification"))]),
    );
    record.validate().unwrap();
    let seal = seal_record(&process, &record, "local");
    process.ratify(record).unwrap();
    process.seal(seal).unwrap();
    let review = implementation_review(&process, "local");
    process.review_implementation(review).unwrap();
    assert!(process.implementation_review.is_some());
}

#[test]
fn court_certification_is_not_ratification() {
    let process = prepare_certified_process(
        proposal(AmendmentScope::HouseLocal(House::Flynt), "cert-only"),
        "cert-only",
    );
    assert!(process.certification.as_ref().unwrap().path_lawful);
    assert!(!process.certification.as_ref().unwrap().court_ratified);
    assert!(process.ratification.is_none());
    assert!(process.seal.is_none());
}

#[test]
fn ratification_cannot_precede_constitutional_process_certification() {
    let proposal = proposal(AmendmentScope::HouseLocal(House::Stonebend), "premature");
    let mut process = AmendmentProcess::new(proposal).unwrap();
    let record = ratification(
        &process,
        "premature",
        BTreeMap::from([(House::Stonebend, id("authority.stonebend.ratification"))]),
    );
    assert!(matches!(
        process.ratify(record),
        Err(CourtValidationError::RatificationBeforeCertification)
    ));
}

#[test]
fn stonebend_seal_cannot_precede_ratification_or_replace_it() {
    let mut process = prepare_certified_process(
        proposal(AmendmentScope::HouseLocal(House::Glaushouse), "seal-order"),
        "seal-order",
    );
    let record = ratification(
        &process,
        "seal-order",
        BTreeMap::from([(House::Glaushouse, id("authority.glaushouse.ratification"))]),
    );
    let seal = seal_record(&process, &record, "seal-order");
    assert!(matches!(
        process.seal(seal.clone()),
        Err(CourtValidationError::SealBeforeRatification)
    ));
    process.ratify(record).unwrap();
    process.seal(seal).unwrap();
}

#[test]
fn cross_house_amendment_requires_every_affected_house() {
    let scope = AmendmentScope::CrossHouse(BTreeSet::from([House::Flynt, House::Glaushouse]));
    let process = prepare_certified_process(proposal(scope, "cross"), "cross");
    let one_house = ratification(
        &process,
        "cross-one",
        BTreeMap::from([(House::Flynt, id("authority.flynt.ratification"))]),
    );
    assert!(matches!(
        one_house.validate(),
        Err(CourtValidationError::InvalidRatification(_))
    ));
    let both = ratification(
        &process,
        "cross-both",
        BTreeMap::from([
            (House::Flynt, id("authority.flynt.ratification")),
            (House::Glaushouse, id("authority.glaushouse.ratification")),
        ]),
    );
    both.validate().unwrap();
}

#[test]
fn foundational_amendment_requires_exactly_all_four_houses() {
    let process = prepare_certified_process(
        proposal(AmendmentScope::Foundational, "foundation"),
        "foundation",
    );
    let three = ratification(
        &process,
        "foundation-three",
        BTreeMap::from([
            (House::Stonebend, id("authority.stonebend.ratification")),
            (House::Sandmanor, id("authority.sandmanor.ratification")),
            (House::Flynt, id("authority.flynt.ratification")),
        ]),
    );
    assert!(three.validate().is_err());
    let four = ratification(
        &process,
        "foundation-four",
        BTreeMap::from([
            (House::Stonebend, id("authority.stonebend.ratification")),
            (House::Sandmanor, id("authority.sandmanor.ratification")),
            (House::Glaushouse, id("authority.glaushouse.ratification")),
            (House::Flynt, id("authority.flynt.ratification")),
        ]),
    );
    four.validate().unwrap();
    assert_eq!(four.house_assents.len(), 4);
    assert!(!four.central_junction_counted_as_house);
}

#[test]
fn amendment_implementation_must_reach_restitution_review() {
    let mut process = prepare_certified_process(
        proposal(
            AmendmentScope::HouseLocal(House::Stonebend),
            "implementation",
        ),
        "implementation",
    );
    let record = ratification(
        &process,
        "implementation",
        BTreeMap::from([(House::Stonebend, id("authority.stonebend.ratification"))]),
    );
    let seal = seal_record(&process, &record, "implementation");
    process.ratify(record).unwrap();
    process.seal(seal).unwrap();
    assert!(process.implementation_review.is_none());
    process
        .review_implementation(implementation_review(&process, "implementation"))
        .unwrap();
    assert_eq!(
        process
            .implementation_review
            .as_ref()
            .unwrap()
            .restitution_disposition,
        RestitutionDisposition::EquilibriumConfirmed
    );
}

#[test]
fn restitution_cannot_silently_rewrite_ratified_text() {
    let process = prepare_certified_process(
        proposal(AmendmentScope::HouseLocal(House::Sandmanor), "rewrite"),
        "rewrite",
    );
    let mut review = implementation_review(&process, "rewrite");
    review.implemented_text = "Different text smuggled through implementation.".into();
    assert!(matches!(
        review.validate(),
        Err(CourtValidationError::RatifiedTextSilentlyRewritten(_))
    ));
}

#[test]
fn amendment_stage_order_is_semantic_and_insertion_independent() {
    let proposal = id("amendment.semantic");
    let stages = [
        AmendmentStage::Proposal,
        AmendmentStage::PublicNotice,
        AmendmentStage::AffectedPartyTestimony,
        AmendmentStage::Conciliation,
        AmendmentStage::ProceduralHearing,
        AmendmentStage::Appeal,
        AmendmentStage::ConstitutionalReview,
        AmendmentStage::Ratification,
        AmendmentStage::StonebendSeal,
        AmendmentStage::Implementation,
        AmendmentStage::RestitutionReview,
        AmendmentStage::Equilibrium,
    ];
    let forward = stages
        .into_iter()
        .map(|stage| amendment_stage(&proposal, stage, "semantic"))
        .collect::<Vec<_>>();
    let mut reverse = forward.clone();
    reverse.reverse();
    assert_eq!(
        semantic_amendment_history(&forward)
            .iter()
            .map(|record| record.stage)
            .collect::<Vec<_>>(),
        semantic_amendment_history(&reverse)
            .iter()
            .map(|record| record.stage)
            .collect::<Vec<_>>()
    );
}

#[test]
fn no_new_house_currency_sovereign_or_duplicate_title_engine_exists() {
    assert_eq!(House::Stonebend.as_str(), "Stonebend");
    assert_eq!(House::Sandmanor.as_str(), "Sandmanor");
    assert_eq!(House::Glaushouse.as_str(), "Glaushouse");
    assert_eq!(House::Flynt.as_str(), "Flynt");
    assert_eq!(
        minoan_county_court_system_id(),
        hollow_grove::world::sandmanor::milestone::minoan_county_courthouse_id()
    );
}
