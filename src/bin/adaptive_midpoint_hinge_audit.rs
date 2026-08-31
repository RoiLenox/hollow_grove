use hollow_grove::NormalizedCoordinate;
use hollow_grove::world::adaptive_midpoint_hinges::{
    CanonicalMidpointHinge, audit_adaptive_topology_morph, build_adaptive_midpoint_hinge_witness,
    canonical_adaptive_midpoint_hinges, canonical_adaptive_morph_contract_json,
};
use hollow_grove::world::spatial_inheritance::HollowGroveSpatialScope;
use hollow_grove::world::spatial_topology_v2::canonical_hollow_grove_spatial_topology_v2;

pub(crate) fn tool_main() -> Result<(), Box<dyn std::error::Error>> {
    let structure = canonical_adaptive_midpoint_hinges()?;
    let map = canonical_hollow_grove_spatial_topology_v2()?;
    print!("{}", build_adaptive_midpoint_hinge_witness(&structure)?);
    println!("\n## Adaptive Pivots\n");
    for (label, rectangularity) in [
        ("circle", NormalizedCoordinate::zero()),
        ("halfway", NormalizedCoordinate::midpoint()),
        ("rectangle", NormalizedCoordinate::one()),
    ] {
        println!("- {label}:");
        for id in CanonicalMidpointHinge::ALL {
            let pose =
                structure.resolve(&map, HollowGroveSpatialScope::World, id, rectangularity)?;
            println!(
                "  - {}: pivot=({}, {}), inner=({}, {}), outer=({}, {})",
                id.display_name(),
                pose.local.adaptive_pivot.x.parts(),
                pose.local.adaptive_pivot.y.parts(),
                pose.local.slots[0].adaptive_midpoint.x.parts(),
                pose.local.slots[0].adaptive_midpoint.y.parts(),
                pose.local.slots[1].adaptive_midpoint.x.parts(),
                pose.local.slots[1].adaptive_midpoint.y.parts(),
            );
        }
    }
    let audit = audit_adaptive_topology_morph()?;
    println!(
        "\n## Bottom-Up Morph Lock\n\n\
         - sample_count: {}\n\
         - fixed_anchor_count: {}\n\
         - locked_region_count: {}\n\
         - endpoints_remain_fixed: {}\n\
         - slots_remain_isolated: {}\n\
         - canonical_circle_is_preserved: {}\n\
         - rectangular_symmetry_is_exact: {}\n\
         - central_spine_remains_fixed: {}",
        audit.sample_count,
        audit.fixed_anchor_count,
        audit.locked_region_count,
        audit.endpoints_remain_fixed,
        audit.slots_remain_isolated,
        audit.canonical_circle_is_preserved,
        audit.rectangular_symmetry_is_exact,
        audit.central_spine_remains_fixed,
    );
    println!(
        "\n## Presentation Contract\n\n{}",
        canonical_adaptive_morph_contract_json()?
    );
    Ok(())
}
