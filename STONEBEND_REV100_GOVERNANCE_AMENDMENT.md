# Stonebend Revision 100 governance amendment

Status: creator-approved integration, 2026-10-25. This is a versioned amendment
to `STONEBEND_CONSTITUTION_V2.md`, not another constitution or kernel.
Approval of implementation is not an in-world ratification or election event.

Source: `~/HOLLOW_GROVE_REV_C/` (previously named
`HOLLOW_GROVE_REV100_TUI_PACKAGE/`) — Revision 100 handoff, Part II,
Stonebend active constitution and Revision 99 implementation defaults.
The consulted master SHA-256 is
`f4286d60d316ce52e252aec583dcd27c1adaebadc8f45c78c1ed8bf736a260c8`.

This amendment controls conflicts concerning training, offices, accession,
elections, removal and seals. The common Compromise, Constitutional Runtime V2,
Bond, identity, Synthesis, consent, Court and other Houses retain their domains.
The complete earlier Stonebend constitution is preserved in
`docs/reference/STONEBEND_CONSTITUTION_V2_PRE_REV100.md` as LEGACY / SUPERSEDED
governance history. Old selection structs and fixtures are historical readers,
not authority for a new accession.

## Paths and distinct identities

| Path | Progression |
| --- | --- |
| Construction | Journeyman → Brickmason → Proliteriate |
| Craftsmanship | Craftsman → Artisan → Freemason |
| Duty | Squire → Justice → Hypergiant |

Mage belongs exclusively to the Syndicate. Older Stonebend pairing labels and
Superstar/Rockstar glosses remain only in historical data pending explicit
identity reconciliation; saved mastery and XP are not silently relabeled.

Multiple qualified Huemans can master Proliteriate and Freemason Forms.
Mastery grants no office. The three singular offices are Proliteriate (Yield),
Freemason (Claim) and Hypergiant (Title); any may be vacant. Institutions,
headquarters and distributed Yield witnesses survive occupants. An elected
officer cannot rewrite a witness's authentic findings or expand a mandate.

`office.stonebend.proliteriate` identifies the elected public-works office.
`office.stonebend.high-freemason` remains the stable ID of the single elected
Freemason; High Freemason is that officer's seal function, not a fourth seat.
`office.stonebend.hypergiant` remains the sovereign office. No identity graph
or alternate Bond aggregate is introduced.

## Professional elections and seal continuity

The respective registered professional branch electorate chooses independently
qualified, relevant-Form masters by majority of valid votes cast, with a runoff
when necessary. Terms are six in-world years; scheduled branch elections are
staggered by three years. Reelection is unlimited while lawful eligibility
continues. A special election fills only the unexpired remainder.

Recall requires a petition from at least 20% of the eligible branch roll,
at least 40% turnout, and strictly more than 50% removal votes, with independent
Court review. A certified tied or unsuccessful recall preserves the incumbent
and remains in history. Hypergiant removal follows the Trial, not branch recall.
The independent Court reviewer cannot be the recall administrator, a petition
signer, a recall voter, or an affected incumbent.

The outgoing Freemason seal loses authority at tenure closure. Its identifier
and tenure history remain archived. Independent election certification binds
a fresh, unique seal to the incoming tenure. The candidate and outgoing bearer
cannot certify the transfer. Vacancy has no active seal and grants no additional
Claim authority. A sovereign cannot substitute an appointee for a certified
professional result or dismiss either officer at personal pleasure.

## Acclamation and Trial of Duty

The Acclamation requires lawful vacancy, independently evidenced Justice
standing for each candidate, independent incumbent Freemason Claim and
Proliteriate Yield verification for each candidate, and registered Stonebend
citizens' majority vote or top-two runoff. Certification invests the victor for
an indefinite sovereign tenure. Inheritance, mastery, professional office,
outgoing preference and geographic travel confer no automatic accession.

The Trial of Duty has exactly these stages:

1. The Challenge: either current professional officer presents documented grounds.
2. The Defense: independent screening and notice permit a fair response.
3. The Proof: independent examination of demonstrable evidence.
4. The Judgment: independent Court factual and legal findings.
5. The Reckoning: dismissal, proportionate remedy, or certified removal.

Removal requires both **current** professional tenures' lawful support and
independently reviewed substantive grounds. A vacancy, recusal or former
tenure's consent cannot count affirmatively. Neither the sovereign nor an
accusing officer adjudicates the case. Removal ends and archives the tenure;
it does not elect or appoint a successor. Successor selection is a separate
Acclamation. Stage names are Proof and Reckoning, not Proving or Disposition.
Professional participation in a Trial remains part of its conflict record after
resignation, recusal or replacement of a supporting tenure. A former participant
cannot become the independent Court reviewer for that same case.

Only the incumbent may manifest the Hypergiant Form. Expression still needs
separate Glaushouse clearance and personal consent. Ending tenure revokes that
entitlement and restores a previously earned ordinary Form in the same accepted
state transition. Ordinary and professional mastery survive. This adapter
records entitlement and expression; it does not simulate anatomy or perform a
clinical procedure. The ordinary Gremlin → Goblin → Ghoul → Spectre → Troll →
Orc → Ogre → Troglodyte progression remains unchanged.

