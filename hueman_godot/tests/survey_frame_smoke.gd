extends SceneTree

const SCENE := preload("res://scenes/retro_overworld.tscn")


func _init() -> void:
	call_deferred("_run")


func _run() -> void:
	var game := SCENE.instantiate()
	root.add_child(game)
	await process_frame
	game.set_process(false)
	game.overworld = {"map_id": "aura-way.workshop", "player": {"x": 1, "y": 1}}
	game.survey_frame = {
		"object_revision": 2, "phase": "RigIsolated",
		"measurement_proven": false, "support_proven": false,
		"available_actions": ["RepairJoint", {"SupportResidentChoice": {"choice": "WaitSafely"}}],
		"missing_authorities": [], "recovery_required": false, "complete": false,
	}
	var open := InputEventKey.new()
	open.keycode = KEY_O
	open.pressed = true
	game._input(open)
	assert(game.workshop_menu_open)
	game._refresh_screen_semantics()
	var semantics: Dictionary = game.get_meta("hollow_grove_screen_semantics")
	assert(semantics.mode == "workshop_menu")
	assert("Tess Splice" in semantics.menu and "Ivo Mark" in semantics.menu)
	assert("measuring proof false, support proof false" in semantics.menu)
	assert("Player advice is not Mercy's decision" in semantics.menu)
	assert("O or Escape closes" in semantics.controls)
	var down := InputEventKey.new()
	down.keycode = KEY_DOWN
	down.pressed = true
	game._input(down)
	assert(game.workshop_action_index == 1)
	assert(game._workshop_action_label(game.survey_frame.available_actions[1]) == "SUPPORT A SAFE WAIT")
	game.survey_frame.recovery_required = true
	game._refresh_screen_semantics()
	assert("Withdrawal requires recovery" in game.get_meta("hollow_grove_screen_semantics").menu)
	game.case_pending = true
	game._input(open)
	assert(game.workshop_menu_open)
	game.case_pending = false
	game._input(open)
	assert(not game.workshop_menu_open)
	assert(game.survey_frame.object_revision == 2 and not game.survey_frame.support_proven)
	game.queue_free()
	print("Hollow Grove survey-frame projection smoke passed")
	quit()
