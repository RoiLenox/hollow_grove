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
| A | AI | Artificial intelligence |
| B | Browse | Web |
| C | Code | Source code |
| D | Downloads | Acquire files |
| E | Email | Mail |
| F | Files | Filesystem |
| G | Games | PLAY |
| H | ← Movement | Flynt |
| I | Inspect | Look inside |
| J | ↓ Movement | Glaüshouse |
| K | ↑ Movement | Stonebend |
| L | → Movement | Sandmanor |
| M | Music | HEAR |
| N | Network | Connections |
| O | Office | Productivity |
| P | Photos | Photography |
| Q | Query | Find information |
| R | Record | Capture |
| S | Settings | Configure |
| T | Time | Clock and timing |
| U | Utilities | System tools |
| V | Videos | WATCH |
| W | Wrappers | Windows compatibility |
| X | eXecute | Run commands and workflows |
| Y | sYnc | Folder/device sync |
| Z | Zip | Archives |

Capitalization in `eXecute` and `sYnc` is intentional: it exposes the physical
letter used by a word whose natural initial is already claimed. The uppercase
hooks PLAY, HEAR, and WATCH are mnemonic emphasis, not alternate names.

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
tool. X now means eXecute under the adopted amendment; its alternate branch
remains empty until another distinct execution method is selected.

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
- active personal readable standard: `~/.config/niri/HOLLOW_GROVE_KEYBOARD_STANDARD_V2.md`
  (the V1 file remains the frozen migration record);
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

## Adopted amendment — A / AI (2026-10-10)

At the user's direction, A changes from Assets to AI (Artificial Intelligence).
Its personal implementation is a local AI cluster: Primary is everyday local
chat, Adjacent is a reasoning-oriented form, Focused is model inspection and
selection, and Alternate is another local chat model. The HOW grammar and
all other letter meanings remain unchanged. Creation applications remain
available through the existing launcher; they no longer define A.

The AI keyboard domain does not expand the authority of any Hollow Grove
AI persona or grant a chatbot control of the constitutional runtime.

## Adopted amendment — C / Code (2026-10-10)

At the user's direction, C changes from Chat to Code (source code). Its branches
remain within writing, understanding, and modifying source code: Primary is
Codex, Adjacent is graphical coding in Zed, Focused is terminal editing in
Helix, and Alternate is OpenCode. Applications remain replaceable implementations.
The HOW grammar and all other letter meanings are unchanged. D / Develop and
E / Edit retain their existing shortcuts pending their individual reviews.
Displaced communication tools remain available through discovery and Controls.

## Current implementation refinement — manual Code / AI agents

The user clarified C as manual coding and A as AI-assisted/automatic work.
Codex is A Primary and OpenCode A Adjacent. A Focused retains local model
inspection/selection; A Alternate retains the personal Hollow Grove model
as work in progress. Coding agents use their existing provider configurations;
this shortcut refinement does not change model-provider or approval settings.

C Primary is Helix, C Adjacent is Lapce, and C Focused is Zed, implementing
the requested Helix/Zed swap. C Alternate is unbound pending a distinct manual
workflow. These supersede the preceding application assignments, without
changing AI / Code meanings or HOW grammar. D / Develop and E / Edit remain
unchanged until their own reviews.

## Adopted amendment — D / Downloads (2026-10-10)

The user accepted D as Downloads, replacing Develop. D acquires files;
F manages files already on the computer. Primary is qBittorrent's graphical
torrent workflow; Adjacent is rTorrent's terminal torrent workflow. Focused
is reserved for distinct queue/transfer control and Alternate for another
download method; these branches remain honestly unbound in both profiles.
No new download tools are installed simply to occupy these slots. Coding
agent and source-editing routes remain in A and C. The HOW grammar and all
other WHAT meanings remain unchanged.

## Adopted amendment — E / Email (2026-10-10)

The user selected E as Email, replacing Edit. Source-code editing remains in C.
Email Primary is Thunderbird's graphical client, Adjacent is Himalaya's
terminal interface, and Focused inspects Himalaya account definitions. Alternate
remains unbound pending another chosen client/service. Applications remain
replaceable implementations behind Email's stable meaning. The HOW grammar
and every other WHAT meaning remain unchanged.

## Adopted amendment — I / Inspect (2026-10-10)

The user selected I as Inspect, replacing Images. Its atomic purpose is to
examine system state: Primary observes live processes with htop, Adjacent
shows a system snapshot with Fastfetch, Focused reads hardware Sensors, and
Alternate examines user-session logs with Journalctl. S configures system
preferences; I provides inspection views; U remains the broader utility
family pending its own review. Existing U bindings are not removed during
this amendment. imv remains available through application discovery.

