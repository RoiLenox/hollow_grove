use std::process::ExitCode;

use hollow_grove::composition::{PhysicalPosition, RelativePolarity};
use hollow_grove::world::world_point::{
    CENTRAL_JUNCTION_REGION_ID, DARK_AURA_REGION_ID, LIGHT_AURA_REGION_ID, WORLD_NEGATIVE_POLE_ID,
    WORLD_POINT_ID, WORLD_POSITIVE_POLE_ID,
};
use hollow_grove::world::world_point_archive::{
    WORLD_POINT_ARCHIVE_VERSION, decode_world_point_archive, encode_world_point_archive,
};
use hollow_grove::world::world_point_fixture::canonical_world_point_archive_fixture;

fn main() -> ExitCode {
    match audit() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("FAIL hollow-grove-world-point-audit: {error}");
            ExitCode::FAILURE
        }
    }
}

fn require(condition: bool, label: &str) -> Result<(), String> {
    if condition {
        println!("PASS {label}");
        Ok(())
    } else {
        Err(label.into())
    }
}

fn audit() -> Result<(), String> {
    let fixture = canonical_world_point_archive_fixture();
    let bytes = encode_world_point_archive(&fixture).map_err(|error| error.to_string())?;
    let decoded = decode_world_point_archive(&bytes).map_err(|error| error.to_string())?;
    let replayed = decode_world_point_archive(&bytes).map_err(|error| error.to_string())?;
    let binding = &decoded.state.binding;
    let point = &binding.point;

    require(
        decoded.archive_version == WORLD_POINT_ARCHIVE_VERSION
            && point.point_id.as_str() == WORLD_POINT_ID
            && point.scale.as_str() == "scale.world",
        &format!(
            "world Point identity={} scale={} archive-version={}",
            point.point_id, point.scale, decoded.archive_version
        ),
    )?;
    require(
        point.positive_pole_id.as_str() == WORLD_POSITIVE_POLE_ID
            && point.negative_pole_id.as_str() == WORLD_NEGATIVE_POLE_ID
            && point.positive_pole_id != point.negative_pole_id,
        "positive and negative pole identities are stable and distinct",
    )?;
    require(
        point.orientation == decoded.state.source_point.orientation
            && point.center == decoded.state.source_point.center
            && point.positive_pole_id == decoded.state.source_point.positive_pole_id
            && point.negative_pole_id == decoded.state.source_point.negative_pole_id,
        "lawful scaling preserved center, axis orientation, and both poles",
    )?;
    require(
        binding.light_aura_region_id.as_str() == LIGHT_AURA_REGION_ID
            && binding.field.positive_region_id == binding.light_aura_region_id,
        "positive world region is Light Aura",
    )?;
    require(
        binding.central_junction_region_id.as_str() == CENTRAL_JUNCTION_REGION_ID
            && binding.field.center_region_id == binding.central_junction_region_id,
        "world Point center is Central Junction",
    )?;
    require(
        binding.dark_aura_region_id.as_str() == DARK_AURA_REGION_ID
            && binding.field.negative_region_id == binding.dark_aura_region_id,
        "negative world region is Dark Aura",
    )?;
    require(
        binding.classify(PhysicalPosition { x: 0, y: 1, z: 0 }) == RelativePolarity::Positive
            && binding.classify(PhysicalPosition::origin()) == RelativePolarity::Center
            && binding.classify(PhysicalPosition { x: 0, y: -1, z: 0 })
                == RelativePolarity::Negative,
        "axis projection classifies positive, center, and negative positions",
    )?;
    let inverted = &decoded.state.explicit_inversion_probe_result;
    require(
        inverted.point_id != point.point_id
            && inverted.positive_pole_id == point.negative_pole_id
            && inverted.negative_pole_id == point.positive_pole_id
            && point.point_id.as_str() == WORLD_POINT_ID,
        "inversion is explicit, separately identified, and did not mutate the world Point",
    )?;
    require(
        binding.lawfulness_requires_separate_determination
            && !binding.presentation_may_change_constitutional_polarity,
        "polarity and morality are distinct; presentation cannot alter orientation",
    )?;

    const KERNEL: &str = include_str!("../../hollow-grove-kernel/src/lib.rs");
    const ORIENTED: &str = include_str!("../../hollow-grove-kernel/src/oriented_point.rs");
    require(
        [
            "Central Junction",
            "Light Aura",
            "Dark Aura",
            "Stonebend",
            "Sandmanor",
            "Glaüshouse",
            "Flynt",
        ]
        .into_iter()
        .all(|name| !KERNEL.contains(name) && !ORIENTED.contains(name)),
        "Hollow Grove and House proper names are absent from universal kernel",
    )?;
    require(
        decoded.state == replayed.state
            && encode_world_point_archive(&decoded.payload).map_err(|error| error.to_string())?
                == bytes,
        "archive checksum is valid and deterministic replay succeeds",
    )?;
    println!("PASS archive checksum={}", decoded.checksum);
    Ok(())
}
