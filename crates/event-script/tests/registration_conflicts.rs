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

//! The one conflict policy (D2, annotation→macro consistency round): within
//! the simple-plugin registry a duplicate name = WARN + last-wins, matching
//! the Java `SimplePluginLoader` wording — including a user
//! `#[simple_plugin]` shadowing a built-in. A dedicated test binary: it
//! installs a capturing logger (nothing else boots here) and deliberately
//! shadows a built-in name, which must never leak into other suites.

use std::sync::{Mutex, OnceLock};

use event_script::plugins;
use event_script::simple_plugin;
use rmpv::Value;

struct CaptureLogger;

fn captured() -> &'static Mutex<Vec<String>> {
    static LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LOGS.get_or_init(|| Mutex::new(Vec::new()))
}

impl log::Log for CaptureLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        captured().lock().expect("capture log").push(format!(
            "{}: {}",
            record.level(),
            record.args()
        ));
    }

    fn flush(&self) {}
}

/// A user plugin that shadows the built-in `isEmpty` — the adversarial case
/// the conflict policy must WARN about (Java: "Reloading SimplePlugin ...").
/// Deliberately uses the `name = "..."` alias form: the regression that the
/// alias grammar keeps working alongside the canonical positional form.
#[simple_plugin(name = "isEmpty")]
fn shadow_is_empty(_args: &[Value]) -> Result<Value, String> {
    Ok(Value::from("shadowed"))
}

/// The canonical positional form (`#[simple_plugin("...")]` — the
/// `#[fetch_feature]` grammar): the string is the registered name.
#[simple_plugin("positionalDemo")]
fn any_fn_name(_args: &[Value]) -> Result<Value, String> {
    Ok(Value::from("positional"))
}

/// One sequential test fn: the capture logger must be installed before the
/// registry's one-time inventory fold, and the assertions read the shared
/// captured log.
#[test]
fn duplicate_names_warn_and_last_wins() {
    log::set_logger(&CaptureLogger).expect("no other logger in this binary");
    log::set_max_level(log::LevelFilter::Debug);

    // 1. the user shadow of a built-in warns during the inventory fold
    //    (which body wins is link-order-dependent, exactly like a Java
    //    classpath-scan order — the WARN is the contract, and the name
    //    still resolves)
    assert!(plugins::contains_simple_plugin("isEmpty"));
    let warned = captured().lock().unwrap().iter().any(|line| {
        line.contains("WARN")
            && line.contains("Reloading SimplePlugin isEmpty - please check duplicated plugin name")
    });
    assert!(
        warned,
        "a user #[simple_plugin] shadowing a built-in must warn: {:?}",
        captured().lock().unwrap()
    );
    assert!(plugins::calculate("isEmpty", &[Value::Array(vec![])]).is_ok());

    // 2. explicit register_plugin: duplicate name warns + last-wins
    plugins::register_plugin("dupTest", |_args| Ok(Value::from("first")));
    plugins::register_plugin("dupTest", |_args| Ok(Value::from("second")));
    assert_eq!(
        plugins::calculate("dupTest", &[]),
        Ok(Value::from("second")),
        "last registration wins"
    );
    let warned = captured().lock().unwrap().iter().any(|line| {
        line.contains("WARN")
            && line.contains("Reloading SimplePlugin dupTest - please check duplicated plugin name")
    });
    assert!(
        warned,
        "an explicit duplicate registration must warn: {:?}",
        captured().lock().unwrap()
    );

    // 3. the positional argument grammar registers under the given string
    //    (the fn name is deliberately unrelated); the name= alias was proven
    //    by the shadow plugin above, and the no-argument camelCase derivation
    //    by the flow_runtime `shout` fixture
    assert_eq!(
        plugins::calculate("positionalDemo", &[]),
        Ok(Value::from("positional"))
    );
    assert!(!plugins::contains_simple_plugin("anyFnName"));

    // 4. the built-in floor: all 46 engine plugins arrived through the
    //    inventory (the count the SimplePluginLoader hook asserts at boot)
    assert!(
        plugins::load_inventory_plugins() >= plugins::BUILTIN_PLUGIN_COUNT,
        "the built-in #[simple_plugin] declarations must all be collected"
    );
}
