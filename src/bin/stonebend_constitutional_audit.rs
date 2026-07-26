use std::collections::BTreeSet;

use hollow_grove::world::{house_institutions, stonebend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    stonebend::validate_principal_authorities()?;
    stonebend::foundation::validate_foundation()?;
    stonebend::second_pass::validate_three_gate_topology(
        &stonebend::second_pass::canonical_stonebend_gates(),
    )?;
    if !stonebend::third_pass::TitleLifecycleStage::ALL
        .windows(2)
        .all(|stages| stages[0].semantic_order() < stages[1].semantic_order())
    {
        return Err("Stonebend Title lifecycle semantic order drifted".into());
    }
    stonebend::third_pass::ProliteriateContinuityPolicy::default().validate()?;
    let catalog = house_institutions::canonical_house_institutions();
    catalog
        .validate()
        .map_err(|error| format!("neutral institution catalog failed: {error:?}"))?;

    let institutions = catalog
        .institutions
        .iter()
        .filter(|entry| entry.house == Some(hollow_grove::hollow_grove_contract::House::Stonebend))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    let expected_institutions = BTreeSet::from([
        "institution.stonebend.constitution",
        "institution.stonebend.proliteriate",
        "institution.stonebend.freemason",
    ]);
    if institutions != expected_institutions {
        return Err("Stonebend institution roster drifted".into());
    }

    let offices = catalog
        .offices
        .iter()
        .filter(|entry| entry.house == Some(hollow_grove::hollow_grove_contract::House::Stonebend))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if offices
        != BTreeSet::from([
            "office.stonebend.hypergiant",
            "office.stonebend.high-freemason",
        ])
    {
        return Err("Stonebend office roster drifted".into());
    }

    if catalog.roles.iter().any(|role| {
        matches!(
            role.name.as_str(),
            "Hypergiant" | "Proliteriate" | "Freemason"
        )
    }) {
        return Err("a principal authority was flattened into an ordinary role".into());
    }

    println!("Stonebend Constitutional Audit: pass");
    println!("source: {}", stonebend::STONEBEND_CONSTITUTION_SOURCE);
    println!("governing verb: {}", stonebend::STONEBEND_GOVERNING_VERB);
    println!(
        "signature offense: {}",
        stonebend::STONEBEND_SIGNATURE_OFFENSE
    );
    println!("constitutional people: Geralds");
    println!("singular highest office: Hypergiant");
    println!("distributed Yield network: Proliteriate");
    println!("execution institution: Freemason");
    println!("singular institutional head: High Freemason");
    println!("principal authority placements: exact");
    println!("neutral institution projection: exact");
    println!("transformation creates office: false");
    println!("recognition substitutes for Title: false");
    println!("clearance substitutes for consent: false");
    println!("legacy progression creates authority: false");
    println!("Mt. Aura vertical pole: Aether");
    println!("Riptide vertical pole: Bathos");
    println!("Aether–Current continuum: one lineage");
    println!("Aura Way route kind: standard institutional");
    println!("Stonebend recognition declares perfection: false");
    println!("material Hollowing preserves essential fractions: required");
    println!("Aether batch provenance: source Current retained");
    println!("stone refraction creates Aether: false");
    println!("final House-to-stone assignment: none");
    println!("ordinary Aura manifestation requires melting: false");
    println!("principal Stonebend gates: 3");
    println!("gate facings: Flynt, Central Junction, Sandmanor");
    println!("Central Junction modeled as House: false");
    println!("sovereign Title: Diamond");
    println!("Diamond bearer office: Hypergiant");
    println!("Diamond vacancy supported: true");
    println!("Freemason constitutional dimension: Claim");
    println!("Proliteriate constitutional dimension: Yield");
    println!("permanent Spartacus office: false");
    println!("removal concurrence: two distinct powers");
    println!("Lazerhorn required for Diamond accession: true");
    println!("Title recognition distinct from activation: true");
    println!("Title maintenance distinct from renewal: true");
    println!("Diamond vacancy creates Regent: false");
    println!("replacement Freemason review: independent");
    println!("permanent Proliteriate speaker: false");
    println!("recursion kernel dependency: none");
    Ok(())
}
