extends Control

const LOGICAL_SIZE := Vector2(160.0, 144.0)
const VISUAL_COLOR_CONSTITUTION_RELATIVE_PATH := "src/constitutional/hollow_grove_visual_color_palette.json"
const SESSION_ID := "session.hollow-grove.local"
const RETRY_INTERVAL_SECONDS := 1.0
const STEP_DURATION_SECONDS := 0.12
# EnterRegionIntent remains a protocol/archive compatibility command. This
# client traverses only the runtime-projected physical exits below.
const ROUTE_MAP_CYCLE := [
	"aura-ridge.grove-approach",
	"current-sea.deep-certification-landing",
	"aura-beach.coastal-commons",
	"glausbahn.refinement-span",
	"current-seanad.deliberation-chamber",
	"aura-way.design-corridor",
	"aura-field.working-land",
	"mnt-aura.aspiration-path",
	"basin-motor-speedway.production-circuit",
	"aura-basin.collision-grounds",
	"stairway-to-heaven.ascent-path",
	"boardwalk.return-vestibule",
	"riptide.emergency-intake",
	"riptide.current-recovery-rig",
	"current-sea.depth-production-rig",
	"mnt-aura.high-mine",
	"stairway-to-heaven.burden-mine",
	"highway-to-hell.deepworks",
]

const FONT_3X5 := {
	" ": ["000", "000", "000", "000", "000"],
	"A": ["010", "101", "111", "101", "101"],
	"B": ["110", "101", "110", "101", "110"],
	"C": ["011", "100", "100", "100", "011"],
	"D": ["110", "101", "101", "101", "110"],
	"E": ["111", "100", "110", "100", "111"],
	"F": ["111", "100", "110", "100", "100"],
	"G": ["011", "100", "101", "101", "011"],
	"H": ["101", "101", "111", "101", "101"],
	"I": ["111", "010", "010", "010", "111"],
	"J": ["001", "001", "001", "101", "010"],
	"K": ["101", "101", "110", "101", "101"],
	"L": ["100", "100", "100", "100", "111"],
	"M": ["101", "111", "111", "101", "101"],
	"N": ["101", "111", "111", "111", "101"],
	"O": ["010", "101", "101", "101", "010"],
	"P": ["110", "101", "110", "100", "100"],
	"Q": ["010", "101", "101", "111", "011"],
	"R": ["110", "101", "110", "101", "101"],
	"S": ["011", "100", "010", "001", "110"],
	"T": ["111", "010", "010", "010", "010"],
	"U": ["101", "101", "101", "101", "111"],
	"V": ["101", "101", "101", "101", "010"],
	"W": ["101", "101", "111", "111", "101"],
	"X": ["101", "101", "010", "101", "101"],
	"Y": ["101", "101", "010", "010", "010"],
	"Z": ["111", "001", "010", "100", "111"],
	"0": ["111", "101", "101", "101", "111"],
	"1": ["010", "110", "010", "010", "111"],
	"2": ["110", "001", "010", "100", "111"],
	"3": ["110", "001", "010", "001", "110"],
	"4": ["101", "101", "111", "001", "001"],
	"5": ["111", "100", "110", "001", "110"],
	"6": ["011", "100", "111", "101", "111"],
	"7": ["111", "001", "010", "010", "010"],
	"8": ["111", "101", "111", "101", "111"],
	"9": ["111", "101", "111", "001", "110"],
	"-": ["000", "000", "111", "000", "000"],
	".": ["000", "000", "000", "000", "010"],
	":": ["000", "010", "000", "010", "000"],
	"?": ["110", "001", "010", "000", "010"],
}

var repo_root_path := ""
var colors_by_semantic_identity := {}
var runtime_client: HollowGroveRuntimeClient
var overworld := {}
var status_message := "CONNECTING"
var retry_elapsed := RETRY_INTERVAL_SECONDS
var was_connected := false
var sync_pending := false
var establish_pending := false
var movement_pending := false
var interaction_pending := false
var case_pending := false
var request_serial := 0
var display_position := Vector2.ZERO
var target_position := Vector2.ZERO
var animation_from := Vector2.ZERO
var animation_elapsed := STEP_DURATION_SECONDS
var player_facing := "North"
var has_player_position := false
var dialogue_target_id := ""
var dialogue_speaker := ""
var dialogue_pages: Array = []
var dialogue_page_index := 0
var boardwalk_case := {}
var stonebend_case := {}
var route_view := {}
var surface_view := {}
var extraction_view := {}
var living_world := {}
var deep_pressure := {}
var party := {}
var physical_exits: Array = []
var selected_exit_index := 0
var party_menu_open := false
var recruitment_menu_open := false
var party_roster_index := 0


func _ready() -> void:
	repo_root_path = _resolve_repo_root()
	if not _load_visual_color_constitution():
		push_error("The canonical Hollow Grove visual color constitution could not be loaded.")
		set_process(false)
		set_process_input(false)
		return
	runtime_client = HollowGroveRuntimeClient.new(SESSION_ID)
	runtime_client.response_received.connect(_on_runtime_response)
	runtime_client.protocol_error.connect(_on_protocol_error)
	set_process(true)
	set_process_input(true)
	queue_redraw()


func _process(delta: float) -> void:
	if runtime_client == null:
		return

	runtime_client.poll()
	var connected := runtime_client.is_runtime_connected()
	if connected and not was_connected:
		was_connected = true
		retry_elapsed = 0.0
		status_message = "SYNCING GROVE"
		_send_sync()
	elif not connected:
		if was_connected:
			was_connected = false
			sync_pending = false
			establish_pending = false
			movement_pending = false
			interaction_pending = false
			case_pending = false
			_clear_dialogue()
			status_message = "RUNTIME OFFLINE"
		retry_elapsed += delta
		if retry_elapsed >= RETRY_INTERVAL_SECONDS:
			retry_elapsed = 0.0
			runtime_client.disconnect_from_runtime()
			var connection_error := runtime_client.connect_to_runtime()
			if connection_error != OK:
				status_message = "RUNTIME OFFLINE"

	if animation_elapsed < STEP_DURATION_SECONDS:
		animation_elapsed = minf(animation_elapsed + delta, STEP_DURATION_SECONDS)
		var progress := animation_elapsed / STEP_DURATION_SECONDS
		display_position = animation_from.lerp(target_position, progress)
		queue_redraw()


