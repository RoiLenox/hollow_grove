use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::constitutional::ChallengeId;
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::IdentityId;
use hollow_grove::world::central_junction::JunctionApproach;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::stonebend::second_pass::{
    ChallengeGround, ClaimAccountability, ClaimRecord, ConstitutionalChallenge,
    ConstitutionalConcurrence, ConstitutionalDimension, ConstitutionalTarget, DelegatedPowerTrace,
    DiamondState, DiamondTenure, DiamondTenureStatus, DomainEvidenceAuthority,
    GateCrossingDirection, GateCrossingRecord, GateEvidenceTransfer, GateFailureKind, GateScope,
    GateScopeRecognition, HypergiantSuccession, HypergiantSuccessionStage, MandateAuthority,
    NetworkMandate, NetworkMembership, OfficeEnding, ProliteriateNetwork, ProliteriateNode,
    ProliteriateNodeKind, RaisedWitness, ReturnedEvidenceDisposition, StonebendConstitutionalPower,
    StonebendGateFacing, StonebendTitleCore, SuccessionStageEvidence, TitleBoundaryAccountability,
    TitleScopeDisposition, YieldAccountability, YieldImpact, YieldRecord,
    canonical_stonebend_gates, diamond_title_id, remove_active_freemason,
    validate_three_gate_topology,
};
use hollow_grove::world::stonebend::{
    DecisionRecordId, EvidenceRecordId, NameRecordId, SealRecordId, SuccessionRecordId,
    TitleRecordId, TombstoneRecordId, freemason_institution_id, high_freemason_office_id,
    hypergiant_office_id, proliteriate_id, stonebend_constitution_id,
};

