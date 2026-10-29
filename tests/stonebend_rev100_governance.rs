//! Current governance acceptance tests. The older succession suites remain
//! historical interpreters and do not authorize Revision 100 accessions.
use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::constitutional::InterfaceApplyOutcome;
use hollow_grove::institution::{InstitutionalBeingId, OfficeHolder};
use hollow_grove::world::session::WorldSession;
use hollow_grove::world::stonebend::governance::*;

fn proof(record: &str, signer: &str) -> Attestation {
    Attestation {
        record: record.into(),
        signer: signer.into(),
    }
}

fn policy() -> Policy {
    Policy {
        authorization: proof("mandate.election-administration", "court.authorizer"),
        administrator: "election.admin".into(),
        credential_reviewers: BTreeSet::from(["credential.reviewer".into()]),
        court_reviewers: BTreeSet::from(["court.reviewer".into()]),
        first_proliteriate_year: Some(0),
        first_freemason_year: Some(3),
    }
}

fn fresh() -> Governance {
    fresh_with_policy(policy())
}

fn fresh_with_policy(policy: Policy) -> Governance {
    let mut state = WorldSession::canonical().institutional().clone();
    state.catalog.office_holders.push(OfficeHolder {
        office: hollow_grove::world::glaushouse::prima_donna_office_id(),
        being: InstitutionalBeingId::new("being.clinician").unwrap(),
        active: true,
    });
    Governance::new(
        &WorldSession::from_institutional_state(state).unwrap(),
        policy,
        0,
    )
    .unwrap()
}

fn command(g: &Governance, year: u64, actor: &str, action: Action) -> Command {
    // Attach all supplied record references, just as a trusted adapter would.
    fn records(value: &serde_json::Value, out: &mut BTreeSet<String>) {
        match value {
            serde_json::Value::Object(fields) => {
                if let Some(serde_json::Value::String(id)) = fields.get("record") {
                    out.insert(id.clone());
                }
                for value in fields.values() {
                    records(value, out);
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    records(value, out);
                }
            }
            _ => {}
        }
    }
    let id = format!("event.test.{}", g.revision());
    let mut evidence = BTreeSet::from([format!("evidence.{}", g.revision())]);
    records(&serde_json::to_value(&action).unwrap(), &mut evidence);
    Command {
        id,
        expected_revision: g.revision(),
        year,
        actor: actor.into(),
        evidence,
        action,
    }
}

fn apply(g: &mut Governance, year: u64, actor: &str, action: Action) {
    let cmd = command(g, year, actor, action);
    assert_eq!(g.execute(cmd), Ok(InterfaceApplyOutcome::Applied));
}

fn rejects(g: &mut Governance, year: u64, actor: &str, action: Action) -> Error {
    let before = g.clone();
    let cmd = command(g, year, actor, action);
    let error = g.execute(cmd).expect_err("must reject");
    assert_eq!(
        *g, before,
        "a rejected command must not change state or history"
    );
    error
}

fn qualify(g: &mut Governance, person: &str, office: Office) {
    let year = g.year();
    if office != Office::Hypergiant {
        apply(
            g,
            year,
            "credential.reviewer",
            Action::RecognizeMastery {
                person: person.into(),
                form: office.form().into(),
            },
        );
    }
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

fn roll(office: Office, id: &str) -> ElectoralRoll {
    ElectoralRoll {
        office,
        eligible: (0..10).map(|i| format!("voter.{i}")).collect(),
        certification: proof(&format!("{id}.roll"), "election.admin"),
    }
}

fn votes(id: &str, choices: &[&str]) -> Vec<Vote> {
    choices
        .iter()
        .enumerate()
        .map(|(i, candidate)| Vote {
            voter: format!("voter.{i}"),
            candidate: (*candidate).into(),
            receipt: proof(&format!("{id}.vote.{i}"), &format!("voter.{i}")),
        })
        .collect()
}

fn election(id: &str, office: Office, winner: &str) -> Election {
    Election {
        id: id.into(),
        office,
        candidates: BTreeSet::from([winner.into()]),
        roll: roll(office, id),
        rounds: vec![votes(id, &[winner; 6])],
        winner: winner.into(),
        tenure: format!("{id}.tenure"),
        special: false,
        certification: proof(&format!("{id}.certification"), "election.admin"),
        claims: BTreeMap::new(),
        yield_reviews: BTreeMap::new(),
        seal: (office == Office::Freemason).then(|| format!("{id}.seal")),
    }
}

fn professionals() -> Governance {
    elect_professionals(fresh())
}

fn elect_professionals(mut g: Governance) -> Governance {
    qualify(&mut g, "being.builder", Office::Proliteriate);
    qualify(&mut g, "being.crafter", Office::Freemason);
    apply(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection {
            election: election("election.p1", Office::Proliteriate, "being.builder"),
        },
    );
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection {
            election: election("election.f1", Office::Freemason, "being.crafter"),
        },
    );
    g
}

