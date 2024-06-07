use convert_case::{Case, Casing};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};

/// Read the version field in the project root package.json at compile time
///
/// # Example
/// You can get a compile-time constant version number using this macro:
/// ```
/// # use macro_rs::read_version_from_package_json;
/// // VERSION == "YYYYMMDD" (or "YYYYMMDD-X")
/// const VERSION: &str = read_version_from_package_json!();
/// ```
#[proc_macro]
pub fn read_version_from_package_json(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(serde::Deserialize)]
    struct PackageJson {
        version: String,
    }

    let file = std::fs::File::open("package.json").expect("Failed to open package.json");
    let json: PackageJson = serde_json::from_reader(file).unwrap();
    let version = &json.version;

    quote! { #version }.into()
}

/// Export an enum to TypeScript, and derive [Clone].
///
/// You need this macro because [`napi_derive::napi`](https://docs.rs/napi-derive/latest/napi_derive/attr.napi.html)
/// automatically derives the [Clone] trait for enums and causes conflicts.
///
/// This is a wrapper of [`napi_derive::napi`](https://docs.rs/napi-derive/latest/napi_derive/attr.napi.html)
/// that expands to
/// ```no_run
/// #[cfg_attr(not(feature = "napi"), derive(Clone))]
/// #[cfg_attr(feature = "napi", napi_derive::napi(attr))]
/// # fn f() {} // to work around doc test compilation error
/// ```
/// where `attr` is given attribute(s).
#[proc_macro_attribute]
pub fn derive_clone_and_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();

    quote! {
        #[cfg_attr(not(feature = "napi"), derive(Clone))]
        #[cfg_attr(feature = "napi", napi_derive::napi(#attr))]
        #item
    }
    .into()
}

/// Export a function, struct, enum, const, etc. to TypeScript.
///
/// This is a wrapper of [macro@napi] that expands to
/// ```no_run
/// #[cfg_attr(feature = "napi", macro_rs::napi(attr))]
/// # fn f() {} // to work around doc test compilation error
/// ```
/// where `attr` is given attribute(s). See [macro@napi] for more details.
#[proc_macro_attribute]
pub fn export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();

    quote! {
        #[cfg_attr(feature = "napi", macro_rs::napi(#attr))]
        #item
    }
    .into()
}

/// Export a function, struct, enum, const, etc. to TypeScript
/// and make it unable to use in Rust.
///
/// This is a wrapper of [macro@napi] that expands to
/// ```no_run
/// #[cfg(feature = "napi")]
/// #[macro_rs::napi(attr)]
/// # fn f() {} // to work around doc test compilation error
/// ```
/// where `attr` is given attribute(s). See [macro@napi] for more details.
#[proc_macro_attribute]
pub fn ts_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();

    quote! {
            #[cfg(feature = "napi")]
            #[macro_rs::napi(#attr)]
            #item
    }
    .into()
}

