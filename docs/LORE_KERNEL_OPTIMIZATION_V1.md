# Lore Kernel Optimization V1

Date: 2026-10-07

Status: implemented first milestone; additive validation and read-only inspection

This document records the original milestone. The later
[Stonebend continuity binding](STONEBEND_CONTINUITY_LORE_BINDING_V1.md) adds
read-only inspection V3 for the existing Current Sea case; current coverage
has three verified bindings including the additional Sympiote foundation,
with ten unbound historical routes. Measurements and original verification
counts below remain historical.

Authority: implements existing canon beneath the Compromise and Foundation Freeze

## Result

The Hollow Deck now agrees with the ratified Flynt order: combined Manticorp
then perfected Chimera. All twelve functional-lore definitions resolve their
declared handoffs by exact role endpoints. The Boardwalk return definition has
an explicit read-only link to its four actual gameplay outcomes, evidence,
House decisions, affected actor commitment, and ordered event witnesses.

Inspect definitions:

```text
hollow-grove lore
hollow-grove lore --json
```

Inspect an existing gameplay save:

```text
hollow-grove lore --save path/to/save.json
hollow-grove lore --save path/to/save.json --json
```

The CLI verifies the save checksum and reconstructs its runtime through the
existing replay path before projecting any execution. It reads the file
without writing it. A malformed archive produces an error and no success
projection. Source-tree invocation is `cargo run --offline --bin hollow-grove
-- lore` followed by the same options.

## Canon and semantic migration

The controlling amendment is
[Flynt Synthesis and Sandmanor Hands](../HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md).
`FlyntSynthesisState` now exposes combined/perfected projections consumed by
both the Deck seed and validator. Higher-synthesis rules carry their source
document, and their mechanically OPEN status remains unchanged.

The Deck catalog version advances from `1.5.0` to `1.6.0` to identify this
semantic correction. Its reciprocal Nature pair and stat budgets are
unchanged. Tross-specific Manticore identity and the unique First Companion
remain separate from general synthesis attainment. Hueman's live copy no
longer presents Manticore as a general tier after Chimera. The Deck's formal
service diagram takes its combined and perfected names from the validated
catalog, and Hueman's constitution, presence, and procedure projections follow
the same amended order.

Persisted Deck state remains V1. Existing `manticorp` and `chimera` cultivation
keys keep their exact values and meanings as Nature references. No recipe,
discovery, authority, or advancement is inferred from the new ordering. The
V1 payload/checksum compatibility test exercises both keys and compares the
whole envelope before and after decoding and re-encoding.

## Authored responsibilities

Each current catalog definition declares the chain:

```text
decision owner → executor → witness → recipient
```

`FunctionalLoreDefinition::operational_handoffs` parses the existing V2 strings
into typed capacity references, scoped by stable lore definition ID. It checks
three exact edges and a nonempty question for each. Mentioning a required role
inside unrelated prose cannot satisfy the contract. Reordered edges,
disconnected receivers, and similarly named impostors fail closed.

`src/kernel_invariants.rs::audit_functional_lore_definitions` exposes this as a
scoped invariant report, including identity, jurisdiction, and coverage checks.
The inspection command requires that report to pass before producing output.

These references identify authored capacities, not invented incarnations or
office holders. A bearer may occupy multiple lawful capacities; for example,
the carrier may also receive the final custody receipt. Runtime actor identity
must come from a case's recorded binding rather than a role's display label.

This strengthens catalog validation around the existing twelve definitions;
it does not prescribe one universal sequence of House gates. A later catalog
with an expanded handoff topology must introduce its explicit contract rather
than smuggling new stages into prose.

## Boardwalk execution

The binding is `lore.flynt.boardwalk-return-recognition` to
`case.boardwalk.returning-goon.v1`. The case retains four choices; the catalog
retains three descriptions:

| Gameplay choice | Catalog choice |
|---|---|
| Pimp patronage | seek finite affiliation |
| Goon Bond | seek finite affiliation |
| Limited cooperation | accept scoped work |
| Independent return | return independently |

Before commitment, inspection lists missing evidence, faculty disclosure,
available authority, and the affected actor's pending decision. Player support
alone is incomplete and emits no executed House decisions in the inspection.

Completion requires the reducer's actual commitment event to agree with the
case subject, outcome, and optional relationship. The projection exposes the
recorded Glaüshouse Clearance and Flynt recognition. A formed relationship also
links Stonebend naming, Sandmanor proof, and ordered formation/validation/
activation events. Independent return requires no relationship and remains a
complete, protected outcome.

