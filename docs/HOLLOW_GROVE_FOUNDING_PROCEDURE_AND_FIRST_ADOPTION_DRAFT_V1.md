# Hollow Grove founding procedure and first adoption — draft V1

**FC03_FC06_DRAFT_READY / NEW_TERMS_PROPOSED / NOT_ENACTED**

Prepared 27 October 2026. This supplies a concrete procedure proposal for
FC-03 through FC-06 of the [founding compact](HOLLOW_GROVE_FOUR_HOUSE_FOUNDING_COMPACT_DRAFT_V1.md),
alongside the [civic-roll and administration schedule](HOLLOW_GROVE_FOUNDING_CIVIC_ROLLS_AND_ADMINISTRATION_DRAFT_V1.md).
It does not claim completed source-law closure, appointments or adoption.

## Decision carried forward

The creator's **“yes go”**, following the checkpoint identifying the two
administration recommendations, is carried forward as authorization to proceed
with those recommendations as the drafting baseline:

- One ballot in each House where a person's civic eligibility is separately
  established; one person never receives two counted ballots in the same House.
- Named temporary coordinators, independent verifiers and alternates receive
  limited authority from an explicit prospective founding-origin warrant,
  effective before official roll certification or ballot issuance.

This records acceptance of that design direction, not actual appointments or
approval of unseen clauses. The four-House origin model and founding thresholds
remain settled design: all four Houses separately pass, with `E > 0`,
`V = Y + N`, `0 <= V <= E`, `2*V > E` and `3*Y >= 2*V`.
The new timing values, remedies, source additions, effective-event wording and
finite bridge below are **recommended proposals for review**.

## FC-03 — notice, ballot and review procedure

### Before any ballot

The completed origin instrument must expressly authorize the preparatory rules
in FC-01/02 and this FC-03 schedule. Their competence must exist before use;
placing them only in the compact being voted upon would leave administration
self-authorizing. The warrant supplies temporary administration, not advance
assent or new sovereign, judicial or sealing powers.

Publish one process identifier and a frozen ballot package: exact compact
text and incorporated annexes, source additions and effects, House scope,
eligibility rules, staff mandates, authenticated calendar binding, notice and
filing access, ballot-validity rules, remedies and prospective effective
conditions. Provide accessible notice to all four civic communities and
affected parties, including both Sandmanor traditions. Provide private access
to individual eligibility evidence without publishing clinical information.

Follow the shared sequence: proposal, public notice, affected testimony,
conciliation where appropriate, first procedural hearing, appeal where
challenged, constitutional review, proper ratification, Stonebend seal/public
record and implementation/Restitution review
(`MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md:315–376`). Pre-ballot
review must resolve standing, scope, improper bundling, material roll defects
and authority defects before opening. Later tally challenges receive their
own disposition before final certification and effectivity.

### Founding-only time binding — proposed

The canonical year runs from one exact Winter Solstice to the next;
observations, suppliers and evidence are distinct from civic celebrations
(`CENTRAL_JUNCTION_SEASONAL_FUNCTIONS_V1.md:25–29,45–73`). That supplies
chronology, not the missing election deadlines.

**Recommended rule:** bind this process to the most recent completed,
authenticated canonical year available at the origin warrant's effective
event. Record its year ID, opening/closing observations, source version,
supplier and evidence. Let `L` be the exact elapsed SI seconds between those
instants. Keep that reference length fixed throughout this process; a later
solstice does not move a published deadline. Missing, disputed or invalid
reference evidence blocks scheduling; staff cannot select a convenient older
year or substitute a fixture or fixed Gregorian year.

This is a new founding-interval rule, not a redefinition of the canonical year.
It does not decide four-/six-year terms or the later Flynt Parley clock. Each
later enabling ballot needs an explicit binding in its transition warrant.
Historical timestamps and annual archives remain unchanged.

Name one authenticated time authority and a versioned conversion from source
instants to a continuous elapsed-second scale, including leap-second treatment
and real calendar-date validation. Bind exact integer-second instants, retaining
source values and provenance; do not silently round subsecond evidence.
Local process time, file dates and unsynchronized clocks cannot govern
(`HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md:5679–5694`).

For a minimum fraction `p/q`, use `ceil(p*L/q) = (p*L + q - 1) div q`.
For a maximum use `floor(p*L/q)`. Use exact integers. Activity windows are
`[start, end)`: opening is included; closing is excluded. A record at the
closing instant is late. The receiving service supplies an authenticated
receipt instant and preserves failed-access claims for review; filing latency
must not silently shorten access.

