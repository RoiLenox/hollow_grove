# Hollow Grove Visual Color Constitution

Status: canonical constitutional law
Machine schema: 1.0.0
Constitutional identity: `hollow-grove.visual-color-constitution`

## Constitutional Authority

Visual color is a first-class Hollow Grove constitutional domain alongside
Houses, Symbols, Materials, and Titles. It owns the canonical palette, House
palettes, universal sprite foundation, semantic color identities, pure-black
exceptions, validation rules, and renderer-facing access.

The single executable source of truth is:

`src/constitutional/hollow_grove_visual_color_palette.json`

The Rust domain in `src/constitutional/visual_identity.rs` loads and validates
that file. Godot and every other visual consumer read the file or use the Rust
accessors. A presentation layer may select a constitutional semantic identity
for a visual role; it may not define an RGB value, copy the palette, or create a
competing default.

This domain is outside the recursion kernel. It consumes the canonical `House`
identity but neither changes recursion nor influences kernel output.

## Universal Hollow Grove Foundation

| Canonical identifier | Name | RGB | Semantic identity |
| --- | --- | --- | --- |
| `universal.oxford_blue` | Oxford Blue | `#0A1024` | `hollow_grove.universal.outline` |
| `universal.midnight_blue` | Midnight Blue | `#121B38` | `hollow_grove.universal.shadow.deep` |
| `universal.space_cadet` | Space Cadet | `#1C2A52` | `hollow_grove.universal.shadow.raised` |

Every ordinary sprite begins with these values in the listed order before
House coloration. Oxford Blue is the universal outline. Midnight Blue and
Space Cadet are the deep and raised stops of the universal shadow layer.

Pure black is not an outline or an ordinary sprite default.

## Stonebend

| Palette role | Canonical identifier | Name | RGB |
| --- | --- | --- | --- |
| Dark | `stonebend.prussian_blue` | Prussian Blue | `#163A5F` |
| Primary | `stonebend.lapis_lazuli` | Lapis Lazuli | `#2F6FA3` |
| Highlight | `stonebend.air_superiority_blue` | Air Superiority Blue | `#79B7D8` |

Canonical meanings: Body, Structure, Architecture, Title, Naming, Mercury.

## Sandmanor

| Palette role | Canonical identifier | Name | RGB |
| --- | --- | --- | --- |
| Dark | `sandmanor.wine` | Wine | `#5A1F2A` |
| Primary | `sandmanor.redwood` | Redwood | `#A6404F` |
| Highlight | `sandmanor.blush` | Blush | `#E07A86` |

Canonical meanings: Soul, Proof, Design, Interior, Exterior, Crystal.

## Glaüshouse

| Palette role | Canonical identifier | Name | RGB |
| --- | --- | --- | --- |
| Dark | `glaushouse.brunswick_green` | Brunswick Green | `#163C35` |
| Primary | `glaushouse.viridian` | Viridian | `#2F7A62` |
| Highlight | `glaushouse.eton_blue` | Eton Blue | `#78C9A4` |

Canonical meanings: Mind, Medicine, Aura, Synthesis, Healing, Jade.

The stable machine identifier remains ASCII `glaushouse`; the canonical display
name is Glaüshouse.

## Flynt

| Palette role | Canonical identifier | Name | RGB |
| --- | --- | --- | --- |
| Dark | `flynt.rich_black_blue` | Rich Black Blue | `#101525` |
| Primary | `flynt.gunmetal` | Gunmetal | `#242C42` |
| Highlight | `flynt.powder_blue` | Powder Blue | `#A7B6D9` |
| Authority semantic overlay | `flynt.tropical_indigo` | Tropical Indigo | `#7F8BC4` |

Canonical meanings: Spirit, Recognition, Engineering, Opal, Motion, Authority.

Flynt is blue-black. Flynt is not true black. Tropical Indigo is a
constitutionally named semantic overlay for Authority and does not replace the
three required House base roles.

## Rendering Law

Every House renderer constructs its palette in this order:

1. universal outline;
2. universal shadow;
3. House dark;
4. House primary;
5. House highlight;
6. Current, Aura, or material overlays.

The universal shadow is one ordered layer with two stops: Midnight Blue, then
Space Cadet. A House palette supplements the foundation and never replaces it.
Any constitutional semantic overlay, including Flynt Authority, belongs after
the House highlight rather than inside the base sequence.

Overlay color values must also resolve from the canonical color catalog. A
renderer may not introduce an RGB literal to represent Current, Aura, material,
or Authority.

## Pure-Black Exception Law

`#000000` has the reserved identifier `constitutional.pure_black`. It is not an
ordinary canonical palette entry and is available only for an explicitly named
constitutional state. The currently ratified states are:

- Void;
- Absolute Absence;
- Erasure.

Future use requires an explicit constitutional definition. The ordinary color
lookup and House renderer accessors never return pure black. The separate
`reserved_pure_black` accessor fails closed for unlisted states.

## Renderer Boundary

Rust consumers use `canonical_visual_color_constitution`, `semantic_color`,
`house_palette`, `ordinary_sprite_defaults`, or `renderer_palette`.
`RendererHousePalette::ordered_base_layers` supplies the required order, and
`resolve_overlay` accepts only ratified overlay families and a canonical color
identifier.

Godot reads the canonical JSON and resolves semantic identities at runtime. The
terminal client exposes `visual-colors show` and `visual-colors validate` from
the Rust constitutional source. The Aseprite exporter generates an ephemeral
GPL adapter through `visual_color_constitution aseprite-gpl` and applies it to
each export; that generated adapter is never checked in. Shaders, UI themes, icons, and
future presentation tooling must read the same JSON or use an adapter generated
from it; generated adapters are projections and never authority.

No visual consumer may retain local six-digit RGB strings, numeric RGB
constructors, a copied House palette, or a pure-black ordinary fallback.

## Validation Law

The constitutional validator rejects:

- a missing or duplicated House palette;
- a House without its three required base colors;
- Flynt without its fourth Authority color;
- duplicate canonical color identifiers;
- duplicate or empty semantic identities;
- invalid color syntax or pure black in the ordinary catalog;
- missing, repeated, shared, or unassigned palette references;
- a universal foundation that does not precede House color;
- a rendering order other than the constitutional order;
- overlay families other than Current, Aura, and material;
- a pure-black policy that permits an ordinary default or an implicit future
  exception.

Tests also audit presentation source so renderer code cannot silently restore
local color literals.
