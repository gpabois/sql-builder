use paste::paste;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use sql_builder_def::{detect_loop, fetch_deps, SymbolDef, SYMBOL_MAP};
use sql_builder_meta_macros::create_symbol_derivations;
use syn::{parse_macro_input, token::Token, DeriveInput, Ident};

#[proc_macro]
/// Creates a SQL identifier.
pub fn id(input: TokenStream) -> TokenStream {
    let ident: syn::Ident = parse_macro_input!(input);
    let re = Regex::new("^[A-Za-z_]([A-Za-z0-9_])*").unwrap();
    let str = ident.to_string();

    if !re.is_match(&str) {
        return quote! {
            compile_error!(&format("{} not an identifier", &str));
        }
        .into();
    }

    quote! { ::sql_builder::id(#str) }.into()
}

#[proc_macro]
pub fn check_symbol_loops(_: TokenStream) -> TokenStream {
    SYMBOL_MAP
        .keys()
        .copied()
        .map(detect_loop)
        .map(|r| match r{
            Ok(_) => quote!{},
            Err(detloop) => {
                let msg = format!("Loop detected: {:?}", detloop);
                quote! {
                    compile_error!(#msg)
                }
            },
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
/// Create the symbols traits
pub fn create_symbol_traits(_: TokenStream) -> TokenStream {
    SYMBOL_MAP
        .entries()
        .map(|(symbol, flags)| {
            let mut deps = fetch_deps(symbol)
                .into_iter()
                .map(|dep| syn::Ident::new(dep, Span::call_site()))
                .fold(quote! {crate::ToQuery}, |acc, dep| quote! { #acc + #dep });

            let trait_id = syn::Ident::new(symbol, Span::call_site());

            if flags.with_helpers() {
                deps = quote! {#deps + crate::helpers::#trait_id}
            }

            let body = if flags.with_blank_impl() {
                quote! {const IS_IMPL: bool;}
            } else {
                quote! {}
            };

            quote! {
                /// Symbol "#symbol"
                pub trait #trait_id: #deps {
                    #body
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

macro_rules! create_symbol_derivation {
    ($symbol:ident) => {
        paste! {
            #[proc_macro_derive($symbol)]
            pub fn [<derive_$symbol:snake>](input: TokenStream) -> TokenStream {
                let input: DeriveInput = parse_macro_input!(input);
                derive_symbol(stringify!{$symbol}, &input).into()
            }
        }
    };
}

create_symbol_derivations!{}

/// Creates a symbol derivation
fn derive_symbol(symbol: &str, ast: &DeriveInput) -> proc_macro2::TokenStream {
    fetch_deps(symbol)
        .map(|symbol| {
            impl_symbol_trait(
                &symbol,
                ast,
                SYMBOL_MAP
                    .get(&symbol)
                    .unwrap_or_else(|| panic!("missing symbol {symbol} in the map")),
            )
        })
        .collect()
}

fn impl_symbol_trait(
    symbol: &str,
    ast: &DeriveInput,
    def: &SymbolDef,
) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    let with_impl = match def.with_blank_impl() {
        true => quote! {const IS_IMPL: bool = true;},
        false => quote! {},
    };

    let trait_ident = Ident::new(symbol, proc_macro2::Span::call_site());
    quote! {
        impl #impl_generics crate::grammar:: #trait_ident for #name #type_generics #where_clause {
            #with_impl
        }
    }
}
