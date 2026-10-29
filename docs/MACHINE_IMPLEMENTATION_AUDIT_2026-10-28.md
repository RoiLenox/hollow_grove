# Hollow Grove on noahsarch — implementation and deployment audit

Audited 2026-10-28. Source: `/home/warren/Projects/active/hollow_grove`,
HEAD `d51f506d3a4d9f00381d5e4d44471b316f7a5c1c`, with substantial existing local
changes, including the new Boardwalk implementation.

The machine has a working development environment and a healthy Niri integration.
It does **not** yet have one consistently deployed current Hollow Grove release.
Several planned world systems also remain explicitly unimplemented. Passing
source tests, completing the local installation, and finishing the intended
product are three separate acceptance targets. No percentage is assigned to them.

This audit built and tested source and used disposable gameplay state. It did
not install packages, change Niri/systemd configuration, migrate live world
state, alter CPU/GPU policy, or restart the desktop. The only authored repository
file added by this audit is this report. Earlier Boardwalk changes are recorded
separately in `BOARDWALK_GOVERNING_DOCTRINE_V0_1_IMPLEMENTATION.md`.

## What is verified

| Layer | Observed result | Limits |
| --- | --- | --- |
| Linux/hardware | CachyOS BORE `7.2.10-1-cachyos-bore`; Ryzen 7 9800X3D; Radeon RX 7900 XTX; about 60 GiB usable RAM with about 54 GiB available at inspection | No controlled comparison against another kernel or sustained graphics benchmark |
| CPU policy | AMD P-State active, `powersave` governor, `balance_performance` preference | This is the existing adaptive policy; the governor name alone does not establish a performance problem |
| Niri | Version 26.04; configuration validates; Niri and `hollow-niri` enabled/active with zero recorded restarts in the inspected session | Login, suspend/resume and every interactive workflow were not exercised |
| GPU selection | Niri startup identifies the discrete GPU at PCI `03:00.0` as its primary render device | Does not establish the rendering device or presentation timing of every application |
| Niri broker | Doctor passes; sampled version/windows round trips of 108 and 88 microseconds | Single observations, not a latency distribution |
| Installed packages | Core `0.1.0-2`, General `0.1.0-11`, Desktop `0.1.0-1`, Hueman `0.1.0-2`; host package-integrity checks pass | Installed code is older than the current source |
| Source tests | `cargo test --workspace --all-features --locked --offline`: 1,829 passed, zero failed/ignored, 36 result blocks | Existing incremental build environment; not a clean-chroot release qualification |
| Godot | `./verify-godot.sh` passes with Godot 4.7.2; previous Boardwalk rendering and Godot-to-Rust save/restart/load checks passed in this session | Current source checks do not validate the old installed game pack |
| Packaging/audit tooling | Package preparation tests: 4 passed; distribution tests: 7 passed; machine audit tests: 3 passed | Does not substitute for installed-package upgrade and rollback testing |
| Rust release | Optimized CLI and game service build successfully with locked, offline dependencies | Built into `target/release`; not installed |

Both meanings of “kernel” were inspected. The dependency-free recursion kernel
and semantic geometry/composition kernel are covered by the workspace tests and
the earlier Boardwalk inspection. Linux is the host environment, not the owner
of Hollow Grove jurisdiction or faction authority. No OS kernel patch is needed
to implement the Boardwalk.

## Deployment inconsistencies to fix first

### Multiple versions are selected by different entry points

| Entry point | Current resolution | Evidence |
| --- | --- | --- |
| Terminal `hollow-grove` | `~/.cargo/bin/hollow-grove`, July 29 binary | Rejects the current `architecture` command |
| Another local CLI | `~/.local/lib/hollow-grove/bin/hollow-grove` | Also rejects `architecture` |
| Niri/systemd desktop launch | Session PATH `/usr/local/bin:/usr/bin`; system `hollow-grove-with-hueman` and `hueman` | Core/Hueman manifests identify commit `8bdc91d8…`, dirty development candidate |
| User portable release | `~/.local/share/hollow-grove/current-core` and `~/.local/share/hueman/current` | Manifests identify commit `8c13a270ad3c4a24f7960d20827962a2fdc09042` |
| Current source release | `target/release/hollow_grove_game_service` | Contains the new Boardwalk maps and neutral territorial projection |

