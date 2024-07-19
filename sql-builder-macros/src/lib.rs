use phf::phf_map;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
use paste::paste;

macro_rules! impl_symbol_derivation {
    ($symbol:ident) => {
        paste!{
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

impl_symbol_derivation!(DerivedColumn);
impl_symbol_derivation!(SelectList);

impl_symbol_derivation!(SearchCondition);
impl_symbol_derivation!(BooleanTerm);
impl_symbol_derivation!(BooleanFactor);
impl_symbol_derivation!(BooleanTest);
impl_symbol_derivation!(BooleanPrimary);
impl_symbol_derivation!(ComparisonPredicate);

impl_symbol_derivation!(NumericValueExpression);
impl_symbol_derivation!(Term);
impl_symbol_derivation!(Factor);

impl_symbol_derivation!(SchemaName);
impl_symbol_derivation!(QualifiedName);
impl_symbol_derivation!(QualifiedIdentifier);
impl_symbol_derivation!(Identifier);

impl_symbol_derivation!(Literal);

struct SymbolFlags {
    with_impl: bool,
    inherits: &'static [&'static str],
}

/// Defines the way how symbols are derived.
static SYMBOL_MAP: phf::Map<&'static str, SymbolFlags> = phf_map! {
    "TableExpression" => SymbolFlags {
        with_impl: false,
        inherits: &[]
    },
    "FromClause" => SymbolFlags {
        with_impl: false,
        inherits: &[]
    },
    "WhereClause" => SymbolFlags {
        with_impl: true,
        inherits: &[]
    },
    "GroupByClause" => SymbolFlags {
        with_impl: true,
        inherits: &[]
    },
    "HavingClause" => SymbolFlags {
        with_impl: true,
        inherits: &[]
    },
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
    "TableReferenceList" => SymbolFlags {
        with_impl: false,
        inherits: &[]
    },
    "TableReference" => SymbolFlags {
        with_impl: false,
        inherits: &["TableReferenceList"]
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
        inherits: &["DerivedColumn", "RowValueConstructorElement"]
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
    "ValueExpressionPrimary" => SymbolFlags{
        with_impl: false, 
        inherits: &["NumericPrimary"]
    },
    "ValueSpecification" => SymbolFlags{
        with_impl: false, 
        inherits: &["ValueExpressionPrimary"]
    },
    "Literal" => SymbolFlags{
        with_impl: false, 
        inherits: &["ValueSpecification"]
    },
    "SchemaName" => SymbolFlags{
        with_impl: false,
        inherits: &[]
    },
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
        inherits: &["BooleanTest"]
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

    "RowValueConstructor" => SymbolFlags {
        with_impl: false,
        inherits: &[]
    }, 
    "RowValueConstructorList" => SymbolFlags {
        with_impl: false,
        inherits: &["RowValueConstructor"]
    }, 
    "RowValueConstructorElement" => SymbolFlags {
        with_impl: false,
        inherits: &["RowValueConstructor", "RowValueConstructorList"]
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
    fetch_impl(symbol)
        .iter()
        .map(|symbol| {
            impl_symbol_trait(
                symbol,
                ast,
                SYMBOL_MAP
                    .get(symbol)
                    .expect(&format!("missing symbol {symbol} in the map")),
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

    let with_impl = match flags.with_impl {
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

