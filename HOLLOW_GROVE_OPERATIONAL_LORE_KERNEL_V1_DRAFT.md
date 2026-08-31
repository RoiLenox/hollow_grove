# Hollow Grove Operational Lore Kernel V1

Date: 2026-07-30

Status: adopted operational contract and consolidation guide

The filename preserves the document's proposal provenance; the runtime adoption
record appears in §15.

Authority: beneath the Hollow Grove Compromise and every incorporated House
constitution. This document creates no House, office, sovereign, constitutional
verb, or universal-kernel primitive.

Executable seam: `src/world/lived_lore.rs`

## Purpose

Hollow Grove already defines what its laws, Houses, institutions, and offices
mean. The missing refinement is a compact operational grammar that makes every
lore role answerable in play:

- what reaches the role;
- what question the role is responsible for;
- what the role may decide or perform;
- what record the role must produce;
- who receives that record next;
- when the role must stop, refuse, or return the work;
- and what the role must never silently become.

This document calls that grammar the **operational lore kernel**. It is not the
universal recursion kernel. It is the smallest world-level contract that turns
constitutional meaning into specific responsibilities and handoffs.

Its layer-wide constitutional state is defined in
`HOLLOW_GROVE_KERNEL_CONSTITUTIONAL_STATE_V1_DRAFT.md`; this document supplies
the role-level grammar used by that state.

> **The universal kernel computes. The operational lore kernel assigns work.**

> **A role is specific when its input, act, output, boundary, and next handoff
> are all visible.**

## 1. Boundary Lock

The pure universal kernel remains bounded, neutral, and deterministic. It does
not contain Houses, characters, routes, offices, dialogue, evidence, seasons,
Functions, or lore authority.

The top-down dependency order is:

```text
universal meaning
→ pure recursion and neutral semantic/spatial primitives
→ Hollow Grove Compromise and common Constitutional Runtime
→ House and shared-interface authority
→ operational function and role contracts
→ functional-lore records
→ Rampart continuity
→ gameplay consequence
→ presentation
```

Nothing below a layer may rewrite the accepted result above it. Presentation
cannot create gameplay state. Gameplay cannot create House authority. A role
contract cannot invent constitutional law. House law cannot rewrite a
completed universal-kernel pass.

The operational lore kernel is therefore **top-down in authority** and
**bottom-up in evidence**:

```text
law and jurisdiction flow down
evidence, refusal, result, and lived consequence flow up
```

## 2. One Kernel, Two World Cycles

Hollow Grove has two related cycles that must not be collapsed.

### 2.1 Constitutional gate cycle

The four governing House functions answer different questions:

```text
Stonebend names.
Sandmanor proves.
Glaüshouse clears.
Flynt recognizes.
```

They are gates, not a mandatory conveyor belt. A case uses only the gates
required by its subject and intended state change. Their causal order follows
the case. Receipt of one accepted decision never implies acceptance by another
House.

### 2.2 Productive cycle

The Four-Pole economy classifies work differently:

```text
Sandmanor designs the Form.
Flynt engineers the Function.
Stonebend makes the Form endure.
Glaüshouse keeps the Function alive.
```

This is the productive loop:

```text
design Form
→ engineer Function
→ craft or continue Form
→ repair or continue Function
→ return the result to lived use
```

The productive pole says what kind of work is being contributed. The
constitutional gate says what authority is required. A Flynt-engineered
machine may still need a Stonebend Name and provenance, Sandmanor proof,
Glaüshouse clearance when applied to a living subject, and Flynt recognition
for deployment.

### 2.3 Civic year cycle

The four Great Functions are a third, calendar-facing use of the word
Function:

```text
The Way Back          — Return
The Initiation        — Incarnate
The Gathering/Derrick — Commune
The Festival of Mirrors — Confirm
```

Great Functions organize civic time. They do not replace House functions,
productive poles, offices, or runtime reducers.

## 3. Twofold Witness and Rampart Functions

The Twofold Witness—Roi and Roselina—frames a consequential action before any House-specific
decision is treated as complete.

