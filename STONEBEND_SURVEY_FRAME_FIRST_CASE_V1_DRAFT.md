# Stonebend Survey Frame: First Case V1

Date: 2026-10-08

Status: WORKING first-case proposal; unratified and unimplemented

Owner: the [PrimePunk Object Lineage](STONEBEND_PRIMEPUNK_OBJECT_LINEAGE_V1_DRAFT.md)
under [Stonebend Constitution V2](STONEBEND_CONSTITUTION_V2.md)

## The opening scene

A resident needs a damaged doorway surveyed before choosing whether to let a
workshop fit a temporary rig. The workshop has recovered an old measuring
frame. Some members can be traced; one damaged joint and its earlier load
rating cannot. An alternate safe route is available before work begins.

The player follows this particular frame through recovery, a measuring trial,
a proposed rig, a failed support claim, and a documented repair. The object
keeps its identity and its history. Its appearance cannot supply a missing
rating. The resident decides whether the offered use actually helps their
life, and can decline while the workshop retains responsibility for unfinished
work.

Working case title: **The Frame That Must Hold Its History**. This is story
copy, not a committed wire ID, named Recipe or new Title.

## What the first episode demonstrates

Gremlin/Scavenge asks what can be recovered and honestly reused. Goblin/Rig
asks how the arrangement carries its claimed function, including its release
and limits. Routine maintenance in this case does not certify attainment of
Troll/Regenerate. None of the eight Nature labels is an automatic player level
or a mandatory sequence of transformations.

An early Sympiote can explore the workshop and assist within established
capability. Their Form, bodily practice and rest remain governed by the adopted
foundation. An object revision grants no new Form, Aura, Title or office. The
resident's access decision grants no clinical Clearance.

## Proposed playable sequence

| Moment | Player action and affected person's choice | Durable consequence |
|---|---|---|
| Receive the need | Hear which passage is useful and preserve the alternate route | Attribute the access request; record scope and responsibility without permission to begin work |
| Inspect recovered parts | Compare traces, segregate the damaged joint, disclose unknown history | Record source members, custody, unknowns and exclusions against the same frame |
| Establish measuring use | Help conduct a repeatable measuring trial | Record only the exact revision's measuring result; support capability remains unproven |
| Offer a temporary rig | Explain the proposed arrangement, release, limits and unfinished proof | An offer can be accepted for further examination, refused or deferred; an offer is not installation consent |
| Witness a failed claim | A support trial slips at the disclosed test boundary | Commit the negative result, isolate the rig, retain measuring use only if its independent evidence remains valid |
| Respect the resident's refusal | Receive “not in my doorway” without persuading away its meaning | Preserve access and refusal; remove the offer from deployment and keep responsibility with the workshop |
| Repair and review | Replace the suspect joint, retain its identity and failure record, retest the changed revision | Invalidate affected earlier claims; link replacement provenance and fresh proof to this revision |
| Return a scoped result | Offer measuring-only service, a newly demonstrated temporary use, or a protected delay | Resident receives the actual disposition, outstanding duties, stop/release route and custody record |

A resident's refusal is a valid ending. A successful repair does not cancel
that refusal. A renewed offer requires renewed, specific acceptance. The
resident can accept measurement and refuse support, or accept a bounded rig
without accepting later modification. Withdrawal stops future deployment
and returns to the recorded safe disposition.

## Proposed endings

1. **Measured, not supported.** Useful measurements are delivered; the frame's
   support claim remains unproven. The resident keeps the alternate route.
2. **A limited rig, accepted.** Fresh proof covers the repaired revision and
   exact proposed use. Specific resident acceptance permits that use, with
   visible limits, release and a responsible return.
3. **Work waits safely.** The resident declines or evidence remains incomplete.
   The access need, evidence, object and workshop duties remain addressable.

None grants a later Synthesis Form or erases an unresolved problem. Completion
means that this episode delivers its recorded disposition; it does not mean
every doorway is repaired or every lineage stage has been demonstrated.

## Lawful responsibilities to bind before implementation

