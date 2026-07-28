# Flynt Constitution V2

Status: canonical and locked.

This document is the repository-facing constitutional authority for Flynt's institutional architecture, stable identity relationships, and maintained Synthesis hierarchy. `flynt-constitution` is the executable authority. World catalogs, Current Synthesis projections, Hueman, Godot, artifacts, and examples must conform to it and may not create a parallel Flynt hierarchy.

## Common constitutional spine

```text
Tross
  |
  v
Chimera
```

Tross is Flynt's sovereign executive. Every Flynt institution and office ultimately derives constitutional authority from Tross. The Tross is the formal constitutional leader and commander-in-chief of Manticorp.

There is exactly one constitutional Chimera. Chimera is the First Companion of Tross and stands immediately beneath Tross. Chimera is the unique constitutional synthesis of Flynt's three founding regional peoples:

```text
Gargoyle * Merman * Werewolf -> Chimera
```

Chimera is a constitutional companion and maintained lower-apex Synthesis, not an office. Manticorp is both a distinct maintained Synthesis Form and a distinct military institution named after that Form. No ordinary Manticorp member becomes the Form merely by serving in the institution, and no personal creature-progression sequence grants the Tross office.

### Absolute identity invariant

**Tross = Mystery Man = Mr. X = presently maintained living holder of the Manticorp Synthesis Form.**

These names do not identify separate people. They are one stable person with
different public, underground, classified, operational, and constitutional
expressions.

## Urban expression

```text
Tross (public sovereign)
  ├── Manticorp Institution (military command)
  │     └── Mystery Men (classified bureau)
  └── The Gallows (underground command)
        └── Mystery Man / Mr. X (same Tross, operational face)
```

Manticorp is Flynt's formal military institution. It is responsible for territorial defense, military command, constitutional protection, disciplined force, military training, and lawful deployment. The Tross is its formal constitutional leader and commander-in-chief: he recognizes its command structure, authorizes major deployments and emergencies, and maintains the Manticorp Form. The institution is larger than the officeholder; ordinary personnel are not Manticorp Forms.

Mystery Men is Flynt's one unified federal investigative bureau. It combines investigation, intelligence, counterintelligence, covert operations, organized-crime investigation, contraband enforcement, espionage response, and constitutional security. The agency is publicly recognized; its operations are highly classified.

The Mystery Man and Mr. X are the Tross's underground, classified, and operational identities. Under these faces, the same man leads the Gallows and may act through illicit trade, black-market Recipes, prohibited Synthesis materials, smuggling, debts, favors, covert enforcement, political pressure, and deniable violence. He is not the Tross's subordinate or separate agent.

## Rural expression

```text
Tross (underground sovereign face)
  |
  v
The Gallows
  |
  v
We Fairy Men and the Basin bands
```

The Gallows is Flynt's underground Yakuza-, mafia-, and organized-crime-like body. It is organized through families and regional crews, loyalty, territory, favors, obligation, contraband routes, black-market Recipes, and cultural identity. Many citizens question whether it exists.

We Fairy Men is the legendary folk expression of the Gallows: a traveling band, roaming crew, body of folk heroes, and local protectors. Music is one cultural expression and not the whole institution.

The Gallowry is the hidden home of the Gallows. It is a site, not an institution and not a synonym for the Gallows. Its canonical functions are meeting place, headquarters, cultural center, gallery, and operational hub.

## Founding leaders and constitutional union

The Gallows recognizes exactly three enduring Founding Leader offices. In the maintained Manticorp Recipe, these same leaders are the three indispensable Basin custodians; their traditions remain complete cultures and independent powers, not mere ingredients:

| Founding people | Founding Leader office | Crew | Associations |
|---|---|---|---|
| Gargoyle | Bro White | Bro White and the 7 Brothas | southern Flynt, stone, architecture, guardianship |
| Merman | Cinderellaman | Cinderellaman and His Midnight Crew | Riptide, Aura Sea, waterways, smuggling, midnight transformation |
| Werewolf | The Beauty | The Beauty and His Beasts | northern Flynt, wilderness, packs, roaming protection |

