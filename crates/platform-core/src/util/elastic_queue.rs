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

//! Rust port of the Java `ElasticQueue` + `FileElasticStore`
//! (`org.platformlambda.core.util`) — the **reactive back-pressure overflow
//! buffer** behind every route's manager (`ServiceQueue` analog).
//!
//! A per-route two-tier FIFO: the first [`MEMORY_BUFFER`] events of a burst stay
//! in memory, then the overflow spills to fixed-size append-only **segment**
//! files. Record format is portable and kept **byte-identical to the Java file
//! store**: `[4-byte big-endian length][payload]`. A segment is sealed once it
//! passes the size threshold (`elastic.queue.segment.size.bytes`, default 16 MB,
//! min 512); a sealed, fully-consumed segment is **deleted immediately** — O(1)
//! reclamation, no compaction, no cleaner thread (the whole point vs. the legacy
//! Berkeley DB store, which is **not ported** — maintainer decision 2026-07-15;
//! with one store the Java `ElasticStore` strategy facade collapses into this
//! single type).
//!
//! Threading: **single-threaded per route** (driven by the route's manager
//! task). The buffer is transient — not durable across restart; there is no
//! fsync. Base directory: `transient.data.store` (default `/tmp/reactive`,
//! kept verbatim — D9) + `<application name>-<process origin>` (Platform
//! identity) unless `running.in.cloud=true`.
//!
//! **Housekeeping** (Java `ensureBaseDir` + periodic keep-alive + shutdown
//! hook, wired by the lifecycle): [`start_housekeeping`] refreshes the RUNNING
//! liveness marker every 20 s and runs [`scan_expired_stores`] once (removes
//! sibling holding areas whose marker is stale for > 1 h, or unknown dirs
//! holding segment files); [`shutdown_cleanup`] purges segments + the marker on
//! graceful exit (Java uses a JVM shutdown hook; signal handling is a later
//! increment — call it from your main).

use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use crate::function::AppError;
use crate::platform::Platform;
use crate::util::app_config_reader::AppConfigReader;

/// First this many events of a burst are held in memory before spilling to disk
/// (Java `ElasticStore.MEMORY_BUFFER`).
pub const MEMORY_BUFFER: u64 = 20;

const LENGTH_PREFIX: usize = 4;
const SEGMENT_PREFIX: &str = "eq-";
const SEGMENT_SUFFIX: &str = ".dat";
const DEFAULT_SEGMENT_BYTES: u64 = 16 * 1024 * 1024;
const MIN_SEGMENT_BYTES: u64 = 512;
const SEGMENT_SIZE_CONFIG: &str = "elastic.queue.segment.size.bytes";
const RUNNING: &str = "RUNNING";

static BASE_DIR: OnceLock<PathBuf> = OnceLock::new();

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(20);
const EXPIRY: Duration = Duration::from_secs(60 * 60); // one hour (Java ONE_HOUR)

/// The holding area for segment files, created once per process
/// (Java `FileElasticStore.ensureBaseDir`).
pub fn base_dir() -> &'static Path {
    BASE_DIR.get_or_init(|| {
        let config = AppConfigReader::get_instance();
        let tmp_root =
            PathBuf::from(config.get_property_or("transient.data.store", "/tmp/reactive"));
        let running_in_cloud = config.get_property_or("running.in.cloud", "false") == "true";
        let dir = if running_in_cloud {
            tmp_root
        } else {
            tmp_root.join(format!("{}-{}", Platform::name(), Platform::origin()))
        };
        if !dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&dir) {
                log::error!("Unable to create {} - {e}", dir.display());
            } else {
                log::info!("{} created", dir.display());
            }
        }
        write_running_marker(&dir);
        purge_leftover_segments(&dir, None);
        log::info!("Elastic file store ready ({})", dir.display());
        dir
    })
}

/// Refresh the RUNNING liveness marker (Java `keepAlive`).
fn write_running_marker(dir: &Path) {
    let epoch_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_default();
    let _ = std::fs::write(dir.join(RUNNING), epoch_ms);
}

