# Hollow Grove Core v1.0.0

Date: 2026-07-06

## Core

### Kernel

```text
Symptom 1
↓
Triway
↓
HollowGrove
↓
Bond + Atmosphere
↓
GroveSeam
↓
HollowBeam
↓
Symptom 2
```

### KernelPass

Canonical deterministic witness of one completed recursion.
`Symptom` is now the vertical kernel type from start to landing.
`Point` remains the inner anchor carried within each `Symptom`.
`GroveSeam` and `HollowBeam` belong to the Hollow Grove layer beneath Hueman.
Hueman reads `HollowBeam` as `Aura Beam`; it does not create it.

### Consumers

- `consumer_prompt`
- `consumer_tree`
- `consumer_export`

### Clients

- `client_snapshot`
- `client_prompt_artifact`
- `client_desktop_status`

### Artifacts

- `kernel_pass_snapshot.json`
- `consumer_prompt.md`
- `desktop_status.txt`
- `artifacts/index.md`

### Interpretive Artifact

- `ai_interpretation.md`

### Operational Entry Points

- `hollow-grove` prints the canonical witness by default.
- `hollow-grove runtime ...` runs the integrated runtime loop.
- `hollow-grove bridge ...` runs the Niri bridge against runtime memory.
- `hollow-grove desktop ...` launches the runtime loop with the Niri bridge attached.
- `hollow-grove benchmark ...` benchmarks the full downstream route and writes a benchmark artifact set.
- the benchmark artifact set includes an engineering report, a JSON snapshot, and a release-facing summary.
- `run-runtime.sh` and `run-runtime-niri.sh` are shell wrappers around the same integrated entrypoints.

## Conclusions

1. The recursive kernel is complete as a stable foundation.
2. The kernel boundary is proven.
3. Multiple consumers can consume `KernelPass` without kernel changes.
4. Clients produce deterministic artifacts outside the kernel.
5. Desktop and AI boundaries are proven as read-only.
6. Interpretation remains outside the kernel.
7. Future work belongs to applications built on Hollow Grove Core, not to expanding the kernel without necessity.
