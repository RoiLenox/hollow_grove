class_name HollowGroveRuntimeClient
extends RefCounted

## Transport-only client for the authoritative Rust gameplay service.
##
## This class sends typed intents, validates response envelopes, and exposes
## immutable response dictionaries. It does not decide movement, party state,
## Bonds, actions, progression, or Synthesis.

signal response_received(response: Dictionary)
signal protocol_error(message: String)

const PROTOCOL_VERSION := 1
const DEFAULT_HOST := "127.0.0.1"
const DEFAULT_PORT := 47819
const HOST_ENVIRONMENT_VARIABLE := "HOLLOW_GROVE_HOST"
const PORT_ENVIRONMENT_VARIABLE := "HOLLOW_GROVE_PORT"
const MAX_RESPONSE_BYTES := 64 * 1024
const MAX_PENDING_REQUESTS := 64

var revision: int = 0
var _session_id: String
var _peer := StreamPeerTCP.new()
var _receive_buffer := ""
var _pending_requests := {}


func _init(session_id: String) -> void:
	_session_id = session_id


func connect_to_runtime(host := DEFAULT_HOST, port := DEFAULT_PORT) -> Error:
	if host not in ["127.0.0.1", "::1", "localhost"]:
		return ERR_INVALID_PARAMETER
	if port < 1 or port > 65535:
		return ERR_INVALID_PARAMETER
	return _peer.connect_to_host(host, port)


func connect_to_configured_runtime() -> Error:
	var host := OS.get_environment(HOST_ENVIRONMENT_VARIABLE).strip_edges()
	if host == "":
		host = DEFAULT_HOST
	var port := DEFAULT_PORT
	var configured_port := OS.get_environment(PORT_ENVIRONMENT_VARIABLE).strip_edges()
	if configured_port != "":
		if not configured_port.is_valid_int():
			return ERR_INVALID_PARAMETER
		port = configured_port.to_int()
	return connect_to_runtime(host, port)


func disconnect_from_runtime() -> void:
	_peer.disconnect_from_host()
	_receive_buffer = ""
	_pending_requests.clear()


func is_runtime_connected() -> bool:
	return _peer.get_status() == StreamPeerTCP.STATUS_CONNECTED


func send_intent(request_id: String, intent_type: String, payload := {}) -> Error:
	if not is_runtime_connected():
		return ERR_UNCONFIGURED
	if (
		request_id == ""
		or intent_type == ""
		or _pending_requests.has(request_id)
		or _pending_requests.size() >= MAX_PENDING_REQUESTS
	):
		return ERR_INVALID_PARAMETER
	var intent: Dictionary = payload.duplicate(true)
	intent["type"] = intent_type
	var request := {
		"protocol_version": PROTOCOL_VERSION,
		"session_id": _session_id,
		"request_id": request_id,
		"expected_revision": revision,
		"intent": intent,
	}
	var encoded := (JSON.stringify(request) + "\n").to_utf8_buffer()
	var result := _peer.put_data(encoded)
	if result == OK:
		_pending_requests[request_id] = true
	return result


func poll() -> void:
	var poll_result := _peer.poll()
	if poll_result != OK:
		protocol_error.emit("runtime transport poll failed: %s" % error_string(poll_result))
		return
	if not is_runtime_connected():
		return
	var available := _peer.get_available_bytes()
	if available <= 0:
		return
	var read_result := _peer.get_data(available)
	if read_result[0] != OK:
		protocol_error.emit("runtime transport read failed: %s" % error_string(read_result[0]))
		return
	_receive_buffer += read_result[1].get_string_from_utf8()
	_consume_complete_lines()
	if _receive_buffer.to_utf8_buffer().size() > MAX_RESPONSE_BYTES:
		_fail_oversized_response()


func _consume_complete_lines() -> void:
	while true:
		var newline := _receive_buffer.find("\n")
		if newline < 0:
			return
		var line := _receive_buffer.substr(0, newline).strip_edges()
		_receive_buffer = _receive_buffer.substr(newline + 1)
		if line == "":
			continue
		if line.to_utf8_buffer().size() > MAX_RESPONSE_BYTES:
			_fail_oversized_response()
			return
		var parsed = JSON.parse_string(line)
		if typeof(parsed) != TYPE_DICTIONARY:
			protocol_error.emit("runtime response was not a JSON object")
			continue
		_accept_response(parsed)


func _fail_oversized_response() -> void:
	_peer.disconnect_from_host()
	_receive_buffer = ""
	_pending_requests.clear()
	protocol_error.emit("runtime response exceeded the %d byte transport limit" % MAX_RESPONSE_BYTES)


func _accept_response(response: Dictionary) -> void:
	if int(response.get("protocol_version", -1)) != PROTOCOL_VERSION:
		protocol_error.emit("runtime response used an unsupported protocol version")
		return
	if str(response.get("session_id", "")) != _session_id:
		protocol_error.emit("runtime response session did not match this client")
		return
	var request_id := str(response.get("request_id", ""))
	if request_id != "unparseable" and not _pending_requests.has(request_id):
		protocol_error.emit("runtime response did not match a pending request")
		return
	if not response.has("status") or not response.has("revision") or not response.has("events"):
		protocol_error.emit("runtime response omitted required envelope fields")
		return
	_pending_requests.erase(request_id)
	revision = int(response["revision"])
	response_received.emit(response.duplicate(true))
