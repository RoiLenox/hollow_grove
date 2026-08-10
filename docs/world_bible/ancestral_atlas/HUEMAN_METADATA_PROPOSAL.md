# Hueman Ancestral Atlas Metadata Proposal

Status: Working, read-only data proposal; no runtime consumer implemented

## Compatibility decision

The repository already uses JSON for schemas, fixtures, palettes, and artifacts. The Atlas therefore publishes JSON at [`data/atlas.json`](data/atlas.json) and a proposal schema at [`schemas/ancestral-atlas-v1.schema.json`](schemas/ancestral-atlas-v1.schema.json). It does not introduce YAML, a database, a loader, a reducer, or a new authority path.

## Ownership boundary

Atlas metadata may label and retrieve authored lore. It may not grant a Title, create Proof, infer consent, create Clearance, Recognize a function, choose for a Being, mutate constitutional state, or redefine a route. A future consumer should treat every Atlas record as read-only descriptive context.

## Record shape

```json
{
  "id": "atlas.age_02.point_01.stonebend.kingdom_of_israel",
  "age": 2,
  "point": 1,
  "point_suffix": "",
  "point_coordinate": "1",
  "house": "stonebend",
  "culture": "Kingdom of Israel",
  "status": "working",
  "temporal_fit": "People full / Kingdom emergent",
  "relevance_range": "Israel named c. 1208 BCE; highland settlement through Iron I; disputed monarchic formation especially eleventh–tenth centuries BCE",
  "gifts": [
    "gift.stonebend.kingdom_of_israel.clustered_highland_settlement_networks",
    "gift.stonebend.kingdom_of_israel.household_agrarian_storage_and_water_practice",
    "gift.stonebend.kingdom_of_israel.layered_preservation_of_foundation_and_kingship_traditions"
  ],
  "manifestations": [
    "manifestation.stonebend.layered_foundations_register"
  ],
  "canon_links": [
    "stonebend_constitution_v2",
    "stonebend_three_gates_offices_and_title_scope_v1"
  ],
  "hueman_hooks": [
    "curriculum",
    "district_reference",
    "artifact_provenance",
    "npc_background"
  ],
  "dossier_document": "dossiers/age_02_point_01/KINGDOM_OF_ISRAEL.md",
  "review_level": "reviewed_dossier"
}
```

The example correctly remains `working`: the framework is canonical, but Working Version 1 does not explicitly ratify individual assignments.

Age I, Points I–V, Age II, Points I–XII, and Age III, Points I–V assignment records also expose
`dossier_document` and a review level. Age I, Point I retains
`reviewed_benchmark`; the other reviewed Points use `reviewed_dossier`.
Their eighty-eight flagship manifestation records add purpose,
physical appearance, institutional role, implementation notes, confidence,
and scholarly-source links. These fields remain descriptive.

## Safe future consumers

- read-only library and museum filters;
- NPC background and education references;
- district and architecture palette lookup;
- artifact provenance display;
- curriculum and quest-authoring search;
- ambient music, literature, and public-memory tags.

Any runtime use must obtain a separate owner, versioned contract, tests, replay behavior where relevant, and constitutional review. This proposal alone authorizes none of those changes.
