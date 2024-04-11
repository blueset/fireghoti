use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Creates extra wrapper function for napi.
///
/// The types of the function arguments is converted with following rules:
/// - `&str` and `&mut str` are converted to `String`
/// - `&T` and `&mut T` are converted to `T`
/// - Other `T` remains `T`
///
/// # Examples
/// ## Example with `i32` argument
/// ```rust
/// #[macro_rs::napi]
/// fn add_one(x: i32) -> i32 {
///   x + 1
/// }
/// ```
///
/// becomes
///
/// ```rust
/// fn add_one(x: i32) -> i32 {
///   x + 1
/// }
/// #[cfg_attr(feature = "napi", napi_derive::napi(js_name = "addOne"))]
/// fn add_one_napi(x: i32) -> i32 {
///   add_one(x)
/// }
/// ```
///
/// ## Example with `&str` argument
/// ```rust
/// #[macro_rs::napi]
/// fn concatenate_string(str1: &str, str2: &str) -> String {
///   str1.to_owned() + str2
/// }
/// ```
///
/// becomes
///
/// ```rust
/// fn concatenate_string(str1: &str, str2: &str) -> String {
///   str1.to_owned() + str2
/// }
/// #[cfg_attr(feature = "napi", napi_derive::napi(js_name = "concatenateString"))]
/// fn concatenate_string_napi(str1: String, str2: String) -> String {
///   concatenate_string(&str1, &str2)
/// }
/// ```
///
/// TODO: macro attributes are ignored
#[proc_macro_attribute]
pub fn napi(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    napi_impl(attr.into(), item.into()).into()
}
fn napi_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse2(item).expect("Failed to parse TokenStream to syn::Item");
    // handle functions only
    let syn::Item::Fn(item_fn) = item else {
        // fallback to use napi_derive
        return quote! {
          #[napi_derive::napi(#attr)]
          #item
        };
    };

    let ident = &item_fn.sig.ident;
    let js_name = ident.to_string().to_case(Case::Camel);

    let item_fn_attrs = &item_fn.attrs;
    let item_fn_vis = &item_fn.vis;
    let mut item_fn_sig = item_fn.sig.clone();

    // append "_napi" to function name
    item_fn_sig.ident = syn::parse_str(&format!("{}_napi", &ident)).unwrap();

    // arguments in function call
    let called_args: Vec<TokenStream> = item_fn_sig
        .inputs
        .iter_mut()
        .map(|input| match input {
            // self
            syn::FnArg::Receiver(arg) => {
                let mut tokens = TokenStream::new();
                if let Some((ampersand, lifetime)) = &arg.reference {
                    ampersand.to_tokens(&mut tokens);
                    lifetime.to_tokens(&mut tokens);
                }
                arg.mutability.to_tokens(&mut tokens);
                arg.self_token.to_tokens(&mut tokens);
                tokens
            }
            // typed argument
            syn::FnArg::Typed(arg) => {
                match &mut *arg.pat {
                    syn::Pat::Ident(ident) => {
                        let name = &ident.ident;
                        match &*arg.ty {
                            // reference type argument => move ref from sigature to function call
                            syn::Type::Reference(r) => {
                                // add reference anotations to arguments in function call
                                let mut tokens = TokenStream::new();
                                r.and_token.to_tokens(&mut tokens);
                                if let Some(lifetime) = &r.lifetime {
                                    lifetime.to_tokens(&mut tokens);
                                }
                                r.mutability.to_tokens(&mut tokens);
                                name.to_tokens(&mut tokens);

                                // modify napi argument types in function sigature
                                // (1) add `mut` token to `&mut` type
                                ident.mutability = r.mutability;
                                // (2) remove reference
                                let elem_tokens = r.elem.to_token_stream();
                                *arg.ty =
                                    syn::Type::Verbatim(match elem_tokens.to_string().as_str() {
                                        // &str => String
                                        "str" => quote! { String },
                                        // &T => T
                                        _ => elem_tokens,
                                    });

                                // return arguments in function call
                                tokens
                            }
                            // o.w., return it as is
                            _ => quote! { #name },
                        }
                    }
                    pat => panic!("Unexpected FnArg: {pat:#?}"),
                }
            }
        })
        .collect();

    // TODO handle macro attr
    quote! {
      #item_fn

      #[cfg_attr(feature = "napi", napi_derive::napi(js_name = #js_name))]
      #(#item_fn_attrs)*
      #item_fn_vis #item_fn_sig {
        #ident(#(#called_args),*)
      }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn primitive_argument() {
        let generated = super::napi_impl(
            TokenStream::new(),
            quote! {
              fn add_one(x: i32) -> i32 {
                x + 1
              }
            },
        );
        let expected = quote! {
          fn add_one(x: i32) -> i32 {
            x + 1
          }
          #[cfg_attr(feature = "napi", napi_derive::napi(js_name = "addOne"))]
          fn add_one_napi(x: i32) -> i32 {
            add_one(x)
          }
        };
        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn str_ref_argument() {
        let generated = super::napi_impl(
            TokenStream::new(),
            quote! {
              fn concatenate_string(str1: &str, str2: &str) -> String {
                str1.to_owned() + str2
              }
            },
        );
        let expected = quote! {
          fn concatenate_string(str1: &str, str2: &str) -> String {
            str1.to_owned() + str2
          }
          #[cfg_attr(feature = "napi", napi_derive::napi(js_name = "concatenateString"))]
          fn concatenate_string_napi(str1: String, str2: String) -> String {
            concatenate_string(&str1, &str2)
          }
        };
        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn mut_ref_argument() {
        let generated = super::napi_impl(
            TokenStream::new(),
            quote! {
              fn append_string_and_clone(base_str: &mut String, appended_str: &str) -> String {
                base_str.push_str(appended_str);
                base_str.to_owned()
              }
            },
        );
        let expected = quote! {
          fn append_string_and_clone(base_str: &mut String, appended_str: &str) -> String {
            base_str.push_str(appended_str);
            base_str.to_owned()
          }
          #[cfg_attr(feature = "napi", napi_derive::napi(js_name = "appendStringAndClone"))]
          fn append_string_and_clone_napi(mut base_str: String, appended_str: String) -> String {
            append_string_and_clone(&mut base_str, &appended_str)
          }
        };
        assert_eq!(generated.to_string(), expected.to_string());
    }
}
