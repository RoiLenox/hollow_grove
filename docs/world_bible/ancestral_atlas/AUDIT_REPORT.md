# Ancestral Atlas Integration Audit

Date: 2026-08-01<br>
Status: implementation audit; observational, not constitutional authority

## Current Atlas structure

Before this work, the active repository had no `docs/world_bible/` Atlas implementation. The first and integration passes created 6 Age indexes, 40 supplied Point pages, 160 full assignment records, 4 House lineages, 4 cultural syntheses, 4 libraries, registries, templates, reports, and read-only JSON metadata. Chronology Revision 1 now publishes 41 operational Point pages and 164 assignments while retaining the supplied forms and legacy lineages in [its revision report](CHRONOLOGY_REVISION_01_REPORT.md) and [Master Timeline](MASTER_TIMELINE.md).

## Relevant existing lore audited

- [Hollow Grove Compromise](../../../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md) and [Constitutional Architecture](../../../HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md) establish the authority ceiling.
- All four V2 House constitutions define present House jurisdiction: [Stonebend](../../../STONEBEND_CONSTITUTION_V2.md), [Sandmanor](../../../SANDMANOR_CONSTITUTION_V2.md), [Glaüshouse](../../../GLAUSHOUSE_CONSTITUTION_V2.md), and [Flynt](../../../FLYNT_CONSTITUTION_V2.md).
- [Constitutional Geography](../../../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md), [Functional Lore](../../../HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md), [Hueman](../../../HUEMAN_v0.1.0.md), and [Hueman Faculties](../../../HUEMAN_FACULTIES_V1.md) supply the place, lived-lore, player, and faculty boundaries.
- [Visual Color Constitution](../../../HOLLOW_GROVE_VISUAL_COLOR_CONSTITUTION.md) and [Visual Bible](../../../HOLLOW_GROVE_VISUAL_BIBLE_V1.md) require the universal dark-blue foundation and House-specific translation.
- The repository uses JSON for schemas, fixtures, palettes, and artifacts; Atlas metadata therefore uses JSON and adds no runtime loader.

## Connections now formalized

- Every assignment connects culture → Gift IDs → one reviewed Point manifestation → existing House canon → Hueman reference ID; all 164 assignments now have a reviewed dossier path.
- Each House has a cumulative ancestry plus a separate modern cultural synthesis, preventing the Point chronology from being duplicated as prose.
- Gifts cover all Points and assignments; manifestations provide exactly one reviewed Working flagship per House assignment, 41 per House.
- Libraries are cultural institutions with historical wings, foundational book records, professional collections, programs, curriculum links, and read-only navigation hooks.
- Cultural memory distinguishes common, educated, and specialist knowledge.
- All creator-facing proposals require an ancestral provenance block.

## Missing connections found

- Existing House, geography, visual, and Hueman documents contained no structured historical provenance IDs.
- The foundational book lists had no repository records, Point links, cultural association status, or secondary-study handling.
- Existing runtime/data schemas had no Atlas-compatible descriptive record. The proposal now documents one without binding it to production.
- Ages IV–VI use completion-level Point source bundles and explicit uncertainty fields. Their culture-specific specialist bibliographies may deepen, but no assignment remains without a dossier, Gift boundary, manifestation, and source trail.

## Duplicates avoided

- Constitutional House descriptions, geography, routes, palettes, offices, and faculties remain in their existing authorities and are linked rather than copied into new law.
- Chronology lives in Point pages and the preserved master timeline; synthesis files summarize recurring patterns and cite Points rather than reproducing full assignment records.
- The JSON file mirrors identifiers for tools but is explicitly non-authoritative and read-only.

## Contradictions and uncertainties

