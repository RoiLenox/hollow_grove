Historical performance record: use the [Operational Entrypoint](../HOLLOW_GROVE_OPERATIONAL_ENTRYPOINT_V1.md)
and live `architecture --json` for current coverage. The measurements and
counts below belong to their stated checkpoints.

Recorded extension: the adopted [survey-frame case](../STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md)
raises coverage to four verified bindings and 22 bounded architecture components,
with ten historical route contracts still unbound. Current reports use architecture
V2 and lore V4. The performance measurements below remain the recorded V6 comparison.

# Architecture Coverage and History Profile V1

This records the V6 optimization and its original coverage. The subsequent
[Stonebend continuity binding](STONEBEND_CONTINUITY_LORE_BINDING_V1.md) raises
the earlier verified coverage to three bindings, with ten unbound historical
routes; the performance evidence below still belongs to V6.

This pass makes the lore-to-runtime relationship inspectable and reduces the
cost of gameplay transactions as their event journal grows. It adds no new
Forms, shop products, House authority or constitutional law. The proposed
Boardwalk catalog remains unratified.

## Inspect the architecture

```bash
cargo run --locked --offline --bin hollow-grove -- architecture --json
cargo run --locked --offline --bin hollow-grove -- architecture \
  --source-root . --save path/to/gameplay-archive.json --json
```

The additive read-only report has its own schema version 1. It records 21
bounded core components: the Compromise, recursion, semantics, constitutional
runtime, four Houses, geography, spatial inheritance, shared interfaces,
Central Junction, Service Tournament, Rampart, Guides, gameplay, operational
lore, Sympiote foundation, recipes, Sympiote grafting and presentation.
Each entry links its primary contract, implementation, declared proof source,
and available persistence/projection owners. The audit rejects duplicate
component identities, duplicate primary contract ownership, missing or unsafe
relative paths and substitutions of the four principal House functions.
Stonebend Names, Sandmanor Proves, Glaüshouse Clears and Flynt Recognizes,
using the existing typed `HouseFunction` ownership rule.

File presence and declared proof links do not demonstrate execution or prove
that the whole setting is implemented. A failing source audit emits its
violations and exits unsuccessfully. A supplied archive must pass checksum
validation and full replay before any completion is reported. No save or
world state is written by inspection. Without a supplied runtime, completion
and replay revision remain absent.

The historical twelve-route roster remains unchanged. Boardwalk Return has
its verified reducer/persistence/projection binding; eleven entries still lack
that complete verified execution binding. Related runtime code can exist for
an unbound entry. The additional adopted Sympiote foundation is separately
bound to its clinical/practice reducer. It reports completion only when the
actual Form is established, the case is discharged, comfortable use is
recorded and no rest is outstanding. Interest in a direction or offer does
not establish a later Form or finish a case.

## Transaction optimization

Previously `HollowGroveGameRuntime::execute` cloned the entire aggregate,
including its accumulated gameplay events and event-ID index, before either
accepting or refusing a command. Every new ID also searched the journal for
a retry. These costs grew with history even for ordinary movement.

The candidate now clones every domain field while starting with an empty
gameplay journal/index. Metadata and retry checks use the live journal; the
existing event-ID index skips the linear retry search for new IDs. After
reducers, sequence conversion and revision advancement all succeed, the
accepted candidate takes ownership of the original journal/index and appends
the new event. Refused commands never take ownership or change live state.
The explicit complete candidate initializer makes newly added aggregate
fields a compile-time obligation. Reducers must continue to operate on staged
domain state; code that needs historical gameplay events belongs outside this
candidate boundary or requires a reviewed extension of it.

This preserves full event history, retry equality and conflicts, causal
ordering, revision semantics, archive encoding and validated replay. Gameplay
archive schema V5, protocol V1, GameView, LoreInspection V2, HGSYM/V1 and the
neutral kernels are unchanged. No save migration, snapshot or history pruning
is introduced. Domain subruntime histories still clone; whole-aggregate cloning,
encoding, projection and replay have their existing costs.