fn identity(value: &str) -> IdentityId {
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

fn decision(value: &str) -> DecisionRecordId {
    DecisionRecordId::new(value).expect("stable fixture decision")
}

fn seal(value: &str) -> SealRecordId {
    SealRecordId::new(value).expect("stable fixture Seal")
}

fn tombstone(value: &str) -> TombstoneRecordId {
    TombstoneRecordId::new(value).expect("stable fixture Tombstone")
}

fn succession_id(value: &str) -> SuccessionRecordId {
    SuccessionRecordId::new(value).expect("stable fixture succession")
}

fn challenge_id(value: &str) -> ChallengeId {
    ChallengeId::new(value).expect("stable fixture challenge")
}

fn scope_recognition(
    core: &TitleRecordId,
    facing: StonebendGateFacing,
    disposition: TitleScopeDisposition,
) -> GateScopeRecognition {
    let authority = match facing {
        StonebendGateFacing::Flynt => DomainEvidenceAuthority::FlyntProofOfPersistence,
        StonebendGateFacing::CentralJunction => {
            DomainEvidenceAuthority::CentralJunctionPublicStandard
        }
        StonebendGateFacing::Sandmanor => DomainEvidenceAuthority::SandmanorDesignAndFormation,
    };
    GateScopeRecognition {
        identity: identity(&format!(
            "recognition.stonebend.fixture.{}",
            facing.stable_id().replace("gate.stonebend.", "")
        )),
        title: core.clone(),
        facing,
        scope: facing.scope(),
        disposition,
        domain_authority: authority,
        evidence: vec![evidence(&format!(
            "evidence.stonebend.fixture.{}",
            facing.stable_id().replace("gate.stonebend.", "")
        ))],
        boundary: format!("bounded {} recognition", facing.stable_id()),
        returned_evidence: Vec::new(),
    }
}

fn shared_title(reverse: bool) -> StonebendTitleCore {
    let core_id = title("title.stonebend.fixture.shared");
    let mut core = StonebendTitleCore::new(
        core_id.clone(),
        identity("subject.stonebend.fixture.shared"),
        name("name.stonebend.fixture.shared"),
        identity("claim.stonebend.fixture.shared"),
        vec![evidence("evidence.stonebend.fixture.core")],
        "bounded public fixture identity",
    );
    let mut scopes = vec![
        scope_recognition(
            &core_id,
            StonebendGateFacing::Sandmanor,
            TitleScopeDisposition::Recognized,
        ),
        scope_recognition(
            &core_id,
            StonebendGateFacing::CentralJunction,
            TitleScopeDisposition::Recognized,
        ),
        scope_recognition(
            &core_id,
            StonebendGateFacing::Flynt,
            TitleScopeDisposition::Rejected,
        ),
    ];
    if reverse {
        scopes.reverse();
    }
    for scope in scopes {
        core.record_scope(scope).expect("valid independent scope");
    }
    core
}

fn yield_record(issue: &str) -> YieldRecord {
    YieldRecord {
        identity: identity(&format!("yield.{issue}")),
        issue: identity(issue),
        impacts: vec![YieldImpact {
            description: "public benefit and burden".into(),
            beneficiaries: BTreeSet::from([identity("community.stonebend.fixture-beneficiary")]),
            burden_bearers: BTreeSet::from([identity("community.stonebend.fixture-burden")]),
        }],
        evidence: vec![evidence("evidence.stonebend.fixture-yield")],
    }
}

fn populated_network() -> ProliteriateNetwork {
    let mut network = ProliteriateNetwork::default();
    for (id, kind) in [
        (
            "node.stonebend.fixture-district",
            ProliteriateNodeKind::District,
        ),
        (
            "node.stonebend.fixture-workshop",
            ProliteriateNodeKind::GuildOrWorkshop,
        ),
        (
            "node.stonebend.fixture-worksite",
            ProliteriateNodeKind::LaborCrewOrWorksite,
        ),
        (
            "node.stonebend.fixture-commonwealth",
            ProliteriateNodeKind::InheritedOrCommonwealthCommunity,
        ),
    ] {
        network
            .add_node(ProliteriateNode {
                identity: identity(id),
                kind,
                name: id.replace('.', " "),
            })
            .expect("valid stable node");
    }
    network
}

fn mandate(id: &str) -> NetworkMandate {
    NetworkMandate {
        identity: identity(id),
        participating_nodes: BTreeSet::from([
            identity("node.stonebend.fixture-district"),
            identity("node.stonebend.fixture-workshop"),
            identity("node.stonebend.fixture-worksite"),
            identity("node.stonebend.fixture-commonwealth"),
        ]),
        issue_claim: identity("claim.stonebend.fixture-yield"),
        affected_yield: yield_record("issue.stonebend.fixture-yield"),
        testimony: vec![evidence("evidence.stonebend.fixture-testimony")],
        authorities: BTreeSet::from([
            MandateAuthority::PresentYield,
            MandateAuthority::OpenChallenge,
        ]),
        boundary: "present this Yield and open one bounded challenge".into(),
        active_witness: None,
        recalls: Vec::new(),
        completed: false,
        invalidated: false,
    }
}

fn reviewed_challenge(
    id: &str,
    challenger: StonebendConstitutionalPower,
    target: ConstitutionalTarget,
) -> ConstitutionalChallenge {
    let mut challenge = ConstitutionalChallenge::open(
        challenge_id(id),
        challenger,
        target,
        ChallengeGround::ConstitutionalViolation,
        vec![evidence("evidence.stonebend.fixture-challenge")],
    )
    .expect("valid challenge");
    challenge
        .answer(vec![evidence("evidence.stonebend.fixture-answer")])
        .expect("lawful answer");
    challenge.open_review().expect("review opens");
    challenge
}

fn stage_evidence(
    stage: HypergiantSuccessionStage,
    candidate: &IdentityId,
) -> SuccessionStageEvidence {
    let authority = if stage == HypergiantSuccessionStage::FreemasonExamination {
        identity("being.stonebend.fixture-freemason")
    } else {
        identity("authority.stonebend.fixture-succession")
    };
    assert_ne!(
        &authority, candidate,
        "fixture may not self-certify the sovereign Claim"
    );
    SuccessionStageEvidence {
        stage,
        evidence: evidence(&format!(
            "evidence.stonebend.fixture-succession.{}",
            stage.semantic_order()
        )),
        responsible_authority: authority,
    }
}

fn complete_succession(reverse: bool) -> HypergiantSuccession {
    let candidate = identity("being.stonebend.fixture-successor");
    let mut process = HypergiantSuccession::new(
        succession_id("succession.stonebend.fixture-diamond"),
        candidate.clone(),
        identity("claim.stonebend.fixture-diamond"),
    );
    let mut stages = HypergiantSuccessionStage::ALL
        .into_iter()
        .filter(|stage| *stage != HypergiantSuccessionStage::DiamondInvested)
        .collect::<Vec<_>>();
    if reverse {
        stages.reverse();
    }
    for stage in stages {
        process
            .record_stage(stage_evidence(stage, &candidate))
            .expect("valid stable succession stage");
    }
    process
}

fn delegated_trace(
    power: StonebendConstitutionalPower,
    gate: StonebendGateFacing,
) -> DelegatedPowerTrace {
    DelegatedPowerTrace {
        power,
        constitutional_source: power.institution(),
        delegated_actor: identity(&format!(
            "delegate.stonebend.fixture.{}.{}",
            gate.stable_id().replace("gate.stonebend.", ""),
            match power {
                StonebendConstitutionalPower::Hypergiant => "title",
                StonebendConstitutionalPower::Freemason => "claim",
                StonebendConstitutionalPower::Proliteriate => "yield",
            }
        )),
        evidence: evidence("evidence.stonebend.fixture-delegation"),
    }
}

#[test]
fn exactly_three_stable_principal_gates_are_bidirectional() {
    let gates = canonical_stonebend_gates();
    validate_three_gate_topology(&gates).expect("canonical three-gate topology");
    assert_eq!(gates.len(), 3);
    assert_eq!(
        gates
            .iter()
            .map(|gate| gate.facing)
            .collect::<BTreeSet<_>>(),
        StonebendGateFacing::ALL.into_iter().collect()
    );
    assert!(gates.iter().all(|gate| {
        gate.supported_directions
            == GateCrossingDirection::ALL
                .into_iter()
                .collect::<BTreeSet<_>>()
    }));
}

#[test]
fn central_junction_is_a_district_endpoint_and_never_a_house() {
    assert_eq!(StonebendGateFacing::CentralJunction.house_endpoint(), None);
    assert_eq!(
        StonebendGateFacing::CentralJunction.junction_approach(),
        Some(JunctionApproach::CraftCorridor)
    );
    assert_eq!(
        StonebendGateFacing::Flynt.house_endpoint(),
        Some(House::Flynt)
    );
    assert_eq!(
        StonebendGateFacing::Sandmanor.house_endpoint(),
        Some(House::Sandmanor)
    );
}

#[test]
fn gate_routes_preserve_existing_flynt_and_sandmanor_geography() {
    assert_eq!(
        StonebendGateFacing::Flynt.routes(),
        &[
            ConstitutionalRouteId::StairwayToHeaven,
            ConstitutionalRouteId::BasinMotorspeedway,
        ]
    );
    assert_eq!(
        StonebendGateFacing::Sandmanor.routes(),
        &[
            ConstitutionalRouteId::AuraWay,
            ConstitutionalRouteId::MntAura,
        ]
    );
    assert!(StonebendGateFacing::CentralJunction.routes().is_empty());
    assert!(
        !StonebendGateFacing::Sandmanor
            .routes()
            .contains(&ConstitutionalRouteId::Riptide)
    );
}

#[test]
fn mt_aura_is_a_route_landmark_not_a_fourth_gate() {
    assert_eq!(StonebendGateFacing::ALL.len(), 3);
    assert!(
        !StonebendGateFacing::ALL
            .iter()
            .any(|facing| facing.stable_id().contains("mnt-aura"))
    );
}

#[test]
fn gate_identity_and_domain_are_insertion_order_independent() {
    let mut gates = canonical_stonebend_gates().to_vec();
    gates.reverse();
    validate_three_gate_topology(&gates).expect("reversed records preserve semantic topology");
    let domains = gates
        .into_iter()
        .map(|gate| (gate.facing, gate.domain))
        .collect::<BTreeMap<_, _>>();
    for facing in StonebendGateFacing::ALL {
        assert_eq!(domains[&facing], facing.domain());
    }
}

#[test]
fn one_core_title_holds_independent_gate_scopes() {
    let core = shared_title(false);
    assert_eq!(core.scopes().len(), 3);
    assert!(core.authorizes(GateScope::FormationRecognition));
    assert!(core.authorizes(GateScope::PublicCirculation));
    assert!(!core.authorizes(GateScope::OperationalDeployment));
    assert!(
        core.scopes()
            .values()
            .all(|scope| scope.title == core.title)
    );
}

#[test]
fn one_gate_recognition_never_implies_another() {
    let core_id = title("title.stonebend.fixture.formation-only");
    let mut core = StonebendTitleCore::new(
        core_id.clone(),
        identity("subject.stonebend.fixture.formation-only"),
        name("name.stonebend.fixture.formation-only"),
        identity("claim.stonebend.fixture.formation-only"),
        vec![evidence("evidence.stonebend.fixture.formation-only")],
        "formation only",
    );
    core.record_scope(scope_recognition(
        &core_id,
        StonebendGateFacing::Sandmanor,
        TitleScopeDisposition::Recognized,
    ))
    .expect("formation recognition");
    assert!(core.authorizes(GateScope::FormationRecognition));
    assert!(!core.authorizes(GateScope::PublicCirculation));
    assert!(!core.authorizes(GateScope::OperationalDeployment));
}

#[test]
fn flynt_rejection_does_not_erase_formation_or_circulation() {
    let core = shared_title(false);
    assert_eq!(
        core.scope(GateScope::OperationalDeployment)
            .expect("Flynt record")
            .disposition,
        TitleScopeDisposition::Rejected
    );
    assert!(core.authorizes(GateScope::FormationRecognition));
    assert!(core.authorizes(GateScope::PublicCirculation));
}

#[test]
fn title_scope_semantics_ignore_record_insertion_order() {
    assert_eq!(shared_title(false), shared_title(true));
}

#[test]
fn a_scope_cannot_create_a_duplicate_core_title() {
    let core_id = title("title.stonebend.fixture.one");
    let mut core = StonebendTitleCore::new(
        core_id,
        identity("subject.stonebend.fixture.one"),
        name("name.stonebend.fixture.one"),
        identity("claim.stonebend.fixture.one"),
        vec![evidence("evidence.stonebend.fixture.one")],
        "one identity",
    );
    let error = core
        .record_scope(scope_recognition(
            &title("title.stonebend.fixture.two"),
            StonebendGateFacing::Sandmanor,
            TitleScopeDisposition::Recognized,
        ))
        .expect_err("scope cannot swap Title identity");
    assert!(format!("{error}").contains("ScopeCreatesDuplicateTitle"));
}

#[test]
fn core_identity_challenge_is_explicit_and_separate_from_scope_failure() {
    let mut core = shared_title(false);
    let challenge = challenge_id("challenge.stonebend.fixture-core");
    assert!(!core.has_explicit_core_challenge(&challenge));
    core.open_core_challenge(challenge.clone());
    assert!(core.has_explicit_core_challenge(&challenge));
    assert!(core.authorizes(GateScope::FormationRecognition));
}

#[test]
fn gate_failure_returns_typed_evidence_without_flattening_failure_into_fraud() {
    let core_id = title("title.stonebend.fixture-returned-evidence");
    let honest = GateEvidenceTransfer {
        identity: identity("transfer.stonebend.fixture-honest-failure"),
        source_gate: StonebendGateFacing::Flynt,
        title: core_id.clone(),
        evidence: vec![evidence("evidence.stonebend.fixture-honest-failure")],
        failure_kind: GateFailureKind::HonestFailure,
        recommended_disposition: ReturnedEvidenceDisposition::Remediation,
    };
    let fraud = GateEvidenceTransfer {
        identity: identity("transfer.stonebend.fixture-fraud"),
        source_gate: StonebendGateFacing::Flynt,
        title: core_id,
        evidence: vec![evidence("evidence.stonebend.fixture-fraud")],
        failure_kind: GateFailureKind::Fraud,
        recommended_disposition: ReturnedEvidenceDisposition::CoreTitleChallenge,
    };
    assert_ne!(honest.failure_kind, fraud.failure_kind);
    assert_ne!(
        honest.recommended_disposition,
        fraud.recommended_disposition
    );
}

#[test]
fn each_gate_preserves_the_other_domains_evidence_authority() {
    assert_eq!(
        scope_recognition(
            &title("title.stonebend.fixture-authority"),
            StonebendGateFacing::Sandmanor,
            TitleScopeDisposition::Recognized,
        )
        .domain_authority,
        DomainEvidenceAuthority::SandmanorDesignAndFormation
    );
    assert_eq!(
        scope_recognition(
            &title("title.stonebend.fixture-authority"),
            StonebendGateFacing::Flynt,
            TitleScopeDisposition::Recognized,
        )
        .domain_authority,
        DomainEvidenceAuthority::FlyntProofOfPersistence
    );
    assert_eq!(
        scope_recognition(
            &title("title.stonebend.fixture-authority"),
            StonebendGateFacing::CentralJunction,
            TitleScopeDisposition::Recognized,
        )
        .domain_authority,
        DomainEvidenceAuthority::CentralJunctionPublicStandard
    );
}

#[test]
fn diamond_is_stable_and_hypergiant_is_only_its_active_bearer() {
    let mut diamond = DiamondState::default();
    assert_eq!(diamond.title, diamond_title_id());
    assert!(diamond.is_vacant());
    diamond
        .invest(DiamondTenure {
            identity: identity("tenure.stonebend.fixture-first"),
            diamond: diamond_title_id(),
            bearer: identity("being.stonebend.fixture-first-hypergiant"),
            supporting_claim: identity("claim.stonebend.fixture-first-hypergiant"),
            succession: succession_id("succession.stonebend.fixture-first"),
            began_at: 10,
            status: DiamondTenureStatus::Active,
        })
        .expect("lawful tenure");
    assert_eq!(diamond.title, diamond_title_id());
    assert!(!diamond.is_vacant());
    assert_ne!(
        diamond
            .active_tenure
            .as_ref()
            .expect("bearer")
            .bearer
            .as_str(),
        diamond.title.as_str()
    );
}

#[test]
fn ended_hypergiant_tenure_leaves_diamond_vacant_and_tombstoned() {
    let mut diamond = DiamondState::default();
    let first = identity("being.stonebend.fixture-ended-hypergiant");
    diamond
        .invest(DiamondTenure {
            identity: identity("tenure.stonebend.fixture-ended"),
            diamond: diamond.title.clone(),
            bearer: first.clone(),
            supporting_claim: identity("claim.stonebend.fixture-ended"),
            succession: succession_id("succession.stonebend.fixture-ended"),
            began_at: 10,
            status: DiamondTenureStatus::Active,
        })
        .expect("lawful tenure");
    let record = diamond
        .end_active_tenure(
            tombstone("tombstone.stonebend.fixture-ended"),
            OfficeEnding::HonorableCompletion,
            20,
            None,
            Some(identity("yield.stonebend.fixture-ended")),
            None,
        )
        .expect("lawful completion");
    assert!(diamond.is_vacant());
    assert_eq!(diamond.title, diamond_title_id());
    assert_eq!(record.bearer_or_representation, first);
    assert_eq!(record.ending, OfficeEnding::HonorableCompletion);
    assert_eq!(diamond.ended_tenures.len(), 1);
}

#[test]
fn removal_for_failure_cannot_bypass_two_power_concurrence() {
    let mut diamond = DiamondState::default();
    diamond
        .invest(DiamondTenure {
            identity: identity("tenure.stonebend.fixture-no-removal-bypass"),
            diamond: diamond.title.clone(),
            bearer: identity("being.stonebend.fixture-no-removal-bypass"),
            supporting_claim: identity("claim.stonebend.fixture-no-removal-bypass"),
            succession: succession_id("succession.stonebend.fixture-no-removal-bypass"),
            began_at: 1,
            status: DiamondTenureStatus::Active,
        })
        .expect("active tenure");
    assert!(
        diamond
            .end_active_tenure(
                tombstone("tombstone.stonebend.fixture-no-removal-bypass"),
                OfficeEnding::RemovedForFailure,
                2,
                None,
                None,
                None,
            )
            .is_err()
    );
    assert!(!diamond.is_vacant());
}

#[test]
fn later_hypergiant_does_not_inherit_predecessor_person_identity() {
    let mut diamond = DiamondState::default();
    let first = identity("being.stonebend.fixture-predecessor");
    diamond
        .invest(DiamondTenure {
            identity: identity("tenure.stonebend.fixture-predecessor"),
            diamond: diamond.title.clone(),
            bearer: first.clone(),
            supporting_claim: identity("claim.stonebend.fixture-predecessor"),
            succession: succession_id("succession.stonebend.fixture-predecessor"),
            began_at: 1,
            status: DiamondTenureStatus::Active,
        })
        .expect("first tenure");
    diamond
        .end_active_tenure(
            tombstone("tombstone.stonebend.fixture-predecessor"),
            OfficeEnding::Succession,
            2,
            None,
            None,
            Some(identity("being.stonebend.fixture-successor-two")),
        )
        .expect("first tenure ends");
    let second = identity("being.stonebend.fixture-successor-two");
    diamond
        .invest(DiamondTenure {
            identity: identity("tenure.stonebend.fixture-successor-two"),
            diamond: diamond.title.clone(),
            bearer: second.clone(),
            supporting_claim: identity("claim.stonebend.fixture-successor-two"),
            succession: succession_id("succession.stonebend.fixture-successor-two"),
            began_at: 3,
            status: DiamondTenureStatus::Active,
        })
        .expect("second tenure");
    assert_ne!(first, second);
    assert_eq!(diamond.title, diamond_title_id());
}

#[test]
fn hypergiant_and_freemason_are_distinct_active_offices() {
    assert_ne!(hypergiant_office_id(), high_freemason_office_id());
    assert_eq!(
        StonebendConstitutionalPower::Hypergiant.domain(),
        ConstitutionalDimension::Title
    );
    assert_eq!(
        StonebendConstitutionalPower::Freemason.domain(),
        ConstitutionalDimension::Claim
    );
}

#[test]
fn freemason_can_forge_a_claim_but_cannot_self_certify_diamond() {
    let freemason = identity("being.stonebend.fixture-freemason-candidate");
    let claim = ClaimRecord {
        identity: identity("claim.stonebend.fixture-self-certified"),
        subject: freemason.clone(),
        proposed_title: diamond_title_id(),
        evidence: vec![evidence("evidence.stonebend.fixture-self-certified")],
        examiner: freemason.clone(),
        examination: decision("decision.stonebend.fixture-self-certified"),
        seal: Some(seal("seal.stonebend.fixture-self-certified")),
        sovereign_claim: true,
    };
    assert!(claim.validate_freemason_examination(&freemason).is_err());

    let independent = identity("being.stonebend.fixture-independent-freemason");
    let valid = ClaimRecord {
        examiner: independent.clone(),
        ..claim
    };
    valid
        .validate_freemason_examination(&independent)
        .expect("independent constitutional examination");
}

#[test]
fn proliteriate_is_a_distributed_network_with_four_extensible_node_kinds() {
    let network = populated_network();
    assert_eq!(network.identity, proliteriate_id());
    assert_eq!(network.nodes().len(), 4);
    assert_eq!(
        network
            .nodes()
            .values()
            .map(|node| node.kind)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            ProliteriateNodeKind::District,
            ProliteriateNodeKind::GuildOrWorkshop,
            ProliteriateNodeKind::LaborCrewOrWorksite,
            ProliteriateNodeKind::InheritedOrCommonwealthCommunity,
        ])
    );
    assert_eq!(StonebendConstitutionalPower::Proliteriate.office(), None);
}