func _input(event: InputEvent) -> void:
	if not (event is InputEventKey) or not event.pressed:
		return
	if movement_pending or interaction_pending or case_pending or animation_elapsed < STEP_DURATION_SECONDS:
		return
	if recruitment_menu_open:
		match event.keycode:
			KEY_1:
				_send_recruitment_path("shared-work")
			KEY_2:
				_send_recruitment_path("recovery-first")
			KEY_3:
				_send_recruitment_path("independent-company")
			KEY_ESCAPE, KEY_R:
				recruitment_menu_open = false
				status_message = _case_status()
				queue_redraw()
		return
	if party_menu_open:
		match event.keycode:
			KEY_P, KEY_ESCAPE:
				party_menu_open = false
				status_message = _case_status()
				queue_redraw()
			KEY_UP, KEY_W, KEY_K:
				_cycle_party_selection(-1)
			KEY_DOWN, KEY_S, KEY_J:
				_cycle_party_selection(1)
			KEY_ENTER, KEY_SPACE, KEY_Z, KEY_X:
				_send_switch_party_lead()
			KEY_U:
				_send_party_action()
		return
	if not dialogue_pages.is_empty():
		if event.keycode == KEY_R:
			_open_recruitment_menu()
			return
		if _is_action_key(event.keycode):
			if dialogue_page_index + 1 < dialogue_pages.size():
				dialogue_page_index += 1
			else:
				_clear_dialogue()
			queue_redraw()
		return
	if overworld.is_empty():
		return
	match event.keycode:
		KEY_P:
			_open_party_menu()
			return
		KEY_R:
			_open_recruitment_menu()
			return
		KEY_B:
			_send_map_toggle()
			return
		KEY_TAB:
			_select_next_exit()
			return
		KEY_T:
			_send_advance_shift()
			return
		KEY_F:
			_send_next_faculty()
			return
		KEY_1:
			_send_case_support(1)
			return
		KEY_2:
			_send_case_support(2)
			return
		KEY_3:
			_send_case_support(3)
			return
		KEY_4:
			_send_case_support(4)
			return
		KEY_C:
			_send_case_decision()
			return
		KEY_F5:
			_send_save()
			return
		KEY_F9:
			_send_load()
			return
	if _is_action_key(event.keycode):
		interaction_pending = true
		status_message = "LISTENING"
		var interaction_error := runtime_client.send_intent(
			_next_request_id("interact"),
			"InteractIntent"
		)
		if interaction_error != OK:
			interaction_pending = false
			status_message = "INTERACT SEND ERROR"
		queue_redraw()
		return

	var direction := ""
	match event.keycode:
		KEY_UP, KEY_W, KEY_K:
			direction = "North"
		KEY_RIGHT, KEY_D, KEY_L:
			direction = "East"
		KEY_DOWN, KEY_S, KEY_J:
			direction = "South"
		KEY_LEFT, KEY_A, KEY_H:
			direction = "West"
		_:
			return

	movement_pending = true
	status_message = _map_label()
	var error := runtime_client.send_intent(_next_request_id("move"), "MoveIntent", {
		"direction": direction,
	})
	if error != OK:
		movement_pending = false
		status_message = "MOVE SEND ERROR"
	queue_redraw()


func _draw() -> void:
	if colors_by_semantic_identity.is_empty():
		return
	var viewport_size := get_viewport_rect().size
	draw_rect(Rect2(Vector2.ZERO, viewport_size), _color("hollow_grove.universal.outline"))

	if overworld.is_empty():
		_draw_waiting_screen(viewport_size)
		return

	var tile_rows: Array = overworld.get("tile_rows", [])
	for y in range(tile_rows.size()):
		var row := str(tile_rows[y])
		for x in range(row.length()):
			_draw_tile(row.substr(x, 1), x, y, viewport_size)

	_draw_environment(viewport_size)
	_draw_scheduled_people(viewport_size)
	_draw_player(viewport_size)
	_draw_hud(viewport_size)
	if party_menu_open:
		_draw_party_menu(viewport_size)
	elif recruitment_menu_open:
		_draw_recruitment_menu(viewport_size)


func _send_sync() -> void:
	if sync_pending or not runtime_client.is_runtime_connected():
		return
	sync_pending = true
	var error := runtime_client.send_intent(_next_request_id("sync"), "SyncIntent")
	if error != OK:
		sync_pending = false
		status_message = "SYNC SEND ERROR"


func _send_establish_hueman() -> void:
	if establish_pending:
		return
	establish_pending = true
	status_message = "ESTABLISHING HUEMAN"
	var error := runtime_client.send_intent(
		_next_request_id("establish"),
		"EstablishHuemanIntent",
		{
			"continuity_id": "being-continuity.hueman",
			"participant_id": "participant.hueman",
			"institutional_being_id": "being.hueman",
		}
	)
	if error != OK:
		establish_pending = false
		status_message = "HUEMAN SEND ERROR"


func _send_map_toggle() -> void:
	if physical_exits.is_empty():
		status_message = "NO PHYSICAL EXIT HERE"
		queue_redraw()
		return
	var exit_record: Dictionary = _dict_or_empty(
		physical_exits[selected_exit_index % physical_exits.size()]
	)
	var player: Dictionary = _dict_or_empty(overworld.get("player"))
	if int(player.get("x", -1)) != int(exit_record.get("exit_x", -2)) \
			or int(player.get("y", -1)) != int(exit_record.get("exit_y", -2)):
		status_message = "RETURN TO THE EXIT MARKER"
		queue_redraw()
		return
	var target_map := str(exit_record.get("destination_map_id", ""))
	if target_map == "":
		status_message = "EXIT HAS NO DESTINATION"
		queue_redraw()
		return
	case_pending = true
	status_message = "EXIT TO %s" % str(exit_record.get("destination_label", "REGION")).to_upper()
	var error := runtime_client.send_intent(
		_next_request_id("exit"),
		"TraverseExitIntent",
		{"destination_map_id": target_map}
	)
	if error != OK:
		case_pending = false
		status_message = "EXIT SEND ERROR"


func _select_next_exit() -> void:
	if physical_exits.is_empty():
		status_message = "NO CONNECTED EXIT"
		queue_redraw()
		return
	selected_exit_index = (selected_exit_index + 1) % physical_exits.size()
	var exit_record: Dictionary = _dict_or_empty(physical_exits[selected_exit_index])
	status_message = "B: %s" % str(exit_record.get("destination_label", "EXIT")).to_upper()
	queue_redraw()


func _send_advance_shift() -> void:
	case_pending = true
	status_message = "TIME MOVES"
	var error := runtime_client.send_intent(
		_next_request_id("shift"),
		"AdvanceWorldShiftIntent"
	)
	if error != OK:
		case_pending = false
		status_message = "SHIFT SEND ERROR"


func _open_party_menu() -> void:
	if party.is_empty():
		status_message = "PARTY NOT ESTABLISHED"
		queue_redraw()
		return
	party_menu_open = true
	recruitment_menu_open = false
	_sync_party_roster_index()
	status_message = "PARTY"
	var error := runtime_client.send_intent(_next_request_id("open-party"), "OpenPartyIntent")
	if error != OK:
		party_menu_open = false
		status_message = "PARTY SEND ERROR"
	queue_redraw()


func _open_recruitment_menu() -> void:
	var candidate := _current_recruitment_candidate()
	if candidate.is_empty():
		status_message = "FACE A RECRUITMENT CANDIDATE"
		queue_redraw()
		return
	if candidate.get("decision", null) != null:
		status_message = "THEIR DECISION ALREADY STANDS"
		queue_redraw()
		return
	recruitment_menu_open = true
	party_menu_open = false
	status_message = "CHOOSE HOW TO ASK"
	queue_redraw()


func _send_recruitment_path(path: String) -> void:
	var candidate := _current_recruitment_candidate()
	if candidate.is_empty():
		recruitment_menu_open = false
		status_message = "CANDIDATE IS NOT PRESENT"
		queue_redraw()
		return
	case_pending = true
	recruitment_menu_open = false
	status_message = "THEY DECIDE"
	var error := runtime_client.send_intent(
		_next_request_id("recruit"),
		"RecruitIntent",
		{
			"target_id": str(candidate.get("stable_id", "")),
			"recruitment_path": path,
		}
	)
	if error != OK:
		case_pending = false
		status_message = "RECRUITMENT SEND ERROR"


func _party_roster() -> Array:
	var roster: Array = [{
		"display_name": "Hueman",
		"continuity_id": str(party.get("hueman_continuity_id", "being-continuity.hueman")),
		"role": "persistent Hueman",
		"availability": "Ready",
		"field_action_id": "FACULTIES",
		"is_lead": str(party.get("lead_continuity_id", "")) == "being-continuity.hueman",
	}]
	for member_value in party.get("members", []):
		roster.append(_dict_or_empty(member_value))
	return roster


func _sync_party_roster_index() -> void:
	var roster := _party_roster()
	var selected_id := str(party.get("selected_continuity_id", "being-continuity.hueman"))
	for index in range(roster.size()):
		if str(_dict_or_empty(roster[index]).get("continuity_id", "")) == selected_id:
			party_roster_index = index
			return
	party_roster_index = 0