/// Start the housekeeping the lifecycle owns (idempotent — Java wires this in
/// `ensureBaseDir` + a Vert.x periodic timer): refresh the RUNNING marker every
/// 20 s so siblings can tell this holding area is alive, and scan once for
/// expired/unknown holding areas. Must be called within a Tokio runtime
/// (`AppStarter::run` does — its "essential services" phase).
pub fn start_housekeeping() {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        let dir = base_dir().to_path_buf();
        let running_in_cloud =
            AppConfigReader::get_instance().get_property_or("running.in.cloud", "false") == "true";
        if !running_in_cloud {
            scan_expired_stores();
        }
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(KEEP_ALIVE_INTERVAL).await;
                write_running_marker(&dir);
            }
        });
    });
}

/// Remove sibling holding areas left behind by dead processes (Java
/// `scanExpiredStores`/`removeExpiredStore`): a dir whose RUNNING marker went
/// stale (> 1 h) while still holding segment files has expired; a dir holding
/// segment files with **no** marker is unknown debris. Both are removed.
pub fn scan_expired_stores() {
    let current = base_dir();
    let Some(tmp_root) = current.parent() else {
        return;
    };
    let Ok(entries) = std::fs::read_dir(tmp_root) else {
        return;
    };
    for entry in entries.flatten() {
        let folder = entry.path();
        if !folder.is_dir() || folder == current {
            continue;
        }
        let marker = folder.join(RUNNING);
        if marker.exists() {
            let stale = std::fs::metadata(&marker)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|modified| modified.elapsed().ok())
                .is_some_and(|age| age > EXPIRY);
            if stale && has_segment_files(&folder) && std::fs::remove_dir_all(&folder).is_ok() {
                log::info!("Elastic file holding area {} expired", folder.display());
            }
        } else if has_segment_files(&folder) && std::fs::remove_dir_all(&folder).is_ok() {
            log::warn!(
                "Unknown elastic file holding area {} removed",
                folder.display()
            );
        }
    }
}

fn has_segment_files(folder: &Path) -> bool {
    std::fs::read_dir(folder)
        .map(|entries| {
            entries.flatten().any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                e.path().is_file()
                    && name.starts_with(SEGMENT_PREFIX)
                    && name.ends_with(SEGMENT_SUFFIX)
            })
        })
        .unwrap_or(false)
}

/// Graceful-exit cleanup (Java's JVM shutdown hook): purge this process's
/// segment files and remove the RUNNING marker. Call from your main after the
/// lifecycle completes; OS-signal wiring is a later increment.
pub fn shutdown_cleanup() {
    let dir = base_dir();
    purge_leftover_segments(dir, None);
    if let Err(e) = std::fs::remove_file(dir.join(RUNNING)) {
        log::debug!("Unable to delete {RUNNING} marker - {e}");
    }
}

/// Best-effort removal of leftover segment files — all of them, or only those
/// for one sanitized route id (Java `purgeLeftoverSegments`).
fn purge_leftover_segments(dir: &Path, id_prefix: Option<&str>) {
    let prefix = match id_prefix {
        Some(id) => format!("{SEGMENT_PREFIX}{id}-"),
        None => SEGMENT_PREFIX.to_string(),
    };
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && name.ends_with(SEGMENT_SUFFIX) {
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    log::debug!("Unable to delete leftover {name} - {e}");
                }
            }
        }
    }
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// One append-only spill file (Java `FileElasticStore.Segment`): sealed when it
/// passes the size threshold; reclaimed (deleted) as soon as fully consumed.
struct Segment {
    index: u32,
    path: PathBuf,
    file: Option<File>,
    write_pos: u64,
    read_pos: u64,
    records_written: u64,
    records_read: u64,
    sealed: bool,
}