## Measurements

`examples/gameplay_history_bench.rs` creates disposable histories and measures
aggregate cloning, accepted/refused commands, archive encoding and decode/replay.
It checks replay byte equivalence and unchanged archives after refusal before
timing. Fixture creation, forks and command construction are outside each
command measurement; no real saves are used.

```bash
cargo run --release --locked --offline --example gameplay_history_bench -- \
  --world-root . --histories 0,128,1024,4096 --samples 20 --warmup 3
```

On 2026-10-08 the baseline used runtime source
`6350d848f378f34b5a0ac9af535e78b1925c9d59`; the optimized source is in
`lore-sync-2026-10-08-v6`. Each machine ran three alternating baseline/optimized
pairs, twenty samples and three warmups per operation. The table gives the
median of the three per-run medians, in microseconds.

| Machine | Events | Accepted before | Accepted after | Refused before | Refused after |
|---|---:|---:|---:|---:|---:|
| Laptop, i7-10875H | 1 | 28.555 | 21.881 | 23.080 | 17.652 |
| Laptop, i7-10875H | 129 | 54.465 | 25.671 | 49.718 | 19.220 |
| Laptop, i7-10875H | 1,025 | 603.585 | 28.720 | 292.309 | 19.957 |
| Laptop, i7-10875H | 4,097 | 611.439 | 28.064 | 641.433 | 20.465 |
| noahsarch, 9800X3D | 1 | 10.520 | 11.031 | 9.097 | 8.866 |
| noahsarch, 9800X3D | 129 | 18.915 | 10.770 | 16.891 | 8.005 |
| noahsarch, 9800X3D | 1,025 | 77.525 | 11.962 | 72.496 | 9.218 |
| noahsarch, 9800X3D | 4,097 | 463.428 | 12.122 | 278.852 | 9.247 |

Baseline accepted-command medians at 4,097 events varied from 280.766 to
479.208 µs on noahsarch; optimized medians ranged from 12.013 to 12.503 µs.
Laptop timings also varied. Near-empty AMD histories show no demonstrated
speed improvement. These are in-process CPU observations with warm disposable
state, not disk durability, transport, Godot rendering or RX 7900 XTX FPS
measurements. Encoding and replay remain history-dependent; at 4,097 events,
AMD decode/replay remained about 16 ms. The raw run files and binary hashes
are in `dist/kernel-history-{before,after}-{laptop,amd}-{1,2,3}.json` and
`dist/KERNEL-HISTORY-COMPARISON.json`; final source/release provenance is in
the V6 sync receipt. Preserve these measurements when refreshing artifacts.

## Validation and next growth

Six added integration tests cover coverage versus execution, duplicate owners,
House substitution, unsafe/missing paths, long-history retries and atomic
refusal, byte-equivalent replay, the actual rested foundation milestone and
CLI checksum rejection/read-only behavior. The workspace passes 1,671 tests,
strict Clippy and formatting. A replayed comfortable-use witness reports
revision 13 and actual foundation completion.

For each subsequent lore episode, complete this chain before calling it an
implemented architectural feature: adopted contract → typed state/action →
authority and reducer checks → accepted event → archive/replay → read-only
projection → refusal/tamper/lifecycle proofs → crosswalk and measured workload.
Keep new civic law above the neutral recursion and semantic kernels.

The next candidate is the existing Stonebend object-lineage proposal: agree
on its bounded first case, then connect its existing domain code to a verified
gameplay binding and replay witness. The rival product offers likewise need
an adopted contract before purchase, delivery, refusal or restitution mechanics
are implemented. Existing eleven binding gaps provide the tracking list;
they are not permission to invent or ratify unfinished lore. Profile encoding,
view construction, nested domain history and replay separately when a real
case demonstrates their cost. Any archive/snapshot change needs its own
versioned migration and equivalence proof.
