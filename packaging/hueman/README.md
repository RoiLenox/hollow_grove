# Hueman

Hueman is the independent Godot presentation client for Hollow Grove. It owns
rendering, input, accessibility semantics, and presentation assets. It contains
no authoritative gameplay service and writes no Hollow Grove saves.

Run `hueman` to connect to a separately running Hollow Grove Core service. The
client accepts `HOLLOW_GROVE_HOST` and `HOLLOW_GROVE_PORT`; the host remains
restricted to loopback. For the usual one-command experience, install both
packages and run `hollow-grove-with-hueman`.
