# Hollow Deck V1

Status: canonical gameplay-data catalog schema 1.6.0 and terminal compendium;
persisted player-state envelope V1

`HOLLOW DECK` is Hollow Grove's central index for the supernatural cultivation
system. It is not a generic class picker, bestiary, or spreadsheet. The typed
catalog lives in `src/hollow_deck.rs`, its canonical seed projection lives in
`src/hollow_deck_seed.rs`, and terminal presentation lives in
`src/hollow_deck_tui.rs` plus `src/bin/hollow_deck.rs`. Renderer and asset
composition guidance lives in `HOLLOW_ART_BIBLE_V1.md`.

The former constitutional reconciliation hold is resolved by
`HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md`. Schema
1.6.0 locks Manticorp as combined synthesis, Chimera as perfected synthesis,
Hecaton as the perfected-force Right Hand, Pegasus as the perfected-motion Left
Hand, and The Sandman as the sovereign culmination above both. Existing Nature
IDs and persisted state V1 remain stable.

## Hierarchy

```text
Hueman (50 / 50 / 50 / 50)
  → cultivates a Nature
    → belongs to a House + Dominion
    → materializes through Current + Aura Material
    → has an independent Current Form (body)
    → has an independent Aura Set (mind)
    → may learn a collectible Synthesis Recipe
      → which produces a named Ability or Characteristic
```

Ordinary `Recipe` remains a separate first-class type for real-world crafting.
An `EvolutionPath` relates one Nature to another; it is not a Current Form.
The seed carries four reciprocal Flynt pairs. Manticorp/Chimera peer combat
mechanics do not reverse their directional synthesis order.

The Hollow Deck authoritatively places Stonebend at North, Sandmanor at East,
Glaüshouse at South, and Flynt at West. Glaüshouse is the South, Hollow Current,
Precog House and exposes two coequal Aura Materials in one alchemical position:

- Presensor is the Matron/Jade branch: Pixie → Fairy → Nymph → Muse, with
  Detection → Mending → Restoration → Hope.
- Preceptor is the Marshall/Emerald branch: Sprite → Spirit → Sylph → Siren,
  with Prediction → Projection → Deflection → Compulsion.
- Its five ordered seats are Calli, Cleo, Melony, Teri, and Talia. A member may
  be `Nightingale de Melony`; an achieved branch title is either
  `Matron de Melony` or `Marshall de Melony`. Branch and role are parallel
  identities, not sequential ranks.

The five Aura Materials occupy four House slots. Jade and Emerald share the
Glaüshouse slot and are neutral to each other. The soft Aura cycle is:

```text
Diamond → Glaüshouse (Jade / Emerald) → Opal → Crystal → Diamond
```

Favorable Aura/applicable Synthesis effects use `×1.10`, unfavorable effects
use `×0.90`, and neutral relationships use `×1.00`. Pure Current/physical
actions do not automatically receive the material modifier.

## Visual asset roles

Hollow Grove is a modern game using Pokémon Yellow's visual discipline. Deck
data should eventually project into separate Overworld, Deck, Encounter /
Battle, and optional Expression / Portrait asset classes; one class is not a
default substitute for another. Every Nature must first read through 16×16
actor logic, while larger representations may carry more expression without
abandoning silhouette, palette, and animation economy.

Visual production proves this grammar with the Hueman base and then the full
Gargoyle transformation vertical slice before expanding across all Natures.
The authoritative restrictions and production order live in
`HOLLOW_ART_BIBLE_V1.md`; this catalog continues to own identity and progression
meaning rather than sprite implementation.

## Flynt contextual visual identities

Flynt's principal visual identities are separate presentation dimensions over
one biological Nature. They are neither additional Natures nor evolution or
growth stages:

| Nature | Mystery Men | We Fairy Men | The Gallows | Symbol |
|---|---|---|---|---|
| Gargoyle | Hunter | Bro White | Johnny Apple | Apple |
| Werewolf | Chief | Beauty the Beast | Tommy Rose | Rose |
| Mariner | Mariner | Cindarellaman | Jimmy Pumps | Shoe / pump |

