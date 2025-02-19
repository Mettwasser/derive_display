use {
    super::util::parse_display_attribute,
    darling::{util, Error, FromDeriveInput, FromMeta},
    proc_macro2::TokenStream,
    quote::quote,
    std::str::FromStr,
    syn::{
        punctuated::Punctuated, Data, DeriveInput, Expr, Fields, Ident, LifetimeParam, LitStr,
        Token, TypeParam,
    },
};

fn validate_body(body: &Data) -> darling::Result<()> {
    {
        let struct_check = util::ShapeSet::new(vec![
            util::Shape::Named,
            util::Shape::Tuple,
            util::Shape::Newtype,
            util::Shape::Unit,
        ]);
        let enum_check = util::ShapeSet::new(vec![]);
        match *body {
            Data::Enum(ref data) => {
                if enum_check.is_empty() {
                    return Err(Error::unsupported_shape_with_expected(
                        "enum",
                        &format!("struct with {}", struct_check),
                    ));
                }
                let mut variant_errors = Error::accumulator();
                for variant in &data.variants {
                    variant_errors.handle(enum_check.check(variant));
                }
                variant_errors.finish()
            }
            Data::Struct(ref struct_data) => {
                if struct_check.is_empty() {
                    return Err(Error::unsupported_shape_with_expected(
                        "struct",
                        &format!("enum with {}", enum_check),
                    ));
                }
                struct_check.check(struct_data)
            }
            Data::Union(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct InputReceiver {
    ident: Ident,
    generics: syn::Generics,
    fmt: LitStr,
    args: Option<Punctuated<Expr, Token![,]>>,
}

impl FromDeriveInput for InputReceiver {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        validate_body(&input.data)?;
        if let Some(attr) = input
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("display"))
        {
            let (fmt, args) = parse_display_attribute(attr)?;

            Ok(Self {
                ident: input.ident.clone(),
                generics: input.generics.clone(),
                fmt,
                args,
            })
        } else {
            Err(Error::custom("Struct must have a display attribute").with_span(input))
        }
    }
}

pub fn parse_struct(input: &DeriveInput) -> darling::Result<TokenStream> {
    let struct_receiver = InputReceiver::from_derive_input(input)?;

    let ty = struct_receiver.ident;

    let Data::Struct(data_struct) = &input.data else {
        unreachable!();
    };

    let let_stmt = match &data_struct.fields {
        Fields::Named(fields) => {
            let field_idents = fields
                .named
                .iter()
                .cloned()
                .map(|field| field.ident.unwrap())
                .collect::<Vec<_>>();

            quote! {
                let #ty { #( #field_idents ),* } = self;
            }
        }
        Fields::Unnamed(fields) => {
            let idents =
                (0..fields.unnamed.len()).map(|i| Ident::from_string(&format!("_{i}")).unwrap());

            quote! {
                let #ty ( #( #idents ),* ) = self;
            }
        }
        Fields::Unit => TokenStream::new(),
    };

    let lifetimes: Punctuated<LifetimeParam, Token![,]> =
        Punctuated::from_iter(struct_receiver.generics.lifetimes().cloned());

    let generics: Punctuated<TypeParam, Token![,]> = Punctuated::from_iter(
        struct_receiver
            .generics
            .type_params()
            .cloned()
            .map(|param| {
                if param.default.is_some() {
                    let mut param = param.clone();
                    param.default = None;
                    param
                } else {
                    param
                }
            }),
    );

    let optional_comma = if !lifetimes.is_empty() {
        TokenStream::from_str(",").unwrap()
    } else {
        TokenStream::new()
    };
    let where_clause = struct_receiver.generics.where_clause;

    let fmt = struct_receiver.fmt;
    let args = struct_receiver.args.unwrap_or_else(Punctuated::new);

    Ok(quote! {
        impl<#lifetimes #optional_comma #generics> std::fmt::Display for #ty<#lifetimes #optional_comma #generics>
        #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #let_stmt
                write!(f, #fmt, #args)
            }
        }
    })
}