/// Creates an extra wrapper function for [napi_derive](https://docs.rs/napi-derive/latest/napi_derive/).
///
/// The macro is simply converted into `napi_derive::napi(...)`
/// if it is not applied to a function.
///
/// The macro sets the following attributes by default if not specified:
/// - `use_nullable = true` (if `object` or `constructor` attribute is specified)
/// - `js_name` to the camelCase version of the original function name (for functions)
///
/// The types of the function arguments is converted with following rules:
/// - `&str` and `&mut str` are converted to [`String`]
/// - `&[T]` and `&mut [T]` are converted to [`Vec<T>`]
/// - `&T` and `&mut T` are converted to `T`
/// - Other `T` remains `T`
///
/// In addition, return type [`Result<T>`] and [`Result<T, E>`] are converted to [`napi::Result<T>`](https://docs.rs/napi/latest/napi/type.Result.html).
/// Note that `E` must implement [`std::string::ToString`] trait.
///
/// # Examples
/// ## Applying the macro to a struct
/// ```
/// #[macro_rs::napi(object)]
/// struct Person {
///     id: i32,
///     name: String,
/// }
/// ```
/// simply becomes
/// ```
/// #[napi_derive::napi(use_nullable = true, object)]
/// struct Person {
///     id: i32,
///     name: String,
/// }
/// ```
///
/// ## Function with explicitly specified `js_name`
/// ```
/// #[macro_rs::napi(js_name = "add1")]
/// pub fn add_one(x: i32) -> i32 {
///     x + 1
/// }
/// ```
/// generates
/// ```
/// # pub fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// #[napi_derive::napi(js_name = "add1",)]
/// pub fn add_one_napi(x: i32) -> i32 {
///     add_one(x)
/// }
/// ```
///
/// ## Function with `i32` argument
/// ```
/// #[macro_rs::napi]
/// pub fn add_one(x: i32) -> i32 {
///     x + 1
/// }
/// ```
/// generates
/// ```
/// # pub fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// #[napi_derive::napi(js_name = "addOne",)]
/// pub fn add_one_napi(x: i32) -> i32 {
///     add_one(x)
/// }
/// ```
///
/// ## Function with `&str` argument
/// ```
/// #[macro_rs::napi]
/// pub fn concatenate_string(str1: &str, str2: &str) -> String {
///     str1.to_owned() + str2
/// }
/// ```
/// generates
/// ```
/// # pub fn concatenate_string(str1: &str, str2: &str) -> String {
/// #     str1.to_owned() + str2
/// # }
/// #[napi_derive::napi(js_name = "concatenateString",)]
/// pub fn concatenate_string_napi(str1: String, str2: String) -> String {
///     concatenate_string(&str1, &str2)
/// }
/// ```
///
/// ## Function with `&[String]` argument
/// ```
/// #[macro_rs::napi]
/// pub fn string_array_length(array: &[String]) -> u32 {
///     array.len() as u32
/// }
/// ```
/// generates
/// ```
/// # pub fn string_array_length(array: &[String]) -> u32 {
/// #     array.len() as u32
/// # }
/// #[napi_derive::napi(js_name = "stringArrayLength",)]
/// pub fn string_array_length_napi(array: Vec<String>) -> u32 {
///     string_array_length(&array)
/// }
/// ```
///
/// ## Function with `Result<T, E>` return type
/// ```
/// #[derive(thiserror::Error, Debug)]
/// pub enum IntegerDivisionError {
///     #[error("Divided by zero")]
///     DividedByZero,
///     #[error("Not divisible with remainder = {0}")]
///     NotDivisible(i64),
/// }
///
/// #[macro_rs::napi]
/// pub fn integer_divide(dividend: i64, divisor: i64) -> Result<i64, IntegerDivisionError> {
///     match divisor {
///         0 => Err(IntegerDivisionError::DividedByZero),
///         _ => match dividend % divisor {
///             0 => Ok(dividend / divisor),
///             remainder => Err(IntegerDivisionError::NotDivisible(remainder)),
///         },
///     }
/// }
/// ```
/// generates
/// ```
/// # #[derive(thiserror::Error, Debug)]
/// # pub enum IntegerDivisionError {
/// #     #[error("Divided by zero")]
/// #     DividedByZero,
/// #     #[error("Not divisible with remainder = {0}")]
/// #     NotDivisible(i64),
/// # }
/// # pub fn integer_divide(dividend: i64, divisor: i64) -> Result<i64, IntegerDivisionError> {
/// #     match divisor {
/// #         0 => Err(IntegerDivisionError::DividedByZero),
/// #         _ => match dividend % divisor {
/// #             0 => Ok(dividend / divisor),
/// #             remainder => Err(IntegerDivisionError::NotDivisible(remainder)),
/// #         },
/// #     }
/// # }
/// #[napi_derive::napi(js_name = "integerDivide",)]
/// pub fn integer_divide_napi(dividend: i64, divisor: i64) -> napi::Result<i64> {
///     integer_divide(dividend, divisor).map_err(|err| napi::Error::from_reason(err.to_string()))
/// }
/// ```
#[proc_macro_attribute]
pub fn napi(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    napi_impl(attr.into(), item.into()).into()
}

