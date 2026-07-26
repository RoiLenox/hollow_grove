use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::constitutional::{ChallengeId, EvidenceRef};
use hollow_grove::institution::IdentityId;
use hollow_grove::world::stonebend::second_pass::{
    DomainEvidenceAuthority, GateFailureKind, GateScope, GateScopeRecognition, NetworkMandate,
    NetworkMembership, ProliteriateNetwork, ProliteriateNode, ProliteriateNodeKind, RaisedWitness,
    StonebendGateFacing, StonebendTitleCore, TitleScopeDisposition, YieldImpact, YieldRecord,
};
use hollow_grove::world::stonebend::third_pass::{
    ActivationMode, ActivationRequirement, ChallengePeriodPolicy, ClaimTemporalRecord,
    ClaimTitleYieldReview, ConstitutionalReferral, ContinuityActionKind, ContinuityMandateStatus,
    ContinuityTerminationCondition, DiamondContinuityMandate, EmergencyContinuityAction,
    ForgeReviewDisposition, FreemasonInvestiture, FreemasonOfficeState, FreemasonRecordRef,
    FreemasonSuccessionClaim, FreemasonTenure, FreemasonTenureStatus, IndependentForgeReview,
    ProliteriateContinuityPolicy, RequiredReturnPath, RestorationRecord,
    RestrictedRestorationSubject, StonebendTitleLifecycle, SupervisionTerms,
    TemporalDimensionTrace, TemporalReviewDisposition, TitleActivation, TitleExerciseState,
    TitleIntervention, TitleInterventionKind, TitleInterventionTarget, TitleLifecyclePolicy,
    TitleLifecycleStage, TitleMaintenanceRecord, TitleMaintenanceRequirement, TitleRecognition,
    TitleRenewalDisposition, TitleRenewalPolicy, TitleRenewalRecord, TitleStageRecord,
    TitleTemporalRecord, TitleTermTombstone, TitleTerminalDisposition, YieldTemporalRecord,
    canonical_vacancy_dimension_traces, diamond_vacancy_has_no_bearer, required_return_path,
};
use hollow_grove::world::stonebend::{
    EvidenceRecordId, NameRecordId, SealRecordId, TitleRecordId, TombstoneRecordId,
    freemason_institution_id, proliteriate_id,
};

fn id(value: &str) -> IdentityId {
    IdentityId::new(value).expect("stable fixture identity")
}

fn evidence(value: &str) -> EvidenceRecordId {
    EvidenceRecordId::new(value).expect("stable fixture evidence")
}

fn title(value: &str) -> TitleRecordId {
    TitleRecordId::new(value).expect("stable fixture Title")
}

fn name(value: &str) -> NameRecordId {
    NameRecordId::new(value).expect("stable fixture Name")
}

fn seal(value: &str) -> SealRecordId {
    SealRecordId::new(value).expect("stable fixture Seal")
}

fn tombstone(value: &str) -> TombstoneRecordId {
    TombstoneRecordId::new(value).expect("stable fixture Tombstone")
}

fn challenge(value: &str) -> ChallengeId {
    ChallengeId::new(value).expect("stable fixture challenge")
}

fn scope_record(title: &TitleRecordId, facing: StonebendGateFacing) -> GateScopeRecognition {
    let domain_authority = match facing {
        StonebendGateFacing::Sandmanor => DomainEvidenceAuthority::SandmanorDesignAndFormation,
        StonebendGateFacing::CentralJunction => {
            DomainEvidenceAuthority::CentralJunctionPublicStandard
        }
        StonebendGateFacing::Flynt => DomainEvidenceAuthority::FlyntProofOfPersistence,
    };
    GateScopeRecognition {
        identity: id(&format!("scope.record.{}", facing.stable_id())),
        title: title.clone(),
        facing,
        scope: facing.scope(),
        disposition: TitleScopeDisposition::Recognized,
        domain_authority,
        evidence: vec![evidence(&format!("evidence.scope.{}", facing.stable_id()))],
        boundary: format!("bounded {}", facing.stable_id()),
        returned_evidence: Vec::new(),
    }
}

fn title_core() -> StonebendTitleCore {
    let title_id = title("title.stonebend.third-pass.fixture");
    let mut core = StonebendTitleCore::new(
        title_id.clone(),
        id("subject.stonebend.third-pass.fixture"),
        name("name.stonebend.third-pass.fixture"),
        id("claim.stonebend.third-pass.fixture"),
        vec![evidence("evidence.stonebend.third-pass.core")],
        "bounded ordinary professional Title",
    );
    for facing in StonebendGateFacing::ALL {
        core.record_scope(scope_record(&title_id, facing))
            .expect("valid fixture scope");
    }
    core
}

fn lifecycle_policy() -> TitleLifecyclePolicy {
    TitleLifecyclePolicy {
        identity: id("policy.stonebend.third-pass.fixture"),
        required_pre_recognition_stages: [
            TitleLifecycleStage::NameEstablished,
            TitleLifecycleStage::ClaimPresented,
            TitleLifecycleStage::ApplicationSubmitted,
            TitleLifecycleStage::EvidenceAssembled,
            TitleLifecycleStage::EligibilityReviewed,
            TitleLifecycleStage::GateReviewed,
        ]
        .into_iter()
        .collect(),
        required_gate_scopes: GateScope::ALL.into_iter().collect(),
        activation_mode: ActivationMode::Explicit,
        activation_requirements: [
            ActivationRequirement::GateScope(GateScope::FormationRecognition),
            ActivationRequirement::GateScope(GateScope::PublicCirculation),
            ActivationRequirement::GateScope(GateScope::OperationalDeployment),
            ActivationRequirement::ResponsibilityAccepted,
        ]
        .into_iter()
        .collect(),
        maintenance_requirements: [
            TitleMaintenanceRequirement::ContinuingCompetence,
            TitleMaintenanceRequirement::LawfulOperation,
            TitleMaintenanceRequirement::PublicAccountability,
        ]
        .into_iter()
        .collect(),
        renewal: TitleRenewalPolicy::ConditionBased {
            policy: id("renewal.policy.stonebend.third-pass.fixture"),
            condition: "review after the recognized work interval ends".into(),
            expires_without_renewal: true,
        },
        challenge_period: None,
        succession_supported: false,
        direct_restoration_allowed: true,
        permitted_terminal_dispositions: [
            TitleTerminalDisposition::HonorableCompletion,
            TitleTerminalDisposition::Surrender,
            TitleTerminalDisposition::Expiration,
            TitleTerminalDisposition::Death,
            TitleTerminalDisposition::EndOfForm,
            TitleTerminalDisposition::Succession,
            TitleTerminalDisposition::RemovalForFailure,
            TitleTerminalDisposition::RemovalForFraud,
            TitleTerminalDisposition::RemovalForIllegality,
            TitleTerminalDisposition::ConstitutionalDissolution,
            TitleTerminalDisposition::Supersession,
        ]
        .into_iter()
        .collect(),
    }
}

