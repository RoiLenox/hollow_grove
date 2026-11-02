extends Node
## Local presentation preferences; runtime authority and simulation are untouched.

var foreground_fps := 60
var background_fps := 15
var render_on_demand := true
var _previous_fps := 0
var _previous_low_usage := false
var _previous_sleep := 6900


func _ready() -> void:
	_previous_fps = Engine.max_fps
	_previous_low_usage = OS.low_processor_usage_mode
	_previous_sleep = OS.low_processor_usage_mode_sleep_usec
	var config := ConfigFile.new()
	var config_root := OS.get_environment("XDG_CONFIG_HOME")
	if config_root.is_empty():
		config_root = OS.get_environment("HOME").path_join(".config")
	var result := config.load(config_root.path_join("hueman/performance.cfg"))
	if result != OK and result != ERR_FILE_NOT_FOUND:
		push_warning("Hueman performance preferences could not be loaded; using defaults.")
	foreground_fps = _read_fps(config.get_value("performance", "max_fps", 60), 60)
	background_fps = _read_fps(config.get_value("performance", "background_fps", 15), 15)
	render_on_demand = config.get_value("performance", "render_on_demand", true) == true
	if OS.has_environment("HUEMAN_MAX_FPS"):
		foreground_fps = _read_fps(OS.get_environment("HUEMAN_MAX_FPS"), foreground_fps)
	if OS.has_environment("HUEMAN_BACKGROUND_FPS"):
		background_fps = _read_fps(OS.get_environment("HUEMAN_BACKGROUND_FPS"), background_fps)
	OS.low_processor_usage_mode = render_on_demand
	OS.low_processor_usage_mode_sleep_usec = 1000
	get_window().focus_entered.connect(set_background.bind(false))
	get_window().focus_exited.connect(set_background.bind(true))
	set_background(not get_window().has_focus())


func _read_fps(value: Variant, fallback: int) -> int:
	var text := str(value)
	if not text.is_valid_int():
		return fallback
	var fps := int(text)
	return fps if fps >= 0 and fps <= 360 else fallback


func set_background(background: bool) -> void:
	var limit := background_fps if background else foreground_fps
	if background and foreground_fps > 0:
		limit = foreground_fps if limit == 0 else mini(limit, foreground_fps)
	Engine.max_fps = limit


func _exit_tree() -> void:
	Engine.max_fps = _previous_fps
	OS.low_processor_usage_mode = _previous_low_usage
	OS.low_processor_usage_mode_sleep_usec = _previous_sleep