func _cycle_party_selection(delta: int) -> void:
	var roster := _party_roster()
	if roster.is_empty():
		return
	party_roster_index = posmod(party_roster_index + delta, roster.size())
	var member: Dictionary = _dict_or_empty(roster[party_roster_index])
	case_pending = true
	status_message = "SELECT %s" % str(member.get("display_name", "MEMBER")).to_upper()
	var error := runtime_client.send_intent(
		_next_request_id("party-select"),
		"SelectPartyMemberIntent",
		{"continuity_id": str(member.get("continuity_id", ""))}
	)
	if error != OK:
		case_pending = false
		status_message = "SELECT SEND ERROR"


func _send_switch_party_lead() -> void:
	var roster := _party_roster()
	if roster.is_empty():
		return
	var member: Dictionary = _dict_or_empty(roster[party_roster_index % roster.size()])
	case_pending = true
	status_message = "ASKING %s TO LEAD" % str(member.get("display_name", "MEMBER")).to_upper()
	var error := runtime_client.send_intent(
		_next_request_id("party-lead"),
		"SwitchLeadIntent",
		{"continuity_id": str(member.get("continuity_id", ""))}
	)
	if error != OK:
		case_pending = false
		status_message = "LEAD SEND ERROR"


func _send_party_action() -> void:
	var roster := _party_roster()
	if roster.is_empty():
		return
	var member: Dictionary = _dict_or_empty(roster[party_roster_index % roster.size()])
	var action_id := str(member.get("field_action_id", ""))
	if action_id == "" or action_id == "FACULTIES":
		status_message = "NO COMPANION FIELD ACTION"
		queue_redraw()
		return
	case_pending = true
	status_message = "FIELD ACTION"
	var error := runtime_client.send_intent(
		_next_request_id("party-action"),
		"UseActionIntent",
		{
			"actor_continuity_id": str(member.get("continuity_id", "")),
			"action_id": action_id,
			"target_continuity_id": null,
		}
	)
	if error != OK:
		case_pending = false
		status_message = "ACTION SEND ERROR"


func _send_next_faculty() -> void:
	var active_case := boardwalk_case
	if str(overworld.get("map_id", "")) == "current-sea.deep-certification-landing":
		active_case = stonebend_case
	if active_case.is_empty():
		status_message = "ENTER A CASE REGION FIRST"
		queue_redraw()
		return
	var observed: Array = active_case.get("faculties", [])
	for faculty in ["Reason", "Memory", "Imagination", "Perception", "Will"]:
		if faculty not in observed:
			case_pending = true
			status_message = "DISCLOSING %s" % faculty.to_upper()
			var error := runtime_client.send_intent(
				_next_request_id("faculty"),
				"DiscloseFacultyObservationIntent",
				{"faculty": faculty}
			)
			if error != OK:
				case_pending = false
				status_message = "FACULTY SEND ERROR"
			return
	status_message = "FIVE FACULTIES COMPLETE"
	queue_redraw()


func _send_case_support(option: int) -> void:
	var current_map := str(overworld.get("map_id", ""))
	var intent_type := "SupportBoardwalkOptionIntent"
	var choice := ""
	if current_map == "current-sea.deep-certification-landing":
		if stonebend_case.is_empty() or option > 3:
			status_message = "STONEBEND OPTIONS ARE 1 TO 3"
			queue_redraw()
			return
		intent_type = "SupportStonebendContinuityOptionIntent"
		choice = [
			"AffirmExistingName",
			"ProvisionalTransformedFormName",
			"ReferIdentityConflict",
		][option - 1]
	elif current_map == "boardwalk.return-vestibule":
		if str(deep_pressure.get("phase", "")) == "BoardwalkSettlement" \
				and deep_pressure.get("outcome", null) == null:
			var deep_choices := [
				"SharedBurdenCompact",
				"CrewAndCoastRestitution",
				"ProductionUnderReview",
				"ProtectedRefusal",
			]
			if option == 3 and not bool(
				deep_pressure.get("production_under_review_available", false)
			):
				status_message = "COMPROMISED RECORD BARS OPTION 3"
				queue_redraw()
				return
			case_pending = true
			status_message = "RECORDING RECOVERY SUPPORT"
			var deep_error := runtime_client.send_intent(
				_next_request_id("deep-pressure-support"),
				"SupportDeepPressureSettlementIntent",
				{"choice": deep_choices[option - 1]}
			)
			if deep_error != OK:
				case_pending = false
				status_message = "SETTLEMENT SEND ERROR"
			return
		if boardwalk_case.is_empty():
			status_message = "ENTER BOARDWALK FIRST"
			queue_redraw()
			return
		choice = [
			"PimpPatronage",
			"GoonBond",
			"LimitedCooperation",
			"IndependentReturn",
		][option - 1]
	else:
		var living_case := _active_living_case()
		if living_case.is_empty():
			status_message = "NO ACTIVE CASE HERE"
			queue_redraw()
			return
		var choices: Array = _living_case_choices(str(living_case.get("case_id", "")))
		if option > choices.size():
			status_message = "NO OPTION %d HERE" % option
			queue_redraw()
			return
		case_pending = true
		status_message = "DUTY OFFICER DECIDES"
		var living_error := runtime_client.send_intent(
			_next_request_id("living-case"),
			"SupportLivingCaseOptionIntent",
			{
				"case_id": living_case.get("case_id"),
				"choice": choices[option - 1],
			}
		)
		if living_error != OK:
			case_pending = false
			status_message = "CASE SEND ERROR"
		return
	case_pending = true
	status_message = "RECORDING SUPPORT"
	var error := runtime_client.send_intent(
		_next_request_id("support"),
		intent_type,
		{"choice": choice}
	)
	if error != OK:
		case_pending = false
		status_message = "SUPPORT SEND ERROR"


func _send_case_decision() -> void:
	var current_map := str(overworld.get("map_id", ""))
	var intent_type := "AskReturningGoonToDecideIntent"
	if current_map == "current-sea.deep-certification-landing":
		if stonebend_case.is_empty():
			status_message = "ENTER CURRENT SEA FIRST"
			queue_redraw()
			return
		intent_type = "AskStonebendToDetermineContinuityIntent"
		status_message = "STONEBEND DETERMINES"
	elif current_map == "boardwalk.return-vestibule":
		if str(deep_pressure.get("phase", "")) == "BoardwalkSettlement" \
				and deep_pressure.get("supported_settlement", null) != null:
			intent_type = "AskDeepPressureAssemblyToCommitIntent"
			status_message = "AFFECTED ASSEMBLY COMMITS"
		elif str(deep_pressure.get("phase", "")) == "BoardwalkSettlement":
			status_message = "SUPPORT A SETTLEMENT FIRST"
			queue_redraw()
			return
		elif str(deep_pressure.get("phase", "")) == "PersistentAftermath":
			status_message = "DEEP PRESSURE AFTERMATH PERSISTS"
			queue_redraw()
			return
		elif boardwalk_case.is_empty():
			status_message = "ENTER BOARDWALK FIRST"
			queue_redraw()
			return
		else:
			status_message = "RETURNING GOON DECIDES"
	else:
		var living_case := _active_living_case()
		if living_case.is_empty():
			status_message = "ENTER A CASE REGION FIRST"
			queue_redraw()
			return
		if living_case.get("supported_choice", null) == null:
			status_message = "SUPPORT A LAWFUL OPTION FIRST"
			queue_redraw()
			return
		intent_type = "AskLivingDutyOfficerToDecideIntent"
		status_message = "DUTY OFFICER DECIDES"
	case_pending = true
	var payload := {}
	if intent_type == "AskLivingDutyOfficerToDecideIntent":
		payload = {"case_id": _active_living_case().get("case_id")}
	var error := runtime_client.send_intent(_next_request_id("decide"), intent_type, payload)
	if error != OK:
		case_pending = false
		status_message = "DECISION SEND ERROR"


