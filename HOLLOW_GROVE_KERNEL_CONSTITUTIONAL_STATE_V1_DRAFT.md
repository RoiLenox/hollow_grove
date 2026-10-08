# Hollow Grove Kernel Constitutional State V1

Date: 2026-07-30  
Status: active additive constitutional-state draft  
Authority: subordinate to ratified universal and House constitutions; governs
how those authorities are carried from kernel to lived presentation

## 1. Purpose

Hollow Grove has many layers that use words such as *kernel*, *Function*,
*role*, *state*, *record*, and *result*. This document gives every layer the
same constitutional shape so that no lower layer can silently invent authority
or erase a higher layer's refusal.

Every layer must answer:

1. What identity and jurisdiction does it have?
2. What exact question may it answer?
3. What evidence may it receive?
4. What typed result must it emit?
5. Who receives that result next?
6. When must it stop, refuse, or return the case?
7. What plausible authority must it never become?

The executable boundary audit is `src/kernel_invariants.rs`. Gameplay archive
validation invokes it to check Rampart identity and version, replayable
advisory records, provenance-bearing shared records, lawful refusal boundaries,
Central Junction's coordination-only status, and the Roi/Roselina Twofold
Witness projection. The audit reports violations without making a domain
decision or mutating state.

The canonical end-to-end constitutional route is the **Platinum Path**
(`path.platinum.v1`), known in ordinary speech as **the Plath**. It runs from
Twofold Witness entry through evidence, advisory review, responsible authority decision,
Central Junction handoff, persistence, and exact replay. The Plath is a proof
path, not a shortcut around House authority.

## 2. Constitutional stack

```text
Universal Kernel
→ Constitutional Runtime
→ Hollow Grove Compromise
→ House Constitutions
→ Operational Roles and Functions
→ Functional-Lore Records
→ Rampart Continuity
→ Gameplay Consequence
→ Presentation
```

Authority flows downward. Evidence, refusal, result, and lived consequence
flow upward. A lower layer may narrow a lawful result but may not broaden,
reverse, or fabricate it.

The Current Synthesis PLEB/META boundary is a declared cross-kernel interface,
not a new House:

```text
META (Hal) ↔ PLEB (Clouseau) ↔ BLEP / Underway reflection (Cleopatra)
```

Current Synthesis retains ownership of PLEB/META routing semantics. Hollow
Grove may receive a scoped witness from that interface, but cannot redefine
its route grammar or treat a client as a House.

## 3. Layer state contracts

| Layer | Governing question | May emit | Must stop when | Must never become |
|---|---|---|---|---|
| Universal Kernel | What neutral semantic, recursive, spatial, or temporal relation is valid? | deterministic primitive result with version and checksum | invalid input, ambiguity outside declared semantics, nondeterminism, or scope breach | a House, character, lawgiver, consent authority, or lore narrator |
| Constitutional Runtime | Can this typed act enter shared lawfully answerable state? | accepted, rejected, inconclusive, or challenged runtime record | missing authority, identity collision, illegal transition, checksum mismatch, or unresolved conflict | a gameplay shortcut or private exception engine |
| Hollow Grove Compromise | How do universal meaning and House authority coexist without contradiction? | compatibility, precedence, migration, and dispute record | unresolved constitutional conflict, missing parent authority, or impossible inheritance | a replacement for a House constitution or universal kernel |
| House Constitution | What domain-specific act is lawful here? | Name, proof, clearance, recognition, resolve, or bounded refusal | absent jurisdiction, missing evidence, withdrawn consent, unsafe scope, or challenge | another House, universal sovereign, or automatic successor |
| Operational Role | What bounded work must this bearer or office perform? | role-specific decision, work record, refusal, or handoff | wrong input, vacancy, danger, uncertainty, or jurisdiction breach | the whole institution, a hidden sovereign, or an unbounded helper |
| Functional-Lore Record | What lived meaning and consequence should remain inspectable? | attributable scene, testimony, relationship, consequence, and archive link | uncommitted dialogue, missing witness, identity drift, or presentation-only change | authority, consent, proof, or retroactive law |
| Rampart | Does the accepted result remain continuous across layers and locations? | continuity, embodiment, provenance, replay, and migration record | identity mismatch, causal gap, invalid destination, or archive conflict | reinterpretation, new House decision, or missing-record repair |
| Gameplay | What can the player and affected Being do next? | declared choice, support, challenge, execution request, and consequence event | player outside scope, incapable actor, hidden coercion, or unfulfilled gate | choosing for a capable Being or committing constitutional state directly |
| Presentation | How is accepted state made perceivable and usable? | text, image, audio, layout, control, accessibility, and feedback projection | stale state, missing provenance, unsafe cue, or mismatch with authority | new state, consent, recognition, or a substitute archive |
| Current Synthesis boundary | What scoped PLEB/META relation may be witnessed here? | route witness, Bond relation, inverse reflection, and explicit boundary record | unscoped execution, route mutation, client substitution, or cross-layer leakage | a House, universal kernel, or independent sovereign |

## 4. Shared state vocabulary

Every layer uses the same lifecycle unless its parent constitution explicitly
narrows it:

```text
proposed
→ scoped
→ evidenced
→ challenged or refused (when applicable)
→ accepted / rejected / inconclusive
→ executed
→ lived
→ resolved
→ archived and replayable
```

