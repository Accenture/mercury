import unittest
import importlib.util
import os
import tempfile
from pathlib import Path
import sys

# Load memory-lint.py dynamically since it has a hyphen in the name
script_path = Path(__file__).parent / "memory-lint.py"
spec = importlib.util.spec_from_file_location("memory_lint", str(script_path))
memory_lint = importlib.util.module_from_spec(spec)
sys.modules["memory_lint"] = memory_lint
spec.loader.exec_module(memory_lint)

class TestMemoryLint(unittest.TestCase):
    def test_pinned_open_threads_flat(self):
        text = """
- [ ] Parent task
  <!-- id: t1 -->
- [x] Done task
  <!-- id: t2 -->
"""
        self.assertEqual(memory_lint.pinned_open_threads(text), {"t1"})

    def test_pinned_open_threads_nested(self):
        # Nested list inside an open thread
        text = """
- [ ] Parent task
  - Subtask 1
  - Subtask 2
  <!-- id: t3 -->
"""
        self.assertEqual(memory_lint.pinned_open_threads(text), {"t3"})

    def test_pinned_open_threads_nested_open(self):
        text = """
- [ ] Parent task
  - [ ] Nested open
    <!-- id: t4 -->
"""
        self.assertEqual(memory_lint.pinned_open_threads(text), {"t4"})

    def test_pinned_open_threads_sibling_reset(self):
        text = """
- [ ] Parent task
  <!-- id: t5 -->
- Regular bullet that should reset
  <!-- id: t6 -->
"""
        self.assertEqual(memory_lint.pinned_open_threads(text), {"t5"})

    def test_pinned_open_threads_mixed(self):
        text = """
- [ ] Open task 1
  - Subtask
  <!-- id: mix-1 -->
- [x] Done task
  <!-- id: mix-2 -->
- [ ] Open task 2
  <!-- id: mix-3 -->
- Regular sub-bullet
  <!-- id: mix-4 -->
"""
        self.assertEqual(memory_lint.pinned_open_threads(text), {"mix-1", "mix-3"})

    def test_memref_ids_ignores_prose_and_review_summary(self):
        # The ot-review-step6-prose livelock: a fact named only in prose / a
        # '## Memory Review' summary is NOT a use — only '## Memory References' counts.
        text = """# Session
## What happened
Archiving `foo-fact` because it is overdue.
## Memory Review (2026-06-19)
- Archived: 1 (`foo-fact` -> archive, faded)
- Tier changes: foo-fact archive-candidate->archived
## Memory References
- Created: bar-fact
- Referenced: baz-fact
"""
        ids = memory_lint.memref_ids(text)
        self.assertIn("bar-fact", ids)
        self.assertIn("baz-fact", ids)
        self.assertNotIn("foo-fact", ids)  # prose / review-summary mention is not a reference

    def test_memref_ids_bounded_by_next_heading(self):
        text = """## Memory References
- Referenced: in-block-id
## Next Section
- not-a-ref-id mentioned here
"""
        ids = memory_lint.memref_ids(text)
        self.assertIn("in-block-id", ids)
        self.assertNotIn("not-a-ref-id", ids)


class TestDanglingCrossFile(unittest.TestCase):
    # Regression for the dangling-link false positive: a supersession target whose
    # footer lives in another memory/*.md (e.g. vision.md) must resolve, not warn.
    # The bug was in load_repo (it only pooled continuity + archive footers), so this
    # exercises load_repo end-to-end against a temp memory/ layer, not check_dangling alone.
    @staticmethod
    def _write(path, text):
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)

    def test_supersession_target_in_vision_is_not_dangling(self):
        with tempfile.TemporaryDirectory() as root:
            mem = os.path.join(root, "memory")
            self._write(os.path.join(mem, "continuity.md"), """# Continuity
## Open Threads
- [x] Old vision retired
  <!-- id: old-fact | created: 2026-06-19 | last_used: 2026-06-19 | uses: 1 | tier: superseded | superseded-by: new-fact -->
- [x] Orphaned link
  <!-- id: orphan-fact | created: 2026-06-19 | last_used: 2026-06-19 | uses: 1 | tier: superseded | superseded-by: ghost-fact -->
""")
            self._write(os.path.join(mem, "vision.md"), """# Vision
<!-- id: new-fact | created: 2026-06-19 | last_used: 2026-06-19 | uses: 1 | tier: core -->
""")
            os.makedirs(os.path.join(mem, "sessions"), exist_ok=True)

            cont, pinned, arch, extra, sessions, refs = memory_lint.load_repo(root)
            # the vision fact is available for link resolution but NOT counted as a fact
            self.assertIn("new-fact", extra)
            self.assertNotIn("new-fact", cont)
            self.assertNotIn("new-fact", arch)

            warns = memory_lint.check_dangling({**cont, **arch, **extra})
            # superseded-by a vision.md fact resolves -> no dangling
            self.assertFalse(any("old-fact" in w for w in warns), warns)
            # a genuinely missing target still dangles (negative control)
            self.assertTrue(any("orphan-fact" in w and "ghost-fact" in w for w in warns), warns)


