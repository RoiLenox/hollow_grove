extends SceneTree

const RETRO_OVERWORLD_SCENE := preload("res://scenes/retro_overworld.tscn")
const SCREEN_SEMANTICS_META_KEY := "hollow_grove_screen_semantics"


func _init() -> void:
	call_deferred("_run")


func _run() -> void:
	var game := RETRO_OVERWORLD_SCENE.instantiate()
	root.add_child(game)
	await process_frame
	game.set_process(false)

	assert(game.texture_filter == CanvasItem.TEXTURE_FILTER_NEAREST)
	assert(game.get_window().title == "Hollow Grove")
	assert(game.get_window().min_size.x >= 160)
	assert(game.get_window().min_size.y >= 144)
	assert(game.get_node("ScreenContext").accessibility_live == AccessibilityServer.LIVE_OFF)
	assert(game.get_node("ScreenStatus").accessibility_live == AccessibilityServer.LIVE_POLITE)
	assert(game.get_node("ScreenDialogue").accessibility_live == AccessibilityServer.LIVE_POLITE)
	assert(game.get_node("ScreenMenu").accessibility_live == AccessibilityServer.LIVE_POLITE)
	assert(game.get_node("ScreenContext").get_theme_color("font_color").a == 0.0)
	game._update_logical_transform(Vector2(3840, 2160))
	assert(game.logical_scale == 15.0)
	assert(game.logical_origin == Vector2(720, 0))

	game.overworld = {
		"map_id": "glausbahn.refinement-span",
		"width": 20,
		"height": 18,
		"tile_size": 8,
		"tile_rows": [],
		"player": {"x": 7, "y": 9, "facing": "East"},
	}
	game.route_view = {
		"route_id": "glausbahn",
		"display_name": "Glausbahn",
		"geometry": "Round",
		"dominant_verb": "Refine",
	}
	game.guide_context = {
		"layer": "hollow_grove",
		"guides": [
			{
				"identity": "roi",
				"form": "hollow_grove_guide",
				"display_name": "Roi",
				"jurisdiction": "when",
			},
			{
				"identity": "roselina",
				"form": "hollow_grove_guide",
				"display_name": "Roselina",
				"jurisdiction": "where_what",
			},
		],
	}
	game.physical_exits = [{
		"exit_x": 18,
		"exit_y": 9,
		"destination_map_id": "current-seanad.deliberation-chamber",
		"destination_label": "Current Seanad",
	}]
	game.status_message = "READY"
	game.deep_pressure = {
		"present_people": [{
			"display_name": "Mara Venn",
			"role": "weather reader",
			"position": {"x": 5, "y": 6},
		}],
	}
	game._refresh_screen_semantics()
	var world_semantics: Dictionary = game.get_meta(SCREEN_SEMANTICS_META_KEY)
	assert(world_semantics.get("schema_version") == "1.0.0")
	assert(world_semantics.get("contract_id") == "hollow-grove.retro-screen-semantics")
	assert(world_semantics.get("mode") == "world")
	assert("Glausbahn" in str(world_semantics.get("context")))
	assert("tile x 7, y 9, facing East" in str(world_semantics.get("context")))
	assert("Current Seanad" in str(world_semantics.get("context")))
	assert("Roi and Roselina are embodied Guides" in str(world_semantics.get("context")))
	assert(game._active_guide_display_names() == ["Roi", "Roselina"])
	assert("Mara Venn, weather reader, at tile x 5, y 6" in str(world_semantics.get("context")))
	assert("Arrow keys" in str(world_semantics.get("controls")))

	var long_page := "THIS COMPLETE DIALOGUE LINE IS LONGER THAN THIRTY SIX PIXEL CHARACTERS"
	game._apply_interaction({
		"target_id": "interaction.test",
		"speaker": "Glaüshouse Witness",
		"pages": [long_page],
	})
	assert(game.dialogue_pages.size() > 1)
	for line_value in game.dialogue_pages:
		assert(str(line_value).length() <= 36)
	assert(" ".join(game.dialogue_pages) == long_page)
	game._refresh_screen_semantics()
	var dialogue_semantics: Dictionary = game.get_meta(SCREEN_SEMANTICS_META_KEY)
	assert(dialogue_semantics.get("mode") == "dialogue")
	assert("Glaüshouse Witness" in str(dialogue_semantics.get("dialogue")))
	assert(
		"Page 1 of %d" % game.dialogue_pages.size()
		in str(dialogue_semantics.get("dialogue"))
	)

	assert(game._pixel_character("Ü") == "U")
	assert(game.FONT_3X5.has("/"))
	assert(game.FONT_3X5.has("'"))
	assert(game.FONT_3X5.has("*"))

	game._clear_dialogue()
	game.party = {
		"hueman_continuity_id": "being-continuity.hueman",
		"selected_continuity_id": "being-continuity.hueman",
		"lead_continuity_id": "being-continuity.hueman",
		"members": [],
	}
	game.party_menu_open = true
	game._refresh_screen_semantics()
	var party_semantics: Dictionary = game.get_meta(SCREEN_SEMANTICS_META_KEY)
	assert(party_semantics.get("mode") == "party_menu")
	assert("Selected Hueman" in str(party_semantics.get("menu")))
	assert("P or Escape closes" in str(party_semantics.get("controls")))

	game.queue_free()
	quit()
