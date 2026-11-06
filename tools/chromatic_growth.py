#!/usr/bin/env python3
"""Review-only v0.4 sizing; never a source of gameplay stats or production geometry."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import re

import chromatic_color_standard as colors

DOCUMENT = colors.DESIGN / 'HUEMAN_CHROMATIC_GROWTH_SYSTEM_V0_4_HEX_DISTANCE_CODEX.md'
DOCUMENT_SHA256 = '72943c48740e47fcc16e08abe71964fda0775da74442e921e0954716c8bc156d'
MANIFEST = colors.DESIGN / 'hueman_chromatic_growth_system_v0_4_hex_distance_working.json'
PROJECTION = colors.ROOT / 'hueman_godot/tools/chromatic_growth_preview.json'
BRANCH_IDS = ('stonebend', 'sandmanor_first', 'sandmanor_second', 'glaushouse', 'flynt_comparison')
DIRECTED_INDICES = {'design.form.stonebend.gremlin': 0, 'design.form.stonebend.goblin': 1,
                    'design.form.flynt.gargoyle': 2}
require = colors.require


def document_tables():
    source = DOCUMENT.read_bytes()
    require(hashlib.sha256(source).hexdigest() == DOCUMENT_SHA256, 'growth source document changed')
    text = source.decode('utf-8')
    anchors = re.findall(r'^\| ([^|]+) \| ([0-9.]+)× \| ([0-9.]+)× \|', text, re.MULTILINE)
    rows = re.findall(r'^\| (\d+) \| \*\*([^*]+)\*\* \| [^|]+ \| `(#[0-9A-F]{6})` \| ([0-9.]+) \| \*\*([0-9.]+)%\*\* \| (\d+)px \|$', text, re.MULTILINE)
    require(len(anchors) == 5 and len(rows) == 32, 'complete growth tables required')
    return anchors, rows


def expected_branches(palette):
    anchors, rows = document_tables()
    forms = colors.form_bindings(palette)
    branches, offset = [], 0
    for branch_id, anchor, count in zip(BRANCH_IDS, anchors, (8, 4, 4, 8, 8)):
        label, start, end = anchor
        entries = []
        for row, form in zip(rows[offset:offset + count], forms[offset:offset + count]):
            index, name, hexcode, _, _, _ = row
            require((form['name'], form['hex']) == (name, hexcode), 'growth/color binding mismatch')
            directed = form['id'] in DIRECTED_INDICES
            if directed:
                require(int(index) == DIRECTED_INDICES[form['id']], 'creator-directed index changed')
            entries.append({'form_id': form['id'], 'growth_index': int(index),
                            'index_status': 'creator-directed' if directed else 'working'})
        branches.append({'id': branch_id, 'label': label.strip(), 'start_height': float(start),
                         'end_height': float(end), 'anchors_status': 'working',
                         'comparison_only': branch_id == 'flynt_comparison', 'forms': entries})
        offset += count
    return branches


def validate(data, palette=None):
    palette = palette if palette is not None else colors.load()
    colors.validate(palette)
    require(palette['version'] == '2.3', 'growth requires explicit v2.3 Form bindings')
    require(data.get('schema_version') == '1.0.0' and data.get('version') == '0.4'
            and data.get('system_id') == 'hueman.chromatic-growth'
            and data.get('status') == 'working-review-only', 'growth approval boundary changed')
    require(data.get('palette') == {'manifest': colors.MANIFEST.name, 'version': '2.3'},
            'growth palette reference changed')
    source = data.get('provenance', {})
    require(source.get('document') == DOCUMENT.name and source.get('document_sha256') == DOCUMENT_SHA256,
            'growth provenance changed')
    require(data.get('formula') == {
        'metric': 'Euclidean distance of 8-bit sRGB channel triples',
        'path': 'normalized cumulative adjacent distances within each branch',
        'height': 'start + normalized_path * (end - start)', 'status': 'working',
        'illustrative_standing_pixels': 32, 'rounding': 'half-up'}, 'growth formula changed')
    require(data.get('boundaries') == {
        'approved_geometry': False, 'gameplay_stat_binding': False, 'progression_gates': False,
        'production_hitboxes': False, 'sprite_uniform_scaling': False,
        'base_hueman_house_swatch': None, 'natural_skin_personalized': True,
        'one_active_current_form': True}, 'growth/production boundary changed')
    require(data.get('branches') == expected_branches(palette), 'working growth anchors/indices changed')
    return data


def load():
    return validate(json.loads(MANIFEST.read_text(encoding='utf-8')))


def cumulative_heights(hexes, start, end):
    """Distances follow the polyline; indices and packed hex integers are irrelevant."""
    require(len(hexes) >= 2 and math.isfinite(start) and math.isfinite(end)
            and 0 < start < end, 'positive increasing working anchors required')
    triplets = [colors.rgb(value) for value in hexes]
    gaps = [0.0] + [math.dist(a, b) for a, b in zip(triplets, triplets[1:])]
    total = math.fsum(gaps)
    require(total > 0, 'zero color path cannot define growth')
    result = []
    for index, gap in enumerate(gaps):
        path = math.fsum(gaps[:index + 1])
        position = path / total
        result.append({'rgb_gap': gap, 'cumulative_distance': path,
                       'normalized_distance': position, 'height_ratio': start + position * (end - start)})
    return total, result


def project(data, palette=None):
    palette = palette if palette is not None else colors.load()
    validate(data, palette)
    forms = {form['id']: form for form in colors.form_bindings(palette)}
    branches = []
    for branch in data['branches']:
        bound = [forms[entry['form_id']] for entry in branch['forms']]
        total, targets = cumulative_heights([form['hex'] for form in bound], branch['start_height'], branch['end_height'])
        records = []
        for form, entry, target in zip(bound, branch['forms'], targets):
            pixels = math.floor(target['height_ratio'] * data['formula']['illustrative_standing_pixels'] + 0.5)
            records.append({**form, **entry, **target, 'illustrative_pixels': pixels})
        branches.append({**branch, 'total_rgb_distance': total, 'forms': records})
    return {'schema_version': '1.0.0', 'generated_by': 'tools/chromatic_growth.py project',
            'status': 'working-review-only', 'palette_version': palette['version'], 'growth_version': data['version'],
            'palette_sha256': hashlib.sha256(colors.MANIFEST.read_bytes()).hexdigest(),
            'growth_sha256': hashlib.sha256(MANIFEST.read_bytes()).hexdigest(),
            'illustrative_standing_pixels': data['formula']['illustrative_standing_pixels'], 'branches': branches}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=('validate', 'project'), nargs='?', default='validate')
    parser.add_argument('--check', action='store_true', help='verify the checked Godot projection without writing')
    args = parser.parse_args()
    try:
        data = load()
        output = json.dumps(project(data), ensure_ascii=False, indent=2, allow_nan=False) + '\n'
        if args.command == 'project':
            if args.check:
                require(PROJECTION.read_text(encoding='utf-8') == output, 'generated growth preview drift; run project')
                print('PASS: Godot growth projection matches both sources')
            else:
                PROJECTION.write_text(output, encoding='utf-8')
                print('Wrote review-only Godot projection:', PROJECTION)
        else:
            print('PASS: v0.4 working proposal; 32 derived targets; only 3 creator-directed Growth Indices')
    except (ValueError, KeyError, TypeError, OSError) as error:
        parser.exit(1, f'Chromatic growth: {error}\n')


if __name__ == '__main__':
    main()
