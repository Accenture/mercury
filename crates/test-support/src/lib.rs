//! Test support shared by Mercury's test suites (dev-only, never published).
//!
//! A test binary runs its tests in parallel and has no hook after the last one, so a folder that
//! several tests share cannot be removed by any one of them. One `atexit` hook, which the test
//! harness reaches when it exits whether the tests passed or failed, does the cleanup instead:
//!
//! - [`temp_root`] gives the process one temporary folder, `<system temp>/mercury-test-<pid>`,
//!   removed at exit. A test points what it writes there: the elastic queue's
//!   `transient.data.store`, the graph temporary folder, a generated `rest.yaml`.
//! - [`run_at_exit`] runs a cleanup the code under test owns, such as the elastic queue's
//!   `shutdown_cleanup`, which only the application lifecycle's graceful exit would otherwise run.
//!
//! A test that owns its files outright removes them itself, with a drop guard.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, Once, OnceLock};

extern "C" {
    fn atexit(callback: extern "C" fn()) -> std::os::raw::c_int;
}

static ROOT: OnceLock<PathBuf> = OnceLock::new();
static CLEANUPS: Mutex<Vec<fn()>> = Mutex::new(Vec::new());
static HOOK: Once = Once::new();

/// Runs the registered cleanups, then removes the temporary folder.
extern "C" fn exit_hook() {
    let cleanups = CLEANUPS.lock().map(|c| c.clone()).unwrap_or_default();
    for cleanup in cleanups {
        cleanup();
    }
    if let Some(root) = ROOT.get() {
        let _ = std::fs::remove_dir_all(root);
    }
}

fn register_exit_hook() {
    // SAFETY: atexit registers a function that takes no arguments; the C runtime calls it once,
    // when the process exits
    HOOK.call_once(|| unsafe {
        atexit(exit_hook);
    });
}

/// This process's temporary folder, created on first use and removed when the process exits.
pub fn temp_root() -> &'static Path {
    ROOT.get_or_init(|| {
        let root = std::env::temp_dir().join(format!("mercury-test-{}", std::process::id()));
        // a folder left by an earlier process with the same id is debris
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("a temporary folder for the test process");
        register_exit_hook();
        root
    })
}

/// A path under this process's temporary folder, for a file or folder a test writes.
pub fn temp_path(name: &str) -> PathBuf {
    temp_root().join(name)
}

/// Run `cleanup` when the process exits, before the temporary folder is removed. Every test that
/// needs the cleanup may register it: a cleanup registered again is run once.
pub fn run_at_exit(cleanup: fn()) {
    register_exit_hook();
    let mut cleanups = CLEANUPS.lock().unwrap_or_else(|e| e.into_inner());
    if !cleanups.iter().any(|f| *f as usize == cleanup as usize) {
        cleanups.push(cleanup);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nothing() {}

    #[test]
    fn one_root_per_process() {
        let root = temp_root();
        assert!(root.is_dir(), "the root is created on first use");
        assert_eq!(root, temp_root(), "every call returns the same root");
        assert_eq!(root.join("store"), temp_path("store"));
    }

    #[test]
    fn a_cleanup_registered_twice_runs_once() {
        run_at_exit(nothing);
        run_at_exit(nothing);
        let count = CLEANUPS
            .lock()
            .expect("cleanups")
            .iter()
            .filter(|f| **f as usize == nothing as fn() as usize)
            .count();
        assert_eq!(1, count);
    }
}
