# Backend House Handoffs V1

Scope: ten existing historical route contracts, implemented as headless backend
services. No art, scenes, controls, new product choices, biological mechanisms,
offices or canonical ordinary NPCs are adopted by this implementation.

The owning route definitions remain `src/world/lived_lore.rs` and their House
constitutions. `src/world/lore_handoffs.rs::HandoffService` now owns executable
handoff records, independent authority checks, typed consequences, attribution,
delivery and replay. `tests/backend_house_handoffs.rs` supplies private witnesses.
The generic recursion and semantic kernels retain their existing scope.

## What closes each backend gap

| Existing route | Required facts | Scoped executable consequence |
|---|---|---|
| Flynt Stairway | performed work, contribution attribution, bounded function | recognized function, retained contractor practice or attribution review; no office |
| Flynt Basin | continuing cargo, custodian, both institutional scopes, contraband uncertainty | selected urban/rural expression or held review; both scopes and one Tross retained |
| Stonebend public Name | former Names, requested reference, earlier liabilities | provisional public reference, retained Name or pending challenge; no deleted history |
| Stonebend illegal Hollowing | distinct living subject and extracted sample, separate custody, missing-custody history, restoration uncertainty | protected custody and scoped emergency seal, investigation or refused inspection; no destructive authorization |
| Sandmanor public design | versioned shelter, interior/exterior observations, tested conditions, negative history, maintenance uncertainty | proof within conditions, revision required or rejected claim; failure never becomes a positive proof |
| Sandmanor recovery design | laboratory result, moving-route failure, revision and fresh moving result, subject capacity | revised method, laboratory-only claim or stopped testing; separate clinical Clear precedes applicable Prove |
| Sandmanor coastal role | independent original/reproducing bodies, actual reproduction result, weather/test conditions | reproducible regional role, wider reproduction request or preserved Satyr practice; no Centaur transformation |
| Glaüshouse emergency intake | capacity status, immediate threat, bounded stabilization scope | stabilization, refused elective intervention or Nightingale stop; emergency never implies elective consent |
| Glaüshouse high-risk clearance | capacity, separate proven claim, uncertainty about reversal, explicit conditions | conditional clearance disposition, reversible alternative request or refusal; no Synthesis procedure performed |
| Glaüshouse recovery discharge | limited travel result, duty restrictions, full-load uncertainty, continuing recovery plan | restricted transfer, continued recovery or refused receiving conditions; no full-duty clearance |

Each of the three canonical alternatives is preserved verbatim in the result.
An accepted House decision means acceptance of the scoped disposition record;
it does not turn a rejected design into successful proof or a refused procedure
into permission. Each typed consequence represents only its contract's scope.

## Actors, authority and evidence

Every service receives explicit executor, independent witness/recorder,
recipient and recovery-owner IDs. Executor, witness and recipient differ.
Living subjects receive their own disposition. Cargo/design identity is separate
from the human recipient. These bindings are case inputs, not assignments of
the source's ordinary role labels to new canonical people.

The existing institutional catalog authenticates a requested House decision
against an active holder, office, jurisdiction and required authority through
`HouseDecision::from_catalog`. Stonebend Name, Sandmanor Prove, Glaüshouse Clear
and Flynt Recognize remain independent. The decision references the exact
evidence revision and tested conditions. The authority owner cannot double as
the case's ordinary executor, witness or recipient. Vacant authority fails closed.

Clinical clearance and design proof are separate records. A high-risk positive
clearance requires an independently recorded Prove disposition first. Applicable
recovery-design Prove requires independent Clear first. Unknown or unable
capacity cannot become consent to the positive elective alternative.

For an urgent intake with uncertain capacity, the bound emergency operator may
request stabilization only. Clear still requires its live authority. The result
records `consent_recorded: false` and never authorizes an elective intervention.
A Nightingale stop belongs to the intake's independent witness. Clinical review,
further treatment, qualifications and larger outcomes retain their source owners.

The backend records attributed observations supplied by its caller. Actor IDs
do not authenticate a remote user, and reported observations are not sensor
truth or independent certification of a clinician's competence. An application
adapter must bind its authenticated caller to these actor IDs and supply the
actual qualified-worker/evidence records. Private fixtures demonstrate behavior;
they establish neither a real-world clinical outcome nor a new NPC roster.

## Workflow and failure

Record evidence → actual recipient choice → required independent House decisions
→ executor records work → independent witness accepts its exact scope → actual
recipient acknowledges delivery. These are separate events. Delivery before
observation, substituted actors, wrong functions and missing prerequisites fail
without mutation. The receiving-worksite witness must accept the limited scope
before a recovery transfer can be delivered; travel does not authorize full duty.

A changed evidence revision clears earlier choice and decision eligibility and
requires fresh decisions. Prior events remain attributable. Earlier negative
results cannot be removed. Public Name revisions cannot erase former Names or
liabilities. A revision cannot quietly change work that has already been performed.

Subject withdrawal may stop the case before evidence or after work. After work,
recovery remains required. The recovery owner records protected return, an
independent witness observes it, and the actual recipient receives the stopped
disposition. Earlier work, decisions and failures remain in history. A completed
handoff closes its scope; it does not erase continuing care or wider disputes.

## Persistence and compatibility

`hollow-grove-backend-handoff`, schema **1**, is a new, separate archive for an
actual headless case. It stores the source route, subject, actors, validated
institutional snapshot and full ordered action/result history. No historical
game or catalog save is reclassified as a completed handoff. Game archive V6,
functional-lore catalog V2, HGSYM1 and original save files retain their formats.
There is no older handoff schema to migrate; unsupported schemas are rejected.
Future changes require explicit versioned migration rather than invented history.

Loading verifies the checksum, rebuilds through the same service and compares
every recorded result with replay. Unknown fields, unsupported formats, duplicate
events, causal disorder, mismatched authority and changed consequences fail
closed. Recomputing a checksum cannot make an inconsistent result pass replay.
The checksum detects corruption; it is not a signature or proof against replacing
the entire archive and its trusted institutional context.

Identical request-ID retries return the original event without repeating work.
Conflicting reuse fails without mutation. Execution clones bounded current state,
not the full prior event journal. Each case has at most 32 evidence revisions;
evidence lists are bounded and scalar text is limited. The historical journal is
retained in full; encoding and replay necessarily scale with retained history.

## Inspection and evidence

Architecture inspection schema **3** reports **23 components**, **14 combined
backend/gameplay bindings**, **ten headless handoff bindings** and **zero unbound
canonical backend routes**. The four earlier gameplay bindings remain; the ten
new route services have **no game presentation integration**. This is separately
reported as `unbound_gameplay_route_count: 10`. It is outside this backend pass.

Source inspection reports registered implementations and proof-file links. It
does not pretend to execute their tests or complete an unsupplied case.

```sh
cargo test --test integration backend_house_handoffs
cargo run --bin hollow-grove -- architecture --json
cargo run --bin hollow-grove -- architecture --json --handoff PRIVATE_CASE.json
```

`--handoff` reads and replays without modifying the supplied archive. Only the
supplied route receives a completion result; other routes remain `null`, rather
than inheriting the result. `--save` separately inspects an existing game archive.
Neither inspector changes source, authority, people, products or saved progress.

The private witness suite covers all **30 canonical dispositions**, protected
refusal before/after work, witnessed recovery, stale evidence, negative-history
retention, live/mismatched authority, separate proof/clearance, unknown capacity,
bounded emergency care, retries, conflicts, causal order, archive replay,
altered-result rejection and read-only CLI inspection. Tests use anonymous
private fixture actors and institutional holders, never production save state.
