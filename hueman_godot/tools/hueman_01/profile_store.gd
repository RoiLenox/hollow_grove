extends RefCounted
## Separate prototype file; never reads or migrates a Core archive.
const Appearance = preload("res://tools/hueman_01/appearance.gd")
const FILE_NAME := "hueman-01-appearance.json"
const MAX_BYTES := 16384
var directory: String

func _init(profile_directory: String = "user://hueman-01-preview") -> void:
	directory = ProjectSettings.globalize_path(profile_directory)

func path() -> String:
	return directory.path_join(FILE_NAME)

func load_profile() -> Dictionary:
	if not FileAccess.file_exists(path()):
		return {"ok": false, "missing": true, "error": "No saved avatar yet."}
	var file := FileAccess.open(path(), FileAccess.READ)
	if file == null:
		return _error("Cannot open the saved avatar.")
	if file.get_length() > MAX_BYTES:
		return _error("Saved avatar is too large; file preserved.")
	var json := JSON.new()
	if json.parse(file.get_as_text()) != OK:
		return _error("Saved avatar is damaged; file preserved.")
	var value: Variant = json.data
	if not Appearance.exact_keys(value, ["schema_version", "avatar_id", "revision", "appearance"]):
		return _error("Unrecognized saved avatar format; file preserved.")
	if not _whole_number(value.schema_version) or value.schema_version != 1:
		return _error("Unsupported saved avatar version; file preserved.")
	if not _whole_number(value.revision) or value.revision < 1 or value.revision > 1000000000:
		return _error("Invalid avatar revision; file preserved.")
	if not Appearance.valid_avatar_id(value.avatar_id) or not Appearance.valid(value.appearance):
		return _error("Invalid saved appearance; file preserved.")
	return {"ok": true, "profile": value}

func save_profile(avatar_id: String, appearance: Dictionary, expected_revision: int) -> Dictionary:
	if not Appearance.valid_avatar_id(avatar_id) or not Appearance.valid(appearance) or expected_revision < 0:
		return _error("Invalid appearance; nothing saved.")
	if DirAccess.make_dir_recursive_absolute(directory) != OK:
		return _error("Cannot create the preview save folder.")
	# Exclusive local writer. An interrupted write leaves a visible lock for recovery.
	var lock_path := path() + ".lock"
	if DirAccess.make_dir_absolute(lock_path) != OK:
		return _error("Another preview is saving, or a save was interrupted. See the preview recovery notes.")
	var result := _save_locked(avatar_id, appearance, expected_revision)
	DirAccess.remove_absolute(lock_path)
	return result

func _save_locked(avatar_id: String, appearance: Dictionary, expected_revision: int) -> Dictionary:
	var previous := load_profile()
	if not previous.ok and not previous.get("missing", false):
		return previous
	var disk_revision: int = int(previous.profile.revision) if previous.ok else 0
	if expected_revision != disk_revision or (previous.ok and previous.profile.avatar_id != avatar_id):
		return _error("This avatar changed in another preview. Load it before saving again.")
	if disk_revision >= 1000000000:
		return _error("Avatar revision limit reached; saved appearance preserved.")
	var profile := {"schema_version": 1, "avatar_id": avatar_id, "revision": disk_revision + 1,
		"appearance": appearance.duplicate(true)}
	var temporary := path() + ".tmp"
	var file := FileAccess.open(temporary, FileAccess.WRITE)
	if file == null:
		return _error("Cannot write the avatar; previous save preserved.")
	file.store_string(JSON.stringify(profile, "\t") + "\n")
	file.flush()
	var write_error := file.get_error()
	file.close()
	if write_error != OK:
		return _error("Avatar write failed; previous save preserved.")
	if previous.ok and DirAccess.copy_absolute(path(), path() + ".backup.json") != OK:
		return _error("Cannot preserve the previous avatar; save cancelled.")
	if DirAccess.rename_absolute(temporary, path()) != OK:
		return _error("Cannot finish saving; previous save preserved.")
	return {"ok": true, "profile": profile}

static func _whole_number(value: Variant) -> bool:
	return (value is int or value is float) and is_finite(float(value)) and float(value) == floor(float(value))

static func _error(message: String) -> Dictionary:
	return {"ok": false, "error": message}
