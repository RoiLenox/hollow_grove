# Software checkpoint: 1 November 2026

Status: verified local development checkpoint on `os/package-foundation`.
This records implementation and test evidence; it does not change authored
constitutional authority or declare the product complete.

## Scope

The supported local Core/Hueman stack works on both noahsarch (the canonical
development PC) and occamsrazer (the laptop canary) in the checks performed.
This checkpoint captures the previously uncommitted device handoff tool,
launcher locking, Hueman performance policy, optional shared-session foundation,
and their package integration and tests.

The runtime retains authority in Rust; Hueman sends intents and presents returned
views. Device handoff explicitly transfers world/save state over pinned SSH,
checks installed payload compatibility, parks the sender, rejects independent
destination progress, and retains backups and recovery records. It is separate
from Git synchronization. See [Two-device operation](TWO_DEVICE_OPERATION_V1.md).

The optional same-player shared-session bridge includes credential checks,
passive observation and host checkpoints. Neither audited machine has a shared
client or host configuration. Authenticated multiplayer collaboration remains
future work.

Hueman defaults to 60 FPS in the foreground, 15 FPS in the background and
rendering on demand. The laptop machine profile describes its observed NVIDIA
display/render configuration. Package recipes include the device/shared-session
helpers and the local Fish-to-Niri login guard.

## Verification

The audit started on 1 November US/Central and continued into 2 November UTC.
Both checkouts and their installed source inventories initially matched the
same 1,300 file contents, based on `d9fbf15` plus the changes captured here.

| Check | Result |
| --- | --- |
| Formatting, source contracts, Registrar, geography, art and keyboard contracts | Pass |
| Rust workspace, default features | 1,695 passed; zero failed or ignored |
| Rust workspace, all features | 1,833 passed; zero failed or ignored |
| Clippy, all workspace targets/features, warnings denied | Pass |
| Python suites, including optional shared-authority integration | 68 cases passed |
| Godot source smoke suite and Core/Godot lifecycle | Pass |
| Installed Core/Hueman on each host | Entry/exit, Boardwalk traversal and exact-view save/restart/load passed |
| GPU rendering | Actual AMD and NVIDIA renders inspected |
| Presentation policy | Foreground/background limits observed on both hosts |
| Disposable cross-host handoff | Forward transfer, retry and reverse transfer passed with live state preserved |
| Installed package integrity | All five packages intact on both hosts |

The two Rust configurations overlap; their counts are not unique-test totals.
Performance samples establish configured limits and short idle behavior, not
long-session frame-time qualification. The handoff rehearsal used disposable
state and the installed device code with real SSH transport.

Commit review corrected a malformed portable-release `install` command that
would prevent staging the new Python helpers. The corrected Core staging block
was exercised in a temporary release root. This packaging-only correction and
checkpoint documentation were made after the full runtime audit; installed
runtime payloads were unchanged. A complete portable release build remains a
separate qualification step.

The laptop certificate symlink and invalid paccache timer override were also
repaired and independently verified. Those are host configuration repairs,
outside this source commit.

## Remaining work

- Ten headless House route bindings still need gameplay presentation. The
  backend register retains 17 open and four partially adopted decision families.
- General Synthesis, Synblock crafting, Synbalm healing execution and grade
  advancement remain unfinished.
- Shared-session deployment and multiplayer acceptance are not complete.
- Clean-chroot release qualification, signing/promotion, installation media,
  upgrade and disaster-recovery rehearsals remain release gates.
- Privileged access/recovery inspection and interactive lock, suspend,
  peripheral and reboot checks remain host acceptance work.

The working source on noahsarch is the checkpoint authority. A local Git commit
does not synchronize the laptop checkout, publish packages or back up live saves.
Resume from this checkpoint by selecting the next bounded implementation goal;
use the canary-to-stable process for subsequent deployment.

Detailed machine-specific logs and receipts remain outside the repository at
`/home/warren/Projects/audits/hollow-grove-20261101/`, beginning with `AUDIT.md`
and `REPAIRS_APPLIED.md`.
