extends Node2D

const MAP_CONTRACT_RELATIVE_PATH := "artifacts/hueman_screen_map.json"
const SPATIAL_TOPOLOGY_V2_OVERLAY_RELATIVE_PATH := "artifacts/hollow_grove_spatial_topology_v2_overlay.json"
const ADAPTIVE_MORPH_CONTRACT_RELATIVE_PATH := "artifacts/hollow_grove_adaptive_morph_v1.json"
const ADAPTIVE_TOPOLOGY_STATE_RELATIVE_PATH := "artifacts/hollow_grove_adaptive_topology_state_v1.json"
const ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_RELATIVE_PATH := "artifacts/hollow_grove_adaptive_topology_rebuild_command_v1.json"
const ADAPTIVE_TOPOLOGY_REBUILD_ACK_RELATIVE_PATH := "artifacts/hollow_grove_adaptive_topology_rebuild_ack_v1.json"
const ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID := "ruleset.hollow-grove.adaptive-topology-live-v1"
const COORDINATE_CONTRACT_RELATIVE_PATH := "artifacts/hollow_grove_hueman_coordinate_contract.json"
const VISUAL_COLOR_CONSTITUTION_RELATIVE_PATH := "src/constitutional/hollow_grove_visual_color_palette.json"
const LIVE_STATE_RELATIVE_PATH := "artifacts/screen_map_state.json"
const INTENT_RELATIVE_PATH := "artifacts/screen_map_intent.json"
const PAIR_STATE_RELATIVE_PATH := "artifacts/hueman_pair_state.json"
const PAIR_PREVIEW_IMAGE_RELATIVE_PATH := "artifacts/hueman_pair_preview.png"
const PAIR_PREVIEW_STATE_RELATIVE_PATH := "artifacts/hueman_pair_preview_state.json"
const ASSET_EXPORT_RELATIVE_PATH := "hueman_godot/assets/export"
const ASSET_EXPORT_REVISION_RELATIVE_PATH := "artifacts/aseprite_export_revision.json"
const POLL_INTERVAL_SECONDS := 0.25

var repo_root_path := ""
var map_contract := {}
var adaptive_morph_contract := {}
var adaptive_topology_state := {}
var adaptive_topology_rebuild_command := {}
var adaptive_rectangularity := 0.0
var adaptive_morph_developer_override := false
var topology_review_mode := false
var coordinate_contract := {}
var visual_color_constitution := {}
var colors_by_semantic_identity := {}
var live_state := {}
var pair_state := {}
var pair_preview_state := {}
var pair_preview_texture = null
var pair_preview_mtime := -1
var live_state_mtime := -2
var adaptive_topology_rebuild_command_id := ""
var adaptive_topology_rebuild_in_flight := false
var adaptive_pending_rebuild_command := {}
var pair_state_mtime := -2
var pair_preview_state_mtime := -2
var asset_revision_mtime := -2
var asset_preview_texture = null
var asset_preview_path := ""
var asset_preview_mtime := -1
var asset_preview_name := ""
var asset_preview_count := 0
var current_resolution := {}
var poll_elapsed := 0.0
var pair_transition_elapsed := 0.0
var pair_transition_duration := 0.0
var pair_transition_message := ""
var pair_transition_detail := ""
var last_pair_window_id := -1
var last_pair_binding_status := ""
var last_pair_binding_source := ""
var last_pair_mode_active := false

var hud_layer: CanvasLayer
var status_label: Label
var hint_label: Label
var adaptive_morph_label: Label
var adaptive_morph_slider: HSlider
var adaptive_world_state_button: Button
var adaptive_collision_area: Area2D
var adaptive_navigation_root: Node2D


func _ready() -> void:
	repo_root_path = _resolve_repo_root()
	if not _load_visual_color_constitution():
		push_error("The canonical Hollow Grove visual color constitution could not be loaded.")
		set_process(false)
		set_process_input(false)
		return
	_ensure_hud()
	_ensure_adaptive_topology_physics_roots()
	get_viewport().size_changed.connect(_on_adaptive_topology_viewport_size_changed)
	_load_all_state()
	set_process(true)
	set_process_input(true)


func _process(delta: float) -> void:
	poll_elapsed += delta
	if poll_elapsed >= POLL_INTERVAL_SECONDS:
		poll_elapsed = 0.0
		_load_all_state()

	if pair_transition_elapsed < pair_transition_duration:
		pair_transition_elapsed = min(pair_transition_elapsed + delta, pair_transition_duration)
		queue_redraw()


func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and not event.echo:
		if event.keycode == KEY_ENTER or event.keycode == KEY_KP_ENTER:
			_write_intent("inspect")
		elif event.keycode == KEY_SPACE:
			_write_intent("move")


func _draw() -> void:
	if colors_by_semantic_identity.is_empty():
		return
	var viewport_size: Vector2 = get_viewport_rect().size
	draw_rect(Rect2(Vector2.ZERO, viewport_size), _constitutional_color("hollow_grove.universal.outline"))

	if map_contract.is_empty():
		return

	var aura_field_color: Color = _constitutional_color("hollow_grove.house.sandmanor.highlight")
	aura_field_color.a = 0.18
	var content_farm_color: Color = _constitutional_color("hollow_grove.house.sandmanor.primary")
	content_farm_color.a = 0.22
	var aura_beach_color: Color = _constitutional_color("hollow_grove.house.glaushouse.highlight")
	aura_beach_color.a = 0.18
	var aura_basin_color: Color = _constitutional_color("hollow_grove.house.stonebend.highlight")
	aura_basin_color.a = 0.18

	_draw_surface("aura_field", aura_field_color, viewport_size)
	_draw_surface("content_farm", content_farm_color, viewport_size)
	_draw_surface("aura_beach", aura_beach_color, viewport_size)
	_draw_surface("aura_basin", aura_basin_color, viewport_size)

	if not topology_review_mode:
		_draw_motion_grid_overlay(viewport_size)
	_draw_straight_routes(viewport_size)
	_draw_curved_routes(viewport_size)
	_draw_adaptive_midway_hinges(viewport_size)
	_draw_nodes(viewport_size)
	_draw_surface_labels(viewport_size)
	if not topology_review_mode:
		_draw_niri_layout_guides(viewport_size)
		_draw_pair_overlay(viewport_size)
		_draw_pair_transition(viewport_size)
		_draw_asset_preview(viewport_size)
		_draw_player_probe(viewport_size)


func _ensure_hud() -> void:
	hud_layer = CanvasLayer.new()
	add_child(hud_layer)

	status_label = Label.new()
	status_label.position = Vector2(16.0, 16.0)
	status_label.size = Vector2(520.0, 160.0)
	status_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	status_label.add_theme_font_size_override("font_size", 18)
	status_label.modulate = _constitutional_color("hollow_grove.house.flynt.highlight")
	hud_layer.add_child(status_label)

	hint_label = Label.new()
	hint_label.position = Vector2(16.0, 170.0)
	hint_label.size = Vector2(560.0, 80.0)
	hint_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	hint_label.add_theme_font_size_override("font_size", 15)
	hint_label.modulate = _constitutional_color("hollow_grove.house.flynt.authority")
	hud_layer.add_child(hint_label)

	adaptive_morph_label = Label.new()
	adaptive_morph_label.position = Vector2(16.0, 252.0)
	adaptive_morph_label.size = Vector2(560.0, 24.0)
	adaptive_morph_label.modulate = _constitutional_color("hollow_grove.house.stonebend.highlight")
	hud_layer.add_child(adaptive_morph_label)

	adaptive_morph_slider = HSlider.new()
	adaptive_morph_slider.position = Vector2(16.0, 278.0)
	adaptive_morph_slider.size = Vector2(320.0, 28.0)
	adaptive_morph_slider.min_value = 0.0
	adaptive_morph_slider.max_value = 1.0
	adaptive_morph_slider.step = 0.05
	adaptive_morph_slider.value = adaptive_rectangularity
	adaptive_morph_slider.value_changed.connect(_on_adaptive_rectangularity_changed)
	hud_layer.add_child(adaptive_morph_slider)

	adaptive_world_state_button = Button.new()
	adaptive_world_state_button.position = Vector2(344.0, 278.0)
	adaptive_world_state_button.size = Vector2(144.0, 28.0)
	adaptive_world_state_button.text = "Use world state"
	adaptive_world_state_button.pressed.connect(_on_use_adaptive_world_state_pressed)
	hud_layer.add_child(adaptive_world_state_button)
	_update_adaptive_morph_label()