Successors may inherit a Founding Leader office. Inheritance changes the holder, not the office's identity or constitutional place. The custodians remain distinct and no one fragment is sufficient to reproduce, repair, stabilize, restrain, renew, or transfer Manticorp.

```text
Bro White * Cinderellaman * The Beauty
                    |
                    v
              We Fairy Men
                    |
                    v
                 Chimera
```

This union is the constitutional folk expression of cooperation among distinct traditions. It does not permanently fuse Bro White, Cinderellaman, and The Beauty into one person or Form, and it does not erase their separate crews, territories, loyalties, or Recipe custody.

## Dual expression law

Urban and rural Flynt are complementary expressions of one authority. They are not competing governments.

| Urban | Rural |
|---|---|
| state | private |
| federal | regional |
| institutional | folk |
| professional | traveling |
| Manticorp and Mystery Men | the Gallows, We Fairy Men, and the Basin bands |

Both expressions answer to the same sovereign Tross, while Chimera remains the lower-apex First Companion and Manticorp remains the distinct maintained Form beyond it. The public and underground commands are intentionally unresolved political tension: the Tross may control the Gallows to protect Flynt, or use lawful authority to protect and expand it. Neither reading is canonically forced.

## Executable mapping

The authoritative implementation lives in `officials-and-outlaws/src/lib.rs`; the source directory name remains only as a migration boundary, while its package and crate name is `flynt-constitution` / `flynt_constitution`.

Canonical domain identifiers:

- `flynt.office.tross`
- `flynt.person.tross`
- `flynt.identity.mystery-man`
- `flynt.identity.mr-x`
- `flynt.companion.chimera`
- `flynt.institution.manticorp`
- `flynt.form.manticorp`
- `flynt.recipe.divided-manticorp`
- `flynt.institution.mystery-men`
- `flynt.expression.the-mystery-man`
- `flynt.institution.gallows`
- `flynt.expression.we-fairy-men`
- `flynt.site.gallowry`
- `flynt.office.bro-white`
- `flynt.crew.bro-white-and-the-7-brothas`
- `flynt.office.cinderellaman`
- `flynt.crew.cinderellaman-and-his-midnight-crew`
- `flynt.office.the-beauty`
- `flynt.crew.the-beauty-and-his-beasts`

The neutral world projection lives in `src/world/flynt.rs`. It uses neutral `InstitutionCatalog` records only for presentation and access integration. It does not own or reinterpret constitutional law.

`cargo run --bin flynt_constitutional_audit` executes the repository-level audit. It validates the domain, the neutral world projection, and the checked documentation before reporting placements and uniqueness counts.

## Validation lock

A conforming repository must prove:

- one Tross root and no second sovereign Flynt executive;
- exactly one constitutional Chimera;
- exactly one superior for every non-root authority node;
- all authority paths terminate at Tross;
- no authority cycles or duplicate placements;
- the exact urban and rural chains above;
- exactly one Chimera recipe with Gargoyle, Merman, and Werewolf as its sources;
- exactly one presently maintained Manticorp Form distinct from the Manticorp institution;
- one divided Manticorp Recipe with indispensable Gargoyle, Werewolf, and Merman custodians;
- the Tross identity lock binds the public office, Mystery Man, Mr. X, Gallows leadership, and Manticorp Form to one person;
- Manticorp institution command terminates at Tross, while ordinary personnel remain non-Form members;
- the Gallowry exists only as the hidden Gallows site;
- exactly three Founding Leader offices with their canonical crews and lineages;
- deterministic validation independent of insertion order;
- the recursion kernel remains neutral and contains no Flynt semantics.

Tross succession beyond the identity and supremacy of the office is not specified here. Implementations must reject invented succession rules rather than derive the office from transformation, mastery, recognition, or Synthesis.

The neutral world projection uses `being.flynt.tross` as the stable active presentation identity of the canonical Tross. `flynt.person.tross`, `flynt.identity.mystery-man`, and `flynt.identity.mr-x` are aliases and projections of that same person, not additional beings. This is not a succession candidate or a succession rule.