impl Segment {
    fn open(path: PathBuf, index: u32) -> Result<Self, AppError> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| {
                AppError::new(
                    500,
                    format!("Unable to open elastic segment {}: {e}", path.display()),
                )
            })?;
        Ok(Segment {
            index,
            path,
            file: Some(file),
            write_pos: 0,
            read_pos: 0,
            records_written: 0,
            records_read: 0,
            sealed: false,
        })
    }

    /// Reopen lazily when a sealed segment becomes the read head (Java parity:
    /// at most the write tail and the read head hold open files).
    fn handle(&mut self) -> Result<&mut File, AppError> {
        if self.file.is_none() {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&self.path)
                .map_err(|e| {
                    AppError::new(
                        500,
                        format!(
                            "Unable to reopen elastic segment {}: {e}",
                            self.path.display()
                        ),
                    )
                })?;
            self.file = Some(file);
        }
        Ok(self.file.as_mut().expect("segment file just ensured"))
    }

    fn close_quietly(&mut self) {
        self.file = None; // dropping the handle closes it
    }

    fn close_and_delete(&mut self) {
        self.close_quietly();
        if let Err(e) = std::fs::remove_file(&self.path) {
            log::debug!("Unable to delete segment {} - {e}", self.path.display());
        }
    }
}

/// The per-route elastic overflow buffer. **Reserved for system use** — the
/// route manager drives it; do not use directly in application code (Java
/// carries the same warning on `ServiceQueue`).
pub struct ElasticQueue {
    id: String,
    safe_id: String,
    segment_bytes: u64,
    memory: VecDeque<Vec<u8>>,
    segments: VecDeque<Segment>,
    read_counter: u64,
    write_counter: u64,
    empty: bool,
    peeked: Option<Vec<u8>>,
    generation: u32,
}

