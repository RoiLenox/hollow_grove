# The Syndicate — next Revolution C design phase

Status: **DESIGN PREPARATION; nonexecuting title catalog only**.
Prepared after 37 targeted Flynt tests and strict Clippy passed. This does not
authorize Q succession, new Form anatomy, Incantation effects, fabrication or
protected-data access.

## Approved design

Sources: `/home/warren/Downloads/HOLLOW_GROVE_REV_C.md:218–226` and
`/home/warren/HOLLOW_GROVE_REV_C/HOLLOW_GROVE_MASTER_CANON_WORKING_REV100_TUI_HANDOFF.md`,
§6.5 (1434–1446). Creator-approved design is distinct from implemented mechanics
and enacted rights; source-era meanings remain preserved.

| Discipline | Rank progression | Method theme | Senior counterpart |
|---|---|---|---|
| **The Word** | Warlock → Wizard → Sorcerer | Instructions and formulation | Navigator of the Tross / Sorcerer |
| **The Work** | Wiccan → Witch → Sorceress | Conditions and materials | Mariner of the Tross / Sorceress |
| **The Performance** | Mage → Magician → Mime | Perception, mimicry and performed action | Tross Chief / Mime |

Exactly three disciplines and three lead figures: Sorcerer, Sorceress and Mime,
in an unstable collaborative/competitive relationship. No fourth boss exists.
The Syndicate is separate from Mystery Men, We Fairy Men and Gallows; it is
not a fourth Face, fifth House or alternative Tross government. Counterpart
pairings describe rivalries, not guaranteed defeats or damage multipliers.

Mage belongs exclusively to the active Syndicate design. Rank, physical Form,
qualification, Agency, office, licensed work and ownership stay distinct.
Titles do not automatically issue credentials, membership or authority.
Gendered folklore terminology adds no unapproved eligibility restriction.

## Decisions required before code

| Decision | Required specification | Current boundary |
|---|---|---|
| Each of nine ranks | Any credential, Agency specialization, or physical Form beyond its title? | Rank titles only; no credential, Agency or Form is implied or assigned. |
| Anatomy/manifestation | Which practical folklore Forms exist, their anatomy, prerequisites and continuity within one expressed body? | No new transformation, anatomy or physical effect. |
| Incantation semantics | Allowed inputs, state changes, costs, duration, limits and verifiable outcomes for each Recipe. | No successful execution placeholder or implicit capability from a theme. |
| Prototype Recipes | Retain/revise/reject False Directive, False Wake and Silent Double; approve their discipline mappings and mechanics. | Three unratified names only; no inferred mappings or effects. |
| Advancement/leadership | Whether advancement is recorded; whether lead titles are earned ranks, leadership tenures or both; lawful exit/history. | No advancement event/evidence or tenure is implemented; do not copy command elections, terms, thresholds or add a fourth leader. |
| Membership/Agency | Affiliation versus professional assignment and compatibility with House status/one active Agency. | No automatic House admission, parallel Agencies or license. |
| Illicit activity/response | Which in-world acts are prohibited, admissible evidence, and existing investigative/adjudicative/remedy authority. | No invented Court power, collective guilt or unrestricted infiltration/private-memory access. |
| Historical migration | Approved stable identities and explicit treatment of old Warlock/Lycan/practitioner meanings. | No global renames or invented credentials from old titles. |

The general Folklore-to-Form principle is approved. It does not establish that
every rank has a separate physical body or executable ability. The three
methods create no fifth energy and bypass no material accounting. Later
clinical/material procedures need independently validated scope, consent and
qualified execution.

## Narrow source review for the next phase

- `FLYNT_CONSTITUTION_V2.md:228–240` retains Warlock/Gargoyle, Lycan/Werewolf
  and Mariner/Merman grammar. Preserve historical meanings; no new Syndicate
  credential follows from that vocabulary.
