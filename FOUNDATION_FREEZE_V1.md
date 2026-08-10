# Hollow Grove Foundation Freeze V1

Status: active foundation lock for the art phase  
Scope: everything beneath gameplay presentation

The foundation is now considered stable enough for an art-focused phase. The
following contracts are frozen unless a change is explicitly marked as a
foundation migration:

- the universal kernel and constitutional runtime authority boundary;
- the four House jurisdictions and Central Junction's coordination-only role;
- one Federation and exactly two continuous witnesses, Roi and Roselina, known
  in-world as the Twofold Witness;
- the continuity/embodiment witness seam, form transitions, and operational
  session records (the legacy technical API may still use `Guide` names);
- typed evidence, advisory, decision, refusal, provenance, and consequence
  boundaries;
- Federation binding, archive checksums, schema migrations, and replay rules;
- the Platinum Path (`path.platinum.v1`), also called the Plath;
- the Platinum Path Cultural Canon's shared five tests and local definitions;
- the Rust/Godot protocol's rule that Godot presents records but never decides.

## What art may change

Art work may change sprites, tiles, palettes, animations, sound, layout,
dialogue presentation, accessibility copy, and other renderer-owned assets. It
must consume the existing stable IDs and projections rather than inventing
new authority, identity, House, Federation, or kernel state.

## What requires a foundation migration

Any change to a stable ID, archive schema, causal ordering rule, Guide
identity, House authority, refusal semantics, advisory commitment boundary,
Federation binding, or protocol ownership requires:

1. a versioned migration note;
2. a replay fixture before and after migration;
3. an invariant-audit update;
4. full Rust validation;
5. explicit approval before art-facing projections are changed.

## Required validation

```text
cargo fmt --check
cargo test --offline
godot --headless --path hueman_godot --script res://tests/retro_overworld_semantics_smoke.gd
git diff --check
```

The live restart/load lifecycle should additionally be run outside restricted
sandboxes with `./verify-playable-lifecycle.sh`.

The foundation's governing proof is the executable kernel audit in
`src/kernel_invariants.rs`. Its output is exposed as the read-only
`kernel_audit` gameplay projection, allowing art and presentation work to
surface foundation health without acquiring authority over it.
