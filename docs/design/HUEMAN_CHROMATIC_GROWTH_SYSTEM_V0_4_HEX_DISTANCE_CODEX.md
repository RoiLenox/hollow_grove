# HOLLOW GROVE — HUEMAN CHROMATIC GROWTH SYSTEM v0.4
## Hex-distance sizing guide · Companion to Chromatic Color Standard v2.3

**Date:** November 3, 2026  
**Status:** Applied to the working character-art sizing guide; **not** approved exact physical sizes or a live Godot implementation.  
**Color authority:** `hollow_grove_chromatic_color_standard_v2_3_0.json` (34 creator-locked colors and 32 approved Form bindings unchanged).  
**Data:** `hueman_chromatic_growth_system_v0_4_hex_distance_working.json`.  
**Replaces:** v0.3 *equal-index/interpolated size-target suggestions only*; retains the exact v2.3 palette and the creator-directed Gremlin 0, Goblin 1, Gargoyle 2 growth indices.

### Design rationale

**Light = emergence; dark = mastery; real RGB gaps drive visual size increments.** Within each visual progression, adjacent approved `#RRGGBB` swatches can have different channel distances; large gaps produce larger changes in relative body height. This rule is a visual design proposal, not a claim that hexadecimal codes have intrinsic biological units.

**Base Hueman:** natural personalized flesh/skin colors, ordinary human stature, not allocated a House swatch. The Gremlin begins at human height (1.00×). Synthesis replaces pigmentation and develops *new* upright humanoid anatomy; never merely enlarge the same sprite.

### Mathematical mapping

For each lineage / visual sequence, parse each approved hexadecimal color as three 8-bit **sRGB channel values**. Do **not** use a raw six-digit hex integer: that has no meaningful physical-distance ordering.

```text
d(i-1,i) = sqrt((R_i-R_(i-1))² + (G_i-G_(i-1))² + (B_i-B_(i-1))²)
p_i      = [sum from k=1 through i of d(k-1,k)] / [sum all branch d]
height_i = H_start + p_i * (H_end - H_start)
example_native_pixels_i = round_half_up(height_i * 32)
```

`p_i` is normalized **cumulative adjacent-color path distance**, not the raw Euclidean distance from the first endpoint. With no intermediate spacings, a simple end-to-end distance would lose the designed midpoint geometry. `H_start` and `H_end` are retained from the previous **working** visual-size proposal; changing those anchors adjusts the magnitude of growth but not its hexadecimal spacing pattern.

### Proposed size anchors

| Visual sequence | Starting size | Ending size | Explanation |
|---|---:|---:|---|
| Stonebend | 1.00× | 2.10× | One native eight-Form progression; Gremlin is human-height, Troglodyte is apex ordinary Stonebend Form. |
| Sandmanor — first branch | 1.00× | 1.40× | Creator-approved visual assignment Gnome/Satyr/Minotaur/Hecaton; existing Satyr/Faun runtime ancestry conflict remains unresolved. |
| Sandmanor — second branch | 1.00× | 1.40× | Creator-approved visual assignment Elf/Faun/Centaur/Pegasus; independent four-Form sizing interval. |
| Glaüshouse | 1.00× | 2.10× | One native eight-Form progression; Pixie starts light, Muse is darkest and largest ordinary stage. |
| Flynt | 1.25× | 2.50× | A comparative visual palette of three sibling pairs plus Chimera and Manticorp, NOT a chronological eight-stage evolution chain. |

**32px is only a comparative standing-height example**, not a mandatory 32px sprite frame or 2D uniform scaling instruction. The historical 24×32 study and 16×16 world tile grid are distinct concepts. Broader bodies, wings, arms, and tails may require larger textures while retaining a common feet/ground pivot.

### Stonebend

Color path distance across this sequence: **277.6 RGB units**. One native eight-Form progression; Gremlin is human-height, Troglodyte is apex ordinary Stonebend Form.

