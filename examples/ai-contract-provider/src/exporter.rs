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

//! Writes the offline mercury-platform Agent Skill (Java `OfflineSkillWriter`
//! analog): exactly the rendered snapshot the REST endpoints serve, plus
//! manifest.json written LAST as the completion marker (a directory without
//! manifest.json is an incomplete export). Safety properties: the target
//! directory must not pre-exist, every file is created with create_new (never
//! overwrites), all content is verified by re-reading before the manifest is
//! written, and a failed export cleans up the partial directory - but never one
//! that already carries a manifest.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use platform_core::AppError;
use serde_json::json;

use super::snapshot::{SkillSnapshot, MANIFEST};

pub const SKILL_DIRECTORY: &str = "mercury-platform";

pub fn export(directory: Option<&str>) -> Result<serde_json::Value, AppError> {
    let Some(directory) = directory.map(str::trim).filter(|d| !d.is_empty()) else {
        return Err(AppError::new(400, "Missing export directory"));
    };
    let root = PathBuf::from(directory);
    if !root.is_dir() || root.is_symlink() {
        return Err(AppError::new(
            400,
            "Export root must be an existing directory",
        ));
    }
    let target = root.join(SKILL_DIRECTORY);
    if target.exists() {
        return Err(AppError::new(
            409,
            format!(
                "Snapshot already exists - remove {SKILL_DIRECTORY} from the export root first"
            ),
        ));
    }
    let snapshot = SkillSnapshot::get_instance();
    if let Err(e) = write_snapshot(&target, snapshot) {
        cleanup_incomplete(&target);
        return Err(e);
    }
    Ok(json!({
        "skill_directory": target.display().to_string(),
        "files": snapshot.files().len() + 1,
        "snapshot_sha256": snapshot.manifest()["snapshot_sha256"],
    }))
}

fn write_snapshot(target: &Path, snapshot: &SkillSnapshot) -> Result<(), AppError> {
    fs::create_dir(target).map_err(|e| io_error("create export directory", e))?;
    for (path, content) in snapshot.files() {
        let destination = target.join(path);
        // snapshot paths are validated at build time; keep the containment check anyway
        if path.starts_with('/') || path.contains("..") {
            return Err(AppError::new(500, format!("Invalid snapshot path: {path}")));
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|e| io_error("create parent directory", e))?;
        }
        write_new(&destination, content)?;
    }
    for (path, content) in snapshot.files() {
        let written = fs::read(target.join(path)).map_err(|e| io_error("re-read for verify", e))?;
        if &written != content {
            return Err(AppError::new(
                500,
                format!("Verification failed for {path}"),
            ));
        }
    }
    let manifest = serde_json::to_string_pretty(snapshot.manifest())
        .map_err(|e| AppError::new(500, format!("Unable to serialize manifest - {e}")))?;
    write_new(&target.join(MANIFEST), manifest.as_bytes())
}

fn write_new(destination: &Path, content: &[u8]) -> Result<(), AppError> {
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .map_err(|e| io_error("create snapshot file", e))?;
    out.write_all(content)
        .map_err(|e| io_error("write snapshot file", e))
}

fn io_error(action: &str, e: std::io::Error) -> AppError {
    AppError::new(
        500,
        format!("Skill export failed - unable to {action}: {e}"),
    )
}

fn cleanup_incomplete(target: &Path) {
    if !target.is_dir() || target.join(MANIFEST).exists() {
        return;
    }
    if let Err(e) = fs::remove_dir_all(target) {
        log::warn!(
            "Unable to clean up incomplete export at {} - {e}",
            target.display()
        );
    }
}
