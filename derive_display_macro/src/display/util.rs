use {
    darling::Error,
    syn::{punctuated::Punctuated, Attribute, Expr, Lit, LitStr, Token},
};

pub fn parse_display_attribute(
    attr: &Attribute,
) -> darling::Result<(LitStr, Option<Punctuated<Expr, Token![,]>>)> {
    let list = attr
        .meta
        .require_list()
        .map_err(|_| Error::custom("Invalid #[display] attribute format"))?;

    let args = list
        .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        .map_err(|_| Error::custom("Invalid #[display] arguments"))?;

    let first_value = args.first().cloned();
    let args: Punctuated<Expr, Token![,]> = args.into_iter().skip(1).collect();

    match first_value {
        Some(Expr::Lit(syn::ExprLit {
            lit: Lit::Str(s), ..
        })) => Ok((s, Some(args))),
        Some(expr) => Err(Error::custom("Expected a string literal").with_span(&expr)),
        None => Err(Error::custom("Expected a string literal").with_span(&attr)),
    }
}
