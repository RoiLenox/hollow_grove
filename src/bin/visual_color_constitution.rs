use std::io;

use hollow_grove::constitutional::{
    build_aseprite_gpl_palette, build_visual_color_palette_output,
    build_visual_color_validation_report, try_canonical_visual_color_constitution,
};

fn main() -> io::Result<()> {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("validate"));
    let output = match command.as_str() {
        "validate" => {
            try_canonical_visual_color_constitution().map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("visual color constitution failed: {error}"),
                )
            })?;
            build_visual_color_validation_report()
        }
        "show" => build_visual_color_palette_output(),
        "aseprite-gpl" => build_aseprite_gpl_palette(),
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown visual color command: {other}"),
            ));
        }
    };
    print!("{output}");
    Ok(())
}
