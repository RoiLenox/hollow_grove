# Hollow Grove Keyboard Constitution V1

Date: 2026-08-14

Status: official; A–Z vocabulary and modifier grammar semantically frozen

## Purpose

This constitution makes the Hollow Grove keyboard predictable without changing
ordinary typing or application-local shortcuts. Modified letter keys name the
field of work. Modifiers name the way the user approaches that field.

The governing law is:

> **Key = WHAT · Modifier = HOW**

Applications are replaceable implementations. The key meaning and modifier
meaning remain stable when an application is installed, removed, or replaced.

## Modifier Grammar — HOW

In Niri configuration, `Mod` is the implementation spelling of `Super`.

| Modifier | Meaning | Think |
|---|---|---|
| `Super` | Primary | Normal way |
| `Super+Shift` | Adjacent | Different form |
| `Super+Ctrl` | Focused | Specialized/control |
| `Super+Alt` | Alternate | Other way |

These four meanings are fixed across the keyboard:

- **Primary** is the normal or default route into the key's field.
- **Adjacent** is a neighboring form of the same work.
- **Focused** is a specialized, inspecting, or controlling form.
- **Alternate** is another valid route into the same field.

Combined modifiers express the real intersection of their meanings. They are
exceptional and must not be used as overflow storage. The canonical written
order is `Super+Ctrl+Alt+Shift+Key`, omitting unused modifiers; Niri may
serialize an equivalent chord in another order.

## A–Z — WHAT

This table is the complete constitutional letter vocabulary.

| Key | Meaning | Memory hook |
|---|---|---|
| A | Assets | MAKE |
| B | Browse | Web |
| C | Chat | Communicate |
| D | Develop | Code/build |
| E | Edit | Modify |
| F | Files | Filesystem |
| G | Games | PLAY |
| H | ← Movement | Flynt |
| I | Images | View images |
| J | ↓ Movement | Glaüshouse |
| K | ↑ Movement | Stonebend |
| L | → Movement | Sandmanor |
| M | Music | HEAR |
| N | News | Feeds |
| O | Office | Productivity |
| P | Photos | Photography |
| Q | Query | Ask AI/search |
| R | Record | Capture |
| S | Settings | Configure |
| T | Tasks | To-do/work |
| U | Utilities | System tools |
| V | Video | WATCH |
| W | Write | Compose |
| X | eXchange | Finance/markets |
| Y | sYnc | Transfer/sync |
| Z | Zip | Archives |

Capitalization in `eXchange` and `sYnc` is intentional: it exposes the physical
letter used by a word whose natural initial is already claimed. The uppercase
hooks MAKE, PLAY, HEAR, and WATCH are mnemonic emphasis, not alternate names.

### Letter-form lock

The 22 non-movement letters are domain families. H, J, K, and L are the
movement tool rather than application domains:

```text
                    K — ↑ Movement — Stonebend
H — ← Movement — Flynt                  L — → Movement — Sandmanor
                    J — ↓ Movement — Glaüshouse
```

Every modifier branch remains inside the WHAT named by its key. For example,
`Super+Shift+P` must still be Photos, and `Super+Ctrl+Q` must still be Query.
A program's vendor, toolkit, or implementation does not determine membership.

A family may retain empty branches when no honest implementation exists.
Completeness is never a reason to install or bind a semantically unrelated
tool. In particular, X remains eXchange for finance and markets even when the
machine has no suitable finance application.

## Keyboard Ontology

The keyboard is one Grove with stable forms:

| Constitutional term | Grove reading | Keyboard meaning |
|---|---|---|
| Foundation | bedrock and soil | stable addresses, coherent naming, and explicit amendment |
| House | growing environment | an affinity that explains a form without owning it exclusively |
| Plot | pot or fixed place | a physical key address |
| Plant | rooted domain family | a WHAT attached to one non-movement letter |
| Branch | growth from one plant | a HOW modifier lane inside that domain |
| Fruit | replaceable yield | the current application or concrete action |
| Tool | crafted implement | a family that operates on the desktop itself |
| Landmark | recognized boundary | a non-letter House anchor |

