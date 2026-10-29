extends SceneTree

const SCENE := preload("res://scenes/retro_overworld.tscn")


func _init() -> void:
	call_deferred("_run")


func _run() -> void:
	var directory := OS.get_environment("BOARDWALK_VIEW_OUTPUT_DIR")
	if directory.is_empty():
		push_error("Set BOARDWALK_VIEW_OUTPUT_DIR to the Rust boardwalk_territory test output directory")
		quit(1)
		return
	var game := SCENE.instantiate()
	root.add_child(game)
	await process_frame
	game.set_process(false)
	for level in ["upper", "lower"]:
		var fixture_path := directory.path_join(level + ".json")
		assert(FileAccess.file_exists(fixture_path))
		var fixture: Dictionary = JSON.parse_string(FileAccess.get_file_as_string(fixture_path))
		game._apply_view(fixture)
		game.display_position = game.target_position
		game.status_message = game._map_label()
		game.queue_redraw()
		game._refresh_screen_semantics()
		assert(game._map_display_name() == level.capitalize() + " Boardwalk")
		assert(game.boardwalk_territory.get("formal_neutrality"))
		assert(game.boardwalk_territory.get("sovereign_faction") == null)
		assert(game.boardwalk_territory.get("public_access"))
		assert(game.boardwalk_territory.get("local_presence").size() == 3)
		var context: String = game.get_meta("hollow_grove_screen_semantics").get("context")
		assert("neither faction is sovereign" in context)
		assert("Trooners" in context and "Gooners" in context)
		assert("Passage to" in context and "Press B" in context)
		var selected: Dictionary = game.physical_exits[game.selected_exit_index]
		assert(int(selected.get("exit_y")) == int(fixture.overworld.player.y))
		if "--capture-boardwalk" in OS.get_cmdline_user_args():
			await process_frame
			await RenderingServer.frame_post_draw
			var capture := root.get_texture().get_image()
			assert(not capture.is_empty())
			assert(capture.save_png(directory.path_join(level + ".png")) == OK)
	# A legacy or non-Boardwalk view clears the local projection.
	game._apply_view({})
	assert(game.boardwalk_territory.is_empty())
	game.queue_free()
	await process_frame
	print("Boardwalk territory smoke: pass; runtime fixtures, stairs, neutrality and accessible context")
	quit(0)
