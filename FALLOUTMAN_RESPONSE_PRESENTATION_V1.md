# FALLOUTMAN RESPONSE PRESENTATION V1

Falloutman is the presentation layer for Hollow Grove.

It does not replace Stanislavski V2 and it does not replace Hollow Grove action mechanics.

Canonical separation:

- Stanislavski = decision logic
- Hollow Grove = action mechanics
- Falloutman = player-facing presentation

The sentence is presentation.

The tactic is the real choice.

## Purpose

Falloutman presents typed tactics as readable response options in conversations and other dramatic beats.

It is not:

- a dialogue tree
- a morality wheel
- a second decision engine
- a direct world-state mutator

Each visible response wraps an existing typed Stanislavski candidate tactic and preserves the action boundary:

`AttemptedAction -> V2 ChosenDecision -> Recipe -> frozen V1.1`

## Response Wrapper

The presentation layer uses typed wrappers over the existing Stanislavski beat data.

Core wrapper fields:

- `visible_text`
- `presentation_kind`
- `visible_tags`
- `candidate_tactic_id`
- `attempted_action`
- `availability`
- `projection_summary`

Mechanical truth remains in the wrapped action:

- `Being`
- `Skill`
- `Domain`
- `Gesture`
- `Mode`
- `Object`
- `AddressingMode`
- `Aim`
- optional route
- optional Aura polarity

## Presentation Kinds

Supported kinds:

- `Spoken`
- `Nonverbal`
- `PhysicalAction`
- `MedicalAction`
- `ToolAction`
- `Silence`
- `Leave`

Spoken and nonverbal responses coexist in the same menu.

## Visible Tags

Compact menu tags come from typed mechanics.

Canonical examples:

- `SHOW • LIGHT • BEAM`
- `GRIT • LIGHT • GLEAM`
- `SHOW • DARK • BEAM`
- `GRIP • FLOW • SEAM`
- `LEAVE`

Inspector output may expose more detail, including `AddressingMode`, route, skill, risk, uncertainty, and recipe status.

## Availability

Each response is one of:

- `Available`
- `UnavailableVisible`
- `Hidden`

`Available` responses are selectable.

`UnavailableVisible` responses remain on-screen with a grounded reason such as `ROUTE`, `SKILL`, `ROLE`, or `RISK`.

`Hidden` responses are omitted because the actor cannot truthfully conceive or attempt them.

In the Hidden Wound slice:

- `forcibly open memory` remains hidden from the player-facing menu
- `Request Minorian Measurement` is visible but unavailable in beat one because no measurement route is established yet
- the same measurement response becomes available in beat two after the trace changes the circumstances

## Actor-Limited Observation

Falloutman preserves the existing Stanislavski constraint:

world truth != actor observation

Visible response text must not reveal hidden facts that the acting Being has not legally observed, inferred, diagnosed, or learned.

In the Hidden Wound slice, the initial menu does not reveal:

- Siren interference as confirmed fact
- Riptide as confirmed cause
- manipulated memory attachment as confirmed truth

The witness surface may still render hidden state explicitly for debugging, but it is marked as not shown in the initial menu.

## Magic-If and Sense of Truth

Falloutman reuses Stanislavski V2 evaluation data instead of inventing new projections.

It shows compact summaries of:

- likely immediate consequence
- risk
- uncertainty
- objective progress
- super-objective alignment
- agency effect
- recipe status

Magic If remains bounded projection, not prophecy.

Sense of Truth remains plausibility, not morality.

## NPC and Player Loop

NPCs use the same Stanislavski/V2 mechanics as players.

NPC loop:

`observation -> objective -> obstacle -> candidate tactics -> V2 choose -> Recipe -> frozen V1.1`

Player loop:

`scene prompt -> Stanislavski candidate tactics -> Falloutman response options -> player selection -> AttemptedAction wrapper -> V2 choose -> Recipe -> frozen V1.1`

There is no separate NPC dialogue engine.

## Hidden Wound Vertical Slice

The first Falloutman proof slice is:

`The Hidden Wound and the Riptide Misdirection`

Scene state:

- Glaüshouse clinic
- distressed injured Hueman
- shallow breathing
- inconsistent recollection
- abnormal Aura signal
- examination consent granted
- memory alteration consent not granted

NPC dramatic core includes:

- surface objective
- true objective
- super objective
- secret
- pressure point
- red line
- relationship state
- known evidence

Beat one menu includes:

- `Tell me what happened.`
- `You're safe. Take your time.`
- `I know you're hiding something.`
- `Stabilize the wound.`
- `Trace the lesion pattern.`
- `Route the signal for Minorian measurement.` (unavailable)
- `End examination.`

Canonical beat one selection:

- `TraceAuraLesion`
- Stanislavski chosen tactic: `Aura Lesion Trace`

Beat two adapts after changed circumstances.

Canonical beat two selection:

- `RequestMinorianMeasurement`
- Stanislavski chosen tactic: `Request Minorian Measurement`

The second menu differs because the measurement response becomes available and the scene prompt changes.

## CLI and TUI Surfaces

CLI/TUI inspection commands:

- `hollow-grove falloutman witness`
- `hollow-grove falloutman validate`
- `hollow-grove falloutman menu witness`
- `hollow-grove falloutman beat witness`
- `hollow-grove falloutman hidden-wound witness`

These commands are also routed through `current_synthesis_tui`.

The TUI surface is read-only and uses the same source-of-truth runtime structures as CLI and tests.

## Tests

Positive coverage includes:

- spoken response wraps typed tactic
- nonverbal response wraps typed tactic
- visible text remains presentation-only
- candidate tactic id remains linked
- hidden truth absent from initial menu
- unavailable option has grounded reason
- beat one preserves `Aura Lesion Trace`
- beat two preserves `Request Minorian Measurement`
- second menu differs from the first
- Light, Dark, and unoriented responses coexist
- Leave option remains available
- Recipe remains required

Negative coverage includes:

- no second decision engine
- no direct world mutation
- no direct relationship mutation
- no direct practice grant
- no V2 bypass
- no Recipe bypass
- no CurrentPrism mutation
- no capacity mutation
- no Point³
- no Position 13
- no automatic Aura Frame grant

## Persistence and Compatibility

Persistence decision:

- no save migration
- no new persisted dialogue tree
- no new persistent morality state

Compatibility:

- Stanislavski V2 remains the selector
- Hollow Grove mechanics remain the action truth
- `ChosenDecision` remains one assignment
- frozen V1.1 remains the executor
- foundation checkpoint remains unchanged

Falloutman is a presentation wrapper over existing typed decision and action structures, not a second gameplay substrate.
