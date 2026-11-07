extends RefCounted
## Prototype cosmetic vocabulary. Samples are not House tokens or approved canon.

const CATALOG_PATH := "res://protocol/hueman_cosmetics_v1.json"
static var CATALOG: Dictionary = _load_catalog()
static var SKINS: Array = CATALOG.skins
static var FACES: Array = CATALOG.faces
static var HAIR_STYLES: Array = CATALOG.hair_styles
static var HAIR_COLORS: Array = CATALOG.hair_colors

static func _load_catalog() -> Dictionary:
	var data: Variant = JSON.parse_string(FileAccess.get_file_as_string(CATALOG_PATH))
	assert(data is Dictionary and data.get("schema_version") == 1 \
		and data.get("scope") == "hueman-cosmetic-presets", "Invalid cosmetic catalog")
	return data

static func material(id: String) -> Color:
	assert(CATALOG.materials.has(id), "Unknown cosmetic material")
	return Color(CATALOG.materials[id])

static func defaults() -> Dictionary:
	return {"skin": "medium_warm", "face": "calm", "hair": {"style": "curls", "color": "black"}}

static func entry(options: Array, id: String) -> Dictionary:
	for option: Dictionary in options:
		if option.id == id:
			return option
	return {}

static func exact_keys(value: Variant, keys: Array) -> bool:
	if not value is Dictionary or value.size() != keys.size():
		return false
	for key: String in keys:
		if not value.has(key):
			return false
	return true

static func valid(value: Variant) -> bool:
	if not exact_keys(value, ["skin", "face", "hair"]):
		return false
	if not value.skin is String or not value.face is String:
		return false
	if not exact_keys(value.hair, ["style", "color"]):
		return false
	if not value.hair.style is String or not value.hair.color is String:
		return false
	return not entry(SKINS, value.skin).is_empty() and not entry(FACES, value.face).is_empty() \
		and not entry(HAIR_STYLES, value.hair.style).is_empty() and not entry(HAIR_COLORS, value.hair.color).is_empty()

static func new_avatar_id() -> String:
	return "avatar." + Crypto.new().generate_random_bytes(16).hex_encode()

static func valid_avatar_id(value: Variant) -> bool:
	if not value is String or value.length() != 39 or not value.begins_with("avatar."):
		return false
	for character in value.substr(7):
		if not character in "0123456789abcdef":
			return false
	return true
