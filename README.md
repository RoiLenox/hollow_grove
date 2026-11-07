# Hollow Grove Canon Index

Status: repository navigation index

For the operating-system product and packaging work, start with the
[Hollow Grove OS distribution foundation](distro/README.md). Hollow Grove OS
includes Hueman as its distinct companion; authenticated multiplayer
collaboration is the next application milestone. The existing local
Core/Hueman runtime remains the current implemented capability.

This file is the front door to Hollow Grove. It organizes existing authority;
it does not create lore or outrank any source below.

## Start Here

For the current implementation path, start with the concise
[Operational Entrypoint](HOLLOW_GROVE_OPERATIONAL_ENTRYPOINT_V1.md), then use
the constitutional reading order below for authority and lore depth.

For the next Hueman art/color construction phase, use the creator-approved
[Chromatic Color Standard v2.3](docs/design/HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_3_CODEX_ALL_IN_ONE.md)
and its [integration plan and swatch preview](docs/design/CHROMATIC_INTEGRATION_PLAN_V1.md).
The active neutral manifest has 34 colors and 32 visual Form assignments, with
all historical colors preserved. Form identities now progress from light to
dark; swatch positions stay fixed. The [growth v0.4 study](docs/design/HUEMAN_CHROMATIC_GROWTH_SYSTEM_V0_4_HEX_DISTANCE_CODEX.md)
uses separate working size proposals. Twelve formal names remain unassigned.
The current 16-color runtime contract awaits an explicit compatibility migration.

The current source prototype is [HUEMAN-02: persistent character introduction](docs/design/HUEMAN_02_GAMEPLAY_HANDOFF_20261106.md):
Skin, Face and Hair customization bound to runtime identity, Stonebend recovery,
and complete save/restart continuity. Run `./run-hollow-grove-game.sh --dev --hueman`
from a desktop session. The [V6→V7 migration](docs/HUEMAN_PROFILE_MIGRATION_V1.md)
preserves legacy identities without inventing cosmetics or progress.
[HUEMAN-01](docs/design/HUEMAN_01_CREATOR_HANDOFF_20261104.md) remains the isolated
appearance studio; both entry scenes are development tools outside installed packs.

Read the setting in this order:

1. [Hollow Grove Compromise](HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md) — the shared
   Hollow Grove Constitution. The filename retains draft provenance, but the
   document's internal status is authoritative.
2. [Constitutional Architecture](HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md)
   — how shared law, the four Houses, Hueman, and regional projections relate.
3. [Rampart Sovereignty and Seer Ceremony](RAMPART_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md)
   — The Rampart of Roi and Roselina, the paired Roi/Roï and
   Roselina/Rosaliña Guide seam,
   and the singular Seer Ceremony. Continue with the
   [Seer Ceremony Protocol](SEER_CEREMONY_PROTOCOL_V1.md),
   Continue with [Guide and Seer Identity](docs/GUIDE_AND_SEER_IDENTITY_V1.md),
   [Central Junction Transition](docs/CENTRAL_JUNCTION_TRANSITION_V1.md), and
   [Surface–Hueman Guide Protocol](docs/SURFACE_HUEMAN_GUIDE_PROTOCOL_V1.md).
   The crash-safe typed execution boundary is specified by
   [Guide State-Machine Hardening V2](docs/GUIDE_STATE_MACHINE_HARDENING_V2.md),
   and its live gameplay-service binding is
   [Guide Operational Seam V1](docs/GUIDE_OPERATIONAL_SEAM_V1.md).
4. [Repository Authority Map](REPOSITORY_AUTHORITY_MAP.md) — what is
   constitutional law, executable enforcement, a draft, a witness, or a
   generated projection.
5. [Flynt Synthesis and Sandmanor Hands Amendment](HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md)
   — the ratified resolution of the two former schema/constitution holds, then
   [Lore, Technology, and Project-State Consolidation](HOLLOW_GROVE_LORE_TECHNOLOGY_CONSOLIDATION_V1.md)
   — the status-controlled spiral model, PrimePunk/MachPunk/PetPunk/CordPunk
   languages, inter-House dependencies, Flynt weapons, synthesis tracks,
   and Stonebend development board.