Definition handoffs and execution witnesses are different report fields.
The report never fabricates a separate steward delivery or witness receipt
that the current event journal does not record. Completion here attests the
implemented case outcome; it does not assert that every authored handoff has
its own executed receipt. Such a receipt requires a future versioned event
contract, not an inference from a catalog label.

All twelve definitions receive typed handoff inspection and House source
references. Eleven remain explicitly without a verified gameplay binding in
this inspection; this is a coverage limit, not a claim that equivalent content
does not exist. Additional bindings must prove equivalence and negative paths
before they may report execution.

## Foundation and archive compatibility

This is the versioned additive validation record under
[Foundation Freeze V1](../FOUNDATION_FREEZE_V1.md). House jurisdiction, actor
identity, consent/refusal, causal ordering, Rampart binding, neutral kernels,
and Rust/Godot ownership are unchanged. Inspection accepts immutable runtime
references and has no command/commit interface.

Functional-lore V1/V2 wire payloads remain unchanged. Their existing checksum
and exact replay paths still apply; the authentic schema-V1 fixture verifies
its checksum before role normalization. The three-edge declarations were
already present in every adopted definition, so no historical payload rewrite
is needed. That milestone retained gameplay archive V4 and protocol V1 shapes.
Its original inspection JSON was a separate, read-only schema V1 and cannot be loaded
as a gameplay archive or used as constitutional evidence by itself.

The later [Sympiote Foundation](../HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md)
adds a distinct clinical/practice contract and replay-backed inspection under
read-only lore schema V2. Gameplay now writes V5 through the
[explicit V4→V5 migration](SYMPIOTE_FOUNDATION_MIGRATION_V1.md); V1–V4 remain
readable without inferred foundation progress. The historical twelve-route
catalog and functional-lore V1/V2 payloads remain unchanged.

## Lore development

[Stonebend PrimePunk Object Lineage](../STONEBEND_PRIMEPUNK_OBJECT_LINEAGE_V1_DRAFT.md)
is the next WORKING design case. It follows one repairable survey frame through
the eight existing practical verbs. It creates no wire identity, Recipe,
arsenal lock, authority, or additional growth rule.

## Verification

Behavioral coverage includes rejection of the superseded Flynt order, exact V1
Deck payload preservation, malformed handoffs, all four Boardwalk outcomes,
nonbinding support, missing evidence and office holders, read-only inspection,
save/replay equality, and tampered-save rejection.

| Check | Result |
|---|---|
| `cargo test --workspace --all-features --locked --offline` | 1,653 tests passed across 35 targets; zero failures or ignored tests |
| `cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings` | passed |
| `cargo fmt --all -- --check` and `git diff --check` | passed |
| `./verify-art-assets.sh` | passed: two assets, sixteen colors |
| `./verify-godot.sh` | runtime client and retro overworld semantics smoke checks passed |
| `packaging/hollow-grove-general/verify-keyboard-grove.sh` | passed |
| `./verify-playable-lifecycle.sh` | save, restart, and load phases passed |
| CLI text/JSON inspection | twelve definitions, thirty-six exact handoffs, one gameplay binding; no fabricated execution without a save |

The live lifecycle check ran outside the restricted sandbox after its localhost
bind was blocked. Godot emitted a local X509 certificate-loader diagnostic;
renderer and lifecycle assertions completed successfully.

## Two-machine checkpoint

The initial 2026-10-07 local readiness audit found uncommitted source changes and
release/state archives built from an older commit. The main PC is offline;
no SSH connection or transfer was attempted.

The checkpoint preparation workflow and generated acceptance receipt are
documented in [Lore Sync Checkpoint](LORE_SYNC_CHECKPOINT_V1.md).

Before transfer, commit the complete milestone, including the new lore
inspection module, implementation notes, optimization plan, and Stonebend
draft. Then run `./hollow-grove-update.sh prepare` from the clean checkout to
build matching source, release, and state payloads. The existing archives must
be rebuilt before this milestone can be transferred.

When the main PC is available, inspect its checkout and saves before choosing
the sending machine. Confirm the SSH target and active checkout path, run
`status`, then `stage`, review the migration record, and only then `activate`.
The sender supplies the selected source checkpoint and durable state; saves
are not merged automatically. Verify matching source commits and release
manifests on both machines after activation. See
[Laptop Migration](LAPTOP_MIGRATION.md) for the commands and recovery behavior.

This checkpoint includes the adopted Boardwalk occupant organization and the
implemented first lore milestone. Stonebend's object lineage remains a WORKING
draft, and eleven other lore definitions still await verified gameplay
bindings; those are future development rather than completed gameplay claims.
