extends SceneTree
## Source-checkout design preview; does not load the game, service, or user state.

const MANIFEST_RELATIVE := "../docs/design/hollow_grove_chromatic_color_standard_v2_3_0.json"


class Swatches extends Control:

	var standard: Dictionary
	var growth: Dictionary
	var ink: Color
	var paper: Color
	var font: Font


	func _ready() -> void:
		ink = Color.from_string(standard["universal"][0]["hex"], Color.TRANSPARENT)
		paper = Color.from_string(standard["universal"][1]["hex"], Color.TRANSPARENT)
		font = ThemeDB.fallback_font
		resized.connect(queue_redraw)


	func _text(value: String, x: float, y: float, font_size: int = 14) -> void:
		draw_string(font, Vector2(x, y), value, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, ink)


	func _swatch(token: Dictionary, x: float, y: float, width: float) -> void:
		var rectangle := Rect2(x, y, width, 26)
		draw_rect(rectangle, Color.from_string(token["hex"], Color.TRANSPARENT))
		draw_rect(rectangle, ink, false, 1)
		_text(str(token["name"]) if token["name"] != null else "UNASSIGNED", x, y + 44, 13)
		_text(token["hex"], x + width - 68, y + 44, 12)


	func _draw() -> void:
		if standard.is_empty():
			return
		draw_rect(Rect2(Vector2.ZERO, size), paper)
		var scale_factor := minf(size.x / 1160.0, size.y / 920.0)
		var offset := (size - Vector2(1160, 920) * scale_factor) / 2.0
		draw_set_transform(offset, 0, Vector2.ONE * scale_factor)
		_text("HOLLOW GROVE / HUEMAN", 28, 40, 24)
		_text("Chromatic Color Standard v2.3 · 34 approved colors / 32 visual Form assignments", 28, 67, 16)
		var houses: Array = standard["houses"]
		for column in range(houses.size()):
			var house: Dictionary = houses[column]
			var x := 28.0 + column * 282.0
			_text(house["name"] + " / " + house["family"], x, 106, 19)
			var direction: Dictionary = house["orientation"]
			_text("%s · %s · %s" % [str(direction["cardinal"]).capitalize(),
				str(int(direction["clock"])), direction["faculty"]], x, 128, 13)
			var shades: Array = house["shades"]
			for row in range(shades.size()):
				var y := 152.0 + row * 78.0
				var bound: Dictionary = {}
				for form in house["forms"]:
					if form["swatch_slot"] == shades[row]["slot"]:
						bound = form
				_text("Slot %s · %s (seq %s)" % [row + 1, bound["name"], int(bound["sequence"])], x, y, 14)
				_swatch(shades[row], x, y + 10, 264)
		_text("UNIVERSAL / available to all Houses", 28, 792, 15)
		_swatch(standard["universal"][0], 28, 810, 546)
		_swatch(standard["universal"][1], 592, 810, 546)
		_text("Dark → light swatches stay fixed. Form sequences are separate; Flynt is not a serial evolution ladder.", 28, 884, 13)
		_text("UNASSIGNED = formal name pending. Font preview uses Godot's built-in fallback.", 28, 906, 12)


	func _form(name: String) -> Dictionary:
		for branch in growth["branches"]:
			for form in branch["forms"]:
				if form["name"] == name:
					return form
		return {}


	func validate_view() -> bool:
		return true