func _load_all_state() -> void:
	var changed := false
	if map_contract.is_empty():
		map_contract = _read_json_file(_repo_file(MAP_CONTRACT_RELATIVE_PATH))
		_apply_spatial_topology_v2_overlay()
		changed = true
	if adaptive_morph_contract.is_empty():
		adaptive_morph_contract = _read_json_file(_repo_file(ADAPTIVE_MORPH_CONTRACT_RELATIVE_PATH))
		if not adaptive_morph_contract.is_empty():
			var unit := float(adaptive_morph_contract.get("normalization_unit", 1000000000))
			adaptive_rectangularity = clampf(
				float(adaptive_morph_contract.get("default_rectangularity_parts", 0)) / unit,
				0.0,
				1.0
			)
			adaptive_morph_slider.step = float(adaptive_morph_contract.get("developer_step_parts", 50000000)) / unit
			adaptive_morph_slider.set_value_no_signal(adaptive_rectangularity)
			_update_adaptive_morph_label()
		changed = true

	var next_topology_state := _read_json_file(_repo_file(ADAPTIVE_TOPOLOGY_STATE_RELATIVE_PATH))
	if next_topology_state != adaptive_topology_state:
		adaptive_topology_state = next_topology_state
		if not adaptive_morph_developer_override:
			_apply_adaptive_topology_world_state()
		changed = true

	var next_rebuild_command := _read_json_file(_repo_file(ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_RELATIVE_PATH))
	var next_rebuild_command_id := str(next_rebuild_command.get("command_id", ""))
	if next_rebuild_command_id != "" and next_rebuild_command_id != adaptive_topology_rebuild_command_id:
		if _adaptive_topology_command_is_valid(next_rebuild_command):
			adaptive_topology_rebuild_command_id = next_rebuild_command_id
			adaptive_topology_rebuild_command = next_rebuild_command
			_queue_adaptive_topology_rebuild(next_rebuild_command)
			changed = true
		else:
			push_error("Rejected an invalid adaptive-topology rebuild command.")
	if coordinate_contract.is_empty():
		coordinate_contract = _read_json_file(_repo_file(COORDINATE_CONTRACT_RELATIVE_PATH))
		changed = true

	var live_path := _repo_file(LIVE_STATE_RELATIVE_PATH)
	var next_live_mtime := _modified_time_or_missing(live_path)
	if next_live_mtime != live_state_mtime:
		live_state_mtime = next_live_mtime
		live_state = _read_json_file(live_path)
		changed = true

	var pair_path := _repo_file(PAIR_STATE_RELATIVE_PATH)
	var next_pair_mtime := _modified_time_or_missing(pair_path)
	var pair_changed := next_pair_mtime != pair_state_mtime
	if pair_changed:
		pair_state_mtime = next_pair_mtime
		pair_state = _read_json_file(pair_path)
		changed = true

	var preview_state_path := _repo_file(PAIR_PREVIEW_STATE_RELATIVE_PATH)
	var next_preview_state_mtime := _modified_time_or_missing(preview_state_path)
	if next_preview_state_mtime != pair_preview_state_mtime:
		pair_preview_state_mtime = next_preview_state_mtime
		pair_preview_state = _read_json_file(preview_state_path)
		changed = true

	var previous_preview_mtime := pair_preview_mtime
	_reload_pair_preview_texture()
	changed = changed or previous_preview_mtime != pair_preview_mtime

	var revision_path := _repo_file(ASSET_EXPORT_REVISION_RELATIVE_PATH)
	var next_asset_revision_mtime := _modified_time_or_missing(revision_path)
	if next_asset_revision_mtime != asset_revision_mtime:
		asset_revision_mtime = next_asset_revision_mtime
		_reload_asset_preview_texture()
		changed = true

	if not changed:
		return
	if pair_changed:
		_refresh_pair_transition_state()
	current_resolution = _resolve_current_zone()
	_update_hud()
	queue_redraw()


func _apply_spatial_topology_v2_overlay() -> void:
	var overlay := _read_json_file(_repo_file(SPATIAL_TOPOLOGY_V2_OVERLAY_RELATIVE_PATH))
	if overlay.is_empty():
		return
	for key in ["straight_routes", "curved_routes", "surfaces"]:
		if overlay.has(key):
			map_contract[key] = overlay[key]
	map_contract["spatial_topology_v2"] = {
		"schema_version": overlay.get("schema_version", 2),
		"rule_set": overlay.get("rule_set", ""),
		"legacy_decode_aliases": overlay.get("legacy_decode_aliases", {})
	}


func _modified_time_or_missing(path: String) -> int:
	if not FileAccess.file_exists(path):
		return -1
	return FileAccess.get_modified_time(path)


func _update_hud() -> void:
	var state_status := str(live_state.get("status", "missing"))
	if bool(pair_state.get("paired_window_mode", false)):
		state_status = str(pair_state.get("binding_status", state_status))
	var zone_name := str(current_resolution.get("name", "(none)"))
	var zone_kind := str(current_resolution.get("kind", "(none)"))
	var normalized := _active_normalized_probe()
	var center: Dictionary = normalized.get("center", {})
	var reference_output: Dictionary = map_contract.get("reference_output", {})
	var focused_window := _active_probe_window()
	var paired_window: Dictionary = _dict_or_empty(pair_state.get("focused_window"))
	var probe_source := _active_probe_source()
	var center_x := _format_decimal(float(center.get("x", 0.0)))
	var center_y := _format_decimal(float(center.get("y", 0.0)))
	var window_title := str(focused_window.get("title", "(none)"))
	var paired_window_title := str(paired_window.get("title", "(none)"))
	var paired_app_id := str(paired_window.get("app_id", "(none)"))
	var diagonal_angle := str(pair_state.get("diagonal_angle_degrees", 135))
	var spread_ratio := _format_decimal(float(pair_state.get("spread_ratio", 0.25)))
	var preview_status := str(pair_preview_state.get("status", "missing"))
	var preview_detail := str(pair_preview_state.get("detail", "(none)"))
	var application_attachment := _application_attachment()
	var application_name := str(application_attachment.get("canonical_name", "(none)"))
	var sprite_preview_name := asset_preview_name if asset_preview_name != "" else "(waiting)"
	var output_label := "%sx%s %s %s-inch" % [
		str(reference_output.get("width_px", 3840)),
		str(reference_output.get("height_px", 2160)),
		str(reference_output.get("aspect_ratio", "16:9")),
		str(reference_output.get("diagonal_inches", 32))
	]

	status_label.text = "Hueman Screen Shell\nstatus: %s\nwindow: %s\npair: %s [%s]\nmanaged application: %s\nzone: %s (%s)\nprobe source: %s\nprobe: (%s, %s)\ndiagonal: %s\nspread: %s\npreview: %s\npreview detail: %s\nsprite preview: %s (%s assets)\ncalibration: %s" % [
		state_status,
		window_title,
		paired_window_title,
		paired_app_id,
		application_name,
		zone_name,
		zone_kind,
		probe_source,
		center_x,
		center_y,
		diagonal_angle,
		spread_ratio,
		preview_status,
		preview_detail,
		sprite_preview_name,
		str(asset_preview_count),
		output_label,
	]

	hint_label.text = "Drag the topology slider from canonical circle to locked rectangle. Enter writes inspect; Space writes move. Super+Alt+Enter opens Hueman."
	_update_adaptive_morph_label()


func _on_adaptive_rectangularity_changed(value: float) -> void:
	adaptive_morph_developer_override = true
	adaptive_rectangularity = clampf(value, 0.0, 1.0)
	_update_adaptive_morph_label()
	current_resolution = _resolve_current_zone()
	queue_redraw()


func _on_use_adaptive_world_state_pressed() -> void:
	adaptive_morph_developer_override = false
	_apply_adaptive_topology_world_state()
	current_resolution = _resolve_current_zone()
	queue_redraw()


func _apply_adaptive_topology_world_state() -> void:
	if adaptive_topology_state.is_empty():
		return
	var unit := float(adaptive_morph_contract.get("normalization_unit", 1000000000))
	if unit <= 0.0:
		return
	adaptive_rectangularity = clampf(
		float(adaptive_topology_state.get("rectangularity", 0)) / unit,
		0.0,
		1.0
	)
	if adaptive_morph_slider != null:
		adaptive_morph_slider.set_value_no_signal(adaptive_rectangularity)
	_update_adaptive_morph_label()


func set_topology_review_mode(enabled: bool) -> void:
	topology_review_mode = enabled
	status_label.visible = not enabled
	hint_label.visible = not enabled
	adaptive_morph_label.position = Vector2(16.0, 16.0) if enabled else Vector2(16.0, 252.0)
	adaptive_morph_slider.position = Vector2(16.0, 42.0) if enabled else Vector2(16.0, 278.0)
	adaptive_world_state_button.visible = not enabled
	_update_adaptive_morph_label()
	queue_redraw()


func _update_adaptive_morph_label() -> void:
	if adaptive_morph_label == null:
		return
	var pose_name := "Adaptive"
	if is_zero_approx(adaptive_rectangularity):
		pose_name = "Canonical circle"
	elif is_equal_approx(adaptive_rectangularity, 1.0):
		pose_name = "Locked rectangle"
	if not topology_review_mode:
		if adaptive_morph_developer_override:
			pose_name += " · developer override"
		elif not adaptive_topology_state.is_empty():
			pose_name += " · world state r%s %s" % [
				str(adaptive_topology_state.get("revision", 0)),
				str(adaptive_topology_state.get("phase", "committed"))
			]
	adaptive_morph_label.text = "Hollow Grove topology: %s (%d%%)" % [
		pose_name,
		roundi(adaptive_rectangularity * 100.0)
	]


