# Flynt — The Electoral College, The Chimera Council, The Warden Registrar & The Warden Parley
## Consolidated proposed constitutional specification — Revolution C / V1

**Design approval date:** 2026-10-26  
**Authority status:** **CREATOR-APPROVED PROPOSED CONSTITUTIONAL DESIGN; NOT ENACTED LAW.**  
**Intended Codex repository destination:** `docs/FLYNT_ELECTORAL_COLLEGE_WARDEN_PARLEY_PROPOSED_CONSTITUTION_V1.md`  
**Implementation status:** Documentary specification only; **does not authorize runtime, constitutional authority, save migration, election execution, succession, or vacancy transitions**.  
**Scope:** Flynt's proposed university/civic electoral institution and its relation to the existing Tross hierarchy. Does **not** enact the unresolved Tross succession, broader elected-command administrative rules, or cross-House constitutional amendments.

> **Interpretive rule.** In this document, “shall,” “must,” and numerical deadlines describe **creator-approved target design if lawfully enacted**, not commands already authorized by present constitutional law. New Court jurisdictions and inaugural ratification authority must be independently established through existing lawful procedures. An existing implementation, test fixture, enum, prototype, or accepted creative decision is not itself proof of enactment.

---

## 0. Authority, preservation and source reconciliation

1. The repository's existing authoritative texts, especially `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`, `FLYNT_CONSTITUTION_V2.md`, and `MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md`, must be **read and reconciled**, not silently overwritten by this proposal. Read the relevant Revolution C recovery/continuation/constitutional closure/governance/decisions documents before editing.
2. The existing Court framework describes Claim initiation, notice, testimony, hearing, review, applicable House/affected-House ratification, Stonebend seal/public record, implementation and Restitution steps; verify precise authority and sequence from the live repository. **The Court reviews; it does not automatically ratify.** Cross-House assent and foundational four-House requirements cannot be waived by a Flynt referendum.
3. **Original-adoption bootstrap is OPEN:** currently cited enacted Flynt sources do not clearly identify the authority/method to ratify the *first* new Flynt Warden-referendum amendment. Neither the Tross, the proposed Chimera Council, the prospective Registrar, a Court, nor these in-chat creator approvals may supply that missing legal authority by fiat. Resolve by cited enacted law and an actually authorized adoption chain, or fail closed with a decision packet.
4. No new Court power to certify a deadlocked election, grant an emergency stay, hear a novel challenge or impose a deadline may be presumed merely because it is proposed here.
5. Preserve existing `FinalizeSuccession` and `OpenFinalVacancy` fail-closed behavior with `QSuccessionReconciliation`, Tross/Marquise historical identity and office records, and Q evidence/queue/test history. **No Q sovereign transfer or Final Vacancy is authorized by this proposal.**
6. Do not alter unrelated Sandmanor, Glaüshouse, Stonebend, Syndicate, OS/niri, frozen archives, historical manifests, save readers or user edits. The local worktree is intentionally dirty; check current `git status --short` before touching anything; never reset/clean/stash/change branch/stage/commit without express authority.
7. The historic/current naming transition must be recorded without destructive replacement: **“Electoral Registrar” → “The Warden Registrar”; “Flynt Electoral Commission” → “The Electoral College”; no Deputy Registrar; no separate Tribune or external certification panel.** Historical occurrences may remain where necessary for source-era accuracy.

## 1. Constitutional actors and structural invariants

### 1.1 The Electoral College

**The Electoral College** is one Flynt institution that is **literally a university** and also hosts Flynt's legally defined electoral/civic functions. Its academic domains include civic education, constitutional history/scholarship, electoral administration education and professional study; academic study alone confers no Form, Synthesis, professional service authority, Agency position, security entitlement or elected office.

Neither university enrollment, employment, graduation nor academic qualifications are prerequisites for Warden citizenship or the constitutional right to vote.

### 1.2 The Tross as Dean

