import copy
from pathlib import Path
import tempfile
import unittest

import chromatic_color_standard as standard


class ChromaticColorStandardTests(unittest.TestCase):
    def setUp(self):
        self.data = standard.load(standard.V1_MANIFEST)

    def rejects(self, mutate, reason):
        candidate = copy.deepcopy(self.data)
        mutate(candidate)
        with self.assertRaisesRegex(ValueError, reason):
            standard.validate(candidate)

    def test_all_22_locked_slots_match_the_approved_source(self):
        self.assertEqual(len(standard.tokens(self.data)), 22)
        self.assertEqual(len({token['slot'] for token in standard.tokens(self.data)}), 22)
        for column, house in enumerate(self.data['houses']):
            self.assertEqual([(shade['name'], shade['hex']) for shade in house['shades']],
                             [row[column] for row in standard.approved_table()])

    def test_fifth_house_missing_house_and_wrong_cardinals_are_rejected(self):
        self.rejects(lambda d: d['houses'].append({'id': 'central-junction'}), 'four ordered Houses')
        self.rejects(lambda d: d['houses'].pop(), 'four ordered Houses')
        self.rejects(lambda d: d['houses'][0]['orientation'].update(cardinal='east'), 'orientation')
        self.rejects(lambda d: d['houses'][2]['orientation'].update(faculty='Body'), 'orientation')

    def test_order_missing_shades_and_duplicate_slots_are_rejected(self):
        self.rejects(lambda d: d['houses'][0]['shades'].reverse(), 'five ordered levels')
        self.rejects(lambda d: d['houses'][0]['shades'].pop(), 'five ordered levels')
        self.rejects(lambda d: d['universal'][1].update(slot='universal.full_black'), '22 unique token slots')

    def test_grayscale_derivation_and_integer_rounding(self):
        self.assertEqual([standard.neutral_gray(standard.rgb(shade['hex']))
                          for shade in self.data['houses'][0]['shades']],
                         [(22, 22, 22), (58, 58, 58), (107, 107, 107), (140, 140, 140), (173, 173, 173)])
        self.assertEqual(standard.neutral_gray((0, 0, 1)), (0, 0, 0))
        self.assertEqual(standard.neutral_gray((0, 1, 1)), (1, 1, 1))
        self.rejects(lambda d: d['houses'][3]['shades'][0].update(hex='#171616'), 'grayscale')

    def test_red_green_midpoints_round_half_up(self):
        self.assertEqual(standard.midpoint((0, 2, 254), (1, 3, 255)), (1, 3, 255))
        for house_index in (1, 2):
            for level_index in (1, 3):
                shades = self.data['houses'][house_index]['shades']
                expected = standard.midpoint(standard.rgb(shades[level_index - 1]['hex']),
                                             standard.rgb(shades[level_index + 1]['hex']))
                self.assertEqual(standard.rgb(shades[level_index]['hex']), expected)
        self.rejects(lambda d: d['houses'][1]['shades'][1].update(hex='#80303C'), 'midpoint')

    def test_name_and_anchor_edits_cannot_redefine_v1(self):
        self.rejects(lambda d: d['houses'][0]['shades'][0].update(name='Oxford Blue'), 'locked color')
        def recolor_anchor(data):
            data['houses'][0]['shades'][0]['hex'] = '#0C1026'
            gray = standard.neutral_gray(standard.rgb('#0C1026'))
            data['houses'][3]['shades'][0]['hex'] = '#' + ''.join(f'{channel:02X}' for channel in gray)
        self.rejects(recolor_anchor, 'locked color')
        self.rejects(lambda d: d['universal'][1].update(hex='#FEFEFE'), 'locked universal')

    def test_mixing_remains_caller_selected_and_not_a_runtime_control(self):
        self.rejects(lambda d: d['mathematics']['black_white_mix'].update(t_range=[-1, 1]), 'mixing contract')
        for binding in ('gameplay_stat_binding', 'temperature_binding', 'fan_speed_binding'):
            self.rejects(lambda d: d['mathematics']['black_white_mix'].update({binding: True}), 'mixing contract')
        self.rejects(lambda d: d['mathematics']['flynt'].update(equal_perceived_brightness=True), 'grayscale contract')

    def test_approved_document_is_immutable(self):
        with tempfile.TemporaryDirectory() as directory:
            document = Path(directory) / 'changed.md'
            document.write_bytes(standard.DOCUMENT.read_bytes() + b'\nchanged\n')
            with self.assertRaisesRegex(ValueError, 'approved Markdown changed'):
                standard.validate(self.data, document)

    def test_gpl_projection_contains_the_exact_manifest_rgb_values(self):
        projection = standard.gpl(self.data)
        records = [line for line in projection.splitlines() if '\t' in line]
        self.assertEqual(len(records), 22)
        for record, token in zip(records, standard.tokens(self.data)):
            color, label = record.split('\t', 1)
            self.assertEqual(tuple(map(int, color.split())), standard.rgb(token['hex']))
            self.assertEqual(label, f"{token['name']} ({token['slot']})")


