use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::Enum;

pub enum FromBuilder {
    Enum(EnumFromContext),
}


pub struct EnumFromContext {}

impl Parse for FromBuilder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(o) = input.parse::<Enum>() {


            return Ok(FromBuilder::Enum(EnumFromContext {}))
        }
        Err(syn::Error::new(input.span(), "Expected enum"))
    }
}


impl ToTokens for FromBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}