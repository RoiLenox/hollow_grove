extends SceneTree

const CHECKPOINTS := [0.0, 0.25, 0.5, 0.75, 1.0]
const CAPTURE_SIZE := Vector2i(1280, 720)
const SHELL_SCENE := "res://scenes/hueman_screen_shell.tscn"


func _initialize() -> void:
	call_deferred("_capture_checkpoints")


func _capture_checkpoints() -> void:
	var arguments := OS.get_cmdline_user_args()
	var output_directory := "/tmp/hollow_grove_morph_checkpoints"
	if not arguments.is_empty():
		output_directory = ProjectSettings.globalize_path(str(arguments[0]))

	var directory_error := DirAccess.make_dir_recursive_absolute(output_directory)
	if directory_error != OK:
		push_error("Could not create morph checkpoint directory: %s" % output_directory)
		quit(1)
		return

	var packed_scene := load(SHELL_SCENE) as PackedScene
	if packed_scene == null:
		push_error("Could not load the Hueman screen shell scene.")
		quit(1)
		return

	root.size = CAPTURE_SIZE
	var shell := packed_scene.instantiate()
	root.add_child(shell)
	await process_frame
	shell.call("set_topology_review_mode", true)

	for rectangularity in CHECKPOINTS:
		shell.set("adaptive_rectangularity", rectangularity)
		var slider := shell.get("adaptive_morph_slider") as HSlider
		if slider != null:
			slider.set_value_no_signal(rectangularity)
		shell.call("_update_adaptive_morph_label")
		shell.queue_redraw()
		await process_frame
		RenderingServer.force_draw(false)
		await process_frame

		var percentage := roundi(rectangularity * 100.0)
		var output_path := output_directory.path_join(
			"hollow_grove_morph_%03d.png" % percentage
		)
		var viewport_texture := root.get_texture()
		if viewport_texture == null:
			push_error("The active Godot renderer does not expose a viewport texture.")
			quit(1)
			return
		var image := viewport_texture.get_image()
		if image == null or image.is_empty() or image.save_png(output_path) != OK:
			push_error("Could not save morph checkpoint: %s" % output_path)
			quit(1)
			return
		print(output_path)

	quit()
