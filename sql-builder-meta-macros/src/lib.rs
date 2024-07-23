use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use sql_builder_def::*;

#[proc_macro]
/// Create symbol derivations
pub fn create_symbol_derivations(_: TokenStream) -> TokenStream {
    SYMBOL_MAP
        .keys()
        .copied()
        .map(|symbol| {
            let symbol_ident = syn::Ident::new(symbol, Span::call_site());
            let derive_ident = syn::Ident::new(
                &format!("derive_{}", symbol.to_case(convert_case::Case::Snake)),
                Span::call_site(),
            );

            quote! {
                #[proc_macro_derive(#symbol_ident)]
                pub fn #derive_ident(input: TokenStream) -> TokenStream {
                    let input: DeriveInput = parse_macro_input!(input);
                    derive_symbol(#symbol, &input).into()
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