func _resolve_repo_root() -> String:
	var env_root := OS.get_environment("HOLLOW_GROVE_ROOT")
	if env_root != "":
		return env_root

	var project_root := ProjectSettings.globalize_path("res://")
	return project_root.get_base_dir()


func _repo_file(relative_path: String) -> String:
	return repo_root_path.path_join(relative_path)


func _read_json_file(path: String) -> Dictionary:
	if not FileAccess.file_exists(path):
		return {}

	var handle := FileAccess.open(path, FileAccess.READ)
	if handle == null:
		return {}

	var parsed = JSON.parse_string(handle.get_as_text())
	if typeof(parsed) == TYPE_DICTIONARY:
		return parsed
	return {}


func _ensure_adaptive_topology_physics_roots() -> void:
	adaptive_collision_area = Area2D.new()
	adaptive_collision_area.name = "AdaptiveTopologyRouteCollision"
	adaptive_collision_area.collision_layer = 1 << 6
	adaptive_collision_area.collision_mask = 0
	adaptive_collision_area.monitoring = true
	adaptive_collision_area.monitorable = true
	add_child(adaptive_collision_area)

	adaptive_navigation_root = Node2D.new()
	adaptive_navigation_root.name = "AdaptiveTopologyRouteNavigation"
	add_child(adaptive_navigation_root)


func _on_adaptive_topology_viewport_size_changed() -> void:
	if not adaptive_topology_rebuild_command.is_empty():
		_queue_adaptive_topology_rebuild(adaptive_topology_rebuild_command)


func _queue_adaptive_topology_rebuild(command: Dictionary) -> void:
	if not _adaptive_topology_command_is_valid(command):
		push_error("Rejected an invalid adaptive-topology rebuild command.")
		return
	adaptive_pending_rebuild_command = command.duplicate(true)
	if not adaptive_topology_rebuild_in_flight:
		call_deferred("_drain_adaptive_topology_rebuild_queue")


func _drain_adaptive_topology_rebuild_queue() -> void:
	if adaptive_topology_rebuild_in_flight:
		return
	while not adaptive_pending_rebuild_command.is_empty():
		var command: Dictionary = adaptive_pending_rebuild_command
		adaptive_pending_rebuild_command = {}
		adaptive_topology_rebuild_in_flight = true
		await _rebuild_adaptive_topology_physics(command)
		adaptive_topology_rebuild_in_flight = false


func _adaptive_topology_command_is_valid(command: Dictionary) -> bool:
	if int(command.get("schema_version", 0)) != 1:
		return false
	if str(command.get("rule_set", "")) != ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID:
		return false
	if str(command.get("command_id", "")) == "":
		return false
	var status := str(command.get("status", ""))
	if status != "committed" and status != "prepared":
		return false
	var geometry := _dict_or_empty(command.get("geometry"))
	var normalization_unit := int(geometry.get("normalization_unit", 0))
	var sample_segments := int(geometry.get("sample_segments", 0))
	var routes: Array = geometry.get("routes", [])
	if normalization_unit <= 0 or sample_segments <= 0 or routes.is_empty():
		return false
	if int(geometry.get("corridor_half_width_parts", 0)) <= 0:
		return false
	var route_ids := {}
	for route_value in routes:
		var route := _dict_or_empty(route_value)
		var route_id := str(route.get("route_id", ""))
		var points: Array = route.get("points", [])
		if route_id == "" or route_ids.has(route_id) or points.size() != sample_segments + 1:
			return false
		route_ids[route_id] = true
		for point_value in points:
			var point := _dict_or_empty(point_value)
			var x := int(point.get("x", -1))
			var y := int(point.get("y", -1))
			if x < 0 or x > normalization_unit or y < 0 or y > normalization_unit:
				return false
	if str(geometry.get("geometry_digest", "")) != _adaptive_topology_geometry_digest(geometry):
		return false
	if status == "prepared":
		var prepared := _dict_or_empty(command.get("prepared"))
		if str(prepared.get("transition_id", "")) == "":
			return false
		if str(prepared.get("collision_snapshot_id", "")) == "":
			return false
		if str(prepared.get("navigation_snapshot_id", "")) == "":
			return false
	return true


func _adaptive_topology_geometry_digest(geometry: Dictionary) -> String:
	var canonical_parts := PackedStringArray([
		"v1",
		str(int(geometry.get("rectangularity", 0))),
		str(int(geometry.get("corridor_half_width_parts", 0))),
		str(int(geometry.get("sample_segments", 0)))
	])
	for route_value in geometry.get("routes", []):
		var route := _dict_or_empty(route_value)
		canonical_parts.append(str(route.get("route_id", "")))
		for point_value in route.get("points", []):
			var point := _dict_or_empty(point_value)
			canonical_parts.append(str(int(point.get("x", 0))))
			canonical_parts.append(str(int(point.get("y", 0))))
	var hash_value: int = 2166136261
	for byte in "|".join(canonical_parts).to_utf8_buffer():
		hash_value = ((hash_value ^ int(byte)) * 16777619) & 0xffffffff
	return "fnv1a32-adaptive-topology-geometry-v1:%08x" % hash_value


func _rebuild_adaptive_topology_physics(command: Dictionary) -> void:
	_clear_adaptive_topology_physics()
	var geometry := _dict_or_empty(command.get("geometry"))
	var normalization_unit := float(geometry.get("normalization_unit", 1))
	var viewport_size := get_viewport_rect().size
	var corridor_half_width: float = (
		float(geometry.get("corridor_half_width_parts", 0)) /
		normalization_unit * min(viewport_size.x, viewport_size.y)
	)
	var collision_polygon_count := 0
	var navigation_region_count := 0
	var navigation_source_geometry := NavigationMeshSourceGeometryData2D.new()
	var route_index := 0
	for route_value in geometry.get("routes", []):
		var route := _dict_or_empty(route_value)
		var route_points := PackedVector2Array()
		for point_value in route.get("points", []):
			var point := _dict_or_empty(point_value)
			route_points.append(Vector2(
				float(point.get("x", 0)) / normalization_unit * viewport_size.x,
				float(point.get("y", 0)) / normalization_unit * viewport_size.y
			))
		var corridor_polygons := Geometry2D.offset_polyline(
			route_points,
			corridor_half_width,
			Geometry2D.JOIN_ROUND,
			Geometry2D.END_ROUND
		)
		var polygon_index := 0
		for polygon_value in corridor_polygons:
			var polygon: PackedVector2Array = polygon_value
			if polygon.size() < 3:
				continue

			var collision_polygon := CollisionPolygon2D.new()
			collision_polygon.name = "RouteCollision%d_%d" % [route_index, polygon_index]
			collision_polygon.polygon = polygon
			adaptive_collision_area.add_child(collision_polygon)
			collision_polygon_count += 1
			navigation_source_geometry.add_traversable_outline(polygon)
			polygon_index += 1
		route_index += 1

	var navigation_polygon := NavigationPolygon.new()
	navigation_polygon.agent_radius = 0.0
	NavigationServer2D.bake_from_source_geometry_data(
		navigation_polygon,
		navigation_source_geometry
	)
	if navigation_polygon.get_polygon_count() > 0:
		var navigation_region := NavigationRegion2D.new()
		navigation_region.name = "AdaptiveRouteNavigationRegion"
		navigation_region.navigation_polygon = navigation_polygon
		adaptive_navigation_root.add_child(navigation_region)
		navigation_region_count = 1

	await get_tree().physics_frame
	NavigationServer2D.map_force_update(get_world_2d().navigation_map)
	await get_tree().physics_frame
	print(
		"ADAPTIVE_TOPOLOGY_REALIZED command=%s status=%s rectangularity=%s collision_polygons=%d navigation_regions=%d digest=%s" % [
			str(command.get("command_id", "")),
			str(command.get("status", "")),
			str(int(geometry.get("rectangularity", 0))),
			collision_polygon_count,
			navigation_region_count,
			str(geometry.get("geometry_digest", ""))
		]
	)

	if str(command.get("status", "")) == "prepared":
		var route_count: int = geometry.get("routes", []).size()
		if collision_polygon_count < route_count or navigation_region_count == 0:
			push_error("Adaptive topology rebuild did not produce complete collision and navigable corridor geometry.")
			return
		_write_adaptive_topology_rebuild_ack(
			command,
			collision_polygon_count,
			navigation_region_count
		)


func _clear_adaptive_topology_physics() -> void:
	for child in adaptive_collision_area.get_children():
		adaptive_collision_area.remove_child(child)
		child.queue_free()
	for child in adaptive_navigation_root.get_children():
		adaptive_navigation_root.remove_child(child)
		child.queue_free()


