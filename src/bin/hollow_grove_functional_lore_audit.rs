//! Deterministic human/JSON witness for the four-House functional-lore layer.

use std::path::PathBuf;

use hollow_grove::constitutional::CausalPosition;
use hollow_grove::world::lived_lore::FunctionalLoreCatalog;
use hollow_grove::world::session::WorldSession;

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("functional-lore audit failed: {error}");
        std::process::exit(1);
    }
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut world_root = PathBuf::from(".");
    let mut json = false;
    let mut arguments = arguments.into_iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--world-root" => {
                world_root = PathBuf::from(
                    arguments
                        .next()
                        .ok_or("--world-root requires a directory")?,
                );
            }
            "--json" => json = true,
            "--help" | "-h" => {
                println!("Usage: hollow_grove_functional_lore_audit [--world-root DIR] [--json]");
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {argument}").into()),
        }
    }
    let world = WorldSession::load_or_canonical_at(&world_root)?;
    let catalog = FunctionalLoreCatalog::instantiate(&world, CausalPosition::new(1))?;
    if json {
        println!("{}", catalog.encode()?);
    } else {
        print!("{}", catalog.witness_markdown());
    }
    Ok(())
}