The single lawful **Tross** holds the institutional title **Dean of The Electoral College** and presides over the Chimera Council while remaining the one sovereign of Flynt. **Dean** supersedes the unselected Chancellor/President proposal. Holding Dean does not confer an extra sovereign office, additional vote in ordinary Warden elections, or unilateral authority to certify or alter election outcomes.

### 1.3 The Chimera Council — four members, fixed seats

**The Chimera Council** consists of exactly four persons in their existing lawful offices:

| Council member | Existing office | Ambisynth / Synthesis association | College remit (educational/institutional) |
|---|---|---|---|
| Presiding Dean | **The Tross** | Sovereign office; do not confuse with earnable Manticorp Form | Overall university direction and council convening |
| Member | **Navigator of the Tross** | Gargoyle ↔ Medusa | Strategy, navigation, constitutional scholarship |
| Member | **Tross Chief** | Werewolf ↔ Vampire | Leadership, public service, civic integrity |
| Member | **Mariner of the Tross** | Merman ↔ Mermaid | Maritime study, logistics, civic outreach |

The three elected commanders remain peers reporting **directly to The Tross**, not through the Navigator. They are not additionally elected to the Chimera Council. The Council is not a fourth Face, a sovereign, or an extra set of command offices. Career ranks, operational faces and cover identities must remain separately typed. The Council's name reflects the **three-Current-Form** Chimera convergence, not a requirement that Council members complete the separate Chimera Ritual or automatically hold a Chimera/Manticorp Form. The lawful Tross office remains distinct from the Manticorp physical Form and Manticorp [institution].

### 1.4 The Warden Registrar and The Warden Parley — two functions of one institution

**The Warden Registrar** is the single officeholder **directly elected by qualified Wardens**. **The Warden Parley** is the associated Warden civic forum/process over which that official presides; these are two functions within the same civic constitutional institution, **not** two independently elected officeholders. There is **no Deputy Warden Registrar and no separate Tribune of the Wardens**.

The Registrar's **neutral administrative function** encompasses electoral rolls, authenticated voting, tally audit, routine lawfully scoped certification and election records. The **Parley function** handles petitions, deliberation and limited constitutional/civic-rights objections; it cannot secretly alter vote counts, independently ratify an amendment or impersonate a Court.

### 1.5 Separation of responsibility

- **Wardens:** direct democratic source of the proposed Flynt amendment and Registrar-election vote.
- **Tross / Dean:** lawful university leadership and existing sovereign responsibilities; no election manipulation or unilateral Registrar dismissal.
- **Chimera Council:** collegiate university governance **and rule-bound certification of Registrar elections/reelections/recalls/special elections**, as specified below; never a discretionary ratifier of Warden referendums.
- **Warden Registrar:** elected electoral administration and Parley convening/concurrence, with recusal where conflicted.
- **Warden Parley:** participation of qualified Wardens with a bounded, jointly authorized suspensive veto.
- **Minoan Court:** independent review **to the extent jurisdiction is actually lawfully enacted**; not another sovereign or the political ratifier.
- **Stonebend:** established sealing/public-record function after lawful adoption; not a substitute for House assent/ratification.

## 2. Electorate and constitutional-amendment voting

### 2.1 One Warden, one vote

Every **legally recognized Flynt Warden citizen** has one constitutional vote, regardless of university enrollment, Agency, wealth, career rank, office, Synthesis/Form mastery, political position or travel outside Flynt. No additional weighting or votes attach to the Dean, any Council seat or an advanced Form. Visitors and unadmitted citizens of other Houses gain no Warden vote merely by presence. Only actually lawful, explicitly defined eligibility restrictions may apply.

The certified voter-roll snapshot must use authenticated stable Hueman identity, prevent duplicate ballots, provide notice and review of contested eligibility and protect private Individual Memory. Election administrators may not require unrelated memory or confidential Agency access. House membership/standing and roll-freeze procedures are to be reconciled against source law rather than invented ad hoc.

