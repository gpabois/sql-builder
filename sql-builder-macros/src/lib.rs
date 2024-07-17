use std::default;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
use phf::phf_map;

#[proc_macro_derive(DerivedColumn)]
pub fn derive_derived_column(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("DerivedColumn", &input).into()
}

#[proc_macro_derive(SelectList)]
pub fn derive_select_list(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("SelectList", &input).into()
}

#[proc_macro_derive(Identifier)]
pub fn derive_identifier(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("Identifier", &input).into()
}

#[proc_macro_derive(SearchCondition)]
pub fn derive_search_condition(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("SearchCondition", &input).into()
}

#[proc_macro_derive(BooleanTerm)]
pub fn derive_boolean_term(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("BooleanTerm", &input).into()
}

#[proc_macro_derive(BooleanFactor)]
pub fn derive_boolean_factor(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("BooleanFactor", &input).into()
}

#[proc_macro_derive(BooleanTest)]
pub fn derive_boolean_test(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("BooleanTest", &input).into()
}

#[proc_macro_derive(BooleanPrimary)]
pub fn derive_boolean_primary(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derive_symbol("BooleanPrimary", &input).into()
}
struct SymbolFlags {
    with_impl: bool,
    inherits: &'static[&'static str]
}

static SYMBOL_MAP: phf::Map<&'static str, SymbolFlags> = phf_map! {
    "SelectList" => SymbolFlags {
        with_impl: true, 
        inherits: &[]
    },
    "DerivedColumn" => SymbolFlags{
        with_impl: false, 
        inherits: &["SelectList"]
    },
    "ColumnReference" => SymbolFlags {
        with_impl: false,
        inherits: &["ValueExpressionPrimary"]
    },
    "ColumnName" => SymbolFlags{
        with_impl: false, 
        inherits: &["ColumnReference"]
    },
    "TableReference" => SymbolFlags {
        with_impl: false,
        inherits: &[]
    },
    "TableName" => SymbolFlags {
        with_impl: false,
        inherits: &["TableReference", "Qualifier"],       
    },
    "Qualifier" => SymbolFlags {
        with_impl: false,
        inherits: &["ColumnReference", "SelectList"]
    },
    "ValueExpression" => SymbolFlags{
        with_impl: false, 
        inherits: &["DerivedColumn"]
    },
    "NumericValueExpression" => SymbolFlags{
        with_impl: false, 
        inherits: &["ValueExpression"]
    },
    "StringValueExpression" => SymbolFlags{
        with_impl: false, 
        inherits: &["ValueExpression"]
    },
    "DateTimeValueExpression" => SymbolFlags{
        with_impl: false, 
        inherits: &["ValueExpression"]
    },
    "IntervalValueExpression" => SymbolFlags{with_impl: false, inherits: &["ValueExpression"]},

    "Term" => SymbolFlags{with_impl: false, inherits: &["NumericValueExpression"]},
    "Factor" => SymbolFlags{with_impl: false, inherits: &["Term"]},
    "NumericPrimary" => SymbolFlags{with_impl: false, inherits: &["Factor"]},
    "ValueExpressionPrimary" => SymbolFlags{with_impl: false, inherits: &["NumericPrimary"]},

    "QualifiedName" => SymbolFlags{
        with_impl: false, 
        inherits: &["TableName"]
    },
    "QualifiedIdentifier" => SymbolFlags{
        with_impl: false, 
        inherits: &["QualifiedName"]
    },
    "Identifier" => SymbolFlags{
        with_impl: false, 
        inherits: &[
            "QualifiedIdentifier", 
            "ColumnName"
        ]
    },
    "SearchCondition" => SymbolFlags {
        with_impl: false, 
        inherits: &[]   
    },
    "BooleanTerm" => SymbolFlags {
        with_impl: false,
        inherits: &["SearchCondition"]
    },
    "BooleanFactor" => SymbolFlags {
        with_impl: false,
        inherits: &["BooleanTerm"]
    },
    "BooleanTest" => SymbolFlags {
        with_impl: false,
        inherits: &["BooleanFactor"]
    },
    "BooleanPrimary" => SymbolFlags {
        with_impl: false,
        inherits: &["BooleanTest", "SearchCondition"]
    },

    "Predicate" => SymbolFlags {
        with_impl: false,
        inherits: &["BooleanPrimary"]
    },

    "ComparisonPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "BetweenPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "InPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "LikePredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "NullPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "ExistsPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "MatchPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
    "OverlapsPredicate" => SymbolFlags {
        with_impl: false,
        inherits: &["Predicate"]
    },
};

fn fetch_impl(symbol: &str) -> Vec<&str> {
    let mut stack = vec![symbol];
    let mut symbols = Vec::<&'static str>::default();
    while let Some(symbol) = stack.pop() {
        if symbols.contains(&symbol) {
            continue;
        }
        symbols.push(symbol);
        stack.extend(SYMBOL_MAP[symbol].inherits);
    }
    symbols
}

fn derive_symbol(symbol: &str, ast: &DeriveInput) -> proc_macro2::TokenStream {
    fetch_impl(symbol).iter().map(|symbol| 
        impl_symbol_trait(
            symbol, 
            ast, 
            SYMBOL_MAP
                .get(symbol)
                .expect(&format!("missing symbol {symbol} in the map"))
        )
    ).collect()
}

fn impl_symbol_trait(symbol: &str, ast: &DeriveInput, flags: &SymbolFlags) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    let with_impl = match flags.with_impl {
        true => quote! {const IS_IMPL: bool = true;},
        false => quote!{}
    };

    let trait_ident = Ident::new(symbol, proc_macro2::Span::call_site());
    quote! {
        impl #impl_generics crate::traits:: #trait_ident for #name #type_generics #where_clause {
            #with_impl
        }
    }
}