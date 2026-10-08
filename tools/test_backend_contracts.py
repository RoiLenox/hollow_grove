#!/usr/bin/env python3
"""Adversarial checks for the authored backend register and its read-only audit."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("backend_contracts", Path(__file__).with_name("verify-backend-contracts.py"))
audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)
ROOT = Path(__file__).resolve().parents[1]


class BackendContractTests(unittest.TestCase):
    def setUp(self):
        self.data = audit.read_json(ROOT / "contracts/backend/lore_registry.v1.json")

    def rejected(self, mutate, message):
        data = copy.deepcopy(self.data)
        mutate(data)
        with self.assertRaisesRegex(audit.AuditError, message):
            audit.validate(ROOT, data)

    def test_source_audit_is_read_only_and_does_not_claim_execution(self):
        names = [s["path"] for s in self.data["sources"]] + ["contracts/backend/lore_registry.v1.json", "artifacts/current_synthesis_world_context.md"]
        before = {name: (ROOT / name).read_bytes() for name in names}
        report = audit.validate(ROOT, self.data)
        self.assertTrue(report["passed"])
        self.assertFalse(report["world_state_mutated"])
        self.assertIn("no new domain execution", report["attestation"])
        self.assertEqual(before, {name: (ROOT / name).read_bytes() for name in names})

    def test_deprecated_matrix_cannot_be_promoted_to_active_authority(self):
        def mutate(data):
            next(s for s in data["sources"] if s["role"] == "legacy-only")["role"] = "active-canon"
        self.rejected(mutate, "legacy source promoted")

    def test_requirement_cannot_use_legacy_matrix_even_when_role_is_honest(self):
        self.rejected(lambda d: d["requirements"][1]["sources"].append("HUEMAN_CURRENT_BODY_AURA_MIND_PROGRESSION_MATRIX_V1.md"), "non-active source")

    def test_unknown_price_cannot_be_treated_as_free(self):
        self.rejected(lambda d: d["candidate_offers"][0]["exchange"].update(amount_minor_units=0), "unknown exchange")

    def test_catalog_does_not_create_form_or_membership(self):
        self.rejected(lambda d: d["candidate_offers"][0]["grants"].append("Trooner membership"), "commercial promise grants")

    def test_proposal_cannot_claim_ratification_or_implementation(self):
        for change in ({"status": "adopted"}, {"runtime_implemented": True}, {"supplier_binding": "being.invented.supplier"}):
            with self.subTest(change=change):
                self.rejected(lambda d: d["candidate_offers"][0].update(change), "falsely|silently")

    def test_superseded_refinement_order_and_shifted_pairing_fail(self):
        self.rejected(lambda d: d["synthesis"].update(flynt_refinement=["Chimera", "Manticore"]), "superseded")
        self.rejected(lambda d: d["synthesis"]["stonebend_pairs"][2].update(aura="Sylph"), "pairing differs")

    def test_biological_branch_cannot_erase_hand_investiture(self):
        self.rejected(lambda d: d["synthesis"]["sandmanor_complete_branches"].update(Minorian=["Gnome", "Faun", "Minotaur", "Demiurge"]), "branch disagrees")

    def test_duplicate_identity_and_unowned_decision_fail(self):
        self.rejected(lambda d: d["candidate_offers"].append(copy.deepcopy(d["candidate_offers"][0])), "duplicate offer")
        self.rejected(lambda d: d["requirements"][0]["open_decisions"].append("decision.unowned"), "unknown decision")

    def test_open_decision_cannot_be_assigned_to_legacy_authority(self):
        self.rejected(lambda d: d["open_decisions"][0].update(owner="HUEMAN_CURRENT_BODY_AURA_MIND_PROGRESSION_MATRIX_V1.md"), "legacy authority")

    def test_partial_tone_adoption_cannot_erase_customs_or_select_products(self):
        report = audit.validate(ROOT, self.data)
        self.assertEqual(report["open_decisions"], 17)
        self.assertEqual(report["partially_adopted_decisions"], 4)
        self.assertEqual(report["unresolved_decision_families"], 21)
        def tone(data):
            return next(d for d in data["open_decisions"] if d["id"] == "decision.boardwalk.rivalry-tone")
        self.rejected(lambda d: tone(d).update(remaining_question=""), "erases remaining decision")
        self.rejected(lambda d: tone(d).update(adopted_scope="all venues, products and customs"), "overbroad partial")
        self.rejected(lambda d: d["open_decisions"][0].update(status="partially-adopted"), "unrecorded or overbroad partial")

    def test_crew_structure_adoption_preserves_open_jobs_and_prior_history(self):
        def crew(data):
            return next(d for d in data["open_decisions"] if d["id"] == "decision.boardwalk.crew-identities")
        self.rejected(lambda d: crew(d).update(adopted_scope="all Gooners are staff; prior Bonds create sexual conduct"), "overbroad partial")
        self.rejected(lambda d: crew(d).update(adoption_marker="Adopted all Gooner venues, worker duties and prices."), "exact source marker")
        self.rejected(lambda d: crew(d).update(adopted_scope="underground venue entry is successful Underway emergence and House clearance"), "overbroad partial")
        self.rejected(lambda d: crew(d).update(adopted_scope="all guests automatically become Gooners and have agreed to all club services"), "overbroad partial")
        self.rejected(lambda d: crew(d).update(remaining_question=""), "erases remaining decision")
        self.rejected(lambda d: crew(d).update(user_statement="invented undisclosed adoption"), "source/user statement")

    def test_grade_index_rejects_extra_grades_hybrids_and_invented_progress(self):
        def grades(data):
            return data["synthesis"]["sympiote_grades"]
        self.rejected(lambda d: grades(d).update(grades=["I", "III", "II"]), "grade order/position")
        self.rejected(lambda d: grades(d)["grades"].append("IV"), "grade order/position")
        self.rejected(lambda d: grades(d).update(branch_examples=["Gargoyle/Werewolf", "Mariner/Merman"]), "branches conflated")
        self.rejected(lambda d: grades(d).update(forbidden_combined_forms=[]), "branches conflated")
        for field in ("runtime_advancement_implemented", "historical_cases_graded"):
            with self.subTest(field=field):
                self.rejected(lambda d: grades(d).update({field: True}), "runtime promotion or historical grading")
        self.rejected(lambda d: grades(d).update(criteria_status="adopted"), "unadopted grade criteria")

    def test_grade_adoption_keeps_owner_and_body_review_explicit(self):
        def decision(data):
            return next(r for r in data["open_decisions"] if r["id"] == "decision.synthesis.foundation-outcomes")
        self.rejected(lambda d: decision(d).update(owner="FLYNT_CONSTITUTION_V2.md"), "overbroad partial")
        self.rejected(lambda d: decision(d).update(remaining_question=""), "erases remaining decision")
        self.rejected(lambda d: d["synthesis"]["sympiote_grades"].update(higher_state_body_requirements="reconciled"), "falsely reconciled")
        report = audit.validate(ROOT, self.data)
        self.assertEqual(report["sympiote_grades"], ["I", "II", "III"])
        self.assertFalse(report["grade_runtime_advancement_implemented"])
        self.assertFalse(report["historical_foundation_cases_graded"])
        self.assertIn("not reconciled", report["higher_state_body_requirements"])

    def test_synthesis_origin_preserves_target_and_maker_boundaries(self):
        def origin(data):
            return data["synthesis"]["process_origin"]
        self.rejected(lambda d: origin(d).update(source_house="Stonebend"), "origin or subject scope")
        self.rejected(lambda d: origin(d).update(target_domains=["Being"]), "origin or subject scope")
        self.rejected(lambda d: origin(d).update(item_synthesis_grants_maker_grade_or_form=True), "maker progression")
        self.rejected(lambda d: origin(d).update(owner="HOLLOW_GROVE_SYNTHESIS_CANON_V1.md"), "owning adoption")
        report = audit.validate(ROOT, self.data)
        self.assertEqual(report["synthesis_source_house"], "Glaushouse")
        self.assertEqual(report["synthesis_target_domains"], ["Object", "Being"])

    def test_synthesis_origin_cannot_invent_mechanisms_or_rewrite_repairs(self):
        def origin(data):
            return data["synthesis"]["process_origin"]
        for field in ("material_policy", "execution_location_policy"):
            with self.subTest(field=field):
                self.rejected(lambda d: origin(d).update({field: "adopted"}), "unadopted Synthesis")
        for field in ("unified_runtime_implemented", "historical_repairs_reclassified"):
            with self.subTest(field=field):
                self.rejected(lambda d: origin(d).update({field: True}), "invents runtime execution")
        def decision(data):
            return next(r for r in data["open_decisions"] if r["id"] == "decision.synthesis.process-scale")
        self.rejected(lambda d: decision(d).update(remaining_question=""), "erases remaining decision")
        self.rejected(lambda d: decision(d).update(status="open"), "remaining decisions not indexed")
        report = audit.validate(ROOT, self.data)
        self.assertEqual(report["synthesis_material_policy"], "open")
        self.assertFalse(report["unified_synthesis_runtime_implemented"])

    def test_synbiote_units_and_sympiote_body_names_stay_distinct(self):
        def units(data):
            return data["synthesis"]["living_scale_units"]
        self.rejected(lambda d: units(d).update(small_unit_name="Sympiote block"), "names or kinds conflated")
        self.rejected(lambda d: units(d).update(body_expression_name="Synbiote Form"), "names or kinds conflated")
        self.rejected(lambda d: units(d).update(small_unit_kind="whole-body-form"), "names or kinds conflated")
        self.rejected(lambda d: units(d).update(grades_apply_to="Synbiote block"), "body Form or grade")
        self.rejected(lambda d: units(d).update(item_use_grants_body_form=True), "body Form or grade")
        report = audit.validate(ROOT, self.data)
        self.assertEqual(report["small_synthesis_unit"], "Synbiote block")
        self.assertEqual(report["bodily_synthesis_expression"], "Sympiote Form")

    def test_scale_naming_does_not_invent_block_execution_or_archive_history(self):
        def units(data):
            return data["synthesis"]["living_scale_units"]
        for field in ("block_crafting_implemented", "historical_grafts_decomposed_into_blocks"):
            with self.subTest(field=field):
                self.rejected(lambda d: units(d).update({field: True}), "historical inventory invented")
        self.rejected(lambda d: units(d).update(exact_material_conversion_status="lightening-implemented"), "mechanisms or historical inventory")
        self.rejected(lambda d: units(d).update(definition_source="BEING_OBJECT_ROOT_ONTOLOGY_V1.md"), "owning adoption")
        self.assertFalse(audit.validate(ROOT, self.data)["synbiote_block_crafting_implemented"])

    def test_relative_escape_absolute_path_and_symlink_escape_fail(self):
        for name in ("../secret", "/etc/passwd", "a/../../b", "a\\b"):
            with self.subTest(name=name), self.assertRaises(audit.AuditError):
                audit.safe_source(ROOT, name)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "root"
            root.mkdir()
            outside = Path(temporary) / "outside.md"
            outside.write_text("outside")
            (root / "link.md").symlink_to(outside)
            with self.assertRaisesRegex(audit.AuditError, "escapes root"):
                audit.safe_source(root, "link.md")

    def test_json_duplicate_keys_are_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "bad.json"
            path.write_text('{"status":"proposed","status":"adopted"}')
            with self.assertRaisesRegex(audit.AuditError, "duplicate JSON key"):
                audit.read_json(path)

    def test_gap_cannot_be_verified_by_changing_a_label(self):
        self.rejected(lambda d: d["route_gaps"][0].update(binding_status="verified"), "falsely treated")
        self.rejected(lambda d: d["route_gaps"][0].update(function="Name"), "authority function conflated")

    def test_optional_architecture_report_must_match_gap_set_and_owners(self):
        gaps = self.data["route_gaps"]
        report = {"schema_version": 3, "source_links_passed": True, "verified_binding_count": 14,
                  "unbound_canonical_route_count": 0, "backend_handoff_binding_count": 10, "unbound_gameplay_route_count": 10,
                  "canonical_route_contracts": [{"lore_id": g["lore_id"], "house": g["house"],
                                                "authority_source": g["authority_source"], "definition_owner": g["definition_owner"],
                                                "reducer_owner": g["backend_owner"], "persistence_owner": g["backend_owner"],
                                                "gameplay_case": None, "backend_handoff": g["lore_id"]} for g in gaps]}
        self.assertEqual(audit.validate(ROOT, self.data, report)["architecture_crosscheck"], "passed")
        report["canonical_route_contracts"].pop()
        with self.assertRaisesRegex(audit.AuditError, "gaps differ"):
            audit.validate(ROOT, self.data, report)

    def test_headless_binding_requires_its_exact_owner_and_cannot_claim_gameplay(self):
        self.rejected(lambda d: d["route_gaps"][0].update(backend_owner="src/gameplay/boardwalk.rs"), "conflated headless")
        self.rejected(lambda d: d["route_gaps"][0].update(gameplay_binding="invented.game.case"), "conflated headless")
        self.rejected(lambda d: d["route_gaps"][0].update(lore_id="lore.invented.route"), "executable backend identity")

    def test_cli_invalid_input_has_failure_exit_and_json_diagnostic(self):
        process = subprocess.run(["python3", str(ROOT / "tools/verify-backend-contracts.py"), "--registry", "../outside.json"],
                                 text=True, capture_output=True)
        self.assertEqual(process.returncode, 1)
        self.assertFalse(json.loads(process.stdout)["passed"])
        self.assertEqual(process.stderr, "")


if __name__ == "__main__":
    unittest.main()