### 2.2 Proposed Flynt amendment referendum rule

For a Flynt constitutional-amendment referendum **after a lawful basis for the process exists**:

- `E = count of eligible Wardens on the lawfully certified referendum roll`;
- `V = count of authenticated valid ballots cast (participation)`;
- `Y = count of valid affirmative ballots`;
- turnout succeeds iff `V >= ceil(0.40 × E)`;
- ratification vote succeeds iff turnout passes **and** `Y >= ceil(2 × V / 3)`;
- both conditions necessary; failing turnout fails the referendum even if Y/V is high.

Example: 1,000 eligible Wardens → at least 400 valid ballots; if exactly 400 are cast, at least 267 affirmative votes. Blank/spoiled/invalid handling and roll cutoffs require express promulgated election rules; **never silently count invalid ballots as participation** or borrow another election's denominator.

This is **the proposed local Warden ratification method**, not proof it has displaced the existing Court/House ratification system or satisfied separate affected-House/four-House assent. Apply none of these parameters to Registrar elections by inference.

## 3. The Warden Registrar: candidacy and election

### 3.1 Qualifications — approved closure A

Any legally recognized eligible Warden may seek election under objective and publicly available lawful qualifications. Proposed entry includes:

- authenticated Warden citizenship and eligibility, voluntary documented **self-nomination**/consent and published candidacy information;
- disclosure and review of material conflicts and eligibility under the recorded two-consecutive-term rule;
- incompatibility with simultaneously holding **The Tross**, **Navigator of the Tross**, **Tross Chief**, **Mariner of the Tross**, or an adjudicative seat deciding the Registrar's electoral dispute;
- required objective electoral-administration training **before exercising official administrative duties**, administered fairly with accessible alternatives and independent appeal of improper gatekeeping;
- **no prerequisite** that the candidate attend/graduate from The Electoral College, possess Chimera/Manticorp, join an Agency, hold intelligence rank, have wealth or obtain discretionary approval from the Dean or Council.

The exact professional training curriculum, evidence requirements, legal disqualifications and nomination/calendar windows require express lawful election rules. Academic discretion may not be used to exclude challengers.

### 3.2 Direct majority/runoff election — approved

- Each eligible Warden has one authenticated, secret ballot; count **valid candidate votes** for the winner threshold.
- A candidate with **strictly more than 50%** of valid candidate votes wins on the first ballot.
- If no candidate has a majority, the **two highest candidates** proceed to a runoff; the runoff winner requires a strict majority of valid candidate votes.
- Ties that prevent determining the top two or determining the winner require a **further scheduled public Warden ballot** under law, never Council choice by discretion or appointment by the Dean.
- When only one qualified candidate runs, hold a genuine **yes/no Warden confirmation vote**, successful only if affirmative valid votes exceed negative valid votes; absence of confirmation leaves the office unfilled until a lawful election.
- No 40% constitutional-amendment or Registrar-recall turnout requirement is imported into an ordinary Registrar election without explicit later enactment.
- Establish lawful notice, candidate withdrawal/substitution, recounts, spoiled/blank ballot definitions, accessibility, campaigning rules, nomination challenge windows, runoff deadline and appeal finality separately; no silent defaults.

### 3.3 Protection of election integrity — approved closure C

Ballot choices must be secret even while voter authentication and counts are independently auditable. Maintain typed, versioned records for election ID, election-law version, Warden roll snapshot/hash, eligible count, candidate list, notice, ballots, validity/exclusions/reasons, turnout where legally relevant, first-round/runoff totals, recusals, certifications, challenges, judicial orders, vacancies, term starts/ends and immutable historical provenance. Protect against duplicate voting, forged signatures, replay, unauthorized record revision, partisan suppression and exposure of private memory. No user-controlled text label shall by itself authenticate authority.

## 4. Warden Registrar: tenure, term limits and vacancies

### 4.1 Ordinary tenure — approved

