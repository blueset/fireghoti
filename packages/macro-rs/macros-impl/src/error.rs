//! Automatically generate doc comments for error variants from the error messages

use proc_macro2::TokenStream;
use quote::quote;

/// Generates doc comments for error enums.
///
/// # Example
/// ```
/// # use macros_impl::error::error_variants as errors;
/// # macros_impl::macro_doctest!({
/// #[macros::errors]
/// pub enum Error {
///     #[error("config file name is not set")]
///     NoConfigFileName,
///     #[error("failed to read the config file")]
///     ReadConfigFile(#[from] io::Error),
///     #[error("invalid file content ({0})")]
///     #[doc = "invalid file content"]
///     InvalidContent(String),
///     #[error(transparent)]
///     #[doc = "database error"]
///     Db(#[from] sea_orm::DbErr)
/// }
/// 
/// # }, {
/// /******* the code above expands to *******/
/// 
/// pub enum Error {
///     #[error("config file name is not set")]
///     #[doc = "config file name is not set"]
///     NoConfigFileName,
///     #[error("failed to read the config file")]
///     #[doc = "failed to read the config file"]
///     ReadConfigFile(#[from] io::Error),
///     #[error("invalid file content ({0})")]
///     #[doc = "invalid file content"]
///     InvalidContent(String),
///     #[error(transparent)]
///     #[doc = "database error"]
///     Db(#[from] sea_orm::DbErr)
/// }
/// # });
/// ```
pub fn error_variants(_attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let mut item: syn::ItemEnum = syn::parse2(item)?;

    item.variants = item
        .variants
        .into_iter()
        .map(|mut variant| {
            // check if doc attribute is alredy there
            if variant.attrs.iter().any(|attr| attr.path().is_ident("doc")) {
                return variant;
            }

            let msg = variant.attrs.iter().find_map(|attr| {
                if !attr.path().is_ident("error") {
                    return None;
                }
                let lit: syn::LitStr = attr.parse_args().ok()?;
                Some(lit.value())
            });

            // add #[doc] attribute
            if let Some(msg) = msg {
                variant.attrs.push(syn::parse_quote! { #[doc = #msg] });
            }

            variant
        })
        .collect();

    Ok(quote! { #item })
}
