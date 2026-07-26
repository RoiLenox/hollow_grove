use hollow_grove::gameplay::WorldMapId;
use hollow_grove::gameplay::{
    DeepPressurePersonId, DeepPressureState, LivingWorldState, deep_pressure_functional_lore,
    scheduled_people_on_map,
};
use hollow_grove::world::extraction::{ExtractionSiteId, canonical_extraction_sites};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let definition = deep_pressure_functional_lore();
    definition.validate()?;
    let campaign = DeepPressureState::new();
    campaign.validate()?;
    let living = LivingWorldState::canonical()?;
    let extraction = canonical_extraction_sites()?;
    let dawn_riptide = scheduled_people_on_map(
        &living,
        WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig),
    );

    if !dawn_riptide
        .iter()
        .any(|person| person.person_id == DeepPressurePersonId::CorinWake)
    {
        return Err("Corin Wake is missing from the Dawn Riptide witness".into());
    }

    println!("Hollow Grove Deep Pressure campaign: pass");
    println!("stable identity: {}", definition.stable_identity);
    println!("phases: 7");
    println!("operational cases: {}", living.cases.len());
    println!("extraction sites: {}", extraction.len());
    println!("scheduled people: {}", living.people.len());
    println!(
        "required affected statements: {}",
        hollow_grove::gameplay::DeepPressureStatementId::REQUIRED.len()
    );
    println!("settlement endings: 4");
    println!("finite-Bond endings: 3");
    println!("protected-refusal endings: 1");
    Ok(())
}
