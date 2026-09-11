# Hollow Grove / Hueman Seam V1

Hollow Grove and Hueman are independent products joined by one narrow,
versioned local protocol.

## Ownership

Hollow Grove Core owns the Rust authoritative service, intent validation,
revision ordering, gameplay decisions, durable saves, and world state. Its XDG
namespace is `hollow-grove`. Hueman owns the Godot presentation client,
rendering, controls, accessibility semantics, and visual assets. Its installed
XDG namespace is `hueman`; it does not mutate authoritative state directly.

## Protocol boundary

The seam is newline-delimited JSON over TCP protocol version 1. The service
binds only to loopback. Every request carries `protocol_version`, `session_id`,
`request_id`, `expected_revision`, and a typed `intent`. Every accepted response
carries the matching envelope plus an authoritative revision, events, and an
immutable projection. Messages are bounded to 64 KiB.

Hueman discovers the endpoint through `HOLLOW_GROVE_HOST` and
`HOLLOW_GROVE_PORT`, defaulting to `127.0.0.1:47819`. Non-loopback hosts and
invalid ports are rejected by both the integration launcher and client.

## Seamless composition

`hollow-grove-with-hueman` is an integration command owned by Hollow Grove
Core. It starts the authoritative service, waits until the listener is ready,
passes the endpoint to the independent `hueman` executable, and owns service
cleanup. Neither release archive embeds the other. `hollow-grove-hueman` is a
compatibility alias for this integration command, not a package boundary.
