# Hollow Grove Core

Hollow Grove Core owns the authoritative gameplay service, protocol, durable
saves, and world state. It contains no Hueman presentation assets and has no
Godot dependency.

Run `hollow-grove-service` to expose the versioned loopback protocol directly.
Run `hollow-grove-with-hueman` for the seamless experience: it starts the core,
launches the separately installed `hueman` client, and stops the service when
the client exits. The compatibility command `hollow-grove-hueman` may point to
that integration launcher, but it is not a combined package.

The integration launcher uses an explicit `HUEMAN_BIN` first, then the
user-installed Hueman client beneath `$XDG_DATA_HOME/hueman/current`, and then
`hueman` from `PATH`. This keeps an older command with the same name from
shadowing the installed presentation client.

Durable data lives beneath `$XDG_DATA_HOME/hollow-grove`; machine-local state
lives beneath `$XDG_STATE_HOME/hollow-grove`.
# Installed world and device handoff

Use `hollow-grove-device status` for read-only readiness, `import-world PATH`
to migrate an existing catalog into an empty installed world, and
`configure-peer SSH_ALIAS` followed by `handoff` to move default-world progress
between matching installed Core/Hueman pairs. Handoff parks the sender, refuses
divergent state, verifies payloads and retains recovery copies. See
`docs/TWO_DEVICE_OPERATION_V1.md` in the source inventory for the complete flow.
The Core launcher requires Python and util-linux; SSH is needed for handoff.
