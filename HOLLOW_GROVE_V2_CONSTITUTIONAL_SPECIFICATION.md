# Hollow Grove V2 Constitutional Specification

Status: canonical shared Bond and Constitutional Runtime specification
Authority: normative within its delegated runtime domain beneath the Hollow Grove Compromise
Date: 2026-07-18

## 1. Status, Scope, and Conformance

### 1.1 Constitutional status

The Hollow Grove Constitution is
`HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`. This document is its detailed normative
specification for the shared Bond grammar and Constitutional Runtime. It is not
a second Hollow Grove Constitution, a House constitution, or a superior
sovereign source.

Within its delegated runtime domain, this document is the primary technical
authority for the Hollow Grove concepts that it defines.

An implementation conforms only when its externally observable behavior, persisted history, state transitions, validation outcomes, and replay results satisfy every applicable MUST, MUST NOT, SHALL, and SHALL NOT requirement in this document.

Repository code, tests, comments, generated artifacts, earlier specifications, and implementation names are evidence of existing behavior. They do not override this document where their behavior is narrower, incomplete, or contradictory.

Existing frozen execution behavior remains valid where it does not violate this document. In particular, the existing Recipe, compiler, Aim, Fire, Miss, Kiss, and Point² transaction path remains a lawful execution mechanism. It is not, by itself, the complete constitutional Bond lifecycle.

### 1.2 Scope

This document specifies:

- Wave as the causal predecessor of Current movement;
- signed Current;
- signed Aura;
- Bond;
- Bond term and maturity;
- Bond formation, validation, activation, circulation, accumulation, observation, evaluation, default, challenge, appeal, condensation, and resolution;
- Tombstone;
- Toke;
- Synthesis Resolution;
- institutional and House authority as they apply to those concepts;
- Recipe and transformation participation in a Bond;
- ownership, inheritance, parentage, and succession;
- evidence, witnesses, deterministic replay, persistence, serialization, and migration;
- constitutional verification requirements;
- architectural mapping into the existing repository.

This document does not redesign unrelated Hollow Grove world, presentation, route, species, profession, desktop, or Hueman mechanics.

Major route purpose is governed separately by `HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md`. That upper world-facing law may project Constitutional Runtime state but MUST NOT alter the lifecycle, authority, persistence, replay, or causality specified here.

### 1.3 Normative language

The words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are normative.

- MUST and SHALL establish an unconditional constitutional requirement.
- MUST NOT and SHALL NOT establish an unconditional constitutional prohibition.
- SHOULD establishes a strong implementation direction that may be departed from only with recorded constitutional justification.
- MAY establishes lawful implementation freedom.

Descriptive examples never weaken a normative rule.

### 1.4 Constitutional precedence

When two rules appear to conflict, the following precedence applies:

1. causal integrity;
2. non-creation and non-destruction of Current;
3. preservation of validated historical fact;
4. required lifecycle ordering;
5. authority and jurisdiction;
6. deterministic accounting and replay;
7. local governing Bond rules;
8. implementation convenience.

A lower-precedence rule MUST NOT be used to violate a higher-precedence rule.

### 1.5 Conformance boundary

A conforming implementation MUST reject an illegal state before that state becomes authoritative.

Rejection MUST:

- identify the violated rule;
- preserve every previously committed lawful fact;
- commit no partial successor state;
- create no Tombstone;
- create no Toke;
- perform no Synthesis Resolution that assumes the rejected transition occurred.

Automatic correction of constitutional history is forbidden.

### 1.6 Constitutional questions

Every authoritative transition MUST be able to answer:

- Who participated?
- What occurred?
- Why was it lawful?
- When did it occur in causal order?
- Where did authority reside?
- How did Current move?
- How was Aura observed?
- Who owned or held custody of Current before and after the movement?
- Which House governed?
- Which Institution governed?
- Which obligation or permission authorized the transition?
- Which evidence supports the claim?
- Which witness observed or attested?
- Which validator decided legality?
- Which resolver concluded the Bond?
- Can an independent replay derive the same result?

An implementation that cannot answer an applicable question is constitutionally incomplete.

## 2. Constitutional Vocabulary and Type Distinctions

### 2.1 Core law

Bond governs.

Wave moves.

Current carries.

Aura reveals.

Current condenses.

Tombstone proves.

Toke records.

Synthesis resolves.

### 2.2 Wave

A Wave is a lawful causal origin or causal continuation that precedes a Current movement.

A Wave is not Current. It authorizes or causes movement; it does not replace the moved quantity.

Every Current movement MUST reference exactly one immediate causal Wave. A Wave MAY itself reference predecessor evidence, prior Waves, a parent Bond, a Recipe execution, a transformation, an institutional act, or another lawful cause.

A Wave MUST be identifiable, replayable, and ordered before every movement that cites it.

### 2.3 Current

Current is signed constitutional capacity or burden that can move.

Positive Current represents productive capacity.

Negative Current represents burden.

Negative Current is not missing Positive Current. Positive and Negative Current are independently real historical quantities.

The sign of Current is distinct from:

- Regular Current;
- Hollow Current;
- Current Form;
- Flow;
- CurrentPrism;
- Current Capacity;
- a Current-favored decision orientation;
- semantic side, route side, or screen position.

Those existing concepts MAY describe state, form, capability, storage, orientation, or presentation. None determines constitutional Current sign without an explicit governing rule.

### 2.4 Aura

Aura is signed constitutional revelation of Current and its surrounding relationship.

Positive Aura represents truthful visibility, clarity, legitimacy, recognition, witness, or institutional acceptance.

Negative Aura represents concealment, distortion, ambiguity, illegitimacy, rejection, or obscured recognition.

Negative Aura is not an absence of Aura. Absence of observation and a recorded Negative Aura observation are different facts.

The sign of Aura is distinct from:

- Reflective Aura;
- Holographic Aura;
- Light Aura;
- Dark Aura;
- Glow;
- Aura Frame;
- Aura Capacity;
- an Aura-favored decision orientation;
- route geometry or semantic side.

The distinction is REQUIRED. Existing Hollow Grove law permits Dark Aura to be proportional and defensible and permits conduct labeled Light Aura to be coercive or false. Therefore Light MUST NOT be automatically stored as Positive Aura, and Dark MUST NOT be automatically stored as Negative Aura. The constitutional sign MUST be derived from the observation and governing rules.

### 2.5 Bond

A Bond is a deterministic causal container with:

- exactly one immutable identity;
- exactly one formation;
- one finite or explicitly perpetual term;
- one or more explicit participants;
- explicit roles;
- an initiating Wave;
- a governing House;
- a governing Institution;
- obligations;
- permissions;
- signed Current accounting;
- signed Aura observation;
- a lawful lifecycle;
- zero or one Tombstone;
- zero or one Toke;
- exactly one final resolution if the Bond reaches final resolution.

A Bond is not merely a contract, relationship, debt, transaction, selection candidate, route link, or ownership record. Those may participate in or specialize a Bond.

### 2.6 Tombstone

A Tombstone is immutable durable proof formed from the non-zero mature net excess of exactly one Bond.

A Tombstone is not:

- the Bond;
- the live Current ledger;
- an evaluation guess;
- a cache;
- a mutable summary;
- a Toke;
- Synthesis Resolution.

### 2.7 Toke

A Toke is an immutable permanent historical record that points to exactly one validated Tombstone.

A Toke does not prove a Bond by itself. It records that proof has already been validated and makes the proof addressable in constitutional history.

### 2.8 Synthesis Resolution

Synthesis Resolution is the final deterministic constitutional process that resolves a Bond after every applicable preceding lifecycle gate has completed.

Synthesis Resolution is distinct from condensation.

Synthesis Resolution is also distinct from a Recipe execution. A Recipe execution MAY be an input, obligation, transformation, or evidence item within a Bond. It does not become final Bond resolution merely because execution produced a Kiss or Point².

### 2.9 Evidence

Evidence is an immutable reference to an authoritative fact, trace, artifact, observation, validation, execution, or external record.

Evidence MUST be referenced rather than silently copied when another domain owns the detailed record.

Evidence is not automatically true merely because it exists. Evidence integrity and relevance MUST be validated.

### 2.10 Witness

A Witness is an explicit participant or authoritative external observer that attests to a defined fact.

A Witness MUST have:

- stable identity;
- explicit witnessed scope;
- explicit relationship to the Bond or evidence;
- applicable authority or observational standing;
- a deterministic attestation record.

A witness statement MUST NOT substitute for Current accounting or causal replay.

### 2.11 House and Institution

The governing House establishes the constitutional function under which the Bond is evaluated.

The governing Institution establishes the concrete jurisdiction and authority under which the Bond is formed, validated, observed, challenged, recorded, or resolved.

House and Institution are distinct. A House is not an Institution, and an Institution does not acquire House authority merely by naming a House.

### 2.12 Recipe and transformation

A Recipe defines a lawful transformation procedure.

A transformation is an executed change governed by an applicable Recipe or other already-authorized transformation rule.

Neither a Recipe nor a transformation is a Bond. A Bond records how a Recipe or transformation participates, who authorized it, what Current moved, what Aura revealed, and what lasting consequence remained.

## 3. Deterministic Value Classification

### 3.1 Immutable values

The following values MUST become immutable at the stated boundary:

- Bond identity: at formation;
- initiating Wave reference: at formation;
- original participant and role declarations: at formation;
- governing House and governing Institution: at formation;
- original term: at formation;
- committed Current movement: when the movement is accepted;
- committed Aura observation: when the observation is accepted;
- committed evidence reference: when attached to a historical event;
- maturity boundary: when maturity is declared;
- mature accounting snapshot: at maturity;
- net excess result: when excess calculation succeeds;
- Tombstone payload: at Tombstone formation;
- Tombstone validation decision: when committed;
- Toke payload: at recording;
- Synthesis Resolution record: at final resolution.

Later events MAY challenge, supersede, interpret, or build upon an immutable value. They MUST NOT rewrite it.

### 3.2 Mutable values

Before maturity, the following living Bond projections MAY change only through lawful recorded transitions:

- active participant availability;
- custody assignments;
- open obligation status;
- permission status;
- accumulated Current totals;
- accumulated Aura totals;
- current evaluation result;
- challenge status;
- default findings;
- evidence sufficiency status;
- unresolved Current;
- projected maturity readiness.

Mutable projections MUST be derived from immutable committed events. Direct unrecorded mutation is forbidden.

### 3.3 Derived values

The following values MUST be derived:

- positive Current total;
- negative Current total;
- net Current;
- incoming Current;
- outgoing Current;
- retained Current;
- transferred Current;
- resolved Current;
- unresolved Current;
- positive Aura total;
- negative Aura total;
- net Aura;
- current constitutional polarity state;
- term completion;
- maturity readiness;
- net excess;
- condensation eligibility;
- replay outcome;
- lookup indexes.

Derived values MUST NOT be accepted as authoritative without the inputs from which they can be recomputed.

### 3.4 Persistent values

The authoritative event history, formed Bond declaration, maturity declaration, net excess decision, Tombstone, Tombstone validation decision, Toke, and final Synthesis Resolution MUST persist for the repository lifetime required by constitutional history.

Caches and indexes MAY be discarded and rebuilt. Constitutional records MUST NOT depend upon cache survival.

### 3.5 Computed values

Computed values MUST:

- use exact arithmetic;
- define units and scale;
- reject overflow and underflow;
- avoid floating-point dependence;
- use stable ordering;
- produce identical results from identical inputs;
- preserve positive and negative gross values separately even when a net value is also computed.

Saturating arithmetic is forbidden for constitutional accounting because it conceals loss of Current.

### 3.6 Replayable values

Every value used to validate maturity, excess, condensation, Tombstone validity, Toke recording, or Synthesis Resolution MUST be replayable from:

- the formed Bond declaration;
- the ordered event history;
- referenced governing rules;
- referenced evidence;
- referenced authority state at the relevant causal positions.

## 4. Signed Current and Signed Aura

### 4.1 Signed quantity representation

A constitutional signed quantity MUST be represented without ambiguity as:

- a sign, Positive or Negative; and
- a strictly positive exact magnitude.

Zero is a lawful aggregate result but is not a lawful Current movement or Aura observation contribution.

Mixed contributions MUST be decomposed into separate signed entries. A single entry MUST NOT claim both signs.

#### Common domain model

Every Current movement entry MUST contain or reference:

- entry identity;
- Bond identity;
- immediate causal Wave identity;
- sign;
- exact magnitude;
- unit and scale;
- source and destination;
- ownership and custody before and after movement;
- obligation or permission authorizing movement;
- causal position;
- authority;
- evidence;
- schema version.

Every Aura observation entry MUST contain or reference:

- entry identity;
- Bond identity;
- observer or recognizing authority identity;
- sign;
- exact magnitude;
- observation scope;
- observed Current movements, events, obligations, or causal range;
- causal position;
- jurisdiction;
- evidence;
- schema version.

Entry identity, sign, magnitude, unit, causal position, authority, and evidence references become immutable when the entry commits.

Current ownership and custody are explicit. Aura observations are attributable to their observer or recognizing authority but do not confer ownership of the Current they reveal.

Committed entries persist as historical facts. Access policy MAY restrict visibility; it MUST NOT alter sign, magnitude, causal position, or existence.

Derived totals are mutable projections before maturity and immutable mature projections after maturity. The committed entries from which those totals are derived are never mutable.

#### Common lifecycle

Each proposed Current movement or Aura observation progresses through:

1. Proposed;
2. Validated;
3. Committed; and
4. Historical at or after the applicable maturity boundary.

Proposed MAY transition to Rejected. Rejected is terminal for that proposal and contributes no signed quantity.

Validated MAY transition only to Committed or Rejected-before-commit. A persistence interruption does not create a partial committed entry.

Committed MAY transition only to Historical as a classification of lifetime. Historical does not replace, edit, or remove the committed payload.

Retry with the same intended identity and identical payload is idempotent. Retry with the same identity and conflicting payload is corruption.

After Bond maturity, no new Current movement or Aura observation may be committed into the mature causal range. A later statement about mature history is challenge evidence in a new lawful event or successor Bond; it is not a backdated entry.

#### Common constitutional questions

Every signed entry answers:

- Which Bond contains it?
- Which signed domain does it affect?
- Is its sign Positive or Negative?
- What is its exact non-zero magnitude and unit?
- What is its causal position?
- Who authorized or observed it?
- Which evidence supports it?
- Is it living history or immutable mature history?

Every Current entry additionally answers where Current came from, where it went, and who owned and held custody before and after movement.

Every Aura entry additionally answers what it reveals, conceals, clarifies, recognizes, rejects, or renders ambiguous and who had standing to make that observation.

### 4.2 Positive Current

#### Purpose

Positive Current records productive capacity carried by a Wave through a Bond.

#### Responsibilities

Positive Current MUST:

- retain its causal origin;
- retain its unit and magnitude;
- identify source and destination custody;
- participate in positive accumulation;
- remain available for mature offset and excess calculation;
- remain distinct from recognition or moral approval.

#### Non-responsibilities

Positive Current MUST NOT:

- imply Positive Aura;
- imply lawful conduct;
- imply successful maturity;
- erase Negative Current;
- authorize itself;
- create a Tombstone before maturity.

#### Required invariants

- Every Positive Current movement has one prior Wave.
- Every Positive Current magnitude is greater than zero.
- Every split preserves the input magnitude across outputs.
- Every merge preserves the sum of inputs.
- Every boundary entry identifies an external source.
- Every boundary exit identifies an external destination.
- Positive Current history remains positive even if later offset by Negative Current.

#### Illegal states

- Positive Current without a source.
- Positive Current with zero magnitude.
- Positive Current represented only as a net after negative offset.
- Positive Current silently converted to Positive Aura.
- Positive Current duplicated by split, transfer, branch, inheritance, or migration.

#### Verification

Tests MUST prove conservation, sign preservation, source legality, exact accumulation, overflow rejection, and survival of gross history after netting.

### 4.3 Negative Current

#### Purpose

Negative Current records real burden carried by a Wave through a Bond.

#### Responsibilities

Negative Current MUST:

- retain its causal origin;
- retain its exact magnitude;
- identify who bears or holds custody of the burden;
- participate in negative accumulation;
- remain available for default, maturity, offset, and excess calculation;
- remain independently visible even when positive capacity is larger.

#### Non-responsibilities

Negative Current MUST NOT:

- be treated as missing data;
- be discarded because it is undesirable;
- imply Negative Aura;
- imply default without an obligation rule;
- be converted into a positive magnitude without retaining its sign.

#### Required invariants

- Every Negative Current movement has one prior Wave.
- Every Negative Current magnitude is greater than zero.
- A transferred burden leaves one custodian exactly as it enters another, subject to a recorded boundary transfer.
- Negative Current history remains negative even if later offset by Positive Current.
- Default MAY produce or reveal a negative mature remainder only through already-governing rules; the Bond does not invent the burden.

#### Illegal states

- Negative Current stored as unsigned loss.
- Negative Current erased by positive offset.
- Negative Current assigned to a participant without authority, ownership, or transfer evidence.
- Negative Current appearing from an unrecorded default calculation.
- Negative Current silently treated as Negative Aura.

#### Verification

Tests MUST prove burden conservation, custody continuity, gross-history preservation, exact negative accumulation, lawful default treatment, and replay equality.

### 4.4 Positive Aura

#### Purpose

Positive Aura records truthful constitutional revelation, recognition, legitimacy, clarity, witness, or acceptance.

#### Responsibilities

Positive Aura MUST:

- identify the observer or recognizing authority;
- identify the Current, event, obligation, or Bond range being observed;
- identify evidence supporting the observation;
- preserve its observation time in causal order;
- participate in positive Aura accumulation and polarity evaluation.

#### Non-responsibilities

Positive Aura MUST NOT:

- add Current;
- remove Current;
- prove that Positive Current exists without Current evidence;
- convert an unlawful act into a lawful act;
- be inferred solely from a Light label, House, route, or presentation.

#### Required invariants

- Every Positive Aura contribution has a strictly positive magnitude.
- Every contribution has an explicit observer or authority.
- Positive Aura may reveal Positive or Negative Current.
- Positive Aura never changes historical Current.
- Recognition after an event does not retroactively move the event in causal order.

#### Illegal states

- Positive Aura without observer or recognizing authority.
- Positive Aura without an observation scope.
- Positive Aura with zero magnitude.
- Positive Aura created solely because Current is Positive.
- Positive Aura inferred solely from Light, House, route, or presentation state.
- Positive Aura used to increase or erase Current.
- Positive Aura backdated into mature history.
- Conflicting Positive Aura entries collapsed by deletion rather than evaluated.

#### Verification

Tests MUST prove observer identity, scope, evidence linkage, sign independence from Current, and inability to mutate Current.

### 4.5 Negative Aura

#### Purpose

Negative Aura records constitutional concealment, distortion, ambiguity, illegitimacy, rejection, or obscured recognition.

#### Responsibilities

Negative Aura MUST:

- identify the observer, rejecting authority, or evidenced condition;
- identify the Current or event scope affected;
- preserve the distinction between concealment and absence of observation;
- participate in negative Aura accumulation and polarity evaluation;
- remain available to challenge and appeal.

#### Non-responsibilities

Negative Aura MUST NOT:

- subtract Current;
- rewrite Current;
- automatically make Positive Current negative;
- be inferred solely from a Dark label;
- be silently discarded when a later observer provides Positive Aura.

#### Required invariants

- Every Negative Aura contribution has a strictly positive magnitude.
- Negative Aura may reveal or obscure Positive or Negative Current.
- Conflicting Aura observations coexist in history until lawfully evaluated.
- Later clarity does not delete earlier concealment or distortion.

#### Illegal states

- Negative Aura without observer, rejecting authority, or evidenced condition.
- Negative Aura without an observation scope.
- Negative Aura with zero magnitude.
- Negative Aura used as a substitute for absent observation.
- Negative Aura created solely because Current is Negative.
- Negative Aura inferred solely from Dark, House, route, or presentation state.
- Negative Aura used to reduce or reverse Current.
- Earlier Negative Aura deleted when later Positive Aura is committed.
- Negative Aura backdated into mature history.

#### Verification

Tests MUST distinguish no observation from Negative Aura, preserve conflicting observations, and prove that Negative Aura cannot change Current totals.

### 4.6 Current and Aura interaction

Current and Aura are evaluated together but accounted separately.

Aura MUST NOT offset Current.

Current MUST NOT determine Aura by itself.

For every successful constitutional evaluation:

- positive Current is evaluated;
- negative Current is evaluated;
- positive Aura is evaluated;
- negative Aura is evaluated;
- the applicable evidence and authority are evaluated;
- the Current result and Aura result are each independently signed;
- exactly one four-polarity state is produced.

If either domain has an exact zero net result or lacks sufficient evidence for a sign, the evaluation is incomplete. Incomplete evaluation is a gate status, not a fifth polarity state. An incompletely evaluated Bond MUST NOT condense.

### 4.7 Four constitutional polarity states

The only successful evaluation states are:

| Current sign | Aura sign | Constitutional meaning |
|---|---|---|
| Positive Current | Positive Aura | Productive capacity is positively revealed or recognized. |
| Positive Current | Negative Aura | Productive capacity exists while being concealed, distorted, rejected, or rendered illegible. |
| Negative Current | Positive Aura | Burden exists and is truthfully revealed, witnessed, recognized, or accepted. |
| Negative Current | Negative Aura | Burden exists while being concealed, distorted, rejected, or rendered illegible. |

Positive Current bonding to Negative Aura is lawful and MUST be represented directly.

Negative Current bonding to Positive Aura is lawful and MUST be represented directly.

No state is automatically moral, successful, beneficial, criminal, official, or terminal.

The two signs MUST NOT be collapsed into one score.

### 4.8 Signed-domain state machine

Legal evaluation transitions are:

- Unevaluated to one of the four polarity states after complete evaluation.
- Any polarity state to any other polarity state before maturity when new lawful Current or Aura history changes the derived signs.
- Any polarity state to EvaluationPending when a challenge suspends reliance on its inputs.
- EvaluationPending to a polarity state after deterministic reevaluation.
- The final pre-maturity polarity state to one immutable mature polarity state at maturity.

Illegal transitions are:

- direct mutation of a polarity state without new history or reevaluation;
- a fifth successful polarity state;
- change of mature polarity without a successful post-record challenge Bond;
- sign change caused only by display text or collection order;
- Aura sign overwriting Current sign;
- Current sign overwriting Aura sign.

### 4.9 Signed-domain failure modes

Failure modes include:

- arithmetic overflow or underflow;
- missing Wave;
- invalid unit conversion;
- duplicate movement;
- broken custody chain;
- missing observer;
- observation outside jurisdiction;
- contradictory evidence;
- exact net zero at a required sign gate;
- ambiguous Aura;
- missing governing offset rule;
- nondeterministic collection order.

Failure MUST suspend or reject the affected evaluation. It MUST NOT guess a sign.

### 4.10 Repository mapping

The current repository contains related but non-equivalent structures:

- CurrentPrism and PrismDelta represent attribute state and signed attribute change, not the Bond Current ledger.
- ResourceComposition stores unsigned Aura and Current property totals, not signed constitutional accounting.
- SemanticSide Left and Right labels Aura and Current families, not positive and negative sign.
- AuraPolarity Light and Dark describes Glow orientation, not constitutional Aura sign.
- InferredAuraOrientation provides useful truth, consent, agency, pressure, proportionality, reversibility, and consequence evidence, but MUST pass through signed Aura evaluation.
- Regular and Hollow Current describe Current state, not sign.
- Reflective and Holographic Aura describe Aura state, not sign.

These structures SHOULD be refined through adapters or explicit projections. They SHOULD NOT be replaced merely to rename them, and they MUST NOT be silently reinterpreted as signed accounting.

### 4.11 Wave

#### Purpose

Wave exists to answer:

> What lawful cause made this Current begin moving now?

Without Wave, Current would appear without causal origin.

#### Responsibilities

A Wave MUST:

- possess stable identity;
- identify its causal predecessor or authorized origin;
- identify its issuing participant, process, Institution, Recipe, transformation, or prior Bond;
- identify the Bond or proposed Bond scope it affects;
- precede every Current movement that cites it;
- identify authority;
- identify evidence;
- be immutable after a dependent movement commits;
- be replayable.

#### Non-responsibilities

A Wave MUST NOT:

- contain Current as a substitute for movement accounting;
- create Current inside a Bond;
- imply activation;
- imply permission where no permission exists;
- alter Aura;
- mature a Bond;
- form proof.

#### Domain model

A Wave record MUST include:

- Wave identity;
- schema version;
- origin reference;
- issuer identity;
- authority reference;
- cause reference;
- intended Bond or relationship scope;
- causal position;
- evidence;
- optional predecessor Wave references where governing causality requires them.

#### Lifecycle

A Wave is:

- proposed;
- validated as a lawful cause;
- committed;
- referenced by zero or more causally dependent events;
- historical.

A committed Wave is immutable.

A proposed Wave that never becomes lawful has no Current movement and is not authoritative history unless the governing process records the failed proposal as evidence.

#### State machine

Legal transitions are:

- Proposed to Validated.
- Proposed to Rejected.
- Validated to Committed.
- Committed to Historical after its dependent causal boundary passes.

Committed and Historical Waves MUST NOT return to Proposed.

#### Constitutional questions

Wave answers:

- Who or what initiated movement?
- Under which authority?
- For what cause?
- Which Bond does it affect?
- Which evidence supports the cause?
- What is its position before Current?

#### Required invariants

- Wave identity is unique.
- Wave precedes Current.
- A movement references exactly one immediate Wave.
- A Wave MAY cause multiple movements only when governing rules explicitly define their shared causal origin.
- A Wave cannot be its own predecessor.
- Wave predecessor graph is acyclic.
- Rejected Wave moves no Current.
- Wave history is immutable after dependent movement.

#### Illegal states

