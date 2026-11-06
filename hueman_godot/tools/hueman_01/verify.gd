extends SceneTree
## Exercises the actual scene controls and saves across independent processes.
const Appearance = preload("res://tools/hueman_01/appearance.gd")
const Store = preload("res://tools/hueman_01/profile_store.gd")
const Art = preload("res://tools/hueman_01/avatar_art.gd")
const Scene = preload("res://tools/hueman_01/creator.tscn")
var failures := 0
var checks := 0
var directory := ""
var phase := ""
var capture := ""

func _initialize() -> void:
	for argument in OS.get_cmdline_user_args():
		if argument.begins_with("--profile-dir="):
			directory = argument.trim_prefix("--profile-dir=")
		if argument.begins_with("--phase="):
			phase = argument.trim_prefix("--phase=")
		if argument.begins_with("--capture="):
			capture = argument.trim_prefix("--capture=")
	_run.call_deferred()

func _check(condition: bool, message: String) -> void:
	checks += 1
	if not condition:
		failures += 1
		push_error("HUEMAN-01: " + message)

func _hash(image: Image) -> String:
	var context := HashingContext.new()
	context.start(HashingContext.HASH_SHA256)
	context.update(image.get_data())
	return context.finish().hex_encode()

func _write(path: String, value: String) -> void:
	var file := FileAccess.open(path, FileAccess.WRITE)
	_check(file != null, "fixture file opens")
	if file != null:
		file.store_string(value)
		file.close()

func _contracts() -> void:
	var base := Appearance.defaults()
	_check(Appearance.valid(base), "default appearance valid")
	for field in ["body", "height", "growth_index", "active_form", "house", "stats"]:
		var rejected := base.duplicate(true)
		rejected[field] = "forbidden"
		_check(not Appearance.valid(rejected), "reject " + field + " in appearance")
	_check(not Appearance.valid({"skin": [], "face": "calm", "hair": {}}), "reject wrong value types")
	var fixed_lower_body := Art.hueman(base).get_region(Rect2i(0, 24, 24, 8)).get_data()
	var distinct := {}
	for skin: Dictionary in Appearance.SKINS:
		for face: Dictionary in Appearance.FACES:
			for hair: Dictionary in Appearance.HAIR_STYLES:
				for color: Dictionary in Appearance.HAIR_COLORS:
					var look := {"skin": skin.id, "face": face.id, "hair": {"style": hair.id, "color": color.id}}
					var art := Art.hueman(look)
					_check(art.get_size() == Vector2i(24, 32), "fixed canvas for every appearance")
					_check(art.get_region(Rect2i(0, 24, 24, 8)).get_data() == fixed_lower_body, "cosmetics preserve body/legs")
					_check(art.get_pixel(12, 6).to_html(false) == skin.hex, "selected natural skin rendered")
					distinct[_hash(art)] = true
	# Shaved intentionally ignores the four retained hair colors: 6 * 4 * (1 + 4 * 4).
	_check(distinct.size() == 408, "every visible preset combination is distinct")
	var gremlin := Art.gremlin()
	_check(gremlin != null and gremlin.get_pixel(7, 2).to_html(false) == "79b7d8", "Gremlin uses exact v2.3 binding")
	_check(base == Appearance.defaults(), "Form rendering cannot mutate appearance")
	var store := Store.new(directory.path_join("contracts"))
	var id := Appearance.new_avatar_id()
	_check(store.load_profile().get("missing", false), "missing profile distinguished from invalid")
	_check(store.save_profile(id, base, 0).ok, "first save")
	var first_bytes := FileAccess.get_file_as_string(store.path())
	var edited := base.duplicate(true)
	edited.skin = "deep_neutral"
	_check(store.save_profile(id, edited, 1).ok, "next revision")
	var second_bytes := FileAccess.get_file_as_string(store.path())
	_check(FileAccess.get_file_as_string(store.path() + ".backup.json") == first_bytes, "previous valid save backed up")
	_check(not store.save_profile(id, base, 1).ok, "stale writer rejected")
	_check(not store.save_profile(Appearance.new_avatar_id(), base, 2).ok, "different identity cannot overwrite")
	_check(FileAccess.get_file_as_string(store.path()) == second_bytes, "conflicts preserve saved avatar")
	DirAccess.make_dir_absolute(store.path() + ".lock")
	_check(not store.save_profile(id, base, 2).ok, "concurrent/interrupted writer rejected")
	DirAccess.remove_absolute(store.path() + ".lock")
	var record: Dictionary = store.load_profile().profile
	var invalid: Array[String] = ["{broken", "x".repeat(Store.MAX_BYTES + 1)]
	for bad_version in [2, "1", true, null]:
		var bad := record.duplicate(true)
		bad.schema_version = bad_version
		invalid.append(JSON.stringify(bad))
	for bad_revision in [0, -1, 1.5, "2", true]:
		var bad := record.duplicate(true)
		bad.revision = bad_revision
		invalid.append(JSON.stringify(bad))
	var extra := record.duplicate(true)
	extra.active_form = "gremlin"
	invalid.append(JSON.stringify(extra))
	var unknown := record.duplicate(true)
	unknown.appearance.hair.style = "../../asset.png"
	invalid.append(JSON.stringify(unknown))
	for raw: String in invalid:
		_write(store.path(), raw)
		_check(not store.load_profile().ok, "invalid profile refused")
		_check(not store.save_profile(id, base, 2).ok, "invalid profile not overwritten")
		_check(FileAccess.get_file_as_string(store.path()) == raw, "invalid file preserved byte-for-byte")
	_write(store.path(), second_bytes)
	_check(store.load_profile().ok, "valid backup/manual recovery remains possible")
	var blocked_path := directory.path_join("not-a-directory")
	_write(blocked_path, "fixture")
	_check(not Store.new(blocked_path).save_profile(id, base, 0).ok, "save I/O failure reported")

