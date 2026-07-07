# Consumer v0.2.1

Date: 2026-07-06

## Boundary

Consumer lives outside the kernel.

Flow is one-way:

```text
Kernel
↓
KernelPass
↓
Consumer
```

Never:

```text
Consumer
↓
Kernel
```

## Role

Consumer may interpret, explain, visualize, or transform the witnessed pass for an external purpose.

## Lock

Consumer must not mutate, redefine, or feed back into:

- `Point`
- `Triway`
- `HollowGrove`
- `Bond`
- `Atmosphere`
- `CurrentSeam`
- `AuraBeam`
- `KernelPass`