class CharacterStudy extends Swatches:
	var stages: Array[ImageTexture] = []
	var images: Array[Image] = []
	var skin_samples: Array[ImageTexture] = []
	var records: Array[Dictionary] = []
	var pilot: Texture2D
	const Art = preload("res://tools/chromatic_study_art.gd")


	func _ready() -> void:
		super._ready()
		texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
		records.append({"name": "Hueman", "height_ratio": 1.0, "illustrative_pixels": 32})
		for name in ["Gremlin", "Goblin", "Gargoyle"]:
			records.append(_form(name))
		for index in range(4):
			var kind: String = ["hueman", "gremlin", "goblin", "gargoyle"][index]
			var pigment := _shade(2, 4) if index == 0 else Color(records[index]["hex"])
			var shade := _shade(3 if index == 3 else 0, 2)
			var art: Image = Art.body(kind, int(records[index]["illustrative_pixels"]), pigment, shade)
			images.append(art)
			stages.append(ImageTexture.create_from_image(art))
		pilot = load("res://assets/export/characters/player/hueman_gargoyle_pilot.png")
		for skin_index in range(3):
			skin_samples.append(ImageTexture.create_from_image(Art.body("hueman", 32, _shade(2, 4), _shade(0, 2), skin_index)))


	func _shade(house_index: int, level: int) -> Color:
		return Color(standard["houses"][house_index]["shades"][level - 1]["hex"])


	func validate_view() -> bool:
		if pilot == null or pilot.get_size() != Vector2(32, 32) or images.size() != 4 or skin_samples.size() != 3:
			return false
		for index in range(4):
			var expected_height := int(records[index]["illustrative_pixels"])
			var used := images[index].get_used_rect()
			if images[index].get_height() != expected_height or used.position.y != 0 or used.end.y != expected_height:
				return false
			if index > 0:
				var contains_identity := false
				for y in range(images[index].get_height()):
					for x in range(images[index].get_width()):
						if images[index].get_pixel(x, y).to_html(false).to_upper() == str(records[index]["hex"]).trim_prefix("#"):
							contains_identity = true
				if not contains_identity:
					return false
		print("Four new in-memory poses: 32/32/36/40px standing examples, shared feet pivot, canonical pigments")
		return true


	func _draw() -> void:
		if stages.size() != 4:
			return
		draw_rect(Rect2(Vector2.ZERO, size), paper)
		_text("HUEMAN / CHROMATIC GROWTH STUDY 02", 28, 38, 25)
		_text("v2.3 color identities approved · v0.4 proportions WORKING · fresh anatomy, common pixel density", 28, 65, 16)
		_text("Hueman → Gremlin → Goblin     |     Gremlin → Gargoyle is the direct alternate branch", 28, 94, 17)
		var notes := ["Natural individual skin", "Long ears / narrow thorax", "Brow, jaw and rib cage", "Horns / forearm shields"]
		for index in range(4):
			var x := 28.0 + index * 312.0
			var record: Dictionary = records[index]
			_text(record["name"], x, 137, 23)
			_text("Personalized flesh tones" if index == 0 else str(record["color_name"] if record["color_name"] != null else "UNASSIGNED") + " · " + record["hex"], x, 163, 14)
			_text("%.3f× · %spx standing example" % [float(record["height_ratio"]), int(record["illustrative_pixels"])], x, 188, 15)
			_text("No House swatch / no Growth Index" if index == 0 else "GI %s · sequence %s · swatch %s" % [int(record["growth_index"]), int(record["sequence"]), int(record["swatch_level"])], x, 211, 13)
			var dimensions := stages[index].get_size()
			draw_texture_rect(stages[index], Rect2(x + 130 - dimensions.x * 2.5, 445 - dimensions.y * 5, dimensions.x * 5, dimensions.y * 5), false)
			_text(notes[index], x, 478, 15)
			_text("1× native     2× pixels          2× / dark sample", x, 521, 13)
			draw_texture_rect(stages[index], Rect2(x + 12, 660 - dimensions.y, dimensions.x, dimensions.y), false)
			draw_texture_rect(stages[index], Rect2(x + 70, 660 - dimensions.y * 2, dimensions.x * 2, dimensions.y * 2), false)
			draw_rect(Rect2(x + 172, 548, 112, 114), _shade(0, 1))
			draw_texture_rect(stages[index], Rect2(x + 228 - dimensions.x, 660 - dimensions.y * 2, dimensions.x * 2, dimensions.y * 2), false)
		draw_line(Vector2(28, 445), Vector2(1248, 445), ink, 1)
		_text("Shared ground plane / every pose shown at 5× nearest pixels", 28, 700, 14)
		_text("Personal skin examples (not House tokens)", 28, 740, 15)
		for skin_index in range(3):
			draw_texture_rect(skin_samples[skin_index], Rect2(28 + skin_index * 70, 759, 48, 64), false)
		_text("Historical pilot / unchanged 16×16 frames", 370, 740, 15)
		for index in range(4):
			draw_texture_rect_region(pilot, Rect2(370 + index * 80, 759, 48, 48), Rect2((index % 2) * 16, (index / 2) * 16, 16, 16))
		_text("32px is an illustrative standing height. Existing production frames remain 16×16; historical 24×32 is a separate study.", 28, 862, 14)
		_text("No approved geometry, new unlocks, hitbox edits or production animation. Twelve formal color names remain pending.", 28, 888, 14)
		_text("Review silhouettes and feet pivots before authored idle/walk frames. Flynt comparisons are not an evolution ladder.", 28, 914, 14)


