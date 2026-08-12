# The Ancestral Atlas

Framework status: Canonical descriptive ancestry layer<br>
Assignment status: Working Version 1<br>
Runtime status: documentation and read-only metadata only

> The Ancestral Atlas records how human history becomes living culture inside Hollow Grove.

The Atlas explains what real historical inheritances contribute to Stonebend, Sandmanor, Glaüshouse, and Flynt. It is not a fictional alternate timeline, a fifth House, a second constitution, or evidence that historical cultures literally became the Houses.

## Authority

```text
Hollow Grove Compromise
    ↓
House Constitutions and established canon
    ↓
Ancestral Atlas
    ↓
Hueman manifestations, cultural design, assets, curriculum, and gameplay
```

Locked canon always wins. Start with [Atlas Principles](ATLAS_PRINCIPLES.md).

## Navigate

- [Atlas Index](ATLAS_INDEX.md) — all 41 operational Points and both navigation directions.
- [Master Timeline](MASTER_TIMELINE.md) — revised operational chronology and preserved legacy master lineages.
- [Chronology Revision 1](CHRONOLOGY_REVISION_01_REPORT.md) — approved boundary, Point-type, and terminology refinements with the supplied baseline retained.
- [Age indexes](ages/README.md) — chronological navigation.
- [House ancestries and syntheses](houses/README.md) — cumulative lineage and modern cultural outcomes.
- [House libraries](libraries/README.md) — foundational books, historical wings, programs, and Hueman hooks.
- [Gifts Registry](ANCESTRAL_GIFTS_REGISTRY.md) — concrete inheritance records for every assignment.
- [Manifestations Registry](ANCESTRAL_MANIFESTATIONS_REGISTRY.md) — 41 reviewed Working flagships per House.
- [Historical Dossiers](dossiers/README.md) — source-reviewed coverage for all 41 operational Points and 164 assignments, plus the specialist dossier template.
- [Cultural Memory Model](CULTURAL_MEMORY_MODEL.md) — common, educated, and specialist knowledge.
- [Hueman Metadata Proposal](HUEMAN_METADATA_PROPOSAL.md) — JSON-compatible, read-only future hooks.
- [How to Use the Atlas](HOW_TO_USE_THE_ANCESTRAL_ATLAS.md) — provenance-first creator workflow.
- [Atlas Completion Report](ATLAS_COMPLETION_REPORT.md) — Ages IV–VI completion series, evidence tiers, exceptions, and final counts.
- [Current refinement bookmark](CURRENT_REFINEMENT_BOOKMARK.md) — coverage-complete state and future specialist-expansion guidance.
- [Schemas and templates](schemas/README.md) — required record shapes.
- [Audit Report](AUDIT_REPORT.md), [Integration Report](INTEGRATION_REPORT.md), [Chronology Revision 1](CHRONOLOGY_REVISION_01_REPORT.md), and latest [Refinement Pass 25 Report](REFINEMENT_PASS_25_REPORT.md).

## Status discipline

The system's descriptive place is canonical, but no Working Version 1 assignment was explicitly supplied as locked. Point and Gift records therefore remain Working, with stronger audit flags on uncertain chronology, House fit, titles, and modern data. This separation prevents a useful framework from silently ratifying unsettled content.

## Machine-readable reference

[`data/atlas.json`](data/atlas.json) mirrors Point, assignment, Gift, manifestation, and book identifiers for later read-only consumption. It has no runtime loader and creates no gameplay behavior or constitutional state.

## Validation

From this directory:

```text
python3 scripts/build_atlas.py
python3 scripts/validate_atlas.py
```

The validator checks 6 Ages, 41 operational Points, four assignments per Point, Point semantics, House coverage, library ownership, manifestation counts, registry references, expected generated files, and local Markdown links.