func _write_adaptive_topology_rebuild_ack(
	command: Dictionary,
	collision_polygon_count: int,
	navigation_region_count: int
) -> void:
	var geometry := _dict_or_empty(command.get("geometry"))
	var prepared := _dict_or_empty(command.get("prepared"))
	var payload := {
		"schema_version": 1,
		"rule_set": ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID,
		"command_id": command.get("command_id", ""),
		"transition_id": prepared.get("transition_id", ""),
		"base_revision": int(prepared.get("base_revision", 0)),
		"next_revision": int(prepared.get("next_revision", 0)),
		"rectangularity": int(geometry.get("rectangularity", 0)),
		"geometry_digest": geometry.get("geometry_digest", ""),
		"collision_snapshot_id": prepared.get("collision_snapshot_id", ""),
		"navigation_snapshot_id": prepared.get("navigation_snapshot_id", ""),
		"route_count": geometry.get("routes", []).size(),
		"collision_polygon_count": collision_polygon_count,
		"navigation_region_count": navigation_region_count
	}
	var target_path := _repo_file(ADAPTIVE_TOPOLOGY_REBUILD_ACK_RELATIVE_PATH)
	var pending_path := target_path + ".godot.%d.tmp" % OS.get_process_id()
	var handle := FileAccess.open(pending_path, FileAccess.WRITE)
	if handle == null:
		push_error("Could not stage adaptive-topology rebuild acknowledgement.")
		return
	handle.store_string(JSON.stringify(payload, "\t") + "\n")
	handle.flush()
	handle = null
	var rename_error := DirAccess.rename_absolute(pending_path, target_path)
	if rename_error != OK:
		DirAccess.remove_absolute(pending_path)
		push_error("Could not atomically publish adaptive-topology rebuild acknowledgement.")


func _load_visual_color_constitution() -> bool:
	visual_color_constitution = _read_json_file(_repo_file(VISUAL_COLOR_CONSTITUTION_RELATIVE_PATH))
	colors_by_semantic_identity.clear()
	if visual_color_constitution.is_empty():
		return false

	var colors: Array = visual_color_constitution.get("colors", [])
	for value in colors:
		if typeof(value) != TYPE_DICTIONARY:
			return false
		var color_record: Dictionary = value
		var semantic_identity := str(color_record.get("semantic_identity", ""))
		var hexadecimal := str(color_record.get("hex", ""))
		if semantic_identity == "" or hexadecimal == "" or colors_by_semantic_identity.has(semantic_identity):
			return false
		colors_by_semantic_identity[semantic_identity] = color_record

	return not colors_by_semantic_identity.is_empty()


func _constitutional_color(semantic_identity: String, alpha := 1.0) -> Color:
	assert(colors_by_semantic_identity.has(semantic_identity), "Unknown constitutional color identity: %s" % semantic_identity)
	var color_record: Dictionary = colors_by_semantic_identity[semantic_identity]
	var color := Color.from_string(str(color_record["hex"]), Color.TRANSPARENT)
	color.a = alpha
	return color


func _resolve_current_zone() -> Dictionary:
	var application_attachment := _application_attachment()
	if not application_attachment.is_empty():
		var world_anchor := _dict_or_empty(application_attachment.get("world_anchor"))
		if not world_anchor.is_empty():
			return {
				"id": str(world_anchor.get("id", "")),
				"name": str(world_anchor.get("name", "")),
				"kind": str(world_anchor.get("kind", "kingdom"))
			}

	var normalized := _active_normalized_probe()
	var center: Dictionary = normalized.get("center", {})
	if center.is_empty():
		return {}

	var spatial_viewpoint := _active_spatial_viewpoint()
	var inherited_local := _dict_or_empty(spatial_viewpoint.get("local"))
	if not inherited_local.is_empty():
		center = inherited_local
	var probe := Vector2(float(center.get("x", 0.0)), float(center.get("y", 0.0)))
	var nodes: Array = map_contract.get("nodes", [])
	var straight_routes: Array = map_contract.get("straight_routes", [])
	var curved_routes: Array = map_contract.get("curved_routes", [])
	var surfaces: Array = map_contract.get("surfaces", [])

	for node in nodes:
		var node_point := _node_point(node)
		var radius := float(node.get("radius", 0.09))
		if probe.distance_to(node_point) <= radius:
			return _with_spatial_scope({
				"id": str(node.get("id", "")),
				"name": str(node.get("name", "")),
				"kind": str(node.get("kind", "kingdom"))
			}, spatial_viewpoint)

	for route in straight_routes:
		if _distance_to_polyline(probe, _adaptive_route_points(route, 32)) <= float(route.get("band_width", 0.045)):
			return _with_spatial_scope({
				"id": str(route.get("id", "")),
				"name": _public_route_name(route),
				"kind": "straight_route"
			}, spatial_viewpoint)

	for route in curved_routes:
		if _distance_to_polyline(probe, _adaptive_route_points(route, 32)) <= float(route.get("band_width", 0.045)):
			return _with_spatial_scope({
				"id": str(route.get("id", "")),
				"name": _public_route_name(route),
				"kind": "curved_route"
			}, spatial_viewpoint)

	for surface in surfaces:
		var polygon := PackedVector2Array()
		for point in surface.get("polygon", []):
			polygon.append(Vector2(float(point.get("x", 0.0)), float(point.get("y", 0.0))))
		if Geometry2D.is_point_in_polygon(probe, polygon):
			return _with_spatial_scope({
				"id": str(surface.get("id", "")),
				"name": str(surface.get("name", "")),
				"kind": "surface"
			}, spatial_viewpoint)

	var motion_grid_resolution := _resolve_motion_grid_cell(probe)
	if not motion_grid_resolution.is_empty():
		return _with_spatial_scope(motion_grid_resolution, spatial_viewpoint)

	var nearest_route := _nearest_route_name(probe)
	return _with_spatial_scope({
		"id": nearest_route.to_snake_case(),
		"name": nearest_route,
		"kind": "nearest_route"
	}, spatial_viewpoint)


func _with_spatial_scope(resolution: Dictionary, viewpoint: Dictionary) -> Dictionary:
	if viewpoint.is_empty():
		return resolution
	var scoped_resolution := resolution.duplicate()
	scoped_resolution["spatial_frame_id"] = viewpoint.get("frame_id", "")
	scoped_resolution["spatial_template_id"] = viewpoint.get("template_id", "")
	scoped_resolution["spatial_scale"] = viewpoint.get("scale", "")
	scoped_resolution["spatial_depth"] = viewpoint.get("depth", 0)
	scoped_resolution["spatial_rule_set"] = viewpoint.get("rule_set", "")
	scoped_resolution["spatial_selection_policy"] = viewpoint.get("selection_policy", "")
	scoped_resolution["spatial_candidate_count"] = viewpoint.get("candidate_count", 0)
	scoped_resolution["spatial_catalog_digest"] = viewpoint.get("catalog_digest", "")
	scoped_resolution["spatial_template_digest"] = viewpoint.get("template_digest", "")
	scoped_resolution["spatial_replay_digest"] = viewpoint.get("replay_digest", "")
	scoped_resolution["spatial_root"] = _dict_or_empty(viewpoint.get("root"))
	scoped_resolution["spatial_local"] = _dict_or_empty(viewpoint.get("local"))
	return scoped_resolution


# Current signage over stable map IDs; the source contract remains replayable.
func _public_route_name(route: Dictionary) -> String:
	match str(route.get("constitutional_route_id", "")):
		"geography.route.stairway-to-heaven":
			return "H.T.T. — Highway to Hell / S.T.H. — Stairway to Heaven"
		"geography.route.basin-motorspeedway":
			return "Basin Motor Speedway (B.M.S.)"
	return str(route.get("name", ""))


func _nearest_route_name(probe: Vector2) -> String:
	var best_name := "Aura Ridge"
	var best_distance := INF

	for route in map_contract.get("straight_routes", []):
		var distance := _distance_to_polyline(probe, _adaptive_route_points(route, 32))
		if distance < best_distance:
			best_distance = distance
			best_name = _public_route_name(route)

	for route in map_contract.get("curved_routes", []):
		var distance := _distance_to_polyline(probe, _adaptive_route_points(route, 32))
		if distance < best_distance:
			best_distance = distance
			best_name = _public_route_name(route)

	return best_name