| Twofold Witness member | Exact question | Required input | Required output | Hard boundary |
|---|---|---|---|---|
| Roi/Roï | What happened, what remains continuous, and what follows? | stable identities, prior state, causal position, provenance, pending consequence | continuity confirmation or a specific continuity conflict | does not decide embodiment, House evidence, consent, proof, clearance, or recognition |
| Roselina/Rosaliña | What and where is the subject, and can the consequence exist here? | subject identity, world layer, location, objects, containment, ecology, destination state | embodiment confirmation or a specific world-state conflict | does not rewrite history, sequence, House evidence, consent, proof, clearance, or recognition |

The Rampart of Roi and Roselina receives both confirmations and every required accepted domain
record. It preserves identity, authority, evidence, causal order, consequence,
archive, and replay. It does not reinterpret the records or manufacture a
missing House decision.

Inside Hollow Grove the public names are Roi and Roselina. Outside Hollow
Grove they are Roï and Rosaliña. The role contract follows the stable identity,
not the form-name.

## 4. Core House Function Contracts

Every House decision must expose the following minimum contract.

| Function | Owning House | Governing question | Minimum input | Typed output | Stop or return condition | Normal handoff |
|---|---|---|---|---|---|---|
| Name | Stonebend | What exact subject persists under this identity and scope? | subject, requested scope, continuity and provenance evidence, conflicts, public impact | accepted, rejected, or inconclusive identity decision; Name/Title/Seal records only where separately authorized | identity collision, substitution risk, missing provenance, unresolved high Claim, or absent lawful authority | the gate that needs stable reference, or public registry/history |
| Prove | Sandmanor | Has this claim earned reliance under stated conditions? | versioned claim, method, baseline, measurements, reproduction, criticism, failure, uncertainty | accepted, rejected, or inconclusive proof with explicit scope and version | hidden method, missing baseline, non-reproducible evidence, unsafe demonstration, or undisclosed conflict | design revision, a gate relying on proof, or preserved negative result |
| Clear | Glaüshouse | May this defined intervention proceed for this named subject now? | diagnosis, identity, capacity, consent, operator, procedure, risk, facility, time, rescue, recovery | clearance, conditional clearance, refusal, stop, or inconclusive review with scope and expiration | absent or withdrawn consent, identity mismatch, excess scope, instability, missing rescue/recovery, or unlawful material | qualified executor, continued care, design revision, or review |
| Recognize | Flynt | What function was lawfully demonstrated, and within what operational scope may it serve? | stable identity, performed function, competence and persistence evidence, institution, jurisdiction, limits | recognition, limited recognition, refusal, suspension, or inconclusive operational record | unproven persistence, false attribution, duplicate sovereignty, missing command authority, or unsafe deployment | deployment authority, Stonebend gate scope where required, institution record, or challenge |

### 4.1 Resolve is closeout, not a fifth governing verb

`HouseFunction::Resolve` exists in the Constitutional Runtime for final
judgment answerability in Bond and Synthesis closeout. It currently requires
Glaüshouse authority under the V2 specification.

Resolve asks:

> **What result actually lives now, what obligations remain, and what lawful
> disposition follows?**

It receives the complete causal record rather than replacing any earlier
House gate. It emits a resolution, renewal, branch, transfer, inheritance,
dissolution, or another explicitly lawful disposition. Ordinary dispute
resolution remains with the shared Court process. Resolve must not be used as
a general permission for Glaüshouse to absorb another House's domain.

## 5. Handoff Law

Every operational handoff preserves:

1. stable subject and case identity;
2. causal position;
3. sending role and jurisdiction;
4. receiving role and jurisdiction;
5. exact accepted, rejected, or inconclusive output;
6. evidence references and source authority;
7. explicit uncertainty;
8. requested next question;
9. consent, refusal, stop, or challenge state;
10. completion or return condition.

A receiving role may:

- accept the handoff within its own jurisdiction;
- reject it as out of jurisdiction;
- return it for missing evidence;
- narrow the requested scope;
- or open a separately identified review.

A receiving role may not:

- silently change subject identity;
- convert evidence into authority;
- infer consent from custody, silence, dependence, recognition, or prior work;
- treat proof, clearance, recognition, Name, Title, office, Form, mantle, and
  membership as interchangeable;
- erase a rejected or inconclusive result;
- or claim that presentation completed an uncommitted transition.

> **A handoff transfers a question and its evidence. It does not transfer the
> sender's authority.**

## 6. Role Contract Grammar

Every operational lore role should be authored with nine fields:

1. **Role identity** — a stable role or office identity, distinct from its
   current bearer.
