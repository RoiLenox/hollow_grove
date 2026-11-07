extends RefCounted
## Shared fixed-body Hueman rendering; no Synthesis or authority decisions.
const Appearance = preload("res://scripts/hueman_appearance.gd")

static func hueman(appearance: Dictionary) -> Image:
	assert(Appearance.valid(appearance))
	var art := Image.create(24, 32, false, Image.FORMAT_RGBA8)
	art.fill(Color.TRANSPARENT)
	var skin := Color(Appearance.entry(Appearance.SKINS, appearance.skin).hex)
	var hair := Color(Appearance.entry(Appearance.HAIR_COLORS, appearance.hair.color).hex)
	var ink := Appearance.material("ink")
	var shadow := skin.darkened(0.19)
	# Hair behind the head; all styles fit the same canvas and body.
	if appearance.hair.style == "long":
		art.fill_rect(Rect2i(6, 1, 12, 16), hair)
	art.fill_rect(Rect2i(8, 1, 8, 9), skin)
	art.fill_rect(Rect2i(7, 4, 1, 4), shadow)
	art.fill_rect(Rect2i(16, 4, 1, 4), shadow)
	art.fill_rect(Rect2i(10, 10, 4, 2), shadow)
	# Clothing and body are fixed; no relationship to House, rank, or ability.
	art.fill_rect(Rect2i(6, 12, 12, 11), Appearance.material("shirt"))
	art.fill_rect(Rect2i(5, 13, 2, 6), Appearance.material("shirt"))
	art.fill_rect(Rect2i(17, 13, 2, 6), Appearance.material("shirt"))
	art.fill_rect(Rect2i(5, 19, 2, 5), skin)
	art.fill_rect(Rect2i(17, 19, 2, 5), skin)
	art.fill_rect(Rect2i(9, 12, 6, 2), Appearance.material("collar"))
	art.fill_rect(Rect2i(6, 21, 12, 2), Appearance.material("belt"))
	art.fill_rect(Rect2i(7, 23, 4, 7), Appearance.material("trousers"))
	art.fill_rect(Rect2i(13, 23, 4, 7), Appearance.material("trousers"))
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
			art.fill_rect(Rect2i(10, 8, 4, 1), Appearance.material("teeth"))
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
