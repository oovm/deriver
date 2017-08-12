use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub enum DisplayBuilder {
    Enum,
}

impl Parse for DisplayBuilder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DisplayBuilder::Enum)
    }
}

impl ToTokens for DisplayBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}
