use syn::{spanned::Spanned, Error, GenericArgument, PathArguments, Type};

pub enum WrapperKind {
    Normal,
    Option,
    Boxed,
}

pub struct WrapperType {
    pub kind: WrapperKind,
    pub typing: Type,
}

impl WrapperType {
    pub fn normal(typing: Type) -> Self {
        Self { kind: WrapperKind::Normal, typing }
    }
}

impl WrapperType {
    pub fn new(typing: &Type) -> Result<Self, Error> {
        match typing {
            Type::Path(p) => {
                let mut name = vec![];
                for segment in p.path.segments.iter() {
                    name.push(segment.ident.to_string());
                }
                let name = name.join("::");
                let kind = match name.as_str() {
                    "Option" => WrapperKind::Option,
                    "Box" | "alloc::boxed::Box" => WrapperKind::Boxed,
                    _ => return Ok(WrapperType::normal(typing.clone())),
                };
                let typing = match p.path.segments.last() {
                    None => Err(Error::new(typing.span(), "Expected single field enum"))?,
                    Some(s) => match &s.arguments {
                        PathArguments::AngleBracketed(generic) => {
                            let mut types = generic.args.iter();
                            let first = types.next();
                            let first = match first {
                                Some(s) => match s {
                                    GenericArgument::Type(t) => t.clone(),
                                    _ => Err(Error::new(typing.span(), "Expected single field enum"))?,
                                },
                                None => Err(Error::new(typing.span(), "Expected single field enum"))?,
                            };
                            if let Some(_) = types.next() {
                                Err(Error::new(typing.span(), "Expected single field enum"))?
                            }
                            first
                        }
                        _ => Err(Error::new(typing.span(), "Expected single field enum"))?,
                    },
                };
                Ok(Self { kind, typing })
            }
            _ => Ok(Self::normal(typing.clone())),
        }
    }
}
