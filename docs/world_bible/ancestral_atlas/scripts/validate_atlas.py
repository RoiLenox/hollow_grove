#!/usr/bin/env python3
"""Validate Ancestral Atlas coverage, references, and local navigation."""

from __future__ import annotations

import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXPECTED_POINT_COUNTS = {1: 5, 2: 12, 3: 8, 4: 5, 5: 6, 6: 5}
EXPECTED_POINT_TOTAL = sum(EXPECTED_POINT_COUNTS.values())
EXPECTED_ASSIGNMENT_TOTAL = EXPECTED_POINT_TOTAL * 4
HOUSES = {"stonebend", "sandmanor", "glaushouse", "flynt"}


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def validate_data(errors: list[str]) -> dict:
    data_path = ROOT / "data" / "atlas.json"
    data = json.loads(data_path.read_text(encoding="utf-8"))

    if data.get("framework_status") != "canonical_descriptive_layer":
        fail(errors, "framework status is not canonical_descriptive_layer")
    if data.get("assignment_status") != "working":
        fail(errors, "assignment status is not working")
    if len(data.get("ages", [])) != 6:
        fail(errors, f"expected 6 Ages, found {len(data.get('ages', []))}")

    point_counts = {age["number"]: len(age["points"]) for age in data["ages"]}
    if point_counts != EXPECTED_POINT_COUNTS:
        fail(errors, f"Point counts differ: {point_counts}")
    if sum(point_counts.values()) != EXPECTED_POINT_TOTAL:
        fail(errors, f"expected {EXPECTED_POINT_TOTAL} Points")

    point_records = [point for age in data["ages"] for point in age["points"]]
    point_ids = [point["id"] for point in point_records]
    if len(point_ids) != len(set(point_ids)):
        fail(errors, "duplicate Point IDs")
    allowed_types = {"sequential", "transition", "nested", "open"}
    for point in point_records:
        if point.get("point_type") not in allowed_types:
            fail(errors, f"invalid Point type: {point.get('id')} -> {point.get('point_type')}")
        if not point.get("boundary_rationale"):
            fail(errors, f"Point is missing boundary rationale: {point.get('id')}")
        if point.get("point_type") == "open" and (not point.get("as_of") or not point.get("review_due")):
            fail(errors, f"Open Point is missing review dates: {point.get('id')}")
    age_three_six = {
        (point["coordinate"], point["title"])
        for point in data["ages"][2]["points"]
        if point["number"] == 6
    }
    expected_split = {
        ("6A", "Mongol Conquests and Late Song"),
        ("6B", "Khanates, Yuan, and Mamluk World"),
    }
    if age_three_six != expected_split:
        fail(errors, f"Age III Point VI split differs: {age_three_six}")
    points_by_id = {point["id"]: point for point in point_records}
    expected_point_types = {
        "atlas.age_01.point_03": "transition",
        "atlas.age_02.point_01": "transition",
        "atlas.age_02.point_06": "transition",
        "atlas.age_02.point_08": "nested",
        "atlas.age_03.point_05": "transition",
        "atlas.age_06.point_02": "nested",
        "atlas.age_06.point_05": "open",
    }
    for point_id, expected_type in expected_point_types.items():
        if points_by_id.get(point_id, {}).get("point_type") != expected_type:
            fail(errors, f"Chronology Revision 1 Point type differs: {point_id}")
    expected_titles = {
        "atlas.age_03.point_05": "Reform, Seljuk, and Crusader World",
        "atlas.age_05.point_01": "Industrial Acceleration",
    }
    for point_id, expected_title in expected_titles.items():
        if points_by_id.get(point_id, {}).get("title") != expected_title:
            fail(errors, f"Chronology Revision 1 title differs: {point_id}")

    assignments = data.get("assignments", [])
    if len(assignments) != EXPECTED_ASSIGNMENT_TOTAL:
        fail(errors, f"expected {EXPECTED_ASSIGNMENT_TOTAL} assignments, found {len(assignments)}")
    by_point = defaultdict(list)
    for record in assignments:
        coordinate = (record["age"], record["point"], record.get("point_suffix", ""))
        by_point[coordinate].append(record)
        if record["status"] != "working":
            fail(errors, f"assignment not Working: {record['id']}")
        for field in ("point_coordinate", "temporal_fit", "relevance_range", "relevance_note"):
            if not record.get(field):
                fail(errors, f"assignment missing {field}: {record['id']}")
        if len(record.get("gifts", [])) < 3:
            fail(errors, f"assignment has fewer than three Gifts: {record['id']}")
    for coordinate, records in by_point.items():
        houses = {record["house"] for record in records}
        if len(records) != 4 or houses != HOUSES:
            fail(errors, f"Point {coordinate} does not contain exactly four Houses: {houses}")
    digital_glaushouse = next(
        (
            record for record in assignments
            if record["age"] == 6 and record["point_coordinate"] == "3" and record["house"] == "glaushouse"
        ),
        None,
    )
    if not digital_glaushouse or digital_glaushouse["culture"] != "European Communities and Western Europe":
        fail(errors, "Age VI, Point III does not use the revised European Communities label")
    assignments_by_id = {record["id"]: record for record in assignments}
    expected_temporal_fits = {
        "atlas.age_01.point_03.stonebend.ur_iii": "Partial / Terminal",
        "atlas.age_01.point_03.sandmanor.middle_kingdom_egypt": "Emergent / Partial",
        "atlas.age_01.point_03.glaushouse.minoan_crete": "Boundary / Transforming",
        "atlas.age_01.point_03.flynt.simaski": "Partial / Transition",
        "atlas.age_01.point_04.stonebend.babylon": "Full / Transforming",
        "atlas.age_01.point_04.sandmanor.middle_kingdom_egypt": "Partial / Terminal",
        "atlas.age_01.point_04.glaushouse.minoan_crete": "Full / Transforming",
        "atlas.age_01.point_04.flynt.old_assyria": "Full period label / Uneven evidence",
        "atlas.age_01.point_05.stonebend.new_kingdom_egypt": "Full / Continues",
        "atlas.age_01.point_05.sandmanor.hittite_empire": "Partial / Terminal",
        "atlas.age_01.point_05.glaushouse.kassite_babylonia": "Full / Extends beyond boundary",
        "atlas.age_01.point_05.flynt.middle_assyria": "Partial / Continues",
        "atlas.age_02.point_01.stonebend.kingdom_of_israel": "People full / Kingdom emergent",
        "atlas.age_02.point_01.sandmanor.phoenicia": "Full regional continuity / Emergent label",
        "atlas.age_02.point_01.glaushouse.aram": "Emergent / Dispersed",
        "atlas.age_02.point_01.flynt.neo_assyria": "Boundary / Eleven-year slice",
        "atlas.age_02.point_02.stonebend.israel": "Full / Terminal",
        "atlas.age_02.point_02.sandmanor.phoenicia": "Full / Continues",
        "atlas.age_02.point_02.glaushouse.aram": "Full regional horizon / Uneven terminal",
        "atlas.age_02.point_02.flynt.assyria": "Full / Continues",
        "atlas.age_02.point_03.stonebend.judah": "Full / Continues",
        "atlas.age_02.point_03.sandmanor.egypt_twenty_fifth_and_twenty_sixth_dynasties": "Dynasty 25 full / terminal; Dynasty 26 partial / continues",
        "atlas.age_02.point_03.glaushouse.babylon": "Full / Regime-changing / Continues",
        "atlas.age_02.point_03.flynt.neo_assyria": "Full / Nineveh-terminal; polity continues",
        "atlas.age_02.point_04.stonebend.judah": "Kingdom partial / terminal; communities continue",
        "atlas.age_02.point_04.sandmanor.saite_egypt": "Full / Continues",
        "atlas.age_02.point_04.glaushouse.neo_babylonian_empire": "Full / Terminal; institutions continue",
        "atlas.age_02.point_04.flynt.medes": "Partial / Terminal; legacy continues",
        "atlas.age_02.point_05.stonebend.achaemenid": "Full / Continues into Point VI",
        "atlas.age_02.point_05.sandmanor.greek": "Full regional horizon / Politically plural",
        "atlas.age_02.point_05.glaushouse.late_egypt": "Full regional horizon / Multiple regimes / Continues",
        "atlas.age_02.point_05.flynt.macedonia": "Full kingdom horizon / Late-dominant",
        "atlas.age_02.point_06.stonebend.achaemenid": "Political 336–330 / Institutional legacy thereafter",
        "atlas.age_02.point_06.sandmanor.macedonia": "Full horizon / Referent transforms",
        "atlas.age_02.point_06.glaushouse.ptolemaic": "Egyptian precursor 332–323 / Satrapy 323–305 / Kingdom 305–281",
        "atlas.age_02.point_06.flynt.seleucid": "Precursor 323–315 / Formation 312–305 / Kingdom 305–281",
        "atlas.age_02.point_07.stonebend.antigonid": "Precursor 281–277/276 / Kingdom 277/276–168 / Post-dynastic transition 168–146",
        "atlas.age_02.point_07.sandmanor.hellenistic_greek": "Full civic-federal horizon / Politically plural / Continues",
        "atlas.age_02.point_07.glaushouse.ptolemaic": "Full / Multiple internal ruptures / Continues to 30 BCE",
        "atlas.age_02.point_07.flynt.seleucid": "Full / Major contraction after 188 / Continues to 63 BCE",
        "atlas.age_02.point_11.flynt.sasanian_persia": "Partial",
        "atlas.age_06.point_03.glaushouse.european_communities_and_western_europe": "Full / Renamed",
    }
    for assignment_id, expected_fit in expected_temporal_fits.items():
        if assignments_by_id.get(assignment_id, {}).get("temporal_fit") != expected_fit:
            fail(errors, f"Chronology Revision 1 temporal fit differs: {assignment_id}")
    reviewed_levels = {"reviewed_benchmark", "reviewed_dossier"}
    reviewed_assignments = [
        record for record in assignments if record.get("review_level") in reviewed_levels
    ]
    for coordinate, records in by_point.items():
        reviewed_at_coordinate = [
            record for record in records if record.get("review_level") in reviewed_levels
        ]
        if reviewed_at_coordinate and len(reviewed_at_coordinate) != len(records):
            fail(errors, f"reviewed Point is incomplete across Houses: {coordinate}")
    for record in reviewed_assignments:
        if not record.get("dossier_document"):
            fail(errors, f"reviewed assignment is missing dossier_document: {record['id']}")
        if len(record["gifts"]) < 3 or len(set(record["gifts"])) != len(record["gifts"]):
            fail(errors, f"reviewed assignment lacks at least three distinct Gifts: {record['id']}")

    benchmark_assignments = by_point[(1, 1, "")]
    if any(record.get("review_level") != "reviewed_benchmark" for record in benchmark_assignments):
        fail(errors, "Age I, Point I assignments are not all marked reviewed_benchmark")
    point_three_assignments = by_point[(1, 3, "")]
    point_four_assignments = {
        record["house"]: record for record in by_point[(1, 4, "")]
    }
    for house in ("sandmanor", "glaushouse"):
        point_three_record = next(record for record in point_three_assignments if record["house"] == house)
        if set(point_three_record["gifts"]) & set(point_four_assignments[house]["gifts"]):
            fail(errors, f"Point III reviewed slice silently reuses Point IV phase Gifts: {house}")
    gift_ids = [record["id"] for record in data.get("gifts", [])]
    if len(gift_ids) != len(set(gift_ids)):
        fail(errors, "duplicate Gift IDs")
    gift_id_set = set(gift_ids)
    gifts_by_id = {record["id"]: record for record in data["gifts"]}
    for record in assignments:
        for gift in record["gifts"]:
            if gift not in gift_id_set:
                fail(errors, f"assignment references missing Gift: {record['id']} -> {gift}")
    reviewed_assignment_gifts = {
        gift
        for record in assignments
        if record.get("review_level") in {"reviewed_benchmark", "reviewed_dossier"}
        for gift in record["gifts"]
    }
    for gift in reviewed_assignment_gifts:
        if not gifts_by_id[gift].get("status", "").startswith("Reviewed"):
            fail(errors, f"reviewed assignment Gift lacks reviewed status: {gift}")

    manifestations = data.get("manifestations", [])
    if len(manifestations) != len(reviewed_assignments):
        fail(errors, f"expected one manifestation per reviewed assignment ({len(reviewed_assignments)}), found {len(manifestations)}")
    manifestation_ids = [record["id"] for record in manifestations]
    if len(manifestation_ids) != len(set(manifestation_ids)):
        fail(errors, "duplicate manifestation IDs")
    expected_counts = Counter(record["house"] for record in reviewed_assignments)
    counts = Counter(record["house"] for record in manifestations)
    if counts != expected_counts:
        fail(errors, f"manifestation counts differ: {dict(counts)}")
    for record in manifestations:
        provenance = {
            (item["age"], item["point"], item.get("point_suffix", ""))
            for item in record["provenance"]
        }
        for gift in record["source_gifts"]:
            if gift not in gift_id_set:
                fail(errors, f"manifestation references missing Gift: {record['id']} -> {gift}")
                continue
            gift_points = {
                (source["age"], source["point"], source.get("point_suffix", ""))
                for source in gifts_by_id[gift]["sources"]
            }
            if provenance.isdisjoint(gift_points):
                fail(errors, f"manifestation provenance does not contain a source Point for Gift: {record['id']} -> {gift}")
    reviewed = [record for record in manifestations if record.get("review_level") == "reviewed_flagship"]
    if len(reviewed) != len(reviewed_assignments) or Counter(record["house"] for record in reviewed) != expected_counts:
        fail(errors, "reviewed flagship manifestation coverage differs from reviewed assignments")
    reviewed_coordinates = Counter(
        (source["age"], source["point"], source.get("point_suffix", ""))
        for record in reviewed
        for source in record["provenance"]
    )
    expected_reviewed_coordinates = Counter(
        (record["age"], record["point"], record.get("point_suffix", ""))
        for record in reviewed_assignments
    )
    if reviewed_coordinates != expected_reviewed_coordinates:
        fail(errors, f"reviewed flagship Point coverage differs: {dict(reviewed_coordinates)}")
    reviewed_fields = (
        "purpose", "historical_ancestry", "modern_house_interpretation",
        "physical_appearance", "institutional_role", "gameplay_potential",
        "implementation_notes", "confidence", "sources", "dossier_document",
    )
    for record in reviewed:
        for field in reviewed_fields:
            if not record.get(field):
                fail(errors, f"reviewed manifestation missing {field}: {record['id']}")

    books = data.get("books", [])
    if len(books) != 19:
        fail(errors, f"expected 19 foundational records, found {len(books)}")
    if {record["primary_house"] for record in books} != HOUSES:
        fail(errors, "not every House owns at least one foundational book")
    return data


