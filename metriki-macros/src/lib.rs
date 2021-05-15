use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, ItemFn, Lit, Meta, NestedMeta, Result as SynResult, Token};

#[derive(Debug)]
struct TimedAttributes {
    registry: String,
    name: String,
}

impl Parse for TimedAttributes {
    /// to parse #[timed(name="...", registry="...")]
    fn parse(input: ParseStream) -> SynResult<Self> {
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut hash = HashMap::new();

        // convert attribute metas to key-value map
        for i in metas {
            if let Meta::NameValue(mnv) = i {
                //                    hash.insert(mnv.path, mvn.lit);
                if mnv.path.is_ident("name") {
                    if let Lit::Str(ref litstr) = mnv.lit {
                        hash.insert("name", litstr.value());
                    }
                }

                if mnv.path.is_ident("registry") {
                    if let Lit::Str(ref litstr) = mnv.lit {
                        hash.insert("registry", litstr.value());
                    }
                }
            }
        }

        // TODO: unwrap
        Ok(TimedAttributes {
            registry: hash
                .get("registry")
                .cloned()
                .unwrap_or_else(|| "::metriki_core::global::global_registry()".to_owned()),
            name: hash.get("name").cloned().unwrap(),
        })
    }
}

#[proc_macro_attribute]
pub fn timed(attrs: TokenStream, input: TokenStream) -> TokenStream {
    //    let input_fn = parse_macro_input!(input as ItemFn);
    let attrs = parse_macro_input!(attrs as TimedAttributes);

    dbg!(attrs);

    input
}