fn stage_record(stage: TitleLifecycleStage) -> TitleStageRecord {
    TitleStageRecord {
        identity: id(&format!(
            "stage.stonebend.fixture.{}",
            stage.semantic_order()
        )),
        title: title("title.stonebend.third-pass.fixture"),
        stage,
        evidence: vec![evidence(&format!(
            "evidence.stonebend.fixture.stage.{}",
            stage.semantic_order()
        ))],
        sequence: u64::from(stage.semantic_order()),
    }
}

fn recognized_lifecycle(reverse: bool) -> StonebendTitleLifecycle {
    let policy = lifecycle_policy();
    let mut lifecycle =
        StonebendTitleLifecycle::new(title_core(), policy).expect("valid lifecycle policy");
    let mut stages = lifecycle
        .policy
        .required_pre_recognition_stages
        .iter()
        .copied()
        .collect::<Vec<_>>();
    if reverse {
        stages.reverse();
    }
    for stage in stages {
        lifecycle
            .record_stage(stage_record(stage))
            .expect("valid preparatory record");
    }
    lifecycle
        .recognize(TitleRecognition {
            identity: id("recognition.stonebend.third-pass.fixture"),
            title: lifecycle.core.title.clone(),
            authority: id("authority.stonebend.fixture-freemason"),
            evidence: vec![evidence("evidence.stonebend.fixture-recognition")],
            boundary: "recognized ordinary professional identity".into(),
            sequence: 20,
        })
        .expect("lawful recognition");
    lifecycle
}

fn active_lifecycle(reverse: bool) -> StonebendTitleLifecycle {
    let mut lifecycle = recognized_lifecycle(reverse);
    lifecycle
        .activate(TitleActivation {
            identity: id("activation.stonebend.third-pass.fixture"),
            title: lifecycle.core.title.clone(),
            satisfied_requirements: lifecycle.policy.activation_requirements.clone(),
            authority: id("authority.stonebend.fixture-activation"),
            evidence: vec![evidence("evidence.stonebend.fixture-activation")],
            accepted_responsibility: true,
            term: id("term.stonebend.third-pass.first"),
            sequence: 21,
        })
        .expect("lawful activation");
    lifecycle
}

fn maintenance_record() -> TitleMaintenanceRecord {
    TitleMaintenanceRecord {
        identity: id("maintenance.stonebend.third-pass.first"),
        title: title("title.stonebend.third-pass.fixture"),
        term: id("term.stonebend.third-pass.first"),
        subject: id("subject.stonebend.third-pass.fixture"),
        satisfied_requirements: lifecycle_policy().maintenance_requirements,
        evidence: vec![evidence("evidence.stonebend.fixture-maintenance")],
        reviewed_by: id("reviewer.stonebend.fixture-maintenance"),
        sequence: 30,
    }
}

fn renewal_record(disposition: TitleRenewalDisposition) -> TitleRenewalRecord {
    let renewed = matches!(
        disposition,
        TitleRenewalDisposition::Renewed | TitleRenewalDisposition::RenewedWithLimitations
    );
    TitleRenewalRecord {
        identity: id("renewal.stonebend.third-pass.first"),
        title: title("title.stonebend.third-pass.fixture"),
        current_term: id("term.stonebend.third-pass.first"),
        renewal_policy: id("renewal.policy.stonebend.third-pass.fixture"),
        evidence: vec![evidence("evidence.stonebend.fixture-renewal")],
        gate_scopes_reviewed: GateScope::ALL.into_iter().collect(),
        maintenance_records: vec![id("maintenance.stonebend.third-pass.first")],
        known_failures: Vec::new(),
        known_remediation: Vec::new(),
        yield_evidence: vec![evidence("evidence.stonebend.fixture-renewal-yield")],
        renewal_authority: id("authority.stonebend.fixture-renewal"),
        disposition,
        renewed_boundaries: "same bounded professional practice".into(),
        effective_sequence: 40,
        next_term: renewed.then(|| id("term.stonebend.third-pass.second")),
        prior_term_tombstone: renewed
            .then(|| tombstone("tombstone.stonebend.third-pass.first-term")),
    }
}

fn intervention(
    identity: &str,
    target: TitleInterventionTarget,
    kind: TitleInterventionKind,
    failure: GateFailureKind,
) -> TitleIntervention {
    TitleIntervention {
        identity: id(identity),
        title: title("title.stonebend.third-pass.fixture"),
        target,
        kind,
        failure,
        evidence: vec![evidence(&format!("evidence.{identity}"))],
        boundary: "bounded constitutional intervention".into(),
        supervision: (kind == TitleInterventionKind::Supervision).then(|| SupervisionTerms {
            supervising_authority: id("authority.stonebend.fixture-supervisor"),
            supervised_scope: "ordinary work under observation".into(),
            required_evidence: vec![evidence("evidence.stonebend.fixture-supervision")],
            completion_or_review_condition: "review after demonstrated competence".into(),
        }),
        remediation_condition: (kind == TitleInterventionKind::Remediation)
            .then(|| "complete the recorded correction".into()),
        referral: (failure == GateFailureKind::Illegality)
            .then_some(ConstitutionalReferral::FutureCourt),
        core_challenge: None,
        sequence: 50,
    }
}

fn populated_network() -> ProliteriateNetwork {
    let mut network = ProliteriateNetwork::default();
    for (identity, kind) in [
        (
            "node.stonebend.third-pass.district",
            ProliteriateNodeKind::District,
        ),
        (
            "node.stonebend.third-pass.workshop",
            ProliteriateNodeKind::GuildOrWorkshop,
        ),
    ] {
        network
            .add_node(ProliteriateNode {
                identity: id(identity),
                kind,
                name: identity.into(),
            })
            .expect("valid node");
    }
    network
}

fn network_mandate(identity: &str) -> NetworkMandate {
    NetworkMandate {
        identity: id(identity),
        participating_nodes: BTreeSet::from([id("node.stonebend.third-pass.district")]),
        issue_claim: id("claim.stonebend.third-pass-network"),
        affected_yield: YieldRecord {
            identity: id("yield.stonebend.third-pass-network"),
            issue: id("issue.stonebend.third-pass-network"),
            impacts: vec![YieldImpact {
                description: "ordinary public burden".into(),
                beneficiaries: BTreeSet::from([id("being.stonebend.beneficiary")]),
                burden_bearers: BTreeSet::from([id("being.stonebend.burden-bearer")]),
            }],
            evidence: vec![evidence("evidence.stonebend.network-yield")],
        },
        testimony: vec![evidence("evidence.stonebend.network-testimony")],
        authorities: BTreeSet::from([
            hollow_grove::world::stonebend::second_pass::MandateAuthority::PresentYield,
        ]),
        boundary: "present this issue-specific Yield".into(),
        active_witness: None,
        recalls: Vec::new(),
        completed: false,
        invalidated: false,
    }
}