- Current without Wave.
- Current preceding Wave.
- Wave without issuer.
- Wave without cause.
- Wave without authority.
- Self-predecessor.
- Causal cycle.
- Rejected Wave cited by movement.
- Wave changed after movement.
- Wave identity based on randomness or insertion order.

#### Failure modes and recovery

Missing authority, missing origin, causal cycle, or rejected evidence prevents commitment.

Interrupted Wave commitment produces no lawful dependent Current until the Wave is durably committed.

Retry is idempotent.

#### Verification

Tests MUST cover lawful origin, prior-Bond origin, Recipe origin, rejection, one-to-many authorized use, cycle detection, ordering, restart, mutation rejection, and Current-without-Wave rejection.

#### Repository mapping

The implemented canonical Wave type is `constitutional::WaveRecord`, submitted
through `ConstitutionalRuntime::record_wave`. Kernel Symptom, Triway,
DecisionObservation, CurrentSynthesisEvent, Recipe execution, and institutional
events may act as evidence or origins, but none is itself a Wave. The
`record_kernel_wave` adapter records a completed pass without moving Current.

## 5. Bond

### 5.1 Purpose

Bond exists to answer:

> What is the lasting constitutional consequence of this relationship?

The engine cannot lawfully mature Current, condense excess, form a Tombstone, record a Toke, or perform final Synthesis Resolution without a Bond.

Bond is the sole constitutional container in which signed Current and signed Aura become a governed causal history.

### 5.2 Responsibilities

A Bond SHALL:

- establish constitutional existence;
- possess one immutable identity;
- identify every participant;
- identify every participant role;
- identify the initiating Wave;
- identify the governing House;
- identify the governing Institution;
- define obligations;
- define permissions;
- define a finite or explicitly perpetual term;
- identify starting Current;
- identify initial Aura state;
- validate participant legality;
- validate role legality;
- validate institutional and House jurisdiction;
- validate ownership and custody;
- validate Recipe and transformation eligibility;
- activate exactly once;
- accept only lawful Current movements;
- preserve causal order;
- account for Positive and Negative Current separately;
- observe Positive and Negative Aura separately;
- accumulate deterministic history;
- evaluate all four signed inputs;
- permit challenge, investigation, witness, clarification, and correction without rewriting history;
- determine maturity from the term;
- compute mature excess;
- determine condensation eligibility;
- form at most one Tombstone;
- submit that Tombstone to independent validation;
- permit at most one Toke recording;
- resolve exactly once;
- support deterministic replay;
- preserve history permanently.

### 5.3 Non-responsibilities

A Bond SHALL NOT:

- invent Current;
- destroy Current;
- invent Aura;
- invent evidence;
- fabricate a witness;
- infer an undeclared participant;
- infer an undeclared role;
- grant House or institutional authority;
- validate itself through the same decision that formed it;
- move Current before activation;
- rewrite a committed movement;
- rewrite a committed Aura observation;
- rewrite maturity;
- condense before maturity;
- form proof from zero excess;
- mutate a validated Tombstone;
- mutate a recorded Toke;
- perform final resolution before all applicable gates complete;
- copy a parent Bond's history into a child;
- reuse a resolved Bond as Active;
- depend on wall-clock race, randomness, memory address, insertion order, or mutable hashing.

### 5.4 Constitutional questions

Bond answers:

- Who entered the relationship?
- In which explicit roles?
- Which Wave caused formation?
- Which House governs the constitutional function?
- Which Institution has jurisdiction?
- What is owned?
- Who holds custody?
- What obligations exist?
- What permissions exist?
- When does the active term begin?
- When and how can it complete?
- How does Current enter, move, and leave?
- How is Current revealed by Aura?
- What remains unresolved?
- Has default occurred?
- Is a challenge pending?
- Has the term completed?
- What mature excess remains?
- May that excess condense?
- Who validates the Tombstone?
- Who records the Toke?
- Who performs Synthesis Resolution?

### 5.5 Domain model

#### 5.5.1 Required identity and references

Every Bond MUST contain or reference:

- Bond identity;
- schema version;
- formation record;
- initiating Wave identity;
- participant identities;
- explicit role assignments;
- governing House identity;
- governing Institution identity;
- validating authority identity;
- resolving authority identity;
- term;
- obligations;
- permissions;
- ownership declarations;
- custody declarations;
- starting signed Current;
- initial signed Aura observations or an explicit absence of observation;
- parent Bond identities, if any;
- child Bond identities, once lawfully created;
- Recipe identities, if any;
- transformation identities, if any;
- evidence references;
- witness references;
- ordered event history;
- derived accounting projection;
- lifecycle state;
- challenge and appeal status;
- maturity record, when mature;
- excess calculation, when calculated;
- condensation eligibility decision;
- Tombstone identity, when formed;
- Toke identity, when recorded;
- Synthesis Resolution record, when resolved.

#### 5.5.2 Identity

A Bond identity MUST:

- be caller-controlled or authority-assigned through a deterministic rule;
- be unique in its constitutional repository;
- be immutable;
- be independent of insertion order;
- be independent of memory address;
- be independent of wall-clock time;
- be independent of randomness;
- not be a hash of mutable Bond fields;
- remain resolvable after Bond termination.

Reusing a Bond identity for a correction, renewal, appeal, split child, merge child, inherited Bond, or migrated semantic replacement is forbidden.

#### 5.5.3 Ownership

Ownership states who possesses constitutional title to Current, an obligation, a result, or another governed interest.

Custody states who presently holds, carries, or controls the governed item.

Ownership and custody MAY differ. They MUST be recorded separately when they differ.

Every ownership or custody change MUST:

- identify the prior holder;
- identify the successor holder;
- identify authority;
- identify the causal Wave;
- identify the affected quantity or interest;
- preserve history;
- conserve Current.

#### 5.5.4 Lifetime

A Bond begins at formation.

Its active lifetime begins at activation.

Its term is measured from activation unless the term is explicitly tied to an external deterministic completion condition.

Its living chemistry ends at maturity.

Its constitutional existence ends at final Synthesis Resolution.

Its history never ends.

#### 5.5.5 Mutability

The formed declaration is immutable.

Living state changes only by append-only lawful events.

Derived projections MAY be replaced by recomputation.

Mature history is immutable.

Tombstone payload, Tombstone validation decision, Toke, and final resolution are immutable.

#### 5.5.6 Visibility

Bond visibility is determined by governing permissions, institutional policy, Aura, and evidence access.

Visibility restrictions MAY limit who can inspect a record. They MUST NOT delete the record or change constitutional fact.

Concealed history remains history.

#### 5.5.7 Persistence

Formation, committed events, maturity, excess, eligibility, Tombstone, validation, Toke, and resolution MUST survive process restart.

A process restart MUST NOT:

- reactivate a resolved Bond;
- repeat activation;
- duplicate Current;
- duplicate a Tombstone;
- duplicate a Toke;
- recompute a different identity;
- reorder committed events.

### 5.6 Term

#### Purpose

The term establishes when the Bond's living chemistry may begin, continue, mature, renew, dissolve, default, or resolve.

A Bond without a term cannot determine maturity. A Bond without maturity cannot lawfully produce a Tombstone.

#### Responsibilities

The term MUST:

- classify itself as finite or explicitly perpetual;
- define the active lifetime's start boundary;
- define its applicable logical clock, event count, milestone, completion predicate, or external deterministic condition;
- define how completion is evaluated;
- define interruption treatment;
- define whether early termination is lawful;
- define whether renewal is lawful;
- define required default checkpoints;
- identify the authority that decides disputed completion;
- remain replayable under its governing rule version.

#### Non-responsibilities

The term MUST NOT:

- form or validate the Bond;
- activate the Bond;
- move Current;
- observe Aura;
- infer success or failure;
- declare default without an obligation rule;
- mature the Bond merely because a host process stops or wall-clock time passes outside the governing clock;
- extend itself in place;
- condense Current;
- resolve the Bond.

#### Constitutional questions

The term answers:

- Is this Bond finite or perpetual?
- When does its active lifetime begin?
- Which deterministic boundary completes it?
- Which events advance or suspend it?
- What happens when execution is interrupted?
- Can the relationship end early?
- Can it renew, and only as which successor relationship?
- Who decides a disputed boundary?
- Can replay reach the same completion result?

#### Required model

Every Bond term MUST be exactly one of:

- finite; or
- explicitly perpetual.

A finite term MUST define a deterministic completion boundary.

An explicitly perpetual term MUST state that no automatic time-based maturity exists. It MUST also identify the already-governing lawful termination or conversion conditions, if any, under which the Bond can reach maturity. If no such condition occurs, the Bond does not mature and cannot condense.

A term MUST define:

- activation start semantics;
- applicable logical clock, event boundary, milestone, or completion predicate;
- completion rule;
- interruption treatment;
- renewal permission or prohibition;
- early termination permission or prohibition;
- default evaluation points;
- jurisdiction for deciding completion.

#### Lifecycle

A term is Declared as part of formation.

After Bond validation it is Ready but has not started.

Activation moves the term to Running at the activation causal position.

A Running finite term becomes Completed exactly when its completion predicate is satisfied. A Running perpetual term remains Running until an already-governing lawful termination or conversion condition establishes a completion boundary.

A Running term MAY be Interrupted only when the governing term defines interruption. Interruption preserves the term's prior progress. It returns to Running under the same identity and original rule; it does not create renewal.

Completed is terminal for the source Bond's term. Renewal declares a new term only inside a newly formed successor Bond.

#### State machine

Legal term states are:

- Declared;
- Ready;
- Running;
- Interrupted;
- Completed.

Legal transitions are:

- Declared to Ready when Bond validation succeeds;
- Ready to Running when Bond activation commits;
- Running to Interrupted when the declared interruption rule applies;
- Interrupted to Running when that rule permits continuation;
- Running to Completed when the deterministic completion predicate succeeds;
- Interrupted to Completed only when the declared term permits completion during interruption and the completion predicate succeeds.

Completed is terminal. Rejected Bond validation and pre-activation cancellation terminate the Bond without starting or completing its term; they do not fabricate a Completed term.

All other term transitions are illegal.

#### Invariants

- Exactly one original term exists.
- A term never has negative duration.
- A finite term has one unambiguous completion result for a given history.
- A perpetual term is never treated as finite by implementation timeout.
- Pausing execution does not advance a logical term unless the governing clock says it does.
- Renewal creates a successor Bond and never edits the original term.
- Migration does not silently change term meaning.

#### Illegal states

- Missing term.
- Simultaneously finite and perpetual.
- Completion dependent on local wall-clock scheduling race.
- Term changed after formation.
- Perpetual Bond matured merely because a process stopped.
- Finite Bond matured before its completion predicate.
- Renewal extending the same identity.

- Running term before activation.
- Completed term returning to Running.
- Interrupted state not defined by the formed term.
- Completion evaluated under an unrecorded rule version.
- Different replay results for identical history and authority state.

#### Failure modes and recovery

If the completion predicate lacks required evidence or historical authority state, completion remains undecidable and maturity is blocked.

If a process stops while a term is Running, restart reconstructs the same state from formation, activation, committed events, and the governing clock. Restart is not interruption unless the declared term defines it as such.

If the governing completion rule is unavailable, the system preserves Running or Interrupted state and reports the missing rule. It MUST NOT substitute a current rule.

If two authorities dispute completion, the recorded challenge or appeal process decides reliance. Neither side may rewrite the term declaration or activation boundary.

Retry of a completion decision is idempotent. A conflicting second completion boundary is corruption.

#### Verification

Tests MUST cover exact boundary completion, one position before completion, interruption, restart, perpetual non-completion, lawful perpetual termination, early termination, and renewal as successor formation.

Tests MUST also cover rejected validation before start, cancellation before start, unauthorized interruption, unavailable governing rule, conflicting completion decisions, idempotent retry, and prohibition on returning from Completed to Running.

#### Repository mapping

No audited Bond-related type contains this complete term model.

Scenario ticks, timestamps, Recipe completion, Point² landing, KernelPass completion, institutional event positions, and transformation completion MAY supply a deterministic milestone or clock input when the formed term explicitly references them. None is a Bond term by itself.

Existing wall-clock timestamp fields MAY remain evidentiary metadata. They MUST NOT become maturity authority without an explicit deterministic term rule and replayable clock semantics.

### 5.7 Primary lifecycle

Every Bond MUST progress through the following ordered constitutional gates:

1. Bond Formation.
2. Bond Validation.
3. Bond Activation.
4. Current Circulation.
5. Current Accumulation.
6. Aura Observation.
7. Current/Aura Evaluation.
8. Maturity.
9. Net Excess Calculation.
10. Condensation Eligibility.
11. Tombstone Formation.
12. Tombstone Validation.
13. Toke Recording.
14. Synthesis Resolution.

No applicable gate may be bypassed.

For a Bond with zero excess or failed eligibility:

- the eligibility gate MUST still execute;
- Tombstone Formation MUST produce the explicit outcome Not Formed;
- Tombstone Validation MUST produce Not Applicable;
- Toke Recording MUST produce Not Applicable;
- Synthesis Resolution MAY then resolve the non-condensing Bond.

These explicit outcomes preserve the lifecycle decision sequence without fabricating artifacts.

For a Bond that fails before activation:

- no Current circulation gates execute;
- no Tombstone is possible;
- the Bond ends through a recorded pre-activation termination outcome;
- a corrected attempt requires a new Bond identity.

### 5.8 Primary state machine

The primary legal states are:

- Formed;
- ValidationPending;
- Validated;
- Active;
- EvaluationPending;
- Mature;
- ExcessCalculated;
- CondensationEligible;
- CondensationIneligible;
- TombstoneFormed;
- TombstoneValidated;
- Recorded;
- ResolvedWithoutTombstone;
- ResolvedWithTombstone;
- ValidationRejected;
- CancelledBeforeActivation.

Challenge, appeal, investigation, default, transfer preparation, split preparation, merge preparation, and renewal preparation are procedural statuses attached to a primary state. They do not erase the primary state.

Legal primary transitions are:

| From | To | Constitutional justification |
|---|---|---|
| Formed | ValidationPending | Legality must be decided before chemistry. |
| Formed | CancelledBeforeActivation | Formation may end before any Current moves. |
| ValidationPending | Validated | All activation prerequisites passed. |
| ValidationPending | ValidationRejected | At least one constitutional prerequisite failed. |
| Validated | Active | Activation begins chemistry exactly once. |
| Active | EvaluationPending | Living history is frozen for an evaluation boundary or challenge. |
| EvaluationPending | Active | Evaluation permits continued living chemistry before term completion. |
| EvaluationPending | Mature | The term completed and maturity checks succeeded. |
| Active | Mature | Permitted only when the implementation atomically performs the required final evaluation as part of maturity. |
| Mature | ExcessCalculated | Mature remainder was deterministically computed. |
| ExcessCalculated | CondensationEligible | Every eligibility requirement passed and excess is non-zero. |
| ExcessCalculated | CondensationIneligible | At least one eligibility requirement failed or excess is zero. |
| CondensationEligible | TombstoneFormed | Mature excess condensed exactly once. |
| TombstoneFormed | TombstoneValidated | Independent proof validation passed. |
| TombstoneValidated | Recorded | One Toke was atomically recorded. |
| Recorded | ResolvedWithTombstone | Synthesis Resolution concluded the proven Bond. |
| CondensationIneligible | ResolvedWithoutTombstone | Synthesis Resolution concluded without fabricating proof. |

Recovery transitions are:

- EvaluationPending to Active after a pre-maturity challenge is resolved and the term remains open.
- EvaluationPending to Mature after a challenge is resolved and the term is complete.
- TombstoneFormed to TombstoneFormed after a failed validation attempt only when the payload is unchanged and validation is retried against corrected external availability; a changed payload is forbidden.
- ValidationRejected to Validated only when an authorized appeal proves that the validation decision, rather than the formed Bond data, was erroneous and no final pre-activation termination was recorded.

All other primary transitions are illegal unless expressly defined by this document.

Terminal primary states are:

- ResolvedWithoutTombstone;
- ResolvedWithTombstone;
- CancelledBeforeActivation;
- ValidationRejected after its appeal window or final rejection is recorded.

No terminal Bond may return to Active.

### 5.9 Bond Formation

#### Purpose

Formation creates constitutional existence and records the complete declaration that later validators inspect.

Formation answers:

- Who entered?
- Why did they enter?
- Through which Wave?
- Under which House?
- Under which Institution?
- With which roles?
- With which obligations?
- With which permissions?
- For what term?
- With what starting Current?
- With what initial Aura?

#### Responsibilities

Formation MUST atomically establish:

- Bond identity;
- schema version;
- initiating Wave;
- participants;
- roles;
- governing House;
- governing Institution;
- term;
- obligations;
- permissions;
- ownership;
- custody;
- starting signed Current;
- initial Aura observations or explicit absence;
- Recipe and transformation references, if already known;
- parent references, if any;
- evidence required at formation.

#### Non-responsibilities

Formation MUST NOT:

- validate its own legality;
- activate;
- circulate Current;
- evaluate success;
- declare default;
- declare maturity;
- calculate excess;
- form a Tombstone;
- record a Toke;
- resolve the Bond.

#### Lifecycle and transitions

Formation occurs exactly once.

The only lawful immediate outcomes are:

- Formed, followed by ValidationPending; or
- CancelledBeforeActivation, if formation is lawfully withdrawn before validation and activation.

Correction of formed data creates a new Bond. An implementation MAY store an uncommitted construction draft, but a draft is not a Bond and MUST NOT be addressable as constitutional history.

#### Required invariants

- A Bond has exactly one formation.
- Every participant is explicit.
- Every role assignment is explicit.
- Every participant has at least one role.
- The initiating Wave exists and precedes formation.
- Starting Current traces to the initiating Wave or an explicitly referenced predecessor transfer.
- Governing House and Institution are present.
- Term is present.
- Obligations and permissions are distinguishable.
- Ownership and custody are distinguishable.
- Parent references never include the Bond itself.
- Formation is an atomic commit.

#### Illegal states

- Bond without participants.
- Participant without role.
- Role without participant.
- Formation without Wave.
- Formation without House or Institution.
- Formation without term.
- Duplicate Bond identity.
- Self-parent.
- Starting Current without causal source.
- Tombstone, Toke, maturity, or resolution data embedded as already completed formation facts.

#### Failure modes and recovery

Malformed identity, missing reference, duplicate participant-role pair, invalid signed amount, missing term, or failed atomic persistence prevents formation.

No partially formed Bond exists. A retry using the same intended identity is lawful only if no formation was committed. If a formation was committed, correction requires a new identity and an explicit relationship to the rejected or cancelled Bond.

#### Verification

Constitutional tests MUST prove:

- atomic all-or-none formation;
- exactly one formation;
- duplicate identity rejection;
- explicit participant-role coverage;
- Wave precedence;
- term presence;
- preservation across restart;
- inability to move Current from Formed state.

#### Repository mapping

No current type captures complete formation.

- The root Bond stores only one selected Way.
- PlebMetaBond stores two routing modes.
- BondCandidate stores participant display strings, selected arms, side, unsigned properties, viability, cost, and need.
- Aim stores a narrow kernel Bond plus Recipe provenance.

These are not formation records. They SHOULD remain narrow until a canonical Bond aggregate is introduced, and their names SHOULD be clarified to prevent accidental constitutional use.

### 5.10 Bond Validation

#### Purpose

Validation answers:

> Can this formed Bond legally enter living chemistry?

#### Responsibilities

Validation MUST verify:

- identity uniqueness;
- formation completeness;
- initiating Wave legality;
- participant existence and legality;
- role legality;
- role compatibility;
- validating authority;
- governing Institution jurisdiction;
- governing House jurisdiction;
- obligations;
- permissions;
- ownership;
- custody;
- Current source legality;
- Aura observation legality;
- term validity;
- Recipe legality;
- transformation eligibility;
- parent and inheritance legality;
- conflicts with existing Bonds;
- prohibited duplicates;
- impossible state combinations;
- evidence prerequisites.

Validation MUST produce a deterministic decision with explicit findings.

#### Non-responsibilities

Validation MUST NOT:

- move Current;
- infer missing participants;
- add missing authority;
- repair formation data;
- alter the term;
- activate the Bond;
- predict success;
- create evidence;
- treat silence as approval.

#### Authority separation

The validator MUST be authorized by the governing Institution and applicable House law.

The formation actor MAY also be a validator only if existing institutional law explicitly grants both roles and the validation evidence remains independently replayable. Role coincidence MUST NOT eliminate the validation stage.

#### Legal outcomes

- Validated.
- ValidationRejected.
- ValidationPending due to a declared, deterministic missing prerequisite.

Pending MUST NOT be used as indefinite implicit approval.

#### Required invariants

- Validation occurs after formation and before activation.
- Activation references one successful validation decision.
- Validation inputs are immutable formed data plus authoritative external state at the validation causal position.
- Identical inputs produce identical findings.
- Every failed check is reported.
- No Current circulates through an invalid Bond.

#### Illegal states

- Activated Bond without successful validation.
- Validation using authority outside jurisdiction.
- Validator silently modifying formed data.
- Duplicate Bonds accepted because repository order differs.
- Recipe accepted without required transformation prerequisites.
- Participant accepted by display name when stable identity is required.
- Validation result dependent on randomness or map iteration.

#### Failure modes and recovery

Missing external authority data MAY leave validation pending.

Constitutional invalidity produces rejection.

Infrastructure interruption produces no validation decision and may be retried.

An appeal MAY overturn an erroneous decision using the same formed data. Correcting the formed data requires a new Bond.

#### Verification

Tests MUST cover every validation dimension independently and in combination. They MUST prove failure prevents activation, pending is not approval, findings are stable under input reordering, and appeal cannot alter formed data.

#### Repository mapping

Candidate authority and validation structures include:

- House in hollow_grove_contract;
- InstitutionCatalog, Office, AuthorityLevel, and institutional relationships;
- InstitutionalWorldState and can_perform;
- Recipe compiler validation;
- transformation validators;
- alignment diagnostics;
- OfficialsOutlawsRegistry registration checks.

No current validator combines them at a Bond boundary. The constitutional direction is one orchestration boundary that calls existing domain validators without duplicating their rules.

### 5.11 Bond Activation

#### Purpose

Activation begins living chemistry.

Activation answers:

> Has this validated Bond begun the active lifetime in which Current may move and Aura may observe?

#### Responsibilities

Activation MUST:

- reference the successful validation decision;
- establish the term start;
- establish the first active causal position;
- open Current movement;
- open Aura observation;
- create the initial living projection;
- commit exactly once.

#### Non-responsibilities

Activation MUST NOT:

- create Current;
- change starting Current;
- change participants, roles, House, Institution, obligations, permissions, ownership, or term;
- imply success;
- imply maturity;
- form proof.

#### Required invariants

- Exactly one activation occurs.
- Formation and validation precede activation.
- The term starts exactly once.
- Current movement is forbidden before the activation commit.
- Aura observation of living chemistry is forbidden before activation, except formation-time initial Aura explicitly labeled as initial state.
- Activation is atomic.

#### Illegal states

- Multiple activations.
- Activation after terminal pre-activation rejection.
- Activation without term.
- Activation with unresolved required validation findings.
- Activation retroactively placed before validation.
- Activation that modifies history.

#### Failure modes and recovery

An interrupted activation commits either no activation or exactly one activation. Retry MUST be idempotent.

If authority is revoked before activation commits, activation fails and the Bond remains Validated or proceeds to lawful pre-activation termination.

#### Verification

Tests MUST prove exactly-once behavior, atomicity under interruption, term-start stability, and rejection of all pre-activation Current movement.

#### Repository mapping

The existing Current Synthesis activation gate is a read-only feature gate, not Bond activation. The Recipe Aim status prepared and Fire commitment are execution stages, not Bond activation. They MAY supply evidence inside an already Active Bond.

### 5.12 Current Circulation

#### Purpose

Circulation governs every lawful movement of signed Current during the active term.

#### Responsibilities

Every Current movement MUST record:

- movement identity;
- Bond identity;
- prior Wave identity;
- causal position;
- signed quantity;
- unit and scale;
- source owner;
- source custodian;
- destination owner;
- destination custodian;
- movement operation;
- obligation or permission;
- evidence;
- authority;
- resulting custody.

Lawful movement operations are:

- enter;
- leave;
- split;
- merge;
- branch;
- reverse;
- circulate;
- stall;
- decay;
- transfer;
- expire;
- consume;
- receive production from a lawful external Wave.

These operation names describe accounting treatment. They do not relax conservation.

#### Conservation law

The Bond never creates or destroys Current.

- Enter receives Current from an explicit external source.
- Leave sends Current to an explicit external destination.
- Split preserves one input as multiple outputs whose exact signed magnitudes sum to the input magnitude.
- Merge preserves multiple inputs as one output whose exact signed magnitude equals their sum.
- Branch is a split with independently governed destinations.
- Reverse changes direction or custody and does not silently change sign or magnitude.
- Circulate moves Current internally and does not count as new boundary input.
- Stall changes no quantity and records no zero movement; it MAY record a non-Current condition event.
- Decay transfers Current into an explicitly identified lawful remainder or sink.
- Transfer changes ownership or custody without duplication.
- Expire transfers Current into the disposition required by the governing rule.
- Consume transfers Current into an explicit result, output, or sink.
- External production enters only from a lawful external Wave with its own causal origin.

#### Non-responsibilities

Circulation MUST NOT:

- validate a Bond after the fact;
- infer ownership;
- infer a Wave;
- mutate Aura;
- calculate mature excess;
- continue after maturity;
- use saturating arithmetic;
- omit a sink for consumed, decayed, or expired Current.

#### State machine

Current movement is lawful only while the primary state is Active.

EvaluationPending suspends new movements unless the governing evaluation explicitly permits continuation and commits the movement after the evaluation boundary.

Mature and every later state reject movement.

#### Required invariants

