# Linux Kernel and Runtime Policy

## Decision

Keep the active CachyOS BORE kernel as the development baseline for the
foundation RC. Hollow Grove has no measured reason to maintain a project-owned
kernel, scheduler patch, global sysctl profile, CPU isolation setup, or fixed
GPU/CPU clock policy. Those would add upgrade and laptop portability risk before
the art workload exists.

The application-level "kernel" is instead split into the dependency-free
recursion crate and the semantic geometry/composition crate. Linux remains a
measured deployment environment, not part of Hollow Grove's constitutional
authority.

## Current workstation baseline

At foundation freeze the workstation runs `7.1.6-1-cachyos-bore` with dynamic
preemption on an AMD Ryzen 7 9800X3D. AMD P-State is active, the exposed scaling
governor is `powersave`, and the energy policy is `balance_performance`. That
combination is normal for an active P-State policy and is recorded rather than
rewritten by the project.

## Comparison protocol

Use `./benchmark-linux-kernel.sh` after booting each candidate kernel. Keep the
same firmware, power profile, compositor session, Rust commit, warmup, sample
count, and background workload. Record at least three runs per kernel and
compare medians plus p95/max behavior; do not select a kernel from one run or a
single average.

The first useful comparison set is:

1. the current CachyOS BORE kernel;
2. the distribution's ordinary supported kernel;
3. the distribution's LTS kernel.

A kernel change needs a repeatable improvement in end-to-end frame pacing,
input-to-presentation latency, build/test time, or sustained simulation p95
without regressions in suspend, audio, networking, controllers, or the older
laptop. Microbenchmark noise alone is not enough.

## Deferred until real art exists

Once representative tilesets, animation, shaders, audio, and loading patterns
are present, add Godot frame-time traces, asset import/export time, peak RSS,
VRAM pressure, and save/replay latency to the same run record. Until then,
kernel or driver tuning would optimize a placeholder renderer rather than the
game being shipped.

No global tuning is installed by the Hueman or General Edition packages. Each
machine keeps its distribution kernel, power policy, and hardware discovery.
