# Hollow Grove General Edition

This directory is the shared-data root for the portable Niri desktop profile.
The base package deliberately contains no personal wallpaper, browser profile,
mail state, credentials, hardware output file, or repository checkout.

## Installed interfaces

- `/usr/bin/hollow-launch` checks optional commands and reports clear failures.
- `/usr/bin/hollow-hueman` is the stable public command for the Hueman concept.
- `/usr/lib/hollow-grove/bin/` contains private desktop helpers.
- `/etc/hollow-grove/` contains distribution-owned defaults and the application
  inventory.
- `/etc/skel/.config/` contains clean defaults copied to newly created users.

The separately packaged Hueman application may install an executable launcher
at `/usr/lib/hollow-grove/hueman/launch` and a pair-state adapter at
`/usr/lib/hollow-grove/hueman/pair`. Its immutable assets may instead live
under `/usr/share/hollow-grove/hueman/`. Runtime state belongs under the
installed user's XDG state directory.

Machine-specific output configuration is intentionally outside this package.
