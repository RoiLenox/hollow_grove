use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use hollow_grove::constitutional::{
    CANONICAL_VISUAL_COLOR_PALETTE_PATH, OverlayFamily, RenderLayer,
    canonical_visual_color_constitution, validate_visual_color_constitution,
};
use hollow_grove::hollow_grove_contract::House;

const CANONICAL_COLORS: [(&str, &str, &str); 16] = [
    ("universal.oxford_blue", "Oxford Blue", "#0A1024"),
    ("universal.midnight_blue", "Midnight Blue", "#121B38"),
    ("universal.space_cadet", "Space Cadet", "#1C2A52"),
    ("stonebend.prussian_blue", "Prussian Blue", "#163A5F"),
    ("stonebend.lapis_lazuli", "Lapis Lazuli", "#2F6FA3"),
    (
        "stonebend.air_superiority_blue",
        "Air Superiority Blue",
        "#79B7D8",
    ),
    ("sandmanor.wine", "Wine", "#5A1F2A"),
    ("sandmanor.redwood", "Redwood", "#A6404F"),
    ("sandmanor.blush", "Blush", "#E07A86"),
    ("glaushouse.brunswick_green", "Brunswick Green", "#163C35"),
    ("glaushouse.viridian", "Viridian", "#2F7A62"),
    ("glaushouse.eton_blue", "Eton Blue", "#78C9A4"),
    ("flynt.rich_black_blue", "Rich Black Blue", "#101525"),
    ("flynt.gunmetal", "Gunmetal", "#242C42"),
    ("flynt.powder_blue", "Powder Blue", "#A7B6D9"),
    ("flynt.tropical_indigo", "Tropical Indigo", "#7F8BC4"),
];

#[test]
fn canonical_palette_locks_every_ratified_color_and_semantic_identity() {
    let constitution = canonical_visual_color_constitution();
    validate_visual_color_constitution(constitution)
        .expect("canonical visual color constitution must validate");

    assert_eq!(constitution.colors.len(), CANONICAL_COLORS.len());
    for (id, name, hex) in CANONICAL_COLORS {
        let color = constitution
            .color(id)
            .unwrap_or_else(|| panic!("missing canonical color {id}"));
        assert_eq!(color.name, name);
        assert_eq!(color.hex, hex);
        assert!(!color.semantic_identity.is_empty());
    }
    assert_eq!(
        constitution
            .colors
            .iter()
            .map(|color| color.semantic_identity.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        CANONICAL_COLORS.len()
    );

    let expected_houses = [
        (
            House::Stonebend,
            "stonebend.prussian_blue",
            "stonebend.lapis_lazuli",
            "stonebend.air_superiority_blue",
            [
                "Body",
                "Structure",
                "Architecture",
                "Title",
                "Naming",
                "Mercury",
            ],
        ),
        (
            House::Sandmanor,
            "sandmanor.wine",
            "sandmanor.redwood",
            "sandmanor.blush",
            ["Soul", "Proof", "Design", "Interior", "Exterior", "Crystal"],
        ),
        (
            House::Glaushouse,
            "glaushouse.brunswick_green",
            "glaushouse.viridian",
            "glaushouse.eton_blue",
            ["Mind", "Medicine", "Aura", "Synthesis", "Healing", "Jade"],
        ),
        (
            House::Flynt,
            "flynt.rich_black_blue",
            "flynt.gunmetal",
            "flynt.powder_blue",
            [
                "Spirit",
                "Recognition",
                "Engineering",
                "Opal",
                "Motion",
                "Authority",
            ],
        ),
    ];
    for (house, dark, primary, highlight, meanings) in expected_houses {
        let palette = constitution
            .house_palette(house)
            .unwrap_or_else(|| panic!("missing {house:?} palette"));
        assert_eq!(palette.dark, dark);
        assert_eq!(palette.primary, primary);
        assert_eq!(palette.highlight, highlight);
        assert_eq!(
            palette.meanings,
            meanings.map(String::from),
            "{house:?} canonical meanings changed"
        );
    }
}

#[test]
fn every_house_is_renderer_accessible_only_after_the_universal_foundation() {
    let constitution = canonical_visual_color_constitution();
    for house in [
        House::Stonebend,
        House::Sandmanor,
        House::Glaushouse,
        House::Flynt,
    ] {
        let palette = constitution
            .renderer_palette(house)
            .unwrap_or_else(|error| panic!("{house:?} palette must resolve: {error}"));
        let layers = palette.ordered_base_layers();
        let layer_kinds = layers.iter().map(|layer| layer.layer).collect::<Vec<_>>();
        assert_eq!(
            layer_kinds,
            [
                RenderLayer::UniversalOutline,
                RenderLayer::UniversalShadow,
                RenderLayer::UniversalShadow,
                RenderLayer::HouseDark,
                RenderLayer::HousePrimary,
                RenderLayer::HouseHighlight,
            ]
        );
        assert!(layers.iter().all(|layer| {
            !layer
                .color
                .rgb()
                .expect("validated renderer color")
                .is_pure_black()
        }));
        assert!(
            palette
                .resolve_overlay(OverlayFamily::Aura, palette.house_highlight.id.as_str())
                .is_ok()
        );
    }
}

#[test]
fn flynt_is_blue_black_and_never_true_black() {
    let palette = canonical_visual_color_constitution()
        .renderer_palette(House::Flynt)
        .expect("Flynt palette must resolve");
    assert_eq!(palette.house_dark.hex, "#101525");
    assert!(
        !palette
            .house_dark
            .rgb()
            .expect("Flynt dark")
            .is_pure_black()
    );
    assert_eq!(palette.semantic_overlays.len(), 1);
    assert_eq!(palette.semantic_overlays[0].identity, "authority");
}

#[test]
fn godot_consumes_semantic_identities_without_defining_local_colors() {
    let source = include_str!("../hueman_godot/scripts/hueman_screen_shell.gd");
    assert!(source.contains(CANONICAL_VISUAL_COLOR_PALETTE_PATH));
    assert!(source.contains("_constitutional_color("));
    assert!(source.contains("colors_by_semantic_identity"));
    assert!(!source.contains("Color(\""));
    assert!(!source.contains("Color8("));
    assert!(!source.lines().any(has_numeric_color_constructor));
    assert!(quoted_hex_literals(source).is_empty());
}

#[test]
fn tracked_presentation_sources_do_not_contain_a_second_palette() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut presentation_files = Vec::new();
    collect_presentation_files(&root.join("hueman_godot"), &mut presentation_files);
    presentation_files.push(root.join("src/bin/current_synthesis_tui.rs"));
    presentation_files.push(root.join("export-aseprite.sh"));

    for path in presentation_files {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let literals = quoted_hex_literals(&source);
        assert!(
            literals.is_empty(),
            "presentation file {} defines local RGB literal(s): {literals:?}",
            path.display()
        );
        assert!(
            !source.lines().any(has_numeric_color_constructor),
            "presentation file {} defines a numeric Godot Color",
            path.display()
        );
    }
}

