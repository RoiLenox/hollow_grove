# Hollow Grove Deep Pressure Campaign V1

Status: implemented playable cross-region vertical slice
Stable identity: `case.arc.hollow-grove.deep-pressure.v1`
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
Runtime owner: `src/gameplay/deep_pressure.rs`

## Purpose

Deep Pressure turns the living Field, Beach, Basin, mines, and offshore wells
into one character-driven campaign without creating a generic quest engine or
moving authority into narrative presentation.

The initiating event is the Riptide well blowout. Its pressure passes through:

```text
Riptide crew exposure
→ Aura Beach contamination and rescue
→ Aura Basin injury, care, and attempted salvage
→ Mt. Aura stone and Highway deep-iron repair demand
→ Aura Field competition for public-works stone
→ Current Sea production certification
→ Boardwalk responsibility, restitution, recognition, refusal, and return
```

No arrow rewrites the act before it. A local duty officer still decides the
local case. Deep Pressure only links the committed result into later memory,
testimony, settlement, and aftermath.

## Functional Lore Contract

1. **Stable identity** — `case.arc.hollow-grove.deep-pressure.v1`.
2. **Authority class** — linked four-House public recovery settlement; local
   duty authority stays local.
3. **Location and jurisdiction** — Riptide, Aura Beach, Aura Basin, Mt. Aura,
   Highway to Hell, Aura Field, Current Sea, and final public return on the
   Boardwalk.
4. **Involved** — named crew, rescuers, Field workers, mine and rig workers,
   custody keepers, affected Beings, four House authorities, and Boardwalk
   witnesses.
5. **Dominant verbs** — retrieve, name, prove, clear, recognize, remember,
   repair, restitute, review, or refuse.
6. **Trigger** — loss of Riptide well control exposes crew and sends
   Current-bearing contamination toward the coast and Basin.
7. **Evidence and uncertainty** — twenty-one operational witnesses plus
   classified speech; each record retains its limit.
8. **Player-visible choice** — support shared burden, crew-and-coast
   restitution, production under review, or protected refusal.
9. **Lawful state change** — local outcomes, character condition, trust,
   promises, an optional finite Bond, four House acts, and regional aftermath.
10. **Persistence and replay** — journal entries, local links, support,
    settlement, constitutional events, memories, and aftermath replay from the
    ordered gameplay archive.
11. **Presentation** — scheduled people move by shift, occupy maps, block
    movement, speak in classified dialogue, and retain client-visible memory.
12. **Failure or refusal** — missing evidence fails closed; compromised
    evidence bars one ending; protected refusal forms no Bond and creates no
    debt; no path commodifies living blood.

The executable definition is `deep_pressure_functional_lore()`.

## Campaign Phases

### 1. Riptide Emergency

The pressure recorder, spill survey, and crew manifest establish the authored
blowout case. Corin Wake's crew is not an abstract resource counter.

The Riptide rescue controller may commit:

- `ShutInAndRetrieve` — stronger containment, quarantined recovered brine, and
  a recovering crew;
- `RescueCrewFirst` — stronger immediate crew protection, but the open
  containment obligation and downstream exposure persist.

`ContinueFlow` remains forbidden.

### 2. Shoreline Aftermath

Aura Beach decides between narrow shelter closure and bounded guided rescue.
Aura Basin separately decides care transfer or stabilization in place for a
living Being falsely claimed as salvage.

The campaign remembers who became exhausted, who accepted additional care
burden, and which transfer remains promised. Beach evidence cannot decide the
Basin case; Basin care cannot certify the well.

### 3. Burden of Repair

Three needs collide:

- Mt. Aura stone can reinforce Field irrigation and Beach dunes;
- Highway deep iron can restore damaged Basin Frames;
- withdrawal or flooding protects crews but leaves a material shortage.

Aura Field then allocates drought water under the actual repair outcome. A
crew's safe withdrawal is never narrated as cowardice or failed production.

### 4. Depth Certification

Current Sea may certify a reduced-rate well or suspend it for repair. The
certificate governs tested transfer only.

It does not certify:

- innocence;
- ownership;
- restitution;
- responsibility for the Riptide failure;
- blood taken from a living Being.

