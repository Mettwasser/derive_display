mod impl_enum;
mod impl_struct;
mod util;

use {impl_enum::parse_enum, impl_struct::parse_struct};

use darling::Error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: &DeriveInput) -> darling::Result<TokenStream> {
    let parsed = match &input.data {
        syn::Data::Enum(_e) => parse_enum(input),
        syn::Data::Struct(_s) => parse_struct(input),
        syn::Data::Union(_u) => Err(Error::custom("Unions are not supported!")),
    }?;

    Ok(quote! {
        #parsed
    })
}
