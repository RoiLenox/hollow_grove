use super::*;
use hollow_grove::world::flynt::governance::q::{
    Action as Q, Admission, Assessment, Contract, Eligibility, EndReason, EntryStatus, Judgment,
    Round,
};

fn act(action: Q) -> Action {
    Action::Q { action }
}
fn configure(g: &mut Governance) {
    apply(
        g,
        g.date(),
        "law.authorizer",
        act(Q::Configure {
            contract: Contract {
                authorization: proof("law.q-records", "law.authorizer"),
                legacy_review: proof("review.q-law-conflict", "court.reviewer"),
                assessors: (0..3).map(|i| format!("assessor.{i}")).collect(),
                judges: (0..6).map(|i| format!("judge.{i}")).collect(),
                witnesses: BTreeSet::from(["q.witness".into()]),
            },
        }),
    );
}
fn master(g: &mut Governance, person: &str) {
    for pair in Pair::ALL {
        let c = pair_credential(person, pair);
        record_pair_forms(g, &c);
        certify_pair(g, c);
    }
    certify_convergence(g, person, Convergence::Manticorp);
}
fn fresh_q() -> Governance {
    let mut g = fresh();
    adopt_convergence(&mut g);
    configure(&mut g);
    for person in ["person.one", "person.two", "being.flynt.tross"] {
        master(&mut g, person);
    }
    g
}
fn eligibility(person: &str, id: &str) -> Eligibility {
    Eligibility {
        person: person.into(),
        valid_until: date(1, 0),
        certification: proof(&format!("{id}.standing"), "credential.reviewer"),
    }
}
fn admission(person: &str, id: &str) -> Admission {
    Admission {
        id: id.into(),
        eligibility: eligibility(person, id),
        assessments: std::array::from_fn(|i| Assessment {
            affirmative: i < 2,
            finding: proof(&format!("{id}.assessment.{i}"), &format!("assessor.{i}")),
        }),
        consent: proof(&format!("{id}.consent"), person),
    }
}
fn admit(g: &mut Governance, person: &str, id: &str) {
    apply(
        g,
        g.date(),
        "election.admin",
        act(Q::Admit {
            admission: admission(person, id),
        }),
    );
}
fn schedule(at: Date) -> Action {
    act(Q::Schedule {
        contest: "contest.one".into(),
        entries: vec!["entry.one".into()],
        at,
        readiness: [
            eligibility("person.one", "ready.one"),
            eligibility("being.flynt.tross", "ready.tross"),
        ],
    })
}
fn scheduled() -> Governance {
    let mut g = fresh_q();
    admit(&mut g, "person.one", "entry.one");
    admit(&mut g, "person.two", "entry.two");
    apply(&mut g, date(0, 0), "election.admin", schedule(date(0, 5)));
    g
}
fn round(round: Round, winner: &str) -> Action {
    let offset = if round == Round::CloseOffPoseOff {
        0
    } else {
        3
    };
    act(Q::RecordRound {
        contest: "contest.one".into(),
        round,
        panel: std::array::from_fn(|i| Judgment {
            winner: winner.into(),
            finding: proof(
                &format!("judgment.{}", offset + i),
                &format!("judge.{}", offset + i),
            ),
        }),
    })
}
fn sudden(reason: EndReason) -> Action {
    act(Q::RecordSuddenDeath {
        contest: "contest.one".into(),
        loser: "being.flynt.tross".into(),
        reason,
        outcome: proof(
            "contest.conclusion",
            if reason == EndReason::Concession {
                "being.flynt.tross"
            } else {
                "execution.reviewer"
            },
        ),
        witness: proof("contest.witness", "q.witness"),
    })
}

#[test]
fn q_requires_an_explicit_record_contract_and_never_enacts_succession() {
    let mut g = fresh();
    assert_eq!(
        reject(
            &mut g,
            date(0, 0),
            "election.admin",
            act(Q::Admit {
                admission: admission("person.one", "entry.one")
            })
        ),
        Error::UnresolvedLaw(UnresolvedLaw::QRecordContract)
    );
    configure(&mut g);
    assert_eq!(
        reject(
            &mut g,
            date(0, 0),
            "law.authorizer",
            act(Q::FinalizeSuccession {
                contest: "anything".into()
            })
        ),
        Error::UnresolvedLaw(UnresolvedLaw::QSuccessionReconciliation)
    );
    assert_eq!(g.tross(), Some("being.flynt.tross"));
}