#[test]
fn one_person_may_join_multiple_nodes_without_duplicate_identity() {
    let mut network = populated_network();
    let person = identity("being.stonebend.fixture-worker");
    for node in [
        "node.stonebend.fixture-district",
        "node.stonebend.fixture-workshop",
        "node.stonebend.fixture-worksite",
    ] {
        network
            .add_membership(NetworkMembership {
                person: person.clone(),
                node: identity(node),
            })
            .expect("overlapping network membership");
    }
    assert_eq!(network.memberships().len(), 3);
    assert_eq!(
        network
            .memberships()
            .iter()
            .map(|membership| &membership.person)
            .collect::<BTreeSet<_>>()
            .len(),
        1
    );
}

#[test]
fn bounded_mandate_raises_recallable_temporary_witness() {
    let mut network = populated_network();
    let mandate_id = identity("mandate.stonebend.fixture-temporary");
    network
        .add_mandate(mandate(mandate_id.as_str()))
        .expect("lawful bounded mandate");
    let first = identity("witness.stonebend.fixture-first");
    network
        .raise_witness(RaisedWitness {
            identity: first.clone(),
            person: identity("being.stonebend.fixture-witness-one"),
            mandate: mandate_id.clone(),
            active: true,
        })
        .expect("temporary witness");
    assert!(network.witness_has_authority(&first, MandateAuthority::PresentYield));
    assert!(!network.witness_has_authority(&first, MandateAuthority::GiveGateTestimony));
    network
        .recall_witness(
            &first,
            evidence("evidence.stonebend.fixture-witness-recall"),
        )
        .expect("network recalls its voice");
    assert!(!network.witness_has_authority(&first, MandateAuthority::PresentYield));

    let replacement = identity("witness.stonebend.fixture-replacement");
    network
        .raise_witness(RaisedWitness {
            identity: replacement.clone(),
            person: identity("being.stonebend.fixture-witness-two"),
            mandate: mandate_id.clone(),
            active: true,
        })
        .expect("replacement voice");
    network
        .complete_mandate(&mandate_id)
        .expect("completed matter returns authority");
    assert!(!network.witness_has_authority(&replacement, MandateAuthority::PresentYield));
    assert!(network.nodes().len() == 4);
}

