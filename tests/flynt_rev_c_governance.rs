//! Simulated legal parameters and authenticated records; never production defaults.
use hollow_grove::constitutional::InterfaceApplyOutcome;
use hollow_grove::institution::{InstitutionalEntityId, RelationshipKind};
use hollow_grove::world::flynt::{self, governance::*};
use hollow_grove::world::session::WorldSession;
use std::collections::{BTreeMap, BTreeSet};

#[path = "flynt_rev_c_closure.rs"]
mod closure_tests;
#[path = "flynt_rev_c_equipment.rs"]
mod equipment_tests;
#[path = "flynt_rev_c_q.rs"]
mod q_tests;

fn proof(record: &str, signer: &str) -> Attestation {
    Attestation {
        record: record.into(),
        signer: signer.into(),
    }
}
fn date(year: u64, day: u64) -> Date {
    Date { year, day }
}
fn policy() -> Policy {
    Policy {
        authorization: proof("law.flynt", "law.authorizer"),
        administrator: "election.admin".into(),
        credential_reviewers: BTreeSet::from(["credential.reviewer".into()]),
        court_reviewers: BTreeSet::from(["court.reviewer".into()]),
        execution_reviewers: BTreeSet::from(["execution.reviewer".into()]),
        first_elections: BTreeMap::from([
            (Office::Navigator, date(0, 0)),
            (Office::Chief, date(2, 0)),
            (Office::Mariner, date(4, 0)),
        ]),
        days_per_year: Some(LegalDays {
            value: 100,
            authorization: proof("law.test-calendar", "law.authorizer"),
        }),
        veto_days: Some(LegalDays {
            value: 5,
            authorization: proof("law.test-veto", "law.authorizer"),
        }),
    }
}
fn fresh() -> Governance {
    Governance::new(&WorldSession::canonical(), policy(), date(0, 0)).unwrap()
}
fn command(g: &Governance, when: Date, actor: &str, action: Action) -> Command {
    fn collect(v: &serde_json::Value, ids: &mut BTreeSet<String>) {
        match v {
            serde_json::Value::Object(fields) => {
                if let Some(serde_json::Value::String(s)) = fields.get("record") {
                    ids.insert(s.clone());
                }
                for v in fields.values() {
                    collect(v, ids);
                }
            }
            serde_json::Value::Array(values) => {
                for v in values {
                    collect(v, ids);
                }
            }
            _ => {}
        }
    }
    let mut evidence = BTreeSet::from([format!("evidence.{}", g.revision())]);
    collect(&serde_json::to_value(&action).unwrap(), &mut evidence);
    Command {
        id: format!("event.{}", g.revision()),
        expected_revision: g.revision(),
        date: when,
        actor: actor.into(),
        evidence,
        action,
    }
}
fn apply(g: &mut Governance, when: Date, actor: &str, action: Action) {
    let c = command(g, when, actor, action);
    assert_eq!(g.execute(c), Ok(InterfaceApplyOutcome::Applied));
}
fn reject(g: &mut Governance, when: Date, actor: &str, action: Action) -> Error {
    let before = g.clone();
    let bytes = g.encode().unwrap();
    let c = command(g, when, actor, action);
    let e = g.execute(c).expect_err("must reject");
    assert_eq!(*g, before);
    assert_eq!(g.encode().unwrap(), bytes);
    e
}
fn pair_credential(person: &str, pair: Pair) -> AmbisynthCredential {
    let prefix = format!("{person}.{pair:?}").to_ascii_lowercase();
    AmbisynthCredential {
        person: person.into(),
        pair,
        forms: pair.forms().map(|form| FormMastery {
            person: person.into(),
            form,
            certification: proof(
                &format!("{prefix}.{form:?}").to_ascii_lowercase(),
                "credential.reviewer",
            ),
        }),
        switching: proof(&format!("{prefix}.switching"), "credential.reviewer"),
        certification: proof(&format!("{prefix}.ambisynth"), "credential.reviewer"),
    }
}
fn qualification(person: &str, office: Office) -> Action {
    Action::QualifyCommander {
        office,
        credential: pair_credential(person, office.pair()),
        competence: proof(
            &format!("{person}.{office:?}.competence").to_ascii_lowercase(),
            "credential.reviewer",
        ),
    }
}
fn qualify(g: &mut Governance, person: &str, office: Office) {
    apply(
        g,
        g.date(),
        "credential.reviewer",
        qualification(person, office),
    );
}
fn votes(id: &str, choices: &[&str]) -> Vec<Vote> {
    choices
        .iter()
        .enumerate()
        .map(|(i, c)| Vote {
            voter: format!("peer.{i}"),
            candidate: (*c).into(),
            receipt: proof(&format!("{id}.vote.{i}"), &format!("peer.{i}")),
        })
        .collect()
}
fn election(id: &str, office: Office, winner: &str) -> Election {
    Election {
        id: id.into(),
        office,
        candidates: BTreeSet::from([winner.into()]),
        roll: ElectoralRoll {
            office,
            eligible: (0..5).map(|i| format!("peer.{i}")).collect(),
            certification: proof(&format!("{id}.roll"), "election.admin"),
        },
        rounds: vec![votes(id, &[winner; 3])],
        winner: winner.into(),
        special: false,
        certification: proof(&format!("{id}.certification"), "election.admin"),
    }
}
fn record(g: &mut Governance, year: u64, e: Election) {
    apply(
        g,
        date(year, 0),
        "election.admin",
        Action::RecordElection { election: e },
    );
}
fn seat(id: &str) -> Action {
    Action::SeatCommander {
        election: id.into(),
        tenure: format!("{id}.tenure"),
        appeals_resolved: proof(&format!("{id}.appeals"), "court.reviewer"),
    }
}

