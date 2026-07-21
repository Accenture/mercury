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

//! Integration tests for increment 1 — configuration management
//! (`docs/design/platform-core-port.md` §4.6).
//!
//! `setup()` runs exactly once: it prepends the test resource root (mirroring
//! Java's test-resources-shadow-main-resources), sets the env vars the base
//! config resolves at init, and *then* initializes the `AppConfigReader`
//! singleton. Every test calls it first, so ordering is deterministic and env
//! mutation never races test threads.

use platform_core::{overrides, resources, AppConfigReader, ConfigError, ConfigReader};
use std::sync::Once;

fn setup() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        // test resources shadow the main `resources/` root (Java behavior)
        resources::prepend_resource_root("tests/resources");
        // env vars must exist before the singleton resolves references at init
        std::env::set_var("APP_PROFILES_ACTIVE", "test");
        std::env::set_var("PC_TEST_HOME", "/custom-home");
        let _ = AppConfigReader::get_instance();
    });
}

// ---- formats ----

#[test]
fn yaml_json_properties_expose_the_same_composite_keys() {
    setup();
    let yaml = ConfigReader::load("classpath:/test.yaml").unwrap();
    let json = ConfigReader::load("classpath:/test.json").unwrap();
    let props = ConfigReader::load("classpath:/test.properties").unwrap();
    for reader in [&yaml, &json, &props] {
        assert_eq!(
            reader.get_property("hello.world").as_deref(),
            Some("some value")
        );
        assert_eq!(reader.get_property("hello.list[1]").as_deref(), Some("2"));
        assert!(reader.exists("hello.list[2]"));
        assert!(!reader.exists("hello.list[3]"));
    }
}

#[test]
fn classpath_prefix_is_optional() {
    setup();
    let reader = ConfigReader::load("test.yaml").unwrap();
    assert_eq!(
        reader.get_property("hello.world").as_deref(),
        Some("some value")
    );
}

#[test]
fn yml_and_yaml_extensions_are_interchangeable() {
    setup();
    // only alt.yaml exists on disk — requesting .yml must fall back
    let reader = ConfigReader::load("classpath:/alt.yml").unwrap();
    assert_eq!(
        reader.get_property("alternative.extension").as_deref(),
        Some("works")
    );
}

