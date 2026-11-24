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

//! The number of plugins the engine ships. `BUILTIN_PLUGIN_COUNT` is the floor the
//! `SimplePluginLoader` checks at startup with "at least", so a constant that falls behind
//! fails nothing there: it stayed one short from `rangeLookup` until the month and year date
//! plugins. This test binary declares no plugin of its own, so the link-time inventory holds
//! the engine's built-ins alone and the count is exact.

use event_script::plugins::{load_inventory_plugins, BUILTIN_PLUGIN_COUNT};

#[test]
fn the_constant_is_the_number_of_built_in_plugins() {
    assert_eq!(
        BUILTIN_PLUGIN_COUNT,
        load_inventory_plugins(),
        "BUILTIN_PLUGIN_COUNT must equal the number of #[simple_plugin] declarations the engine \
         ships - update the constant when a plugin is added or removed"
    );
}
