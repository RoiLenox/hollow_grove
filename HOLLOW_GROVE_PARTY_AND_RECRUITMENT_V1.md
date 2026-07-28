# Hollow Grove Party and Recruitment V1

Status: implemented playable vertical slice
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
Campaign source: `HOLLOW_GROVE_DEEP_PRESSURE_CAMPAIGN_V1.md`
Runtime owner: `src/gameplay/party.rs`

## Party Shape

The active party contains at most six Beings:

```text
Hueman
+ up to five companions
= six total
```

Hueman remains the persistent gameplay Being. A companion does not become
Hueman, property, an inventory unit, an institutional office, or a
constitutional Bond merely by joining.

The party records one selected member and one ready lead. Lead status governs
whose authored field action may be used; it does not transfer another
member's Current, capable-subject authority, House authority, identity, or
right of refusal.

## Rename-Safe Identity

Current personal names are display labels. Saves use role-based candidate and
continuity identities:

| Current display name | Stable candidate | Party role | Field action |
|---|---|---|---|
| Corin Wake | `recruitment-candidate.riptide-pressure-keeper` | Riptide pressure keeper | read pressure |
| Brindle Reed | `recruitment-candidate.field-engagement-steward` | Aura Field engagement steward | read Engagement Farm work |
| Harrow Vale | `recruitment-candidate.basin-care-runner` | Basin care runner | survey care route |
| Oren Pike | `recruitment-candidate.high-mine-support-reader` | high-mine support reader | inspect support |
| Maela Downroad | `recruitment-candidate.deepworks-air-keeper` | Deepworks air keeper | test air |
| Tess Breakwater | `recruitment-candidate.breakwater-current-reader` | breakwater Current reader | read Current |

Changing a display name later does not change the candidate ID, party
continuity ID, recorded decision, field-action history, or save replay.

## Recruitment Is Their Decision

Recruitment becomes available only after Deep Pressure has a committed
aftermath. Hueman must face and speak to the physically present scheduled
candidate. The player selects how to ask:

1. **Shared Work** — join a disclosed common task;
2. **Recovery First** — put rest and recovery before field duty;
3. **Independent Company** — travel together without inferring a common
   compact.

The request is nonbinding. The candidate decides through the authoritative
party reducer using their own:

- Deep Pressure affinity and reliability memory;
- injury, exposure, exhaustion, recovery, or well condition;
- remembered campaign ending;
- accepted work paths;
- permanent agency and role boundaries.

An accepted record explicitly preserves the right to stop, refuse, and leave
without debt. A declined record is equally canonical, creates no debt or
penalty, and prevents repeated requests from becoming harassment.

## Outcome and Condition Gates

- `Injured` or `Exposed` candidates decline while care or safety remains
  unresolved.
- `Exhausted` candidates accept only a compatible Recovery First request and
  join as resting.
- A resting companion cannot lead or use a field action.
- Advancing one living-world shift changes an authored resting companion to
  ready through a nested party recovery event.
- Protected Refusal requires Independent Company; asking for Shared Work after
  that ending is a persistent decline.
- Every path has a minimum combined affinity and reliability threshold.
- Every candidate has two accepted paths. A request outside those paths is a
  role-boundary decline.

None of these gates invents incapacity. They preserve the condition and choice
already recorded.

## Engagement Farm

The Engagement Farm is the Aura Field home for disclosed, finite joint work.
It is not a recruitment office and does not decide whether anyone joins.

Brindle's field action reads:

- the named shared task;
- its measurable Field need;
- its term and safety edge;
- its exit and leave-without-debt condition;
- the current Deep Pressure Field-security aftermath.

The action produces bounded evidence only. It does not form a Bond, grant
Title, prove consent, compel recruitment, or make a constitutional decision.

## Six Field Actions

| Action | Lawful locations | Finding | Limit |
|---|---|---|---|
| Read Pressure | Riptide and Current Sea rigs | separates pressure, production posture, rescue, and shut-in duty | no certification or duty decision |
| Read Engagement Work | Aura Field | names finite Engagement Farm work against Field security | no recruitment or Bond |
| Survey Care Route | Aura Beach and Aura Basin | preserves care, continuity, transfer refusal, and Basin condition | no clinical Clearance |
| Inspect Support | Mt. Aura and Stairway mines | distinguishes support, roof condition, and withdrawal edge | no mineral Title |
| Test Air | Stairway mine and Highway Deepworks | names a gas stop before production | no mine-safety decision |
| Read Current | Aura Beach, Riptide, and Current Sea | separates observed Current movement from rumor | no proof or certification |

Only the current ready lead may use their own action. Wrong actor, wrong
action, wrong map, unavailable condition, or a Hueman-only request fails
without a gameplay revision or partial record.

## Event and Save Contract

`PartyState` is event-sourced through:

- `RecruitmentDecided`;
- `MemberSelected`;
- `LeadChanged`;
- `FieldActionResolved`;
- `ShiftRecoveryApplied`.

Recruitment decisions store the exact relationship scores, condition, campaign
outcome, path, reason, agency statement, nonbinding-player flag, and
debt-free-refusal flags used at the time.

Gameplay archive schema V3 serializes every party event under the existing
checksum and binds the history to The Runtime Federation. Replay recomputes
recruitment, selection, lead, action, and shift recovery against prior Deep
Pressure and map state. Divergence fails closed.
Recruited people leave the scheduled-map projection and no longer block a tile
as if they were still at work.

## Protocol and Godot

Protocol V1 now implements:

- `OpenPartyIntent`;
- `SelectPartyMemberIntent`;
- `RecruitIntent`;
- `SwitchLeadIntent`;
- `UseActionIntent`.

Godot controls:

- `P`: open or close the party panel;
- party Up/Down: select a member;
- party Enter/Space/Z/X: ask the selected ready member to lead;
- party `U`: use that lead's field action;
- `R` while facing a candidate: open the recruitment request;
- recruitment `1`: Shared Work;
- recruitment `2`: Recovery First;
- recruitment `3`: Independent Company.

Godot displays the party, candidate state, lead, readiness, field action, and
committed decision. It does not calculate acceptance.

## Completion Boundary

This vertical slice implements party capacity, authored recruitment,
conditions, recovery, lead selection, one field action per recruit, protocol,
Godot presentation, checksummed persistence, and exact replay.

It does not yet implement formation walking, combat, encounter capture,
equipment, action loadouts, levels, party dismissal, death, generic affection,
procedural recruits, or regional Synthesis. Those later systems must consume
these stable identities and decisions without bypassing refusal or authority.

Verification:

```bash
cargo test --test party_recruitment
cargo test --test deep_pressure_campaign
cargo run --quiet --bin party_recruitment_audit
```
