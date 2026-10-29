# Glaüshouse Revision 100 governance amendment

Status: creator-approved implementation milestone, 2026-10-25, following the
Stonebend/Sandmanor checkpoint. This amendment belongs to
`GLAUSHOUSE_CONSTITUTION_V2.md`; implementation approval is not an in-world
election, ratification, Court finding or clinical authorization.

Source: `~/HOLLOW_GROVE_REV_C/`, master SHA-256
`f4286d60d316ce52e252aec583dcd27c1adaebadc8f45c78c1ed8bf736a260c8`,
§99-A, §99-G and Part II §5. These supersede conflicting older succession,
Form/rank correspondences and civic-identity rules. The Compromise, Bond,
Constitutional Runtime V2, independent Court, consent, clinical evidence and
other Houses retain their authority. The original constitution is preserved
at `docs/reference/GLAUSHOUSE_CONSTITUTION_V2_PRE_REV100.md`.

## Distinct institutions, people, Forms and permissions

Exactly nine Muse Houses have equal elected seats:

| Muse House | Origin Muse | Stable institutional suffix / seat suffix |
| --- | --- | --- |
| Calli | Calliope | `calli` |
| Cleo | Clio | `cleo` |
| Melanie | Melpomene | `melanie` |
| Sherry | Terpsichore | `sherry` |
| Talia | Thalia | `talia` |
| Terri | Euterpe | `terri` |
| Erin | Erato | `erin` |
| Polly | Polyhymnia | `polly` |
| Nia | Urania | `nia` |

Each institution uses `institution.glaushouse.muse-house.<suffix>`; its office
uses `office.glaushouse.persephone-de-<suffix>`, displayed as **Persephone de
[House]**. `institution.glaushouse.house-of-persephone` coordinates the nine;
it is neither a tenth Muse House nor another Great House, sovereign or chief
Persephone. The existing `office.glaushouse.prima-donna` ID remains unchanged.
The new offices grant coordination and an elector role, not clinical practice,
patient-record access, asset ownership, Factory permissions or extra Agencies.

The ordinary progression is exactly **Pixie → Sprite → Fairie → Spirit →
Siren → Nymph → Sylph → Muse**. Advanced post-Muse **Persephone** requires
separately evidenced mastery and a current elected Persephone seat. **Prima
Donna** is a separate incumbent-only sovereign Form. Exit revokes exclusive
manifestation rights and safely reverts an expressed Form to a previously
earned ordinary Form in the same accepted transition. Mastery survives.

**Nightingale** is civic admission, not a clinical credential. **Matron** is
Presensor/Emerald; **Marshall** is Preceptor/Jade. Both are separately licensed
professional roles, not split Form ladders. Working clinical specialties remain
deferred. The old Nightingale clinical role ID and explicit clinical grants
retain their meanings; the adapter never creates them from civic admission.
Prima Donna office alone no longer opens private Chroma Cord charts or recovery
chambers. Independent existing clinical roles or scoped grants remain necessary.

The legacy `GlaushouseSeat` values `calli/cleo/melony/teri/talia`, historical
`fairy` fields, branch affiliations and earned XP remain readable unchanged.
No name, including apparently matching labels, automatically migrates a person
to a new institution. In particular, `teri` cannot choose Sherry or Terri.
Explicit identity/progression reconciliation remains separate work. The current
governance types and CLI expose current doctrine without relabelling saves.

## Peer elections, terms and veto

Each House's certified professional electorate selects its qualified advanced
master by majority of valid votes, with a top-two runoff when needed. A tie at
the qualifying cutoff requires a lawful parameter; a tied runoff has no winner.
Unique eligible voters and unique receipts are required across rounds/events.
Form mastery and an elected seat remain separate. The winner must retain the
independently evidenced originating Muse-House Agency; a seat cannot switch
that Agency or grant a second one. A former officer may change Agency without
losing learned mastery.

Six-year terms follow three supplied cohorts of three Houses, two years apart.
The adapter does not choose the initial cohort assignments. Term dates are
explicit certified dates; a proposal cannot shorten an incumbent's term. A
scheduled result may be recorded before expiry but cannot install a second
holder. Reelection requires a fresh election. Special elections need an
independently authorized unexpired-remainder policy and the previous term;
missing or incompatible dates fail rather than inventing a term.
Tenure history distinguishes the certified term boundary (`began`) from actual
installation (`installed`); delayed review never grants retroactive authority.

