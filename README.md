# Hollow Grove Canon Index

Status: repository navigation index

This file is the front door to Hollow Grove. It organizes existing authority;
it does not create lore or outrank any source below.

## Start Here

Read the setting in this order:

1. [Hollow Grove Compromise](HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md) — the shared
   Hollow Grove Constitution. The filename retains draft provenance, but the
   document's internal status is authoritative.
2. [Constitutional Architecture](HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md)
   — how shared law, the four Houses, Hueman, and regional projections relate.
3. [Federal Sovereignty and Seer Ceremony](FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md)
   — Roi and Roselina, the two complementary federations, Worldright, and the
   outside-participation freeze.
4. [Repository Authority Map](REPOSITORY_AUTHORITY_MAP.md) — what is
   constitutional law, executable enforcement, a draft, a witness, or a
   generated projection.
5. [V2 Constitutional Specification](HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md)
   — the shared Bond and Constitutional Runtime contract.
6. The relevant House shelf below.
7. [Constitutional Geography](HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md) and
   [Route Network](HOLLOW_GROVE_ROUTE_NETWORK_V1.md) for place and travel.
8. [Functional Lore Integration](HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md)
   for lived, playable lore.

The short rule is: authored constitutional documents define meaning;
`src/constitutional/` and `src/world/` enforce it; `tests/` prove it; and
`artifacts/` only project it.

## The Four House Shelves

### Stonebend

Stonebend owns Body/Presynce, Craft, Continuance of Form, lawful Hollowing,
Name, Claim, Title, material identity, and provenance.

- Primary authority:
  [Stonebend Constitution V2](STONEBEND_CONSTITUTION_V2.md)
- Incorporated law:
  [Aura Way, Aether, and Hollowing Foundation](STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md),
  [Three Gates, Offices, and Title Scope](STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md),
  and [Title Lifecycle and Constitutional Continuity](STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md)
- Playable lore:
  [Current Sea Vertical Slice](STONEBEND_CURRENT_SEA_VERTICAL_SLICE_V1.md)
- Executable authority:
  [src/world/stonebend.rs](src/world/stonebend.rs),
  [foundation](src/world/stonebend/foundation.rs),
  [second pass](src/world/stonebend/second_pass.rs), and
  [third pass](src/world/stonebend/third_pass.rs)
- Gameplay:
  [src/gameplay/stonebend.rs](src/gameplay/stonebend.rs)
- Proof:
  `tests/stonebend_*` and `tests/gameplay_stonebend.rs`
- Historical source:
  [Stonebend Constitution V1 Draft](STONEBEND_CONSTITUTION_V1_DRAFT.md)
- Generated projection:
  `artifacts/hueman_stonebend_roles.md`

### Sandmanor

Sandmanor owns Soul, Prefog/Prefig, Design, reciprocity, formation,
Minorian/Minoan traditions, Aura Farm/Beach/coast, and guardian Synthesis.

- Primary authority:
  [Sandmanor Constitution V2](SANDMANOR_CONSTITUTION_V2.md)
- Incorporated law:
  [Lineage Lock](SANDMANOR_LINEAGE_LOCK_v1.md) and
  [Guardian and Succession](SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md)
- Conformance record:
  [Constitutional Audit V2](SANDMANOR_CONSTITUTIONAL_AUDIT_V2.md)
- Executable authority:
  [src/world/sandmanor.rs](src/world/sandmanor.rs),
  [milestone](src/world/sandmanor/milestone.rs), and
  [src/sandmanor_lineage.rs](src/sandmanor_lineage.rs)
- Proof:
  `tests/sandmanor_*`
- Historical source:
  [Sandmanor Constitution V1 Draft](SANDMANOR_CONSTITUTION_V1_DRAFT.md)
- Generated projection:
  `artifacts/hueman_sandmanor_roles.md`

### Glaüshouse

Glaüshouse owns Mind/Precog, Repair, Continuance of Function, compatibility,
consent, care, maintained Synthesis, and clinical records.

- Primary authority:
  [Glaüshouse Constitution V2](GLAUSHOUSE_CONSTITUTION_V2.md)
- Clinical specification:
  [Chroma Cord Clinical Charting](CHROMA_CORD_GLAUSHOUSE_CLINICAL_CHARTING_v0.1.0.md)
- Conformance record:
  [Constitutional Audit V2](GLAUSHOUSE_CONSTITUTIONAL_AUDIT_V2.md)
- Executable authority:
  [src/world/glaushouse.rs](src/world/glaushouse.rs) and
  [src/world/chroma_cord.rs](src/world/chroma_cord.rs)
- Proof:
  `tests/glaushouse_*`
- Historical source:
  [Glaüshouse Constitution V1 Draft](GLAUSHOUSE_CONSTITUTION_V1_DRAFT.md)
- Generated projection:
  `artifacts/hueman_glaushouse_roles.md`

### Flynt

Flynt owns Spirit/Resynce, Engineering, Function, persistence, deployment,
infrastructure, Manticorp, recognition, and its institutional hierarchy.

- Primary authority:
  [Flynt Constitution V2](FLYNT_CONSTITUTION_V2.md)
- Incorporated law:
  [Dual Leadership and Manticorp Recipe](FLYNT_DUAL_LEADERSHIP_AND_MANTICORP_RECIPE_V1.md)
- Active synthesis drafts:
  [Boardwalk Social Constitution](FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md)
  and [Goon Bond Vertical Slice](BOARDWALK_GOON_BOND_VERTICAL_SLICE_V1_DRAFT.md)
- Executable authority:
  `officials-and-outlaws/`, projected through
  [src/world/flynt.rs](src/world/flynt.rs) and
  [Gallowry](src/world/flynt/gallowry.rs)
