#!/usr/bin/env python3
"""Validate or project the approved design tokens; never install a runtime theme."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
DESIGN = ROOT / 'docs/design'
V1_MANIFEST = DESIGN / 'hollow_grove_chromatic_color_standard_v1_0.json'
V1_MANIFEST_SHA256 = '93069b65055ea439d639c0233cbc2ca7c173bc7d17acdfac1ba74fbaef9f037f'
V2_MANIFEST = DESIGN / 'hollow_grove_chromatic_color_standard_v2_2.json'
V2_MANIFEST_SHA256 = 'fc814525360c4a7949c23bc7325a85ef2f8e6701ccedc4b7fb61e69912752bbc'
MANIFEST = DESIGN / 'hollow_grove_chromatic_color_standard_v2_3_0.json'
DOCUMENT = DESIGN / 'CODEX_HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD.md'
DOCUMENT_SHA256 = 'fa12129c3148351e0fdbc502de8df610298e3432ab6194ae86f8a34ee2f7dc19'
V2_DOCUMENT = DESIGN / 'HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_2_CODEX_ALL_IN_ONE.md'
V2_DOCUMENT_SHA256 = '61f37432ea5de763630e9b003aa4a35804b6cfab02d3c570c42834c60f9f1d23'
V23_DOCUMENT = DESIGN / 'HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_3_CODEX_ALL_IN_ONE.md'
V23_DOCUMENT_SHA256 = 'f1a7a7d9f34f2e39feceaae7c12baf676b5cc815afcc45aed71b9bb5a42c4a54'
LEVEL_MIGRATION = {1: 1, 2: 2, 3: 4, 4: 6, 5: 8}
HOUSE_IDS = ('stonebend', 'sandmanor', 'glaushouse', 'flynt')
ORIENTATIONS = (('north', 12, 'Body'), ('east', 3, 'Soul'),
                ('south', 6, 'Mind'), ('west', 9, 'Spirit'))
LEVEL_NAMES = ('Deep', 'Shadow', 'Core', 'Highlight', 'Peak')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def rgb(value):
    require(isinstance(value, str) and re.fullmatch(r'#[0-9A-F]{6}', value),
            'colors must use uppercase six-digit RGB hex')
    return tuple(int(value[index:index + 2], 16) for index in (1, 3, 5))


def midpoint(first, second):
    return tuple((left + right + 1) // 2 for left, right in zip(first, second))


def neutral_gray(color):
    channel = (sum(color) + 1) // 3
    return (channel, channel, channel)


def approved_table(document=DOCUMENT):
    source = document.read_bytes()
    require(hashlib.sha256(source).hexdigest() == DOCUMENT_SHA256,
            'approved Markdown changed; preserve v1.0 and review a new version')
    rows = []
    for line in source.decode('utf-8').splitlines():
        if re.match(r'^\| [1-5] — ', line):
            cells = [cell.strip() for cell in line.strip('|').split('|')]
            colors = [re.fullmatch(r'(.+) `(#[0-9A-F]{6})`', cell) for cell in cells[1:]]
            require(len(colors) == 4 and all(colors), 'invalid approved color table')
            rows.append([(match[1], match[2]) for match in colors])
    require(len(rows) == 5, 'approved table must contain five levels')
    return rows


def tokens(data):
    return [shade for house in data['houses'] for shade in house['shades']] + data['universal']


def validate_v1(data, document=DOCUMENT):
    require(isinstance(data, dict), 'standard must be a JSON object')
    require(data.get('schema_version') == '1.0.0' and data.get('version') == '1.0',
            'unsupported Chromatic Color Standard version')
    require(data.get('standard_id') == 'hollow-grove.chromatic-color-standard'
            and data.get('status') == 'creator-approved-design'
            and data.get('approved_on') == '2026-11-01', 'design identity/approval changed')
    require(data.get('provenance', {}).get('document') == DOCUMENT.name
            and data['provenance'].get('document_sha256') == DOCUMENT_SHA256,
            'approved source provenance changed')
    require(data.get('levels') == [{'level': i, 'name': name}
                                  for i, name in enumerate(LEVEL_NAMES, 1)],
            'five ordered shade levels are required')
    houses = data.get('houses', [])
    require([house.get('id') for house in houses] == list(HOUSE_IDS),
            'exactly four ordered Houses are required; Central Junction is not a House')
    for house, orientation in zip(houses, ORIENTATIONS):
        require(house.get('orientation') == dict(zip(('cardinal', 'clock', 'faculty'), orientation)),
                'House orientation changed: ' + house['id'])
        shades = house.get('shades', [])
        require([shade.get('level') for shade in shades] == list(range(1, 6)),
                'each House needs five ordered levels')
        for shade in shades:
            require(shade.get('slot') == f"house.{house['id']}.level.{shade['level']}",
                    'House token slot does not match its House and level')
    require(len(data.get('universal', [])) == 2, 'exactly two universal slots are required')
    all_tokens = tokens(data)
    require(len(all_tokens) == 22 and len({token.get('slot') for token in all_tokens}) == 22,
            'exactly 22 unique token slots are required')
    # Slot uniqueness is structural. Pairwise-unique RGB values are not a schema rule.
    for token in all_tokens:
        rgb(token.get('hex'))
        require(isinstance(token.get('name'), str) and token['name'], 'color name is required')
    colors = {house['id']: [rgb(shade['hex']) for shade in house['shades']] for house in houses}
    require(colors['flynt'] == [neutral_gray(color) for color in colors['stonebend']],
            'Flynt must use the approved unweighted RGB grayscale derivation')
    for house in ('sandmanor', 'glaushouse'):
        for index in (1, 3):
            require(colors[house][index] == midpoint(colors[house][index - 1], colors[house][index + 1]),
                    'red/green midpoint rounding changed: ' + house)
    approved = approved_table(document)
    for column, house in enumerate(houses):
        require([(shade['name'], shade['hex']) for shade in house['shades']]
                == [row[column] for row in approved], 'locked color name/hex changed: ' + house['id'])
    require([(token['slot'], token['name'], token['hex']) for token in data['universal']] == [
        ('universal.full_black', 'Full Black', '#000000'),
        ('universal.full_white', 'Full White', '#FFFFFF')], 'locked universal tokens changed')
    require([token.get('regional_associations') for token in data['universal']] == [
        ['South Stonebend', 'Glaüshouse', 'South Flynt'], ['North Flynt']],
        'universal regional associations changed')
    math = data.get('mathematics', {})
    require(math.get('flynt') == {
        'source_house': 'stonebend', 'formula': 'Y = floor((R+G+B+1)/3)',
        'output': '(Y,Y,Y)', 'metric': 'unweighted RGB Euclidean distance',
        'equal_perceived_brightness': False}, 'grayscale contract changed')
    require(math.get('midpoints') == {
        'houses': ['sandmanor', 'glaushouse'], 'anchor_levels': [1, 3, 5],
        'derived_levels': [2, 4], 'rounding': 'half-up per 8-bit RGB channel'},
        'midpoint contract changed')
    require(math.get('black_white_mix') == {
        'formula': '(1-t)*channel + t*target_channel', 't_range': [0, 1],
        't_source': 'caller-selected', 'gameplay_stat_binding': False,
        'temperature_binding': False, 'fan_speed_binding': False,
        'integer_quantization': 'not specified in the approved Markdown'},
        'caller-selected mixing contract changed')
    rules = data.get('design_rules', {})
    require(rules.get('central_junction_is_house') is False
            and rules.get('regional_associations_mandate_recoloring') is False,
            'design boundary changed')
    require(rules.get('master_font') == 'Microgramma'
            and [house.get('font') for house in houses] == ['Narkis Block', 'Averta', 'Eurostile', 'Frutiger'],
            'approved typography changed')
    return data


def v2_slot(house, level):
    return f'chromatic.v2_2.house.{house}.level.{level}'


def approved_v2_tables(document=V2_DOCUMENT):
    source = document.read_bytes()
    require(hashlib.sha256(source).hexdigest() == V2_DOCUMENT_SHA256,
            'approved Markdown changed; preserve v2.2 and review a new version')
    lines = source.decode('utf-8').splitlines()
    colors = [re.findall(r'\*\*([^*]+)\*\* `(#[0-9A-F]{6})`', line)
              for line in lines if re.match(r'^\| \*\*[1-8]\*\*', line)]
    forms = [[cell.strip().strip('`') for cell in line.strip('|').split('|')]
             for line in lines if re.match(r'^\| [1-8] \|', line)]
    require(len(colors) == 8 and all(len(row) == 4 for row in colors)
            and len(forms) == 32, 'invalid approved v2.2 tables')
    pair_line = next(line for line in lines if line.startswith('**Preserve the established'))
    form_names = {row[1] for row in forms}
    pairs = [(a, b) for a, b in re.findall(r'(\w+)↔(\w+)', pair_line) if a in form_names]
    require(len(pairs) == 8, 'eight explicit cross-pair links are required')
    return colors, forms, pairs


def validate_v2(data, document=V2_DOCUMENT):
    colors, forms, pairs = approved_v2_tables(document)
    require(data.get('schema_version') == '2.0.0' and data.get('version') == '2.2',
            'unsupported Chromatic Color Standard version')
    require(data.get('standard_id') == 'hollow-grove.chromatic-color-standard'
            and data.get('status') == 'creator-approved-design'
            and data.get('approved_on') == '2026-11-01', 'design identity/approval changed')
    provenance = data.get('provenance', {})
    require(provenance.get('document') == V2_DOCUMENT.name
            and provenance.get('document_sha256') == V2_DOCUMENT_SHA256
            and provenance.get('previous_manifest') == V1_MANIFEST.name
            and provenance.get('previous_manifest_sha256') == V1_MANIFEST_SHA256,
            'approved source provenance changed')
    require(hashlib.sha256(V1_MANIFEST.read_bytes()).hexdigest() == V1_MANIFEST_SHA256,
            'historical v1.0 manifest changed')
    previous = load(V1_MANIFEST)
    require(data.get('levels') == list(range(1, 9)), 'eight ordered levels are required')
    houses = data.get('houses', [])
    require([house.get('id') for house in houses] == list(HOUSE_IDS),
            'exactly four ordered Houses are required; Central Junction is not a House')
    inverse = {new: old for old, new in LEVEL_MIGRATION.items()}
    form_ids = {}
    for column, house in enumerate(houses):
        hid = house['id']
        for key in ('name', 'family', 'font', 'orientation'):
            require(house.get(key) == previous['houses'][column][key],
                    'House identity/typography/orientation changed: ' + hid)
        if hid == 'flynt':
            require(house.get('place_name') == 'Flynt, MI', 'Flynt place identity changed')
        shades = house.get('shades', [])
        require([shade.get('level') for shade in shades] == list(range(1, 9)),
                'each House needs eight ordered levels')
        for shade in shades:
            level = shade['level']
            require(shade.get('slot') == v2_slot(hid, level), 'versioned token slot changed')
            name, hexcode = colors[level - 1][column]
            require(shade.get('hex') == hexcode, 'locked v2.2 hex changed')
            require((shade.get('name'), shade.get('name_status')) ==
                    ((None, 'unassigned') if name == 'UNASSIGNED' else (name, 'approved')),
                    'formal name/status changed')
            approved_form = forms[column * 8 + level - 1]
            require(approved_form[0] == str(level) and approved_form[2] == hexcode,
                    'approval table/Form table disagreement')
            form_name = approved_form[1]
            form_id = f'design.form.{hid}.{form_name.lower()}'
            expected_form = {'id': form_id, 'name': form_name}
            if hid == 'sandmanor':
                expected_form['branch'] = approved_form[3].lower()
            require(shade.get('form') == expected_form, 'locked Form assignment changed')
            form_ids[form_name] = form_id
            old_slot = f'house.{hid}.level.{inverse[level]}' if level in inverse else None
            require(shade.get('provenance') == {'section': 'A; B', 'previous_slot': old_slot},
                    'token provenance changed')
            if hid == 'flynt':
                derived = {'method': 'rgb_mean_gray', 'slots': [v2_slot('stonebend', level)]}
            elif level in (3, 5, 7):
                derived = {'method': 'rgb_midpoint_half_up',
                           'slots': [v2_slot(hid, level - 1), v2_slot(hid, level + 1)]}
            else:
                derived = {'method': 'preserved_v1', 'version': '1.0', 'slots': [old_slot]}
            require(shade.get('derived_from') == derived, 'token derivation provenance changed')
    require(len(data.get('universal', [])) == 2, 'exactly two universal endpoints are required')
    for token, old in zip(data['universal'], previous['universal']):
        require(token == {**old, 'name_status': 'approved', 'scope': 'universal',
                          'provenance': {'section': 'A', 'previous_slot': old['slot']},
                          'derived_from': {'method': 'preserved_v1', 'version': '1.0',
                                           'slots': [old['slot']]}}, 'universal endpoint changed')
    all_tokens = tokens(data)
    require(len(all_tokens) == 34 and len({t['hex'] for t in all_tokens}) == 34
            and len({t['slot'] for t in all_tokens}) == 34, '34 unique colors/slots required')
    for token in all_tokens:
        rgb(token['hex'])
    require(len(form_ids) == 32, '32 unique design Form IDs required')
    for column in range(3):
        shades = houses[column]['shades']
        for index in (2, 4, 6):
            require(rgb(shades[index]['hex']) == midpoint(rgb(shades[index - 1]['hex']),
                                                        rgb(shades[index + 1]['hex'])),
                    'RGB midpoint half-up equation failed')
    for blue, gray in zip(houses[0]['shades'], houses[3]['shades']):
        require(neutral_gray(rgb(blue['hex'])) == rgb(gray['hex']), 'Flynt grayscale equation failed')
    aliases = {f'house.{hid}.level.{old}': v2_slot(hid, new)
               for hid in HOUSE_IDS for old, new in LEVEL_MIGRATION.items()}
    aliases.update({token['slot']: token['slot'] for token in previous['universal']})
    require(data.get('compatibility') == {
        'previous_version': '1.0', 'level_migration': {str(k): v for k, v in LEVEL_MIGRATION.items()},
        'slot_aliases': aliases,
        'unversioned_house_references': 'require explicit source_version; never reinterpret a v1 level as a v2 level',
        'runtime_form_mapping': 'deferred; design.form IDs are not save/enum IDs',
        'save_migration': False}, 'v1 compatibility/index migration changed')
    by_slot = {t['slot']: t for t in all_tokens}
    for old in tokens(previous):
        current = by_slot[aliases[old['slot']]]
        require((old['name'], old['hex']) == (current['name'], current['hex']),
                'all 22 historical colors/names must survive migration')
    f = form_ids.__getitem__
    families = [{'origin': f(origin), 'forms': [f(a), f(b)], 'independently_learnable': True,
                 'switching_mastery': 'earned-pair-specific-challenge'} for origin, a, b in
                [('Gremlin', 'Gargoyle', 'Medusa'), ('Gnome', 'Werewolf', 'Vampire'),
                 ('Elf', 'Merman', 'Mermaid')]]
    require(data.get('relationships') == {
        'scope': 'visual-design-only; not serialized runtime IDs or enacted prerequisites',
        'sandmanor_branches': [[f(n) for n in ('Gnome', 'Satyr', 'Minotaur', 'Hecaton')],
                              [f(n) for n in ('Elf', 'Faun', 'Centaur', 'Pegasus')]],
        'stonebend_glaushouse_cross_pairs': [[f(a), f(b)] for a, b in pairs],
        'flynt': {'palette_order_is_evolution_order': False, 'sibling_families': families,
                  'chimera': {'form': f('Chimera'),
                              'requires_primary_mastery': [f(n) for n in ('Gargoyle', 'Werewolf', 'Merman')],
                              'alternate_siblings_required': False},
                  'manticorp': {'form': f('Manticorp'),
                                'distinct_from': ['Manticorp organization', 'Tross sovereign office'],
                                'grants_office': False},
                  'merman': {'form': f('Merman'), 'distinct_from': 'Mariner occupational specialty'}},
        'automatic_grants': dict.fromkeys(('xp', 'employment', 'agency', 'citizenship', 'office'), False)
    }, 'visual relationships/branching/identity boundaries changed')
    require(data.get('mathematics') == {
        'color_space': '8-bit sRGB channels; not perceptually uniform',
        'midpoints': {'houses': list(HOUSE_IDS[:3]), 'derived_levels': [3, 5, 7],
                      'anchor_pairs': [[2, 4], [4, 6], [6, 8]], 'formula': '(a+b+1)//2'},
        'flynt': {'source_house': 'stonebend', 'levels': list(range(1, 9)),
                  'formula': '(R+G+B+1)//3', 'output': '(Y,Y,Y)', 'equal_perceived_brightness': False},
        'black_white_mix': {'black_formula': 'round((1-t)*c)', 'white_formula': 'round((1-t)*c+255*t)',
                            't_range': [0, 1], 't_source': 'caller-selected',
                            'round_tie_policy': 'unspecified; no mixing adapter implemented',
                            'changes_source_tokens': False, 'gameplay_stat_binding': False,
                            'temperature_binding': False, 'fan_speed_binding': False}
    }, 'mathematics/mixing boundary changed')
    require(data.get('design_rules') == {
        **previous['design_rules'], 'form_ids_are_runtime_ids': False,
        'palette_indices_are_prerequisites': False, 'commit_form_only_after': 'lawful Kiss Landing',
        'preview_on_miss_commits': False,
        'current_synthesis': ['Point', 'Triway', 'Fourway', 'HollowGrove', 'CurrentSeam', 'AuraBeam', 'Point²'],
        'execution': ['Recipe', 'Compiler', 'Scripts', 'Aim', 'Fire', 'Miss|Kiss', 'Point²'],
        'frame_state': ['Frame', 'Current Prism', 'Flow', 'Glow']}, 'design/gameplay boundary changed')
    return data


def approved_v23_forms(document=V23_DOCUMENT):
    source = document.read_bytes()
    require(hashlib.sha256(source).hexdigest() == V23_DOCUMENT_SHA256,
            'approved v2.3 Markdown changed')
    rows = re.findall(r'^\| (\d+) \| \*\*([^*]+)\*\* \| ([^|]+) \| (\d+) \| `(#[0-9A-F]{6})` \|$',
                      source.decode('utf-8'), re.MULTILINE)
    require(len(rows) == 32, '32 approved v2.3 Form bindings required')
    return [(int(sequence), name, color.strip(), int(slot), hexcode)
            for sequence, name, color, slot, hexcode in rows]


def validate_v23(data, document=V23_DOCUMENT):
    rows = approved_v23_forms(document)
    require(hashlib.sha256(V2_MANIFEST.read_bytes()).hexdigest() == V2_MANIFEST_SHA256,
            'historical v2.2 manifest changed')
    previous = load(V2_MANIFEST)
    require(data.get('schema_version') == '3.0.0' and data.get('version') == '2.3',
            'unsupported Chromatic Color Standard version')
    require(data.get('standard_id') == previous['standard_id']
            and data.get('status') == 'creator-approved-design'
            and data.get('approved_on') == '2026-11-03', 'design identity/approval changed')
    provenance = data.get('provenance', {})
    require(provenance.get('document') == V23_DOCUMENT.name
            and provenance.get('document_sha256') == V23_DOCUMENT_SHA256
            and provenance.get('previous_manifest') == V2_MANIFEST.name
            and provenance.get('previous_manifest_sha256') == V2_MANIFEST_SHA256,
            'approved source provenance changed')
    require(data.get('levels') == previous['levels'], 'swatch order must remain dark to light')
    houses = data.get('houses', [])
    require([house.get('id') for house in houses] == list(HOUSE_IDS), 'four ordered Houses required')
    for column, (house, old) in enumerate(zip(houses, previous['houses'])):
        expected = copy.deepcopy(old)
        for shade in expected['shades']:
            del shade['form']
        forms = []
        for sequence, name, color, level, hexcode in rows[column * 8:column * 8 + 8]:
            require(sequence == len(forms) + 1, 'visual Form sequence changed')
            token = old['shades'][level - 1]
            require(token['hex'] == hexcode and (token['name'] or 'Name pending') == color,
                    'v2.3 approval changes historical swatches')
            form = {'id': f"design.form.{old['id']}.{name.lower()}", 'name': name,
                    'sequence': sequence, 'swatch_slot': token['slot']}
            if old['id'] == 'sandmanor':
                form['branch'] = 'first' if sequence <= 4 else 'second'
            forms.append(form)
        expected['forms'] = forms
        require(house == expected, 'locked v2.3 swatch identity or Form binding changed: ' + old['id'])
    for field in ('universal', 'mathematics', 'relationships'):
        require(data.get(field) == previous[field], 'historical swatches/math/relationships changed')
    require(data.get('compatibility') == {
        'previous_version': '2.2',
        'color_slot_aliases': {
            '1.0': previous['compatibility']['slot_aliases'],
            '2.2': {t['slot']: t['slot'] for t in tokens(previous)},
            '2.3': {t['slot']: t['slot'] for t in tokens(previous)}},
        'swatch_ids': 'v2.2 token IDs retained; swatch colors and dark-to-light indices are unchanged',
        'form_bindings': 'require explicit source_version; historical Form colors are not silently reinterpreted',
        'runtime_form_mapping': 'deferred; design.form IDs are not save/enum IDs',
        'save_migration': False}, 'explicit color/Form compatibility changed')
    require(data.get('design_rules') == {
        **previous['design_rules'], 'light_emergence_dark_mastery': True,
        'base_hueman_house_swatch': None, 'color_controls_gameplay': False},
        'design/gameplay boundary changed')
    return data


def validate(data, document=None):
    require(isinstance(data, dict), 'standard must be a JSON object')
    if data.get('version') == '1.0':
        return validate_v1(data, document or DOCUMENT)
    if data.get('version') == '2.2':
        return validate_v2(data, document or V2_DOCUMENT)
    if data.get('version') == '2.3':
        return validate_v23(data, document or V23_DOCUMENT)
    raise ValueError('unsupported Chromatic Color Standard version')


def resolve(data, slot, *, source_version):
    """Resolve a design reference without changing its historical meaning or saves."""
    if data.get('version') == '2.3':
        validate_v23(data)
        aliases = data['compatibility']['color_slot_aliases']
        require(source_version in aliases, 'explicit supported source_version required')
        slot = aliases[source_version].get(slot)
        by_slot = {token['slot']: token for token in tokens(data)}
        require(slot in by_slot, 'unknown or unversioned token reference')
        return by_slot[slot]
    validate_v2(data)
    if source_version == '1.0':
        slot = data['compatibility']['slot_aliases'].get(slot)
    else:
        require(source_version == '2.2', 'explicit supported source_version required')
    by_slot = {token['slot']: token for token in tokens(data)}
    require(slot in by_slot, 'unknown or unversioned token reference')
    return by_slot[slot]


def load(path=MANIFEST):
    return validate(json.loads(path.read_text(encoding='utf-8')))


def form_bindings(data):
    """Project visual identities from an explicitly loaded design version."""
    validate(data)
    require(data['version'] in ('2.2', '2.3'), 'this version has no Form bindings')
    by_slot = {token['slot']: token for token in tokens(data)}
    records = []
    for house in data['houses']:
        forms = house.get('forms') if data['version'] == '2.3' else [
            {**s['form'], 'sequence': s['level'], 'swatch_slot': s['slot']} for s in house['shades']]
        for form in forms:
            token = by_slot[form['swatch_slot']]
            records.append({**form, 'house_id': house['id'], 'swatch_level': token['level'],
                            'hex': token['hex'], 'color_name': token['name']})
    return records


def resolve_form(form_id, *, source_version):
    require(source_version in ('2.2', '2.3'), 'explicit Form source_version required')
    data = load(V2_MANIFEST if source_version == '2.2' else MANIFEST)
    records = {form['id']: form for form in form_bindings(data)}
    require(form_id in records, 'unknown design Form ID')
    return records[form_id]


def gpl(data):
    validate(data)
    lines = ['GIMP Palette', f"Name: Hollow Grove Chromatic Color Standard v{data['version']}",
             f"Columns: {len(data['levels'])}", '# Design preview']
    for token in tokens(data):
        red, green, blue = rgb(token['hex'])
        lines.append(f"{red:3d} {green:3d} {blue:3d}\t{token['name'] or 'UNASSIGNED'} ({token['slot']})")
    return '\n'.join(lines) + '\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=('validate', 'gpl'), nargs='?', default='validate')
    parser.add_argument('--version', choices=('1.0', '2.2', '2.3'), default='2.3')
    args = parser.parse_args()
    try:
        data = load({'1.0': V1_MANIFEST, '2.2': V2_MANIFEST, '2.3': MANIFEST}[args.version])
        print(gpl(data) if args.command == 'gpl' else
              f"PASS: Chromatic Color Standard v{data['version']}; {len(tokens(data))} locked slots, four Houses, equations and source verified",
              end='' if args.command == 'gpl' else '\n')
    except (ValueError, KeyError, TypeError, OSError) as error:
        parser.exit(1, f'Chromatic Color Standard: {error}\n')


if __name__ == '__main__':
    main()
