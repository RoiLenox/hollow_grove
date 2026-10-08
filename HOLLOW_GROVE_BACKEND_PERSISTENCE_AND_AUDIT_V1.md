# Backend Persistence and Audit V1

Status: current-source inventory and future requirement model. Existing formats
retain their owners; proposed commercial and general progression records are
not newly supported runtime events. Scope is backend administration and validation.

## Current owners

| Concern | Existing owner | What the evidence proves |
|---|---|---|
| Authored lore meaning/status | Compromise, Synthesis canon, amendments and House sources | normative meaning, including fields explicitly open |
| Ten historical handoff definitions | `src/world/lived_lore.rs` | declared source roles, alternatives and scoped completion |
| Ten headless handoff services | `src/world/lore_handoffs.rs`, `docs/BACKEND_HOUSE_HANDOFFS_V1.md` | separate schema-1 case archives, actor inputs, scoped decisions/consequences, observation, delivery, recovery and replay; game archives unchanged |
| Gameplay transactions | `src/gameplay/runtime.rs` | staged domain changes and accepted parent journal; exact retry/conflict semantics |
| Aggregate archives | `src/gameplay/archive.rs` | gameplay V6, verified checksums, V1–V5 migration and domain replay |
| Application migration receipt retention | `src/gameplay/application.rs` | loaded migration provenance retained when saved again |
| Institutional snapshot | `src/world/session.rs`, `src/world/persistence.rs` | loaded authority history used by applicable replay |
| Sympiote graft | `src/world/sympiote.rs` | HGSYM V1 compatibility and distinct integration outcomes |
| Clinical foundation | `src/world/sympiote_foundation.rs` | one bounded profile and actual assessment/consent/care lifecycle |
| Continuing frame | `src/world/survey_frame.rs` | bounded revisions, proof, consent, failure/repair and recovery |
| Shared economy | `src/world/central_junction.rs` | amount/instrument distinctions, market evidence and applicable event settlement; not a general retail engine |
| Shared disputes | `src/world/minoan_court.rs` | court/jurisdiction/restoration boundaries in the existing model |
| Existing read-only views | `src/gameplay/view.rs`, `src/gameplay/lore_binding.rs`, `src/architecture_audit.rs` | projected records and source links; declared links alone do not attest execution |
| Authored backend register | `contracts/backend/lore_registry.v1.json` | source/status index, open decisions and design requirements; not a world archive |

## Record identity and authority

Use explicit stable references for one continuing Being, object, contract or
case. Different records may describe one subject; they must not replace the
subject with a role, counter, current name or index. A revised quote, object or
procedure retains predecessor history and requires new evidence where the scope
changes. Keep actors, mandates, evidence producers, recipients and recovery
owners attributable. A referenced office is not evidence of a live holder.

Keep requests, decisions, actual results and personal acceptance distinct.
A rejected unauthorized request leaves authoritative state unchanged. A lawful
physical failure remains an accepted historical event with a negative result;
it is not removed through transactional rollback. Withdrawal prevents future
scope use and may require recovery; it does not erase previous performance,
proof, consent or continuing Form history.

## What remains bounded

The transaction candidate excludes the full accepted parent journal and its
request index. Once fallible domain work succeeds, it carries that history
forward and appends one accepted event. Every new domain field must be staged
in the explicit candidate constructor before this guarantee can cover it.
Nested domain histories can still contribute clone cost.

The current survey-frame child has at most three revisions, six offers,
thirteen resident decisions and two delivery records; measurement receipts are
bounded per revision. Repeated advisory changes stay in the parent journal.
The foundation view bounds recent receipt projection while preserving its full
archive. These are specific implemented bounds, not a claim that every world
history has a fixed size or that a future catalog must use those limits.

For future commerce, keep static catalog descriptions with one authored owner;
project references and exact current terms rather than cloning every catalog
entry into each receipt. Use separate quote, delivery, acceptance, remedy and
settlement references, with explicit record bounds or summary policies. Never
silently truncate authoritative history to meet a transport budget.

## Compatibility requirements

Gameplay archive V6 and protocol V1 are existing contracts. New backend design
metadata uses its own index schema and is not fed into Core. `Pixy`, `Faerie`,
legacy physical Current/Aura fields, `manticore` and divided-Recipe identifiers
remain readable under their existing compatibility meanings. Active canon uses
Pixie, Fairy and Manticorp → Chimera; historical field names grant no active
ontology or authority.

A future runtime format change needs an explicit version, old-format fixtures,
checksum verification, deterministic replay, old-event preservation, rejection
of new events smuggled into old envelopes, and no invented progress for missing
case history. Migrating a source/status index is distinct from migrating a save.
Original user saves are not rewrite targets for research or validation.

## Audit behavior

`tools/verify-backend-contracts.py` is a read-only administrative checker. It
validates the authored register, safe relative source references, source status,
exact active correspondences, unresolved proposal fields and declared route gap
coverage. It emits digests and actionable diagnostics; it writes no world state.

Optional `--architecture-json` cross-checks a report from the existing trusted
architecture audit. It can compare declared sets and ownership; it cannot
independently prove that an external report executed the reducer or that its
statements are authentic. Treat the report's provenance as a separate question.

The checker must reject unsafe references, duplicate identities/JSON keys,
missing owners, legacy material promoted to active authority, inconsistent
pairings/branch geometry, superseded active refinement order, proposals marked
as ratified/implemented, unknown price interpreted as zero, and registered backend
bindings falsely promoted to gameplay or actual-case completion. Open decisions are not audit failures when their status
and blockers are accurately recorded.

## Backend verification and optimization order

First run the source/status checker and its adversarial fixtures. Review actual
coverage and open-decision output. For requested domain implementation, add
scope-relevant headless witnesses and measure validation, encode/decode, replay,
lookup and read-only report costs across small and long histories. Record source
commit, machine, workload, sizes and latency distributions. A service round trip
is not a graphical performance result.

The existing 64 KiB service limit must be measured for any future connected
view; a bounded child record count alone does not prove all combined views fit.
Use indexes or cached immutable definitions only after identifying measured
work. Preserve exact replay, complete historical evidence and independent
House decisions while optimizing. No Linux-kernel or driver tuning follows
from this authored-contract pass.