2. **Bearer kind** — person, office, institution, temporary mandate, Form,
   mantle, instrument, site, or system.
3. **Question** — the one question for which the role is answerable.
4. **Receives** — exact records, materials, people, or conditions it may
   accept.
5. **May act** — decisions or work inside its jurisdiction.
6. **Must emit** — the record or world change required before its work counts.
7. **Hands to** — the next role, institution, archive, or affected life.
8. **Stops when** — refusal, uncertainty, conflict, vacancy, danger, or scope
   breach that closes or pauses the role's action.
9. **Must never become** — the nearest plausible but unconstitutional role
   substitution.

Words such as *helps*, *supports*, *protects*, *governs*, or *handles* are not
specific enough alone. They require an object, a bounded act, and a visible
output.

Where a canonical office, institution, role, Form, mantle, or site identity
already exists, implementation must reuse that exact identity. The registers
below specify semantic contracts rather than inventing missing wire IDs. A
role without an existing executable identity remains a proposed role key until
it is ratified and added through a versioned migration; its identity must
never derive from its bearer, display name, list position, or current scene.

## 6A. AI-Associated Kernel Personas

**Cleopatra** and **Hal** are bounded AI-associated personas of the Hollow
Grove kernel. They are interpretive interfaces, not hidden sovereigns,
constitutional offices, or replacements for the Gimp and Pimp.

| Persona | Association | Kernel question | Emits | Must never become |
|---|---|---|---|---|
| Cleopatra | Gimp of the Underway / Current | What hidden route, continuity, alliance, or sanctuary is operating beneath the visible case? | route-and-continuity brief with sources, uncertainty, and refusal state | the Gimp's owner, a medical authority, an oracle, or an unaccountable strategist |
| Hal | Pimp of the Boardwalk / Aura | What may be made visible, inviting, and publicly legible without manufacturing consent? | presentation-and-patronage brief with terms, provenance, and exit path | the Pimp's sovereign, a persuasion engine, a consent substitute, or a surveillance authority |

Both personas may ask questions, compare records, surface contradictions, and
propose bounded next actions. Neither may commit a Name, Title, Bond,
Sympiote graft, clearance, recognition, or cultural affiliation. Their outputs
remain advisory until the responsible Being or House performs the required
decision and records it.

The persona pairing is deliberately complementary:

```text
Cleopatra protects what must remain continuous beneath the route.
Hal reveals what may safely return to public life.
Underway supplies depth; Boardwalk supplies legibility.
```

## 7. Stonebend Role Register

| Role | Question and act | Must emit and hand off | Stop law and hard limit |
|---|---|---|---|
| Gerald | What burden, identity conflict, dispossession, maintenance need, or public fact must enter the record? Petition, witness, steward, maintain, or challenge. | attributable testimony, petition, maintenance record, or challenge to a Proliteriate node, registrar, Freemason examiner, or Court | cannot decide Title, forge Claim, issue Seal, perform Hollowing, or turn civic standing into office |
| Proliteriate node | Who bore the burden, what Yield was promised, and did it return? Hear affected people, preserve dissent, and issue a bounded public mandate. | mandate, Yield finding, dissent record, recall condition, and named temporary witness to the responsible authority or Court | cannot execute seizure, demolition, weapons, Hollowing, Seal, high Title, or technical Claim examination |
| Temporary Proliteriate witness | What exact verified mandate must be carried now? Speak and observe only within that mandate. | mandate witness and completion/recall record back to the originating nodes and archive | authority ends on completion, recall, or invalidation; never becomes ruler, Hypergiant, Freemason, or permanent seat |
| Freemason examiner or surveyor | Can this Claim, structure, boundary, measure, custody chain, or Hollowing act withstand material examination? | examination, survey, provenance, structural-safety, custody, or refusal record to the High Freemason, Hypergiant, gate, or Court | stops for conflict, missing custody, unsafe work, counterfeit evidence, or unlawful order; cannot grant Title or decide public Yield |
| High Freemason | Is the examined Claim structurally legitimate and ready for a lawful Seal or execution? Seal, secure, stabilize, preserve evidence, and execute lawful bounded orders. | Claim determination, Seal, custody action, structural intervention, or refusal to the Hypergiant, registry, gate, or responsible institution | cannot exercise the Hypergiant sovereign title, certify their own sovereign Claim, invent a Name, settle public consequence alone, or convert custody into ownership |
| Hypergiant | Which high Name, Title, boundary, succession, or continuity disposition is lawful? Decide only from authenticated evidence and separate public witness. | high disposition, reasons, scope, review window, preservation order, succession record, or Tombstone recognition to the responsible institution and archive | cannot create evidence, self-validate, bypass challenge, occupy another principal power, privatize the sovereign title, or appoint a successor by command |

