extends SceneTree

const POLICY := preload("res://scripts/performance_policy.gd")

func _init() -> void:
	call_deferred("_run")

func _run() -> void:
	OS.set_environment("HUEMAN_MAX_FPS", "120")
	OS.set_environment("HUEMAN_BACKGROUND_FPS", "15")
	var previous := Engine.max_fps
	var policy := POLICY.new()
	root.add_child(policy)
	assert(policy.foreground_fps == 120)
	policy.set_background(false)
	assert(Engine.max_fps == 120)
	policy.set_background(true)
	assert(Engine.max_fps == 15)
	assert(policy._read_fps("invalid", 60) == 60)
	assert(policy._read_fps(-1, 60) == 60)
	assert(policy._read_fps(0, 60) == 0)
	policy.foreground_fps = 10
	policy.set_background(true)
	assert(Engine.max_fps == 10)
	policy.free()
	assert(Engine.max_fps == previous)
	OS.unset_environment("HUEMAN_MAX_FPS")
	OS.unset_environment("HUEMAN_BACKGROUND_FPS")
	print("Hueman foreground/background performance policy passed")
	quit()
