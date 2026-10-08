extends SceneTree

const CLIENT := preload("res://scripts/runtime_client.gd")
const SESSION := "session.transport-benchmark"
const REQUEST := "request.transport-benchmark.sync"
const SAMPLES := 100
const WARMUP := 10
var responses: Array = []

func _init() -> void:
	var text := "Glaüshouse, Roï, Rosaliña"
	var unicode_packet := _packet(text)
	var client = _client()
	for byte in unicode_packet:
		_feed(client, PackedByteArray([byte]))
	var unicode_ok := responses.size() == 1 and str(responses[0]["view"]["text"]) == text
	var packet := _packet("X".repeat(32000))
	var chunks: Array[PackedByteArray] = []
	for offset in range(0, packet.size(), 256):
		chunks.append(packet.slice(offset, mini(offset + 256, packet.size())))
	var durations: Array[int] = []
	for sample in range(WARMUP + SAMPLES):
		client = _client()
		var started := Time.get_ticks_usec()
		for chunk in chunks:
			_feed(client, chunk)
		var elapsed := Time.get_ticks_usec() - started
		assert(responses.size() == 1)
		assert(str(responses[0]["view"]["text"]).length() == 32000)
		if sample >= WARMUP:
			durations.append(elapsed)
	durations.sort()
	print(JSON.stringify({"samples": SAMPLES, "warmup": WARMUP, "packet_bytes": packet.size(), "chunk_bytes": 256, "median_us": durations[SAMPLES / 2], "p95_us": durations[95], "max_us": durations[-1], "unicode_ok": unicode_ok, "byte_buffer": client.has_method("_append_response_bytes")}))
	quit()

func _packet(text: String) -> PackedByteArray:
	return (JSON.stringify({"protocol_version": 1, "session_id": SESSION, "request_id": REQUEST, "revision": 1, "status": "Completed", "events": [], "view": {"text": text}}) + "\n").to_utf8_buffer()

func _client():
	responses.clear()
	var client = CLIENT.new(SESSION)
	client._pending_requests[REQUEST] = true
	client.response_received.connect(func(response): responses.append(response))
	return client

func _feed(client, chunk: PackedByteArray) -> void:
	if client.has_method("_append_response_bytes"):
		client.call("_append_response_bytes", chunk)
	else:
		client._receive_buffer += chunk.get_string_from_utf8()
		client._consume_complete_lines()
		if client._receive_buffer.to_utf8_buffer().size() > client.MAX_RESPONSE_BYTES:
			client._fail_oversized_response()