| Growth Index | Form | Canonical Color | Hex | RGB gap from previous | Calculated height | 32px illustration |
|---:|---|---|---|---:|---:|---:|
| 0 | **Gremlin** | Air Superiority Blue | `#79B7D8` | 0.0 | **100.0%** | 32px |
| 1 | **Goblin** | *Name unassigned* | `#67A6CA` | 28.4 | **111.3%** | 36px |
| 2 | **Ghoul** | Steel Blue | `#5594BB` | 29.5 | **123.0%** | 39px |
| 3 | **Spectre** | *Name unassigned* | `#4282AF` | 28.8 | **134.4%** | 43px |
| 4 | **Troll** | Lapis Lazuli | `#2F6FA3` | 29.4 | **146.0%** | 47px |
| 5 | **Orc** | *Name unassigned* | `#235581` | 44.5 | **163.7%** | 52px |
| 6 | **Ogre** | Prussian Blue | `#163A5F` | 45.3 | **181.6%** | 58px |
| 7 | **Troglodyte** | Void Blue | `#0B1026` | 71.7 | **210.0%** | 67px |

**Feature of the spacing:** Ogre → Troglodyte has the largest RGB step, so the final height jump is larger than any early change. Gremlin → Goblin calculates to about 111.3%, close to the previous 112% draft; Gremlin → Gargoyle is a **direct cross-House branch**. Goblin and Gargoyle are different branch outcomes from Gremlin, not consecutive evolutionary Forms.

### Sandmanor — first branch

Color path distance across this sequence: **90.7 RGB units**. Creator-approved visual assignment Gnome/Satyr/Minotaur/Hecaton; existing Satyr/Faun runtime ancestry conflict remains unresolved.

| Growth Index | Form | Canonical Color | Hex | RGB gap from previous | Calculated height | 32px illustration |
|---:|---|---|---|---:|---:|---:|
| 0 | **Gnome** | Redwood | `#A6404F` | 0.0 | **100.0%** | 32px |
| 1 | **Satyr** | *Name unassigned* | `#933846` | 22.5 | **109.9%** | 35px |
| 2 | **Minotaur** | Burgundy | `#80303D` | 22.5 | **119.8%** | 38px |
| 3 | **Hecaton** | Wine | `#5A1F2A` | 45.8 | **140.0%** | 45px |


### Sandmanor — second branch

Color path distance across this sequence: **73.4 RGB units**. Creator-approved visual assignment Elf/Faun/Centaur/Pegasus; independent four-Form sizing interval.

| Growth Index | Form | Canonical Color | Hex | RGB gap from previous | Calculated height | 32px illustration |
|---:|---|---|---|---:|---:|---:|
| 0 | **Elf** | Blush | `#E07A86` | 0.0 | **100.0%** | 32px |
| 1 | **Faun** | *Name unassigned* | `#D26C79` | 23.7 | **112.9%** | 36px |
| 2 | **Centaur** | Dusty Rose | `#C35D6B` | 25.4 | **126.8%** | 41px |
| 3 | **Pegasus** | *Name unassigned* | `#B54F5D` | 24.2 | **140.0%** | 45px |


### Glaüshouse

Color path distance across this sequence: **206.8 RGB units**. One native eight-Form progression; Pixie starts light, Muse is darkest and largest ordinary stage.

| Growth Index | Form | Canonical Color | Hex | RGB gap from previous | Calculated height | 32px illustration |
|---:|---|---|---|---:|---:|---:|
| 0 | **Pixie** | Eton Blue | `#78C9A4` | 0.0 | **100.0%** | 32px |
| 1 | **Sprite** | *Name unassigned* | `#66B694` | 30.7 | **116.3%** | 37px |
| 2 | **Fairie** | Jade Green | `#54A283` | 31.8 | **133.2%** | 43px |
| 3 | **Spirit** | *Name unassigned* | `#428E73` | 31.3 | **149.9%** | 48px |
| 4 | **Siren** | Viridian | `#2F7A62` | 32.4 | **167.1%** | 53px |
| 5 | **Nymph** | *Name unassigned* | `#296B57` | 19.5 | **177.5%** | 57px |
| 6 | **Sylph** | Pine Green | `#235B4C` | 20.3 | **188.3%** | 60px |
| 7 | **Muse** | Brunswick Green | `#163C35` | 40.7 | **210.0%** | 67px |


### Flynt

Color path distance across this sequence: **261.5 RGB units**. A comparative visual palette of three sibling pairs plus Chimera and Manticorp, NOT a chronological eight-stage evolution chain.