- Wave always precedes Current.
- Activation always precedes circulation.
- Every movement is non-zero.
- Every movement has exactly one immediate source and destination accounting edge, even when a higher-level operation has multiple edges.
- No movement duplicates identity.
- No movement changes a committed predecessor.
- Conservation holds per sign and unit.
- Cross-unit conversion requires an explicit deterministic governing rule and evidence.
- Internal circulation does not inflate boundary totals.

#### Illegal states

- Current before activation.
- Current after maturity.
- Current without Wave.
- Current without unit.
- Current with zero magnitude.
- Split outputs exceeding or falling short of input.
- Merge output differing from inputs.
- Consumption without result or sink.
- Decay interpreted as deletion.
- Transfer changing owner but leaving full Current with both owners.
- Reversal changing sign without a sign-changing external rule and causal source.

#### Failure modes and recovery

Invalid movement is rejected atomically.

Partial multi-edge movement commits none of its edges.

If persistence fails after validation but before commit, retry MUST not duplicate Current.

If a committed movement is later disputed, the movement remains historical while a challenge determines reliance and consequence.

#### Verification

Tests MUST cover every operation, both signs, unit mismatch, exact conservation, duplicate retry, crash boundaries, ownership and custody, and prohibition after maturity.

#### Repository mapping

ResourceComposition and Residue provide deterministic property accumulation, but they use unsigned totals, saturating addition, and convert unused Bond candidates to residue. They do not enforce constitutional Current conservation. They MAY inform an adapter or migration fixture but cannot serve as the canonical ledger unchanged.

### 5.13 Current Accumulation

#### Purpose

Accumulation converts individual movements into deterministic living history without claiming proof or maturity.

#### Required accounting projections

The Bond MUST maintain or be able to derive:

- total Positive Current history;
- total Negative Current history;
- net Current;
- incoming Positive Current;
- incoming Negative Current;
- outgoing Positive Current;
- outgoing Negative Current;
- retained Positive Current;
- retained Negative Current;
- transferred Positive Current;
- transferred Negative Current;
- resolved Positive Current;
- resolved Negative Current;
- unresolved Positive Current;
- unresolved Negative Current.

The implementation MAY present signed combined values in addition to these gross values. It MUST NOT discard the gross values.

#### Accounting definitions

- Incoming Current is boundary Current accepted into Bond custody.
- Outgoing Current is boundary Current released from Bond custody.
- Retained Current is Current still under Bond custody at the evaluated causal position.
- Transferred Current is gross internal movement between owners or custodians and is a throughput measure, not a new source.
- Resolved Current is Current whose constitutional disposition against obligations has been deterministically classified.
- Unresolved Current is accountable Current not yet assigned a final constitutional disposition.
- Historical Current is the immutable ordered movement history.
- Net Current is Positive Current magnitude minus Negative Current magnitude under the governing comparison scope.

Resolved and unresolved are classification views. They MUST NOT alter conservation totals.

#### Required equations

For each sign and unit:

- starting custody plus incoming equals retained plus outgoing;
- accountable Current equals resolved plus unresolved under the governing classification scope;
- internal transferred Current does not appear as additional incoming Current;
- net Current equals gross positive comparison magnitude minus gross negative comparison magnitude.

If a governing rule excludes a category from mature comparison, the exclusion MUST be explicit and replayable. Exclusion does not delete historical Current.

#### Non-responsibilities

Accumulation MUST NOT:

- prove legality;
- create a Tombstone;
- net away gross history;
- infer Aura;
- treat throughput as source creation;
- silently combine incompatible units.

#### Required invariants

- Accumulation follows circulation.
- Every total derives from committed events.
- Reordering storage does not change totals.
- Duplicate events are rejected by identity.
- Overflow rejects the projection or transition.
- A rebuilt projection equals the persisted projection.

#### Illegal states

- Net-only ledger.
- Negative totals stored as unsigned underflow.
- Internal transfer counted as new Current.
- Unresolved Current omitted.
- Mature snapshot whose totals do not replay.
- Accumulation altered by Aura.

#### Failure modes and recovery

Projection corruption requires rebuilding from events.

If rebuild differs from a persisted authoritative mature snapshot, the Bond enters constitutional corruption handling and MUST NOT condense until resolved.

#### Verification

Tests MUST prove each equation, gross-history survival, unit separation, order independence, duplicate rejection, projection rebuild, and zero net with non-zero gross history.

### 5.14 Aura Observation

#### Purpose

Aura Observation continuously reveals living chemistry through an ordered set of explicit observations.

Continuous means that observation remains available throughout the Active term. It does not require nondeterministic wall-clock sampling.

#### Responsibilities

Each observation MUST record:

- observation identity;
- Bond identity;
- causal position;
- observer identity;
- observer role or authority;
- observed subject;
- observed event or causal range;
- Positive or Negative sign;
- exact non-zero magnitude;
- unit and scale;
- evidence;
- visibility;
- institutional recognition or rejection, if applicable;
- any challenge status.

Observation MUST evaluate applicable dimensions including:

- visibility;
- legitimacy;
- recognition;
- clarity;
- concealment;
- distortion;
- witness;
- institutional acceptance;
- institutional rejection.

#### Non-responsibilities

Observation MUST NOT:

- move Current;
- modify Current;
- fabricate an unobserved fact;
- silently replace a conflicting observation;
- equate presentation with truth;
- infer authority from visibility.

#### Lifecycle

Initial Aura MAY be recorded at formation as formation context.

Living Aura Observation begins at activation.

Observations accumulate until maturity.

At maturity, observations become historical and no new observation may be inserted into the matured Bond. Later discoveries require challenge or successor-Bond treatment and cannot be backdated.

#### Required invariants

- Observer is explicit.
- Observed scope is explicit.
- Sign and magnitude are explicit.
- Observation cannot precede its observed event, except a formation-time observation of starting state.
- Current history is unchanged by observation.
- Conflicting observations coexist.
- Visibility controls access, not truth.

#### Illegal states

- Anonymous constitutional observation.
- Observation without scope.
- Zero-magnitude observation.
- Observation after maturity inserted into mature history.
- Aura observation changing Current.
- Light/Dark label directly used as sign without evaluation.
- Missing evidence where governing rules require evidence.

#### Failure modes and recovery

Ambiguous, contradictory, out-of-jurisdiction, or unsupported Aura MAY remain in history but MUST be marked unevaluable or challenged. It MUST NOT satisfy condensation eligibility until lawfully evaluated.

#### Verification

Tests MUST cover both signs over both Current signs, conflicting witnesses, delayed observation rejection after maturity, visibility restrictions, and Current immutability.

#### Repository mapping

AuraPolarityActionRequest and AuraPolarityEvaluation already capture observer-adjacent facts such as truth, consent, agency, pressure, proportionality, reversibility, consequence, route, House, and requested versus inferred orientation. They are strong evidence candidates. They lack Bond identity, causal range, signed magnitude, term, and maturity semantics and therefore require an explicit constitutional projection.

### 5.15 Current/Aura Evaluation

#### Purpose

Evaluation answers:

> Is the accumulated signed Current history and signed Aura history internally coherent at this causal boundary?

#### Responsibilities

Evaluation MUST inspect:

- Positive Current;
- Negative Current;
- Positive Aura;
- Negative Aura;
- participants;
- roles;
- obligations;
- permissions;
- ownership;
- custody;
- evidence;
- witnesses;
- governing House;
- governing Institution;
- authority;
- term status;
- default findings;
- pending challenges;
- arithmetic integrity;
- causal completeness.

Evaluation MUST produce:

- Current gross totals;
- Aura gross totals;
- signed net Current, if evaluable;
- signed net Aura, if evaluable;
- exactly one four-polarity state, if both domains are evaluable and non-zero;
- findings;
- continuation, investigation, challenge, clarification, correction-by-future-event, or maturity readiness.

#### Non-responsibilities

Evaluation MUST NOT:

- rewrite history;
- invent missing evidence;
- backdate correction;
- offset Aura against Current;
- declare maturity before term completion;
- form a Tombstone;
- choose a sign when net is zero or evidence is insufficient.

#### Lifecycle and state machine

Evaluation MAY occur repeatedly while Active.

Each evaluation references an exact causal prefix.

An evaluation of a later prefix supersedes the earlier living projection but does not delete the earlier evaluation record.

The final evaluation at the maturity boundary becomes the mature evaluation.

#### Required invariants

- Identical causal prefixes and governing rules produce identical results.
- Every input is traceable.
- Both gross signs in both domains remain visible.
- A successful evaluation yields exactly one polarity state.
- Investigation and challenge suspend reliance but do not erase history.
- Correction is a new event, never mutation.

#### Illegal states

- Evaluation without exact causal boundary.
- Evaluation ignoring one signed domain.
- Aura offsetting Current.
- Random or floating tie resolution.
- Successful polarity with zero net domain.
- Mature evaluation with open unhandled challenge.
- Changed result with unchanged inputs and rules.

#### Failure modes and recovery

Failure may trigger investigation, challenge, witness request, clarification, or a lawful future correction event.

If the term completes while evaluation remains incomplete, the Bond MAY become mature as a temporal fact but MUST remain condensation-ineligible until the mature history can be evaluated. Maturity does not imply proof.

#### Verification

Tests MUST cover all four polarity states, zero in either domain, mixed gross histories, conflicting evidence, repeated evaluations, unchanged replay, and prohibition on historical rewrite.

### 5.16 Maturity

#### Purpose

Maturity establishes that the Bond's constitutional term has completed and living chemistry has ended.

Maturity is independent of success. Successful, failed, defaulted, challenged, burdensome, productive, recognized, and concealed Bonds may all mature.

#### Responsibilities

Maturity MUST:

- verify activation occurred;
- verify the term completion predicate;
- identify the exact final causal boundary;
- close Current circulation;
- close living Aura observation;
- freeze participant obligations as completed, remaining, defaulted, transferred, or otherwise lawfully classified;
- freeze ownership and custody at the maturity boundary;
- freeze the final living accounting prefix;
- identify the final evaluation status;
- commit exactly once.

#### Non-responsibilities

Maturity MUST NOT:

- imply success;
- imply validation of every event;
- invent evidence;
- resolve open accounting by guessing;
- calculate excess as part of the same uninspectable step;
- condense;
- form a Tombstone;
- record a Toke;
- resolve the Bond.

#### State machine

Only an Activated Bond may mature.

An Active Bond may move to Mature only at term completion and through an explicit final evaluation boundary.

An EvaluationPending Bond may move to Mature when:

- the term is complete; and
- the pending evaluation has produced a mature snapshot or an explicit incomplete-evaluation finding.

Incomplete evaluation does not prevent the temporal fact of maturity. It prevents later eligibility.

#### Required invariants

- Exactly one maturity record exists.
- Activation precedes maturity.
- The term completes before maturity.
- No Current enters or leaves after maturity.
- No Aura observation is inserted after maturity.
- No historical Current disappears.
- No historical Aura is rewritten.
- Mature participant and obligation states are fixed.
- The maturity boundary is replayable.

#### Illegal states

- Maturity before activation.
- Maturity before term completion.
- Multiple maturities.
- Negative term duration.
- Current after maturity.
- Backdated Aura after maturity.
- Mature state changed by restart.
- Mature history dependent on later storage order.

#### Failure modes and recovery

If the term completion predicate cannot be evaluated because required authority or evidence is unavailable, maturity remains pending.

If temporal completion is certain but accounting is corrupt, maturity MAY be recorded with an explicit accounting-invalid finding. The Bond remains ineligible for condensation.

Infrastructure interruption MUST produce no partial maturity.

#### Verification

Tests MUST cover finite boundaries, perpetual terms, successful and failed Bonds, defaulted Bonds, incomplete evaluation, restart, exactly-once maturity, and rejection of post-maturity movement.

#### Repository mapping

No current repository type carries a Bond term or maturity. Completed ticks, Point² landing, Recipe execution completion, and DecisionTrace completion are not maturity. They MAY be used as deterministic term milestones when a formed Bond explicitly names them.

### 5.17 Net Excess Calculation

#### Purpose

Net Excess Calculation determines the lasting signed Current remainder after the mature Bond's obligations and governing offset rules have been applied.

#### Responsibilities

The calculation MUST:

- use the immutable mature accounting snapshot;
- use the governing rules fixed by formation or authoritative references;
- classify eligible Positive Current remainder;
- classify eligible Negative Current remainder;
- exclude only categories explicitly excluded by governing rules;
- retain excluded categories in history;
- offset positive and negative eligible remainder deterministically;
- produce positive excess, negative excess, or zero;
- record all inputs and intermediate classifications;
- reject arithmetic error.

#### Canonical offset

After governing classification:

- let P be eligible Positive Current remainder;
- let N be eligible Negative Current remainder;
- let O be the lesser of P and N;
- positive excess equals P minus O;
- negative excess equals N minus O;
- signed net excess equals P minus N.

At most one of positive excess and negative excess is non-zero.

A governing rule MAY determine which mature quantities enter P and N. It MUST NOT change the exact offset arithmetic after those quantities are selected unless the formation explicitly references a different constitutional offset rule.

Aura does not enter P, N, O, or net excess.

#### Non-responsibilities

Net Excess Calculation MUST NOT:

- alter mature Current;
- delete offset history;
- use Aura as Current;
- decide Tombstone validity;
- decide final resolution;
- hide a zero result;
- use floating-point approximation.

#### Required invariants

- Maturity precedes calculation.
- Exactly one authoritative result exists for one mature snapshot and rule version.
- Gross Positive and Negative Current remain recorded.
- Offset never exceeds either side.
- Zero excess produces no Tombstone.
- Arithmetic overflow or unit mismatch rejects the calculation.
- Replay produces the identical result.

#### Illegal states

- Excess before maturity.
- Aura offset against Current.
- Positive and negative excess both non-zero.
- Net result without gross inputs.
- Changed governing rule after maturity.
- Rounding-dependent excess.
- Silent unit conversion.

#### Failure modes and recovery

Missing rule, corrupt snapshot, unresolved unit conversion, contradiction, or arithmetic failure prevents a valid excess result and therefore prevents eligibility.

Retry is lawful only with the same mature snapshot and governing rule, or after a challenge process supplies a new authoritative decision without rewriting the original inputs.

#### Verification

Tests MUST cover positive excess, negative excess, exact zero, large exact values, incompatible units, excluded classifications, overflow, and deterministic replay.

### 5.18 Condensation Eligibility

#### Purpose

Eligibility answers:

> May this mature Bond's non-zero net excess become durable constitutional proof?

#### Required conditions

Eligibility requires all of:

- completed term;
- one maturity record;
- internally consistent causality;
- lawful participants;
- lawful roles;
- valid governing House;
- valid governing Institution;
- valid ownership and custody;
- validated evidence;
- deterministic accounting;
- evaluable mature Aura;
- one successful mature polarity state;
- no unresolved blocking challenge;
- a valid net excess calculation;
- non-zero net excess;
- authorized condenser;
- replayable history.

Failure of any condition produces CondensationIneligible.

#### Non-responsibilities

Eligibility MUST NOT:

- repair a failed condition;
- create evidence;
- form a Tombstone;
- treat zero as proof;
- bypass a pending challenge;
- reinterpret the term.

#### Required invariants

- Eligibility follows excess calculation.
- Eligibility is deterministic.
- Every condition produces an explicit pass or fail finding.
- Non-zero excess is necessary but not sufficient.
- A defaulted Bond MAY be eligible if every condition passes.
- A successful Bond MAY be ineligible if any condition fails.

#### Illegal states

- Eligible Bond with zero excess.
- Eligible Bond with unevaluable Aura.
- Eligible Bond with corrupt accounting.
- Eligible Bond before maturity.
- Eligibility based only on a success label.
- Eligibility decided by the same unrecorded mutation that forms the Tombstone.

#### Failure modes and recovery

An unavailable validator or evidence source MAY produce pending eligibility.

A constitutional failure produces ineligibility.

Where the cause is external unavailability rather than invalid history, retry MAY re-evaluate the unchanged mature Bond. Retry MUST NOT add backdated history.

#### Verification

Tests MUST independently negate every required condition and prove ineligibility, plus prove that a fully valid positive-excess Bond and fully valid negative-excess Bond are eligible.

### 5.19 Condensation

#### Purpose

Condensation irreversibly converts eligible mature net excess Current into durable constitutional matter represented by a Tombstone.

Condensation is not Synthesis.

#### Responsibilities

Condensation MUST:

- consume one eligibility decision;
- use exactly the mature net excess;
- preserve the sign and magnitude of excess;
- preserve the source Bond reference;
- preserve all supporting historical references;
- form at most one immutable Tombstone payload;
- commit atomically.

Condensation changes representation and constitutional durability. It does not erase Current history and does not create additional Current.

#### Non-responsibilities

Condensation MUST NOT:

- occur before maturity;
- operate on zero excess;
- operate on unresolved living Current;
- select a different excess;
- perform Tombstone validation;
- record a Toke;
- resolve the Bond;
- execute a Recipe merely because both use transformation language.

#### State machine

Only CondensationEligible may transition to TombstoneFormed.

CondensationIneligible MUST produce Not Formed.

Condensation is irreversible after the Tombstone payload commits.

#### Required invariants

- At most one condensation per Bond.
- At most one Tombstone per Bond.
- Condensed sign and magnitude equal eligible excess.
- The source Bond remains addressable.
- No other Bond claims the same condensation output.
- A retry after commit returns the already formed Tombstone and does not duplicate it.

#### Illegal states

- Premature condensation.
- Multiple condensations.
- Partial Tombstone payload.
- Condensation from mutable chemistry.
- Condensation from unverified history.
- Condensation changing sign.
- Condensation adding or losing magnitude.

#### Failure modes and recovery

Infrastructure interruption commits either no Tombstone or exactly one complete Tombstone.

A failed write MAY be retried idempotently.

A payload validation failure after formation does not permit forming a different Tombstone from the same Bond. It requires validation resolution, challenge, or a successor Bond where constitutionally appropriate.

#### Verification

Tests MUST prove one-to-one Bond-to-Tombstone formation, exact excess preservation, zero rejection, premature rejection, atomicity, idempotent retry, and irreversibility.

### 5.20 Default

#### Purpose

Default records that an obligation was not lawfully completed under its governing condition.

Default is a constitutional finding within a Bond. It is not automatic Bond destruction and is not the same as Negative Current, failure, maturity, termination, or resolution.

#### Responsibilities

A default finding MUST identify:

- affected obligation;
- responsible participant or role;
- required completion condition;
- applicable deadline or term boundary;
- actual historical state;
- authority declaring default;
- evidence;
- resulting Current classification under existing rules;
- challenge and appeal rights.

#### Non-responsibilities

Default MUST NOT:

- invent Negative Current;
- erase Positive Current;
- mature the Bond early unless the term already declares default as a maturity condition;
- automatically dissolve the Bond;
- automatically produce a Tombstone;
- rewrite the obligation.

#### Lifecycle

Default MAY be evaluated during the active term at an obligation deadline, at maturity, or during challenge.

Default MAY:

- leave the Bond Active;
- trigger investigation;
- trigger challenge;
- trigger a term-defined early maturity condition;
- contribute to mature classification;
- remain as an obligation in a successor Bond through explicit inheritance.

#### Required invariants

- Every default references an existing obligation.
- Default authority has jurisdiction.
- Default consequence comes from governing rules.
- Default history remains even if cured.
- Cure is a later event, not deletion of default.
- A defaulted Bond may still mature and condense lawfully.

#### Illegal states

- Default without obligation.
- Default inferred solely from Negative Current.
- Default declared before its condition can fail.
- Default deleting completed history.
- Default assigning burden without causal source.
- Default automatically treated as Negative Aura.

#### Failure modes and recovery

Ambiguous completion, missing evidence, authority dispute, or participant loss MAY suspend the finding.

Challenge or appeal MAY change the authoritative interpretation while preserving the original default decision as history.

#### Verification

Tests MUST cover timely completion, partial completion, missed deadline, cure, challenge, term-defined early maturity, and defaulted Bonds with positive, negative, and zero excess.

#### Repository mapping

InstitutionalObligation and ObligationStatus already represent Open, Called, PartiallyPaid, Settled, Forgiven, Defaulted, and Inherited. They are useful domain references. They lack Bond identity, signed Current, term-specific evidence, and append-only decision history. They SHOULD remain obligation authority rather than being duplicated inside a generic Bond enum.

### 5.21 Challenge

#### Purpose

Challenge permits an authorized objection to legality, evidence, accounting, Aura interpretation, default, maturity readiness, Tombstone validation, or authority without rewriting history.

#### Required model

Every challenge MUST identify:

- challenge identity;
- challenger;
- challenger standing;
- challenged Bond or artifact;
- exact challenged decision or causal range;
- grounds;
- evidence;
- deciding authority;
- filing causal position;
- whether progress is suspended;
- outcome.

#### Lifecycle

A pre-maturity challenge MAY suspend evaluation or specific disputed movements while undisputed history remains intact.

A pre-record Tombstone challenge MAY suspend validation or Toke recording.

A post-record challenge that could change lasting consequence MUST itself be governed by a new Bond. The original Tombstone and Toke remain immutable. A successful challenge Bond may produce new proof that supersedes interpretation without deleting prior history.

#### Outcomes

- upheld;
- rejected;
- clarified;
- remanded for deterministic reevaluation;
- withdrawn.

Outcome labels do not replace detailed findings.

#### Non-responsibilities

Challenge MUST NOT:

- delete the challenged event;
- mutate a Tombstone;
- mutate a Toke;
- fabricate standing;
- pause unrelated Bonds without authority;
- create Current outside a Bond.

#### Required invariants

- Challenger standing is explicit.
- Challenged scope is exact.
- Suspension status is explicit.
- Original history remains.
- Resolution is deterministic.
- Post-record material correction uses a new Bond.

#### Illegal states

- Anonymous challenge.
- Challenge without target.
- Challenge mutating history.
- Post-record in-place Tombstone edit.
- Challenge used to reactivate a resolved Bond.
- Challenge outcome without deciding authority.

#### Failure modes and recovery

Missing standing rejects the challenge.

Missing evidence MAY leave it pending if governing procedure permits.

Authority conflict invokes institutional dispute handling and suspends affected reliance.

#### Verification

Tests MUST cover challenge at validation, active evaluation, default, maturity, Tombstone validation, and post-record history; standing rejection; suspension; withdrawal; and preservation of original history.

### 5.22 Appeal

#### Purpose

Appeal provides authorized review of an already-issued validation, default, challenge, eligibility, or Tombstone-validation decision.

Appeal is not a second attempt to provide different formed Bond facts.

#### Required model

Every appeal MUST identify:

- appeal identity;
- appellant;
- standing;
- appealed decision;
- allowed grounds;
- reviewing authority;
- evidence scope;
- whether new evidence is permitted by governing procedure;
- outcome;
- effect on lifecycle progress.

#### Outcomes

- affirmed;
- reversed;
- modified by explicit lawful interpretation;
- remanded;
- dismissed;
- withdrawn.

#### Required invariants

- The appealed decision remains in history.
- Reviewing authority is distinct or explicitly superior under institutional law.
- Appeal does not change formation.
- Reversal produces a new decision record.
- Appeal cannot create a second Tombstone for the same Bond.
- Post-Toke material appeal proceeds through a new Bond.

#### Illegal states

- Appeal without prior decision.
- Appeal by a party without standing.
- Appeal that silently replaces evidence.
- Appeal that edits a Toke.
- Appeal that changes Bond identity.
- Appeal that reopens Active state after final resolution.

#### Failure modes and recovery

Unavailable reviewing authority leaves the appeal pending only if governing procedure permits.

Expired or unauthorized appeal is dismissed with an immutable decision.

#### Verification

Tests MUST prove affirmation, reversal, remand, dismissal, standing, authority hierarchy, unchanged formation, and post-record successor-Bond handling.

#### Repository mapping

The repository has validation diagnostics, institutional recognition, claims, and disputed verification results, but no shared challenge or appeal record. These SHOULD reference existing institutional authority rather than introduce a parallel authority system.

### 5.23 Transfer

#### Purpose

Transfer moves ownership, custody, obligation, permission, or remaining Current from one lawful holder or Bond to another without duplication.

#### Responsibilities

Transfer MUST identify:

- transferred subject;
- source owner and custodian;
- destination owner and custodian;
- signed Current quantity, if any;
- source Bond;
- destination Bond, if any;
- authority;
- Wave;
- evidence;
- effective causal boundary;
- obligations preserved, completed, or inherited.

#### In-place prohibition

A transfer that changes formed participants, roles, governing House, governing Institution, or term MUST NOT mutate the original formation.

Such a transfer MUST:

- mature or lawfully terminate the source chemistry at the transfer boundary as governed;
- create a successor Bond with a new identity;
- reference the source as parent;
- move Current through balanced exit and entry events;
- preserve original history.

Custody transfer among already-declared participants MAY occur within the same Active Bond when formation permissions allow it.

#### Required invariants

- No transferred Current is duplicated.
- Source and destination quantities match by sign, magnitude, and unit.
- Transfer Wave precedes both accounting edges.
- Ownership and custody are explicit.
- New constitutional relationship uses a new Bond identity.

#### Illegal states

- Copying Current to destination while retaining it at source.
- Changing formation in place.
- Transfer without authority.
- Transfer after maturity into the mature ledger.
- Destination Bond receiving Current before activation.
- Transfer to self presented as ownership change.

#### Failure modes and recovery

Cross-Bond transfer MUST commit atomically or through a recoverable protocol that cannot expose duplicated authoritative Current.

Interruption before completion leaves source ownership unchanged or records an explicit pending custody state authorized by the governing protocol. Ambiguous double ownership is forbidden.

#### Verification

Tests MUST prove same-Bond custody transfer, successor transfer, cross-Bond conservation, crash recovery, authority, and prohibition after maturity.

### 5.24 Split and Branch

#### Purpose

