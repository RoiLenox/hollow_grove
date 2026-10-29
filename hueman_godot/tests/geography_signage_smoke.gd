extends SceneTree

const SCREEN_SHELL := preload("res://scripts/hueman_screen_shell.gd")


func _init() -> void:
	var shell := SCREEN_SHELL.new()
	var contract: Dictionary = JSON.parse_string(FileAccess.get_file_as_string(
		"res://../artifacts/hueman_screen_map.json"
	))
	var before := contract.duplicate(true)
	var highway_count := 0
	var speedway_count := 0
	for route in contract["curved_routes"]:
		if route.get("constitutional_route_id") == "geography.route.stairway-to-heaven":
			highway_count += 1
			assert(shell._public_route_name(route) == "H.T.T. — Highway to Hell / S.T.H. — Stairway to Heaven")
	for route in contract["straight_routes"]:
		if route.get("constitutional_route_id") == "geography.route.basin-motorspeedway":
			speedway_count += 1
			assert(shell._public_route_name(route) == "Basin Motor Speedway (B.M.S.)")
	assert(highway_count == 1)
	assert(speedway_count == 1)
	assert(contract == before)
	shell.free()
	print("Geography signage smoke: pass; one curve, separate straight, source contract unchanged")
	quit(0)