func _run() -> void:
	if directory.is_empty() or phase not in ["contracts", "create", "reload"]:
		push_error("Pass an isolated --profile-dir and --phase=contracts|create|reload")
		quit(2)
		return
	DirAccess.make_dir_recursive_absolute(directory)
	if phase == "contracts":
		_contracts()
	else:
		await _scene_checks()
	if failures == 0:
		print("HUEMAN-01 PASS ", phase, " (", checks, " checks)")
	else:
		push_error("HUEMAN-01 FAIL: %s / %s" % [failures, checks])
	quit(0 if failures == 0 else 1)

func _scene_checks() -> void:
	var creator = Scene.instantiate()
	creator.profile_directory = directory
	root.add_child(creator)
	await process_frame
	await process_frame
	if DisplayServer.get_name() == "headless":
		await _layout_checks(creator)
	_check(creator.skin_buttons.size() == 6, "six skin controls")
	_check(creator.face_choice.item_count == 4 and creator.hair_style_choice.item_count == 5, "face/hair controls")
	var expected_path := directory.path_join("expected.json")
	if phase == "create":
		_check(creator.revision == 0, "isolated fresh avatar")
		creator.skin_buttons[5].pressed.emit()
		creator.face_choice.item_selected.emit(1)
		creator.hair_style_choice.item_selected.emit(4)
		creator.hair_color_choice.item_selected.emit(2)
		_check(creator.appearance == {"skin": "deep_neutral", "face": "bright", "hair": {"style": "long", "color": "auburn"}}, "UI choices drive appearance")
		var before_form: Dictionary = creator.appearance.duplicate(true)
		var before_hash := _hash(creator.appearance_image())
		creator.form_choice.select(1)
		creator.form_choice.item_selected.emit(1)
		_check(creator.active_view == "gremlin", "Form comparison selected")
		creator.save_button.pressed.emit()
		_check(creator.last_result.ok and creator.revision == 1, "save through UI while viewing another Form")
		_check(creator.appearance == before_form, "Form view preserves customization")
		creator.form_choice.select(0)
		creator.form_choice.item_selected.emit(0)
		_check(_hash(creator.portrait.texture.get_image()) == before_hash, "return restores identical Hueman pixels")
		_write(expected_path, JSON.stringify({"avatar_id": creator.avatar_id, "appearance": before_form, "pixel_sha256": before_hash}))
	else:
		var expected: Dictionary = JSON.parse_string(FileAccess.get_file_as_string(expected_path))
		_check(creator.last_result.ok and creator.revision == 1, "fresh process automatically loads saved profile")
		_check(creator.avatar_id == expected.avatar_id, "same avatar identity across processes")
		_check(creator.appearance == expected.appearance, "same customized appearance across processes")
		_check(_hash(creator.appearance_image()) == expected.pixel_sha256, "same rendered pixels across processes")
		_check(creator.active_view == "hueman", "visual Form was never persisted")
		creator.skin_buttons[0].pressed.emit()
		creator.load_button.pressed.emit()
		_check(creator.appearance == expected.appearance, "Load saved restores edited draft")
		# Failed load must keep the user's in-memory appearance and preserve the file.
		var saved_bytes := FileAccess.get_file_as_string(creator.store.path())
		_write(creator.store.path(), "damaged fixture")
		creator.load_button.pressed.emit()
		_check(not creator.last_result.ok and creator.appearance == expected.appearance, "failed load retains current draft")
		_write(creator.store.path(), saved_bytes)
		creator.load_button.pressed.emit()
	creator.studio.grab_focus()
	var start: Vector2 = creator.studio.feet
	var key := InputEventKey.new()
	key.keycode = KEY_D
	key.pressed = true
	Input.parse_input_event(key)
	await process_frame
	creator.studio._process(0.25)
	key.pressed = false
	Input.parse_input_event(key)
	await process_frame
	_check(creator.studio.feet.x > start.x, "focused keyboard movement works")
	creator.face_choice.grab_focus()
	start = creator.studio.feet
	key.pressed = true
	Input.parse_input_event(key)
	creator.studio._process(0.25)
	key.pressed = false
	Input.parse_input_event(key.duplicate())
	_check(creator.studio.feet == start, "editing controls does not move avatar")
	creator.studio.move_avatar(Vector2(1, 1), 1000)
	_check(creator.studio.feet.x <= creator.studio.size.x - 30 and creator.studio.feet.y <= creator.studio.size.y - 18, "movement stays inside studio")
	creator.studio.feet = Vector2(creator.studio.size.x / 2, creator.studio.size.y / 2 + 20)
	creator.studio.queue_redraw()
	if not capture.is_empty():
		await RenderingServer.frame_post_draw
		var review_rect := Rect2i(creator.page_frame.get_global_rect())
		_check(root.get_texture().get_image().get_region(review_rect).save_png(capture) == OK, "capture saved")
		creator.form_choice.select(1)
		creator.form_choice.item_selected.emit(1)
		await RenderingServer.frame_post_draw
		_check(root.get_texture().get_image().get_region(review_rect).save_png(capture.get_basename() + "-gremlin.png") == OK, "Form capture saved")
	creator.queue_free()
	await process_frame

func _layout_checks(creator: Control) -> void:
	var original_size := root.size
	for viewport_size in [Vector2i(1120, 920), Vector2i(900, 600), Vector2i(1280, 1800)]:
		root.size = viewport_size
		for frame in range(5):
			await process_frame
		var page: Rect2 = creator.page_frame.get_global_rect()
		_check(page.position.x >= 0 and page.end.x <= root.size.x, "page fits window width")
		if viewport_size.y == 600:
			_check(creator.page_scroll.get_v_scroll_bar().visible, "short window can scroll")
			creator.page_scroll.ensure_control_visible(creator.save_button)
			await process_frame
			var button: Rect2 = creator.save_button.get_global_rect()
			_check(button.position.y >= 0 and button.end.y <= root.size.y, "save control reachable in short window")
		else:
			_check(Rect2(Vector2.ZERO, Vector2(root.size)).encloses(page), "page stays inside default/tall window")
			_check(page.size.y <= 920, "tall window keeps bounded studio")
	root.size = original_size
	creator.page_scroll.scroll_vertical = 0
	for frame in range(5):
		await process_frame