class TestSessionFilenames(unittest.TestCase):
    def test_date_only_filenames_flagged(self):
        sessions = [
            "/repo/memory/sessions/2026-06-12.md",
            "/repo/memory/sessions/2026-06-23.md",
        ]
        warns = memory_lint.check_session_filenames(sessions)
        self.assertEqual(len(warns), 2)
        self.assertTrue(all("[date-only-session]" in w for w in warns))

    def test_timestamped_filenames_not_flagged(self):
        sessions = [
            "/repo/memory/sessions/2026-06-23-153401.md",
            "/repo/memory/sessions/2026-06-13-011149.md",
        ]
        warns = memory_lint.check_session_filenames(sessions)
        self.assertEqual(len(warns), 0)

    def test_mixed(self):
        sessions = [
            "/repo/memory/sessions/2026-06-12.md",
            "/repo/memory/sessions/2026-06-23-153401.md",
        ]
        warns = memory_lint.check_session_filenames(sessions)
        self.assertEqual(len(warns), 1)
        self.assertIn("2026-06-12.md", warns[0])


class TestVersionManifest(unittest.TestCase):
    # Regression for the empty-version.md bug: a present-but-empty/malformed
    # .agent/version.md breaks Mode B upgrade detection and must be flagged; a
    # MISSING file is the valid pre-versioning baseline and must NOT be flagged.
    @staticmethod
    def _setup(root, version_md=None):
        os.makedirs(os.path.join(root, "memory"), exist_ok=True)
        with open(os.path.join(root, "memory", "continuity.md"), "w", encoding="utf-8") as f:
            f.write("# c\n")
        if version_md is not None:
            os.makedirs(os.path.join(root, ".agent"), exist_ok=True)
            with open(os.path.join(root, ".agent", "version.md"), "w", encoding="utf-8") as f:
                f.write(version_md)

    def test_empty_version_md_flagged(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, version_md="")
            errs = memory_lint.check_version_manifest(root)
            self.assertEqual(len(errs), 1)
            self.assertIn("[version-manifest]", errs[0])

    def test_malformed_version_md_flagged(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, version_md="# manifest\n(no version line here)\n")
            self.assertEqual(len(memory_lint.check_version_manifest(root)), 1)

    def test_valid_version_md_ok(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, version_md="- **version:**       4.20.3\n")
            self.assertEqual(memory_lint.check_version_manifest(root), [])

    def test_missing_version_md_ok(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, version_md=None)
            self.assertEqual(memory_lint.check_version_manifest(root), [])