6. [V2 Constitutional Specification](HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md)
   — the shared Bond and Constitutional Runtime contract.
7. The relevant House shelf below.
8. [Constitutional Geography](HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md) and
   [Route Network](HOLLOW_GROVE_ROUTE_NETWORK_V1.md) for place and travel,
   followed by [Spatial Inheritance](SPATIAL_INHERITANCE_V1.md) for recursive
   placement, viewpoint, archive, and replay law.
9. [Functional Lore Integration](HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md)
   for lived, playable lore, followed by the
   [Operational Lore Kernel proposal](HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md)
   and [Gimp-to-Pimp Sympiote Integrity Trial](BOARDWALK_GIMP_TO_PIMP_SYMPIOTE_TRIAL_V1_DRAFT.md)
   and [AI-Associated Personas Constitution](HOLLOW_GROVE_AI_ASSOCIATED_PERSONAS_CONSTITUTION_V1_DRAFT.md)
   and [Kernel Constitutional State](HOLLOW_GROVE_KERNEL_CONSTITUTIONAL_STATE_V1_DRAFT.md)
   and [Whole Constitutional Reconciliation](HOLLOW_GROVE_WHOLE_CONSTITUTIONAL_RECONCILIATION_V1.md)
   for the top-down function, role, handoff, refusal, and completion model.

The short rule is: authored constitutional documents define meaning;
`src/constitutional/` and `src/world/` enforce it; `tests/` prove it; and
`artifacts/` only project it.

## Backend development scope

The backend work covers lore contracts, architecture, source consistency,
validation and persistence/audit requirements. Follow the
[Backend Completion Plan](HOLLOW_GROVE_BACKEND_COMPLETION_PLAN_V1.md).
The expanded [Boardwalk backend contracts](BOARDWALK_BACKEND_CONTRACTS_V1_DRAFT.md)
cover six candidate offers, disclosure, exchange, delivery and remedy boundaries.
The [active Synthesis requirements](HOLLOW_GROVE_ACTIVE_SYNTHESIS_REQUIREMENTS_V1.md)
keep source-backed relationships and open mechanisms distinct. The
[House handoff crosswalk](HOLLOW_GROVE_BACKEND_HANDOFF_CONTRACTS_V1.md) and
[persistence/audit inventory](HOLLOW_GROVE_BACKEND_PERSISTENCE_AND_AUDIT_V1.md)
track the remaining work. The
ten historical House handoff gaps now have
[headless backend services](docs/BACKEND_HOUSE_HANDOFFS_V1.md), with typed results,
live authority, independent observation, protected recovery and replay.
Architecture V3 reports fourteen combined bindings and zero unbound canonical
backend routes; ten game presentation integrations remain deferred. The
[object-lineage backend draft](STONEBEND_OBJECT_LINEAGE_BACKEND_REQUIREMENTS_V1_DRAFT.md)
adds input/residue, passage, force and continuity requirements. Run `python3 tools/verify-backend-contracts.py` to
check the authored [backend register](contracts/backend/lore_registry.v1.json);
its consistency result does not claim new commerce or Form realization. The
[decision register](HOLLOW_GROVE_BACKEND_DECISION_REGISTER_V1.md) keeps the
remaining choices visible. Those backend passes excluded art and game
production; the separate Chromatic design phase is described above.

## Developer commands

The checked `rust-toolchain.toml` pins Rust, formatting, and Clippy. Ordinary
root-package builds compile nine runtime/test-facing binaries; the historical
audits and generators remain available without inflating every edit cycle.

Use `./verify-edit.sh` for source conformance and default-runtime compilation;
append an integration-test filter for a focused behavior check. Use
`./verify-integration.sh` for both feature configurations, strict lint, source
contracts, Python tools, Godot and the playable lifecycle. The existing
`./verify-full.sh` remains the extended release adversarial/fuzz/soak gate;
`./verify-fast.sh` retains its historical broader integration/benchmark behavior.
These workflows share Cargo's target directory instead of retaining an extra
CI build tree for the default feature configuration.