Inspect's memory hook is Look inside. The HOW grammar and all other WHAT
meanings remain unchanged.

## Adopted amendment — N / Network (2026-10-10)

The user selected Network, replacing News. Its atomic domain is the computer's
connections: Primary opens connection management, Adjacent edits connection
profiles (including VPN where supported by installed plugins), Focused inspects
connection state, addresses, routes and DNS, and Alternate opens remote access.
The memory hook is Connections. The HOW grammar remains unchanged.
Dedicated network branches move from S into N. The personal Settings menu
remains the general configuration entrance; emptied Settings branches await
that family's review. A remote-access shortcut asks for a destination and
never chooses a host or initiates a connection automatically. Applications
remain replaceable implementations; movement and layout are preserved.

## Adopted scope refinement — Q / Query (2026-10-10)

The user accepted Query as finding information on the computer. Primary
finds files by name, Adjacent searches their text contents, and Focused
finds local system manuals. Alternate remains empty. Query's memory hook
is Find information. Grave/Tilde remains Navigate, with Walker and Anyrun
serving application and window navigation; their bindings are unchanged.

Gemini, Ollama's launcher, and AIChat move from Q into the Ctrl+A AI tool
chooser, alongside the existing local-model inspector. A Primary remains
Codex, Adjacent OpenCode, and Alternate the Hollow Grove model in progress.
AI provider settings and approval behavior are unchanged. The WHAT meanings
and HOW grammar remain stable; this refinement clarifies their boundary.

## Adopted amendment — T / Time (2026-10-10)

The user rejected Tasks and Transfer, selected Time, and requested a large
peaclock clock on Mod+T. Primary now opens the current time as a large
responsive digital clock. The memory hook is Clock and timing. Other
Time branches remain empty until separately selected; the former Taskwarrior
and Planify shortcuts are removed from T, while their installed tools remain
available independently. The HOW grammar and every other key meaning remain
unchanged. Clock sizing occurs inside Kitty and preserves niri movement and
layout controls, the locked Record cluster, Office, and Grave navigation.

## Adopted refinement — V / Videos, local library (2026-10-10)

The user retained Videos as V's domain and requested a local movie collection
fed by Downloads. Primary becomes Kodi's local library, Adjacent opens the
collection folders, Focused selects a local video for mpv, and Alternate retains
FreeTube as an online viewing route. The memory hook remains WATCH.

Movie and TV-show catalog roots are under the user's Videos directory; their
Downloads directory remains a separate source accessible in Kodi. Download
clients and torrent paths remain unchanged. Choosing the Movies/TV Shows
folder as a download destination feeds the corresponding catalog. The HOW
grammar, other key meanings and movement/layout controls remain unchanged.

## Adopted W / Wrappers amendment — 2026-10-10

The user adopted W as Wrappers / Windows compatibility, replacing Write.
Its atomic domain is the compatibility machinery used to run Windows software
on Hollow Grove. Mod+W opens Bottles; Shift picks an executable and delegates
bottle selection to Bottles; Ctrl selects a bottle and opens its Windows
configuration; Alt opens Protontricks for Steam/Proton compatibility repair.
G remains Games. Bottles manages dependencies through its primary hub.
Existing writing applications remain installed and available from launchers.
No movement, layout, Office, or Record bindings change under this amendment.

## Adopted X / eXecute amendment — 2026-10-10

The user adopted X as eXecute / Run commands and workflows, replacing eXchange.
Mod+X runs a command in the user's login shell; Shift selects an executable
script with a preview; Ctrl selects a project and one of its available tasks.
Output remains visible in Kitty. Alt stays reserved. C remains manual coding,
I remains inspection, and Grave retains application/window navigation.
Browsing projects or previews never executes their task definitions.
All bindings outside X remain unchanged under this amendment.

## Adopted Y / synchronization and Z / archives refinement — 2026-10-10

The user delegated completion of the remaining Y and Z clusters. Y's atomic
domain is folder/device synchronization: Primary Syncthing, Adjacent Grsync,
Focused Syncthing status and user-service controls, Alternate reserved for cloud
sync. Its old one-time transfer tools remain installed but lose Y bindings.
Z retains Zip / Archives and provides an archive manager, creation from a
folder, extraction with a destination chooser, and terminal content inspection.
The final consistency pass routes shared media transport to the most recently
active supported desktop player via playerctld/playerctl, replacing retired
cmus-only personal routes. Movement, layout, Office, and Record remain verbatim.
