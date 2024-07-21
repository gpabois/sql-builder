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

struct SymbolFlags {
    with_helpers: bool,
    with_blank_impl: bool,
    deps: &'static [&'static str],
}

impl SymbolFlags {
    #[inline]
    pub const fn new(
        deps: &'static [&'static str],
        with_blank_impl: bool,
        with_helpers: bool,
    ) -> Self {
        Self {
            with_helpers,
            with_blank_impl,
            deps,
        }
    }

    pub const fn new_with_impl(deps: &'static [&'static str]) -> Self {
        Self {
            with_blank_impl: true,
            with_helpers: false,
            deps,
        }
    }
}

/// Defines the way how symbols are derived.
static SYMBOL_MAP: phf::Map<&'static str, SymbolFlags> = phf_map! {
    "Asterisk" => SymbolFlags::new(&[], false, false),
    "QualifiedAsterisk" => SymbolFlags::new(&[], false, false),

    "Select" => SymbolFlags::new(&[], false, true),
    "TableExpression" => SymbolFlags::new(&[], false, true),
    "FromClause" => SymbolFlags::new(&[], false, true),
    "WhereClause" => SymbolFlags::new_with_impl(&[]),
    "GroupByClause" => SymbolFlags::new_with_impl(&[]),
    "HavingClause" => SymbolFlags::new_with_impl(&[]),
    "SelectList" => SymbolFlags::new_with_impl(&["SelectSublist", "Asterisk"]),
    "SelectSublist" => SymbolFlags::new_with_impl(&["DerivedColumn", "QualifiedAsterisk"]),
    "DerivedColumn" => SymbolFlags::new(&["ValueExpression"]),

    "ValueExpression" => SymbolFlags::new(&["CommonValueExpression", "BooleanValueExpression", "RowValueExpression"]),
    "CommonValueExpression" => SymbolFlags::new(&[
        "NumericValueExpression",
        "StringValueExpression",
        "DatetimeValueExpression",
        "IntervalValueExpression",
        "UserDefinedTypeValueExpression",
        "ReferenceValueExpression",
        "CollectionValueExpression"
    ]),
    "NumericValueExpression" => SymbolFlags::new(&["Term"]),
    "StringValueExpression" => SymbolFlags::new(&[]),
    "DatetimeValueExpression" => SymbolFlags::new(&[]),
    "IntervalValueExpression" => SymbolFlags::new(&[]),
    "UserDefinedTypeValueExpression" => SymbolFlags::new(&[]),
    "ReferenceValueExpression" => SymbolFlags::new(&[]),
    "CollectionValueExpression" => SymbolFlags::new(&[]),

    "Term" => SymbolFlags::new(&["NumericValueExpression"]),
    "Factor" => SymbolFlags::new(&["NumericPrimary"]),
    "NumericPrimary" => SymbolFlags::new(&["ValueExpressionPrimary", "NumericValueFunction"]),
    "ValueExpressionPrimary" => SymbolFlags::new(&["ParenthesizedValueExpression", "NonParenthesizedValueExpressionPrimary"]),
    // Need to call to_parenthesized_value_expression on ValueExpression
    "ParenthesizedValueExpression" => SymbolFlags::new(&[]),
    "NonParenthesizedValueExpressionPrimary" => SymbolFlags::new(&[
        "UnsignedValueSpecification",
        "ColumnReference",
        "SetFunctionReference",
        "WindowFunction",
        "ScalarSubquery",
        "CaseExpression",
        "CastSpecification",
        "FieldReference",
        "SubtypeTreatment",
        "MethodInvocation",
        "StaticMethodInvocation",
        "NewSpecification",
        "AttributeOrMethodReference",
        "ReferenceResolution",
        "CollectionValueConstructor",
        "ArrayElementReference",
        "MultisetElementReference",
        "RoutineInvocation",
        "NextValueExpression"
    ]),

    "ColumnReference" => SymbolFlags {
        with_blank_impl: false,
        deps: &["ValueExpressionPrimary"]
    },
    "ColumnName" => SymbolFlags{
        with_blank_impl: false,
        deps: &["ColumnReference"]
    },
    "TableReferenceList" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "TableReference" => SymbolFlags {
        with_blank_impl: false,
        deps: &["TableReferenceList"]
    },
    "TableName" => SymbolFlags {
        with_blank_impl: false,
        deps: &["TableReference", "Qualifier", "InsertionTarget"],
    },
    "Qualifier" => SymbolFlags {
        with_blank_impl: false,
        deps: &["ColumnReference", "SelectList"]
    },
    "Insert" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "InsertionTarget" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "InsertColumnsAndSources" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "FromSubQuery" => SymbolFlags {
        with_blank_impl: false,
        deps: &["InsertColumnsAndSources"]
    },
    "FromConstructor" => SymbolFlags {
        with_blank_impl: false,
        deps: &["InsertColumnsAndSources"]
    },
    "FromDefault" => SymbolFlags {
        with_blank_impl: false,
        deps: &["InsertColumnsAndSources"]
    },
    "ContextuallyTypedTableValueConstructor" => SymbolFlags {
        with_blank_impl: false,
        deps: &["FromConstructor"]
    },
    "ContextuallyTypedRowValueExpressionList" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "ContextuallyTypedRowValueExpression" => SymbolFlags {
        with_blank_impl: false,
        deps: &["ContextuallyTypedRowValueExpressionList"]
    },
    "ContextuallyTypedRowValueConstructor" => SymbolFlags {
        with_blank_impl: false,
        deps: &["ContextuallyTypedRowValueExpression"]
    },
    "ContextuallyTypedRowValueConstructorElementList" => SymbolFlags {
        with_blank_impl: false,
        deps: &[
            "ContextuallyTypedRowValueConstructor"
        ]
    },
    "ContextuallyTypedRowValueConstructorElement" => SymbolFlags {
        with_blank_impl: false,
        deps: &[
            "ContextuallyTypedRowValueConstructor",
            "ContextuallyTypedRowValueConstructorElementList"
        ]
    },
       "ValueSpecification" => SymbolFlags{
        with_blank_impl: false,
        deps: &["ValueExpressionPrimary"]
    },
    "Literal" => SymbolFlags{
        with_blank_impl: false,
        deps: &["ValueSpecification"]
    },
    "SchemaName" => SymbolFlags{
        with_blank_impl: false,
        deps: &[]
    },
    "UnqualifiedSchemaName" => SymbolFlags{
        with_blank_impl: false,
        deps: &["SchemaName"]
    },
    "QualifiedName" => SymbolFlags{
        with_blank_impl: false,
        deps: &["TableName"]
    },
    "QualifiedIdentifier" => SymbolFlags{
        with_blank_impl: false,
        deps: &["QualifiedName"]
    },
    "Identifier" => SymbolFlags{
        with_blank_impl: false,
        deps: &[
            "QualifiedIdentifier",
            "ColumnName"
        ]
    },
    "SearchCondition" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "BooleanTerm" => SymbolFlags {
        with_blank_impl: false,
        deps: &["SearchCondition"]
    },
    "BooleanFactor" => SymbolFlags {
        with_blank_impl: false,
        deps: &["BooleanTerm"]
    },
    "BooleanTest" => SymbolFlags {
        with_blank_impl: false,
        deps: &["BooleanFactor"]
    },
    "BooleanPrimary" => SymbolFlags {
        with_blank_impl: false,
        deps: &["BooleanTest"]
    },

    "Predicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["BooleanPrimary"]
    },

    "ComparisonPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "BetweenPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "InPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "LikePredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "NullPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "ExistsPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "MatchPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },
    "OverlapsPredicate" => SymbolFlags {
        with_blank_impl: false,
        deps: &["Predicate"]
    },

    "RowValueConstructor" => SymbolFlags {
        with_blank_impl: false,
        deps: &[]
    },
    "RowValueConstructorList" => SymbolFlags {
        with_blank_impl: false,
        deps: &["RowValueConstructor"]
    },
    "RowValueConstructorElement" => SymbolFlags {
        with_blank_impl: false,
        deps: &["RowValueConstructor", "RowValueConstructorList"]
    },

};

fn fetch_deps(symbol: &str) -> Vec<&str> {
    let mut stack = vec![symbol];
    let mut symbols = Vec::<&'static str>::default();
    while let Some(symbol) = stack.pop() {
        if symbols.contains(&symbol) {
            continue;
        }
        symbols.push(symbol);

        stack.extend(SYMBOL_MAP.keys().find(move |key| {
            let flags = &SYMBOL_MAP[**key];
            flags.deps.contains(&symbol)
        }));
    }
    symbols
}

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