#[test]
fn no_permanent_spartacus_office_is_created() {
    assert_eq!(StonebendConstitutionalPower::ALL.len(), 3);
    assert_eq!(StonebendConstitutionalPower::Proliteriate.office(), None);
    for power in StonebendConstitutionalPower::ALL {
        assert!(!power.institution().as_str().contains("spartacus"));
    }
}

#[test]
fn one_power_opens_a_challenge_but_cannot_remove_alone() {
    let challenge = reviewed_challenge(
        "challenge.stonebend.fixture-one-power",
        StonebendConstitutionalPower::Freemason,
        ConstitutionalTarget::Hypergiant(identity("being.stonebend.fixture-hypergiant")),
    );
    assert!(!challenge.removes_target());
    assert!(
        ConstitutionalConcurrence::new(&challenge, vec![StonebendConstitutionalPower::Freemason])
            .is_err()
    );
}

#[test]
fn two_distinct_powers_remove_the_third_and_duplicates_are_rejected() {
    let challenge = reviewed_challenge(
        "challenge.stonebend.fixture-two-power",
        StonebendConstitutionalPower::Freemason,
        ConstitutionalTarget::Hypergiant(identity("being.stonebend.fixture-hypergiant")),
    );
    let valid = ConstitutionalConcurrence::new(
        &challenge,
        vec![
            StonebendConstitutionalPower::Freemason,
            StonebendConstitutionalPower::Proliteriate,
        ],
    )
    .expect("two independent powers");
    assert_eq!(valid.endorsements().len(), 2);
    assert!(
        ConstitutionalConcurrence::new(
            &challenge,
            vec![
                StonebendConstitutionalPower::Freemason,
                StonebendConstitutionalPower::Freemason,
            ],
        )
        .is_err()
    );
}

