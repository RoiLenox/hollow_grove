# Hollow Grove V2 Performance Characterization

Measurement date: 2026-07-18
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
| Single transition reduction | 500 | 0.670 µs | 0.592 µs | 7.324 µs |
| Full lawful Bond lifecycle | 100 | 94.531 µs | 90.580 µs | 155.462 µs |
| Full lifecycle plus persistence/decode | 100 | 153.700 µs | 150.302 µs | 214.042 µs |
| Regional V0→V1 migration | 500 | 6.593 µs | 6.482 µs | 10.870 µs |
| Regional archive digest | 1,000 | 1.828 µs | 1.823 µs | 4.438 µs |
| Regional trace generation | 500 | 16.783 µs | 16.641 µs | 23.674 µs |
| Idempotent Synthesis retry | 1,000 | 0.445 µs | 0.441 µs | 2.284 µs |
| Complete scenario catalog execution | 50 | 845.807 µs | 842.361 µs | 957.837 µs |
| Gnome→Minotaur complete scenario | 500 | 44.042 µs | 43.852 µs | 60.133 µs |
| Elf→Centaur complete scenario | 500 | 45.691 µs | 45.566 µs | 52.869 µs |
| Regional persistence encode/decode | 500 | 7.602 µs | 7.494 µs | 23.705 µs |
| Regional live replay | 500 | 8.467 µs | 8.346 µs | 15.859 µs |
| Regional lineage inspection | 10,000 | 0.021 µs | 0.020 µs | 0.121 µs |
| Aura Fields stewardship lookup | 10,000 | 0.021 µs | 0.020 µs | 1.923 µs |
| Aura Sea guardianship lookup | 10,000 | 0.021 µs | 0.020 µs | 0.150 µs |
| Rejected cross-regional Synthesis | 1,000 | 20.814 µs | 20.649 µs | 40.957 µs |
| Replay 10 events | 200 | 10.501 µs | 10.360 µs | 15.179 µs |
| Replay 100 events | 100 | 101.601 µs | 101.150 µs | 122.890 µs |
| Replay 1,000 events | 20 | 1.769 ms | 1.761 ms | 1.928 ms |
| Replay 10,000 events | 3 | 107.914 ms | 107.348 ms | 110.171 ms |

## What Each Measurement Includes

### Single transition reduction

Creates a fresh `RegionalSynthesisRuntime`, constructs an institution-backed
Gnome registration for the Aura Fields, validates it, applies it, and updates
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
operation. The difference between the medians is approximately 59.7 µs on this
run.

### Regional Synthesis scenarios

Each 43–45 µs scenario includes fixture construction, origin registration,
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
| 10 | 10.360 µs | 1.036 µs | — |
| 100 | 101.150 µs | 1.012 µs | 9.76× for 10× events |
| 1,000 | 1.761 ms | 1.761 µs | 17.41× for 10× events |
| 10,000 | 107.348 ms | 10.735 µs | 60.96× for 10× events |

The small histories are close to linear. The large registration-only history
exposes superlinear behavior because the regional runtime currently scans the
event vector for idempotent retry detection before each append, while also
maintaining ordered maps and cloning owned event data. The 10,000-event result
is practical for demonstration and recovery but identifies an optimization
target for sustained high-volume operation.

Any optimization must preserve exact event equality, conflict detection,
caller-supplied identities, causal ordering, and fail-closed replay. A narrow
`RegionalEventId → index` map would be the first candidate; it would consolidate
an existing index responsibility rather than add domain law.

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