def validate_point_pages(errors: list[str], data: dict) -> None:
    required_point_fields = (
        "Status: Working",
        "Point type:",
        "Boundary rationale:",
        "## Historical summary",
        "## Reason this Point exists",
        "## Transition from previous Point",
        "## Transition into next Point",
        "## Four-House coordinate",
    )
    required_assignment_fields = (
        "**House:**",
        "**Primary culture:**",
        "**Temporal fit:**",
        "**Date relevance:**",
        "**Historical role:**",
        "**Major figures:**",
        "**Major locations:**",
        "**Major works or records:**",
        "**Government and institutions:**",
        "**Religion and worldview:**",
        "**Science and knowledge:**",
        "**Technology and infrastructure:**",
        "**Art and cultural expression:**",
        "**Warfare and strategic capacity:**",
        "**Resources and trade:**",
        "**Major conflicts and relationships:**",
        "### Permanent gifts",
        "### Modern Hollow Grove manifestation",
        "### Related existing Hollow Grove canon",
        "### Implementation opportunities",
        "### Source or research status",
    )
    documents = []
    for age in data["ages"]:
        for point in age["points"]:
            path = ROOT / point["document"]
            documents.append(path)
            if not path.is_file():
                fail(errors, f"missing Point document: {path}")
                continue
            text = path.read_text(encoding="utf-8")
            for field in required_point_fields:
                if field not in text:
                    fail(errors, f"{point['id']} missing Point field: {field}")
            if text.count("**Assignment ID:**") != 4:
                fail(errors, f"{point['id']} does not expose four assignment IDs")
            for field in required_assignment_fields:
                if text.count(field) != 4:
                    fail(errors, f"{point['id']} field count for {field!r} is {text.count(field)}, expected 4")
            for house in ("STONEBEND_ANCESTRY.md", "SANDMANOR_ANCESTRY.md", "GLAUSHOUSE_ANCESTRY.md", "FLYNT_ANCESTRY.md"):
                if house not in text:
                    fail(errors, f"{point['id']} does not link {house}")
    if len(documents) != EXPECTED_POINT_TOTAL:
        fail(errors, f"expected {EXPECTED_POINT_TOTAL} Point document paths, found {len(documents)}")


