extends "res://scripts/hueman_avatar_art.gd"
## Preview-only comparison retained independently of the shared Hueman art.
const StudyArt = preload("res://tools/chromatic_study_art.gd")
const MANIFEST := "res://../docs/design/hollow_grove_chromatic_color_standard_v2_3_0.json"

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
