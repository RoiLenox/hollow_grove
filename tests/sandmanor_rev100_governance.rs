//! Explicitly simulated evidence and voter rolls, never production defaults.
use hollow_grove::constitutional::InterfaceApplyOutcome;
use hollow_grove::institution::{InstitutionalBeingId, OfficeHolder};
use hollow_grove::world::sandmanor::governance::*;
use hollow_grove::world::sandmanor::milestone::TrialDomain;
use hollow_grove::world::session::WorldSession;
use std::collections::{BTreeMap, BTreeSet};
fn proof(id: &str, signer: &str) -> Attestation {
    Attestation {
        record: id.into(),
        signer: signer.into(),
    }
}
fn policy() -> Policy {
    Policy {
        authorization: proof("authority.policy", "court.authorizer"),
        administrator: "election.admin".into(),
        credential_reviewers: BTreeSet::from(["credential.reviewer".into()]),
        court_reviewers: BTreeSet::from(["court.reviewer".into()]),
        recall: BTreeMap::new(),
        special_elections: BTreeMap::new(),
    }
}
fn source() -> WorldSession {
    let mut state = WorldSession::canonical().institutional().clone();
    state.catalog.office_holders.push(OfficeHolder {
        office: hollow_grove::world::glaushouse::prima_donna_office_id(),
        being: InstitutionalBeingId::new("being.clinician").unwrap(),
        active: true,
    });
    WorldSession::from_institutional_state(state).unwrap()
}
fn command(g: &Governance, year: u64, actor: &str, action: Action) -> Command {
    fn records(v: &serde_json::Value, out: &mut BTreeSet<String>) {
        match v {
            serde_json::Value::Object(m) => {
                if let Some(serde_json::Value::String(s)) = m.get("record") {
                    out.insert(s.clone());
                }
                for v in m.values() {
                    records(v, out);
                }
            }
            serde_json::Value::Array(a) => {
                for v in a {
                    records(v, out);
                }
            }
            _ => {}
        }
    }
    let mut evidence = BTreeSet::from([format!("evidence.{}", g.revision())]);
    records(&serde_json::to_value(&action).unwrap(), &mut evidence);
    Command {
        id: format!("event.{}", g.revision()),
        expected_revision: g.revision(),
        year,
        actor: actor.into(),
        evidence,
        action,
    }
}
fn apply(g: &mut Governance, year: u64, actor: &str, action: Action) {
    let c = command(g, year, actor, action);
    assert_eq!(g.execute(c), Ok(InterfaceApplyOutcome::Applied));
}
fn reject(g: &mut Governance, year: u64, actor: &str, action: Action) -> Error {
    let before = g.clone();
    let c = command(g, year, actor, action);
    let e = g.execute(c).expect_err("must reject");
    assert_eq!(*g, before);
    e
}
fn qualify(g: &mut Governance, person: &str, office: Office) {
    let year = g.year();
    apply(
        g,
        year,
        "credential.reviewer",
        Action::RecognizeMastery {
            person: person.into(),
            form: office.form().into(),
        },
    );
    apply(
        g,
        year,
        "credential.reviewer",
        Action::Qualify {
            person: person.into(),
            office,
        },
    );
}
fn votes(id: &str, choices: &[&str]) -> Vec<Vote> {
    choices
        .iter()
        .enumerate()
        .map(|(i, p)| Vote {
            voter: format!("voter.{i}"),
            candidate: (*p).into(),
            receipt: proof(&format!("{id}.vote.{i}"), &format!("voter.{i}")),
        })
        .collect()
}
fn roll(id: &str, office: Office) -> ElectoralRoll {
    ElectoralRoll {
        office,
        eligible: (0..10).map(|i| format!("voter.{i}")).collect(),
        certification: proof(&format!("{id}.roll"), "election.admin"),
    }
}
fn election(id: &str, office: Office, person: &str) -> Election {
    Election {
        id: id.into(),
        office,
        candidates: BTreeSet::from([person.into()]),
        roll: roll(id, office),
        rounds: vec![votes(id, &[person; 6])],
        winner: person.into(),
        tenure: format!("{id}.tenure"),
        special: false,
        certification: proof(&format!("{id}.certification"), "election.admin"),
    }
}
fn governors(p: Policy) -> Governance {
    let mut g = Governance::new(&source(), p, 0).unwrap();
    for (office, person, id) in [
        (Office::Mino, "being.north", "election.n"),
        (Office::Cent, "being.south", "election.s"),
    ] {
        qualify(&mut g, person, office);
        apply(
            &mut g,
            0,
            "election.admin",
            Action::CertifyElection {
                election: election(id, office, person),
            },
        );
    }
    g
}
fn open(g: &mut Governance) {
    apply(
        g,
        g.year(),
        "court.reviewer",
        Action::OpenContest {
            id: "contest.one".into(),
            court_case: "court.contest-one".into(),
        },
    );
}
fn contest_proof(id: &str, choices: &[&str]) -> ContestProof {
    let performances = [
        (Office::Mino, "being.north", 10),
        (Office::Cent, "being.south", 2),
    ]
    .into_iter()
    .map(|(o, p, delta)| {
        (
            o,
            Performance {
                person: p.into(),
                baseline: TrialDomain::ALL.into_iter().map(|d| (d, 10)).collect(),
                final_competencies: TrialDomain::ALL
                    .into_iter()
                    .map(|d| (d, 10 + delta))
                    .collect(),
                baseline_evidence: proof(&format!("{id}.{p}.baseline"), "credential.reviewer"),
                improvement_evidence: proof(
                    &format!("{id}.{p}.improvement"),
                    "credential.reviewer",
                ),
                integration_demonstrated: true,
                imitation_only: false,
                degraded_other_candidate: false,
            },
        )
    })
    .collect();
    let trials = TrialDomain::ALL
        .into_iter()
        .enumerate()
        .map(|(i, domain)| {
            let (candidate, teacher) = if i < 2 {
                ("being.south", "being.north")
            } else {
                ("being.north", "being.south")
            };
            LearningTrial {
                id: format!("{id}.trial.{i}"),
                domain,
                candidate: candidate.into(),
                teacher: teacher.into(),
                evidence: proof(&format!("{id}.trial.{i}.evidence"), "credential.reviewer"),
                candidate_completed_work: true,
                valid: true,
            }
        })
        .collect();
    ContestProof {
        performances,
        trials,
        teaching: ["being.north", "being.south"]
            .into_iter()
            .map(|p| Teaching {
                teacher: p.into(),
                sincere: true,
                complete_enough_for_safety: true,
                deliberately_false: false,
                deliberately_dangerous: false,
                certification: proof(&format!("{id}.{p}.teaching"), "credential.reviewer"),
            })
            .collect(),
        spectators: (0..10).map(|i| format!("voter.{i}")).collect(),
        roll_certification: proof(&format!("{id}.spectators"), "election.admin"),
        judgments: votes(id, choices)
            .into_iter()
            .map(|vote| Judgment {
                assessed_improvement: if vote.candidate == "being.north" {
                    50
                } else {
                    10
                },
                vote,
                conflicted: false,
                coerced: false,
            })
            .collect(),
        court_review: proof(&format!("{id}.court"), "court.reviewer"),
    }
}
fn certify(p: ContestProof) -> Action {
    Action::CertifyContest {
        id: "contest.one".into(),
        proof: p,
        sovereign_tenure: "tenure.sandman-one".into(),
    }
}

