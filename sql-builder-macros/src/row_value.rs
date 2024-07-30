use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, Expr, Token};

pub struct RowValue(Vec<syn::Expr>);

impl Parse for RowValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let list = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        Ok(Self(list.into_iter().collect()))
    }
}

impl ToTokens for RowValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.0.len() == 1 {
            let lhs = self.0.first().unwrap();
            tokens.extend(quote! {#lhs});
        } else if self.0.len() > 1 {
            let lhs = self.0.first().unwrap();
            let rhs = self.0[1..]
                .iter()
                .map(|b| quote! {.add_row_element(#b)})
                .collect::<TokenStream>();

            tokens.extend(quote! {
                #lhs #rhs.into_row_value()
            })
        }
    }
}