## Synthesis Continuance law

A true Synthesis changes the Hueman's actual body. It is not a costume,
detachable class, temporary loadout, or combat-only power-up. It establishes
real strengths, weaknesses, bodily and psychological needs, losses,
institutional recognition, environmental requirements, treatment obligations,
and future paths while maintained. Its Continuance depends on Recipe practice,
the Form's Ways, bodily discipline, renewal, environmental compatibility, and
institutional care. Visible suppression, Aspect emphasis, regulation, disguise,
and controlled heightened states may express the maintained body; regression,
revision, discontinuation, replacement, and collapse are distinct recorded
lifecycle events.

**Synthesis does not grant a costume. It establishes a new body. Every new body creates powers, weaknesses, needs, losses, obligations, and relationships.**

**Synthesis is not permanence. Synthesis is Continuance through renewal.**

**The Form is real while sustained. Its future depends on the Recipe, the Ways,
the Hueman, and the conditions that keep it coherent.**

The physical Flynt grammar is Wolf → Lion, Bat → Eagle, and Snake/Fish → Hydra.
Chimera integrates the three lower principles and supports meaningful internal
refinement; Manticorp is the sovereign transfiguration beyond Chimera. Manticorp
is the Tross's presently maintained body, with exceptional output and Resynce as well as
metabolic demand, instability, identity risk, environmental vulnerabilities,
maintenance needs, political fear, and dependence on divided custodians and
Glaüshouse care. The Tross may maintain Manticorp throughout his natural life
through bodily discipline, Recipe renewal, divided Basin knowledge,
institutional recognition, and specialized Glaüshouse care.

## Manticorp Recipe and Synthesis Ledger

The complete Manticorp Recipe is divided among Bro White (Gargoyle body,
density, anchoring, endurance), The Beauty (Werewolf instinct, pursuit, pack
synchronization, living force), and Cinderellaman (Merman adaptation, flow,
Current redistribution, traversal, recovery). No ordinary archive, person, or
institution holds the complete reproducible Recipe.

Glaüshouse's Synthesis Ledger is the authoritative body of knowledge for
maintained Forms. It records strengths, weaknesses, bodily and environmental
needs, Recipe, Ways, maintenance, renewal, expected Continuance, regressions,
collapse risks, psychological effects, lost capacities, complications,
medicines, grafts, dangerous interactions, recovery, warning signs, social and
legal condition, institutional dependencies, stabilization, revision,
restraint, and Severance. Public, Living, Recipe, and Black access layers
separate accommodations from patient histories, production methods, and
forbidden discoveries. Persephones govern Living Ledger viability and Prima
Donna governs Recipe Ledger transformation.

The Ledger is simultaneously medical text, constitutional registry, trade
catalog, intelligence archive, vulnerability record, and political leverage.
Its use remains a constitutional tension: care and accommodation can become
withheld treatment, artificial dependency, price manipulation, disclosure,
anti-Form weapons, or Illegal Synthesis.

## Service Tournament Interface

At the Service Tournament, Flynt, MI expresses the fixed **ATF & Army**
reference as one Flynt cultural-government identity. Combat engineering,
controlled mechanisms, breaching, explosives knowledge, fortification, route
creation, territorial control, field improvisation, and lawful recognition of
force remain subject to Tross, the existing Manticorp hierarchy, and Flynt
law. No separate agency team, army government, or parallel command is created.
The Gallows remains the unlawful or unrecognized mirror.

Flynt owns the Black family in the War of a Thousand Hues, expressed through
the established onyx, obsidian, gunmetal, Rich Black Blue, and related dark
mixtures rather than featureless pure black. Tournament victory grants no
permanent sovereignty and does not alter Flynt or another House's succession.

Tournament Synthesis records distinguish inward **Resynce** during Aim from
outward **Recog** after an accepted Kiss. Resynce does not itself create Recog,
and display animation creates neither. Flynt's flint, edge, spark, activation,
and mechanism tradition may inform prize materials and lawful refinements,
including recorded contributions to the Edge of Tomorrow, without replacing
the Edge's singular identity or turning custody into ownership.
