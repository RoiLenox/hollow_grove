#!/usr/bin/env python3
import copy
import json
from pathlib import Path
import unittest

import build_registrar


class RegistrarGenerationTests(unittest.TestCase):
    def setUp(self):
        self.data = json.loads((build_registrar.ROOT / "data/warden_registrar_v0_2.json").read_text())
        self.source = (build_registrar.ROOT / "src/bin/warden_registrar_v03.rs").read_text()

    def test_authored_projection_is_reproducible(self):
        self.assertEqual(self.source, build_registrar.render(self.source, self.data))

    def test_json_changes_reach_projection_with_rust_escaping(self):
        data = copy.deepcopy(self.data)
        data["records"][0]["name"] = 'Quoted "name" with \\ and\nnew line'
        rendered = build_registrar.render(self.source, data)
        self.assertIn(r'name: "Quoted \"name\" with \\ and\nnew line"', rendered)
        self.assertNotEqual(rendered, self.source)

    def test_duplicate_record_identity_is_rejected(self):
        self.data["records"][1]["key"] = self.data["records"][0]["key"]
        with self.assertRaisesRegex(ValueError, "unique"):
            build_registrar.generate(self.data)

    def test_unknown_register_is_rejected(self):
        self.data["records"][0]["registers"] = ["invented"]
        with self.assertRaisesRegex(ValueError, "category"):
            build_registrar.generate(self.data)


if __name__ == "__main__":
    unittest.main()
