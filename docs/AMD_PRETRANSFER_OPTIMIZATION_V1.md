# AMD Pre-transfer Optimization V1

Date: 2026-10-08

Target: AMD Ryzen 7 9800X3D and AMD Radeon RX 7900 XTX

Local measurement host: Intel Core i7-10875H, x86_64, CachyOS BORE
`7.2.9-1-cachyos-bore`. The AMD PC is offline. These are local measurements,
not AMD frame-rate claims.

Checkpoint: `lore-sync-2026-10-08-v2`, following the preserved
`lore-sync-2026-10-07-v1` baseline.

## Review from the kernel through presentation

| Layer | Evidence and disposition |
|---|---|
| Linux kernel and hardware | Keep the measured deployment policy in [Linux Kernel Policy](LINUX_KERNEL_POLICY.md). Target kernel, graphics driver, frame pacing, and thermal behavior require an AMD-machine run. |
| Pure recursion kernel | Dependency-free, stack-owned routing and `const` grammar operations. Lore remains above this layer. |
| Neutral semantic kernel | Stable typed references, bounded geometry, and indexed composition records retain their existing validation and deterministic projections. |
| Constitutional runtime | Production reduction, lawful lifecycle, persistence, and replay were measured through the existing release harness. Authority, refusals, ordering, checksums, and replay remain the governing checks. |
| Gameplay service | A new reusable harness measures active-view synchronization with disposable state; local median 0.131 ms, p95 0.238 ms, and responses about 20.9 KB. |
| Client transport | Fixed fragmented UTF-8 corruption and repeated whole-buffer text conversion/scanning. Measured fragmented-response assembly improved substantially. |
| Godot presentation | Existing animation redraws are limited to their cadence; semantic updates compare their projected content before publishing. Keep the current Mobile renderer through local acceptance; compare GPU frame times on the AMD PC. |
| Packaging and saves | Thin LTO, one release codegen unit, stripped binaries, verified source manifests, separate Core/Hueman payloads, and copy-based state acceptance provide the portable checkpoint. |
| Lore and gameplay | The adopted Boardwalk organization and first lore milestone are included. Working designs and unverified gameplay bindings remain explicitly identified in the [lore implementation notes](LORE_KERNEL_OPTIMIZATION_V1.md). |

The common release remains portable between the Intel development machine and
the AMD PC. CPU-specific compilation and global power, scheduler, or graphics
settings are separate experiments whose benefit must be measured on the target.

## Transport correction and measured effect

The old client decoded each TCP chunk independently, then repeatedly encoded
the growing partial string to count bytes. A UTF-8 character split between
chunks could become replacement characters inside otherwise valid JSON.

The client now appends raw bytes, searches only the newly received suffix for
a newline, checks the byte limit, and decodes a complete response once. Each
poll reads at most 65,537 bytes; an incomplete response is capped at the
existing 65,536-byte limit. Request matching, response ownership, authority,
and wire schema are unchanged. API behavior is documented by Godot's
[PackedByteArray](https://docs.godotengine.org/en/stable/classes/class_packedbytearray.html)
and [StreamPeer](https://docs.godotengine.org/en/stable/classes/class_streampeer.html)
references.

Three alternating before/after runs used the same headless Godot build, ten
warmup iterations, and 100 measured responses each. A 32,179-byte JSON response
arrived in 256-byte chunks. The harness uses the prior accumulation/consume
sequence for the baseline and the production byte-append path for the fix.

| Measurement | Before | After |
|---|---:|---:|
| Median of run medians | 3,084 µs | 396 µs |
| Median of run p95 values | 3,314 µs | 481 µs |
| One-byte fragmented accented-text round trip | corrupted | exact |

That is approximately 7.8 times faster for this fragmented-response workload,
or an 87% median-time reduction. It is neither an end-to-end latency guarantee
nor a game-FPS measurement. Scheduling noise appears in individual runs; the
unfragmented localhost workload may spend much less time in reassembly.

`runtime_client_transport_smoke.gd` verifies two-, three-, and four-byte UTF-8
characters split into single bytes, combined frames with an incomplete tail,
invalid-envelope recovery, exact-limit acceptance, oversized-frame rejection,
pending-request cleanup, and disconnect recovery. It is part of
`./verify-godot.sh`.

## Runtime baseline

The existing `constitutional_v2_bench` release harness measured real production
operations on this Intel machine:

| Operation | Median |
|---|---:|
| Single transition reduction | 0.857 µs |
| Full lawful Bond lifecycle | 378.330 µs |
| Lifecycle plus persistence | 466.262 µs |
| Regional replay, 1,000 events | 1.634 ms |
| Regional replay, 10,000 events | 22.707 ms |

These values characterize this run and its workload. The historical August
figures came from a different baseline and do not establish a regression on
this machine. The active gameplay sync harness measures a different operation:
request write/flush through response-line receipt, with JSON response decoding
outside the timing. Its state and guide transitions are temporary.

## Reproduce on the AMD PC

After the peer checkpoint is installed and checked:

```bash
python3 tools/benchmark-gameplay-service.py \
  --service ~/.local/share/hollow-grove/current-core/lib/hollow-grove/core/hollow_grove_game_service
godot --headless --path hueman_godot \
  --script res://tests/runtime_client_transport_bench.gd
cargo run --release --locked --offline --example constitutional_v2_bench
```

Use the corresponding XDG data path if the default data home is overridden.
The service and transport harnesses need no network connection and do not
write existing saves. Rust dependencies are needed only for the Cargo harness.
For controlled Linux-kernel comparisons, use `benchmark-linux-kernel.sh` and
the three-run protocol in the kernel policy.

Launch the packaged Core/Hueman pair on the actual display, exercise movement,
dialogue, the Boardwalk choices, and save/restart/load, and record frame-time
tails before changing rendering or system policy. Driver and display behavior
remain peer acceptance work.

Raw local timings and release proofs are generated under ignored `dist/`.
`LOCAL-TRANSFER-READINESS.json` and `TRANSFER-SHA256SUMS` identify the final
checked source and payloads. See [Lore Sync Checkpoint](LORE_SYNC_CHECKPOINT_V1.md)
for transfer and recovery steps.