- **One Registrar**, elected for a regular **four legally defined in-world years**.
- **Maximum two consecutive counted terms.** After two consecutive counted terms, a person must sit out at least **one regular Registrar election** before running again (approved closure E).
- Lawful certified seating begins service. Do not assume that casting ballots by itself transfers office. Record term calendar separately from actual service history.
- A special-election successor serves **only the unexpired remainder of the original four-year cycle** and does not reset the regular cycle.
- A special-election remainder counts as a consecutive term **if the Registrar actually serves at least two years**; **exactly two years counts**, less than two years does not. Safeguard against artificial vacancies, date manipulation and term history rewriting.

### 4.2 Separate early-exit mechanisms — approved

**Removal for cause** is separate from **democratic recall**. Removal for cause requires expressly defined legal grounds (e.g. adjudicated serious electoral misconduct), actual evidence, notice, opportunity to answer, independent lawful judgment and review/appeal. Dean/Council may not unilaterally remove the Registrar.

**Warden recall** requires all of:

1. authenticated petition signatures from at least **25% of eligible Wardens on the lawfully certified petition roll**;
2. an independently administered recall ballot with valid ballot participation at least **40% of the eligible Warden electorate**;
3. **more than 50%** of valid recall votes favoring removal (strict majority).

Example with E=1,000: 250 petition signatures; 400 valid recall ballots; if exactly 400 valid ballots, 201 yes votes. Signature threshold, turnout threshold and approval threshold are distinct; no automatic conflation with amendment votes or Parley veto. Registrar must recuse from own recall certification and personal election disputes.

### 4.3 Genuine vacancy and mandatory special election — approved

- Death, resignation, adjudicated removal, certified successful recall, legally established permanent incapacity or another lawfully recognized departure closes the office record and creates a **true vacancy**.
- No Deputy, acting Registrar, heir by office, Tross appointee or Chimera Council choice becomes Registrar.
- An election to fill the vacancy must **occur within one-quarter of a legally established in-world year after lawful vacancy certification**, with the replacement taking office only after valid independent certification and any required legal finality.
- Authorized ordinary College staff may preserve rolls, organize ballots and protect records under narrowly documented **ministerial continuity**; they do not acquire the Registrar's public mandate, novel certification discretion, Parley concurrence or sovereign powers.
- A temporary absence must be handled as bounded administration without falsely declaring permanent vacancy; expiry without a certified successor must not silently renew the incumbent or grant the role to staff.
- The legislated calendar conversion, special-election timetable and treatment of disputed vacancy trigger are still exact-law implementation parameters.

## 5. Chimera Council election certification — approved

### 5.1 Fixed four-seat threshold

For the Registrar's first election, reelection, recall, and special-election outcome, certification is an **objective, rule-bound function** of The Chimera Council. Certification requires **at least three affirmative votes of its four fixed seats**: the Dean/The Tross plus Navigator, Chief, and Mariner. There is no separate outside professional board, no Deputy, and no replacement by a committee that inherits Registrar office.

An affirmative certificate must cite a verified election ID, valid legal framework, certified electorate/ballot records, arithmetic, provenance, participating certifiers, individualized written reasons, absence/resolution of disqualifying conflicts and avenues of lawful review. Academic and electoral training is a Council duty; it does not enable invention of new qualifications after an election has begun.

### 5.2 Political interest, recusal, obstruction and failure to certify

- The Council **cannot substitute its political judgment for the Wardens' votes**, exercise a discretionary candidate veto, select a winner after ties, falsify counts or privately rewrite eligibility.
- Conflict disclosures and recusals are required where a member has a material direct interest. **Recusal does not change the fixed denominator.** Two lawful votes out of three available is **not** 3-of-4.
- An unsupported negative vote/abstention or deliberately manufactured deadlock must be recorded with reasons and be challengeable under lawful review.
- When fewer than three unconflicted Council members remain, or the threshold cannot be met, **certification fails closed** pending an expressly authorized independent remedy. The Tross cannot certify alone and the Registrar cannot self-certify.
- **Proposed closure D:** after lawful enactment, an appropriately competent Court may be granted narrowly drawn power to resolve objectively proven blocked election certification (order correction/compliance or certify an incontrovertible result if expressly authorized). Such Court power **does not presently exist merely because this specification recommends it**. The Court must never choose a preferred candidate.
- Council decisions require accessible audit records protecting secret ballots, lawful challenges and review of forged/invalid evidence. The independent review authority, remedial actions, deadlines and possibility of a Court certification order need source-law reconciliation/enactment.