The current implementation and release blockers are summarized in the
[Operational Entrypoint](HOLLOW_GROVE_OPERATIONAL_ENTRYPOINT_V1.md).
Installed world readiness, recoverable progress handoff and local Hueman
performance settings are described in [Two-device operation](docs/TWO_DEVICE_OPERATION_V1.md).
The [1 November 2026 software checkpoint](docs/SOFTWARE_CHECKPOINT_2026-11-01.md)
records the two-machine verification and remaining product and release work.
All five package recipes, including the installed CLI and source inspection
inventory, are owned under `packaging/`. Regenerate Registrar data with
`python3 tools/build_registrar.py`; `--check` detects drift without writing.

```text
cargo test --workspace --all-features --offline
cargo clippy --workspace --all-targets --all-features --offline -- -D warnings
cargo dev-tool visual_color_constitution
./verify-art-assets.sh
./build-portable-release.sh
hollow-grove-update prepare
```

Generated output belongs under `artifacts/` or `dist/`; see
[Artifact Ownership](docs/ARTIFACT_OWNERSHIP.md) before adding a new output.
Read-only [Lore Kernel Inspection](docs/LORE_KERNEL_OPTIMIZATION_V1.md) is
available through `hollow-grove lore`. Use `--save path/to/save.json` to verify
and replay a gameplay archive before inspecting its Boardwalk consequence.
The report distinguishes authored contracts from recorded execution.
The read-only `hollow-grove architecture` command adds a bounded contract,
implementation, persistence and projection crosswalk, with replay-backed lore
coverage when `--save` is supplied. See
[Architecture Coverage and History Profile](docs/ARCHITECTURE_COVERAGE_AND_HISTORY_PROFILE_V1.md)
for the measured transaction optimization on both machines and the remaining
verified-binding work.
The [Stonebend continuity binding](docs/STONEBEND_CONTINUITY_LORE_BINDING_V1.md)
adds replay-backed inspection of Mercy Deep's existing Name/Seal case.
Lore report V4 covers the four earlier gameplay bindings. Installed V7's
architecture report V2 historically reports four bindings and ten gaps;
current source architecture V3 adds the ten headless backend bindings. The adopted
[survey-frame first case](STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md) implements
Mercy Deep's doorway, Tess Splice's custody and recovery, Ivo Mark's recording,
live House decisions, failed trial, protected refusal and exact-revision repair.
Open its workshop with **O**, choose with arrows and ask with Enter. Use
`HOLLOW_GROVE_WORLD_ROOT=/home/warren/Projects/active/hollow_grove`
when launching with the checkout's existing institutional catalog; a vacant
catalog continues to refuse authority-dependent work. Gameplay archives write
V7 and preserve older histories through the
[survey-frame](docs/SURVEY_FRAME_MIGRATION_V1.md) and
[Hueman-profile](docs/HUEMAN_PROFILE_MIGRATION_V1.md) migrations.
Hollow Grove Core and the Hueman presentation client are independently shipped
and joined through the versioned
[Hollow Grove / Hueman Seam](docs/HOLLOW_GROVE_HUEMAN_SEAM_V1.md). For a
recoverable source, state, Core, Hueman, and desktop-profile migration, use
the [Laptop Migration Plan](docs/LAPTOP_MIGRATION.md) after the RC tag exists.
The same `hollow-grove-update` command then provides guarded, explicitly
directed updates from either device to the other.
The [Lore Sync Checkpoint](docs/LORE_SYNC_CHECKPOINT_V1.md) records the local
handoff artifacts and the checks to finish when the main PC is available.
The [AMD Pre-transfer Review](docs/AMD_PRETRANSFER_OPTIMIZATION_V1.md) records
measured transport improvements and the target-machine profiling steps.
For the next lore milestone, use the
[Future Lore Workshop](HOLLOW_GROVE_FUTURE_LORE_WORKSHOP_V1_DRAFT.md). It gathers
open canon decisions, concrete story proposals, and the discussion order;
its open proposals remain unratified. Adopted sections link to their owning
contracts and implementation status.
The [Boardwalk offers and first-return proposal](BOARDWALK_SYMPIOTE_OFFERS_AND_FIRST_RETURN_V1_DRAFT.md)
develops six candidate crew offerings and an accountable Sympiote customer
story. Its exact catalog and new commercial mechanics remain for user adoption.
The [growth and performance pass](docs/BOARDWALK_GROWTH_AND_PERFORMANCE_V1.md)
records measured first-return service workloads on the laptop and AMD PC and
the implementation boundary for those proposals.
The user-selected [Sympiote Foundation](HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md)
develops an early Form established at Glaüshouse for pursuing supported
Synthesis paths across Hueman and Hollow Grove. Its clinical/practice lifecycle
is implemented; anatomical assets, shop inventories, and later Form mechanisms
remain open. The [V4→V5 migration](docs/SYMPIOTE_FOUNDATION_MIGRATION_V1.md)
preserves old saves without inventing foundation progress.

