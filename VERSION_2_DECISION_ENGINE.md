# VERSION 2 DECISION ENGINE

Current Synthesis (Hollow Grove) Version 2.0 adds deterministic decision selection in front of the frozen Version 1.1 execution backend.

## Mission

Version 2 reasons about possibilities.

Version 1.1 executes reality.

The Version 2 decision grammar is:

Observe
↓
Generate
↓
Evaluate
↓
Choose
↓
Recipe

Version 2 also records an immutable `DecisionTrace` for every completed decision.

The Version 1.1 execution grammar remains unchanged:

Recipe
↓
Compiler
↓
Synthesis Scripts
↓
Aim
↓
Fire
↓
Miss | Kiss
↓
Point²

## Permanent Version 1.1 Boundary

Version 2 may inspect `Point`, `FrameState`, `CurrentPrism`, `Frame`, `Flow`, `Glow`, and canonical route or manager information available through public read-only APIs.

Version 2 may not:

- mutate `FrameState`
- apply scripts directly
- call private transactional helpers
- create `Point²`
- forge `Kiss`
- convert `Miss` into `Kiss`
- bypass `Recipe`
- bypass `Compiler`
- bypass `Aim`
- bypass `Fire`
- duplicate rollback or landing logic

The only legal bridge is:

Chosen Decision
↓
Recipe
↓
Version 1.1 façade

## Observation

`DecisionObservation` is a pure immutable snapshot of:

- the current `Point`
- the current `FrameState` through that `Point`
- one explicit `DecisionIntent`
- optional observed route geometry when the observation comes from a `KernelPass`

The first deterministic intents are:

- `FavorCurrent`
- `FavorAura`
- `Neutral`

## Generation

`generate_decision_candidates(...)` returns the first proof candidates in canonical order:

1. `GremlinTinker`
2. `PixyConfusion`

Generation is pure.

Generation does not score, choose, compile, aim, fire, or land.

## Candidate Model

Each `DecisionCandidate` carries:

- stable candidate identity
- canonical manager classification
- transformation orientation

The first canonical candidates are:

- `GremlinTinker`
  - manager: `Clouseau`
  - orientation: `Current`
- `PixyConfusion`
  - manager: `HAL`
  - orientation: `Aura`

Manager-domain metadata is not duplicated inside Version 2.

Candidate manager locks resolve through the existing authoritative Version 1.1 `manager_domain_lock(...)`.

## Evaluation

`evaluate_decision_candidate(...)` is pure and deterministic.

The first scoring rule is explicit:

### FavorCurrent

- `GremlinTinker` = 2, preferred orientation
- `PixyConfusion` = 1, nonpreferred orientation

### FavorAura

- `PixyConfusion` = 2, preferred orientation
- `GremlinTinker` = 1, nonpreferred orientation

### Neutral

- `GremlinTinker` = 1, neutral baseline when not already realized
- `PixyConfusion` = 1, neutral baseline when not already realized
- an already-realized canonical outcome is penalized to `0`
- the first-proof `Hueman` state still yields `1` and `1`

No randomness, floating-point scoring, prediction, hidden weights, or generated reasoning exists in Version 2.0.

The first observation-dependent extension uses only facts already present in `FrameState`:

- `GremlinTinker` counts as already realized when the observed state is already `Gremlin` or already knows `TinkerGrip`
- `PixyConfusion` counts as already realized when the observed state is already `Pixy` or already knows `Confusion`

## Stanislavski Action Logic

The next typed V2 layer grounds candidate generation and evaluation in objective-driven action rather than plot-convenient guessing.

Canonical mapping:

- Given Circumstances = actor-limited `DecisionObservation`
- Objective = immediate `Aim`
- Super-objective = larger active purpose across multiple beats
- Obstacle = the condition blocking direct achievement
- Action / Tactic = candidate `Move` attempt against the obstacle
- Magic If = bounded consequence projection from observed facts, practiced knowledge, and uncertainty
- Unit / Beat = one `Observe -> Generate -> Evaluate -> Choose -> Execute` cycle
- Adaptation = changing tactic when execution changes circumstances
- Sense of Truth = plausibility validation for this specific Being in these specific circumstances
- Unbroken Line = continuity of purpose across multiple chosen decisions

