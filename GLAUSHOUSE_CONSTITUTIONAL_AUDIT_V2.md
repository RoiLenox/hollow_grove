# Glaüshouse Constitutional Audit V2

Status: canonical repository conformance record

Authority: `GLAUSHOUSE_CONSTITUTION_V2.md`

## Frozen Boundary

The Glaüshouse migration is implemented above Constitutional Runtime V2. It
does not redefine Current, Aura, Bond causality, common Synthesis execution,
persistence, replay, or the universal recursion kernel. The House layer decides
whether a clinical or Synthetic act is lawful; the common runtime remains the
authority that executes and records shared causality.

## Institutional Placement

| Constitutional subject | Stable identity | Kind | Constitutional placement |
| --- | --- | --- | --- |
| Prima Donna | `office.glaushouse.prima-donna` | singular House office | highest Recipe-transforming clinical authority |
| Persephone | `role.glaushouse.persephone` | multiple advanced rank | balanced whole-course Continuance authority |
| Matron | `role.glaushouse.matron` | multiple clinical rank | equal Aura-forward branch |
| Marshal | `role.glaushouse.marshal` | multiple clinical rank | equal Current-forward branch |
| Nightingale | `role.glaushouse.nightingale` in `institution.glaushouse.nightingales` | universal clinical foundation | bedside care, maintenance, renewal, Living Ledger, and protected stop |
| Glauspitals | `institution.glaushouse.glauspitals` | care institution | treatment, stabilization, recovery, and clinical sites |
| Chromacord | `institution.glaushouse.chromacord` | clinical-record institution | charts, evidence, and presentation without clearance authority |
| Medical civilization | `institution.glaushouse.medical-civilization` | compatibility umbrella | House-wide common-runtime jurisdiction; not a replacement institution or office |

Doctor Ratchet is the active holder identity of Prima Donna. Nurse House is one
canonical Persephone identity. Rank and office remain separate records, and
neither is inferred from species, transformation, recognition, or legacy
state. Advancement evidence always remains attached to one stable person ID.

## Authority Audit

- At most one active Prima Donna may exist; zero represents a vacancy.
- Multiple Persephones may exist simultaneously.
- Prima Donna is the only Glaüshouse office in the clinical ladder.
- Nightingale is the universal foundation and never a generic nurse species.
- Matron and Marshal are equal complementary ranks; neither is a prerequisite
  for the other and both remain open while incumbents serve.
- Persephone requires Nightingale foundation and lawful proof of both Matron and
  Marshal domains.
- Prima Donna candidacy requires qualified Persephone rank plus physician,
  diagnostic, Recipe, surgical, Ledger, outcome, teaching, and open-path
  evidence.
- `PublicClearance` and `FinalJudgmentAnswerability` remain adapter capabilities
  on Prima Donna for the frozen common House-decision protocol. Typed consent,
  capacity, clearance, privilege, provenance, and recovery records remain the
  Glaüshouse authority for whether a procedure may proceed.
- Chromacord records and presents evidence but cannot manufacture consent,
  clearance, diagnosis, Title, recognition, or Synthesis outcome.
- Glauspitals may host Chromacord through an explicit institutional
  `GrantsAccessTo` relationship; site control is not silently inferred.

No Glaüshouse institution occupies a second constitutional position. Clinical
advancement preserves one person record and accumulated rank evidence. No
clinical record type duplicates common runtime causality.

## Executable Law Mapping

`src/world/glaushouse.rs` owns the House-law types and registry validation for:

- stable-person clinical standing, branch advancement, Toke/Token evidence,
  cross-training, Persephone recognition, Prima Donna candidacy, accession, and
  office vacancy;
- diagnosis and capacity;
- explicit, scoped consent and withdrawal;
- time-bounded clearance;
- operator privileges;
- material provenance;
- recovery plans and inherited obligations;
- Synthesis depth, lifecycle, maintained Continuance, renewal, regression,
  collapse, two-way rejection, technical and lived viability, Ledger layers,
  actual outcomes, and identity history;
- Nightingale clinical stops and mandatory review;
- clinical custody without ownership.

`src/world/house_institutions.rs` owns canonical institutional placement.
`src/world/persistence.rs` owns explicit legacy migration. `src/world/chroma_cord.rs`
and `src/application_protocol.rs` project records and access without becoming a
second clinical rules engine.

## Legacy Migration Audit

- Legacy Nightingale membership moves to the Nightingales institution.
- Legacy recovery staff and medical-district access move to Glauspitals.
- Legacy `role.glaushouse.persephone` preserves historical service without
  manufacturing modern cross-domain proofs or Prima Donna candidacy.
- Expired clearances, withdrawn consent, revoked privileges, failed Synthesis,
  and Tombstoned offices remain historical records but cannot authorize new
  acts.
- No legacy progression value infers consent, clearance, privilege, Title, or
  office.

## Validation Surfaces

- `tests/glaushouse_constitutional_architecture.rs` proves the executable
  invariants and invalid states.
- `tests/glaushouse_documentation_conformance.rs` prevents constitutional and
  terminology drift.
- `src/bin/glaushouse_constitutional_audit.rs` checks exact institutional and
  office placement against the canonical catalog.
- `tests/constitutional_application_service.rs` proves that the House-specific
  law remains compatible with the frozen common application service.

The conformance commands are:

```text
cargo run --bin glaushouse_constitutional_audit
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --manifest-path hollow-grove-kernel/Cargo.toml
cargo test --manifest-path officials-and-outlaws/Cargo.toml
```

## Audit Judgment

The canonical Glaüshouse ladder has a universal Nightingale foundation, equal
Matron and Marshal branches, multiple Persephones, and one singular Prima Donna
office. It combines typed advancement and Continuance preconditions,
historical rather than authority-inventing migration, generative succession,
and a clean boundary from the universal recursion kernel. Documentation,
executable validation, public projections, and persistence use the same stable
identities.