Thus P is the stable plot, Photos is the plant, `Super+Ctrl+P` is its Focused
branch, and the currently selected photo application is replaceable fruit.
HJKL, workspace numbers, brackets, and compositor punctuation are tools because
they move through or reshape the environment.

House relationships are affinities, not exclusive ownership. They may explain
a family, but visible keyboard language must remain understandable without
Hollow Grove lore.

## Domain and Branch Naming

A constitutional domain follows these rules:

1. One letter carries exactly one meaning from the A–Z table.
2. The domain is a concrete, plain-language field of work.
3. Every branch acts inside that field from the user's point of view.
4. `Super+Letter` is the Primary route when a primary implementation exists.
5. Shift, Ctrl, and Alt retain the fixed HOW meanings; they never rename or
   escape the domain.
6. Missing implementations remain explicitly empty.
7. Replacing fruit does not rename the plant or move its plot.
8. Changing a WHAT or HOW meaning requires an explicit constitutional
   amendment.

The canonical overlay title for an implemented domain binding is:

```text
Meaning · Modifier meaning: Action
```

Examples are `Office · Primary: LibreOffice Writer` and `Music · Focused:
cmus`. Product names belong only in `Action`.

## House Landmarks and Special Families

The A–Z lock coexists with five non-letter House landmarks. Their unmodified
forms always retain ordinary keyboard behavior.

| Physical key | House reading | User-facing family |
|---|---|---|
| Escape | Stonebend boundary, stop, and stable return | Exit |
| Backspace | Sandmanor returning threshold and revision | History |
| Enter | Sandmanor entering threshold and commitment | Terminal |
| Space | Glaüshouse clearance and usable room | Room/layout actions |
| Grave/Tilde | Flynt invocation and release | Navigate |

The user-facing family describes WHAT the modified key does. The House reading
explains why that non-letter landmark is situated there. Neither layer changes
the A–Z vocabulary.

## Movement Grammar

HJKL keeps its familiar directional shape across modifier layers:

| Layer | HOW applied to movement |
|---|---|
| `Super+H/J/K/L` | Primary focus movement |
| `Super+Shift+H/J/K/L` | Adjacent displacement or move |
| `Super+Ctrl+H/J/K/L` | Focused reshape or structural control |
| `Super+Alt+H/J/K/L` | Alternate scope or transport context |

Additional combined-modifier movement chords may express a genuine
intersection, such as focused movement in another scope. They remain movement
tools and never become application families.

## Operational Projection and Authority

This Markdown file is the authority for desktop keyboard semantics. It does
not alter the game kernel, world metaphysics, House sovereignty, or the Hollow
Grove constitutional runtime.

The related implementation surfaces are:

- discovery and audit command: `hollow-keyboard`;
- personal Niri profile: `~/.config/niri/config.kdl`;
- personal readable standard: `~/.config/niri/HOLLOW_GROVE_KEYBOARD_STANDARD_V1.md`;
- General Edition profile:
  `packaging/hollow-grove-general/etc/skel/.config/niri/config.kdl`;
- machine-readable semantic projection:
  `packaging/hollow-grove-general/etc/hollow-grove/keyboard-grove.json`;
- generated visual and accessible maps:
  `artifacts/hollow_grove_keyboard_grove_map.svg` and
  `artifacts/hollow_grove_keyboard_grove_map.md`;
- portable application inventory:
  `packaging/hollow-grove-general/etc/hollow-grove/applications.toml`.

The registry and generated maps project the full A–Z vocabulary and modifier
grammar. Profiles may use different executable paths and may expose different
installed fruit, but they must not change WHAT or HOW. A disagreement with this
constitution is an implementation defect.

`hollow-keyboard` reads the registry rather than redefining it. Its list,
search, menu, map, and doctor views are operational projections and carry no
independent constitutional authority.

## Maintenance Order

1. Amend this constitution before changing a frozen WHAT or HOW meaning.
2. Update the registry, readable standard, profiles, and application inventory
   in the same semantic change.
3. Regenerate the keyboard maps.
4. Run the keyboard verifier and validate every changed Niri profile.
5. Preserve unavailable branches as honest empty slots rather than filling
   them with unrelated software.

Ordinary fruit replacement and deployment-specific command paths do not
require a constitutional amendment when the key and modifier meanings remain
unchanged.
