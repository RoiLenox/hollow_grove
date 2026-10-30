#!/usr/bin/env python3
"""Reproduce the read-only Registrar's Rust data from its authored JSON."""
import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
BEGIN = "// BEGIN GENERATED REGISTRAR DATA"
END = "// END GENERATED REGISTRAR DATA"


def string(value):
    if not isinstance(value, str):
        raise ValueError("Registrar text must be a string")
    escapes = {'"': '\\"', '\\': '\\\\', '\n': '\\n', '\r': '\\r', '\t': '\\t'}
    return '"' + ''.join(escapes.get(c, f'\\u{{{ord(c):x}}}' if ord(c) < 32 else c)
                         for c in value) + '"'


def option(value):
    return "None" if value is None else f"Some({string(value)})"


def array(values):
    if not isinstance(values, list):
        raise ValueError("Registrar arrays must be lists")
    return "&[" + ", ".join(string(value) for value in values) + "]"


def generate(data):
    records = data["records"]
    if not isinstance(records, list) or not records:
        raise ValueError("Registrar needs an ordered record list")
    for field in ("key", "accession"):
        values = [record[field] for record in records]
        if any(not isinstance(value, str) or not value for value in values) or len(set(values)) != len(values):
            raise ValueError(f"Registrar {field} values must be nonempty and unique")
    rows = [BEGIN, "const RECORDS: &[Record] = &["]
    for record in records:
        location = record["location"]
        if not set(record["registers"]).issubset(data["register_categories"]):
            raise ValueError("unknown Registrar register category")
        fields = {
            "accession": string(record["accession"]),
            "key": string(record["key"]), "name": string(record["name"]),
            "aliases": array(record["also_known_as"]), "kind": string(record["entity_kind"]),
            "registers": array(record["registers"]), "origin_house": string(record["origin_house"]),
            "location_kind": string(location["kind"]), "house": option(location["house"]),
            "district": option(location["district"]), "area": option(location["area"]),
            "point": option(location["point_ref"]), "owner_status": string(record["ownership"]["status"]),
            "functions": array(record["functions"]),
        }
        rows.append("Record {")
        rows.extend(f"{key}: {value}," for key, value in fields.items())
        rows.append("relationships: &[")
        for link in record["affiliations"]:
            rows.extend([
                "Relationship {", f"relation: {string(link['relation'])},",
                f"target: {option(link.get('target'))},",
                f"target_houses: {array(link.get('target_houses', []))},",
                f"role: {option(link.get('role'))},", f"status: {string(link['status'])},", "},",
            ])
        rows.extend(["],", f"special_rules: {array(record['special_rules'])},", "},"])
    rows.extend(["];", END])
    return "\n".join(rows)


def render(source, data):
    if source.count(BEGIN) != 1 or source.count(END) != 1:
        raise ValueError("Registrar needs exactly one generated block")
    prefix, rest = source.split(BEGIN, 1)
    _, suffix = rest.split(END, 1)
    result = subprocess.run(
        ["rustfmt", "--edition", "2024", "--emit", "stdout"],
        input=prefix + generate(data) + suffix, text=True, capture_output=True, check=True,
        cwd=ROOT,
    )
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="reject drift without writing")
    args = parser.parse_args()
    path = ROOT / "src/bin/warden_registrar_v03.rs"
    try:
        current = path.read_text()
        rendered = render(current, json.loads((ROOT / "data/warden_registrar_v0_2.json").read_text()))
        if args.check and current != rendered:
            parser.exit(1, "Registrar data drift: run python3 tools/build_registrar.py\n")
        if not args.check and current != rendered:
            path.write_text(rendered)
        print("PASS: Registrar projection matches its authored JSON")
    except (ValueError, KeyError, TypeError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Registrar generation failed: {error}\n")


if __name__ == "__main__":
    main()
