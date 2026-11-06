extends Control
## HUEMAN-01 source-only preview. No authority client, world save, or production art.
const Appearance = preload("res://tools/hueman_01/appearance.gd")
const Store = preload("res://tools/hueman_01/profile_store.gd")
const Art = preload("res://tools/hueman_01/avatar_art.gd")
const Studio = preload("res://tools/hueman_01/studio.gd")

var appearance := Appearance.defaults()
var avatar_id := Appearance.new_avatar_id()
var revision := 0
var profile_directory := "user://hueman-01-preview"
var store: RefCounted
var active_view := "hueman"
var skin_buttons: Array[Button] = []
var face_choice: OptionButton
var hair_style_choice: OptionButton
var hair_color_choice: OptionButton
var form_choice: OptionButton
var save_button: Button
var load_button: Button
var status: Label
var portrait: TextureRect
var appearance_caption: Label
var form_caption: Label
var studio: Control
var last_result: Dictionary = {}
var _hueman_image: Image
var _gremlin_image: Image
var page_frame: Control
var page_scroll: ScrollContainer

func _ready() -> void:
	for argument in OS.get_cmdline_user_args():
		if argument.begins_with("--profile-dir="):
			profile_directory = argument.trim_prefix("--profile-dir=")
	store = Store.new(profile_directory)
	_gremlin_image = Art.gremlin()
	get_window().title = "Hollow Grove · Create Your Hueman · isolated preview"
	get_window().min_size = Vector2i(900, 600)
	get_window().size = Vector2i(1120, 920)
	texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
	_build_ui()
	load_saved(true)
	print("HUEMAN-01 preview appearance file: ", store.path())

func _box(color: Color, radius: int = 8) -> StyleBoxFlat:
	var box := StyleBoxFlat.new()
	box.bg_color = color
	box.set_corner_radius_all(radius)
	box.content_margin_left = 14
	box.content_margin_right = 14
	box.content_margin_top = 10
	box.content_margin_bottom = 10
	return box

func _label(parent: Node, value: String, font_size: int = 16, color: Color = Color("ecede7")) -> Label:
	var label := Label.new()
	label.text = value
	label.add_theme_font_size_override("font_size", font_size)
	label.add_theme_color_override("font_color", color)
	parent.add_child(label)
	return label

func _button(parent: Node, value: String, action: Callable) -> Button:
	var button := Button.new()
	button.text = value
	button.custom_minimum_size.y = 42
	button.pressed.connect(action)
	parent.add_child(button)
	return button

func _options(parent: Node, options: Array, action: Callable) -> OptionButton:
	var choice := OptionButton.new()
	choice.custom_minimum_size.y = 42
	for option: Dictionary in options:
		choice.add_item(option.label)
	choice.item_selected.connect(action)
	parent.add_child(choice)
	return choice

func _section(parent: Node, title: String) -> void:
	var spacer := Control.new()
	spacer.custom_minimum_size.y = 7
	parent.add_child(spacer)
	_label(parent, title, 21)