A disposable protocol probe entered the existing Boardwalk vestibule and tried
to traverse to `boardwalk.upper-span` against each service. Both installed
services rejected the new map as unknown. The current debug and release services
completed traversal and returned the territory projection with a passing kernel
audit. Thus **the new Boardwalk is implemented in source but not deployed to the
installed desktop yet**.

The general `kernel_audit.passed` flag on an older binary is not proof that a
new feature exists: each old binary passed its own older audit scope.

The four current package recipes do not install the general `hollow-grove` CLI.
`install-local-release.sh` installs it under `~/.local/lib/hollow-grove/bin`, but
does not replace or expose it ahead of the stale Cargo-installed command. Merely
rebuilding the four packages will therefore not fix terminal command selection.

Eleven active personal helper wrappers also reference this source checkout by
absolute path, including `hollow-ai`, `hollow-time`, `hollow-query`, and
`hollow-media`. Their packaged counterparts must become the stable desktop entry
points if the installation is to work independently of the development checkout.

### Desktop PATH and keyboard behavior

Static inspection of 119 direct launch/require checks found two dependencies
missing from Niri's actual PATH: `spicetify` and `spt`. Both exist in
`~/.local/bin`. The affected music bindings are at lines 411–412 of the inspected
personal Niri configuration. Shell availability does not imply desktop
availability. Nested application behavior has not been comprehensively tested.

The source keyboard profile verifier passes. The personal profile is syntactically
valid but fails the project's “immediate animation” requirement. Decide whether
that requirement should apply to the personal desktop or be documented as an
intentional preference. Do not replace the personal configuration with the
packaged `/etc/skel` file merely to satisfy this check.

### Installed world state differs from source state

The installed launcher's default world root is
`~/.local/share/hollow-grove/world`. Its `artifacts/institutional_state.txt` was
absent at inspection; the source checkout has that artifact. Neither inspected
default save directory contained `.hgplay.json` saves. These observations cover
those locations only, not arbitrary custom save roots.

Select the intended authoritative world state, back it up, validate it, then
migrate it deliberately. Do not silently copy vacant state over a developed
world, grant officeholders, or treat package installation as world adoption.
New saves containing the Boardwalk span IDs require the new service; an old
binary cannot replay those IDs. Rollback must retain the corresponding pre-upgrade
data snapshot as well as the old binaries.

## Niri and graphics findings

The AORUS FO32U2P on DP-2 is configured at 3840×2160, 120 Hz, scale 1.25. A
240 Hz mode is available, but the existing configuration intentionally chooses
120 Hz for link stability. Retain this working baseline until a controlled
comparison demonstrates stable presentation at the higher rate.

The output has `variable-refresh-rate on-demand=true`, but the active configuration
contains no matching window rule enabling variable refresh rate. Niri reports
VRR supported but inactive. If VRR is desired for Hueman or games, observe each
actual application ID with `niri msg --json windows`, add a narrow matching rule,
validate it, and check live output state while that application is focused.
Niri's [official window-rule documentation](https://niri-wm.github.io/niri/Configuration:-Window-Rules.html#variable-refresh-rate)
describes this opt-in behavior. Do not guess a packaged Godot application's ID or
enable a broad rule without checking its effects.

The current session journal contains six `GL_INVALID_VALUE` texture-size errors
and a startup warning that `XDG_SESSION_ID` is unset or invalid, preventing
logind LockedHint updates. Neither caused a recorded Niri restart. Their causes
are not established. Reproduce the graphics error with the responsible client,
and verify the real login/session registration and lock behavior before changing
drivers or session configuration. Setting an invented session ID is not a fix.
Niri's [systemd integration documentation](https://niri-wm.github.io/niri/Example-systemd-Setup.html)
is the reference for session-managed services.

PipeWire, WirePlumber, the relevant desktop portals and a polkit agent are
running. The Niri-specific portal selection exists. That is evidence of the
supporting processes, not proof of successful screen sharing, file dialogs,
audio-device switching, or lock/suspend recovery; those remain interactive checks.

