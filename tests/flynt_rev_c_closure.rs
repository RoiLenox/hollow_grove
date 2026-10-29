use super::*;

#[test]
fn source_vacancy_cannot_be_silently_replaced_by_the_canonical_tross() {
    for omit in [false, true] {
        let mut state = WorldSession::canonical().institutional().clone();
        if omit {
            state
                .catalog
                .office_holders
                .retain(|h| h.office != flynt::tross_office_id());
        } else {
            for holder in &mut state.catalog.office_holders {
                if holder.office == flynt::tross_office_id() {
                    holder.active = false;
                }
            }
        }
        let world = WorldSession::from_institutional_state(state).unwrap();
        let original = world.persisted_state_output();
        assert_eq!(
            Governance::new(&world, policy(), date(0, 0)).unwrap_err(),
            Error::UnresolvedLaw(UnresolvedLaw::SourceSovereignContinuity)
        );
        assert_eq!(world.persisted_state_output(), original);
        assert!(
            !world
                .institutional()
                .catalog
                .office_holders
                .iter()
                .any(|h| h.office == flynt::tross_office_id() && h.active)
        );
    }
    let frozen = include_bytes!("fixtures/flynt-governance-a-v1.json");
    assert_eq!(
        Governance::decode(frozen).unwrap().encode().unwrap(),
        frozen
    );
}

#[test]
fn all_three_peer_elections_preserve_distinct_forms_ranks_tenures_and_direct_reporting() {
    let mut g = fresh();
    let source = g.source_snapshot().to_owned();
    for (office, person) in [
        (Office::Navigator, "person.nav"),
        (Office::Chief, "person.chief"),
        (Office::Mariner, "person.mar"),
    ] {
        let credential = pair_credential(person, office.pair());
        record_pair_forms(&mut g, &credential);
        certify_pair(&mut g, credential);
        apply(
            &mut g,
            date(0, 0),
            "credential.reviewer",
            Action::RecognizeCareer {
                person: person.into(),
                rank: office.career()[2],
                certification: proof(&format!("{person}.career"), "credential.reviewer"),
            },
        );
        assert!(!g.qualified(person, office));
        assert!(g.incumbent(office).is_none());
    }
    // Pair mastery and the senior career title still do not certify command competence.
    reject(
        &mut g,
        date(0, 0),
        "election.admin",
        Action::RecordElection {
            election: election("unqualified.nav", Office::Navigator, "person.nav"),
        },
    );
    for (office, person) in [
        (Office::Navigator, "person.nav"),
        (Office::Chief, "person.chief"),
        (Office::Mariner, "person.mar"),
        (Office::Chief, "person.nav"),
    ] {
        qualify(&mut g, person, office);
    }
    for (office, year, person, id) in [
        (Office::Navigator, 0, "person.nav", "closure.nav"),
        (Office::Chief, 2, "person.chief", "closure.chief"),
        (Office::Mariner, 4, "person.mar", "closure.mar"),
    ] {
        if office == Office::Chief {
            reject(
                &mut g,
                date(2, 0),
                "election.admin",
                Action::RecordElection {
                    election: election("dual.office", office, "person.nav"),
                },
            );
        }
        reject(
            &mut g,
            date(year, 0),
            "being.flynt.tross",
            Action::RecordElection {
                election: election(id, office, person),
            },
        );
        record(&mut g, year, election(id, office, person));
        apply(&mut g, date(year, 5), "election.admin", seat(id));
        assert_eq!(g.incumbent(office).unwrap().holder, person);
        assert_eq!(g.expressed_form(person), None);
        assert!(!g.has_bodily_mastery(person, BodilyForm::Manticorp));
        assert!(!g.has_bodily_mastery(person, BodilyForm::Chimera));
    }
    let chief = g.incumbent(Office::Chief).unwrap().clone();
    let mariner = g.incumbent(Office::Mariner).unwrap().clone();
    reject(
        &mut g,
        date(4, 5),
        "person.nav",
        Action::ResignCommander {
            office: Office::Mariner,
            tenure: mariner.id.clone(),
        },
    );
    reject(
        &mut g,
        date(4, 5),
        "person.nav",
        Action::VetoElection {
            election: "closure.mar".into(),
            grounds: proof("nav.cannot-veto", "person.nav"),
        },
    );
    let catalog = g.command_catalog().unwrap();
    assert_eq!(catalog.offices.len(), 4);
    assert_eq!(
        catalog.office_holders.iter().filter(|h| h.active).count(),
        4
    );
    assert_eq!(catalog.relationships.len(), 3);
    for office in Office::ALL {
        let edges: Vec<_> = catalog
            .relationships
            .iter()
            .filter(|r| r.source == InstitutionalEntityId::Office(office.id()))
            .collect();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].kind, RelationshipKind::AnswersTo);
        assert_eq!(
            edges[0].target,
            InstitutionalEntityId::Office(flynt::tross_office_id())
        );
    }
    apply(&mut g, date(6, 0), "election.admin", Action::ObserveDate);
    assert!(g.incumbent(Office::Navigator).is_none());
    assert_eq!(g.incumbent(Office::Chief), Some(&chief));
    assert_eq!(g.incumbent(Office::Mariner), Some(&mariner));
    assert!(g.has_career("person.nav", CareerRank::Navigator));
    assert!(g.has_form("person.nav", Form::Gargoyle));
    assert!(g.ambisynth("person.nav", Pair::GargoyleMedusa).is_some());
    assert_eq!(g.tross(), Some("being.flynt.tross"));
    assert_eq!(g.source_snapshot(), source);
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn election_participant_cannot_resolve_own_appeal_even_when_on_court_registry() {
    let mut p = policy();
    p.court_reviewers.insert("peer.0".into());
    p.court_reviewers.insert("being.flynt.tross".into());
    let mut g = Governance::new(&WorldSession::canonical(), p, date(0, 0)).unwrap();
    qualify(&mut g, "person.nav", Office::Navigator);
    record(
        &mut g,
        0,
        election("recusal.nav", Office::Navigator, "person.nav"),
    );
    apply(
        &mut g,
        date(0, 1),
        "being.flynt.tross",
        Action::VetoElection {
            election: "recusal.nav".into(),
            grounds: proof("recusal.veto", "being.flynt.tross"),
        },
    );
    for actor in ["peer.0", "being.flynt.tross"] {
        reject(
            &mut g,
            date(0, 2),
            actor,
            Action::ResolveVeto {
                election: "recusal.nav".into(),
                upheld: false,
                review: proof(&format!("{actor}.appeal"), actor),
            },
        );
    }
    apply(
        &mut g,
        date(0, 2),
        "court.reviewer",
        Action::ResolveVeto {
            election: "recusal.nav".into(),
            upheld: false,
            review: proof("recusal.independent", "court.reviewer"),
        },
    );
    let mut seating = seat("recusal.nav");
    if let Action::SeatCommander {
        appeals_resolved, ..
    } = &mut seating
    {
        *appeals_resolved = proof("recusal.self-certification", "peer.0");
    }
    reject(&mut g, date(0, 5), "election.admin", seating);
    apply(&mut g, date(0, 5), "election.admin", seat("recusal.nav"));
    assert_eq!(g.incumbent(Office::Navigator).unwrap().holder, "person.nav");
    assert_eq!(g.tross(), Some("being.flynt.tross"));
}
