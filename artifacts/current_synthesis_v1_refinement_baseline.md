# Current Synthesis (Hollow Grove) V1 Refinement Baseline

Date: 2026-07-14

## Scope

This report captures the pre-refinement baseline for the Hollow Grove V1 synthesis path before any refinement-pass code changes.

## Commands Run

```bash
cargo fmt --check
cargo test
cargo build
cargo build --release
cargo test -- --list | rg ': test$' | wc -l
cargo run --quiet --bin client_desktop_status
./target/release/current_synthesis_benchmark --warmup 1 --samples 3 --no-write
python3 - <<'PY'
import subprocess, time
root='/home/warren/hollow-grove'
for name, cmd in [('cargo build',['cargo','build']),('cargo build --release',['cargo','build','--release'])]:
    t0=time.perf_counter()
    subprocess.run(cmd,cwd=root,check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    dt=time.perf_counter()-t0
    print(f'{name}: {dt:.3f}s')
PY
python3 - <<'PY'
import os, time, subprocess, statistics
root='/home/warren/hollow-grove'
cmd=[os.path.join(root,'target/release/client_desktop_status')]
runs=[]
for _ in range(8):
    t0=time.perf_counter()
    subprocess.run(cmd,cwd=root,check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    runs.append(time.perf_counter()-t0)
trimmed=runs[1:]
print('runs=' + ', '.join(f'{r:.6f}' for r in runs))
print('avg=' + f'{statistics.mean(trimmed):.6f}')
print('min=' + f'{min(trimmed):.6f}')
print('max=' + f'{max(trimmed):.6f}')
PY
ls -lh target/debug/client_desktop_status target/release/client_desktop_status
ls -lh target/release/current_synthesis_benchmark target/release/hollow-grove
```

## Verification Baseline

- `cargo fmt --check`: pass
- `cargo test`: pass
- `cargo build`: pass
- `cargo build --release`: pass
- exact discovered test count: `348`

## Warm Build Timing

These timings were measured against an already-built workspace, so they are warm incremental timings rather than cold builds.

- `cargo build`: `0.014s`
- `cargo build --release`: `0.868s`

## Binary Sizes

- `target/debug/client_desktop_status`: `6.5M`
- `target/release/client_desktop_status`: `578K`
- `target/release/current_synthesis_benchmark`: `1.1M`
- `target/release/hollow-grove`: `591K`

## Direct Witness Runtime Baseline

Measured by running the built release binary directly rather than via `cargo run`.

- command: `./target/release/client_desktop_status`
- samples: 8
- discarded first-run warmup sample: yes
- retained sample average: `0.000353s`
- retained sample min: `0.000311s`
- retained sample max: `0.000390s`

## Existing Benchmark Baseline

Applicable benchmark path:

- `./target/release/current_synthesis_benchmark --warmup 1 --samples 3 --no-write`

Observed sample completions:

- sample 1: `80.370 us`
- sample 2: `72.476 us`
- sample 3: `68.429 us`

Generated release summary already present in repo:

- `artifacts/current_synthesis_benchmark_release.md`

Relevant release summary points:

- average full-route runtime: `0.074 ms`
- p95 full-route runtime: `0.080 ms`
- drift count: `0`
- release profile: pass
- binary footprint (`hollow-grove`): `604840` bytes

## Stress Coverage Status

Existing shell scripts are stale and were not used as authoritative stress coverage for V1 refinement:

- `bench-local.sh`
- `stress-local.sh`

Reason:

- both hard-code obsolete witness text:
  - `start Symptom 1`
  - `GroveSeam`
  - `HollowBeam`
  - `landed Symptom 2`

They no longer match the canonical V1 public witness:

- `Point`
- `Triway`
- `Fourway`
- `HollowGrove`
- `CurrentSeam`
- `AuraBeam`
- `Point² (Landed Point)`

## Public API Surface Snapshot

The synthesis-related public surface currently includes:

- `frame_state.rs`
  - `FrameId`
  - `FlowId`
  - `GlowId`
  - `CurrentPrism`
  - `FrameState`
- `synthesis_recipe.rs`
  - `PrismDelta`
  - `RecipeIntent`
  - `SynthesisRecipe`
  - `SynthesisScript`
  - `SynthesisRecipeCompileError`
  - `compile_recipe`
  - canonical fixtures
- `aim.rs`
  - `AimId`
  - `Aim`
  - `AimBuildError`
  - `construct_aim`
  - canonical fixtures
- `fire.rs`
  - `ContactOutcome`
  - `FireContext`
  - `fire`
  - `fire_with_context`
