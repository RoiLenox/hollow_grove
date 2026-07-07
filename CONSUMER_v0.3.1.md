# Consumer v0.3.1

Date: 2026-07-06

## Boundary Proof

```text
Kernel
↓
KernelPass
├─ consumer_prompt
├─ consumer_tree
└─ consumer_export
```

## Validation

- Kernel code did not change.
- `KernelPass` did not change.
- All Consumers read the same `KernelPass`.
- Consumers only transform presentation.
- Flow remains one-way.
- No Consumer feeds back into the kernel.
- Tests passed with 11/11.
