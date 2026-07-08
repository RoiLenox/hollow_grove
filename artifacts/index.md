# Artifact Index

## Boundary

`Symptom -> Triway -> HollowGrove -> GroveSeam -> HollowBeam -> landed Symptom 2 -> KernelPass -> Client Artifacts -> Current Synthesis -> Hueman`

Upper layers consume lower layers without rewriting lower ownership.

## Entry Points

- `hollow-grove` prints the canonical witness by default.
- `hollow-grove runtime ...` drives the runtime loop and refreshes pipeline artifacts.
- `hollow-grove bridge ...` applies or previews bridge actions against Niri.
- `hollow-grove desktop ...` runs the runtime loop with the Niri bridge attached.
- `hollow-grove benchmark ...` benchmarks the full downstream route and writes a Current-Synthesis-style report.
- `run-runtime.sh` and `run-runtime-niri.sh` wrap the same integrated paths for shell use.

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

### `artifacts/runtime_input.txt`

- What it is: the runtime input contract that tells the loop to `run`, `hold`, or `stop`.
- Produced by: operator or `hollow_grove_runtime` template generation.
- Consumes: runtime loop only.
- Deterministic: no.
- Feeds back into the kernel: indirectly, through the runtime loop.

### `artifacts/runtime_memory.txt`

- What it is: the runtime memory contract that persists the last accepted cycle, mode, action, and visible witness across loop ticks and restarts.
- Produced by: `hollow_grove_runtime`.
- Consumes: runtime loop only.
- Deterministic: no.
- Feeds back into the kernel: indirectly, through the runtime loop.

### `artifacts/runtime_loop_status.md`

- What it is: the runtime-facing status artifact for the last loop tick, including the selected input mode and last visible witness.
- Produced by: `hollow_grove_runtime`.
- Consumes: runtime loop state for one tick.
- Deterministic: no.
- Feeds back into the kernel: no.

### `artifacts/niri_bridge_memory.txt`

- What it is: the niri bridge memory contract that records the last runtime mode and safe niri action already aligned by the bridge.
- Produced by: `hollow_grove_niri_bridge`.
- Consumes: niri bridge only.
- Deterministic: no.
- Feeds back into the kernel: no.

### `artifacts/niri_bridge_status.md`

- What it is: the current bridge status artifact showing the observed runtime memory, desired niri action, and whether the bridge applied, skipped, or is waiting.
- Produced by: `hollow_grove_niri_bridge`.
- Consumes: runtime memory and bridge memory.
- Deterministic: no.
- Feeds back into the kernel: no.

### `artifacts/current_synthesis_benchmark.md`

- What it is: a Current-Synthesis-style benchmark report covering kernel, clients, Current Synthesis, and Hueman stage timings.
- Produced by: `current_synthesis_benchmark`.
- Consumes: local benchmark samples of the full downstream route.
- Deterministic: no.
- Feeds back into the kernel: no.

### `artifacts/current_synthesis_benchmark.json`

- What it is: a machine-readable benchmark snapshot with per-stage timing stats, group load, and weak-point summaries.
- Produced by: `current_synthesis_benchmark`.
- Consumes: local benchmark samples of the full downstream route.
- Deterministic: no.
- Feeds back into the kernel: no.

### `artifacts/current_synthesis_benchmark_release.md`

- What it is: a release-facing one-page benchmark summary with headline claim, gate checks, and the main weak points that still block stronger messaging.
- Produced by: `current_synthesis_benchmark`.
- Consumes: the full benchmark report and snapshot.
- Deterministic: no.
- Feeds back into the kernel: no.

### `artifacts/hueman_stonebend_roles.md`

- What it is: the Hueman civic-role artifact for Stonebend, defining the equal-power triad of Proliteriate, Hypergiant, and Freemason, with Hypergiant as the public face.
- Produced by: `hueman_stonebend_roles` and the integrated Hueman/runtime routes.
- Consumes: `hueman_start_choices` and `hueman_fourway`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/hueman_tross_helpers.md`

- What it is: the Hueman Flynt-anchored helper-line artifact for Tross, defining Juvenile as the North head, Delinquent as the South end, and the four White Dwarfs as Tross's personal guard.
- Produced by: `hueman_tross_helpers` and the integrated Hueman/runtime routes.
- Consumes: `hueman_start_choices` and `hueman_fourway`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/hueman_glaushouse_roles.md`

- What it is: the Hueman south-facing kingdom-role artifact for Glaushouse, defining Prima Donna, Persephone, and the Nightengales.
- Produced by: `hueman_glaushouse_roles` and the integrated Hueman/runtime routes.
- Consumes: `hueman_start_choices` and `hueman_fourway`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/hueman_sandmanor_roles.md`

- What it is: the Hueman Sandmanor kingdom-role artifact defining southern Minoans, northern Minorians, and the crowd-judged Sandman contest for rule.
- Produced by: `hueman_sandmanor_roles` and the integrated Hueman/runtime routes.
- Consumes: `hueman_start_choices` and `hueman_fourway`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/hueman_procedural_uplift.md`

- What it is: the Hueman procedural-uplift artifact that maps Current Synthesis execution, behavior, transition, selection, and consequence contracts into kingdom-facing procedures for Stonebend, Flynt, Glaushouse, and Sandmanor.
- Produced by: `hueman_procedural_uplift` and the integrated Hueman/runtime routes.
- Consumes: Current Synthesis execution contracts plus Hueman kingdom-role artifacts.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/hueman_inverse_circle.md`

- What it is: the Hueman underground interior mirror-ring artifact for the secret tunnel sequence of Stairway to Heaven, Riptide, Current Sea, and Aura Way.
- Produced by: `hueman_inverse_circle` and the integrated Hueman/runtime routes.
- Consumes: `hueman_fourway` and `hueman_link_physics`.
- Deterministic: yes.
- Feeds back into the kernel: no.

### `artifacts/vertical_integration_stack.md`

- What it is: the generated full-stack alignment artifact showing how kernel recursion, KernelPass, client artifacts, Current Synthesis, and Hueman sit in one downstream chain.
- Produced by: the integrated Hueman/runtime routes.
- Consumes: `current_synthesis_base`, `hueman_boundary`, `hueman_scene_presence`, `hueman_scene_intent`, and `hueman_scene_drift`.
- Deterministic: yes.
- Feeds back into the kernel: no.