| Stage | Recommended interval and trigger |
|---|---|
| Public notice to eligibility cutoff | At least `ceil(L/16)` after authenticated publication and accessible service of the complete package. |
| Roll correction | At least `ceil(L/32)` after cutoff. Freeze also requires completed eligibility review; time alone does not dismiss a case. |
| Frozen-roll and final opening notice | At least `ceil(L/32)` after certified roll publication and service of the final package. Required hearings/review must also be complete. |
| Voting | Exactly `ceil(L/16)` from the announced opening, unless a competent bounded order interrupts the process. |
| Provisional tally publication | Within `floor(L/32)` after close. Missing the target is a reviewable administrative failure, never a deemed pass or NO vote. |
| Administrative result objections | At least `ceil(L/32)` after authenticated publication and accessible service of the tally, reasons and audit access. A material corrected tally starts a fresh window. |

Use one announced opening and closing for all four founding House ballots;
opening awaits readiness of all four. Tallies are published only after close.
Longer mandatory protections control. Publish actual instants before use.
This draft sets no new judicial appeal deadline: identify the applicable
appeal basis, service and deadline in the process record. If no competent rule
supplies closure, resolve that gap in the final source package; the
administrative objection period cannot extinguish an available judicial appeal.

Silence or an expired administrative target supplies no judicial certification.
Existing appeals review error and do not automatically stay judgment
(`MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md:193–199`). A competent
stay records scope, reason, start, termination and review (`:278–290`). Only an
express lawful order may pause or replace a window. Preserve original/revised
deadlines, elapsed/remaining time and notice; renaming a process never resets
its clock. Staff cannot revive an expired order or silently reopen a ballot.

**Implementation gap:** `AuthoritativeTimestamp::new` checks string shape;
year validation checks ordered boundaries, observations and evidence references
(`src/constitutional/canonical_calendar.rs:96–115,206–240`). These checks do
not establish real date validity, elapsed-second arithmetic or an authenticated
civic deadline policy. No calculator or runtime timing verification is added.

### Ballot validity, counting and remedies — proposed

1. Each eligible voter receives one secret YES/NO question on the exact frozen
   package. Issuance verifies House eligibility against the certified roll.
   Keep choice and voter identity in separate records.
2. A countable ballot is authenticated as authorized, unique and received in
   the voting window, with exactly one unambiguous YES or NO under the
   published marking rule. Record blank, spoiled, ambiguous and unauthorized
   ballots separately with reasons; exclude them from `V`, never from `E`.
3. Replace a lost/spoiled credential only after its unused predecessor is
   provably cancelled without revealing a choice. An accepted ballot allows
   no second vote or administrative change of choice. Preserve conflicting
   apparent uses for independent review; do not count both or choose by result.
4. Reconcile issuance, cancellations, used credentials, accepted choices and
   exclusions. The coordinator prepares the tally; the independent verifier
   checks original evidence and exact arithmetic. Record disagreements for
   competent review; do not average incompatible counts.
5. Objections identify standing, act, grounds and requested remedy. Provide
   notice, privacy-preserving evidence access, response and reasoned disposition.
   Staff may correct clerical errors within their mandate; disputed eligibility,
   competence and constitutional effect require lawful review. No person
   decides their own challenged mandate, conflict or conduct.
6. A recount uses preserved original evidence and an independent verifier.
   Record old/new tallies and reasons, recertify and provide a fresh objection
   period. It does not change text, denominator, cutoff or voter intent.
7. An incurable defect requires a reasoned competent remedy determination.
   An uncertifiable ballot supplies no assent. Review may require a fresh
   ballot; it cannot invent YES votes, shrink the roll or backdate a certificate.

**Repeat rule proposed for the compact:** a failed required House result,
required reballot or material package change closes the attempt after lawful
disposition. A new attempt requires a newly effective origin warrant, fresh
notice/roll review and all four House ballots. Preserve previous assents as
history; do not carry them into the new attempt. Clerical correction or recount
of the same lawful ballots is not itself a new attempt. There is one all-House
adoption record per attempt, without indefinite retention of three approvals.

## FC-04 — exact source relationship and proposed additions

The Compromise is the shared Constitution; House domains remain protected,
and runtime law is subordinate within its delegated domain
(`HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md:9–49`). These proposed **additions** delete
no existing sentence and require two separate effective source versions. The
reserved identifiers below name proposed final instruments, not existing files
or effective law.

| Instrument | Competence and order |
|---|---|
| `HOLLOW_GROVE_FOUNDING_ORIGIN_V1` | Original constituent declaration, named warrant and finalized FC-01/02/03 preparatory rules. Its authenticated public effective event precedes official administration. It supplies no advance compact assent. |
| `HOLLOW_GROVE_FOUR_HOUSE_FOUNDING_COMPACT_V1` | Exact compact and frozen annexes. Substantive first-adoption competence begins only at the separate FC-05 event after four valid ballots and all remaining conditions. |
| Named enabling instruments | Separate proposals, ballots, review, required concurrent acts, seal and public effective events under FC-06. The compact does not enact their text or appoint their officers. |