#[test]
fn every_target_has_exactly_two_distinct_removing_powers() {
    let cases = [
        (
            ConstitutionalTarget::Hypergiant(identity("being.stonebend.fixture-hypergiant")),
            StonebendConstitutionalPower::Freemason,
            vec![
                StonebendConstitutionalPower::Freemason,
                StonebendConstitutionalPower::Proliteriate,
            ],
        ),
        (
            ConstitutionalTarget::Freemason(identity("being.stonebend.fixture-freemason")),
            StonebendConstitutionalPower::Hypergiant,
            vec![
                StonebendConstitutionalPower::Hypergiant,
                StonebendConstitutionalPower::Proliteriate,
            ],
        ),
        (
            ConstitutionalTarget::ProliteriateRepresentation(identity(
                "mandate.stonebend.fixture-corrupted",
            )),
            StonebendConstitutionalPower::Hypergiant,
            vec![
                StonebendConstitutionalPower::Hypergiant,
                StonebendConstitutionalPower::Freemason,
            ],
        ),
    ];
    for (index, (target, challenger, powers)) in cases.into_iter().enumerate() {
        let challenge = reviewed_challenge(
            &format!("challenge.stonebend.fixture-target-{index}"),
            challenger,
            target,
        );
        ConstitutionalConcurrence::new(&challenge, powers)
            .expect("target removed only by the other two powers");
    }
}