class TestConflictMarkers(unittest.TestCase):
    # A leftover VCS merge-conflict marker corrupts shared memory and must be an ERROR.
    # A bare `=======` line is a valid Markdown setext heading underline and must NOT trip.
    @staticmethod
    def _setup(root, files):
        os.makedirs(os.path.join(root, "memory"), exist_ok=True)
        for rel, body in files.items():
            full = os.path.join(root, "memory", rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            with open(full, "w", encoding="utf-8") as f:
                f.write(body)

    def test_git_markers_flagged(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {
                "continuity.md": "# c\n<<<<<<< HEAD\nmine\n=======\ntheirs\n>>>>>>> branch\n",
            })
            errs = memory_lint.check_conflict_markers(root)
            self.assertEqual(len(errs), 1)
            self.assertIn("[conflict-marker]", errs[0])
            self.assertIn("memory/continuity.md:2", errs[0])

    def test_diff3_marker_flagged(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {"continuity.md": "# c\n||||||| base\n"})
            self.assertEqual(len(memory_lint.check_conflict_markers(root)), 1)

    def test_setext_heading_underline_not_flagged(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {"continuity.md": "Title\n=======\n\nbody\n"})
            self.assertEqual(memory_lint.check_conflict_markers(root), [])

    def test_clean_memory_ok(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {
                "continuity.md": "# c\nall good\n",
                "sessions/2026-06-27-120000.md": "# Session\nfine\n",
            })
            self.assertEqual(memory_lint.check_conflict_markers(root), [])

    def test_one_report_per_file(self):
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {"continuity.md": "<<<<<<< a\n>>>>>>> b\n<<<<<<< c\n"})
            self.assertEqual(len(memory_lint.check_conflict_markers(root)), 1)

    def test_session_log_marker_ignored(self):
        # sessions/ legitimately QUOTES conflict markers (documenting a diff/terminal output);
        # check 7 scans only top-level memory/*.md, so a marker there must NOT be flagged.
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {
                "continuity.md": "# c\nclean\n",
                "sessions/2026-06-27-120000.md": "# Session\n```\n<<<<<<< HEAD\nx\n=======\ny\n>>>>>>> b\n```\n",
            })
            self.assertEqual(memory_lint.check_conflict_markers(root), [])

    def test_archive_marker_ignored(self):
        # archive/ is cold append-storage, likewise excluded from check 7.
        with tempfile.TemporaryDirectory() as root:
            self._setup(root, {
                "continuity.md": "# c\nclean\n",
                "archive/2026-Q2.md": "<<<<<<< HEAD\nx\n>>>>>>> b\n",
            })
            self.assertEqual(memory_lint.check_conflict_markers(root), [])


class TestContinuityHealth(unittest.TestCase):
    # (8) advisory cadence/size triggers (v4.24.0): catch a layer whose review never fired,
    # or a continuity.md grown past the fact/line budget. All advisory (WARN).
    @staticmethod
    def _facts(n):
        return {f"fact-{i}": {} for i in range(n)}

    def test_review_overdue_flagged(self):
        cont_text = "- **last_review:** 2026-06-20 | through 2026-06-20-120000\n"
        sessions = [f"2026-06-2{d}-120000.md" for d in range(1, 8)]  # 7 strictly after the stamp
        w = memory_lint.check_continuity_health(self._facts(1), sessions, cont_text, 10, 5, 30, 600)
        self.assertEqual(len(w), 1)
        self.assertIn("[review-overdue]", w[0])
        self.assertIn("7 session(s) since last review >= review_every 5", w[0])

    def test_review_recent_ok(self):
        cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n"
        sessions = ["2026-06-27-120000.md"]  # 0 strictly after
        self.assertEqual(
            memory_lint.check_continuity_health(self._facts(1), sessions, cont_text, 10, 10, 30, 600), []
        )

    def test_never_reviewed_counts_all_sessions(self):
        cont_text = "# Continuity\n(no last_review recorded yet)\n"
        sessions = [f"2026-06-2{d}-120000.md" for d in range(1, 5)]  # 4 sessions
        w = memory_lint.check_continuity_health(self._facts(1), sessions, cont_text, 10, 3, 30, 600)
        self.assertTrue(any("[review-overdue]" in x and "4 session(s)" in x for x in w))

    def test_sessions_since_review_prefers_through_token(self):
        cont_text = "- **last_review:** 2026-06-20 | through 2026-06-25-120000\n"
        sessions = ["2026-06-24-120000.md", "2026-06-26-120000.md", "2026-06-27-120000.md"]
        self.assertEqual(memory_lint.sessions_since_review(sessions, cont_text), 2)

    def test_fact_bloat_flagged(self):
        cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n"
        sessions = ["2026-06-27-120000.md"]
        w = memory_lint.check_continuity_health(self._facts(31), sessions, cont_text, 10, 10, 30, 600)
        self.assertEqual(len(w), 1)
        self.assertIn("31 decay-eligible facts > continuity_max_facts 30", w[0])

    def test_fact_bloat_excludes_core_and_pinned(self):
        # tier:core and pinned open threads must NOT count toward the cap — they can never be
        # archived, so counting them produces permanent noise (field report: mercury-composable).
        cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n"
        sessions = ["2026-06-27-120000.md"]
        cont = {
            **{f"core-{i}": {"tier": "core"} for i in range(14)},     # 14 core — never decay
            **{f"pinned-{i}": {"tier": "working"} for i in range(11)}, # 11 pinned open threads
            **{f"working-{i}": {"tier": "working"} for i in range(16)}, # 16 decay-eligible
        }
        pinned = {f"pinned-{i}" for i in range(11)}
        # 41 total facts, only 16 decay-eligible — should be well under the cap of 30
        w = memory_lint.check_continuity_health(cont, sessions, cont_text, 10, 10, 30, 600, pinned=pinned)
        self.assertEqual(w, [], "core + pinned facts must not trigger continuity-bloat")

    def test_line_bloat_flagged(self):
        # archivable > 0 → a review CAN lean it down → the actionable "review is due" message.
        cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n"
        sessions = ["2026-06-27-120000.md"]
        w = memory_lint.check_continuity_health(self._facts(1), sessions, cont_text, 700, 10, 30, 600, archivable=3)
        self.assertEqual(len(w), 1)
        self.assertIn("continuity.md 700 lines > continuity_max_lines 600", w[0])
        self.assertIn("a review is due to lean it down", w[0])

    def test_line_bloat_active_verbosity_when_nothing_archivable(self):
        # archivable == 0 → a review has no honest lever (nothing faded/superseded). The message must
        # NOT claim a review will fix it (that nudges toward premature archival of active facts —
        # REVIEW.md's costliest error); it names the real lever instead (v4.28.3, mercury-composable).
        cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n"
        sessions = ["2026-06-27-120000.md"]
        w = memory_lint.check_continuity_health(self._facts(1), sessions, cont_text, 700, 10, 30, 600, archivable=0)
        self.assertEqual(len(w), 1)
        self.assertIn("continuity.md 700 lines > continuity_max_lines 600", w[0])
        self.assertIn("nothing is archivable yet", w[0])
        self.assertIn("Condense shipped decisions", w[0])
        self.assertNotIn("a review is due to lean it down", w[0])

    def test_healthy_ok(self):
        cont_text = "- **last_review:** 2026-06-27 | through 2026-06-27-120000\n"
        sessions = ["2026-06-27-120000.md"]
        self.assertEqual(
            memory_lint.check_continuity_health(self._facts(24), sessions, cont_text, 490, 10, 30, 600), []
        )


