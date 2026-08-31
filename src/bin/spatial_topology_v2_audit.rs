use hollow_grove::world::spatial_topology_v2::{
    build_spatial_topology_v2_witness, canonical_hollow_grove_spatial_topology_v2,
};

pub(crate) fn tool_main() -> Result<(), Box<dyn std::error::Error>> {
    let topology = canonical_hollow_grove_spatial_topology_v2()?;
    print!("{}", build_spatial_topology_v2_witness(&topology)?);
    Ok(())
}