## Measured performance and optimization policy

The release profile already enables thin LTO, one codegen unit, and symbol
stripping. No extra dependency or speculative compiler/OS tuning was introduced.

The existing benchmark ran sequentially, after release compilation finished,
with disposable state, 50 warmup requests and 1,000 timed active-view requests:

| Service | Median | p95 | Maximum | Response size |
| --- | ---: | ---: | ---: | ---: |
| Current release | 84.754 µs | 122.830 µs | 335.219 µs | 23,420–23,422 bytes |
| Existing local portable release | 74.084 µs | 104.616 µs | 274.445 µs | 22,630–22,632 bytes |

This measures stdio request write/flush through response-line receipt. JSON
decoding, TCP transport, drawing, compositor scheduling, and display latency
are outside the timing. The builds also differ in behavior and response size,
so this is not an isolated attribution of overhead to the Boardwalk or a
statistically established regression. Both responses fit the existing 65,536-byte
Godot transport limit for this scenario. The older system service does not
provide a field expected by the current benchmark harness, so that harness could
not produce a valid system-service comparison report.

There is no measured case here for CPU pinning, fixed clocks, scheduler patches,
global sysctl changes, or switching kernels. Follow `docs/LINUX_KERNEL_POLICY.md`:
compare at least three controlled runs per candidate using the same representative
workload and verify desktop regressions as well as speed.

After the correct client/service pair is deployed, capture representative Godot
CPU/GPU frame times, p95/p99 frame times, memory/VRAM use, cold startup and
save/load latency. At 120 Hz the presentation interval is about 8.33 ms; at
240 Hz it is about 4.17 ms. These are display intervals, not measured game frame
times. Check the actual packaged renderer/display backend before comparing
Vulkan and compatibility rendering. The earlier compatibility-renderer Boardwalk
screenshots establish visual correctness only. Choose settings from repeatable
results and stability, not maximum utilization or an uncapped FPS number.

## What installation alone cannot finish

The backend contract verifier passes its source/status consistency checks but
explicitly reports:

- 10 headless backend route bindings with 10 corresponding deferred gameplay
  bindings. This does not mean the existing map has no traversable routes.
- 17 open and 4 partially adopted decision families.
- Unified Synthesis execution, Synblock crafting, Synbalm healing execution,
  and grade advancement are not implemented.

Bounded Winged Gargoyle, Sympiote Foundation and Survey Frame capabilities do
exist; the broader Synthesis system is not finished. The current protocol
explicitly marks general Synthesis execution unavailable. Remote Hueman
collaboration also remains planned. Neither an installer nor a Niri binding can
create these domain behaviors.

The distribution is an Arch-based foundation, not a finished standalone OS
release. The distribution audit reports no signed-release readiness; clean-chroot,
signing, installer, upgrade/recovery and two-device collaboration gates lack
completion evidence. Those public-distribution gates exceed what is required to
make this one workstation run the current supported local features correctly.

## Recommended implementation sequence

1. **Choose and record a reviewed source snapshot.** Preserve the substantial
   uncommitted House, governance, keyboard and Boardwalk work. Review it before a
   release commit; do not use a blanket commit to manufacture a clean worktree.
   Record source, toolchain and artifact hashes, plus backups of installed package
   versions, personal configuration, and world/save state.
2. **Close the known source verification failures.** Fix the formatting and
   `clippy::print_literal` failure at `src/bin/warden_registrar_v03.rs:206`.
   Repair `tools/verify-active-geography.py` occurrence scanning so build caches
   are excluded and its reviewed source inventory reflects new documentation.
   Its current occurrence-drift failure is not itself evidence of broken route
   geometry. Re-run the checks below after those changes.
3. **Build one coherent package set.** Use the existing source-bundle/package
   workflow for Core, Hueman, General and Desktop from that same snapshot. Add
   an explicit package ownership decision for the current general CLI and any
   required developer tools. Ensure helper commands use installed resources,
   not `/home/warren/Projects/active/hollow_grove`. Export the current Godot pack
   together with the service that understands its views. Do not deploy only one
   half of that pair.
