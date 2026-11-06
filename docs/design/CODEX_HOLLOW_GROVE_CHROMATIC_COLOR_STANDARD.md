# HOLLOW GROVE — CHROMATIC COLOR STANDARD v1.0 / HUEMAN + CODEX HANDOFF

**Approved by creator:** 2026-11-01
**Status:** creator-locked design specification; runtime implementation NOT verified.
**Source:** `hollow_grove_chromatic_color_standard_v1_0.json` accompanying this document.

## Permanent project designation
- **Formal name:** **Chromatic Color Standard**.
- **Full document title:** **Hollow Grove — Chromatic Color Standard v1.0**.
- **Purpose:** permanent canonical art/color specification for building the Hueman game; reusable by independently packaged Hollow Grove OS visual tools.
- **Approval:** creator approved all 22 colors, all 22 names, and mathematical color relationships. Treat this specific v1.0 data as immutable; any change requires a documented new version and explicit creator approval.
- **Design/runtime boundary:** the standard is approved as **design canon**, but inclusion in your actual local git repository, Godot scenes, runtime palettes and installer is **not verified** by these downloaded files.
- **Existing law/runtime is untouched.** This document is not a new gameplay kernel or a change in legal authority.

## Goal
Integrate the 22-color specification in the existing Hollow Grove source project with one versioned, platform-neutral source of truth, then create safe independent consumers for Hollow Grove OS and Hueman/Godot only where existing architecture supports them. **Do not perform a redesign or migration without checking existing dependencies.**

## Canonical colors

| Level | Stonebend / Blue | Sandmanor / Red | Glaüshouse / Green | Flynt / Gray |
|---|---|---|---|---|
| 1 — Deep | Void Blue `#0B1026` | Wine `#5A1F2A` | Brunswick Green `#163C35` | Eerie Black `#161616` |
| 2 — Shadow | Prussian Blue `#163A5F` | Burgundy `#80303D` | Pine Green `#235B4C` | Charcoal Gray `#3A3A3A` |
| 3 — Core | Lapis Lazuli `#2F6FA3` | Redwood `#A6404F` | Viridian `#2F7A62` | Dim Gray `#6B6B6B` |
| 4 — Highlight | Steel Blue `#5594BB` | Dusty Rose `#C35D6B` | Jade Green `#54A283` | Stone Gray `#8C8C8C` |
| 5 — Peak | Air Superiority Blue `#79B7D8` | Blush `#E07A86` | Eton Blue `#78C9A4` | Silver Gray `#ADADAD` |

**Universal**: Full Black `#000000`; Full White `#FFFFFF`. These are Hollow Grove colors, not separate House shade slots.
**Regional associations**: Full Black—South Stonebend, Glaüshouse, South Flynt; Full White—North Flynt. These associations do not mandate recoloring all regional assets.

## Mathematical contract
- Flynt: per Stonebend 8-bit RGB triplet `(R,G,B)`, compute `Y = floor((R+G+B+1)/3)` and output `(Y,Y,Y)`. This is nearest neutral gray under unweighted RGB Euclidean distance, not equal perceived brightness. Verify exactly: `#161616, #3A3A3A, #6B6B6B, #8C8C8C, #ADADAD`.
- Sandmanor and Glaüshouse: canonical levels 1, 3, 5; levels 2 and 4 are each computed componentwise midpoint of adjacent anchored levels with `.5` rounded upward. All resulting hexes are now creator-approved and locked.
- Black/white mixes: interpolate per 8-bit RGB channel at a caller-selected `t∈[0,1]`; do not tie `t` to gameplay stats, runtime temperature or fan speed.
- House orientations: Stonebend north/12/Body; Sandmanor east/3/Soul; Glaüshouse south/6/Mind; Flynt west/9/Spirit. Central Junction is not a fifth House.

