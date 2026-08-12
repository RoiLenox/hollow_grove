extends SceneTree

const SESSION_ID := "session.lifecycle"
const RESPONSE_TIMEOUT_MILLISECONDS := 5000

var client: HollowGroveRuntimeClient
var responses: Array[Dictionary] = []
var protocol_errors: Array[String] = []


func _init() -> void:
	call_deferred("_run")


func _run() -> void:
	var phase := OS.get_environment("HOLLOW_GROVE_LIFECYCLE_PHASE")
	var port := int(OS.get_environment("HOLLOW_GROVE_LIFECYCLE_PORT"))
	var snapshot_path := OS.get_environment("HOLLOW_GROVE_LIFECYCLE_SNAPSHOT")
	if phase not in ["save", "load"] or port <= 0 or snapshot_path == "":
		_fail("playable lifecycle environment is incomplete")
		return

	client = HollowGroveRuntimeClient.new(SESSION_ID)
	client.response_received.connect(_on_response_received)
	client.protocol_error.connect(_on_protocol_error)
	var connection_error := client.connect_to_runtime("127.0.0.1", port)
	if connection_error != OK:
		_fail("runtime connection could not start: %s" % error_string(connection_error))
		return
	if not await _wait_until_connected():
		return

	if phase == "save":
		await _run_save_phase(snapshot_path)
	else:
		await _run_load_phase(snapshot_path)


func _run_save_phase(snapshot_path: String) -> void:
	var response := await _send(
		"request.lifecycle.sync-before-save",
		"SyncIntent"
	)
	if not _require_completed(response, "initial sync"):
		return
	if int(response.get("revision", -1)) != 0:
		_fail("new service did not begin at revision zero")
		return
	var outside_context := _dictionary(_dictionary(response.get("view")).get("guide_context"))
	if str(outside_context.get("layer", "")) != "outside_hollow_grove":
		_fail("new service did not begin with Roï and Rosaliña outside Hollow Grove")
		return

	response = await _send(
		"request.lifecycle.guide-entry",
		"EnterHollowGroveIntent"
	)
	if not _require_completed(response, "Guide entry"):
		return
	if int(response.get("revision", -1)) != 1:
		_fail("Guide entry did not advance the unified revision")
		return
	var entry_view := _dictionary(response.get("view"))
	var entry_context := _dictionary(entry_view.get("guide_context"))
	if str(entry_context.get("layer", "")) != "hollow_grove":
		_fail("Guide entry did not produce Roi and Roselina inside Hollow Grove")
		return
	var transition_events: Array = entry_view.get("guide_transition_events", [])
	if transition_events.is_empty() \
			or str(_dictionary(transition_events.back()).get("kind", "")) != "confirmed":
		_fail("Guide entry did not deliver the confirmed crossing event")
		return

	response = await _send(
		"request.lifecycle.establish",
		"EstablishHuemanIntent",
		{
			"continuity_id": "being-continuity.hueman",
			"participant_id": "participant.hueman",
			"institutional_being_id": "being.hueman",
		}
	)
	if not _require_completed(response, "Hueman establishment"):
		return

	var directions := ["West", "West", "West", "West", "North", "North"]
	for index in range(directions.size()):
		response = await _send(
			"request.lifecycle.move-%d" % index,
			"MoveIntent",
			{"direction": directions[index]}
		)
		if not _require_completed(response, "movement %d" % index):
			return

	response = await _send(
		"request.lifecycle.interact",
		"InteractIntent"
	)
	if not _require_completed(response, "Ridgefolk Guide interaction"):
		return
	var interaction_view: Dictionary = _dictionary(response.get("view")).get(
		"interaction",
		{}
	)
	if str(interaction_view.get("target_id", "")) != "interaction.aura-ridge.ridgefolk-guide":
		_fail("movement did not reach the authoritative Ridgefolk Guide interaction")
		return

	var pilot_actions := [
		"RecoverStonebendStructure",
		"RecoverSandmanorMeasurement",
		"ObtainGlaushouseCompatibility",
		"EarnFlyntWitness",
		"RecoverStonebendMaterial",
		"PreserveSandmanorMethod",
		"RecordStableTrial",
		"GrantClinicalClearance",
		"CompleteSafeFlightAndReturn",
	]
	for index in range(pilot_actions.size()):
		response = await _send(
			"request.lifecycle.gargoyle-pilot-%d" % index,
			"AdvanceWingedGargoylePilotIntent",
			{"action": pilot_actions[index]}
		)
		if not _require_completed(response, "winged Gargoyle pilot step %d" % index):
			return
	var completed_pilot := _dictionary(
		_dictionary(response.get("view")).get("winged_gargoyle_pilot")
	)
	if not bool(completed_pilot.get("completed", false)) \
			or str(completed_pilot.get("form", "")) != "Hueman" \
			or str(completed_pilot.get("capability_id", "")) \
				!= "capability.flynt.telekinesis":
		_fail("winged Gargoyle pilot did not complete a safe return with its unlock")
		return

	response = await _send(
		"request.lifecycle.save",
		"SaveIntent",
		{"slot_id": "slot-lifecycle"}
	)
	if not _require_completed(response, "save"):
		return
	if int(response.get("revision", -1)) != 18:
		_fail("save phase ended at an unexpected revision")
		return
	var view := _dictionary(response.get("view"))
	var overworld := _dictionary(view.get("overworld"))
	var player := _dictionary(overworld.get("player"))
	if (
		int(player.get("x", -1)) != 5
		or int(player.get("y", -1)) != 14
		or str(player.get("facing", "")) != "North"
	):
		_fail("saved player position did not match the exercised path")
		return

	var snapshot := FileAccess.open(snapshot_path, FileAccess.WRITE)
	if snapshot == null:
		_fail("lifecycle snapshot could not be written")
		return
	snapshot.store_string(JSON.stringify(view))
	snapshot.close()
	client.disconnect_from_runtime()
	print("Hollow Grove playable lifecycle save phase passed")
	quit()