class ChromaticV2Tests(unittest.TestCase):
    def setUp(self):
        self.data = standard.load(standard.V2_MANIFEST)

    def rejects(self, mutate, reason):
        candidate = copy.deepcopy(self.data)
        mutate(candidate)
        with self.assertRaisesRegex(ValueError, reason):
            standard.validate(candidate)

    def test_exact_34_distinct_values_and_twelve_unassigned_names(self):
        colors, _, _ = standard.approved_v2_tables()
        expected = {hexcode for row in colors for _, hexcode in row} | {'#000000', '#FFFFFF'}
        tokens = standard.tokens(self.data)
        self.assertEqual(len(tokens), 34)
        self.assertEqual({t['hex'] for t in tokens}, expected)
        self.assertEqual(len(expected), 34)
        unassigned = [t for t in tokens if t['name_status'] == 'unassigned']
        self.assertEqual(len(unassigned), 12)
        self.assertTrue(all(t['name'] is None and t['level'] in (3, 5, 7) for t in unassigned))
        self.rejects(lambda d: d['houses'][0]['shades'][2].update(name='Invented Blue'), 'formal name/status')
        self.rejects(lambda d: d['houses'][0]['shades'][2].update(name_status='approved'), 'formal name/status')
        self.rejects(lambda d: d['houses'][0]['shades'][0].update(hex='#0A1024'), 'locked v2.2 hex')

    def test_nine_half_up_midpoints_and_all_eight_grays(self):
        checks = 0
        for house in self.data['houses'][:3]:
            for index in (2, 4, 6):
                shades = house['shades']
                self.assertEqual(standard.rgb(shades[index]['hex']),
                                 standard.midpoint(standard.rgb(shades[index - 1]['hex']),
                                                   standard.rgb(shades[index + 1]['hex'])))
                checks += 1
        self.assertEqual(checks, 9)
        for blue, gray in zip(self.data['houses'][0]['shades'], self.data['houses'][3]['shades']):
            self.assertEqual(standard.neutral_gray(standard.rgb(blue['hex'])), standard.rgb(gray['hex']))
        self.rejects(lambda d: d['houses'][3]['shades'][0]['derived_from'].update(
            slots=[standard.v2_slot('stonebend', 2)]), 'derivation provenance')

    def test_every_form_assignment_is_locked_to_its_approved_position(self):
        _, forms, _ = standard.approved_v2_tables()
        self.assertEqual([s['form']['name'] for h in self.data['houses'] for s in h['shades']],
                         [f[1] for f in forms])
        for hi in range(4):
            for si in range(8):
                with self.subTest(house=hi, shade=si):
                    self.rejects(lambda d: d['houses'][hi]['shades'][si]['form'].update(name='Other'),
                                 'locked Form assignment')

    def test_cross_pairs_are_explicit_and_not_equal_palette_positions(self):
        pairs = self.data['relationships']['stonebend_glaushouse_cross_pairs']
        self.assertIn(['design.form.stonebend.ghoul', 'design.form.glaushouse.spirit'], pairs)
        self.assertIn(['design.form.stonebend.spectre', 'design.form.glaushouse.fairie'], pairs)
        self.rejects(lambda d: d['relationships']['stonebend_glaushouse_cross_pairs'][2].__setitem__(
            1, 'design.form.glaushouse.fairie'), 'relationships')

    def test_flynt_branches_masteries_and_distinct_identities(self):
        flynt = self.data['relationships']['flynt']
        self.assertEqual(len(flynt['sibling_families']), 3)
        self.assertEqual(flynt['chimera']['requires_primary_mastery'], [
            'design.form.flynt.gargoyle', 'design.form.flynt.werewolf', 'design.form.flynt.merman'])
        mutations = [
            lambda f: f.update(palette_order_is_evolution_order=True),
            lambda f: f['sibling_families'][0].update(independently_learnable=False),
            lambda f: f['sibling_families'][1].update(switching_mastery='automatic'),
            lambda f: f['chimera']['requires_primary_mastery'].append('design.form.flynt.medusa'),
            lambda f: f['chimera'].update(alternate_siblings_required=True),
            lambda f: f['manticorp'].update(distinct_from=['Tross sovereign office']),
            lambda f: f['manticorp'].update(grants_office=True),
            lambda f: f['merman'].update(distinct_from='Merman'),
        ]
        for mutate in mutations:
            self.rejects(lambda d: mutate(d['relationships']['flynt']), 'relationships')
        for key in self.data['relationships']['automatic_grants']:
            self.rejects(lambda d: d['relationships']['automatic_grants'].update({key: True}), 'relationships')

    def test_all_22_old_references_retain_names_and_colors(self):
        previous = standard.load(standard.V1_MANIFEST)
        for old in standard.tokens(previous):
            resolved = standard.resolve(self.data, old['slot'], source_version='1.0')
            self.assertEqual((old['name'], old['hex']), (resolved['name'], resolved['hex']))
        lapis = standard.resolve(self.data, 'house.stonebend.level.3', source_version='1.0')
        self.assertEqual((lapis['level'], lapis['name']), (4, 'Lapis Lazuli'))
        new = standard.resolve(self.data, standard.v2_slot('stonebend', 3), source_version='2.2')
        self.assertEqual(new['hex'], '#235581')
        for version in ('2.2', None, '2.1'):
            with self.assertRaises(ValueError):
                standard.resolve(self.data, 'house.stonebend.level.3', source_version=version)
        self.rejects(lambda d: d['compatibility']['level_migration'].update({'3': 3}), 'index migration')
        self.rejects(lambda d: d['compatibility']['slot_aliases'].update({
            'house.stonebend.level.3': standard.v2_slot('stonebend', 3)}), 'index migration')

    def test_no_fifth_house_extra_universal_or_missing_shade(self):
        self.rejects(lambda d: d['houses'].append({'id': 'central-junction'}), 'four ordered Houses')
        self.rejects(lambda d: d['houses'][0]['shades'].pop(), 'eight ordered levels')
        self.rejects(lambda d: d['universal'].append(d['universal'][0]), 'two universal endpoints')
        self.rejects(lambda d: d['universal'][0].update(level=0), 'universal endpoint')

    def test_visual_data_cannot_redefine_gameplay_or_hardware_boundary(self):
        for field in ('form_ids_are_runtime_ids', 'palette_indices_are_prerequisites', 'preview_on_miss_commits'):
            self.rejects(lambda d: d['design_rules'].update({field: True}), 'design/gameplay boundary')
        self.rejects(lambda d: d['design_rules'].update(commit_form_only_after='preview'), 'design/gameplay boundary')
        self.rejects(lambda d: d['design_rules']['current_synthesis'].insert(1, 'Color'), 'design/gameplay boundary')
        self.rejects(lambda d: d['compatibility'].update(save_migration=True), 'index migration')
        for field in ('gameplay_stat_binding', 'temperature_binding', 'fan_speed_binding', 'changes_source_tokens'):
            self.rejects(lambda d: d['mathematics']['black_white_mix'].update({field: True}), 'mixing boundary')

    def test_v2_approval_record_is_immutable(self):
        with tempfile.TemporaryDirectory() as directory:
            document = Path(directory) / 'changed.md'
            document.write_bytes(standard.V2_DOCUMENT.read_bytes() + b'\nchanged\n')
            with self.assertRaisesRegex(ValueError, 'approved Markdown changed'):
                standard.validate(self.data, document)

    def test_gpl_reads_active_source_and_does_not_name_new_colors(self):
        records = [line for line in standard.gpl(self.data).splitlines() if '\t' in line]
        self.assertEqual(len(records), 34)
        self.assertEqual(sum('UNASSIGNED' in record for record in records), 12)
        for record, token in zip(records, standard.tokens(self.data)):
            self.assertEqual(tuple(map(int, record.split('\t')[0].split())), standard.rgb(token['hex']))


