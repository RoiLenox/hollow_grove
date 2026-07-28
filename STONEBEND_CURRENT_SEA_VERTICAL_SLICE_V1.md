# Stonebend Current Sea Vertical Slice V1

Status: implemented playable witness
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
Normative law: `STONEBEND_CONSTITUTION_V2.md`
Runtime: `src/gameplay/stonebend.rs`

## Case

At the Current Sea Many-Witness Concourse, **Mercy Deep** returns from a
Glaüshouse restoration with one persistent Being continuity and a materially
changed form. The route is a sea of people rather than a body of water:
ordinary contact, old acquaintances, officials, strangers, and competing
claims put continuity under public pressure. Clinical success establishes
neither identity nor failure. Stonebend must decide what Name may lawfully act
while preserving every uncertainty the evidence cannot settle.

The case is `case.stonebend.current-sea-continuity.v1`. Mercy Deep is
`being.current-sea.mercy-deep`; this is additive case canon, not an office,
Title, transformation class, or inferred regional role.

## Functional-Lore Contract

1. Stable identity: the case, Being, evidence, decision, Name, Seal, outcome,
   map, interactions, commands, and events all have stable IDs.
2. Authority: Stonebend `Name` /
   `ConstitutionalIdentity`; the active Hypergiant is resolved from the live
   `WorldSession`.
3. Place: Current Sea Many-Witness Concourse at the
   Glaüshouse–Stonebend boundary.
4. Entities: Mercy Deep, Gerald registrar, Glaüshouse restoration archive,
   crowd witness, Name ledger, Mercury Mirror, and Freemason Seal.
5. Dominant verbs: `Stonebend names`; Current Sea `Certify`.
6. Trigger: restoration preserves life and memory while materially changing
   form.
7. Evidence: capable subject testimony, prior Name, restoration record, public
   circulation witness, and Mercury Mirror correspondence. Partial continuity
   and the legal meaning of clinical success remain uncertain.
8. Player choice: advisory support for an existing Name, a provisional
   transformed-form Name, or high identity review.
9. State change: one typed outcome and exact Stonebend records.
10. Persistence: gameplay archive schema V3 stores the event, institutional
    snapshot, and explicit Runtime Federation binding; replay reconstructs and
    compares the exact outcome and authority.
11. Presentation: map gate, silhouette, dialogue, registry view, and refusal
    language change after commitment.
12. Failure/refusal: missing authority fails closed; uncertainty does not erase
    Mercy Deep; challenge remains open; no outcome grants Title.

## Evidence and Faculties

The case requires all five addressable evidence sources and all five Hueman
faculties. Faculties disclose a player-visible reading of the record; they
cannot decide for Mercy Deep or manufacture Stonebend authority.

The Mercury Mirror tests correspondence only. It cannot name, grant Title,
prove clinical safety, recognize function, or choose an outcome.

The serialized map, interaction, and evidence identifiers retain historical
`deep` and `depth-witness` wording for schema-V2/V3 replay compatibility. Those
wire names do not override the Many-Witness Concourse presentation or the
current civic meaning of Current Sea.

## Outcomes

### Existing continuity sealed

`outcome.stonebend.existing-continuity-sealed.v1`

The active personal Name **Mercy Deep** is sealed in the Current Sea civic
scope. The decision does not claim that every bodily feature remained
unchanged.

### Provisional transformed-form Name

`outcome.stonebend.provisional-transformed-form-name.v1`

The active former Name **Mercy Deep** remains in history and the nonexclusive
provisional transformed-form Name **Mercy Deep, Aftertide** is linked to it.
Provisional status cannot grant Title and cannot overwrite the former Name.

### Identity conflict referred

`outcome.stonebend.identity-conflict-referred.v1`

Evidence and uncertainty are sealed for high review. No final Name record is
created by the case. Delay cannot become disappearance, substitution, forced
renaming, or adverse Title inference.

## Exact Authority Boundary

The runtime creates typed Stonebend `EvidenceRecord`, `DecisionRecord`,
`NameRecord`, and `SealRecord` values. The live Hypergiant supplies the exact
House `Name` decision. The High Freemason issues the durable Seal. Presentation
receives immutable strings and outcome projections only.

The case categorically preserves:

- Name is not proof;
- restoration is not identity authority;
- a Mirror is evidence, not authority;
- Flynt recognition is not Stonebend Title;
- transformation is not accession;
- support is not the final decision;
- uncertainty is not erasure.

## Verification

`tests/gameplay_stonebend.rs` covers all three outcomes, exact typed records,
former-Name linkage, no-Title behavior, live non-fixture authority, fail-closed
vacancy, map projection, dialogue, archive checksum/replay, and the Godot
authority boundary.