If Riptide chose `RescueCrewFirst` and Current Sea later chose
`CertifyReducedRate`, the campaign records a contested certificate. That
combination remains lawful local history, but its unresolved containment
evidence bars the `ProductionUnderReview` settlement ending. The player cannot
erase the contradiction by selecting a menu option.

### 5. Gather Affected Voices

Operational records are necessary but not sufficient. The final assembly
requires these twelve statements:

| Speaker | Classification | Functional claim |
|---|---|---|
| Brindle Reed | private belief | repair allocation may make the Field's next planting an unseen casualty |
| Sella Windward | public record | the pressure warning preceded the visible Riptide sheen |
| Harrow Vale | constitutional fact | a living Being belongs in care, never a salvage ledger |
| Oren Pike | public record | measured reinforcement or withdrawal are lawful; blasting is not |
| Maela Downroad | public record | repair need cannot erase a gas stop |
| Bram Burden | local tradition | every lifted burden owes a safe descent |
| Corin Wake | public record | rescue and shut-in remain distinct duties |
| Iona Depth | constitutional fact | certification is bounded and never covers living blood |
| Pel Marrow | public custody record | every lot keeps claimant, custodian, destination, and refusal |
| Tess Breakwater | rumor | dock talk about pre-failure certification is explicitly unverified |
| Boardwalk Pimp | deliberate deception | the Pimp knowingly claims certification erases restitution |
| Boardwalk Gimp | local tradition | shared burden requires a disclosed edge, term, and exit |

A character may be knowledgeable, limited, mistaken, or deceptive without
changing constitutional truth. Speech classification is stored in the
evidence journal and shown in dialogue.

### 6. Boardwalk Settlement

The player records nonbinding support. The affected assembly identified as
`participant.boardwalk.deep-pressure-assembly` commits.

Every ending receives four separate acts:

```text
Stonebend names the losses, wells, lots, claims, and settlement identity.
Sandmanor proves the failure, repair, custody, and disclosed terms.
Glaüshouse clears care, recovery, and bounded return within scope.
Flynt recognizes responsibility, public function, or protected refusal.
```

No act substitutes for another.

## Four Endings

### Shared Burden Compact

Forms `bond.deep-pressure.shared-burden-compact.v1`.

The finite Bond joins crew care, coast and Basin recovery, Field reserve,
repair material, and published custody. It includes challenge, unsafe-work
refusal, custody inspection, and exit at term. It creates no ownership between
regions or people.

### Crew and Coast Restitution

Forms `bond.deep-pressure.crew-and-coast-restitution.v1`.

Care, crew restitution, ecological repair, and a public harm record take
priority. Payment cannot buy silence or consent. Production remains separately
reviewable and material shortages do not vanish.

### Production Under Review

Forms `bond.deep-pressure.production-under-review.v1`.

Reduced production funds recovery under recurring public pressure, assay,
custody, and living-blood-exclusion review. Any failed record stops transfer.
This option fails transactionally when the campaign contains the contested
Riptide/Current Sea certificate combination.

### Protected Refusal

Forms no Bond.

Flynt recognizes the affected assembly's refusal to create a common compact.
Care, rescue, custody, and safety obligations remain. Refusal creates no debt,
retaliation, adverse status, or compelled affiliation. The public return lane
stays open for a later lawful reopening.

## Character Memory

Every scheduled person has a persistent relationship record containing:

- affinity toward Hueman's demonstrated support;
- perceived reliability;
- current condition: well, exposed, injured, recovering, or exhausted;
- remembered local outcomes;
- unresolved promises;
- permanent authority and agency boundaries;
- the recovery Bond ID, when the committed ending forms one.

Examples:

- crew-first rescue leaves Corin exposed and remembers the unfinished shut-in;
- guided Beach rescue leaves Harrow carrying added Basin care;
- Mt. Aura withdrawal protects Oren while Brindle remembers replacement-stone
  need;
- Highway flooding protects Maela while Basin repair remains under-supplied;
- Current Sea suspension raises Iona's trust in the public stop record;
- Shared Burden clears authored repair promises into one finite Bond;
- Protected Refusal preserves the promises instead of pretending they were
  settled.