#[test]
fn sprite_export_derives_an_ephemeral_palette_from_the_constitution() {
    let source = include_str!("../export-aseprite.sh");
    assert!(source.contains("--bin visual_color_constitution"));
    assert!(source.contains("-- aseprite-gpl"));
    assert!(source.contains("--palette \"$PALETTE_FILE\""));
    assert!(source.contains("mktemp"));
    assert!(quoted_hex_literals(source).is_empty());
}

#[test]
fn visual_identity_domain_does_not_depend_on_the_recursion_kernel() {
    let source = include_str!("../src/constitutional/visual_identity.rs");
    assert!(!source.contains("hollow_grove_kernel"));
    assert!(!source.contains("KernelPass"));
    assert!(!source.contains("run_kernel_cycle"));
}

fn has_numeric_color_constructor(line: &str) -> bool {
    let mut remainder = line;
    while let Some(position) = remainder.find("Color(") {
        remainder = &remainder[position + "Color(".len()..];
        let first = remainder.trim_start().as_bytes().first().copied();
        if first.is_some_and(|byte| byte.is_ascii_digit() || byte == b'.') {
            return true;
        }
    }
    false
}

fn quoted_hex_literals(source: &str) -> Vec<&str> {
    source
        .split('"')
        .skip(1)
        .step_by(2)
        .filter(|value| {
            let digits = value.strip_prefix('#').unwrap_or(value);
            matches!(digits.len(), 6 | 8) && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
        .collect()
}

fn collect_presentation_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("presentation directory entry must be readable");
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|name| name.to_str()) != Some(".godot") {
                collect_presentation_files(&path, files);
            }
            continue;
        }
        if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some(
                "gd" | "tscn"
                    | "tres"
                    | "res"
                    | "theme"
                    | "svg"
                    | "gdshader"
                    | "gdshaderinc"
                    | "shader"
            )
        ) {
            files.push(path);
        }
    }
}