#[test]
fn claim_is_not_automatically_a_title() {
    let lifecycle =
        StonebendTitleLifecycle::new(title_core(), lifecycle_policy()).expect("valid policy");
    assert_eq!(lifecycle.state, TitleExerciseState::ClaimPending);
    assert!(lifecycle.recognition().is_none());
    assert!(!lifecycle.may_exercise());
}

#[test]
fn recognition_and_activation_are_distinct() {
    let lifecycle = recognized_lifecycle(false);
    assert_eq!(lifecycle.state, TitleExerciseState::RecognizedInactive);
    assert!(lifecycle.recognition().is_some());
    assert!(lifecycle.activation().is_none());
    assert!(!lifecycle.may_exercise());
}

#[test]
fn activation_requires_recognition_and_policy_requirements() {
    let mut lifecycle =
        StonebendTitleLifecycle::new(title_core(), lifecycle_policy()).expect("valid policy");
    let activation = TitleActivation {
        identity: id("activation.stonebend.invalid"),
        title: lifecycle.core.title.clone(),
        satisfied_requirements: BTreeSet::new(),
        authority: id("authority.stonebend.invalid"),
        evidence: vec![evidence("evidence.stonebend.invalid")],
        accepted_responsibility: true,
        term: id("term.stonebend.invalid"),
        sequence: 1,
    };
    assert!(lifecycle.activate(activation).is_err());
}

#[test]
fn stage_order_is_semantic_and_insertion_independent() {
    let forward = recognized_lifecycle(false);
    let reverse = recognized_lifecycle(true);
    let forward_stages = forward
        .ordered_stage_records()
        .into_iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    let reverse_stages = reverse
        .ordered_stage_records()
        .into_iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    assert_eq!(forward_stages, reverse_stages);
    assert!(
        forward_stages
            .windows(2)
            .all(|pair| { pair[0].semantic_order() < pair[1].semantic_order() })
    );
}

#[test]
fn missing_required_stage_blocks_recognition() {
    let mut lifecycle =
        StonebendTitleLifecycle::new(title_core(), lifecycle_policy()).expect("valid policy");
    for stage in [
        TitleLifecycleStage::NameEstablished,
        TitleLifecycleStage::ClaimPresented,
        TitleLifecycleStage::EvidenceAssembled,
        TitleLifecycleStage::EligibilityReviewed,
        TitleLifecycleStage::GateReviewed,
    ] {
        lifecycle.record_stage(stage_record(stage)).unwrap();
    }
    assert!(
        lifecycle
            .recognize(TitleRecognition {
                identity: id("recognition.stonebend.missing-stage"),
                title: lifecycle.core.title.clone(),
                authority: id("authority.stonebend.fixture"),
                evidence: vec![evidence("evidence.stonebend.missing-stage")],
                boundary: "bounded".into(),
                sequence: 20,
            })
            .is_err()
    );
}

#[test]
fn minimal_policy_may_omit_optional_stages() {
    let mut policy = lifecycle_policy();
    policy.required_pre_recognition_stages = [
        TitleLifecycleStage::NameEstablished,
        TitleLifecycleStage::ClaimPresented,
        TitleLifecycleStage::EvidenceAssembled,
    ]
    .into_iter()
    .collect();
    policy.required_gate_scopes.clear();
    policy.activation_requirements =
        BTreeSet::from([ActivationRequirement::ResponsibilityAccepted]);
    let mut lifecycle = StonebendTitleLifecycle::new(title_core(), policy).unwrap();
    for stage in [
        TitleLifecycleStage::EvidenceAssembled,
        TitleLifecycleStage::NameEstablished,
        TitleLifecycleStage::ClaimPresented,
    ] {
        lifecycle.record_stage(stage_record(stage)).unwrap();
    }
    lifecycle
        .recognize(TitleRecognition {
            identity: id("recognition.stonebend.minimal"),
            title: lifecycle.core.title.clone(),
            authority: id("authority.stonebend.minimal"),
            evidence: vec![evidence("evidence.stonebend.minimal")],
            boundary: "minimal bounded Title".into(),
            sequence: 8,
        })
        .unwrap();
    assert_eq!(lifecycle.state, TitleExerciseState::RecognizedInactive);
}

#[test]
fn challenge_period_is_policy_defined_not_universal() {
    let mut policy = lifecycle_policy();
    policy
        .required_pre_recognition_stages
        .insert(TitleLifecycleStage::ChallengePeriod);
    policy.challenge_period = Some(ChallengePeriodPolicy {
        eligible_challengers: "affected people with evidence".into(),
        affected_nodes: BTreeSet::new(),
        closing_condition: "all filed challenges receive disposition".into(),
        activation_stayed: true,
        resolving_authority: id("authority.stonebend.challenge-period"),
    });
    assert!(policy.validate().is_ok());
    assert!(lifecycle_policy().challenge_period.is_none());
}

#[test]
fn stable_title_identity_survives_recognition_and_activation() {
    let lifecycle = active_lifecycle(false);
    assert_eq!(
        lifecycle.core.title,
        title("title.stonebend.third-pass.fixture")
    );
    assert_eq!(lifecycle.activation().unwrap().title, lifecycle.core.title);
}

#[test]
fn maintenance_is_distinct_from_renewal() {
    let mut lifecycle = active_lifecycle(false);
    lifecycle
        .record_maintenance(maintenance_record())
        .expect("maintenance recorded");
    assert_eq!(lifecycle.maintenance_records().len(), 1);
    assert!(lifecycle.renewal_records().is_empty());
}

#[test]
fn renewal_preserves_title_and_creates_a_new_term_not_title() {
    let mut lifecycle = active_lifecycle(false);
    lifecycle.record_maintenance(maintenance_record()).unwrap();
    lifecycle
        .record_renewal(renewal_record(TitleRenewalDisposition::Renewed))
        .unwrap();
    assert_eq!(
        lifecycle.core.title.as_str(),
        "title.stonebend.third-pass.fixture"
    );
    assert_eq!(lifecycle.terms().len(), 2);
    assert_eq!(lifecycle.renewal_records().len(), 1);
    assert_eq!(lifecycle.tombstones().len(), 1);
}