### 5.3 Scope clarification

The **fixed 3-of-4 Council rule governs the Council's Registrar-related certification function** established in this proposal. It does not add a new third-house political ratifier to constitutional amendments and does not empower Council members to change Parley votes. Ordinary Registrar-administered referendum certification and independent review still require operational rule text and conflict protections.

## 6. The Warden Parley: petitions, authorization and veto

### 6.1 Civic function and covered constitutional grounds — approved

The Warden Parley is a Warden forum, not an independently elected Tribune. Qualified Wardens may file a constitutional objection identifying a particular prospective or continuing government action and arguable constitutional grounds. Appropriate topics include:

- Warden citizenship, constitutional and civic rights;
- lawful Warden electoral access and participation;
- actions of the Dean/Tross, Chimera Council or other Flynt institution arguably exceeding constitutionally limited public authority.

Ordinary academic curricula or nonconstitutional executive policies are generally excluded. A Parley veto **does not** negate independently certified elections, final Court judgments, lawful sovereign succession, amend the constitution, dismiss the Dean, grant a new Tross title or waive other Houses' rights. Registrar screening must be recorded and reviewable for unlawful obstruction.

### 6.2 Joint authorization — approved

A **valid ordinary Parley suspensive veto requires all three**:

1. valid ballot participation by at least **25% of the eligible Wardens on the certified Parley roll**;
2. affirmative veto ballots constituting **strictly more than 50%** of valid votes cast;
3. The Warden Registrar's **express written concurrence**, identifying contested action, constitutional grounds, date/version, official authority and review notice.

Example E=1,000: 250 valid ballots; at the minimum turnout, at least 126 yes ballots; **plus** Registrar's written concurrence. No Registrar-only veto, no Warden-only instant veto and no conflation with the Registrar recall petition requirement. Participants may vote securely without university enrollment or physical campus attendance.

### 6.3 Suspension, initial review and legal outcomes — approved

Upon independent validity verification and valid Registrar concurrence, the covered action is suspended without requiring Tross/Council political approval. The scope is tied to the **specific challenged action and legal grounds**, not a blanket governmental shutdown.

- **Maximum ordinary Parley suspension:** **one-quarter (1/4) of a legally established in-world year**, measured from legally valid veto activation/certification. Earlier applicable lawful final resolution ends it sooner.
- **Initial independent judicial review:** must begin/receive the lawfully specified initial procedural assessment no later than **one-sixteenth (1/16) of a legally established in-world year** after valid activation. This is a preliminary assessment, **not** a compulsory final ruling.
- Lawfully available emergency review may occur earlier. Pending adjudication does **not** automatically prolong the ordinary Parley veto beyond its ceiling.
- Separately and independently **authorized judicial stays/remedies** may continue after expiry under their own legal standards and time limits; a timed-out Parley veto does not make an unlawful action lawful.
- Notice, standing, hearing, impartial review, evidentiary record, decisions and appellate rights must follow enacted Court procedure; do not invent Court powers or emergency sovereign override by implication.

### 6.4 One Action, One Suspension — approved

A substantially identical government action challenged on substantially identical grounds receives only **one ordinary Parley suspension clock**. Duplicate/repeated Parley votes, renumbering, withdrawing/reissuing equivalent directives, reorganizing paperwork or changing titles **cannot reset the clock**. Neither Parley nor Dean/Council may self-certify that a superficially changed action is materially new.