Split resolves one Bond into multiple child Bonds or divides Current among multiple lawful paths.

Branch is a split whose child paths remain independently governed.

#### Required model

A Bond-level split MUST:

- identify one parent Bond;
- identify two or more child Bond identities;
- define a deterministic allocation rule;
- allocate Current without duplication;
- allocate or reference obligations without silent loss;
- allocate permissions;
- identify ownership and custody;
- identify governing authority;
- preserve parent history;
- form each child independently;
- validate each child independently.

#### Non-responsibilities

Split MUST NOT:

- clone parent Current;
- clone completed obligations as newly open;
- give children the parent identity;
- copy parent event history into child history;
- imply every child is valid because the parent was valid.

#### Required invariants

- One parent may have multiple children.
- Every child has exactly one formation and its own term.
- Allocated Current sums exactly to the parent disposition.
- Child histories begin at child formation.
- Parent remains immutable.
- A failed child does not invalidate a separately lawful sibling unless governing rules explicitly couple them.

#### Illegal states

- Child without parent reference.
- Parent referencing itself as child.
- Allocation exceeding parent Current.
- Missing remainder.
- Same child listed twice.
- Child activated without validation.

#### Failure modes and recovery

If atomic child formation is required and any child fails, no child becomes Active.

If governing rules permit independent formation, the resolution record MUST state which children formed and where unallocated Current remains.

#### Verification

Tests MUST cover two-child and multi-child allocation, both signs, obligation partition, one child failure, independent terms, and replay.

### 5.25 Merge

#### Purpose

Merge resolves two or more parent Bonds into one child Bond without collapsing their histories.

#### Required model

A merge MUST:

- identify all parent Bonds;
- require each parent to reach its lawful merge boundary;
- create exactly one child formation with a new identity;
- preserve parent identities and histories;
- transfer Current through explicit balanced movements;
- reconcile obligations by an explicit governing rule;
- declare the child term, participants, roles, House, Institution, ownership, custody, and permissions;
- validate the child independently.

#### Non-responsibilities

Merge MUST NOT:

- concatenate event histories and call them one Bond;
- reuse a parent identity;
- double-count shared Current;
- erase conflicting obligations;
- assume parent authority applies to the child.

#### Required invariants

- At least two distinct parents exist.
- One distinct child exists.
- Each parent references the same merge resolution.
- Child Current equals explicit transferred inputs, not parent gross history.
- Parent Tombstones and Tokes remain separate.
- Child maturity is determined only by the child term.

#### Illegal states

- One-parent merge.
- Parent equal to child.
- Unresolved parent Current silently copied.
- Parent history rewritten.
- Child activated because one parent was valid.
- Cross-jurisdiction merge without authority.

#### Failure modes and recovery

If a parent is not ready, merge remains pending or fails according to governing rules.

Partial merge MUST NOT expose a child as Active until all required inputs and child validation complete.

#### Verification

Tests MUST cover two and multiple parents, shared participants, conflicting obligations, cross-jurisdiction authority, Current conservation, and independent child validation.

### 5.26 Renewal

#### Purpose

Renewal continues a relationship under a new term while preserving the completed history of the prior Bond.

#### Required model

Renewal MUST:

- occur only if the original term permits renewal;
- create a successor Bond with a new identity;
- reference the prior Bond as parent;
- create one new formation;
- declare the new term;
- explicitly carry forward any ownership, custody, Current, obligation, permission, Recipe, or evidence reference;
- validate and activate the successor independently.

#### Non-responsibilities

Renewal MUST NOT:

- extend the original term in place;
- reactivate a mature or resolved Bond;
- copy unresolved Current without transfer;
- copy default as though it never occurred;
- inherit authority implicitly.

#### Required invariants

- Original Bond remains mature or resolved.
- Successor identity differs.
- New term is explicit.
- Carried Current conserves across exit and entry.
- Remaining obligations are explicitly inherited or left with the parent.
- Renewal does not produce a Tombstone by itself.

#### Illegal states

- Same-identity renewal.
- Renewal before permission check.
- Renewal without parent reference.
- New term omitted.
- Successor activated without validation.
- Original Bond returned to Active.

#### Failure modes and recovery

Failed renewal leaves the original Bond unchanged.

If transfer into the successor fails, no duplicated Current may exist.

#### Verification

Tests MUST prove identity change, term independence, obligation carry-forward, Current conservation, failed successor validation, and original immutability.

### 5.27 Termination and Dissolution

#### Purpose

Termination ends a Bond's living or pre-activation existence under a lawful condition.

Dissolution is a resolution disposition in which no successor relationship continues unless separately formed.

#### Pre-activation termination

A Formed or Validated Bond MAY be cancelled before activation if governing rules permit.

Pre-activation termination:

- moves no Current;
- produces no maturity;
- produces no Tombstone;
- produces no Toke;
- records the reason and authority;
- is terminal for that Bond identity.

#### Active termination

An Active Bond may terminate only through:

- term completion;
- a term-declared early termination condition;
- lawful transfer, split, merge, renewal, default consequence, or dissolution rule;
- authorized institutional action.

Active termination MUST establish a maturity boundary before excess and eligibility.

#### Required invariants

- Termination reason is explicit.
- Authority is explicit.
- Current disposition is complete.
- Active termination reaches maturity; it does not skip maturity.
- Pre-activation cancellation never claims maturity.
- Dissolution preserves history.

#### Illegal states

- Silent deletion.
- Active termination without Current accounting.
- Dissolution erasing obligations.
- Pre-activation cancellation producing proof.
- Termination used to bypass challenge.
- Resolved Bond terminated again.

#### Failure modes and recovery

Participant loss, authority revocation, corruption, or interrupted operation MAY trigger termination review. None permits silent cleanup.

#### Verification

Tests MUST cover pre-activation cancellation, early active termination, ordinary term completion, dissolution, participant loss, and history persistence.

### 5.28 Institutional Authority

#### Purpose

Institutional authority establishes who may form, validate, observe, challenge, transfer, record, or resolve a Bond within concrete jurisdiction.

#### Responsibilities

Every authority-dependent act MUST identify:

- Institution;
- acting Being, office, role, or other authorized entity;
- authority source;
- authority scope;
- jurisdiction;
- causal position at which authority was evaluated;
- any required witness, vote, sponsorship, clearance, or grant;
- evidence of authority.

#### Non-responsibilities

Institutional authority MUST NOT:

- create House authority;
- invent Current;
- overwrite evidence;
- validate an act outside jurisdiction;
- retroactively grant authority to a completed unauthorized act;
- infer authority from display name or social prominence.

#### Authority loss

Loss of authority after a committed lawful act does not retroactively invalidate that act.

Loss of authority before commit prevents the act.

Authority state used by replay MUST be the state at the act's causal position, not current institutional state.

#### Required invariants

- Governing Institution exists.
- Every validator and resolver has applicable authority.
- Cross-institution acts identify coordination or superior authority.
- Singular offices obey singular-holder rules.
- Visibility and secrecy do not substitute for authority.
- Institutional dispute suspends affected reliance.

#### Illegal states

- Bond without governing Institution.
- Self-declared authority without source.
- Authority inferred from a string role.
- Revoked authority used for a later act.
- Current Institution state retroactively applied to old history.
- Institution resolving outside House jurisdiction.

#### Failure modes and recovery

Institution deletion, missing office holder, disputed succession, revoked grant, or incompatible jurisdictions MAY block a transition.

Recovery requires an authoritative institutional decision or successor process. The Bond MUST NOT invent a temporary authority.

#### Verification

Tests MUST cover role, office, membership, explicit grant, emergency authority, revocation, historical authority replay, singular office, and cross-institution coordination.

#### Repository mapping

Institution, Office, Membership, InstitutionalCapability, AuthoritySource, AccessGrant, and InstitutionalRelationship already provide most authority vocabulary. Their current mutable state and WorldTimestamp usage require historical snapshots or event references for Bond replay. Bond SHOULD reference these authoritative domain records rather than copy their rules.

### 5.29 House Authority

#### Purpose

House authority identifies which constitutional function governs the Bond.

The four existing Houses retain their established constitutional functions:

- Stonebend names;
- Sandmanor proves;
- Glaüshouse clears and alone performs final Synthesis;
- Flynt recognizes.

#### Responsibilities

Formation MUST identify exactly one governing House.

House authority MUST determine:

- which constitutional function is primary;
- which House-specific rules apply;
- which institutions may exercise jurisdiction;
- which validation or evidence requirements apply;
- whether cross-House participation is lawful;
- which House authority may challenge or recognize the result.

#### Non-responsibilities

A governing House MUST NOT:

- replace the governing Institution;
- imply a specific participant;
- imply a Current or Aura sign;
- imply success;
- grant Glaüshouse Synthesis authority to another House;
- change after formation.

#### Cross-House Bonds

Participants and institutions from multiple Houses MAY participate when jurisdiction permits.

Exactly one House remains governing for the Bond. Other House authorities MUST be recorded as participating, witnessing, validating, challenging, or recognizing roles rather than silently becoming co-governing Houses.

If no governing House can be selected under existing law, formation is invalid.

#### Required invariants

- Exactly one governing House.
- House is immutable after formation.
- House and Institution both exist.
- Glaüshouse retains final Synthesis authority.
- House sign stereotypes are forbidden.
- House-specific evidence remains externally authoritative.

#### Verification

Tests MUST cover each House, cross-House participants, jurisdiction conflict, immutable House, and exclusive Glaüshouse resolution authority.

#### Repository mapping

The House enum and canonical Fourway mapping are stable candidates. Existing House fixtures and institution catalogs supply authority evidence. No new House abstraction is required.

The following companion House instruments consolidate the audited non-Flynt law:

- `STONEBEND_CONSTITUTION_V2.md` (canonical and executable above this frozen common runtime);
- `STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md` (bounded vertical
  law, standard Aura Way, material Hollowing, proofed Aether provenance, and
  geographic stone-refraction supplement);
- `STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md` (bounded three-gate,
  scoped-Title, Diamond bearer, Claim/Title/Yield, Proliteriate mandate,
  two-power removal, Tombstone, and Lazerhorn succession supplement);
- `STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md` (bounded
  Stonebend recognition/activation, maintenance/renewal, targeted
  intervention, restoration, Diamond vacancy continuity, independent Forge
  replacement, and temporal Claim/Title/Yield supplement);
- `SANDMANOR_CONSTITUTION_V2.md` (canonical and executable above this frozen common runtime);
- `GLAUSHOUSE_CONSTITUTION_V2.md` (canonical and executable above this frozen common runtime).

The Stonebend, Sandmanor, and Glaüshouse V2 instruments supersede their V1
drafts and own only their House-specific law above this frozen common runtime.
They do not modify the common Bond lifecycle, accounting, persistence, replay,
or final Synthesis Resolution semantics defined here. The frozen common
`SandmanorContestSuccession` entry point remains closed because it cannot bypass
the ratified House-specific Contest and accession registry.

### 5.30 Recipe Participation

#### Purpose

A Recipe may specify a lawful operation performed during a Bond.

#### Required model

Recipe participation MUST identify:

- Recipe identity and version;
- Recipe owner;
- required inputs;
- intended outputs;
- participant roles;
- permission;
- execution authority;
- execution trace;
- Current movements caused by external Waves;
- Aura observations;
- obligation satisfied or created;
- whether the Recipe completed, missed, failed, or rolled back.

#### Non-responsibilities

A Recipe MUST NOT:

- become the Bond;
- bypass formation or validation;
- move Current before activation;
- define Bond maturity unless the term explicitly names Recipe completion as its completion predicate;
- form a Tombstone;
- record a Toke;
- perform final Synthesis Resolution merely because its type is named SynthesisRecipe.

#### Transactional participation

If Recipe execution is transactional, failed execution MUST commit no partial state change. Evidence of the failed attempt MAY remain.

Current movements that are part of the same atomic Recipe MUST commit consistently with the Recipe result.

#### Required invariants

- Recipe is legal before execution.
- Recipe version is fixed for one execution.
- Script or step order is deterministic.
- Execution evidence is immutable.
- Miss and failure do not fabricate successful obligations.
- Recipe success does not imply Bond success.

#### Illegal states

- Direct execution outside an Active validated Bond when the operation moves governed Current.
- Recipe version changed mid-execution.
- Partial script commit presented as success.
- Recipe output used as Tombstone.
- Recipe completion used as Toke.

#### Verification

Tests MUST cover lawful success, Miss, rollback, invalid Recipe, version mismatch, Current conservation, and Bond continuation after execution failure.

#### Repository mapping

The existing SynthesisRecipe, SynthesisScript, Aim, Fire, LandingOutcome, and SynthesisExecution path already provides deterministic transactional Recipe evidence. It SHOULD remain the execution authority. A Bond adapter should reference its trace rather than duplicate scripts or landing logic.

OfficialsOutlaws SynthesisRecipe represents domain composition sources and result. It is semantically narrower and SHOULD retain domain ownership. Shared naming SHOULD be clarified at module boundaries rather than forcing unlike Recipes into one type.

### 5.31 Transformation Participation

#### Purpose

Transformation participation records a lawful change of form, composition, state, or system that occurs inside a Bond.

#### Responsibilities

A transformation MUST identify:

- persistent underlying subject identity;
- before state;
- after state;
- governing Recipe or rule;
- eligibility decision;
- execution evidence;
- Current input and output custody;
- Aura observation;
- affected obligations;
- authority.

#### Non-responsibilities

Transformation MUST NOT:

- delete persistent Being identity;
- imply Bond maturity;
- imply proof;
- erase source materials or Current without explicit outputs or sinks;
- bypass domain-specific eligibility.

#### Required invariants

- Before state exists.
- After state differs only as authorized.
- Persistent identity survives when existing domain law requires it.
- Transformation legality remains owned by its domain.
- Bond records consequence and participation rather than duplicating the transformation engine.

#### Illegal states

- Transformation without subject.
- Result without legal source.
- Direct form fabrication.
- Identity deletion disguised as transformation.
- Bond validator replacing domain transformation validator.

#### Verification

Tests MUST cover legal and illegal transformation, persistent identity, rollback, evidence linkage, and separation from maturity.

#### Repository mapping

FrameState, BeingState, Sandmanor transition validation, OfficialsOutlaws transformation lineages, SynthesisExecution, and CompositionRecord are candidate evidence and provenance sources. None should be copied wholesale into the Bond.

### 5.32 Ownership and Custody

#### Purpose

Ownership answers who constitutionally possesses title. Custody answers who currently carries or controls.

#### Responsibilities

The Bond MUST track ownership and custody for:

- starting Current;
- every Current movement;
- retained Current;
- transferred Current;
- Recipe inputs and outputs where applicable;
- transformation materials and results where applicable;
- mature remainder;
- Tombstone ownership or custodianship;
- inherited obligations or interests.

#### Required invariants

- Ownership has an explicit source.
- Custody has an explicit source.
- A change is an event.
- Multiple ownership shares, if governing law permits them, have exact non-duplicating allocation.
- Custody cannot create ownership.
- Ownership cannot imply physical custody.
- Tombstone proof is historically associated with participants but is not mutable private property that may be destroyed.

#### Illegal states

- Ownerless Current inside a Bond.
- Custodianless retained Current.
- Duplicate full ownership.
- Ownership inferred from possession alone.
- Transfer without prior owner.
- Mature ownership changed in place.

#### Failure modes and recovery

Ownership dispute suspends affected transfer and eligibility.

Participant loss does not erase ownership. Authority may transfer or inherit through a successor Bond.

#### Verification

Tests MUST cover owner-custodian difference, shared allocation, dispute, transfer, inheritance, participant loss, and mature immutability.

### 5.33 Inheritance

#### Purpose

Inheritance carries a lawful interest, obligation, permission, custody, or Current remainder from a predecessor into a successor relationship.

#### Required model

Inheritance MUST identify:

- predecessor Bond;
- successor Bond;
- inherited subject;
- predecessor holder;
- successor holder;
- authority;
- inheritance rule;
- causal Wave;
- exact Current quantity, if any;
- obligation status;
- evidence.

#### Non-responsibilities

Inheritance MUST NOT:

- copy Current;
- copy history;
- reuse identity;
- silently forgive default;
- imply the successor has the predecessor's term;
- activate the successor.

#### Required invariants

- Inheritance uses a child or successor Bond.
- Parent history remains immutable.
- Successor formation declares inherited items.
- Current exits parent custody and enters successor custody exactly once.
- Inherited obligations preserve prior status and evidence.
- Completed obligations are not reopened unless governing law explicitly creates a new obligation.

#### Illegal states

- Inheritance without predecessor.
- Same Bond as predecessor and successor.
- Inherited Current lacking balanced transfer.
- Inherited authority without jurisdiction.
- Inherited term.
- Child activated solely by inheritance.

#### Verification

Tests MUST cover Current, obligation, permission, ownership, custody, default, multiple heirs, failed successor validation, and non-duplication.

#### Repository mapping

Current Grip inheritance concerns learned capability, not Bond inheritance. Institutional ObligationStatus Inherited is a useful obligation status but not the whole succession process. These concepts MUST remain distinct.

### 5.34 Parent Bonds and Child Bonds

#### Purpose

Parent and child references preserve lawful lineage across renewal, transfer, split, branch, merge, inheritance, challenge, and evolution.

#### Required relationships

- Renewal: one parent to one child.
- Transfer successor: one parent to one child unless governing transfer splits.
- Split or branch: one parent to two or more children.
- Merge: two or more parents to one child.
- Post-record material challenge: challenged Bond or proof lineage to one challenge Bond.
- Evolution into successor: one or more parents to one or more explicitly formed children as governing rules permit.

#### Required invariants

- References are by immutable identity.
- The lineage graph is acyclic.
- Parent and child are distinct.
- Every child has a complete independent formation.
- Every parent remains addressable.
- History is referenced, not copied.
- Child Current begins only with explicit starting or transferred Current.
- Child maturity is independent.

#### Illegal states

- Lineage cycle.
- Self-parent.
- Child without formation.
- Parent silently adopting child events.
- Child claiming parent Tombstone as its own.
- Merge child missing one parent.

#### Failure modes and recovery

Missing parent reference invalidates child validation.

Corrupt lineage index is rebuilt from authoritative formation and resolution records.

#### Verification

Tests MUST cover every relationship cardinality, cycles, missing references, independent terms, independent Tombstones, and deterministic traversal.

#### Repository mapping

CompositionCatalog already validates containment cycles but not Bond causal lineage. Its CompositionRecord sources and result can index parent-child provenance without becoming the Bond authority. A canonical Bond lineage SHOULD project into that neutral catalog rather than introduce a second generic graph.

### 5.35 Evidence

#### Purpose

Evidence permits constitutional claims to be inspected and replayed without copying the authoritative detail into Bond storage.

#### Required evidence classes

Where applicable, a Bond MUST reference evidence for:

- formation;
- initiating Wave;
- participant identity;
- role;
- House jurisdiction;
- institutional authority;
- ownership;
- custody;
- Current movements;
- Aura observations;
- obligations;
- permissions;
- Recipe execution;
- transformation;
- default;
- challenge;
- appeal;
- maturity;
- accounting;
- excess;
- condensation eligibility;
- Tombstone validation;
- Toke recording;
- Synthesis Resolution.

#### Evidence reference requirements

An evidence reference MUST identify:

- owning domain or namespace;
- stable key;
- relevant version;
- relevant causal position or immutable record;
- expected evidence role.

The Bond MUST NOT parse or reinterpret opaque domain evidence beyond the contract declared by that domain.

#### Required invariants

- Evidence is explicit.
- Evidence role is explicit.
- Evidence owner is explicit.
- Evidence is not copied when a stable authoritative reference exists.
- Missing evidence cannot be inferred.
- Contradictory evidence remains visible.
- Evidence added after maturity cannot be backdated into mature history.

#### Illegal states

- Tombstone without evidence.
- Mutable pointer whose target can change meaning.
- Evidence reference without owner namespace.
- Copied trace diverging from authoritative trace.
- Validation using evidence from the wrong causal position.

#### Failure modes and recovery

Unavailable evidence MAY block validation or eligibility.

Corrupt evidence requires challenge, recovery from authoritative storage, or ineligibility. The Bond MUST NOT fabricate a replacement.

#### Verification

Tests MUST cover missing, duplicate, contradictory, unavailable, corrupt, wrong-version, wrong-role, and replay-equivalent evidence.

#### Repository mapping

ExternalRef is the leading neutral reference candidate. KernelPass, DecisionTrace, SynthesisExecution, AuraPolarityEvaluation, PointSquaredAscension, ConstitutionalWitness, composition records, and artifact build records are candidate evidence owners.

### 5.36 Witnesses

#### Purpose

Witnesses provide explicit observational or institutional attestation where governing law requires it.

#### Responsibilities

A witness record MUST include:

- Witness identity;
- witnessed fact or causal range;
- role;
- standing;
- Institution or House relationship where applicable;
- attestation;
- evidence;
- causal position.

#### Non-responsibilities

Witnesses MUST NOT:

- create Current;
- replace Current accounting;
- replace deterministic replay;
- validate outside standing;
- remain implicit;
- overwrite conflicting witnesses.

#### Required invariants

- Required witness count and roles come from governing rules.
- Duplicate identity does not satisfy multiple witness positions unless rules explicitly permit it.
- Conflicting witness statements coexist.
- Witness loss after attestation does not erase the attestation.
- Anonymous display strings are insufficient.

#### Illegal states

- Required witness omitted.
- Empty attestation.
- Witness to an event occurring before the witness had standing, unless the witness is attesting from lawful evidence rather than direct observation.
- One identity duplicated to fake quorum.
- Witness statement treated as movement.

#### Verification

Tests MUST cover quorum, duplicate identity, conflicting statements, institutional standing, historical replay, and missing witness.

#### Repository mapping

ConstitutionalWitness and institutional witness requirements provide candidates. Current String-based participant and witness presentation is not sufficient for canonical identity.

### 5.37 Deterministic Replay

#### Purpose

Replay proves that the Bond's authoritative results follow from its immutable inputs and ordered history.

#### Replay inputs

Replay MUST consume:

- formation;
- governing rule versions;
- ordered events;
- historical authority references;
- evidence references;
- term definition;
- maturity boundary;
- persisted validation decisions where the decision itself is historical;
- no hidden mutable global input.

#### Replay outputs

Replay MUST reproduce:

- lifecycle state;
- ownership and custody;
- all Current totals;
- all Aura totals;
- evaluations;
- polarity;
- default findings derived by rule;
- maturity readiness and boundary;
- excess;
- eligibility;
- Tombstone payload, if any;
- expected Toke reference, if any;
- final resolution result.

#### Required invariants

- Identical inputs produce byte-equivalent canonical outputs or semantically identical outputs under an explicitly versioned canonicalization rule.
- Replay does not create a new event.
- Replay does not execute external side effects.
- Replay uses historical authority, not current authority.
- Replay rejects unknown required rules.
- Replay reports the earliest divergence.

#### Illegal states

- Replay that writes history.
- Replay using current wall clock.
- Replay using randomness.
- Replay that skips missing events.
- Replay that accepts a different Tombstone.
- Replay that cannot explain a Toke.

#### Failure modes and recovery

Missing rule version, missing event, corrupt evidence, or arithmetic divergence is a replay failure and blocks condensation or recording.

#### Verification

Tests MUST replay every canonical fixture twice, reorder physical storage, rebuild indexes, restart the process, inject corruption at every stage, and prove equal results or precise failure.

#### Repository mapping

DecisionTrace replay, Current Synthesis event reconstruction, state snapshots, KernelPass witnesses, and composition witnesses are strong foundations. Current Synthesis snapshots already reject unknown fields and require selected candidates to exist. The Bond layer SHOULD reuse those patterns while adding schema version, signed accounting, term, authority, and immutable artifact boundaries.

### 5.38 Persistence

#### Purpose

Persistence preserves constitutional history across process, machine, repository, and implementation lifetime.

#### Responsibilities

Persistence MUST:

- commit each authoritative transition atomically;
- preserve causal order;
- preserve immutable payloads exactly or through lossless versioned representation;
- detect duplicates;
- detect torn writes;
- detect missing references;
- support replay from authoritative records;
- permit caches and indexes to rebuild;
- preserve terminality.

#### Non-responsibilities

Persistence MUST NOT:

- become domain authority;
- infer missing events from a later snapshot without an explicit migration rule;
- silently discard a torn authoritative record;
- rewrite a validated Tombstone;
- compact away Current or Aura history needed for replay.

#### Authoritative and derived storage

Authoritative storage includes committed formation and event records and immutable lifecycle artifacts.

Snapshots are derived acceleration structures unless explicitly designated as immutable authoritative artifacts.

Indexes are derived.

Display artifacts are derived.

#### Required invariants

- Restart produces the same state.
- Snapshot disagreement yields replay from authoritative events or corruption failure; snapshot never silently wins.
- Duplicate retry is idempotent.
- Terminal state survives.
- Tombstone and Toke survive independent of the live Bond process.

#### Verification

Tests MUST cover restart after every transition, torn final record, duplicate record, stale snapshot, missing snapshot, corrupt index, cache deletion, and immutable artifact survival.

### 5.39 Serialization

#### Purpose

Serialization provides a deterministic, versioned representation of constitutional records.

#### Required format properties

Every authoritative serialized record MUST include:

- schema identity;
- schema version;
- record kind;
- stable identity;
- all required fields;
- explicit sign and magnitude;
- explicit units;
- explicit references;
- canonical collection ordering rules;
- a deterministic representation of optional and empty values.

Canonical serialization MUST:

- use exact base-ten integers;
- forbid floating-point constitutional values;
- distinguish missing from empty;
- sort semantic sets by stable identity;
- preserve ordered sequences where causal or Recipe order matters;
- reject duplicate map keys;
- reject unknown required enum values;
- preserve Unicode text without using display text as identity;
- define escaping;
- be deterministic across supported platforms.

#### Non-responsibilities