class GrowthChart extends Swatches:

	func _draw() -> void:
		if growth.is_empty():
			return
		draw_rect(Rect2(Vector2.ZERO, size), paper)
		_text("32 FORMS / WORKING HEX-DISTANCE HEIGHT STUDY", 28, 37, 24)
		_text("v2.3 bindings approved · v0.4 heights provisional · normalized cumulative adjacent sRGB distance", 28, 65, 16)
		var groups := [
			growth["branches"][0]["forms"],
			growth["branches"][1]["forms"] + growth["branches"][2]["forms"],
			growth["branches"][3]["forms"],
			growth["branches"][4]["forms"]]
		var labels := ["STONEBEND · 1.00–2.10×", "SANDMANOR · two independent 1.00–1.40× branches", "GLAÜSHOUSE · 1.00–2.10×", "FLYNT · 1.25–2.50× comparison; NOT sequential unlocks"]
		for row in range(4):
			var top := 98.0 + row * 262.0
			var ground := top + 196
			_text(labels[row], 28, top, 17)
			draw_line(Vector2(28, ground), Vector2(1330, ground), ink, 1)
			draw_dashed_line(Vector2(28, ground - 56), Vector2(1330, ground - 56), Color(0.65, 0.65, 0.65), 1, 4)
			for column in range(8):
				var form: Dictionary = groups[row][column]
				var x := 28.0 + column * 164.0
				_text(form["name"], x, top + 23, 16)
				_text(form["hex"], x, top + 42, 12)
				var height := float(form["illustrative_pixels"]) * 1.75
				draw_rect(Rect2(x + 49, ground - height, 48, height), Color(form["hex"]))
				draw_rect(Rect2(x + 49, ground - height, 48, height), ink, false, 1)
				_text("%.1f%% / %spx" % [float(form["height_ratio"]) * 100, int(form["illustrative_pixels"])], x, ground + 20, 14)
				_text("GI %s%s · seq %s · slot %s" % [int(form["growth_index"]), "*" if form["index_status"] == "working" else "", int(form["sequence"]), int(form["swatch_level"])], x, ground + 39, 12)
			if row == 1:
				draw_dashed_line(Vector2(673, top + 8), Vector2(673, ground + 42), ink, 1, 4)
		_text("Dashed horizontal line = 100% / illustrative 32px. All bars use the same scale; these are measurements, not approved anatomy.", 28, 1150, 14)
		_text("GI* = working Growth Index; only Gremlin 0, Goblin 1 and Gargoyle 2 are creator-directed. All exact heights remain provisional.", 28, 1175, 14)


func _init() -> void:
	call_deferred("_run")


func _run() -> void:
	var base := ProjectSettings.globalize_path("res://")
	var path := base.path_join(MANIFEST_RELATIVE).simplify_path()
	var growth_path := base.path_join("../docs/design/hueman_chromatic_growth_system_v0_4_hex_distance_working.json").simplify_path()
	var parsed: Variant = JSON.parse_string(FileAccess.get_file_as_string(path))
	var projection: Variant = JSON.parse_string(FileAccess.get_file_as_string("res://tools/chromatic_growth_preview.json"))
	if not parsed is Dictionary or parsed.get("version") != "2.3" or parsed.get("status") != "creator-approved-design":
		push_error("Cannot load approved v2.3 design source")
		quit(1)
		return
	if not projection is Dictionary or projection.get("status") != "working-review-only" \
			or projection.get("palette_sha256") != FileAccess.get_sha256(path) \
			or projection.get("growth_sha256") != FileAccess.get_sha256(growth_path):
		push_error("Stale or missing working projection; run python3 tools/chromatic_growth.py project")
		quit(1)
		return
	if parsed.get("houses", []).size() != 4 or parsed.get("universal", []).size() != 2:
		quit(1)
		return
	var count := 2
	for house in parsed["houses"]:
		if house.get("shades", []).size() != 8 or house.get("forms", []).size() != 8:
			quit(1)
			return
		count += house["shades"].size()
	var form_count := 0
	for branch in projection["branches"]:
		form_count += branch["forms"].size()
	if form_count != 32:
		quit(1)
		return
	var arguments := OS.get_cmdline_user_args()
	var is_study := "--study" in arguments
	var is_growth := "--growth" in arguments
	if is_study and is_growth:
		push_error("Choose --study or --growth")
		quit(1)
		return
	var view: Swatches
	if is_study:
		view = CharacterStudy.new()
	elif is_growth:
		view = GrowthChart.new()
	else:
		view = Swatches.new()
	root.title = "Hueman — Chromatic v2.3 / working growth v0.4"
	root.size = Vector2i(1360, 1200) if is_growth else (Vector2i(1280, 960) if is_study else Vector2i(1160, 920))
	root.min_size = root.size
	view.standard = parsed
	view.growth = projection
	root.add_child(view)
	view.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	if not view.validate_view():
		push_error("Preview geometry or identity validation failed")
		quit(1)
		return
	if "--validate-only" in arguments:
		print("PASS: v2.3 preview; %s swatches, %s provisional growth comparisons, historical assets retained" % [count, form_count])
		quit()
		return
	var capture_path := ""
	for argument in arguments:
		if argument.begins_with("--capture="):
			capture_path = argument.trim_prefix("--capture=")
	if capture_path != "":
		if DisplayServer.get_name() == "headless":
			push_error("Capture requires a rendering display; use --validate-only headlessly")
			quit(1)
			return
		await process_frame
		await RenderingServer.frame_post_draw
		var result := root.get_texture().get_image().save_png(capture_path)
		if result != OK:
			push_error("Could not save preview: " + error_string(result))
			quit(1)
			return
		print("Saved chromatic preview: " + capture_path)
		quit()
