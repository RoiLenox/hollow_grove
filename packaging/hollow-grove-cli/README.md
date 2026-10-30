# Hollow Grove command line package

Owns `/usr/bin/hollow-grove`, the command suite under
`/usr/lib/hollow-grove/cli`, and the authored source/proof inventory under
`/usr/share/hollow-grove/source`. Core owns the game service and General owns
the Niri broker. The source inventory contains no developer build caches or
personal world state. Architecture inspection uses it by default from any
working directory; `--source-root PATH` explicitly selects another inventory.

This recipe incorporates the working October 29, 2026 occamsrazer CLI package.
Its build and source identity now participate in the same five-package release
as Core, General, Desktop and Hueman. Developer audits remain behind Cargo's
`dev-tools` feature; the command package includes their existing installed
aliases for compatibility. A package installation is not world-state adoption.

Retire stale `~/.cargo/bin` and `~/.local` launchers only after a checksummed
backup and successful installed-pair validation. Never replace personal Niri,
GPU or monitor policy with packaged skeleton defaults.
