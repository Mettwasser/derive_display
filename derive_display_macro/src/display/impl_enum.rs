use {
    super::util::parse_display_attribute,
    darling::{
        ast::{self, Style},
        util, Error, FromDeriveInput, FromVariant,
    },
    proc_macro2::TokenStream,
    quote::quote,
    std::str::FromStr,
    syn::{
        punctuated::Punctuated, DeriveInput, Expr, Ident, LifetimeParam, LitStr, Token, TypeParam,
    },
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(display), supports(enum_any))]
struct InputReceiver {
    ident: Ident,
    generics: syn::Generics,
    data: ast::Data<Variant, util::Ignored>,
}

#[derive(Debug)]
struct Variant {
    ident: Ident,
    fields: ast::Fields<syn::Field>,
    fmt: LitStr,
    args: Option<Punctuated<Expr, Token![,]>>,
}

impl FromVariant for Variant {
    fn from_variant(variant: &syn::Variant) -> darling::Result<Self> {
        if let Some(attr) = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("display"))
        {
            let (fmt, args) = parse_display_attribute(attr)?;

            Ok(Self {
                ident: variant.ident.clone(),
                fields: ast::Fields::try_from(&variant.fields)?,
                fmt,
                args,
            })
        } else {
            Err(Error::custom("Every variant must have a display attribute").with_span(variant))
        }
    }
}

pub fn parse_enum(input: &DeriveInput) -> darling::Result<TokenStream> {
    let enum_receiver = InputReceiver::from_derive_input(input)?;

    let ty = enum_receiver.ident;

    let mut arms = Vec::new();

    for variant in enum_receiver.data.take_enum().unwrap() {
        let fields = variant.fields.fields;
        let variant_ident = variant.ident;
        let fmt_lit = variant.fmt;
        let fmt_args = variant.args;

        let arm = match variant.fields.style {
            Style::Tuple => {
                let idents =
                    (0..fields.len()).map(|i| Ident::new(&format!("_{}", i), variant_ident.span()));

                quote! {
                    #ty::#variant_ident(#( #idents ),*) => write!(f, #fmt_lit, #fmt_args)
                }
            }
            Style::Struct => {
                let field_idents = fields.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();

                    Ident::new(&field_ident.to_string(), field_ident.span())
                });

                quote! {
                    #ty::#variant_ident { #( #field_idents ),* } => write!(f, #fmt_lit, #fmt_args)
                }
            }
            Style::Unit => {
                quote! {
                    #ty::#variant_ident => write!(f, #fmt_lit, #fmt_args)
                }
            }
        };

        arms.push(arm);
    }

    let lifetimes: Punctuated<LifetimeParam, Token![,]> =
        Punctuated::from_iter(enum_receiver.generics.lifetimes().cloned());

    let generics: Punctuated<TypeParam, Token![,]> =
        Punctuated::from_iter(enum_receiver.generics.type_params().cloned().map(|param| {
            if param.default.is_some() {
                let mut param = param.clone();
                param.default = None;
                param
            } else {
                param
            }
        }));

    let optional_comma = if !lifetimes.is_empty() {
        TokenStream::from_str(",").unwrap()
    } else {
        TokenStream::new()
    };
    let where_clause = enum_receiver.generics.where_clause;

    Ok(quote! {
        impl<#lifetimes #optional_comma #generics> std::fmt::Display for #ty<#lifetimes #optional_comma #generics>
        #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #( #arms ),*
                }
            }
        }
    })
}