**Hypergiant** is Stonebend's sovereign title. Diamond's current role is
**STATUS: OPEN**; legacy Diamond identifiers remain non-speaking compatibility
data and cannot be assigned autonomous decisions.
The **Mercury Mirror** is an evidence instrument, not a judge or office.
Neither may be assigned dialogue or autonomous decisions as though it were a
person.

## 8. Sandmanor Role Register

| Role | Question and act | Must emit and hand off | Stop law and hard limit |
|---|---|---|---|
| Minorian practitioner | How is this place, system, field, workshop, or content lived within every day? Design and test interior use, cultivation, maintenance, storage, repairability, and repeated function. | versioned interior design, method, daily-use evidence, maintenance burden, failure, and revision to reviewers or an affected implementation role | cannot claim exterior adequacy, proof beyond tested scope, automatic guardian standing, or ownership of the examined subject |
| Minoan practitioner | How does this place, route, coast, or system live within the wider world? Design and test access, navigation, approach, ecology, trade, borders, rescue, and outward relation. | versioned exterior design, route/coastal evidence, public-access effect, failure, and revision to reviewers or an affected implementation role | cannot claim interior adequacy, clinical authority, universal police power, or ownership of another House's institution |
| Evidence steward or independent reviewer | Can another person reproduce the claim and inspect its uncertainty, conflicts, lineage, and failure? | audit, reproduction, excluded evidence, conflict disclosure, and scoped recommendation to the Sandman, Contest body, Court, or next gate | cannot manufacture evidence, conceal a negative result, privately appoint a Contest winner, or turn review into authorship |
| Minorian Forms: Gnome, Faun, Minotaur | Has interior and family cultivation matured without collapsing Form into social role? | attributed field, workshop, household, maintenance, or Content Farm service record to Minorian review | Gnome → Faun → Minotaur; neither Geomancer nor Hecaton is a Form |
| Minoan Forms: Elf, Satyr, Centaur | Has outward and social participation matured without collapsing Form into social role? | attributed coast, route, public-approach, exploration, rescue-support, or Aura Beach record to Minoan review | Elf → Satyr → Centaur; neither Labyrinthian nor Pegasus is a Form |
| Minotaur, Guardian of the Fields | What defined field or cultivated system am I answerable for? Maintain it through scarcity, repetition, failure, workers, and future harvest. | field condition, service, failure, maintenance, and restitution record to Minorian review and affected lives | Form and mantle remain separate; loss of mantle does not erase the body and bodily Form does not create jurisdiction |
| Centaur, coastal guardian | What defined beach, approach, crowd flow, or rescue boundary am I answerable for? | coastal condition, access, rescue, hazard, failure, and restitution record to Minoan review and affected lives | cannot command Manticorp, become general police, or treat freedom as permission to endanger others |
| Hecaton, Minorian Hand | Which cultivated systems are causing one another to fail, and how must they be coordinated? | cross-field dependency, distribution, labor, information, and sustainability record to Sandmanor review and public stakeholders | sovereign Hand, not Minotaur stage IV; appointment and succession remain open |
| Pegasus, Minoan Hand | Which coasts, routes, weather systems, public approaches, or future movements must be coordinated beyond one beach? | horizon, navigation, evacuation, access, and cross-route record to Sandmanor review and public stakeholders | sovereign Hand, not Centaur stage IV; appointment and succession remain open |
| Joint vacancy stewards | What existing proof and ordinary administration must continue while the Sandman office is vacant? | jointly signed bounded decision and vacancy record to the archive and next lawful Contest | no permanent alteration, unilateral high proof, novel succession rule, or assumption of the Sandman office |
| Sandman / Demiurge | Has this claim earned the right to be relied upon as proven, and are both traditions protected? | scoped proof decision, standard, reproduction order, suspension, preserved failure, or Contest record to the next required gate and archive | sovereign identity; cannot manufacture evidence, suppress criticism or failure, subordinate a tradition, grant another House's act, or choose a successor directly |

