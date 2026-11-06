extends RefCounted
## Newly drawn disposable 24x32 humanoid. Fixed anatomy, no growth calculation.
const Appearance = preload("res://tools/hueman_01/appearance.gd")
const StudyArt = preload("res://tools/chromatic_study_art.gd")
const MANIFEST := "res://../docs/design/hollow_grove_chromatic_color_standard_v2_3_0.json"

static func hueman(appearance: Dictionary) -> Image:
	assert(Appearance.valid(appearance))
	var art := Image.create(24, 32, false, Image.FORMAT_RGBA8)
	art.fill(Color.TRANSPARENT)
	var skin := Color(Appearance.entry(Appearance.SKINS, appearance.skin).hex)
	var hair := Color(Appearance.entry(Appearance.HAIR_COLORS, appearance.hair.color).hex)
	var ink := Color("24252a")
	var shadow := skin.darkened(0.19)
	# Hair behind the head; all styles fit the same canvas and body.
	if appearance.hair.style == "long":
		art.fill_rect(Rect2i(6, 1, 12, 16), hair)
	art.fill_rect(Rect2i(8, 1, 8, 9), skin)
	art.fill_rect(Rect2i(7, 4, 1, 4), shadow)
	art.fill_rect(Rect2i(16, 4, 1, 4), shadow)
	art.fill_rect(Rect2i(10, 10, 4, 2), shadow)
	# Clothing and body are fixed; no relationship to House, rank, or ability.
	art.fill_rect(Rect2i(6, 12, 12, 11), Color("77796b"))
	art.fill_rect(Rect2i(5, 13, 2, 6), Color("77796b"))
	art.fill_rect(Rect2i(17, 13, 2, 6), Color("77796b"))
	art.fill_rect(Rect2i(5, 19, 2, 5), skin)
	art.fill_rect(Rect2i(17, 19, 2, 5), skin)
	art.fill_rect(Rect2i(9, 12, 6, 2), Color("d9d1bb"))
	art.fill_rect(Rect2i(6, 21, 12, 2), Color("514b40"))
	art.fill_rect(Rect2i(7, 23, 4, 7), Color("454754"))
	art.fill_rect(Rect2i(13, 23, 4, 7), Color("454754"))
	art.fill_rect(Rect2i(6, 30, 5, 2), ink)
	art.fill_rect(Rect2i(13, 30, 5, 2), ink)
	# Facial marks change expression, never head shape or body proportions.
	art.set_pixel(9, 5, ink)
	art.set_pixel(14, 5, ink)
	art.fill_rect(Rect2i(11, 8, 2, 1), shadow)
	match appearance.face:
		"bright":
			art.set_pixel(9, 4, ink)
			art.set_pixel(14, 4, ink)
			art.fill_rect(Rect2i(10, 8, 4, 1), Color("fff0dc"))
		"focused":
			art.fill_rect(Rect2i(9, 4, 2, 1), ink)
			art.fill_rect(Rect2i(13, 4, 2, 1), ink)
		"freckled":
			for point in [Vector2i(8, 7), Vector2i(10, 6), Vector2i(13, 6), Vector2i(15, 7)]:
				art.set_pixelv(point, shadow)
	match appearance.hair.style:
		"crop":
			art.fill_rect(Rect2i(8, 1, 8, 2), hair)
		"curls":
			art.fill_rect(Rect2i(7, 1, 10, 2), hair)
			for x in [7, 10, 13, 16]:
				art.fill_rect(Rect2i(x, 0, 2, 4), hair)
		"side_part":
			art.fill_rect(Rect2i(7, 0, 10, 2), hair)
			art.fill_rect(Rect2i(7, 2, 6, 1), hair)
			art.fill_rect(Rect2i(7, 3, 2, 2), hair)
		"long":
			art.fill_rect(Rect2i(7, 0, 10, 3), hair)
			art.fill_rect(Rect2i(7, 3, 2, 8), hair)
	return art

static func gremlin() -> Image:
	# Resolve the existing v2.3 binding. The 32px pose is illustrative, not a size rule.
	var source := ProjectSettings.globalize_path(MANIFEST).simplify_path()
	var manifest: Variant = JSON.parse_string(FileAccess.get_file_as_string(source))
	if not manifest is Dictionary or manifest.get("version") != "2.3":
		push_error("HUEMAN-01 requires the v2.3 source manifest")
		return null
	for house: Dictionary in manifest.houses:
		if house.id != "stonebend":
			continue
		for form: Dictionary in house.forms:
			if form.name != "Gremlin":
				continue
			for swatch: Dictionary in house.shades:
				if swatch.slot == form.swatch_slot:
					return StudyArt.body("gremlin", 32, Color(swatch.hex), Color(house.shades[1].hex))
	push_error("Missing Gremlin v2.3 binding")
	return null