#[test]
fn q_admission_requires_current_manticorp_and_three_independent_two_yes_findings() {
    let mut g = fresh_q();
    for invalid in [
        "unqualified",
        "incumbent",
        "expired",
        "self",
        "duplicate",
        "forged",
        "one-yes",
        "consent",
        "reuse",
    ] {
        let mut a = admission("person.one", "entry.one");
        match invalid {
            "unqualified" => a.eligibility.person = "person.unqualified".into(),
            "incumbent" => a = admission("being.flynt.tross", "entry.tross"),
            "expired" => a.eligibility.valid_until = date(0, 0),
            "self" => a.assessments[0].finding.signer = "person.one".into(),
            "duplicate" => {
                a.assessments[1].finding.signer = a.assessments[0].finding.signer.clone()
            }
            "forged" => a.assessments[0].finding.signer = "being.flynt.tross".into(),
            "one-yes" => a.assessments[1].affirmative = false,
            "consent" => a.consent.signer = "election.admin".into(),
            "reuse" => a.assessments[1].finding.record = a.assessments[0].finding.record.clone(),
            _ => unreachable!(),
        }
        reject(
            &mut g,
            date(0, 1),
            "election.admin",
            act(Q::Admit { admission: a }),
        );
    }
    let mut cmd = command(
        &g,
        date(0, 0),
        "election.admin",
        act(Q::Admit {
            admission: admission("person.one", "entry.one"),
        }),
    );
    cmd.evidence.remove("entry.one.assessment.0");
    let before = g.clone();
    assert!(g.execute(cmd).is_err());
    assert_eq!(g, before);
    admit(&mut g, "person.one", "entry.one");
    assert_eq!(g.q_entries().len(), 1);
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        act(Q::Admit {
            admission: admission("person.one", "entry.duplicate"),
        }),
    );
    reject(
        &mut g,
        date(0, 0),
        "being.flynt.tross",
        act(Q::Admit {
            admission: admission("person.two", "entry.two"),
        }),
    );
    assert!(g.must_enter_q("person.one"));
    assert_eq!(g.expressed_form("person.one"), None);
}

#[test]
fn q_fifo_schedule_and_thirty_day_boundary_require_fresh_readiness() {
    let mut g = fresh_q();
    admit(&mut g, "person.one", "entry.one");
    admit(&mut g, "person.two", "entry.two");
    let mut reordered = schedule(date(0, 5));
    if let Action::Q {
        action: Q::Schedule { entries, .. },
    } = &mut reordered
    {
        entries[0] = "entry.two".into();
    }
    reject(&mut g, date(0, 0), "election.admin", reordered);
    reject(
        &mut g,
        date(0, 0),
        "being.flynt.tross",
        schedule(date(0, 5)),
    );
    reject(&mut g, date(0, 0), "election.admin", schedule(date(0, 31)));
    let mut bad = schedule(date(0, 5));
    if let Action::Q {
        action: Q::Schedule { readiness, .. },
    } = &mut bad
    {
        readiness[1].valid_until = date(0, 4);
    }
    reject(&mut g, date(0, 0), "election.admin", bad);
    apply(&mut g, date(0, 0), "election.admin", schedule(date(0, 30)));
    assert_eq!(
        g.q_contest().unwrap().participants,
        ["person.one", "being.flynt.tross"]
    );
    assert_eq!(
        g.q_entries()[0].status,
        EntryStatus::InContest("contest.one".into())
    );
    assert_eq!(g.q_entries()[1].status, EntryStatus::Waiting);
}