Nature defines the body. Mystery Men supplies a hidden, disciplined, occult
professional presentation; We Fairy Men supplies rural workwear, travel,
mutual-aid, reputation, and folk-legend presentation; The Gallows supplies a
contemporary urban, dock, club, office, obligation, and underworld
presentation. The renderer selection type keeps `nature`, `growth_stage`,
`organization`, `outfit`, `accessories`, `environment`, and `animation_state`
separate, then derives the social identity from the organization.

Apple, rose, shoe/pump, midnight, carriage, and related motifs must remain
subtle iconography or environmental storytelling. Literal fairy-tale cosplay,
generic wizard robes, uniformed-army We Fairy Men, and stereotypical gangster
costumes are excluded. Existing constitutional offices named `Beauty` and
`Cinderellaman` remain untouched; `Beauty the Beast` and `Cindarellaman` are
the exact contextual visual identities in this Deck schema.

## Institutional grammars

Biology, biological growth, citizenship, service, mastery, specialist
discipline, Synthesis/apex attainment, operating organization, social alias,
and visual presentation are separate catalog axes. Institutional or social
presentation never automatically changes Nature.

The four active House grammars are intentionally asymmetric:

```text
GLAÜSHOUSE
Nightingale de <Seat>
        │
    ┌───┴───┐
 Matron   Marshall
Presensor Preceptor
  Jade     Emerald

FLYNT
Warden → Corpman → prove Capacity → Mystery Man
                                      ↓
                         Hunter | Chief | Mariner
                            ╲       │       ╱
                    Current | Aura | Weapon tracks
                                    ↓
                         Manticorp / combined
                                    ↓
                          Chimera / perfected

Separate Flynt synthesis-manufacturing system:
Male    Wizard → Warlock → Sorcerer
Female  Witch → Enchantress → Sorceress
Peers   Wizard ↔ Witch; Warlock ↔ Enchantress; Sorcerer ↔ Sorceress

STONEBEND
Gerald → Apprentice
          ├─ Mason Current + Mage Diamond Aura
          │       → Freemason / Superstar
          └─ Pugilist Current + Foreman Diamond Aura
                  → Proliteriate / Rockstar

Freemason / Superstar + Proliteriate / Rockstar → Hypergiant

SANDMANOR
              Minorian                         Minoan
Social        Hearthkeeper → Householder       Beach Bum → Party Boy
                       → Husbandman                      → Socialite
Aura          Geomancer → Labyrinthian          Acolyte → Navigator
                       → Demiurge                        → Ascendant
Nature        Gnome → Faun → Minotaur           Elf → Satyr → Centaur
Hand          Hecaton                           Pegasus

Demiurge + Ascendant → The Transcendent → The Sandman
```

Every Mystery Man was a Corpman; not every Corpman becomes a Mystery Man.
Corpman is general service and is not Manticorp. Hunter, Chief, and Mariner are
parallel sects mapped respectively to Gargoyle/Fortification,
Werewolf/Siphone, and Mariner/Anchoring. Their three isolated disciplines feed
independently refinable Current, Aura, and Weapon Synthesis tracks. Each track
may reach Manticorp combination and Chimera perfection independently; neither
state is a fourth ordinary sect. The canonical isolated weapons are
Gargoyle/Hunter **Bolo**, Werewolf/Chief **Crossbow**, and Mariner/Mariner
**Harpoon**. These weapon doctrines are authored canon; catalog schema 1.6.0
projects the amended synthesis order. Weapon inventory, combat behavior, and
the combined/perfected weapon mechanisms remain unimplemented.

Flynt sorcery is a separate paired synthesis-manufacturing and formulation
ladder. Wizard/Witch manufacture established Synthesis Recipes and products;
Warlock/Enchantress combine, counterfeit, corrupt, strengthen, weaken,
repurpose, or reformulate existing patterns; Sorcerer/Sorceress author new
Synthesis Recipes and patterns. The Tross governs the wider ecosystem, while
Mystery Men regulate and interdict, The Gallows distributes through urban vice
and commerce, We Fairy Men supply rural legitimacy and mutual aid, and The
Marquise extends maritime routing. Uncontrolled synthesis is the systemic
danger; sanctioned sorcery practitioners are not automatically criminal,
hostile, villainous, or anti-Tross. Sorcery is neither a Nature path, a
Sandmanor Aura ladder, nor a Mystery Men sect.

