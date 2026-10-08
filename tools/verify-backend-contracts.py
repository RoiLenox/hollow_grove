#!/usr/bin/env python3
"""Read-only validation of authored Hollow Grove backend contracts, not world state."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys


FUNCTIONS = {"Stonebend": "Name", "Sandmanor": "Prove", "Glaushouse": "Clear", "Flynt": "Recognize"}
ROLES = {"active-constitution", "active-canon", "ratified-amendment", "active-crosswalk",
         "adopted-bounded-contract", "compatibility-catalog", "legacy-only",
         "working-proposal", "source-backed-index"}
REQUIREMENT_STATUSES = {"bounded-case-verified", "source-defined-mechanics-open",
                        "investiture-open", "general-mechanics-open", "identity-pair-only"}
# Historical tier/provenance text; its body-composition model needs revision.
HISTORICAL_REFINEMENT = "Gargoyle + Werewolf + Mariner/Merman → Manticorp → Chimera"
SUPERSEDED_REFINEMENT_ORDER = "Gargoyle + Werewolf + Mariner/Merman → Chimera → Manticore"
PARTIAL_ADOPTIONS = {
    "decision.boardwalk.rivalry-tone": (
        "FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md",
        "district atmosphere and business rivalry",
        "Adopted atmosphere, 2026-10-08: tourist red-light waterfront."),
    "decision.boardwalk.crew-identities": (
        "FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md",
        "Troons crew identity, rubber Gimp persona, Gooner internet-cultural roots and customer-and-crew structure, upper/lower district relationship and private Underway nightclub identity",
        "Adopted Underway venue, 2026-10-08: closed-off private nightclub section beneath the Boardwalk."),
    "decision.synthesis.foundation-outcomes": (
        "HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md",
        "three ordered Sympiote foundation grades before distinct supported branches, including Gargoyle, Werewolf and Mariner/Merman; Gargoyle and Werewolf never combine",
        "Adopted Sympiote grade structure, 2026-10-08: Grade I → Grade II → Grade III → distinct supported Synthesis branches."),
}


class AuditError(ValueError):
    pass


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise AuditError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def read_json(path):
    return json.loads(path.read_text(), object_pairs_hook=unique_object)


def safe_source(root, relative):
    if not isinstance(relative, str) or not relative or "\\" in relative:
        raise AuditError(f"invalid source reference: {relative!r}")
    path = Path(relative)
    if path.is_absolute() or ".." in path.parts or path.as_posix() != relative:
        raise AuditError(f"unsafe source reference: {relative}")
    resolved = (root / path).resolve()
    if not resolved.is_relative_to(root.resolve()):
        raise AuditError(f"source escapes root: {relative}")
    if not resolved.is_file():
        raise AuditError(f"missing source: {relative}")
    return resolved


def require(condition, message):
    if not condition:
        raise AuditError(message)


def nonempty_strings(value, name):
    require(isinstance(value, list) and bool(value), f"missing {name}")
    require(all(isinstance(v, str) and v.strip() for v in value), f"invalid {name}")


def unique_rows(rows, key, name):
    require(isinstance(rows, list), f"invalid {name} list")
    require(all(isinstance(row, dict) and isinstance(row.get(key), str) and row[key].strip() for row in rows), f"invalid {name} identity")
    values = [row[key] for row in rows]
    require(len(values) == len(set(values)), f"duplicate {name} identity")


def validate(root, data, architecture=None):
    require(isinstance(data, dict), "authored registry must be an object")
    require(set(data) == {"registry_schema_version", "registry_kind", "scope", "sources", "synthesis",
                         "candidate_offers", "requirements", "route_gaps", "open_decisions", "scope_notes"},
            "unknown or missing authored registry fields")
    require(type(data["registry_schema_version"]) is int and data["registry_schema_version"] == 1,
            "unsupported authored-registry schema")
    require(data["registry_kind"] == "authored-backend-contract-index", "not an authored backend index")
    require(data["scope"] == "backend-lore-architecture-validation", "unexpected scope")
    unique_rows(data["sources"], "path", "source")
    sources, digests = {}, {}
    for source in data["sources"]:
        name, role = source["path"], source["role"]
        require(role in ROLES, f"unknown source role: {name}")
        path = safe_source(root, name)
        payload = path.read_bytes()
        text = payload.decode("utf-8")
        # Legacy aliases later in an active document do not make it deprecated.
        deprecated = "LEGACY / DEPRECATED" in text[:600]
        require(not deprecated or role == "legacy-only", f"legacy source promoted to active authority: {name}")
        nonempty_strings(source["required_markers"], f"status markers for {name}")
        for marker in source["required_markers"]:
            require(marker in text, f"source status/meaning marker missing: {name}: {marker}")
        sources[name] = (role, text)
        digests[name] = hashlib.sha256(payload).hexdigest()

    def owned(name, active=False):
        require(name in sources, f"unregistered owner: {name}")
        if active:
            require(sources[name][0] not in {"legacy-only", "compatibility-catalog", "working-proposal"},
                    f"non-active source used as active progression authority: {name}")
        return sources[name][1]

    decisions = data["open_decisions"]
    unique_rows(decisions, "id", "open decision")
    decision_ids = {d["id"] for d in decisions}
    for decision in decisions:
        require(decision["status"] in {"open", "partially-adopted"} and bool(decision["question"].strip()), "open decision silently resolved")
        require(re.fullmatch(r"decision\.[a-z0-9.-]+", decision["id"]) is not None, "invalid decision identity")
        owned(decision["owner"])
        require(sources[decision["owner"]][0] not in {"legacy-only", "compatibility-catalog"},
                "open decision assigned to legacy authority")
        if decision["status"] == "partially-adopted":
            adoption = PARTIAL_ADOPTIONS.get(decision["id"])
            require(adoption is not None
                    and decision["owner"] == adoption[0]
                    and decision.get("adopted_scope") == adoption[1],
                    "unrecorded or overbroad partial adoption")
            require(decision.get("adoption_marker") == adoption[2],
                    "partial adoption has no exact source marker")
            owner_text = owned(decision["owner"], active=True)
            require(decision["adoption_marker"] in owner_text
                    and isinstance(decision.get("user_statement"), str)
                    and bool(decision["user_statement"].strip())
                    and decision["user_statement"] in owner_text,
                    "partial adoption lacks its source/user statement")
            require(isinstance(decision.get("remaining_question"), str)
                    and bool(decision["remaining_question"].strip()), "partial adoption erases remaining decision")

    def unresolved(row):
        nonempty_strings(row["open_decisions"], "open decisions")
        require(set(row["open_decisions"]) <= decision_ids, "unknown decision reference")

    synthesis = data["synthesis"]
    canon = owned(synthesis["source"], active=True)
    grades = synthesis.get("sympiote_grades")
    require(isinstance(grades, dict) and set(grades) == {
        "owner", "grades", "position", "branch_examples", "additional_branches",
        "forbidden_combined_forms", "criteria_status", "runtime_advancement_implemented",
        "historical_cases_graded", "higher_state_body_requirements"}, "invalid Sympiote grade index")
    require(grades["owner"] == "HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md"
            and PARTIAL_ADOPTIONS["decision.synthesis.foundation-outcomes"][2] in owned(grades["owner"], active=True),
            "Sympiote grade structure lacks its owning adoption")
    require(grades["grades"] == ["I", "II", "III"]
            and grades["position"] == "before-supported-form-specialization", "Sympiote grade order/position changed")
    require(grades["branch_examples"] == ["Gargoyle", "Werewolf", "Mariner/Merman"]
            and grades["additional_branches"] == "open"
            and grades["forbidden_combined_forms"] == [["Gargoyle", "Werewolf"]], "distinct Sympiote branches conflated")
    require("Adopted Form separation, 2026-10-08: Gargoyle and Werewolf never combine." in owned(grades["owner"]),
            "missing adopted Form separation")
    require(grades["criteria_status"] == "open"
            and grades["runtime_advancement_implemented"] is False
            and grades["historical_cases_graded"] is False, "unadopted grade criteria, runtime promotion or historical grading claimed")
    require(grades["higher_state_body_requirements"] == "needs-revision",
            "unresolved higher-state body requirements falsely reconciled")
    section = canon.split("### Locked Current/Aura level pairing", 1)[1].split("### Sandmanor", 1)[0]
    actual_pairs = [{"level": int(level), "current": current, "aura": aura}
                    for level, current, aura in re.findall(r"^\| ([1-8]) \| (\w+) \| (\w+) \|$", section, re.M)]
    require(len(actual_pairs) == 8 and synthesis["stonebend_pairs"] == actual_pairs,
            "active Current/Aura pairing differs from its owning canon")
    amendment = owned("HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md", active=True)
    for branch, line in synthesis["sandmanor_complete_branches"].items():
        require(branch in {"Minorian", "Minoan"} and " → ".join(line) in amendment,
                "Sandmanor complete branch disagrees with ratified amendment")
    require(set(synthesis["sandmanor_complete_branches"]) == {"Minorian", "Minoan"}, "missing parallel branch")
    require(synthesis["flynt_refinement"] == ["Manticorp", "Chimera"], "superseded Flynt refinement order")
    require(synthesis["flynt_tracks"] == ["Current", "Aura", "Weapon"], "independent Flynt tracks collapsed")
    require(synthesis["compatible_aliases"] == {"Pixy": "Pixie", "Faerie": "Fairy"}, "legacy spelling aliases changed")
    require(synthesis["clinical_geometry"] == {"common": "Nightingale/Pixie", "physical": ["Marshal/Sprite", "Spirit"],
            "aura": ["Matron/Fairy", "Nymph"], "reunification": "Persephone/Siren", "apex": "Prima Donna/Muse"},
            "clinical branch/office geometry changed")
    for name in ("HOLLOW_GROVE_SYNTHESIS_CANON_V1.md", "HOLLOW_GROVE_CLASSIFICATION_RECONCILIATION_V1.md",
                 "CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md"):
        text = owned(name, active=True)
        require(HISTORICAL_REFINEMENT in text and SUPERSEDED_REFINEMENT_ORDER not in text,
                f"historical Flynt tier-order record differs in {name}")
        require("Current Form boundary, 2026-10-08: Gargoyle and Werewolf never combine." in text,
                f"missing current Form boundary in {name}")
    require("six locked equations" not in owned("HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md"),
            "legacy power catalog manufactures active Sandmanor equations")
    mirror = safe_source(root, "artifacts/current_synthesis_world_context.md")
    mirror_payload = mirror.read_bytes()
    require(mirror_payload.decode("utf-8") == owned("CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md"),
            "world-context projection differs from owning source")
    digests["artifacts/current_synthesis_world_context.md"] = hashlib.sha256(mirror_payload).hexdigest()

    offers = data["candidate_offers"]
    unique_rows(offers, "key", "offer")
    require(len(offers) == 6, "candidate catalog lost or gained an unreviewed entry")
    camps = {"PimpAndTrooners": 0, "GimpAndGooners": 0}
    for offer in offers:
        require(offer["key"].startswith("draft.offer."), "proposal key masquerades as runtime inventory identity")
        require(offer["status"] == "proposed" and offer["runtime_implemented"] is False,
                "candidate offer falsely ratified or implemented")
        require(offer["supplier_binding"] is None, "ordinary supplier binding silently adopted")
        require(offer["camp"] in camps, "unknown Boardwalk camp")
        camps[offer["camp"]] += 1
        require(owned(offer["source"]) and sources[offer["source"]][0] == "working-proposal", "offer source is not a proposal")
        for name in ("name", "customer_need", "promised_scope"):
            require(isinstance(offer[name], str) and bool(offer[name].strip()), f"missing offer {name}")
        for name in ("delivery_evidence", "refusal_and_remedy"):
            nonempty_strings(offer[name], name)
        exchange = offer["exchange"]
        owned(exchange["source"], active=True)
        require(exchange["instrument"] == "StandardCurrency", "fictional instrument confused with ordinary currency")
        require(exchange["terms_status"] == "open" and exchange["amount_minor_units"] is None,
                "unknown exchange interpreted as settled/free/agreed amount")
        require(offer["grants"] == [], "commercial promise grants affiliation, Form or authority")
        unresolved(offer)
    require(set(camps.values()) == {3}, "candidate offerings do not retain three per camp")

    requirements = data["requirements"]
    unique_rows(requirements, "key", "requirement")
    for row in requirements:
        require(row["status"] in REQUIREMENT_STATUSES, "unknown requirement status")
        nonempty_strings(row["sources"], "requirement source owners")
        for owner in row["sources"]:
            owned(owner, active=True)
        nonempty_strings(row["prerequisites"], "prerequisites")
        nonempty_strings(row["boundaries"], "independent boundaries")
        unresolved(row)
    expected_families = {"sympiote-foundation", "stonebend-pairings", "seer", "minorian", "minoan", "sandmanor-hands",
                        "glaushouse-common", "glaushouse-physical", "glaushouse-aura", "glaushouse-reunification",
                        "flynt-gargoyle", "flynt-werewolf", "flynt-mariner", "flynt-refinement", "flynt-creature-pairs", "winged-gargoyle"}
    require({r["key"] for r in requirements} == {"requirements." + k for k in expected_families}, "supported family coverage changed")
    verified = {r["key"] for r in requirements if r["status"] == "bounded-case-verified"}
    require(verified == {"requirements.sympiote-foundation", "requirements.winged-gargoyle"},
            "generalized realization falsely claimed as a verified bounded case")

    gaps = data["route_gaps"]
    unique_rows(gaps, "lore_id", "route")
    require(len(gaps) == 10, "historical ten route contracts changed without architecture proof")
    for gap in gaps:
        require(gap["house"] in FUNCTIONS and gap["function"] == FUNCTIONS[gap["house"]], "House authority function conflated")
        owned(gap["authority_source"], active=True)
        for name in ("definition_owner", "related_module"):
            safe_source(root, gap[name])
        require(gap["binding_status"] == "headless-backend-bound" and gap["actor_bindings_complete"] is False,
                "source role labels falsely treated as verified actor/execution bindings")
        require(gap.get("backend_owner") == "src/world/lore_handoffs.rs"
                and gap.get("backend_proof_source") == "tests/backend_house_handoffs.rs"
                and gap.get("backend_contract_source") == "docs/BACKEND_HOUSE_HANDOFFS_V1.md"
                and gap.get("gameplay_binding") is None, "missing or conflated headless backend owner/proof")
        for name in ("backend_owner", "backend_proof_source", "backend_contract_source"):
            safe_source(root, gap[name])
        require(f'"{gap["lore_id"]}"' in owned(gap["backend_owner"]), "route has no registered executable backend identity")
        for name in ("choices", "failure_and_refusal", "required_receipts", "open_fields"):
            nonempty_strings(gap[name], name)
        require(bool(gap["completion_condition"].strip()), "missing bounded completion condition")

    if architecture is not None:
        require(architecture["schema_version"] == 3 and architecture["source_links_passed"] is True,
                "incompatible or failed architecture report")
        expected = {r["lore_id"]: r for r in architecture["canonical_route_contracts"] if r.get("backend_handoff") is not None}
        require(set(expected) == {r["lore_id"] for r in gaps}, "declared gaps differ from architecture audit")
        for gap in gaps:
            source = expected[gap["lore_id"]]
            require(source["house"] == gap["house"] and source["authority_source"] == gap["authority_source"]
                    and source["definition_owner"] == gap["definition_owner"]
                    and source["backend_handoff"] == gap["lore_id"]
                    and source["reducer_owner"] == gap["backend_owner"]
                    and source["persistence_owner"] == gap["backend_owner"]
                    and source["gameplay_case"] is None, "route source owner differs from architecture audit")
        require(architecture["verified_binding_count"] == 14 and architecture["unbound_canonical_route_count"] == 0
                and architecture["backend_handoff_binding_count"] == 10
                and architecture["unbound_gameplay_route_count"] == 10,
                "architecture counts differ from recorded baseline")

    return {"schema_version": 1, "passed": True, "scope": data["scope"], "source_files_checked": len(digests),
            "candidate_offers": len(offers), "requirement_families": len(requirements), "unverified_route_contracts": 0,
            "headless_backend_route_bindings": len(gaps), "deferred_gameplay_route_bindings": len(gaps),
            "open_decisions": sum(d["status"] == "open" for d in decisions),
            "partially_adopted_decisions": sum(d["status"] == "partially-adopted" for d in decisions),
            "unresolved_decision_families": len(decisions),
            "sympiote_grades": grades["grades"], "grade_runtime_advancement_implemented": False,
            "historical_foundation_cases_graded": False,
            "higher_state_body_requirements": "needs-revision; not reconciled with the old executable constitution",
            "architecture_crosscheck": "passed" if architecture else "not supplied",
            "source_sha256": digests, "world_state_mutated": False,
            "attestation": "authored source/status consistency; no new domain execution or proposal adoption"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--registry", default="contracts/backend/lore_registry.v1.json")
    parser.add_argument("--architecture-json", type=Path)
    args = parser.parse_args()
    try:
        data = read_json(safe_source(args.source_root, args.registry))
        report = validate(args.source_root, data, read_json(args.architecture_json) if args.architecture_json else None)
    except (AuditError, OSError, ValueError, KeyError, TypeError, IndexError, AttributeError) as error:
        print(json.dumps({"schema_version": 1, "passed": False, "error": str(error), "world_state_mutated": False}))
        return 1
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