def validate_expected_files(errors: list[str]) -> None:
    expected = [
        "README.md", "ATLAS_INDEX.md", "ATLAS_PRINCIPLES.md", "MASTER_TIMELINE.md",
        "ANCESTRAL_GIFTS_REGISTRY.md", "ANCESTRAL_MANIFESTATIONS_REGISTRY.md",
        "CULTURAL_MEMORY_MODEL.md", "HOW_TO_USE_THE_ANCESTRAL_ATLAS.md",
        "HUEMAN_METADATA_PROPOSAL.md", "AUDIT_REPORT.md", "INTEGRATION_REPORT.md",
        "CHRONOLOGY_REVISION_01_REPORT.md",
        "REFINEMENT_PASS_01_REPORT.md", "REFINEMENT_PASS_02_REPORT.md", "REFINEMENT_PASS_03_REPORT.md", "REFINEMENT_PASS_04_REPORT.md", "REFINEMENT_PASS_05_REPORT.md", "REFINEMENT_PASS_06_REPORT.md", "REFINEMENT_PASS_07_REPORT.md", "REFINEMENT_PASS_08_REPORT.md", "REFINEMENT_PASS_09_REPORT.md", "REFINEMENT_PASS_10_REPORT.md", "REFINEMENT_PASS_11_REPORT.md", "REFINEMENT_PASS_12_REPORT.md", "REFINEMENT_PASS_13_REPORT.md", "REFINEMENT_PASS_14_REPORT.md", "REFINEMENT_PASS_15_REPORT.md", "dossiers/README.md",
        "dossiers/age_01_point_01/README.md", "dossiers/age_01_point_01/SUMER.md",
        "dossiers/age_01_point_01/EARLY_DYNASTIC_EGYPT.md",
        "dossiers/age_01_point_01/EARLY_INDUS_CIVILIZATION.md",
        "dossiers/age_01_point_01/ELAM.md",
        "dossiers/age_01_point_02/README.md", "dossiers/age_01_point_02/AKKAD.md",
        "dossiers/age_01_point_02/OLD_KINGDOM_EGYPT.md",
        "dossiers/age_01_point_02/MATURE_INDUS_CIVILIZATION.md",
        "dossiers/age_01_point_02/AWAN.md",
        "dossiers/age_01_point_03/README.md", "dossiers/age_01_point_03/UR_III.md",
        "dossiers/age_01_point_03/MIDDLE_KINGDOM_EGYPT.md",
        "dossiers/age_01_point_03/MINOAN_CRETE.md",
        "dossiers/age_01_point_03/SIMASKI.md",
        "dossiers/age_01_point_04/README.md", "dossiers/age_01_point_04/BABYLON.md",
        "dossiers/age_01_point_04/MIDDLE_KINGDOM_EGYPT_LATER_PHASE.md",
        "dossiers/age_01_point_04/MINOAN_CRETE_PROTOPALATIAL_EARLY_NEOPALATIAL.md",
        "dossiers/age_01_point_04/OLD_ASSYRIA.md",
        "dossiers/age_01_point_05/README.md", "dossiers/age_01_point_05/NEW_KINGDOM_EGYPT.md",
        "dossiers/age_01_point_05/HITTITE_EMPIRE.md",
        "dossiers/age_01_point_05/KASSITE_BABYLONIA.md",
        "dossiers/age_01_point_05/MIDDLE_ASSYRIA.md",
        "dossiers/age_02_point_01/README.md", "dossiers/age_02_point_01/KINGDOM_OF_ISRAEL.md",
        "dossiers/age_02_point_01/PHOENICIA.md",
        "dossiers/age_02_point_01/ARAM.md",
        "dossiers/age_02_point_01/NEO_ASSYRIA_EARLY_BOUNDARY.md",
        "dossiers/age_02_point_02/README.md", "dossiers/age_02_point_02/ISRAEL_NORTHERN_KINGDOM.md",
        "dossiers/age_02_point_02/PHOENICIA_NINTH_EIGHTH_CENTURIES.md",
        "dossiers/age_02_point_02/ARAM_POLITIES_NINTH_EIGHTH_CENTURIES.md",
        "dossiers/age_02_point_02/ASSYRIA_NINTH_EIGHTH_CENTURIES.md",
        "dossiers/age_02_point_03/README.md", "dossiers/age_02_point_03/JUDAH_ASSYRIAN_DOMINANCE.md",
        "dossiers/age_02_point_03/EGYPT_DYNASTIES_25_26_TRANSITION.md",
        "dossiers/age_02_point_03/BABYLON_CONTESTED_CITY_AND_RESTORATION.md",
        "dossiers/age_02_point_03/NEO_ASSYRIA_SARGONID_APEX_AND_FALL.md",
        "dossiers/age_02_point_04/README.md", "dossiers/age_02_point_04/JUDAH_KINGDOM_FALL_AND_COMMUNITIES.md",
        "dossiers/age_02_point_04/SAITE_EGYPT_LATE_DYNASTY_26.md",
        "dossiers/age_02_point_04/NEO_BABYLONIAN_EMPIRE_612_539.md",
        "dossiers/age_02_point_04/MEDES_POWER_AND_EVIDENCE_LIMITS.md",
        "dossiers/age_02_point_05/README.md", "dossiers/age_02_point_05/ACHAEMENID_EMPIRE_539_336.md",
        "dossiers/age_02_point_05/GREEK_WORLD_539_336.md",
        "dossiers/age_02_point_05/LATE_EGYPT_539_336.md",
        "dossiers/age_02_point_05/MACEDONIA_539_336.md",
        "dossiers/age_02_point_06/README.md", "dossiers/age_02_point_06/ACHAEMENID_END_AND_INSTITUTIONAL_CONTINUITY_336_281.md",
        "dossiers/age_02_point_06/MACEDONIAN_EMPIRE_AND_SUCCESSOR_CLAIMS_336_281.md",
        "dossiers/age_02_point_06/EARLY_PTOLEMAIC_FORMATION_332_281.md",
        "dossiers/age_02_point_06/EARLY_SELEUCID_FORMATION_323_281.md",
        "dossiers/age_02_point_07/README.md", "dossiers/age_02_point_07/ANTIGONID_MACEDONIA_AND_POST_MONARCHY_TRANSITION_281_146.md",
        "dossiers/age_02_point_07/HELLENISTIC_GREEK_CIVIC_AND_FEDERAL_WORLDS_281_146.md",
        "dossiers/age_02_point_07/PTOLEMAIC_EGYPT_INSTITUTIONS_RUPTURE_AND_RECOVERY_281_146.md",
        "dossiers/age_02_point_07/SELEUCID_REGIONAL_SYSTEMS_AND_CAPACITY_CONTRACTION_281_146.md",
        "dossiers/age_02_point_08/README.md", "dossiers/age_02_point_08/ROMAN_REPUBLIC_INSTITUTIONS_AND_EXPANSION_264_146.md",
        "dossiers/age_02_point_08/GREEK_WORLD_ROMAN_ENCOUNTERS_264_146.md", "dossiers/age_02_point_08/PTOLEMAIC_EGYPT_WESTERN_WAR_CONTEXT_264_146.md",
        "dossiers/age_02_point_08/CARTHAGE_MARITIME_REPUBLIC_AND_PUNIC_WARS_264_146.md",
        "dossiers/age_02_point_09/README.md", "dossiers/age_02_point_09/ROMAN_REPUBLIC_LATE_CRISIS_AND_TRANSITION_146_27.md",
        "dossiers/age_02_point_09/HELLENISTIC_GREEK_WORLD_LATE_REPUBLICAN_ENCOUNTERS_146_27.md", "dossiers/age_02_point_09/PTOLEMAIC_EGYPT_LATE_DYNASTY_AND_ROMAN_PRESSURE_146_27.md",
        "dossiers/age_02_point_09/PONTUS_MITHRIDATIC_FRONTIER_AND_ROMAN_WARS_146_27.md",
        "dossiers/age_02_point_10/README.md", "dossiers/age_02_point_10/ROMAN_EMPIRE_INFRASTRUCTURE_AND_PROVINCIAL_ORDER_27BCE_180CE.md",
        "dossiers/age_02_point_10/JUDEA_EARLY_CHRISTIANITY_AND_ROMAN_RULE_27BCE_180CE.md", "dossiers/age_02_point_10/ALEXANDRIA_SCHOLARSHIP_MEDICINE_AND_PORT_SYSTEMS_27BCE_180CE.md",
        "dossiers/age_02_point_10/PARTHIAN_EMPIRE_FRONTIER_AND_CARAVAN_SYSTEMS_27BCE_180CE.md",
        "houses/STONEBEND_ANCESTRY.md", "houses/SANDMANOR_ANCESTRY.md",
        "houses/GLAUSHOUSE_ANCESTRY.md", "houses/FLYNT_ANCESTRY.md",
        "houses/STONEBEND_ANCESTRAL_SYNTHESIS.md", "houses/SANDMANOR_ANCESTRAL_SYNTHESIS.md",
        "houses/GLAUSHOUSE_ANCESTRAL_SYNTHESIS.md", "houses/FLYNT_ANCESTRAL_SYNTHESIS.md",
        "libraries/STONEBEND_LIBRARY.md", "libraries/SANDMANOR_LIBRARY.md",
        "libraries/GLAUSHOUSE_LIBRARY.md", "libraries/FLYNT_LIBRARY.md",
        "schemas/HISTORICAL_DOSSIER_TEMPLATE.md",
        "schemas/ancestral-atlas-v1.schema.json", "data/atlas.json",
    ]
    for relative in expected:
        if not (ROOT / relative).is_file():
            fail(errors, f"missing expected file: {relative}")

    repository_root = ROOT.parents[2]
    repository_readme = (repository_root / "README.md").read_text(encoding="utf-8")
    authority_map = (repository_root / "REPOSITORY_AUTHORITY_MAP.md").read_text(encoding="utf-8")
    if "docs/world_bible/ancestral_atlas/README.md" not in repository_readme:
        fail(errors, "repository README does not expose the Ancestral Atlas")
    if "### Descriptive Cultural Ancestry" not in authority_map:
        fail(errors, "authority map does not classify descriptive cultural ancestry")
    if "Law of Capacity" not in authority_map:
        fail(errors, "authority map does not name Flynt's Law of Capacity")