func _build_ui() -> void:
	var ui_theme := Theme.new()
	ui_theme.default_font_size = 16
	for type_name in ["Button", "OptionButton"]:
		ui_theme.set_stylebox("normal", type_name, _box(Color("303d43")))
		ui_theme.set_stylebox("hover", type_name, _box(Color("405157")))
		ui_theme.set_stylebox("pressed", type_name, _box(Color("52695d")))
		var focus := _box(Color.TRANSPARENT)
		focus.border_color = Color("d1e3bd")
		focus.set_border_width_all(2)
		ui_theme.set_stylebox("focus", type_name, focus)
		ui_theme.set_color("font_color", type_name, Color("f5f3e8"))
	theme = ui_theme
	var background := ColorRect.new()
	background.color = Color("182328")
	background.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	background.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(background)
	page_scroll = ScrollContainer.new()
	page_scroll.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	page_scroll.horizontal_scroll_mode = ScrollContainer.SCROLL_MODE_DISABLED
	page_scroll.follow_focus = true
	add_child(page_scroll)
	var center := CenterContainer.new()
	center.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	center.size_flags_vertical = Control.SIZE_EXPAND_FILL
	page_scroll.add_child(center)
	var margin := MarginContainer.new()
	page_frame = margin
	for edge in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + edge, 28)
	center.add_child(margin)
	resized.connect(_fit_page.bind(margin))
	_fit_page.call_deferred(margin)
	var page := VBoxContainer.new()
	page.add_theme_constant_override("separation", 10)
	margin.add_child(page)
	_label(page, "HOLLOW GROVE     /     HUEMAN–01", 15, Color("b4c9b6"))
	_label(page, "Create Your Hueman", 34)
	_label(page, "Your skin, your face, your hair. A familiar self to return to.", 18, Color("b6c1c4"))
	var columns := HBoxContainer.new()
	columns.add_theme_constant_override("separation", 28)
	columns.size_flags_vertical = Control.SIZE_EXPAND_FILL
	page.add_child(columns)
	var left := VBoxContainer.new()
	left.custom_minimum_size.x = 360
	left.add_theme_constant_override("separation", 8)
	columns.add_child(left)
	_section(left, "01   Skin")
	_label(left, "Natural tones", 14, Color("b6c1c4"))
	var grid := GridContainer.new()
	grid.columns = 2
	grid.add_theme_constant_override("h_separation", 8)
	grid.add_theme_constant_override("v_separation", 8)
	left.add_child(grid)
	for index in range(Appearance.SKINS.size()):
		var option: Dictionary = Appearance.SKINS[index]
		var button := _button(grid, option.label, _skin_selected.bind(index))
		button.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		button.toggle_mode = true
		button.add_theme_font_size_override("font_size", 14)
		var swatch := Image.create(14, 20, false, Image.FORMAT_RGBA8)
		swatch.fill(Color(option.hex))
		button.icon = ImageTexture.create_from_image(swatch)
		skin_buttons.append(button)
	_section(left, "02   Face")
	face_choice = _options(left, Appearance.FACES, _face_selected)
	_section(left, "03   Hair")
	_label(left, "Style", 14, Color("b6c1c4"))
	hair_style_choice = _options(left, Appearance.HAIR_STYLES, _hair_style_selected)
	_label(left, "Color", 14, Color("b6c1c4"))
	hair_color_choice = _options(left, Appearance.HAIR_COLORS, _hair_color_selected)
	var flexible_space := Control.new()
	flexible_space.size_flags_vertical = Control.SIZE_EXPAND_FILL
	left.add_child(flexible_space)
	var actions := HBoxContainer.new()
	actions.add_theme_constant_override("separation", 8)
	left.add_child(actions)
	save_button = _button(actions, "Save appearance", save_current)
	load_button = _button(actions, "Load saved", load_saved)
	save_button.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	load_button.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	status = _label(left, "", 14, Color("d1e3bd"))
	status.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	status.custom_minimum_size.y = 55
	var right := VBoxContainer.new()
	right.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	right.add_theme_constant_override("separation", 10)
	columns.add_child(right)
	var display := PanelContainer.new()
	display.add_theme_stylebox_override("panel", _box(Color("253239"), 12))
	right.add_child(display)
	var portrait_row := HBoxContainer.new()
	portrait_row.add_theme_constant_override("separation", 22)
	display.add_child(portrait_row)
	portrait = TextureRect.new()
	portrait.custom_minimum_size = Vector2(168, 224)
	portrait.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	portrait.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_CENTERED
	portrait_row.add_child(portrait)
	var notes := VBoxContainer.new()
	notes.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	notes.alignment = BoxContainer.ALIGNMENT_CENTER
	notes.add_theme_constant_override("separation", 12)
	portrait_row.add_child(notes)
	_label(notes, "Your appearance", 21)
	appearance_caption = _label(notes, "", 16, Color("c8d2d0"))
	appearance_caption.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	var fixed_note := _label(notes, "Standard humanoid body.\nBody growth belongs to Current Synthesis.", 14, Color("a5b8be"))
	fixed_note.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	_label(right, "Form comparison · visual only", 16)
	form_choice = _options(right, [{"label": "Hueman"}, {"label": "Gremlin · study pose"}], _form_selected)
	form_caption = _label(right, "", 14, Color("b6c1c4"))
	form_caption.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	form_caption.custom_minimum_size.y = 40
	studio = Studio.new()
	studio.custom_minimum_size.y = 160
	studio.size_flags_vertical = Control.SIZE_EXPAND_FILL
	right.add_child(studio)
	var walk := _button(right, "Walk in preview  ·  WASD / arrow keys", studio.grab_focus)
	walk.tooltip_text = "Click the floor or this button to move. Tab returns to the controls."
	_label(page, "Isolated character studio   ·   Illustrative art   ·   No world progression", 13, Color("9baeb2"))