class ChromaticV23Tests(unittest.TestCase):
    def setUp(self):
        self.data = standard.load()

    def test_all_32_inverted_bindings_match_approval(self):
        forms = standard.form_bindings(self.data)
        self.assertEqual(len({form['id'] for form in forms}), 32)
        self.assertEqual([(f['sequence'], f['name'], f['color_name'] or 'Name pending',
                           f['swatch_level'], f['hex']) for f in forms], standard.approved_v23_forms())
        self.assertEqual([f['swatch_level'] for f in forms],
                         list(range(8, 0, -1)) + [4, 3, 2, 1, 8, 7, 6, 5] + list(range(8, 0, -1)) * 2)
        by_name = {f['name']: f for f in forms}
        self.assertEqual(by_name['Gremlin']['hex'], '#79B7D8')
        self.assertEqual(by_name['Gargoyle']['hex'], '#ADADAD')
        self.assertEqual(by_name['Hecaton']['hex'], '#5A1F2A')

    def test_all_historical_color_references_keep_names_and_hexes(self):
        for version, path in [('1.0', standard.V1_MANIFEST), ('2.2', standard.V2_MANIFEST)]:
            for old in standard.tokens(standard.load(path)):
                new = standard.resolve(self.data, old['slot'], source_version=version)
                self.assertEqual((new['name'], new['hex']), (old['name'], old['hex']))
        self.assertEqual(len({t['hex'] for t in standard.tokens(self.data)}), 34)
        self.assertEqual(sum(t['name'] is None for t in standard.tokens(self.data)), 12)

    def test_historical_form_bindings_require_explicit_version(self):
        form_id = 'design.form.stonebend.gremlin'
        self.assertEqual(standard.resolve_form(form_id, source_version='2.2')['hex'], '#0B1026')
        self.assertEqual(standard.resolve_form(form_id, source_version='2.3')['hex'], '#79B7D8')
        with self.assertRaises(TypeError):
            standard.resolve_form(form_id)
        with self.assertRaises(ValueError):
            standard.resolve_form(form_id, source_version='1.0')
        with self.assertRaises(ValueError):
            standard.resolve(self.data, 'house.stonebend.level.3', source_version='2.3')

    def test_sequence_slot_and_index_cannot_be_conflated(self):
        mutations = [
            lambda d: d['houses'][0]['shades'].reverse(),
            lambda d: d['houses'][0]['forms'][0].update(swatch_slot=standard.v2_slot('stonebend', 1)),
            lambda d: d['houses'][0]['forms'][0].update(sequence=8),
            lambda d: d['houses'][0]['forms'][0].update(growth_index=0),
            lambda d: d['houses'][1]['forms'][0].update(branch='second'),
            lambda d: d['houses'][0]['shades'][0].update(form={'name': 'Gremlin'}),
        ]
        for mutation in mutations:
            data = copy.deepcopy(self.data)
            mutation(data)
            with self.assertRaisesRegex(ValueError, 'swatch identity or Form binding'):
                standard.validate(data)

    def test_relationships_approval_and_gameplay_boundary_are_preserved(self):
        self.assertEqual(self.data['relationships'], standard.load(standard.V2_MANIFEST)['relationships'])
        for field in ('color_controls_gameplay', 'base_hueman_house_swatch'):
            data = copy.deepcopy(self.data)
            data['design_rules'][field] = True
            with self.assertRaisesRegex(ValueError, 'design/gameplay'):
                standard.validate(data)
        data = copy.deepcopy(self.data)
        data['compatibility']['save_migration'] = True
        with self.assertRaisesRegex(ValueError, 'compatibility'):
            standard.validate(data)


if __name__ == '__main__':
    unittest.main()