func _run_load_phase(snapshot_path: String) -> void:
	if not FileAccess.file_exists(snapshot_path):
		_fail("pre-restart lifecycle snapshot is missing")
		return
	var snapshot := FileAccess.open(snapshot_path, FileAccess.READ)
	if snapshot == null:
		_fail("pre-restart lifecycle snapshot could not be read")
		return
	var expected_view_value = JSON.parse_string(snapshot.get_as_text())
	snapshot.close()
	if typeof(expected_view_value) != TYPE_DICTIONARY:
		_fail("pre-restart lifecycle snapshot is not a game view")
		return
	var expected_view: Dictionary = expected_view_value

	var response := await _send(
		"request.lifecycle.sync-after-restart",
		"SyncIntent"
	)
	if not _require_completed(response, "post-restart sync"):
		return
	if int(response.get("revision", -1)) != 1:
		_fail("restarted service did not restore the Guide entry revision before load")
		return
	var restored_context := _dictionary(_dictionary(response.get("view")).get("guide_context"))
	if str(restored_context.get("layer", "")) != "hollow_grove":
		_fail("restarted service did not restore Roi and Roselina inside Hollow Grove")
		return

	response = await _send(
		"request.lifecycle.load",
		"LoadIntent",
		{"slot_id": "slot-lifecycle"}
	)
	if not _require_completed(response, "post-restart load"):
		return
	if int(response.get("revision", -1)) != 18:
		_fail("loaded service did not restore revision eighteen")
		return
	var loaded_view := _dictionary(response.get("view"))
	if loaded_view != expected_view:
		_fail("loaded authoritative view differs from the pre-restart saved view")
		return
	var interaction := _dictionary(loaded_view.get("interaction"))
	if str(interaction.get("target_id", "")) != "interaction.aura-ridge.ridgefolk-guide":
		_fail("loaded view did not preserve the exercised interaction")
		return
	var loaded_pilot := _dictionary(loaded_view.get("winged_gargoyle_pilot"))
	if not bool(loaded_pilot.get("completed", false)) \
			or str(loaded_pilot.get("form", "")) != "Hueman" \
			or str(loaded_pilot.get("power_recipe_id", "")) \
				!= "recipe.flight.resynce-propulsion":
		_fail("loaded view did not preserve safe-return Synthesis progress")
		return

	response = await _send(
		"request.lifecycle.guide-exit",
		"LeaveHollowGroveIntent"
	)
	if not _require_completed(response, "Guide exit after load"):
		return
	if int(response.get("revision", -1)) != 19:
		_fail("Guide exit did not advance the unified revision")
		return
	var exit_view := _dictionary(response.get("view"))
	var exit_context := _dictionary(exit_view.get("guide_context"))
	if str(exit_context.get("layer", "")) != "outside_hollow_grove":
		_fail("Guide exit did not return to the outside layer")
		return

	response = await _send(
		"request.lifecycle.guide-reentry",
		"EnterHollowGroveIntent"
	)
	if not _require_completed(response, "Guide re-entry after load"):
		return
	if int(response.get("revision", -1)) != 20:
		_fail("Guide re-entry did not advance the unified revision")
		return
	var reentry_context := _dictionary(_dictionary(response.get("view")).get("guide_context"))
	if str(reentry_context.get("layer", "")) != "hollow_grove":
		_fail("Guide re-entry did not restore the Hollow Grove layer")
		return

	client.disconnect_from_runtime()
	print("Hollow Grove playable lifecycle load phase passed")
	quit()


func _wait_until_connected() -> bool:
	var deadline := Time.get_ticks_msec() + RESPONSE_TIMEOUT_MILLISECONDS
	while Time.get_ticks_msec() < deadline:
		client.poll()
		if not protocol_errors.is_empty():
			_fail(protocol_errors.pop_front())
			return false
		if client.is_runtime_connected():
			return true
		await process_frame
	_fail("runtime connection timed out")
	return false


func _send(request_id: String, intent_type: String, payload := {}) -> Dictionary:
	var send_error := client.send_intent(request_id, intent_type, payload)
	if send_error != OK:
		_fail("%s could not be sent: %s" % [intent_type, error_string(send_error)])
		return {}
	var deadline := Time.get_ticks_msec() + RESPONSE_TIMEOUT_MILLISECONDS
	while Time.get_ticks_msec() < deadline:
		client.poll()
		if not protocol_errors.is_empty():
			_fail(protocol_errors.pop_front())
			return {}
		if not responses.is_empty():
			return responses.pop_front()
		await process_frame
	_fail("%s response timed out" % intent_type)
	return {}


func _require_completed(response: Dictionary, operation: String) -> bool:
	if response.is_empty():
		return false
	if str(response.get("status", "")) == "Completed":
		return true
	var rejection := _dictionary(response.get("rejection"))
	_fail("%s was rejected: %s" % [operation, rejection.get("message", "unknown rejection")])
	return false


func _dictionary(value) -> Dictionary:
	return value if typeof(value) == TYPE_DICTIONARY else {}


func _on_response_received(response: Dictionary) -> void:
	responses.append(response)


func _on_protocol_error(message: String) -> void:
	protocol_errors.append(message)


func _fail(message: String) -> void:
	push_error(message)
	if client != null:
		client.disconnect_from_runtime()
	quit(1)
