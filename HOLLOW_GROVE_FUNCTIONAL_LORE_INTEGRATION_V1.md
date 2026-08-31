# Hollow Grove Functional Lore Integration V1

Status: executable integration contract
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
House authority: the four locked V2 House constitutions
Runtime owner: `src/world/lived_lore.rs`

## Purpose

This layer turns the Compromise's functional-lore rule into a validated world
contract. It does not add another constitutional reducer. It binds ordinary
places and conflicts to existing House authority so lore can become playable,
fail lawfully, persist, and replay.

`HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md` supplies the operational
role envelope layered over the twelve-field contract: affected actor, decision
owner, executor, witness, ordered handoffs, completion, Restitution owner, and
recipient. Functional-lore archive schema V2 carries these fields, while schema
V1 records first verify against their exact V1 payload shape and then receive
the explicit current role envelope selected by stable lore identity. Migration
never assigns roles from entity-list position or invents a new decision.

The causal boundary remains:

```text
frozen recursion kernel
→ constitutional facts and House authority
→ functional-lore record
→ gameplay event and state change
→ presentation
```

Nothing in this layer feeds a cultural title, route, dialogue choice, office
holder, or presentation result back into universal recursion.

## Required Record

Every functional-lore definition supplies all twelve Compromise fields:

1. stable identity;
2. exact House function and required authority class;
3. route, location, and jurisdiction;
4. involved entities;
5. the dominant House verb;
6. the trigger;
7. evidence and explicit uncertainty;
8. at least two player-visible choices;
9. one positionally paired lawful state change per choice;
10. persistence and replay behavior;
11. presentation behavior;
12. failure and refusal behavior.

Validation rejects duplicate or malformed identity, House substitution,
incorrect authority class, incorrect dominant verb, route jurisdiction outside
the House boundary, missing contract fields, fewer than three loops per House,
or incomplete coverage of the ten constitutional routes.

Instantiation then resolves a real `HouseDecision` from the supplied
`WorldSession`. A missing or inactive office holder fails closed. The catalog
does not manufacture a holder and does not fall back to a test fixture.

## Live Authority State

`artifacts/institutional_state.txt` is schema V2 dynamic world state. It
currently records role-addressed incumbency for:

- `being.stonebend.current-hypergiant`;
- `being.sandmanor.current-sandman`;
- `being.glaushouse.current-prima-donna`.

Flynt's active Tross remains the locked canonical
`being.flynt.tross`. The three dynamic IDs identify current office-bearing
institutional actors for runtime authority and replay. They are not personal
names, visual character designs, succession rules, or permission to invent
biography. Replacing an incumbent requires a lawful House-specific accession
and a new persisted office-holder record.

Schema V1 institutional records remain loadable but never infer an office from
membership, a legacy role string, transformation, or presentation.

## Twelve Lived Loops

| Stable identity | House act | Route | Core conflict |
|---|---|---|---|
| `lore.flynt.boardwalk-return-recognition` | Recognize | Boardwalk | return with affiliation, scoped work, or protected independence |
| `lore.flynt.stairway-functional-recognition` | Recognize | Stairway to Heaven | performed function versus office or Title |
| `lore.flynt.basin-dual-expression` | Recognize | Basin Motor Speedway | urban and rural Flynt expression without duplicate sovereignty |
| `lore.stonebend.current-sea-continuity` | Name | Current Sea | restored form and preserved identity continuity |
| `lore.stonebend.aura-ridge-public-name` | Name | Aura Ridge | provisional civic renaming with conflict and history |
| `lore.stonebend.mnt-aura-illegal-hollowing` | Name | Mt. Aura | subject identity, extracted Hollow, provenance, and protected refusal |
| `lore.sandmanor.aura-way-public-design` | Prove | Aura Way | Minorian interior and Minoan exterior improvement |
| `lore.sandmanor.glausbahn-recovery-design` | Prove | Glausbahn | failed field demonstration, clinical stop, and revision |
| `lore.sandmanor.aura-beach-current-sea-role-proof` | Prove | Aura Way review | regional role proof without automatic Synthesis |
| `lore.glaushouse.riptide-emergency-intake` | Clear | Riptide | narrow emergency stabilization without manufactured consent |
| `lore.glaushouse.current-seanad-high-risk-clearance` | Clear | Current Seanad | proven design versus consent-bound high-risk procedure |
| `lore.glaushouse.aura-ridge-recovery-discharge` | Clear | Aura Ridge | recovery, restricted transfer, and refusal of premature duty |

Together the definitions use every constitutional route at least once:
Boardwalk, Riptide, Current Sea, Aura Ridge, Glausbahn, Current Seanad, Aura
Way, Mt. Aura, Basin Motor Speedway, and Stairway to Heaven.

## House Depth

### Flynt

Flynt recognizes actual function, return, and institution. Recognition never
becomes Stonebend Title, Sandmanor proof, Glaüshouse Clearance, automatic
membership, or duplicate Tross sovereignty. Urban and rural expressions stay
distinct and equally Flynt.