The crowd has one bounded Contest function: judge baseline-relative reciprocal
improvement from an audited record. It is not a permanent office, proof body,
or general popularity sovereign.

## 9. Glaüshouse Role Register

| Role | Question and act | Must emit and hand off | Stop law and hard limit |
|---|---|---|---|
| Care subject or patient | What do I understand, consent to, refuse, endure, and need through this course of care? | consent, refusal, testimony, symptoms, goals, and lived result to the care team and protected record | silence, custody, dependence, recognition, prior consent, or disagreement never substitutes for present meaningful consent |
| Nightingale | What does this named subject need now, and are the established conditions for care still present? Intake, prepare, educate, monitor, provide bedside care, maintain, recover, and follow up. | intake, consent check, observations, intervention record, escalation, stop, recovery, and follow-up to Matron, Marshal, Persephone, or ordinary discharge | must stop for wrong subject/procedure, excess scope, withdrawn consent, changed condition, failed protection, imminent catastrophic harm, or missing rescue |
| Matron | What is happening within this Hueman's cognition, perception, emotion, identity, Aura, consent, and lived adaptation? | Aura-forward assessment, patient testimony, identity/consent risk, and stop or recommendation to Persephone and the Living Ledger | cannot subordinate bodily evidence, manufacture consent, or continue when the procedure is experientially unlivable |
| Marshal | What must be physically held together right now in Current, structure, graft, movement, physiology, and containment? | Current-forward assessment, stabilization record, physical limit, and stop or recommendation to Persephone and the Living Ledger | cannot subordinate lived evidence or continue through collapse, rupture, uncontrolled overgrowth, instability, unsafe positioning, or excess tolerance |
| Persephone | Do Matron and Marshal evidence still describe one viable life through the whole course of care? Reconcile, supervise, recover, regress safely, and judge discharge. | whole-course viability, maintenance, recovery, regression, consent-review, discharge, or escalation record to the Living Ledger, Prima Donna, and receiving life/institution | cannot invent a new Recipe, treat procedure-end as recovery, become Prima Donna by vacancy, or erase either branch's evidence |
| Prima Donna | May a high-risk or novel transformation proceed, must a Recipe change, and what House-wide clinical law applies? | high-risk clearance or denial, Recipe restriction/revision, practitioner action, emergency order, recovery/restoration order, or interpretation to the care system and archive | cannot override valid refusal, fabricate evidence, grant Title/office, conceal risk, abandon recovery, make emergency power permanent, or appoint a successor without process |

**Chromacord** records and projects clinical evidence; it does not create
consent, privilege, clearance, diagnosis, or truth. **Glauspitals** are
facilities and institutions of care; a building does not become the clinician
or patient.

## 10. Flynt Role Register