The seed's reciprocal Manticorp/Chimera Nature-pair combat relationship uses
`Convergence` and `Divergence`. Those mechanics are orthogonal peer behavior:
directional synthesis still runs Manticorp → Chimera. Manticorp also names the
combined-synthesis military institution; institutional membership alone does
not grant the state.

Flynt's We Fairy Men, The Gallows, and The Marquise are operating contexts,
not career steps. The Marquise is a maritime extension with a strong Mariner
affinity. The existing rural/Gallows alias matrix remains separate from formal
service.

Stonebend's active citizen/training entry is `Gerald → Apprentice`; Applicant
is retired from active canon. Mason and Mage are parallel competencies whose
completed Synthesis identity is Freemason, culturally also Superstar. Pugilist
and Foreman are parallel competencies whose completed Synthesis identity is
Proliteriate, culturally also Rockstar. Neither completed mastery is above the
other; their integration produces Hypergiant.

Stonebend's institutional grammar does not change its biological path:

```text
Gremlin → Goblin → Ghoul → Spectre → Troll → Orc → Ogre → Troglodyte
```

Presynce remains bounded perception of matter, structure, support, load,
fault, and physical relationship. The Cut selects the physical boundary where
one material state, relationship, or structure becomes another. Hollowing
remains the terminal Stonebend Nature/process expression. The Deck's mastery
register also does not overwrite the existing constitutional Claim/Yield/Title
meanings of Freemason, Proliteriate, and Hypergiant.

### Sandmanor's two complete cultures

Sandmanor is East, Regular Current, Crystal, and the physical philosophy of
The Way. Minorian and Minoan are parallel ways of life, not sequential ranks.
Each has separate social/Current, Aura/faculty, and Nature ladders:

| Axis | Minorian | Minoan |
|---|---|---|
| Social / Current | Hearthkeeper → Householder → Husbandman | Beach Bum → Party Boy → Socialite |
| Aura / faculty | Geomancer → Labyrinthian → Demiurge | Acolyte → Navigator → Ascendant |
| Biological Forms | Gnome → Faun → Minotaur | Elf → Satyr → Centaur |
| Complete Nature/Hand branch | Gnome → Faun → Minotaur → Hecaton | Elf → Satyr → Centaur → Pegasus |
| Sovereign Hand | Hecaton / Right Hand | Pegasus / Left Hand |
| Primary Trait | Grip → Traction → Leverage → Multiplication | Balance → Agility → Momentum → Flight |

The Minorian social movement is maintain → organize → steward. Hearthkeeper
maintains immediate continuity and is the baseline counterpart to Beach Bum;
Householder organizes a full household and pairs with Party Boy's organization
of a gathering; Husbandman is the apex identity of mature material and
resource stewardship and pairs with Socialite's mature stewardship of social
and network relationships. Husbandman is not a beginner farmer rank.

The Minoan social movement is move freely → create connection → master
networks. Beach Bum is low-attachment coastal mobility and adaptability;
Party Boy activates social environments through timing, attention,
invitation, and connection; Socialite carries reputation and access between
networks, groups, places, and relationships.

Prefog asks, “What is going to be needed?” It anticipates pressure, need,
shortage, support requirements, and continuity demands, including resource
depletion, structural strain, household and land needs, environmental
pressure, and future support requirements. It is not generic prophecy.
Geomancer reads place and asks what that place permits. Labyrinthian reads the
dependencies of a system and asks where an effect travels when a condition
changes. Demiurge makes a system by arranging conditions into a functioning,
sustainable world; it means mature systems construction, not the omnipotent
creation of a universe.

