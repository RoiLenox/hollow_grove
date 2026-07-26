# Boardwalk Goon Bond Vertical Slice V1

Status: first playable implementation complete; all four outcomes are authoritative
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
World law: `FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md`

## Outcome

The first Boardwalk slice proves that Hollow Grove lore can become one small,
authoritative, replayable character case:

> A capable Goon returns from Glaüshouse recovery, hears offers from the Pimp
> and Gimp, receives testimony from Hoes and Goons, and makes a choice whose
> relationship and public consequences persist.

The player may investigate, disclose faculty observations, present evidence,
and support an option. The returning Goon makes the final capable choice.

## Constitutional Boundaries

The slice must preserve:

- Boardwalk as the **Return** route;
- voluntary Glaüshouse discharge;
- one persistent Being continuity for the returning Goon;
- distinct Name, proof, consent, Clearance, recognition, and Title acts;
- Pimp and Gimp as cultural titles only;
- autonomous adult Hoes;
- Goons as people rather than inventory or units;
- common Bond law for any formed Goon Bond;
- exact replay and top-level game revision;
- Godot as presentation only.

The slice does not implement full party management, combat, recruitment,
procedural generation, or general-purpose quest scripting.

## Working Title

**A Place to Return**

The title is presentation text, not a stable identity.

## Stable Content Identities

The first implementation should use stable IDs independent of display names.
Display names can be authored later without changing replay identity.

| Stable ID | Working identity | Function |
|---|---|---|
| `case.boardwalk.returning-goon.v1` | returning-Goon case | aggregate narrative case |
| `being.boardwalk.returning-goon.v1` | Returning Goon | capable decision subject returning from recovery |
| `outcome.boardwalk.pimp-patronage.v1` | voluntary patronage outcome | typed outcome with finite patronage Bond |
| `outcome.boardwalk.goon-bond.v1` | Goon Bond outcome | typed outcome with finite Goon Bond |
| `outcome.boardwalk.limited-cooperation.v1` | scoped cooperation outcome | typed outcome with short three-party Bond |
| `outcome.boardwalk.independent-return.v1` | self-directed return outcome | typed refusal/return record with no Bond |
| `bond.boardwalk.pimp-patronage.case-a` | first patronage Bond | common constitutional Bond identity |
| `bond.boardwalk.returning-goon.case-a` | first Goon Bond | common constitutional Bond identity |
| `bond.boardwalk.limited-cooperation.case-a` | first limited-cooperation Bond | common constitutional Bond identity |
| `being.boardwalk.pimp.v1` | The Pimp | cultural glamour pole |
| `being.boardwalk.gimp.v1` | The Gimp | cultural endurance pole |
| `being.boardwalk.hoe-witness-a.v1` | Hoe witness A | testimony about opportunity and Pimp affiliation |
| `being.boardwalk.hoe-witness-b.v1` | Hoe witness B | testimony about boundaries, leverage, and withdrawal |
| `being.boardwalk.goon-witness-a.v1` | Goon witness A | testimony about protection and Goon Bond service |
| `being.boardwalk.goon-witness-b.v1` | Goon witness B | testimony about debt, challenge, and exit |
| `being.glaushouse.discharge-advocate.v1` | discharge advocate | confirms capacity, voluntary discharge, and available support |
| `site.boardwalk.return-gate.v1` | Return Gate | entry from recovery |
| `site.boardwalk.neutral-promenade.v1` | Neutral Promenade | evidence gathering without required affiliation |
| `site.boardwalk.pimp-court.v1` | Pimp Court | glamour offer and Hoe testimony |
| `site.boardwalk.gimp-retinue.v1` | Gimp Retinue | protection offer and Goon testimony |
| `site.boardwalk.recognition-stage.v1` | Recognition Stage | public consequence presentation |

These IDs establish content identity only. They grant no office, membership,
access, or authority.

## Map Increment

The first map should be a compact Boardwalk return vestibule rather than the
entire route.

```text
Glaüshouse Return Gate
        |
        v
Neutral Promenade
   /             \
  v               v
Pimp Court    Gimp Retinue
   \             /
    v           v
   Recognition Stage
```

The runtime owns map identity, collision, position, facing, target identity,
and accepted movement. Godot selects tiles, animation, sound, lighting, and
camera from committed views.

No location alone creates affiliation or recognition.

## Case State

The authoritative gameplay aggregate needs enough state to answer:

- whether the case is open or resolved;
- the returning Goon's stable identity and current location;
- whether voluntary discharge and capacity are established;
- which testimonies have been heard;
- which faculty observations have been disclosed;
- which offers are currently available;
- which option the returning Goon selected;
- whether a common Bond was formed;
- the resulting public recognition state;
- the exact source event sequence and game revision.

Suggested presentation-neutral states:

```text
Unavailable
→ ReadyForReturn
→ OffersOpen
→ EvidenceGathering
→ ChoiceReady
→ ChoiceCommitted
→ PublicConsequencePresented
```

These are gameplay case states, not common `BondPhase` replacements.

## Evidence Packet