## Typography and art rules
- Master typography: Microgramma. Stonebend: Narkis Block. Sandmanor: Averta. Glaüshouse: Eurostile. Flynt: Frutiger. Use existing legitimate font assets or documented fallbacks, never bundle unlicensed fonts.
- Pixel-art: classic Pokémon Yellow readability and strong silhouettes, upright humanoid/bipedal Synthesis Forms; inspect local Godot sprites/assets first. Previous Mold B humanoid baseline was 24×32 native pixels, with 16×16 readability as a reference, not a blanket limit.
- Monochrome pen-and-ink/engraving for maps/editorial with restrained regional color accents.
- Do not override meaningful physical and optical materials (Diamond/Crystal/Jade/Opal, Mercury Mirror, Opal Oil), natural skin/hair, accessibility, or context-dependent lighting.
- Void Blue `#0B1026` is the darkest nonblack **chromatic palette member** in the new approved palette. Historical Oxford Blue `#0A1024` is darker; audit rather than blindly replacing legacy outline assets.

## Instructions to Codex
1. Locate the actual repository and read existing `AGENTS.md`, current project handoff, chromatic/art token files, Godot palette imports, OS theme adapters, tests, and dirty worktree state. Do not assume paths, existing classes or deployment state.
2. Compare existing definitions against this locked spec and report conflicts, especially original independent Flynt colors and the legacy Oxford Blue outline. Preserve old assets, project saves, and prior work.
3. Add or integrate a single versioned neutral data manifest; if one already exists, migrate it carefully rather than adding a competing source. Label it creator-approved design data, NOT enacted constitutional law or a proven runtime feature.
4. Add validation tests for 22 unique token slots (20 House shade positions + 2 universal), four House IDs, each five ordered levels, exact hex and names, Flynt grayscale derivation, red/green midpoint calculations, preserved cardinals, and no fifth House. Note that unique slots do not require 22 pairwise-distinct colors as an invariant unless confirmed.
5. Start with the minimal safe OS/Godot consumers already supported by the project and a small visual swatch preview. Do not alter the Current Synthesis kernel (`Point→Triway→Fourway→HollowGrove→CurrentSeam→AuraBeam→Point²`), saved FrameState, compositors, system package configuration, fan controls, rights or law.
6. Run only relevant tests/linters and Godot import checks. Report changed paths, coverage, remaining open decisions, and exact instructions to preview. Avoid auto-commit, auto-push, release deployment, overwrite or destructive cleanup without explicit authorization.

## Make it persistent in the project
1. Place the JSON and this Markdown document under the existing repository's `docs/design/` directory (or the current canonical art documentation directory if one already exists).
2. Add a small pointer to the **existing** root `AGENTS.md` rather than replacing that file; e.g.: `Hueman colors: read docs/design/hollow_grove_chromatic_color_standard_v1_0.json and docs/design/CODEX_HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD.md. All 22 colors/names and equations are locked in v1.0; do not introduce competing palettes.`
3. Ask Codex to find present art and theme consumers, migrate or project via the single source, and build a tiny **Hueman/Godot swatch preview** before any scene-wide changes.
4. Validate and commit the intended docs/tests in your real Git repository after reviewing the diff; merely downloading a file or updating ChatGPT memory does not install it into the game.
5. On a later approved revision, increment the version; do not rewrite v1.0 history in place.

## Suggested starter prompt
> Read `docs/design/hollow_grove_chromatic_color_standard_v1_0.json` and any existing `AGENTS.md`, compare against the current repository, and integrate this as the canonical shared Chromatic Color Standard v1.0 design-token source. First report conflicts and a small safe implementation plan; then implement only the manifest + validation tests and show the resulting diffs. Preserve game saves, kernel semantics, existing asset history, and all unrelated OS settings. Do not deploy, commit, or change the renderer/theme globally without approval.

## TUI-ready instruction to paste
> Establish **Hollow Grove — Chromatic Color Standard v1.0** as the permanent art/color specification for Hueman. Read the two files under `docs/design/` and the existing repository `AGENTS.md`. Inspect current palette and Godot asset code first. Treat the 22 approved hex codes/names, four five-shade Houses, universal Black/White, and RGB midpoint/grayscale formulas as locked. Add one versioned shared design-token source plus unit tests; provide a simple Godot swatch preview only if appropriate in the existing project. Do not silently replace legacy Oxford Blue outlines, overwrite assets, alter game mechanics or saves, change system settings, or deploy. Show me the diff and tests before a commit.