Prefig asks, “What is this becoming?” It anticipates form, trajectory,
possibility, social movement, and emerging opportunity, including routes
opening, groups forming, events becoming important, opportunities emerging,
trends gaining momentum, and trajectories becoming viable. It is not generic
fortune-telling. Acolyte is a disciplined initiate who recognizes possibility,
not a generic religious follower. Navigator selects and maintains a route
through changing possibility. Ascendant exceeds the limits of existing
trajectories and possibility structures.

The Aura pairings are Geomancer ↔ Acolyte, Labyrinthian ↔ Navigator, and
Demiurge ↔ Ascendant. Demiurge and Ascendant are peer apex worldviews:

```text
Demiurge + Ascendant → The Transcendent → The Sandman
```

The Transcendent masters The Way between world-making stability and
world-exceeding possibility: the Minorian says to build the house, the Minoan
says to leave the house, and The Transcendent knows when to do either. The
Transcendent is not an ordinary social rank or Nature rung. The Sandman is the
sovereign embodiment of The Transcendent and elevates the folkloric Sandman
archetype without importing Morpheus, Hypnos, or another separate deity.
Hecaton and Pegasus complete their Nature/Hand branches but are separately
invested rather than ordinary biological Form transitions. Hecaton is perfected
force and the Right Hand; Pegasus is perfected motion and the Left Hand. Where
Pegasus wings are shown, visual development may use Buds → Winglets → Wings.
The Sandman remains above both as command of the complete pair.

Culture, social/Current identity, Aura mastery, Nature, and sovereign synthesis
remain separate typed dimensions. Husbandman is not Hecaton, Socialite is not
Pegasus, Demiurge is not Hecaton, and Ascendant is not Pegasus.

## Terminal entrypoints

```bash
# Interactive indexed deck (TTY only)
cargo run --bin hollow_deck --offline

# Integrated launcher
HOLLOW_GROVE_DEV_BUILD=1 cargo run --bin hollow-grove --offline -- deck

# Non-interactive cards and boards
cargo run --bin hollow_deck --offline -- home
cargo run --bin hollow_deck --offline -- natures --house Flynt --sort total
cargo run --bin hollow_deck --offline -- nature gargoyle
cargo run --bin hollow_deck --offline -- compare gargoyle medusa
cargo run --bin hollow_deck --offline -- glaushouse
cargo run --bin hollow_deck --offline -- muse-seats
cargo run --bin hollow_deck --offline -- institutions
cargo run --bin hollow_deck --offline -- institutions flynt
cargo run --bin hollow_deck --offline -- institutions stonebend
cargo run --bin hollow_deck --offline -- institutions glaushouse
cargo run --bin hollow_deck --offline -- institutions sandmanor
cargo run --bin hollow_deck --offline -- flynt-identities
cargo run --bin hollow_deck --offline -- flynt-identities gargoyle
cargo run --bin hollow_deck --offline -- progression
cargo run --bin hollow_deck --offline -- counterparts
cargo run --bin hollow_deck --offline -- sandmanor
cargo run --bin hollow_deck --offline -- current-forms gargoyle
cargo run --bin hollow_deck --offline -- alchemy
cargo run --bin hollow_deck --offline -- synthesis-recipes
cargo run --bin hollow_deck --offline -- validate
cargo run --bin hollow_deck --offline -- export-json

# Persisted player cultivation (basis points: 0–10000)
cargo run --bin hollow_deck --offline -- cultivate gargoyle 5000
cargo run --bin hollow_deck --offline -- set-growth gargoyle "Wing Buds"
cargo run --bin hollow_deck --offline -- set-growth gargoyle Winglets
cargo run --bin hollow_deck --offline -- set-growth gargoyle Wings
cargo run --bin hollow_deck --offline -- set-technique werewolf siphone Rush
cargo run --bin hollow_deck --offline -- learn-aura werewolf scent
cargo run --bin hollow_deck --offline -- equip-aura werewolf scent
cargo run --bin hollow_deck --offline -- state
```

Interactive keys follow the terminal client's compact keyboard language:
arrow keys or `j`/`k` select, Enter or `l` opens, `h`/left returns, `/` searches
the Nature index, `c` opens comparison, Tab changes the active comparison side,
`+`/`-` develops or reduces the selected Nature's Current expression in 5%
steps, `?` opens help, and `q` quits. House accents are resolved from the existing
visual color constitution rather than copied into the Deck.