Serialization MUST NOT:

- generate identity;
- validate domain legality by itself;
- infer defaults for missing required constitutional fields;
- depend on field insertion order;
- treat a parser's permissiveness as authority.

#### Illegal states

- Unversioned Tombstone or Toke.
- Signed Current serialized as an ambiguous unsigned loss.
- Map with duplicate keys.
- Unknown lifecycle state accepted as Active.
- Missing term defaulted to perpetual.
- Missing evidence defaulted to valid.

#### Verification

Tests MUST cover round trip, canonical byte stability, field reordering at parse, duplicate keys, unknown versions, unknown variants, escaping, large integers, sign, empty versus missing, and cross-platform fixtures.

### 5.40 Migration

#### Purpose

Migration carries constitutional records across schema versions without changing their identity, causal meaning, Current, Aura, proof, or history.

#### Responsibilities

A migration MUST:

- identify source and destination schema versions;
- be deterministic;
- be idempotent;
- preserve stable identities for the same constitutional entities;
- preserve all immutable facts;
- preserve gross and net accounting;
- preserve event order;
- preserve evidence references;
- preserve Tombstone and Toke meaning;
- record migration evidence;
- support replay equivalence.

#### Non-responsibilities

Migration MUST NOT:

- invent a missing term;
- invent participants;
- infer roles from names;
- create signed values from ambiguous unsigned history without authoritative evidence;
- turn a legacy candidate into a validated Bond by assumption;
- change a resolved Bond to Active;
- merge unlike entities because names match;
- discard unknown historical data needed for proof.

#### Legacy incompleteness

A legacy record missing a required constitutional field MUST remain explicitly non-canonical or migration-incomplete until authoritative evidence supplies that field.

It MUST NOT form a Tombstone or Toke merely because migration code can choose a default.

#### Required invariants

- Old and migrated replay results are constitutionally equivalent.
- Migration can be repeated without further change.
- Failed migration leaves source data intact.
- Tombstone and Toke remain immutable historical artifacts.
- Semantic change requires a new Bond or successor artifact rather than hidden migration.

#### Verification

Tests MUST cover every version edge, repeated migration, partial failure, legacy missing term, ambiguous sign, unknown evidence, identity preservation, and replay equivalence.

#### Repository mapping

Current composition persistence is explicitly deferred, while Current Synthesis uses custom text event and snapshot formats without a general schema version. A canonical Bond path requires versioning before runtime adoption. Existing artifacts SHOULD be imported as evidence or legacy records, not silently promoted.

### 5.41 Bond Resolution

#### Purpose

Bond Resolution records the final result selected by Synthesis Resolution.

#### Lawful resolution dispositions

Resolution MAY:

- complete;
- renew;
- merge;
- branch;
- split;
- challenge;
- transfer;
- dissolve;
- inherit;
- evolve into a successor Bond.

These dispositions are not interchangeable. Each MUST satisfy its own rules.

#### Required inputs

Resolution MUST consume:

- a mature Bond, except pre-activation cancellation which uses its own terminal outcome;
- excess result;
- eligibility result;
- validated Tombstone and Toke when condensation occurred;
- explicit Not Formed and Not Applicable outcomes when condensation did not occur;
- all blocking challenge and appeal decisions;
- resolving Glaüshouse authority;
- successor formations or references where the disposition creates successors;
- complete Current disposition.

#### Non-responsibilities

Resolution MUST NOT:

- create a Tombstone;
- validate a Tombstone;
- create a Toke except through the prior recording stage;
- reopen living chemistry;
- delete history;
- imply a successor without formation;
- copy Current to a successor.

#### Required invariants

- At most one final resolution.
- Resolution is terminal.
- Toke precedes final resolution when a Tombstone exists.
- Explicit ineligibility precedes resolution when no Tombstone exists.
- Every successor has a new identity.
- Every remaining Current quantity has a lawful disposition.
- History remains addressable.

#### Illegal states

- Resolution before maturity for an activated Bond.
- Resolution before Tombstone validation when a Tombstone formed.
- Resolution with an unrecorded validated Tombstone.
- Resolution claiming a Toke for zero excess.
- Multiple resolutions.
- Resolved Bond returning to Active.
- Successor lacking formation.

#### Failure modes and recovery

If final resolution persistence fails, the Bond remains at its last committed pre-resolution state and retry is idempotent.

If successor formation fails, resolution MUST either fail atomically or record a disposition that does not claim the successor exists, according to the governing resolution rule.

#### Verification

Tests MUST cover every disposition, with and without Tombstone, atomic successor formation, Current disposition, exactly-once terminality, and replay.

### 5.42 Consolidated Bond invariants

The implementation MUST guarantee all of the following:

- Exactly one Bond identity.
- Exactly one formation.
- Exactly one initiating Wave reference at formation.
- One or more participants.
- At least one explicit role per participant.
- Exactly one governing House.
- Exactly one governing Institution.
- Exactly one original term.
- Exactly one successful activation at most.
- Exactly one maturity at most.
- Exactly one authoritative excess result at most.
- At most one condensation.
- At most one Tombstone.
- At most one successful Tombstone validation decision.
- At most one Toke.
- At most one final resolution.
- Wave precedes every Current movement.
- Formation precedes validation.
- Validation precedes activation.
- Activation precedes circulation.
- Circulation precedes accumulation.
- Accumulation and observation precede evaluation.
- Term completion precedes maturity.
- Maturity precedes excess calculation.
- Excess calculation precedes eligibility.
- Eligibility precedes Tombstone formation.
- Tombstone formation precedes Tombstone validation.
- Tombstone validation precedes Toke recording.
- Toke recording precedes final resolution when a Tombstone exists.
- Explicit ineligibility precedes final resolution when no Tombstone exists.
- Positive and Negative Current remain separately reconstructable.
- Positive and Negative Aura remain separately reconstructable.
- Aura never changes Current.
- Current never changes historical Aura.
- No Current appears without causal origin.
- No Current disappears without destination or disposition.
- Ownership and custody remain explicit.
- History is append-only.
- Mature history is immutable.
- Validated Tombstones are immutable forever.
- Recorded Tokes are immutable forever.
- Resolved Bonds never become Active.
- Parent and child identity graphs remain acyclic.
- Replay reproduces every authoritative derived result.

### 5.43 Consolidated illegal Bond states

The following configurations are constitutionally impossible and MUST be rejected:

- Bond without identity.
- Bond with multiple identities.
- Bond without formation.
- Bond with multiple formations.
- Bond without participant.
- Implicit participant.
- Participant without role.
- Role without participant.
- Bond without initiating Wave.
- Bond without House.
- Bond without Institution.
- Bond without term.
- Negative duration.
- Simultaneously finite and perpetual term.
- Activated Bond without validation.
- Multiple activations.
- Current before activation.
- Current after maturity.
- Current without source.
- Current without destination.
- Current without unit.
- Zero Current movement.
- Duplicated Current.
- Destroyed Current.
- Split imbalance.
- Merge imbalance.
- Consumed Current without result or sink.
- Aura observation without observer.
- Aura observation without scope.
- Zero Aura contribution.
- Aura modifying Current.
- Current modifying historical Aura.
- Evaluation omitting a signed domain.
- Successful fifth polarity state.
- Successful polarity with zero net Current.
- Successful polarity with zero or unevaluable net Aura.
- Maturity before activation.
- Maturity before term completion.
- Multiple maturities.
- Mutable mature history.
- Excess before maturity.
- Aura offset as Current.
- Simultaneous positive and negative excess.
- Eligible zero excess.
- Condensation before eligibility.
- Condensation before maturity.
- Multiple condensations.
- Tombstone without condensation.
- Tombstone without evidence.
- Tombstone whose excess differs from the Bond.
- Multiple Tombstones from one Bond.
- Mutable Tombstone payload.
- Toke without validated Tombstone.
- Multiple Tokes for one Tombstone.
- Mutable Toke.
- Final resolution with blocking challenge.
- Resolution before required recording.
- Multiple final resolutions.
- Resolved Bond becoming Active.
- Renewal under same identity.
- Transfer duplicating ownership or Current.
- Split child reusing parent identity.
- Merge child reusing parent identity.
- Parent-child cycle.
- Child without independent formation.
- Deleted constitutional history.
- Automatic correction of committed history.
- Identity derived from randomness, wall clock, insertion order, memory address, or mutable fields.
- Nondeterministic replay.

### 5.44 Consolidated Bond failure modes

#### Validation failure

Prevents activation. Constitutional invalidity produces rejection. Infrastructure failure produces no decision.

#### Evidence failure

Prevents the dependent validation, evaluation, eligibility, or recording gate. Evidence is recovered from its authority or the gate fails; evidence is never fabricated.

#### Institutional dispute

Suspends affected authority. A competent reviewing authority must resolve jurisdiction. Current MUST NOT move under disputed unauthorized control.

#### Current imbalance

Blocks the affected movement or mature accounting. Committed lawful history remains. No saturating correction is permitted.

#### Aura ambiguity

Produces incomplete evaluation. It does not produce a guessed sign and cannot satisfy eligibility.

#### Participant loss

Does not delete the participant or obligations. It may lead to default, transfer, inheritance, early maturity, or dissolution under governing rules.

#### Interrupted Bond

Restarts from the last committed authoritative state. No stage is assumed complete because an operation began.

#### Partial execution

The prospective atomic transition rolls back. Already committed prior history remains.

#### Corruption

The affected record is isolated from authoritative use. Replay from intact authoritative data is attempted. If replay cannot restore certainty, condensation and recording are blocked.

#### Rollback

Rollback applies only to an uncommitted working transition. Rollback never removes a committed historical event.

#### Retry

Retry is idempotent and uses the same transition identity or expected state. It never duplicates Current, Tombstone, or Toke.

#### Cancellation

Pre-activation cancellation is terminal without maturity or proof. Active cancellation follows the term and termination rules and reaches maturity.

#### Challenge and appeal

They preserve original decisions, suspend only authorized scope, and produce new decision history.

### 5.45 Constitutional Bond verification suite

A conforming implementation MUST provide tests that prove:

1. Formation is complete, unique, atomic, and non-active.
2. Every validation rule independently blocks activation.
3. Activation occurs exactly once.
4. Every Current operation conserves sign, magnitude, and unit.
5. Positive and Negative Current coexist without loss.
6. Positive and Negative Aura coexist without loss.
7. All four polarity states are reachable and deterministic.
8. Zero or ambiguous domains never produce a successful polarity.
9. Aura cannot mutate Current.
10. Current cannot mutate historical Aura.
11. Accumulation equations hold under arbitrary lawful event orderings.
12. Physical storage order does not change causal results.
13. Finite and perpetual term rules hold.
14. Maturity is success-independent and exactly once.
15. No post-maturity living history is accepted.
16. Positive, negative, and zero excess calculate exactly.
17. Every eligibility condition is necessary.
18. Condensation is exact, atomic, irreversible, and at most once.
19. Default does not invent Current or force automatic resolution.
20. Challenge and appeal preserve original history.
21. Transfer, split, merge, renewal, and inheritance conserve Current.
22. Parent and child graphs are acyclic.
23. Every child validates independently.
24. Recipe and transformation failure do not partially commit.
25. Authority is evaluated at the historical causal position.
26. Evidence and witnesses remain explicit.
27. Replay reproduces every authoritative result.
28. Restart after every stage preserves state.
29. Torn writes and stale snapshots do not become history.
30. Serialization is canonical and versioned.
31. Migration is lossless, idempotent, and replay-equivalent.
32. Zero excess resolves without Tombstone or Toke.
33. Non-zero valid excess resolves only after Tombstone and Toke.
34. A resolved Bond cannot become Active.

Property and model-based tests SHOULD generate every legal state path and attempt every illegal transition from every state.

### 5.46 Consolidated Bond repository mapping

#### Candidate types to retain and refine

- House is the canonical House identifier candidate.
- InstitutionId, InstitutionCatalog, Office, Role, Membership, InstitutionalCapability, and institutional relationships are authority candidates.
- InstitutionalObligation is an obligation reference candidate.
- ExternalRef is the evidence-reference candidate.
- CompositionNodeId and CompositionRecordId provide proven stable cross-domain identity patterns.
- KernelPass, DecisionTrace, SynthesisExecution, AuraPolarityEvaluation, PointSquaredAscension, and ConstitutionalWitness are evidence candidates.
- CurrentSynthesisEvent and replay/snapshot code provide event-sourcing and recovery patterns.
- CompositionCatalog provides neutral source/result and containment indexing.

#### Narrow abstractions that are not constitutional Bonds

- Bond in hollow_grove.rs is a selected kernel Way.
- PlebMetaBond is a routing-mode pair.
- Aim.bond is a kernel contact relation.
- BondCandidate is an unformed decision option.
- BondResult is one selection result.
- MoxyRelation::Bond is a world-map relation label.
- InstitutionalRelationship is a relationship fact.

These SHOULD NOT be deleted merely because their names overlap. They SHOULD be renamed or scoped clearly when the canonical Bond aggregate is introduced.

#### Duplicate or overlapping abstractions

- Stable identifier lexical validation is repeated in the composition, institution, affiliation, and Flynt constitutional layers. The lexical validator SHOULD be consolidated while retaining distinct typed identities.
- SynthesisRecipe names two domain-specific concepts. Their execution and composition semantics differ, so a blind type merge is forbidden; clearer namespaces or shared reference contracts are preferable.
- Replay and persistence patterns exist in DecisionTrace and Current Synthesis. Canonical Bond persistence SHOULD consolidate infrastructure without merging domain meaning.
- Evidence currently appears as ExternalRef, ConstitutionalWitness, DecisionTrace, KernelPass witness text, SynthesisExecution, and artifact references. The Bond SHOULD reference these through one evidence-reference boundary rather than duplicate their payloads.

#### Missing abstractions

The repository lacks:

- canonical Bond identity and aggregate;
- term;
- lifecycle state;
- signed Current ledger;
- signed Aura observation ledger;
- four-state evaluation result;
- maturity record;
- net excess result;
- condensation eligibility decision;
- Tombstone;
- Tombstone validation decision;
- Toke;
- challenge and appeal record;
- Bond parent-child lineage;
- final Synthesis Resolution record;
- versioned canonical Bond serialization.

#### Dead abstractions

The audit found no existing abstraction that can safely be declared dead solely from this specification. Existing narrow Bond names have active tests and consumers. Deletion requires a later implementation audit after adapters exist.

#### Simplification and deletion direction

- Do not add a parallel House, Institution, Recipe, transformation, or evidence engine.
- Do not copy detailed traces into Bond records.
- Reuse the neutral composition catalog for cross-domain indexing where its contract fits.
- Delete transitional adapters after all consumers use the canonical boundary.
- Delete duplicate identifier grammar implementations after a shared validator exists.
- Preserve typed identity distinctions even if validation code is consolidated.
- Keep frozen Recipe execution behavior behind its existing façade.

## 6. Tombstone

### 6.1 Purpose

Tombstone exists to answer:

> What mature constitutional consequence was proven to remain after this Bond completed?

The engine requires Tombstone because living Bond history is not itself durable condensed proof. A Bond may contain extensive history yet have zero excess or fail eligibility. Tombstone identifies the exact mature excess that lawfully became permanent constitutional matter.

### 6.2 Material meaning

A Tombstone represents the durable material consequence of a completed Bond.

It permanently represents:

- what actually occurred;
- what actually remained;
- which Bond produced the remainder;
- who participated;
- which roles they held;
- which House governed;
- which Institution governed;
- what term completed;
- which obligations completed;
- which obligations remained;
- which defaults occurred;
- what ownership and custody existed at maturity;
- what transformation occurred;
- what signed Current excess remained;
- how signed Aura revealed that Current;
- which polarity state applied;
- which evidence and witnesses support the result.

The Tombstone never replaces the Bond.

The Tombstone survives the Bond.

### 6.3 Responsibilities

A Tombstone SHALL:

- possess exactly one immutable identity;
- reference exactly one source Bond;
- reference exactly one condensation event;
- preserve the sign, magnitude, unit, and scale of mature excess;
- preserve the mature polarity state;
- identify participants and roles;
- identify House and Institution;
- identify term and maturity boundary;
- identify completed and remaining obligations;
- identify default findings;
- identify ownership and custody at maturity;
- identify Recipe and transformation outcomes where relevant;
- reference evidence;
- reference witnesses;
- permit deterministic validation and replay;
- remain immutable;
- remain permanently addressable after validation;
- be eligible for exactly one Toke recording after validation.

### 6.4 Non-responsibilities

A Tombstone SHALL NOT:

- contain living mutable chemistry;
- circulate Current;
- observe new Aura;
- change the source Bond;
- replace the complete event ledger;
- act as its own validation decision;
- record itself as a Toke;
- perform Synthesis Resolution;
- authorize a Recipe;
- create ownership;
- conceal a remaining obligation;
- summarize away contradictory evidence;
- change after challenge.

### 6.5 Constitutional questions

Tombstone answers:

- Which Bond matured?
- Which exact maturity boundary applies?
- What signed excess remained?
- Which Current was offset and which remained?
- What did Aura reveal about the remainder?
- Which of the four polarity states applies?
- Who participated and in which roles?
- Which obligations completed?
- Which obligations remain?
- Which ownership and custody facts apply?
- Which evidence proves the claims?
- Which witnesses attest?
- Which authority formed the proof?
- Can independent validation reproduce it?

### 6.6 Domain model

Every Tombstone payload MUST contain or reference:

- Tombstone identity;
- schema identity and version;
- source Bond identity;
- condensation identity;
- formation causal position;
- mature Bond state reference;
- maturity record reference;
- excess calculation reference;
- eligibility decision reference;
- excess sign;
- excess magnitude;
- unit and scale;
- positive mature remainder before offset;
- negative mature remainder before offset;
- offset amount;
- mature polarity state;
- participant identities;
- role assignments;
- governing House;
- governing Institution;
- term reference;
- completed obligation references;
- remaining obligation references;
- default references;
- ownership references;
- custody references;
- Recipe references;
- transformation references;
- evidence references;
- witness references;
- replay input boundary;
- payload version.

A Tombstone MUST NOT embed mutable validation status in its immutable proof payload. Validation is a separate immutable decision associated with the payload.

### 6.7 Identity

Tombstone identity MUST:

- be stable;
- be unique;
- be immutable;
- not be derived from mutable Bond fields;
- not depend on time, randomness, memory address, or insertion order;
- remain resolvable for repository lifetime;
- be distinct from Bond identity and Toke identity.

The one-to-one Bond-to-Tombstone rule MUST be enforceable by source Bond identity, not merely by naming convention.

### 6.8 Formation

#### Required conditions

Tombstone Formation requires:

- Mature Bond;
- valid excess calculation;
- CondensationEligible decision;
- non-zero excess;
- authorized condensation;
- no existing Tombstone for the Bond.

#### Atomicity

Formation MUST atomically create one complete immutable payload.

No partially formed Tombstone is constitutional.

#### Required invariants

- Condensation precedes formation.
- Exactly one source Bond.
- Payload excess equals calculated excess exactly.
- Payload polarity equals mature polarity.
- Payload participants and roles equal formed Bond history.
- Payload obligation states equal the maturity snapshot.
- Evidence references are complete under governing rules.

### 6.9 Proof semantics

The Tombstone proves only claims encoded in its payload and supported by its validated evidence.

It does not prove:

- unrelated facts;
- future Bond behavior;
- moral goodness;
- universal institutional recognition;
- that no challenge will ever occur;
- that remaining obligations disappeared;
- that every participant succeeded.

Proof is durable but scoped.

The source Bond is authoritative for full causal detail. The Tombstone is authoritative for the validated mature consequence.

### 6.10 Evidence

Tombstone evidence MUST be sufficient to:

- resolve the source Bond;
- replay formation through maturity;
- reproduce Current accumulation;
- reproduce Aura evaluation;
- reproduce excess;
- reproduce eligibility;
- verify authority;
- verify participant and role integrity;
- verify obligations;
- verify Recipe or transformation outcomes claimed by the payload.

Evidence MUST be referenced through stable authoritative records where possible.

Evidence unavailability during validation blocks validation. It does not mutate the payload.

### 6.11 Validation

#### Purpose

Tombstone Validation independently answers:

> Does this formed Tombstone lawfully and exactly prove the mature consequence of its source Bond?

#### Responsibilities

Validation MUST verify:

- Tombstone identity uniqueness;
- source Bond existence;
- source Bond maturity;
- source Bond eligibility;
- one-to-one Bond relationship;
- condensation legality;
- excess equality;
- polarity equality;
- participant integrity;
- role integrity;
- House and Institution integrity;
- obligation integrity;
- ownership and custody integrity;
- Recipe and transformation integrity;
- evidence integrity;
- witness integrity;
- causal completeness;
- deterministic replay;
- absence of contradiction;
- validator authority.

#### Independence

Tombstone validation is a stage separate from:

- Bond validation;
- maturity;
- eligibility;
- condensation;
- recording.

The same implementation component MAY perform multiple computations, but it MUST produce distinct decisions and MUST NOT use one stage label to bypass another.

#### Outcomes

- Validated;
- Rejected;
- Pending due to explicitly unavailable required evidence or authority.

Only Validated may proceed to Toke Recording.

#### Required invariants

- Payload never changes during validation.
- Findings are explicit.
- Identical inputs produce identical decision.
- Rejection creates no Toke.
- Pending creates no Toke.
- Successful validation commits exactly once.

#### Illegal states

- Self-validating payload flag treated as decision.
- Validation before formation.
- Validation without source Bond replay.
- Validation ignoring contradictory evidence.
- Validator modifying payload to make it pass.
- Toke created from pending or rejected proof.

### 6.12 Lifecycle and state machine

Legal Tombstone states are:

- Formed;
- ValidationPending;
- Validated;
- Rejected;
- Recorded.

Legal transitions are:

- Formed to ValidationPending.
- ValidationPending to Validated.
- ValidationPending to Rejected.
- ValidationPending to ValidationPending for idempotent retry with unchanged payload and inputs not yet available.
- Validated to Recorded after Toke commit.

Validated and Rejected are terminal for the validation decision.

Recorded is terminal for the Tombstone lifecycle.

Challenge status MAY be attached without mutating these states.

Illegal transitions include:

- Rejected to Validated by changing payload;
- Recorded to Formed;
- Validated to Formed;
- any state to a different source Bond;
- any state to a second Tombstone identity for the same Bond.

### 6.13 Ownership and institutional recognition

Tombstone ownership MUST NOT mean a participant may alter, conceal, or destroy constitutional proof.

The payload MAY identify:

- beneficiary;
- custodian;
- repository;
- recognizing Institution;
- recording authority.

Custody may transfer only as a recorded archival or institutional act that leaves the proof unchanged and addressable.

Recognition by one Institution does not imply recognition by every Institution. Recognition status MUST be represented by explicit evidence or later Bonds rather than mutation of the Tombstone.

### 6.14 Challenges

A challenge before Toke Recording MAY suspend validation or recording.

A challenge after Toke Recording MUST NOT mutate or delete the Tombstone.

A material post-record challenge MUST:

- form a new Bond;
- reference the challenged Tombstone and Toke;
- carry its own term, evidence, Current, Aura, maturity, and possible proof;
- produce a later historical consequence if successful.

The historical record then contains both the original proof and the proven challenge consequence.

### 6.15 Immutability and historical permanence

The Tombstone payload is immutable from formation.

After validation:

- it MUST never be overwritten;
- it MUST never be deleted as ordinary cleanup;
- it MUST never be compacted into an unreplayable summary;
- it MUST never change owner references in place;
- it MUST never change evidence references in place;
- it MUST never change sign, magnitude, polarity, or source Bond;
- it MUST remain available through its Toke and direct identity.

Physical storage replacement is lawful only through lossless migration that preserves identity, payload meaning, and replay equivalence.

### 6.16 Relationship to Bonds

- Every Tombstone references exactly one Bond.
- A Bond has at most one Tombstone.
- A Tombstone is downstream of maturity and eligibility.
- A Tombstone contains mature proof, not living state.
- The Bond remains the source of full history.
- Parent and child Bonds have separate Tombstones.
- A child MUST NOT claim its parent's Tombstone.

### 6.17 Relationship to Tokes

- A validated Tombstone may have exactly one Toke.
- The Tombstone precedes the Toke.
- The Toke points to the Tombstone.
- The Tombstone does not point to speculative or failed recording.
- Toke indexes do not change Tombstone meaning.

### 6.18 Relationship to Synthesis

Synthesis Resolution consumes the validated-and-recorded result when a Tombstone exists.

Synthesis MUST NOT:

- form the Tombstone;
- mutate the Tombstone;
- replace the Tombstone with a successor;
- resolve a condensing Bond before the Toke exists.

### 6.19 Persistence and serialization

Tombstone persistence MUST satisfy the Bond persistence and serialization rules plus:

- payload bytes or canonical semantics remain stable;
- source Bond reference is mandatory;
- all quantities are exact;
- validation decision is serialized separately;
- unknown Tombstone version prevents validation until migrated;
- no parser default may supply missing proof fields;
- archival copies MUST be distinguishable from additional Tombstones.

### 6.20 Migration

Tombstone migration MUST:

- preserve Tombstone identity;
- preserve source Bond;
- preserve every proof claim;
- preserve excess;
- preserve polarity;
- preserve evidence;
- preserve validation decision;
- preserve Toke reference;
- be deterministic and idempotent;
- prove replay equivalence.

If a target schema cannot represent every source claim, migration MUST fail and preserve the source representation.

A migration MUST NOT form a replacement Tombstone from the same Bond.

### 6.21 Required invariants

- Exactly one immutable Tombstone identity.
- Exactly one source Bond.
- Exactly one condensation reference.
- Source Bond is mature.
- Source Bond is eligible.
- Excess is non-zero.
- Excess equals source calculation.
- Polarity equals source mature evaluation.
- Participants and roles match source history.
- Evidence is explicit.
- Tombstone payload is immutable.
- Validation is independent.
- Only validated Tombstone may be recorded.
- At most one Toke.
- Historical permanence survives Bond resolution.