func _fit_page(page: Control) -> void:
	# Let containers measure wrapped text before centering. Small windows scroll;
	# tall/tiled displays keep a bounded studio instead of stretching the floor.
	page.custom_minimum_size = Vector2(minf(size.x - 24, 1120), minf(size.y, 900))

func _skin_selected(index: int) -> void:
	appearance.skin = Appearance.SKINS[index].id
	_changed()

func _face_selected(index: int) -> void:
	appearance.face = Appearance.FACES[index].id
	_changed()

func _hair_style_selected(index: int) -> void:
	appearance.hair.style = Appearance.HAIR_STYLES[index].id
	_changed()

func _hair_color_selected(index: int) -> void:
	appearance.hair.color = Appearance.HAIR_COLORS[index].id
	_changed()

func _changed() -> void:
	_refresh()
	status.text = "Appearance changed. Save to keep it."

func _form_selected(index: int) -> void:
	active_view = "hueman" if index == 0 else "gremlin"
	_refresh()

func _refresh() -> void:
	for index in range(skin_buttons.size()):
		skin_buttons[index].set_pressed_no_signal(appearance.skin == Appearance.SKINS[index].id)
	_select(face_choice, Appearance.FACES, appearance.face)
	_select(hair_style_choice, Appearance.HAIR_STYLES, appearance.hair.style)
	_select(hair_color_choice, Appearance.HAIR_COLORS, appearance.hair.color)
	hair_color_choice.disabled = appearance.hair.style == "shaved"
	hair_color_choice.tooltip_text = "Color is kept for when you choose a hairstyle." if hair_color_choice.disabled else ""
	_hueman_image = Art.hueman(appearance)
	var displayed: Image = _gremlin_image if active_view == "gremlin" else _hueman_image
	if displayed == null:
		active_view = "hueman"
		form_choice.select(0)
		displayed = _hueman_image
	var texture := ImageTexture.create_from_image(displayed)
	portrait.texture = texture
	studio.avatar = texture
	studio.queue_redraw()
	appearance_caption.text = "%s\n%s\n%s · %s" % [Appearance.entry(Appearance.SKINS, appearance.skin).label,
		Appearance.entry(Appearance.FACES, appearance.face).label, Appearance.entry(Appearance.HAIR_STYLES, appearance.hair.style).label,
		Appearance.entry(Appearance.HAIR_COLORS, appearance.hair.color).label]
	form_caption.text = "Your customized Hueman. The same appearance is kept when comparing Forms."
	if active_view == "gremlin":
		form_caption.text = "Your appearance is kept. Return to Hueman to see it. Study size is provisional."

func _select(choice: OptionButton, options: Array, id: String) -> void:
	for index in range(options.size()):
		if options[index].id == id:
			choice.select(index)

func save_current() -> void:
	last_result = store.save_profile(avatar_id, appearance, revision)
	if last_result.ok:
		revision = int(last_result.profile.revision)
		status.text = "Appearance saved. You can return to this Hueman."
	else:
		status.text = last_result.error

func load_saved(initial: bool = false) -> void:
	last_result = store.load_profile()
	if last_result.ok:
		appearance = last_result.profile.appearance.duplicate(true)
		avatar_id = last_result.profile.avatar_id
		revision = int(last_result.profile.revision)
		status.text = "Saved avatar loaded. Welcome back."
	elif initial and last_result.get("missing", false):
		status.text = "Choose your appearance, then save your Hueman."
	else:
		status.text = last_result.error
	_refresh()

func appearance_image() -> Image:
	return _hueman_image