| Capacity | Existing law constrains the role | Required case record |
|---|---|---|
| Affected resident | Specific acceptance, refusal, access and challenge belong to the affected person | Stable Being/participant identity and the exact accepted scope |
| Workshop executor | Work stays within demonstrated craft and deployment scope | Actor identity, offered work, stop and recovery obligation |
| Object/material custodian | Stonebend preserves identity, custody and source/replacement provenance | Frame identity, revision links, damaged part, replacement and custody |
| Independent reviewer | An executor's claim cannot stand in for proof; measuring and load claims stay distinct | Reviewer identity, method, exact revision, result and limitations |
| Decision owner | Formal Name/boundary/provenance questions belong to Stonebend; Sandmanor Prove applies to a claimed demonstration when required | Existing lawful authority and a scoped decision, rather than a fictional default holder |
| Witness and recipient | The delivered result must be attributable and challengeable | What was observed, which disposition was delivered, who received it |
| Recovery owner | Failure returns work, evidence and responsibility | Isolation, custody, alternate access and unresolved duties |

These are proposed case capacities. They do not create new offices or assign
incarnations by display name. Freemason and Proliteriate participation must
follow their existing powers; a civic request does not itself command an
installation. Formal Stonebend or Sandmanor questions are resolved at their
actual required boundaries. Ordinary workshop acts need no invented four-House
approval chain. Clinical intervention, if later introduced, requires its own
Glaüshouse scope; public recognition, if sought, has Flynt's own scope.

The present implementation decision still needs exact actors, custody start,
independent review binding, delivered receipt and recovery owner. Those
choices must be adopted and checked against the live catalog before the case
is added to production gameplay. No placeholder actor can supply authority.

## Proposed kernel contract

The implementation should have one reducer-owned case aggregate with stable
object identity and explicitly linked revisions. Its state distinguishes need,
scope, custody, source parts/unknowns, measurements, load claims, proof,
resident choices, failed trials, repair and delivered disposition. Technical
revision counters identify object changes; they do not award player XP.

Typed actions should separate observing a part, recording a trial, offering a
use, recording acceptance/refusal, repairing a revision and delivering a
disposition. Claims and proof identify the exact revision and scope. A repair
invalidates affected claims and retains their previous records. No projection
may promote “measures correctly” into “safe to support”.

A lawful action that records a failed trial must commit an accepted negative
result event. Otherwise transactional rejection would erase the evidence the
story requires. A malformed or unauthorized request is refused atomically;
it cannot invent a trial, acceptance, proof or repair. Duplicate request IDs
retain the existing retry semantics and cannot repeat the physical consequence.

All fallible domain work runs in the gameplay transaction candidate. Adding
this aggregate requires including it explicitly in the candidate initializer.
Accepted events preserve the journal. Replay must reconstruct each revision,
failure and refusal and compare the recorded result against the lawful reducer.
Godot reads the resulting state and sends typed intent; it cannot certify,
grant consent, assign custody or edit revision history.

## Proof and performance plan

Required witnesses are a recovered measuring revision, one retained negative
support result, a resident refusal, a repair with invalidated old claims and
fresh exact-revision proof, plus all three bounded endings. Negative checks
include proof reused after repair, measurement promoted to a load rating,
revoked/refused consent ignored, forged authority, invented custody, tampered
events and duplicate consequences. Save/restart/load must preserve all of them.

Keep the twelve historical route definitions intact. This object episode
needs its own adopted functional-lore contract and verified execution binding;
it must not borrow Mercy Deep's continuity case as evidence of completion.
Any new gameplay event or archive contract needs a versioned compatibility
note, legacy fixtures and full validation before deployment on either machine.

Measure commands, projection size and encode/replay time with increasing
revision histories. Reuse the V6 journal ownership optimization and benchmark
method; optimize additional history only after the case supplies a measurable
cost. Full history remains the authority. UI summaries can be bounded with a
total count while archives preserve all records.

Presentation can disclose the replaced joint, retained damaged part,
measuring marks, tension/release, withheld support claim and clear alternate
route through existing art constraints. Numerical engineering specifications,
new assets, later force/energy modules, prices and a universal crafting recipe
are outside this proposal.

## Open discussion

The doorway is the current working scene. The practical need can change
without reopening the continuing object's identity/history principle. Before
adoption, settle whether this is the desired first scene and choose its actual
people and workshop. Later Ghoul input/residue, Spectre passage and the deeper
force/Hollowing mechanisms remain separate open questions in the parent draft.
