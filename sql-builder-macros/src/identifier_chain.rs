use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use regex::Regex;
use syn::{parse::Parse, punctuated::Punctuated, Expr, Token};

fn check_if_valid_identifer(id: &str) -> bool {
    let re = Regex::new("^[A-Za-z_]([A-Za-z0-9_])*").unwrap();

    re.is_match(id)
}

/// A SQL identifier.
pub struct Identifier(syn::Ident);

impl Parse for Identifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(syn::Ident::parse(input)?))
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tok = self.0.to_string();
        tokens.extend(quote! {::sql_builder::id(#tok)});
    }
}

/// A chain of identifiers punct-separated.
pub struct IdentifierChain(Vec<Identifier>);

impl Parse for IdentifierChain {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let list = Punctuated::<Identifier, Token![.]>::parse_terminated(input)?;
        Ok(Self(list.into_iter().collect()))
    }
}

impl ToTokens for IdentifierChain {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if self.0.len() == 1 {
            let lhs = self.0.first().unwrap();
            tokens.extend(quote! {#lhs});
        } else if self.0.len() > 1 {
            let lhs = self.0.first().unwrap();
            let rhs = self.0[1..]
                .iter()
                .map(|b| quote! {.add_identifier(#b)})
                .collect::<TokenStream>();

            tokens.extend(quote! {
                #lhs #rhs
            })
        }
    }
}