- Gameplay:
  [src/gameplay/boardwalk.rs](src/gameplay/boardwalk.rs)
- Proof:
  `officials-and-outlaws/tests/`, `tests/flynt_*`, and
  `tests/gameplay_boardwalk.rs`
- Generated projection:
  `artifacts/hueman_flynt_constitution.md`

## Shared Canon Shelves

| Shelf | Start with | Scope |
|---|---|---|
| Universal meaning | [Semantic Foundation](HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md) | Current, Aura, Relativity, Synthesis, Being/Object roots |
| Pure recursion kernel | [Kernel v0.1.2](KERNEL_v0.1.2.md), [hollow-grove-kernel](hollow-grove-kernel/) | deterministic recursion only; no House, route, character, evidence, or lore authority |
| Composition provenance | [Constitutional Review](COMPOSITION_PROVENANCE_CONSTITUTIONAL_REVIEW_v0.1.0.md), [Contract Proposal v0.2.0](COMPOSITION_PROVENANCE_CONTRACT_PROPOSAL_v0.2.0.md) | how composed inputs retain identity and provenance without putting world law into the kernel |
| Shared constitutional runtime | [V2 Constitutional Specification](HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md) | identity, evidence, authority, Bonds, lifecycle, replay |
| Geography and travel | [Constitutional Geography](HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md), [Route Network](HOLLOW_GROVE_ROUTE_NETWORK_V1.md), [Screen Map](HUEMAN_SCREEN_MAP_v0.1.0.md) | House boundaries, route verbs, route shape, map projection |
| Shared judiciary | [Minoan County Court](MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md) | disputes, amendment review, Restitution |
| Shared economy | [Central Junction Four-Pole Economy](CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md) | public exchange, indexes, settlement |
| Federal sovereignty and runtime continuity | [Federal Sovereignty and Seer Ceremony](FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md), [The Runtime Federation](THE_RUNTIME_FEDERATION_V1.md), [Executable Implementation V1](THE_RUNTIME_FEDERATION_IMPLEMENTATION_V1.md) | Roi and Roselina, Worldright, deterministic cross-domain manifest, replay, inspection, migration, and accepted consequence |
| Shared civic Function | [Service Tournament — Central Junction Canon](SERVICE_TOURNAMENT_CENTRAL_JUNCTION_CANON_V1.md) | four paired-service House identities, War of a Thousand Hues, nonlethal scenarios, paint records, Service Marks, constitutional scoring |
| Hueman | [Hueman](HUEMAN_v0.1.0.md), [Faculties](HUEMAN_FACULTIES_V1.md) | constitutional entry point, Body/Spirit/Mind/Soul faculties |
| Regional surfaces | [Aura Field](AURA_FIELD_SURFACE_V1.md), [Aura Beach](AURA_BEACH_SURFACE_V1.md), [Aura Basin](AURA_BASIN_SURFACE_V1.md) | the three interior world surfaces |
| Lived and playable lore | [Functional Lore Integration](HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md), [Party and Recruitment](HOLLOW_GROVE_PARTY_AND_RECRUITMENT_V1.md) | actors, triggers, choices, consequences, persistence |
| Capability status | [Capability Report](HOLLOW_GROVE_V2_CAPABILITY_REPORT.md), [Matrix](V2_CAPABILITY_MATRIX.md), [Inventory](V2_CAPABILITY_INVENTORY.md) | what currently exists versus what is deferred |

## Route Shape at a Glance

Shape supports navigation and presentation; the constitutional verb still
defines why a route exists.

- Straight: Aura Ridge, Aura Way, Current Sea, Boardwalk, Basin Motor
  Speedway.
- Round: Mt. Aura, Current Seanad, Riptide, Stairway to Heaven, Glausbahn.

Glausbahn is the Round road sector: a repeated refinement loop between
Sandmanor design and Glaüshouse repair. Current Sea is a Straight civic
concourse between Glaüshouse and Stonebend. Its “sea” is primarily a sea of
people—dense public contact, competing claims, and ordinary civic pressure—not
a body of water.

The stable wire name `current-sea.deep-certification-landing` and similarly
named depth-witness identifiers remain for archive compatibility. Their
presentation is governed by the current canon above, not by the old wording.
Append-only runtime snapshots under `artifacts/` may still contain the wording
that was true when an older event was recorded. Treat that as historical state,
not as current route authority; new engine inputs use the classification above.

## Where New Lore Belongs

| New material | Put it here first |
|---|---|
| A change to one House's powers, offices, or required evidence | that House's V2 constitution or an explicitly incorporated supplement |
| A relationship between two or more Houses | the Compromise or a named shared-interface authority |
| A Service Tournament scenario, mark, Service Mark, objective, or result | Service Tournament canon and `src/world/service_tournament.rs` |
| A route's boundary, verb, purpose, or shape | Constitutional Geography, then Route Network and Screen Map |
| A character, place encounter, conflict, or choice | Functional Lore or a clearly labeled vertical-slice document |
| Executable state, validation, or replay | `src/world/`, `src/gameplay/`, and matching tests |
| A proposal not yet ratified | a filename and internal status marked `DRAFT` |
| Generated output | `artifacts/`; never cite it as the source of canon |

When a lore change reaches gameplay, refine it through the same chain:

```text
authority → place and verb → actor and evidence → choice → typed state change
→ persistence/replay → presentation
```

That chain keeps lore useful to the constitutional runtime without placing
story meaning inside either pure recursion kernel.

The kernel works better when lore is more structured around it, not when lore
is imported into it. Route identity, House authority, evidence, choice, and
consequence therefore belong in the constitutional/world/gameplay layers;
kernel inputs should remain neutral typed observations.