### Stonebend

Stonebend gives stable reference to citizens, restored forms, claims, material,
and evidence continuity. A new or provisional Name preserves former history.
Mercury Mirror evidence tests correspondence but creates no authority.
Extracted Hollow receives distinct provenance and custody without becoming the
subject or the custodian's property.

### Sandmanor

Sandmanor preserves method, measurement, criticism, failed demonstration,
revision, intellectual lineage, and proof scope. Negative results remain
visible. A successor version does not silently inherit proof. Proof of a
regional role never grants consent, Clearance, Title, recognition, or office.

### Glaüshouse

Glaüshouse separates diagnosis, capacity, consent, Clearance, intervention,
recovery, and discharge. Riptide urgency authorizes only necessary emergency
scope. A Nightingale stop remains immediately binding and reviewable. Recovery
does not end when a technical procedure ends, and economic dependence never
manufactures consent.

## Persistence and Replay

`FunctionalLoreCatalog::encode` emits schema-V2 JSON containing:

- every functional-lore field;
- the observation position;
- exact authority decision ID;
- exact active authority actor and office;
- an FNV-1a checksum over the canonical payload.

Schema V2 additionally persists the operational role envelope and ordered
handoffs. Replay requires one affected actor, decision owner, executor, witness,
recipient, completion condition, Restitution owner, and at least two explicit
handoffs. The decision owner must be the live House office, and the handoff
chain must address the decision owner, executor, witness, and recipient. Schema
V1 remains readable through an explicit, stable-identity role migration after
its original payload checksum succeeds.

`FunctionalLoreCatalog::replay` verifies format, schema, checksum, the complete
canonical definition set, and the live authority snapshot. Altered prose,
choices, transitions, jurisdiction, authority, or ordering causes replay
divergence rather than silent acceptance.

Gameplay archives use Rampart-aware schema V3. They embed the exact
schema-V2 institutional state needed to replay their House decisions and an
explicit binding to The Rampart of Roi and Roselina. Schema V1 and V2 gameplay archives
migrate through recorded decisions without inventing events; new gameplay
never loads the historical fixture actors.

## Boardwalk as First Playable Witness

The Returning Goon case is the first functional-lore record projected all the
way through a game client. Every committed option has:

- Glaüshouse discharge Clearance;
- six addressable observations and five disclosed Hueman faculties;
- nonbinding player support;
- commitment by the Returning Goon;
- Flynt recognition;
- a stable outcome identity;
- persistent uncertainty and refusal protections;
- a presentation consequence.

Pimp Patronage, Goon Bond, and Limited Cooperation each form a distinct finite
common constitutional Bond through Stonebend Name, Sandmanor proof, and
`Formed → Validated → Activated`. Independent Return forms no Bond and
durably records self-direction and refusal without punishment.

Formal Pimp patronage does not establish Hoe identity, sexual consent,
ownership, or permanent affiliation. The open constitutional question about
formal Hoe affiliation therefore remains open.

## Current Sea as Second Playable Witness

The Stonebend Current Sea continuity case is the second functional-lore record
projected through the client. Mercy Deep supplies capable subject testimony
alongside a prior Name, Glaüshouse restoration record, crowd witness, Mercury
Mirror comparison, and all five Hueman faculties. The Current Sea setting is
the Many-Witness Concourse surrounding the distinct Current body: dense public
contact tests continuity, while the civic crowd and literal Current Sea remain
separate but interacting layers.

The three outcomes affirm the existing Name, link a provisional
transformed-form Name without erasure, or refer the conflict while allowing no
final Name to act. Each outcome carries a live Hypergiant `Name` decision,
typed Stonebend evidence and decision records, and a High Freemason Seal. None
creates Title, proof, Clearance, recognition, office, or accession.

## Verification

`tests/functional_lore_integration.rs` proves complete House and route coverage,
all twelve fields, explicit operational roles and handoffs, exact authority
classes, no House substitution, non-fixture live authority, archive replay,
checksum rejection, fail-closed vacant offices, and absence of kernel imports.
The runtime unit suite also replays an authentic schema-V1 fixture to prove
checksum verification occurs before role-envelope migration.

`tests/gameplay_boardwalk.rs` proves all four outcome types, all three active
finite Bonds, Independent Return without a Bond, non-ownership, subject
attribution, replay, archive authority persistence, collision, dialogue, and
Godot's presentation boundary.

`tests/gameplay_stonebend.rs` proves all three continuity outcomes, exact Name
history, Mirror non-authority, non-fixture Hypergiant authority, High Freemason
Seal, no-Title behavior, vacancy failure, map consequence, dialogue, archive
replay, and Godot's presentation boundary.

The live witness is inspectable without a presentation client:

```bash
cargo run --quiet --features dev-tools --bin hg-dev -- hollow_grove_functional_lore_audit
cargo run --quiet --features dev-tools --bin hg-dev -- hollow_grove_functional_lore_audit --json
```
