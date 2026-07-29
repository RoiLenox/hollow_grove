const RETRO_CLIENT: &str = include_str!("../hueman_godot/scripts/retro_overworld.gd");
const RETRO_SCENE: &str = include_str!("../hueman_godot/scenes/retro_overworld.tscn");
const GODOT_PROJECT: &str = include_str!("../hueman_godot/project.godot");

#[test]
fn retro_screen_keeps_pixel_graphics_and_semantics_in_one_projection() {
    for required in [
        "SCREEN_SEMANTICS_SCHEMA_VERSION := \"1.0.0\"",
        "hollow-grove.retro-screen-semantics",
        "_screen_semantics()",
        "_world_context_semantics()",
        "_active_guide_display_names()",
        "_draw_embodied_guides(",
        "guide_context = _dict_or_empty(view.get(\"guide_context\"))",
        "_dialogue_semantics()",
        "_menu_semantics()",
        "accessibility_name",
        "accessibility_description",
        "accessibility_live",
        "AccessibilityServer.LIVE_POLITE",
    ] {
        assert!(
            RETRO_CLIENT.contains(required),
            "retro screen semantic projection is missing {required}"
        );
    }
    assert!(
        RETRO_SCENE.contains("texture_filter = 1"),
        "the retro scene must inherit nearest-neighbor texture filtering"
    );
    assert!(
        GODOT_PROJECT.contains("window/stretch/mode=\"disabled\""),
        "Godot must not stretch the already integer-scaled custom canvas"
    );
    assert!(
        GODOT_PROJECT.contains("window/stretch/aspect=\"keep\""),
        "the project must preserve the logical playfield aspect ratio"
    );
    assert!(
        GODOT_PROJECT.contains("run/main_scene=\"res://scenes/retro_overworld.tscn\""),
        "the playable retro overworld must be the distributable default scene"
    );
}

#[test]
fn retro_screen_prevents_silent_text_and_motion_regressions() {
    for required in [
        "PIXEL_LINE_WIDTH := 36",
        "_wrap_pixel_text(",
        "next_pages.append_array",
        "\"/\":",
        "\"'\":",
        "\"*\":",
        "_pixel_character(",
        "_update_logical_transform(",
        "get_window().min_size",
        "accessibility_should_reduce_animation",
    ] {
        assert!(
            RETRO_CLIENT.contains(required),
            "retro screen hardening is missing {required}"
        );
    }
}

#[test]
fn screen_semantics_remain_non_authoritative() {
    let semantics_start = RETRO_CLIENT
        .find("func _screen_semantics()")
        .expect("semantic projection");
    let semantics_end = RETRO_CLIENT[semantics_start..]
        .find("func _load_visual_color_constitution()")
        .map(|offset| semantics_start + offset)
        .expect("semantic projection boundary");
    let semantics_source = &RETRO_CLIENT[semantics_start..semantics_end];
    for forbidden in [
        "FileAccess.open",
        "runtime_client.send_intent",
        "ConstitutionalRuntime",
        "HouseDecision",
        "BondFormation",
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "screen semantics crossed the presentation boundary with {forbidden}"
        );
    }
}