#[test]
fn q_withdrawal_expiry_and_reentry_preserve_history_and_fifo() {
    let mut g = fresh_q();
    admit(&mut g, "person.one", "entry.one");
    admit(&mut g, "person.two", "entry.two");
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        act(Q::Withdraw {
            entry: "entry.one".into(),
        }),
    );
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        act(Q::Expire {
            entry: "entry.one".into(),
        }),
    );
    apply(
        &mut g,
        date(0, 1),
        "person.one",
        act(Q::Withdraw {
            entry: "entry.one".into(),
        }),
    );
    admit(&mut g, "person.one", "entry.rejoined");
    assert_eq!(g.q_entries()[0].status, EntryStatus::Withdrawn);
    assert_eq!(g.q_entries()[2].admission.eligibility.person, "person.one");
    apply(
        &mut g,
        date(1, 1),
        "election.admin",
        act(Q::Expire {
            entry: "entry.two".into(),
        }),
    );
    assert_eq!(g.q_entries()[1].status, EntryStatus::Expired);
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn q_rounds_need_order_independent_panels_and_no_reused_evidence() {
    let mut g = scheduled();
    reject(
        &mut g,
        date(0, 4),
        "election.admin",
        round(Round::CloseOffPoseOff, "person.one"),
    );
    reject(
        &mut g,
        date(0, 5),
        "election.admin",
        round(Round::SocialEngineering, "person.one"),
    );
    for invalid in ["forged", "duplicate", "outsider", "reuse"] {
        let mut a = round(Round::CloseOffPoseOff, "person.one");
        if let Action::Q {
            action: Q::RecordRound { panel, .. },
        } = &mut a
        {
            match invalid {
                "forged" => panel[0].finding.signer = "being.flynt.tross".into(),
                "duplicate" => panel[1].finding.signer = panel[0].finding.signer.clone(),
                "outsider" => panel[0].winner = "person.two".into(),
                "reuse" => panel[1].finding.record = panel[0].finding.record.clone(),
                _ => unreachable!(),
            }
        }
        reject(&mut g, date(0, 5), "election.admin", a);
    }
    apply(
        &mut g,
        date(0, 5),
        "election.admin",
        round(Round::CloseOffPoseOff, "person.one"),
    );
    let mut reused = round(Round::SocialEngineering, "person.one");
    if let Action::Q {
        action: Q::RecordRound { panel, .. },
    } = &mut reused
    {
        panel[0].finding.signer = "judge.0".into();
    }
    reject(&mut g, date(0, 5), "election.admin", reused);
    // A dissenting vote is allowed; majority is still exactly two of three.
    let mut second = round(Round::SocialEngineering, "person.one");
    if let Action::Q {
        action: Q::RecordRound { panel, .. },
    } = &mut second
    {
        panel[2].winner = "being.flynt.tross".into();
    }
    apply(&mut g, date(0, 5), "election.admin", second);
    assert_eq!(g.q_contest().unwrap().result.as_deref(), Some("person.one"));
    reject(
        &mut g,
        date(0, 5),
        "election.admin",
        sudden(EndReason::Concession),
    );
    assert_eq!(
        reject(
            &mut g,
            date(0, 5),
            "election.admin",
            act(Q::FinalizeSuccession {
                contest: "contest.one".into()
            })
        ),
        Error::UnresolvedLaw(UnresolvedLaw::QSuccessionReconciliation)
    );
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert!(g.has_bodily_mastery("being.flynt.tross", BodilyForm::Manticorp));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn q_split_requires_verified_concession_or_inability_without_final_jury() {
    for reason in [EndReason::Concession, EndReason::InabilityToContinue] {
        let mut g = scheduled();
        reject(&mut g, date(0, 5), "election.admin", sudden(reason));
        apply(
            &mut g,
            date(0, 5),
            "election.admin",
            round(Round::CloseOffPoseOff, "person.one"),
        );
        apply(
            &mut g,
            date(0, 5),
            "election.admin",
            round(Round::SocialEngineering, "being.flynt.tross"),
        );
        assert_eq!(g.q_contest().unwrap().result, None);
        let mut bad = sudden(reason);
        if let Action::Q {
            action: Q::RecordSuddenDeath { outcome, .. },
        } = &mut bad
        {
            outcome.signer = "judge.0".into();
        }
        reject(&mut g, date(0, 5), "election.admin", bad);
        let mut bad = sudden(reason);
        if let Action::Q {
            action: Q::RecordSuddenDeath { witness, .. },
        } = &mut bad
        {
            witness.signer = "person.one".into();
        }
        reject(&mut g, date(0, 5), "election.admin", bad);
        let cmd = command(&g, date(0, 5), "election.admin", sudden(reason));
        assert_eq!(g.execute(cmd.clone()), Ok(InterfaceApplyOutcome::Applied));
        let bytes = g.encode().unwrap();
        assert_eq!(g.execute(cmd), Ok(InterfaceApplyOutcome::Idempotent));
        assert_eq!(g.encode().unwrap(), bytes);
        assert_eq!(Governance::decode(&bytes).unwrap().encode().unwrap(), bytes);
        assert_eq!(g.q_contest().unwrap().result.as_deref(), Some("person.one"));
        assert_eq!(g.tross(), Some("being.flynt.tross"));
        assert_eq!(
            reject(&mut g, date(0, 12), "election.admin", schedule(date(0, 12))),
            Error::UnresolvedLaw(UnresolvedLaw::QSuccessionReconciliation)
        );
    }
}

#[test]
fn q_stale_and_conflicting_commands_cannot_change_admission_order() {
    let mut g = fresh_q();
    let cmd = command(
        &g,
        date(0, 0),
        "election.admin",
        act(Q::Admit {
            admission: admission("person.one", "entry.one"),
        }),
    );
    g.execute(cmd.clone()).unwrap();
    let before = g.clone();
    let mut stale = cmd.clone();
    stale.id = "stale.entry".into();
    assert!(g.execute(stale).is_err());
    assert_eq!(g, before);
    let mut conflict = cmd;
    conflict.actor = "being.flynt.tross".into();
    assert!(g.execute(conflict).is_err());
    assert_eq!(g, before);
}

#[test]
fn q_final_vacancy_is_blocked_pending_lawful_vacancy_and_source_codec_reconciliation() {
    let mut g = fresh_q();
    admit(&mut g, "person.one", "entry.one");
    for enough_candidates in [false, true] {
        if enough_candidates {
            admit(&mut g, "person.two", "entry.two");
        }
        assert_eq!(
            reject(
                &mut g,
                date(0, 0),
                "election.admin",
                act(Q::OpenFinalVacancy)
            ),
            Error::UnresolvedLaw(UnresolvedLaw::QSuccessionReconciliation)
        );
        assert_eq!(g.tross(), Some("being.flynt.tross"));
    }
}

#[test]
fn completed_legacy_gate_record_and_q_victory_still_cannot_transfer_or_expire_offices() {
    use hollow_grove::{
        TrossSuccessionRecord, TrossSuccessionStage as Stage, TrossSuccessionTrigger,
    };
    let mut g = scheduled();
    qualify(&mut g, "person.nav", Office::Navigator);
    record(
        &mut g,
        0,
        election("closure.nav", Office::Navigator, "person.nav"),
    );
    apply(&mut g, date(0, 5), "election.admin", seat("closure.nav"));
    apply(
        &mut g,
        date(0, 5),
        "election.admin",
        round(Round::CloseOffPoseOff, "person.one"),
    );
    apply(
        &mut g,
        date(0, 5),
        "election.admin",
        round(Round::SocialEngineering, "person.one"),
    );
    // Legacy staged records are not signatures or a legally enacted Q bridge.
    let mut case = TrossSuccessionRecord::open(
        "case.legacy-closure",
        "being.flynt.tross",
        "person.one",
        TrossSuccessionTrigger::Abdication,
    )
    .unwrap();
    for (stage, id) in [
        (Stage::StonebendNamed, "stonebend.name"),
        (Stage::SandmanorProven, "sandmanor.proof"),
        (Stage::GlaushouseCleared, "glaüshouse.clearance"),
        (Stage::FlyntRecognized, "flynt.recognition"),
        (Stage::CourtReviewed, "court.review"),
    ] {
        case.advance(stage, Some(id.into())).unwrap();
    }
    case.advance(Stage::StonebendSealed, None).unwrap();
    assert_eq!(case.active_tross_id, "being.flynt.tross");
    for actor in [
        "election.admin",
        "law.authorizer",
        "court.reviewer",
        "person.one",
        "being.flynt.tross",
        "person.nav",
    ] {
        let before = g.clone();
        let bytes = g.encode().unwrap();
        let mut cmd = command(
            &g,
            date(6, 0),
            actor,
            act(Q::FinalizeSuccession {
                contest: "contest.one".into(),
            }),
        );
        cmd.evidence.extend([
            "case.legacy-closure".into(),
            "court.review".into(),
            "stonebend.sealed-title".into(),
        ]);
        assert_eq!(
            g.execute(cmd),
            Err(Error::UnresolvedLaw(
                UnresolvedLaw::QSuccessionReconciliation
            ))
        );
        assert_eq!(g, before);
        assert_eq!(g.encode().unwrap(), bytes);
        assert!(g.incumbent(Office::Navigator).is_some());
    }
    assert_eq!(g.q_contest().unwrap().result.as_deref(), Some("person.one"));
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
