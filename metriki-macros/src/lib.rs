use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ItemFn, Lit, LitStr, Meta, Result as SynResult, Token};

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
                .unwrap_or_else(|| "metriki_core::global::global_registry()".to_owned()),
            name: hash.get("name").cloned().unwrap(),
        })
    }
}

#[proc_macro_attribute]
pub fn timed(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let timer_data = parse_macro_input!(attrs as TimedAttributes);

    // function data
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = f;
    let stmts = &block.stmts;

    // FIXME: unwrap
    let registry_expr: Expr = syn::parse_str(&timer_data.registry).unwrap();
    let name = Lit::Str(LitStr::new(&timer_data.name, Span::call_site()));

    // generated code
    let tokens = quote! {
        #(#attrs)*
        #vis #sig {
            let __timer = #registry_expr.timer(#name);
            let __timer_ctx = __timer.start();

            #(#stmts)*
        }
    };
    tokens.into()
}