fn acclamation(g: &mut Governance, id: &str, candidate: &str) -> Election {
    qualify(g, candidate, Office::Hypergiant);
    let mut e = election(id, Office::Hypergiant, candidate);
    e.claims.insert(
        candidate.into(),
        proof(&format!("{id}.claim"), "being.crafter"),
    );
    e.yield_reviews.insert(
        candidate.into(),
        proof(&format!("{id}.yield"), "being.builder"),
    );
    e
}

fn reigning() -> Governance {
    let mut g = professionals();
    let e = acclamation(&mut g, "election.h1", "being.justice");
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    g
}

fn open_trial(g: &mut Governance) {
    apply(
        g,
        3,
        "being.builder",
        Action::OpenTrial {
            case: "case.duty".into(),
            initiating_office: Office::Proliteriate,
        },
    );
}

fn judgment(g: &mut Governance, finding: Finding) {
    for stage in [TrialStage::Defense, TrialStage::Proof, TrialStage::Judgment] {
        apply(
            g,
            3,
            "court.reviewer",
            Action::AdvanceTrial {
                case: "case.duty".into(),
                stage,
                finding: (stage == TrialStage::Judgment).then_some(finding),
            },
        );
    }
}

fn support(g: &mut Governance, office: Office, actor: &str) {
    apply(
        g,
        3,
        actor,
        Action::SupportRemoval {
            case: "case.duty".into(),
            office,
        },
    );
}

fn reckoning() -> Action {
    Action::AdvanceTrial {
        case: "case.duty".into(),
        stage: TrialStage::Reckoning,
        finding: None,
    }
}

#[test]
fn mastery_is_portable_and_never_an_office_or_a_sovereign_form() {
    let mut g = fresh();
    for person in ["being.first", "being.second"] {
        qualify(&mut g, person, Office::Freemason);
        assert!(g.may_manifest(person, "freemason"));
        assert!(!g.may_manifest(person, "hypergiant"));
    }
    assert!(g.incumbent(Office::Freemason).is_none());
    rejects(
        &mut g,
        0,
        "credential.reviewer",
        Action::RecognizeMastery {
            person: "being.first".into(),
            form: "hypergiant".into(),
        },
    );
    rejects(
        &mut g,
        0,
        "being.first",
        Action::Qualify {
            person: "being.first".into(),
            office: Office::Hypergiant,
        },
    );
}

#[test]
fn unknown_cohorts_and_bad_stagger_fail_without_inventing_law() {
    let mut p = policy();
    p.first_freemason_year = Some(0);
    assert!(Governance::new(&WorldSession::canonical(), p, 0).is_err());
    let mut p = policy();
    p.first_proliteriate_year = None;
    let mut g = Governance::new(&WorldSession::canonical(), p, 0).unwrap();
    qualify(&mut g, "being.builder", Office::Proliteriate);
    assert!(matches!(
        rejects(
            &mut g,
            0,
            "election.admin",
            Action::CertifyElection {
                election: election("election.unknown", Office::Proliteriate, "being.builder")
            }
        ),
        Error::RequiresLegalParameter(_)
    ));
}

#[test]
fn terms_stagger_vacancies_remainders_and_reelection_preserve_mastery() {
    let mut g = professionals();
    assert_eq!(g.incumbent(Office::Proliteriate).unwrap().expires, Some(6));
    assert_eq!(g.incumbent(Office::Freemason).unwrap().expires, Some(9));
    rejects(
        &mut g,
        4,
        "election.admin",
        Action::CertifyElection {
            election: election("election.occupied", Office::Freemason, "being.crafter"),
        },
    );
    apply(
        &mut g,
        4,
        "being.crafter",
        Action::Resign {
            office: Office::Freemason,
            tenure: "election.f1.tenure".into(),
        },
    );
    assert!(g.active_seal().is_none());
    assert!(g.may_manifest("being.crafter", "freemason"));
    let mut special = election("election.f2", Office::Freemason, "being.crafter");
    special.special = true;
    apply(
        &mut g,
        4,
        "election.admin",
        Action::CertifyElection { election: special },
    );
    assert_eq!(g.incumbent(Office::Freemason).unwrap().expires, Some(9));
    assert_eq!(g.active_seal(), Some("election.f2.seal"));
    apply(
        &mut g,
        9,
        "election.admin",
        Action::CertifyElection {
            election: election("election.f3", Office::Freemason, "being.crafter"),
        },
    );
    assert!(g.incumbent(Office::Proliteriate).is_none());
    assert_eq!(g.incumbent(Office::Freemason).unwrap().expires, Some(15));
    assert_eq!(g.tenures().len(), 4);
    assert!(
        g.tenures()
            .iter()
            .filter(|t| t.ended.is_some())
            .all(|t| t.closure_event.is_some())
    );
}

