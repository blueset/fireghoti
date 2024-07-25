use proc_macro2::TokenStream;
use quote::quote;

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
