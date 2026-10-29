//! Simulated legal mandates, calendar and evidence; never production defaults.
use hollow_grove::constitutional::InterfaceApplyOutcome;
use hollow_grove::hollow_deck::GlaushouseSeat;
use hollow_grove::institution::{InstitutionalBeingId, OfficeHolder};
use hollow_grove::institution_affiliation::ZoneEntryResult;
use hollow_grove::world::glaushouse::governance::*;
use hollow_grove::world::{chroma_cord, glaushouse, house_scene_context, session::WorldSession};
use std::collections::{BTreeMap, BTreeSet};

fn date(year: u64, day: u64) -> Date {
    Date { year, day }
}
fn proof(id: &str, signer: &str) -> Attestation {
    Attestation {
        record: id.into(),
        signer: signer.into(),
    }
}
fn policy() -> Policy {
    Policy {
        authorization: proof("law.policy", "court.authorizer"),
        administrator: "election.admin".into(),
        credential_reviewers: BTreeSet::from(["credential.reviewer".into()]),
        court_reviewers: BTreeSet::from(["court.reviewer".into()]),
        clearance_reviewers: BTreeSet::from(["clinical.reviewer".into()]),
        days_per_year: Some(360),
        first_election_years: MuseHouse::ALL
            .into_iter()
            .enumerate()
            .map(|(i, h)| (h, (i / 3 * 2) as u64))
            .collect(),
        special_elections_fill_remainder: None,
        corination_vacates_own_seat: None,
    }
}
fn fresh() -> Governance {
    Governance::new(&WorldSession::canonical(), policy(), date(0, 0)).unwrap()
}
fn command(g: &Governance, at: Date, actor: &str, action: Action) -> Command {
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
    if let Action::VetoElection {
        constitutional_ground,
        ..
    } = &action
    {
        evidence.insert(constitutional_ground.clone());
    }
    Command {
        id: format!("event.{}", g.revision()),
        expected_revision: g.revision(),
        at,
        actor: actor.into(),
        evidence,
        action,
    }
}
fn apply(g: &mut Governance, at: Date, actor: &str, action: Action) {
    assert_eq!(
        g.execute(command(g, at, actor, action)),
        Ok(InterfaceApplyOutcome::Applied)
    );
}
fn now(g: &mut Governance, actor: &str, action: Action) {
    apply(g, g.date(), actor, action);
}
fn reject(g: &mut Governance, at: Date, actor: &str, action: Action) -> Error {
    let before = g.clone();
    let err = g
        .execute(command(g, at, actor, action))
        .expect_err("must reject");
    assert_eq!(*g, before);
    err
}
fn qualify(g: &mut Governance, person: &str, office: Office) {
    now(
        g,
        "credential.reviewer",
        Action::AdmitNightingale {
            person: person.into(),
        },
    );
    now(
        g,
        "credential.reviewer",
        Action::RecognizeMastery {
            person: person.into(),
            form: "muse".into(),
        },
    );
    if let Office::Persephone(h) = office {
        now(
            g,
            "credential.reviewer",
            Action::RecordAgency {
                person: person.into(),
                house: h,
            },
        );
        now(
            g,
            "credential.reviewer",
            Action::RecognizeMastery {
                person: person.into(),
                form: "persephone".into(),
            },
        );
    }
    now(
        g,
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
        .map(|(i, c)| Vote {
            voter: format!("peer.{i}"),
            candidate: (*c).into(),
            receipt: proof(&format!("{id}.vote.{i}"), &format!("peer.{i}")),
        })
        .collect()
}
fn roll(id: &str, h: MuseHouse) -> ElectoralRoll {
    ElectoralRoll {
        house: h,
        eligible: (0..10).map(|i| format!("peer.{i}")).collect(),
        certification: proof(&format!("{id}.roll"), "election.admin"),
    }
}
fn election(id: &str, h: MuseHouse, person: &str, year: u64) -> Election {
    Election {
        id: id.into(),
        house: h,
        candidates: BTreeSet::from([person.into()]),
        roll: roll(id, h),
        rounds: vec![votes(id, &[person; 6])],
        winner: person.into(),
        tenure: format!("{id}.tenure"),
        begins: date(year, 14),
        expires: date(year + 6, 14),
        special: false,
        certification: proof(&format!("{id}.certificate"), "election.admin"),
    }
}
fn college_with(p: Policy) -> Governance {
    let mut g = Governance::new(&WorldSession::canonical(), p, date(0, 0)).unwrap();
    for group in 0..3 {
        let year = group * 2;
        apply(&mut g, date(year, 0), "election.admin", Action::ObserveDate);
        for h in &MuseHouse::ALL[group as usize * 3..group as usize * 3 + 3] {
            let person = format!("being.{}", h.slug());
            let id = format!("election.{}", h.slug());
            qualify(&mut g, &person, Office::Persephone(*h));
            now(
                &mut g,
                "election.admin",
                Action::ProposeElection {
                    election: election(&id, *h, &person, year),
                },
            );
        }
        for h in &MuseHouse::ALL[group as usize * 3..group as usize * 3 + 3] {
            apply(
                &mut g,
                date(year, 14),
                "election.admin",
                Action::CertifyElection {
                    election: format!("election.{}", h.slug()),
                },
            );
        }
    }
    g
}
fn college() -> Governance {
    college_with(policy())
}
fn ready(g: &mut Governance, id: &str, candidate: &str) {
    qualify(g, candidate, Office::PrimaDonna);
    now(
        g,
        "election.admin",
        Action::OpenCall {
            id: id.into(),
            court_case: format!("case.{id}"),
            candidates: BTreeSet::from([candidate.into()]),
            screening: proof(&format!("{id}.screen"), "court.reviewer"),
        },
    );
    now(
        g,
        "election.admin",
        Action::Audition {
            id: id.into(),
            reviews: BTreeMap::from([(
                candidate.into(),
                Audition {
                    evaluation: proof(&format!("{id}.audition"), "credential.reviewer"),
                    non_patient_exposing: true,
                },
            )]),
        },
    );
    now(
        g,
        "election.admin",
        Action::Testimony {
            id: id.into(),
            reviews: BTreeMap::from([(
                candidate.into(),
                Testimony {
                    service_record: proof(&format!("{id}.service"), "credential.reviewer"),
                    witness_record: proof(&format!("{id}.witness"), "being.witness"),
                    privacy_review: proof(&format!("{id}.privacy"), "court.reviewer"),
                },
            )]),
        },
    );
}
fn seats(g: &Governance, id: &str, n: usize, choice: &str) -> Vec<SeatVote> {
    MuseHouse::ALL
        .into_iter()
        .filter_map(|h| {
            g.incumbent(Office::Persephone(h)).map(|t| SeatVote {
                house: h,
                tenure: t.id.clone(),
                choice: Some(choice.into()),
                receipt: proof(&format!("{id}.{}", h.slug()), &t.holder),
            })
        })
        .take(n)
        .collect()
}
fn crown(id: &str) -> Action {
    Action::Corinate {
        id: id.into(),
        tenure: format!("tenure.{id}"),
        certification: proof(&format!("{id}.certificate"), "court.reviewer"),
        appeals_resolved: proof(&format!("{id}.appeals"), "court.reviewer"),
    }
}
fn sovereign() -> Governance {
    sovereign_with(policy())
}
fn sovereign_with(p: Policy) -> Governance {
    let mut g = college_with(p);
    ready(&mut g, "call.one", "being.leader");
    let ballots = seats(&g, "ovation.one", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    now(&mut g, "election.admin", crown("call.one"));
    g
}
fn manifest(g: &mut Governance, person: &str, form: &str) {
    now(
        g,
        person,
        Action::Manifest {
            form: form.into(),
            ordinary_reversion: "muse".into(),
            clearance: proof(&format!("clearance.{person}"), "clinical.reviewer"),
            consent: proof(&format!("consent.{person}"), person),
        },
    );
}

#[test]
fn nine_distinct_institutions_and_equal_seats_preserve_legacy_decoding() {
    let world = WorldSession::canonical();
    let catalog = &world.institutional().catalog;
    assert_eq!(
        MuseHouse::ALL.map(MuseHouse::name),
        [
            "Calli", "Cleo", "Melanie", "Sherry", "Talia", "Terri", "Erin", "Polly", "Nia"
        ]
    );
    assert_eq!(
        MuseHouse::ALL
            .map(MuseHouse::institution_id)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .len(),
        9
    );
    assert_eq!(
        MuseHouse::ALL
            .map(MuseHouse::office_id)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .len(),
        9
    );
    assert!(catalog.institution(&house_of_persephone_id()).is_some());
    for h in MuseHouse::ALL {
        assert!(catalog.institution(&h.institution_id()).is_some());
        let o = catalog
            .offices
            .iter()
            .find(|o| o.id == h.office_id())
            .unwrap();
        assert!(o.singular);
        assert_eq!(o.authority, vec!["MuseHouseCoordination", "CallElector"]);
    }
    for (wire, seat) in [
        ("calli", GlaushouseSeat::Calli),
        ("cleo", GlaushouseSeat::Cleo),
        ("melony", GlaushouseSeat::Melony),
        ("teri", GlaushouseSeat::Teri),
        ("talia", GlaushouseSeat::Talia),
    ] {
        assert_eq!(
            serde_json::from_str::<GlaushouseSeat>(&format!("\"{wire}\"")).unwrap(),
            seat
        );
        assert_eq!(serde_json::to_string(&seat).unwrap(), format!("\"{wire}\""));
    }
    assert!(serde_json::from_str::<MuseHouse>("\"teri\"").is_err());
    assert_eq!(
        ORDINARY_FORMS,
        [
            "pixie", "sprite", "fairie", "spirit", "siren", "nymph", "sylph", "muse"
        ]
    );
    assert_eq!(
        CARE_ROLES,
        [
            ("Matron", "Presensor", "Emerald"),
            ("Marshall", "Preceptor", "Jade")
        ]
    );
}
#[test]
fn admission_mastery_agency_and_office_are_independent() {
    let mut g = fresh();
    now(
        &mut g,
        "credential.reviewer",
        Action::RecognizeMastery {
            person: "being.guest".into(),
            form: "muse".into(),
        },
    );
    assert!(!g.is_nightingale("being.guest"));
    assert!(!g.may_manifest("being.guest", "persephone"));
    reject(
        &mut g,
        date(0, 0),
        "credential.reviewer",
        Action::Qualify {
            person: "being.guest".into(),
            office: Office::PrimaDonna,
        },
    );
    qualify(&mut g, "being.master", Office::Persephone(MuseHouse::Calli));
    assert!(g.has_mastery("being.master", "persephone"));
    assert!(!g.may_manifest("being.master", "persephone"));
    now(
        &mut g,
        "credential.reviewer",
        Action::RecordAgency {
            person: "being.master".into(),
            house: MuseHouse::Cleo,
        },
    );
    assert!(g.has_mastery("being.master", "muse"));
    assert!(g.has_mastery("being.master", "persephone"));
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        Action::ProposeElection {
            election: election("wrong-agency", MuseHouse::Calli, "being.master", 0),
        },
    );
    assert!(
        g.world_session()
            .unwrap()
            .institutional()
            .memberships
            .is_empty()
    );
}
#[test]
fn elections_require_unique_peers_majority_and_runoff_without_tie_guessing() {
    let mut g = fresh();
    for p in ["being.a", "being.b", "being.c"] {
        qualify(&mut g, p, Office::Persephone(MuseHouse::Calli));
    }
    let mut e = election("runoff", MuseHouse::Calli, "being.a", 0);
    e.candidates = ["being.a", "being.b", "being.c"].map(String::from).into();
    e.rounds = vec![
        votes(
            "first",
            &["being.a", "being.a", "being.b", "being.b", "being.c"],
        ),
        votes("second", &["being.a", "being.a", "being.b"]),
    ];
    let mut bad = e.clone();
    bad.rounds[0] = votes("cutoff", &["being.a", "being.a", "being.b", "being.c"]);
    assert!(matches!(
        reject(
            &mut g,
            date(0, 0),
            "election.admin",
            Action::ProposeElection { election: bad }
        ),
        Error::RequiresLegalParameter(_)
    ));
    let mut bad = e.clone();
    bad.rounds[1][0].voter = "outside".into();
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        Action::ProposeElection { election: bad },
    );
    let mut bad = e.clone();
    let repeated = bad.rounds[1][0].clone();
    bad.rounds[1].push(repeated);
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        Action::ProposeElection { election: bad },
    );
    now(
        &mut g,
        "election.admin",
        Action::ProposeElection { election: e },
    );
    assert!(g.incumbent(Office::Persephone(MuseHouse::Calli)).is_none());
    reject(
        &mut g,
        date(0, 13),
        "election.admin",
        Action::CertifyElection {
            election: "runoff".into(),
        },
    );
    apply(
        &mut g,
        date(0, 14),
        "election.admin",
        Action::CertifyElection {
            election: "runoff".into(),
        },
    );
    assert_eq!(
        g.incumbent(Office::Persephone(MuseHouse::Calli))
            .unwrap()
            .expires,
        date(6, 14)
    );
    reject(
        &mut g,
        date(0, 14),
        "credential.reviewer",
        Action::RecordAgency {
            person: "being.a".into(),
            house: MuseHouse::Cleo,
        },
    );
}
#[test]
fn missing_calendars_cohorts_and_special_term_law_never_invent_authority() {
    for missing_calendar in [true, false] {
        let mut p = policy();
        if missing_calendar {
            p.days_per_year = None;
        } else {
            p.first_election_years.clear();
        }
        let mut g = Governance::new(&WorldSession::canonical(), p, date(0, 0)).unwrap();
        qualify(&mut g, "being.a", Office::Persephone(MuseHouse::Calli));
        assert!(matches!(
            reject(
                &mut g,
                date(0, 0),
                "election.admin",
                Action::ProposeElection {
                    election: election("missing-law", MuseHouse::Calli, "being.a", 0)
                }
            ),
            Error::RequiresLegalParameter(_)
        ));
    }
    let mut bad = policy();
    bad.first_election_years.insert(MuseHouse::Calli, 1);
    assert!(Governance::new(&WorldSession::canonical(), bad, date(0, 0)).is_err());
    let mut g = college();
    now(
        &mut g,
        "being.calli",
        Action::Resign {
            office: Office::Persephone(MuseHouse::Calli),
            tenure: "election.calli.tenure".into(),
        },
    );
    let mut e = election("special", MuseHouse::Calli, "being.calli", 4);
    e.special = true;
    e.begins = date(4, 28);
    e.expires = date(6, 14);
    assert!(matches!(
        reject(
            &mut g,
            date(4, 14),
            "election.admin",
            Action::ProposeElection { election: e }
        ),
        Error::RequiresLegalParameter(_)
    ));
}
#[test]
fn special_election_remainder_and_history_survive_projection_and_replay() {
    let mut p = policy();
    p.special_elections_fill_remainder = Some(proof("law.remainder", "court.reviewer"));
    let mut g = college_with(p);
    manifest(&mut g, "being.calli", "persephone");
    now(
        &mut g,
        "being.calli",
        Action::Resign {
            office: Office::Persephone(MuseHouse::Calli),
            tenure: "election.calli.tenure".into(),
        },
    );
    assert_eq!(g.expressed_form("being.calli"), Some("muse"));
    assert!(g.has_mastery("being.calli", "persephone"));
    let mut e = election("special", MuseHouse::Calli, "being.calli", 4);
    e.special = true;
    e.begins = date(4, 28);
    e.expires = date(6, 14);
    let mut bad = e.clone();
    bad.expires = date(10, 28);
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::ProposeElection { election: bad },
    );
    now(
        &mut g,
        "election.admin",
        Action::ProposeElection { election: e },
    );
    apply(
        &mut g,
        date(4, 28),
        "election.admin",
        Action::CertifyElection {
            election: "special".into(),
        },
    );
    let world = g.world_session().unwrap();
    let rows = world
        .institutional()
        .catalog
        .office_holders
        .iter()
        .filter(|h| h.office == MuseHouse::Calli.office_id())
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows.iter().filter(|h| h.active).count(), 1);
    assert_eq!(
        g.incumbent(Office::Persephone(MuseHouse::Calli))
            .unwrap()
            .expires,
        date(6, 14)
    );
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn call_requires_exact_stages_and_non_patient_audition_with_private_testimony() {
    let mut g = college();
    qualify(&mut g, "being.leader", Office::PrimaDonna);
    now(
        &mut g,
        "election.admin",
        Action::OpenCall {
            id: "call.one".into(),
            court_case: "case.call".into(),
            candidates: BTreeSet::from(["being.leader".into()]),
            screening: proof("screen", "court.reviewer"),
        },
    );
    let ballots = seats(&g, "early", 6, "being.leader");
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    let mut a = Audition {
        evaluation: proof("audition", "credential.reviewer"),
        non_patient_exposing: false,
    };
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::Audition {
            id: "call.one".into(),
            reviews: BTreeMap::from([("being.leader".into(), a.clone())]),
        },
    );
    a.non_patient_exposing = true;
    now(
        &mut g,
        "election.admin",
        Action::Audition {
            id: "call.one".into(),
            reviews: BTreeMap::from([("being.leader".into(), a)]),
        },
    );
    let t = Testimony {
        service_record: proof("service", "credential.reviewer"),
        witness_record: proof("witness", "being.witness"),
        privacy_review: proof("privacy", "being.leader"),
    };
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::Testimony {
            id: "call.one".into(),
            reviews: BTreeMap::from([("being.leader".into(), t)]),
        },
    );
    assert_eq!(
        CallStage::ALL.map(CallStage::name),
        [
            "The Pitch",
            "Audition",
            "Testimony",
            "Ovation",
            "Corination"
        ]
    );
}
#[test]
fn five_votes_fail_even_with_vacancies_and_fresh_six_elect_non_incumbent_muse() {
    let mut g = college();
    ready(&mut g, "call.one", "being.leader");
    for h in [MuseHouse::Erin, MuseHouse::Polly, MuseHouse::Nia] {
        now(
            &mut g,
            &format!("being.{}", h.slug()),
            Action::Resign {
                office: Office::Persephone(h),
                tenure: format!("election.{}.tenure", h.slug()),
            },
        );
    }
    let ballots = seats(&g, "five", 5, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots: ballots.clone(),
        },
    );
    assert!(g.calls()["call.one"].selected.is_none());
    reject(&mut g, date(4, 14), "election.admin", crown("call.one"));
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    let ballots = seats(&g, "six", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    now(&mut g, "election.admin", crown("call.one"));
    assert_eq!(
        g.incumbent(Office::PrimaDonna).unwrap().holder,
        "being.leader"
    );
    assert_eq!(
        g.incumbent(Office::PrimaDonna).unwrap().expires,
        date(13, 14)
    );
    assert!(!g.has_mastery("being.leader", "persephone"));
    assert!(g.may_manifest("being.leader", "prima-donna"));
}
#[test]
fn stale_elector_tenures_and_duplicate_seats_cannot_corinate() {
    let mut g = college();
    ready(&mut g, "call.one", "being.leader");
    let mut duplicate = seats(&g, "duplicate", 6, "being.leader");
    duplicate.push(duplicate[0].clone());
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots: duplicate,
        },
    );
    let ballots = seats(&g, "six", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    now(
        &mut g,
        "being.calli",
        Action::Resign {
            office: Office::Persephone(MuseHouse::Calli),
            tenure: "election.calli.tenure".into(),
        },
    );
    reject(&mut g, date(4, 14), "election.admin", crown("call.one"));
    let ballots = seats(&g, "retry", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    now(&mut g, "election.admin", crown("call.one"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn elected_candidate_cannot_review_own_call_and_needs_explicit_seat_disposition() {
    for authorized in [false, true] {
        let mut p = policy();
        p.court_reviewers.insert("being.calli".into());
        if authorized {
            p.corination_vacates_own_seat = Some(proof("law.seat-disposition", "court.reviewer"));
        }
        let mut g = college_with(p);
        ready(&mut g, "call.one", "being.calli");
        let ballots = seats(&g, "six", 6, "being.calli");
        now(
            &mut g,
            "election.admin",
            Action::Ovation {
                id: "call.one".into(),
                ballots,
            },
        );
        let mut bad = crown("call.one");
        if let Action::Corinate { certification, .. } = &mut bad {
            certification.signer = "being.calli".into();
        }
        reject(&mut g, date(4, 14), "election.admin", bad);
        if authorized {
            manifest(&mut g, "being.calli", "persephone");
            now(&mut g, "election.admin", crown("call.one"));
            assert!(g.incumbent(Office::Persephone(MuseHouse::Calli)).is_none());
            assert_eq!(g.expressed_form("being.calli"), Some("muse"));
        } else {
            assert!(matches!(
                reject(&mut g, date(4, 14), "election.admin", crown("call.one")),
                Error::RequiresLegalParameter(_)
            ));
        }
    }
}
#[test]
fn term_expiry_revokes_form_without_renewal_and_preserves_mastery() {
    let mut g = sovereign();
    manifest(&mut g, "being.leader", "prima-donna");
    apply(&mut g, date(13, 14), "election.admin", Action::ObserveDate);
    assert!(g.incumbent(Office::PrimaDonna).is_none());
    assert_eq!(g.expressed_form("being.leader"), Some("muse"));
    assert!(g.has_mastery("being.leader", "muse"));
    reject(&mut g, date(13, 14), "election.admin", crown("call.one"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}
#[test]
fn sovereign_office_and_civic_admission_grant_no_private_chart_or_recovery_access() {
    let g = sovereign();
    let world = g.world_session().unwrap();
    let who = InstitutionalBeingId::new("being.leader").unwrap();
    for policy in [
        chroma_cord::chroma_cord_access_policy(),
        house_scene_context::glaushouse_clinical_access_policy(&chroma_cord::chroma_cord_zone_id()),
    ] {
        assert!(matches!(
            world.institutional().request_zone_entry(
                &who,
                &glaushouse::chromacord_id(),
                &chroma_cord::chroma_cord_site_id(),
                &chroma_cord::chroma_cord_zone_id(),
                &policy
            ),
            ZoneEntryResult::Denied(_)
        ));
    }
    assert!(world.institutional().memberships.is_empty());
    assert!(world.institutional().access_grants.is_empty());
}
#[test]
fn archive_replay_retries_conflicts_tampering_and_ordering_are_checked() {
    let mut g = sovereign();
    let c = command(&g, date(4, 15), "election.admin", Action::ObserveDate);
    g.execute(c.clone()).unwrap();
    let before = g.clone();
    assert_eq!(g.execute(c.clone()), Ok(InterfaceApplyOutcome::Idempotent));
    let mut bad = c;
    bad.at.day = 16;
    assert!(g.execute(bad).is_err());
    assert_eq!(g, before);
    let mut stale = command(&g, date(4, 16), "election.admin", Action::ObserveDate);
    stale.expected_revision -= 1;
    assert!(g.execute(stale).is_err());
    assert_eq!(g, before);
    let bytes = g.encode().unwrap();
    assert_eq!(Governance::decode(&bytes).unwrap().encode().unwrap(), bytes);
    let forged = String::from_utf8(bytes.clone())
        .unwrap()
        .replace("being.leader", "being.forged");
    assert!(Governance::decode(forged.as_bytes()).is_err());
    let mut wire: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    wire["events"].as_array_mut().unwrap().reverse();
    assert_eq!(
        Governance::decode(&serde_json::to_vec(&wire).unwrap()).unwrap(),
        g
    );
}
#[test]
fn source_history_is_preserved_and_unevidenced_legacy_terms_are_refused() {
    let mut state = WorldSession::canonical().institutional().clone();
    let historical = OfficeHolder {
        office: MuseHouse::Terri.office_id(),
        being: InstitutionalBeingId::new("being.historical").unwrap(),
        active: false,
    };
    state.catalog.office_holders.push(historical.clone());
    let source = WorldSession::from_institutional_state(state.clone()).unwrap();
    let bytes = source.persisted_state_output();
    let g = Governance::new(&source, policy(), date(0, 0)).unwrap();
    assert_eq!(g.world_session().unwrap().persisted_state_output(), bytes);
    assert!(!g.has_mastery("being.historical", "persephone"));
    state.catalog.office_holders.push(OfficeHolder {
        office: Office::PrimaDonna.id(),
        being: InstitutionalBeingId::new("being.legacy-sovereign").unwrap(),
        active: true,
    });
    assert!(matches!(
        Governance::new(
            &WorldSession::from_institutional_state(state).unwrap(),
            policy(),
            date(0, 0)
        ),
        Err(Error::RequiresLegalParameter(_))
    ));
    assert_eq!(source.persisted_state_output(), bytes);
}

#[test]
fn bounded_veto_crosses_calendar_year_and_independent_appeal_preserves_tenures() {
    let mut p = policy();
    p.court_reviewers.insert("being.leader".into());
    let mut g = sovereign_with(p);
    let e = election("reelection", MuseHouse::Calli, "being.calli", 6);
    apply(
        &mut g,
        date(5, 350),
        "election.admin",
        Action::ProposeElection { election: e },
    );
    let veto = Action::VetoElection {
        election: "reelection".into(),
        constitutional_ground: "ground.constitution".into(),
    };
    assert_eq!(
        reject(&mut g, date(6, 4), "being.leader", veto.clone()),
        Error::Invalid("fourteen-day veto window closed")
    );
    apply(&mut g, date(6, 3), "being.leader", veto);
    reject(
        &mut g,
        date(6, 14),
        "election.admin",
        Action::CertifyElection {
            election: "reelection".into(),
        },
    );
    // Resigning does not erase the vetoing sovereign's appeal conflict.
    now(
        &mut g,
        "being.leader",
        Action::Resign {
            office: Office::PrimaDonna,
            tenure: "tenure.call.one".into(),
        },
    );
    reject(
        &mut g,
        date(6, 3),
        "being.leader",
        Action::ReviewVeto {
            election: "reelection".into(),
            court_case: "case.veto".into(),
            uphold: false,
        },
    );
    now(
        &mut g,
        "court.reviewer",
        Action::ReviewVeto {
            election: "reelection".into(),
            court_case: "case.veto".into(),
            uphold: false,
        },
    );
    apply(
        &mut g,
        date(6, 14),
        "election.admin",
        Action::CertifyElection {
            election: "reelection".into(),
        },
    );
    let old = g
        .tenures()
        .iter()
        .find(|t| t.id == "election.calli.tenure")
        .unwrap();
    assert_eq!(old.ended, Some(date(6, 14)));
    assert_eq!(old.ending.as_deref(), Some("term-expired"));
    assert_eq!(
        g.incumbent(Office::Persephone(MuseHouse::Calli))
            .unwrap()
            .id,
        "reelection.tenure"
    );
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn upheld_veto_does_not_install_substitute_and_expired_pending_result_does_not_block() {
    let mut g = sovereign();
    apply(
        &mut g,
        date(5, 350),
        "election.admin",
        Action::ProposeElection {
            election: election("vetoed", MuseHouse::Calli, "being.calli", 6),
        },
    );
    now(
        &mut g,
        "being.leader",
        Action::VetoElection {
            election: "vetoed".into(),
            constitutional_ground: "ground.constitution".into(),
        },
    );
    now(
        &mut g,
        "court.reviewer",
        Action::ReviewVeto {
            election: "vetoed".into(),
            court_case: "case.veto".into(),
            uphold: true,
        },
    );
    reject(
        &mut g,
        date(6, 14),
        "election.admin",
        Action::CertifyElection {
            election: "vetoed".into(),
        },
    );
    now(
        &mut g,
        "election.admin",
        Action::ProposeElection {
            election: election("pending", MuseHouse::Calli, "being.calli", 6),
        },
    );
    apply(&mut g, date(12, 14), "election.admin", Action::ObserveDate);
    assert_eq!(g.elections()["pending"].status, ElectionStatus::Expired);
    reject(
        &mut g,
        date(12, 14),
        "election.admin",
        Action::CertifyElection {
            election: "pending".into(),
        },
    );
    now(
        &mut g,
        "election.admin",
        Action::ProposeElection {
            election: election("next", MuseHouse::Calli, "being.calli", 12),
        },
    );
}

fn recall(id: &str) -> Recall {
    Recall {
        house: MuseHouse::Calli,
        tenure: "election.calli.tenure".into(),
        court_case: "case.recall".into(),
        roll: roll(id, MuseHouse::Calli),
        petition: (0..2)
            .map(|i| proof(&format!("{id}.petition.{i}"), &format!("peer.{i}")))
            .collect(),
        ballots: votes(id, &["remove", "remove", "remove", "retain"]),
        court_review: proof(&format!("{id}.court"), "court.reviewer"),
    }
}
#[test]
fn recall_needs_twenty_forty_strict_majority_and_independent_case_review() {
    let mut p = policy();
    p.court_reviewers.insert("peer.0".into());
    let mut g = college_with(p);
    for which in 0..3 {
        let mut r = recall("bad");
        match which {
            0 => {
                r.petition.pop();
            }
            1 => {
                r.ballots.pop();
            }
            _ => {
                r.court_review.signer = "peer.0".into();
            }
        }
        reject(
            &mut g,
            date(4, 14),
            "election.admin",
            Action::CertifyRecall { recall: r },
        );
    }
    let mut r = recall("tied");
    r.ballots = votes("tied", &["remove", "remove", "retain", "retain"]);
    now(
        &mut g,
        "election.admin",
        Action::CertifyRecall { recall: r },
    );
    assert!(g.incumbent(Office::Persephone(MuseHouse::Calli)).is_some());
    manifest(&mut g, "being.calli", "persephone");
    now(
        &mut g,
        "election.admin",
        Action::CertifyRecall {
            recall: recall("removed"),
        },
    );
    assert!(g.incumbent(Office::Persephone(MuseHouse::Calli)).is_none());
    assert_eq!(g.expressed_form("being.calli"), Some("muse"));
    assert!(g.has_mastery("being.calli", "persephone"));
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

fn removal(g: &Governance, id: &str, n: usize) -> Removal {
    Removal {
        tenure: "tenure.call.one".into(),
        court_case: "case.removal".into(),
        grounds: proof(&format!("{id}.grounds"), "court.reviewer"),
        notice_and_response: proof(&format!("{id}.notice"), "court.reviewer"),
        appeals_resolved: proof(&format!("{id}.appeals"), "court.reviewer"),
        court_review: proof(&format!("{id}.review"), "court.reviewer"),
        ballots: seats(g, id, n, "remove"),
    }
}
#[test]
fn six_seat_removal_is_separate_from_succession_and_reverts_sovereign_form() {
    let mut g = sovereign();
    manifest(&mut g, "being.leader", "prima-donna");
    let mut bad = removal(&g, "bad", 6);
    bad.court_review.signer = "being.leader".into();
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::RemoveSovereign { removal: bad },
    );
    let five = removal(&g, "five", 5);
    now(
        &mut g,
        "election.admin",
        Action::RemoveSovereign { removal: five },
    );
    assert!(g.incumbent(Office::PrimaDonna).is_some());
    let six = removal(&g, "six", 6);
    now(
        &mut g,
        "election.admin",
        Action::RemoveSovereign { removal: six },
    );
    assert!(g.incumbent(Office::PrimaDonna).is_none());
    assert_eq!(g.expressed_form("being.leader"), Some("muse"));
    assert_eq!(
        g.tenures()
            .iter()
            .filter(|t| t.office == Office::PrimaDonna)
            .count(),
        1
    );
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn former_electors_remain_conflicted_on_call_certification_and_removal_retry() {
    let mut p = policy();
    p.court_reviewers.insert("being.calli".into());
    let mut g = college_with(p.clone());
    ready(&mut g, "call.one", "being.leader");
    let ballots = seats(&g, "first", 5, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    now(
        &mut g,
        "being.calli",
        Action::Resign {
            office: Office::Persephone(MuseHouse::Calli),
            tenure: "election.calli.tenure".into(),
        },
    );
    let ballots = seats(&g, "second", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    let mut g = Governance::decode(&g.encode().unwrap()).unwrap();
    let mut bad = crown("call.one");
    if let Action::Corinate { certification, .. } = &mut bad {
        certification.signer = "being.calli".into();
    }
    reject(&mut g, date(4, 14), "election.admin", bad);
    now(&mut g, "election.admin", crown("call.one"));
    let mut g = sovereign_with(p);
    let first = removal(&g, "first.removal", 5);
    now(
        &mut g,
        "election.admin",
        Action::RemoveSovereign { removal: first },
    );
    now(
        &mut g,
        "being.calli",
        Action::Resign {
            office: Office::Persephone(MuseHouse::Calli),
            tenure: "election.calli.tenure".into(),
        },
    );
    let mut second = removal(&g, "second.removal", 6);
    second.court_review.signer = "being.calli".into();
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::RemoveSovereign { removal: second },
    );
}

#[test]
fn legacy_affiliation_and_nonzero_form_progress_round_trip_without_new_membership() {
    use hollow_grove::hollow_deck::canonical_hollow_deck;
    use hollow_grove::hollow_deck_state::{
        HollowDeckState, decode_hollow_deck_state, encode_hollow_deck_state,
    };
    let catalog = canonical_hollow_deck();
    for seat in ["Melony", "Teri"] {
        let mut state = HollowDeckState::default();
        state
            .set_glaushouse_affiliation(&catalog, seat, Some("Presensor"))
            .unwrap();
        for form in ["fairy", "muse", "siren"] {
            state.cultivate(&catalog, form, 4200).unwrap();
        }
        let bytes = encode_hollow_deck_state(&catalog, &state).unwrap();
        let restored = decode_hollow_deck_state(&catalog, bytes.as_bytes()).unwrap();
        assert_eq!(restored, state);
        assert_eq!(
            encode_hollow_deck_state(&catalog, &restored).unwrap(),
            bytes
        );
        let g = fresh();
        assert!(g.agency("being.legacy").is_none());
        assert!(!g.may_manifest("being.legacy", "persephone"));
    }
}

#[test]
fn certified_winner_cannot_be_discarded_by_rerunning_ovation() {
    let mut g = college();
    ready(&mut g, "call.one", "being.leader");
    let ballots = seats(&g, "six", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots,
        },
    );
    reject(
        &mut g,
        date(4, 14),
        "election.admin",
        Action::Ovation {
            id: "call.one".into(),
            ballots: vec![],
        },
    );
    now(&mut g, "election.admin", crown("call.one"));
}

#[test]
fn renewal_requires_fresh_call_and_cannot_shorten_existing_nine_year_term() {
    let mut g = sovereign();
    manifest(&mut g, "being.leader", "prima-donna");
    for (year, group) in [(6, 0), (8, 1), (10, 2), (12, 0)] {
        for h in &MuseHouse::ALL[group * 3..group * 3 + 3] {
            apply(
                &mut g,
                date(year, 0),
                "election.admin",
                Action::ProposeElection {
                    election: election(
                        &format!("reelect.{year}.{}", h.slug()),
                        *h,
                        &format!("being.{}", h.slug()),
                        year,
                    ),
                },
            );
        }
        for h in &MuseHouse::ALL[group * 3..group * 3 + 3] {
            apply(
                &mut g,
                date(year, 14),
                "election.admin",
                Action::CertifyElection {
                    election: format!("reelect.{year}.{}", h.slug()),
                },
            );
        }
    }
    ready(&mut g, "call.two", "being.leader");
    let ballots = seats(&g, "renewal", 6, "being.leader");
    now(
        &mut g,
        "election.admin",
        Action::Ovation {
            id: "call.two".into(),
            ballots,
        },
    );
    reject(&mut g, date(12, 14), "election.admin", crown("call.two"));
    apply(&mut g, date(13, 14), "election.admin", crown("call.two"));
    let current = g.incumbent(Office::PrimaDonna).unwrap();
    assert_eq!(current.id, "tenure.call.two");
    assert_eq!(current.expires, date(22, 14));
    assert_eq!(
        g.tenures()
            .iter()
            .find(|t| t.id == "tenure.call.one")
            .unwrap()
            .ended,
        Some(date(13, 14))
    );
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn manifestation_requires_independent_clinical_clearance_and_distinct_consent() {
    let mut g = sovereign();
    for bad in 0..3 {
        let mut action = Action::Manifest {
            form: "prima-donna".into(),
            ordinary_reversion: "muse".into(),
            clearance: proof("clearance", "clinical.reviewer"),
            consent: proof("consent", "being.leader"),
        };
        if let Action::Manifest {
            clearance,
            consent,
            ordinary_reversion,
            ..
        } = &mut action
        {
            match bad {
                0 => clearance.signer = "being.leader".into(),
                1 => consent.record = clearance.record.clone(),
                _ => *ordinary_reversion = "pixie".into(),
            }
        }
        reject(&mut g, date(4, 14), "being.leader", action);
    }
    manifest(&mut g, "being.leader", "prima-donna");
    assert_eq!(g.expressed_form("being.leader"), Some("prima-donna"));
}

#[test]
fn inspection_cli_replays_without_rewriting_and_rejects_corruption() {
    let g = sovereign();
    let bytes = g.encode().unwrap();
    let path =
        std::env::temp_dir().join(format!("glaushouse-rev100-cli-{}.json", std::process::id()));
    std::fs::write(&path, &bytes).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["glaushouse", "inspect"])
        .arg(&path)
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let view: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(view["rule_set"], RULE_SET);
    assert_eq!(view["current_offices"].as_array().unwrap().len(), 10);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let corrupt = String::from_utf8(bytes)
        .unwrap()
        .replace("being.leader", "being.forged");
    std::fs::write(&path, &corrupt).unwrap();
    let bad = std::process::Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["glaushouse", "inspect"])
        .arg(&path)
        .arg("--json")
        .output()
        .unwrap();
    assert!(!bad.status.success());
    assert!(bad.stdout.is_empty());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), corrupt);
    std::fs::remove_file(path).unwrap();
}
