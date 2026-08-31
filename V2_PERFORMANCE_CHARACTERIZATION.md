# Hollow Grove V2 Performance Characterization

Measurement date: 2026-08-22
Harness: `examples/constitutional_v2_bench.rs`
Profile: Cargo `--release`
Clock: `std::time::Instant`

## Method

The harness runs actual production scenario functions, archives, reducers, and
lookups. It does not substitute synthetic arithmetic for constitutional work.
Each operation is measured repeatedly in-process. Samples are sorted for the
median; average is total elapsed nanoseconds divided by iterations; worst is
the largest observed sample. `std::hint::black_box` prevents the measured
result from being optimized away.

These observations characterize this machine and build, not a protocol-level
latency guarantee. Scheduler activity, allocator state, CPU frequency, and
toolchain changes can move the values. Constitutional correctness never depends
on a timing threshold.

Command:

```text
cargo run --release --quiet --example constitutional_v2_bench
```

## Recorded Results

| Operation | Iterations | Average | Median | Worst observed |
|---|---:|---:|---:|---:|
| Single transition reduction | 500 | 0.481 µs | 0.461 µs | 6.172 µs |
| Full lawful Bond lifecycle | 100 | 169.729 µs | 155.996 µs | 487.215 µs |
| Full lifecycle plus persistence/decode | 100 | 224.188 µs | 217.042 µs | 317.533 µs |
| Regional V0→V1 migration | 500 | 5.950 µs | 5.731 µs | 13.917 µs |
| Regional archive digest | 1,000 | 2.033 µs | 2.014 µs | 3.667 µs |
| Regional trace generation | 500 | 16.126 µs | 15.459 µs | 50.576 µs |
| Idempotent Synthesis retry | 1,000 | 0.426 µs | 0.421 µs | 2.535 µs |
| Complete scenario catalog execution | 50 | 1.707 ms | 1.673 ms | 2.091 ms |
| legacy `gnome-minotaur` slug (now Faun→Minotaur) | 500 | 79.028 µs | 75.834 µs | 317.292 µs |
| legacy `elf-centaur` slug (now Satyr→Centaur) | 500 | 80.147 µs | 77.136 µs | 275.473 µs |
| Regional persistence encode/decode | 500 | 6.951 µs | 6.523 µs | 26.901 µs |
| Regional live replay | 500 | 7.021 µs | 6.853 µs | 17.594 µs |
| Regional lineage inspection | 10,000 | 0.020 µs | 0.020 µs | 1.102 µs |
| Aura Field stewardship lookup | 10,000 | 0.021 µs | 0.020 µs | 2.265 µs |
| Current Sea guardianship lookup | 10,000 | 0.019 µs | 0.020 µs | 0.211 µs |
| Rejected cross-regional Synthesis | 1,000 | 55.043 µs | 52.660 µs | 221.210 µs |
| Replay 10 events | 200 | 8.356 µs | 8.025 µs | 20.369 µs |
| Replay 100 events | 100 | 81.400 µs | 77.748 µs | 174.762 µs |
| Replay 1,000 events | 20 | 970.088 µs | 929.444 µs | 1.263 ms |
| Replay 10,000 events | 3 | 11.208 ms | 10.972 ms | 12.047 ms |

## What Each Measurement Includes

### Single transition reduction

Creates a fresh `RegionalSynthesisRuntime`, constructs an institution-backed
Gnome registration for the Aura Field, validates it, applies it, and updates
the event and Being indexes. The institutional fixture is prepared outside the
timed closure. This is representative of a small accepted state transition.

### Full lawful lifecycle

Runs `run_ordinary_lifecycle`, including Wave recording, Stonebend naming,
Sandmanor proof, activation, Current movement, accumulation, Aura observation,
evaluation, maturity, excess, Glaüshouse clearance, Tombstone formation and
validation, Flynt recognition, Toke recording, Glaüshouse renewal resolution,
successor Wave, successor formation, and integrity verification.

### Full lifecycle with persistence

Adds canonical V1 archive encoding and decode-through-replay to the preceding
operation. The difference between the medians is approximately 61.0 µs on this
run.

### Regional Synthesis scenarios

Each 76–77 µs scenario includes fixture construction, origin registration,
institution-derived Sandmanor and Glaüshouse decisions, subject-evidence checks,
lineage reduction, typed assignment, one idempotent retry, direct replay,
archive encoding, archive decoding through the reducer, and canonical byte
comparison. These values therefore exceed the cost of the single Synthesis
transition itself and represent the executable proof fixture as a whole.

### Regional inspection

Lineage, stewardship, and guardianship are `BTreeMap` lookup plus typed enum
projection. Their measured medians are approximately 20 nanoseconds. The exact
value is close to timer-resolution and loop-overhead territory; treat it as an
indication that lookup is negligible relative to replay or scenario creation,
not as a guaranteed nanosecond SLA.

### Invalid cross-regional Synthesis

Builds an institution-backed registered Gnome and attempts Gnome→Centaur.
Lineage validation returns a typed error before committing an event or result
Being. Failure remains cheaper than a complete accepted scenario while still
performing the real setup and reducer call.

## Replay Scaling

| Events | Median | Median per event | Growth from prior row |
|---:|---:|---:|---:|
| 10 | 8.025 µs | 0.803 µs | — |
| 100 | 77.748 µs | 0.777 µs | 9.69× for 10× events |
| 1,000 | 929.444 µs | 0.929 µs | 11.95× for 10× events |
| 10,000 | 10.972 ms | 1.097 µs | 11.80× for 10× events |

Replay is now close to linear across the measured range. The regional runtime
maintains a narrow `RegionalEventId → position` index for duplicate and
idempotent-retry detection instead of scanning the accumulated event vector on
every append. The 10,000-event median fell from the prior 107.348 ms baseline
to 10.972 ms (about 9.8× faster) while preserving exact event equality,
conflict detection, caller-supplied identities, causal ordering, and
fail-closed replay.

## Allocation-Sensitive Observations

- Event and evidence records own stable strings so archive/replay cost includes
  allocations and cloning.
- `BTreeMap`/`BTreeSet` provide deterministic ordering at a higher constant cost
  than hash-based collections. This is deliberate.
- Archive decode allocates strings and vectors, then reconstructs state through
  the reducer; it does not deserialize an unchecked state snapshot.
- Trace generation allocates presentation strings and duty lists. It remains
  read-only and costs roughly twice a two-event regional replay in this run.
- The scenario catalog repeatedly constructs the institutional fixture. A
  long-lived host may retain one validated catalog without changing semantics.

## Performance Guarantees and Non-Guarantees

The implementation guarantees deterministic results, not deterministic wall
time. No benchmark authorizes:

- skipping authority or evidence validation;
- trusting persisted assignments;
- replacing ordered canonical encoding with insertion-order output;
- moving Current when adapting a Wave;
- caching a rejection as an accepted event;
- making trace or presentation code decide Synthesis;
- weakening identity conflicts or idempotency checks.

## Reproduction and Regression Use

Compile check:

```text
cargo check --example constitutional_v2_bench
```

Release characterization:

```text
cargo run --release --quiet --example constitutional_v2_bench
```

Future runs should retain the raw harness output in review notes. A regression
should be investigated when it changes scaling class or causes operational
pain; correctness and replay equality remain the release gates.
