# Hollow Grove Backend Completion Plan V1

Date: 2026-10-08

Status: backend-only scope adopted by the user; work order and unresolved lore
choices remain a working plan. This document creates no new constitutional law,
product, price, Recipe, Form, actor, office or authority.

## Scope of the next passes

The user's direction is to develop Hollow Grove's backend without doing art
or writing the game. The immediate work is authored lore, architecture,
responsibility and evidence contracts, consistency validation, persistence
requirements and read-only audits. Art, scenes, maps, dialogue scripts, animation,
Godot controls and new game features are outside these passes. Existing verified
releases and saves remain usable.

Reusable domain-rule and persistence implementation can be a separate backend
pass when requested. Authored contracts can be completed and audited before
any such implementation. A proposed model is not an executed consequence.

## Verified starting point

The V7 code checkpoint is `8c13a270ad3c4a24f7960d20827962a2fdc09042`.
Its source, installed-package acceptance and 1,687 passing workspace tests are
recorded in `dist/SURVEY-FRAME-HANDOFF.md`. The read-only architecture audit
historically reports 22 bounded components, four verified bindings and ten historical route
contracts without verified execution bindings. Related code may exist for a
route without satisfying that complete binding.

| Existing bounded contract | Owning source | Verified backend responsibility |
|---|---|---|
| Boardwalk Returning Goon | `FLYNT_CONSTITUTION_V2.md`, existing Returning Goon case | independent cultural choice and existing Recognition consequence |
| Current Sea continuity | `STONEBEND_CURRENT_SEA_VERTICAL_SLICE_V1.md` | Mercy Deep's evidence, lawful Name decision and scoped outcome |
| Sympiote foundation | `HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md` | assessment, consent, establishment, recovery, practice/rest and care |
| Survey-frame doorway | `STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md` | continuing artifact, live Name/Prove, separate resident decision, failure/repair and recovery |

These bindings are evidence for their particular contracts. They do not prove
that every Form, commercial exchange or route has a completed backend.

## Completion standard for every lore family

Each family needs one owning definition and an explicit status: proposed,
adopted, modeled, implemented, or verified. A summary must link to its owner.
For the documentation pass, complete the following contract:

1. **Meaning and identity:** definitions, continuing identity, source and version.
2. **Responsibility:** actual executor, custodian, recipient, recorder and recovery
   owner; any unresolved binding is labeled.
3. **Authority:** who can decide which fact, jurisdiction, eligibility, evidence,
   and what happens when authority is vacant or disputed.
4. **Conditions and outcomes:** prerequisites, accepted changes, protected refusal,
   failure, repair, withdrawal and unfinished obligations.
5. **Continuity:** what survives a change, what becomes stale, what requires fresh
   evidence, and which independent decisions remain necessary.
6. **Persistence:** stable references, recorded facts, causal order, retry/conflict,
   migration requirements and historical evidence that cannot be discarded.
7. **Validation:** contradictions to reject, lawful alternatives to preserve,
   source crosswalk, and headless fixtures required before verification.

When implementation is requested, map the adopted contract into reusable domain
records and validators, then verify replay and migration through private
fixtures. No presentation work is needed to establish that evidence. A source
link, a passing static audit, and a verified reducer are distinct attestations.

## Completed source-and-validation expansion

The [Boardwalk backend draft](BOARDWALK_BACKEND_CONTRACTS_V1_DRAFT.md),
[active Synthesis requirements](HOLLOW_GROVE_ACTIVE_SYNTHESIS_REQUIREMENTS_V1.md),
[House handoff contracts](HOLLOW_GROVE_BACKEND_HANDOFF_CONTRACTS_V1.md), and
[persistence/audit inventory](HOLLOW_GROVE_BACKEND_PERSISTENCE_AND_AUDIT_V1.md),
and [object-lineage requirements](STONEBEND_OBJECT_LINEAGE_BACKEND_REQUIREMENTS_V1_DRAFT.md)
now provide source-backed review material. The authored
[backend register](contracts/backend/lore_registry.v1.json) includes six candidate
offers, sixteen requirement families, ten historical handoff contracts and twenty decision families.
Eighteen are wholly open; two entries record partial adoption of the Boardwalk
atmosphere and crew identities. Troons are the Pimp's adult trans sex-worker
crew, the Gimp has the classic rubber persona and Gooners retain internet-cultural
roots. Their broad structure is also adopted: devoted customers and fetish fans,
with a smaller working inner circle. Exact customs, worker duties and commercial
terms remain pending.
The Pimp's public domain is above, and the Gimp's underground Underway base is
below the Boardwalk. The Underway's closed-off private nightclub identity is
also adopted; the Pimp's precise establishments, club rooms, connecting passages
and admission/access terms remain open. District location does not substitute
for a Current Sea force event, a House decision or an executed traversal.
The [decision register](HOLLOW_GROVE_BACKEND_DECISION_REGISTER_V1.md) lists
those questions with their owning sources.
`tools/verify-backend-contracts.py` checks status/ownership and optional existing
architecture-report agreement. It is a read-only administrative validator.

This closes the source inventory/validation pass. Exact open commercial terms,
Synthesis mechanisms and actor bindings retain their pending status. The
four earlier gameplay bindings and ten headless backend bindings remain verified, with
ten further gameplay integrations deferred; the checker does not execute a new
domain consequence. The old classification/world-context refinement order and legacy
power-catalog equation claim have been reconciled to their controlling sources.