#[test]
fn registry_has_one_sovereign_three_direct_commanders_and_no_cover_offices() {
    let g = fresh();
    let c = g.command_catalog().unwrap();
    assert_eq!(c.offices.len(), 4);
    assert_eq!(c.office_holders.len(), 1);
    for o in Office::ALL {
        assert_eq!(c.offices.iter().filter(|x| x.id == o.id()).count(), 1);
        assert!(
            c.relationships
                .iter()
                .any(|r| r.source == InstitutionalEntityId::Office(o.id())
                    && r.kind == RelationshipKind::AnswersTo
                    && r.target == InstitutionalEntityId::Office(flynt::tross_office_id()))
        );
    }
    assert_eq!(
        Office::ALL
            .iter()
            .flat_map(|o| o.career())
            .collect::<BTreeSet<_>>()
            .len(),
        9
    );
    assert_eq!(Office::Mariner.career()[2], CareerRank::Marquise);
    assert_ne!(Office::Mariner.id(), flynt::marquise_office_id());
    assert_eq!(
        Office::Chief.historical_covers(),
        ["The Beautiful Beast", "Tommy Rose"]
    );
    assert_eq!(Face::ALL.len(), 3);
    assert!(Face::Gallows.purpose().contains("not the Syndicate"));
    assert_eq!(
        g.source_snapshot(),
        WorldSession::canonical().persisted_state_output()
    );
}
#[test]
fn careers_qualifications_and_offices_are_independent() {
    let mut g = fresh();
    apply(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::RecognizeCareer {
            person: "person.nav".into(),
            rank: CareerRank::Navigator,
            certification: proof("career.nav", "credential.reviewer"),
        },
    );
    assert!(g.has_career("person.nav", CareerRank::Navigator));
    assert!(!g.qualified("person.nav", Office::Navigator));
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        Action::RecordElection {
            election: election("e.nav", Office::Navigator, "person.nav"),
        },
    );
    qualify(&mut g, "person.nav", Office::Navigator);
    assert!(g.incumbent(Office::Navigator).is_none());
    assert!(!g.qualified("person.nav", Office::Chief));
    assert_eq!(g.tross(), Some("being.flynt.tross"));
}
#[test]
fn three_pair_qualified_commanders_have_staggered_six_year_terms() {
    let mut g = fresh();
    for (office, year, person, id) in [
        (Office::Navigator, 0, "person.nav", "e.nav"),
        (Office::Chief, 2, "person.chief", "e.chief"),
        (Office::Mariner, 4, "person.mar", "e.mar"),
    ] {
        qualify(&mut g, person, office);
        record(&mut g, year, election(id, office, person));
        assert!(g.incumbent(office).is_none());
        apply(&mut g, date(year, 5), "election.admin", seat(id));
        let t = g.incumbent(office).unwrap();
        assert_eq!(t.holder, person);
        assert_eq!(t.expires, date(year + 6, 0));
        assert_eq!(t.began, date(year, 5));
    }
    assert_eq!(
        g.command_catalog()
            .unwrap()
            .office_holders
            .iter()
            .filter(|h| h.active)
            .count(),
        4
    );
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn pair_scope_self_review_missing_proofs_and_repeated_credentials_fail_atomically() {
    for failure in ["pair", "subject", "form", "self", "duplicate"] {
        let mut g = fresh();
        let mut a = qualification("person.nav", Office::Navigator);
        if let Action::QualifyCommander {
            credential: c,
            competence,
            ..
        } = &mut a
        {
            match failure {
                "pair" => *c = pair_credential("person.nav", Pair::WerewolfVampire),
                "subject" => c.forms[0].person = "person.other".into(),
                "form" => c.forms[0].form = Form::Werewolf,
                "self" => c.certification.signer = "person.nav".into(),
                "duplicate" => competence.record = c.switching.record.clone(),
                _ => unreachable!(),
            }
        }
        reject(&mut g, date(0, 0), "credential.reviewer", a);
    }
    let mut g = fresh();
    let before = g.clone();
    let mut c = command(
        &g,
        date(0, 0),
        "credential.reviewer",
        qualification("person.nav", Office::Navigator),
    );
    c.evidence.clear();
    assert!(g.execute(c).is_err());
    assert_eq!(g, before);
    qualify(&mut g, "person.nav", Office::Navigator);
    reject(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        qualification("person.nav", Office::Navigator),
    );
    reject(
        &mut g,
        date(0, 0),
        "being.flynt.tross",
        qualification("person.other", Office::Navigator),
    );
}
#[test]
fn majority_runoff_and_ballot_integrity_are_enforced() {
    let mut g = fresh();
    for p in ["a", "b", "c"] {
        qualify(&mut g, p, Office::Navigator);
    }
    let mut e = election("runoff", Office::Navigator, "a");
    e.candidates = BTreeSet::from(["a".into(), "b".into(), "c".into()]);
    e.rounds = vec![
        votes("r1", &["a", "a", "b", "b", "c"]),
        votes("r2", &["a", "a", "a", "b", "b"]),
    ];
    for bad in [
        "duplicate",
        "ineligible",
        "forged",
        "winner",
        "repeat-receipt",
    ] {
        let mut invalid = e.clone();
        match bad {
            "duplicate" => {
                let duplicate = invalid.rounds[0][0].clone();
                invalid.rounds[0].push(duplicate);
            }
            "ineligible" => {
                invalid.roll.eligible.remove("peer.0");
            }
            "forged" => invalid.rounds[0][0].receipt.signer = "forger".into(),
            "winner" => invalid.winner = "c".into(),
            "repeat-receipt" => {
                invalid.rounds[1][0].receipt.record = invalid.rounds[0][0].receipt.record.clone()
            }
            _ => unreachable!(),
        }
        reject(
            &mut g,
            date(0, 0),
            "election.admin",
            Action::RecordElection { election: invalid },
        );
    }
    record(&mut g, 0, e);
    apply(&mut g, date(0, 5), "election.admin", seat("runoff"));
    assert_eq!(g.incumbent(Office::Navigator).unwrap().holder, "a");
}
#[test]
fn missing_law_never_invents_veto_recall_cohorts_or_special_terms() {
    for missing in ["veto", "calendar", "cohort"] {
        let mut p = policy();
        let expected = match missing {
            "veto" => {
                p.veto_days = None;
                UnresolvedLaw::VetoWindow
            }
            "calendar" => {
                p.days_per_year = None;
                UnresolvedLaw::Calendar
            }
            _ => {
                p.first_elections.remove(&Office::Navigator);
                UnresolvedLaw::FirstElectionCohort
            }
        };
        let mut g = Governance::new(&WorldSession::canonical(), p, date(0, 0)).unwrap();
        qualify(&mut g, "a", Office::Navigator);
        assert_eq!(
            reject(
                &mut g,
                date(0, 0),
                "election.admin",
                Action::RecordElection {
                    election: election("e", Office::Navigator, "a")
                }
            ),
            Error::UnresolvedLaw(expected)
        );
    }
    let mut g = fresh();
    assert_eq!(
        reject(
            &mut g,
            date(0, 0),
            "person.nav",
            Action::RequestRecall {
                office: Office::Navigator
            }
        ),
        Error::UnresolvedLaw(UnresolvedLaw::RecallThresholds)
    );
    let mut e = election("special", Office::Navigator, "a");
    e.special = true;
    assert_eq!(
        reject(
            &mut g,
            date(0, 0),
            "election.admin",
            Action::RecordElection { election: e }
        ),
        Error::UnresolvedLaw(UnresolvedLaw::SpecialElectionTerm)
    );
    let mut p = policy();
    p.first_elections.insert(Office::Chief, date(3, 0));
    assert!(Governance::new(&WorldSession::canonical(), p, date(0, 0)).is_err());
}
#[test]
fn bounded_veto_and_independent_appeal_cannot_install_sovereign_favorites() {
    for upheld in [true, false] {
        let mut g = fresh();
        qualify(&mut g, "a", Office::Navigator);
        record(&mut g, 0, election("e", Office::Navigator, "a"));
        reject(&mut g, date(0, 4), "election.admin", seat("e"));
        apply(
            &mut g,
            date(0, 2),
            "being.flynt.tross",
            Action::VetoElection {
                election: "e".into(),
                grounds: proof("veto.grounds", "being.flynt.tross"),
            },
        );
        reject(&mut g, date(0, 5), "election.admin", seat("e"));
        reject(
            &mut g,
            date(0, 3),
            "a",
            Action::ResolveVeto {
                election: "e".into(),
                upheld,
                review: proof("appeal.self", "a"),
            },
        );
        apply(
            &mut g,
            date(0, 3),
            "court.reviewer",
            Action::ResolveVeto {
                election: "e".into(),
                upheld,
                review: proof("appeal.independent", "court.reviewer"),
            },
        );
        if upheld {
            reject(&mut g, date(0, 5), "election.admin", seat("e"));
        } else {
            apply(&mut g, date(0, 5), "election.admin", seat("e"));
        }
        assert_eq!(g.tross(), Some("being.flynt.tross"));
    }
}
#[test]
fn tenure_exit_preserves_qualification_and_career_and_re_election_is_not_automatic() {
    let mut g = fresh();
    qualify(&mut g, "a", Office::Navigator);
    record(&mut g, 0, election("e", Office::Navigator, "a"));
    apply(&mut g, date(0, 5), "election.admin", seat("e"));
    reject(
        &mut g,
        date(1, 0),
        "being.flynt.tross",
        Action::ResignCommander {
            office: Office::Navigator,
            tenure: "e.tenure".into(),
        },
    );
    apply(&mut g, date(6, 0), "election.admin", Action::ObserveDate);
    assert!(g.incumbent(Office::Navigator).is_none());
    assert!(g.qualified("a", Office::Navigator));
    assert_eq!(g.tenures()[0].ended, Some(date(6, 0)));
    assert!(g.tenures()[0].closure_event.is_some());
    record(&mut g, 6, election("reelect", Office::Navigator, "a"));
    apply(&mut g, date(6, 5), "election.admin", seat("reelect"));
    assert_eq!(g.tenures().len(), 2);
    assert_eq!(g.incumbent(Office::Navigator).unwrap().expires, date(12, 0));
    apply(
        &mut g,
        date(7, 0),
        "a",
        Action::ResignCommander {
            office: Office::Navigator,
            tenure: "reelect.tenure".into(),
        },
    );
    assert!(g.qualified("a", Office::Navigator));
    assert!(g.incumbent(Office::Navigator).is_none());
}
#[test]
fn archives_preserve_source_idempotency_revision_order_and_tamper_detection() {
    let mut g = fresh();
    qualify(&mut g, "a", Office::Navigator);
    let c = command(&g, date(1, 0), "election.admin", Action::ObserveDate);
    assert_eq!(g.execute(c.clone()), Ok(InterfaceApplyOutcome::Applied));
    let before = g.clone();
    assert_eq!(g.execute(c.clone()), Ok(InterfaceApplyOutcome::Idempotent));
    let mut conflicting = c.clone();
    conflicting.date = date(2, 0);
    assert!(g.execute(conflicting).is_err());
    assert_eq!(g, before);
    let mut stale = command(&g, date(2, 0), "election.admin", Action::ObserveDate);
    stale.expected_revision -= 1;
    assert!(g.execute(stale).is_err());
    assert_eq!(g, before);
    let bytes = g.encode().unwrap();
    let restored = Governance::decode(&bytes).unwrap();
    assert_eq!(restored, g);
    assert_eq!(restored.encode().unwrap(), bytes);
    let corrupt = String::from_utf8(bytes)
        .unwrap()
        .replace("person.nav", "person.fake")
        .replace("credential.reviewer", "credential.forged");
    assert!(Governance::decode(corrupt.as_bytes()).is_err());
    reject(&mut g, date(0, 0), "election.admin", Action::ObserveDate);
}
#[test]
fn legacy_aliases_remain_in_source_without_inventing_elected_tenures() {
    let world = WorldSession::canonical();
    let g = Governance::new(&world, policy(), date(0, 0)).unwrap();
    assert_eq!(g.source_snapshot(), world.persisted_state_output());
    assert!(g.tenures().is_empty());
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert!(flynt::canonical_flynt_institutions().validate().is_ok());
    let mut state = world.institutional().clone();
    let c = g.command_catalog().unwrap();
    state.catalog.offices.push(
        c.offices
            .iter()
            .find(|o| o.id == Office::Navigator.id())
            .unwrap()
            .clone(),
    );
    state
        .catalog
        .office_holders
        .push(hollow_grove::institution::OfficeHolder {
            office: Office::Navigator.id(),
            being: hollow_grove::institution::InstitutionalBeingId::new("existing.commander")
                .unwrap(),
            active: true,
        });
    let existing = WorldSession::from_institutional_state(state).unwrap();
    assert_eq!(
        Governance::new(&existing, policy(), date(0, 0)).unwrap_err(),
        Error::UnresolvedLaw(UnresolvedLaw::ExistingCommanderContinuity)
    );
}

use hollow_grove::world::flynt::synthesis::{BodilyForm, ExpressionRecord, PairTerm};
fn record_pair_forms(g: &mut Governance, c: &AmbisynthCredential) {
    for mastery in &c.forms {
        apply(
            g,
            g.date(),
            "credential.reviewer",
            Action::RecordFormMastery {
                mastery: mastery.clone(),
            },
        );
    }
}
fn certify_pair(g: &mut Governance, c: AmbisynthCredential) {
    apply(
        g,
        g.date(),
        "credential.reviewer",
        Action::CertifyAmbisynth {
            credential: c,
            terminology: PairTerm::Ambisynth,
        },
    );
}
fn expression(g: &Governance, person: &str, form: BodilyForm, id: &str) -> Action {
    Action::RecordExpression {
        record: ExpressionRecord {
            person: person.into(),
            form,
            previous: g.expressed_form(person),
            execution: proof(&format!("{id}.execution"), "execution.reviewer"),
            consent: proof(&format!("{id}.consent"), person),
        },
    }
}
#[test]
fn all_three_ambisynths_require_their_own_two_physical_forms_and_switching() {
    let mut g = fresh();
    assert_eq!(Form::ALL.len(), 6);
    assert_eq!(Form::ALL.iter().filter(|f| f.is_current()).count(), 3);
    for pair in Pair::ALL {
        let c = pair_credential("person.synth", pair);
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyAmbisynth {
                credential: c.clone(),
                terminology: PairTerm::Ambisynth,
            },
        );
        apply(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::RecordFormMastery {
                mastery: c.forms[0].clone(),
            },
        );
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyAmbisynth {
                credential: c.clone(),
                terminology: PairTerm::Ambisynth,
            },
        );
        apply(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::RecordFormMastery {
                mastery: c.forms[1].clone(),
            },
        );
        assert!(g.ambisynth("person.synth", pair).is_none());
        certify_pair(&mut g, c.clone());
        assert_eq!(g.ambisynth("person.synth", pair), Some(&c));
        assert!(g.ambisynth("person.other", pair).is_none());
    }
    assert!(Form::ALL.iter().all(|f| g.has_form("person.synth", *f)));
    assert!(Office::ALL.iter().all(|o| g.incumbent(*o).is_none()));
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn pair_certification_rejects_wrong_subject_pair_forged_mastery_and_missing_switching() {
    let mut g = fresh();
    let c = pair_credential("person.synth", Pair::GargoyleMedusa);
    record_pair_forms(&mut g, &c);
    for invalid in ["subject", "pair", "mastery", "self", "duplicate"] {
        let mut bad = c.clone();
        match invalid {
            "subject" => bad.person = "person.other".into(),
            "pair" => bad.pair = Pair::MermanMermaid,
            "mastery" => bad.forms[0].certification.record = "mastery.forged".into(),
            "self" => bad.switching.signer = bad.person.clone(),
            "duplicate" => bad.switching.record = bad.forms[0].certification.record.clone(),
            _ => unreachable!(),
        }
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyAmbisynth {
                credential: bad,
                terminology: PairTerm::Ambisynth,
            },
        );
    }
    let mut cmd = command(
        &g,
        date(0, 0),
        "credential.reviewer",
        Action::CertifyAmbisynth {
            credential: c.clone(),
            terminology: PairTerm::Ambisynth,
        },
    );
    cmd.evidence.remove(&c.switching.record);
    let before = g.clone();
    assert!(g.execute(cmd).is_err());
    assert_eq!(g, before);
    certify_pair(&mut g, c.clone());
    reject(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::CertifyAmbisynth {
            credential: c,
            terminology: PairTerm::Ambisynth,
        },
    );
    assert!(g.ambisynth("person.synth", Pair::WerewolfVampire).is_none());
}
#[test]
fn one_bodily_expression_requires_execution_receipt_consent_and_pair_specific_switching() {
    let mut g = fresh();
    let c = pair_credential("person.synth", Pair::GargoyleMedusa);
    record_pair_forms(&mut g, &c);
    let start = expression(
        &g,
        "person.synth",
        BodilyForm::Sibling(Form::Gargoyle),
        "body.first",
    );
    reject(&mut g, date(0, 0), "being.flynt.tross", start.clone());
    reject(&mut g, date(0, 0), "person.synth", start.clone());
    let mut bad = start.clone();
    if let Action::RecordExpression { record } = &mut bad {
        record.consent.signer = "being.flynt.tross".into();
    }
    reject(&mut g, date(0, 0), "execution.reviewer", bad);
    apply(&mut g, date(0, 0), "execution.reviewer", start.clone());
    let switch = expression(
        &g,
        "person.synth",
        BodilyForm::Sibling(Form::Medusa),
        "body.switch",
    );
    reject(&mut g, date(0, 0), "execution.reviewer", switch.clone());
    certify_pair(&mut g, c);
    apply(&mut g, date(0, 0), "execution.reviewer", switch.clone());
    assert_eq!(
        g.expressed_form("person.synth"),
        Some(BodilyForm::Sibling(Form::Medusa))
    );
    reject(&mut g, date(0, 0), "execution.reviewer", switch);
    reject(&mut g, date(0, 0), "execution.reviewer", start);
    let wrong = expression(
        &g,
        "person.synth",
        BodilyForm::Sibling(Form::Vampire),
        "body.unearned",
    );
    reject(&mut g, date(0, 0), "execution.reviewer", wrong);
    assert!(g.has_form("person.synth", Form::Gargoyle));
    assert!(g.has_form("person.synth", Form::Medusa));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn earned_ambisynth_credential_can_support_command_qualification_without_auto_election() {
    let mut g = fresh();
    let c = pair_credential("person.nav", Pair::GargoyleMedusa);
    record_pair_forms(&mut g, &c);
    certify_pair(&mut g, c);
    qualify(&mut g, "person.nav", Office::Navigator);
    assert!(g.incumbent(Office::Navigator).is_none());
    record(&mut g, 0, election("e", Office::Navigator, "person.nav"));
    apply(&mut g, date(0, 5), "election.admin", seat("e"));
    apply(
        &mut g,
        date(1, 0),
        "person.nav",
        Action::ResignCommander {
            office: Office::Navigator,
            tenure: "e.tenure".into(),
        },
    );
    assert!(g.has_form("person.nav", Form::Gargoyle));
    assert!(g.ambisynth("person.nav", Pair::GargoyleMedusa).is_some());
    assert!(!g.has_form("person.nav", Form::Werewolf));
    assert_eq!(g.expressed_form("person.nav"), None);
}
#[test]
fn external_command_credential_is_not_an_automatic_body_or_execution_license() {
    let mut g = fresh();
    qualify(&mut g, "person.nav", Office::Navigator);
    assert!(!g.has_form("person.nav", Form::Gargoyle));
    let a = expression(
        &g,
        "person.nav",
        BodilyForm::Sibling(Form::Gargoyle),
        "body.unregistered",
    );
    reject(&mut g, date(0, 0), "execution.reviewer", a);
    let c = pair_credential("person.nav", Pair::GargoyleMedusa);
    record_pair_forms(&mut g, &c);
    certify_pair(&mut g, c);
    assert!(g.qualified("person.nav", Office::Navigator));
    let before = g.clone();
    let mut false_fact = pair_credential("person.other", Pair::GargoyleMedusa).forms[0].clone();
    false_fact.certification.record = pair_credential("person.nav", Pair::GargoyleMedusa).forms[0]
        .certification
        .record
        .clone();
    reject(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::RecordFormMastery {
            mastery: false_fact,
        },
    );
    assert_eq!(g, before);
}
#[test]
fn historical_bisynth_term_and_checkpoint_a_archive_remain_readable_without_rewrite() {
    let bytes = include_bytes!("fixtures/flynt-governance-a-v1.json");
    let a = Governance::decode(bytes).unwrap();
    assert_eq!(a.encode().unwrap(), bytes);
    assert_eq!(a.incumbent(Office::Navigator).unwrap().holder, "a");
    let mut g = fresh();
    let c = pair_credential("person.synth", Pair::MermanMermaid);
    record_pair_forms(&mut g, &c);
    apply(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::CertifyAmbisynth {
            credential: c,
            terminology: PairTerm::Bisynth,
        },
    );
    assert_eq!(PairTerm::Bisynth.active_name(), "Ambisynth");
    let bytes = g.encode().unwrap();
    assert!(
        String::from_utf8(bytes.clone())
            .unwrap()
            .contains("bisynth")
    );
    assert_eq!(Governance::decode(&bytes).unwrap().encode().unwrap(), bytes);
}

use hollow_grove::world::flynt::synthesis::{
    Convergence, ConvergenceInterpretation, ConvergenceVerification, TRIPLE_CROWN_COMPONENTS,
};
fn adopt_convergence(g: &mut Governance) {
    apply(
        g,
        g.date(),
        "law.authorizer",
        Action::AdoptConvergenceInterpretation {
            interpretation: ConvergenceInterpretation::RevolutionC99B,
            authorization: proof("law.revc-convergence", "law.authorizer"),
            legacy_conflict_review: proof("review.legacy-convergence", "court.reviewer"),
        },
    );
}
fn convergence_verification(person: &str, outcome: Convergence) -> ConvergenceVerification {
    let id = format!("{person}.{outcome:?}").to_ascii_lowercase();
    ConvergenceVerification {
        person: person.into(),
        outcome,
        interpretation: ConvergenceInterpretation::RevolutionC99B,
        recipe_id: outcome.recipe_id().into(),
        recipe_revision: "v1.test".into(),
        execution: proof(&format!("{id}.recipe-execution"), "execution.reviewer"),
        competence: proof(&format!("{id}.competence"), "credential.reviewer"),
    }
}
fn record_currents(g: &mut Governance, person: &str) {
    for pair in Pair::ALL {
        let c = pair_credential(person, pair);
        apply(
            g,
            g.date(),
            "credential.reviewer",
            Action::RecordFormMastery {
                mastery: c.forms[0].clone(),
            },
        );
    }
}
fn certify_convergence(g: &mut Governance, person: &str, outcome: Convergence) {
    apply(
        g,
        g.date(),
        "credential.reviewer",
        Action::CertifyConvergence {
            verification: convergence_verification(person, outcome),
        },
    );
}
#[test]
fn chimera_requires_three_current_forms_and_verified_trifecta_but_no_aura_pairs() {
    let mut g = fresh();
    adopt_convergence(&mut g);
    reject(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::CertifyConvergence {
            verification: convergence_verification("person.synth", Convergence::Chimera),
        },
    );
    record_currents(&mut g, "person.synth");
    assert!(
        g.convergence("person.synth", Convergence::Chimera)
            .is_none()
    );
    certify_convergence(&mut g, "person.synth", Convergence::Chimera);
    assert!(g.has_bodily_mastery("person.synth", BodilyForm::Chimera));
    assert!(
        Pair::ALL
            .iter()
            .all(|p| g.ambisynth("person.synth", *p).is_none())
    );
    assert_eq!(Convergence::Chimera.recipe_name(), "The Trifecta Ritual");
    assert_eq!(
        Convergence::Chimera.names(),
        &["Trisynthesis", "The Trifecta"]
    );
    assert_eq!(
        TRIPLE_CROWN_COMPONENTS,
        [
            (Form::Gargoyle, "Bolo"),
            (Form::Werewolf, "Crossbow"),
            (Form::Merman, "Harpoon")
        ]
    );
    assert!(!g.has_bodily_mastery("person.synth", BodilyForm::Manticorp));
    assert!(!g.must_enter_q("person.synth"));
    let body = expression(&g, "person.synth", BodilyForm::Chimera, "body.chimera");
    apply(&mut g, date(0, 0), "execution.reviewer", body);
    assert_eq!(g.tross(), Some("being.flynt.tross"));
}
#[test]
fn manticorp_requires_six_forms_three_pairs_and_own_recipe_without_extra_chimera_gate() {
    let mut g = fresh();
    adopt_convergence(&mut g);
    for pair in Pair::ALL {
        let c = pair_credential("person.synth", pair);
        record_pair_forms(&mut g, &c);
    }
    let action = Action::CertifyConvergence {
        verification: convergence_verification("person.synth", Convergence::Manticorp),
    };
    reject(&mut g, date(0, 0), "credential.reviewer", action.clone());
    for (i, pair) in Pair::ALL.into_iter().enumerate() {
        certify_pair(&mut g, pair_credential("person.synth", pair));
        if i < 2 {
            reject(&mut g, date(0, 0), "credential.reviewer", action.clone());
        }
    }
    assert!(
        g.convergence("person.synth", Convergence::Manticorp)
            .is_none()
    );
    apply(&mut g, date(0, 0), "credential.reviewer", action);
    assert!(
        g.convergence("person.synth", Convergence::Chimera)
            .is_none()
    );
    assert!(g.must_enter_q("person.synth"));
    let body = expression(&g, "person.synth", BodilyForm::Manticorp, "body.manticorp");
    apply(&mut g, date(0, 0), "execution.reviewer", body);
    assert_eq!(
        g.expressed_form("person.synth"),
        Some(BodilyForm::Manticorp)
    );
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert!(Office::ALL.iter().all(|o| g.incumbent(*o).is_none()));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn aura_only_manticorp_and_unmastered_current_convergence_fail_without_mutation() {
    let mut g = fresh();
    adopt_convergence(&mut g);
    for pair in Pair::ALL {
        let c = pair_credential("person.aura", pair);
        apply(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::RecordFormMastery {
                mastery: c.forms[1].clone(),
            },
        );
    }
    for outcome in [Convergence::Chimera, Convergence::Manticorp] {
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyConvergence {
                verification: convergence_verification("person.aura", outcome),
            },
        );
    }
}
#[test]
fn convergence_requires_explicit_version_and_legacy_conflict_review() {
    let mut g = fresh();
    record_currents(&mut g, "person.synth");
    assert_eq!(
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyConvergence {
                verification: convergence_verification("person.synth", Convergence::Chimera)
            }
        ),
        Error::UnresolvedLaw(UnresolvedLaw::ConvergenceInterpretation)
    );
    for interpretation in [
        ConvergenceInterpretation::LegacyCombinedThenPerfected,
        ConvergenceInterpretation::EnactedChimeraPrerequisite,
    ] {
        assert_eq!(
            reject(
                &mut g,
                date(0, 0),
                "law.authorizer",
                Action::AdoptConvergenceInterpretation {
                    interpretation,
                    authorization: proof("law.new", "law.authorizer"),
                    legacy_conflict_review: proof("review.legacy", "court.reviewer")
                }
            ),
            Error::UnresolvedLaw(UnresolvedLaw::LegacyConvergenceConflict)
        );
    }
    reject(
        &mut g,
        date(0, 0),
        "being.flynt.tross",
        Action::AdoptConvergenceInterpretation {
            interpretation: ConvergenceInterpretation::RevolutionC99B,
            authorization: proof("law.forged", "being.flynt.tross"),
            legacy_conflict_review: proof("review.forged", "being.flynt.tross"),
        },
    );
    adopt_convergence(&mut g);
    let mut legacy = convergence_verification("person.synth", Convergence::Chimera);
    legacy.interpretation = ConvergenceInterpretation::LegacyCombinedThenPerfected;
    assert_eq!(
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyConvergence {
                verification: legacy
            }
        ),
        Error::UnresolvedLaw(UnresolvedLaw::LegacyConvergenceConflict)
    );
    assert_ne!(
        Convergence::Chimera.recipe_id(),
        flynt_constitution::RECIPE_CHIMERA
    );
    assert_ne!(
        Convergence::Manticorp.recipe_id(),
        flynt_constitution::RECIPE_MANTICORE
    );
    assert_eq!(
        flynt_constitution::RECIPE_MANTICORE,
        "flynt.recipe.divided-manticorp"
    );
}
#[test]
fn convergence_rejects_wrong_recipe_forged_or_missing_execution_and_self_certification() {
    let mut g = fresh();
    adopt_convergence(&mut g);
    record_currents(&mut g, "person.synth");
    for invalid in [
        "recipe",
        "execution",
        "self",
        "same-proof",
        "subject",
        "reviewer",
    ] {
        let mut v = convergence_verification("person.synth", Convergence::Chimera);
        match invalid {
            "recipe" => v.recipe_id = Convergence::Manticorp.recipe_id().into(),
            "execution" => v.execution.signer = "being.flynt.tross".into(),
            "self" => v.competence.signer = v.person.clone(),
            "same-proof" => v.competence.record = v.execution.record.clone(),
            "subject" => v.person = "person.unqualified".into(),
            "reviewer" => v.execution.signer = v.competence.signer.clone(),
            _ => unreachable!(),
        }
        reject(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::CertifyConvergence { verification: v },
        );
    }
    let v = convergence_verification("person.synth", Convergence::Chimera);
    let mut cmd = command(
        &g,
        date(0, 0),
        "credential.reviewer",
        Action::CertifyConvergence {
            verification: v.clone(),
        },
    );
    cmd.evidence.remove(&v.execution.record);
    let before = g.clone();
    assert!(g.execute(cmd).is_err());
    assert_eq!(g, before);
    certify_convergence(&mut g, "person.synth", Convergence::Chimera);
    reject(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::CertifyConvergence { verification: v },
    );
}
#[test]
fn two_convergence_masteries_never_create_two_bodies_or_transfer_the_tross() {
    let mut g = fresh();
    adopt_convergence(&mut g);
    for pair in Pair::ALL {
        let c = pair_credential("person.synth", pair);
        record_pair_forms(&mut g, &c);
        certify_pair(&mut g, c);
    }
    certify_convergence(&mut g, "person.synth", Convergence::Chimera);
    certify_convergence(&mut g, "person.synth", Convergence::Manticorp);
    for (form, id) in [
        (BodilyForm::Chimera, "body.one"),
        (BodilyForm::Manticorp, "body.two"),
    ] {
        let a = expression(&g, "person.synth", form, id);
        apply(&mut g, date(0, 0), "execution.reviewer", a);
        assert_eq!(g.expressed_form("person.synth"), Some(form));
    }
    assert!(g.has_bodily_mastery("person.synth", BodilyForm::Chimera));
    assert!(g.has_bodily_mastery("person.synth", BodilyForm::Manticorp));
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert_eq!(
        g.command_catalog()
            .unwrap()
            .office_holders
            .iter()
            .filter(|h| h.active)
            .count(),
        1
    );
    let bytes = g.encode().unwrap();
    assert_eq!(Governance::decode(&bytes).unwrap().encode().unwrap(), bytes);
}