#[test]
fn votes_require_unique_eligible_voters_and_independent_certification() {
    let mut g = fresh();
    qualify(&mut g, "being.builder", Office::Proliteriate);
    let base = election("election.p", Office::Proliteriate, "being.builder");
    let mut e = base.clone();
    let duplicate = e.rounds[0][0].clone();
    e.rounds[0].push(duplicate);
    rejects(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    let mut e = base.clone();
    e.rounds[0][0].voter = "voter.outsider".into();
    rejects(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    let mut e = base.clone();
    e.certification.signer = "being.builder".into();
    rejects(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    let mut cmd = command(
        &g,
        0,
        "election.admin",
        Action::CertifyElection { election: base },
    );
    cmd.evidence.remove("election.p.roll");
    assert!(g.execute(cmd).is_err());
}

#[test]
fn top_two_runoff_and_undefined_ties_are_distinct() {
    let mut g = fresh();
    for person in ["being.a", "being.b", "being.c"] {
        qualify(&mut g, person, Office::Proliteriate);
    }
    let mut e = election("election.runoff", Office::Proliteriate, "being.a");
    e.candidates = ["being.a", "being.b", "being.c"].map(String::from).into();
    e.rounds = vec![
        votes(
            "round.first",
            &[
                "being.a", "being.a", "being.a", "being.b", "being.b", "being.c",
            ],
        ),
        votes("round.last", &["being.a", "being.a", "being.b"]),
    ];
    let mut tie = e.clone();
    tie.rounds[0] = votes("round.tie", &["being.a", "being.a", "being.b", "being.c"]);
    assert!(matches!(
        rejects(
            &mut g,
            0,
            "election.admin",
            Action::CertifyElection { election: tie }
        ),
        Error::RequiresLegalParameter(_)
    ));
    let mut reused = e.clone();
    reused.rounds[1][0].receipt = reused.rounds[0][0].receipt.clone();
    rejects(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: reused },
    );
    apply(
        &mut g,
        0,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    assert_eq!(g.incumbent(Office::Proliteriate).unwrap().holder, "being.a");
}

#[test]
fn acclamation_requires_all_candidates_verified_and_is_not_geographic() {
    let mut g = professionals();
    let mut e = acclamation(&mut g, "election.h", "being.justice");
    qualify(&mut g, "being.other", Office::Hypergiant);
    e.candidates.insert("being.other".into());
    rejects(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection {
            election: e.clone(),
        },
    );
    e.claims.insert(
        "being.other".into(),
        proof("election.h.other.claim", "being.crafter"),
    );
    e.yield_reviews.insert(
        "being.other".into(),
        proof("election.h.other.yield", "being.builder"),
    );
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    assert_eq!(g.incumbent(Office::Hypergiant).unwrap().expires, None);
    let another = acclamation(&mut g, "election.h2", "being.other");
    rejects(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection { election: another },
    );
    apply(&mut g, 100, "election.admin", Action::ObserveYear);
    assert_eq!(
        g.incumbent(Office::Hypergiant).unwrap().holder,
        "being.justice"
    );
    assert!(g.may_manifest("being.justice", "hypergiant"));
    assert!(
        !String::from_utf8(g.encode().unwrap())
            .unwrap()
            .contains("Lazerhorn")
    );
}

fn recall(id: &str, choices: &[&str]) -> Recall {
    Recall {
        office: Office::Proliteriate,
        tenure: "election.p1.tenure".into(),
        roll: roll(Office::Proliteriate, id),
        petition: (0..2)
            .map(|i| proof(&format!("{id}.petition.{i}"), &format!("voter.{i}")))
            .collect(),
        ballots: votes(id, choices),
        court_review: proof(&format!("{id}.court"), "court.reviewer"),
    }
}

#[test]
fn recall_enforces_twenty_forty_and_strict_majority_with_review() {
    let mut g = professionals();
    let mut r = recall("recall.short", &["remove"; 4]);
    r.petition.pop();
    rejects(
        &mut g,
        3,
        "election.admin",
        Action::CertifyRecall { recall: r },
    );
    rejects(
        &mut g,
        3,
        "election.admin",
        Action::CertifyRecall {
            recall: recall("recall.turnout", &["remove"; 3]),
        },
    );
    let mut r = recall("recall.self-review", &["remove"; 4]);
    r.court_review.signer = "being.builder".into();
    rejects(
        &mut g,
        3,
        "election.admin",
        Action::CertifyRecall { recall: r },
    );
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyRecall {
            recall: recall("recall.tie", &["remove", "remove", "retain", "retain"]),
        },
    );
    assert!(g.incumbent(Office::Proliteriate).is_some());
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyRecall {
            recall: recall("recall.success", &["remove", "remove", "remove", "retain"]),
        },
    );
    assert!(g.incumbent(Office::Proliteriate).is_none());
    assert!(g.may_manifest("being.builder", "proliteriate"));
}

#[test]
fn trial_requires_exact_stages_independent_court_and_both_current_offices() {
    let mut g = reigning();
    rejects(
        &mut g,
        3,
        "being.outsider",
        Action::OpenTrial {
            case: "case.duty".into(),
            initiating_office: Office::Proliteriate,
        },
    );
    open_trial(&mut g);
    rejects(
        &mut g,
        3,
        "court.reviewer",
        Action::AdvanceTrial {
            case: "case.duty".into(),
            stage: TrialStage::Judgment,
            finding: Some(Finding::RemovalWarranted),
        },
    );
    rejects(
        &mut g,
        3,
        "being.justice",
        Action::AdvanceTrial {
            case: "case.duty".into(),
            stage: TrialStage::Defense,
            finding: None,
        },
    );
    judgment(&mut g, Finding::RemovalWarranted);
    support(&mut g, Office::Proliteriate, "being.builder");
    rejects(&mut g, 3, "court.reviewer", reckoning());
    support(&mut g, Office::Freemason, "being.crafter");
    let mut recused = g.clone();
    apply(
        &mut recused,
        3,
        "being.crafter",
        Action::Recuse {
            case: "case.duty".into(),
            office: Office::Freemason,
        },
    );
    rejects(&mut recused, 3, "court.reviewer", reckoning());
    let mut vacant = g.clone();
    apply(
        &mut vacant,
        3,
        "being.crafter",
        Action::Resign {
            office: Office::Freemason,
            tenure: "election.f1.tenure".into(),
        },
    );
    rejects(&mut vacant, 3, "court.reviewer", reckoning());
    let mut e = election("election.replacement", Office::Freemason, "being.crafter");
    e.special = true;
    apply(
        &mut vacant,
        3,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    rejects(&mut vacant, 3, "court.reviewer", reckoning());
    apply(&mut g, 3, "court.reviewer", reckoning());
    assert!(g.incumbent(Office::Hypergiant).is_none());
    assert_eq!(g.trial("case.duty").unwrap().stage.name(), "The Reckoning");
    assert!(!g.may_manifest("being.builder", "hypergiant"));
}

#[test]
fn remedy_or_dismissal_does_not_remove_and_cannot_reopen() {
    for finding in [Finding::Dismissed, Finding::ProtectiveRemedy] {
        let mut g = reigning();
        open_trial(&mut g);
        judgment(&mut g, finding);
        apply(&mut g, 3, "court.reviewer", reckoning());
        assert!(g.incumbent(Office::Hypergiant).is_some());
        rejects(&mut g, 3, "court.reviewer", reckoning());
    }
}

#[test]
fn removal_reverts_only_incumbent_form_preserves_mastery_and_projects_world() {
    let mut g = reigning();
    apply(
        &mut g,
        3,
        "credential.reviewer",
        Action::RecognizeMastery {
            person: "being.justice".into(),
            form: "troll".into(),
        },
    );
    adopt_clinical_policy(&mut g);
    register_clinical(&mut g, clinical_authorization());
    let manifest = Action::Manifest {
        form: "hypergiant".into(),
        ordinary_reversion: "troll".into(),
        clearance: proof("clearance.hypergiant", "being.clinician"),
        consent: proof("consent.hypergiant", "being.justice"),
    };
    apply(&mut g, 3, "being.justice", manifest);
    assert_eq!(g.expressed_form("being.justice"), Some("hypergiant"));
    // Withdrawing consent cannot block the existing safe-reversion obligation.
    apply(
        &mut g,
        3,
        "being.justice",
        Action::UpdateClinicalAuthorization {
            update: ClinicalUpdate::Revoke {
                record: "clinical.authorization".into(),
                certification: proof("clinical.withdrawal", "being.justice"),
            },
        },
    );
    open_trial(&mut g);
    judgment(&mut g, Finding::RemovalWarranted);
    support(&mut g, Office::Proliteriate, "being.builder");
    support(&mut g, Office::Freemason, "being.crafter");
    apply(&mut g, 3, "court.reviewer", reckoning());
    assert_eq!(g.expressed_form("being.justice"), Some("troll"));
    assert!(g.may_manifest("being.justice", "troll"));
    let world = g.world_session().unwrap();
    let holders = &world.institutional().catalog.office_holders;
    assert!(
        holders
            .iter()
            .any(|h| h.being.as_str() == "being.clinician")
    );
    assert!(
        holders
            .iter()
            .any(|h| h.office == Office::Proliteriate.id())
    );
    assert!(
        !holders
            .iter()
            .any(|h| h.active && h.office == Office::Hypergiant.id())
    );
    let source = fresh().world_session().unwrap();
    assert_eq!(
        world.institutional().memberships,
        source.institutional().memberships
    );
}

#[test]
fn archive_replay_is_deterministic_idempotent_and_tamper_evident() {
    let mut g = reigning();
    let cmd = command(&g, 4, "election.admin", Action::ObserveYear);
    assert_eq!(g.execute(cmd.clone()), Ok(InterfaceApplyOutcome::Applied));
    assert_eq!(
        g.execute(cmd.clone()),
        Ok(InterfaceApplyOutcome::Idempotent)
    );
    let before = g.clone();
    let mut conflict = cmd;
    conflict.year = 5;
    assert!(g.execute(conflict).is_err());
    assert_eq!(g, before);
    let bytes = g.encode().unwrap();
    let restored = Governance::decode(&bytes).unwrap();
    assert_eq!(restored, g);
    assert_eq!(restored.encode().unwrap(), bytes);
    assert_eq!(
        restored.world_session().unwrap().persisted_state_output(),
        g.world_session().unwrap().persisted_state_output()
    );
    let corrupted = String::from_utf8(bytes)
        .unwrap()
        .replace("being.justice", "being.forgery");
    assert!(Governance::decode(corrupted.as_bytes()).is_err());
    let mut stale = command(&g, 5, "election.admin", Action::ObserveYear);
    stale.expected_revision -= 1;
    assert!(g.execute(stale).is_err());
    rejects(&mut g, 2, "election.admin", Action::ObserveYear);
}

#[test]
fn legacy_snapshot_import_preserves_holder_without_fabricating_terms() {
    let mut state = WorldSession::canonical().institutional().clone();
    state.catalog.office_holders.push(OfficeHolder {
        office: Office::Hypergiant.id(),
        being: InstitutionalBeingId::new("being.legacy").unwrap(),
        active: true,
    });
    let world = WorldSession::from_institutional_state(state.clone()).unwrap();
    let g = Governance::new(&world, policy(), 12).unwrap();
    assert!(g.incumbent(Office::Hypergiant).unwrap().imported);
    assert_eq!(
        g.incumbent(Office::Hypergiant).unwrap().holder,
        "being.legacy"
    );
    state.catalog.office_holders.push(OfficeHolder {
        office: Office::Freemason.id(),
        being: InstitutionalBeingId::new("being.legacy-crafter").unwrap(),
        active: true,
    });
    let world = WorldSession::from_institutional_state(state).unwrap();
    assert!(matches!(
        Governance::new(&world, policy(), 12),
        Err(Error::RequiresLegalParameter(_))
    ));
}

#[test]
fn institutional_projection_preserves_closed_and_legacy_officeholder_records() {
    let mut state = fresh().world_session().unwrap().institutional().clone();
    for (office, person, active) in [
        (Office::Hypergiant, "being.former-sovereign", false),
        (Office::Freemason, "being.former-crafter", false),
        (Office::Hypergiant, "being.legacy-sovereign", true),
    ] {
        state.catalog.office_holders.push(OfficeHolder {
            office: office.id(),
            being: InstitutionalBeingId::new(person).unwrap(),
            active,
        });
    }
    let source = WorldSession::from_institutional_state(state).unwrap();
    let original = source.persisted_state_output();
    let mut g = Governance::new(&source, policy(), 0).unwrap();
    assert_eq!(
        g.world_session().unwrap().persisted_state_output(),
        original
    );
    g = elect_professionals(g);
    let tenure = g.incumbent(Office::Hypergiant).unwrap().id.clone();
    apply(
        &mut g,
        3,
        "being.legacy-sovereign",
        Action::Resign {
            office: Office::Hypergiant,
            tenure,
        },
    );
    apply(&mut g, 6, "election.admin", Action::ObserveYear);
    let projected = g.world_session().unwrap();
    for (office, person) in [
        (Office::Hypergiant, "being.former-sovereign"),
        (Office::Freemason, "being.former-crafter"),
        (Office::Hypergiant, "being.legacy-sovereign"),
        (Office::Proliteriate, "being.builder"),
    ] {
        assert!(
            projected
                .institutional()
                .catalog
                .office_holders
                .contains(&OfficeHolder {
                    office: office.id(),
                    being: InstitutionalBeingId::new(person).unwrap(),
                    active: false,
                })
        );
    }
    assert!(
        projected
            .institutional()
            .catalog
            .office_holders
            .iter()
            .any(|h| {
                h.active
                    && h.office == Office::Freemason.id()
                    && h.being.as_str() == "being.crafter"
            })
    );
    let other_houses = |world: &WorldSession| {
        world
            .institutional()
            .catalog
            .office_holders
            .iter()
            .filter(|h| !Office::ALL.iter().any(|o| o.id() == h.office))
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(other_houses(&projected), other_houses(&source));
    let projection = projected.persisted_state_output();
    assert_eq!(
        WorldSession::from_persisted_output(&projection)
            .unwrap()
            .persisted_state_output(),
        projection
    );
    assert_eq!(
        Governance::decode(&g.encode().unwrap())
            .unwrap()
            .world_session()
            .unwrap()
            .persisted_state_output(),
        projection
    );
    assert_eq!(
        source.persisted_state_output(),
        original,
        "adoption must not mutate its source"
    );
}

#[test]
fn recall_participants_cannot_certify_their_own_court_review() {
    for reviewer in ["voter.4", "voter.3", "election.admin"] {
        let mut p = policy();
        p.court_reviewers.insert(reviewer.into());
        let mut g = elect_professionals(fresh_with_policy(p));
        let mut r = recall("recall.conflict", &["remove"; 4]);
        // voter.4 signs the petition only; voter.3 casts a ballot only.
        r.petition[0].signer = "voter.4".into();
        r.court_review.signer = reviewer.into();
        assert_eq!(
            rejects(
                &mut g,
                3,
                "election.admin",
                Action::CertifyRecall { recall: r.clone() }
            ),
            Error::Invalid("independent Court review required")
        );
        // A neutral reviewer can certify the same petition and ballots.
        r.court_review.signer = "court.reviewer".into();
        apply(
            &mut g,
            3,
            "election.admin",
            Action::CertifyRecall { recall: r },
        );
        assert!(g.incumbent(Office::Proliteriate).is_none());
        assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
    }
}

#[test]
fn former_trial_supporter_cannot_review_after_a_new_tenure_replaces_their_vote() {
    let mut p = policy();
    p.court_reviewers.insert("being.crafter".into());
    let mut g = elect_professionals(fresh_with_policy(p));
    let e = acclamation(&mut g, "election.h1", "being.justice");
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    open_trial(&mut g);
    support(&mut g, Office::Freemason, "being.crafter");
    apply(
        &mut g,
        3,
        "being.crafter",
        Action::Resign {
            office: Office::Freemason,
            tenure: "election.f1.tenure".into(),
        },
    );
    qualify(&mut g, "being.successor", Office::Freemason);
    let mut replacement = election("election.f2", Office::Freemason, "being.successor");
    replacement.special = true;
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection {
            election: replacement,
        },
    );
    support(&mut g, Office::Freemason, "being.successor");
    let mut g = Governance::decode(&g.encode().unwrap()).unwrap();
    assert_eq!(
        rejects(
            &mut g,
            3,
            "being.crafter",
            Action::AdvanceTrial {
                case: "case.duty".into(),
                stage: TrialStage::Defense,
                finding: None,
            }
        ),
        Error::Invalid("independent Court review required")
    );
    judgment(&mut g, Finding::RemovalWarranted);
    support(&mut g, Office::Proliteriate, "being.builder");
    apply(&mut g, 3, "court.reviewer", reckoning());
    assert!(g.incumbent(Office::Hypergiant).is_none());
}

#[test]
fn recusal_and_resignation_do_not_erase_a_trial_participants_conflict() {
    let mut p = policy();
    p.court_reviewers.insert("being.crafter".into());
    let mut g = elect_professionals(fresh_with_policy(p));
    let e = acclamation(&mut g, "election.h1", "being.justice");
    apply(
        &mut g,
        3,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    open_trial(&mut g);
    apply(
        &mut g,
        3,
        "being.crafter",
        Action::Recuse {
            case: "case.duty".into(),
            office: Office::Freemason,
        },
    );
    apply(
        &mut g,
        3,
        "being.crafter",
        Action::Resign {
            office: Office::Freemason,
            tenure: "election.f1.tenure".into(),
        },
    );
    let mut g = Governance::decode(&g.encode().unwrap()).unwrap();
    assert_eq!(
        rejects(
            &mut g,
            3,
            "being.crafter",
            Action::AdvanceTrial {
                case: "case.duty".into(),
                stage: TrialStage::Defense,
                finding: None,
            }
        ),
        Error::Invalid("independent Court review required")
    );
    judgment(&mut g, Finding::ProtectiveRemedy);
    apply(&mut g, 3, "court.reviewer", reckoning());
    assert!(g.incumbent(Office::Hypergiant).is_some());
}

fn professional_source() -> (WorldSession, Vec<ProfessionalContinuity>) {
    let mut state = fresh().world_session().unwrap().institutional().clone();
    let mut records = Vec::new();
    for (office, holder, began, expires, id) in [
        (Office::Proliteriate, "being.builder", 0, 6, "legacy.p"),
        (Office::Freemason, "being.crafter", 3, 9, "legacy.f"),
    ] {
        state.catalog.office_holders.push(OfficeHolder {
            office: office.id(),
            being: InstitutionalBeingId::new(holder).unwrap(),
            active: true,
        });
        records.push(ProfessionalContinuity {
            office,
            holder: holder.into(),
            tenure: format!("{id}.tenure"),
            source_rule_set: "stonebend.legacy-law".into(),
            election_id: format!("{id}.election"),
            began,
            expires,
            election: proof(&format!("{id}.election-proof"), "election.admin"),
            term: proof(&format!("{id}.term-proof"), "election.admin"),
            qualification: proof(&format!("{id}.qualification"), "credential.reviewer"),
            mastery: proof(&format!("{id}.mastery"), "credential.reviewer"),
            court_review: proof(&format!("{id}.court"), "court.reviewer"),
            seal: (office == Office::Freemason).then(|| SealContinuity {
                id: "legacy.f.seal".into(),
                certification: proof("legacy.f.seal-proof", "election.admin"),
            }),
        });
    }
    (
        WorldSession::from_institutional_state(state).unwrap(),
        records,
    )
}

#[test]
fn professional_reconciliation_preserves_proven_terms_seal_source_and_replay() {
    let (source, records) = professional_source();
    let before = source.persisted_state_output();
    assert!(matches!(
        Governance::new(&source, policy(), 4),
        Err(Error::RequiresLegalParameter(_))
    ));
    let mut g =
        Governance::from_reconciled_snapshot(&source, policy(), 4, records.clone()).unwrap();
    assert_eq!(g.world_session().unwrap().persisted_state_output(), before);
    assert_eq!(g.incumbent(Office::Freemason).unwrap().began, 3);
    assert_eq!(g.incumbent(Office::Freemason).unwrap().expires, Some(9));
    assert_eq!(g.active_seal(), Some("legacy.f.seal"));
    assert!(g.may_manifest("being.builder", "proliteriate"));
    assert!(!g.may_manifest("being.builder", "hypergiant"));
    let bytes = g.encode().unwrap();
    assert_eq!(Governance::decode(&bytes).unwrap(), g);
    assert_eq!(
        Governance::from_reconciled_snapshot(&source, policy(), 4, records)
            .unwrap()
            .encode()
            .unwrap(),
        bytes
    );
    assert!(
        String::from_utf8(bytes)
            .unwrap()
            .contains("stonebend.legacy-law")
    );
    apply(&mut g, 6, "election.admin", Action::ObserveYear);
    assert!(g.incumbent(Office::Proliteriate).is_none());
    assert!(g.may_manifest("being.builder", "proliteriate"));
    assert_eq!(g.active_seal(), Some("legacy.f.seal"));
    apply(&mut g, 9, "election.admin", Action::ObserveYear);
    assert!(g.active_seal().is_none());
    assert_eq!(source.persisted_state_output(), before);
}

#[test]
fn professional_reconciliation_refuses_missing_conflicting_or_fabricated_records() {
    let (source, records) = professional_source();
    let before = source.persisted_state_output();
    for scenario in 0..10 {
        let mut r = records.clone();
        match scenario {
            0 => {
                r.pop();
            }
            1 => r[1].holder = "being.impostor".into(),
            2 => r[1].office = Office::Proliteriate,
            3 => r[1].seal = None,
            4 => r[0].seal = r[1].seal.clone(),
            5 => r[1].expires = 15,
            6 => r[1].began = 5,
            7 => r[1].tenure = r[0].tenure.clone(),
            8 => r[1].election_id = r[0].election_id.clone(),
            9 => r[1].source_rule_set.clear(),
            _ => unreachable!(),
        }
        assert!(
            Governance::from_reconciled_snapshot(&source, policy(), 4, r).is_err(),
            "scenario {scenario}"
        );
        assert_eq!(source.persisted_state_output(), before);
    }
    let mut p = policy();
    p.first_freemason_year = None;
    assert!(matches!(
        Governance::from_reconciled_snapshot(&source, p, 4, records.clone()),
        Err(Error::RequiresLegalParameter(_))
    ));
    assert!(matches!(
        Governance::from_reconciled_snapshot(&source, policy(), 9, records),
        Err(Error::RequiresLegalParameter(_))
    ));
}

#[test]
fn professional_reconciliation_requires_distinct_independent_attached_proofs() {
    let (source, records) = professional_source();
    for scenario in 0..5 {
        let mut r = records.clone();
        match scenario {
            0 => r[1].election.signer = "being.crafter".into(),
            1 => r[1].court_review.signer = "being.builder".into(),
            2 => r[1].mastery.signer = "being.crafter".into(),
            3 => r[1].seal.as_mut().unwrap().certification.signer = "being.crafter".into(),
            4 => r[1].term.record = r[1].election.record.clone(),
            _ => unreachable!(),
        }
        assert!(Governance::from_reconciled_snapshot(&source, policy(), 4, r).is_err());
    }
    let mut g = Governance::default();
    let mut cmd = command(
        &g,
        4,
        "court.authorizer",
        Action::InitializeReconciled {
            policy: policy(),
            institutional_snapshot: source.persisted_state_output(),
            professional_tenures: records,
        },
    );
    cmd.evidence.remove("legacy.f.seal-proof");
    let before = g.clone();
    assert!(g.execute(cmd).is_err());
    assert_eq!(g, before);
}

#[test]
fn reconciled_tenures_cannot_reuse_old_election_or_seal_and_special_terms_do_not_extend() {
    let (source, records) = professional_source();
    let mut g = Governance::from_reconciled_snapshot(&source, policy(), 4, records).unwrap();
    apply(
        &mut g,
        4,
        "being.crafter",
        Action::Resign {
            office: Office::Freemason,
            tenure: "legacy.f.tenure".into(),
        },
    );
    let mut e = election("legacy.f.election", Office::Freemason, "being.crafter");
    e.special = true;
    rejects(
        &mut g,
        4,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    let mut e = election("new.f.election", Office::Freemason, "being.crafter");
    e.special = true;
    e.seal = Some("legacy.f.seal".into());
    rejects(
        &mut g,
        4,
        "election.admin",
        Action::CertifyElection {
            election: e.clone(),
        },
    );
    e.seal = Some("new.f.seal".into());
    apply(
        &mut g,
        4,
        "election.admin",
        Action::CertifyElection { election: e },
    );
    assert_eq!(g.incumbent(Office::Freemason).unwrap().expires, Some(9));
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
        subject: "being.justice".into(),
        procedure: Procedure {
            house: House::Stonebend,
            form: "hypergiant".into(),
        },
        valid_from: 3,
        valid_until: 3 + 3,
        clearance: proof("clearance.hypergiant", "being.clinician"),
        consent: proof("consent.hypergiant", "being.justice"),
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
        form: "hypergiant".into(),
        ordinary_reversion: "troll".into(),
        clearance: a.clearance.clone(),
        consent: a.consent.clone(),
    }
}

fn clinical_ready() -> Governance {
    let mut g = reigning();
    apply(
        &mut g,
        3,
        "credential.reviewer",
        Action::RecognizeMastery {
            person: "being.justice".into(),
            form: "troll".into(),
        },
    );
    g
}

#[test]
fn clinical_office_alone_cannot_clear_or_authorize_credentials() {
    let mut g = clinical_ready();
    let a = clinical_authorization();
    assert_eq!(
        rejects(&mut g, 3, "being.justice", clinical_manifest(&a)),
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
        rejects(&mut g, 3, actor, adopt);
    }
    let mut unsupported = clinical_policy();
    if let Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy { version, .. },
    } = &mut unsupported
    {
        *version = 1;
    }
    rejects(&mut g, 3, "court.authorizer", unsupported);
    let mut reused_mandate = clinical_policy();
    if let Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy { authorization, .. },
    } = &mut reused_mandate
    {
        *authorization = policy().authorization;
    }
    rejects(&mut g, 3, "court.authorizer", reused_mandate);
    adopt_clinical_policy(&mut g);
    rejects(&mut g, 3, "being.justice", clinical_manifest(&a));
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
        rejects(&mut g, 3, actor, register);
    }
    register_clinical(&mut g, a);
    rejects(
        &mut g,
        3,
        "being.clinician",
        Action::RecognizeMastery {
            person: "being.clinician".into(),
            form: "troll".into(),
        },
    );
    rejects(&mut g, 3, "being.clinician", Action::ObserveYear);
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
        apply(&mut g, 3, "being.justice", clinical_manifest(&a));
        assert_eq!(g.expressed_form("being.justice"), Some("hypergiant"));
        assert!(!g.may_manifest(reviewer, "hypergiant"));
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
            "procedure" => a.procedure.form = "troll".into(),
            "house" => a.procedure.house = House::Sandmanor,
            "future" => a.valid_from += 1,
            "expired" => a.valid_until = 3 + 1,
            _ => unreachable!(),
        }
        if invalid == "house" {
            rejects(
                &mut g,
                3,
                "clinical.credential-authority",
                clinical_registration(a),
            );
        } else {
            register_clinical(&mut g, a.clone());
            let year = if invalid == "expired" { 3 + 1 } else { 3 };
            let mut manifest = clinical_manifest(&a);
            if let Action::Manifest { consent, .. } = &mut manifest {
                consent.signer = "being.justice".into();
            }
            rejects(&mut g, year, "being.justice", manifest);
        }
    }
    for (start, end) in [(3, 3), (3 + 1, 3), (0, 3)] {
        let mut g = clinical_ready();
        adopt_clinical_policy(&mut g);
        let mut a = clinical_authorization();
        a.valid_from = start;
        a.valid_until = end;
        rejects(
            &mut g,
            3,
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
        rejects(&mut g, 3, "clinical.credential-authority", register);
    }
    let before = g.clone();
    let mut missing = command(
        &g,
        3,
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
        rejects(&mut g, 3, "being.justice", manifest);
    }
    let before = g.clone();
    let mut missing = command(&g, 3, "being.justice", clinical_manifest(&a));
    missing.evidence.remove(&a.consent.record);
    assert!(g.execute(missing).is_err());
    assert_eq!(g, before);
}