## Completed headless handoff implementation

The user prioritized clearing the ten remaining gaps. The
[headless backend handoff service](docs/BACKEND_HOUSE_HANDOFFS_V1.md) now implements
all ten existing route contracts, with three canonical dispositions each,
typed consequences, separate live authority, actual actor inputs, protected
refusal/recovery, independent observation, delivery and archive replay.
Architecture V3 reports 23 components, fourteen combined bindings and zero
unbound canonical backend routes. The ten gameplay/presentation integrations
remain separately deferred, and installed V7 game archives are unchanged.
The source checker validates declared ownership; private headless tests establish
bounded behavior. Neither establishes completion of an unsupplied world case.

## First lore family: Boardwalk offers and responsibilities

Start with the Pimp/Trooners and Gimp/Gooners because their rival operations and
Sympiote connection are adopted while the exact products remain unresolved.
The working catalog is in
[Boardwalk Offers and First Return](BOARDWALK_SYMPIOTE_OFFERS_AND_FIRST_RETURN_V1_DRAFT.md).
Its six names and service descriptions remain candidates.

Settle one offer per crew first, then apply the same completeness standard to
all adopted offers. Each product/service entry must specify:

| Contract question | Required authored answer |
|---|---|
| Supply | supplier, actual good or service, scope, provenance and delivery responsibility |
| Customer | continuing person, current needs, capacity and voluntary selection |
| Qualification | evidence needed for the promise; any distinct clinical/House authority |
| Exchange | agreed price or exchange, payment timing and owning economic rules |
| Delivery | exact inspectable result, acceptance, defect and unfinished work |
| Change and exit | cancellation, correction, non-delivery, withdrawal, remedy and dispute owner |
| Public account | voluntary publicity, attributed contribution and conflicting testimony |
| Separation | buying, affiliation, a Bond, recognition and later Synthesis remain independently decided |

Use the existing [four-pole economy](CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md)
and relevant restitution/House sources before specifying commercial terms.
Do not invent a wallet, automatic debt, universal price or new membership rule.
`ConsiderBoardwalkOffer` presently records consideration; it does not prove an
order, payment, inventory, delivery or fulfillment.

The first decision session should settle the offer pair, ordinary customer
need, rivalry tone, supplier responsibility and lawful exit. Unanswered prices,
quantities and remedies stay open. The documentation pass can define the
contract structure without selecting those answers for the user.

## Subsequent backend work order

| Priority | Family | Required result |
|---|---|---|
| 2 | Sympiote and supported Synthesis | complete source-backed progression matrix for all supported directions: prerequisites, maintenance, costs, failure, reversal and independent authority |
| 3 | Stonebend continuing object | resolve the wider lineage's input/residue, passage, repair, force and Hollowing boundaries while preserving the adopted first case |
| 4 | House handoffs and regional contracts | ten bounded headless bindings complete; use actual-case evidence and retain game presentation integration as deferred |
| 5 | Current/Aura/Recipe and economic dependencies | clarify custody, supply, knowledge, contribution, renewal, exit and dispute across domains |
| 6 | Higher/composite development and singular boundaries | resolve open proposals against existing source law; retain separate clinical, sovereign and identity decisions |
| 7 | Persistence and performance coverage | inventory owners and record bounds, historical compatibility, indexes and audit cost; measure actual backend workloads before tuning |

Sympiote/Sympiotes remains the adopted spelling. The foundation does not grant
a later Form, House office or crew identity. Higher or composite development
must use the requirements of its actual owning source.

The object lineage's Ghoul drive, Spectre passage, later force modules and
terminal Hollowing remain proposals. Repair in the adopted frame case grants
no Troll Nature or Regenerate power. Object revisions and person development
must not be conflated.

## Historical routes now bound to headless backend services

The historical gap list, now covered by `src/world/lore_handoffs.rs`, is:

- `lore.flynt.stairway-functional-recognition`
- `lore.flynt.basin-dual-expression`
- `lore.stonebend.aura-ridge-public-name`
- `lore.stonebend.mnt-aura-illegal-hollowing`
- `lore.sandmanor.aura-way-public-design`
- `lore.sandmanor.glausbahn-recovery-design`
- `lore.sandmanor.aura-beach-current-sea-role-proof`
- `lore.glaushouse.riptide-emergency-intake`
- `lore.glaushouse.current-seanad-high-risk-clearance`
- `lore.glaushouse.aura-ridge-recovery-discharge`

The four principal functions remain Stonebend **Name**, Sandmanor **Prove**,
Glaüshouse **Clear** and Flynt **Recognize**. Domain cooperation cannot merge
these authorities or replace the affected person's independent choice.

## Checkpoint and machine continuity

Documentation-only source checkpoints can advance on both machines while the
installed Core/Hueman remain the verified V7 binaries. Record those versions
separately in `dist/SESSION-HANDOFF.md`. Such a pass needs source consistency,
link/status checks and the existing read-only audit; it does not require a
new game release or art verification.

A requested domain-code or archive change requires relevant headless tests,
strict checks, explicit migration where needed, private save/replay acceptance,
and a matching recoverable checkpoint before synchronizing it. Keep prior
complete kits and original saves. The user's backend-only direction persists
in later sessions until they request a scope change.