func _send_save() -> void:
	case_pending = true
	status_message = "SAVING SLOT A"
	var error := runtime_client.send_intent(
		_next_request_id("save"),
		"SaveIntent",
		{"slot_id": "slot-a"}
	)
	if error != OK:
		case_pending = false
		status_message = "SAVE SEND ERROR"


func _send_load() -> void:
	case_pending = true
	status_message = "LOADING SLOT A"
	var error := runtime_client.send_intent(
		_next_request_id("load"),
		"LoadIntent",
		{"slot_id": "slot-a"}
	)
	if error != OK:
		case_pending = false
		status_message = "LOAD SEND ERROR"


func _on_runtime_response(response: Dictionary) -> void:
	sync_pending = false
	establish_pending = false
	movement_pending = false
	interaction_pending = false
	case_pending = false
	var view: Dictionary = _dict_or_empty(response.get("view"))
	if not view.is_empty():
		_apply_view(view)

	if str(response.get("status", "Rejected")) == "Rejected":
		var rejection: Dictionary = _dict_or_empty(response.get("rejection"))
		status_message = str(rejection.get("message", "PROTOCOL REJECTED")).to_upper()
		if str(rejection.get("code", "")) == "StaleRevision":
			_send_sync()
		queue_redraw()
		return

	if view.get("hueman", null) == null:
		_send_establish_hueman()
	elif not overworld.is_empty():
		status_message = _case_status()
		for event_value in response.get("events", []):
			var event_record: Dictionary = _dict_or_empty(event_value)
			if str(event_record.get("kind", "")) == "ActionResolved":
				var actions: Array = party.get("field_actions", [])
				if not actions.is_empty():
					var action: Dictionary = _dict_or_empty(actions.back())
					status_message = str(action.get("finding", "FIELD ACTION RECORDED")).to_upper()
			elif str(event_record.get("kind", "")) == "PartyChanged":
				status_message = "PARTY DECISION PERSISTS"
			elif str(event_record.get("kind", "")) == "LeadChanged":
				status_message = "PARTY LEAD CHANGED"
	queue_redraw()


func _apply_view(view: Dictionary) -> void:
	var next_overworld: Dictionary = _dict_or_empty(view.get("overworld"))
	if next_overworld.is_empty():
		return
	overworld = next_overworld
	boardwalk_case = _dict_or_empty(view.get("boardwalk_case"))
	stonebend_case = _dict_or_empty(view.get("stonebend_case"))
	route_view = _dict_or_empty(view.get("route"))
	surface_view = _dict_or_empty(view.get("surface"))
	extraction_view = _dict_or_empty(view.get("extraction_site"))
	living_world = _dict_or_empty(view.get("living_world"))
	deep_pressure = _dict_or_empty(view.get("deep_pressure"))
	party = _dict_or_empty(view.get("party"))
	physical_exits = view.get("physical_exits", [])
	selected_exit_index = 0
	var player: Dictionary = _dict_or_empty(overworld.get("player"))
	var next_position := Vector2(float(player.get("x", 0)), float(player.get("y", 0)))
	player_facing = str(player.get("facing", "North"))
	if not has_player_position:
		display_position = next_position
		target_position = next_position
		animation_from = next_position
		animation_elapsed = STEP_DURATION_SECONDS
		has_player_position = true
	elif next_position != target_position:
		animation_from = display_position
		target_position = next_position
		animation_elapsed = 0.0
	else:
		display_position = next_position
		target_position = next_position
	_apply_interaction(_dict_or_empty(view.get("interaction")))
	if party_menu_open:
		_sync_party_roster_index()
	queue_redraw()


func _apply_interaction(interaction: Dictionary) -> void:
	if interaction.is_empty():
		_clear_dialogue()
		return
	var next_pages: Array = []
	for page in interaction.get("pages", []):
		if typeof(page) == TYPE_STRING and str(page) != "":
			next_pages.append(str(page))
	if next_pages.is_empty():
		_clear_dialogue()
		return
	var next_target_id := str(interaction.get("target_id", ""))
	if next_target_id != dialogue_target_id or dialogue_pages.is_empty():
		dialogue_page_index = 0
	dialogue_target_id = next_target_id
	dialogue_speaker = str(interaction.get("speaker", "AURA RIDGE"))
	dialogue_pages = next_pages


func _clear_dialogue() -> void:
	dialogue_target_id = ""
	dialogue_speaker = ""
	dialogue_pages.clear()
	dialogue_page_index = 0


func _is_action_key(keycode: Key) -> bool:
	return keycode in [KEY_ENTER, KEY_SPACE, KEY_Z, KEY_X]


func _on_protocol_error(message: String) -> void:
	status_message = "PROTOCOL ERROR"
	push_warning(message)
	queue_redraw()


func _draw_waiting_screen(viewport_size: Vector2) -> void:
	_draw_logical_rect(Rect2(0, 0, 160, 144), _color("hollow_grove.house.glaushouse.dark"), viewport_size)
	for y in range(0, 144, 8):
		for x in range(0, 160, 8):
			if int((x + y) / 8) % 2 == 0:
				_draw_logical_rect(Rect2(x, y, 8, 8), _color("hollow_grove.house.glaushouse.primary"), viewport_size)
	_draw_message_box(status_message, viewport_size)