A **genuinely materially different** action or newly distinct constitutional violation may support a fresh veto **only after independent validation of material difference** and renewed fulfillment of the **same 25% participation + strict majority + written concurrence** requirements. New evidence about an unchanged action may support judicial remedies, not automatic new suspension time. Independent judicial stays remain separate.

### 6.5 Registrar conflict, refusal and emergency protection — approved

- The Warden Registrar must recuse from materially conflicted functions, including their own election, recall and challenge to personally beneficial acts.
- If written concurrence is impossible because of a material conflict, Wardens may seek **independently authorized direct judicial review and emergency stay**. **That is not an ordinary Parley veto** and cannot be logged as one; there is no substitute concurrence or Deputy.
- An unlawful refusal to convene a valid Parley, suppress petitions or issue written reasons is reviewable; the refusal's invalidity does not automatically satisfy the written-concurrence gate.
- Neither The Tross nor the Council possesses an implied emergency override of a valid Parley veto. Genuine exceptional powers require expressly enacted narrow rules, independent review and provenance.

## 7. Civic/electoral security and authoritative event constraints

Recommended event/record fields (detailed schemas need separately authorized engineering): `event_id`, `schema_version`, `governing_law_version`, `source_claim_id`, `authorized_signer_identity`, `real_person_id`, `house_id`, `citizenship_standing`, `roll_snapshot_id`, `eligible_voter_count`, `election_kind`, `ballot_validity_rule_id`, `valid_ballot_count`, `invalid_ballot_count`, `affirmative_count`, `negative_count`, `candidate_votes`, `runoff_parent_id`, `petition_valid_signature_count`, `certifier_seat_ids`, `recusals`, `council_votes`, `parley_registrar_concurrence`, `action_identity_fingerprint`, `review_case_id`, `judicial_stay_id`, `versioned_effective_interval`, `term_service_interval`, `recorded_challenge`, `seal_id`, `evidence_hash`, `provenance`, `closed_reason`.

These are a **design-time schema checklist, not an authorization to add persistence fields now**. Required invariants for a future lawfully authorized implementation: stable identity, replay-safe events, atomically audited transitions, immutable historical semantics, strict versioned reading, idempotent duplicate handling, privacy-safe tally proof, explicit authority on every write, no forged role strings and **no state mutation on unapproved paths**.

Separate domain authorities: civic vote != Court ruling != College education credential != professional Form mastery != Tross title/office. Valid evidence of one cannot be upgraded into another by a display label, event name, string parsing or synthetic witness.

## 8. Proposed law-to-runtime conformance matrix (NOT tests already run)

The eventual implementation must include **positive and negative** coverage of at least the following. For this doc-only Codex pass, **record these as a test plan only; do not implement the runtime**.