def validate_reviewed_dossiers(errors: list[str], data: dict) -> None:
    minimum_sections = (
        "## Evidence boundary",
        "## Permanent Gifts",
        "## Modern Hollow Grove manifestation",
        "## House interpretation",
        "## Sources",
        "## Cross-references",
    )
    reviewed_assignments = [
        record
        for record in data.get("assignments", [])
        if record.get("review_level") in {"reviewed_benchmark", "reviewed_dossier"}
    ]
    for record in reviewed_assignments:
        relative_path = record.get("dossier_document")
        if not relative_path:
            fail(errors, f"reviewed assignment has no dossier path: {record['id']}")
            continue
        dossier_path = ROOT / relative_path
        if not dossier_path.is_file():
            fail(errors, f"reviewed dossier does not exist: {relative_path}")
            continue
        text = dossier_path.read_text(encoding="utf-8")
        if record["age"] < 4:
            continue
        for section in minimum_sections:
            if section not in text:
                fail(errors, f"{relative_path} missing minimum dossier section: {section}")
        if "assignment remains Working" not in text and "Atlas assignment remains Working" not in text:
            fail(errors, f"{relative_path} does not preserve Working assignment status")
        if text.count("https://") < 3:
            fail(errors, f"{relative_path} has fewer than three external scholarly or collection references")

    required_sections = (
        "## Evidence boundary", "## Historical overview", "## Chronology",
        "## Major figures", "## Major cities", "## Government", "## Religion",
        "## Economy", "## Trade", "## Science", "## Technology",
        "## Architecture", "## Military", "## Art", "## Language",
        "## Major texts", "## Major discoveries or innovations",
        "## Historical relationships", "## Permanent Gifts",
        "## Modern Hollow Grove manifestation", "## House interpretation",
        "## Library integration", "### Primary historical records",
        "### Archaeological evidence", "### Major modern scholarship",
        "### Recommended House library placement",
        "### Relationship to foundational House classics", "## Research notes",
        "## Sources", "## Cross-references",
    )
    batches = {
        "age_01_point_01": ("SUMER.md", "EARLY_DYNASTIC_EGYPT.md", "EARLY_INDUS_CIVILIZATION.md", "ELAM.md"),
        "age_01_point_02": ("AKKAD.md", "OLD_KINGDOM_EGYPT.md", "MATURE_INDUS_CIVILIZATION.md", "AWAN.md"),
        "age_01_point_03": ("UR_III.md", "MIDDLE_KINGDOM_EGYPT.md", "MINOAN_CRETE.md", "SIMASKI.md"),
        "age_01_point_04": ("BABYLON.md", "MIDDLE_KINGDOM_EGYPT_LATER_PHASE.md", "MINOAN_CRETE_PROTOPALATIAL_EARLY_NEOPALATIAL.md", "OLD_ASSYRIA.md"),
        "age_01_point_05": ("NEW_KINGDOM_EGYPT.md", "HITTITE_EMPIRE.md", "KASSITE_BABYLONIA.md", "MIDDLE_ASSYRIA.md"),
        "age_02_point_01": ("KINGDOM_OF_ISRAEL.md", "PHOENICIA.md", "ARAM.md", "NEO_ASSYRIA_EARLY_BOUNDARY.md"),
        "age_02_point_02": ("ISRAEL_NORTHERN_KINGDOM.md", "PHOENICIA_NINTH_EIGHTH_CENTURIES.md", "ARAM_POLITIES_NINTH_EIGHTH_CENTURIES.md", "ASSYRIA_NINTH_EIGHTH_CENTURIES.md"),
        "age_02_point_03": ("JUDAH_ASSYRIAN_DOMINANCE.md", "EGYPT_DYNASTIES_25_26_TRANSITION.md", "BABYLON_CONTESTED_CITY_AND_RESTORATION.md", "NEO_ASSYRIA_SARGONID_APEX_AND_FALL.md"),
        "age_02_point_04": ("JUDAH_KINGDOM_FALL_AND_COMMUNITIES.md", "SAITE_EGYPT_LATE_DYNASTY_26.md", "NEO_BABYLONIAN_EMPIRE_612_539.md", "MEDES_POWER_AND_EVIDENCE_LIMITS.md"),
        "age_02_point_05": ("ACHAEMENID_EMPIRE_539_336.md", "GREEK_WORLD_539_336.md", "LATE_EGYPT_539_336.md", "MACEDONIA_539_336.md"),
        "age_02_point_06": ("ACHAEMENID_END_AND_INSTITUTIONAL_CONTINUITY_336_281.md", "MACEDONIAN_EMPIRE_AND_SUCCESSOR_CLAIMS_336_281.md", "EARLY_PTOLEMAIC_FORMATION_332_281.md", "EARLY_SELEUCID_FORMATION_323_281.md"),
        "age_02_point_07": ("ANTIGONID_MACEDONIA_AND_POST_MONARCHY_TRANSITION_281_146.md", "HELLENISTIC_GREEK_CIVIC_AND_FEDERAL_WORLDS_281_146.md", "PTOLEMAIC_EGYPT_INSTITUTIONS_RUPTURE_AND_RECOVERY_281_146.md", "SELEUCID_REGIONAL_SYSTEMS_AND_CAPACITY_CONTRACTION_281_146.md"),
        "age_02_point_08": ("ROMAN_REPUBLIC_INSTITUTIONS_AND_EXPANSION_264_146.md", "GREEK_WORLD_ROMAN_ENCOUNTERS_264_146.md", "PTOLEMAIC_EGYPT_WESTERN_WAR_CONTEXT_264_146.md", "CARTHAGE_MARITIME_REPUBLIC_AND_PUNIC_WARS_264_146.md"),
        "age_02_point_09": ("ROMAN_REPUBLIC_LATE_CRISIS_AND_TRANSITION_146_27.md", "HELLENISTIC_GREEK_WORLD_LATE_REPUBLICAN_ENCOUNTERS_146_27.md", "PTOLEMAIC_EGYPT_LATE_DYNASTY_AND_ROMAN_PRESSURE_146_27.md", "PONTUS_MITHRIDATIC_FRONTIER_AND_ROMAN_WARS_146_27.md"),
        "age_02_point_10": ("ROMAN_EMPIRE_INFRASTRUCTURE_AND_PROVINCIAL_ORDER_27BCE_180CE.md", "JUDEA_EARLY_CHRISTIANITY_AND_ROMAN_RULE_27BCE_180CE.md", "ALEXANDRIA_SCHOLARSHIP_MEDICINE_AND_PORT_SYSTEMS_27BCE_180CE.md", "PARTHIAN_EMPIRE_FRONTIER_AND_CARAVAN_SYSTEMS_27BCE_180CE.md"),
    }
    for dirname, filenames in batches.items():
        dossier_dir = ROOT / "dossiers" / dirname
        for filename in filenames:
            text = (dossier_dir / filename).read_text(encoding="utf-8")
            for section in required_sections:
                if section not in text:
                    fail(errors, f"{filename} missing dossier section: {section}")
            if "Atlas assignment remains Working" not in text and "assignment remains Working" not in text:
                fail(errors, f"{filename} does not preserve Working assignment status")
            if text.count("https://") < 3:
                fail(errors, f"{filename} has fewer than three external scholarly or collection references")