#[test]
fn hypergiant_removal_vacates_diamond_and_links_a_tombstone() {
    let bearer = identity("being.stonebend.fixture-removed-hypergiant");
    let mut diamond = DiamondState::default();
    diamond
        .invest(DiamondTenure {
            identity: identity("tenure.stonebend.fixture-removed"),
            diamond: diamond.title.clone(),
            bearer: bearer.clone(),
            supporting_claim: identity("claim.stonebend.fixture-removed"),
            succession: succession_id("succession.stonebend.fixture-removed"),
            began_at: 1,
            status: DiamondTenureStatus::Active,
        })
        .expect("active Hypergiant");
    let mut challenge = reviewed_challenge(
        "challenge.stonebend.fixture-remove-hypergiant",
        StonebendConstitutionalPower::Freemason,
        ConstitutionalTarget::Hypergiant(bearer),
    );
    let authorization = ConstitutionalConcurrence::new(
        &challenge,
        vec![
            StonebendConstitutionalPower::Freemason,
            StonebendConstitutionalPower::Proliteriate,
        ],
    )
    .expect("lawful concurrence")
    .authorize();
    challenge
        .record_removal_authorized(&authorization)
        .expect("challenge records two-power authorization");
    let record = diamond
        .remove_active_hypergiant(
            &authorization,
            tombstone("tombstone.stonebend.fixture-removed"),
            2,
            Some(identity("yield.stonebend.fixture-removed")),
        )
        .expect("lawful removal");
    challenge
        .record_removed(&record)
        .expect("challenge closes with Tombstone-linked removal");
    assert!(diamond.is_vacant());
    assert!(record.diamond_vacant);
    assert_eq!(
        record.tombstone,
        tombstone("tombstone.stonebend.fixture-removed")
    );
    assert_eq!(
        diamond.ended_tenures[0].ending,
        OfficeEnding::RemovedForFailure
    );
}