**Addition A — origin source version:** insert after the Compromise Authority
Posture paragraph ending “explicit constitutional reconciliation and executable
migration.” (`HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md:28–30`):

> The prospective Hollow Grove Founding Origin V1 records the original
> constituent authority of the four existing lawful civic communities and
> supplies only the bounded preparatory authority in its effective warrant
> and procedural annexes. It does not stand as a House's compact assent,
> confer sovereign office, replace the Court or transfer sealing authority.
> Its exact version and public effective record govern commencement; it has
> no retrospective operation.

**Addition B — compact effective source version:** immediately after A, only
upon the compact's own lawful effectivity:

> The Hollow Grove Four-House Founding Compact V1 supplies the finite
> first-adoption authority expressly identified in its effective annexes.
> Each House's required assent, shared Court review, lawful Stonebend sealing
> and public effectivity remain separate acts. The compact alters only the
> competence it expressly states; it does not silently supersede House law,
> adopt an enabling proposal or authorize runtime execution.

**Addition C — shared procedure link:** append to the Court source's
Ratification scopes subsection, after the paragraph ending “None substitutes
for another.” (`MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md:356–358`),
in the compact effective source version:

> For the first enabling instruments exhaustively identified by an effective
> Four-House Founding Compact, its limited civic assent method is an additional
> authorized House process. Scope review still determines every affected House;
> existing required concurrent acts remain necessary unless an exact effective
> clause expressly reconciles them. Neither the Court nor a seal supplies a
> missing civic assent. Exhaustion of the compact's bridge ends that method;
> it creates no permanent alternative amendment procedure.

These additions do not change merits jurisdiction, select judges, add appeal
remedies or resolve Stonebend's executive-appeal conflict. No source constitution
is edited here. Adoption needs exact added text, source versions/digests and
incorporation records; a mutable or future annex reference is insufficient.

## FC-05 — seal, record and public effectivity

**Proposed compact effective clause, completing the earlier placeholder:**

> This compact becomes effective at the authenticated instant of its first
> complete public constitutional-record publication after all four separate
> House assents, required review and challenge dispositions, Court procedural
> certification and lawful Stonebend sealing have been established. Publication
> must bind the exact adopted compact and annex versions, origin warrant, all
> prerequisite records and authoritative time binding. Incomplete or premature
> publication has no activating effect. A later valid publication is a new
> prospective event, never a backdated cure.

The publication service records the event; it cannot waive prerequisites.
Independently authenticate custody, publication and the prerequisite chain.
The origin warrant's earlier administrative event has distinct authority and
cannot stand as the compact's later adoption event.

The seal binds correct decision, subject, authority, scope, record and sequence
(`STONEBEND_CONSTITUTION_V2.md:283`). Independently confirm the lawful bearer
and single active seal; neither the ballot nor a future seal-transfer amendment
may validate its own bearer. Respect the vacancy restriction on permanent
amendments (`:119–123`). An unresolved vacancy or disputed seal is a dependency,
not authority to invent a temporary sovereign or substitute staff signature.

Retain each prerequisite's identity, exact source version, actor/mandate,
authenticated act, time/order, evidence, challenge status and custody. The
public manifest binds four separate roll/tally/assent records, Court
certification, lawful appeal closure, seal provenance and final text. Keep
restricted identity evidence separate from secret-choice audit. A digest or
unverified signer string proves no underlying act.

An empty inbox supplies no finality. Identify applicable challenge/appeal
closure rules and dispositions. Stonebend's finality/reopening rule remains
explicit (`STONEBEND_CONSTITUTION_V2.md:333`); later fraud or corrupted evidence
requires competent review while preserving published history. No silent edit
to an effective record may substitute.

Effectivity permits only the granted legal competence. Implementation still
needs authorization, faithful source binding, compatible migration and
verification. Restitution compares adopted and implemented text
(`MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md:360–376`).

## FC-06 — finite first-adoption bridge

**Recommended bridge:** use the founding `>50%` participation and `>=2/3` YES
rule separately in every House whose assent is required for a named first
enabling instrument. This is a new transitional proposal. Flynt's later local
amendment design remains 40% participation and two-thirds YES, usable only
after lawful enabling effectivity
(`docs/FLYNT_ELECTORAL_COLLEGE_WARDEN_PARLEY_PROPOSED_CONSTITUTION_V1.md:73–86`).

Before the compact ballot, its frozen manifest must exhaustively name eligible
enabling text/versions and separately voted parts. Recommended candidate
families are:

