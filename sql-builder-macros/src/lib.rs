use paste::paste;
use phf::phf_map;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro]
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
pub fn create_symbol_traits(_: TokenStream) -> TokenStream {
    SYMBOL_MAP
        .entries()
        .map(|(symbol, flags)| {
            let mut deps = fetch_deps(symbol)
                .into_iter()
                .map(|dep| syn::Ident::new(dep, Span::call_site()))
                .fold(quote! {crate::ToQuery}, |acc, dep| quote! { #acc + #dep });

            let trait_id = syn::Ident::new(symbol, Span::call_site());

            if flags.with_helpers {
                deps = quote! {#deps + crate::helpers::#trait_id}
            }

            let body = if flags.with_blank_impl {
                quote! {const IS_IMPL: bool;}
            } else {
                quote! {}
            };

            quote! {
                /// Symbol #symbol
                pub trait #trait_id: #deps {
                    #body
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

macro_rules! impl_symbol_derivation {
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

impl_symbol_derivation!(Select);
impl_symbol_derivation!(TableExpression);
impl_symbol_derivation!(TableReferenceList);

impl_symbol_derivation!(FromClause);
impl_symbol_derivation!(WhereClause);
impl_symbol_derivation!(GroupByClause);
impl_symbol_derivation!(HavingClause);

impl_symbol_derivation!(FromDefault);

impl_symbol_derivation!(DerivedColumn);
impl_symbol_derivation!(SelectList);

impl_symbol_derivation!(SearchCondition);
impl_symbol_derivation!(BooleanTerm);
impl_symbol_derivation!(BooleanFactor);
impl_symbol_derivation!(BooleanTest);
impl_symbol_derivation!(BooleanPrimary);
impl_symbol_derivation!(ComparisonPredicate);

impl_symbol_derivation!(ContextuallyTypedRowValueExpressionList);
impl_symbol_derivation!(ContextuallyTypedRowValueConstructorElementList);

impl_symbol_derivation!(NumericValueExpression);
impl_symbol_derivation!(Term);
impl_symbol_derivation!(Factor);

impl_symbol_derivation!(SchemaName);
impl_symbol_derivation!(UnqualifiedSchemaName);

impl_symbol_derivation!(QualifiedName);

impl_symbol_derivation!(QualifiedIdentifier);
impl_symbol_derivation!(Identifier);

impl_symbol_derivation!(Literal);


fn derive_symbol(symbol: &str, ast: &DeriveInput) -> proc_macro2::TokenStream {
    fetch_deps(symbol)
        .iter()
        .map(|symbol| format!("crate::grammar::{}", symbol))
        .chain([format!("crate::helpers::{}", symbol)])
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
    flags: &SymbolFlags,
) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    let with_impl = match flags.with_blank_impl {
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
