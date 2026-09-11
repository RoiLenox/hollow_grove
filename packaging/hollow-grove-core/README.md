# Hollow Grove Core

Hollow Grove Core owns the authoritative gameplay service, protocol, durable
saves, and world state. It contains no Hueman presentation assets and has no
Godot dependency.

Run `hollow-grove-service` to expose the versioned loopback protocol directly.
Run `hollow-grove-with-hueman` for the seamless experience: it starts the core,
launches the separately installed `hueman` client, and stops the service when
the client exits. The compatibility command `hollow-grove-hueman` may point to
that integration launcher, but it is not a combined package.

Durable data lives beneath `$XDG_DATA_HOME/hollow-grove`; machine-local state
lives beneath `$XDG_STATE_HOME/hollow-grove`.
