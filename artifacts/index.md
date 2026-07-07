# Artifact Index

## Boundary

`Kernel -> KernelPass -> Client -> Artifact`

## Artifacts

### `artifacts/kernel_pass_snapshot.json`

- What it is: a structured snapshot of one completed `KernelPass`.
- Produced by: `client_snapshot`.
- Consumes: `KernelPass`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/consumer_prompt.md`

- What it is: a local AI-client-ready prompt artifact with the canonical witness, snapshot reference, inverse-path question, and boundary reminder.
- Produced by: `client_prompt_artifact`.
- Consumes: `KernelPass`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/desktop_status.txt`

- What it is: a read-only desktop-facing status artifact with the canonical witness.
- Produced by: `client_desktop_status`.
- Consumes: `KernelPass`.
- Deterministic: yes.
- Feeds back into the kernel: no.