func _draw_tile(tile: String, x: int, y: int, viewport_size: Vector2) -> void:
	var map_id := str(overworld.get("map_id", ""))
	if map_id == "aura-field.working-land":
		_draw_aura_field_tile(tile, x, y, viewport_size)
		return
	if map_id == "aura-beach.coastal-commons":
		_draw_aura_beach_tile(tile, x, y, viewport_size)
		return
	if map_id == "aura-basin.collision-grounds":
		_draw_aura_basin_tile(tile, x, y, viewport_size)
		return
	if not extraction_view.is_empty():
		_draw_extraction_tile(tile, x, y, viewport_size)
		return
	var origin := Vector2(x * 8, y * 8)
	var grove_dark := _color("hollow_grove.house.glaushouse.dark")
	var grove := _color("hollow_grove.house.glaushouse.primary")
	var grove_light := _color("hollow_grove.house.glaushouse.highlight")
	var outline := _color("hollow_grove.universal.outline")
	var stone := _color("hollow_grove.house.stonebend.primary")
	var stone_light := _color("hollow_grove.house.stonebend.highlight")
	var sand := _color("hollow_grove.house.sandmanor.primary")
	var sand_light := _color("hollow_grove.house.sandmanor.highlight")
	var flynt_light := _color("hollow_grove.house.flynt.highlight")

	_draw_logical_rect(Rect2(origin, Vector2(8, 8)), grove, viewport_size)
	match tile:
		"~":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), _color("hollow_grove.house.stonebend.dark"), viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(0, 2), Vector2(5, 1)), stone_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 6), Vector2(5, 1)), stone, viewport_size)
		"x":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 4)), stone_light, viewport_size)
		"T":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), grove_dark, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 5)), grove, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(3, 1)), grove_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 6), Vector2(2, 2)), outline, viewport_size)
		"=":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), flynt_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(2, 1)), stone_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(5, 5), Vector2(2, 1)), stone, viewport_size)
		"H":
			for blade_x in [1, 4, 6]:
				_draw_logical_rect(Rect2(origin + Vector2(blade_x, 2), Vector2(1, 5)), grove_dark, viewport_size)
				_draw_logical_rect(Rect2(origin + Vector2(blade_x + 1, 4), Vector2(1, 3)), grove_light, viewport_size)
		"W":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(0, 2), Vector2(5, 1)), stone_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 6), Vector2(5, 1)), stone_light, viewport_size)
		"C":
			if y == 5:
				_draw_logical_rect(Rect2(origin, Vector2(8, 8)), _color("hollow_grove.house.sandmanor.dark"), viewport_size)
				_draw_logical_rect(Rect2(origin + Vector2(0, 4), Vector2(8, 4)), sand, viewport_size)
			elif y == 6:
				_draw_logical_rect(Rect2(origin, Vector2(8, 3)), sand, viewport_size)
				_draw_logical_rect(Rect2(origin + Vector2(0, 3), Vector2(8, 5)), flynt_light, viewport_size)
			else:
				_draw_logical_rect(Rect2(origin, Vector2(8, 8)), flynt_light, viewport_size)
			_draw_logical_rect(Rect2(origin, Vector2(8, 1)), outline, viewport_size)
		"D":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), flynt_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(4, 7)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 2), Vector2(2, 6)), _color("hollow_grove.universal.shadow.raised"), viewport_size)
		"F":
			for flower in [Vector2(2, 2), Vector2(5, 5)]:
				_draw_logical_rect(Rect2(origin + flower, Vector2(3, 3)), sand, viewport_size)
				_draw_logical_rect(Rect2(origin + flower + Vector2(1, 1), Vector2(1, 1)), sand_light, viewport_size)
		"S":
			_draw_logical_rect(Rect2(origin + Vector2(3, 3), Vector2(2, 5)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 4)), sand, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 1)), sand_light, viewport_size)
		"N":
			_draw_logical_rect(Rect2(origin + Vector2(2, 0), Vector2(4, 3)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 1), Vector2(3, 2)), flynt_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 3), Vector2(5, 4)), _color("hollow_grove.house.stonebend.dark"), viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 7), Vector2(1, 1)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(5, 7), Vector2(1, 1)), outline, viewport_size)
		"P", "G", "O", "R", "A", "M", "m", "V", "L", "Y", "K", "?":
			var body_color := flynt_light
			if tile in ["G", "N", "M", "m", "V", "L", "Y", "K", "?"]:
				body_color = stone
			elif tile == "A":
				body_color = sand_light
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(4, 3)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 2), Vector2(2, 2)), body_color, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 4), Vector2(5, 4)), body_color, viewport_size)
			_draw_pixel_text(tile, origin + Vector2(3, 2), outline, viewport_size)
		"I":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), sand, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), flynt_light, viewport_size)
			_draw_pixel_text("I", origin + Vector2(2, 2), outline, viewport_size)
		_:
			if (x * 3 + y * 5) % 7 == 0:
				_draw_logical_rect(Rect2(origin + Vector2(2, 3), Vector2(1, 1)), grove_light, viewport_size)


func _draw_aura_field_tile(tile: String, x: int, y: int, viewport_size: Vector2) -> void:
	var origin := Vector2(x * 8, y * 8)
	var outline := _color("hollow_grove.universal.outline")
	var soil := _color("hollow_grove.house.sandmanor.dark")
	var crop := _color("hollow_grove.house.glaushouse.primary")
	var crop_light := _color("hollow_grove.house.glaushouse.highlight")
	var timber := _color("hollow_grove.house.stonebend.dark")
	var stone := _color("hollow_grove.house.stonebend.primary")
	var path := _color("hollow_grove.house.flynt.highlight")
	var aura := _color("hollow_grove.house.sandmanor.highlight")

	_draw_logical_rect(Rect2(origin, Vector2(8, 8)), soil, viewport_size)
	match tile:
		"T":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), crop, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(3, 2)), crop_light, viewport_size)
		".":
			_draw_logical_rect(Rect2(origin + Vector2(2, 3), Vector2(1, 1)), crop_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(6, 6), Vector2(1, 1)), crop, viewport_size)
		"=":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), path, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 2), Vector2(2, 1)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(5, 6), Vector2(2, 1)), stone, viewport_size)
		"c":
			for crop_x in [1, 4, 6]:
				_draw_logical_rect(Rect2(origin + Vector2(crop_x, 1), Vector2(1, 7)), crop, viewport_size)
				_draw_logical_rect(Rect2(origin + Vector2(crop_x + 1, 3), Vector2(1, 2)), crop_light, viewport_size)
		"o":
			_draw_logical_rect(Rect2(origin + Vector2(3, 5), Vector2(2, 3)), timber, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 5)), crop, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(3, 2)), crop_light, viewport_size)
		"p":
			_draw_logical_rect(Rect2(origin + Vector2(0, 2), Vector2(8, 1)), path, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(1, 7)), timber, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(6, 1), Vector2(1, 7)), timber, viewport_size)
		"B":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), timber, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 2)), aura, viewport_size)
		"G":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 1)), aura, viewport_size)
		"h":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), path, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), timber, viewport_size)
		"m":
			_draw_logical_rect(Rect2(origin + Vector2(0, 1), Vector2(8, 2)), aura, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 3), Vector2(1, 5)), timber, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(6, 3), Vector2(1, 5)), timber, viewport_size)
		_:
			var marker_color := crop_light
			if tile in ["I", "V", "M"]:
				marker_color = aura
			elif tile in ["N", "K", "H", "L", "S"]:
				marker_color = stone
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 4)), marker_color, viewport_size)
			_draw_pixel_text(tile, origin + Vector2(3, 2), outline, viewport_size)


func _draw_aura_beach_tile(tile: String, x: int, y: int, viewport_size: Vector2) -> void:
	var origin := Vector2(x * 8, y * 8)
	var outline := _color("hollow_grove.universal.outline")
	var deep_water := _color("hollow_grove.house.stonebend.dark")
	var water := _color("hollow_grove.house.stonebend.primary")
	var foam := _color("hollow_grove.house.stonebend.highlight")
	var sand := _color("hollow_grove.house.sandmanor.highlight")
	var wet_sand := _color("hollow_grove.house.sandmanor.primary")
	var signal := _color("hollow_grove.house.flynt.highlight")

	_draw_logical_rect(Rect2(origin, Vector2(8, 8)), wet_sand, viewport_size)
	match tile:
		"~":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), deep_water, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(0, 2), Vector2(5, 1)), foam, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 6), Vector2(5, 1)), water, viewport_size)
		"=":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), sand, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 5), Vector2(2, 1)), wet_sand, viewport_size)
		".":
			_draw_logical_rect(Rect2(origin + Vector2(2, 3), Vector2(2, 1)), sand, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(6, 6), Vector2(1, 1)), foam, viewport_size)
		_:
			var marker_color := signal
			if tile in ["V", "A", "E", "C"]:
				marker_color = _color("hollow_grove.house.sandmanor.primary")
			elif tile in ["R", "H"]:
				marker_color = _color("hollow_grove.house.glaushouse.highlight")
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 4)), marker_color, viewport_size)
			_draw_pixel_text(tile, origin + Vector2(3, 2), outline, viewport_size)