### 6.22 Illegal states

- Tombstone without identity.
- Tombstone without Bond.
- Tombstone with multiple source Bonds.
- Tombstone before maturity.
- Tombstone from zero excess.
- Tombstone without eligibility.
- Tombstone without condensation.
- Tombstone with mismatched excess.
- Tombstone with mismatched polarity.
- Tombstone missing participants or roles.
- Tombstone missing required evidence.
- Mutable payload.
- Payload modified during validation.
- Validated Tombstone with failed replay.
- Multiple Tombstones for one Bond.
- Recorded Tombstone without Toke.
- Toke pointing to rejected Tombstone.
- Deleted validated Tombstone.
- Migrated Tombstone with changed meaning.

### 6.23 Failure modes and recovery

#### Formation failure

No Tombstone commits. Retry is idempotent.

#### Evidence failure

Validation remains pending or rejects according to governing rules. Payload remains unchanged.

#### Replay failure

Validation rejects and recording is forbidden.

#### Authority failure

Validation or recording cannot proceed.

#### Contradiction

Validation rejects or a challenge suspends reliance. Contradiction is preserved in findings.

#### Storage corruption

Recover from an identical archival copy or reconstruct the expected payload by replay solely to verify the copy. Reconstruction MUST NOT silently create a second Tombstone identity.

#### Post-record error claim

Use a challenge Bond. Do not edit history.

### 6.24 Verification

Constitutional tests MUST prove:

- exact formation from both positive and negative excess;
- zero-excess rejection;
- premature rejection;
- source Bond one-to-one enforcement;
- payload immutability;
- evidence completeness;
- participant, role, House, Institution, term, obligation, ownership, Recipe, and transformation equality;
- independent validation;
- replay equality;
- contradiction rejection;
- no Toke on pending or rejection;
- exactly one Toke on success;
- post-record challenge through a new Bond;
- persistence across restart;
- canonical serialization;
- lossless migration;
- permanent lookup after source Bond resolution.

### 6.25 Repository mapping

No Tombstone type exists in the audited repository.

Candidate supporting structures are:

- CompositionNode as an addressable durable result projection;
- CompositionRecord as causal provenance;
- ExternalRef as evidence;
- the Flynt constitutional audit as a domain example of evidence-backed hierarchy validation;
- the unique Chimera CompositionRecord as a domain example of evidence-backed achieved synthesis;
- DecisionTrace, SynthesisExecution, KernelPass, and Current Synthesis history as replay evidence.

None is equivalent to Tombstone.

CompositionNode MAY project a Tombstone into the neutral provenance catalog. CompositionRecord MAY record the condensation operation. Neither neutral type may validate Bond legality or replace the immutable Tombstone payload.

No existing abstraction is a deletion candidate merely because Tombstone is introduced. The implementation SHOULD add the missing proof aggregate and reuse the neutral indexing boundary.

## 7. Toke

### 7.1 Purpose

Toke exists to answer:

> Where is this already-validated Tombstone permanently recorded in constitutional history?

A Tombstone proves. A Toke records.

The engine requires Toke because durable proof and permanent historical indexing are distinct constitutional acts.

### 7.2 Responsibilities

A Toke SHALL:

- possess exactly one immutable identity;
- reference exactly one validated Tombstone;
- reference the Tombstone's source Bond;
- identify the recording Institution;
- identify the recording authority;
- identify the causal position of recording;
- identify the schema version;
- identify the applicable historical repository or namespace;
- preserve evidence of successful Tombstone validation;
- support deterministic lookup;
- support replay navigation;
- remain immutable;
- remain permanent;
- survive Tombstone source Bond resolution;
- be recorded atomically and at most once.

### 7.3 Non-responsibilities

A Toke SHALL NOT:

- speculate;
- point to an unvalidated Tombstone;
- prove the Bond independently;
- copy the Tombstone payload;
- copy the Bond event history;
- change Current;
- change Aura;
- change ownership;
- validate the Tombstone;
- perform Synthesis Resolution;
- become a mutable status row;
- depend on repository insertion order.

### 7.4 Constitutional questions

Toke answers:

- Which validated Tombstone was recorded?
- Which Bond produced it?
- Which Institution recorded it?
- Which authority performed recording?
- At which causal position was recording committed?
- Under which schema and repository namespace can it be found?
- Can lookup locate the same immutable proof?
- Can replay navigate from Toke to Tombstone to Bond?

### 7.5 Domain model

Every Toke MUST contain:

- Toke identity;
- schema identity and version;
- Tombstone identity;
- source Bond identity;
- Tombstone validation decision identity;
- recording Institution identity;
- recording authority identity;
- recording causal position;
- repository or namespace identity;
- optional authorized classification or visibility references;
- migration provenance when represented under a later schema.

Every Toke field is immutable.

The Toke SHOULD contain references only. It SHOULD NOT duplicate the Tombstone's participants, excess, polarity, evidence, or obligations because those remain authoritative in the Tombstone.

### 7.6 Identity

Toke identity MUST:

- be stable;
- be unique;
- be immutable;
- be independent of insertion order;
- be independent of database sequence position;
- be independent of wall-clock time;
- be independent of randomness;
- not be a hash of mutable fields;
- remain resolvable for repository lifetime;
- differ from Bond and Tombstone identities.

Historical ordering MUST be derived from explicit causal positions or indexes, never from identity lexical order unless the governing schema explicitly makes that order semantic.

### 7.7 Recording

#### Required conditions

Recording requires:

- one formed Tombstone;
- one successful Tombstone validation decision;
- no existing Toke for the Tombstone;
- authorized recording Institution;
- authorized recorder;
- no blocking challenge;
- durable Tombstone storage;
- deterministic Toke payload.

#### Atomicity

Recording MUST atomically:

- establish the Toke;
- establish the unique Tombstone-to-Toke association;
- make lookup possible;
- preserve the Tombstone unchanged.

A partial index entry is not a Toke.

#### Idempotency

Retry after successful recording MUST return or confirm the existing Toke.

Retry MUST NOT create a second Toke, advance a sequence twice, or change causal position.

### 7.8 Historical indexing

Toke indexes MAY support lookup by:

- Toke identity;
- Tombstone identity;
- Bond identity;
- participant identity;
- role;
- House;
- Institution;
- polarity state;
- excess sign;
- Recipe;
- transformation;
- obligation;
- causal position;
- parent or child Bond lineage.

These indexes are derived and rebuildable.

An index:

- MUST NOT become the only copy of the Toke;
- MUST NOT change proof meaning;
- MUST NOT make identity depend on insertion;
- MUST preserve multiple matching Tokes without overwriting;
- MUST use deterministic result ordering when order is presented.

### 7.9 Lookup

Lookup by valid Toke identity MUST produce exactly one of:

- the one matching Toke;
- Not Found;
- Corrupt or Ambiguous, if repository invariants are violated.

Lookup MUST NOT silently choose one of multiple records with the same identity.

Navigation MUST permit:

- Toke to Tombstone;
- Tombstone to source Bond;
- Bond to full replay evidence;
- Bond to parent and child lineages;
- Tombstone to validation decision.

Access control MAY restrict returned detail. It MUST NOT report false absence when the governing policy requires existence disclosure.

### 7.10 Immutability and repository lifetime

Once recorded, a Toke:

- MUST never be edited;
- MUST never be reused;
- MUST never be deleted as routine cleanup;
- MUST never point to another Tombstone;
- MUST never change source Bond;
- MUST never change recording authority;
- MUST never be reordered by changing identity;
- MUST remain addressable for the lifetime of constitutional history.

Physical relocation and lossless migration are permitted. Semantic mutation is forbidden.

### 7.11 Relationship to Tombstones

- Every Toke references exactly one Tombstone.
- Every referenced Tombstone is validated.
- Every Tombstone has at most one Toke.
- Tombstone validation precedes Toke recording.
- Toke never copies or changes Tombstone proof.
- A rejected or pending Tombstone has no Toke.

### 7.12 Relationship to Bonds

- Every Toke indirectly and explicitly references the one Bond that produced its Tombstone.
- A Bond with zero excess has no Toke.
- A Bond that fails eligibility has no Toke.
- A condensing Bond cannot reach final resolution before Toke recording.
- A Toke remains after the Bond resolves.

### 7.13 Relationship to Synthesis

For a condensing Bond, Synthesis Resolution consumes the existence of the recorded Toke as a required historical input.

Synthesis MUST NOT:

- create a replacement index;
- mutate the Toke;
- omit the Toke from a resolution that claims proven history;
- treat a Toke as a Recipe.

For a non-condensing Bond, Synthesis consumes explicit Not Applicable recording status rather than fabricating a Toke.

### 7.14 Relationship to replay

Toke itself is not the replay trace.

Replay navigation uses the Toke to locate:

- validation decision;
- Tombstone;
- source Bond;
- formation;
- event history;
- governing rules;
- evidence.

A Toke is valid only while that navigation remains referentially complete. Missing referenced storage is corruption, not permission to synthesize a new Toke.

### 7.15 Lifecycle and state machine

Toke has no mutable living state.

The recording operation has:

- NotRecorded;
- RecordingPending;
- Recorded;
- RecordingFailed.

Only Recorded yields a Toke.

Legal transitions are:

- NotRecorded to RecordingPending.
- RecordingPending to Recorded.
- RecordingPending to RecordingFailed.
- RecordingFailed to RecordingPending for idempotent retry with the same intended Toke.

Recorded is terminal.

### 7.16 Required invariants

- Exactly one Toke identity.
- Exactly one Tombstone reference.
- Exactly one source Bond reference consistent with the Tombstone.
- Exactly one validation decision reference.
- Tombstone is validated.
- Recorder is authorized.
- Recording is atomic.
- At most one Toke per Tombstone.
- Payload is immutable.
- Lookup is deterministic.
- Indexes are derived.
- Repository lifetime is permanent.

### 7.17 Illegal states

- Toke without identity.
- Toke without Tombstone.
- Toke referencing multiple Tombstones.
- Toke referencing unvalidated Tombstone.
- Toke referencing rejected Tombstone.
- Toke with mismatched Bond.
- Toke without recorder.
- Toke without Institution.
- Toke identity based on insertion sequence.
- Multiple Tokes for one Tombstone.
- Mutable Toke.
- Deleted Toke.
- Index entry treated as Toke without authoritative payload.
- Lookup silently resolving duplicate identity.
- Migration changing target Tombstone.

### 7.18 Failure modes and recovery

#### Validation unavailable

Recording does not begin.

#### Authority unavailable

Recording remains uncommitted.

#### Partial write

No Toke exists unless the full authoritative payload and unique association committed.

#### Duplicate retry

Return the existing Toke if identical. Reject if the same intended identity or Tombstone is associated with conflicting data.

#### Index corruption

Rebuild indexes from authoritative Tokes.

#### Toke payload corruption

Recover from immutable archival copy or fail lookup as corrupt. Do not fabricate replacement history.

#### Missing Tombstone

Report referential corruption and block dependent Synthesis replay.

### 7.19 Persistence and serialization

Toke serialization MUST:

- be versioned;
- be canonical;
- preserve every reference;
- reject missing required fields;
- reject unknown lifecycle meaning;
- avoid identity derived from physical record position;
- distinguish authoritative payload from derived index;
- round trip exactly.

Storage SHOULD support append-only or write-once semantics. The constitutional requirement is immutability, not a particular database.

### 7.20 Migration

Migration MUST:

- preserve Toke identity;
- preserve Tombstone target;
- preserve Bond target;
- preserve validation decision;
- preserve recorder and Institution;
- preserve causal position;
- preserve repository lifetime;
- preserve lookup semantics;
- be deterministic and idempotent.

A migration MUST NOT renumber Tokes merely to match new insertion order.

If an old record lacks a validated Tombstone reference, it is not a canonical Toke and MUST remain migration-incomplete.

### 7.21 Verification

Constitutional tests MUST prove:

- successful recording only after validation;
- failure before validation;
- one-to-one Tombstone association;
- atomic commit;
- idempotent retry;
- duplicate conflict rejection;
- immutable payload;
- lookup by every required primary reference;
- deterministic multi-result ordering;
- index rebuild;
- missing index recovery;
- missing Tombstone corruption;
- navigation to Bond replay;
- survival after Bond resolution;
- canonical serialization;
- lossless migration;
- no Toke for zero-excess or ineligible Bond.

### 7.22 Repository mapping

No Toke type exists in the audited repository.

The closest structural candidate is CompositionRecord plus CompositionCatalog indexing:

- CompositionRecord has stable identity;
- it references sources, result, operation, and optional evidence;
- CompositionCatalog indexes source and result independently of insertion-controlled identity.

CompositionRecord is not automatically a Toke because:

- it may represent any causal composition;
- it does not require a validated Tombstone;
- it is mutable through an in-memory catalog construction boundary;
- it has no permanent recording authority or repository-lifetime contract;
- serialization is deferred;
- one-to-one Tombstone recording is not enforced.

The preferred direction is to project a canonical Toke into the neutral composition/provenance layer or reuse its stable identity and indexing machinery, while retaining a distinct Toke domain type and validator.

ConstitutionalRecognition is also Toke-like in that it acknowledges an already-proven mastery and does not create the achievement. It remains a domain-specific recognition and SHOULD be referenced as evidence rather than renamed Toke.

## 8. Synthesis Resolution

### 8.1 Purpose

Synthesis Resolution exists to answer:

> Now that this Bond's chemistry and proof process are complete, how does the constitutional relationship lawfully conclude and what, if anything, continues?

Synthesis Resolution is the last Bond lifecycle stage.

It integrates completed decisions into one terminal result. It does not redo the preceding stages.

### 8.2 Exclusive authority

Glaüshouse alone performs final Synthesis.

Every Synthesis Resolution MUST identify:

- an authorized Glaüshouse Institution;
- an authorized resolver;
- the resolver's office, role, grant, or other authority source;
- any required cross-House or cross-Institution coordination;
- authority evidence at the resolution causal position.

Another House MAY name, prove, recognize, witness, challenge, or participate. It MUST NOT silently replace Glaüshouse as final resolver.

### 8.3 Responsibilities

Synthesis Resolution SHALL:

- consume exactly one source Bond;
- verify the Bond is at a lawful resolution boundary;
- verify maturity for every activated Bond;
- verify excess calculation;
- verify condensation eligibility outcome;
- require a validated Tombstone and recorded Toke when condensation occurred;
- require explicit Not Formed and Not Applicable outcomes when condensation did not occur;
- verify no blocking challenge or appeal remains;
- verify resolver authority;
- select exactly one lawful resolution disposition;
- account for all remaining Current;
- account for all remaining obligations;
- identify completed obligations;
- identify inherited obligations;
- identify dissolved obligations only when governing law permits dissolution;
- create or reference successor Bond formations where applicable;
- preserve parent and child lineage;
- preserve Tombstone and Toke;
- commit one immutable final resolution;
- make the source Bond terminal.

### 8.4 Non-responsibilities

Synthesis Resolution SHALL NOT:

- form or validate the Bond;
- activate chemistry;
- move historical Current;
- add new Aura to mature history;
- calculate maturity;
- calculate excess;
- form a Tombstone;
- validate a Tombstone;
- fabricate a Toke;
- mutate a Bond formation;
- mutate a mature event;
- mutate a Tombstone;
- mutate a Toke;
- reuse the source Bond identity for a successor;
- duplicate Current;
- act as an ungoverned transformation engine.

### 8.5 Constitutional questions

Synthesis Resolution answers:

- Which Bond is concluding?
- Has its active term completed?
- Did it condense?
- If it condensed, where are the validated Tombstone and Toke?
- If it did not condense, why was proof not formed?
- Which challenges and appeals are closed?
- Which obligations completed?
- Which obligations remain?
- Where does every remaining Current quantity go?
- Does the relationship complete, renew, merge, branch, split, challenge, transfer, dissolve, inherit, or evolve?
- Which successor Bonds are formed?
- Who has authority to resolve?
- Can replay reproduce the same terminal result?

### 8.6 Domain model

Every Synthesis Resolution process MUST contain or reference:

- intended resolution identity;
- schema identity and version;
- source Bond identity;
- source lifecycle boundary;
- resolver identity;
- resolver Institution and authority evidence;
- complete proof-branch status;
- proposed resolution disposition;
- final Current disposition proposal;
- final obligation disposition proposal;
- successor formation proposals, if any;
- parent-child lineage proposal, if any;
- ordered validation findings;
- causal position;
- persistence status.

Before commit, the proposal is mutable only by replacing it with a newly validated proposal derived from recorded inputs. It is not constitutional history.

At commit, the complete resolution record becomes immutable and persistent. The source Bond becomes terminal in the same constitutional transaction.

The resolution identity remains distinct from the source Bond, Tombstone, Toke, Recipe execution, and every successor Bond identity.

Resolution visibility MAY be restricted by governing policy. Existence, terminality, Current disposition, and proof references remain constitutional facts and MUST NOT vary with presentation.

The process is deterministic: identical source history, proof status, authority state, governing rules, and proposed disposition MUST produce the same validation result and committed record.

### 8.7 Inputs

Every Synthesis Resolution MUST consume or reference:

- source Bond identity;
- source Bond formation;
- source Bond lifecycle state;
- maturity record, for an activated Bond;
- mature accounting snapshot;
- mature polarity state or explicit incomplete-evaluation finding;
- net excess result;
- condensation eligibility decision;
- Tombstone identity and validation decision, if formed;
- Toke identity, if recorded;
- explicit Not Applicable proof stages if no Tombstone formed;
- closed challenge decisions;
- closed appeal decisions;
- default findings;
- completed obligations;
- remaining obligations;
- ownership and custody state;
- proposed disposition;
- proposed successor formations, if any;
- resolver authority;
- evidence;
- governing rule versions.

Missing a required input blocks resolution.

### 8.8 Outputs

Every successful Synthesis Resolution MUST produce:

- immutable resolution identity;
- source Bond identity;
- one resolution disposition;
- resolution causal position;
- resolver identity and authority;
- final Current disposition;
- final obligation disposition;
- Tombstone and Toke references or explicit absence reasons;
- successor Bond identities, if any;
- parent-child relationships, if any;
- evidence references;
- schema version;
- terminal source Bond state.

### 8.9 Required conditions

Resolution is permitted only when:

- the source Bond exists;
- the source Bond is not already resolved;
- formation is complete;
- if activated, maturity exists;
- excess calculation exists;
- eligibility decision exists;
- the proof branch is complete;
- every required challenge and appeal is closed;
- every remaining Current quantity has one lawful disposition;
- every remaining obligation has one lawful disposition;
- resolver authority is valid;
- proposed successors have valid formations or are atomically formed as part of the resolution rule;
- replay is successful.

### 8.10 Forbidden conditions

Resolution is forbidden when:

- chemistry is Active;
- term is incomplete;
- mature Current remains unaccounted;
- Aura evaluation required for eligibility is silently omitted;
- a Tombstone formed but is not validated;
- a Tombstone validated but is not recorded;
- a blocking challenge is pending;
- resolver lacks Glaüshouse authority;
- a successor reuses an existing identity;
- a transfer, split, or merge duplicates Current;
- an obligation silently disappears;
- replay diverges;
- the source Bond is already terminal.

### 8.11 Resolution dispositions

#### Complete

Complete ends the relationship with no successor required.

All Current and obligations MUST have final disposition.

#### Renew

Renew creates one successor Bond with a new term and identity.

#### Merge

Merge joins the source with one or more other ready parent Bonds into one new child Bond.

#### Branch or Split

Branch or Split creates two or more independently formed child Bonds.

#### Challenge

Challenge disposition creates a challenge Bond when lasting post-record consequence remains unresolved.

#### Transfer

Transfer creates or references a successor relationship and balanced Current movement.

#### Dissolve

Dissolve ends the relationship without successor while preserving proof and history.

#### Inherit

Inherit transfers explicitly named interests or obligations to one or more successor Bonds.

#### Evolve

Evolve creates a successor Bond under an already-governing transformation or relationship rule. It MUST NOT serve as an open-ended permission to invent mechanics.

### 8.12 New Bond creation

When resolution creates a successor Bond:

- the successor MUST have a new identity;
- formation MUST be complete;
- term MUST be explicit;
- participants and roles MUST be explicit;
- House and Institution MUST be explicit;
- parent references MUST be explicit;
- starting Current MUST have explicit transfer or external Wave sources;
- inherited obligations MUST be explicit;
- validation MUST occur independently;
- activation MUST occur separately after validation.

Synthesis Resolution MAY atomically commit a successor formation. It MUST NOT atomically assume successor validation or activation unless those stages remain separately recorded and all their requirements are independently evaluated.

### 8.13 Termination

Successful Synthesis Resolution terminates the source Bond's constitutional existence as a living relationship.

Termination means:

- no further Current movement;
- no further living Aura observation;
- no second maturity;
- no second excess;
- no second Tombstone;
- no second Toke;
- no second resolution.

Termination does not mean deletion.

### 8.14 Inheritance

Synthesis Resolution is the only final stage that may authorize a resolution disposition of inheritance.

It MUST:

- name each inherited subject;
- name each predecessor and successor;
- preserve Current conservation;
- preserve obligation status;
- preserve ownership history;
- reference governing inheritance authority;
- form successor Bonds.

Inheritance does not copy mature proof. Parent Tombstone and Toke remain parent history.

### 8.15 Relationship to condensation

Condensation converts mature excess into Tombstone material.

Synthesis Resolution integrates the completed proof or explicit absence of proof into a terminal Bond result.

Therefore:

- condensation precedes Tombstone;
- Tombstone precedes validation;
- validation precedes Toke;
- Toke precedes Synthesis for condensing Bonds;
- no-condensation eligibility outcome precedes Synthesis for non-condensing Bonds.

Condensation and Synthesis MUST NOT be implemented as one uninspectable operation.

### 8.16 Relationship to Recipe execution

The repository's existing Synthesis Recipe path:

Recipe to Compiler to Scripts to Aim to Fire to Miss or Kiss to Point²

remains a lawful transformation execution path.

It is not the final Synthesis Resolution state machine.

The two uses of Synthesis are related by constitutional scope:

- Recipe execution performs a bounded operation inside a Bond.
- Synthesis Resolution concludes the Bond after maturity and proof handling.

An implementation MUST use namespacing, types, or module boundaries that prevent one from being mistaken for the other.

### 8.17 Lifecycle and state machine

Legal Synthesis processing states are:

- ResolutionNotReady;
- ResolutionReady;
- ResolutionValidating;
- ResolutionCommitted;
- ResolutionFailed.

Legal transitions are:

- ResolutionNotReady to ResolutionReady when every required input exists.
- ResolutionReady to ResolutionValidating.
- ResolutionValidating to ResolutionCommitted when all conditions pass and atomic persistence succeeds.
- ResolutionValidating to ResolutionFailed when a constitutional condition fails.
- ResolutionFailed to ResolutionValidating only for idempotent retry after external availability is restored or an authorized prior-stage decision changes.

ResolutionCommitted is terminal.

### 8.18 Required invariants

- Exactly one source Bond per resolution record.
- Exactly one disposition.
- Glaüshouse authority is explicit.
- Activated Bond is mature.
- Excess and eligibility exist.
- Proof branch is complete.
- Tombstone and Toke remain unchanged.
- All Current has disposition.
- All obligations have disposition.
- Every successor identity is new.
- Every successor formation is complete.
- Parent-child graph remains acyclic.
- Source becomes terminal exactly once.
- Replay is deterministic.

### 8.19 Illegal states

- Synthesis without Bond.
- Synthesis of an Active Bond.
- Synthesis before maturity.
- Synthesis before excess.
- Synthesis before eligibility.
- Synthesis before Toke for a condensing Bond.
- Synthesis fabricating Toke for a non-condensing Bond.
- Synthesis with blocking challenge.
- Synthesis without Glaüshouse authority.
- Multiple dispositions.
- Multiple final resolutions.
- Unaccounted Current.
- Disappearing obligation.
- Successor with parent identity.
- Successor without formation.
- Parent history copied into child.
- Resolved source becoming Active.
- Recipe execution mislabeled as final resolution.
- Condensation hidden inside final resolution.

### 8.20 Failure modes and recovery

#### Missing proof input

Resolution remains NotReady.

#### Resolver authority failure

Resolution fails without changing the Bond.

#### Successor formation failure

Atomic resolution fails, or a non-successor disposition must be explicitly chosen by the governing process. The implementation must not claim a nonexistent child.

#### Current disposition imbalance

Resolution fails.

#### Obligation conflict

Resolution remains blocked or creates an authorized challenge path. It does not discard the obligation.

#### Persistence interruption

Either no resolution commits or exactly one complete resolution commits.

#### Duplicate retry

Returns the existing identical resolution. Conflicting duplicate data is corruption.

#### Post-resolution dispute

Creates a new challenge Bond. The source resolution remains historical.

### 8.21 Persistence and serialization

Resolution records MUST be:

- immutable;
- versioned;
- canonically serialized;
- permanently linked to the source Bond;
- linked to every successor;
- linked to Tombstone and Toke when applicable;
- linked to explicit non-condensation outcome otherwise;
- replayable;
- independent of physical insertion order.

### 8.22 Migration

Migration MUST preserve:

- resolution identity;
- source Bond;
- disposition;
- resolver authority;
- Current disposition;
- obligation disposition;
- Tombstone and Toke references;
- successor references;
- causal position;
- terminality.

A migration MUST NOT:

- change disposition;
- invent a missing successor;
- turn non-condensing resolution into condensing resolution;
- reactivate the source;
- reinterpret Recipe completion as final Synthesis.

### 8.23 Verification

Constitutional tests MUST prove:

