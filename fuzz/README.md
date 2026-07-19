# Hollow Grove Fuzz Targets

Targets:

- `decision_input`: structured hostile decision inputs covering frame, flow, glow, intent, route surface, and invalid candidate arrangements.
- `decision_trace_replay`: canonical traces plus bounded authoritative corruptions; valid traces must replay, corrupted traces must reject.
- `recipe_compiler`: canonical and malformed recipes; compilation must be deterministic and rollback must hold for canonical injected cuts.
- `snapshot_boundaries`: canonical and malformed snapshot boundary payloads; valid payloads must parse deterministically and malformed payloads must reject.

Corpora:

- `fuzz/corpus/<target>/`

Crash artifacts and logs:

- `artifacts/verification/fuzz/`

Crash reproduction:

```bash
PATH=/home/warren/.cargo/bin:$PATH cargo +nightly fuzz run <target> fuzz/artifacts/<target>/crash-*
```
