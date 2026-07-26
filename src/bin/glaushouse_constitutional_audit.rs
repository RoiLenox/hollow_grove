use std::collections::BTreeSet;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::{glaushouse, house_institutions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    glaushouse::validate_principal_authorities()?;
    let catalog = house_institutions::canonical_house_institutions();
    catalog
        .validate()
        .map_err(|error| format!("neutral institution catalog failed: {error:?}"))?;

    let institutions = catalog
        .institutions
        .iter()
        .filter(|entry| entry.house == Some(House::Glaushouse))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if institutions
        != BTreeSet::from([
            "institution.glaushouse.medical-civilization",
            "institution.glaushouse.glauspitals",
            "institution.glaushouse.chromacord",
            "institution.glaushouse.nightingales",
        ])
    {
        return Err("Glaüshouse institution roster drifted".into());
    }

    let offices = catalog
        .offices
        .iter()
        .filter(|entry| entry.house == Some(House::Glaushouse))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    if offices != BTreeSet::from(["office.glaushouse.prima-donna"]) {
        return Err("Glaüshouse office roster drifted".into());
    }
    if catalog.roles.iter().any(|role| role.name == "Prima Donna") {
        return Err("a constitutional office was flattened into a role".into());
    }
    let clinical_ranks = catalog
        .roles
        .iter()
        .filter(|role| role.institution == glaushouse::nightingales_id())
        .map(|role| role.id.as_str())
        .collect::<BTreeSet<_>>();
    if clinical_ranks
        != BTreeSet::from([
            "role.glaushouse.nightingale",
            "role.glaushouse.matron",
            "role.glaushouse.marshal",
            "role.glaushouse.persephone",
        ])
    {
        return Err("Glaüshouse clinical ladder drifted".into());
    }

    println!("Glaüshouse Constitutional Audit: pass");
    println!("source: {}", glaushouse::GLAUSHOUSE_CONSTITUTION_SOURCE);
    println!("governing verb: {}", glaushouse::GLAUSHOUSE_GOVERNING_VERB);
    println!(
        "signature offense: {}",
        glaushouse::GLAUSHOUSE_SIGNATURE_OFFENSE
    );
    println!("singular highest clinical office: Prima Donna / Doctor Ratchet");
    println!("multiple balanced rank: Persephone / Nurse House is one holder");
    println!("equal branches: Matron / Marshal");
    println!("universal clinical foundation: Nightingale");
    println!("clinical facilities: Glauspitals");
    println!("clinical record institution: Chromacord");
    println!("clinical ladder cardinality and placements: exact");
    println!("consent inferred from silence, custody, dependence, recognition: false");
    println!("Hollowing consent implies Synthesis consent: false");
    println!("transformation creates Title or office: false");
    println!("clinical custody becomes ownership: false");
    println!("legacy state creates consent, clearance, privilege, or office: false");
    println!("recursion kernel dependency: none");
    Ok(())
}