func _write_intent(intent_kind: String) -> void:
	if current_resolution.is_empty():
		return

	var payload := {
		"schema_version": "0.2.0",
		"intent": intent_kind,
		"zone": {
			"id": current_resolution.get("id", ""),
			"name": current_resolution.get("name", ""),
			"kind": current_resolution.get("kind", "")
		},
		"source": "hueman_godot_shell",
		"probe_source": _active_probe_source()
	}

	if _paired_probe_is_active():
		var paired_window := _dict_or_empty(pair_state.get("focused_window"))
		payload["pair"] = {
			"paired_window_mode": true,
			"window_id": paired_window.get("id", null),
			"window_title": paired_window.get("title", null),
			"app_id": paired_window.get("app_id", null),
			"diagonal_angle_degrees": pair_state.get("diagonal_angle_degrees", 135),
			"spread_ratio": pair_state.get("spread_ratio", 0.25)
		}

	if current_resolution.has("spatial_frame_id"):
		payload["spatial_scope"] = {
			"frame_id": current_resolution.get("spatial_frame_id", ""),
			"template_id": current_resolution.get("spatial_template_id", ""),
			"scale": current_resolution.get("spatial_scale", ""),
			"depth": current_resolution.get("spatial_depth", 0),
			"rule_set": current_resolution.get("spatial_rule_set", ""),
			"selection_policy": current_resolution.get("spatial_selection_policy", ""),
			"candidate_count": current_resolution.get("spatial_candidate_count", 0),
			"catalog_digest": current_resolution.get("spatial_catalog_digest", ""),
			"template_digest": current_resolution.get("spatial_template_digest", ""),
			"replay_digest": current_resolution.get("spatial_replay_digest", ""),
			"root": current_resolution.get("spatial_root", {}),
			"local": current_resolution.get("spatial_local", {})
		}

	var application_attachment := _application_attachment()
	if not application_attachment.is_empty():
		var world_anchor := _dict_or_empty(application_attachment.get("world_anchor"))
		var privacy := _dict_or_empty(application_attachment.get("privacy"))
		payload["application"] = {
			"application_id": application_attachment.get("application_id", null),
			"canonical_name": application_attachment.get("canonical_name", null),
			"lifecycle": application_attachment.get("lifecycle", null),
			"world_anchor": {
				"id": world_anchor.get("id", null),
				"institution_id": world_anchor.get("institution_id", null),
				"site_id": world_anchor.get("site_id", null),
				"zone_id": world_anchor.get("zone_id", null)
			},
			"projection": privacy.get("projection", null)
		}

	var handle := FileAccess.open(_repo_file(INTENT_RELATIVE_PATH), FileAccess.WRITE)
	if handle == null:
		return
	handle.store_string(JSON.stringify(payload, "\t") + "\n")


func _draw_nodes(viewport_size: Vector2) -> void:
	for node in map_contract.get("nodes", []):
		var point: Vector2 = _to_screen(_node_point(node), viewport_size)
		var radius: float = float(node.get("radius", 0.09)) * min(viewport_size.x, viewport_size.y)
		var color: Color = _constitutional_color("hollow_grove.house.flynt.highlight")
		draw_circle(point, radius, color)
		draw_circle(point, radius * 0.55, _constitutional_color("hollow_grove.universal.outline"))
		var label_offset := Vector2(-radius * 0.8, -radius * 1.25)
		var label_font_size := 28
		match str(node.get("id", "")):
			"stonebend":
				label_offset = Vector2(-radius * 0.8, -radius * 1.05)
			"glaushouse":
				label_offset = Vector2(-radius * 0.9, radius * 1.2)
			"aura_ridge_junction":
				label_offset = Vector2(-radius * 1.85, radius * 1.65)
				label_font_size = 18
		_draw_label(str(node.get("name", "")), point + label_offset, label_font_size, _constitutional_color("hollow_grove.house.flynt.highlight"))


func _draw_straight_routes(viewport_size: Vector2) -> void:
	for route in map_contract.get("straight_routes", []):
		var points := PackedVector2Array()
		for point in _adaptive_route_points(route, 32):
			points.append(_to_screen(point, viewport_size))
		draw_polyline(points, _constitutional_color("hollow_grove.house.stonebend.primary"), 5.0, true)


func _draw_curved_routes(viewport_size: Vector2) -> void:
	for route in map_contract.get("curved_routes", []):
		var points := PackedVector2Array()
		for point in _adaptive_route_points(route, 32):
			points.append(_to_screen(point, viewport_size))
		draw_polyline(points, _constitutional_color("hollow_grove.house.glaushouse.primary"), 4.0, true)


func _draw_adaptive_midway_hinges(viewport_size: Vector2) -> void:
	if adaptive_morph_contract.is_empty():
		return
	var inner_color := _constitutional_color("hollow_grove.house.stonebend.highlight")
	var outer_color := _constitutional_color("hollow_grove.house.glaushouse.highlight")
	var pivot_color := _constitutional_color("hollow_grove.house.flynt.authority")
	for hinge in adaptive_morph_contract.get("hinges", []):
		var slots: Array = hinge.get("slots", [])
		if slots.size() != 2:
			continue
		var inner := _adaptive_midway_position(slots[0])
		var outer := _adaptive_midway_position(slots[1])
		var pivot := (inner + outer) * 0.5
		var inner_screen := _to_screen(inner, viewport_size)
		var outer_screen := _to_screen(outer, viewport_size)
		var pivot_screen := _to_screen(pivot, viewport_size)
		draw_line(inner_screen, outer_screen, pivot_color, 1.5, true)
		draw_circle(inner_screen, 6.0, inner_color)
		draw_circle(outer_screen, 6.0, outer_color)
		draw_circle(pivot_screen, 3.0, pivot_color)
		if adaptive_rectangularity >= 0.75:
			var hinge_id := str(hinge.get("hinge_id", ""))
			var upper_hinge := hinge_id.contains("north")
			var inner_label_offset := Vector2(8.0, 18.0) if upper_hinge else Vector2(8.0, -8.0)
			var outer_label_offset := Vector2(8.0, -8.0) if upper_hinge else Vector2(8.0, 16.0)
			if hinge_id.ends_with("northwest"):
				inner_label_offset = Vector2(-210.0, 18.0)
			_draw_label(str(slots[0].get("midway_name", "")), inner_screen + inner_label_offset, 14, inner_color)
			_draw_label(str(slots[1].get("midway_name", "")), outer_screen + outer_label_offset, 14, outer_color)


func _draw_surface(surface_id: String, color: Color, viewport_size: Vector2) -> void:
	for surface in map_contract.get("surfaces", []):
		if str(surface.get("id", "")) != surface_id:
			continue

		var polygon := PackedVector2Array()
		for point in surface.get("polygon", []):
			polygon.append(_to_screen(Vector2(float(point.get("x", 0.0)), float(point.get("y", 0.0))), viewport_size))
		draw_colored_polygon(polygon, color)
		return


func _draw_surface_labels(viewport_size: Vector2) -> void:
	for surface in map_contract.get("surfaces", []):
		var points: Array = surface.get("polygon", [])
		if points.size() != 3:
			continue

		var center := Vector2.ZERO
		for point in points:
			center += Vector2(float(point.get("x", 0.0)), float(point.get("y", 0.0)))
		center /= 3.0
		var label_offset := Vector2(-70.0, 8.0)
		match str(surface.get("id", "")):
			"aura_field":
				label_offset = Vector2(-90.0, 8.0)
			"content_farm":
				label_offset = Vector2(-25.0, 26.0)
			"aura_beach":
				label_offset = Vector2(10.0, 18.0)
		_draw_label(
			str(surface.get("name", "")),
			_to_screen(center, viewport_size) + label_offset,
			24,
			_constitutional_color("hollow_grove.house.flynt.highlight")
		)


func _draw_motion_grid_overlay(viewport_size: Vector2) -> void:
	var motion_cells := _motion_grid_cells()
	if motion_cells.is_empty():
		return
	var active_cell_id := str(current_resolution.get("id", ""))

	var grid_line_color := _constitutional_color("hollow_grove.house.flynt.highlight")
	grid_line_color.a = 0.12
	var cell_fill_color := _constitutional_color("hollow_grove.universal.shadow.deep")
	cell_fill_color.a = 0.42
	var cell_outline_color := _constitutional_color("hollow_grove.house.flynt.authority")
	cell_outline_color.a = 0.45
	var text_color := _constitutional_color("hollow_grove.house.flynt.highlight")
	text_color.a = 0.82

	for x_ratio in [0.2, 0.5, 0.8]:
		draw_line(
			Vector2(float(x_ratio) * viewport_size.x, viewport_size.y * 0.14),
			Vector2(float(x_ratio) * viewport_size.x, viewport_size.y * 0.86),
			grid_line_color,
			1.5,
			true
		)
	for y_ratio in [0.2, 0.5, 0.8]:
		draw_line(
			Vector2(viewport_size.x * 0.2, float(y_ratio) * viewport_size.y),
			Vector2(viewport_size.x * 0.8, float(y_ratio) * viewport_size.y),
			grid_line_color,
			1.5,
			true
		)

	for cell in motion_cells:
		var cell_point := _to_screen(Vector2(float(cell.get("x", 0.0)), float(cell.get("y", 0.0))), viewport_size)
		var cell_rect := Rect2(cell_point - Vector2(52.0, 22.0), Vector2(104.0, 44.0))
		var cell_id := str(cell.get("id", ""))
		var is_center := cell_id == "human_core"
		var is_active := cell_id == active_cell_id
		var marker_color := _constitutional_color("hollow_grove.house.flynt.highlight")
		marker_color.a = 0.92 if is_active else (0.75 if is_center else 0.58)
		var active_fill_color := cell_fill_color
		if is_active:
			active_fill_color = _constitutional_color("hollow_grove.universal.shadow.raised")
			active_fill_color.a = 0.78
		var active_outline_color := cell_outline_color
		if is_active:
			active_outline_color = _constitutional_color("hollow_grove.house.flynt.highlight")
			active_outline_color.a = 0.94
		var active_text_color := text_color
		if is_active:
			active_text_color = _constitutional_color("hollow_grove.house.flynt.highlight")
			active_text_color.a = 0.98

		draw_rect(cell_rect, active_fill_color, true)
		draw_rect(cell_rect, active_outline_color, false, 1.0 if not is_active else 2.0)
		draw_circle(cell_point, 5.0 if (is_center or is_active) else 4.0, marker_color)

		var cell_number := str(cell.get("cell", ""))
		var cell_name := str(cell.get("name", ""))
		_draw_label(cell_number, cell_rect.position + Vector2(8.0, 18.0), 15, active_text_color)
		_draw_label(cell_name, cell_rect.position + Vector2(26.0, 18.0), 15, active_text_color)


