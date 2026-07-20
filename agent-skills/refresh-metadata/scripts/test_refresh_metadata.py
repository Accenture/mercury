import importlib.util
import os
import re
import sys
import tempfile
import unittest
from pathlib import Path

script_path = Path(__file__).parent / "refresh-metadata.py"
spec = importlib.util.spec_from_file_location("refresh_metadata", str(script_path))
rm = importlib.util.module_from_spec(spec)
sys.modules["refresh_metadata"] = rm
spec.loader.exec_module(rm)

CONT = """# Continuity

## Open Threads
- [x] promoted
  <!-- id: promoted-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
- [x] aged
  <!-- id: aged-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
- [x] fresh
  <!-- id: fresh-fact | created: 2026-06-06 | last_used: 2026-06-06 | uses: 1 | tier: working -->
- [ ] open work
  <!-- id: open-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
- [x] core thing
  <!-- id: core-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 5 | tier: core -->
- [x] retired
  <!-- id: super-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: superseded | superseded-by: promoted-fact -->
- [x] legacy never referenced
  <!-- id: legacy-fact | created: 2026-01-01 | last_used: 2026-01-01 | uses: 1 | tier: working -->
"""

POLICY = "## Lifecycle windows\n- working_window: 3\n- active_window: 2\n- archive_window: 4\n"

# 6 sessions; refs place each fact's last use at a known index.
SESSIONS = {
    "2026-06-01-000000": ["aged-fact"],
    "2026-06-02-000000": [],
    "2026-06-03-000000": [],
    "2026-06-04-000000": [],
    "2026-06-05-000000": ["promoted-fact"],
    "2026-06-06-000000": ["promoted-fact", "fresh-fact", "open-fact"],
}


class TestRefreshMetadata(unittest.TestCase):
    def _setup(self):
        root = tempfile.mkdtemp()
        os.makedirs(os.path.join(root, "memory", "sessions"))
        with open(os.path.join(root, "memory", "continuity.md"), "w") as f:
            f.write(CONT)
        with open(os.path.join(root, "memory", "decay-policy.md"), "w") as f:
            f.write(POLICY)
        for stem, ids in SESSIONS.items():
            body = "# Session\n\n## Memory References\n" + "".join(f"- Referenced: {i}\n" for i in ids)
            with open(os.path.join(root, "memory", "sessions", stem + ".md"), "w") as f:
                f.write(body)
        return root

    def _tier(self, root, fid):
        text = open(os.path.join(root, "memory", "continuity.md")).read()
        m = re.search(r"<!--\s*id:\s*" + re.escape(fid) + r"\s*\|([^\n]*?)-->", text)
        fields = {}
        for p in m.group(1).split("|"):
            if ":" in p:
                k, _, v = p.partition(":")
                fields[k.strip()] = v.strip()
        return fields

    def test_dry_run_changes_nothing(self):
        root = self._setup()
        before = open(os.path.join(root, "memory", "continuity.md")).read()
        code, msg = rm.refresh(root, True)
        self.assertEqual(code, 0)
        self.assertIn("DRY-RUN", msg)
        self.assertEqual(open(os.path.join(root, "memory", "continuity.md")).read(), before)

    def test_retier_and_recount(self):
        root = self._setup()
        rm.refresh(root, False)
        self.assertEqual(self._tier(root, "promoted-fact")["tier"], "active")    # working→active
        self.assertEqual(self._tier(root, "promoted-fact")["uses"], "2")          # uses recount
        self.assertEqual(self._tier(root, "aged-fact")["tier"], "archive-candidate")  # sslu>aw
        self.assertEqual(self._tier(root, "fresh-fact")["tier"], "working")       # recent + uses≤1 stays working
        self.assertEqual(self._tier(root, "open-fact")["tier"], "working")        # pinned: tier label left as-is
        self.assertEqual(self._tier(root, "open-fact")["last_used"], "2026-06-06")  # …but factual fields still refresh

    def test_core_and_superseded_untouched(self):
        root = self._setup()
        rm.refresh(root, False)
        self.assertEqual(self._tier(root, "core-fact")["tier"], "core")
        self.assertEqual(self._tier(root, "core-fact")["uses"], "5")              # not recounted
        self.assertEqual(self._tier(root, "super-fact")["tier"], "superseded")

    def test_never_referenced_preserved(self):
        root = self._setup()
        rm.refresh(root, False)
        self.assertEqual(self._tier(root, "legacy-fact")["tier"], "working")      # no refs → preserved

    def test_idempotent(self):
        root = self._setup()
        rm.refresh(root, False)
        code, msg = rm.refresh(root, False)
        self.assertIn("nothing to refresh", msg)                                  # second run is a no-op


if __name__ == "__main__":
    unittest.main()
