# AI Prompt v0.6.1

Date: 2026-07-06

## Boundary

```text
Kernel
↓
KernelPass
↓
Client
↓
Artifact
↓
AI Prompt
```

## Current Implementation

- `client_prompt_artifact`
- `artifacts/consumer_prompt.md`
- `artifacts/kernel_pass_snapshot.json`

## State

- AI prompt is local-only.
- No AI API calls yet.
- AI prompt consumes artifacts only.
- Kernel is unchanged.
- `KernelPass` is unchanged.
- `niri`/`river` configs are untouched.
- Flow remains one-way.
- Do not mutate the kernel. Interpret only.