func _draw_player_probe(viewport_size: Vector2) -> void:
	var normalized := _active_normalized_probe()
	var center: Dictionary = normalized.get("center", {})
	if center.is_empty():
		return

	var point: Vector2 = _to_screen(Vector2(float(center.get("x", 0.0)), float(center.get("y", 0.0))), viewport_size)
	draw_circle(point, 12.0, _constitutional_color("hollow_grove.house.sandmanor.primary"))
	draw_circle(point, 5.0, _constitutional_color("hollow_grove.house.sandmanor.highlight"))


func _draw_niri_layout_guides(viewport_size: Vector2) -> void:
	if not bool(pair_state.get("paired_window_mode", false)):
		return
	var dimensions: Dictionary = _dict_or_empty(coordinate_contract.get("niri_layout_dimensions"))
	var horizontal: Array = dimensions.get("horizontal_column_presets", [])
	var vertical: Array = dimensions.get("vertical_window_presets", [])
	var guide_color := _constitutional_color("hollow_grove.house.flynt.highlight")
	guide_color.a = 0.10
	for ratio in horizontal:
		var x := float(ratio) * viewport_size.x
		draw_line(Vector2(x, 0.0), Vector2(x, viewport_size.y), guide_color, 1.0, true)
	for ratio in vertical:
		var y := float(ratio) * viewport_size.y
		draw_line(Vector2(0.0, y), Vector2(viewport_size.x, y), guide_color, 1.0, true)


func _draw_pair_overlay(viewport_size: Vector2) -> void:
	if not bool(pair_state.get("paired_window_mode", false)):
		return

	var diagonal_angle: float = deg_to_rad(float(pair_state.get("diagonal_angle_degrees", 135.0)))
	var spread_ratio: float = clampf(float(pair_state.get("spread_ratio", 0.25)), 0.25, 1.0)
	var center: Vector2 = viewport_size * 0.5
	var direction: Vector2 = Vector2(cos(diagonal_angle), sin(diagonal_angle)).normalized()
	var normal: Vector2 = Vector2(-direction.y, direction.x).normalized()
	var diagonal_length: float = viewport_size.length()
	var gap_size: float = min(viewport_size.x, viewport_size.y) * (0.06 + (0.06 * spread_ratio))

	var line_start_a: Vector2 = center - direction * diagonal_length + normal * (gap_size * 0.5)
	var line_end_a: Vector2 = center + direction * diagonal_length + normal * (gap_size * 0.5)
	var line_start_b: Vector2 = center - direction * diagonal_length - normal * (gap_size * 0.5)
	var line_end_b: Vector2 = center + direction * diagonal_length - normal * (gap_size * 0.5)

	draw_line(line_start_a, line_end_a, _constitutional_color("hollow_grove.house.flynt.highlight", 0.85), 4.0, true)
	draw_line(line_start_b, line_end_b, _constitutional_color("hollow_grove.house.flynt.highlight", 0.85), 4.0, true)

	var paired_window: Dictionary = _dict_or_empty(pair_state.get("focused_window"))
	_draw_pair_geometry(viewport_size)
	var window_title := str(paired_window.get("title", "Paired Window"))
	var app_id := str(paired_window.get("app_id", "(none)"))
	var preview_rect := _pair_preview_rect(viewport_size, center, normal, gap_size)
	var hueman_label_position: Vector2 = center - normal * (gap_size * 1.1)
	var application_attachment := _application_attachment()
	var desktop_label := "Desktop Pair"
	var layer_label := "Hueman Layer"
	var mode_label := "paired mode"
	if not application_attachment.is_empty():
		desktop_label = "Glaüshouse Clinical Surface"
		layer_label = "Hollow Grove Control"
		mode_label = "managed application"

	_draw_pair_preview(preview_rect)
	_draw_label(desktop_label, preview_rect.position + Vector2(0.0, -18.0), 28, _constitutional_color("hollow_grove.house.flynt.highlight"))
	_draw_label(window_title, preview_rect.position + Vector2(0.0, preview_rect.size.y + 28.0), 24, _constitutional_color("hollow_grove.house.flynt.highlight"))
	_draw_label(app_id, preview_rect.position + Vector2(0.0, preview_rect.size.y + 58.0), 20, _constitutional_color("hollow_grove.house.flynt.authority"))

	_draw_label(layer_label, hueman_label_position + Vector2(-90.0, -10.0), 28, _constitutional_color("hollow_grove.house.flynt.highlight"))
	_draw_label(str(current_resolution.get("name", "(zone)")), hueman_label_position + Vector2(-90.0, 22.0), 24, _constitutional_color("hollow_grove.house.flynt.highlight"))
	_draw_label(mode_label, hueman_label_position + Vector2(-90.0, 52.0), 20, _constitutional_color("hollow_grove.house.flynt.authority"))


func _pair_preview_rect(viewport_size: Vector2, center: Vector2, normal: Vector2, gap_size: float) -> Rect2:
	var preview_max_size := Vector2(viewport_size.x * 0.28, viewport_size.y * 0.28)
	var preview_size := preview_max_size

	if pair_preview_texture != null:
		var texture_size: Vector2 = pair_preview_texture.get_size()
		if texture_size.x > 0.0 and texture_size.y > 0.0:
			var scale: float = min(preview_max_size.x / texture_size.x, preview_max_size.y / texture_size.y)
			preview_size = texture_size * scale

	var preview_center := center + normal * (gap_size * 2.0 + preview_size.length() * 0.35)
	var preview_position := preview_center - (preview_size * 0.5)
	preview_position.x = clampf(preview_position.x, 24.0, viewport_size.x - preview_size.x - 24.0)
	preview_position.y = clampf(preview_position.y, 48.0, viewport_size.y - preview_size.y - 96.0)

	return Rect2(preview_position, preview_size)


func _draw_pair_geometry(viewport_size: Vector2) -> void:
	var normalized := _dict_or_empty(pair_state.get("normalized"))
	var rect := _dict_or_empty(normalized.get("rect"))
	var width := float(rect.get("width", 0.0))
	var height := float(rect.get("height", 0.0))
	if width <= 0.0 or height <= 0.0:
		return
	var pair_rect := Rect2(
		Vector2(float(rect.get("x", 0.0)) * viewport_size.x, float(rect.get("y", 0.0)) * viewport_size.y),
		Vector2(width * viewport_size.x, height * viewport_size.y)
	)
	var geometry_color := _constitutional_color("hollow_grove.house.glaushouse.highlight")
	geometry_color.a = 0.72
	draw_rect(pair_rect, geometry_color, false, 3.0)


func _draw_pair_preview(preview_rect: Rect2) -> void:
	var frame_rect := preview_rect.grow(10.0)
	draw_rect(frame_rect, _constitutional_color("hollow_grove.universal.outline", 0.84), true)
	draw_rect(frame_rect, _constitutional_color("hollow_grove.house.flynt.highlight", 0.92), false, 2.0)
	var application_attachment := _application_attachment()
	var privacy := _dict_or_empty(application_attachment.get("privacy"))
	var managed_identity_missing_attachment := _managed_identity_missing_attachment()
	if managed_identity_missing_attachment or (not application_attachment.is_empty() and not bool(privacy.get("capture_allowed", false))):
		draw_rect(preview_rect, _constitutional_color("hollow_grove.house.glaushouse.dark", 0.96), true)
		draw_rect(preview_rect, _constitutional_color("hollow_grove.house.glaushouse.highlight", 0.92), false, 2.0)
		var managed_name := "Managed application" if application_attachment.is_empty() else str(application_attachment.get("canonical_name", "Managed application"))
		var projection_label := "Registry attachment required" if managed_identity_missing_attachment else "Semantic-only clinical projection"
		_draw_label(managed_name, preview_rect.position + Vector2(18.0, preview_rect.size.y * 0.45), 28, _constitutional_color("hollow_grove.house.glaushouse.highlight"))
		_draw_label(projection_label, preview_rect.position + Vector2(18.0, preview_rect.size.y * 0.58), 18, _constitutional_color("hollow_grove.house.glaushouse.highlight"))
		_draw_label("Window capture disabled", preview_rect.position + Vector2(18.0, preview_rect.size.y * 0.68), 16, _constitutional_color("hollow_grove.house.flynt.highlight"))
		return

	if pair_preview_texture != null:
		draw_texture_rect(pair_preview_texture, preview_rect, false)
		return

	draw_rect(preview_rect, _constitutional_color("hollow_grove.universal.shadow.deep", 0.88), true)
	draw_rect(preview_rect, _constitutional_color("hollow_grove.house.flynt.authority", 0.80), false, 1.0)
	_draw_label("Preview waiting", preview_rect.position + Vector2(18.0, preview_rect.size.y * 0.5), 22, _constitutional_color("hollow_grove.house.flynt.highlight"))