#[test]
fn renewal_rejection_preserves_previous_term_history() {
    let mut lifecycle = active_lifecycle(false);
    lifecycle.record_maintenance(maintenance_record()).unwrap();
    lifecycle
        .record_renewal(renewal_record(TitleRenewalDisposition::Rejected))
        .unwrap();
    let term = lifecycle
        .terms()
        .get(&id("term.stonebend.third-pass.first"))
        .unwrap();
    assert!(term.ended_at.is_none());
    assert_eq!(lifecycle.renewal_records().len(), 1);
}

#[test]
fn title_without_renewal_policy_rejects_renewal_record() {
    let mut policy = lifecycle_policy();
    policy.renewal = TitleRenewalPolicy::NotRequired;
    let mut lifecycle = StonebendTitleLifecycle::new(title_core(), policy).unwrap();
    for stage in lifecycle.policy.required_pre_recognition_stages.clone() {
        lifecycle.record_stage(stage_record(stage)).unwrap();
    }
    lifecycle
        .recognize(TitleRecognition {
            identity: id("recognition.stonebend.no-renewal"),
            title: lifecycle.core.title.clone(),
            authority: id("authority.stonebend.no-renewal"),
            evidence: vec![evidence("evidence.stonebend.no-renewal")],
            boundary: "bounded".into(),
            sequence: 20,
        })
        .unwrap();
    lifecycle
        .activate(TitleActivation {
            identity: id("activation.stonebend.no-renewal"),
            title: lifecycle.core.title.clone(),
            satisfied_requirements: lifecycle.policy.activation_requirements.clone(),
            authority: id("authority.stonebend.no-renewal"),
            evidence: vec![evidence("evidence.stonebend.no-renewal-activation")],
            accepted_responsibility: true,
            term: id("term.stonebend.third-pass.first"),
            sequence: 21,
        })
        .unwrap();
    lifecycle.record_maintenance(maintenance_record()).unwrap();
    assert!(
        lifecycle
            .record_renewal(renewal_record(TitleRenewalDisposition::Renewed))
            .is_err()
    );
}

#[test]
fn scope_suspension_is_independent_and_core_survives() {
    let mut lifecycle = active_lifecycle(false);
    let core_id = lifecycle.core.title.clone();
    lifecycle
        .apply_intervention(intervention(
            "intervention.stonebend.operational-suspension",
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Suspension,
            GateFailureKind::HonestFailure,
        ))
        .unwrap();
    assert!(!lifecycle.core.authorizes(GateScope::OperationalDeployment));
    assert!(lifecycle.core.authorizes(GateScope::FormationRecognition));
    assert!(lifecycle.core.authorizes(GateScope::PublicCirculation));
    assert_eq!(lifecycle.core.title, core_id);
}

#[test]
fn scope_removal_does_not_destroy_core_title() {
    let mut lifecycle = active_lifecycle(false);
    let original = lifecycle.core.title.clone();
    lifecycle
        .apply_intervention(intervention(
            "intervention.stonebend.scope-removal",
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Removal,
            GateFailureKind::Fraud,
        ))
        .unwrap();
    assert_eq!(lifecycle.core.title, original);
    assert_eq!(
        lifecycle
            .core
            .scope(GateScope::OperationalDeployment)
            .unwrap()
            .disposition,
        TitleScopeDisposition::Removed
    );
    assert!(lifecycle.core.authorizes(GateScope::FormationRecognition));
}

#[test]
fn core_title_removal_requires_explicit_tombstone_path() {
    let mut lifecycle = active_lifecycle(false);
    assert!(
        lifecycle
            .apply_intervention(intervention(
                "intervention.stonebend.core-removal",
                TitleInterventionTarget::CoreTitle,
                TitleInterventionKind::Removal,
                GateFailureKind::Fraud,
            ))
            .is_err()
    );
    assert_eq!(lifecycle.state, TitleExerciseState::Active);
}

#[test]
fn limitation_is_distinct_from_suspension() {
    let mut limited = active_lifecycle(false);
    limited
        .apply_intervention(intervention(
            "intervention.stonebend.limitation",
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Limitation,
            GateFailureKind::Negligence,
        ))
        .unwrap();
    assert_eq!(
        limited
            .core
            .scope(GateScope::OperationalDeployment)
            .unwrap()
            .disposition,
        TitleScopeDisposition::Limited
    );
    assert!(limited.core.authorizes(GateScope::OperationalDeployment));

    let mut suspended = active_lifecycle(false);
    suspended
        .apply_intervention(intervention(
            "intervention.stonebend.suspension",
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Suspension,
            GateFailureKind::Negligence,
        ))
        .unwrap();
    assert!(!suspended.core.authorizes(GateScope::OperationalDeployment));
}

#[test]
fn supervision_requires_explicit_terms_but_does_not_remove() {
    let mut lifecycle = active_lifecycle(false);
    lifecycle
        .apply_intervention(intervention(
            "intervention.stonebend.supervision",
            TitleInterventionTarget::Activation,
            TitleInterventionKind::Supervision,
            GateFailureKind::Negligence,
        ))
        .unwrap();
    assert!(lifecycle.may_exercise());
    assert_eq!(lifecycle.interventions().len(), 1);
}

#[test]
fn remediation_requires_a_completion_condition() {
    let mut lifecycle = active_lifecycle(false);
    let mut record = intervention(
        "intervention.stonebend.remediation",
        TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
        TitleInterventionKind::Remediation,
        GateFailureKind::Negligence,
    );
    record.remediation_condition = None;
    assert!(lifecycle.apply_intervention(record).is_err());
}

#[test]
fn honest_failure_cannot_automatically_remove_title() {
    let mut lifecycle = active_lifecycle(false);
    assert!(
        lifecycle
            .apply_intervention(intervention(
                "intervention.stonebend.honest-removal",
                TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
                TitleInterventionKind::Removal,
                GateFailureKind::HonestFailure,
            ))
            .is_err()
    );
}

#[test]
fn five_failure_classes_remain_distinct_and_proportionate() {
    let mut honest = active_lifecycle(false);
    honest
        .apply_intervention(intervention(
            "intervention.failure.honest",
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Remediation,
            GateFailureKind::HonestFailure,
        ))
        .unwrap();

    let mut negligent = active_lifecycle(false);
    negligent
        .apply_intervention(intervention(
            "intervention.failure.negligence",
            TitleInterventionTarget::Activation,
            TitleInterventionKind::Supervision,
            GateFailureKind::Negligence,
        ))
        .unwrap();

    let mut fraud = active_lifecycle(false);
    fraud
        .apply_intervention(intervention(
            "intervention.failure.fraud",
            TitleInterventionTarget::GateScope(GateScope::PublicCirculation),
            TitleInterventionKind::Suspension,
            GateFailureKind::Fraud,
        ))
        .unwrap();

    let mut illegal = active_lifecycle(false);
    illegal
        .apply_intervention(intervention(
            "intervention.failure.illegality",
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Suspension,
            GateFailureKind::Illegality,
        ))
        .unwrap();

    let mut hollow = active_lifecycle(false);
    let challenge_id = challenge("challenge.stonebend.fixture-hollowness");
    hollow.core.open_core_challenge(challenge_id.clone());
    let mut hollow_record = intervention(
        "intervention.failure.hollowness",
        TitleInterventionTarget::CoreTitle,
        TitleInterventionKind::Suspension,
        GateFailureKind::ConstitutionalHollowness,
    );
    hollow_record.core_challenge = Some(challenge_id);
    hollow.apply_intervention(hollow_record).unwrap();

    let failures = [honest, negligent, fraud, illegal, hollow]
        .into_iter()
        .map(|record| record.interventions().values().next().unwrap().failure)
        .collect::<BTreeSet<_>>();
    assert_eq!(failures.len(), 5);
}

