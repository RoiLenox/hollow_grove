use super::*;
use hollow_grove::world::flynt::governance::equipment::{
    Action as Equipment, Component, ComponentRecord, Registration, TRIPLE_CROWN_ID,
};

fn act(request: Equipment) -> Action {
    Action::TripleCrown { request }
}
fn fresh_equipment() -> Governance {
    let mut g = fresh();
    adopt_convergence(&mut g);
    for person in ["person.one", "person.two"] {
        record_currents(&mut g, person);
        certify_convergence(&mut g, person, Convergence::Chimera);
    }
    g
}
fn registration() -> Registration {
    Registration {
        specification: TRIPLE_CROWN_ID.into(),
        item: "crown.one".into(),
        owner: "owner.one".into(),
        custodian: "person.one".into(),
        components: std::array::from_fn(|i| ComponentRecord {
            component: Component::ALL[i],
            item: format!("component.{i}"),
        }),
        inspection: proof("crown.inspection", "execution.reviewer"),
        owner_consent: proof("crown.owner-consent", "owner.one"),
        custodian_consent: proof("crown.custodian-consent", "person.one"),
    }
}
fn register(g: &mut Governance) {
    apply(
        g,
        g.date(),
        "execution.reviewer",
        act(Equipment::Register {
            registration: registration(),
        }),
    );
}
fn transfer() -> Action {
    act(Equipment::Transfer {
        item: "crown.one".into(),
        from: "person.one".into(),
        to: "person.two".into(),
        inspection: proof("transfer.inspection", "execution.reviewer"),
        owner_consent: proof("transfer.owner", "owner.one"),
        outgoing_consent: proof("transfer.outgoing", "person.one"),
        incoming_consent: proof("transfer.incoming", "person.two"),
    })
}

#[test]
fn triple_crown_requires_three_existing_components_and_independent_chimera_custody() {
    let mut g = fresh_equipment();
    for invalid in [
        "spec",
        "qualification",
        "duplicate-kind",
        "duplicate-item",
        "self-item",
        "inspector",
        "consent",
        "reused-proof",
    ] {
        let mut r = registration();
        match invalid {
            "spec" => r.specification = "unknown.equipment".into(),
            "qualification" => r.custodian = "being.flynt.tross".into(),
            "duplicate-kind" => r.components[1].component = r.components[0].component,
            "duplicate-item" => r.components[1].item = r.components[0].item.clone(),
            "self-item" => r.components[0].item = r.item.clone(),
            "inspector" => r.inspection.signer = r.custodian.clone(),
            "consent" => r.owner_consent.signer = r.custodian.clone(),
            "reused-proof" => r.custodian_consent.record = r.owner_consent.record.clone(),
            _ => unreachable!(),
        }
        reject(
            &mut g,
            date(0, 0),
            "execution.reviewer",
            act(Equipment::Register { registration: r }),
        );
    }
    register(&mut g);
    reject(
        &mut g,
        date(0, 0),
        "execution.reviewer",
        act(Equipment::Register {
            registration: registration(),
        }),
    );
    let mut duplicate = registration();
    duplicate.item = "crown.two".into();
    reject(
        &mut g,
        date(0, 0),
        "execution.reviewer",
        act(Equipment::Register {
            registration: duplicate,
        }),
    );
    assert!(!g.may_present_triple_crown("person.one", "crown.one"));
    assert_eq!(g.tross(), Some("being.flynt.tross"));
}

#[test]
fn triple_crown_presentation_tracks_one_body_and_never_follows_sovereign_office() {
    let mut g = fresh_equipment();
    register(&mut g);
    let a = expression(&g, "person.one", BodilyForm::Chimera, "crown.body.one");
    apply(&mut g, date(0, 0), "execution.reviewer", a);
    assert!(g.may_present_triple_crown("person.one", "crown.one"));
    assert!(!g.may_present_triple_crown("being.flynt.tross", "crown.one"));
    let a = expression(
        &g,
        "person.one",
        BodilyForm::Sibling(Form::Gargoyle),
        "crown.body.revert",
    );
    apply(&mut g, date(0, 0), "execution.reviewer", a);
    assert!(!g.may_present_triple_crown("person.one", "crown.one"));
    assert!(g.has_bodily_mastery("person.one", BodilyForm::Chimera));
    apply(
        &mut g,
        date(0, 0),
        "person.one",
        act(Equipment::Release {
            item: "crown.one".into(),
        }),
    );
    assert_eq!(g.triple_crown("crown.one").unwrap().custodian, None);
    assert_eq!(Governance::decode(&g.encode().unwrap()).unwrap(), g);
}

#[test]
fn triple_crown_transfer_requires_all_consents_preserves_owner_and_replays_once() {
    let mut g = fresh_equipment();
    register(&mut g);
    for invalid in [
        "from",
        "unqualified",
        "owner",
        "outgoing",
        "incoming",
        "missing",
    ] {
        let mut a = transfer();
        if let Action::TripleCrown {
            request:
                Equipment::Transfer {
                    from,
                    to,
                    owner_consent,
                    outgoing_consent,
                    incoming_consent,
                    ..
                },
        } = &mut a
        {
            match invalid {
                "from" => *from = "being.flynt.tross".into(),
                "unqualified" => *to = "being.flynt.tross".into(),
                "owner" => owner_consent.signer = "person.two".into(),
                "outgoing" => outgoing_consent.signer = "person.two".into(),
                "incoming" => incoming_consent.signer = "person.one".into(),
                "missing" => {}
                _ => unreachable!(),
            }
        }
        let mut cmd = command(&g, date(0, 0), "execution.reviewer", a);
        if invalid == "missing" {
            cmd.evidence.remove("transfer.incoming");
        }
        let before = g.clone();
        assert!(g.execute(cmd).is_err());
        assert_eq!(g, before);
    }
    let cmd = command(&g, date(0, 0), "execution.reviewer", transfer());
    g.execute(cmd.clone()).unwrap();
    assert_eq!(g.execute(cmd), Ok(InterfaceApplyOutcome::Idempotent));
    let c = g.triple_crown("crown.one").unwrap();
    assert_eq!(c.custodian.as_deref(), Some("person.two"));
    assert_eq!(c.registration.owner, "owner.one");
    reject(
        &mut g,
        date(0, 0),
        "person.one",
        act(Equipment::Release {
            item: "crown.one".into(),
        }),
    );
    apply(
        &mut g,
        date(0, 0),
        "owner.one",
        act(Equipment::Release {
            item: "crown.one".into(),
        }),
    );
    let bytes = g.encode().unwrap();
    assert_eq!(Governance::decode(&bytes).unwrap().encode().unwrap(), bytes);
}
