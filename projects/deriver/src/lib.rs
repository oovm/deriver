#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod derive_display;
mod derive_from;
mod helpers;

use crate::{derive_display::DisplayBuilder, derive_from::FromBuilder};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

/// Derives the `From` trait for a type.
///
/// ## Example
///
/// Derived from:
///
/// ```
/// #[derive(From)]
/// enum Test {
///     A(String),
///     B(Option<String>),
///     C(Box<String>),
/// }
/// ```
///
/// Equivalent to:
///
/// ```
/// # enum Test {
/// #    A(String),
/// #    B(Option<String>),
/// #    C(Box<String>),
/// # }
///
/// impl From<String> for Test {
///     fn from(o: String) -> Self {
///         Self::A(o)
///     }
/// }
/// impl From<String> for Test {
///     fn from(o: String) -> Self {
///         Self::B(Some(o))
///     }
/// }
/// impl From<String> for Test {
///     fn from(o: String) -> Self {
///         Self::C(Box::new(o))
///     }
/// }
/// ```
#[proc_macro_derive(From, attributes(from))]
pub fn derive_from_traits(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as FromBuilder);
    input.to_token_stream().into()
}

/// Derives the `Display` trait for a type.
#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display_traits(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DisplayBuilder);
    input.to_token_stream().into()
}
