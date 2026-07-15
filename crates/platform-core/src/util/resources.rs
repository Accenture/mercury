//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! The `resources/` folder convention — the Rust analog of the Java classpath.
//!
//! Rust has no classpath, but the *convention* is preserved: a `classpath:/x.yml`
//! path searches an **ordered list of resource roots**, first hit wins. The
//! default root is `./resources` (relative to the working directory). Tests and
//! applications may prepend/append additional roots — prepending mirrors Java's
//! test-resources-shadow-main-resources behavior.

use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};

fn roots() -> &'static RwLock<Vec<PathBuf>> {
    static ROOTS: OnceLock<RwLock<Vec<PathBuf>>> = OnceLock::new();
    ROOTS.get_or_init(|| RwLock::new(vec![PathBuf::from("resources")]))
}

/// Prepend a resource root (highest precedence — shadows later roots).
/// Idempotent: an already-registered root is moved to the front, not duplicated.
pub fn prepend_resource_root<P: AsRef<Path>>(root: P) {
    let p = root.as_ref().to_path_buf();
    let mut guard = roots().write().expect("resource roots poisoned");
    guard.retain(|existing| existing != &p);
    guard.insert(0, p);
}

/// Append a resource root (lowest precedence). Idempotent.
pub fn append_resource_root<P: AsRef<Path>>(root: P) {
    let p = root.as_ref().to_path_buf();
    let mut guard = roots().write().expect("resource roots poisoned");
    if !guard.contains(&p) {
        guard.push(p);
    }
}

/// Current resource roots, in precedence order.
pub fn resource_roots() -> Vec<PathBuf> {
    roots().read().expect("resource roots poisoned").clone()
}

/// Resolve a classpath-style resource (path *without* the `classpath:` prefix,
/// leading `/` tolerated) against the resource roots. Returns the first
/// existing file.
pub fn resolve_classpath(resource: &str) -> Option<PathBuf> {
    let rel = resource.trim_start_matches('/');
    for root in resource_roots() {
        let candidate = root.join(rel);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