The canonical decision question becomes:

Given these circumstances, what does this Being want, what stands in the way, and what truthful embodied action can they attempt next?

The typed Stanislavski witness surface preserves the existing execution boundary:

`Given Circumstances + Objective + Obstacle -> candidate Tactics -> Magic-If projections -> evaluated candidates -> one ChosenDecision -> Recipe -> frozen V1.1`

This layer does not reveal hidden world truth.
It projects only from:

- actor-visible circumstances
- practiced knowledge
- registered move semantics
- known Object behavior
- known route conditions
- explicit uncertainty

The canonical proof fixture is `Nightingale Hidden Wound`.
It keeps `preserve the patient's life and agency` as the active super-objective, adapts from `Aura Lesion Trace` to `Request Minorian Measurement`, and rejects `forcibly open memory` on consent, agency, and purpose-contradiction grounds.

## Decision Trace

Version 2 records deterministic immutable evidence for:

- observation
  - observed frame
  - observed flow learnset
  - observed glow learnset
  - explicit intent
  - optional observed route geometry
  - candidate-specific realized-state checks
- generation
  - canonical candidate order
  - candidate identity
  - manager identity
  - manager geometry
  - orientation
- evaluation
  - intent score
  - realized-state penalty
  - final score
  - typed reason codes
- choice
  - highest score
  - tied candidate identities
  - tie occurrence
  - typed tie-break reason
  - geometry-match evidence
  - chosen candidate
- recipe bridge
  - chosen candidate
  - canonical recipe id
  - handoff to the frozen Version 1.1 façade
- execution summary
  - Miss or Kiss
  - landed frame when Point² is produced
  - prism deltas
  - added Flow
  - added Glow
  - Point² production flag

The trace records what happened.
It does not influence what happens.

### Typed Reason Codes

Evaluation reason codes:

- `PreferredCurrentOrientation`
- `NonPreferredCurrentOrientation`
- `PreferredAuraOrientation`
- `NonPreferredAuraOrientation`
- `NeutralBaseScore`
- `AlreadyCanonicalFrame`
- `AlreadyKnowsCanonicalFlow`
- `AlreadyKnowsCanonicalGlow`

Tie-break reason codes:

- `NoTie`
- `ObservedRouteGeometryMatch`
- `CanonicalGenerateOrder`

## Choice

`choose_decision(...)` selects exactly one candidate.

Rules:

- highest score wins
- ties preserve canonical Generate order
- therefore `GremlinTinker` wins the first neutral tie

Choice fails explicitly for:

- no candidates
- missing evaluation
- duplicate candidate identity
- duplicate evaluation
- unknown evaluation identity

## Route-Aware Observation

Version 2 may also observe the current `KernelPass` through the public boundary.

`observe_kernel_pass_decision(...)` records one additional fact:

- observed route geometry

Current mapping:

- straight exterior route → `ManagerGeometry::Straight`
- curved exterior route → `ManagerGeometry::Curved`

No inverse route candidate is generated yet, so `Inverted` remains reserved for later expansion.

## Route-Aware Tie-Break

When candidate scores tie and the observation carries route geometry:

1. prefer the candidate whose canonical manager geometry matches the observed route geometry
2. if still tied, preserve canonical Generate order

This does not alter non-tied choices.

Examples:

- straight `KernelPass` + `Neutral` → choose `GremlinTinker`
- curved `KernelPass` + `Neutral` → choose `PixyConfusion`

The trace records whether observed route geometry resolved the tie or whether canonical Generate order remained the final tie-break.

## Candidate-to-Recipe Bridge

Version 2 resolves candidates into ordinary existing Version 1.1 recipes:

- `GremlinTinker` → `gremlin_tinker_recipe()`
- `PixyConfusion` → `pixy_confusion_recipe()`

Version 2 does not copy script definitions.

## Replay Verification

Version 2 includes read-only replay verification.

Replay recomputes:

- observation
- generation
- evaluation
- tie-break
- choice
- recipe bridge

