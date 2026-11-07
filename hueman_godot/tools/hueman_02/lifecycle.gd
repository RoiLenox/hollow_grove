extends SceneTree
const ENTRY = preload("res://tools/hueman_02/entry.tscn")
var checks := 0
var failures := 0
var phase := OS.get_environment("HUEMAN02_PHASE")
var snapshot_path := OS.get_environment("HUEMAN02_SNAPSHOT")
var capture_dir := OS.get_environment("HUEMAN02_CAPTURE_DIR")

func _initialize() -> void:
	_run.call_deferred()

func _check(condition: bool, message: String) -> void:
	checks += 1
	if not condition:
		failures += 1
		push_error("HUEMAN-02: " + message)
		quit(1)
		assert(condition, message)

func _until(condition: Callable, message: String) -> void:
	var deadline := Time.get_ticks_msec() + 15000
	while not condition.call() and Time.get_ticks_msec() < deadline:
		await process_frame
	_check(bool(condition.call()), message)

func _key(game: Node, key: Key) -> void:
	if key in [KEY_Y, KEY_W] and not game.workshop_menu_open:
		while not game.dialogue_pages.is_empty():
			await _key(game, KEY_ENTER)
	await _until(func(): return not (game.movement_pending or game.interaction_pending or game.case_pending or game.synthesis_pending or game.sync_pending or game.seam_pending) and game.animation_elapsed >= game.STEP_DURATION_SECONDS, "ready for keyboard input")
	var event := InputEventKey.new()
	event.keycode = key
	event.pressed = true
	game._input(event)
	await process_frame
	await _until(func(): return not (game.movement_pending or game.interaction_pending or game.case_pending or game.synthesis_pending or game.sync_pending or game.seam_pending), "keyboard action completed")

func _pixels(game: Node) -> String:
	var digest := HashingContext.new()
	digest.start(HashingContext.HASH_SHA256)
	digest.update(game.custom_hueman_texture.get_image().get_data())
	return digest.finish().hex_encode()

func _snapshot(game: Node) -> Dictionary:
	return {"profile":game.hueman_profile, "overworld":game.overworld, "mission":game.initiation,
		"repair":game.survey_frame, "synthesis":game.synthesis_pilot, "pixels":_pixels(game)}

func _run() -> void:
	var entry = ENTRY.instantiate()
	root.add_child(entry)
	current_scene = entry
	if phase == "create":
		await _until(func(): return entry.flow == "ready", "creator connected to authority")
		entry.skin_buttons[5].pressed.emit()
		entry.face_choice.item_selected.emit(1)
		entry.hair_style_choice.item_selected.emit(4)
		entry.hair_color_choice.item_selected.emit(2)
		entry.enter_button.pressed.emit()
	await _until(func(): return current_scene != null and current_scene.has_method("_send_save"), "creator/resume transitions to existing overworld")
	var game = current_scene
	await _until(func(): return not game.hueman_profile.is_empty() and game.runtime_client.is_runtime_connected(), "authoritative customized character loaded")
	_check(game.custom_hueman_texture != null, "customized avatar rendered")
	_check(game.initiation.title == "The Initiation", "exact mission title")
	_check(game.overworld.map_id == "aura-way.design-corridor", "existing Stonebend Aura Way map")
	if phase != "create":
		var previous: Dictionary = JSON.parse_string(FileAccess.get_file_as_string(snapshot_path))
		_check(_snapshot(game) == previous, "identity, appearance, location, mission, repair and Form survive full restart")
	if phase == "create":
		_check(game.hueman_profile.appearance == {"skin":"deep_neutral","face":"bright","hair":{"style":"long","color":"auburn"}}, "creator controls become authoritative appearance")
		_check(game.synthesis_pilot.form == "Hueman" and not game.initiation.complete, "no advanced Form or mission granted")
		for step in range(10):
			await _key(game, KEY_W)
		_check(game.overworld.player.y == 5, "existing movement reaches witness")
		await _key(game, KEY_ENTER)
		_check(game.initiation.objectives[2].complete, "interaction objective recorded")
		# Existing dialogue and workshop controls; no test-only gameplay intents.
		game._clear_dialogue()
		await _key(game, KEY_O)
		_check(game.workshop_menu_open, "existing repair case opens")
		await _key(game, KEY_ENTER)
		_check(game.survey_frame.phase == "NeedRecorded", "existing ReceiveNeed action")
		_check(not game.initiation.complete and "Recover" in game.initiation.objective, "partial objective ready for resume")
		await _key(game, KEY_O)
	elif phase == "recover":
		await _key(game, KEY_O)
		var actions: Array = game.survey_frame.available_actions
		game.workshop_action_index = actions.find("RecoverParts")
		_check(game.workshop_action_index >= 0, "recovery action offered after restart")
		await _key(game, KEY_ENTER)
		_check(game.survey_frame.object_revision == 1 and game.initiation.complete, "existing recovery completes introduction")
		await _key(game, KEY_O)
		for step in range(7):
			await _key(game, KEY_Y)
		_check(game.synthesis_pilot.form == "Gargoyle", "explicit pilot actions reach existing Gargoyle Form")
	elif phase == "return":
		_check(game.synthesis_pilot.form == "Gargoyle", "transformed save resumes correctly")
		await _key(game, KEY_Y)
		_check(game.synthesis_pilot.form == "WingedGargoyle", "existing cleared pilot Form")
		await _key(game, KEY_Y)
		_check(game.synthesis_pilot.form == "Hueman", "return restores base Hueman")
	elif phase == "final":
		_check(game.initiation.complete and game.synthesis_pilot.completed, "mission and earned pilot capability retained")
		_check(game.synthesis_pilot.form == "Hueman", "resumed as the same base Hueman")
	else:
		_check(false, "known lifecycle phase")
	game._refresh_screen_semantics()
	_check("The Initiation" in str(game.get_meta("hollow_grove_screen_semantics")), "mission exposed in accessibility projection")
	if not capture_dir.is_empty():
		await RenderingServer.frame_post_draw
		_check(root.get_texture().get_image().save_png(capture_dir.path_join(phase + ".png")) == OK, "gameplay capture written")
	var output := FileAccess.open(snapshot_path, FileAccess.WRITE)
	output.store_string(JSON.stringify(_snapshot(game)))
	output.close()
	await _key(game, KEY_F5)
	_check(game.status_message.begins_with("GAME SAVED"), "save acknowledged by authority")
	if failures > 0:
		quit(1)
		return
	print("HUEMAN-02 PASS ", phase, " (", checks, " checks)")
	# Exercise the same save-and-exit path as F10/window close. The process must
	# terminate itself only after the second save receives a successful response.
	game._request_save_and_exit()
