# Sandmanor Revision 100 governance amendment

Status: creator-approved integration, 2026-10-25. The creator authorized this
next milestone after the Stonebend checkpoint. This versioned amendment belongs
to `SANDMANOR_CONSTITUTION_V2.md`; it creates no replacement constitution or
kernel. Implementation approval is not an in-world ratification, election,
Court finding or clinical authorization.

Source: `~/HOLLOW_GROVE_REV_C/`, Revision 100 release gate, §99-E and Part II §4.
Master SHA-256: `f4286d60d316ce52e252aec583dcd27c1adaebadc8f45c78c1ed8bf736a260c8`.
The pre-amendment constitution and guardian supplement are preserved under
`docs/reference/` with `_PRE_REV100` filenames.

## Scope and precedence

This amendment controls regional offices, elections and Contest eligibility,
and states the current intended Form curriculum. Shared Compromise/Bond,
consent, independent Court authority and each other House's domain remain
unchanged. Compatible Sandmanor evidence, reciprocal teaching and improvement
rules survive. Contradictory earlier appointment, guardian-mantle eligibility,
Faun/Satyr pairing and Hand-only fourth-stage rules are historical compatibility
rules, not authority for a new Revision 100 accession.

| Region | Agency ranks | Current ordinary Form curriculum | Elected office |
| --- | --- | --- | --- |
| North / Aura Fields | Hearthkeeper → Householder → Husbandman | Gnome → Satyr → Minotaur → Hecaton | The Mino |
| South / Aura Beach | Beach Bum → Party Boy → Socialite | Elf → Faun → Centaur → Pegasus | The Cent |

Form mastery, Agency rank, guardian service, office and clinical permission
remain separate. Revision 100 recognizes Hecaton/Pegasus mastery as ordinary
advanced Form competence. The old guardian Hand/mantle records retain their
original meaning; they do not automatically become these qualifications.
The current path table is exposed by the governance adapter. Existing Deck
XP, saved `faun`/`satyr` fields and old lineage readers are not swapped or
relabelled. Their explicit progression migration remains separate work.

## Three distinct offices

The stable Sandman office stays `office.sandmanor.sandman`. The two new regional
offices are `office.sandmanor.the-mino` and `office.sandmanor.the-cent`. No old
guardian identity or office is renamed to either ID. Each office is singular
and may be vacant. The regional catalog grants only bounded North/South public
service scope; it grants no fiat issuance, Factory license or asset seizure.

Registered citizens of the respective region elect their governor. Voting does
not require property, Agency rank or personal advanced Form mastery. Candidates
need separately evidenced regional competence and Hecaton (Mino) or Pegasus
(Cent) mastery. Mastery alone never elects anyone. The Sandman cannot appoint,
arbitrarily dismiss or replace a certified governor.

Regular terms last four in-world years. Majority of valid votes cast wins;
otherwise a top-two runoff is required. A tie at the cutoff requires a lawful
parameter, and a tied runoff has no winner. Certified unique voters and ballot
receipts prevent double counting; reelection requires a fresh lawful election.

Recall petition, turnout and removal thresholds are supplied legal parameters,
separately authorized and archived per region. No Stonebend thresholds are
copied as defaults. Independent Court due-process certification excludes the
affected incumbent, administrator, petitioners and voters. A failed recall
retains the incumbent and its record. The regional recall mechanism cannot
remove the sovereign.

A special election after an early vacancy requires an explicit lawful choice
of unexpired remainder or a new four-year term. Revision 100 does not resolve
that choice for Sandmanor. Missing policy returns `RequiresLegalParameter`.
No expiry, death, resignation or recall produces an appointed substitute.

## Contest of Improvement

New succession requires a lawful sovereign vacancy and both sitting qualified
elected governors. The Court opens a case-linked Contest; the adapter binds
both exact tenure IDs. A non-incumbent advanced Form master cannot substitute,
and an outgoing Sandman is not automatically a defender. Challenge to an
occupied sovereign office remains outside this adapter without separately
defined lawful removal authority.

The governors sincerely teach one another, practice and demonstrate improvement
against independently recorded baselines. The five existing trial domains and
audited spectator judgments remain. Both candidates must perform their own
learning; baseline/improvement records, teaching findings and trial evidence
require independent reviewers. The existing improvement evaluator is shared
with the historical reader without manufacturing guardian mantles.

Spectators vote on demonstrated improvement. The greater raw improvement metric
does not itself appoint the sovereign. Ineligible, duplicate-voter, conflicted,
coerced and evidence-mismatched judgments are excluded by the existing tally.
The accepted record retains the tally and excluded judgment IDs.

A tie, no valid winner, or teaching sabotage pauses the Contest. Court-certified
incapacity or compromised proceedings also pause it. Ending/replacing either
governor tenure automatically pauses an open Contest. Neutral Court review,
two currently eligible governors and fresh ballot receipts are required to
resume; previous attempts remain in the journal. Missing governors cannot be
filled by an outgoing sovereign's preference. No caretaker powers, calendar,
recall percentage or deadline is invented; those require separate lawful
mandates and authenticated external procedure records.

Contest conflicts persist across attempts. A former contender remains excluded
from independent baseline, improvement, trial and teaching certification, and
their spectator judgment remains conflicted after their regional tenure ends.
A voter in any recorded attempt cannot later provide Court pause, retry or
outcome review for the same Contest. These histories are rebuilt from existing
journal commands during replay; no new archive schema or save migration is
required. An archive with previously accepted conflicted review is rejected,
not silently repaired or ratified.

A certified winner becomes the sole Sandman in the same accepted transition
that closes only that person's regional tenure. The losing governor retains
their tenure, and both retain earned mastery. The winning region then requires
a special election. Replaying the result never creates another tenure.

Only the incumbent may manifest Sandman. Manifestation separately requires
personal consent, existing Glaüshouse clearance authority and an earned ordinary
reversion Form. Tenure exit revokes the sovereign entitlement and reverts an
expressed sovereign Form atomically without erasing mastery. These are
entitlement records, not a biological or clinical simulation.

## Persistence and integration boundary

`src/world/sandmanor/governance.rs` uses the existing `SharedInterfaceRuntime`
and version-1 shared archive, with `sandmanor.governance.rev100` command identity.
Expected revision, in-world year, actor, evidence and Court-case references are
recorded. Commands validate on a clone; rejection changes neither state nor
history. Identical retries are idempotent, conflicting event IDs fail, and
replay revalidates command checksums and canonical event envelopes. FNV-1a is
accidental-corruption detection, not cryptographic authentication.

An explicit source snapshot can preserve an existing Sandman with adoption-year
provenance, not an invented historical election date. Historical regional
holders without election/term evidence block import. Inactive source holders
and closed tenure holders remain in the institutional projection. Other Houses,
memberships and witnesses are preserved. The full governance journal remains
necessary; a projected institutional snapshot alone is insufficient history.

This is a headless backend adapter. The caller must authenticate evidence,
reviewer mandates, citizenship rolls, lawful notice and applicable deadlines.
Signer strings are not authentication. It does not automatically open an
external Court docket, issue professional permits, activate gameplay authority,
rewrite a live save or migrate earned Deck progress. Its source/tests verify
bounded behavior rather than completion of the full Revision 100 release.

`hollow-grove sandmanor paths` displays the current regional Form curriculum.
`hollow-grove sandmanor inspect ARCHIVE [--json]` replays and validates a supplied
governance archive before reporting officeholders, tenure history and Contests.
Inspection never creates or ratifies authority.