Replay verifies that recomputed evidence matches a recorded `DecisionTrace`.

Replay does not execute Version 1.1 by default.
Replay does not create a new `Point²`.

## Version 1.1 Façade

Version 2 calls the Version 1.1-owned façade:

- `execute_synthesis_recipe(&Point, &SynthesisRecipe)`

That façade alone performs:

- recipe compilation
- canonical aim construction
- fire contact
- transactional landing

Version 2 never calls landing internals directly.

## Canonical Manager Classification

The authoritative manager-domain lock remains:

- `HAL` = `META` = `PLEB ↔ META` = `Curved` = `Information From Beyond`
- `Clouseau` = `PLEB` = `PLEB ↔ PLEB` = `Straight` = `Bond`
- `Cleopatra` = `BLEP` = `PLEB ↔ BLEP` = `Inverted` = `Underworld Reflection`

The first Version 2 proof uses:

- `GremlinTinker` through `Clouseau`
- `PixyConfusion` through `HAL`

No Cleopatra candidate is generated yet.

## Canonical Version 2 Fixtures

### Current-Favored

Input:

- `Hueman`
- `DecisionIntent::FavorCurrent`

Decision:

- choose `GremlinTinker`

Version 1.1 result:

- `Kiss`
- `Gremlin Point²`
- `Body 1 → 3`
- `Flow +TinkerGrip`

### Aura-Favored

Input:

- `Hueman`
- `DecisionIntent::FavorAura`

Decision:

- choose `PixyConfusion`

Version 1.1 result:

- `Kiss`
- `Pixy Point²`
- `Mind 1 → 3`
- `Glow +Confusion`

### Neutral

Input:

- `Hueman`
- `DecisionIntent::Neutral`

Decision:

- equal scores
- tie-break by Generate order
- choose `GremlinTinker`

Version 1.1 result:

- ordinary canonical `Gremlin` landing

### Neutral From Straight KernelPass

Input:

- `Hueman`
- straight observed route geometry
- `DecisionIntent::Neutral`

Decision:

- equal scores
- tie-break by observed straight geometry
- choose `GremlinTinker`

### Neutral From Curved KernelPass

Input:

- `Hueman`
- curved observed route geometry
- `DecisionIntent::Neutral`

Decision:

- equal scores
- tie-break by observed curved geometry
- choose `PixyConfusion`

### Neutral From Realized Canonical States

Input:

- observed `Pixy` with `Confusion`
- `DecisionIntent::Neutral`

Decision:

- `PixyConfusion` is already realized and scores `0`
- `GremlinTinker` scores `1`
- choose `GremlinTinker`

Input:

- observed `Gremlin` with `TinkerGrip`
- `DecisionIntent::Neutral`

Decision:

- `GremlinTinker` is already realized and scores `0`
- `PixyConfusion` scores `1`
- choose `PixyConfusion`

## Concise Witness

The desktop witness now includes a concise Version 2 section for:

- `CURRENT-FAVORED`
- `AURA-FAVORED`
- `NEUTRAL`

Each section shows:

- Observe
- State checks
- Generate
- Evaluate
- Choose
- Recipe
- Version 1.1 execution evidence
- typed trace evidence

This keeps decision evidence separate from execution evidence.

## Forbidden Bypasses

Version 2.0 forbids:

- direct script lists inside Version 2 candidates
- direct `Aim` construction from Version 2
- direct `Fire` orchestration from Version 2
- direct `land_contact(...)` usage from Version 2
- direct `Point²` fabrication
- mutation of the source `Point`
- mutation of the source `FrameState`

## Future Extension Points

Version 2 may later extend:

- observation inputs
- candidate generation breadth
- evaluation rules
- additional deterministic tie-breaks
- additional legal candidate families

Version 2.0 does not yet add:

- living-world simulation
- procedural decisions
- route economics
- inventory or crafting state
- NPC policy
- probabilistic AI
- generated recipes

## Freeze Statement

Version 2.0 proves only that one deterministic decision can cross into the frozen Version 1.1 backend through an ordinary `Recipe`.

It does not redesign or weaken Version 1.1.