- Glaüshouse exclusivity;
- every lawful disposition;
- every forbidden condition;
- proof and no-proof branches;
- exactly-once resolution;
- complete Current disposition;
- complete obligation disposition;
- atomic successor formation;
- independent successor validation;
- immutable source history;
- immutable Tombstone and Toke;
- post-resolution challenge through a new Bond;
- deterministic replay;
- persistence across restart;
- canonical serialization;
- lossless migration.

### 8.24 Repository mapping

Candidate implementation structures are:

- SynthesisExecution for bounded Recipe execution evidence;
- LandingOutcome for transactional Miss or Kiss behavior;
- DecisionExecution and DecisionTrace for deterministic pre-execution choice and replay;
- OfficialsOutlaws SynthesisRecipe and CompositionRecord for domain composition;
- Glaüshouse institution and Prima Donna authority records;
- CompositionCatalog for parent-result provenance;
- Current Synthesis event and snapshot infrastructure for persistence patterns.

Missing is a final Bond-resolution aggregate and state machine.

The implementation direction is refinement:

- retain the frozen Recipe execution façade;
- introduce an explicitly named Bond Synthesis Resolution boundary above maturity and recording;
- reuse existing Glaüshouse authority data;
- reference existing execution and decision traces as evidence;
- project successor lineage into CompositionCatalog where useful;
- avoid a second Recipe compiler or a second decision engine;
- avoid renaming every existing Synthesis type until module scope can distinguish them safely.

## 9. End-to-End Constitutional Processing

### 9.1 Purpose

End-to-end processing exists to guarantee that independently valid concepts compose into one lawful causal sequence.

Local conformance at one stage does not excuse a missing or reordered lifecycle gate.

### 9.2 Normative stage matrix

| Stage | Required input | Required output | Constitutional question |
|---|---|---|---|
| Bond Formation | Initiating Wave and complete declaration | Formed Bond | Does a declared constitutional relationship now exist? |
| Bond Validation | Formed Bond and authority state | Validated, Pending, or Rejected decision | May this Bond legally exist as living chemistry? |
| Bond Activation | Validated Bond | Active Bond and term start | Has chemistry lawfully begun? |
| Current Circulation | Active Bond and prior Wave per movement | Ordered conserved movements | How did Current move? |
| Current Accumulation | Ordered movements | Deterministic gross and net projections | What Current history has accumulated? |
| Aura Observation | Active Bond and observed history | Ordered signed observations | How was Current and the relationship revealed? |
| Current/Aura Evaluation | Accounting, observations, evidence, authority | Polarity or incomplete evaluation | Is the living history internally coherent? |
| Maturity | Activated Bond and completed term | Immutable mature boundary | Has the active lifetime constitutionally completed? |
| Net Excess Calculation | Mature snapshot and governing rules | Positive, negative, or zero excess | What lasting Current remainder exists? |
| Condensation Eligibility | Mature evaluated Bond and excess | Eligible or Ineligible | May the remainder become durable proof? |
| Tombstone Formation | Eligible non-zero excess | Immutable Tombstone or explicit Not Formed | What durable proof payload was born? |
| Tombstone Validation | Formed Tombstone and independent evidence | Validated, Pending, Rejected, or Not Applicable | Does the payload lawfully prove the mature consequence? |
| Toke Recording | Validated Tombstone and recording authority | Immutable Toke or Not Applicable | Where is the proven Tombstone permanently recorded? |
| Synthesis Resolution | Complete proof branch and resolver authority | One terminal resolution | How does the relationship conclude or continue through successors? |

### 9.3 No-skip law

A stage is complete only when its explicit output exists.

The following are not lawful substitutes:

- data presence is not Formation;
- field validation is not Bond Validation unless every constitutional check is represented;
- process start is not Activation;
- unsigned total change is not Current Circulation;
- a snapshot is not Accumulation unless it replays;
- presentation is not Aura Observation;
- one score is not four-domain Evaluation;
- elapsed wall time is not Maturity;
- a balance is not Net Excess without mature classification;
- non-zero value is not Eligibility;
- a result object is not Tombstone without condensation;
- successful construction is not Tombstone Validation;
- a log append is not Toke Recording;
- Recipe execution is not final Synthesis Resolution.

### 9.4 Alternate terminal paths

#### Pre-activation rejection

Formation to Validation to final Rejection.

No activation, circulation, maturity, Tombstone, Toke, or final Synthesis Resolution of active chemistry occurs.

#### Pre-activation cancellation

Formation to Cancellation.

No active chemistry exists.

#### Mature zero excess

All living stages through maturity and excess execute.

Eligibility is Ineligible because excess is zero.

Tombstone Formation is Not Formed.

Tombstone Validation and Toke Recording are Not Applicable.

Synthesis Resolution resolves without proof.

#### Mature non-zero but ineligible

All stages through eligibility execute.

The failed conditions are recorded.

No Tombstone or Toke exists.

Synthesis Resolution resolves only when governing rules permit resolution with that explicit failure outcome.

#### Mature eligible and proven

All fourteen stages execute in order.

### 9.5 Constitutional transaction boundaries

The following operations MUST be atomic:

- Formation commit;
- Validation decision commit;
- Activation commit;
- each simple Current movement;
- each multi-edge split, merge, or transfer unit;
- each Aura observation;
- each evaluation decision;
- maturity commit;
- excess result commit;
- eligibility decision commit;
- Tombstone formation;
- Tombstone validation decision;
- Toke recording;
- final Synthesis Resolution;
- successor formation set when governing resolution requires all successors together.

Atomic means observers can see the state before or after the operation, never a partially authoritative intermediate state.

### 9.6 Causal ordering

Every authoritative record MUST have a deterministic position in its Bond-local causal sequence.

The ordering mechanism MUST:

- impose a total order on events within one Bond;
- preserve Wave-before-Current;
- preserve stage ordering;
- reject duplicate positions;
- reject conflicting predecessors;
- survive serialization and migration;
- not use storage insertion order as an unstated rule.

Events in different Bonds MAY be partially ordered. A cross-Bond transfer, merge, challenge, or inheritance MUST establish explicit causal edges sufficient to reproduce the relationship.

### 9.7 Concurrency

Concurrent proposals against the same Bond MUST serialize deterministically.

A conforming implementation MUST use an equivalent of:

- expected prior causal position;
- expected lifecycle state;
- unique transition identity;
- atomic compare-and-commit.

Last-write-wins behavior is forbidden for constitutional state.

If two transitions are individually lawful but conflict:

- at most one may commit against the expected prior state;
- the other must be reevaluated against the new state or rejected;
- neither may be silently reordered based on thread timing.

Cross-Bond Current movement MUST not expose duplicate authoritative ownership during concurrency.

### 9.8 Constitutional time

Time used for terms and deadlines MUST be deterministic and governed.

Permitted sources include:

- Bond-local causal position;
- authoritative world tick;
- explicitly versioned simulation step;
- external institutional milestone;
- declared completion event;
- exact calendar time from one authoritative source when the governing Bond explicitly requires it.

Local process uptime, scheduler timing, file modification time, and unsynchronized wall clock MUST NOT determine maturity.

Informational timestamps MAY be stored. They MUST NOT determine identity or override causal order.

### 9.9 Exact arithmetic and units

Every constitutional Current or Aura unit MUST define:

- unit identity;
- scale;
- exact magnitude domain;
- comparison compatibility;
- conversion rule, if conversion is lawful;
- overflow boundary.

Different units MUST NOT be added, offset, split together, or merged without an explicit deterministic conversion rule.

Conversion rounding is forbidden unless the governing rule defines exact remainder treatment. Unaccounted remainder is destroyed Current and therefore illegal.

### 9.10 Authority snapshots

Authority-dependent replay MUST resolve authority as of the historical causal position.

A conforming implementation MUST preserve one of:

- an immutable authority-state reference;
- an institutional event prefix;
- a versioned authority decision;
- another replayable historical authority proof.

Current mutable Institution state alone is insufficient for old events.

### 9.11 Visibility and access

Visibility controls disclosure, not existence.

Access policy MAY:

- conceal payload detail;
- restrict evidence;
- restrict witness identity;
- restrict institutional context;
- return a redacted projection.

Access policy MUST NOT:

- delete proof;
- change Current;
- change Aura;
- report a false lifecycle state;
- let a redacted view become the only authoritative record;
- make replay depend on the viewer.

### 9.12 Global recovery law

Recovery proceeds in this order:

1. stop new dependent transitions;
2. locate the last intact authoritative record;
3. validate record identity and causal predecessor;
4. replay forward from intact history;
5. compare derived projections and immutable artifacts;
6. rebuild caches and indexes;
7. resume only if constitutional equality is established;
8. otherwise preserve the failure and block dependent proof or resolution.

Recovery MUST NOT:

- guess missing Current;
- guess missing Aura;
- fabricate a Wave;
- infer a term;
- replace a Tombstone;
- renumber a Toke;
- remove a contradiction;
- convert corruption into successful validation.

### 9.13 Global rollback law

Rollback is permitted only for uncommitted working state.

Committed history is corrected by:

- a later lawful event;
- challenge;
- appeal;
- successor Bond;
- migration that preserves meaning.

Deletion or in-place rewrite is not rollback.

### 9.14 Global retry law

Every retryable constitutional operation MUST be idempotent.

An identical retry after commit returns the existing result.

A conflicting retry using the same identity:

- MUST be rejected;
- MUST identify the conflict;
- MUST NOT overwrite the existing result.

### 9.15 Global deterministic replay test

Given:

- the same formed Bond;
- the same rule versions;
- the same ordered event records;
- the same evidence;
- the same historical authority;

two conforming implementations MUST derive constitutionally equivalent:

- lifecycle states;
- Current totals;
- Aura totals;
- polarity;
- maturity;
- excess;
- eligibility;
- Tombstone;
- Toke association;
- Synthesis Resolution.

If canonical serialization is shared, outputs MUST be byte-equivalent.

## 10. Global Illegal-State and Failure Requirements

### 10.1 Purpose

Global illegal-state rules prevent one concept's permissiveness from corrupting another concept's invariant.

### 10.2 Referential integrity

Every required identity reference MUST resolve to exactly one record of the expected kind.

Illegal configurations include:

- reference to missing record;
- reference to wrong record kind;
- duplicate identity;
- ambiguous identity;
- self-reference where prohibited;
- causal or lineage cycle;
- reference to a future event presented as prior authority.

### 10.3 Partial-state prohibition

The following partial states are illegal:

- Current movement without accumulated projection after committed rebuild boundary;
- mature state with open write access to living ledger;
- Tombstone payload without complete formation;
- validation decision without target payload;
- Toke index without Toke payload;
- resolution claiming successor not formed;
- snapshot ahead of event log without authoritative evidence;
- event log behind immutable artifact without a recorded checkpoint relationship.

### 10.4 Contradiction handling

Contradiction MUST be:

- detected;
- scoped;
- preserved;
- reported;
- evaluated by authority;
- prevented from satisfying proof gates while unresolved.

Contradiction MUST NOT be:

- resolved by collection order;
- resolved by newest wall-clock timestamp alone;
- deleted;
- collapsed into an average;
- hidden by successful unrelated evidence.

### 10.5 Participant loss

Loss, death, deletion from an external directory, disaffiliation, expulsion, or unavailability of a participant:

- does not delete historical identity;
- does not remove obligations;
- does not transfer Current automatically;
- does not erase witness statements;
- may trigger default, transfer, inheritance, challenge, early maturity, or dissolution only under governing rules.

### 10.6 Institutional disappearance or reorganization

An Institution that later dissolves remains the historical authority for acts it lawfully performed.

New authority over an Active Bond requires:

- explicit succession;
- a successor Bond when formation-level governance changes;
- historical authority preservation;
- no retroactive substitution.

### 10.7 Rule-version loss

If the exact governing rule version cannot be resolved:

- replay fails;
- validation dependent on the rule fails or remains pending;
- condensation is forbidden;
- Toke recording is forbidden;
- existing immutable Tombstone and Toke are preserved but reported with the replay availability failure.

### 10.8 Repository divergence

If two repositories contain conflicting records with the same constitutional identity:

- neither may overwrite the other automatically;
- the conflict is corruption or a constitutional dispute;
- causal evidence and authority must resolve it;
- identity reuse MUST NOT be normalized as a merge.

## 11. Constitutional Verification Standard

### 11.1 Purpose

Constitutional verification proves domain law independently of a particular database, programming language, UI, or function layout.

### 11.2 Test classes

Every conforming implementation MUST provide:

- positive conformance tests;
- negative conformance tests;
- state-transition tests;
- invariant tests;
- deterministic replay tests;
- persistence and restart tests;
- serialization tests;
- migration tests;
- corruption tests;
- concurrency tests;
- authority and jurisdiction tests;
- evidence and witness tests;
- end-to-end lifecycle tests.

### 11.3 State-machine coverage

Tests MUST:

- visit every legal state;
- execute every legal transition;
- attempt every unlisted transition;
- prove every terminal state is terminal;
- exercise every recovery transition;
- prove challenge and appeal do not rewrite history;
- prove no lifecycle gate is skipped.

### 11.4 Signed-domain matrix

Tests MUST include:

| Current | Aura | Required result |
|---|---|---|
| Positive | Positive | Positive Current / Positive Aura |
| Positive | Negative | Positive Current / Negative Aura |
| Negative | Positive | Negative Current / Positive Aura |
| Negative | Negative | Negative Current / Negative Aura |
| Zero | Positive | Incomplete Current sign; no polarity |
| Positive | Zero | Incomplete Aura sign; no polarity |
| Zero | Zero | Incomplete evaluation; no polarity |

Each non-zero state MUST be tested with both Light and Dark Aura orientations to prove orientation is not constitutional sign.

Each Current sign MUST be tested with Regular and Hollow Current states to prove state is not sign.

### 11.5 Accounting properties

Generated lawful movement sequences MUST prove:

- conservation per sign and unit;
- boundary balance;
- split and merge equality;
- no duplication under retry;
- internal throughput does not inflate inputs;
- gross totals survive netting;
- exact zero excess can arise from non-zero history;
- overflow and underflow reject.

### 11.6 Evidence and authority properties

For every authority-dependent stage, tests MUST vary:

- valid authority;
- missing authority;
- revoked authority;
- out-of-jurisdiction authority;
- historically valid but currently revoked authority;
- conflicting institutional authorities;
- singular-office conflict.

For every evidence-dependent stage, tests MUST vary:

- complete evidence;
- missing evidence;
- corrupt evidence;
- contradictory evidence;
- wrong-version evidence;
- wrong-owner namespace;
- unavailable evidence;
- duplicated evidence.

### 11.7 Persistence properties

The process MUST be stopped and restarted:

- before each stage;
- during each atomic operation;
- after each stage;
- after snapshot creation;
- after index deletion;
- after a torn final write;
- after a duplicate retry.

The resulting authoritative state MUST be either the exact prior state or the exact committed next state.

### 11.8 Migration properties

Every supported source version MUST be tested against every legal next migration step.

Tests MUST prove:

- identity preservation;
- Current preservation;
- Aura preservation;
- order preservation;
- proof preservation;
- terminality preservation;
- idempotency;
- failure on unrepresentable data;
- source preservation on failure;
- replay equivalence.

### 11.9 Adversarial requirements

Verification MUST attempt:

- duplicate identities;
- reordered maps;
- reordered events;
- missing events;
- future references;
- cycles;
- extreme exact magnitudes;
- invalid signs;
- zero atomic quantities;
- stale authority;
- forged witness;
- forged Tombstone;
- Toke without validation;
- multiple Tokes;
- post-maturity movement;
- resolved reactivation;
- schema downgrade;
- unknown variants;
- partial successor formation;
- cross-Bond double ownership.

### 11.10 Conformance report

A constitutional conformance report MUST identify:

- specification version;
- implementation version;
- schema versions tested;
- rule versions tested;
- tests executed;
- invariant coverage;
- state transition coverage;
- failures;
- known unsupported constitutional paths;
- whether unsupported paths are rejected or silently accepted.

An unsupported path is conforming only if it is rejected before illegal state. Silent acceptance is non-conforming.

## 12. Repository Audit and Architectural Mapping

### 12.1 Audit scope

This mapping reflects a read-only audit of the repository at:

/home/warren/hollow-grove

The worktree contained substantial pre-existing modified and untracked work. No implementation file was changed as part of this specification expansion.

The audit inspected:

- Hollow Grove core and semantic foundation documents;
- Current Synthesis V1 and Version 2 decision documents;
- Aura polarity and Current inheritance documents;
- root kernel and routing types;
- Current Synthesis engine state, events, snapshots, replay, candidates, resources, and inspectors;
- Recipe compilation and transactional execution;
- DecisionTrace and replay verification;
- institution and affiliation models;
- House fixtures;
- composition and provenance kernel;
- the canonical Flynt constitution, unique Chimera synthesis, institutional projection, and hierarchy audit;
- existing constitutional and integration tests.

### 12.2 Existing constitutional strengths

The repository already establishes several useful constitutional patterns:

- deterministic kernel pass structure;
- immutable value-oriented traces;
- transactional Recipe landing;
- explicit Miss and Kiss behavior;
- stable caller-controlled composition identifiers;
- separation of causal composition from containment;
- opaque external evidence references;
- deterministic decision candidate ordering and typed reason codes;
- decision replay verification;
- event-log reconstruction and derived snapshots;
- rejection of unknown event kinds;
- torn-final-line handling;
- institutional stable identities;
- explicit roles, memberships, offices, grants, visibility, and relationships;
- obligation and default vocabulary;
- House identity and fixed House positions;
- evidence-backed recognition downstream from achieved mastery;
- separation of persistent Being identity from transformation Frame.

These structures SHOULD be refined and composed. They SHOULD NOT be replaced without a demonstrated constitutional gap.

### 12.3 Type-by-type mapping

| Repository type or module | Existing meaning | Constitutional mapping | Required direction |
|---|---|---|---|
| hollow_grove::Bond | Selected linked Way at kernel depth | Narrow kernel relation, not canonical Bond | Retain behavior; clarify name or module scope before canonical Bond is public |
| PlebMetaBond | Pair of PLEB and META modes | Routing relation | Retain; never use as Bond lifecycle |
| Aim.bond | Kernel contact relation used by Fire | Recipe execution evidence | Retain behind execution façade |
| BondCandidate | Candidate action with string participants and unsigned properties | Pre-formation proposal only | Rename or explicitly mark candidate; do not promote as formed Bond |
| BondResult | Selected candidate and resulting moment | Decision result evidence | Reference from a Wave or formation if constitutionally adopted |
| MoxyRelation::Bond | World-map relation label | Presentation/domain relation | Keep separate |
| InstitutionalRelationship | Stable institutional relation | Formation and authority evidence | Reference, do not merge into Bond |
| CurrentPrism | Unsigned attributes | Current-related state, not Current ledger | Keep separate from signed Current |
| PrismDelta | Signed attribute delta | Recipe execution change evidence | Do not treat as conserved Current without explicit projection |
| ResourceComposition | Unsigned Aura and Current property totals | Simulation resource projection | Cannot serve as constitutional signed accounting unchanged |
| Residue | Properties from unused candidates | Simulation residue | Not mature excess or Tombstone |
| SemanticSide | Left/Aura and Right/Current | Family/orientation | Not sign |
| AuraPolarity | Light and Dark Glow orientation | Aura observation evidence | Not Positive and Negative Aura |
| InferredAuraOrientation | Neutral, Light, Dark, Mixed evaluation | Evidence about truth and agency | Must be projected into signed Aura under Bond rules |
| FrameState | Being-associated Frame, prism, Flow, Glow | Transformation state evidence | Reference from Recipe and transformation participation |
| SynthesisRecipe in root crate | Ordered transformation intents | Bounded Recipe participation | Retain and reference |
| SynthesisExecution | Transactional execution result | Strong immutable evidence candidate | Reference, do not copy |
| SynthesisRecipe in `flynt-constitution` | Gargoyle, Merman, and Werewolf sources with the unique Chimera result | Domain composition rule | Retain domain ownership and the locked Flynt namespace |
| DecisionTrace | Deterministic observe-to-execute evidence | Evidence and replay pattern | Reuse trace/reference approach |
| KernelPass | Deterministic completed recursion witness | Wave or evidence source candidate | Not Bond lifecycle |
| CurrentSynthesisEvent | Scenario, focus, action, feedback, and tick events | Event-sourcing pattern | Generalize infrastructure only; do not copy domain event vocabulary |
| PersistedCurrentSynthesisState | Derived scenario/tick/focus summary | Compatibility projection | Keep until canonical event store supersedes consumer need |
| Current Synthesis snapshot | Derived full state checkpoint | Snapshot pattern | Add schema and constitutional validation before Bond use |
| InstitutionCatalog | Institutions, offices, roles, sites, relationships | Institutional authority source | Reuse |
| InstitutionalWorldState | Memberships, sponsorships, obligations, claims, events, grants | Authority, obligation, and evidence source | Preserve historical state for replay |
| InstitutionalObligation | Debtor, creditor, kind, status, weight | Bond obligation reference | Extend through reference, not duplicate |
| House | Four canonical Houses | Governing House identity | Reuse directly |
| ExternalRef | Opaque namespace and key | Evidence reference | Leading candidate |
| CompositionNode | Addressable domain projection | Tombstone projection candidate | Neutral index only |
| CompositionRecord | Sources, result, operation, evidence | Toke or lineage projection candidate | Not Toke without canonical constraints |
| CompositionCatalog | Neutral validated in-memory indexes | Cross-domain lookup and lineage support | Reuse; add persistence only under separate versioned adoption |
| FlyntConstitutionalAudit | Deterministic institution, office, superior, and uniqueness proof | Authority evidence candidate | Keep domain-specific and reference immutably |

### 12.4 Existing naming collisions

#### Bond

Bond currently names at least:

- a selected kernel Way;
- a PLEB/META routing relation;
- an Aim contact relation;
- a candidate action;
- a selected decision result;
- a world-map relationship label.

None implements the canonical Bond lifecycle.

The implementation SHOULD introduce one unambiguous canonical aggregate name at the domain boundary and narrow existing names through module qualification or renaming. It SHOULD NOT merge these different concepts into one large enum.

#### Synthesis

Synthesis currently names:

- Recipe compilation and transactional execution;
- a Current Synthesis decision/runtime layer;
- domain form composition;
- Glaüshouse constitutional integration.

The specification preserves them by scope:

- Recipe Synthesis executes bounded change.
- Current Synthesis chooses and coordinates existing execution behavior.
- domain synthesis composes domain forms.
- Bond Synthesis Resolution concludes constitutional chemistry.

Clear namespace boundaries are preferable to one universal Synthesis type.

#### Current and Aura

Current and Aura currently name attributes, resources, orientations, learnsets, capacity, routes, states, and world concepts.

Signed constitutional Current and Aura MUST be explicit new projections. Existing concepts MUST NOT be reinterpreted by name alone.

### 12.5 Duplicate abstractions and consolidation opportunities

#### Stable identity grammar

Stable key validation is implemented repeatedly:

- composition stable keys;
- institutional stable IDs;
- affiliation IDs;
- Flynt constitutional IDs.

A shared lexical validator is a valid consolidation.

Distinct newtypes MUST remain because BondId, TombstoneId, TokeId, InstitutionId, CompositionNodeId, and CompositionRecordId are not interchangeable merely because their text grammar matches.

#### Evidence references

ExternalRef already represents an opaque namespace and key.

Future Bond, Tombstone, and Toke evidence SHOULD use or refine this shared boundary instead of adding separate string pairs for every evidence class.

Detailed evidence payloads remain in their owning domains.

#### Replay infrastructure

DecisionTrace replay and Current Synthesis event reconstruction overlap in infrastructure needs:

- ordered immutable input;
- deterministic derivation;
- divergence detection;
- snapshots;
- restart.

Shared persistence and replay utilities MAY be consolidated. Domain-specific reason codes, events, and state machines MUST remain separate.

#### Authority evaluation

InstitutionCatalog and InstitutionalWorldState already own authority facts.

Bond validation SHOULD call that authority rather than introduce Bond-local copies of office and membership logic.

### 12.6 Missing abstractions by constitutional stage

| Stage | Repository status |
|---|---|
| Formation | Missing complete aggregate |
| Validation | Existing validators are fragmented; orchestration missing |
| Activation | Bond-specific activation missing |
| Current Circulation | Signed conserved ledger missing |
| Current Accumulation | Signed gross/net constitutional projection missing |
| Aura Observation | Bond-scoped signed observation ledger missing |
| Evaluation | Four signed polarity evaluator missing |
| Maturity | Term and maturity missing |
| Excess | Mature offset result missing |
| Eligibility | Complete gate missing |
| Tombstone Formation | Missing |
| Tombstone Validation | Missing |
| Toke Recording | Missing |
| Synthesis Resolution | Final Bond resolution missing |

### 12.7 Existing non-conformities if promoted directly

The following existing behaviors are lawful in their current narrower domains but non-conforming if promoted directly as constitutional Bond behavior:

- Bond identity represented only by Way.
- Candidate Bond identity derived from owner, slug, and tick without a formed identity contract.
- unsigned Current and Aura totals.
- saturating property addition.
- unused candidate residue treated as accumulated resource.
- candidate selection influenced by a seeded stable tiebreak.
- no term.
- no maturity.
- no signed Aura.
- no exact conservation ledger.
- no one-to-one Tombstone and Toke constraints.
- no schema version in Current Synthesis event and snapshot artifacts.
- replay based on current scenario code without an immutable governing rule version.
- mutable institutional state without complete historical authority snapshotting.

These observations do not condemn the existing systems. They establish why adapters and explicit constitutional types are required.

### 12.8 Dead abstractions

No audited abstraction is proven dead.

Specifically:

- root Bond is used by kernel, Aim, Fire, output, and tests;
- PlebMetaBond is used by routing;
- BondCandidate and BondResult are used by Current Synthesis;
- ResourceComposition and Residue are used by simulation and inspectors;
- both SynthesisRecipe types have distinct active domain uses;
- CompositionCatalog is frozen and used by `flynt-constitution` for the unique Chimera provenance record;
- PersistedCurrentSynthesisState remains a compatibility and bootstrap surface.

