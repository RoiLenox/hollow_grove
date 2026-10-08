# Hollow Grove Lore Kernel Optimization Plan V1

Date: 2026-10-07

Reviewed source: `3593e34`

Status: implementation plan; baseline findings retained below

The first implementation milestone is recorded in
[Lore Kernel Optimization V1](docs/LORE_KERNEL_OPTIMIZATION_V1.md). All twelve
definitions have exact handoff inspection, and Boardwalk has the first verified
gameplay binding. Further case adoption must prove execution equivalence.

Focus: clarity, consistency, and executable consequence in the operational lore kernel

## Intended result

Make each lore rule easier to trace from its controlling source to an affected
Being's choices and a persistent consequence. Preserve the different House
grammars, cultural voices, uncertainty, and lawful refusal. Optimization here
means fewer contradictory projections and fewer responsibilities carried only
by prose; performance tuning is a later, measured concern.

The smallest useful unit is already defined by the operational lore contract:

```text
subject and jurisdiction
→ question and evidence
→ responsible decision
→ bounded execution or refusal
→ witnessed consequence and recipient
→ persistence, replay, and safe return
```

Use the Platinum Path's existing five tests—Continuity, Consent, Witness,
Custody, and Return—to review each unit. These are existing cultural tests,
not a new governing layer.

## Boundaries found in the repository

| Layer | Existing owner | Optimization belongs here when… |
|---|---|---|
| Pure recursion | `hollow-grove-recursion-kernel/`, `KERNEL_v0.1.2.md` | a neutral routing computation needs correction; it remains one bounded pass |
| Neutral semantics and geometry | `hollow-grove-semantic-kernel/` | geometry or composition needs improvement without introducing House meaning |
| Constitutional runtime | `src/constitutional/` | identity, evidence, authority, causal order, lifecycle, or replay needs enforcement |
| Operational lore | `src/world/lived_lore.rs` and its adopted contract | questions, responsibilities, stops, handoffs, and completion need exact bindings |
| Playable consequence | `src/gameplay/` | a lore rule must affect an actual choice, event, or recovery path |
| Presentation | Rust projections and Godot | accepted meaning needs clearer dialogue, labels, imagery, or feedback |

The controlling sources are the [Compromise](HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md),
[Authority Map](REPOSITORY_AUTHORITY_MAP.md), incorporated House constitutions,
and explicit ratified amendments. The
[Foundation Freeze](FOUNDATION_FREEZE_V1.md) defines migration requirements.
The [Operational Lore Kernel](HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md)
is adopted despite its historical draft filename. The
[Kernel Constitutional State](HOLLOW_GROVE_KERNEL_CONSTITUTIONAL_STATE_V1_DRAFT.md)
retains an additive draft status. Preserve these distinctions.

Keep three different orders explicit: case-specific constitutional gates,
productive cooperation, and the civic year. Name, Prove, Clear, and Recognize
are separate House questions; a case uses the gates it needs in the causal
order it needs. Designing, engineering, crafting, and repair describe work.
Seasonal Functions describe the calendar. None is a universal rank ladder.

## Findings that determine priority

### 1. The amended Flynt order has drifted between executable projections

The [ratified amendment](HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md)
locks combined **Manticorp → perfected Chimera**. The typed
`HIGHER_SYNTHESIS_RULES` in `src/world/practitioner_ontology.rs` already agrees.
However:

- `src/hollow_deck_seed.rs`, in `FlyntConvergenceMastery`, seeds Chimera first
  and Manticorp as its perfection;
- `src/hollow_deck.rs::validate_flynt_institution` requires that
  reversed order;
- `tests/hollow_deck.rs` asserts the same reversed order;
- `HOLLOW_DECK_V1.md` describes an amended catalog schema `1.6.0`, while
  `HOLLOW_DECK_SCHEMA_VERSION` remains `1.5.0`.