#[test]
fn illegality_requires_referral_without_implementing_criminal_process() {
    let mut lifecycle = active_lifecycle(false);
    let mut record = intervention(
        "intervention.stonebend.illegal-no-referral",
        TitleInterventionTarget::Activation,
        TitleInterventionKind::Suspension,
        GateFailureKind::Illegality,
    );
    record.referral = None;
    assert!(lifecycle.apply_intervention(record).is_err());
}

#[test]
fn constitutional_hollowness_requires_explicit_core_review() {
    let mut lifecycle = active_lifecycle(false);
    assert!(
        lifecycle
            .apply_intervention(intervention(
                "intervention.stonebend.hollow-no-review",
                TitleInterventionTarget::CoreTitle,
                TitleInterventionKind::Suspension,
                GateFailureKind::ConstitutionalHollowness,
            ))
            .is_err()
    );
}

#[test]
fn restoration_preserves_interruption_and_stable_identity() {
    let mut lifecycle = active_lifecycle(false);
    let original = lifecycle.core.title.clone();
    let intervention_id = "intervention.stonebend.restore-scope";
    lifecycle
        .apply_intervention(intervention(
            intervention_id,
            TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            TitleInterventionKind::Suspension,
            GateFailureKind::Negligence,
        ))
        .unwrap();
    lifecycle
        .restore(RestorationRecord {
            identity: id("restoration.stonebend.operational"),
            title: original.clone(),
            target: TitleInterventionTarget::GateScope(GateScope::OperationalDeployment),
            interruption: id(intervention_id),
            cause: GateFailureKind::Negligence,
            remediation_evidence: vec![evidence("evidence.stonebend.restoration")],
            reviewing_authority: id("authority.stonebend.restoration"),
            restored_scopes: BTreeSet::from([GateScope::OperationalDeployment]),
            continuing_limitations: vec!["no unreviewed high-risk deployment".into()],
            new_effective_boundary: "limited operational deployment".into(),
            sequence: 60,
        })
        .unwrap();
    assert_eq!(lifecycle.core.title, original);
    assert_eq!(lifecycle.interventions().len(), 1);
    assert_eq!(lifecycle.restorations().len(), 1);
    assert_eq!(
        lifecycle
            .core
            .scope(GateScope::OperationalDeployment)
            .unwrap()
            .disposition,
        TitleScopeDisposition::Limited
    );
}

#[test]
fn removed_principal_bearers_and_witnesses_require_full_return_paths() {
    assert_eq!(
        required_return_path(&RestrictedRestorationSubject::RemovedHypergiant(id(
            "being.stonebend.former-hypergiant"
        ))),
        RequiredReturnPath::CompleteLazerhornSuccession
    );
    assert_eq!(
        required_return_path(&RestrictedRestorationSubject::RemovedFreemason(id(
            "being.stonebend.former-freemason"
        ))),
        RequiredReturnPath::IndependentForgeReplacement
    );
    assert_eq!(
        required_return_path(&RestrictedRestorationSubject::RecalledProliteriateWitness(
            id("being.stonebend.former-witness")
        )),
        RequiredReturnPath::NewNetworkMandate
    );
}