## Hollow Deck

The canonical gameplay compendium for Houses, Dominions, Natures, Current
Forms, Aura Sets, Primary Traits, progression, alchemy, Recipes, Synthesis
Recipes, Syntheses, and Banes is the [Hollow Deck](HOLLOW_DECK_V1.md). Launch
its interactive terminal surface with `cargo run --bin hollow_deck --offline`
or query it through `hollow-grove deck ...`. Canon remains read-only while a
separate checksummed state file persists cultivated Current Forms, learned and
equipped Aura moves, discoveries, and known Syntheses.

The current Deck catalog schema is `1.6.0`; its persisted state envelope and
default `artifacts/gameplay-saves/hollow-deck-state-v1.json` filename remain V1.
Glaüshouse has coequal Presensor/Matron/Jade and
Preceptor/Marshall/Emerald branches sharing one alchemical slot. Gargoyle uses
one cumulative biological growth path through White Opal Statue Mastery;
`Winged Gargoyle` remains a Wings-dependent safe-flight Synthesis, while White
Opal Statue itself is inherent Fortification rather than a Synthesis.
Flynt's renderer-facing identity matrix separately projects Mystery Men,
We Fairy Men, and Gallows names and visual language over an unchanged
Gargoyle, Werewolf, or Mariner Nature; aliases such as Johnny Apple and Jimmy
Pumps never become biological Nature IDs.

The immutable Deck also exposes distinct institutional grammars rather than a
shared rank tree: Glaüshouse selects one of two coequal seat branches; Flynt
advances from Warden through Corpman and Mystery Man into parallel specialist
sects whose independently refinable Current/Aura/Weapon tracks combine as
Manticorp and perfect as Chimera under the ratified amendment; Stonebend combines paired
Current/Aura competencies as Freemason/Superstar or
Proliteriate/Rockstar before Hypergiant integration; Sandmanor keeps Minorian
and Minoan as two complete ways of life, with independent social, Aura, and
Nature ladders whose peer Aura apexes resolve as
`Demiurge + Ascendant → The Transcendent → The Sandman`. The full rendering and
asset-layer invariants live in [Hollow Art Bible V1](HOLLOW_ART_BIBLE_V1.md).
Flynt also carries a separate paired sorcery ladder—Wizard/Witch,
Warlock/Enchantress, Sorcerer/Sorceress—for established manufacture, pattern
modification, and new Synthesis-pattern authorship under the wider Tross-led
ecosystem; it is not part of the Mystery Men rank tree.
That bible locks a modern implementation to Pokémon Yellow's visual discipline:
16×16-minded overworld readability, separate larger asset roles, bounded
palette and animation complexity, and Hueman then Gargoyle as the first visual
proofs.

The ratified [Flynt Synthesis and Sandmanor Hands Amendment](HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md)
resolves the former Deck/constitution holds: Manticorp is combined synthesis,
Chimera is perfected synthesis, Hecaton is the Right Hand, Pegasus is the Left
Hand, and The Sandman is the final sovereign culmination above both. Stable
legacy `manticore` identifiers remain compatibility projections where required.

## The Four House Shelves

### Stonebend

Stonebend owns Body/Presynce, Craft, Continuance of Form, lawful Hollowing,
Name, Claim, Title, material identity, and provenance.

Its technological design language is **PrimePunk**: primal physical laws,
functional visible mechanisms, precision engineering, and constant redesign.
Its immediate open design task is the bottom-up signature object/weapon pass,
not a premature final metaphysics of Hollowing.
The [PrimePunk Object Lineage draft](STONEBEND_PRIMEPUNK_OBJECT_LINEAGE_V1_DRAFT.md)
develops one repairable survey frame as a WORKING design case.

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

