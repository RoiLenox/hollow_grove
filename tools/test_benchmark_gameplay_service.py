import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).with_name('benchmark-gameplay-service.py')
SPEC = importlib.util.spec_from_file_location('benchmark_gameplay_service', SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class SourceProvenanceTests(unittest.TestCase):
    def test_checkout_root_reports_its_own_commit_and_dirty_state(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            subprocess.run(['git', '-C', str(root), 'config', 'user.email', 'audit@example.invalid'], check=True)
            subprocess.run(['git', '-C', str(root), 'config', 'user.name', 'Audit'], check=True)
            tracked = root / 'tracked'
            tracked.write_text('source\n')
            source = root / 'project'
            source.mkdir()
            subprocess.run(['git', '-C', str(root), 'add', 'tracked'], check=True)
            subprocess.run(['git', '-C', str(root), 'commit', '-qm', 'initial'], check=True)
            commit = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()

            self.assertEqual(MODULE.source_provenance(source, source), (commit, False))
            (root / 'untracked').write_text('change\n')
            self.assertEqual(MODULE.source_provenance(source, source), (commit, True))

    def test_data_directory_inside_checkout_does_not_claim_enclosing_commit(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            data = root / 'installed' / 'world'
            data.mkdir(parents=True)
            source = root / 'project'
            source.mkdir()
            self.assertEqual(MODULE.source_provenance(data, source), (None, None))

    def test_non_checkout_reports_unknown(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.assertEqual(MODULE.source_provenance(root, root), (None, None))


if __name__ == '__main__':
    unittest.main()
