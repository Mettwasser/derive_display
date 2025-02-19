mod display;

use manyhow::manyhow;
use proc_macro::TokenStream;
use syn::{parse, DeriveInput};

#[manyhow]
#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(tokens: TokenStream) -> syn::Result<TokenStream> {
    let input = parse::<DeriveInput>(tokens)?;
    Ok(display::expand(&input)?.into())
}
