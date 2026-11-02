extends SceneTree

const RUNTIME_CLIENT := preload("res://scripts/runtime_client.gd")


func _init() -> void:
	var client = RUNTIME_CLIENT.new("session.smoke")
	assert(client.revision == 0)
	assert(not client.is_runtime_connected())
	assert(not client.is_shared_runtime())
	var config := {"version": 1, "session_id": "session.smoke", "account_id": "player.test",
		"device_id": "desktop", "token": "a".repeat(64)}
	assert(client._valid_shared_config(config))
	for field in ["account_id", "device_id", "token", "version"]:
		var invalid := config.duplicate()
		invalid.erase(field)
		assert(not client._valid_shared_config(invalid))
	config["token"] = "é".repeat(64)
	assert(not client._valid_shared_config(config))
	client._shared_config = {"token": "stale"}
	assert(client.connect_to_runtime("0.0.0.0", 47819) == ERR_INVALID_PARAMETER)
	assert(client.connect_to_runtime("127.0.0.1", 0) == ERR_INVALID_PARAMETER)
	assert(client.connect_to_runtime("127.0.0.1", 65536) == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_HOST", "192.0.2.1")
	assert(client.connect_to_configured_runtime() == ERR_INVALID_PARAMETER)
	assert(not client.is_shared_runtime())
	OS.set_environment("HOLLOW_GROVE_HOST", "127.0.0.1")
	OS.set_environment("HOLLOW_GROVE_PORT", "not-a-port")
	assert(client.connect_to_configured_runtime() == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_PORT", "70000")
	assert(client.connect_to_configured_runtime() == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_HOST", "")
	OS.set_environment("HOLLOW_GROVE_PORT", "")
	quit()
