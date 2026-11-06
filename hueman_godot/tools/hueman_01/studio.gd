extends Control
## Local toy movement only. No runtime client or game intents.
var avatar: Texture2D
var feet := Vector2(280, 170)

func _ready() -> void:
	focus_mode = Control.FOCUS_ALL
	mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
	texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
	resized.connect(_clamp_feet)
	focus_entered.connect(queue_redraw)
	focus_exited.connect(queue_redraw)

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		grab_focus()
	if event is InputEventKey and event.keycode in [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_W, KEY_A, KEY_S, KEY_D]:
		accept_event()

func _process(delta: float) -> void:
	if not has_focus():
		return
	var direction := Vector2(
		float(Input.is_key_pressed(KEY_D) or Input.is_key_pressed(KEY_RIGHT)) - float(Input.is_key_pressed(KEY_A) or Input.is_key_pressed(KEY_LEFT)),
		float(Input.is_key_pressed(KEY_S) or Input.is_key_pressed(KEY_DOWN)) - float(Input.is_key_pressed(KEY_W) or Input.is_key_pressed(KEY_UP)))
	move_avatar(direction, delta)

func move_avatar(direction: Vector2, delta: float) -> void:
	feet += direction.normalized() * 130.0 * delta
	_clamp_feet()

func _clamp_feet() -> void:
	feet = feet.clamp(Vector2(30, 72), Vector2(maxf(30, size.x - 30), maxf(72, size.y - 18)))
	queue_redraw()

func _draw() -> void:
	draw_rect(Rect2(Vector2.ZERO, size), Color("202c32"))
	for x in range(0, int(size.x), 32):
		draw_line(Vector2(x, 0), Vector2(x, size.y), Color("2a383e"))
	for y in range(0, int(size.y), 32):
		draw_line(Vector2(0, y), Vector2(size.x, y), Color("2a383e"))
	draw_rect(Rect2(Vector2.ONE, size - Vector2(2, 2)), Color("b5d4bf") if has_focus() else Color("53636a"), false, 2)
	if avatar != null:
		draw_ellipse_shadow()
		draw_texture_rect(avatar, Rect2(feet.round() - Vector2(24, 64), Vector2(48, 64)), false)

func draw_ellipse_shadow() -> void:
	draw_style_box(_shadow(), Rect2(feet - Vector2(21, 6), Vector2(42, 10)))

func _shadow() -> StyleBoxFlat:
	var box := StyleBoxFlat.new()
	box.bg_color = Color(0, 0, 0, 0.25)
	box.set_corner_radius_all(5)
	return box
