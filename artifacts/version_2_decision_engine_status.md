# Version 2.0 Decision Trace Snapshot

Date: 2026-07-14

## Scope

This increment adds an immutable deterministic `DecisionTrace` to the existing Version 2 decision engine.

Version 2 still performs:

Observe
↓
Generate
↓
Evaluate
↓
Choose
↓
Recipe
↓
Version 1.1

Version 1.1 execution semantics remain unchanged.

## Files Changed

- `src/decision_engine.rs`
- `src/kernel_pass_output.rs`
- `src/bin/client_desktop_status.rs`
- `src/lib.rs`
- `VERSION_2_DECISION_ENGINE.md`
- `artifacts/version_2_decision_engine_status.md`

Refreshed witness artifact:

- `artifacts/desktop_status.txt`

## Commands Run

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features`
- `cargo test -- --list | rg ': test$' | wc -l`
- `cargo test`
- `cargo build`
- `cargo build --release`
- `./bench-local.sh`
- `./stress-local.sh`
- `cargo run --quiet --bin client_desktop_status`
- `ls -l target/release/client_desktop_status target/release/current_synthesis_benchmark target/release/hollow-grove`

## Verification Results

- test count before Decision Trace: `371`
- test count after Decision Trace: `376`
- `cargo fmt --check`: pass
- `cargo clippy --all-targets --all-features`: pass with existing repository warnings
- `cargo test`: pass
- `cargo build`: pass
- `cargo build --release`: pass
- `./bench-local.sh`: pass
- `./stress-local.sh`: pass
- desktop witness refresh: pass

## Decision Trace Surface

The recorded trace now carries:

- observation evidence
  - frame
  - flow learnset
  - glow learnset
  - intent
  - optional observed route geometry
  - candidate-specific realized-state checks
- generation evidence
  - canonical candidate order
  - candidate identity
  - manager
  - manager geometry
  - orientation
- evaluation evidence
  - intent score
  - realized-state penalty
  - final score
  - typed reason codes
- choice evidence
  - highest score
  - tied candidates
  - tie occurrence
  - tie-break reason
  - geometry-match evidence
  - chosen candidate
- recipe bridge evidence
  - chosen candidate
  - resolved recipe id
  - Version 1.1 handoff flag
- execution evidence
  - Miss or Kiss
  - landed frame
  - prism delta
  - added Flow
  - added Glow
  - Point² production flag

## Typed Reasons

Evaluation reason codes:

- `PreferredCurrentOrientation`
- `NonPreferredCurrentOrientation`
- `PreferredAuraOrientation`
- `NonPreferredAuraOrientation`
- `NeutralBaseScore`
- `AlreadyCanonicalFrame`
- `AlreadyKnowsCanonicalFlow`
- `AlreadyKnowsCanonicalGlow`

Tie-break reason codes:

- `NoTie`
- `ObservedRouteGeometryMatch`
- `CanonicalGenerateOrder`

## Canonical Trace Fixtures

### FavorCurrent

- Observe: `Hueman`, `FavorCurrent`
- Generate: `GremlinTinker`, `PixyConfusion`
- Scores: `2` vs `1`
- Tie-break reason: `NoTie`
- Choose: `GremlinTinker`
- Recipe id: `gremlin_tinker`
- Version 1.1: `Kiss`
- Point²: `Gremlin`
- Prism delta: `Body +2`
- Flow: `+TinkerGrip`

### FavorAura

- Observe: `Hueman`, `FavorAura`
- Generate: `GremlinTinker`, `PixyConfusion`
- Scores: `1` vs `2`
- Tie-break reason: `NoTie`
- Choose: `PixyConfusion`
- Recipe id: `pixy_confusion`
- Version 1.1: `Kiss`
- Point²: `Pixy`
- Prism delta: `Mind +2`
- Glow: `+Confusion`

### Neutral Point Observation

- Observe: `Hueman`, `Neutral`
- Scores: `1` vs `1`
- Tie-break reason: `CanonicalGenerateOrder`
- Choose: `GremlinTinker`

### Neutral Straight KernelPass

- Observe route geometry: `Straight`
- Scores: `1` vs `1`
- Tie-break reason: `ObservedRouteGeometryMatch`
- Matching candidate: `GremlinTinker`
- Choose: `GremlinTinker`

### Neutral Curved KernelPass

- Observe route geometry: `Curved`
- Scores: `1` vs `1`
- Tie-break reason: `ObservedRouteGeometryMatch`
- Matching candidate: `PixyConfusion`
- Choose: `PixyConfusion`

### Neutral Realized Gremlin State

- Observe: `Gremlin`, `Neutral`
- `GremlinTinker` penalty: `1`
- reasons include:
  - `AlreadyCanonicalFrame`
  - `AlreadyKnowsCanonicalFlow`
- Choose: `PixyConfusion`

### Neutral Realized Pixy State

- Observe: `Pixy`, `Neutral`
- `PixyConfusion` penalty: `1`
- reasons include:
  - `AlreadyCanonicalFrame`
  - `AlreadyKnowsCanonicalGlow`
- Choose: `GremlinTinker`

## Replay Verification

Production replay now verifies:

- observation
- generation
- evaluation
- tie-break
- choice
- recipe bridge

Verified outcomes:

- unchanged valid trace: accepted
- changed score: rejected
- changed candidate order: rejected
- changed tie-break reason: rejected
- changed chosen candidate: rejected

Replay does not execute Version 1.1 and does not create a new `Point²`.

## Desktop Witness

The desktop witness now renders a `VERSION 2 DECISION TRACE` section.

Each canonical block now includes:

- Observe
- State checks
- Generate
- Evaluate with component scores
- Choose
- Tie-break evidence
- Recipe bridge
- concise Version 1.1 execution summary

## Version 1.1 Regression Status

Confirmed unchanged:

- canonical kernel witness
- canonical `Fourway`
- canonical `Point² (Landed Point)`
- transactional `Kiss`
- `Miss` preserves the original `FrameState`
- rollback guarantees
- canonical Pixy fixture
- canonical Gremlin fixture
- manager-domain invariants

Version 2 still does not:

- mutate `FrameState` directly
- copy script definitions into Version 2 candidates
- bypass `Recipe`
- bypass `Compiler`
- bypass `Aim`
- bypass `Fire`
- bypass transactional landing
- create `Point²` outside Version 1.1

## Benchmark Effects

Baseline before Decision Trace:

- `cargo run --quiet` average: `6791 us`
- `cargo run --release --quiet` average: `6468 us`
- `target/release/hollow-grove` average: `576 us`

After Decision Trace:

- `cargo run --quiet` average: `8291 us`
- `cargo run --release --quiet` average: `6858 us`
- `target/release/hollow-grove` average: `598 us`

Observed effect:

- witness output is larger and now includes trace evidence
- release runtime increased modestly but remained deterministic

## Stress Result

`./stress-local.sh` final results:

- warmup 100 runs: `0` failures, `0` mismatches, avg `768 us`
- standard 10,000 runs: `0` failures, `0` mismatches, avg `742 us`
- heavy 100,000 runs: `0` failures, `0` mismatches, avg `724 us`

## Binary Sizes

Before Decision Trace:

- `target/release/client_desktop_status`: `603672`
- `target/release/current_synthesis_benchmark`: `1087872`
- `target/release/hollow-grove`: `612456`

After Decision Trace:

- `target/release/client_desktop_status`: `625616`
- `target/release/current_synthesis_benchmark`: `1109736`
- `target/release/hollow-grove`: `613656`

## Warnings

`cargo clippy --all-targets --all-features` remains green with existing repository warnings in unrelated areas such as:

- `aim.rs` duplicate `#[must_use]`
- several `from_str` helpers that could implement `FromStr`
- existing large-argument helper functions
- existing needless borrows in some binaries

No clippy errors were introduced by the Decision Trace increment.

## Working-Tree Limitation

The repository worktree was already dirty before this increment began.

A clean isolated commit was therefore not created automatically, to avoid bundling unrelated workspace changes into the Decision Trace boundary.

This file serves as the explicit Version 2.0 Decision Trace snapshot for this run.
