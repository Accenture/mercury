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

//! The graceful-exit cleanup (Java's JVM shutdown hook) in a process of its own: `shutdown_cleanup`
//! removes the holding folder it empties, so it cannot run beside tests that keep using the store.

use platform_core::util::elastic_queue;
use platform_core::{overrides, resources, AppConfigReader};

#[tokio::test]
async fn shutdown_cleanup_removes_marker_segments_and_folder() {
    resources::prepend_resource_root("tests/resources");
    let store = test_support::temp_path("store");
    overrides::set("transient.data.store", &store.display().to_string());
    let _ = AppConfigReader::get_instance();
    // the holding area and its marker exist once housekeeping starts
    let dir = elastic_queue::base_dir().to_path_buf();
    elastic_queue::start_housekeeping();
    assert!(dir.join("RUNNING").is_file());
    std::fs::write(dir.join("eq-leftover-0.dat"), [1u8, 2, 3]).expect("a segment");
    elastic_queue::shutdown_cleanup();
    // the segments and the marker are purged, and the emptied folder is removed
    assert!(!dir.exists(), "the holding folder is removed once empty");
}