- `officials-and-outlaws/src/lib.rs` and `src/world/flynt.rs` retain historical
  hierarchy/identity definitions. No replacement is authorized by this brief.
- `src/world/flynt/governance.rs` separates ranks, qualifications, offices and
  body expression. Preserve those boundaries without granting the Syndicate
  elected command or sovereign authority.
- `src/synthesis_recipe.rs` and `src/synthesis_execution.rs` have bounded
  execution contracts, not a general Incantation engine. Compare a fully
  approved Recipe to the actual dispatcher before proposing integration.

No whole-repository audit is needed. The approved identity and classification
contract has now been implemented as a nonexecuting, versioned title/provenance
catalog. This does not authorize unapproved Forms, effects or gameplay
integration.

## Focused implementation comparison

The focused source scan found no runtime rank/catalog entries for the nine
approved titles in the inspected Flynt governance, succession, synthesis
Recipe/execution, and world-bible paths. `officials-and-outlaws/src/lib.rs:160`
and `:174` use `CriminalSyndicate` only as a generic institution category;
that variant does not instantiate this organization or its membership.

Two title words already have historical enum meanings:
`officials-and-outlaws/src/lib.rs:228–236` defines `PractitionerTradition`
variants `Warlock` and `Lycan`, and `:1401–1431` assigns them to the legacy
Gargoyle and Werewolf lineages. `FLYNT_CONSTITUTION_V2.md:228–240` preserves
that same historical mapping. This does not make those variants Syndicate
ranks, credentials or Forms. Keep the old identifiers and meanings intact;
do not reuse them as rank IDs or infer migration from spelling. Wizard,
Sorcerer, Wiccan, Witch, Sorceress, Mage, Magician and Mime have no matching
representation in those inspected runtime paths. This is a scoped scan, not a
claim about every repository file.

Consequently, there is no legally supported gameplay integration to add yet.
The approved rank names are now represented by the static, nonexecuting
title/provenance catalog documented below. That catalog implements only the
classification and stable-identity contract; it does not create membership,
eligibility, abilities, physical Forms or Incantation execution. No additional
runtime representation is supported until those separate decisions are
approved.

## Recipe executor compatibility review

The existing generic Synthesis Recipe path is a useful comparison boundary,
not an Incantation implementation:

- `src/synthesis_recipe.rs:57–70,123–167` represents four intents (Prism
  delta, learn Flow, learn Glow, change Frame), validates nonempty ID/name and
  intents plus faculty manifestations, then compiles those intents directly
  into scripts. It has no target authorization, resource/cost ledger,
  participant-consent check, duration, reversibility, or outcome adjudication
  contract for Syndicate work.
- `src/synthesis_execution.rs:58–90` builds a canonical Aim for exactly two
  IDs (`pixy_confusion`, `gremlin_tinker`), fires it, and applies its contact to
  a starting `FrameState`. Unknown IDs fail. The returned result is a computed
  landing; this dispatcher is not a general registry and does not record a
  world event or persist costs.
- Unit coverage embedded in `src/synthesis_execution.rs` verifies both
  canonical recipes, unknown-ID rejection, and rejection of altered content
  under both known IDs. It does not test any Syndicate title or prototype.

Reproduced and fixed a legacy executor integrity issue. Previously, dispatch
branched on recipe ID while compilation accepted caller-supplied nonempty
intents; a scratch executable showed `pixy_confusion` plus only
`ChangeFrame(Gremlin)` returned `frame=Gremlin; scripts=[SetFrame(Gremlin)]`.
The executor now compares a known-ID recipe against its full canonical
definition (including title, intents and faculty manifestations) before
building the aim. Altered known-ID content returns
`SynthesisExecutionError::NonCanonicalRecipe`; unknown IDs still return
`UnknownRecipe`. Regression coverage checks altered content for both known IDs,
including a renamed Pixy recipe with otherwise canonical content, while both
canonical recipe tests remain passing. This restores integrity
for the two existing canonical Recipes when called through
`execute_synthesis_recipe`. The lower-level public `construct_aim`, `fire`, and
`land_contact` APIs remain generic caller-supplied primitives and do not
provide recipe authorization or a permission boundary. This guard does not
make those primitives or the dispatcher suitable for Syndicate Incantations,
and approves no prototype behavior.