func _draw_aura_basin_tile(tile: String, x: int, y: int, viewport_size: Vector2) -> void:
	var origin := Vector2(x * 8, y * 8)
	var outline := _color("hollow_grove.universal.outline")
	var ground := _color("hollow_grove.house.flynt.dark")
	var trail := _color("hollow_grove.house.flynt.highlight")
	var stone := _color("hollow_grove.house.stonebend.primary")
	var stone_light := _color("hollow_grove.house.stonebend.highlight")
	var growth := _color("hollow_grove.house.glaushouse.primary")
	var growth_light := _color("hollow_grove.house.glaushouse.highlight")

	_draw_logical_rect(Rect2(origin, Vector2(8, 8)), ground, viewport_size)
	match tile:
		"T":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), growth, viewport_size)
		".":
			if (x * 5 + y * 3) % 7 == 0:
				_draw_logical_rect(Rect2(origin + Vector2(3, 4), Vector2(2, 1)), stone_light, viewport_size)
		"=":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), trail, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 2), Vector2(2, 1)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(5, 6), Vector2(2, 1)), stone, viewport_size)
		"r":
			_draw_logical_rect(Rect2(origin + Vector2(1, 2), Vector2(6, 5)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(3, 2)), stone_light, viewport_size)
		"d":
			_draw_logical_rect(Rect2(origin + Vector2(0, 2), Vector2(8, 6)), growth, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 4), Vector2(4, 4)), outline, viewport_size)
		"t":
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 7)), growth, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 1), Vector2(3, 2)), growth_light, viewport_size)
		"s":
			_draw_logical_rect(Rect2(origin + Vector2(1, 4), Vector2(6, 3)), stone, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 2)), trail, viewport_size)
		_:
			var marker_color := trail
			if tile in ["L", "D"]:
				marker_color = stone_light
			elif tile in ["E", "G", "R"]:
				marker_color = growth_light
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 4)), marker_color, viewport_size)
			_draw_pixel_text(tile, origin + Vector2(3, 2), outline, viewport_size)


func _draw_extraction_tile(tile: String, x: int, y: int, viewport_size: Vector2) -> void:
	var origin := Vector2(x * 8, y * 8)
	var outline := _color("hollow_grove.universal.outline")
	var stone_dark := _color("hollow_grove.house.stonebend.dark")
	var stone := _color("hollow_grove.house.stonebend.primary")
	var stone_light := _color("hollow_grove.house.stonebend.highlight")
	var steel := _color("hollow_grove.house.flynt.highlight")
	var hazard := _color("hollow_grove.house.sandmanor.primary")
	var water := _color("hollow_grove.house.stonebend.primary")
	var offshore := str(extraction_view.get("method", "")) == "OffshoreCurrentWell"

	_draw_logical_rect(Rect2(origin, Vector2(8, 8)), water if offshore else stone_dark, viewport_size)
	match tile:
		"~":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), stone_dark, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(0, 2), Vector2(5, 1)), stone_light, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(3, 6), Vector2(5, 1)), water, viewport_size)
		"T":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), stone, viewport_size)
		"=":
			_draw_logical_rect(Rect2(origin, Vector2(8, 8)), steel, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(1, 3), Vector2(6, 2)), stone_light, viewport_size)
		".":
			if (x * 3 + y * 7) % 8 == 0:
				_draw_logical_rect(Rect2(origin + Vector2(3, 4), Vector2(2, 1)), stone_light, viewport_size)
		_:
			var marker := hazard if tile in ["P", "V", "W", "F", "B"] else stone_light
			_draw_logical_rect(Rect2(origin + Vector2(1, 1), Vector2(6, 6)), outline, viewport_size)
			_draw_logical_rect(Rect2(origin + Vector2(2, 2), Vector2(4, 4)), marker, viewport_size)
			_draw_pixel_text(tile, origin + Vector2(3, 2), outline, viewport_size)


func _draw_environment(viewport_size: Vector2) -> void:
	var weather := str(living_world.get("weather", "Clear"))
	if weather == "Storm":
		for x in range(3, 160, 13):
			_draw_logical_rect(
				Rect2(x, 12 + ((x * 5) % 91), 1, 5),
				_color("hollow_grove.house.stonebend.highlight"),
				viewport_size
			)
	elif weather == "Crosswind":
		for y in range(18, 112, 23):
			_draw_logical_rect(
				Rect2(7 + ((y * 3) % 31), y, 8, 1),
				_color("hollow_grove.house.flynt.highlight"),
				viewport_size
			)
	elif weather == "PressureDrop":
		for x in range(8, 154, 19):
			_draw_logical_rect(
				Rect2(x, 17 + ((x * 7) % 83), 1, 1),
				_color("hollow_grove.house.sandmanor.highlight"),
				viewport_size
			)


func _draw_scheduled_people(viewport_size: Vector2) -> void:
	for record_value in deep_pressure.get("present_people", []):
		var record: Dictionary = _dict_or_empty(record_value)
		var position: Dictionary = _dict_or_empty(record.get("position"))
		var origin := Vector2(
			float(position.get("x", 0)) * 8.0 - 1.0,
			float(position.get("y", 0)) * 8.0 - 7.0
		)
		var outline := _color("hollow_grove.universal.outline")
		var body := _color("hollow_grove.house.flynt.primary")
		var role := str(record.get("role", "")).to_lower()
		if "rescue" in role:
			body = _color("hollow_grove.house.glaushouse.primary")
		elif "mine" in role or "custody" in role or "hoist" in role:
			body = _color("hollow_grove.house.stonebend.primary")
		elif "weather" in role or "irrigation" in role:
			body = _color("hollow_grove.house.sandmanor.primary")
		_draw_logical_rect(Rect2(origin + Vector2(2, 0), Vector2(6, 5)), outline, viewport_size)
		_draw_logical_rect(Rect2(origin + Vector2(3, 1), Vector2(4, 3)), body, viewport_size)
		_draw_logical_rect(Rect2(origin + Vector2(1, 5), Vector2(8, 7)), outline, viewport_size)
		_draw_logical_rect(Rect2(origin + Vector2(2, 6), Vector2(6, 5)), body, viewport_size)
		_draw_logical_rect(Rect2(origin + Vector2(2, 12), Vector2(2, 2)), outline, viewport_size)
		_draw_logical_rect(Rect2(origin + Vector2(6, 12), Vector2(2, 2)), outline, viewport_size)
		_draw_pixel_text(
			str(record.get("initials", "?")).left(1),
			origin + Vector2(4, 6),
			_color("hollow_grove.house.flynt.highlight"),
			viewport_size
		)


func _draw_player(viewport_size: Vector2) -> void:
	var actor_origin := Vector2(
		roundf(display_position.x * 8.0) - 2.0,
		roundf(display_position.y * 8.0) - 8.0
	)
	var outline := _color("hollow_grove.universal.outline")
	var hair := _color("hollow_grove.universal.shadow.deep")
	var cloak_dark := _color("hollow_grove.house.glaushouse.dark")
	var cloak := _color("hollow_grove.house.glaushouse.primary")
	var cloak_light := _color("hollow_grove.house.glaushouse.highlight")
	var face := _color("hollow_grove.house.flynt.highlight")
	var opal := _color("hollow_grove.house.flynt.authority")
	var boots := _color("hollow_grove.house.stonebend.dark")

	_draw_logical_rect(Rect2(actor_origin + Vector2(1, 14), Vector2(10, 2)), hair, viewport_size)
	_draw_logical_rect(Rect2(actor_origin + Vector2(2, 0), Vector2(8, 7)), outline, viewport_size)
	_draw_logical_rect(Rect2(actor_origin + Vector2(3, 0), Vector2(6, 3)), hair, viewport_size)
	_draw_logical_rect(Rect2(actor_origin + Vector2(1, 6), Vector2(10, 7)), outline, viewport_size)
	_draw_logical_rect(Rect2(actor_origin + Vector2(2, 7), Vector2(8, 5)), cloak, viewport_size)

	match player_facing:
		"South":
			_draw_logical_rect(Rect2(actor_origin + Vector2(3, 3), Vector2(6, 3)), face, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(4, 4), Vector2(1, 1)), outline, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(7, 4), Vector2(1, 1)), outline, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(5, 7), Vector2(2, 2)), opal, viewport_size)
		"North":
			_draw_logical_rect(Rect2(actor_origin + Vector2(3, 2), Vector2(6, 4)), hair, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(3, 7), Vector2(6, 4)), cloak_light, viewport_size)
		"East":
			_draw_logical_rect(Rect2(actor_origin + Vector2(6, 3), Vector2(3, 3)), face, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(8, 4), Vector2(1, 1)), outline, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(3, 8), Vector2(2, 3)), cloak_light, viewport_size)
		"West":
			_draw_logical_rect(Rect2(actor_origin + Vector2(3, 3), Vector2(3, 3)), face, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(3, 4), Vector2(1, 1)), outline, viewport_size)
			_draw_logical_rect(Rect2(actor_origin + Vector2(7, 8), Vector2(2, 3)), cloak_light, viewport_size)

	var step_phase := 0
	if animation_elapsed < STEP_DURATION_SECONDS:
		step_phase = int(animation_elapsed * 30.0) % 2
	_draw_logical_rect(Rect2(actor_origin + Vector2(3 - step_phase, 12), Vector2(3, 3)), boots, viewport_size)
	_draw_logical_rect(Rect2(actor_origin + Vector2(7 + step_phase, 12), Vector2(3, 3)), boots, viewport_size)
	_draw_logical_rect(Rect2(actor_origin + Vector2(2, 8), Vector2(1, 3)), cloak_dark, viewport_size)


