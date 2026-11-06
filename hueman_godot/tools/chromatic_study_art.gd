extends RefCounted
## Nonproduction, freshly drawn comparison poses. No resampling or atlas writes.

static func body(kind: String, height: int, pigment: Color, shade: Color, skin_variant: int = 1) -> Image:
	var width := 24
	if kind == "goblin":
		width = 26
	elif kind == "gargoyle":
		width = 32
	var art := Image.create(width, height, false, Image.FORMAT_RGBA8)
	art.fill(Color.TRANSPARENT)
	var ink := Color.BLACK
	var eye := Color.WHITE
	if kind == "hueman":
		# Illustrative personal skin/hair samples, deliberately outside House identity.
		var skin: Color = [Color("e4b18b"), Color("b77a55"), Color("69432f")][skin_variant]
		var hair: Color = [Color("70472c"), Color("33251f"), Color("221b19")][skin_variant]
		art.fill_rect(Rect2i(8, 0, 8, 10), skin)
		art.fill_rect(Rect2i(7, 0, 10, 3), hair)
		art.fill_rect(Rect2i(7, 3, 2, 3), hair)
		art.fill_rect(Rect2i(7, 5, 2, 3), skin)
		art.fill_rect(Rect2i(15, 5, 2, 3), skin)
		art.fill_rect(Rect2i(10, 5, 1, 1), ink)
		art.fill_rect(Rect2i(14, 5, 1, 1), ink)
		art.fill_rect(Rect2i(10, 10, 4, 2), skin)
		art.fill_rect(Rect2i(6, 12, 12, 11), pigment)
		art.fill_rect(Rect2i(4, 13, 3, 11), skin)
		art.fill_rect(Rect2i(17, 13, 3, 11), skin)
		art.fill_rect(Rect2i(7, 23, 4, height - 23), shade)
		art.fill_rect(Rect2i(13, 23, 4, height - 23), shade)
		art.fill_rect(Rect2i(6, height - 2, 5, 2), hair)
		art.fill_rect(Rect2i(13, height - 2, 5, 2), hair)
		art.fill_rect(Rect2i(10, 13, 2, 8), eye)
	elif kind == "gremlin":
		# Tall ears, narrow chest, long fingers and digit-like feet.
		art.fill_rect(Rect2i(1, 0, 3, 6), shade)
		art.fill_rect(Rect2i(20, 0, 3, 6), shade)
		art.fill_rect(Rect2i(3, 2, 5, 5), pigment)
		art.fill_rect(Rect2i(16, 2, 5, 5), pigment)
		art.fill_rect(Rect2i(7, 1, 10, 10), pigment)
		art.fill_rect(Rect2i(8, 5, 3, 2), eye)
		art.fill_rect(Rect2i(13, 5, 3, 2), eye)
		art.set_pixel(9, 6, ink)
		art.set_pixel(14, 6, ink)
		art.fill_rect(Rect2i(11, 8, 2, 2), shade)
		art.fill_rect(Rect2i(10, 10, 4, 3), pigment)
		art.fill_rect(Rect2i(8, 12, 8, 11), pigment)
		art.fill_rect(Rect2i(4, 12, 4, 12), pigment)
		art.fill_rect(Rect2i(16, 12, 4, 12), pigment)
		art.fill_rect(Rect2i(4, 23, 1, 3), shade)
		art.fill_rect(Rect2i(19, 23, 1, 3), shade)
		art.fill_rect(Rect2i(10, 14, 4, 5), shade)
		art.fill_rect(Rect2i(7, 23, 4, height - 23), pigment)
		art.fill_rect(Rect2i(13, 23, 4, height - 23), pigment)
		art.fill_rect(Rect2i(5, height - 2, 6, 2), shade)
		art.fill_rect(Rect2i(13, height - 2, 6, 2), shade)
	elif kind == "goblin":
		# Brow crest, broader jaw and rib cage, bent elbows and longer lower legs.
		art.fill_rect(Rect2i(10, 0, 6, 3), shade)
		art.fill_rect(Rect2i(6, 3, 14, 11), pigment)
		art.fill_rect(Rect2i(2, 3, 4, 5), pigment)
		art.fill_rect(Rect2i(20, 3, 4, 5), pigment)
		art.fill_rect(Rect2i(7, 5, 12, 2), shade)
		art.fill_rect(Rect2i(8, 7, 3, 2), eye)
		art.fill_rect(Rect2i(15, 7, 3, 2), eye)
		art.set_pixel(10, 8, ink)
		art.set_pixel(15, 8, ink)
		art.fill_rect(Rect2i(12, 8, 3, 4), shade)
		art.fill_rect(Rect2i(8, 12, 2, 3), eye)
		art.fill_rect(Rect2i(16, 12, 2, 3), eye)
		art.fill_rect(Rect2i(11, 13, 4, 3), pigment)
		art.fill_rect(Rect2i(6, 15, 14, 12), pigment)
		art.fill_rect(Rect2i(10, 17, 6, 7), shade)
		art.fill_rect(Rect2i(2, 16, 4, 12), pigment)
		art.fill_rect(Rect2i(20, 16, 4, 12), pigment)
		art.fill_rect(Rect2i(1, 26, 5, 4), shade)
		art.fill_rect(Rect2i(20, 26, 5, 4), shade)
		art.fill_rect(Rect2i(7, 27, 5, height - 27), pigment)
		art.fill_rect(Rect2i(14, 27, 5, height - 27), pigment)
		art.fill_rect(Rect2i(5, height - 2, 7, 2), shade)
		art.fill_rect(Rect2i(14, height - 2, 7, 2), shade)
	else:
		# Upright stone body; rooted horns, folded shoulder fins and forearm shields.
		art.fill_rect(Rect2i(7, 0, 3, 7), shade)
		art.fill_rect(Rect2i(22, 0, 3, 7), shade)
		art.fill_rect(Rect2i(10, 5, 12, 11), pigment)
		art.fill_rect(Rect2i(11, 8, 10, 2), shade)
		art.fill_rect(Rect2i(12, 10, 2, 2), ink)
		art.fill_rect(Rect2i(18, 10, 2, 2), ink)
		art.fill_rect(Rect2i(14, 13, 4, 2), shade)
		art.fill_rect(Rect2i(1, 13, 5, 15), shade)
		art.fill_rect(Rect2i(26, 13, 5, 15), shade)
		art.fill_rect(Rect2i(4, 16, 24, 6), pigment)
		art.fill_rect(Rect2i(8, 22, 16, 9), pigment)
		art.fill_rect(Rect2i(11, 22, 10, 2), shade)
		art.fill_rect(Rect2i(14, 24, 4, 5), shade)
		art.fill_rect(Rect2i(2, 23, 6, 10), pigment)
		art.fill_rect(Rect2i(24, 23, 6, 10), pigment)
		art.fill_rect(Rect2i(3, 25, 2, 6), shade)
		art.fill_rect(Rect2i(27, 25, 2, 6), shade)
		art.fill_rect(Rect2i(8, 31, 6, height - 31), pigment)
		art.fill_rect(Rect2i(18, 31, 6, height - 31), pigment)
		art.fill_rect(Rect2i(6, height - 3, 8, 3), shade)
		art.fill_rect(Rect2i(18, height - 3, 8, 3), shade)
	return art