#[test]
fn freemason_removal_ends_bearer_authority_without_erasing_records() {
    let bearer = identity("being.stonebend.fixture-removed-freemason");
    let challenge = reviewed_challenge(
        "challenge.stonebend.fixture-remove-freemason",
        StonebendConstitutionalPower::Hypergiant,
        ConstitutionalTarget::Freemason(bearer.clone()),
    );
    let authorization = ConstitutionalConcurrence::new(
        &challenge,
        vec![
            StonebendConstitutionalPower::Hypergiant,
            StonebendConstitutionalPower::Proliteriate,
        ],
    )
    .expect("lawful concurrence")
    .authorize();
    let (disposition, office_tombstone) = remove_active_freemason(
        &authorization,
        &bearer,
        tombstone("tombstone.stonebend.fixture-freemason"),
        identity("claim.stonebend.fixture-freemason-tenure"),
        5,
        9,
        Some(identity("yield.stonebend.fixture-freemason-tenure")),
    )
    .expect("Freemason bearer removed");
    assert!(!disposition.diamond_vacant);
    assert!(disposition.network_survives);
    assert_eq!(office_tombstone.office, high_freemason_office_id());
    assert_eq!(office_tombstone.ending, OfficeEnding::RemovedForFailure);
}

#[test]
fn corrupted_representation_can_end_but_the_network_cannot_be_abolished() {
    let mut network = populated_network();
    let mandate_id = identity("mandate.stonebend.fixture-corrupted");
    network
        .add_mandate(mandate(mandate_id.as_str()))
        .expect("initial mandate");
    let witness = identity("witness.stonebend.fixture-corrupted");
    network
        .raise_witness(RaisedWitness {
            identity: witness.clone(),
            person: identity("being.stonebend.fixture-corrupted-witness"),
            mandate: mandate_id.clone(),
            active: true,
        })
        .expect("initial representation");
    let challenge = reviewed_challenge(
        "challenge.stonebend.fixture-corrupted-representation",
        StonebendConstitutionalPower::Hypergiant,
        ConstitutionalTarget::ProliteriateRepresentation(mandate_id.clone()),
    );
    let authorization = ConstitutionalConcurrence::new(
        &challenge,
        vec![
            StonebendConstitutionalPower::Hypergiant,
            StonebendConstitutionalPower::Freemason,
        ],
    )
    .expect("lawful representation review")
    .authorize();
    let disposition = network
        .invalidate_representation(
            &mandate_id,
            &authorization,
            tombstone("tombstone.stonebend.fixture-corrupted-representation"),
        )
        .expect("corrupted voice invalidated");
    assert!(disposition.network_survives);
    assert_eq!(network.identity, proliteriate_id());
    assert_eq!(network.nodes().len(), 4);
    assert!(
        network
            .mandates()
            .get(&mandate_id)
            .expect("historical mandate")
            .invalidated
    );
    assert!(!network.witnesses().get(&witness).expect("witness").active);

    let replacement_mandate = identity("mandate.stonebend.fixture-replacement");
    network
        .add_mandate(mandate(replacement_mandate.as_str()))
        .expect("replacement representation remains possible");
}

