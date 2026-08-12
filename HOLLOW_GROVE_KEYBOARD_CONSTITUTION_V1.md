# Hollow Grove Keyboard Constitution V1

Date: 2026-08-12

Status: official; Steps 1 through 3 locked, personal and General Edition profile semantics reconciled

## Purpose

This document organizes the existing Niri keyboard around Hollow Grove without
replacing working muscle memory, ordinary typing, application shortcuts, or
native compositor behavior.

The governing migration rule is:

> Keep what makes sense, interpret it through the Houses, add what is missing,
> alias when a new landmark improves recall, and move only a genuine conflict.

This is not a new keyboard layout. It is a semantic map over the existing
keyboard.

## Frozen Anchor Decision

| Physical key | Niri key name | House | Governing reading |
|---|---|---|---|
| Escape | `Escape` | Stonebend | stop, hold, bound, return to a stable state |
| Backspace | `BackSpace` | Sandmanor, returning threshold | withdraw, reconsider, revise |
| Enter | `Return` / `KP_Enter` | Sandmanor, entering threshold | accept, commit, cross into the selected method |
| Space | `Space` | Glaüshouse | clear, permit, recover, and shape usable room |
| Grave/Tilde (` / ~) | `Grave` | Flynt | invoke, release, execute, transmit |

Backspace and Enter are one split Sandmanor threshold. Backspace is the path
back into revision; Enter is the path forward through acceptance. They are not
competing anchors.

The unmodified forms of all five keys retain their normal behavior. House
meaning governs Hollow Grove's modified Niri layer; it does not seize text
input from applications.

## Official Step 1 Lock — Flynt Invokes Walker

`Mod+Grave` is Walker's sole Hollow Grove operating-system binding.
`Mod+q` is intentionally unbound. This is the first official keyboard step,
not a temporary migration alias.

The existing `Mod+Shift+q`, `Mod+Ctrl+q`, and `Mod+Alt+q` browser/query variants
remain in place because they are useful and do not conflict with the Flynt
anchor.

## Official Step 2 Lock — Glaüshouse Shapes Usable Room

`Mod+Space` cycles the focused column forward through Niri's preset widths.
`Mod+Shift+Space` cycles backward through the same presets. These are the
official primary and reverse expressions of the Glaüshouse Space anchor.

Both actions preserve their established Niri behavior. Step 2 adds a stable
House reading and visible overlay labels; it does not relocate either action.

## Anchor Taxonomy Lock

Hollow Grove recognizes exactly two keyboard-anchor classes:

1. **House anchors** are non-letter landmarks whose physical and ordinary
   keyboard meaning expresses a House: Escape, the Backspace/Enter threshold,
   Space, and Grave/Tilde.
2. **Domain anchors** are mnemonic letter families that gather tools for one
   plain-language field of work, such as `O` for Office and `M` for Music.

A domain anchor is not a new House anchor. It carries no constitutional lore
of its own, and its location must not be used to infer House ownership. This
separation prevents every useful letter family from growing into a competing
constitutional landmark.

## Grove Ontology Lock

The keyboard is one Grove made from a common operational substance, but common
substance does not erase functional category. Hollow Grove uses the following
stable vocabulary:

| Constitutional term | Grove reading | Keyboard meaning |
|---|---|---|
| Foundation | bedrock and soil | Stonebend stability, bounded change, and naming discipline |
| House | growing environment | a broad affinity that explains a form without owning it exclusively |
| Plot | pot or fixed place | the physical key address that remains stable over time |
| Plant | rooted domain family | a literal field such as Office or Music attached to one letter plot |
| Branch | growth from one plant | a modifier lane that remains inside the plant's domain |
| Fruit | replaceable yield | the current application or concrete action reached by a branch |
| Tool | crafted implement | an operational family that tends, moves through, or reshapes the desktop |
| Landmark | recognized boundary | a House anchor with direct physical and constitutional meaning |

In ordinary conversation, `O` may be called the Office plant. Formally, `O` is
the stable plot, Office is the plant, `Super+Ctrl+O` is its Focused branch, and
Impress is the current fruit. This distinction permits an application to be
replaced without uprooting the domain or moving its keyboard address.

Plants and tools may be made from the same binding grammar while remaining
different categories. `O` and `M` grow domain families. HJKL, brackets,
workspace numbers, and compositor punctuation are tools because they operate
on the environment rather than opening a field of work.

House relationships are recorded as **affinities**, not exclusive ownership.
Office is presently Sandmanor-rooted. Music is presently Flynt-rooted and
Glaüshouse-tended. Those readings may explain the families, but the visible
names remain Office and Music. No overlay title should require lore knowledge
to operate the keyboard.

Stonebend governs the foundation rather than claiming every form: a plot keeps
one address, a plant keeps one domain, a branch stays attached to its plant,
and a change crosses an explicit amendment boundary.

### Locked Domain-Naming Criteria

A proposed domain anchor is admitted only when all of these tests pass:

1. Its name is one concrete plain-language noun, written in title case.
2. Its letter is the obvious initial of that noun.
3. One letter owns at most one official domain. Names containing `/`, `&`, or
   a list of unrelated activities are not domain names.
4. Every member acts on the named field from the user's point of view. The
   program's implementation, toolkit, or vendor does not determine membership.
5. The unmodified `Super+Letter` binding is the domain's primary tool or
   action. A domain without a credible primary does not claim the letter.
6. A domain has at least two durable members before it becomes official.
   Speculative slots remain empty; completeness is never a reason to add a
   binding.
7. Shift, Ctrl, and Alt select fixed modifier lanes inside the domain. They do
   not rename, broaden, or escape it.
8. Combined modifiers are exceptional and must express the actual intersection
   of their lanes. They are not overflow storage.
9. Aliases may exist only as documented migration aids. They do not create a
   second official letter for the same domain.
10. Once admitted, a domain name or letter changes only by an explicit
    constitutional amendment, not while adding an application.

The canonical human-readable chord order is `Super+Ctrl+Alt+Shift+Key`, omitting
unused modifiers. Niri may serialize equivalent modifiers in another order,
but documentation and overlay names use this order.

The canonical overlay-name form for a domain binding is:

```text
Domain · Lane: Action
```

Product names belong in `Action`, never in `Domain`. Thus `Office · Primary:
Writer` is constitutional; `LibreOffice · Writer` is not.

## Official Step 3 Lock — Office and Music Domain Anchors

`O` and `M` are the first official domain anchors. Their present actions are
retained and named through the fixed modifier lanes.

| Binding | Canonical name | Present action |
|---|---|---|
| `Super+O` | Office · Primary: Writer | LibreOffice Writer |
| `Super+Shift+O` | Office · Adjacent: Calc | LibreOffice Calc |
| `Super+Ctrl+O` | Office · Focused: Impress | LibreOffice Impress |
| `Super+Alt+O` | Office · Alternate: Draw | LibreOffice Draw |
| `Super+Ctrl+Alt+O` | Office · Focused Alternate: Math | LibreOffice Math |
| `Super+M` | Music · Primary: Library | Cmus |
| `Super+Shift+M` | Music · Adjacent: Visualizer | Cava |
| `Super+Ctrl+M` | Music · Focused: Play/Pause | Cmus playback control |
| `Super+Alt+M` | Music · Alternate: Status | Cmus status readout |

Office membership follows the shared document field, not the individual
LibreOffice executable mode. Music membership follows the listening activity,
so its player, visual companion, focused transport control, and status readout
remain one family.

All other letter groups remain **provisional launch families**. Existing
slash-named comments such as downloads/packages describe working inventory;
they do not satisfy this naming lock and do not become official domain anchors
without a later admission decision.

The machine-readable operational projection is
`packaging/hollow-grove-general/etc/hollow-grove/keyboard-grove.json`. It may
record this ontology, official anchors, affinities, and canonical overlay
titles. Its `profile_reconciliation` ledger locks shared chord semantics while
permitting deployment-specific executable paths, but it may not create new
constitutional meaning. The Markdown
constitution remains authoritative when the projection and prose disagree.
`artifacts/hollow_grove_keyboard_grove_map.svg` and its accessible Markdown
companion are deterministic visual projections generated from that registry
and Hollow Grove's constitutional palette; they are never edited as a second
binding source.

## Physical Reading

The anchor map is deliberately asymmetric because it follows the shape and
native meaning of the keyboard:

```text
[ Esc: Stonebend ]
[ ` / ~: Flynt  ]  [ number row                 ] [ Backspace: Sandmanor-return ]
[ Tab            ]  [ Q W E R T Y U I O P        ]
[ Caps           ]  [ A S D F G H J K L          ] [ Enter: Sandmanor-enter    ]
[ Shift          ]  [ Z X C V B N M              ]
                         [ Space: Glaüshouse ]
```

The existing HJKL diamond supplies the world-facing directional overlay:

```text
                    K — Stonebend / up
H — Flynt / left    J — Glaüshouse / down    L — Sandmanor / right
```

This agrees with the frozen Hueman screen orientation while preserving Niri's
standard focus and movement grammar. The anchors identify House landmarks;
they do not make every nearby letter exclusive property of that House.

## Modifier Grammar Preserved and Clarified

The current configuration already has a useful grammar. This draft keeps it
and gives its left- and right-hand expressions common language.

| Modifier layer | Canonical lane | Application-grid expression | Niri/spatial expression |
|---|---|---|---|
| `Mod` | Primary | default tool or direct action | focus / primary action |
| `Mod+Shift` | Adjacent | companion or neighboring tool | move / displacement |
| `Mod+Ctrl` | Focused | inspect, control, or focused subtask | reshape / merge / compact |
| `Mod+Alt` | Alternate | specialist, alternate, or wider context | change scope |

Combined modifiers compose those meanings. They do not change the House owned
by an anchor key.

## Inventory Baseline

The source profiles inspected for this draft are:

- personal profile: `~/.config/niri/config.kdl`
- General Edition: `packaging/hollow-grove-general/etc/skel/.config/niri/config.kdl`

Both contain 214 binding entries after locking Walker to Flynt and removing its
former Q alias. All 214 binding chords now match exactly. Shared chord semantics
also match for the reconciled Space, HJKL conveyor, and Hueman threshold
families.

There are also intentional command-path differences: the personal profile
uses local programs and paths, while the General Edition uses portable Hollow
Grove launch helpers. Those are packaging adaptations, not keyboard-semantic
changes.

## Anchor Binding Ledger

### Stonebend — Escape

| Current binding | Current action | Reading | Disposition |
|---|---|---|---|
| `Mod+Escape` | close focused window | close the present boundary | Keep |
| `Mod+Shift+Escape` | lock session | hold the whole session | Keep |
| `Mod+Ctrl+Escape` | quit Niri | end the compositor boundary | Keep |
| `Mod+Alt+Escape` | close nested River scope | close an alternate boundary | Keep |
| `Mod+Ctrl+Alt+Escape` | reboot | re-establish the system state | Keep |
| `Mod+Ctrl+Alt+Shift+Escape` | power off | final stable stop | Keep |

The existing Escape family is already a complete Stonebend family. No move is
proposed.

### Sandmanor — Enter, the forward threshold

| Current binding | Current action | Reading | Disposition |
|---|---|---|---|
| `Mod+Return` | open Kitty | enter the command method | Keep |
| `Mod+Shift+Return` | open Kitty with Zellij | enter the structured visual method | Keep |
| `Mod+Ctrl+Return` | open Kitty with Bash | inspect the direct shell method | Keep |
| `Mod+Alt+Return` | launch Hollow Grove / Hueman | cross into the alternate Hollow Grove context | Keep |
| `Mod+Alt+Ctrl+Return` | rebind the Hueman pair | revise the integrated method and commit it | Keep |
| `Mod+Alt+Shift+Return` | open nested River terminal scope | enter a displaced alternate scope | Keep |
| `Mod+Alt+KP_Enter` | launch Hollow Grove / Hueman | keypad threshold alias | Keep |

Terminal actions remain on Enter because a terminal is where a method is
entered and committed. They do not need to move to Flynt merely because a
terminal can execute commands.

### Sandmanor — Backspace, the returning threshold

| Current binding | Current action | Reading | Disposition |
|---|---|---|---|
| `Mod+Alt+BackSpace` | clear the Hueman pair | return the alternate presentation for reconsideration | Keep |
| `Mod+Ctrl+Alt+BackSpace` | reset focused window height | withdraw structural alteration | Keep |

The existing Backspace actions already express revision and return. No new
destructive meaning should be added here.

### Glaüshouse — Space

| Current binding family | Current action | Reading | Disposition |
|---|---|---|---|
| `Mod+Space` | cycle column width forward | adjust usable room | Official Step 2 primary |
| `Mod+Shift+Space` | cycle column width backward | restore the preceding room | Official Step 2 reverse |
| `Mod+Ctrl+Space` | set column width to 25% | restore the default quarter-width chamber | Official shared expression |
| `Mod+Ctrl+Shift+Space` | set focused window height to 100% | displaced structural clearance | Official shared expression |
| `Mod+Alt+Space` | expand to available width | occupy the safely available room | Keep |
| `Mod+Alt+Shift+Space` | perform Hueman step-spread | alternate presentation clearance | Official shared expression with profile-specific launcher |
| `Mod+Alt+Ctrl+Space` | toggle windowed fullscreen | widen the permitted presentation | Official shared expression |
| `Mod+Ctrl+Alt+Shift+Space` | set column width to 25% | legacy alias for the base quarter-width chamber | Keep |

The Space family is already conceptually Glaüshouse: it cares for the room in
which the focused window can live. The former four profile differences are
reconciled to distinct, shared semantics; only executable adaptation differs.

### Flynt — Grave/Tilde

There were no `Grave` bindings in either profile before this pass, so Flynt's
anchor was added without displacement or conflict.

| Proposed binding | Action | Reading | Disposition |
|---|---|---|---|
| `Mod+Grave` | open Walker | direct Flynt invocation | Official exclusive binding |
| `Mod+q` | unbound | former Walker position | Removed by explicit lock |

`Mod+Grave` replaces `Mod+q` as Walker's sole entrance. More Grave combinations
should remain open until normal use reveals which Flynt actions deserve
landmark access.

## Complete Family Migration Ledger

This table covers the remaining binding families. A House classification
describes the dominant intent of the family; it does not require relocating
the family onto an anchor.

| Current binding family | Current launch family or action | Dominant reading | Disposition |
|---|---|---|---|
| Q variants excluding plain `Mod+q` | Firefox, Anyrun, Chromium | Sandmanor query and browsing | Keep; plain `Mod+q` stays open |
| `Mod+w` variants | Helix, Obsidian, Glow, Lapce | Sandmanor design, writing, and review | Keep |
| `Mod+e` variants | game emulators | Flynt execution and testing | Keep |
| `Mod+r` variants | Waveform, Ardour, Audacity, REAPER | Flynt production through practiced tools | Keep |
| `Mod+t` variants | Taskwarrior and Planify | Sandmanor method and planning | Keep |
| `Mod+y` variants | AudioTube, YouTUI, FreeTube, YewTube | Flynt retrieval and transmission | Keep |
| `Mod+u` variants | Yazi and disk inspection | Stonebend custody with Sandmanor inspection | Keep |
| `Mod+i` variants | time, calendar, weather, radar | Sandmanor measurement and comparison | Keep |
| `Mod+o` variants | Office | Sandmanor design and records | Official Step 3 domain anchor |
| `Mod+p` variants | battery and power profiles | Stonebend stability and capacity | Keep |
| `Mod+a` variants | mail and notification history | Flynt transmission with Stonebend record | Keep |
| `Mod+s` variants | system, sensor, and GPU monitors | Sandmanor diagnosis | Keep |
| `Mod+d` variants | downloads and package tools | Flynt retrieval with Glaüshouse maintenance | Keep |
| `Mod+f` variants | fullscreen, tabbed, and edge framing | Stonebend boundary | Keep |
| `Mod+g` variants | AI assistants | Sandmanor proposal, comparison, and method | Keep |
| `Mod+h/j/k/l` | focus left/down/up/right | Flynt/Glaüshouse/Stonebend/Sandmanor spatial diamond | Keep |
| `Mod+Shift+h/j/k/l` | move left/down/up/right | same spatial diamond with displacement | Keep |
| `Mod+Ctrl+h/j/k/l` | consume, resize, and expel | same spatial diamond with structural change | Keep |
| higher-modifier `h/j/k/l` | wrap and transport windows or columns | same spatial diamond across wider scope | Keep; four-direction conveyor reconciled across profiles |
| bracket families | focus or move to horizontal edges | Stonebend boundary addressing | Keep |
| comma/period families | consume, expel, media previous/next, center | Glaüshouse shaping plus Flynt sequence | Keep as paired punctuation |
| semicolon family | floating/tiling focus and state | Sandmanor comparison of presentation modes | Keep |
| `Mod+Alt+Arrow` | focus monitor or workspace by direction | spatial scope | Keep |
| `Mod+1..0` | focus workspace 1 through 10 | Stonebend address and standing | Keep |
| `Mod+Shift+1..0` | send window to workspace and stay | Flynt movement from a stable viewpoint | Keep |
| `Mod+Ctrl+1..0` | send column to workspace and stay | Flynt structural transport | Keep |
| `Mod+Ctrl+Shift+1..0` | send column and follow | Flynt transport and passage | Keep |
| `Mod+z` variants | LMMS, image viewer, PDF inspection, Okular | Sandmanor reading with Flynt production | Keep |
| `Mod+x` variants | LocalSend, Croc, Wormhole receive/send | Flynt exchange and transmission | Keep |
| `Mod+c` variants | screenshots | Stonebend witness and durable capture | Keep |
| `Mod+v` variants | PipeWire status, mixers, audio graph | Glaüshouse adjustment and system care | Keep |
| `Mod+b` variants | Bluetooth tools | Sandmanor diagnosis with Flynt connection | Keep |
| `Mod+n` variants | network tools | Sandmanor diagnosis with Flynt connection | Keep |
| `Mod+m` variants | Music | Flynt performance with Glaüshouse regulation | Official Step 3 domain anchor |
| `Mod+Tab` variants | overview and previous focus at several scopes | native Niri traversal | Keep; Tab is not a House anchor in this map |
| minus/equal families | dynamic cast, OBS, screenshot-to-clipboard | Flynt transmission with Stonebend capture | Keep |
| `XF86Audio*` | volume and mute | Glaüshouse system care | Keep native media placement |
| `XF86MonBrightness*` | brightness | Glaüshouse environmental adjustment | Keep native media placement |
| `Print` | screenshot | Stonebend witness | Keep native placement |

## What Does Not Move

The following decisions are explicit:

- bare Escape, Backspace, Enter, Space, Grave/Tilde, letters, and numbers keep
  their ordinary application behavior;
- HJKL remains Niri focus/move/reshape/scope and also becomes the geographic
  House diamond;
- number keys remain workspace addresses;
- the application launch grid remains mnemonic;
- `O` and `M` are the official Office and Music domain anchors; their modifiers
  stay inside those domains;
- `Mod+Grave` is Walker's exclusive binding and plain `Mod+q` remains open;
- hardware media, brightness, and Print keys retain native placement;
- personal commands may differ from portable package helpers while preserving
  the same key meaning.

This draft does not impose the Rule of Twelve across every letter row. That
pattern can become a later optional overlay, but it is not allowed to erase a
working binding merely to make the keyboard visually regular.

## Reconciliation Lock

Profile reconciliation is complete:

1. The General Edition's distinct Space expressions are the shared baseline.
2. `Mod+Ctrl+Shift+h/j/k/l` is the symmetric four-direction conveyor in both
   profiles; `Mod+Ctrl+j/k` retains forward and reverse height cycling.
3. The personal Hueman bindings use the active repository helpers, while the
   General Edition uses the portable `hollow-hueman` contract.
4. The verifier checks the reconciled semantic ledger, identical chord
   inventory, canonical domain overlays, duplicate chords, executable personal
   reconciliation paths, deterministic map output, and both Niri profiles.

Future behavioral divergence requires an explicit amendment. Packaging-only
path or optional-application guards remain ordinary adaptations when they
preserve the registered semantic action.

## Maintenance Order

1. Amend this constitution before changing a locked anchor or shared semantic.
2. Update the registry and both profiles in the same change.
3. Regenerate the map and run the keyboard verifier against both profiles.
4. Exercise safe changed bindings in normal use before using the portable
   profile as an image source.

The personal profile remains the behavioral reference. The General Edition
may replace hard-coded personal paths with helpers and optional-application
guards, but it must preserve the same recall grammar.