#[test]
fn terminal_dispositions_are_distinct() {
    let dispositions = [
        TitleTerminalDisposition::HonorableCompletion,
        TitleTerminalDisposition::Surrender,
        TitleTerminalDisposition::Expiration,
        TitleTerminalDisposition::Death,
        TitleTerminalDisposition::EndOfForm,
        TitleTerminalDisposition::Succession,
        TitleTerminalDisposition::RemovalForFailure,
        TitleTerminalDisposition::RemovalForFraud,
        TitleTerminalDisposition::RemovalForIllegality,
        TitleTerminalDisposition::ConstitutionalDissolution,
        TitleTerminalDisposition::Supersession,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(dispositions.len(), 11);
}

#[test]
fn honorable_completion_creates_tombstone_without_punishment() {
    let mut lifecycle = active_lifecycle(false);
    lifecycle
        .end_active_term(TitleTermTombstone {
            record: tombstone("tombstone.stonebend.honorable"),
            title: lifecycle.core.title.clone(),
            term: id("term.stonebend.third-pass.first"),
            disposition: TitleTerminalDisposition::HonorableCompletion,
            sequence: 80,
            evidence: vec![evidence("evidence.stonebend.honorable")],
            successor: None,
        })
        .unwrap();
    assert_eq!(lifecycle.state, TitleExerciseState::Ended);
    assert_eq!(
        lifecycle.tombstones()[0].disposition,
        TitleTerminalDisposition::HonorableCompletion
    );
}

#[test]
fn diamond_vacancy_has_no_hidden_bearer() {
    let diamond = hollow_grove::world::stonebend::second_pass::DiamondState::default();
    assert!(diamond_vacancy_has_no_bearer(&diamond));
    assert!(diamond.active_tenure.is_none());
}

fn continuity_mandate() -> DiamondContinuityMandate {
    let diamond = hollow_grove::world::stonebend::second_pass::DiamondState::default();
    let administrator = id("administrator.stonebend.vacancy");
    DiamondContinuityMandate::open(
        &diamond,
        id("mandate.stonebend.vacancy"),
        tombstone("tombstone.stonebend.vacancy-source"),
        evidence("evidence.stonebend.existing-law"),
        BTreeSet::from([
            ContinuityActionKind::PreservePublicRecords,
            ContinuityActionKind::DefendImmediatelyThreatenedBoundary,
            ContinuityActionKind::CompleteRoutineGateProcessing,
        ]),
        administrator.clone(),
        Some(StonebendGateFacing::Flynt),
        "Diamond is lawfully vacant",
        ContinuityTerminationCondition::EmergencyResolved,
        vec![evidence("evidence.stonebend.vacancy")],
        canonical_vacancy_dimension_traces(&administrator),
    )
    .expect("lawful bounded continuity mandate")
}

#[test]
fn vacancy_continuity_is_bounded_and_not_diamond() {
    let mandate = continuity_mandate();
    assert_eq!(
        mandate.diamond,
        hollow_grove::world::stonebend::second_pass::diamond_title_id()
    );
    assert_eq!(mandate.status, ContinuityMandateStatus::Active);
    assert!(
        !mandate
            .permitted_actions
            .contains(&ContinuityActionKind::InvestDiamond)
    );
    assert!(
        !mandate
            .permitted_actions
            .contains(&ContinuityActionKind::AppointHypergiant)
    );
}

#[test]
fn continuity_mandate_cannot_open_while_diamond_is_borne() {
    use hollow_grove::world::stonebend::SuccessionRecordId;
    use hollow_grove::world::stonebend::second_pass::{DiamondTenure, DiamondTenureStatus};
    let mut diamond = hollow_grove::world::stonebend::second_pass::DiamondState::default();
    diamond
        .invest(DiamondTenure {
            identity: id("tenure.stonebend.active"),
            diamond: diamond.title.clone(),
            bearer: id("being.stonebend.hypergiant"),
            supporting_claim: id("claim.stonebend.diamond"),
            succession: SuccessionRecordId::new("succession.stonebend.active").unwrap(),
            began_at: 1,
            status: DiamondTenureStatus::Active,
        })
        .unwrap();
    let administrator = id("administrator.stonebend.invalid-vacancy");
    assert!(
        DiamondContinuityMandate::open(
            &diamond,
            id("mandate.stonebend.invalid-vacancy"),
            tombstone("tombstone.stonebend.invalid-vacancy"),
            evidence("evidence.stonebend.invalid-vacancy"),
            BTreeSet::from([ContinuityActionKind::PreservePublicRecords]),
            administrator.clone(),
            None,
            "invalid",
            ContinuityTerminationCondition::ExistingDutyCompleted,
            vec![evidence("evidence.stonebend.invalid")],
            canonical_vacancy_dimension_traces(&administrator),
        )
        .is_err()
    );
}

#[test]
fn emergency_action_is_recorded_reviewable_and_terminates() {
    let mut mandate = continuity_mandate();
    mandate
        .record_action(EmergencyContinuityAction {
            identity: id("action.stonebend.vacancy-defense"),
            mandate: mandate.identity.clone(),
            emergency: "immediate boundary failure".into(),
            constitutional_basis: "existing delegated boundary defense law".into(),
            action: ContinuityActionKind::DefendImmediatelyThreatenedBoundary,
            responsible_authority: id("administrator.stonebend.vacancy"),
            affected_gate: Some(StonebendGateFacing::Flynt),
            evidence: vec![evidence("evidence.stonebend.vacancy-defense")],
            termination_condition: ContinuityTerminationCondition::EmergencyResolved,
            later_review_required: true,
        })
        .unwrap();
    assert_eq!(mandate.actions().len(), 1);
    mandate
        .terminate(
            ContinuityTerminationCondition::EmergencyResolved,
            evidence("evidence.stonebend.vacancy-review"),
        )
        .unwrap();
    assert_eq!(mandate.status, ContinuityMandateStatus::Terminated);
}

#[test]
fn emergency_cannot_appoint_hypergiant_or_expand_permanently() {
    let diamond = hollow_grove::world::stonebend::second_pass::DiamondState::default();
    let administrator = id("administrator.stonebend.forbidden-emergency");
    for forbidden in [
        ContinuityActionKind::AppointHypergiant,
        ContinuityActionKind::InvestDiamond,
        ContinuityActionKind::PermanentlyExpandGateScope,
        ContinuityActionKind::CreateSovereignLaw,
    ] {
        let label = match forbidden {
            ContinuityActionKind::AppointHypergiant => "appoint-hypergiant",
            ContinuityActionKind::InvestDiamond => "invest-diamond",
            ContinuityActionKind::PermanentlyExpandGateScope => "expand-gate-scope",
            ContinuityActionKind::CreateSovereignLaw => "create-sovereign-law",
            _ => unreachable!("fixture contains only forbidden sovereign actions"),
        };
        assert!(
            DiamondContinuityMandate::open(
                &diamond,
                id(&format!("mandate.stonebend.forbidden.{label}")),
                tombstone(&format!("tombstone.stonebend.forbidden.{label}")),
                evidence(&format!("evidence.stonebend.forbidden.{label}")),
                BTreeSet::from([forbidden]),
                administrator.clone(),
                None,
                "vacancy",
                ContinuityTerminationCondition::EmergencyResolved,
                vec![evidence("evidence.stonebend.forbidden")],
                canonical_vacancy_dimension_traces(&administrator),
            )
            .is_err()
        );
    }
}

fn old_freemason_state() -> FreemasonOfficeState {
    FreemasonOfficeState::from_active(
        FreemasonTenure {
            identity: id("tenure.stonebend.freemason-old"),
            bearer: id("being.stonebend.freemason-old"),
            claim: id("claim.stonebend.freemason-old"),
            seal: seal("seal.stonebend.freemason-old"),
            began_at: 1,
            status: FreemasonTenureStatus::Active,
        },
        BTreeSet::from([
            FreemasonRecordRef::Claim(id("claim.stonebend.prior-valid")),
            FreemasonRecordRef::Seal(seal("seal.stonebend.prior-valid")),
        ]),
    )
    .unwrap()
}

fn freemason_claim(candidate: &str) -> FreemasonSuccessionClaim {
    FreemasonSuccessionClaim {
        identity: id(&format!(
            "claim.stonebend.freemason-replacement.{candidate}"
        )),
        candidate: id(candidate),
        qualification_evidence: vec![evidence(&format!("evidence.{candidate}.qualification"))],
        craft_and_constitutional_evidence: vec![evidence(&format!(
            "evidence.{candidate}.constitutional"
        ))],
        prior_seal_competence: vec![evidence(&format!("evidence.{candidate}.seal-work"))],
        conflict_disclosures: vec![evidence(&format!("evidence.{candidate}.conflicts"))],
        outgoing_recommendation: Some(evidence(&format!("evidence.{candidate}.recommendation"))),
    }
}

fn add_review(
    state: &mut FreemasonOfficeState,
    candidate: &str,
    disposition: ForgeReviewDisposition,
) {
    state
        .record_independent_review(IndependentForgeReview {
            identity: id(&format!("review.stonebend.forge.{candidate}")),
            claim: id(&format!(
                "claim.stonebend.freemason-replacement.{candidate}"
            )),
            candidate: id(candidate),
            examiners: BTreeSet::from([
                id("examiner.stonebend.forge-one"),
                id("examiner.stonebend.forge-two"),
            ]),
            evidence: vec![evidence(&format!("evidence.{candidate}.forge-review"))],
            disposition,
        })
        .unwrap();
}

fn replacement_process(reverse: bool) -> FreemasonOfficeState {
    let mut state = old_freemason_state();
    state
        .end_active_tenure(
            tombstone("tombstone.stonebend.freemason-old"),
            TitleTerminalDisposition::Succession,
            10,
            vec![evidence("evidence.stonebend.freemason-old-ending")],
            None,
        )
        .unwrap();
    let mut candidates = vec![
        freemason_claim("being.stonebend.candidate-qualified"),
        freemason_claim("being.stonebend.candidate-remediate"),
    ];
    if reverse {
        candidates.reverse();
    }
    for candidate in candidates {
        state.present_candidate(candidate).unwrap();
    }
    add_review(
        &mut state,
        "being.stonebend.candidate-qualified",
        ForgeReviewDisposition::Qualified,
    );
    add_review(
        &mut state,
        "being.stonebend.candidate-remediate",
        ForgeReviewDisposition::RemediationRequired,
    );
    state
}

#[test]
fn freemason_vacancy_preserves_records_and_tombstone() {
    let mut state = old_freemason_state();
    state
        .end_active_tenure(
            tombstone("tombstone.stonebend.freemason-vacancy"),
            TitleTerminalDisposition::HonorableCompletion,
            10,
            vec![evidence("evidence.stonebend.freemason-vacancy")],
            None,
        )
        .unwrap();
    assert!(state.is_vacant());
    assert_eq!(state.ended_tenures.len(), 1);
    assert!(
        state
            .preserved_records
            .contains(&FreemasonRecordRef::Seal(seal(
                "seal.stonebend.prior-valid"
            )))
    );
}

#[test]
fn replacement_freemason_cannot_self_certify() {
    let candidate = id("being.stonebend.self-certifier");
    let review = IndependentForgeReview {
        identity: id("review.stonebend.self-certifier"),
        claim: id("claim.stonebend.self-certifier"),
        candidate: candidate.clone(),
        examiners: BTreeSet::from([candidate]),
        evidence: vec![evidence("evidence.stonebend.self-certifier")],
        disposition: ForgeReviewDisposition::Qualified,
    };
    assert!(review.validate().is_err());
}

#[test]
fn outgoing_recommendation_is_evidence_not_appointment() {
    let state = replacement_process(false);
    assert!(state.is_vacant());
    assert!(
        state
            .candidates()
            .values()
            .all(|claim| claim.outgoing_recommendation.is_some())
    );
    assert!(state.active_tenure.is_none());
}

#[test]
fn replacement_selection_is_insertion_order_independent_and_singular() {
    for reverse in [false, true] {
        let mut state = replacement_process(reverse);
        state
            .invest_replacement(
                FreemasonInvestiture {
                    identity: id("tenure.stonebend.freemason-new"),
                    claim: id(
                        "claim.stonebend.freemason-replacement.being.stonebend.candidate-qualified",
                    ),
                    candidate: id("being.stonebend.candidate-qualified"),
                    independent_review: id(
                        "review.stonebend.forge.being.stonebend.candidate-qualified",
                    ),
                    proliteriate_yield_hearing: evidence(
                        "evidence.stonebend.freemason-yield-hearing",
                    ),
                    diamond_boundary_recognition: None,
                    active_diamond_bearer: None,
                    seal: seal("seal.stonebend.freemason-new"),
                    evidence: vec![evidence("evidence.stonebend.freemason-investiture")],
                    began_at: 20,
                },
                None,
                &BTreeSet::new(),
            )
            .unwrap();
        assert_eq!(
            state.active_tenure.as_ref().unwrap().bearer,
            id("being.stonebend.candidate-qualified")
        );
        assert!(
            state
                .invest_replacement(
                    FreemasonInvestiture {
                        identity: id("tenure.stonebend.freemason-second"),
                        claim: id(
                            "claim.stonebend.freemason-replacement.being.stonebend.candidate-qualified",
                        ),
                        candidate: id("being.stonebend.candidate-qualified"),
                        independent_review: id(
                            "review.stonebend.forge.being.stonebend.candidate-qualified",
                        ),
                        proliteriate_yield_hearing: evidence(
                            "evidence.stonebend.freemason-yield-hearing",
                        ),
                        diamond_boundary_recognition: None,
                        active_diamond_bearer: None,
                        seal: seal("seal.stonebend.freemason-second"),
                        evidence: vec![evidence("evidence.stonebend.second")],
                        began_at: 21,
                    },
                    None,
                    &BTreeSet::new(),
                )
                .is_err()
        );
    }
}

#[test]
fn hypergiant_and_witness_cannot_assume_forge_by_other_authority() {
    let mut state = replacement_process(false);
    let candidate = id("being.stonebend.candidate-qualified");
    let investiture = FreemasonInvestiture {
        identity: id("tenure.stonebend.prohibited-freemason"),
        claim: id("claim.stonebend.freemason-replacement.being.stonebend.candidate-qualified"),
        candidate: candidate.clone(),
        independent_review: id("review.stonebend.forge.being.stonebend.candidate-qualified"),
        proliteriate_yield_hearing: evidence("evidence.stonebend.yield"),
        diamond_boundary_recognition: Some(evidence("evidence.stonebend.boundary")),
        active_diamond_bearer: Some(candidate.clone()),
        seal: seal("seal.stonebend.prohibited"),
        evidence: vec![evidence("evidence.stonebend.prohibited")],
        began_at: 20,
    };
    assert!(
        state
            .invest_replacement(investiture, Some(&candidate), &BTreeSet::new())
            .is_err()
    );

    let mut state = replacement_process(false);
    let witness = BTreeSet::from([candidate.clone()]);
    let investiture = FreemasonInvestiture {
        identity: id("tenure.stonebend.prohibited-witness"),
        claim: id("claim.stonebend.freemason-replacement.being.stonebend.candidate-qualified"),
        candidate,
        independent_review: id("review.stonebend.forge.being.stonebend.candidate-qualified"),
        proliteriate_yield_hearing: evidence("evidence.stonebend.yield"),
        diamond_boundary_recognition: None,
        active_diamond_bearer: None,
        seal: seal("seal.stonebend.prohibited-witness"),
        evidence: vec![evidence("evidence.stonebend.prohibited-witness")],
        began_at: 20,
    };
    assert!(
        state
            .invest_replacement(investiture, None, &witness)
            .is_err()
    );
}

#[test]
fn proliteriate_has_no_total_vacancy_threshold_or_permanent_speaker() {
    let policy = ProliteriateContinuityPolicy::default();
    let network = populated_network();
    assert!(policy.validate_network(&network).is_ok());
    assert_eq!(network.identity, proliteriate_id());
    assert!(policy.permanent_selection_threshold.is_none());
    assert!(policy.permanent_speaker.is_none());
}

#[test]
fn dissolved_node_preserves_member_identity_and_history() {
    let mut network = populated_network();
    let person = id("being.stonebend.multi-node-member");
    for node in [
        id("node.stonebend.third-pass.district"),
        id("node.stonebend.third-pass.workshop"),
    ] {
        network
            .add_membership(NetworkMembership {
                person: person.clone(),
                node,
            })
            .unwrap();
    }
    network
        .dissolve_node(&id("node.stonebend.third-pass.workshop"))
        .unwrap();
    assert_eq!(
        network
            .memberships()
            .iter()
            .filter(|membership| membership.person == person)
            .count(),
        2
    );
    assert!(
        network
            .nodes()
            .contains_key(&id("node.stonebend.third-pass.workshop"))
    );
    assert!(!network.node_is_active(&id("node.stonebend.third-pass.workshop")));
}

#[test]
fn witness_completion_and_recall_preserve_network_and_replacement_path() {
    let mut network = populated_network();
    network
        .add_mandate(network_mandate("mandate.stonebend.first"))
        .unwrap();
    network
        .raise_witness(RaisedWitness {
            identity: id("witness.stonebend.first"),
            person: id("being.stonebend.first-witness"),
            mandate: id("mandate.stonebend.first"),
            active: true,
        })
        .unwrap();
    network
        .recall_witness(
            &id("witness.stonebend.first"),
            evidence("evidence.stonebend.witness-recall"),
        )
        .unwrap();
    network
        .add_mandate(network_mandate("mandate.stonebend.replacement"))
        .unwrap();
    network
        .raise_witness(RaisedWitness {
            identity: id("witness.stonebend.replacement"),
            person: id("being.stonebend.replacement-witness"),
            mandate: id("mandate.stonebend.replacement"),
            active: true,
        })
        .unwrap();
    network
        .complete_mandate(&id("mandate.stonebend.replacement"))
        .unwrap();
    assert_eq!(network.identity, proliteriate_id());
    assert_eq!(network.nodes().len(), 2);
    assert!(network.witnesses().values().all(|witness| !witness.active));
}

fn temporal_traces() -> BTreeMap<
    hollow_grove::world::stonebend::second_pass::ConstitutionalDimension,
    TemporalDimensionTrace,
> {
    use hollow_grove::world::stonebend::second_pass::StonebendConstitutionalPower;
    [
        StonebendConstitutionalPower::Freemason,
        StonebendConstitutionalPower::Hypergiant,
        StonebendConstitutionalPower::Proliteriate,
    ]
    .into_iter()
    .map(|authority| {
        let dimension = authority.domain();
        let label = match dimension {
            hollow_grove::world::stonebend::second_pass::ConstitutionalDimension::Claim => "claim",
            hollow_grove::world::stonebend::second_pass::ConstitutionalDimension::Title => "title",
            hollow_grove::world::stonebend::second_pass::ConstitutionalDimension::Yield => "yield",
        };
        (
            dimension,
            TemporalDimensionTrace {
                dimension,
                authority,
                delegated_actor: id(&format!("reviewer.stonebend.{label}")),
                evidence: evidence(&format!("evidence.stonebend.{label}")),
            },
        )
    })
    .collect()
}

fn temporal_review(
    disposition: TemporalReviewDisposition,
    hollow: bool,
    exceeded_scope: Option<GateScope>,
    yield_justifiable: bool,
) -> ClaimTitleYieldReview {
    ClaimTitleYieldReview {
        identity: id("review.stonebend.temporal"),
        title: title("title.stonebend.third-pass.fixture"),
        claim: ClaimTemporalRecord {
            supporting_claim: id("claim.stonebend.third-pass.fixture"),
            evidence: vec![evidence("evidence.stonebend.temporal-claim")],
            provenance_changed: false,
            form_materially_changed: false,
            structurally_hollow: hollow,
        },
        title_state: TitleTemporalRecord {
            title: title("title.stonebend.third-pass.fixture"),
            active: true,
            lawful_bearer: true,
            boundaries_clear: true,
            valid_scopes: GateScope::ALL.into_iter().collect(),
            renewal_required: false,
            exceeded_scope,
        },
        yield_state: YieldTemporalRecord {
            yield_record: YieldRecord {
                identity: id("yield.stonebend.temporal"),
                issue: id("issue.stonebend.temporal"),
                impacts: Vec::new(),
                evidence: vec![evidence("evidence.stonebend.temporal-yield")],
            },
            accumulated_risk: "recorded risk".into(),
            inherited_consequence: "recorded consequence".into(),
            purpose_diverged: !yield_justifiable,
            continuation_justifiable: yield_justifiable,
        },
        dimension_traces: temporal_traces(),
        disposition,
        evidence: vec![evidence("evidence.stonebend.temporal-review")],
        sequence: 100,
    }
}

#[test]
fn claim_title_and_yield_are_all_traceable_through_time() {
    assert!(
        temporal_review(TemporalReviewDisposition::NoAction, false, None, true)
            .validate()
            .is_ok()
    );
}

#[test]
fn deterioration_targets_the_affected_dimension() {
    assert!(
        temporal_review(
            TemporalReviewDisposition::GateScopeReview(GateScope::OperationalDeployment),
            false,
            Some(GateScope::OperationalDeployment),
            true,
        )
        .validate()
        .is_ok()
    );
    assert!(
        temporal_review(TemporalReviewDisposition::CoreClaimReview, true, None, true,)
            .validate()
            .is_ok()
    );
    assert!(
        temporal_review(TemporalReviewDisposition::YieldHearing, false, None, false,)
            .validate()
            .is_ok()
    );
    assert!(
        temporal_review(TemporalReviewDisposition::NoAction, true, None, true)
            .validate()
            .is_err()
    );
}

#[test]
fn no_duplicate_engine_or_unstable_identity_is_required() {
    let core = title_core();
    assert_eq!(core.title.as_str(), "title.stonebend.third-pass.fixture");
    assert!(EvidenceRef::new("stonebend-third-pass", core.title.as_str()).is_ok());
}

#[test]
fn central_junction_remains_a_non_house_gate_endpoint() {
    assert!(
        StonebendGateFacing::CentralJunction
            .house_endpoint()
            .is_none()
    );
}

#[test]
fn first_and_second_pass_types_remain_authoritative() {
    use hollow_grove::world::stonebend::foundation::{BurdenState, MediumState};
    assert!(MediumState::Current.accepts_burden(BurdenState::Heavy));
    assert_eq!(GateScope::ALL.len(), 3);
    assert_eq!(StonebendGateFacing::ALL.len(), 3);
    assert_eq!(
        freemason_institution_id().as_str(),
        "institution.stonebend.freemason"
    );
}