#[test]
fn file_prefix_reads_the_filesystem() {
    setup();
    let dir = std::env::temp_dir().join(format!("pc-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("standalone.yml");
    std::fs::write(&file, "standalone:\n  key: file-value\n").unwrap();
    let reader = ConfigReader::load(&format!("file:{}", file.display())).unwrap();
    assert_eq!(
        reader.get_property("standalone.key").as_deref(),
        Some("file-value")
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn missing_file_is_not_found() {
    setup();
    let err = ConfigReader::load("classpath:/no-such-file.yml").unwrap_err();
    assert!(matches!(err, ConfigError::NotFound(_)));
    let err = ConfigReader::load("file:/no/such/dir/no-such-file.yml").unwrap_err();
    assert!(matches!(err, ConfigError::NotFound(_)));
}

#[test]
fn parent_traversal_is_rejected() {
    setup();
    let err = ConfigReader::load("file:/tmp/../etc/anything.yml").unwrap_err();
    assert!(matches!(err, ConfigError::Invalid(_)));
    let err = ConfigReader::load("classpath:/../secret.yml").unwrap_err();
    assert!(matches!(err, ConfigError::Invalid(_)));
}

// ---- AppConfigReader: merge order, profiles, singleton ----

#[test]
fn later_manifest_files_override_earlier_ones() {
    setup();
    // application.properties sets merge.order=properties; application.yml
    // (merged later per app-config-reader.yml) overrides it
    let app = AppConfigReader::get_instance();
    assert_eq!(app.get_property("merge.order").as_deref(), Some("yaml"));
    // keys unique to one file survive the merge
    assert_eq!(
        app.get_property("from.properties.only").as_deref(),
        Some("props-value")
    );
}

#[test]
fn active_profile_overlays_the_base() {
    setup();
    let app = AppConfigReader::get_instance();
    // APP_PROFILES_ACTIVE=test merges application-test.yml on top
    assert_eq!(
        app.get_property("profile.indicator").as_deref(),
        Some("test-profile")
    );
    assert_eq!(
        app.get_property("profile-only.key").as_deref(),
        Some("only-in-test-profile")
    );
}

#[test]
fn singleton_returns_the_same_instance() {
    setup();
    let a = AppConfigReader::get_instance() as *const AppConfigReader;
    let b = AppConfigReader::get_instance() as *const AppConfigReader;
    assert_eq!(a, b);
    assert!(AppConfigReader::get_instance().is_base_config());
    assert!(!AppConfigReader::get_instance().is_empty());
}

// ---- substitution ----

#[test]
fn env_var_substitution_resolves_at_init() {
    setup();
    let app = AppConfigReader::get_instance();
    assert_eq!(
        app.get_property("subst.env").as_deref(),
        Some("/custom-home")
    );
}

#[test]
fn missing_env_var_falls_back_to_the_brace_default() {
    setup();
    let app = AppConfigReader::get_instance();
    assert_eq!(
        app.get_property("subst.default").as_deref(),
        Some("fallback-value")
    );
}

#[test]
fn multi_segment_reference_reconstructs_the_text() {
    setup();
    let app = AppConfigReader::get_instance();
    // ${server.host} + ${server.port} come from application.properties
    assert_eq!(
        app.get_property("subst.ref").as_deref(),
        Some("http://localhost:8085/x")
    );
}

#[test]
fn config_loop_is_detected_not_followed() {
    setup();
    // looptest.a -> ${looptest.b} -> ${looptest.a}: the loop resolves to
    // nothing instead of hanging (a warning is logged)
    let app = AppConfigReader::get_instance();
    assert_eq!(app.get_property("looptest.a"), None);
    assert!(!app.exists("looptest.a"));
}

#[test]
fn override_registry_beats_the_file_value() {
    setup();
    let app = AppConfigReader::get_instance();
    assert_eq!(app.get_property("override.demo").as_deref(), Some("file"));
    overrides::set("override.demo", "live");
    assert_eq!(app.get_property("override.demo").as_deref(), Some("live"));
    overrides::clear("override.demo");
    assert_eq!(app.get_property("override.demo").as_deref(), Some("file"));
}

#[test]
fn standalone_reader_resolves_references_against_the_base_config() {
    setup();
    // test.properties: ref.from.base=${server.port} — resolves via AppConfigReader
    let reader = ConfigReader::load("classpath:/test.properties").unwrap();
    assert_eq!(
        reader.get_property("ref.from.base").as_deref(),
        Some("8085")
    );
}

// ---- typed access & views ----

#[test]
fn get_property_enforces_strings() {
    setup();
    let app = AppConfigReader::get_instance();
    assert_eq!(app.get_property("typed.int").as_deref(), Some("42"));
    assert_eq!(app.get_property("typed.bool").as_deref(), Some("true"));
    assert_eq!(
        app.get_property_or("typed.missing", "default"),
        "default".to_string()
    );
}

#[test]
fn nested_lists_flatten_and_resolve() {
    setup();
    let app = AppConfigReader::get_instance();
    assert_eq!(
        app.get_property("deep.list[0].name").as_deref(),
        Some("first")
    );
    assert_eq!(
        app.get_property("deep.list[1].name").as_deref(),
        Some("second")
    );
}

#[test]
fn composite_key_values_view_is_substituted_and_cached() {
    setup();
    let app = AppConfigReader::get_instance();
    let view = app.get_composite_key_values();
    assert_eq!(
        view.get("subst.ref")
            .map(|v| v.to_display_string())
            .as_deref(),
        Some("http://localhost:8085/x")
    );
    // same cached map on second call
    let again = app.get_composite_key_values();
    assert_eq!(view as *const _, again as *const _);
}

// ---- increment 55: config parity (findings F11 / F13) ----

/// F11: a repeated `${ref}` in one value — or a diamond through a nested
/// reference — is NOT a cycle (Java keeps a fresh per-segment chain; the old
/// shared visited-list blanked the repeat and warned "Config loop").
#[test]
fn repeated_references_are_not_false_cycles() {
    setup();
    let app = AppConfigReader::get_instance();
    assert_eq!(
        app.get_property("subst.repeated").as_deref(),
        Some("localhost localhost")
    );
    assert_eq!(
        app.get_property("subst.diamond").as_deref(),
        Some("http://localhost:8085/x on localhost")
    );
}

/// F13: `.properties` follows java.util.Properties.load — `:`/whitespace
/// separators, backslash continuations, escapes, trailing whitespace kept.
#[test]
fn properties_syntax_matches_java_util_properties() {
    setup();
    let props = ConfigReader::load("classpath:/props-syntax.properties").unwrap();
    assert_eq!(
        props.get_property("colon.key").as_deref(),
        Some("colon value")
    );
    assert_eq!(
        props.get_property("space.key").as_deref(),
        Some("value with space separator")
    );
    assert_eq!(
        props.get_property("multi.line").as_deref(),
        Some("first second")
    );
    assert_eq!(props.get_property("escaped.tab").as_deref(), Some("a\tb"));
    assert_eq!(props.get_property("unicode.key").as_deref(), Some("ABC"));
    assert_eq!(
        props.get_property("slash.path").as_deref(),
        Some("C:\\temp")
    );
    // Java preserves the value's trailing whitespace (no trimming)
    assert_eq!(
        props.get_property("trailing.space").as_deref(),
        Some("keep   ")
    );
    // both comment forms skipped
    assert!(!props.exists("!"));
    assert!(!props.exists("#"));
}
