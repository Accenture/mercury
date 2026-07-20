import importlib.util
import os
import sys
import tempfile
import unittest
from pathlib import Path

script_path = Path(__file__).parent / "archive-fact.py"
spec = importlib.util.spec_from_file_location("archive_fact", str(script_path))
archive_fact = importlib.util.module_from_spec(spec)
sys.modules["archive_fact"] = archive_fact
spec.loader.exec_module(archive_fact)

CONT = """# Continuity

## Open Threads

- [x] **Shipped vX — alpha.** Did alpha.
  More alpha detail (a continuation line).
  <!-- id: alpha-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->

- [x] **Shipped vY — beta.** Did beta.
  <!-- id: beta-fact | created: 2026-01-02 | last_used: 2026-01-02 | uses: 1 | tier: working -->

- [ ] **Keep me open.**
  <!-- id: keep-fact | created: 2026-01-03 | last_used: 2026-01-03 | uses: 1 | tier: active -->
"""


class TestArchiveFact(unittest.TestCase):
    def _setup(self, cont=CONT):
        root = tempfile.mkdtemp()
        os.makedirs(os.path.join(root, "memory", "archive"))
        with open(os.path.join(root, "memory", "continuity.md"), "w") as f:
            f.write(cont)
        with open(os.path.join(root, "memory", "archive", "INDEX.md"), "w") as f:
            f.write("# Archive INDEX\n")
        with open(os.path.join(root, "memory", "archive", "2026-Q1.md"), "w") as f:
            f.write("# 2026-Q1\n")
        return root

    def _read(self, root, *parts):
        with open(os.path.join(root, "memory", *parts)) as f:
            return f.read()

    def test_dry_run_changes_nothing(self):
        root = self._setup()
        before = self._read(root, "continuity.md")
        code, msg = archive_fact.archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", None, True)
        self.assertEqual(code, 0)
        self.assertIn("DRY-RUN", msg)
        self.assertEqual(self._read(root, "continuity.md"), before)
        self.assertEqual(self._read(root, "archive", "2026-Q1.md"), "# 2026-Q1\n")

    def test_real_move_removes_from_continuity_and_appends(self):
        root = self._setup()
        code, msg = archive_fact.archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", None, False)
        self.assertEqual(code, 0)
        cont = self._read(root, "continuity.md")
        self.assertNotIn("alpha-fact", cont)            # gone from continuity
        self.assertIn("beta-fact", cont)                # neighbour untouched
        self.assertIn("keep-fact", cont)
        arch = self._read(root, "archive", "2026-Q1.md")
        self.assertIn("alpha-fact", arch)
        self.assertIn("More alpha detail", arch)        # multi-line block captured whole
        self.assertIn("archived via archive-fact (faded)", arch)
        self.assertIn("- alpha-fact —", self._read(root, "archive", "INDEX.md"))

    def test_multi_id_move(self):
        root = self._setup()
        code, _ = archive_fact.archive_facts(root, ["alpha-fact", "beta-fact"], "faded", "2026-Q1", None, False)
        self.assertEqual(code, 0)
        cont = self._read(root, "continuity.md")
        self.assertNotIn("alpha-fact", cont)
        self.assertNotIn("beta-fact", cont)
        self.assertIn("keep-fact", cont)

    def test_missing_id_refused_atomic(self):
        root = self._setup()
        before = self._read(root, "continuity.md")
        # one good, one missing → all-or-nothing: refuse, change nothing
        code, msg = archive_fact.archive_facts(root, ["alpha-fact", "nope-fact"], "faded", "2026-Q1", None, False)
        self.assertEqual(code, 1)
        self.assertIn("no footer", msg)
        self.assertEqual(self._read(root, "continuity.md"), before)

    def test_already_archived_refused(self):
        root = self._setup()
        archive_fact.archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", None, False)
        code, msg = archive_fact.archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", None, False)
        self.assertEqual(code, 1)
        self.assertIn("already in the archive", msg)

    def test_note_override_single_id(self):
        root = self._setup()
        archive_fact.archive_facts(root, ["alpha-fact"], "faded", "2026-Q1", "custom note", False)
        self.assertIn("- alpha-fact — custom note — faded", self._read(root, "archive", "INDEX.md"))

    def test_derive_quarter(self):
        import datetime
        self.assertEqual(archive_fact.derive_quarter(datetime.date(2026, 6, 28)), "2026-Q2")
        self.assertEqual(archive_fact.derive_quarter(datetime.date(2026, 1, 1)), "2026-Q1")
        self.assertEqual(archive_fact.derive_quarter(datetime.date(2026, 12, 31)), "2026-Q4")


if __name__ == "__main__":
    unittest.main()
