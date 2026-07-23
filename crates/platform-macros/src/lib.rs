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
//!
//! `#[optional_service("condition")]` is a **first-class attribute** (the Java
//! `@OptionalService` analog): it makes a composable function, a websocket
//! server function, a `#[before_application]` or a `#[main_application]`
//! conditional on application configuration, and works in **either stacking
//! order** — above or below the primary attribute:
//!
//! ```ignore
//! #[optional_service("app.env=dev")]   // Java order: the condition on top
//! #[preload(route = "dev.only.service")]
//! struct DevOnlyService;
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitInt, LitStr};

/// The Java `@PreLoad(route, instances, envInstances)` analog: registers a
/// composable function at startup.
///
/// Parameters:
/// - `route = "my.function.route"` (required). A **comma-separated list**
///   registers the same function under several route names (aliases) with the
///   same instance count and visibility — the Java
///   `@PreLoad(route = "hello.world, hello.declarative")` behavior. Each name
///   is whitespace-trimmed; an empty segment is a compile error.
/// - `instances = N` (default 1)
/// - `env_instances = "config.key"` — read the instance count from application
///   configuration at startup, falling back to `instances` (Java `envInstances`)
/// - `is_private = false` — a PUBLIC function, callable from another
///   application instance through Event over HTTP. Java `@PreLoad` parity:
///   preloaded functions are **private by default** (in-instance only);
///   opt into public visibility explicitly (Java `isPrivate = false`)
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
/// - `#[optional_service("condition")]` — see [`optional_service`]: a
///   first-class attribute that also works stacked *above* this one.
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
    // Java @PreLoad: isPrivate() default TRUE — a preloaded function is
    // private unless it explicitly opts into public visibility
    let mut is_private = true;
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
        } else if meta.path.is_ident("is_private") {
            let lit: syn::LitBool = meta.value()?.parse()?;
            is_private = lit.value();
        } else {
            return Err(meta.error(
                "unknown preload parameter (expected route, instances, env_instances, \
                 typed, zero_tracing, interceptor, is_private; a conditional registration \
                 is declared with the separate #[optional_service(\"...\")] attribute)",
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
    // a comma-separated route list declares ALIASES (Java @PreLoad parity);
    // validate the list shape at compile time — empty segments are an error
    if let Err(message) = validate_route_list(&route.value()) {
        return syn::Error::new_spanned(&route, message)
            .to_compile_error()
            .into();
    }
    // consume stacked marker attributes (Java annotation stacking)
    zero_tracing |= strip_marker(&mut item, "zero_tracing");
    interceptor |= strip_marker(&mut item, "event_interceptor");
    let optional_expr = optional_service_expr(&strip_optional_service(&mut item));
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
                optional_service: #optional_expr,
                zero_tracing: #zero_tracing,
                interceptor: #interceptor,
                is_private: #is_private,
                factory: || #factory,
            }
        }
    };
    expanded.into()
}