class TestStaleMetadata(unittest.TestCase):
    # (9) flag stored tier != tier recomputed from references (review steps 2–3 skipped).
    STEMS = ["2026-06-01-000000", "2026-06-02-000000", "2026-06-03-000000"]

    def test_flags_tier_drift(self):
        cont = {"foo-fact": {"tier": "working", "created": "2026-01-01"}}
        refs = [{"foo-fact"}, {"foo-fact"}, set()]  # uses 2 (bypasses working rule), sslu 1 → active
        w = memory_lint.check_stale_metadata(cont, set(), refs, self.STEMS, 3, 2, 4)
        self.assertEqual(len(w), 1)
        self.assertIn("[stale-metadata]", w[0])
        self.assertIn("should be 'active'", w[0])

    def test_matching_tier_not_flagged(self):
        cont = {"a-fact": {"tier": "active", "created": "2026-01-01"}}
        refs = [{"a-fact"}, {"a-fact"}, set()]
        self.assertEqual(memory_lint.check_stale_metadata(cont, set(), refs, self.STEMS, 3, 2, 4), [])

    def test_core_and_superseded_exempt(self):
        cont = {
            "c-fact": {"tier": "core", "created": "2026-01-01"},
            "s-fact": {"tier": "superseded", "created": "2026-01-01", "superseded-by": "a-fact"},
        }
        refs = [{"c-fact"}, {"s-fact"}, set()]
        self.assertEqual(memory_lint.check_stale_metadata(cont, set(), refs, self.STEMS, 3, 2, 4), [])

    def test_never_referenced_not_flagged(self):
        cont = {"legacy-fact": {"tier": "working", "created": "2026-01-01"}}
        refs = [set(), set(), set()]  # sslu None → can't recompute → no flag
        self.assertEqual(memory_lint.check_stale_metadata(cont, set(), refs, self.STEMS, 3, 2, 4), [])

    def test_pinned_thread_tier_not_flagged(self):
        # v4.26.1 refinement: a pinned `- [ ]` thread never decays; the tool doesn't opine on its
        # tier label, so a 'working'-tagged pinned thread is NOT drift (would be 'active' if unpinned).
        cont = {"open-fact": {"tier": "working", "created": "2026-01-01"}}
        refs = [{"open-fact"}, {"open-fact"}, set()]
        self.assertEqual(memory_lint.check_stale_metadata(cont, {"open-fact"}, refs, self.STEMS, 3, 2, 4), [])


if __name__ == "__main__":
    unittest.main()
