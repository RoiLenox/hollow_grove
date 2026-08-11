class_name HollowGroveArtAssetCatalog
extends RefCounted

const DEFAULT_MANIFEST_PATH := "res://assets/manifest.json"
const SUPPORTED_SCHEMA_VERSION := "1.0.0"

var manifest_path := DEFAULT_MANIFEST_PATH
var manifest := {}
var records_by_id := {}
var error_message := ""


func _init(path: String = DEFAULT_MANIFEST_PATH) -> void:
	manifest_path = path
	_load_manifest()


func is_valid() -> bool:
	return error_message == "" and not records_by_id.is_empty()


func asset_ids() -> Array[String]:
	var identifiers: Array[String] = []
	for asset_id in records_by_id.keys():
		identifiers.append(str(asset_id))
	identifiers.sort()
	return identifiers


func record(asset_id: String) -> Dictionary:
	var value = records_by_id.get(asset_id, {})
	return value.duplicate(true) if typeof(value) == TYPE_DICTIONARY else {}


func texture(asset_id: String) -> Texture2D:
	var asset_record := record(asset_id)
	if asset_record.is_empty():
		return null
	var resource_path := str(asset_record.get("resource_path", ""))
	if not resource_path.begins_with("res://"):
		return null
	var loaded_texture: Texture2D
	if ResourceLoader.exists(resource_path):
		var loaded = load(resource_path)
		if not loaded is Texture2D:
			return null
		loaded_texture = loaded as Texture2D
	else:
		var image := Image.load_from_file(resource_path)
		if image == null or image.is_empty():
			return null
		loaded_texture = ImageTexture.create_from_image(image)
	if loaded_texture.get_width() != int(asset_record.get("width", -1)):
		return null
	if loaded_texture.get_height() != int(asset_record.get("height", -1)):
		return null
	return loaded_texture


func _load_manifest() -> void:
	manifest = {}
	records_by_id = {}
	error_message = ""
	if not FileAccess.file_exists(manifest_path):
		error_message = "missing art asset manifest: %s" % manifest_path
		return
	var handle := FileAccess.open(manifest_path, FileAccess.READ)
	if handle == null:
		error_message = "cannot open art asset manifest: %s" % manifest_path
		return
	var parsed = JSON.parse_string(handle.get_as_text())
	if typeof(parsed) != TYPE_DICTIONARY:
		error_message = "art asset manifest must be a JSON object"
		return
	if str(parsed.get("schema_version", "")) != SUPPORTED_SCHEMA_VERSION:
		error_message = "unsupported art asset manifest schema"
		return
	var assets = parsed.get("assets", [])
	if typeof(assets) != TYPE_ARRAY or assets.is_empty():
		error_message = "art asset manifest has no assets"
		return
	for value in assets:
		if typeof(value) != TYPE_DICTIONARY:
			error_message = "art asset manifest record must be an object"
			return
		var asset_id := str(value.get("asset_id", ""))
		var resource_path := str(value.get("resource_path", ""))
		var width := int(value.get("width", 0))
		var height := int(value.get("height", 0))
		if asset_id == "" or records_by_id.has(asset_id):
			error_message = "art asset manifest contains a missing or duplicate asset ID"
			return
		if not resource_path.begins_with("res://") or width <= 0 or height <= 0:
			error_message = "art asset manifest contains an invalid resource contract"
			return
		records_by_id[asset_id] = value.duplicate(true)
	manifest = parsed.duplicate(true)