/// The Java `@WebSocketService(value, namespace)` analog: registers a
/// websocket server endpoint declaratively. The annotated struct implements
/// `ComposableFunction` and receives the session lifecycle events
/// (`type: open` / `string` / `bytes` / `close`) on its per-connection
/// `{session}.in` route; replies go to the `tx_path` given in the headers.
///
/// ```ignore
/// #[websocket_service("graph")]                     // /ws/graph/{token}
/// struct GraphUserInterface;
///
/// #[websocket_service(name = "json", namespace = "ws")]
/// struct JsonPathHandler;
/// ```
///
/// The AppStarter lifecycle collects these entries and registers the URL
/// paths before the HTTP server starts; the server itself starts when REST
/// automation is enabled **or** at least one websocket service exists
/// (Java parity). One function object is created per connection.
#[proc_macro_attribute]
pub fn websocket_service(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut name: Option<LitStr> = None;
    let mut namespace: Option<LitStr> = None;
    // Java positional form: #[websocket_service("graph")]
    if let Ok(positional) = syn::parse::<LitStr>(args.clone()) {
        name = Some(positional);
    } else {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") || meta.path.is_ident("value") {
                name = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("namespace") {
                namespace = Some(meta.value()?.parse()?);
            } else {
                return Err(meta.error(
                    "unknown websocket_service parameter (expected a service name literal, \
                     name/value = \"...\", namespace = \"...\")",
                ));
            }
            Ok(())
        });
        parse_macro_input!(args with parser);
    }
    let mut item = parse_macro_input!(input as ItemStruct);
    let Some(name) = name else {
        return syn::Error::new_spanned(
            &item.ident,
            "#[websocket_service] requires a service name, e.g. #[websocket_service(\"graph\")]",
        )
        .to_compile_error()
        .into();
    };
    let namespace = namespace.unwrap_or_else(|| LitStr::new("ws", name.span()));
    // consume a stacked `#[optional_service("...")]` marker (Java @OptionalService)
    let optional_expr = optional_service_expr(&strip_optional_service(&mut item));
    let construct = constructor(&item);
    let expanded = quote! {
        #item
        ::platform_core::inventory::submit! {
            ::platform_core::registry::WsServiceEntry {
                name: #name,
                namespace: #namespace,
                optional_service: #optional_expr,
                factory: || ::std::sync::Arc::new(#construct),
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

/// The Java `@OptionalService("condition")` analog — a **first-class**
/// attribute that makes a composable function (`#[preload]`), a websocket
/// server function (`#[websocket_service]`), a `#[before_application]` or a
/// `#[main_application]` **conditional on application configuration**: the
/// item registers only when the condition holds at startup (Java
/// `Feature.isRequired` semantics — comma-separated OR, `!key` negation,
/// `key=value` / `key` / `key=` forms, case-insensitive).
///
/// Works in **either stacking order**:
///
/// ```ignore
/// #[optional_service("app.env=dev")]      // Java order — condition on top
/// #[preload(route = "dev.only.service")]
/// struct DevOnly;
///
/// #[preload(route = "also.dev.only")]     // marker order — condition below
/// #[optional_service("app.env=dev")]
/// struct AlsoDevOnly;
/// ```
///
/// When written above the primary attribute, this macro expands first and
/// re-attaches the condition below it, where the primary attribute consumes
/// it; when written below, the primary attribute consumes it directly. Using
/// it without one of the four primary attributes is a compile error.
#[proc_macro_attribute]
pub fn optional_service(args: TokenStream, input: TokenStream) -> TokenStream {
    let condition = match syn::parse::<LitStr>(args) {
        Ok(lit) if !lit.value().trim().is_empty() => lit,
        _ => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "#[optional_service] requires a condition string, \
                 e.g. #[optional_service(\"app.env=dev\")]",
            )
            .to_compile_error()
            .into();
        }
    };
    let mut item = parse_macro_input!(input as ItemStruct);
    // The condition must reach one of the primary attributes still on the item
    // (they expand after this macro and consume the marker it re-attaches).
    const PRIMARIES: [&str; 4] = [
        "preload",
        "websocket_service",
        "before_application",
        "main_application",
    ];
    let has_primary = item.attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .is_some_and(|seg| PRIMARIES.contains(&seg.ident.to_string().as_str()))
    });
    if !has_primary {
        return syn::Error::new_spanned(
            &item.ident,
            "#[optional_service] must be stacked with #[preload], #[websocket_service], \
             #[before_application] or #[main_application]",
        )
        .to_compile_error()
        .into();
    }
    item.attrs
        .push(syn::parse_quote!(#[optional_service(#condition)]));
    quote!(#item).into()
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
    let mut item = parse_macro_input!(input as ItemStruct);
    // consume a stacked `#[optional_service("...")]` marker (Java @OptionalService)
    let optional_expr = optional_service_expr(&strip_optional_service(&mut item));
    let construct = constructor(&item);
    let expanded = quote! {
        #item
        ::platform_core::inventory::submit! {
            ::platform_core::registry::#entry_type {
                sequence: #sequence,
                optional_service: #optional_expr,
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

/// Consume a stacked `#[optional_service("condition")]` marker (Java
/// `@OptionalService`), returning its condition string literal. Removes the
/// attribute so it does not reach the compiler.
fn strip_optional_service(item: &mut ItemStruct) -> Option<LitStr> {
    let mut found = None;
    item.attrs.retain(|attr| {
        if attr.path().is_ident("optional_service") {
            if let Ok(lit) = attr.parse_args::<LitStr>() {
                found = Some(lit);
            }
            false
        } else {
            true
        }
    });
    found
}

/// Render an `Option<&'static str>` initializer for a registry entry's
/// `optional_service` field.
fn optional_service_expr(cond: &Option<LitStr>) -> proc_macro2::TokenStream {
    match cond {
        Some(c) => quote!(::core::option::Option::Some(#c)),
        None => quote!(::core::option::Option::None),
    }
}

/// Validate a `#[preload]` route value: one route name, or a comma-separated
/// list of route names (aliases — Java `@PreLoad(route = "a.b, c.d")`). Each
/// segment is whitespace-trimmed; an **empty segment** (leading/trailing/
/// doubled comma, or a blank value) is rejected here at compile time.
/// Route-name *shape* (lowercase, at least one dot) stays a startup-time
/// check in `Platform::register`, exactly as for a single route.
fn validate_route_list(route: &str) -> Result<(), String> {
    let segments: Vec<&str> = route.split(',').map(str::trim).collect();
    if segments.iter().any(|segment| segment.is_empty()) {
        return Err(format!(
            "invalid #[preload] route list '{route}' - each comma-separated \
             route name must be non-empty"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_route_list;

    #[test]
    fn single_route_is_valid() {
        assert!(validate_route_list("hello.world").is_ok());
    }

    #[test]
    fn comma_separated_aliases_are_valid_with_or_without_spaces() {
        assert!(validate_route_list("hello.world, hello.declarative").is_ok());
        assert!(validate_route_list("hello.world,hello.declarative").is_ok());
        assert!(validate_route_list("a.b , c.d ,  e.f").is_ok());
    }

    #[test]
    fn empty_segments_are_rejected() {
        assert!(validate_route_list("").is_err());
        assert!(validate_route_list("   ").is_err());
        assert!(validate_route_list("hello.world,").is_err());
        assert!(validate_route_list(",hello.world").is_err());
        assert!(validate_route_list("hello.world,,hello.declarative").is_err());
        assert!(validate_route_list("hello.world, ,hello.declarative").is_err());
    }
}
