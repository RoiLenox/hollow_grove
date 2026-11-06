extends RefCounted
## Prototype cosmetic vocabulary. Samples are not House tokens or approved canon.

const SKINS := [
	{"id": "light_neutral", "label": "Light · neutral", "hex": "f1c9ad"},
	{"id": "light_warm", "label": "Light · warm", "hex": "dfac85"},
	{"id": "medium_golden", "label": "Medium · golden", "hex": "c28c59"},
	{"id": "medium_warm", "label": "Medium · warm", "hex": "a76f4d"},
	{"id": "deep_warm", "label": "Deep · warm", "hex": "805137"},
	{"id": "deep_neutral", "label": "Deep · neutral", "hex": "54392e"},
]
const FACES := [
	{"id": "calm", "label": "Calm"}, {"id": "bright", "label": "Bright"},
	{"id": "focused", "label": "Focused"}, {"id": "freckled", "label": "Freckled"},
]
const HAIR_STYLES := [
	{"id": "shaved", "label": "Shaved"}, {"id": "crop", "label": "Close crop"},
	{"id": "curls", "label": "Curls"}, {"id": "side_part", "label": "Side part"},
	{"id": "long", "label": "Long"},
]
const HAIR_COLORS := [
	{"id": "black", "label": "Soft black", "hex": "211d20"},
	{"id": "brown", "label": "Chestnut", "hex": "583528"},
	{"id": "auburn", "label": "Auburn", "hex": "8b4930"},
	{"id": "blond", "label": "Honey blond", "hex": "c89c55"},
]

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