A certified result is recorded pending the Prima Donna's bounded constitutional
review. During the fourteen-day window after posting, only the current,
unconflicted Prima Donna may record a documented constitutional veto. At the
fourteen-day boundary the window is closed. The legal calendar is supplied;
there is no default real-time conversion. Independent Court review can uphold
or overturn the veto and records the case. Neither a candidate, ballot voter,
administrator, vetoing sovereign nor affected officer adjudicates that appeal.
A veto does not appoint a replacement. Expired pending results cannot install
holders or indefinitely block the next election.

Recall requires 20% of the eligible professional roll to petition, 40% turnout,
and strictly more than half the valid votes for removal, plus independent Court
due-process review. Failed/tied recall stays in history without removal. Case
participants remain conflicted on retries. A recall targets only its named seat;
sovereign removal follows the separate six-seat procedure below.

## The Call and sovereign lifecycle

The five stages are exactly **The Pitch → Audition → Testimony → Ovation →
Corination**. The independent administrator records the process and the shared
Court-case reference. Only one unresolved Call may proceed at a time.

1. **The Pitch:** admitted Nightingales with Muse mastery and independently
   reviewed advanced leadership/coordination competence may stand. Holding a
   Persephone seat or clinical rank is not required. Independent Court screening
   attests lawful standing, conflict/capacity review and admissibility.
2. **Audition:** impartial assessors verify non-patient-exposing interdisciplinary
   demonstrations for every candidate. This adapter never authorizes an actual
   patient intervention through an audition.
3. **Testimony:** every candidate needs separate service, witness and independent
   privacy-review references. The privacy review attests lawful confidential
   admission, need-to-know handling and redacted public presentation. Evidence
   bodies and patient records are not included in governance commands.
4. **Ovation:** one ballot per current elected seat, bound to its exact tenure.
   Six affirmative votes are always required out of the nine-seat college.
   Vacancies, abstentions, ties and plurality never reduce six. No winner permits
   another recorded ballot with fresh receipts and the same threshold. A
   still-valid six-vote winner cannot be discarded by an administrator calling
   another ballot. A
   candidate-elector may vote, but cannot screen or review their own candidacy.
5. **Corination:** independent certification and resolved-appeal evidence verify
   the selected candidate and the same six current seat tenures. A stale seat
   mandate requires a fresh Ovation. Only lawful sovereign vacancy permits
   accession; an incumbent's term cannot be shortened by a new Call. Certification
   commits one nine-year tenure. A renewed term requires another full Call.

If a winning candidate still holds a Persephone seat, the canon does not specify
that seat's disposition as explicitly as Sandmanor's. The adapter therefore
requires an independently authorized seat-vacation policy before closing that
seat and installing the sovereign atomically. Without it the transition returns
`RequiresLegalParameter` with no change. This is not an assumed automatic vacancy.

Expiry, resignation, independently certified death or lawful early removal ends
the sovereign tenure. Early removal needs separate substantiated-grounds,
notice/response, resolved-appeals and independent Court records, plus six current
seat votes. Five votes leave the incumbent in office. Removal does not choose
a successor. No caretaker, holdover, renewed term or sovereign Form is invented.
Unspecified schedules, caretaker powers and legal standards remain external
lawful parameters.

## Runtime and compatibility boundary

`src/world/glaushouse/governance.rs` uses `SharedInterfaceRuntime` and its existing
version-1 archive, under `glaushouse.governance.rev100`. Commands record actor,
evidence, certified in-world date and expected revision. Validation occurs on
a clone; rejected commands change neither authority nor history. Identical
event retries are idempotent; conflicting IDs, stale revisions, time reversal,
receipt reuse and altered canonical envelopes are rejected. FNV-1a checksums
detect accidental corruption, not forged signatures.

`world_session()` preserves the source snapshot, other Houses, memberships,
grants and inactive holder history while projecting elected offices. The full
journal, not the institutional projection alone, retains terms, ballots,
reviews and closure events. Existing active Glaüshouse holders without explicit
term/election reconciliation block initialization; never delete them or infer
dates from labels to bypass that refusal. Historical clinical and succession
readers retain old records but do not authorize new Revision 100 accessions.

The caller must authenticate admission, sole active Agency, credentials,
qualified clinical reviewers, mandates, time, rolls, evidence and due process.
Signer strings alone do not authenticate them. Manifestation requires separate
personal consent and independently authorized clinical clearance; the adapter
records entitlement and expression, not anatomy or medical treatment.
It does not open an external Court docket, activate gameplay authority, migrate
a live save or grant a clinical license. Gameplay dispatch and live integration
remain separate work; this is not the complete Revision 100 release.

`hollow-grove glaushouse paths` and `seats` display the current doctrine.
`hollow-grove glaushouse inspect ARCHIVE [--json]` validates and replays a supplied
archive before reporting authority. These commands do not create an election,
rewrite an archive or activate governance in the gameplay router.