#[test]
fn clinical_credential_revocation_clearance_withdrawal_and_consent_withdrawal_replay() {
    for actor in [
        "clinical.credential-authority",
        "being.clinician",
        "being.justice",
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
        rejects(&mut g, 3, "being.other", revoke("being.other"));
        apply(&mut g, 3, actor, revoke(actor));
        rejects(&mut g, 3, "being.justice", clinical_manifest(&a));
        rejects(
            &mut g,
            3,
            "clinical.credential-authority",
            clinical_registration(a.clone()),
        );
        let bytes = g.encode().unwrap();
        let mut restored = Governance::decode(&bytes).unwrap();
        assert_eq!(restored, g);
        assert_eq!(restored.encode().unwrap(), bytes);
        rejects(&mut restored, 3 + 1, "being.justice", clinical_manifest(&a));
    }
}

#[test]
fn clinical_policy_transition_preserves_frozen_v1_history_without_creating_credentials() {
    // Captured using the pre-change workspace library, including an accepted
    // office-only manifestation. Never regenerated by the new policy.
    let bytes = include_bytes!("fixtures/clinical_authorization/stonebend-v1.json");
    let mut g = Governance::decode(bytes).unwrap();
    assert_eq!(g.encode().unwrap(), bytes);
    assert_eq!(g.expressed_form("being.justice"), Some("hypergiant"));
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
    rejects(&mut g, 3, "being.justice", clinical_manifest(&a));
    adopt_clinical_policy(&mut g);
    rejects(&mut g, 3, "being.justice", clinical_manifest(&a));
    rejects(&mut g, 3, "court.authorizer", clinical_policy());
    let mut downgrade = clinical_policy();
    if let Action::UpdateClinicalAuthorization {
        update: ClinicalUpdate::AdoptPolicy { version, .. },
    } = &mut downgrade
    {
        *version = 1;
    }
    rejects(&mut g, 3, "court.authorizer", downgrade);
    a.clearance.record = "clearance.new-v2".into();
    a.consent.record = "consent.new-v2".into();
    register_clinical(&mut g, a.clone());
    apply(&mut g, 3, "being.justice", clinical_manifest(&a));
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
    apply(&mut g, 3, "being.justice", clinical_manifest(&a));
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
