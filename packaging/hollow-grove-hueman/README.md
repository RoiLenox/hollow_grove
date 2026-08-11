# Hollow Grove Hueman Package

This package contains the authoritative Rust gameplay service, the exported
Godot PCK, and an XDG-safe launcher. It does not require or write to a source
checkout. Runtime saves live beneath `$XDG_DATA_HOME/hollow-grove`, while
machine-local state lives beneath `$XDG_STATE_HOME/hollow-grove`.

The package uses a system Godot 4.7+ runtime. Run `hollow-grove-hueman`; the
launcher starts the loopback-only service, opens the PCK, and always stops the
service when Godot exits.

Build and validate both portable editions from the repository root:

```bash
./build-portable-release.sh
```