| Growth Index | Form | Canonical Color | Hex | RGB gap from previous | Calculated height | 32px illustration |
|---:|---|---|---|---:|---:|---:|
| 2 | **Gargoyle** | Silver Gray | `#ADADAD` | 0.0 | **125.0%** | 40px |
| 3 | **Medusa** | *Name unassigned* | `#9D9D9D` | 27.7 | **138.2%** | 44px |
| 4 | **Werewolf** | Stone Gray | `#8C8C8C` | 29.4 | **152.3%** | 49px |
| 5 | **Vampire** | *Name unassigned* | `#7C7C7C` | 27.7 | **165.6%** | 53px |
| 6 | **Merman** | Dim Gray | `#6B6B6B` | 29.4 | **179.6%** | 57px |
| 7 | **Mermaid** | *Name unassigned* | `#535353` | 41.6 | **199.5%** | 64px |
| 8 | **Chimera** | Charcoal Gray | `#3A3A3A` | 43.3 | **220.2%** | 70px |
| 9 | **Manticorp** | Eerie Black | `#161616` | 62.4 | **250.0%** | 80px |

**Flynt exception:** The eight rows are an ordered size/color **comparison**, not successive unlocks. Gargoyle and Medusa are Gremlin-root siblings, Werewolf and Vampire are Gnome-root siblings, and Merman and Mermaid are Elf-root siblings. Chimera is the convergence of Gargoyle+Werewolf+Merman; Manticorp is a different advanced Form. The Gargoyle is set to the creator-directed **Growth Index 2**, anchoring at **125%** human height even though Flynt’s canonical swatch slot is 8.

### Review gates / safety

1. **Approval boundary:** All 34 v2.3 colors, 32 visual Form names, and assigned color hexes are creator-locked. The growth formula and numerical height endpoints are an applied **working proposal**, not approved geometry or gameplay.
2. **Indices vs dimensions:** Creator-directed Gremlin 0, Goblin 1, Gargoyle 2 stay as labels. Other Growth Indices remain working; *numeric heights now vary by branch and RGB gaps*, not only by index.
3. **Color identity:** The exact Form swatch is never recalculated by size. A flesh-to-Form crossfade may be previewed separately but must end at the canonical hex and should not erase naturally diverse Hueman skin tones.
4. **Sprites:** Redraw proportions and anatomical details; preserve upright, bipedal, humanoid baselines even for Centaur/Pegasus/Merman/Chimera. Foot anchors, world pixel density, animation windows and hitboxes should not be silently changed.
5. **Existing world:** Keep Stonebend native progressions, Sandmanor visual/runtime Satyr-Faun conflict, Glaüshouse Muse offices, Flynt sibling Form learnability/dual switching, Chimera mastery, Manticorp/Tross distinctions and saves untouched. The growth indices are not progression gates.
6. **Legacy palettes:** The 12 unnamed swatches remain unnamed. The Oxford Blue outline conflict remains a separate visual review. No new House color, palette version, font change, kernel, legal, gameplay or hardware configuration changes.

### Codex handoff — safe next phase

> Read the existing approved `hollow_grove_chromatic_color_standard_v2_3_0.json` and this working Hueman Chromatic Growth v0.4 guide/JSON. Do not change any of the 34 hex tokens or 32 Form-to-color bindings. Audit the current Godot sprite pipeline and **add an isolated, review-only size/color comparison scene** for natural flesh-toned Hueman, Gremlin (1.000× / #79B7D8), Goblin (~1.113× / #67A6CA), and the **direct Gremlin → Gargoyle** branch (1.250× / #ADADAD). Use normalized cumulative adjacent RGB distances for the 32 working Form height targets, with explicit branch endpoints as specified. Keep feet on one ground plane; draw form-specific anatomy rather than stretching unchanged sprites. Render a complete 32-Form comparison chart using existing preview tools if possible. Do not change hitboxes, production gameplay, saves, or existing art. Keep Noahsarch and Occamsrazer uncommitted work intact. Run existing color tests, Godot smoke tests, and nonproduction visual checks; report changed files and comparison previews. Do not commit, push, deploy, or globally recolor.

### Validation run for this standalone specification

- 32 Synthesis Form bindings match the canonical creator-approved v2.3 JSON exactly.
- 32 unique House colors used; universal Full Black/Full White remain outside Form assignments.
- Each lineage/sub-branch has a positive, monotonically increasing visual-height target from its retained working start/end anchors.
- Normalized RGB distances and illustrative 32px heights recomputed from approved hex codes; no repo/game rendering or two-host test claimed.