| Candidate | Permitted subject; exact final versions still required |
|---|---|
| Stonebend reconciliation | Five SB-04/06/07 targets in the [clause schedule](STONEBEND_SOURCE_RECONCILIATION_SCHEDULE_2026-10-27.md): professional selection/shared recognition, executive/judicial review, Trial and Acclamation/shared route conflict. Settle remaining nomination, recognition, vacancy and remedy choices first. |
| Flynt civic enabling | The [College/Registrar/Parley proposal](FLYNT_ELECTORAL_COLLEGE_WARDEN_PARLEY_PROPOSED_CONSTITUTION_V1.md), with express first-election administration and certification. Exclude sovereign succession and Q gate waivers. |

Court scope and improper-bundling review precede freezing the manifest. A
family permits no unlimited later sub-instruments. Enumerate any necessary
separate parts/dependencies before the compact vote. Shared-file location
alone does not decide scope: local needs that House, cross-House needs each
affected House, foundational needs all four
(`MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md:333–358`).

The final compact delegates only administration of those listed ballots to
named transition staff/alternates in its annex. Mandates begin after compact
effectivity. Include independent roll/tally verification, conflicts,
substitution, new notices/rolls, exact time binding, review and closure.
Founding staff do not automatically continue. No future Registrar certifies
their own first election; no future Council acts before its authority exists.

**Stonebend concurrence recommendation:** require both applicable civic assent
and a separate lawful Hypergiant ratification record for the Stonebend enabling
instrument while that office is lawfully occupied. This proposed bridge
condition preserves the existing lawful-amendment role
(`STONEBEND_CONSTITUTION_V2.md:91`) and vacancy restriction (`:119–123`); it
does not assert that the current sentence supplies a complete mandatory voting
method. The Hypergiant cannot replace the civic vote, another House's assent
or the seal. Disputes receive independent Court review within existing
competence; no power to compel favorable assent is added. This adds no
executive vote to the accepted four-civic-ballot founding rule.

**Exhaustion:** each listed instrument gets one balloting attempt under this
bridge, including lawful correction/recount of its original ballots. Successful
effectivity consumes its authority; a failed required House vote, final
invalidation requiring a new ballot or withdrawal closes it without adoption.
No retry, material text substitution, new instrument or transfer of unused
authority is implied. Unused balloting authority expires at
`compact_effective_at + L`, using the compact's pinned reference length. No new ballot may
open or remain open after that deadline; publish windows ending no later than
expiry. Final verification/review of ballots already closed may finish after
expiry, preserving records/custody; it cannot authorize fresh voting.

A stay does not extend the bridge's substantive grant. Renewal/expansion
requires a separately competent prospective source and adoption process, not
staff discretion or procedural certification. Exhaustion/expiry ends
administration except express review/custody duties. A new ordinary amendment
process becomes usable only after its enabling source is effective.

The manifest must state the ongoing amendment powers each enabling instrument
actually creates. The Stonebend conflict schedule alone is not a complete new
permanent amendment code. Remaining powers cannot be inferred from this bridge.

## Completion record and next drafting work

| Item | Current result | Still required before use |
|---|---|---|
| FC-01/02 | Two recommendations carried forward as selected drafting direction; detailed schedule written | Final wording, actual civic evidence, named consenting staff/alternates, mandates, lawful reviewers and seal continuity. |
| FC-03 | Concrete timing, ballot, objection/recount and repeat proposal | Approved effective origin rules, authenticated time/dates, judicial appeal closure and actual procedural records. |
| FC-04 | Exact additive targets and two-stage source relationship | Final source versions, incorporation and authentic effective records. |
| FC-05 | Public effective-event rule and evidence requirements | Four assents, review, challenge closure, lawful seal and actual prospective publication. |
| FC-06 | Named finite bridge, thresholds, transition mandate and expiry | Approved terms, frozen enabling parts/scope, actual staff and separate adoption of each instrument. |

Next: settle the marked Stonebend clause choices, then complete Flynt's
first-election administration and the exact enabling manifest. Those source
choices must be resolved before the compact is ready for a real founding
ballot. Do not restart the selected original-constituent-model question.

No electorate, appointment, vote, Court decision, seal or public event is
fabricated. **BLOCKED_NO_INITIAL_ADOPTION_AUTHORITY** remains the activation
status while the origin design is selected. `FinalizeSuccession` and
`OpenFinalVacancy` retain `QSuccessionReconciliation`
(`src/world/flynt/q.rs:528–531`). The single final mint is **NOT_ELIGIBLE**.

This pass changes planning documentation only. No runtime tests, elections,
source-law edits, migrations, commits or issuance occur. Focused document and
preservation checks are recorded in
[the continuation](FLYNT_REV_C_CONTINUATION.md#founding-procedure-and-first-adoption-draft--27-october-2026).