Caller-path review: `src/decision_engine.rs:917–919` preserves the execution
error in `DecisionEngineError::Execution`, and the desktop status renderer now
prints an explicit `NonCanonicalRecipe` error instead of a landing. There is no
fallback to the altered scripts on these high-level paths.

Validation after the guard: `cargo test --offline --locked --lib
synthesis_execution::tests` (4 passed, exit 0);
`cargo test --offline --locked --lib
kernel_pass_output::tests::desktop_recipe_section_reports_noncanonical_known_id_as_rejected`
(1 passed, exit 0);
`cargo test --workspace --all-features --offline --locked --no-fail-fast`
(pass, exit 0); `cargo clippy --workspace --all-targets --all-features
--offline --locked -- -D warnings` (exit 0); and
`cargo fmt --all -- --check` (exit 0).
The `/tmp` probe now prints `rejected=NonCanonicalRecipe` and exits 0.
After adding the renderer regression, the complete default-feature suite also
passed: `cargo test --workspace --offline --locked --no-fail-fast` (451 library
unit tests; 0 failed; overall exit 0). Strict Clippy and formatting were rerun
after that test addition and both exited 0.

The three prototype names False Directive, False Wake and Silent Double remain
unratified with no discipline mapping. Do not route them through the existing
executor or create placeholder success results. Before any separate executor
proposal, an approved specification must define allowed inputs and targets,
authorization and consent, observable state changes, costs/material accounting,
duration and limits, reversibility, failure atomicity, and replay semantics.
These are required design fields, not proposed Incantation behavior.

## Proposed future acceptance checks

These checks are **planned, not implemented/passing tests**:

1. Exactly three paths, nine named ranks and three leaders; no extra sovereign
   office or Face.
2. Rank/affiliation never automatically grants a Form, House, Agency, command,
   clinical scope, private-data permission or unrelated authority.
3. Historical IDs and source-era meanings roundtrip unchanged; no index-based
   rank migration or fabricated credentials.
4. Unapproved anatomy and prototype effects remain unavailable; unsupported
   execution fails before mutation.
5. Any separately approved Recipe has tests for input/output conservation,
   authorization, consent, actual effects, failure atomicity and replay before
   activation.

## Initial classification boundary

For the current design baseline, the nine approved names function as rank
titles only. A title alone grants no credential/evidence, Agency assignment,
physical Form, membership, office, or authority. This is a conservative
classification boundary, not approval of anatomy or executable capability.

| Path/rung | Approved rank title | Credential/evidence | Agency or specialization | Separate physical Form |
|---|---|---|---|---|
| Word I | Warlock | None implied; unresolved | None implied; unresolved | Unresolved |
| Word II | Wizard | None implied; unresolved | None implied; unresolved | Unresolved |
| Word III | Sorcerer | None implied; unresolved | None implied; unresolved | Unresolved |
| Work I | Wiccan | None implied; unresolved | None implied; unresolved | Unresolved |
| Work II | Witch | None implied; unresolved | None implied; unresolved | Unresolved |
| Work III | Sorceress | None implied; unresolved | None implied; unresolved | Unresolved |
| Performance I | Mage | None implied; unresolved | None implied; unresolved | Unresolved |
| Performance II | Magician | None implied; unresolved | None implied; unresolved | Unresolved |
| Performance III | Mime | None implied; unresolved | None implied; unresolved | Unresolved |

Further creator decisions may add explicit qualifications or assignments.
Anatomy and prototype Recipe effects stay open until their own specifications
are approved. This design work is independent of, and cannot bypass, the
blocked Flynt sovereign transfer.