Premature deletion would violate the repository rule.

### 12.9 Possible future deletions after adoption

Deletion MAY become appropriate only after verified migration:

- duplicate stable-key validation functions after shared validation is adopted;
- transitional canonical-Bond adapters after all consumers use one boundary;
- legacy unversioned parsers after every persisted record is migrated and archived;
- redundant derived summary persistence if the event log and snapshot contract fully supersede it;
- ambiguous public re-exports of narrow Bond names after callers use explicit module paths.

Each deletion requires consumer search, migration evidence, and regression verification.

### 12.10 Possible merges

Lawful merges include:

- shared identifier lexical validation;
- shared canonical serialization primitives;
- shared atomic append and checkpoint infrastructure;
- shared ExternalRef evidence boundary;
- shared replay comparison utilities;
- shared institutional authority query boundary.

Forbidden merges include:

- Current sign with Regular/Hollow state;
- Aura sign with Light/Dark orientation;
- Aura sign with Reflective/Holographic state;
- canonical Bond with kernel Way selection;
- Tombstone with CompositionNode;
- Toke with arbitrary CompositionRecord;
- final Synthesis Resolution with Recipe execution;
- ownership with custody;
- House with Institution;
- evidence with witness;
- causal composition with containment.

### 12.11 Refinement sequence

The following sequence minimizes replacement and duplication:

1. Establish shared stable reference and canonical serialization contracts.
2. Introduce canonical Bond, Tombstone, Toke, and resolution identities without runtime adoption.
3. Introduce formed declaration, term, lifecycle state, and append-only event model.
4. Introduce signed Current and Aura projections with conservation and four-state evaluation.
5. Reference existing authority, Recipe, transformation, decision, and evidence domains.
6. Add maturity, excess, eligibility, Tombstone, validation, Toke, and resolution.
7. Project proof and lineage into CompositionCatalog.
8. Add versioned persistence and migrations.
9. Adapt exactly one bounded runtime path.
10. Remove transitional duplication only after conformance and consumer verification.

This is architectural direction, not authorization to change code in this document-only task.

### 12.12 Repository verification mapping

Existing tests that provide reusable verification patterns include:

- kernel purity and boundary tests;
- kernel routing freeze tests;
- Recipe compilation and order tests;
- transactional landing fault-cut tests;
- DecisionTrace replay tests;
- Current Synthesis event, snapshot, and restart tests;
- composition identity, lookup, missing reference, and cycle tests;
- institutional singular-office and membership validation tests;
- Flynt hierarchy, unique Chimera, institutional placement, and kernel-boundary tests;
- Aura polarity mismatch, consent, agency, and route tests.

New constitutional tests SHOULD live at a boundary that can exercise domain types without forcing the neutral kernel to import world meaning.

### 12.13 Dependency direction

The preferred dependency direction is:

1. neutral stable identity, reference, serialization, and persistence primitives;
2. existing Hollow Grove kernel;
3. existing domain authorities for House, Institution, Recipe, transformation, and evidence;
4. canonical Bond lifecycle orchestration;
5. Tombstone and Toke history;
6. Synthesis Resolution;
7. presentation, Current Synthesis, Hueman, and external clients.

The neutral composition kernel MUST NOT import Bond world semantics merely to index them.

The routing kernel MUST NOT be expanded into a Bond database.

Presentation MUST remain downstream and read-only with respect to immutable history.

## 13. Conformance Checklist

### 13.1 Formation and identity

- [ ] Bond identity is stable, unique, immutable, and deterministic.
- [ ] Formation is exactly once and atomic.
- [ ] Initiating Wave exists.
- [ ] Participants and roles are explicit.
- [ ] House and Institution are explicit.
- [ ] Term is finite or explicitly perpetual.
- [ ] Ownership and custody are explicit.
- [ ] Starting Current is sourced.

### 13.2 Living chemistry

- [ ] Activation occurs after validation and exactly once.
- [ ] Every Current movement has a prior Wave.
- [ ] Positive and Negative Current are separate.
- [ ] Conservation holds for every operation.
- [ ] Positive and Negative Aura are separate.
- [ ] Aura observations have observers, scope, and evidence.
- [ ] Aura never changes Current.
- [ ] Accumulation replays exactly.
- [ ] All four polarity states are supported.

### 13.3 Maturity and proof

- [ ] Term completion is deterministic.
- [ ] Maturity is exactly once and success-independent.
- [ ] Mature history is immutable.
- [ ] Excess uses exact gross inputs and offset.
- [ ] Zero excess creates no Tombstone.
- [ ] Eligibility checks every required condition.
- [ ] Condensation is exact and at most once.
- [ ] Tombstone payload is immutable.
- [ ] Tombstone validation is independent.
- [ ] Only validated Tombstone is recorded.
- [ ] Toke is unique, immutable, and permanent.

### 13.4 Resolution and succession

- [ ] Glaüshouse authority performs final Synthesis.
- [ ] Proof or explicit no-proof branch is complete.
- [ ] Blocking challenges and appeals are closed.
- [ ] All Current has disposition.
- [ ] All obligations have disposition.
- [ ] Successor identities are new.
- [ ] Successor formations are complete.
- [ ] Parent-child graph is acyclic.
- [ ] Resolution is exactly once and terminal.

### 13.5 Determinism and storage

- [ ] No identity uses randomness, wall clock, insertion order, memory address, or mutable hashing.
- [ ] No constitutional arithmetic uses floating point or saturation.
- [ ] Event order is explicit.
- [ ] Authority is historical.
- [ ] Serialization is versioned and canonical.
- [ ] Retry is idempotent.
- [ ] Rollback never removes committed history.
- [ ] Restart reproduces state.
- [ ] Indexes rebuild.
- [ ] Migration is lossless and idempotent.
- [ ] Replay reproduces Tombstone, Toke association, and resolution.

### 13.6 Canonical repository integration profile

This subsection fixes the repository boundary used by the V2 constitutional runtime. It does not weaken any preceding requirement.

The recursion kernel remains in `src/hollow_grove.rs` and `src/kernel_pass.rs`.

The recursion-depth ordinal selector is named `KernelBond`. The legacy public name `Bond` is a compatibility alias for `KernelBond` only. It MUST NOT be used as the constitutional Bond aggregate.

The constitutional runtime resides in `src/constitutional/` and is divided as follows:

- `ids.rs` owns caller-controlled stable constitutional identifiers;
- `model.rs` owns causal positions, exact signed quantities, terms, Waves, accounts, and signed totals;
- `houses.rs` owns House functions, historical authority snapshots, institutional jurisdiction snapshots, House decisions, and Reserved-procedure rejection;
- `bond.rs` owns Bond declarations, lifecycle events, the replay reducer, Current accounting, Aura evaluation, challenge, default, maturity, condensation, Tombstone, Toke, and resolution records;
- `runtime.rs` owns the append-only event boundary, Wave registry, replay cache, immutable identity indexes, idempotent retry, parent-successor checks, and pre-commit validation;
- `persistence.rs` owns canonical archive schema version 1, encoding, decoding, migration dispatch, Bond replay digests, and whole-runtime replay digests;
- `adapters.rs` owns one-way evidence adapters from kernel passes, Recipe executions, Aura evaluations, decision traces, and neutral composition projections.

The dependency direction is:

```text
recursion kernel / Recipe / Aura / decision / institution evidence
                              ↓
                   constitutional adapters
                              ↓
                  constitutional runtime
                              ↓
              Tombstone and Toke projections
                              ↓
        neutral composition indexes and presentation
```

No arrow in this diagram is reversible.

Recording a completed `KernelPass` creates a Wave record only. It MUST NOT form a Bond, activate a Bond, move Current, infer signed Current, infer signed Aura, or fabricate a House decision.

The canonical institutional catalog remains the authority for active office holders and office capabilities. Constitutional House decisions are lawful only when projected from, or independently equivalent to, a historical institutional snapshot that establishes:

- Stonebend `ConstitutionalIdentity` for Name;
- Sandmanor `WitnessedImprovement` for Prove;
- Glaüshouse `PublicClearance` for Clear;
- Flynt `InstitutionalRecognition` for Recognize;
- Glaüshouse `FinalJudgmentAnswerability` for Resolve.

The absence of a ratified holder or procedure MUST cause rejection. Test fixtures MAY provide anonymous holders for conformance tests; fixture holders MUST NOT silently establish canonical world succession.

The V2 archive is an append-only binary replay artifact with explicit magic, schema version, length-delimited values, enum tags, exact integer quantities, caller-controlled identities, and no wall-clock or random fields. Decoding MUST rebuild every aggregate and index through the same reducer used for live commands. Unknown versions, invalid tags, corrupt identifiers, truncated values, trailing bytes, illegal transitions, and replay-digest mismatches MUST fail closed.

Tombstone validation MUST cite the digest of exactly one Bond's pre-validation replay prefix and every Wave directly cited by that Bond. The validator identity MUST be distinct from every Bond participant identity. The validation event is appended only after that digest and independence check succeeds.

Neutral `CompositionCatalog` records are downstream indexes. The canonical projection is:

```text
constitutional Bond node
        ↓ condensation-v1
constitutional Tombstone node
        ↓ toke-recording-v1
constitutional Toke node
```

Deletion or mutation of that projection never authorizes deletion or mutation of constitutional history. The constitutional archive and successful deterministic replay remain authoritative.

The conformance suite resides in `tests/constitutional_runtime.rs`. It MUST continue to cover stage ordering, the two cross-polarity states, the full four-House proof path, kernel-to-Wave separation, institution-derived authority, finite-term freezing, default persistence, challenge closure, Reserved appeal rejection, retry idempotence, successor inheritance, archive round-trip, migration idempotence, and replay-digest validation.

## 14. Core Constitutional Statement

Every lawful transfer of Current passes through a Bond.

Every Bond has a term.

Every Current movement has a Wave.

Every Current history preserves Positive and Negative Current.

Every Aura history preserves Positive and Negative Aura.

Positive Current may Bond to Negative Aura.

Negative Current may Bond to Positive Aura.

Every successful evaluation occupies exactly one of the four constitutional polarity states.

Aura reveals Current and never rewrites it.

Maturity completes the active term and does not imply success.

Only non-zero eligible mature excess may condense.

Condensation forms at most one Tombstone.

Only independent validation proves the Tombstone.

Only a validated Tombstone may be recorded as one Toke.

Only after the proof branch is complete may Glaüshouse perform Synthesis Resolution.

The Bond may end.

The Tombstone remains.

The Toke remains.

History remains.

Future Bonds build upon proven history without replacing it.

## 15. Ratified Regional Synthesis Extension

This section is normative. It ratifies two and only two regional Synthesis rules
for the current constitutional version.

### 15.1 Purpose

Regional Synthesis exists to answer:

> Which established regional Being may lawfully become which evolved form, by
> whose authority, upon what evidence, with what preserved lineage, and with
> what durable regional responsibility?

Regional Synthesis is required because form adjacency alone does not establish
authority, evidence, region, identity, or function. Occupation alone does not
establish transformation. Presentation alone establishes nothing.

### 15.2 Ratified Rules

The complete current rule set is:

```text
Gnome → Minotaur
Required standing: Aura Field
Function: field stewardship, work, maintenance, and defense
```

```text
Elf → Centaur
Required standing: Aura Beach
Function: beach patrol and Aura Sea guardianship
```

Gnome→Centaur and Elf→Minotaur are not alternate readings. They are illegal
cross-lineage transitions.

The common regional reducer remains frozen to those two first-stage rules.
`SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md` subsequently ratifies
Minotaur→Hecaton and Centaur→Pegasus through the Sandmanor guardian event model
in `src/world/sandmanor/milestone.rs`, reusing the lineage validator and
Glaüshouse maintained-Synthesis lifecycle without changing this reducer. Their
presence in the lineage table alone still grants nothing.

### 15.3 Responsibilities

The regional Synthesis aggregate SHALL:

- accept caller-controlled stable Being, event, and Synthesis identities;
- register only the Gnome and Elf origin forms;
- preserve the existing `SandmanorLineage` distinction;
- require a typed regional standing;
- verify that standing against the Sandmanor-controlled site;
- require established rather than visitor standing;
- require a Sandmanor Proof decision;
- require a Glaüshouse Resolution decision;
- require the exact institutions authorized by current law;
- require evidence bound to the predecessor Being;
- validate standing, lineage, readiness, constitutional rule, supporting facts,
  and Synthesis evidence;
- validate adjacent lineage through the existing Sandmanor validator;
- create a distinct result Being identity;
- retain the predecessor identity and status;
- record the explicit predecessor on the result;
- retain the complete lineage history;
- derive the result form from the ratified rule;
- derive the regional assignment from the ratified rule;
- commit one immutable event for one successful Synthesis;
- support exact idempotent retry;
- reject conflicting identities;
- persist the constitutional inputs;
- reconstruct the result through the same reducer during decode and replay;
- expose read-only lineage, stewardship, occupation, and guardianship queries;
- return stable typed failure codes without mutation.

### 15.4 Non-responsibilities

The regional aggregate SHALL NOT:

- alter either recursion kernel;
- form, activate, mature, or resolve a Bond automatically;
- execute a bounded Recipe automatically;
- infer transformation from presence in a region;
- infer evidence from presentation or narrative text;
- infer House authority from a House name;
- assign a form that the command did not lawfully request;
- accept Gnome→Centaur or Elf→Minotaur;
- grant Minotaur coastal authority;
- grant Centaur field authority;
- treat Aura Sea as a primary Synthesis site;
- erase or replace the predecessor Being;
- reuse the predecessor identity as the result identity;
- accept an evolved form as an unlined origin;
- mutate a rejected command into a corrected command;
- store a rejected attempt as accepted constitutional history;
- trust a persisted assignment instead of re-running the reducer;
- permit trace, CLI, or TUI code to select law;
- simulate farming, load physics, navigation, combat, escort, sensing, or Current
  stabilization merely by assigning constitutional duties.

### 15.5 Domain Model

Every origin and result is a `RegionalBeingRecord` with:

- one `RegionalBeingId`;
- one `SandmanorForm`;
- one `SandmanorLineage`;
- zero or one predecessor identity;
- one nonempty ordered lineage history;
- one regional standing;
- zero or one rule-derived regional assignment;
- one status;
- evidence.

The origin status begins `Active`. Successful Synthesis changes the predecessor
status to `SynthesizedInto(result)` and creates one active result. A separately
authorized regional Tombstone changes an active Being to `Tombstoned`.

The result does not mutate into the predecessor. Both identities remain
addressable for the lifetime of retained constitutional history.

### 15.6 Regional Standing

`AuraFields` maps to `site.sandmanor.aura-fields`.

`AuraBeach` maps to `site.sandmanor.aura-beach`.

Both sites MUST be controlled by `institution.sandmanor.sandmen` and MUST remain
Sandmanor sites. A jurisdiction snapshot records the region, site, institution,
House, causal observation position, and evidence.

`AuraSea` has no primary standing site in this rule set. It is a guardianship
target reached only through lawful Centaur Synthesis on the Aura Beach.

Standing kind MUST be `Established` for Synthesis. `Visitor` is insufficient.
A future regional transfer requires a constitutional amendment and MUST NOT be
inferred by replacing the standing field.

### 15.7 Authority

Every regional Synthesis command MUST contain:

1. an accepted Sandmanor `HouseFunction::Prove` decision carrying
   `WitnessedImprovement`, issued by the Sandman office in
   `institution.sandmanor.sandmen`;
2. an accepted Glaüshouse `HouseFunction::Resolve` decision carrying
   `FinalJudgmentAnswerability`, issued by the Prima Donna office in
   `institution.glaushouse.medical-civilization`.

Both decisions MUST be causally available no later than the Synthesis event.
Their identities MUST NOT conflict with decisions already consumed by the same
regional runtime.

Sandmanor proves the configuration and lineage prerequisites. Glaüshouse
authorizes the integrated result. Neither decision alone is sufficient.

### 15.8 Evidence

`SubjectEvidence` contains the stable predecessor Being identity and one stable
`EvidenceRef`.

The command MUST supply separate evidence for:

- established regional standing;
- predecessor lineage;
- Synthesis readiness;
- the applicable constitutional rule;
- any supporting prerequisites;
- the Synthesis result request.

Every subject MUST equal the predecessor. Evidence for another Being is not
transferable merely because it describes the same form or region.

### 15.9 Gnome-to-Minotaur Lifecycle

The legal lifecycle is:

```text
absent Gnome identity
    ↓ register with established Aura Field standing
active Gnome
    ↓ prove lineage, readiness, standing, and rule
    ↓ authorize with Glaüshouse Resolution
    ↓ apply Gnome→Minotaur rule
Gnome status = SynthesizedInto(Minotaur identity)
active Minotaur with predecessor and lineage
    ↓ atomically derive Aura Field stewardship
```

The Minotaur is the bull-associated field-working, burden-bearing, territorial,
and agricultural regional form. It is not a generic maze or combat form.

### 15.10 Minotaur Assignment

Every valid Minotaur result MUST possess `AuraFieldsStewardship` naming:

- the Minotaur as steward;
- Aura Field as region;
- the Glaüshouse resolution decision as assignment authority;
- the Synthesis evidence;
- all seven typed duties:
  - tend Aura crops;
  - guard field boundaries;
  - carry field loads;
  - open and maintain field routes;
  - guard harvests;
  - protect field workers;
  - stabilize field Current.

Missing any duty makes the result internally invalid. The assignment grants no
Aura Beach occupation and no Aura Sea guardianship.

### 15.11 Elf-to-Centaur Lifecycle

The legal lifecycle is:

```text
absent Elf identity
    ↓ register with established Aura Beach standing
active Elf
    ↓ prove lineage, readiness, standing, and rule
    ↓ authorize with Glaüshouse Resolution
    ↓ apply Elf→Centaur rule
Elf status = SynthesizedInto(Centaur identity)
active Centaur with predecessor and lineage
    ↓ atomically derive Aura Beach occupation
    ↓ atomically derive Aura Sea guardianship
```

The Centaur is the horse-associated mobile, coastal, horizon-facing,
territorial, and patrol regional form. It is not a generic woodland archer or
unrelated cavalry form.

### 15.12 Centaur Assignments

Every valid Centaur MUST possess `AuraBeachOccupation` naming the Centaur, Aura
Beach, assignment authority, evidence, and all eight duties:

- roam the Aura Beach;
- patrol the shoreline;
- guard Aura Sea access;
- watch coastal routes;
- escort travelers;
- recognize horizon changes;
- defend against coastal incursions;
- maintain the land-sea boundary.

Every valid Centaur MUST simultaneously possess `AuraSeaGuardianship` naming the
same Centaur, Aura Sea, assignment authority, evidence, and all four duties:

- guard access to the Aura Sea;
- watch the Aura Sea horizon;
- defend the Aura Sea boundary;
- maintain lawful land-sea passage.

Missing the guardianship makes the Centaur result internally invalid. The
assignments grant no Aura Field stewardship.

### 15.13 State Machine

Legal Being states are `Active`, `SynthesizedInto(result)`, and `Tombstoned`.

Legal transitions are:

| Prior | Command | Required result |
|---|---|---|
| absent | register Gnome or Elf | Active origin |
| Active Gnome | ratified Aura Field Synthesis | SynthesizedInto(Minotaur) plus Active Minotaur |
| Active Elf | ratified Aura Beach Synthesis | SynthesizedInto(Centaur) plus Active Centaur |
| Active origin/result | regional Tombstone | Tombstoned |
| any accepted event | exact retry | unchanged state and unchanged event count |

`SynthesizedInto` and `Tombstoned` are terminal for further Synthesis under the
current rules. No transition returns either state to Active.

### 15.14 Constitutional Questions

Every accepted Synthesis answers:

- Who is the predecessor?
- Who is the distinct result?
- What are their forms?
- Which lineage connects them?
- Which region establishes standing?
- Which Sandmanor site and institution control that standing?
- Which Sandmanor authority proved the prerequisites?
- Which Glaüshouse authority resolved the transformation?
- What evidence belongs to the predecessor?
- Which ratified rule applies?
- Which regional function results?
- Which stewardship or guardianship is granted?
- Which immutable event records the result?
- Does persistence reconstruct exactly the same result?

### 15.15 Required Invariants

- Every registered origin is exactly Gnome or Elf.
- Every registered origin has one stable identity.
- Every result has a different stable identity.
- Every Minotaur has a Gnome predecessor.
- Every Centaur has an Elf predecessor.
- Every result lineage begins with the predecessor lineage history.
- Every result appends exactly one lineage entry for its Synthesis.
- Every Gnome→Minotaur event is grounded in established Aura Field standing.
- Every Elf→Centaur event is grounded in established Aura Beach standing.
- Every accepted command has both required House decisions.
- Every required item of evidence names the predecessor.
- Every Minotaur has exactly the canonical field assignment.
- Every Centaur has exactly the canonical beach and sea assignments.
- No Minotaur has a Centaur-only assignment.
- No Centaur has a Minotaur-only assignment.
- Location without Synthesis produces no assignment.
- Exact retry produces no duplicate event or Being.
- Conflicting retry produces no mutation.
- Replay and live execution produce equal event and Being state.
- Archive decode and live execution produce equal state.
- Unknown archive versions fail closed.
- Traces and TUI events never change state.

### 15.16 Illegal States

Illegal configurations include:

- result identity equal to predecessor identity;
- Minotaur without Gnome predecessor;
- Centaur without Elf predecessor;
- evolved form registered as an origin;
- lineage history without the source;
- lineage history whose Synthesis identity differs from the committed command;
- Gnome→Centaur;
- Elf→Minotaur;
- Gnome→Minotaur without established Aura Field standing;
- Elf→Centaur without established Aura Beach standing;
- any Synthesis using Aura Sea as primary standing;
- Minotaur with Aura Beach occupation or Aura Sea guardianship;
- Centaur with Aura Field stewardship;
- Minotaur missing a field duty;
- Centaur missing a beach or sea duty;
- Synthesis without Sandmanor proof;
- Synthesis without Glaüshouse resolution;
- Synthesis using the wrong institution;
- Synthesis with a rejected or future decision;
- Synthesis without evidence;
- Synthesis using another Being's evidence;
- Synthesis after source Tombstone or prior Synthesis;
- duplicate non-idempotent Synthesis;
- replayed result differing from live reduction;
- assignment manufactured by trace, CLI, or TUI.

### 15.17 Failure, Recovery, and Retry

Every failure returns `RegionalSynthesisError` with a stable code. Validation is
completed against cloned/read-only predecessor state before any mutation.

On failure:

- no regional event is appended;
- no result Being is inserted;
- predecessor status is unchanged;
- no authority decision is consumed;
- no assignment is created;
- prior accepted history remains persistable and replayable.

Recovery consists of submitting a new command with a new event identity after
lawfully correcting the missing prerequisite. The runtime never edits the
rejected command. Exact retry of an already accepted command returns the
existing event.

No regional appeal or regional transfer recovery procedure is ratified.

### 15.18 Persistence and Migration

The regional archive magic is `HGREGV2\0`. Current version is V1. V0 is the one
accepted legacy fixture version.

The archive persists accepted registration, Synthesis, and regional Tombstone
inputs. Decode MUST submit each input to the production reducer in sequence.
Assignments MUST be derived, not trusted from stored presentation fields.

Migration MUST decode a supported version and encode canonical V1. Unsupported
versions, invalid tags, IDs, references, lengths, sequences, trailing bytes, or
reducer failures MUST reject the entire load.

### 15.19 Verification

Conformance MUST test:

- both lawful transformations;
- all typed field, beach, and sea duties;
- origin and result identity distinction;
- predecessor and complete lineage retention;
- exact idempotent retry;
- non-idempotent duplicate rejection;
- both reversed transformations;
- both wrong-region transformations;
- missing/rejected/wrong-institution authority;
- missing and mismatched evidence;
- Synthesis after Tombstone;
- location-only non-transformation;
- evolved origin rejection;
- Aura Sea primary-standing rejection;
- altered replay history;
- canonical byte round-trip;
- V0→V1 migration;
- unsupported-version rejection;
- trace/TUI non-mutation;
- end-to-end coexistence with kernel Waves and Bond replay.

### 15.20 Repository Mapping

The canonical implementation is:

- `src/sandmanor_lineage.rs`: existing forms and adjacency law;
- `src/world/house_institutions.rs`: Aura Field/Aura Beach sites and Sandmen /
  Glaüshouse institution identities;
- `src/constitutional/regional.rs`: regional model, rules, reducer, assignments,
  lineage, failures, and lookups;
- `src/constitutional/regional_persistence.rs`: V0/V1 archive and migration;
- `src/constitutional/scenarios.rs`: reusable public scenario fixtures;
- `src/constitutional/trace.rs`: read-only observability;
- `src/constitutional/tui.rs`: presentation-neutral event contract;
- `examples/constitutional_v2.rs`: executable demonstration;
- `tests/regional_synthesis.rs`: regional conformance;
- `tests/constitutional_v2_demonstration.rs`: end-to-end proof.

No duplicate regional lineage type is permitted. Existing bounded Recipe
Synthesis and final Bond Resolution remain separate domain processes.

## 16. Final Constitutional Statement

Bond governs lawful Current and Aura history.

Regional Synthesis governs the two ratified region-bound form transformations.

A Gnome may become a Minotaur only through lawful Aura Field Synthesis.

That Minotaur tends, works, maintains, and defends the Aura Field.

An Elf may become a Centaur only through lawful Aura Beach Synthesis.

That Centaur roams and patrols the Aura Beach and guards the Aura Sea.

Standing is required and never sufficient by itself.

Authority and evidence are explicit.

Lineage remains.

Replay remains equal to live law.

Presentation reports and never decides.
