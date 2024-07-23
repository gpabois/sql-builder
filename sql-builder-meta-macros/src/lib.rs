use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use sql_builder_def::*;

#[proc_macro]
/// Create symbol derivations
pub fn create_symbol_derivations(_: TokenStream) -> TokenStream {
    SYMBOL_MAP.keys().copied().map(|symbol| {
        let ident = syn::Ident::new(symbol, Span::call_site());
        quote!{
            create_symbol_derivation!{#ident}
        }
    }).collect::<proc_macro2::TokenStream>().into()
}