## Catalog contract implemented

The nonexecuting catalog was implemented only after settling and recording
these design choices:

1. **Resolved for design only:** adopt the namespaced stable IDs and provenance
   below. Canonical title source is the creator-approved Revolution C brief,
   §4.4; Master Canon handoff §6.5 corroborates it. No historical mapping is
   implied. A future title revision requires an explicit continuity decision.
2. **Resolved baseline:** ranks are titles; rung is display/order metadata only.
   No advancement evidence, credential, eligibility or authority is implied.
3. **Resolved boundary:** membership is outside this rank-title catalog.
   Rank title does not imply membership, House affiliation or Agency
   assignment; no membership field or transition is modeled here.
4. **Resolved source policy:** update this static catalog only alongside an
   explicit creator-approved canon source revision, with a catalog version
   referencing that source. This creates no in-world approval role and borrows
   no Flynt election/Court power.
5. Keep Form anatomy and Incantation execution outside this catalog until each
   has its own approved specification and validation contract.

Decisions 1–4 define the narrow, nonexecuting title/provenance catalog.
Runtime abilities, membership, qualification and authority remain outside its
scope. Two catalog identity/provenance tests pass; no Form or Incantation
behavior is implemented or tested.

## Next phase sequence

The catalog phase is complete. The next design work should resolve each
unsettled row in “Decisions required before code” using explicit creator
approval and a source/version record. Keep separate specifications for rank
qualifications, Agency assignment, membership, lead tenure, each physical
Form and each Incantation/Recipe. Do not derive one decision from another or
start executable integration while its required authority, consent, state,
resource, failure and replay rules are missing. Historical Warlock and Lycan
identities remain untouched throughout.

### Adopted stable-ID design

Per the user's direction to do both, adopt this namespaced ID scheme for
Syndicate design. IDs include discipline, rung, and original title slug. This
keeps Syndicate Warlock distinct from historical
`PractitionerTradition::Warlock`; display titles may be revised only by an
explicit versioned continuity decision, while IDs remain immutable. The IDs
are keys in the static catalog; they are not capability identifiers and do not
drive runtime state transitions or permissions.

| Discipline ID | Rank ID | Source title |
|---|---|---|
| `syndicate.word` | `syndicate.word.rung_1.warlock` | Warlock |
| `syndicate.word` | `syndicate.word.rung_2.wizard` | Wizard |
| `syndicate.word` | `syndicate.word.rung_3.sorcerer` | Sorcerer |
| `syndicate.work` | `syndicate.work.rung_1.wiccan` | Wiccan |
| `syndicate.work` | `syndicate.work.rung_2.witch` | Witch |
| `syndicate.work` | `syndicate.work.rung_3.sorceress` | Sorceress |
| `syndicate.performance` | `syndicate.performance.rung_1.mage` | Mage |
| `syndicate.performance` | `syndicate.performance.rung_2.magician` | Magician |
| `syndicate.performance` | `syndicate.performance.rung_3.mime` | Mime |

Provenance for all nine entries: creator-approved Revolution C brief, §4.4,
with the matching discipline/rank wording also recorded in the cited Master
Canon handoff §6.5. The catalog stores those source references and its own
reviewed version; it does not use source line numbers as IDs. No historical
record is to be migrated to these IDs by matching title text.

Implemented nonexecuting row shape: immutable `id`, `discipline_id`, display
`title`, ordinal `rung`, `source_ref`, and catalog `version`. The ordinal
describes presentation order only; it does not assert qualification,
eligibility, tenure, or authority. Keep Form, Agency,
membership, credential, permission, and Recipe-effect fields out until their
separate meanings and approval sources are decided. A title correction must
not silently rewrite an ID or relabel historical records; any alias or identity
continuity rule needs an explicit versioned decision. This is static data, not a
gameplay state or permission model.
