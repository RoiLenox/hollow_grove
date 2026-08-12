# Functional Lore Archive V2 Migration

Status: applied additive migration
Source: functional-lore archive schema V1
Target: functional-lore archive schema V2

## Change

V2 adds the operational role envelope to each functional-lore record:

- affected actor;
- decision owner;
- executor;
- witness;
- ordered handoffs;
- completion condition;
- Restitution owner;
- final recipient.

The twelve original Compromise fields remain unchanged. No stable House,
route, authority, or recursion-kernel identity changes.

## V1 migration

V1 records are accepted only after their checksum is verified against the exact
schema-V1 payload shape. New V2 fields are not defaulted into that payload
before verification. Only then does migration locate the current canonical
definition by the record's stable lore identity and copy its explicit,
scenario-specific role envelope. An unknown identity or any mismatch with the
canonical definition fails closed.

Migration never assigns a role from entity-list position and does not infer a
new authority, decision, consent state, or handoff.

## Verification

V2 replay compares the complete role envelope. Missing, reordered, or altered
roles fail closed as replay divergence. The pure recursion kernel remains
untouched. A test fixture encoded with the authentic V1 payload shape proves
that checksum verification precedes role migration.