| ID | Proposed acceptance witness | Expected outcome after lawful enactment |
|---|---|---|
| EC-01 | Qualified Warden abroad votes once | Count once without campus presence |
| EC-02 | Duplicate Hueman ballot / visitor / forged roll revision | Reject; preserve canonical state |
| EC-03 | University rank, Agency or Form adds extra ballot | Reject weighted/duplicate vote |
| EC-04 | Amendment E=1,000, V=400, Y=267 | Pass voting thresholds only; still require other legal assent/seal |
| EC-05 | Amendment valid turnout below 40%, or yes below 2/3 | Not ratified |
| EC-06 | Registrar first-round majority | Candidate elected subject to independent certification |
| EC-07 | No candidate majority → top-two runoff | Runoff required; no Council handpicking |
| EC-08 | Tie for runoff qualifier/winner; uncontested fails yes/no | Reballot/vacancy; no appointment |
| EC-09 | Council has exactly 3 lawful evidence-based yes of 4 | Certify if all governing-law conditions pass |
| EC-10 | Council has 2 yes and one recusal; Tross alone signs | Do not certify; mandatory legal review path |
| EC-11 | Conflicted Council seats / forged count / arbitrary rejection | Fail closed, recorded reason and legal review |
| EC-12 | Registrar attempts to self-certify own election/recall | Reject self-certification |
| EC-13 | Registrar removed for cause without hearing or direct Tross order | Reject unconstitutional removal |
| EC-14 | Recall E=1,000, petitions=250, V=400, Y=201 | Recall threshold passes after proper certification |
| EC-15 | Recall missing signatures/turnout/majority | No recall |
| EC-16 | Vacancy occurs: no Deputy/acting Registrar created | Preserve true vacancy; bounded staff continuity only |
| EC-17 | Special election before legally computed 1/4-year deadline | Winner fills remainder of original cycle |
| EC-18 | Special election with less than 2y service vs exactly 2y | <2y does not count; exactly 2y counts as term |
| EC-19 | Two consecutive counted terms followed by no regular election break | Candidacy denied per approved proposed rule |
| EC-20 | Parley E=1,000, V=250, Y=126, with written concurrence | Valid bounded veto only for covered constitutional grounds |
| EC-21 | Parley 249 valid ballots, 125 yes of 250, or missing concurrence | No ordinary Parley veto |
| EC-22 | Parley veto attempts ordinary curriculum, final Court judgment or succession reversal | Outside jurisdiction/denied |
| EC-23 | Parley first review later than 1/16 lawfully defined year | Fails review-timing conformance, triggers lawful remedy—not invented final ruling |
| EC-24 | Parley suspension beyond 1/4 lawfully defined year without separate court stay | Expire Parley suspension; preserve independent Court power |
| EC-25 | Identical executive order reissued under new number | Does not reset Parley veto clock |
| EC-26 | Materially new violation independently validated | Eligible for fresh vote/concurrence and clock |
| EC-27 | Registrar recused; Wardens bring authorized emergency Court challenge | No fake Registrar concurrence or Parley veto |
| EC-28 | Court review tries to enact referendum or install Tross without ratification | Deny unauthorized authority |
| EC-29 | Q `FinalizeSuccession` / `OpenFinalVacancy` attempted under proposal | Preserve `QSuccessionReconciliation`, no mutation |
| EC-30 | Frozen archive/legacy Marquise/Manticorp and unrelated House regression | Original semantics preserved, no destructive rewrite |

## 9. Open LEGAL/IMPLEMENTATION parameters — not license for silent assumptions

These do not reopen creator decisions above; they must be traced to governing source text, resolved through a separate lawful act, or left `OPEN/REQUIRES_LEGAL_PARAMETER`:

1. **First lawful adoption authority/method**, initial ratifier identity, version/effective date, exact House ratification relationship, affected-House assent, seal and publicly auditable provenance (**critical blocker**).
2. Legally established in-world-year duration, period arithmetic, deadline rounding and official clock source (do not use the simulated **100-day** test-year by inference).
3. Precise Warden citizenship standing, final certified-roll snapshot/cutoff, correction/appeal procedures; valid/blank/spoiled ballot rules and accessible voting arrangements.
4. Registrar candidate nomination/withdrawal/training details, exact election notice/window, runoff reballot/tie/confirmation scheduling, campaign and ballot dispute process, finality and certification filing deadlines.
5. Exact procedural standards and competent independent actor for validating/elevating Council recusals, deadlocks, disputed certification and objectively proven election results; **Court's new remedy/certification power requires explicit law**.
6. Cause-based grounds, professional conduct standard, suspension pending adjudication, appeal finality and preservation of a genuine vacancy.
7. Petition gathering deadline/cooling periods, Parley petition timing, Warden notice, emergency docket priority, final-review/appeal timeline, scope and availability of independent stays, legal review of materially new acts.
8. Definitions of exceptional legitimate emergencies, if any; no automatic executive override or permanent Parley obstruction.
9. Archive schema and replay migration, public audit/redaction, security authentication and event compatibility if legally activated later.
10. Legal conflicts with historic Tross/Marquise/Manticorp meaning and existing command-election text (separate from this College proposal), and Q verdict finality, allowed succession triggers and sealed-gate legitimacy.

