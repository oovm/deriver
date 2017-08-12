use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::helpers::{WrapperKind, WrapperType};
use syn::{Error, Fields, ItemEnum};

pub enum FromBuilder {
    Enum(EnumContext),
}

pub struct EnumContext {
    name: Ident,
    variants: Vec<VariantContext>,
}

pub struct VariantContext {
    name: Ident,
    typing: WrapperType,
}

impl Parse for FromBuilder {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        if let Ok(o) = input.parse::<ItemEnum>() {
            let mut variants = Vec::new();
            for term in o.variants {
                match term.fields {
                    Fields::Named(_) => {}
                    Fields::Unnamed(tuple) => {
                        let mut tuples = tuple.unnamed.iter();
                        let first = tuples.next();
                        let first = match first {
                            Some(s) => s,
                            None => {
                                return Err(Error::new(input.span(), "Expected single field enum"));
                            }
                        };
                        if let Some(_) = tuples.next() {
                            return Err(Error::new(input.span(), "Expected single field enum"));
                        }
                        let typing = WrapperType::new(&first.ty)?;
                        variants.push(VariantContext { name: term.ident, typing })
                    }
                    Fields::Unit => {}
                }
            }
            return Ok(FromBuilder::Enum(EnumContext { name: o.ident, variants }));
        }
        Err(Error::new(input.span(), "Expected enum"))
    }
}

impl ToTokens for FromBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FromBuilder::Enum(o) => {
                let type_out = &o.name;
                for i in &o.variants {
                    let field = &i.name;
                    let type_in = &i.typing.typing;
                    match &i.typing.kind {
                        WrapperKind::Normal => {
                            tokens.extend(quote! {
                                impl From<#type_in> for #type_out {
                                    fn from(o: #type_in) -> Self {
                                        Self::#field(o)
                                    }
                                }
                            });
                        }
                        WrapperKind::Option => {
                            tokens.extend(quote! {
                                impl From<#type_in> for #type_out {
                                    fn from(o: #type_in) -> Self {
                                        Self::#field(Some(o))
                                    }
                                }
                            });
                        }
                        WrapperKind::Boxed => {
                            tokens.extend(quote! {
                                impl From<#type_in> for #type_out {
                                    fn from(o: #type_in) -> Self {
                                        Self::#field(Box::new(o))
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
    }
}
