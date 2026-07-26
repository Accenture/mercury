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

//! Attribute macros for the knowledge-graph layer. Java discovers
//! `@FetchFeature` classes by classpath scanning; Rust has no runtime
//! scanning, so `#[fetch_feature]` registers the feature in a link-time
//! inventory the engine collects at startup.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitStr};

/// The Java `@FetchFeature(value)` analog: registers an API-fetcher feature
/// declaratively. The annotated struct implements the `FeatureRunner` trait;
/// a provider node lists the feature by name in its `feature` property. The
/// engine's own built-in features (`log-request-headers` /
/// `log-response-headers`) are declared with this same macro — the engine
/// dogfoods its extension point, like Java's `@FetchFeature` classes.
///
/// Field installations use this for pre/post-processing of provider HTTP
/// calls — e.g. an `oauth-bearer` feature that fetches/refreshes an OAuth
/// 2.0 access token and inserts the bearer token into the outbound request.
///
/// A stacked `#[optional_service("condition")]` marker (the Java
/// `@OptionalService` analog, same grammar as on the platform macros) makes
/// the feature conditional on application configuration — evaluated at boot,
/// never at expansion time:
///
/// ```ignore
/// #[fetch_feature("oauth-bearer")]
/// #[optional_service("oauth.token.endpoint")]
/// struct OAuthBearer;      // impl knowledge_graph::features::FeatureRunner
/// ```
///
/// The engine loads all declared features during startup (the Java
/// `PlaygroundLoader` scan analog), before any graph executes. Note:
/// `#[simple_plugin]` deliberately has NO optional_service — plugins are
/// Event Script capabilities (flow vocabulary), never conditionally on/off.
#[proc_macro_attribute]
pub fn fetch_feature(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut name: Option<LitStr> = None;
    // Java positional form: #[fetch_feature("oauth-bearer")]
    if let Ok(positional) = syn::parse::<LitStr>(args.clone()) {
        name = Some(positional);
    } else {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("name") || meta.path.is_ident("value") {
                name = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error(
                    "unknown fetch_feature parameter (expected a feature name literal or \
                     name/value = \"...\")",
                ))
            }
        });
        parse_macro_input!(args with parser);
    }
    let mut item = parse_macro_input!(input as ItemStruct);
    let Some(name) = name else {
        return syn::Error::new_spanned(
            &item.ident,
            "#[fetch_feature] requires a feature name, e.g. #[fetch_feature(\"oauth-bearer\")]",
        )
        .to_compile_error()
        .into();
    };
    // stacked #[optional_service("...")] marker (the platform-macro pattern)
    let optional_service = strip_optional_service(&mut item);
    let optional_service = match &optional_service {
        Some(cond) => quote!(::core::option::Option::Some(#cond)),
        None => quote!(::core::option::Option::None),
    };
    let ident = &item.ident;
    let construct = match item.fields {
        syn::Fields::Unit => quote!(#ident),
        _ => quote!(<#ident as ::core::default::Default>::default()),
    };
    let expanded = quote! {
        #item
        ::knowledge_graph::inventory::submit! {
            ::knowledge_graph::features::FetchFeatureEntry {
                name: #name,
                optional_service: #optional_service,
                factory: || ::std::sync::Arc::new(#construct),
            }
        }
    };
    expanded.into()
}

/// Consume a stacked `#[optional_service("condition")]` marker (Java
/// `@OptionalService`), returning its condition string literal — the same
/// strip/fold pattern the platform macros use. Removes the attribute so it
/// does not reach the compiler.
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