func _draw_hud(viewport_size: Vector2) -> void:
	_draw_logical_rect(Rect2(2, 2, 48, 9), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(3, 3, 46, 7), _color("hollow_grove.universal.shadow.raised"), viewport_size)
	_draw_pixel_text(_map_label(), Vector2(5, 4), _color("hollow_grove.house.flynt.highlight"), viewport_size)
	var party_count := int(party.get("member_count", 0))
	var party_max := int(party.get("max_members", 6))
	_draw_logical_rect(Rect2(111, 2, 47, 9), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(112, 3, 45, 7), _color("hollow_grove.universal.shadow.raised"), viewport_size)
	_draw_pixel_text(
		"P PARTY %d/%d" % [party_count, party_max],
		Vector2(114, 4),
		_color("hollow_grove.house.flynt.highlight"),
		viewport_size
	)
	if not dialogue_pages.is_empty():
		_draw_dialogue_box(viewport_size)
	elif status_message != _map_label():
		_draw_message_box(status_message, viewport_size)


func _draw_party_menu(viewport_size: Vector2) -> void:
	var outline := _color("hollow_grove.universal.outline")
	var paper := _color("hollow_grove.house.flynt.highlight")
	var accent := _color("hollow_grove.house.flynt.authority")
	_draw_logical_rect(Rect2(5, 14, 150, 102), outline, viewport_size)
	_draw_logical_rect(Rect2(7, 16, 146, 98), paper, viewport_size)
	_draw_pixel_text("PARTY / HUEMAN PLUS FIVE", Vector2(10, 19), accent, viewport_size)
	var roster := _party_roster()
	for index in range(roster.size()):
		var member: Dictionary = _dict_or_empty(roster[index])
		var y := 31 + index * 10
		if index == party_roster_index:
			_draw_logical_rect(Rect2(9, y - 2, 140, 8), _color("hollow_grove.house.sandmanor.highlight"), viewport_size)
		var lead_mark := "*" if bool(member.get("is_lead", false)) else " "
		var availability := str(member.get("availability", "Ready")).to_upper().left(5)
		_draw_pixel_text(
			"%s%s %s" % [
				lead_mark,
				str(member.get("display_name", "MEMBER")).to_upper().left(18),
				availability,
			],
			Vector2(11, y),
			outline,
			viewport_size
		)
	if not roster.is_empty():
		var selected: Dictionary = _dict_or_empty(roster[party_roster_index % roster.size()])
		_draw_pixel_text(
			str(selected.get("role", "ROLE")).to_upper().left(34),
			Vector2(10, 94),
			accent,
			viewport_size
		)
		_draw_pixel_text(
			"ENTER LEAD / U ACTION / P CLOSE",
			Vector2(10, 105),
			outline,
			viewport_size
		)


func _draw_recruitment_menu(viewport_size: Vector2) -> void:
	var candidate := _current_recruitment_candidate()
	var outline := _color("hollow_grove.universal.outline")
	var paper := _color("hollow_grove.house.flynt.highlight")
	_draw_logical_rect(Rect2(7, 92, 146, 49), outline, viewport_size)
	_draw_logical_rect(Rect2(9, 94, 142, 45), paper, viewport_size)
	_draw_pixel_text(
		"ASK %s" % str(candidate.get("display_name", "CANDIDATE")).to_upper().left(24),
		Vector2(12, 98),
		_color("hollow_grove.house.flynt.authority"),
		viewport_size
	)
	_draw_pixel_text("1 SHARED WORK", Vector2(12, 109), outline, viewport_size)
	_draw_pixel_text("2 RECOVERY FIRST", Vector2(12, 118), outline, viewport_size)
	_draw_pixel_text("3 INDEPENDENT COMPANY", Vector2(12, 127), outline, viewport_size)


func _current_recruitment_candidate() -> Dictionary:
	for candidate_value in party.get("candidates", []):
		var candidate: Dictionary = _dict_or_empty(candidate_value)
		if bool(candidate.get("is_current_interaction", false)):
			return candidate
	return {}


func _map_label() -> String:
	if not extraction_view.is_empty():
		return str(extraction_view.get("display_name", "MINE")).to_upper().left(11)
	if not surface_view.is_empty():
		return str(surface_view.get("display_name", "AURA FIELD")).to_upper().left(11)
	if not route_view.is_empty():
		return str(route_view.get("display_name", "ROUTE")).to_upper().left(11)
	return "AURA RIDGE"


func _case_status() -> String:
	var active_living := _active_living_case()
	if not active_living.is_empty():
		var resolved = active_living.get("resolved_choice", null)
		if resolved != null:
			return "%s / LOTS %d" % [
				str(resolved).to_upper().left(20),
				living_world.get("custody", []).size(),
			]
		if active_living.get("supported_choice", null) != null:
			return "C: ASK DUTY OFFICER"
		var required := 3
		var observed: Array = active_living.get("evidence", [])
		if observed.size() >= required:
			return "1/2 LAWFUL  3 REFUSED"
		return "CASE E%d/%d / INSPECT SITES" % [observed.size(), required]
	var deep_phase := str(deep_pressure.get("phase", ""))
	if str(overworld.get("map_id", "")) == "boardwalk.return-vestibule":
		if deep_phase == "PersistentAftermath":
			var aftermath: Dictionary = _dict_or_empty(deep_pressure.get("aftermath"))
			return "AFTERMATH C%d S%d F%d B%d" % [
				int(aftermath.get("crew_care", 0)),
				int(aftermath.get("coast_recovery", 0)),
				int(aftermath.get("field_security", 0)),
				int(aftermath.get("basin_repair", 0)),
			]
		if deep_phase == "BoardwalkSettlement":
			if deep_pressure.get("supported_settlement", null) != null:
				return "C: AFFECTED ASSEMBLY COMMITS"
			return "1 SHARED 2 REPAIR 3 PRODUCE 4 REFUSE"
		if deep_phase == "GatherAffectedVoices":
			return "DEEP PRESSURE / %d VOICES MISSING" % int(
				deep_pressure.get("missing_required_statement_count", 0)
			)
	if str(overworld.get("map_id", "")) == "current-sea.deep-certification-landing":
		if stonebend_case.is_empty():
			return _map_label()
		var stonebend_committed = stonebend_case.get("committed_choice", null)
		if stonebend_committed != null:
			var stonebend_outcome: Dictionary = _dict_or_empty(stonebend_case.get("outcome"))
			return "%s / NO TITLE" % str(
				stonebend_outcome.get("authority_class", stonebend_committed)
			).to_upper()
		if stonebend_case.get("supported_choice", null) != null:
			return "C: ASK STONEBEND"
		if bool(stonebend_case.get("ready_for_support", false)):
			return "1 AFFIRM 2 PROVISIONAL 3 REVIEW"
		return "E%d/5 F%d/5  F: FACULTY" % [
			stonebend_case.get("evidence", []).size(),
			stonebend_case.get("faculties", []).size(),
		]
	if str(overworld.get("map_id", "")) != "boardwalk.return-vestibule":
		if deep_phase != "":
			return "DEEP PRESSURE / %s" % deep_phase.to_snake_case().replace("_", " ").to_upper().left(21)
		if not surface_view.is_empty():
			match str(surface_view.get("surface_id", "")):
				"aura-field":
					return "2 FARMS / 1 FIELD"
				"aura-beach":
					return "15 COASTAL WORKS / SANDMANOR"
				"aura-basin":
					return "16 BASIN WORKS / FLYNT"
				_:
					return _map_label()
		if not route_view.is_empty():
			return "%s / %s" % [
				str(route_view.get("geometry", "ROUTE")).to_upper(),
				str(route_view.get("dominant_verb", "TRAVEL")).to_upper(),
			]
		return _map_label()
	if boardwalk_case.is_empty():
		return _map_label()
	var committed = boardwalk_case.get("committed_choice", null)
	if committed != null:
		var outcome: Dictionary = _dict_or_empty(boardwalk_case.get("outcome"))
		var authority_class := str(outcome.get("authority_class", committed))
		var term_end = outcome.get("relationship_term_end", null)
		if term_end != null:
			return "%s / TERM %s" % [authority_class.to_upper(), str(term_end)]
		return "%s / RECOGNIZED" % authority_class.to_upper()
	var supported = boardwalk_case.get("supported_choice", null)
	if supported != null:
		return "C: ASK RETURNING GOON"
	if bool(boardwalk_case.get("ready_for_support", false)):
		return "1 PIMP 2 BOND 3 LIMITED 4 FREE"
	var evidence_count := boardwalk_case.get("evidence", []).size()
	var faculty_count := boardwalk_case.get("faculties", []).size()
	return "E%d/6 F%d/5  F: FACULTY" % [evidence_count, faculty_count]