func _draw_pair_transition(viewport_size: Vector2) -> void:
	if pair_transition_elapsed >= pair_transition_duration or pair_transition_duration <= 0.0:
		return

	var progress := pair_transition_elapsed / pair_transition_duration
	var alpha := sin(progress * PI)
	if alpha <= 0.0:
		return

	var card_size := Vector2(min(viewport_size.x * 0.42, 760.0), 94.0)
	var card_rect := Rect2(
		Vector2((viewport_size.x - card_size.x) * 0.5, viewport_size.y * 0.08),
		card_size
	)
	var fill_color := _constitutional_color("hollow_grove.universal.shadow.deep")
	fill_color.a = 0.82 * alpha
	var outline_color := _constitutional_color("hollow_grove.house.flynt.highlight")
	outline_color.a = 0.96 * alpha
	var title_color := _constitutional_color("hollow_grove.house.flynt.highlight")
	title_color.a = 0.98 * alpha
	var detail_color := _constitutional_color("hollow_grove.house.flynt.authority")
	detail_color.a = 0.88 * alpha

	draw_rect(card_rect, fill_color, true)
	draw_rect(card_rect, outline_color, false, 2.0)
	_draw_label(pair_transition_message, card_rect.position + Vector2(22.0, 34.0), 28, title_color)
	_draw_label(pair_transition_detail, card_rect.position + Vector2(22.0, 66.0), 18, detail_color)


func _draw_asset_preview(viewport_size: Vector2) -> void:
	var panel_size := Vector2(min(viewport_size.x * 0.22, 460.0), min(viewport_size.y * 0.28, 360.0))
	var panel_rect := Rect2(
		Vector2(viewport_size.x - panel_size.x - 28.0, viewport_size.y - panel_size.y - 28.0),
		panel_size
	)
	var frame_rect := panel_rect.grow(10.0)

	draw_rect(frame_rect, _constitutional_color("hollow_grove.universal.outline", 0.88), true)
	draw_rect(frame_rect, _constitutional_color("hollow_grove.house.flynt.highlight", 0.90), false, 2.0)
	_draw_label("Aseprite Live Preview", panel_rect.position + Vector2(0.0, -16.0), 24, _constitutional_color("hollow_grove.house.flynt.highlight"))

	if asset_preview_texture != null:
		var texture_size: Vector2 = asset_preview_texture.get_size()
		if texture_size.x > 0.0 and texture_size.y > 0.0:
			var scale: float = min(panel_rect.size.x / texture_size.x, panel_rect.size.y / texture_size.y)
			var draw_size: Vector2 = texture_size * scale
			var draw_position: Vector2 = panel_rect.position + ((panel_rect.size - draw_size) * 0.5)
			draw_texture_rect(asset_preview_texture, Rect2(draw_position, draw_size), false)
		_draw_label(asset_preview_name, panel_rect.position + Vector2(10.0, panel_rect.size.y + 28.0), 18, _constitutional_color("hollow_grove.house.flynt.authority"))
		return

	draw_rect(panel_rect, _constitutional_color("hollow_grove.universal.shadow.deep", 0.88), true)
	draw_rect(panel_rect, _constitutional_color("hollow_grove.house.flynt.authority", 0.80), false, 1.0)
	_draw_label("No sprite exports yet", panel_rect.position + Vector2(18.0, panel_rect.size.y * 0.5), 22, _constitutional_color("hollow_grove.house.flynt.highlight"))
	_draw_label("Save an .aseprite file to preview it here", panel_rect.position + Vector2(18.0, panel_rect.size.y * 0.5 + 30.0), 16, _constitutional_color("hollow_grove.house.flynt.authority"))


func _node_point(node: Dictionary) -> Vector2:
	return Vector2(float(node.get("x", 0.0)), float(node.get("y", 0.0)))


func _route_from_point(route: Dictionary) -> Vector2:
	return _named_node_point(str(route.get("from", "")))


func _route_to_point(route: Dictionary) -> Vector2:
	return _named_node_point(str(route.get("to", "")))


func _named_node_point(node_id: String) -> Vector2:
	for node in map_contract.get("nodes", []):
		if str(node.get("id", "")) == node_id:
			return _node_point(node)
	return Vector2.ZERO


func _adaptive_route_points(route: Dictionary, sample_count: int) -> Array[Vector2]:
	var route_id := str(route.get("constitutional_route_id", ""))
	var slot := _adaptive_morph_slot(route_id)
	if slot.is_empty() or is_zero_approx(adaptive_rectangularity):
		var control_data := _dict_or_empty(route.get("control"))
		if not control_data.is_empty():
			return _curve_points(route, sample_count)
		var straight_points: Array[Vector2] = []
		straight_points.append(_route_from_point(route))
		straight_points.append(_route_to_point(route))
		return straight_points

	var start := _route_from_point(route)
	var ending := _route_to_point(route)
	var circular_midpoint := _adaptive_contract_point(_dict_or_empty(slot.get("circular_midpoint")))
	var rectangular_position := _adaptive_contract_point(_dict_or_empty(slot.get("rectangular_position")))
	var circular_first_control := (start + circular_midpoint) * 0.5
	var circular_second_control := (circular_midpoint + ending) * 0.5
	var control_data := _dict_or_empty(route.get("control"))
	if not control_data.is_empty():
		var control := Vector2(float(control_data.get("x", 0.0)), float(control_data.get("y", 0.0)))
		circular_first_control = (start + control) * 0.5
		circular_second_control = (control + ending) * 0.5

	var adaptive_midpoint := circular_midpoint.lerp(rectangular_position, adaptive_rectangularity)
	var rectangular_first_control := (start + rectangular_position) * 0.5
	var rectangular_second_control := (rectangular_position + ending) * 0.5
	var adaptive_first_control := circular_first_control.lerp(rectangular_first_control, adaptive_rectangularity)
	var adaptive_second_control := circular_second_control.lerp(rectangular_second_control, adaptive_rectangularity)
	var segment_samples := maxi(2, int(sample_count / 2))
	var points: Array[Vector2] = []
	_append_quadratic_points(points, start, adaptive_first_control, adaptive_midpoint, segment_samples, true)
	_append_quadratic_points(points, adaptive_midpoint, adaptive_second_control, ending, segment_samples, false)
	return points


func _adaptive_morph_slot(route_id: String) -> Dictionary:
	if route_id == "" or adaptive_morph_contract.is_empty():
		return {}
	for hinge in adaptive_morph_contract.get("hinges", []):
		for slot in hinge.get("slots", []):
			if str(slot.get("route_id", "")) == route_id:
				return slot
	return {}


func _adaptive_contract_point(point: Dictionary) -> Vector2:
	var unit := float(adaptive_morph_contract.get("normalization_unit", 1000000000))
	return Vector2(
		float(point.get("x_parts", 0)) / unit,
		float(point.get("y_parts", 0)) / unit
	)


func _adaptive_midway_position(slot: Dictionary) -> Vector2:
	var circular := _adaptive_contract_point(_dict_or_empty(slot.get("circular_midpoint")))
	var rectangular := _adaptive_contract_point(_dict_or_empty(slot.get("rectangular_position")))
	return circular.lerp(rectangular, adaptive_rectangularity)


func _append_quadratic_points(
	points: Array[Vector2],
	start: Vector2,
	control: Vector2,
	ending: Vector2,
	sample_count: int,
	include_start: bool
) -> void:
	var first_index := 0 if include_start else 1
	for index in range(first_index, sample_count + 1):
		var t := float(index) / float(sample_count)
		points.append(
			((1.0 - t) * (1.0 - t) * start) +
			(2.0 * (1.0 - t) * t * control) +
			((t * t) * ending)
		)


func _curve_points(route: Dictionary, sample_count: int) -> Array[Vector2]:
	var points: Array[Vector2] = []
	var start := _route_from_point(route)
	var control_data: Dictionary = route.get("control", {})
	var control := Vector2(float(control_data.get("x", 0.0)), float(control_data.get("y", 0.0)))
	var ending := _route_to_point(route)

	for index in range(sample_count + 1):
		var t := float(index) / float(sample_count)
		points.append(
			((1.0 - t) * (1.0 - t) * start) +
			(2.0 * (1.0 - t) * t * control) +
			((t * t) * ending)
		)

	return points


func _distance_to_polyline(point: Vector2, polyline: Array[Vector2]) -> float:
	if polyline.size() < 2:
		return INF

	var best_distance := INF
	for index in range(polyline.size() - 1):
		best_distance = min(best_distance, _distance_to_segment(point, polyline[index], polyline[index + 1]))
	return best_distance