The choice becomes available only after a minimum evidence packet exists:

1. Glaüshouse voluntary-discharge and capacity evidence;
2. one Pimp offer with disclosed terms;
3. one Gimp offer with disclosed Goon Bond obligations;
4. at least one Hoe testimony;
5. at least one Goon testimony;
6. the returning Goon's stated want and protected boundary;
7. disclosed uncertainty for every Precog projection shown.

Additional testimony may alter the available interpretation without silently
changing constitutional truth.

## Faculty Presentation

Faculties expose different parts of the same case.

### Presynce

Presynce may notice physical strain, crowd movement, guarded posture, an
approaching shove, blocked exits, or the Gimp's immediate embodied limits. It
does not diagnose illness or guarantee successful intervention.

### Resynce

Resynce may reveal invitation, loyalty, rivalry, fear, obligation, social
momentum, and pressure between the Pimp, Hoes, Gimp, and Goons. It does not read
minds or prove a hidden motive.

### Precog

Precog may project probable consequences of each disclosed option:

- increased visibility with patronage dependence;
- increased protection with service burden;
- reduced immediate support with greater independence;
- a mixed arrangement with conflict risk.

Every projection shows evidence and uncertainty. It never states a guaranteed
future.

### Prefog

Prefog keeps multiple legal choices open and may generate a limited-cooperation
option after the initial Pimp/Gimp polarity is presented.

### Prefig

Prefig may produce a provisional schedule, public-return plan, service plan, or
patronage demonstration for the selected candidate. The demonstration is not a
permanent agreement or proof that the choice will succeed.

## Choice Set

The Returning Goon must retain these lawful possibilities:

### Accept Pimp Patronage

- creates a disclosed patronage or affiliation relationship;
- increases public Aura and access;
- preserves the Returning Goon's independent identity;
- does not make the Being a Hoe automatically;
- permits later challenge, default, withdrawal, or resolution.

### Form a Goon Bond

- uses the common constitutional Bond lifecycle;
- names the Gimp and Returning Goon as distinct participants;
- records protection, service, duration, evidence, challenges, defaults, and
  exit;
- may not transfer bodily autonomy or create ownership.

### Limited Cooperation

- accepts one scoped service or public appearance;
- creates no perpetual affiliation;
- preserves explicit end conditions;
- may later become evidence for a new lawful offer but never auto-upgrades.

### Independent Return

- declines both offers;
- preserves public return and ordinary Flynt civic presence;
- may carry less immediate access or protection;
- does not count as failed gameplay.

The player supports, questions, or presents evidence for an option. The capable
Returning Goon commits the choice.

## Implemented Gameplay Commands

```text
EnterMap
InteractHueman
DiscloseFacultyObservation
SupportBoardwalkOption
AskReturningGoonToDecide
```

Every mutating command carries the existing protocol version, session ID,
request ID, expected game revision, event ID, and causal position as applicable.
Exact retries return exact prior responses; stale requests fail closed.

The protocol exposes these as `EnterRegionIntent`, `InteractIntent`,
`DiscloseFacultyObservationIntent`, `SupportBoardwalkOptionIntent`, and
`AskReturningGoonToDecideIntent`. There is deliberately no protocol intent that
lets the player commit a choice in the Returning Goon's name.

## Implemented Gameplay Events

```text
HuemanMapEntered
HuemanInteractionOpened
BoardwalkFacultyDisclosed
BoardwalkOptionSupported
ReturningGoonChoiceCommitted
```

`ReturningGoonChoiceCommitted` records a stable `BoardwalkOutcomeId` and an
optional relationship Bond. Pimp Patronage, Goon Bond, and Limited Cooperation
reference distinct Bond IDs. The common constitutional runtime holds the exact
`Formed`, `Validated`, and `Activated` child events and projects each
relationship in `Active` phase. Independent Return has no Bond. A label alone
never claims that a relationship exists.

Every outcome records live Glaüshouse discharge Clearance and live Flynt
recognition. Every formal relationship is separately named by Stonebend and
proven by Sandmanor. These acts remain distinct and are snapshotted from the
runtime's `WorldSession`; gameplay does not call an institutional test fixture.

All case events remain append-only. Replay must reproduce the exact choice,
testimony set, relationship identity, child-runtime state, and public
consequence.

## First Goon Bond Terms

The first candidate Bond should remain intentionally small:

| Term | Draft value |
|---|---|
| Participants | the Returning Goon and the Gimp |
| Scope | Boardwalk escort, mutual defense, and one public-return appearance |
| Duration | finite and explicitly stated |
| Gimp obligation | shelter, escort, disclosed protection, no concealed transfer |
| Returning Goon obligation | scoped service and truthful risk disclosure |
| Shared obligation | preserve evidence and honor challenge/refusal procedure |
| Prohibited term | ownership, sexual entitlement, identity erasure, permanent inherited debt |
| Challenge path | either participant may challenge coercion, breach, concealed risk, or false evidence |
| Default path | unmet named obligation is declared and resolved through common Bond law |
| Resolution candidates | Complete, Renew, Transfer, or Dissolve as lawfully supported |

