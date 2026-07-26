use hollow_grove::world::aura_field::{AuraFieldFacilityKind, canonical_aura_field};

fn main() {
    let field = canonical_aura_field().expect("canonical Aura Field contract");
    println!(
        "{} [{}] / map={}",
        field.id.display_name(),
        field.id.stable_id(),
        field.map_id
    );
    println!(
        "boundary={:?} / access={}",
        field.boundary,
        field
            .access_routes
            .iter()
            .map(|route| route.display_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let farm_count = field
        .facilities
        .iter()
        .filter(|facility| facility.kind == AuraFieldFacilityKind::AuraFarm)
        .count();
    println!(
        "one surface / {farm_count} Aura farms / {} total facilities",
        field.facilities.len()
    );
    for facility in &field.facilities {
        println!(
            "{} / {} / {}",
            facility.name,
            facility.kind.as_str(),
            facility.function
        );
    }
}