## Explicit policy and evidence boundary

`Policy` records an independently authorized administrator, independent
credential and Court reviewers, and optional first election cohort years.
There are no production default people, cohorts or voter rolls. Callers supply
lawfully certified branch/citizen rolls and authenticated external evidence.
Qualification attests applicable professional prerequisites or Justice standing;
it does not manufacture those facts. Evidence IDs are references, not digital
signature verification. The service must be called behind a trusted authority
adapter; a player-supplied signer string is not authentication.

Unknown first cohorts, runoff-qualification ties and unrecorded special-term
remainders fail with `RequiresLegalParameter`. No lexicographic tie winner,
real-time conversion, incapacity rule, interim sovereign, appeal deadline,
production coefficient, body mechanic or clinical standard is invented.
Proof standards, notice, admissibility, voter eligibility and the legality of
reviewer appointments remain independently supplied law and records.

## Runtime and persistence

`src/world/stonebend/governance.rs` implements the versioned
`stonebend.governance.rev100` command adapter using the existing
`SharedInterfaceRuntime` and its version-1 archive. Each command payload carries
an FNV-1a checksum for accidental-corruption detection, not authentication. Every accepted
command records actor, evidence, in-world year and expected revision. Trial
events carry the shared Court-case reference. No second persistence format or
constitution is created. The adapter preserves Court record references; it does
not yet open or adjudicate a shared Court docket automatically.

Validation occurs on a clone. Rejected commands change neither authority nor
history. Repeated identical event IDs are idempotent; conflicting IDs and stale
revisions fail. Ballots require eligible unique voters and unique receipt IDs
across rounds and elections. Replay revalidates commands, ordering and canonical
envelopes. Closed tenures retain reason, end year, seal and closure-event link.
These are durable governance closure records; issuing a separate registry
Tombstone remains the responsibility of the existing identity/title adapter.

`world_session()` projects the verified three officeholders into the existing
institution catalog, preserving other Houses, memberships and witnesses.
Inactive source officeholder records remain intact, and closed governance
tenures project inactive holder records. Reelection never reactivates an old
historical row. Full tenure identity, dates and seal history remain in the
governance archive; institutional holder records alone do not encode them.
Exporting that institutional snapshot alone is **not** a substitute for the
governance archive: it has no term, ballot, Court or seal history.

An explicitly supplied old snapshot may preserve an existing Hypergiant without
inventing a new election. Imported `began` means adoption year, not an asserted
historical accession date. Old professional holders lacking term/seal evidence
block import with `RequiresLegalParameter`; no arbitrary term is invented.
`Governance::from_reconciled_snapshot` now accepts explicit professional
continuity records: exact source holder and office, unique tenure and election
IDs, original rule version, independently certified election and term, separate
mastery/qualification records, Court review, and the Freemason's existing seal
certification. Proof records must be distinct and attached to the event.
The supplied active term must fit the authorized six-year/staggered cohorts;
incompatible or expired terms require further legal reconciliation. No date
is inferred from the adoption year. This continues the same evidenced tenure
and seal; it neither elects a new holder nor transfers a seal. Imported IDs
remain reserved against later reuse, and a subsequent special election cannot
extend the imported remainder. The original initialization command encoding is
unchanged; the additive `initialize-reconciled` command retains the source
snapshot and all supplied provenance in a separate replayable result.
Gameplay saves and legacy Deck fields are not rewritten. The main gameplay
command router does not yet dispatch these governance commands automatically.
The participant conflict set is reconstructed from existing commands during
replay; no archive schema change or save rewrite is required. Archives containing
conflicted Court certification are rejected rather than silently repaired.

Use `hollow-grove stonebend paths` for current paths and
`hollow-grove stonebend inspect ARCHIVE [--json]` to validate and inspect an
existing governance archive. Inspection never creates or ratifies authority.

## Remaining Revision 100 work

The current stage supplies governance, persistence replay, authority projection,
the TUI path display and acceptance tests. It is not the complete Rev100 release.
Live election/Court/clinical adapters, reconciliation of incompatible or
unevidenced legacy terms, and gameplay command dispatch remain follow-up work.

The current public route names describe exactly two physical corridors:
Stonebend: H.T.T. — Highway to Hell (descending Stonebend → Flynt) and
Flynt: S.T.H. — Stairway to Heaven (ascending Flynt → Stonebend) identify
one curved mountain highway. Basin Motor Speedway (B.M.S.) remains a separate
straight bidirectional connection. The Hippo Dome occupies the flat intervening
land; Hippodrones are its concession workforce. Presentation uses existing
route IDs; any future archive migration remains separate work.
Preserve the existing mine-gallery Highway identity independently. None is an
Acclamation requirement. Other Houses' ambiguous identity mappings and the
four-fiat/Corn economy remain outside this Stonebend stage.
