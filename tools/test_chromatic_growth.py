import copy
import json
import math
import unittest

import chromatic_color_standard as colors
import chromatic_growth as growth


class GrowthProposalTests(unittest.TestCase):
    def setUp(self):
        self.data = growth.load()
        self.report = growth.project(self.data)

    def test_all_32_targets_match_document_rounding(self):
        _, rows = growth.document_tables()
        records = [form for branch in self.report['branches'] for form in branch['forms']]
        self.assertEqual(len(records), 32)
        for record, row in zip(records, rows):
            index, name, hexcode, gap, percent, pixels = row
            with self.subTest(form=name):
                self.assertEqual((record['name'], record['hex'], record['growth_index']), (name, hexcode, int(index)))
                self.assertEqual(f"{record['rgb_gap']:.1f}", gap)
                self.assertEqual(f"{record['height_ratio'] * 100:.1f}", percent)
                self.assertEqual(record['illustrative_pixels'], int(pixels))

    def test_cumulative_rgb_path_is_not_endpoint_distance_or_packed_hex(self):
        total, records = growth.cumulative_heights(['#000000', '#030000', '#030400'], 1.0, 2.0)
        self.assertEqual(total, 7.0)
        self.assertAlmostEqual(records[1]['height_ratio'], 1 + 3 / 7)
        self.assertNotEqual(records[1]['height_ratio'], 1 + 3 / 5)
        self.assertEqual(records[-1]['height_ratio'], 2.0)
        for start, end in ((0, 2), (2, 1), (1, math.inf), (math.nan, 2)):
            with self.assertRaises(ValueError):
                growth.cumulative_heights(['#000000', '#FFFFFF'], start, end)
        with self.assertRaisesRegex(ValueError, 'zero color path'):
            growth.cumulative_heights(['#FFFFFF', '#FFFFFF'], 1, 2)

    def test_independent_branches_and_flynt_comparison(self):
        self.assertEqual([len(b['forms']) for b in self.report['branches']], [8, 4, 4, 8, 8])
        for branch in self.report['branches']:
            heights = [form['height_ratio'] for form in branch['forms']]
            self.assertEqual(heights[0], branch['start_height'])
            self.assertEqual(heights[-1], branch['end_height'])
            self.assertTrue(all(a < b for a, b in zip(heights, heights[1:])))
        self.assertTrue(self.report['branches'][-1]['comparison_only'])
        gargoyle = self.report['branches'][-1]['forms'][0]
        self.assertEqual((gargoyle['growth_index'], gargoyle['sequence'], gargoyle['swatch_level']), (2, 1, 8))
        self.assertEqual(gargoyle['height_ratio'], 1.25)
        roots = colors.load()['relationships']['flynt']['sibling_families']
        self.assertEqual(roots[0]['origin'], 'design.form.stonebend.gremlin')

    def test_only_three_indices_are_creator_directed_and_no_geometry_is_approved(self):
        entries = [f for b in self.data['branches'] for f in b['forms']]
        approved = {f['form_id']: f['growth_index'] for f in entries if f['index_status'] == 'creator-directed'}
        self.assertEqual(approved, growth.DIRECTED_INDICES)
        self.assertEqual(sum(f['index_status'] == 'working' for f in entries), 29)
        self.assertFalse(self.data['boundaries']['approved_geometry'])
        for mutate in (
            lambda d: d.update(status='creator-approved'),
            lambda d: d['boundaries'].update(approved_geometry=True),
            lambda d: d['boundaries'].update(gameplay_stat_binding=True),
            lambda d: d['branches'][0].update(end_height=3.0),
            lambda d: d['branches'][0]['forms'][2].update(index_status='creator-directed'),
        ):
            changed = copy.deepcopy(self.data)
            mutate(changed)
            with self.assertRaises(ValueError):
                growth.validate(changed)

    def test_growth_only_references_canonical_colors_and_projection_is_current(self):
        self.assertNotIn('#', json.dumps(self.data))
        palette = colors.load()
        original = copy.deepcopy(palette)
        growth.project(self.data, palette)
        self.assertEqual(palette, original)
        self.assertEqual(json.loads(growth.PROJECTION.read_text()), self.report)
        bound = {form['id']: form['hex'] for form in colors.form_bindings(palette)}
        for branch in self.report['branches']:
            for form in branch['forms']:
                self.assertEqual(form['hex'], bound[form['id']])


if __name__ == '__main__':
    unittest.main()