Its technological design language is **MachPunk**: motion, force transfer,
traction, leverage, balance, momentum, lift, and flight. Motion is a strongly
established specialty whose exact institutional monopoly remains open.

- Primary authority:
  [Sandmanor Constitution V2](SANDMANOR_CONSTITUTION_V2.md)
- Incorporated law:
  [Lineage Lock](SANDMANOR_LINEAGE_LOCK_v1.md) and
  [Guardian and Succession](SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md), under the
  controlling
  [Flynt Synthesis and Sandmanor Hands Amendment](HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md)
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

Its technological design language is **CordPunk**: Chromacord/CC,
relationship, resonance, optics, acoustics, medicine, fashion, and composed
systems. The broader Chromacord principle remains open beyond the existing
clinical institution and `chroma_cord` application.

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

Its technological design language is **PetPunk**: highly refined,
near-closed-loop petroleum-derived fuels, fluids, polymers, pressure systems,
capture, and recirculation. Reflection is a strongly established philosophical
direction beneath the locked Law of Capacity.

The Boardwalk's principal occupants are **the Pimp and his Trooners** and
**the Gimp and his Gooners**: rival retinues centered on public Aura and
enduring Current. They compete for trust, willing followers, opportunity, and
public credit while sharing the promenade with independent residents. Their
organization is defined in the
[Boardwalk Social Constitution](FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md).

- Primary authority:
  [Flynt Constitution V2](FLYNT_CONSTITUTION_V2.md)