def validate_capacity_correction(errors: list[str]) -> None:
    repository_root = ROOT.parents[2]
    constitutional_files = (
        "FLYNT_CONSTITUTION_V2.md",
        "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
        "FUNCTION_JUNCTION_SEASONAL_WORLD_CYCLE_AND_PERMANENCE_V1.md",
        "STONEBEND_CONSTITUTION_V2.md",
        "REPOSITORY_AUTHORITY_MAP.md",
    )
    for filename in constitutional_files:
        text = (repository_root / filename).read_text(encoding="utf-8")
        if "Law of Recognition" in text or "Flynt proves Recognition" in text:
            fail(errors, f"obsolete Flynt constitutional-law name remains in {filename}")
    source_text = (repository_root / "src" / "world" / "permanence.rs").read_text(encoding="utf-8")
    obsolete_active_variant = re.search(r"^\s*Recognition,\s*$", source_text, re.MULTILINE)
    capacity_is_active = all(
        required in source_text
        for required in (
            "Capacity,",
            "Self::Capacity,",
            "Self::LegacyRecognition => Self::Capacity",
        )
    )
    if obsolete_active_variant or not capacity_is_active:
        fail(errors, "executable PermanenceLaw does not reflect Law of Capacity")


def validate_markdown_links(errors: list[str]) -> None:
    link_pattern = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
    roots = [ROOT, ROOT.parent / "README.md"]
    paths = list(ROOT.rglob("*.md")) + [p for p in roots[1:] if p.is_file()]
    for path in paths:
        text = path.read_text(encoding="utf-8")
        for raw_target in link_pattern.findall(text):
            target = raw_target.strip()
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            file_target = target.split("#", 1)[0]
            if not file_target:
                continue
            resolved = (path.parent / file_target).resolve()
            if not resolved.exists():
                fail(errors, f"broken local link: {path.relative_to(ROOT.parent.parent.parent)} -> {target}")


def main() -> int:
    errors: list[str] = []
    data = validate_data(errors)
    validate_point_pages(errors, data)
    validate_expected_files(errors)
    validate_reviewed_dossiers(errors, data)
    validate_capacity_correction(errors)
    validate_markdown_links(errors)
    if errors:
        print("Ancestral Atlas validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print(
        "Ancestral Atlas validation passed: "
        f"6 Ages, {EXPECTED_POINT_TOTAL} Points, {EXPECTED_ASSIGNMENT_TOTAL} assignments, {len(data['gifts'])} Gifts, "
        f"{len(data['manifestations'])} manifestations, {len(data['books'])} foundational book records."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
