# Stress v0.1.5

Date: 2026-07-06

## Environment

- Kernel version: `7.1.3-1-cachyos-bore`
- CPU: `AMD Ryzen 7 9800X3D`
- Release binary size: `455K`

## Verification

- `cargo fmt` passed
- `cargo test` passed
- `cargo build --release` passed

## Warmup

- Runs: `100`
- Failures: `0`
- Output mismatches: `0`
- Total elapsed: `74 ms`
- Average runtime: `743 us`
- Min runtime: `681 us`
- Max runtime: `917 us`

## Standard

- Runs: `10,000`
- Failures: `0`
- Output mismatches: `0`
- Total elapsed: `6223 ms`
- Average runtime: `622 us`
- Min runtime: `527 us`
- Max runtime: `1090 us`

## Heavy

- Runs: `100,000`
- Failures: `0`
- Output mismatches: `0`
- Total elapsed: `57754 ms`
- Average runtime: `577 us`
- Min runtime: `524 us`
- Max runtime: `1414 us`

## Canonical Witness

```text
start Point
↓
Triway
↓
HollowGrove
↓
CurrentSeam
↓
AuraBeam
↓
landed Point
```

## Notes

- Kernel code did not change for the stress proof.
- No git commands were used.
