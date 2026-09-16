extends SceneTree

const RUNTIME_CLIENT := preload("res://scripts/runtime_client.gd")


func _init() -> void:
	var client = RUNTIME_CLIENT.new("session.smoke")
	assert(client.revision == 0)
	assert(not client.is_runtime_connected())
	assert(client.connect_to_runtime("0.0.0.0", 47819) == ERR_INVALID_PARAMETER)
	assert(client.connect_to_runtime("127.0.0.1", 0) == ERR_INVALID_PARAMETER)
	assert(client.connect_to_runtime("127.0.0.1", 65536) == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_HOST", "192.0.2.1")
	assert(client.connect_to_configured_runtime() == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_HOST", "127.0.0.1")
	OS.set_environment("HOLLOW_GROVE_PORT", "not-a-port")
	assert(client.connect_to_configured_runtime() == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_PORT", "70000")
	assert(client.connect_to_configured_runtime() == ERR_INVALID_PARAMETER)
	OS.set_environment("HOLLOW_GROVE_HOST", "")
	OS.set_environment("HOLLOW_GROVE_PORT", "")
	quit()
