# Flynt Revolution C bounded adapter

Status: **PARTIAL implementation; not an enacted replacement constitution or a
gameplay authority service.** See `FLYNT_REV_C_CONTINUATION.md` for commands,
results and legal blockers. Creator design is recorded in
`/home/warren/Downloads/HOLLOW_GROVE_REV_C.md` and the working master §99-B/C/D,
§6.4. Existing law and saved records remain intact.

`src/world/flynt/governance.rs` records commands in `SharedInterfaceRuntime`
with ruleset `flynt.governance.revolution-c.v1`. It preserves the legacy source
snapshot and exposes a separate four-office projection: Tross, Navigator,
Chief and Mariner. Career rank, historical cover, Face, qualification, body,
office and equipment custody are distinct. Three elected peer command seats
report directly to Tross; no cover is automatically inherited. Exactly three
Faces remain Mystery Men, We Fairy Men and Gallows; the Syndicate is separate.

**IMPLEMENTED+TESTED:** bounded command qualifications/elections/tenure exit;
six individual physical Forms; three independent Ambisynth pair credentials;
one expressed body; versioned convergence interpretation with separate Recipe
execution and competence attestations. Trisynthesis and The Trifecta name the
same Chimera outcome. Manticorp needs all six Forms and all three pairs and its
own Recipe, without an extra Chimera prerequisite under the explicit Rev C
interpretation. Neither Form automatically grants office. This records actual
external outcomes; it does not execute physical Recipes or authorize care.

**PARTIAL Q:** `q.rs` records an explicitly mandated preparatory contract,
current standing/manifestability, three independent assessors (at least two
affirmative), candidate consent, FIFO admission, withdrawal and expiry. The
administrator cannot select the second challenger ahead of the first. A new
front gets a 30-day scheduling clock; both participants need fresh readiness
evidence covering the scheduled day. The source Tross receives no invented
Manticorp credentials. Each opening round has three distinct authorized judges
and a two-vote majority. Panels do not overlap or include the admission
assessors or participants. Rejected verdicts leave no partial round; valid
evidence can be submitted afterward. Two wins record an outcome. Split rounds
require authenticated concession or independently witnessed inability to
continue; no final subjective jury is created. These records neither conduct
the physical contest nor appoint its winner.

**BLOCKED:** `FinalizeSuccession`, `OpenFinalVacancy`, and replacing an existing
contest return `UnresolvedLaw::QSuccessionReconciliation` without mutation.
The conflict is `FLYNT_CONSTITUTION_V2.md:419–443` (four-House records, Court and
sealed title; bounded death/abdication/removal/incapacity triggers) versus newer
master §6.4 (Q victory). The Compromise does not authorize silently replacing
House law. Also, `src/world/persistence.rs` rehydrates canonical officeholders
and cannot represent removal of the canonical Tross merely by omission.
Required next implementation: a lawfully approved, versioned vacancy/tenure
contract, authenticated Court/appeal disposition, then atomic succession,
seven-day recovery and post-loss thirty-day/improvement reentry. Current
voluntary withdrawal/reentry is not post-loss reentry. No fabricated legal
defaults or automatic vacancy are present.

**IMPLEMENTED+TESTED bounded Triple Crown:** `equipment.rs` names
`equipment.flynt.triple-crown.revolution-c.v1` and records externally existing
Gargoyle bolo, Werewolf crossbow and Merman harpoon component identities, with
independent inspection and scoped owner/custodian consent. Components cannot
be reused in another registration. Transfers require owner, outgoing and
incoming consent; ownership is unchanged. Presentation needs both current
custody and expressed Chimera. Owner/custodian withdrawal remains possible
after a body change; registration/history persist. Release is terminal for
this bounded contract: reissue, ownership sale, repair and component
disassembly need separately specified operations. Fabrication, ammo, damage,
ballistics, industrial licensing and combat execution are **DEFERRED**.

All command application is clone/validate/append/commit. Failures preserve
state and serialized journal. Equal event IDs/payloads are idempotent;
conflicting IDs and stale revisions fail. Replay validates the same envelopes.
Old A archives encode byte-for-byte; old readers cannot read newly added
actions. No old Recipe IDs, Form XP, source tenures or companions are migrated.
The new ordinary Chimera outcome never aliases the unique legacy companion.

Trust boundary: an external authenticated authority must bind every signer,
record, subject, scope and outcome, including current conflicts/recusals and
clinical/material permissions where applicable. Strings and FNV checksums are
not signatures. A caller-supplied registry or assertion is not proof of legal
enactment or physical fact. This adapter is not wired to authoritative gameplay
dispatch or private memory. Revocation/appeal/correction of accepted Q evidence
must be designed before enabling live succession. Numerical calendar, veto,
recall, cohort and special-election parameters remain unresolved unless
independently supplied under lawful authority; test policy values are simulated.

Constitutional closure review (26 October 2026):
`FLYNT_REV_C_CONSTITUTIONAL_CLOSURE.md` identifies exact succession and
Marquise/command-law conflicts with proposed, **unenacted** resolutions.
`Governance::new` now refuses a source/replayed Tross-holder mismatch with
`SourceSovereignContinuity`; this prevents silent incumbent restoration when
the supplied world contains an unsupported vacancy. Historical Initialize
events still replay their original bytes. No vacancy migration or succession
authority was added. Next independent design brief:
`SYNDICATE_REV_C_DESIGN_PHASE.md`; no Syndicate anatomy/effects were implemented.