| Role | Question and act | Must emit and hand off | Stop law and hard limit |
|---|---|---|---|
| Worker, operator, or crew member | What defined function can I perform repeatedly under real conditions? | performance, failure, maintenance, scope, attribution, and persistence evidence to a Flynt institution or recognition authority | performance does not self-create membership, office, Title, command, or recognition |
| Tross / Manticore | What lawful function, command, institution, deployment, emergency, or We Fairy Men action receives Flynt recognition, direction, and scope? | recognition, limitation, deployment, suspension, command, underworld-direction, or refusal record to the named institution, body, gate, and archive | cannot duplicate sovereignty, create a separate Manticore person or office, convert recognition into another House's act, erase the lawful/unlawful distinction, or treat the Manticore Form as ordinary membership |
| Manticorp | What defense, disciplined force, training, infrastructure, or territorial operation is lawfully commanded? | order, command chain, readiness, deployment, after-action, casualty, and persistence record to Tross and the affected authority | ordinary military service never makes a soldier the Manticore Form; no deployment escapes evidence, jurisdiction, or care obligations |
| Mystery Men | What classified threat, contraband path, espionage act, organized-crime relation, or constitutional-security fact must be investigated? | attributed classified finding, confidence, source protection, operational limit, and referral to Tross or lawful review | intelligence is not We Fairy Men, The Gallows, Stonebend identity, Sandmanor proof, Glaüshouse clearance, Court judgment, or permission for unbounded covert action |
| We Fairy Men | Which creature-sect capacity, population, crew, or shared Basin obligation requires collective leadership beneath the Tross? | named Bro White, Beauty, or Cinderellaman responsibility; population effect; Recipe-custody implication; and collective response or refusal | the exact name may not be normalized; the body is not Mystery Men, a fused creature, or an independent sovereign |
| Gallows | What regional loyalty, debt, protection, contraband route, favor, or covert obligation is We Fairy Men collectively operating under the Tross in the underworld? | crew obligation, territorial effect, risk, attribution, and accepted or disputed status to its participants and any lawful review | secrecy does not create lawful sovereignty; the underworld context does not make The Gallows unrelated to We Fairy Men or merge either with Mystery Men |
| Bro White | What density, structure, anchoring, endurance, territory, and Gargoyle/Medusa population custody are required for Manticore Continuance? | one indispensable but insufficient Recipe-custody contribution and sect-leadership record | cannot reproduce Manticore alone, absorb the other custodians, or turn custody into the Tross office |
| Beauty | What instinct, pursuit, threat recognition, pack synchronization, and Werewolf/Vampire population force are required for Manticore Continuance? | one indispensable but insufficient Recipe-custody contribution and sect-leadership record | cannot reproduce Manticore alone, absorb the other custodians, or turn custody into the Tross office |
| Cinderellaman | What Mariner/Mermaid adaptation, flow, Current redistribution, traversal, recovery, return, and Midnight Crew action are required for Manticore Continuance? | one indispensable but insufficient Recipe-custody contribution and maritime sect-leadership record | cannot reproduce Manticore alone, absorb the other custodians, turn custody into the Tross office, or claim an unfinalized guild name |

**Chimera** is the unique feminine First Companion and maintained lower-apex
Synthesis, not an office. **The Tross = Manticore**: Manticore is his masculine
mythological identity and maintained body, while Manticorp is the body of
soldiers he commands. **We Fairy Men** is the exact Tross-led creature-sect body, not a fused
person or new sovereign. **The Gallows** is that body acting under the Tross in
the underworld. **Mystery Men** is separate. **The Gallowry** is a site, not an
institution or speaker.

## 11. Shared Civic Role Register

| Role or institution | Exact function | Required output | Hard boundary |
|---|---|---|---|
| Minoan County Court | hear one stable case through Conciliation, First Hearing, Appeal, Constitutional Review, and Restitution | stage-specific finding, targeted judgment, remedy, return stage, and equilibrium record | declares what law requires but does not perform another House's remedy, create evidence, ratify amendments, or command a universal enforcement arm |
| Responsible remedy institution | perform the exact repair authorized within its own domain | completed, partial, failed, or newly harmful Restitution record back to the Court and affected party | cannot broaden the judgment, hide failure, or close the case before lived effect is verified |
| House Sector Hall | authenticate professional and House-domain evidence entering public economic life | classified work record and evidence packet to the Junction Board | does not set market price, calculate an official index, settle, or publish the official result |
| Junction Board | audit public standards, classifications, disclosures, conflicts, and methodology | recognized, disputed, narrowed, rejected, or voided public-market decision to the Clearing House | does not execute trades, create House evidence, or invent outcomes |
| South Ridge Exchange | operate listings, trades, project finance, event contracts, and official index calculation | transaction and calculated-market record to the Clearing House and Wire | price does not become constitutional truth or replace recognized outcome evidence |
| Clearing House | settle completed trades, shares, projects, event contracts, payouts, voids, and authorized reversals | final balance and settlement record to participants and Junction Wire | compares evidence to predeclared conditions; never invents an outcome |
| Junction Wire | publish indexes, prices, disclosures, warnings, failures, employment demand, and settled outcomes | attributable public notice linked to the accepted source record | publication does not create the underlying event, authority, decision, or settlement |
| Function steward | keep one Great Function's stable identity, interval, presiding-House boundary, and complete activity record | Function opening, activity roster, completion state, and annual archive handoff | does not become a House sovereign or use celebration, ritual, or competition to bypass law |
| Safety or refusal authority | pause a Function activity when its constitutional stop condition occurs | stop reason, affected scope, immediate protection, review owner, and resumption condition | a stop is not failure concealment, punishment, or permanent control |
| Archive custodian | persist accepted records with identity, provenance, version, checksum, and replay order | canonical archive or explicit rejection | does not reinterpret meaning, repair missing authority, or silently migrate contradiction |