## Storage boundary

The canonical catalog remains read-only. Player-owned cultivation is stored as
a separate sparse, versioned overlay at
`artifacts/gameplay-saves/hollow-deck-state-v1.json`. Set
`HOLLOW_DECK_STATE_PATH` to use another save location. Writes use the existing
atomic artifact path and the envelope carries an integrity checksum; invalid
references, invented stages, incompatible Aura moves, equip-before-learn, and
unmet Synthesis requirements are rejected before commit. `export-json` is a
canonical projection for inspection and tooling, while `export-state` projects
the active overlay. Neither becomes a second canonical source of truth.

Catalog schema `1.6.0` and persisted state V1 are deliberately separate version
domains. The state envelope version, sparse state's `schema_version`, default
`hollow-deck-state-v1.json` filename, and checksum contract remain V1. On load,
the checksum is verified against the serialized legacy payload before any
normalization. Valid legacy `gargoyle.feature_stages.wings` values map as
`Buds → Wing Buds`, `Winglets → Winglets`, and `Wings → Wings` in the unified
growth field. A legacy known Synthesis ID `white-opal-statue` is removed and
advances Gargoyle growth to `White Opal Statue Mastery` without discarding a
later stage or unrelated state. Loading performs this normalization in memory;
it does not rewrite the save until a later explicit state-changing command
writes it. Missing files still load as clean empty state.

Gargoyle biological growth is one cumulative ordered path:

```text
Fangs → Claws → Horns → Wing Buds → Winglets → Wings
      → Mature Fortification → White Opal Statue Mastery
```

The current stage implies every prior milestone. Continuous
`development_basis_points` remains independent; the Deck invents no percentage
thresholds for discrete growth. White Opal Statue is inherent mature
Fortification, not a Recipe, Synthesis Recipe, or Synthesis. Discovery, trial,
and clearance for `Winged Gargoyle` remain owned by `recipe_discovery.rs`; the
Deck keeps it as a Wings-or-later safe-flight/grounded-return Synthesis and does
not treat it as the source of biological wings.

The existing root-crate executable `synthesis_recipe::SynthesisRecipe` and the
Flynt constitution's composition `SynthesisRecipe` remain separate bounded
domains. Hollow Deck's collectible `hollow_deck::SynthesisRecipe` is an index
record with compatibility and requirement metadata; it does not replace either
engine.

## Compatibility notes

The Deck follows the active terminology in this milestone while preserving old
runtime/archive identifiers outside the Deck:

- Hollow Deck uses `Orc`; older Frame codecs retain `Ork`.
- Hollow Deck uses the two Glaüshouse gameplay branches above; older
  constitutional records retain their own clinical topology and historical
  pairing order.
- Hollow Deck schema 1.6.0 retains the stable Flynt Nature IDs `manticorp` and
  `chimera`, but locks the direction Manticorp combined → Chimera perfected.
  Tross-specific `flynt.form.manticore` remains an external compatibility ID,
  not a Deck tier after Chimera.
- Hollow Deck schema 1.6.0 retains Hecaton and Pegasus as final Nature-path
  projections of separately invested Hands. The constitutional biological
  Form enum remains three stages per branch; this is an axis boundary rather
  than a nonconformance.
- Hollow Deck uses Acolyte → Navigator → Ascendant as the current Minoan Aura
  ladder. Witch, Wizard, Sorcerer, and Warlock are not current Sandmanor Aura
  ranks; those terms belong to Flynt's sorcery and synthesis-manufacturing
  ecosystem. Historical documents remain historical rather than being
  silently rewritten.

## Validation

```bash
cargo test --test integration hollow_deck --offline
cargo test --test hollow_deck --offline
cargo test --test hollow_deck_state --offline
cargo test --bin hollow_deck --offline
cargo test --bin hollow-grove --offline main_cli_supports_integrated_commands
cargo clippy --lib --bin hollow_deck --bin hollow-grove --offline -- -D warnings
```
