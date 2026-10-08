# Boardwalk Growth and Performance V1

Date: 2026-10-08

Status: first development pass after the verified V5 PC transfer. The new
catalog and customer story are WORKING proposals. This pass extends a profiling
tool and authors lore; it changes no production authority, archive schema,
Form rule, shop inventory, clinical actor, or presentation asset.

## Concrete development

[Boardwalk Offers and First Return](../BOARDWALK_SYMPIOTE_OFFERS_AND_FIRST_RETURN_V1_DRAFT.md)
proposes three products/services for each rival crew. Each offer has a stated
recipient, delivered work, observable success/defect, and correction path.
First Return, First Promise connects actual Sympiote discharge to an ordinary
task, optional help, rest or care, and an account of what was delivered.

The existing foundation can already record both offer considerations. Purchases,
items, suppliers, payment, delivery obligations and this new customer case need
their own explicitly adopted contracts before implementation. The old
returning-Goon case retains its identity, choices, events and outcomes.

The proposed first pair is Practice Salon / Carry Kit; the user is being asked
to select it or another pair. It remains a working recommendation until their
answer is recorded. Rivalry tone and exact terms remain open.

## Measured workload

`tools/benchmark-gameplay-service.py` keeps its original active-view mode and
adds `--scenario sympiote-return`. It creates a disposable session, establishes
a Hueman, uses live Glaüshouse authority from the supplied world root, runs the
foundation through consent/graft/recovery/discharge, explores five directions,
considers both camps, and reaches rested comfortable use before timing Sync.

`--assessment-rounds 80` retains 82 actual clinical receipts after establishment
and discharge. The view exposes the latest 32 and the full count. The script
records response sizes against the existing 65,536-byte Godot frame limit.
It never seeds a missing clinical office; missing authority fails the request.

Each host ran three rounds, alternating ordinary view, first return, and longer
history. Every run used 20 warmups and 200 measured requests. Both used the
same packaged service binary, SHA-256
`393f4331c4ce3e3a148e7578306f33e76a70c2c1f719e4e2018cd1cf5c404907`,
built from V5 source `e6f4f26e93fe5da0c428ce88df3aacb23feecc52`.

| Workload | Laptop median / p95 (µs) | AMD median / p95 (µs) | Largest response (bytes) | Full / projected clinical receipts |
|---|---:|---:|---:|---:|
| Ordinary active view | 342.093 / 472.656 | 54.903 / 67.396 | 21,577 | 0 / 0 |
| First Sympiote return | 591.098 / 702.586 | 57.989 / 73.167 | 23,987 | 3 / 3 |
| Return after 80 assessments | 485.159 / 775.507 | 77.540 / 113.923 | 36,421 | 82 / 32 |

These are medians of the three run medians and of their p95 values. The laptop
is the Intel i7-10875H; the PC is the Ryzen 7 9800X3D. Laptop variation is visible
in the saved per-run records. The hosts were not a controlled hardware
comparison. These observations establish current workload baselines rather
than a measured before/after benefit from an engine change.

Timing starts at request write/flush and ends at receipt of the response line.
JSON decoding is outside the timing. The AMD script was invoked over SSH, but
measured requests use pipes to a service on that same PC. SSH transmission,
GPU rendering, display frame pacing, and FPS are not timed. No original saves
were used or changed.

## What the measurements resolve

The first-return projection fits in 23,987 bytes. The longer-history case fits
in 36,421 bytes, leaving 29,115 bytes beneath the current transport limit for
this measured view. Retaining 82 clinical records did not make the routine
projection include all 82. Keep this bound when adding offer and delivery data.

The measured AMD first-return median is approximately 0.058 ms, and the longer
history median approximately 0.078 ms. These measurements support proceeding
with a small customer story. They do not justify changing lawful replay,
consent, history retention, the kernel scheduler, or graphics policy.

For future implementation, keep one owner for static offer descriptions and
compact current offer/result records in ordinary views. Test the combined
response with other active cases. An ordinary view below the limit here does
not prove every possible ID, composition, history or future catalog fits.

## Reproduction

Run on either machine from the checkout:

```bash
python3 tools/benchmark-gameplay-service.py \
  --service ~/.local/share/hollow-grove/current-core/lib/hollow-grove/core/hollow_grove_game_service \
  --world-root "$PWD" \
  --scenario sympiote-return \
  --assessment-rounds 80 \
  --warmup 20 --samples 200 \
  --output dist/sympiote-return-performance.json
```

On a development machine without installed Core, use
`target/release/hollow_grove_game_service` after building that binary. The
supplied world must contain the active clinical office used by the case.
For the ordinary view omit `--scenario` and `--assessment-rounds`.

Raw current evidence is in `dist/BOARDWALK-GROWTH-{LAPTOP,AMD}-PERFORMANCE.json`
and `dist/BOARDWALK-GROWTH-PERFORMANCE-SUMMARY.json`; each records binary and
harness hashes, samples, sizes, state counts and setup. The extended harness
also records source status so uncommitted profiling work is distinguishable
from a released engine.

## Next implementation boundary

First adopt the offer pair, task and commercial terms. Then implement the
smallest owned delivery/transaction case and clinic interaction required by
that adopted story. Any changes crossing the Foundation Freeze need their
versioned migration, replay fixtures, audit update and applicable approval.
The existing V5 installed runtime remains the verified game release during
this authoring/profiling pass. New source/tool commits should be synced without
claiming that unratified offers have become executable purchases.
