extends SceneTree

const CLIENT := preload("res://scripts/runtime_client.gd")
const SESSION := "session.transport-smoke"
var responses: Array[Dictionary] = []
var errors: Array[String] = []


func _init() -> void:
	if not _fragmented_unicode() or not _multiple_frames() or not _invalid_frame() \
			or not _size_boundaries() or not _disconnect_discards_tail():
		quit(1)
		return
	print("Hollow Grove runtime client transport smoke passed")
	quit()


func _client(request_ids: Array):
	responses.clear()
	errors.clear()
	var client = CLIENT.new(SESSION)
	for request_id in request_ids:
		client._pending_requests[request_id] = true
	client.response_received.connect(func(response: Dictionary): responses.append(response))
	client.protocol_error.connect(func(message: String): errors.append(message))
	return client


func _packet(request_id: String, text: String, revision := 1) -> PackedByteArray:
	return (JSON.stringify({
		"protocol_version": 1, "session_id": SESSION, "request_id": request_id,
		"revision": revision, "status": "Completed", "events": [],
		"view": {"text": text},
	}) + "\n").to_utf8_buffer()


func _check(condition: bool, message: String) -> bool:
	if not condition:
		push_error(message)
	return condition


func _fragmented_unicode() -> bool:
	var request_id := "request.transport.unicode"
	var client = _client([request_id])
	var text := "Glaüshouse, Roï, Rosaliña — 🜁"
	for byte in _packet(request_id, text):
		client._append_response_bytes(PackedByteArray([byte]))
	return _check(errors.is_empty() and responses.size() == 1,
		"fragmented UTF-8 did not produce exactly one valid response") \
		and _check(str(responses[0]["view"]["text"]) == text,
			"fragmented UTF-8 changed authored lore text") \
		and _check(client.revision == 1 and client._pending_requests.is_empty(),
			"fragmented response did not finish its pending request")


func _multiple_frames() -> bool:
	var first := "request.transport.first"
	var second := "request.transport.second"
	var client = _client([first, second])
	var packet := _packet(second, "Roï", 2)
	client._append_response_bytes("\n \r\n".to_utf8_buffer()
		+ _packet(first, "first") + packet.slice(0, 13))
	if not _check(responses.size() == 1 and client._pending_requests.has(second),
			"batched response lost or prematurely accepted its incomplete tail"):
		return false
	client._append_response_bytes(packet.slice(13))
	return _check(errors.is_empty() and responses.size() == 2,
		"batched responses did not preserve frame boundaries") \
		and _check(responses[0]["revision"] == 1 and responses[1]["revision"] == 2,
			"batched responses changed order") \
		and _check(client._receive_buffer.is_empty(), "complete frames left buffered bytes")


func _invalid_frame() -> bool:
	var request_id := "request.transport.invalid"
	var client = _client([request_id])
	client._append_response_bytes("[]\n".to_utf8_buffer() + _packet(request_id, "valid"))
	return _check(errors.size() == 1 and responses.size() == 1,
		"invalid JSON envelope prevented the following valid response") \
		and _check(client.revision == 1 and client._pending_requests.is_empty(),
			"valid response after an invalid envelope did not complete")


func _size_boundaries() -> bool:
	var request_id := "request.transport.limit"
	var client = _client([request_id])
	var overhead := _packet(request_id, "").size() - 1
	var packet := _packet(request_id, "X".repeat(CLIENT.MAX_RESPONSE_BYTES - overhead))
	client._append_response_bytes(packet.slice(0, CLIENT.MAX_RESPONSE_BYTES))
	if not _check(errors.is_empty() and responses.is_empty(),
			"exact-limit incomplete response was rejected or accepted early"):
		return false
	client._append_response_bytes(packet.slice(CLIENT.MAX_RESPONSE_BYTES))
	if not _check(errors.is_empty() and responses.size() == 1,
			"exact-limit complete response was rejected"):
		return false
	client = _client([request_id])
	client._append_response_bytes(" ".repeat(CLIENT.MAX_RESPONSE_BYTES + 1).to_utf8_buffer())
	if not _check(errors.size() == 1 and responses.is_empty()
			and client._receive_buffer.is_empty() and client._pending_requests.is_empty(),
			"oversized incomplete response did not discard its buffer and pending requests"):
		return false
	client = _client([request_id])
	client._append_response_bytes(_packet(request_id,
		"X".repeat(CLIENT.MAX_RESPONSE_BYTES - overhead + 1)))
	return _check(errors.size() == 1 and responses.is_empty() and client.revision == 0,
		"oversized complete response was accepted")


func _disconnect_discards_tail() -> bool:
	var request_id := "request.transport.disconnect"
	var client = _client([request_id])
	var packet := _packet(request_id, "Rosaliña")
	client._append_response_bytes(packet.slice(0, 15))
	client.disconnect_from_runtime()
	if not _check(client._receive_buffer.is_empty() and client._pending_requests.is_empty(),
			"disconnect retained partial response state"):
		return false
	client._pending_requests[request_id] = true
	client._append_response_bytes(packet)
	return _check(errors.is_empty() and responses.size() == 1,
		"a fresh response was contaminated by disconnected bytes")