#[test]
fn succession_stages_are_semantic_and_ignore_record_insertion_order() {
    let forward = complete_succession(false);
    let reverse = complete_succession(true);
    let forward_stages = forward
        .ordered_stage_evidence()
        .into_iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    let reverse_stages = reverse
        .ordered_stage_evidence()
        .into_iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    assert_eq!(forward_stages, reverse_stages);
    assert!(
        forward_stages
            .windows(2)
            .all(|pair| pair[0].semantic_order() < pair[1].semantic_order())
    );
}

#[test]
fn lazerhorn_is_required_and_recommendation_or_lineage_cannot_bypass_it() {
    let candidate = identity("being.stonebend.fixture-missing-lazerhorn");
    let mut process = HypergiantSuccession::new(
        succession_id("succession.stonebend.fixture-missing-lazerhorn"),
        candidate.clone(),
        identity("claim.stonebend.fixture-missing-lazerhorn"),
    );
    process.outgoing_recommendation = Some(evidence("evidence.stonebend.fixture-recommendation"));
    process.lineage_evidence = Some(evidence("evidence.stonebend.fixture-lineage"));
    for stage in HypergiantSuccessionStage::ALL {
        if matches!(
            stage,
            HypergiantSuccessionStage::LazerhornClimbed
                | HypergiantSuccessionStage::DiamondInvested
        ) {
            continue;
        }
        process
            .record_stage(stage_evidence(stage, &candidate))
            .expect("other succession stage");
    }
    assert!(!process.has_stage(HypergiantSuccessionStage::LazerhornClimbed));
    assert!(process.require_accession_eligibility().is_err());
}

#[test]
fn freemason_candidate_cannot_self_certify_succession() {
    let candidate = identity("being.stonebend.fixture-freemason-candidate-two");
    let mut process = HypergiantSuccession::new(
        succession_id("succession.stonebend.fixture-self-certification"),
        candidate.clone(),
        identity("claim.stonebend.fixture-self-certification"),
    );
    assert!(
        process
            .record_stage(SuccessionStageEvidence {
                stage: HypergiantSuccessionStage::FreemasonExamination,
                evidence: evidence("evidence.stonebend.fixture-self-certification-two"),
                responsible_authority: candidate,
            })
            .is_err()
    );
}

#[test]
fn complete_lazerhorn_path_produces_one_new_diamond_tenure() {
    let mut process = complete_succession(true);
    let mut diamond = DiamondState::default();
    let candidate = process.candidate.clone();
    process
        .invest_diamond(
            &mut diamond,
            identity("tenure.stonebend.fixture-lawful-successor"),
            50,
            stage_evidence(HypergiantSuccessionStage::DiamondInvested, &candidate),
        )
        .expect("full lawful path");
    assert_eq!(
        diamond.active_tenure.as_ref().expect("new tenure").bearer,
        candidate
    );
    assert!(process.has_stage(HypergiantSuccessionStage::LazerhornClimbed));
    assert!(process.has_stage(HypergiantSuccessionStage::DiamondInvested));
}

#[test]
fn former_hypergiant_has_no_shortened_return_path() {
    let mut process = complete_succession(false);
    process.former_bearer_returning = true;
    assert_eq!(
        process.ordered_stage_evidence().len(),
        HypergiantSuccessionStage::ALL.len() - 1
    );
    process
        .require_accession_eligibility()
        .expect("former bearer completed every ordinary pre-investiture stage");
}

#[test]
fn each_gate_crossing_preserves_claim_title_and_yield_accountability() {
    for facing in StonebendGateFacing::ALL {
        let record = GateCrossingRecord {
            identity: identity(&format!(
                "crossing.stonebend.fixture.{}",
                facing.stable_id().replace("gate.stonebend.", "")
            )),
            facing,
            direction: GateCrossingDirection::IntoStonebend,
            subject: identity("subject.stonebend.fixture-crossing"),
            claim: ClaimAccountability {
                claim: identity("claim.stonebend.fixture-crossing"),
                trace: delegated_trace(StonebendConstitutionalPower::Freemason, facing),
            },
            title_boundary: TitleBoundaryAccountability {
                title: title("title.stonebend.fixture-crossing"),
                disposition: TitleScopeDisposition::Recognized,
                trace: delegated_trace(StonebendConstitutionalPower::Hypergiant, facing),
            },
            yield_accountability: YieldAccountability {
                yield_record: identity("yield.stonebend.fixture-crossing"),
                trace: delegated_trace(StonebendConstitutionalPower::Proliteriate, facing),
            },
        };
        record
            .validate()
            .expect("delegated crossing remains constitutionally traceable");
        assert_ne!(
            record.claim.trace.delegated_actor,
            identity("being.stonebend.fixture-freemason")
        );
    }
}

#[test]
fn delegation_sources_remain_the_existing_constitutional_institutions() {
    assert_eq!(
        StonebendConstitutionalPower::Hypergiant.institution(),
        stonebend_constitution_id()
    );
    assert_eq!(
        StonebendConstitutionalPower::Freemason.institution(),
        freemason_institution_id()
    );
    assert_eq!(
        StonebendConstitutionalPower::Proliteriate.institution(),
        proliteriate_id()
    );
}
