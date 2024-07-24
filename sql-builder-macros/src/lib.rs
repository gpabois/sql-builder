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
        .map(|r| match r {
            Ok(_) => quote! {},
            Err(detloop) => {
                let msg = format!("Loop detected: {:?}", detloop);
                quote! {
                    std::compile_error!(#msg);
                }
            }
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
                .map(|dep| syn::Ident::new(dep, Span::call_site()))
                .fold(quote! {crate::ToQuery}, |acc, dep| quote! { #acc + #dep });

            deps = quote! {};
            let trait_id = syn::Ident::new(symbol, Span::call_site());

            if flags.with_helpers() {
                deps = quote! {
                    crate::helpers::#trait_id
                }
            }

            let body = if flags.with_blank_impl() {
                quote! {const IS_IMPL: bool;}
            } else {
                quote! {}
            };

            quote! {
                /// Symbol #symbol
                pub trait #trait_id
                where Self: Sized + crate::ToQuery + #deps
                //where Self: #deps
                {
                    #body
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

create_symbol_derivations! {}

#[proc_macro_derive(Either)]
pub fn derive_either(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident = input.ident; 

    if ident.to_string() != "Either" {
        return quote! {
            compile_error!("cannot derive other types than Either")
        }.into()
    }

    SYMBOL_MAP.entries()
    .filter(|(_, flags)| flags.with_either_impl())
    .map(|(&key, flags)| {
        let symbol_ident = Ident::new(key, Span::call_site());
        
        let body_impl = if flags.with_blank_impl() {
            quote! {const IS_IMPL: bool = true;}
        } else {
            quote!{}
        };

        quote! {
            impl<Lhs, Rhs> crate::grammar:: #symbol_ident for #ident<Lhs, Rhs>
                where 
                    Lhs: crate::grammar:: #symbol_ident, 
                    Rhs: crate::grammar:: #symbol_ident
            {
                #body_impl
            }
        }
    })
    .collect::<proc_macro2::TokenStream>()
    .into()
}

#[proc_macro_derive(Blank)]
pub fn derive_blank(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident = input.ident;

    if ident.to_string() != "Blank" {
        return quote! {
            compile_error!("cannot derive other types than Blank")
        }.into()
    }

    SYMBOL_MAP.entries().map(|(&key, flags)| {
        let symbol_ident = Ident::new(key, Span::call_site());
        
        if flags.with_blank_impl() {
            quote! {
                impl crate::grammar:: #symbol_ident for Blank
                {
                    const IS_IMPL: bool = false;
                }
            }
        } else {
            quote! {

            }
        }
    })
    .collect::<proc_macro2::TokenStream>()
    .into()
}

/// Creates a symbol derivation
fn derive_symbol(symbol: &str, ast: &DeriveInput) -> proc_macro2::TokenStream {
    fetch_deps(symbol)
        .chain([symbol])
        .map(|symbol| {
            impl_symbol_trait(
                symbol,
                ast,
                SYMBOL_MAP
                    .get(symbol)
                    .unwrap_or_else(|| panic!("missing symbol {symbol} in the map")),
            )
        })
        .collect()
}

fn impl_symbol_trait(symbol: &str, ast: &DeriveInput, def: &SymbolDef) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    let with_impl = match def.with_blank_impl() {
        true => quote! {const IS_IMPL: bool = true;},
        false => quote! {},
    };

    let trait_ident = Ident::new(
        symbol, 
        proc_macro2::Span::call_site()
    );

    quote! {
        impl #impl_generics crate::grammar:: #trait_ident for #name #type_generics #where_clause {
            #with_impl
        }
    }
}
