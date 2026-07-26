use hollow_grove::world::aura_basin::canonical_aura_basin;
use hollow_grove::world::aura_beach::canonical_aura_beach;
use hollow_grove::world::aura_field::canonical_aura_field;
use hollow_grove::world::geography::canonical_constitutional_geography;
use hollow_grove::world::route_network::{RouteGeometryClass, RouteNetwork};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geography = canonical_constitutional_geography()?;
    let network = RouteNetwork::canonical()?;
    println!("Hollow Grove route network: pass");
    println!(
        "Straight: {}",
        network
            .routes_by_geometry(RouteGeometryClass::Straight)
            .len()
    );
    println!(
        "Round: {}",
        network.routes_by_geometry(RouteGeometryClass::Round).len()
    );
    println!(
        "Sea ordeal: {}",
        network
            .routes_by_geometry(RouteGeometryClass::SeaOrdeal)
            .len()
    );
    for segment in network.segments() {
        let definition = geography
            .route(segment.route)
            .expect("network route exists in constitutional geography");
        println!(
            "{} | {} | {:?} -> {:?} | {} | {}",
            segment.route.display_name(),
            segment.geometry.as_str(),
            segment.endpoints[0],
            segment.endpoints[1],
            definition.verb.as_str(),
            definition.purpose,
        );
    }
    let field = canonical_aura_field()?;
    let beach = canonical_aura_beach()?;
    let basin = canonical_aura_basin()?;
    println!(
        "Interior overlays: {}, {}, {} / implemented and not counted as routes",
        field.id.display_name(),
        beach.id.display_name(),
        basin.id.display_name()
    );
    Ok(())
}