func _distance_to_segment(point: Vector2, segment_start: Vector2, segment_end: Vector2) -> float:
	var segment: Vector2 = segment_end - segment_start
	var segment_length_squared: float = segment.length_squared()
	if segment_length_squared <= 0.000001:
		return point.distance_to(segment_start)

	var t: float = clampf((point - segment_start).dot(segment) / segment_length_squared, 0.0, 1.0)
	var projection: Vector2 = segment_start + (segment * t)
	return point.distance_to(projection)


func _to_screen(point: Vector2, viewport_size: Vector2) -> Vector2:
	return Vector2(point.x * viewport_size.x, point.y * viewport_size.y)


func _draw_label(text: String, position: Vector2, font_size: int, color: Color) -> void:
	if ThemeDB.fallback_font == null:
		return
	draw_string(ThemeDB.fallback_font, position, text, HORIZONTAL_ALIGNMENT_LEFT, -1.0, font_size, color)


func _resolve_motion_grid_cell(probe: Vector2) -> Dictionary:
	var motion_cells := _motion_grid_cells()
	if motion_cells.is_empty():
		return {}

	var best_cell: Dictionary = {}
	var best_distance := INF

	for cell in motion_cells:
		if typeof(cell) != TYPE_DICTIONARY:
			continue
		var cell_dict: Dictionary = cell
		var cell_point := Vector2(float(cell_dict.get("x", 0.0)), float(cell_dict.get("y", 0.0)))
		var distance := probe.distance_to(cell_point)
		if distance < best_distance:
			best_distance = distance
			best_cell = cell_dict

	if best_cell.is_empty():
		return {}

	return {
		"id": str(best_cell.get("id", "")),
		"name": str(best_cell.get("name", "")),
		"kind": "motion_grid_cell"
	}


func _motion_grid_cells() -> Array:
	var shared_anchors := _dict_or_empty(coordinate_contract.get("shared_anchors"))
	var motion_grid: Array = shared_anchors.get("motion_grid", [])
	if motion_grid is Array and not motion_grid.is_empty():
		return motion_grid

	var overlay: Array = map_contract.get("motion_grid_overlay", [])
	if overlay is Array:
		return overlay
	return []


func _dict_or_empty(value: Variant) -> Dictionary:
	if typeof(value) == TYPE_DICTIONARY:
		return value
	return {}


func _application_attachment() -> Dictionary:
	if not bool(pair_state.get("paired_window_mode", false)):
		return {}
	var application_attachment := _dict_or_empty(pair_state.get("application_attachment"))
	if str(application_attachment.get("lifecycle", "")) != "attached":
		return {}
	return application_attachment


func _managed_identity_missing_attachment() -> bool:
	if not bool(pair_state.get("paired_window_mode", false)):
		return false
	var paired_window := _dict_or_empty(pair_state.get("focused_window"))
	var app_id := str(paired_window.get("app_id", ""))
	return app_id.begins_with("hollow-grove.") and _application_attachment().is_empty()


func _refresh_pair_transition_state() -> void:
	var pair_active := bool(pair_state.get("paired_window_mode", false))
	var binding_status := str(pair_state.get("binding_status", ""))
	var binding_source := str(pair_state.get("binding_source", ""))
	var paired_window := _dict_or_empty(pair_state.get("focused_window"))
	var pair_window_id := int(paired_window.get("id", -1))
	var pair_title := str(paired_window.get("title", "Paired Window"))
	var application_attachment := _application_attachment()
	var application_name := str(application_attachment.get("canonical_name", ""))

	if pair_active and pair_window_id >= 0 and pair_window_id != last_pair_window_id:
		if not application_attachment.is_empty():
			_start_pair_transition("Clinical Surface Attached", "%s -> Glaüshouse" % application_name)
		elif last_pair_window_id >= 0:
			_start_pair_transition("Hueman Actor Rebound", pair_title)
		else:
			_start_pair_transition("Hueman Actor Bound", pair_title)
	elif pair_active and binding_source == "rebind-hueman-pair" and binding_source != last_pair_binding_source:
		_start_pair_transition("Hueman Actor Rebound", pair_title)
	elif not pair_active and last_pair_mode_active and binding_status == "released":
		_start_pair_transition("Hueman Actor Released", "Focused-window probe restored")

	last_pair_window_id = pair_window_id if pair_active else -1
	last_pair_binding_status = binding_status
	last_pair_binding_source = binding_source
	last_pair_mode_active = pair_active


func _start_pair_transition(message: String, detail: String) -> void:
	pair_transition_elapsed = 0.0
	pair_transition_duration = 1.4
	pair_transition_message = message
	pair_transition_detail = detail


func _paired_probe_is_active() -> bool:
	return bool(pair_state.get("paired_window_mode", false)) and not _dict_or_empty(pair_state.get("normalized")).get("center", {}).is_empty()


func _active_normalized_probe() -> Dictionary:
	if _paired_probe_is_active():
		return _dict_or_empty(pair_state.get("normalized"))
	return _dict_or_empty(live_state.get("normalized"))


func _active_spatial_viewpoint() -> Dictionary:
	if _paired_probe_is_active():
		return _dict_or_empty(pair_state.get("spatial_viewpoint"))
	return _dict_or_empty(live_state.get("spatial_viewpoint"))


func _active_probe_window() -> Dictionary:
	if _paired_probe_is_active():
		return _dict_or_empty(pair_state.get("focused_window"))
	return _dict_or_empty(live_state.get("focused_window"))


func _active_probe_source() -> String:
	if _paired_probe_is_active():
		return str(pair_state.get("probe_source", "paired_window_center"))
	return "focused_window_center"


func _reload_pair_preview_texture() -> void:
	var application_attachment := _application_attachment()
	var privacy := _dict_or_empty(application_attachment.get("privacy"))
	if _managed_identity_missing_attachment() or (not application_attachment.is_empty() and not bool(privacy.get("capture_allowed", false))):
		pair_preview_texture = null
		pair_preview_mtime = -1
		return

	var image_path := _repo_file(PAIR_PREVIEW_IMAGE_RELATIVE_PATH)
	if not FileAccess.file_exists(image_path):
		pair_preview_texture = null
		pair_preview_mtime = -1
		return

	var modified_time := FileAccess.get_modified_time(image_path)
	if modified_time == pair_preview_mtime and pair_preview_texture != null:
		return

	var image := Image.new()
	var load_error := image.load(image_path)
	if load_error != OK:
		pair_preview_texture = null
		pair_preview_mtime = -1
		return

	pair_preview_texture = ImageTexture.create_from_image(image)
	pair_preview_mtime = modified_time


func _reload_asset_preview_texture() -> void:
	var export_dir := _repo_file(ASSET_EXPORT_RELATIVE_PATH)
	var export_png_paths := _collect_export_png_paths(export_dir)
	asset_preview_count = export_png_paths.size()

	if export_png_paths.is_empty():
		asset_preview_texture = null
		asset_preview_path = ""
		asset_preview_mtime = -1
		asset_preview_name = ""
		return

	var latest_path := _latest_modified_path(export_png_paths)
	if latest_path == "":
		asset_preview_texture = null
		asset_preview_path = ""
		asset_preview_mtime = -1
		asset_preview_name = ""
		return

	var modified_time := FileAccess.get_modified_time(latest_path)
	if latest_path == asset_preview_path and modified_time == asset_preview_mtime and asset_preview_texture != null:
		return

	var image := Image.new()
	var load_error := image.load(latest_path)
	if load_error != OK:
		asset_preview_texture = null
		asset_preview_path = latest_path
		asset_preview_mtime = modified_time
		asset_preview_name = latest_path.get_file()
		return

	asset_preview_texture = ImageTexture.create_from_image(image)
	asset_preview_path = latest_path
	asset_preview_mtime = modified_time
	asset_preview_name = latest_path.get_file()


func _collect_export_png_paths(root_path: String) -> Array[String]:
	var png_paths: Array[String] = []
	var directory := DirAccess.open(root_path)
	if directory == null:
		return png_paths

	_collect_export_png_paths_recursive(directory, root_path, png_paths)
	return png_paths


func _collect_export_png_paths_recursive(directory: DirAccess, directory_path: String, png_paths: Array[String]) -> void:
	directory.list_dir_begin()
	while true:
		var entry := directory.get_next()
		if entry == "":
			break
		if entry == "." or entry == "..":
			continue

		var entry_path := directory_path.path_join(entry)
		if directory.current_is_dir():
			var child_directory := DirAccess.open(entry_path)
			if child_directory != null:
				_collect_export_png_paths_recursive(child_directory, entry_path, png_paths)
		elif entry.get_extension().to_lower() == "png":
			png_paths.append(entry_path)
	directory.list_dir_end()


func _latest_modified_path(paths: Array[String]) -> String:
	var latest_path := ""
	var latest_mtime := -1

	for path in paths:
		var modified_time := FileAccess.get_modified_time(path)
		if modified_time > latest_mtime:
			latest_mtime = modified_time
			latest_path = path

	return latest_path


func _format_decimal(value: float) -> String:
	return "%.3f" % value