#[test]
fn elected_regional_offices_require_mastery_but_voters_need_no_form_or_property() {
    let mut g = Governance::new(&source(), policy(), 0).unwrap();
    reject(
        &mut g,
        0,
        "credential.reviewer",
        Action::Qualify {
            person: "being.north".into(),
            office: Office::Mino,
        },
    );
    qualify(&mut g, "being.north", Office::Mino);
    assert!(g.incumbent(Office::Mino).is_none());
    reject(
        &mut g,
        0,
        "being.north",
        Action::CertifyElection {
            election: election("election.bad", Office::Mino, "being.north"),
        },
    );
    let mut wrong = election("election.bad", Office::Mino, "being.north");
    wrong.roll.office = Office::Cent;
    reject(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: wrong },
    );
    apply(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection {
            election: election("election.n", Office::Mino, "being.north"),
        },
    );
    assert_eq!(g.incumbent(Office::Mino).unwrap().expires, Some(4));
    assert!(!g.may_manifest("being.north", "sandman"));
    for i in 0..10 {
        assert!(!g.may_manifest(&format!("voter.{i}"), "hecaton"));
    }
    reject(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection {
            election: election("election.king", Office::Sandman, "being.north"),
        },
    );
}
#[test]
fn elections_require_unique_eligible_ballots_and_majority_or_top_two_runoff() {
    let mut g = Governance::new(&source(), policy(), 0).unwrap();
    for person in ["being.a", "being.b", "being.c"] {
        qualify(&mut g, person, Office::Mino);
    }
    let mut e = election("election.runoff", Office::Mino, "being.a");
    e.candidates = ["being.a", "being.b", "being.c"].map(String::from).into();
    e.rounds = vec![
        votes(
            "first",
            &["being.a", "being.a", "being.b", "being.b", "being.c"],
        ),
        votes("second", &["being.a", "being.a", "being.b"]),
    ];
    let mut tie = e.clone();
    tie.rounds[0] = votes("tie", &["being.a", "being.a", "being.b", "being.c"]);
    assert!(matches!(
        reject(
            &mut g,
            0,
            "election.admin",
            Action::CertifyElection { election: tie }
        ),
        Error::RequiresLegalParameter(_)
    ));
    let mut duplicate = e.clone();
    let repeated = duplicate.rounds[1][0].clone();
    duplicate.rounds[1].push(repeated);
    reject(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection {
            election: duplicate,
        },
    );
    let mut outsider = e.clone();
    outsider.rounds[0][0].voter = "outsider".into();
    reject(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: outsider },
    );
    apply(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    assert_eq!(g.incumbent(Office::Mino).unwrap().holder, "being.a");
}
#[test]
fn terms_expire_and_missing_special_election_policy_never_invents_a_remainder() {
    let mut g = governors(policy());
    apply(
        &mut g,
        1,
        "being.north",
        Action::Resign {
            office: Office::Mino,
            tenure: "election.n.tenure".into(),
        },
    );
    let mut e = election("election.n2", Office::Mino, "being.north");
    e.special = true;
    assert!(matches!(
        reject(
            &mut g,
            1,
            "election.admin",
            Action::CertifyElection { election: e }
        ),
        Error::RequiresLegalParameter(_)
    ));
    apply(&mut g, 4, "election.admin", Action::ObserveYear);
    assert!(g.incumbent(Office::Cent).is_none());
    assert!(g.may_manifest("being.south", "pegasus"));
    apply(
        &mut g,
        4,
        "election.admin",
        Action::CertifyElection {
            election: election("election.n2", Office::Mino, "being.north"),
        },
    );
    assert_eq!(g.incumbent(Office::Mino).unwrap().expires, Some(8));
}
#[test]
fn supplied_special_term_rule_is_applied_without_extending_remainder() {
    for (rule, expiry) in [
        (SpecialTerm::UnexpiredRemainder, 4),
        (SpecialTerm::FourYears, 5),
    ] {
        let mut p = policy();
        p.special_elections.insert(
            Office::Mino,
            SpecialElectionRule {
                term: rule,
                authorization: proof("law.special", "court.reviewer"),
            },
        );
        let mut g = governors(p);
        apply(
            &mut g,
            1,
            "being.north",
            Action::Resign {
                office: Office::Mino,
                tenure: "election.n.tenure".into(),
            },
        );
        let mut e = election("election.n2", Office::Mino, "being.north");
        e.special = true;
        apply(
            &mut g,
            1,
            "election.admin",
            Action::CertifyElection { election: e },
        );
        assert_eq!(g.incumbent(Office::Mino).unwrap().expires, Some(expiry));
        assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
    }
}
fn recall(id: &str) -> Recall {
    Recall {
        office: Office::Mino,
        tenure: "election.n.tenure".into(),
        roll: roll(id, Office::Mino),
        petition: (0..2)
            .map(|i| proof(&format!("{id}.petition.{i}"), &format!("voter.{i}")))
            .collect(),
        ballots: votes(id, &["remove", "remove", "remove", "retain"]),
        court_review: proof(&format!("{id}.court"), "court.reviewer"),
    }
}
#[test]
fn recall_requires_supplied_thresholds_and_independent_due_process() {
    let mut g = governors(policy());
    assert!(matches!(
        reject(
            &mut g,
            1,
            "election.admin",
            Action::CertifyRecall {
                recall: recall("recall.unknown")
            }
        ),
        Error::RequiresLegalParameter(_)
    ));
    let mut p = policy();
    p.recall.insert(
        Office::Mino,
        RecallRule {
            petition_basis_points: 2000,
            turnout_basis_points: 4000,
            removal_basis_points: 5000,
            authorization: proof("law.recall.simulated", "court.reviewer"),
        },
    );
    p.court_reviewers.insert("voter.0".into());
    g = governors(p);
    open(&mut g);
    let mut r = recall("recall.bad");
    r.court_review.signer = "voter.0".into();
    reject(
        &mut g,
        1,
        "election.admin",
        Action::CertifyRecall { recall: r },
    );
    let mut r = recall("recall.short");
    r.petition.pop();
    reject(
        &mut g,
        1,
        "election.admin",
        Action::CertifyRecall { recall: r },
    );
    let mut r = recall("recall.tie");
    r.ballots = votes("recall.tie", &["remove", "remove", "retain", "retain"]);
    apply(
        &mut g,
        1,
        "election.admin",
        Action::CertifyRecall { recall: r },
    );
    assert!(g.incumbent(Office::Mino).is_some());
    apply(
        &mut g,
        1,
        "election.admin",
        Action::CertifyRecall {
            recall: recall("recall.success"),
        },
    );
    assert!(g.incumbent(Office::Mino).is_none());
    assert!(matches!(
        g.contests()["contest.one"].status,
        ContestStatus::Paused {
            reason: PauseReason::GovernorVacancyOrReplacement
        }
    ));
}
#[test]
fn contest_requires_both_sitting_governors_and_crowd_verdict_vacates_only_winner() {
    let mut g = Governance::new(&source(), policy(), 0).unwrap();
    qualify(&mut g, "being.north", Office::Mino);
    qualify(&mut g, "being.south", Office::Cent);
    reject(
        &mut g,
        0,
        "court.reviewer",
        Action::OpenContest {
            id: "contest.one".into(),
            court_case: "case.one".into(),
        },
    );
    g = governors(policy());
    open(&mut g);
    // North improved more by the simulated metric; spectators choose South.
    apply(
        &mut g,
        1,
        "election.admin",
        certify(contest_proof(
            "contest.first",
            &["being.south", "being.south", "being.north"],
        )),
    );
    assert_eq!(g.incumbent(Office::Sandman).unwrap().holder, "being.south");
    assert!(g.incumbent(Office::Cent).is_none());
    assert_eq!(g.incumbent(Office::Mino).unwrap().id, "election.n.tenure");
    assert!(g.may_manifest("being.south", "pegasus"));
    assert!(g.may_manifest("being.south", "sandman"));
    assert!(!g.may_manifest("being.north", "sandman"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn tie_requires_neutral_retry_and_fresh_ballots_without_automatic_sovereign() {
    let mut g = governors(policy());
    open(&mut g);
    let tie = certify(contest_proof(
        "contest.tie",
        &["being.north", "being.south"],
    ));
    apply(&mut g, 1, "election.admin", tie.clone());
    assert!(g.incumbent(Office::Sandman).is_none());
    assert!(matches!(
        g.contests()["contest.one"].status,
        ContestStatus::Paused {
            reason: PauseReason::Tie
        }
    ));
    reject(
        &mut g,
        1,
        "election.admin",
        certify(contest_proof("premature", &["being.north"])),
    );
    reject(
        &mut g,
        1,
        "being.north",
        Action::ResumeContest {
            id: "contest.one".into(),
        },
    );
    apply(
        &mut g,
        1,
        "court.reviewer",
        Action::ResumeContest {
            id: "contest.one".into(),
        },
    );
    reject(&mut g, 1, "election.admin", tie);
    apply(
        &mut g,
        1,
        "election.admin",
        certify(contest_proof(
            "contest.retry",
            &["being.north", "being.north", "being.south"],
        )),
    );
    assert_eq!(g.contests()["contest.one"].attempt, 2);
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn incapacity_compromise_sabotage_and_expiry_cannot_crown_a_substitute() {
    for reason in [PauseReason::Incapacity, PauseReason::Compromised] {
        let mut g = governors(policy());
        open(&mut g);
        apply(
            &mut g,
            1,
            "court.reviewer",
            Action::PauseContest {
                id: "contest.one".into(),
                reason,
            },
        );
        reject(
            &mut g,
            1,
            "election.admin",
            certify(contest_proof("paused", &["being.north"])),
        );
        assert!(g.incumbent(Office::Sandman).is_none());
    }
    let mut g = governors(policy());
    open(&mut g);
    let mut p = contest_proof("sabotage", &["being.north"]);
    p.teaching[0].deliberately_false = true;
    apply(&mut g, 1, "election.admin", certify(p));
    assert!(matches!(
        g.contests()["contest.one"].status,
        ContestStatus::Paused {
            reason: PauseReason::TeachingSabotage
        }
    ));
    let mut g = governors(policy());
    open(&mut g);
    apply(&mut g, 4, "election.admin", Action::ObserveYear);
    reject(
        &mut g,
        4,
        "court.reviewer",
        Action::ResumeContest {
            id: "contest.one".into(),
        },
    );
    assert!(g.incumbent(Office::Sandman).is_none());
}
#[test]
fn contest_preserves_audited_exclusions_and_requires_reciprocal_evidence() {
    let mut g = governors(policy());
    open(&mut g);
    let mut p = contest_proof("bad.practice", &["being.north"]);
    for t in &mut p.trials {
        t.candidate = "being.north".into();
        t.teacher = "being.south".into();
    }
    reject(&mut g, 1, "election.admin", certify(p));
    let mut p = contest_proof("bad.baseline", &["being.north"]);
    p.performances
        .get_mut(&Office::Mino)
        .unwrap()
        .baseline
        .clear();
    reject(&mut g, 1, "election.admin", certify(p));
    let mut p = contest_proof("bad.person", &["being.north"]);
    p.performances.get_mut(&Office::Mino).unwrap().person = "being.master-without-office".into();
    reject(&mut g, 1, "election.admin", certify(p));
    let mut p = contest_proof("exclusions", &["being.north", "being.north", "being.south"]);
    p.judgments[0].assessed_improvement = 999;
    p.judgments[1].coerced = true;
    apply(&mut g, 1, "election.admin", certify(p));
    assert_eq!(g.incumbent(Office::Sandman).unwrap().holder, "being.south");
    assert_eq!(g.contests()["contest.one"].excluded_judgments.len(), 2);
}
#[test]
fn sovereign_manifestation_and_safe_reversion_do_not_erase_mastery() {
    let mut g = governors(policy());
    open(&mut g);
    apply(
        &mut g,
        1,
        "election.admin",
        certify(contest_proof("winner", &["being.north"])),
    );
    adopt_clinical_policy(&mut g);
    register_clinical(&mut g, clinical_authorization());
    apply(
        &mut g,
        1,
        "being.north",
        Action::Manifest {
            form: "sandman".into(),
            ordinary_reversion: "hecaton".into(),
            clearance: proof("clear.sandman", "being.clinician"),
            consent: proof("consent.sandman", "being.north"),
        },
    );
    assert_eq!(g.expressed_form("being.north"), Some("sandman"));
    // Withdrawing consent cannot block the existing safe-reversion obligation.
    apply(
        &mut g,
        1,
        "being.north",
        Action::UpdateClinicalAuthorization {
            update: ClinicalUpdate::Revoke {
                record: "clinical.authorization".into(),
                certification: proof("clinical.withdrawal", "being.north"),
            },
        },
    );
    reject(
        &mut g,
        1,
        "being.north",
        Action::Resign {
            office: Office::Cent,
            tenure: "election.s.tenure".into(),
        },
    );
    apply(
        &mut g,
        2,
        "being.north",
        Action::Resign {
            office: Office::Sandman,
            tenure: "tenure.sandman-one".into(),
        },
    );
    assert_eq!(g.expressed_form("being.north"), Some("hecaton"));
    assert!(g.may_manifest("being.north", "hecaton"));
    assert!(!g.may_manifest("being.north", "sandman"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn archive_retries_conflicts_ordering_and_corruption_are_checked() {
    let mut g = governors(policy());
    open(&mut g);
    let c = command(
        &g,
        1,
        "election.admin",
        certify(contest_proof("winner", &["being.north"])),
    );
    g.execute(c.clone()).unwrap();
    let before = g.clone();
    assert_eq!(g.execute(c.clone()), Ok(InterfaceApplyOutcome::Idempotent));
    let mut conflict = c;
    conflict.year = 2;
    assert!(g.execute(conflict).is_err());
    assert_eq!(g, before);
    let bytes = g.encode().unwrap();
    assert_eq!(Governance::decode(&bytes).unwrap().encode().unwrap(), bytes);
    let corrupt = String::from_utf8(bytes)
        .unwrap()
        .replace("being.north", "being.forged");
    assert!(Governance::decode(corrupt.as_bytes()).is_err());
    let mut stale = command(&g, 2, "election.admin", Action::ObserveYear);
    stale.expected_revision -= 1;
    assert!(g.execute(stale).is_err());
    assert_eq!(g, before);
    let mut wire: serde_json::Value = serde_json::from_slice(&g.encode().unwrap()).unwrap();
    wire["events"].as_array_mut().unwrap().reverse();
    assert_eq!(
        Governance::decode(&serde_json::to_vec(&wire).unwrap()).unwrap(),
        g
    );
}
#[test]
fn historical_holders_survive_without_inventing_elections_or_legacy_form_mappings() {
    let mut state = source().institutional().clone();
    let historic = OfficeHolder {
        office: Office::Mino.id(),
        being: InstitutionalBeingId::new("being.historical").unwrap(),
        active: false,
    };
    state.catalog.office_holders.push(historic.clone());
    state.catalog.office_holders.push(OfficeHolder {
        office: Office::Sandman.id(),
        being: InstitutionalBeingId::new("being.old-sandman").unwrap(),
        active: true,
    });
    let source = WorldSession::from_institutional_state(state.clone()).unwrap();
    let original = source.persisted_state_output();
    let g = Governance::new(&source, policy(), 10).unwrap();
    assert!(g.incumbent(Office::Sandman).unwrap().imported);
    assert!(
        g.world_session()
            .unwrap()
            .institutional()
            .catalog
            .office_holders
            .contains(&historic)
    );
    assert_eq!(
        g.world_session().unwrap().persisted_state_output(),
        original
    );
    assert!(!g.may_manifest("being.historical", "hecaton"));
    state.catalog.office_holders.push(OfficeHolder {
        office: Office::Mino.id(),
        being: InstitutionalBeingId::new("being.legacy-mino").unwrap(),
        active: true,
    });
    assert!(matches!(
        Governance::new(
            &WorldSession::from_institutional_state(state).unwrap(),
            policy(),
            10
        ),
        Err(Error::RequiresLegalParameter(_))
    ));
    assert_eq!(source.persisted_state_output(), original);
}

#[test]
fn contest_voters_cannot_become_court_reviewers_on_retry() {
    let mut p = policy();
    p.court_reviewers.insert("voter.0".into());
    let mut g = governors(p);
    open(&mut g);
    apply(
        &mut g,
        1,
        "election.admin",
        certify(contest_proof("tie.first", &["being.north", "being.south"])),
    );
    let mut g = Governance::decode(&g.encode().unwrap()).unwrap();
    for action in [
        Action::ResumeContest {
            id: "contest.one".into(),
        },
        Action::PauseContest {
            id: "contest.one".into(),
            reason: PauseReason::Compromised,
        },
    ] {
        assert_eq!(
            reject(&mut g, 1, "voter.0", action),
            Error::Invalid("independent Court review required")
        );
    }
    apply(
        &mut g,
        1,
        "court.reviewer",
        Action::ResumeContest {
            id: "contest.one".into(),
        },
    );
    // A voter from the first attempt cannot review a later attempt even when
    // that later ballot does not contain their name.
    let mut p = contest_proof("tie.retry", &["being.north"]);
    p.judgments[0].vote.voter = "voter.2".into();
    p.judgments[0].vote.receipt.signer = "voter.2".into();
    p.court_review.signer = "voter.0".into();
    assert_eq!(
        reject(&mut g, 1, "election.admin", certify(p.clone())),
        Error::Invalid("independent Court review required")
    );
    p.court_review.signer = "court.reviewer".into();
    apply(&mut g, 1, "election.admin", certify(p));
    assert_eq!(g.incumbent(Office::Sandman).unwrap().holder, "being.north");
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

fn contest_after_governor_replacement() -> Governance {
    let mut p = policy();
    p.credential_reviewers.insert("being.north".into());
    let mut g = governors(p);
    open(&mut g);
    apply(&mut g, 4, "election.admin", Action::ObserveYear);
    qualify(&mut g, "being.replacement", Office::Mino);
    for (office, person, id) in [
        (Office::Mino, "being.replacement", "election.n2"),
        (Office::Cent, "being.south", "election.s2"),
    ] {
        apply(
            &mut g,
            4,
            "election.admin",
            Action::CertifyElection {
                election: election(id, office, person),
            },
        );
    }
    apply(
        &mut g,
        4,
        "court.reviewer",
        Action::ResumeContest {
            id: "contest.one".into(),
        },
    );
    Governance::decode(&g.encode().unwrap()).unwrap()
}

fn replacement_contest_proof(id: &str) -> ContestProof {
    let mut p = contest_proof(id, &["being.south"]);
    p.performances.get_mut(&Office::Mino).unwrap().person = "being.replacement".into();
    for t in &mut p.trials {
        if t.candidate == "being.north" {
            t.candidate = "being.replacement".into();
        }
        if t.teacher == "being.north" {
            t.teacher = "being.replacement".into();
        }
    }
    p.teaching[0].teacher = "being.replacement".into();
    p
}

#[test]
fn former_contest_candidates_cannot_certify_evidence_after_tenure_replacement() {
    let mut g = contest_after_governor_replacement();
    for record in 0..4 {
        let mut p = replacement_contest_proof("retry.evidence");
        let attestation = match record {
            0 => {
                &mut p
                    .performances
                    .get_mut(&Office::Mino)
                    .unwrap()
                    .baseline_evidence
            }
            1 => {
                &mut p
                    .performances
                    .get_mut(&Office::Cent)
                    .unwrap()
                    .improvement_evidence
            }
            2 => &mut p.trials[0].evidence,
            _ => &mut p.teaching[0].certification,
        };
        attestation.signer = "being.north".into();
        assert_eq!(
            reject(&mut g, 4, "election.admin", certify(p)),
            Error::Invalid("independent credential reviewer required")
        );
    }
    apply(
        &mut g,
        4,
        "election.admin",
        certify(replacement_contest_proof("retry.neutral")),
    );
    assert_eq!(g.incumbent(Office::Sandman).unwrap().holder, "being.south");
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn former_contest_candidates_remain_conflicted_spectators_after_replacement() {
    let mut g = contest_after_governor_replacement();
    let mut p = replacement_contest_proof("retry.conflicted");
    p.judgments[0].vote.voter = "being.north".into();
    p.judgments[0].vote.receipt.signer = "being.north".into();
    p.spectators.insert("being.north".into());
    let receipt = p.judgments[0].vote.receipt.record.clone();
    apply(&mut g, 4, "election.admin", certify(p));
    assert!(g.incumbent(Office::Sandman).is_none());
    assert_eq!(
        g.contests()["contest.one"].status,
        ContestStatus::Paused {
            reason: PauseReason::NoWinner
        }
    );
    assert!(
        g.contests()["contest.one"]
            .excluded_judgments
            .contains(&receipt)
    );
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::glaushouse::clinical_authorization::{
    Authorization as ClinicalAuthorization, POLICY_VERSION, Procedure, Update as ClinicalUpdate,
};

fn clinical_policy() -> Action {
    Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy {
            version: POLICY_VERSION,
            authorization: proof("mandate.clinical-v2", "court.authorizer"),
            credential_authorities: BTreeSet::from(["clinical.credential-authority".into()]),
        },
    }
}

fn adopt_clinical_policy(g: &mut Governance) {
    apply(g, g.year(), "court.authorizer", clinical_policy());
}

fn clinical_authorization() -> ClinicalAuthorization {
    ClinicalAuthorization {
        record: "clinical.authorization".into(),
        subject: "being.north".into(),
        procedure: Procedure {
            house: House::Sandmanor,
            form: "sandman".into(),
        },
        valid_from: 1,
        valid_until: 1 + 3,
        clearance: proof("clear.sandman", "being.clinician"),
        consent: proof("consent.sandman", "being.north"),
    }
}

fn clinical_registration(authorization: ClinicalAuthorization) -> Action {
    Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::Register {
            authorization,
            certification: proof("clinical.certification", "clinical.credential-authority"),
        },
    }
}

fn register_clinical(g: &mut Governance, authorization: ClinicalAuthorization) {
    apply(
        g,
        g.year(),
        "clinical.credential-authority",
        clinical_registration(authorization),
    );
}

fn clinical_manifest(a: &ClinicalAuthorization) -> Action {
    Action::Manifest {
        form: "sandman".into(),
        ordinary_reversion: "hecaton".into(),
        clearance: a.clearance.clone(),
        consent: a.consent.clone(),
    }
}

fn clinical_ready() -> Governance {
    let mut g = governors(policy());
    open(&mut g);
    apply(
        &mut g,
        1,
        "election.admin",
        certify(contest_proof("winner", &["being.north"])),
    );
    g
}

#[test]
fn clinical_office_alone_cannot_clear_or_authorize_credentials() {
    let mut g = clinical_ready();
    let a = clinical_authorization();
    assert_eq!(
        reject(&mut g, 1, "being.north", clinical_manifest(&a)),
        Error::RequiresLegalParameter("clinical authorization policy v2")
    );
    for actor in [
        "being.clinician",
        "election.admin",
        "credential.reviewer",
        "court.reviewer",
    ] {
        let mut adopt = clinical_policy();
        if let Action::UpdateClinicalAuthorization {
            update: ClinicalUpdate::AdoptPolicy { authorization, .. },
        } = &mut adopt
        {
            authorization.signer = actor.into();
        }
        reject(&mut g, 1, actor, adopt);
    }
    let mut unsupported = clinical_policy();
    if let Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy { version, .. },
    } = &mut unsupported
    {
        *version = 1;
    }
    reject(&mut g, 1, "court.authorizer", unsupported);
    let mut reused_mandate = clinical_policy();
    if let Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy { authorization, .. },
    } = &mut reused_mandate
    {
        *authorization = policy().authorization;
    }
    reject(&mut g, 1, "court.authorizer", reused_mandate);
    adopt_clinical_policy(&mut g);
    reject(&mut g, 1, "being.north", clinical_manifest(&a));
    for actor in [
        "being.clinician",
        "election.admin",
        "credential.reviewer",
        "court.reviewer",
    ] {
        let mut register = clinical_registration(a.clone());
        if let Action::UpdateClinicalAuthorization {
            update: ClinicalUpdate::Register { certification, .. },
        } = &mut register
        {
            certification.signer = actor.into();
        }
        reject(&mut g, 1, actor, register);
    }
    register_clinical(&mut g, a);
    reject(
        &mut g,
        1,
        "being.clinician",
        Action::RecognizeMastery {
            person: "being.clinician".into(),
            form: "hecaton".into(),
        },
    );
    reject(&mut g, 1, "being.clinician", Action::ObserveYear);
}

#[test]
fn clinical_independent_reviewer_or_separately_authorized_prima_donna_is_accepted() {
    for reviewer in ["clinical.independent-reviewer", "being.clinician"] {
        let mut g = clinical_ready();
        let world_before = g.world_session().unwrap().persisted_state_output();
        let mut a = clinical_authorization();
        a.clearance.signer = reviewer.into();
        adopt_clinical_policy(&mut g);
        register_clinical(&mut g, a.clone());
        apply(&mut g, 1, "being.north", clinical_manifest(&a));
        assert_eq!(g.expressed_form("being.north"), Some("sandman"));
        assert!(!g.may_manifest(reviewer, "sandman"));
        assert_eq!(
            g.world_session().unwrap().persisted_state_output(),
            world_before
        );
        assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
    }
}

#[test]
fn clinical_wrong_subject_procedure_house_and_validity_are_rejected_atomically() {
    for invalid in ["subject", "procedure", "house", "future", "expired"] {
        let mut g = clinical_ready();
        adopt_clinical_policy(&mut g);
        let mut a = clinical_authorization();
        match invalid {
            "subject" => {
                a.subject = "being.other".into();
                a.consent.signer = a.subject.clone();
            }
            "procedure" => a.procedure.form = "hecaton".into(),
            "house" => a.procedure.house = House::Stonebend,
            "future" => a.valid_from += 1,
            "expired" => a.valid_until = 1 + 1,
            _ => unreachable!(),
        }
        if invalid == "house" {
            reject(
                &mut g,
                1,
                "clinical.credential-authority",
                clinical_registration(a),
            );
        } else {
            register_clinical(&mut g, a.clone());
            let year = if invalid == "expired" { 1 + 1 } else { 1 };
            let mut manifest = clinical_manifest(&a);
            if let Action::Manifest { consent, .. } = &mut manifest {
                consent.signer = "being.north".into();
            }
            reject(&mut g, year, "being.north", manifest);
        }
    }
    for (start, end) in [(1, 1), (1 + 1, 1), (0, 1)] {
        let mut g = clinical_ready();
        adopt_clinical_policy(&mut g);
        let mut a = clinical_authorization();
        a.valid_from = start;
        a.valid_until = end;
        reject(
            &mut g,
            1,
            "clinical.credential-authority",
            clinical_registration(a),
        );
    }
}

#[test]
fn clinical_forged_clearance_or_missing_independent_consent_is_rejected() {
    let mut g = clinical_ready();
    adopt_clinical_policy(&mut g);
    let a = clinical_authorization();
    for invalid in [
        "consent-signer",
        "self-review",
        "certifier",
        "same-proof",
        "self-credential",
    ] {
        let mut register = clinical_registration(a.clone());
        if let Action::UpdateClinicalAuthorization {
            update:
                ClinicalUpdate::Register {
                    authorization,
                    certification,
                },
        } = &mut register
        {
            match invalid {
                "consent-signer" => authorization.consent.signer = "being.other".into(),
                "self-review" => authorization.clearance.signer = authorization.subject.clone(),
                "certifier" => certification.signer = "being.clinician".into(),
                "same-proof" => {
                    authorization.consent.record = authorization.clearance.record.clone()
                }
                "self-credential" => authorization.clearance.signer = certification.signer.clone(),
                _ => unreachable!(),
            }
        }
        reject(&mut g, 1, "clinical.credential-authority", register);
    }
    let before = g.clone();
    let mut missing = command(
        &g,
        1,
        "clinical.credential-authority",
        clinical_registration(a.clone()),
    );
    missing.evidence.remove(&a.consent.record);
    assert!(g.execute(missing).is_err());
    assert_eq!(g, before);
    register_clinical(&mut g, a.clone());
    for invalid in [
        "clearance-record",
        "clearance-signer",
        "consent-record",
        "consent-signer",
        "same-proof",
    ] {
        let mut manifest = clinical_manifest(&a);
        if let Action::Manifest {
            clearance, consent, ..
        } = &mut manifest
        {
            match invalid {
                "clearance-record" => clearance.record = "clearance.forged".into(),
                "clearance-signer" => clearance.signer = "being.forged".into(),
                "consent-record" => consent.record = "consent.forged".into(),
                "consent-signer" => consent.signer = "being.other".into(),
                "same-proof" => consent.record = clearance.record.clone(),
                _ => unreachable!(),
            }
        }
        reject(&mut g, 1, "being.north", manifest);
    }
    let before = g.clone();
    let mut missing = command(&g, 1, "being.north", clinical_manifest(&a));
    missing.evidence.remove(&a.consent.record);
    assert!(g.execute(missing).is_err());
    assert_eq!(g, before);
}

#[test]
fn clinical_credential_revocation_clearance_withdrawal_and_consent_withdrawal_replay() {
    for actor in [
        "clinical.credential-authority",
        "being.clinician",
        "being.north",
    ] {
        let mut g = clinical_ready();
        adopt_clinical_policy(&mut g);
        let a = clinical_authorization();
        register_clinical(&mut g, a.clone());
        let revoke = |signer: &str| Action::UpdateClinicalAuthorization {
            update: ClinicalUpdate::Revoke {
                record: a.record.clone(),
                certification: proof("clinical.revocation", signer),
            },
        };
        reject(&mut g, 1, "being.other", revoke("being.other"));
        apply(&mut g, 1, actor, revoke(actor));
        reject(&mut g, 1, "being.north", clinical_manifest(&a));
        reject(
            &mut g,
            1,
            "clinical.credential-authority",
            clinical_registration(a.clone()),
        );
        let bytes = g.encode().unwrap();
        let mut restored = Governance::decode(&bytes).unwrap();
        assert_eq!(restored, g);
        assert_eq!(restored.encode().unwrap(), bytes);
        reject(&mut restored, 1 + 1, "being.north", clinical_manifest(&a));
    }
}

#[test]
fn clinical_policy_transition_preserves_frozen_v1_history_without_creating_credentials() {
    // Captured using the pre-change workspace library, including an accepted
    // office-only manifestation. Never regenerated by the new policy.
    let bytes = include_bytes!("fixtures/clinical_authorization/sandmanor-v1.json");
    let mut g = Governance::decode(bytes).unwrap();
    assert_eq!(g.encode().unwrap(), bytes);
    assert_eq!(g.expressed_form("being.north"), Some("sandman"));
    let old_events = hollow_grove::constitutional::decode_shared_interface_archive(bytes).unwrap();
    let old_command: Command = serde_json::from_str(
        old_events
            .events()
            .last()
            .unwrap()
            .payload
            .get("command")
            .unwrap(),
    )
    .unwrap();
    let before = g.clone();
    assert_eq!(
        g.execute(old_command),
        Ok(InterfaceApplyOutcome::Idempotent)
    );
    assert_eq!(g, before);
    let world_before = g.world_session().unwrap().persisted_state_output();
    let mut a = clinical_authorization();
    reject(&mut g, 1, "being.north", clinical_manifest(&a));
    adopt_clinical_policy(&mut g);
    reject(&mut g, 1, "being.north", clinical_manifest(&a));
    reject(&mut g, 1, "court.authorizer", clinical_policy());
    let mut downgrade = clinical_policy();
    if let Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy { version, .. },
    } = &mut downgrade
    {
        *version = 1;
    }
    reject(&mut g, 1, "court.authorizer", downgrade);
    a.clearance.record = "clearance.new-v2".into();
    a.consent.record = "consent.new-v2".into();
    register_clinical(&mut g, a.clone());
    apply(&mut g, 1, "being.north", clinical_manifest(&a));
    assert_eq!(
        g.world_session().unwrap().persisted_state_output(),
        world_before
    );
    let updated = g.encode().unwrap();
    let restored = Governance::decode(&updated).unwrap();
    assert_eq!(restored, g);
    assert_eq!(restored.encode().unwrap(), updated);
    let events = hollow_grove::constitutional::decode_shared_interface_archive(&updated).unwrap();
    assert_eq!(
        &events.events()[..old_events.events().len()],
        old_events.events()
    );
}

#[test]
fn clinical_v2_replay_checks_authorization_even_with_recomputed_checksum() {
    let mut g = clinical_ready();
    adopt_clinical_policy(&mut g);
    let a = clinical_authorization();
    register_clinical(&mut g, a.clone());
    apply(&mut g, 1, "being.north", clinical_manifest(&a));
    let mut wire: serde_json::Value = serde_json::from_slice(&g.encode().unwrap()).unwrap();
    let event = wire["events"].as_array_mut().unwrap().last_mut().unwrap();
    let mut cmd: Command =
        serde_json::from_str(event["payload"]["command"].as_str().unwrap()).unwrap();
    if let Action::Manifest { clearance, .. } = &mut cmd.action {
        clearance.signer = "being.forged".into();
    }
    let json = serde_json::to_string(&cmd).unwrap();
    let checksum = json.bytes().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    });
    event["payload"]["command"] = json.into();
    event["payload"]["command-checksum"] = format!("fnv1a64:{checksum:016x}").into();
    assert_eq!(
        Governance::decode(&serde_json::to_vec(&wire).unwrap()).unwrap_err(),
        Error::Invalid("clearance or consent does not match certified records")
    );
}