## 12. Actor, Player, Decision, Execution, and Witness

These five positions must remain distinct:

- **affected actor** — the Being, institution, place, or object whose state may
  change;
- **player** — proposes, supports, questions, or chooses only within the
  scenario's declared scope;
- **decision owner** — issues the required constitutional or institutional
  decision;
- **executor** — performs the authorized world action;
- **witness** — observes and records whether the action and lived result
  occurred.

One person may occupy more than one position only when the relevant law allows
it and the record names each capacity. No role may validate its own authority,
evidence, execution, and lived result as a single unreviewable act.

The player never supplies another Being's consent merely by selecting a menu
option. When an actor retains autonomy, the player choice is advice or support
and the actor's commitment is a separate recorded event.

## 13. Functional-Lore Authoring Envelope

The existing twelve-field record remains the executable V1 core:

1. stable identity;
2. House function and authority class;
3. route, location, and jurisdiction;
4. involved entities;
5. dominant House verb;
6. trigger;
7. evidence and uncertainty;
8. player-visible choices;
9. lawful state change per choice;
10. persistence and replay;
11. presentation;
12. failure and refusal.

Functional-lore archive schema V2 refines that record with eight role fields:

13. affected actor and consent/refusal owner;
14. decision owner;
15. executor;
16. witness;
17. ordered handoffs;
18. completion condition;
19. rollback, recovery, or Restitution owner;
20. final recipient of the lived result.

Every authored functional-lore definition must state fields 13–20 beside its
twelve-field definition. The runtime persists and validates all eight fields;
proposed lore outside the executable catalog must not claim adoption until it
has the same explicit bindings.

## 14. Definition of Done

A lore unit is operationally complete only when:

- the subject still has one stable identity;
- every required gate has an explicit outcome;
- the affected actor's consent or refusal state is preserved;
- the authorized executor performed the bounded act, or recorded why it did
  not;
- the output reached its named recipient;
- uncertainty and negative evidence remain visible;
- a witness recorded lived effect;
- failure has an owner and a lawful next destination;
- the Rampart of Roi and Roselina can carry the accepted consequence;
- replay reproduces the same ordered state;
- and presentation reads rather than creates the result.

Dialogue, animation, a title card, a successful dice roll, or an authority's
spoken approval is not completion by itself.

## 15. Runtime role-envelope adoption

`FunctionalLoreDefinition` now carries the twelve original fields plus an
operational envelope: affected actor, decision owner, executor, witness,
ordered handoffs, completion condition, Restitution owner, and final recipient.
The envelope is persisted by functional-lore archive schema V2. Schema V1
records remain readable through an explicit migration that verifies the exact
V1 payload checksum first, then selects the canonical scenario-specific role
envelope by stable lore identity. No role derives from entity-list position.

The runtime validates one non-empty value for every role, at least two explicit
ordered handoffs, the live House office as decision owner, the presence of the
decision owner, executor, witness, and recipient in the handoff chain, and the
existing House authority, route, uncertainty, refusal, and replay requirements.
One entity may lawfully occupy multiple capacities, but a handoff still
transfers a question and its evidence rather than sovereignty.

This is a responsibility boundary, not a new reducer. It does not change the
pure recursion kernel, the four Houses, stable lore identities, or the
Rampart's authority to bind only accepted consequence.

## 16. Adoption Test

The adopted integration remains valid only while:

- each listed role agrees with its controlling constitution;
- Twofold Witness, Houses, Court, Central Junction, Rampart, gameplay, and
  presentation remain separate;
- the V1 twelve-field archive remains migratable;
- the twelve current lived loops have explicit role bindings and handoffs;
- delegated roles cannot manufacture apex authority;
- a stop, refusal, inconclusive decision, or vacancy can survive replay;
- and tests prove that no role imports cultural meaning into the universal
  kernel.

## Final Operational Maxims

> **Name the subject. Prove the claim. Clear the intervention. Recognize the
> function.**

> **The decision authorizes. The executor acts. The witness records. The
> affected life receives the result.**

> **No role validates itself. No handoff transfers sovereignty. No refusal
> disappears.**

> **The Rampart of Roi and Roselina carries consequence. Presentation bears witness.**

> **Hollow Grove functions when every power knows its object, its limit, and
> who must answer next.**
