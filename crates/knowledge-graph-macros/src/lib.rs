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
/// a provider node lists the feature by name in its `feature` property.
///
/// Field installations use this for pre/post-processing of provider HTTP
/// calls — e.g. an `oauth-bearer` feature that fetches/refreshes an OAuth
/// 2.0 access token and inserts the bearer token into the outbound request.
///
/// ```ignore
/// #[fetch_feature("oauth-bearer")]
/// struct OAuthBearer;      // impl knowledge_graph::features::FeatureRunner
/// ```
///
/// The engine loads all declared features during startup (the Java
/// `PlaygroundLoader` scan analog), before any graph executes.
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
    let item = parse_macro_input!(input as ItemStruct);
    let Some(name) = name else {
        return syn::Error::new_spanned(
            &item.ident,
            "#[fetch_feature] requires a feature name, e.g. #[fetch_feature(\"oauth-bearer\")]",
        )
        .to_compile_error()
        .into();
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
                factory: || ::std::sync::Arc::new(#construct),
            }
        }
    };
    expanded.into()
}