impl ElasticQueue {
    /// `id` is the service route path.
    pub fn new(id: &str) -> Self {
        let safe_id = sanitize(id);
        let segment_bytes = AppConfigReader::get_instance()
            .get_property_or(SEGMENT_SIZE_CONFIG, &DEFAULT_SEGMENT_BYTES.to_string())
            .parse::<u64>()
            .unwrap_or(DEFAULT_SEGMENT_BYTES)
            .max(MIN_SEGMENT_BYTES);
        base_dir(); // ensure the holding area exists before any spill
        ElasticQueue {
            id: id.to_string(),
            safe_id,
            segment_bytes,
            memory: VecDeque::new(),
            segments: VecDeque::new(),
            read_counter: 0,
            write_counter: 0,
            empty: true,
            peeked: None,
            generation: 0,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn read_counter(&self) -> u64 {
        self.read_counter
    }

    pub fn write_counter(&self) -> u64 {
        self.write_counter
    }

    /// The queue is closed when the counters are reset (Java `isClosed`).
    pub fn is_closed(&self) -> bool {
        self.write_counter == 0
    }

    /// Append one serialized event: memory tier first, then disk spill
    /// (Java `write`). Empty payloads are ignored.
    pub fn write(&mut self, event: &[u8]) -> Result<(), AppError> {
        if event.is_empty() {
            return Ok(());
        }
        if self.write_counter < MEMORY_BUFFER {
            // for highest performance, save to memory for the first few blocks
            self.memory.push_back(event.to_vec());
        } else {
            self.append_to_disk(event)?;
        }
        self.write_counter += 1;
        self.empty = false;
        Ok(())
    }

    /// Look at the next event without consuming it (Java `peek`).
    pub fn peek(&mut self) -> Result<Vec<u8>, AppError> {
        if let Some(held) = &self.peeked {
            return Ok(held.clone());
        }
        let next = self.read()?;
        if !next.is_empty() {
            self.peeked = Some(next.clone());
        }
        Ok(next)
    }

    /// Consume the next event in FIFO order; an empty vec means the queue has
    /// caught up with writes (and the counters were reset — Java `read`).
    pub fn read(&mut self) -> Result<Vec<u8>, AppError> {
        if let Some(held) = self.peeked.take() {
            return Ok(held);
        }
        if self.read_counter >= self.write_counter {
            // catch up with writes and thus nothing to read
            self.close();
            return Ok(Vec::new());
        }
        if self.read_counter < MEMORY_BUFFER {
            if let Some(event) = self.memory.pop_front() {
                self.read_counter += 1;
                return Ok(event);
            }
            return Ok(Vec::new());
        }
        let event = self.read_from_disk()?;
        if !event.is_empty() {
            self.read_counter += 1;
        }
        Ok(event)
    }

    /// Reset the queue once drained (Java `close` → `resetCounter`).
    pub fn close(&mut self) {
        self.peeked = None;
        if !self.empty {
            self.empty = true;
            self.read_counter = 0;
            self.write_counter = 0;
            self.memory.clear();
            for segment in &mut self.segments {
                segment.close_and_delete();
            }
            self.segments.clear();
            self.generation += 1;
        }
    }

    /// Final clean-up when the route leaves service (Java `destroy`): close and
    /// remove any stray segment files for this route across generations.
    pub fn destroy(&mut self) {
        self.close();
        purge_leftover_segments(base_dir(), Some(&self.safe_id));
    }

    fn append_to_disk(&mut self, event: &[u8]) -> Result<(), AppError> {
        let needs_new_tail = match self.segments.back() {
            None => true,
            Some(tail) => tail.sealed,
        };
        if needs_new_tail {
            let index = self.segments.back().map_or(0, |t| t.index + 1);
            let path = base_dir().join(format!(
                "{SEGMENT_PREFIX}{}-{}-{}{SEGMENT_SUFFIX}",
                self.safe_id, self.generation, index
            ));
            self.segments.push_back(Segment::open(path, index)?);
        }
        let single_segment = self.segments.len() == 1;
        let segment_bytes = self.segment_bytes;
        let tail = self.segments.back_mut().expect("tail segment just ensured");
        // portable record format, byte-identical to the Java store:
        // [4-byte big-endian length][payload]
        let mut record = Vec::with_capacity(LENGTH_PREFIX + event.len());
        record.extend_from_slice(&(event.len() as u32).to_be_bytes());
        record.extend_from_slice(event);
        let write_pos = tail.write_pos;
        let file = tail.handle()?;
        file.seek(SeekFrom::Start(write_pos))
            .and_then(|_| file.write_all(&record))
            .map_err(|e| AppError::new(500, format!("Elastic spill write failed - {e}")))?;
        tail.write_pos += record.len() as u64;
        tail.records_written += 1;
        if tail.write_pos >= segment_bytes {
            tail.sealed = true;
            // keep only the read head open (the tail is not the head unless
            // this is the sole segment)
            if !single_segment {
                tail.close_quietly();
            }
        }
        Ok(())
    }

    fn read_from_disk(&mut self) -> Result<Vec<u8>, AppError> {
        let Some(head) = self.segments.front_mut() else {
            log::error!(
                "Missing segment for {} at read position {}",
                self.id,
                self.read_counter
            );
            return Ok(Vec::new());
        };
        let read_pos = head.read_pos;
        let file = head.handle()?;
        let mut len_buf = [0u8; LENGTH_PREFIX];
        file.seek(SeekFrom::Start(read_pos))
            .and_then(|_| file.read_exact(&mut len_buf))
            .map_err(|e| AppError::new(500, format!("Elastic spill read failed - {e}")))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut payload = vec![0u8; len];
        file.read_exact(&mut payload)
            .map_err(|e| AppError::new(500, format!("Elastic spill read failed - {e}")))?;
        head.read_pos += (LENGTH_PREFIX + len) as u64;
        head.records_read += 1;
        // a sealed, fully-consumed segment is reclaimed immediately
        // (O(1), no cleaner thread)
        if head.sealed && head.records_read >= head.records_written {
            head.close_and_delete();
            self.segments.pop_front();
        }
        Ok(payload)
    }
}

impl Drop for ElasticQueue {
    fn drop(&mut self) {
        self.destroy();
    }
}