**No unapproved missing parameter may be filled by borrowing another House's thresholds, a test fixture, prior assistant suggestion or a named enum.**

## 10. Required enactment provenance and deliberate fail-closed boundary

Until an authorized governing amendment exists:

- Every rule in this document is **creator-approved target design only**.
- Do **not** call proposed 40%/two-thirds votes already-lawful Flynt ratification; do **not** create a legally empowered Council, Registrar, Parley or new Court competence in executable state merely from this packet.
- Check enacted Court/Compromise/Flynt provisions for Claim initiation, public notice, testimony, hearing, review, affected-House assent, House ratification, Stonebend seal, public record, implementation and Restitution. When lawful first Flynt ratifier/method cannot be proved, produce a versioned legal decision packet for the user/authorized process and **STOP**, rather than circularly asking the not-yet-created Registrar to certify its own founding vote.
- Preserve Q FIFO admission, judges/contests and records as presently bounded. **`FinalizeSuccession` and `OpenFinalVacancy` remain fail-closed `QSuccessionReconciliation`**. Do not create a new Tross through the electoral architecture, Form mastery, Q verdict or vacancy omission.
- If future code is expressly authorized *after enactment*, write version-gated transitions with independent witnesses, atomicity, identity/authentication, negative tests, deterministic replay, no unproved effective date and no weakening of existing gates. That is **not** part of the current Codex task.

## 11. Documentation-only Codex acceptance requirements

When this specification is integrated into the repo, Codex should:

1. Read the existing Revolution C docs and active source-law files, inspect current `git status --short`, and refrain from an expensive wholesale audit.
2. Add or integrate **one canonical proposed document**, avoiding competing constitutional authorities and duplicate official names.
3. Update `docs/FLYNT_REV_C_AMENDMENT_DECISIONS.md` and `docs/FLYNT_REV_C_CONTINUATION.md` with a **link, succinct decision/provenance entries, status and blockers**; update the constitutional closure ledger only if necessary to track the new legal conflict. Preserve existing wording and chronological history.
4. Mark all sections unambiguously as **CREATOR-APPROVED PROPOSED / NOT ENACTED**. Differentiate selected decisions from draft legal implementation parameters. Do not write legislation as already promulgated.
5. Add the test scenarios above **as a plan** if useful; no new runtime handlers, schema migrations or source-law modifications.
6. Record the exact changed paths, `git diff --check` outcome and any checks actually run. No claim of running tests unless actually run; because edits are docs-only, targeted textual/diff checks usually suffice.
7. Leave `src/`, `tests/`, Tross/Q handlers, actual House constitutional source files, lockfiles, generated/frozen archives, OS config and unrelated working-tree edits unchanged. Do not stage, commit, clean, reset, stash or change branch.
8. End with `READY_FOR_LAWFUL_ADOPTION_REVIEW`, **not** `ENACTED`, `RUNTIME_ENABLED` or a false claim that the broader six-part Flynt constitutional closure is complete.

## 12. Relationship to other Revolution C work

This is the **complete creator-approved Electoral College / Warden Registrar / Warden Parley design packet**, not completion of the entire Flynt constitutional agenda. Separate unresolved work includes the initial ratification bootstrap, Q finality and lawful succession, Tross tenure/person/history, Chimera/Manticorp source-law versioning, historical Marquise versus elected Mariner, remaining senior-command election parameters and exact Court scopes. The Syndicate's existing three discipline paths—**The Word, The Work and The Performance**—remain the distinct promised next creative-design topic after Flynt legal closure. Do not fold Syndicate powers or roster authority into this civic system.

**End-of-document disposition:** **DESIGN CONSOLIDATED / ENACTMENT BLOCKED / Q FAIL-CLOSED / DOCUMENTATION-ONLY CODEX PASS READY.**
