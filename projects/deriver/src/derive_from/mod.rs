use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser, ParseStream};
use syn::token::Enum;
use syn::{Fields, ItemEnum, PathArguments, PathSegment, Type, TypePath};


pub enum FromBuilder {
    Enum(EnumContext),
}


pub struct EnumContext {
    name: Ident,
    variants: Vec<VariantContext>,
}

pub struct VariantContext {
    name: Ident,
    kind: VariantKind,
    typing: Type,
}

pub enum VariantKind {
    Normal,
    Boxed,
}

impl Parse for FromBuilder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(o) = input.parse::<ItemEnum>() {
            let mut variants = Vec::new();
            for term in o.variants {
                match term.fields {
                    Fields::Named(_) => {}
                    Fields::Unnamed(tuple) => {
                        let mut tuples = tuple.unnamed.iter();
                        let first = tuples.next();
                        let first = match first {
                            Some(s) => { s }
                            None => {
                                return Err(syn::Error::new(
                                    input.span(),
                                    "Expected single field enum",
                                ));
                            }
                        };
                        if let Some(_) = tuples.next() {
                            return Err(syn::Error::new(
                                input.span(),
                                "Expected single field enum",
                            ));
                        }
                        let (typing, kind) = match &first.ty {
                            Type::Path(p) => {
                                let mut name = vec![];
                                for segment in p.path.segments.iter() {
                                    name.push_str(segment.ident.to_string());
                                }
                                let name = name.join("::");
                                let kind = match name.as_str() {
                                    "Box" => (first.ty.clone(), VariantKind::Normal),
                                    _ => (first.ty.clone(), VariantKind::Normal),
                                };


                                match p.path.segments.last() {
                                    None => {
                                        Err(syn::Error::new(
                                            input.span(),
                                            "Expected single field enum",
                                        ))
                                    }
                                    Some(s) => {
                                        match &s.arguments {
                                            PathArguments::None => {}
                                            PathArguments::AngleBracketed(generic) => {
                                                generic.args
                                            }
                                            PathArguments::Parenthesized(_) => {}
                                        }
                                    }
                                }
                            }
                            _ => (first.ty.clone(), VariantKind::Normal)
                        };

                        variants.push(VariantContext { name: term.ident, kind, typing })
                    }
                    Fields::Unit => {}
                }
            }
            return Ok(FromBuilder::Enum(EnumContext { name: o.ident, variants }));
        }
        Err(syn::Error::new(input.span(), "Expected enum"))
    }
}


fn get_typing(typing: &Type) -> (Type, VariantKind) {
    match typing {
        Type::Path(p) => {
            let mut name = vec![];
            for segment in p.path.segments.iter() {
                name.push_str(segment.ident.to_string());
            }
            let name = name.join("::");
            let kind = match name.as_str() {
                "Box" => (p.path.clone(), VariantKind::Boxed),
                _ => (p.path.clone(), VariantKind::Normal),
            };
            (p.path.clone(), kind)
        }
        _ => (typing.clone(), VariantKind::Normal)
    }
}


impl ToTokens for FromBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FromBuilder::Enum(o) => {
                let name = &o.name;
                tokens.extend(quote! {
                    impl From<#name> for Lisp {
                        fn from(value: #name) -> Self {
                            Lisp::Symbol(value.to_string())
                        }
                    }
                });
            }
        }
    }
}