This is a specific conformance defect. Internal consistency tests can pass
while the projected meaning contradicts ratified canon. The current
consolidation document's claim that validators already follow the amendment
also needs correction or implementation-backed verification.

Preserve stable `manticorp`, `chimera`, and Tross compatibility identities.
Keep institutional membership, synthesis state, unique constitutional identity,
and reciprocal Nature-pair mechanics separate. Do not infer new combat or
mixed-track advancement rules from the amendment.

### 2. The lore envelope validates descriptions more strongly than execution

`FunctionalLoreDefinition` already carries twelve contract fields and eight
operational role fields. Its validator checks House ownership, route coverage,
nonempty responsibilities, and handoff strings containing `→`. Role presence
is checked by substring search across the joined handoff text.

That validates an authored catalog, not an executed handoff graph. Choice and
consequence arrays are paired by position; completion is prose. Catalog
instantiation uses `scenario_house_decision`, which creates an accepted,
institution-backed decision. This establishes a catalog authority snapshot;
it does not prove that a player completed that case, supplied its evidence,
or obtained its consent.

Keep definition validation, authority snapshots, and lived execution as
distinct claims in audits and presentation.

### 3. Playable content has several related contract shapes

`FunctionalLoreDefinition`, `LivingCaseDefinition`, and
`DeepPressureFunctionalLoreDefinition` separately express lore requirements.
Living-world and Boardwalk reducers already have typed evidence and choices;
they are useful foundations. Source search found the selected catalog lore
IDs in definitions and tests, but no direct reference from gameplay reducers.
This establishes a missing explicit link for those IDs, not absence of
equivalent playable content.

The twelve catalog loops cover all four Houses and all ten routes. Coverage
and exact catalog replay are valuable; they do not alone prove twelve complete
player-to-consequence lifecycles.

## Implementation sequence

| Step | Concrete work | Completion evidence |
|---|---|---|
| 1. Align active canon | Correct Deck convergence data and validation from the ratified amendment; reuse the existing typed Flynt states/rules; reconcile schema and implementation-status claims | Deck projections agree with the typed ontology; independent conformance tests reject reversal; existing IDs and saved discoveries remain readable |
| 2. Trace rule ownership | Extend existing authority/status registries with source, status, owning axis, amendment/supersession reference, and consumer links where missing | Each rule used by the pilot has a controlling source and owner; OPEN mechanics remain visibly open |
| 3. Bind one lived loop | Add an explicit read-only adapter from a catalog lore ID to an existing gameplay case, evidence, decisions, outcomes, and replay records | Inspecting the pilot distinguishes a definition, a pending case, refusal, and an executed consequence |
| 4. Make responsibilities exact | Bind stable role/office references and typed handoff endpoints, choice IDs, stop reasons, and completion predicates to that adapter | Wrong recipient, mismatched subject, reordered handoff, or unresolved stop cannot appear as completion |
| 5. Project the result | Generate review tables and player-facing summaries from the validated bindings, while keeping authored cultural text | Interface copy shows who decides, what remains uncertain, what happens next, and how to refuse or return |
| 6. Expand from proof | Apply the adapter to the remaining catalog loops and connect living-world/Deep Pressure contracts where equivalent | Each adopted loop has accepted and negative execution/replay proof; coverage reports distinguish declaration from playability |

Start with the existing **Boardwalk return case** and
`lore.flynt.boardwalk-return-recognition`. It already exercises Glaüshouse
discharge, Flynt recognition, nonbinding player support, independent return,
finite relationships, advisory-only clients, and checksummed gameplay replay.
Its four gameplay outcomes and three catalog descriptions need an explicit
mapping; do not pretend their cardinalities already match.

Use actual gameplay records as evidence. A catalog's accepted authority snapshot
must never stand in for the affected actor's commitment or the case outcome.
Reuse existing `HouseDecision`, evidence references, causal positions, and
gameplay event types before introducing new abstractions.

