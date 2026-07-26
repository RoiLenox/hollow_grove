extends SceneTree


func _init() -> void:
	var client := HollowGroveRuntimeClient.new("session.smoke")
	assert(client.revision == 0)
	assert(not client.is_runtime_connected())
	assert(client.connect_to_runtime("0.0.0.0", 47819) == ERR_INVALID_PARAMETER)
	quit()
