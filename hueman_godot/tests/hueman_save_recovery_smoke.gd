extends SceneTree
const Scene = preload("res://scenes/retro_overworld.tscn")

class ControlledRuntime extends "res://scripts/runtime_client.gd":
	var connected := true
	var save_error := OK
	var sent: Array = []
	func _init() -> void:
		super("session.hueman-save-recovery")
	func poll() -> void:
		pass
	func is_runtime_connected() -> bool:
		return connected
	func disconnect_from_runtime() -> void:
		connected = false
	func connect_to_configured_runtime() -> Error:
		connected = true
		return OK
	func send_intent(id: String, intent: String, _payload := {}) -> Error:
		sent.append({"id": id, "intent": intent})
		return save_error if intent == "SaveIntent" else OK

var checks := 0
var failures := 0
func _initialize() -> void:
	_run.call_deferred()

func _check(condition: bool, message: String) -> void:
	checks += 1
	if not condition:
		failures += 1
		push_error(message)

func _game() -> Node:
	var game := Scene.instantiate()
	root.add_child(game)
	game.set_process(false)
	game.runtime_client.disconnect_from_runtime()
	game.runtime_client = ControlledRuntime.new()
	game.was_connected = true
	game.retry_elapsed = 0.0
	game.hueman_profile = {"avatar_id": "test"}
	return game

func _run() -> void:
	var game := _game()
	game.runtime_client.save_error = ERR_CONNECTION_ERROR
	game._request_save_and_exit()
	game._process(0.01)
	_check(not game.exit_requested and not game.exit_save_sent and not game.case_pending, "failed send releases exit/input state")
	_check(game.save_request_id.is_empty(), "failed save cannot be acknowledged later")
	game.runtime_client.save_error = OK
	game.runtime_client.connected = true
	game._request_save_and_exit()
	game._process(0.01)
	_check(game.runtime_client.sent.back().intent == "SyncIntent" and not game.exit_save_sent, "reconnection sync precedes retry")
	game.sync_pending = false
	game._process(0.01)
	_check(game.exit_save_sent and not game.save_request_id.is_empty(), "save can be retried after sync")
	game.runtime_client.connected = false
	game._process(0.01)
	_check(not game.exit_requested and game.save_request_id.is_empty(), "disconnect before acknowledgement keeps application open and retryable")
	game.free()

	game = _game()
	game._request_save_and_exit()
	game._process(0.01)
	game._process(game.SAVE_TIMEOUT_SECONDS + 0.1)
	_check(not game.exit_requested and not game.case_pending, "lost acknowledgement times out without exiting")
	game.free()

	game = _game()
	game._request_save_and_exit()
	game._process(0.01)
	var request: String = game.save_request_id
	var cancel := InputEventKey.new()
	cancel.keycode = KEY_ESCAPE
	cancel.pressed = true
	game._input(cancel)
	_check(not game.exit_requested and not game.exit_save_sent, "Escape cancels pending exit")
	game._on_runtime_response({"request_id":request, "status":"Completed", "events":[{"kind":"SnapshotSaved"}]})
	_check(game.status_message.begins_with("GAME SAVED") and game.is_inside_tree(), "late save acknowledgement after cancel does not exit")
	game.free()

	game = _game()
	game._request_save_and_exit()
	game._process(0.01)
	game._on_runtime_response({"request_id":game.save_request_id, "status":"Rejected", "rejection":{"message":"fixture save refused"}})
	_check(not game.exit_requested and not game.case_pending, "server rejection permits retry")
	game.free()

	game = _game()
	game._request_save_and_exit()
	game._process(0.01)
	game._on_protocol_error("injected invalid response")
	_check(not game.exit_requested and not game.case_pending, "invalid response cannot leave exit pending")
	game.free()
	if failures == 0:
		print("HUEMAN-02 save recovery passed (", checks, " checks)")
	quit(1 if failures else 0)
