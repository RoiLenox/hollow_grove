extends "res://tools/hueman_01/creator.gd"
## Existing creator UI, connected to the authoritative persistent gameplay runtime.
const RuntimeClient = preload("res://scripts/runtime_client.gd")
var client: RefCounted
var enter_button: Button
var flow := "connecting"
var connected := false
var retry_seconds := 0.0
var request_number := 0
var request_prefix := "entry." + Crypto.new().generate_random_bytes(8).hex_encode()
var current_view := {}
var connection_error := ""

func _ready() -> void:
	super._ready()
	get_window().title = "Hollow Grove · Create Your Hueman"
	enter_button = _button(status.get_parent(), "Enter Stonebend", finish_creation)
	enter_button.disabled = true
	status.text = "Connecting to Hollow Grove…"
	client = RuntimeClient.new(OS.get_environment("HOLLOW_GROVE_SESSION") if OS.has_environment("HOLLOW_GROVE_SESSION") else "session.hollow-grove.hueman")
	client.response_received.connect(_on_response)
	client.protocol_error.connect(_on_error)
	client.connect_to_configured_runtime()

func _process(delta: float) -> void:
	if client == null:
		return
	client.poll()
	if client.is_runtime_connected():
		if not connected:
			connected = true
			_send("inspect", "SyncIntent")
	else:
		connected = false
		retry_seconds += delta
		if retry_seconds >= 1.0:
			retry_seconds = 0.0
			client.disconnect_from_runtime()
			client.connect_to_configured_runtime()

func _send(next_flow: String, intent: String, payload := {}) -> void:
	flow = next_flow
	enter_button.disabled = true
	request_number += 1
	var error: Error = client.send_intent("%s.%d" % [request_prefix, request_number], intent, payload)
	if error != OK:
		_on_error("Could not send request: " + error_string(error))

func _on_error(message: String) -> void:
	connection_error = message
	status.text = message
	enter_button.text = "Retry connection / resume"
	enter_button.disabled = false
	flow = "error"

func finish_creation() -> void:
	if flow == "error":
		_send("inspect", "SyncIntent")
		return
	if flow != "ready":
		return
	save_current()
	if not last_result.ok:
		return
	# The profile draft is saved before creating a gameplay identity. Creation is
	# atomic in Rust; gameplay starts only after its complete archive is saved.
	if current_view.get("guide_context", {}).get("layer", "") == "outside_hollow_grove":
		_send("enter", "EnterHollowGroveIntent")
	else:
		_create_character()

func _create_character() -> void:
	_send("create", "CreateHuemanIntent", {"profile": {
		"schema_version": 1, "avatar_id": avatar_id, "appearance": appearance.duplicate(true)}})

func _on_response(response: Dictionary) -> void:
	current_view = response.get("view", {}) if response.get("view") is Dictionary else {}
	if response.get("status") != "Completed":
		_on_error(str(response.get("rejection", {}).get("message", "Request rejected; saved character preserved.")))
		return
	match flow:
		"inspect":
			if current_view.get("character_save_available", false):
				status.text = "Resuming your saved Hueman…"
				_send("resume", "LoadIntent", {"slot_id": "slot-a"})
			elif current_view.get("hueman_profile") is Dictionary:
				_send("save", "SaveIntent", {"slot_id": "slot-a"})
			elif current_view.get("hueman") != null:
				_on_error("This session already has a legacy character. Save it with the existing game launcher.")
			else:
				flow = "ready"
				enter_button.text = "Enter Stonebend"
				enter_button.disabled = false
				status.text = "Choose Skin, Face and Hair, then enter Stonebend."
		"enter":
			_create_character()
		"create":
			status.text = "Saving your Hueman and starting location…"
			_send("save", "SaveIntent", {"slot_id": "slot-a"})
		"save", "resume":
			_open_game()

func _open_game() -> void:
	flow = "playing"
	client.disconnect_from_runtime()
	set_process(false)
	var error := get_tree().change_scene_to_file("res://scenes/retro_overworld.tscn")
	if error != OK:
		_on_error("Could not open Stonebend: " + error_string(error))
