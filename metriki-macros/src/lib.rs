use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ItemFn, Lit, LitStr, Meta, Result as SynResult, Token};

struct FnMetricsAttributes {
    registry: Expr,
    name: Option<Lit>,
}

impl Parse for FnMetricsAttributes {
    /// to parse #[timed(name="...", registry="...")]
    fn parse(input: ParseStream) -> SynResult<Self> {
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut result = FnMetricsAttributes {
            registry: syn::parse_str("metriki_core::global::global_registry()")?,
            name: None,
        };

        // convert attribute metas to key-value map
        for i in metas {
            if let Meta::NameValue(mnv) = i {
                if mnv.path.is_ident("name") {
                    if let Lit::Str(ref litstr) = mnv.lit {
                        result.name = Some(Lit::Str(litstr.clone()));
                    }
                }

                if mnv.path.is_ident("registry") {
                    if let Lit::Str(ref litstr) = mnv.lit {
                        result.registry = syn::parse_str(&litstr.value())?;
                    }
                }
            }
        }

        Ok(result)
    }
}

#[proc_macro_attribute]
pub fn timed(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let timer_data = parse_macro_input!(attrs as FnMetricsAttributes);

    // function data
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = f;
    let stmts = &block.stmts;

    let registry = timer_data.registry;
    // use function name by default
    let name = timer_data
        .name
        .unwrap_or_else(|| Lit::Str(LitStr::new(&sig.ident.to_string(), Span::call_site())));

    // generated code
    let tokens = quote! {
        #(#attrs)*
        #vis #sig {
            let __timer = #registry.timer(#name);
            let __timer_ctx = __timer.start();

            #(#stmts)*
        }
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn metered(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let timer_data = parse_macro_input!(attrs as FnMetricsAttributes);

    // function data
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = f;
    let stmts = &block.stmts;

    let registry = timer_data.registry;
    // use function name by default
    let name = timer_data
        .name
        .unwrap_or_else(|| Lit::Str(LitStr::new(&sig.ident.to_string(), Span::call_site())));

    // generated code
    let tokens = quote! {
        #(#attrs)*
        #vis #sig {
            #registry.meter(#name).mark();

            #(#stmts)*
        }
    };

    tokens.into()
}
