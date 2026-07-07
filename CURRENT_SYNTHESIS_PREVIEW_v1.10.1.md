# Current Synthesis Preview v1.10.1

Date: 2026-07-06

## Boundary

```text
Hollow Grove
↓
KernelPass
↓
Artifacts
↓
Current Synthesis Preview
↓
Current Synthesis Operational View
```

- Hollow Grove remains unchanged.
- `KernelPass` remains unchanged.
- Preview and operational meaning belong to Current Synthesis.

## Preview Proof

- `current_synthesis_preview` consumes existing Current Synthesis artifacts only.
- `current_synthesis_operational` consumes existing Current Synthesis artifacts only.
- Both outputs are deterministic and read-only.
- Neither output executes paths, movement, automation, or runtime state.

## Current Outputs

- `artifacts/current_synthesis_preview.md`
- `artifacts/current_synthesis_operational.md`

## Lock

- HAL remains on `META`.
- Clouseau remains on `PLEB`.
- `PLEB` and `META` remain described, not executed.
- Hollow Grove does not know Current Synthesis exists.
- Flow remains one-way.