- Incorporated law:
  [Dual Leadership and Manticorp Recipe](FLYNT_DUAL_LEADERSHIP_AND_MANTICORP_RECIPE_V1.md)
  under the controlling
  [Flynt Synthesis and Sandmanor Hands Amendment](HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md)
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
| Spiral, technology, weapons, and current design state | [Lore, Technology, and Project-State Consolidation](HOLLOW_GROVE_LORE_TECHNOLOGY_CONSOLIDATION_V1.md) | status-controlled central spiral, four Punk languages, civilizational interdependence, Flynt weapon tracks, reconciliation holds, and Stonebend development priorities |
| Universal meaning | [Semantic Foundation](HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md), [Metaphysical Balance](HOLLOW_GROVE_METAPHYSICAL_BALANCE_V1_DRAFT.md), [Classification Reconciliation](HOLLOW_GROVE_CLASSIFICATION_RECONCILIATION_V1.md) | Current, Aura, Relativity, Hollow, Synthesis, Being/Object roots, material pressures, excess, return, and V1 lineage interpretation |
| Pure recursion kernel | [Kernel v0.1.2](KERNEL_v0.1.2.md), [hollow-grove-recursion-kernel](hollow-grove-recursion-kernel/) | dependency-free, bounded routing grammar; no House, route, character, evidence, or lore authority |
| Semantic geometry and composition provenance | [hollow-grove-semantic-kernel](hollow-grove-semantic-kernel/), [Constitutional Review](COMPOSITION_PROVENANCE_CONSTITUTIONAL_REVIEW_v0.1.0.md), [Contract Proposal v0.2.0](COMPOSITION_PROVENANCE_CONTRACT_PROPOSAL_v0.2.0.md) | neutral geometry plus identity-preserving composition; separate from recursion and world law |
| Shared constitutional runtime | [V2 Constitutional Specification](HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md) | identity, evidence, authority, Bonds, lifecycle, replay |
| Geography and travel | [Constitutional Geography](HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md), [Route Network](HOLLOW_GROVE_ROUTE_NETWORK_V1.md), [Screen Map](HUEMAN_SCREEN_MAP_v0.1.0.md), [Spatial Inheritance](SPATIAL_INHERITANCE_V1.md) | House boundaries, route verbs, route shape, recursively inherited map projection |
| Shared judiciary | [Minoan County Court](MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md) | disputes, amendment review, Restitution |
| Shared economy | [Central Junction Four-Pole Economy](CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md) | public exchange, indexes, settlement |
| Guide identity, The Rampart of Roi and Roselina, and Seer continuity | [Rampart Sovereignty and Seer Ceremony](RAMPART_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md), [Seer Ceremony Protocol](SEER_CEREMONY_PROTOCOL_V1.md), [Guide and Seer Identity](docs/GUIDE_AND_SEER_IDENTITY_V1.md), [Central Junction Transition](docs/CENTRAL_JUNCTION_TRANSITION_V1.md), [Surface–Hueman Guide Protocol](docs/SURFACE_HUEMAN_GUIDE_PROTOCOL_V1.md), [Guide State-Machine Hardening V2](docs/GUIDE_STATE_MACHINE_HARDENING_V2.md), [Guide Operational Seam V1](docs/GUIDE_OPERATIONAL_SEAM_V1.md), [HGRF compatibility architecture](THE_RAMPART_V1.md), [Executable Implementation V1](THE_RAMPART_IMPLEMENTATION_V1.md) | one Rampart, paired operational Guide identities, one individual Seer Ceremony at a time, Continuance or Founding, recursive successor Points, typed seam crossing, replay, inspection, migration, and accepted consequence |
| Shared civic Function | [Service Tournament — Central Junction Canon](SERVICE_TOURNAMENT_CENTRAL_JUNCTION_CANON_V1.md) | four paired-service House identities, War of a Thousand Hues, nonlethal scenarios, paint records, Service Marks, constitutional scoring |
| Hueman | [Hueman](HUEMAN_v0.1.0.md), [Faculties](HUEMAN_FACULTIES_V1.md) | constitutional entry point, Body/Spirit/Mind/Soul faculties |
| Historical and cultural ancestry | [Ancestral Atlas](docs/world_bible/ancestral_atlas/README.md) | descriptive historical inheritance beneath House canon; Working Point assignments, Gifts, manifestations, libraries, cultural memory, and read-only Hueman references |
| Regional surfaces | [Aura Field](AURA_FIELD_SURFACE_V1.md), [Aura Beach](AURA_BEACH_SURFACE_V1.md), [Aura Basin](AURA_BASIN_SURFACE_V1.md) | the three interior world surfaces |
| Lived and playable lore | [Functional Lore Integration](HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md), [Party and Recruitment](HOLLOW_GROVE_PARTY_AND_RECRUITMENT_V1.md) | actors, triggers, choices, consequences, persistence |
| Operational role refinement | [Operational Lore Kernel V1 Draft](HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md) | proposed top-down functions, exact role boundaries, handoffs, stops, completion, and a compatible executable refinement sequence |
| Capability status | [Capability Report](HOLLOW_GROVE_V2_CAPABILITY_REPORT.md), [Matrix](V2_CAPABILITY_MATRIX.md), [Inventory](V2_CAPABILITY_INVENTORY.md) | what currently exists versus what is deferred |

## Spiral at a Glance

> **From above, it looks circular. From the side, it’s actually ascending.**

The exact cosmological phrase is **“Hollow holds. Current flows. Abyss feels.
Aura reveals.”** Recurring principles can return at a higher order without
resetting history or erasing consequence. The new working technology circuit
`Stonebend → Sandmanor → Flynt → Glaüshouse → Stonebend at a higher level`
does not reorder the locked Rule of Twelve or the constitutional
Name/Prove/Clear/Recognize handoff. See the consolidation for the status and
boundary of each sequence.

## Route Shape at a Glance

Shape supports navigation and presentation; the constitutional verb still
defines why a route exists.

- Straight: Aura Ridge, Aura Way, Current Sea, Boardwalk, Basin Motor
  Speedway.
- Round: Mt. Aura, Current Seanad, Riptide, Stairway to Heaven, Glausbahn.

Glausbahn is the Round road sector: a repeated refinement loop between
Sandmanor design and Glaüshouse repair. Current Sea is a Straight civic
concourse between Glaüshouse and Stonebend surrounding the distinct Current
body. Its dense public contact witnesses continuity, but does not replace the
literal Current Sea body with a metaphorical crowd.

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
| A historical assignment, inherited Gift, modern cultural manifestation, or ancestry source status | [Ancestral Atlas](docs/world_bible/ancestral_atlas/README.md); any present law or authority change still belongs in its existing constitutional source |
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
