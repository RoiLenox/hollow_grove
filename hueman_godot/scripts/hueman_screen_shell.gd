extends Node2D

const MAP_CONTRACT_RELATIVE_PATH := "artifacts/hueman_screen_map.json"
const COORDINATE_CONTRACT_RELATIVE_PATH := "artifacts/hollow_grove_hueman_coordinate_contract.json"
const VISUAL_COLOR_CONSTITUTION_RELATIVE_PATH := "src/constitutional/hollow_grove_visual_color_palette.json"
const LIVE_STATE_RELATIVE_PATH := "artifacts/screen_map_state.json"
const INTENT_RELATIVE_PATH := "artifacts/screen_map_intent.json"
const PAIR_STATE_RELATIVE_PATH := "artifacts/hueman_pair_state.json"
const PAIR_PREVIEW_IMAGE_RELATIVE_PATH := "artifacts/hueman_pair_preview.png"
const PAIR_PREVIEW_STATE_RELATIVE_PATH := "artifacts/hueman_pair_preview_state.json"
const ASSET_EXPORT_RELATIVE_PATH := "hueman_godot/assets/export"
const POLL_INTERVAL_SECONDS := 0.25

var repo_root_path := ""
var map_contract := {}
var coordinate_contract := {}
var visual_color_constitution := {}
var colors_by_semantic_identity := {}
var live_state := {}
var pair_state := {}
var pair_preview_state := {}
var pair_preview_texture = null
var pair_preview_mtime := -1
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


func _ready() -> void:
	repo_root_path = _resolve_repo_root()
	if not _load_visual_color_constitution():
		push_error("The canonical Hollow Grove visual color constitution could not be loaded.")
		set_process(false)
		set_process_input(false)
		return
	_ensure_hud()
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
	var aura_beach_color: Color = _constitutional_color("hollow_grove.house.glaushouse.highlight")
	aura_beach_color.a = 0.18
	var aura_basin_color: Color = _constitutional_color("hollow_grove.house.stonebend.highlight")
	aura_basin_color.a = 0.18

	_draw_surface("aura_field", aura_field_color, viewport_size)
	_draw_surface("aura_beach", aura_beach_color, viewport_size)
	_draw_surface("aura_basin", aura_basin_color, viewport_size)

	_draw_motion_grid_overlay(viewport_size)
	_draw_straight_routes(viewport_size)
	_draw_curved_routes(viewport_size)
	_draw_nodes(viewport_size)
	_draw_surface_labels(viewport_size)
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


func _load_all_state() -> void:
	map_contract = _read_json_file(_repo_file(MAP_CONTRACT_RELATIVE_PATH))
	coordinate_contract = _read_json_file(_repo_file(COORDINATE_CONTRACT_RELATIVE_PATH))
	live_state = _read_json_file(_repo_file(LIVE_STATE_RELATIVE_PATH))
	pair_state = _read_json_file(_repo_file(PAIR_STATE_RELATIVE_PATH))
	pair_preview_state = _read_json_file(_repo_file(PAIR_PREVIEW_STATE_RELATIVE_PATH))
	_reload_pair_preview_texture()
	_reload_asset_preview_texture()
	_refresh_pair_transition_state()
	current_resolution = _resolve_current_zone()
	_update_hud()
	queue_redraw()


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

	hint_label.text = "Enter writes an inspect intent. Space writes a move intent. Super+Alt+Enter opens Hueman. Super+Alt+Control+Enter attaches the focused window. Super+Alt+Backspace detaches it. Super+Alt+Shift+Space steps spread by 25%."


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

	var probe := Vector2(float(center.get("x", 0.0)), float(center.get("y", 0.0)))
	var nodes: Array = map_contract.get("nodes", [])
	var straight_routes: Array = map_contract.get("straight_routes", [])
	var curved_routes: Array = map_contract.get("curved_routes", [])
	var surfaces: Array = map_contract.get("surfaces", [])

	for node in nodes:
		var node_point := _node_point(node)
		var radius := float(node.get("radius", 0.09))
		if probe.distance_to(node_point) <= radius:
			return {
				"id": str(node.get("id", "")),
				"name": str(node.get("name", "")),
				"kind": str(node.get("kind", "kingdom"))
			}

	for route in straight_routes:
		if _distance_to_segment(probe, _route_from_point(route), _route_to_point(route)) <= float(route.get("band_width", 0.045)):
			return {
				"id": str(route.get("id", "")),
				"name": str(route.get("name", "")),
				"kind": "straight_route"
			}

	for route in curved_routes:
		if _distance_to_polyline(probe, _curve_points(route, 32)) <= float(route.get("band_width", 0.045)):
			return {
				"id": str(route.get("id", "")),
				"name": str(route.get("name", "")),
				"kind": "curved_route"
			}

	for surface in surfaces:
		var polygon := PackedVector2Array()
		for point in surface.get("polygon", []):
			polygon.append(Vector2(float(point.get("x", 0.0)), float(point.get("y", 0.0))))
		if Geometry2D.is_point_in_polygon(probe, polygon):
			return {
				"id": str(surface.get("id", "")),
				"name": str(surface.get("name", "")),
				"kind": "surface"
			}

	var motion_grid_resolution := _resolve_motion_grid_cell(probe)
	if not motion_grid_resolution.is_empty():
		return motion_grid_resolution

	var nearest_route := _nearest_route_name(probe)
	return {
		"id": nearest_route.to_snake_case(),
		"name": nearest_route,
		"kind": "nearest_route"
	}


func _nearest_route_name(probe: Vector2) -> String:
	var best_name := "Aura Ridge"
	var best_distance := INF

	for route in map_contract.get("straight_routes", []):
		var distance := _distance_to_segment(probe, _route_from_point(route), _route_to_point(route))
		if distance < best_distance:
			best_distance = distance
			best_name = str(route.get("name", best_name))

	for route in map_contract.get("curved_routes", []):
		var distance := _distance_to_polyline(probe, _curve_points(route, 32))
		if distance < best_distance:
			best_distance = distance
			best_name = str(route.get("name", best_name))

	return best_name


func _write_intent(intent_kind: String) -> void:
	if current_resolution.is_empty():
		return

	var payload := {
		"schema_version": "0.1.0",
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
		if str(node.get("id", "")) == "glaushouse":
			label_offset = Vector2(-radius * 0.9, radius * 1.5)
		_draw_label(str(node.get("name", "")), point + label_offset, 28, _constitutional_color("hollow_grove.house.flynt.highlight"))


func _draw_straight_routes(viewport_size: Vector2) -> void:
	for route in map_contract.get("straight_routes", []):
		var points := PackedVector2Array([
			_to_screen(_route_from_point(route), viewport_size),
			_to_screen(_route_to_point(route), viewport_size)
		])
		draw_polyline(points, _constitutional_color("hollow_grove.house.stonebend.primary"), 5.0, true)


func _draw_curved_routes(viewport_size: Vector2) -> void:
	for route in map_contract.get("curved_routes", []):
		var points := PackedVector2Array()
		for point in _curve_points(route, 32):
			points.append(_to_screen(point, viewport_size))
		draw_polyline(points, _constitutional_color("hollow_grove.house.glaushouse.primary"), 4.0, true)


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
		_draw_label(
			str(surface.get("name", "")),
			_to_screen(center, viewport_size) + Vector2(-70.0, 8.0),
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
