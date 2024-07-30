mod column_name_list;
mod row_value;
mod select_sublist;

use column_name_list::ColumnNameList;
use itertools::Itertools;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::{quote, ToTokens as _};
use regex::Regex;
use row_value::RowValue;
use select_sublist::SelectSublist;
use sql_builder_def::{detect_loop, fetch_deps, SymbolDef, SYMBOL_MAP};
use sql_builder_meta_macros::create_symbol_derivations;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro]
/// Creates an SQL identifier.
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
/// Created a bound dynamic parameter
pub fn bind(input: TokenStream) -> TokenStream {
    let expr: syn::Expr = parse_macro_input!(input);
    quote! {::sql_builder::bind(#expr)}.into()
}

#[proc_macro]
/// Creates an SQL literal.
pub fn lit(input: TokenStream) -> TokenStream {
    let lit: syn::Lit = parse_macro_input!(input);
    match lit {
        syn::Lit::Str(lit) => quote! {
            sql_builder::char_str_lit(#lit)
        },
        syn::Lit::Float(lit) => {
            if let Ok(double) = lit.base10_parse::<f64>() {
                if double.is_sign_positive() {
                    quote! {
                        sql_builder::unsigned_numeric_lit(#double)
                    }
                } else {
                    quote! {
                        sql_builder::signed_numeric_lit(#double)
                    }
                }
            } else {
                unreachable!()
            }
        }
        syn::Lit::Int(lit) => {
            if let Ok(unsigned_int) = lit.base10_parse::<u64>() {
                quote! {
                    sql_builder::unsigned_numeric_lit(#unsigned_int)
                }
            } else if let Ok(signed_int) = lit.base10_parse::<i64>() {
                quote! {
                    sql_builder::signed_numeric_lit(#signed_int)
                }
            } else {
                unreachable!()
            }
        }
        _ => quote! {
            compile_error!("literal not implemented yet")
        },
    }
    .into()
}
#[proc_macro]
/// Construct a row value based on a list of element.
pub fn row_value(input: TokenStream) -> TokenStream {
    let value: RowValue = parse_macro_input!(input);
    value.to_token_stream().into()
}

#[proc_macro]
/// Generates a list of column names.
pub fn columns(input: TokenStream) -> TokenStream {
    let columns: ColumnNameList = parse_macro_input!(input);
    columns.to_token_stream().into()
}

#[proc_macro]
/// Generates a select list.
pub fn select_columns(input: TokenStream) -> TokenStream {
    let sublist: SelectSublist = parse_macro_input!(input);
    sublist.to_token_stream().into()
}

#[proc_macro]
/// Checks if the implemented grammar has loops.
/// Used for debugging.
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
                .fold(
                    quote! {Sized + crate::Symbol},
                    |acc, dep| quote! { #acc + #dep },
                );

            let trait_id = syn::Ident::new(symbol, Span::call_site());

            if flags.with_helpers() {
                deps = quote! {
                    #deps + crate::helpers::#trait_id
                }
            }

            let body = if flags.with_blank_impl() {
                quote! {const IS_IMPL: bool;}
            } else {
                quote! {}
            };

            quote! {
                #[doc = #symbol]
                pub trait #trait_id : #deps
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

    if ident != "Either" {
        return quote! {
            compile_error!("cannot derive other types than Either")
        }
        .into();
    }

    let code = SYMBOL_MAP
        .entries()
        .map(|(&key, flags)| {
            let symbol_ident = Ident::new(key, Span::call_site());

            let body_impl = if flags.with_blank_impl() {
                quote! {const IS_IMPL: bool = true;}
            } else {
                quote! {}
            };

            let mut tokens = quote! {
                impl<Lhs, Rhs> crate::grammar:: #symbol_ident for #ident<Lhs, Rhs>
                    where
                        Lhs: crate::grammar:: #symbol_ident,
                        Rhs: crate::grammar:: #symbol_ident
                {
                    #body_impl
                }


            };

            if flags.with_helpers() && !flags.helpers_require_methods_implementations() {
                tokens = quote! {
                    #tokens

                    impl<Lhs, Rhs> crate::helpers:: #symbol_ident for #ident<Lhs, Rhs>
                        where
                            Lhs: crate::grammar:: #symbol_ident,
                            Rhs: crate::grammar:: #symbol_ident
                    {}
                }
            }

            tokens
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        #code
        impl<Lhs, Rhs> crate::Symbol for Either<Lhs, Rhs> {}
    }
    .into()
}

#[proc_macro_derive(Blank)]
pub fn derive_blank(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let ident = input.ident;

    if ident != "Blank" {
        return quote! {
            compile_error!("cannot derive other types than Blank")
        }
        .into();
    }

    SYMBOL_MAP
        .keys()
        .copied()
        .filter(|symbol| {
            let flags = &SYMBOL_MAP[*symbol];
            flags.with_blank_impl()
        })
        .flat_map(|symbol| fetch_deps(symbol).chain([symbol]))
        .unique()
        .map(|symbol| (symbol, &SYMBOL_MAP[symbol]))
        .map(|(key, flags)| {
            let symbol_ident = Ident::new(key, Span::call_site());

            let body = if flags.with_blank_impl() {
                quote! {const IS_IMPL: bool = false;}
            } else {
                quote! {}
            };

            let mut tokens = quote! {
                impl crate::grammar::#symbol_ident for Blank
                {
                    #body
                }
            };

            if flags.with_helpers() && !flags.helpers_require_methods_implementations() {
                tokens = quote! {
                    #tokens

                    impl crate::helpers::#symbol_ident for Blank
                    {}

                }
            }

            tokens
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

/// Creates a symbol derivation
fn derive_symbol(symbol: &str, ast: &DeriveInput) -> proc_macro2::TokenStream {
    let impls = fetch_deps(symbol)
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
        .collect::<proc_macro2::TokenStream>();

    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    quote! {
        #impls

        impl #impl_generics crate::Symbol for #name #type_generics #where_clause {}

    }
}

fn impl_symbol_trait(symbol: &str, ast: &DeriveInput, def: &SymbolDef) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    let with_impl = match def.with_blank_impl() {
        true => quote! {const IS_IMPL: bool = true;},
        false => quote! {},
    };

    let trait_ident = Ident::new(symbol, proc_macro2::Span::call_site());

    let mut tokens = quote! {
        impl #impl_generics crate::grammar:: #trait_ident for #name #type_generics #where_clause {
            #with_impl
        }
    };

    // Automatic helper trait implementation.
    if def.with_helpers() && !def.helpers_require_methods_implementations() {
        tokens = quote! {
            #tokens

            impl #impl_generics crate::helpers:: #trait_ident for #name #type_generics #where_clause {}
        };
    }

    tokens
}