Exact IDs, quantities, units, evidence references, House decisions, and causal
positions live in the authoritative outcome and child Bond records.

## Other Implemented Outcome Terms

| Outcome | Bond scope | Duration | Required protections |
|---|---|---|---|
| Pimp Patronage | disclosed opportunity, compensation, public patronage, and preserved independent identity | finite, 60 causal positions | leave, revoke, challenge, refuse opportunity, no retaliation, no ownership, no role or sexual inference |
| Limited Cooperation | one disclosed three-party job between the Pimp and Gimp poles | finite, 24 causal positions | scope challenge, refuse expansion, stop at the agreed edge, no ownership, no affiliation inference, no automatic renewal |
| Independent Return | no relationship Bond | none | refusal creates no debt or adverse status; witness is not custody; recognition is not Title or ownership |

Pimp Patronage does not establish Hoe identity. The separate open question of
whether formal Hoe affiliation is a Bond remains unratified.

## Public Consequences

The first slice needs visible but bounded consequences:

| Choice | Immediate projection | Persistent consequence |
|---|---|---|
| Pimp patronage | the Recognition Stage becomes a glamour event | affiliation terms and public Aura evidence remain inspectable |
| Goon Bond | Goons form an escort around the Returning Goon | authoritative Bond identity and obligations remain inspectable |
| Limited cooperation | both poles acknowledge a scoped arrangement | end condition and non-affiliation remain explicit |
| Independent return | the Returning Goon crosses the stage without a patron or retinue | refusal and civic return remain recorded without punishment-as-failure |

Consequences should change dialogue, NPC placement, access, or assistance on a
subsequent visit. A choice that changes only one line of text is insufficient.

## Failure and Refusal

The slice must support:

- missing evidence;
- stale revision;
- conflicting request-ID reuse;
- attempted choice before capacity or discharge is established;
- a player attempting to choose for the Returning Goon;
- hidden or prohibited Bond terms;
- a Hoe withdrawing testimony or participation;
- a Goon challenging an obligation;
- the Pimp retaliating against protected refusal;
- the Gimp demanding unlawful force;
- a late refusal before irreversible commitment;
- exact retry after a transport interruption.

Unlawful offers fail without committing a relationship. Refusal preserves
identity and does not trap the case in an invalid state.

## Presentation Requirements

Godot may render:

- the Boardwalk vestibule;
- Pimp and Gimp visual contrast;
- autonomous Hoe and Goon character placement;
- dialogue and testimony pages;
- evidence and uncertainty icons;
- faculty prompts;
- offer comparison;
- committed public consequence;
- follow-up NPC and route presentation.

Godot may not:

- decide capacity;
- infer consent;
- form a Bond;
- select the Returning Goon's choice;
- grant recognition;
- invent state from an animation;
- make a failed network request appear committed.

## Deterministic Acceptance Tests

The implemented slice has deterministic coverage proving:

1. the case cannot open without the Returning Goon's stable identity;
2. the choice cannot commit without voluntary discharge, capacity, and the
   minimum evidence packet;
3. the player cannot commit a choice on behalf of the capable Returning Goon;
4. Pimp patronage records no Hoe identity or sexual-consent term;
5. all three formal relationship choices use accepted common constitutional
   events and explicit no-ownership obligations;
6. Limited Cooperation cannot auto-renew or silently become affiliation;
7. Independent Return resolves successfully with no Bond;
8. the same event history replays to the exact gameplay event sequence, typed
   outcome, live-authority snapshot, and active constitutional Bond state;
9. exact request retries return the exact prior response;
10. stale or conflicting requests commit no event;
11. Godot fixtures contain only authoritative views and presentation state;
12. Tross, Chimera, Flynt institutions, and recursion-kernel neutrality remain
    unchanged.

## Implementation Record

1. `src/gameplay/world.rs` supplies the typed two-map registry while retaining
   all Aura Ridge coordinates and interactions.
2. `src/gameplay/boardwalk.rs` owns the bounded case, evidence, faculty gate,
   four typed outcomes, two House boundary decisions, and three finite common
   Bond constructions.
3. `src/gameplay/runtime.rs` owns region entry, movement, interaction, support,
   subject commitment, replay, and the constitutional child runtime.
4. `src/gameplay/archive.rs` persists the supported gameplay event history and
   exact institutional authority state in a schema-V2, checksum-verified
   archive with explicit V1 migration.
5. `src/gameplay/protocol.rs` exposes the implemented intents, idempotency,
   revision checks, and save/load slots.
6. `hueman_godot/scripts/retro_overworld.gd` renders Boardwalk actors, case
   status, choice controls, saved slots, and post-choice placement/dialogue.
7. `tests/gameplay_boardwalk.rs` proves evidence gating, subject attribution,
   all four lawful outcome records, all three active finite Bonds, fail-closed
   vacant offices, replay, checksum rejection, restart/load, and the
   post-choice revisit.

The next build layer may widen content, but it must not weaken these boundaries
or silently turn this bounded case machinery into a generic authority system.