func _active_living_case() -> Dictionary:
	if living_world.is_empty() or overworld.is_empty():
		return {}
	var case_name := ""
	match str(overworld.get("map_id", "")):
		"aura-field.working-land":
			case_name = "AuraFieldDroughtAllocation"
		"aura-beach.coastal-commons":
			case_name = "AuraBeachStormRescue"
		"aura-basin.collision-grounds":
			case_name = "AuraBasinInjuredBeing"
		"mnt-aura.high-mine":
			case_name = "MntAuraRoofFall"
		"highway-to-hell.deepworks":
			case_name = "HighwayToHellGasPocket"
		"riptide.current-recovery-rig":
			case_name = "RiptideWellBlowout"
		"current-sea.depth-production-rig":
			case_name = "CurrentSeaWellCertification"
	if case_name == "":
		return {}
	var cases: Dictionary = _dict_or_empty(living_world.get("cases"))
	return _dict_or_empty(cases.get(case_name))


func _living_case_choices(case_name: String) -> Array:
	match case_name:
		"AuraFieldDroughtAllocation":
			return ["EquitableRation", "ProtectSeedReserve", "MaximizeImmediateYield"]
		"AuraBeachStormRescue":
			return ["CloseAndShelter", "GuidedRescue", "KeepShoreOpen"]
		"AuraBasinInjuredBeing":
			return ["TransferToCare", "StabilizeInPlace", "SalvageTheSubject"]
		"MntAuraRoofFall":
			return ["ReinforceAndContinue", "WithdrawCrew", "BlastThroughFall"]
		"HighwayToHellGasPocket":
			return ["SealAndVent", "EvacuateAndFlood", "ContinueCutting"]
		"RiptideWellBlowout":
			return ["ShutInAndRetrieve", "RescueCrewFirst", "ContinueFlow"]
		"CurrentSeaWellCertification":
			return ["CertifyReducedRate", "SuspendForRepair", "BypassCertification"]
	return []


func _draw_dialogue_box(viewport_size: Vector2) -> void:
	_draw_logical_rect(Rect2(2, 116, 156, 26), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(4, 118, 152, 22), _color("hollow_grove.house.flynt.highlight"), viewport_size)
	_draw_pixel_text(dialogue_speaker.left(36), Vector2(7, 121), _color("hollow_grove.house.flynt.authority"), viewport_size)
	var page := str(dialogue_pages[dialogue_page_index]).left(36)
	_draw_pixel_text(page, Vector2(7, 132), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(149, 135, 5, 1), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(150, 136, 3, 1), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(151, 137, 1, 1), _color("hollow_grove.universal.outline"), viewport_size)


func _draw_message_box(message: String, viewport_size: Vector2) -> void:
	_draw_logical_rect(Rect2(2, 126, 156, 16), _color("hollow_grove.universal.outline"), viewport_size)
	_draw_logical_rect(Rect2(4, 128, 152, 12), _color("hollow_grove.house.flynt.highlight"), viewport_size)
	var compact := message.left(36)
	_draw_pixel_text(compact, Vector2(7, 132), _color("hollow_grove.universal.outline"), viewport_size)


func _draw_pixel_text(text: String, position: Vector2, color: Color, viewport_size: Vector2) -> void:
	var cursor_x := int(position.x)
	for character_index in range(text.length()):
		var character := text.substr(character_index, 1).to_upper()
		var pattern: Array = FONT_3X5.get(character, FONT_3X5["?"])
		for row_index in range(pattern.size()):
			var row := str(pattern[row_index])
			for column_index in range(row.length()):
				if row.substr(column_index, 1) == "1":
					_draw_logical_rect(
						Rect2(cursor_x + column_index, position.y + row_index, 1, 1),
						color,
						viewport_size
					)
		cursor_x += 4


func _draw_logical_rect(rect: Rect2, color: Color, viewport_size: Vector2) -> void:
	var scale := maxf(1.0, floorf(minf(viewport_size.x / LOGICAL_SIZE.x, viewport_size.y / LOGICAL_SIZE.y)))
	var screen_size := LOGICAL_SIZE * scale
	var screen_origin := ((viewport_size - screen_size) * 0.5).floor()
	draw_rect(Rect2(screen_origin + rect.position * scale, rect.size * scale), color)


func _load_visual_color_constitution() -> bool:
	var path := repo_root_path.path_join(VISUAL_COLOR_CONSTITUTION_RELATIVE_PATH)
	if not FileAccess.file_exists(path):
		return false
	var handle := FileAccess.open(path, FileAccess.READ)
	if handle == null:
		return false
	var parsed = JSON.parse_string(handle.get_as_text())
	if typeof(parsed) != TYPE_DICTIONARY:
		return false
	for record in parsed.get("colors", []):
		if typeof(record) != TYPE_DICTIONARY:
			return false
		var identity := str(record.get("semantic_identity", ""))
		var hexadecimal := str(record.get("hex", ""))
		if identity == "" or hexadecimal == "" or colors_by_semantic_identity.has(identity):
			return false
		colors_by_semantic_identity[identity] = Color.from_string(hexadecimal, Color.TRANSPARENT)
	return not colors_by_semantic_identity.is_empty()


func _color(semantic_identity: String) -> Color:
	assert(colors_by_semantic_identity.has(semantic_identity), "Unknown constitutional color identity: %s" % semantic_identity)
	return colors_by_semantic_identity[semantic_identity]


func _resolve_repo_root() -> String:
	var env_root := OS.get_environment("HOLLOW_GROVE_ROOT")
	if env_root != "":
		return env_root
	return ProjectSettings.globalize_path("res://").trim_suffix("/").get_base_dir()


func _next_request_id(prefix: String) -> String:
	request_serial += 1
	return "request.%s.%s" % [prefix, request_serial]


func _dict_or_empty(value) -> Dictionary:
	return value if typeof(value) == TYPE_DICTIONARY else {}