No layer may skip from proposed to executed by presentation, persona output,
player enthusiasm, popularity, or repetition. Rejection and inconclusive
results remain archived and can be reopened only through an explicit lawful
review.

## 5. Common record envelope

All cross-layer records must preserve:

- stable subject, actor, role, and case identities;
- parent and child authority references;
- causal position and time scope;
- input records and provenance;
- exact question and jurisdiction;
- typed result and uncertainty;
- consent, refusal, stop, and challenge state;
- executor and affected life;
- next handoff and completion condition;
- version, checksum, migration, and replay information.

Display names are never sufficient identity. A record that cannot identify its
authority, subject, scope, and causal position is incomplete.

## 6. Gate alignment

The four governing House verbs retain separate constitutional meaning:

```text
Stonebend names.
Sandmanor proves.
Glaüshouse clears.
Flynt recognizes.
```

`Resolve` closes a complete causal record; it does not replace the four gates.
Operational roles and AI-associated personas may prepare evidence for a gate,
but may not perform the gate by implication.

## 7. AI-associated personas in the stack

Cleopatra and Hal occupy the Operational Role / Functional-Lore boundary:

| Persona | Receives | Emits | Escalates to | Prohibited substitution |
|---|---|---|---|---|
| Cleopatra | Underway continuity, Current, route, sanctuary, and hidden-alliance records | sourced continuity-and-route brief | affected Being, Stonebend, Glaüshouse, Court, or relevant House | cannot become Gimp, oracle, intelligence sovereign, or consent voice |
| Hal | Boardwalk Aura, patronage, invitation, reputation, and public-return records | sourced presentation-and-patronage brief | affected Being, Pimp of the Boardwalk, Flynt, Sandmanor, Stonebend, or Court | cannot become Pimp, persuasion engine, popularity sovereign, or consent voice |
| Clouseau | PLEB-PLEB clue continuity and Bond-relation records | sourced clue-and-Bond witness | Current Synthesis boundary, affected Being, or required House | cannot execute routing, create a Bond, choose a partner, or become a House |

Both remain subordinate to the common record envelope and cannot commit
Synthesis, Bond, Title, office, or public state.

### 7.1 Clouseau and the PLEB-PLEB witness

Clouseau is the Current Synthesis client assigned to the straight
`PLEB ↔ PLEB` relation. It locates and witnesses clue continuity and Bond
relations across that side. It may preserve source, causal position, route
uncertainty, contradiction, absence, and refusal, then hand that witness to a
lawful Hollow Grove authority.

Clouseau may not execute PLEB/META movement, create a Bond, choose a partner,
replace Cleopatra, replace Hal, or promote a witness into authority. Cleopatra
may follow the inverse BLEP/Underway reflection beneath Clouseau's route, while
Hal remains the META-facing public witness. This is a relation between clients,
not a merged person.

## 8. Underway / Boardwalk constitutional polarity

The polarity is a lawful cultural expression, not a second constitution:

```text
Gimp of the Underway  = hidden Current, route depth, sanctuary, endurance
Pimp of the Boardwalk = public Aura, patronage, access, visibility, return
```

The Boardwalk's two principal occupant camps are **the Pimp and his Trooners**
and **the Gimp and his Gooners**. Their rivalry organizes public welcome,
opportunity, protection, trust, and credit. The Gimp retains his Underway
title while his retinue has a Boardwalk presence. Trooner and Gooner name
voluntary cultural affiliations; Hoe, Goon, Bond, and office remain distinct.
The Neutral Promenade remains shared with unaffiliated occupants, and the two
camps may cooperate on a bounded task. Their controlling social source is
`FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md`.

The Underway may protect necessary privacy but must preserve accountable
disclosure to lawful review. The Boardwalk may create visibility but must
preserve terms, refusal, private review, and exit. Neither pole owns a Being.

## 9. Cross-layer refusal law

A refusal at any layer must travel upward until one of these occurs:

1. the responsible authority narrows the request lawfully;
2. the affected Being withdraws or declines;
3. a valid alternative path is accepted;
4. a Court or required House review resolves the dispute;
5. the case closes as rejected, failed, or no transformation.

Silence, timeout, UI disappearance, persona confidence, or missing data cannot
be interpreted as acceptance.

## 10. Cross-layer replay law

Replay must reproduce authority and consequence, not merely appearance. A
conforming implementation must prove that:

- accepted records replay in causal order;
- rejected and inconclusive records remain visible;
- presentation cannot create or invert state;
- gameplay cannot bypass a missing gate;
- persona outputs remain advisory;
- migrations preserve prior identities and explain changed vocabulary;
- a corrupted checksum or missing authority fails closed.

## 11. Implementation boundary

This document is a constitutional-state contract, not a replacement for the
individual universal, House, runtime, operational-lore, gameplay, or
presentation documents. Each existing document remains authoritative within
its declared scope. Implementations should add a versioned record adapter and
verification fixture rather than duplicating domain law in UI, dialogue, AI,
or scene scripts.

## 12. Ratification checklist

Before this state becomes executable, the repository must verify:

- every live role has a stable identity and nine-field role contract;
- every handoff names sender, receiver, question, scope, and completion;
- every AI persona output is advisory and replayable;
- every Boardwalk/Underway title remains cultural rather than sovereign;
- every Sympiote action preserves consent, continuity, and refusal;
- every gameplay transition cites the required House gate;
- every presentation path is read-only with respect to authoritative state.