4. **Make command resolution deliberate.** Prefer package-owned production
   commands under `/usr/bin`. Preserve obsolete Cargo/local copies in a backup,
   then remove their active precedence or rename them as explicit development
   commands. Update personal wrappers and desktop entries to the stable paths.
   Resolve the two music dependencies through explicit paths or a deliberate
   graphical-session PATH; test both shell and Niri launch contexts. A PATH edit
   in `.bashrc` alone will not fix an already running Niri process.
5. **Deploy and migrate with rollback.** Stage and verify the exact built
   artifacts, install the matched packages, and migrate only the validated
   intended world/save data. Keep the existing personal monitor, GPU and keyboard
   policy. Check the packaged desktop entry, both Boardwalk levels, ordinary
   public traversal, rivalry labels and neutral jurisdiction. Save, exit,
   restart and load through the installed launcher, not `cargo run`.
6. **Verify the real session.** Validate the edited Niri configuration before
   applying it. Check the broker, relevant shortcuts, Hueman focus/scaling/input,
   audio, file chooser, screen sharing and lock/suspend behavior. Complete a
   fresh-login check when open work can be closed safely. Trace the journal
   warnings rather than assuming running services prove every workflow works.
7. **Optimize measured bottlenecks.** Profile the deployed renderer and gameplay
   at the existing 120 Hz baseline. Add application-specific VRR if desired,
   then compare refresh/renderer changes one at a time. Keep the current kernel
   and adaptive power policy unless controlled results justify changing them.
8. **Complete product features separately.** Resolve the open canon decisions
   and implement their reducers, persistence/replay contracts, projections and
   Godot interactions in bounded milestones. Do not claim those milestones are
   satisfied by packaging the existing code.

Useful source gates, run from the repository root after source cleanup:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --all-features --locked --offline
python3 tools/verify-backend-contracts.py
python3 tools/verify-active-geography.py
./verify-godot.sh
./verify-playable-lifecycle.sh
packaging/hollow-grove-general/verify-keyboard-grove.sh
python3 tools/test_prepare_arch_packages.py
python3 tools/test_hg_distribution.py
python3 tools/test_hg_machine_audit.py
```

For a reviewed clean source snapshot, generate a new, unused output directory:

```sh
python3 tools/prepare-arch-packages.py --output dist/arch-packages
```

Then run each generated package directory's `./build.sh`, using
`--config /home/warren/Projects/active/hollow_grove/distro/makepkg-x86_64.conf`
for the repository's portable baseline. `--allow-dirty` preparation is available
only for an explicitly labelled development snapshot. Neither option proves
release readiness. Complete package checks before installing; public release
also requires the clean-chroot and signing workflow in `distro/README.md`.

## Secondary machine maintenance findings

The read-only machine audit reports 7 passes, 5 attention items and no blockers.
Besides the dirty source, attention items are SSH listening on wildcard
addresses with no recognized firewall service active, broader boot VFAT
permissions than the declared distribution target, and no visible periodic
Btrfs scrub timer. Snapper timeline/cleanup timers are active. Review these
against the intended remote-access and recovery policy; the audit is not proof
of Internet exposure or absence of all firewall rules. Avoid a firewall change
that disconnects an intended management path. These are maintenance items, not
reasons to change Hollow Grove's faction or kernel architecture.

## Evidence and completion boundary

Raw audit/test/probe/performance records are under `/tmp/hg-audit-20261028`;
the machine enrollment report is `/tmp/hg-machine-audit-20261028.json`.
Earlier Boardwalk fixtures/screenshots and lifecycle evidence are under
`/tmp/boardwalk-work`. These temporary paths may disappear after cleanup/reboot;
the important results and scope are retained here.

For this machine, “installation complete” should mean that one recorded source
snapshot produces the installed service, client, CLI and helpers; both desktop
and terminal resolve them consistently; no required command depends on the
checkout; live state survives an installed-client restart; Niri workflows pass
after a fresh login; and rollback has a tested data-compatible path. Product
completion additionally requires the deferred feature milestones. Neither
criterion supports an absolute promise of bug-free behavior or universally
optimal performance.
