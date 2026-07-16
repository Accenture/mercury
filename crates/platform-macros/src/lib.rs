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

//! Attribute macros emulating mercury-composable's Java annotations. Java
//! discovers annotated classes by classpath scanning at startup; Rust has no
//! runtime scanning, so these macros register each item in a **link-time
//! inventory** (design D6's ergonomic layer) that `AutoStart` collects at
//! startup — the user application needs no manual builder wiring:
//!
//! ```ignore
//! #[preload(route = "greeting.demo", instances = 10, typed)]
//! #[derive(Default)]
//! struct Greetings;            // impl TypedFunction<GreetingRequest, GreetingResponse>
//!
//! #[before_application(sequence = 5)]
//! struct PreflightCheck;       // impl EntryPoint
//!
//! #[main_application]
//! struct MainApp;              // impl EntryPoint
//!
//! platform_core::auto_start_main!();   // the whole main() — Java's AutoStart.main(args)
//! ```
//!
//! Marker attributes placed **below** the primary attribute are consumed by
//! it, mirroring Java's annotation stacking:
//!
//! ```ignore
//! #[preload(route = "some.system.service", instances = 1)]
//! #[zero_tracing]              // this route never traces its own executions
//! struct SystemService;
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitInt, LitStr};

/// The Java `@PreLoad(route, instances, envInstances)` analog: registers a
/// composable function at startup.
///
/// Parameters:
/// - `route = "my.function.route"` (required)
/// - `instances = N` (default 1)
/// - `env_instances = "config.key"` — read the instance count from application
///   configuration at startup, falling back to `instances` (Java `envInstances`)
/// - `typed` — the struct implements `TypedFunction<I, O>`; it is wrapped in a
///   `TypedAdapter` (without this flag the struct must implement
///   `ComposableFunction`)
///
/// Consumed marker attributes (place them below `#[preload]`):
/// - `#[zero_tracing]` — the Java `@ZeroTracing`: this route's executions are
///   excluded from distributed-trace recording.
/// - `#[event_interceptor]` — the Java `@EventInterceptor`: the function
///   receives the raw envelope (`reply_to`/`cid` intact) and replies manually
///   via `po.send`; the worker sends no auto-reply on success (also available
///   as the `interceptor` flag parameter).
///
/// The struct must be a unit struct or implement `Default`.
#[proc_macro_attribute]
pub fn preload(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut route: Option<LitStr> = None;
    let mut instances: usize = 1;
    let mut env_instances: Option<LitStr> = None;
    let mut typed = false;
    let mut zero_tracing = false;
    let mut interceptor = false;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("route") {
            route = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("instances") {
            let lit: LitInt = meta.value()?.parse()?;
            instances = lit.base10_parse()?;
        } else if meta.path.is_ident("env_instances") {
            env_instances = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("typed") {
            typed = true;
        } else if meta.path.is_ident("zero_tracing") {
            zero_tracing = true;
        } else if meta.path.is_ident("interceptor") {
            interceptor = true;
        } else {
            return Err(meta.error(
                "unknown preload parameter (expected route, instances, env_instances, typed, \
                 zero_tracing, interceptor)",
            ));
        }
        Ok(())
    });
    parse_macro_input!(args with parser);
    let mut item = parse_macro_input!(input as ItemStruct);
    let Some(route) = route else {
        return syn::Error::new_spanned(&item.ident, "#[preload] requires route = \"...\"")
            .to_compile_error()
            .into();
    };
    // consume stacked marker attributes (Java annotation stacking)
    zero_tracing |= strip_marker(&mut item, "zero_tracing");
    interceptor |= strip_marker(&mut item, "event_interceptor");
    let construct = constructor(&item);
    let factory = if typed {
        quote!(::platform_core::TypedAdapter::arc(#construct))
    } else {
        quote!(::std::sync::Arc::new(#construct))
    };
    let env_expr = match &env_instances {
        Some(key) => quote!(::core::option::Option::Some(#key)),
        None => quote!(::core::option::Option::None),
    };
    let expanded = quote! {
        #item
        ::platform_core::inventory::submit! {
            ::platform_core::registry::PreloadEntry {
                route: #route,
                instances: #instances,
                env_instances: #env_expr,
                zero_tracing: #zero_tracing,
                interceptor: #interceptor,
                factory: || #factory,
            }
        }
    };
    expanded.into()
}

/// The Java `@BeforeApplication(sequence)` analog: an `EntryPoint` that runs
/// before functions are registered (validation/compilation work). Lower
/// sequences run first (default 10; 0 is framework-reserved; a failing hook
/// aborts startup).
#[proc_macro_attribute]
pub fn before_application(args: TokenStream, input: TokenStream) -> TokenStream {
    entry_point_attribute(args, input, quote!(BeforeAppEntry), 10)
}

/// The Java `@MainApplication(sequence)` analog: the application entry point
/// (`EntryPoint`), run after preload. Lower sequences run first (default 10).
#[proc_macro_attribute]
pub fn main_application(args: TokenStream, input: TokenStream) -> TokenStream {
    entry_point_attribute(args, input, quote!(MainAppEntry), 10)
}

fn entry_point_attribute(
    args: TokenStream,
    input: TokenStream,
    entry_type: proc_macro2::TokenStream,
    default_sequence: u32,
) -> TokenStream {
    let mut sequence: u32 = default_sequence;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("sequence") {
            let lit: LitInt = meta.value()?.parse()?;
            sequence = lit.base10_parse()?;
            Ok(())
        } else {
            Err(meta.error("unknown parameter (expected sequence)"))
        }
    });
    parse_macro_input!(args with parser);
    let item = parse_macro_input!(input as ItemStruct);
    let construct = constructor(&item);
    let expanded = quote! {
        #item
        ::platform_core::inventory::submit! {
            ::platform_core::registry::#entry_type {
                sequence: #sequence,
                factory: || ::std::sync::Arc::new(#construct),
            }
        }
    };
    expanded.into()
}

/// Unit structs construct directly; anything else goes through `Default`
/// (the Java no-arg-constructor analog).
fn constructor(item: &ItemStruct) -> proc_macro2::TokenStream {
    let ident = &item.ident;
    match item.fields {
        syn::Fields::Unit => quote!(#ident),
        _ => quote!(<#ident as ::core::default::Default>::default()),
    }
}

/// Remove a stacked marker attribute (e.g. `#[zero_tracing]`) from the item,
/// returning whether it was present.
fn strip_marker(item: &mut ItemStruct, name: &str) -> bool {
    let before = item.attrs.len();
    item.attrs.retain(|attr| !attr.path().is_ident(name));
    item.attrs.len() != before
}
