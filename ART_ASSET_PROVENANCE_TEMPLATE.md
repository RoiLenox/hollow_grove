# Art Asset Provenance Template

Copy this template for every production asset or asset set.

```yaml
asset_id: asset.example.v1
display_name: Example asset
owner: renderer
source_canon:
  - PLATINUM_PATH_CULTURAL_CANON_V1.md
  - HOLLOW_GROVE_VISUAL_BIBLE_V1.md
location: hueman_godot/assets/...
stable_runtime_ids: []
culture_or_house: central-junction
visual_function: witness | evidence | advisory | decision | refusal | consequence | atmosphere
platinum_test: continuity | consent | witness | custody | return | none
twofold_witness_relation: none | roi | roselina | paired
authority_statement: What this asset communicates without claiming authority.
return_route_statement: How the image preserves or suggests a safe next route.
palette_tokens: []
animation_notes: []
accessibility_text: Describe the visual in plain language.
runtime_boundary_check: unchanged
review_status: draft | approved
```

## Review questions

- Does the asset express a cultural interpretation without creating new law?
- Is its House, route, or civic context clear?
- Does it distinguish evidence/advice from decision?
- Does it leave room for consent, refusal, and return?
- Does it preserve the Twofold Witness as witness rather than ruler?
- Can the asset be replaced without changing an archive or stable ID?