- `landing.rs`
  - `KissLanding`
  - `LandingOutcome`
  - `ScriptApplicationError`
  - `land_contact`
- `manager_domain.rs`
  - canonical manager enums and lock resolution
- `kernel_pass_output.rs`
  - snapshot/prompt/desktop/tree builders

The crate root currently re-exports the full synthesis path publicly.

## Dependency Direction Snapshot

Current observed dependency flow:

- `frame_state` is foundational.
- `synthesis_recipe` depends on ID types from `frame_state`.
- `aim` depends on `synthesis_recipe`, `manager_domain`, and kernel `Bond`/`Way`.
- `fire` depends on `aim` and manager/route semantics.
- `landing` depends on `frame_state`, `aim`, `fire`, and `synthesis_recipe`.
- `kernel_pass_output` currently depends on nearly the entire synthesis path and performs orchestration plus formatting.

Primary architectural smell:

- `kernel_pass_output` is not formatting-only. It currently compiles recipes, constructs aims, fires contact, performs landing, and formats all of those results.

## Clone / Allocation Hotspots From Inspection

Likely hot spots visible from source inspection:

1. `landing.rs`
   - clones `start` for `Miss`
   - clones `start` into transactional working state
   - clones `start` again into `KissLanding.before`
   - clones `aim.scripts()` into `applied_scripts`
   - reconstructs full `FrameState` repeatedly per script application
   - rebuilds `Vec` learnsets via `to_vec()` in `apply_prism_delta`, `add_flow`, `add_glow`, and `set_frame`

2. `kernel_pass_output.rs`
   - renderer constructs canonical recipes, compiles scripts, constructs aims, fires, lands, and also derives miss fixtures inline
   - repeated small `Vec` allocations for learned glow/flow summaries
   - multiple string `format!` paths that can be streamed into a single buffer

3. `aim.rs`
   - stores `source_recipe_id` as owned `String`
   - `named_route` is owned `Option<String>`

4. `synthesis_recipe.rs`
   - `recipe_id` and `display_name` are owned `String`
   - compile path clones intents into a fresh `Vec<SynthesisScript>`

## Architectural Audit

### A. Safe Cleanup

- reduce visibility for synthesis internals that are only used inside the crate
- remove stale public/gameplay wording that survived in support artifacts and benchmark helpers
- remove unused imports and dead helpers after verification
- add `#[must_use]` on pure result-returning APIs where helpful

### B. API Cleanup

- audit whether all synthesis constructors and fixture helpers need crate-root re-exports
- reduce `pub` to `pub(crate)` where external binary consumers do not rely on direct exposure
- keep snapshot/public consumer boundaries stable

### C. Rendering Cleanup

- extract pure frame/prism/script/manager render helpers
- separate synthesis computation from formatting in `kernel_pass_output`
- compute each canonical fixture once per render pass, then render from immutable results

### D. Allocation / Performance Candidates

- centralize transactional script application to reduce repeated full `FrameState` reconstruction
- reduce repeated learned-list `Vec` construction in rendering
- avoid rebuilding canonical fixture pipelines multiple times during one witness render
- prefer `write!`/`writeln!` into one `String` buffer over nested `format!` where duplication is large

### E. Risky Semantic Change — Do Not Perform

- changing kernel topology or public witness semantics
- replacing transactional working clone with incremental in-place mutation
- changing deterministic script ordering
- replacing ordered learnset `Vec`s with unordered sets
- altering manager-domain authority resolution

## Obsolete Terminology Audit

Current obsolete or legacy references found:

- stale benchmark/stress scripts:
  - `bench-local.sh`
  - `stress-local.sh`
- stale current synthesis support artifact index:
  - `src/current_synthesis_support.rs`
- stale public/support artifact text:
  - `src/hueman_support.rs`

Retained legitimate internal kernel names:

- `GroveSeam`
- `HollowBeam`
- `LandedSymptom`

These remain valid internal/kernel implementation names and should not be removed merely for cosmetic alignment, but they should not replace canonical public witness vocabulary.

## Current Sample Witness Status

Current detailed witness successfully shows:

- canonical kernel path including `Fourway` and `Point² (Landed Point)`
- Pixy Aura synthesis fixture
- Gremlin Current synthesis fixture
- canonical `Miss` diagnostics
- transactional `Kiss` landing details

## Worktree Status Note

The repository worktree was already dirty at baseline capture time. Any later milestone snapshot or commit decision must account for unrelated existing changes before attempting a safe freeze point.