1. **Flynt law correction resolved:** the canonical House Law is **Law of Capacity**. Recognition remains a permitted Festival, confirmation, honor, reputation, and judgment theme, but is not the House Law.
2. **Point I — Sumer:** chronology crosses late Uruk, Jemdet Nasr, and Early Dynastic phases; earliest proto-cuneiform cannot always be assigned securely to Sumerian language.
3. **Point I — Early Dynastic Egypt:** Menes is a later tradition and is not silently equated with Narmer or Aha; image-objects are not treated as transparent event reports.
4. **Point I — Early Indus:** the Point crosses the c. 2600 BCE Early/Mature Harappan boundary. Mature drainage, reservoirs, and urban standardization must not be backdated automatically.
5. **Point I — Elam:** `Elam` is retained as supplied, but c. 3200–2900 evidence is Proto-Elamite; its script is not fully deciphered and its language is not proven to be Elamite.
6. **Point II — Akkad:** Agade remains unlocated; control varied across regions, and royal universal-conquest language is not treated as an administrative map.
7. **Point II — Old Kingdom Egypt:** the supplied range covers late Dynasty 5, Dynasty 6, and the terminal transition—not the whole pyramid age. Earlier Giza evidence remains context rather than a Point II Gift source.
8. **Point II — Mature Indus:** script, names, political offices, and religion remain undeciphered or unknown; shared weights and material forms do not prove one centralized government.
9. **Point II — Awan:** the place is unlocated, most evidence is Mesopotamian and adversarial, the Naram-Sin treaty remains difficult, and Puzur-Inshushinak falls at or just after the Point boundary.
10. **Point III — Ur III:** the dynasty conventionally spans c. 2112–2004 BCE and therefore ends near the Point midpoint; administrative abundance does not make every province uniform, and labor documentation includes coercive contexts.
11. **Point III — Middle Kingdom Egypt:** the period begins around 2030/2055 BCE within the supplied range and continues long after it; later Twelfth- and Thirteenth-Dynasty evidence cannot be backdated into this reviewed slice.
12. **Point III — Minoan Crete:** c. 2100–1900 is primarily late Prepalatial EM III–MM IA. First-palace institutions sit at or after the endpoint, `Minoan` is a modern label, and named rulers or readable doctrine are unavailable.
13. **Point III — Šimaški:** exact geography and internal institutions remain unresolved; the principal king list is retrospective, the fall of Ur was multi-causal, and late Šimaški overlaps the early Sukkalmah/Epartid transition.
14. **Point IV — Babylon:** the First Dynasty substantially spans the Point under the Middle Chronology, but Babylon becomes hegemonic only late under Hammurabi; “Code” is retained as a conventional object title while the Atlas describes the monument as royal jurisprudence rather than a modern comprehensive code.
15. **Point IV — later Middle Kingdom Egypt:** most chronologies end the period around 1700 or 1650/1640 BCE, before the supplied 1600 endpoint; Dynasty 13 and the Second Intermediate boundary remain contested.
16. **Point IV — Minoan Crete:** the coordinate crosses Protopalatial and early Neopalatial systems near c. 1700; Cretan Hieroglyphic and Linear A remain interpretively limited, first-palace destructions do not have one proven cause, and older centralized priest-king models are not treated as facts.
17. **Point IV — Old Assyria:** the conventional period label reaches c. 1600, but the richest Kültepe records cluster in the nineteenth and eighteenth centuries; `kārum` is treated as a merchant institution within Anatolian polities, not modern colonial sovereignty.
18. **Point V — New Kingdom Egypt:** the period begins near 1550 but continues to c. 1070, well beyond the supplied endpoint; Ramesses III and Medinet Habu belong chiefly after this reviewed slice, so the terminal label is not an Egyptian fall.
19. **Point V — Hittite Empire:** the imperial phase enters mainly around 1400 and central rule at Hattusa ends around 1200/1180; Hittite-related dynastic and cultural continuity at Carchemish and other successor centers prevents an extinction narrative.
20. **Point V — Kassite Babylonia:** early succession after 1595 is obscure, consolidation is clearest from c. 1475, Nippur's archive is geographically skewed, and the dynasty continues beyond 1200 to c. 1155.
21. **Point V — Middle Assyria:** imperial relevance enters around 1365 and continues beyond 1200; administrative archives expose deportation, forced labor, and dependent status that cannot become Capacity Gifts.
22. **Point V terminal label:** `Bronze Age Collapse` is retained exactly as supplied but interpreted as an uneven regional transition involving different destruction, contraction, persistence, migration, and institutional transformation trajectories rather than one event or cause.
23. **Point II:1 — Kingdom of Israel:** the supplied kingdom label spans evidence of an externally named people, archaeologically varied Iron I highland communities, disputed early monarchy, and the northern kingdom emerging near c. 930. Samaria and the strongest state evidence belong after 900; no single conquest or united-monarchy model is treated as settled.
24. **Point II:1 — Phoenicia:** the label describes independent coastal cities with different histories, not one state. The consonantal alphabet was adapted and transmitted through these networks rather than invented at one moment, and later western colonial expansion is not backdated across the whole Point.
25. **Point II:1 — Aram:** `Aram` is an external and changing regional designation, not one state, ethnicity, or art style. City-state and Old Aramaic evidence becomes clearest near or after the Point's tenth–ninth-century boundary; later imperial lingua-franca status is not projected backward.
26. **Point II:1 — Neo-Assyria:** only c. 911–900 of the supplied Point belongs conventionally to Neo-Assyria. Ashur-dan II is transition context and Adad-nirari II supplies the boundary slice; later capitals, relief programs, siege systems, and mature imperial communications remain outside this dossier.
27. **Point II:2 — Israel:** the supplied shorthand now refers to the northern kingdom centered on Samaria, not the united-monarchy question, Judah, or all later uses of the people-name. Assyrian reduction in 732 and the fall of Samaria form a terminal sequence; whether conquest belongs to Shalmaneser V, Sargon II, or multiple actions remains disputed.
28. **Point II:2 — Phoenicia:** independent cities, rulers, ports, and settlement networks followed different trajectories under growing Assyrian pressure. The regional label is retained, but no unified Phoenician state, policy, or colonial program is inferred.
29. **Point II:2 — Aram:** multiple local polities and mixed cultural fields remain primary. Arpad fell in 740 and Damascus in 732, while Hamath's final annexation around 720 lies just beyond the Point endpoint; annexation did not end Aramaic language or local memory.
30. **Point II:2 — Assyria:** the supplied label is operationally Neo-Assyrian and spans expansion, internal conflict, magnate power, plague, revolt, reform, and succession. Royal victory rhetoric, deportation, extraction, and coercion are evidence and warnings, never Capacity Gifts.
31. **Point II:3 — Judah:** the kingdom continues beyond the Point to 586 BCE. Sennacherib's 701 campaign devastated Lachish and much of Judah while Jerusalem survived under tribute; later reform and literary-development models remain debated and require source-layer separation.
32. **Point II:3 — Egypt, Twenty-Fifth and Twenty-Sixth Dynasties:** the composite label crosses an overlapping transition. Dynasty 25 loses rule in Egypt by 656 BCE while Kushite political and cultural continuity persists to the south; Dynasty 26 begins in 664 and continues beyond 612. The inheritance cannot be reduced to a single national restoration narrative.
33. **Point II:3 — Babylon:** the label crosses direct Assyrian rule, appointed and local kings, Marduk-apla-iddina's resistance, the 689 destruction, Esarhaddon's rebuilding, Shamash-shuma-ukin's rebellion, and Nabopolassar's regime from 626. It is a city-and-institutional continuity label, not one dynasty or policy.
34. **Point II:3 — Neo-Assyria:** the Point contains both Sargonid apex and collapse. Nineveh fell in 612, but Assyrian kingship and allied resistance continued at Harran under Ashur-uballit II until 609; the terminal event is therefore capital-terminal rather than the exact endpoint of the polity.
35. **Point II:4 — Judah:** the kingdom exists only to 587/586; the supplied label then changes referent to homeland, Babylonian, Egyptian, and other communities. Not all Judahites were deported, the common 586 date has a 587/586 scholarly convention, and 539 is not one universal return.
36. **Point II:4 — Saite Egypt:** Dynasty 26 spans the full Point and continues to 525. Naukratis is an Egyptian multi-community harbor rather than simply a Greek colony; foreign military and diplomatic personnel had varied status, and later Herodotus is not a contemporary administrative archive.
37. **Point II:4 — Neo-Babylonian Empire:** the dynasty spans the coordinate and ends in 539, but Babylonian city, temple, family, legal, and scholarly institutions continue. Archive abundance is uneven, and royal construction cannot erase conquest, deportation, extraction, or labor coercion.
38. **Point II:4 — Medes:** Cyaxares's coalition role and Astyages's defeat are strong anchors, but independent Median kingship ends around 550, eleven years before the Point endpoint. A Median empire's extent and administration, Ecbatana's form, site ethnicity, religion, and cavalry reform remain contested.
39. **Point II:5 — Achaemenid:** the empire spans the Point and continues from the 336 boundary to 330. Regional units, local law, languages, archives, taxation, labor, and royal oversight varied; `satrapy`, trilingual communication, and plural administration cannot be generalized as one uniform or tolerant constitutional system.
40. **Point II:5 — Greek:** the supplied umbrella covers politically independent and differently governed poleis, ethnē, leagues, kingdoms, and communities inside Achaemenid territory. Athenian evidence is exceptionally dense but not pan-Greek; civic participation excluded many residents, and alliance could become coercive empire.
41. **Point II:5 — Late Egypt:** the label crosses Saite independence to 525, first Achaemenid rule to 404, independent Dynasties 28–30 to 343, and renewed Achaemenid rule through the endpoint and to 332. Institutional continuity, restoration, resistance, and collaboration must be attached to site, phase, language, and source.
42. **Point II:5 — Macedonia:** the Argead kingdom spans the Point, but Philip II's integrated late-fourth-century system belongs only to 359–336. Early institutions are unevenly evidenced; army reform, manpower, resource control, coalition, conquest, destruction, and hegemonic settlement need source and phase attribution.
43. **Legacy Sandmanor lineage:** the supplied master lineage assigns `Kingdom of Israel` at Age II, Point I, while the Point record assigns `Phoenicia`. Point pages and navigable House lineages follow the Point record; the legacy line remains verbatim.
44. **Legacy label drift:** Stonebend's legacy `Israel` abbreviates `Kingdom of Israel`; Glaüshouse's `Early Indus` and `Mature Indus` abbreviate the Point labels; Sandmanor's `Egypt` abbreviates `Egypt, Twenty-Fifth and Twenty-Sixth Dynasties`.
45. **Point II:6 transition:** the supplied 336–281 coordinate is retained, but its assignments have different timelines. Achaemenid sovereignty ends in 330 while institutional practices continue; `Macedonia` changes from Argead kingdom to imperial and successor claim; Ptolemaic rule in Egypt begins as a 323 satrapy and becomes a kingdom in 305; Seleucid formation is traced from the post-323 contests through the 312 Babylonian recovery and royal title from 305. The 281 boundary closes the first successor-generation formation rather than a universal stabilization.
46. **Point II:7 long phase:** the supplied 281–146 coordinate remains one mature-Hellenistic comparison, but stable Antigonid rule begins only in 277/276 and ends in 168; the 168–146 portion records post-dynastic Macedonian transition. `Hellenistic Greek` is a bounded civic-federal umbrella rather than one polity. Ptolemaic and Seleucid systems undergo major internal ruptures and both continue after 146. Corinth's destruction is therefore a mainland Greek and Roman-Mediterranean transition, not a common end for all four assignments.
47. **Long staggered Point resolved structurally:** supplied Age III, Point VI (1204–1368) is now operationally divided into Point VI-A (1204–1279) and Point VI-B (1279–1368). Their individual assignments remain Working and need dossier-level research.
48. **Modern institutional label corrected operationally:** supplied `European Union and Western Europe` is retained in the revision record; the active Point uses `European Communities and Western Europe`, with the 1993 EU transition stated in its relevance range.
49. **AI Age volatility:** present-day assignments and technology emphases require recurring review and must not be treated as timeless.
50. **Nazi Germany:** the supplied Stonebend assignment is preserved. Its only Gifts are documentation of atrocity, warnings against totalitarian law, and institutional refusal; no celebratory or aesthetic inheritance is permitted.
51. **Book-to-Point fit:** several foundational books arise from cultures not primary to their library House at the selected Point, or from cultures absent from the Point set. They are explicitly secondary or unassigned studies and need House-fit review.
52. **Book titles and collections:** `The Path of Aloneness / Dokkōdō` is preserved exactly and flagged for title/translation verification; `works on living systems, cognition, biology, ecology, and memory` is a collection brief rather than one bibliographic work.
53. **Broad culture labels:** `Greek`, `Hellenistic Greek`, `Western Europe`, `European Union`, `China`, and similar labels are too broad for a single exhaustive dossier. The reviewed Greek dossiers are explicitly bounded umbrella audits, and later labels remain Working because rewriting them would alter supplied data.

## Existing systems that could later consume references

- library navigation and museum exhibits;
- Hueman NPC background, curriculum, artifact, and district-authoring tools;
- read-only world inspection and provenance presentation;
- future quest, profession, music, architecture, or ambient-design authoring.

None is implemented as runtime behavior in this pass.