For typed handoffs, resolve endpoints exactly against the institutional/actor
model. Each handoff should identify sender capacity, recipient capacity,
subject/case, carried record, requested question, and stop disposition. Validate
the case-specific sequence and references rather than merely checking that all
role names appear somewhere. Preserve lawful delegation; operational work need
not be performed personally by a House apex.

Keep the V2 prose archive readable during the adapter phase. The current
functional-lore replay compares exact canonical prose and live authority.
Changing serialized descriptions is therefore a compatibility-sensitive
change. If typed execution records alter an archive or frozen boundary, prepare
a versioned migration, authentic old/new replay fixtures, invariant updates,
and required validation under the Foundation Freeze before changing projections.
Do not change historical bytes or silently derive completed acts from text.

## Lore design after the first executable proof

The next creative task should follow the amendment's existing priority: one
Stonebend PrimePunk object lineage, beginning with Gremlin/Scavenging. Follow
one recognizable object through successive revisions. For each proposed
revision, state what persists, what material principle becomes usable, who
performs the work, what evidence would support it, its cost and failure,
and what lawful return or repair remains.

Use inter-House cooperation to make that object meaningful: Stonebend's
material continuity, Sandmanor's design and proof, Flynt's engineered function,
and Glaüshouse's care or repair where living use requires it. Apply constitutional
gates according to the actual intervention. Cultural/professional expression
must not automatically grant office, Title, Clearance, or synthesis state.

Present this first as a WORKING design case. Exact arsenal names, terminal
Hollowing metaphysics, Chromacord mechanics, Hand appointment procedures, and
mixed-track synthesis rules remain open in their owning canon. Resolve only
the design decisions needed for the object proof, then record any authorized
adoption explicitly.

## Acceptance and verification

For the first implementation milestone, require:

1. Ratified Flynt order agrees across ontology, Deck data, validator, tests,
   and visible copy; independent Current/Aura/Weapon tracks remain independent.
2. Biological Form, Hand investiture, sovereign office, and visual aliases
   remain separate axes. Preserve Hecaton/Right and Pegasus/Left.
3. Pilot success names the actual subject, evidence, responsible authority,
   executor, witness, recipient, and completed effect.
4. Missing evidence, missing office holder, affected-actor refusal, invalid
   handoff, and replay mismatch remain observable without producing success.
5. Save/load and replay reproduce the same ordered decisions and consequences.
6. Neutral-kernel purity and Rust/Godot ownership checks continue to pass.

Use existing focused suites during development: `functional_lore_integration`,
kernel invariants/purity, practitioner ontology, Deck/state, Boardwalk,
and the appropriate institutional-boundary and migration suites. Add behavioral
tests for the new semantic links and refusal paths, rather than tests that only
repeat strings from the implementation. Run the Foundation Freeze's full
validation when an actual foundation migration is prepared.

The 2026-08-22 performance characterization is historical evidence, not a
current timing guarantee. If performance becomes part of the objective,
remeasure production validation, projection, and replay before optimizing.
Reuse validated immutable definitions or derived indexes where measurements
justify it; preserve authority checks, canonical ordering, and replay equality.

## Review-session record

The initial review inspected source and governing documents and added the
proposal before implementation. Runtime and canon behavior were unchanged at
that point. Current implementation and validation are recorded in
`docs/LORE_KERNEL_OPTIMIZATION_V1.md`.

| Command | Result |
|---|---|
| `cargo test --offline --test integration functional_lore_integration -- --nocapture` | 6 passed |
| `cargo test --offline --test integration kernel -- --nocapture` | 31 passed |
| `cargo test --offline --test hollow_deck flynt -- --nocapture` | 4 passed, including assertions of the superseded order |
| `cargo test --offline --test gameplay_boardwalk` | 9 passed |

These are focused baseline checks, not a full workspace or graphical lifecycle
validation. The kernel filter includes one functional-lore test also exercised
by the first command. Passing Deck tests confirm current internal consistency;
the source/amendment comparison establishes the semantic mismatch.
