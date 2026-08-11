# Artifact Ownership

Status: foundation release policy

Hollow Grove keeps authored inputs, reproducible projections, local runtime
state, and distributable output separate.

| Location | Owner | Version-control rule |
|---|---|---|
| `src/`, kernel crates, `hueman_godot/` | authored implementation | tracked |
| constitutional and design Markdown | authored authority/history | tracked |
| `tests/fixtures/` and protocol fixtures | compatibility proof | tracked |
| `artifacts/` | reproducible witnesses and selected canonical baselines | tracked only when a test or authority document names the file |
| ignored paths under `artifacts/` | runtime saves, logs, benchmarks, fuzz/soak output | never tracked; promote a minimized case into `tests/fixtures/` |
| `dist/`, `release/`, `target/` | derived build/package output | never tracked |
| `hueman_godot/assets/source/` | editable art source | tracked |
| `hueman_godot/assets/generated/` | deterministic engine-ready art | tracked only when its source and manifest entry are tracked |

Every generated asset must identify its source, generator, dimensions, stable
ID, and digest in the asset manifest. Release scripts may recreate `dist/`
from a clean checkout; they must not mutate constitutional sources.