fn napi_impl(macro_attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_attr_tokens: Vec<TokenTree> = macro_attr.clone().into_iter().collect();
    // generated extra macro attr TokenStream (prepended before original input `macro_attr`)
    let mut extra_macro_attr = TokenStream::new();

    let item: syn::Item =
        syn::parse2(item).expect("Failed to parse input TokenStream to syn::Item");

    // handle non-functions
    let syn::Item::Fn(item_fn) = item else {
        // append `use_nullable = true` if `object` or `constructor` present but not `use_nullable`
        if macro_attr_tokens.iter().any(|token| {
            matches!(token, TokenTree::Ident(ident) if ident == "object" || ident == "constructor")
        }) && !macro_attr_tokens.iter().any(|token| {
            matches!(token, TokenTree::Ident(ident) if ident == "use_nullable")
        }) {
            quote! { use_nullable = true, }.to_tokens(&mut extra_macro_attr);
        }
        return quote! {
          #[napi_derive::napi(#extra_macro_attr #macro_attr)]
          #item
        };
    };

    // handle functions
    let ident = &item_fn.sig.ident;
    let item_fn_attrs = &item_fn.attrs;
    let item_fn_vis = &item_fn.vis;
    let mut item_fn_sig = item_fn.sig.clone();
    let mut function_call_modifiers = Vec::<TokenStream>::new();

    // append "_napi" to function name
    item_fn_sig.ident = syn::parse_str(&format!("{}_napi", &ident)).unwrap();

    // append `.await` to function call in async function
    if item_fn_sig.asyncness.is_some() {
        function_call_modifiers.push(quote! {
            .await
        });
    }

    // convert return type `...::Result<T, ...>` to `napi::Result<T>`
    if let syn::ReturnType::Type(_, ref mut return_type) = item_fn_sig.output {
        if let Some(result_generic_type) = (|| {
            let syn::Type::Path(return_type_path) = &**return_type else {
                return None;
            };
            // match a::b::c::Result
            let last_segment = return_type_path.path.segments.last()?;
            if last_segment.ident != "Result" {
                return None;
            };
            // extract <T, ...> from Result<T, ...>
            let syn::PathArguments::AngleBracketed(generic_arguments) = &last_segment.arguments
            else {
                return None;
            };
            // return T only
            generic_arguments.args.first()
        })() {
            // modify return type
            *return_type = syn::parse_quote! {
                napi::Result<#result_generic_type>
            };
            // add modifier to function call result
            function_call_modifiers.push(quote! {
                .map_err(|err| napi::Error::from_reason(err.to_string()))
            });
        }
    };

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
                                *arg.ty = syn::Type::Verbatim(match &*r.elem {
                                    syn::Type::Slice(slice) => {
                                        let ty = &*slice.elem;
                                        quote! { Vec<#ty> }
                                    }
                                    _ => {
                                        let elem_tokens = r.elem.to_token_stream();
                                        match elem_tokens.to_string().as_str() {
                                            // &str => String
                                            "str" => quote! { String },
                                            // &T => T
                                            _ => elem_tokens,
                                        }
                                    }
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

    // handle macro attr
    // append js_name if not specified
    if !macro_attr_tokens
        .iter()
        .any(|token| matches!(token, TokenTree::Ident(ident) if ident == "js_name"))
    {
        let js_name = ident.to_string().to_case(Case::Camel);
        quote! { js_name = #js_name, }.to_tokens(&mut extra_macro_attr);
    }

    quote! {
      #item_fn

      #[napi_derive::napi(#extra_macro_attr #macro_attr)]
      #(#item_fn_attrs)*
      #item_fn_vis #item_fn_sig {
        #ident(#(#called_args),*)
        #(#function_call_modifiers)*
      }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;

    macro_rules! test_macro_becomes {
        ($source:expr, $generated:expr) => {
            assert_eq!(
                super::napi_impl(TokenStream::new(), $source).to_string(),
                $generated.to_string(),
            )
        };
        ($macro_attr:expr, $source:expr, $generated:expr) => {
            assert_eq!(
                super::napi_impl($macro_attr, $source).to_string(),
                $generated.to_string(),
            )
        };
    }

    macro_rules! test_macro_generates {
        ($source:expr, $generated:expr) => {
            assert_eq!(
                super::napi_impl(TokenStream::new(), $source).to_string(),
                format!("{} {}", $source, $generated),
            )
        };
        ($macro_attr:expr, $source:expr, $generated:expr) => {
            assert_eq!(
                super::napi_impl($macro_attr, $source).to_string(),
                format!("{} {}", $source, $generated),
            )
        };
    }

    #[test]
    fn primitive_argument() {
        test_macro_generates!(
            quote! {
                pub fn add_one(x: i32) -> i32 {
                    x + 1
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "addOne", )]
                pub fn add_one_napi(x: i32) -> i32 {
                    add_one(x)
                }
            }
        );
    }

    #[test]
    fn str_ref_argument() {
        test_macro_generates!(
            quote! {
                pub fn concatenate_string(str1: &str, str2: &str) -> String {
                    str1.to_owned() + str2
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "concatenateString", )]
                pub fn concatenate_string_napi(str1: String, str2: String) -> String {
                    concatenate_string(&str1, &str2)
                }
            }
        );
    }

    #[test]
    fn mut_ref_argument() {
        test_macro_generates!(
            quote! {
                pub fn append_string_and_clone(
                    base_str: &mut String,
                    appended_str: &str,
                ) -> String {
                    base_str.push_str(appended_str);
                    base_str.to_owned()
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "appendStringAndClone", )]
                pub fn append_string_and_clone_napi(
                    mut base_str: String,
                    appended_str: String,
                ) -> String {
                    append_string_and_clone(&mut base_str, &appended_str)
                }
            }
        );
    }

    #[test]
    fn result_return_type() {
        test_macro_generates!(
            quote! {
                pub fn integer_divide(
                    dividend: i64,
                    divisor: i64,
                ) -> Result<i64, IntegerDivisionError> {
                    match divisor {
                        0 => Err(IntegerDivisionError::DividedByZero),
                        _ => match dividend % divisor {
                            0 => Ok(dividend / divisor),
                            remainder => Err(IntegerDivisionError::NotDivisible(remainder)),
                        },
                    }
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "integerDivide", )]
                pub fn integer_divide_napi(
                    dividend: i64,
                    divisor: i64,
                ) -> napi::Result<i64> {
                    integer_divide(dividend, divisor)
                        .map_err(|err| napi::Error::from_reason(err.to_string()))
                }
            }
        );
    }

    #[test]
    fn async_function() {
        test_macro_generates!(
            quote! {
                pub async fn async_add_one(x: i32) -> i32 {
                    x + 1
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "asyncAddOne", )]
                pub async fn async_add_one_napi(x: i32) -> i32 {
                    async_add_one(x)
                        .await
                }
            }
        )
    }

    #[test]
    fn slice_type() {
        test_macro_generates!(
            quote! {
                pub fn string_array_length(array: &[String]) -> u32 {
                    array.len() as u32
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "stringArrayLength", )]
                pub fn string_array_length_napi(array: Vec<String>) -> u32 {
                    string_array_length(&array)
                }
            }
        )
    }

    #[test]
    fn object() {
        test_macro_becomes!(
            quote! { object },
            quote! {
                struct Person {
                    id: i32,
                    name: Option<String>,
                }
            },
            quote! {
                #[napi_derive::napi(use_nullable = true, object)]
                struct Person {
                    id: i32,
                    name: Option<String>,
                }
            }
        )
    }

    #[test]
    fn object_with_explicitly_set_use_nullable() {
        test_macro_becomes!(
            quote! { object, use_nullable = false },
            quote! {
                struct Person {
                    id: i32,
                    name: Option<String>,
                }
            },
            quote! {
                #[napi_derive::napi(object, use_nullable = false)]
                struct Person {
                    id: i32,
                    name: Option<String>,
                }
            }
        )
    }

    #[test]
    fn macro_attr() {
        test_macro_generates!(
            quote! {
                ts_return_type = "number"
            },
            quote! {
                pub fn add_one(x: i32) -> i32 {
                    x + 1
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "addOne", ts_return_type = "number")]
                pub fn add_one_napi(x: i32) -> i32 {
                    add_one(x)
                }
            }
        )
    }

    #[test]
    fn explicitly_specified_js_name() {
        test_macro_generates!(
            quote! {
                js_name = "add1"
            },
            quote! {
                pub fn add_one(x: i32) -> i32 {
                    x + 1
                }
            },
            quote! {
                #[napi_derive::napi(js_name = "add1")]
                pub fn add_one_napi(x: i32) -> i32 {
                    add_one(x)
                }
            }
        )
    }

    #[test]
    fn explicitly_specified_js_name_and_other_macro_attr() {
        test_macro_generates!(
            quote! { ts_return_type = "number", js_name = "add1" },
            quote! {
                pub fn add_one(x: i32) -> i32 {
                    x + 1
                }
            },
            quote! {
                #[napi_derive::napi(ts_return_type = "number", js_name = "add1")]
                pub fn add_one_napi(x: i32) -> i32 {
                    add_one(x)
                }
            }
        )
    }
}