Memory is an authored relationship state, not a generic affection system and
not constitutional authority.

## Scheduled Presence

The existing Dawn, Day, Dusk, and Night schedules now have physical
presentation:

- a person appears only on the map named by the current schedule;
- deterministic open tiles are assigned without replacing facility witnesses;
- the person occupies the tile and blocks movement;
- action-key interaction from an adjacent faced tile records their statement;
- shift advance moves them to the next scheduled map;
- save/load reconstructs the same time, location, statement, and memory.

Godot renders each present person with role-colored original pixel work and
initials. Rust owns the schedule, position, collision, interaction target, and
journal event.

## Evidence Journal

Each entry records:

- stable evidence identity;
- source;
- operational, testimony, custody, or local-claim kind;
- speech classification when the source is speech;
- claim;
- uncertainty;
- constitutional effect;
- observed day and shift.

Operational evidence has no speech classification. Character speech must have
one. Duplicate observations produce no duplicate campaign event.

The journal never turns evidence into an automatic decision.

## Constitutional Recovery Bonds

The three compact endings use the common constitutional Bond runtime:

```text
choice wave
→ formed with Stonebend Name and finite term
→ validated with Sandmanor Proof
→ activated from the affected assembly's bounded acceptance
```

The Bond contains all ten scheduled affected workers as named participants.
Corin retains his own Current and agency. Obligations differ by ending;
challenge, unsafe-work refusal, custody inspection, and exit remain common.

Glaüshouse Clearance and Flynt recognition are separately recorded outcome
acts. They are not inferred from Bond activation.

## Persistent Aftermath

Every ending stores:

- crew-care score;
- coast-recovery score;
- Field-security score;
- Basin-repair score;
- production posture;
- contested-certificate status;
- unresolved obligations;
- visible Boardwalk changes.

Scores summarize committed consequences for presentation. They do not replace
the events, memories, custody lots, or House acts that produced them.

The Boardwalk HUD shows all four aftermath dimensions after commitment.

## Protocol and Controls

Protocol V1 adds:

- `SupportDeepPressureSettlementIntent`;
- `AskDeepPressureAssemblyToCommitIntent`.

The first is available only on the Boardwalk after all seven local outcomes and
twelve required statements. The second is available only after nonbinding
support.

Godot controls:

- action key: speak to the faced scheduled person or Boardwalk witness;
- `T`: advance the world shift and move scheduled people;
- local `1`–`3`, then `C`: resolve each evidence-gated duty case;
- final Boardwalk `1`: Shared Burden Compact;
- final Boardwalk `2`: Crew and Coast Restitution;
- final Boardwalk `3`: Production Under Review;
- final Boardwalk `4`: Protected Refusal;
- final `C`: ask the affected assembly to commit;
- `F5` / `F9`: save/load the full campaign.

## Persistence and Failure

Gameplay interaction events may contain both:

- the local living-world evidence event;
- the linked Deep Pressure journal event.

Living-world resolution events may contain the linked campaign-resolution
event. Standalone settlement support and commitment use
`DeepPressureChanged`.

Archive schema V2 stores these nested events under the existing checksum.
Replay reconstructs the living world, campaign journal, relationships, House
acts, constitutional Bond, ending, and aftermath. A mismatch fails as
`DeepPressureReplayDivergence`.

The active schema-V2 institutional artifact now includes the current Flynt
Tross office holder along with Stonebend, Sandmanor, and Glaüshouse holders so
all four final acts can resolve through the live authority snapshot.

## Verification

`tests/deep_pressure_campaign.rs` proves:

- all twelve Functional Lore fields validate;
- deliberate deception and rumor remain explicitly classified;
- seven local cases move the campaign through the authored phases;
- choices alter named-character condition, trust, memory, and promises;
- a good record forms and activates the exact finite recovery Bond;
- all four House acts remain distinct;
- the compromised certificate bars only the affected ending without mutation;
- protected refusal forms no Bond;
- scheduled people move between real maps by shift;
- a visible person can be approached and heard through the runtime;
- the evidence journal and campaign replay exactly through a checksummed save.

Run:

```bash
cargo test --test deep_pressure_campaign
cargo run --quiet --bin deep_pressure_audit
```
