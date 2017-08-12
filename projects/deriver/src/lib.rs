#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod derive_from;
mod helpers;

use quote::ToTokens;
use syn::parse_macro_input;
use crate::derive_from::FromBuilder;

use proc_macro::TokenStream;

/// Derives the `From` trait for a type.
#[proc_macro_derive(From)]
pub fn derive_from_traits(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as FromBuilder);
    input.to_token_stream().into()
}