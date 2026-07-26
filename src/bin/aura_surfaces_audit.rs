use hollow_grove::world::aura_basin::canonical_aura_basin;
use hollow_grove::world::aura_beach::canonical_aura_beach;
use hollow_grove::world::aura_field::canonical_aura_field;
use hollow_grove::world::interior_surface::InteriorSurfaceId;

fn main() {
    let field = canonical_aura_field().expect("canonical Aura Field");
    let beach = canonical_aura_beach().expect("canonical Aura Beach");
    let basin = canonical_aura_basin().expect("canonical Aura Basin");

    println!("interior surfaces={}", InteriorSurfaceId::ALL.len());
    println!(
        "{} / house={:?} / routes={} / facilities={} / map={}",
        field.id.display_name(),
        field.dominant_house,
        field.access_routes.len(),
        field.facilities.len(),
        field.map_id
    );
    println!(
        "{} / house={:?} / routes={} / facilities={} / map={}",
        beach.id.display_name(),
        beach.dominant_house,
        beach.access_routes.len(),
        beach.facilities.len(),
        beach.map_id
    );
    println!(
        "{} / house={:?} / routes={} / facilities={} / map={}",
        basin.id.display_name(),
        basin.dominant_house,
        basin.access_routes.len(),
        basin.facilities.len(),
        basin.map_id
    );
}
