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

//! The Java `@SimplePlugin` analog for the event-script layer: annotate a
//! plugin function and it registers itself through the link-time inventory —
//! the `SimplePluginLoader` (`#[before_application(sequence = 3)]`) collects
//! every entry before flows compile, so `f:name(...)` mappings validate at
//! compile time and resolve at runtime.
//!
//! ```ignore
//! #[simple_plugin(name = "myCalc")]           // name defaults to camelCase
//! fn my_calc(args: &[Value]) -> Result<Value, String> { ... }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Register a plugin function (Java `@SimplePlugin` + `PluginFunction`).
/// The function must have the signature
/// `fn(&[rmpv::Value]) -> Result<rmpv::Value, String>`. The plugin name
/// defaults to the camelCase form of the function name (the Java
/// class-name-to-camelCase convention); override with `name = "..."`.
#[proc_macro_attribute]
pub fn simple_plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut name: Option<LitStr> = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            name = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unknown simple_plugin parameter (expected name)"))
        }
    });
    parse_macro_input!(args with parser);
    let item = parse_macro_input!(input as ItemFn);
    let fn_ident = &item.sig.ident;
    let plugin_name = match name {
        Some(lit) => lit.value(),
        None => camel_case(&fn_ident.to_string()),
    };
    let expanded = quote! {
        #item
        ::event_script::inventory::submit! {
            ::event_script::plugins::SimplePluginEntry {
                name: #plugin_name,
                body: #fn_ident,
            }
        }
    };
    expanded.into()
}

/// snake_case → camelCase (the Java default-name convention).
fn camel_case(snake: &str) -> String {
    let mut out = String::new();
    let mut upper_next = false;
    for c in snake.chars() {
        if c == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(c.to_uppercase());
            upper_next = false;
        } else {
            out.push(c);
        }
    }
    out
}
