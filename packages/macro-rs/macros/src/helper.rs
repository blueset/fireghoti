//! Helper macros for developing procedural macros

#[doc(hidden)]
pub(crate) use quote::quote;

/// Defines wrapper #\[proc_macro_attribute]s.
///
/// # Examples
/// ```ignore
/// define_wrapper_proc_macro_attributes! {
///    // expand `#[export(attr)]` to
///    // ```
///    // #[cfg_attr(feature = "napi", macros::napi(#attr))]
///    // ```
///    export(attr, item) {
///        #[cfg_attr(feature = "napi", macros::napi(#attr))]
///        #item
///    }
///
///    // expand `#[ts_export(attr)]` to
///    // ```
///    // #[cfg(feature = "napi")]
///    // #[macros::napi(#attr)]
///    // ```
///    ts_export(attr, item) {
///        #[cfg(feature = "napi")]
///        #[macros::napi(#attr)]
///        #item
///    }
/// }
/// ```
macro_rules! define_wrapper_proc_macro_attributes {
    (
        $(
            $(#[$meta:meta])*
            $macro_name:ident ($arg_attr:ident, $arg_item:ident) {
                $($body:tt)*
            }
        )*
    ) => {
        $(
            $(#[$meta])*
            #[proc_macro_attribute]
            pub fn $macro_name(
                attr: ::proc_macro::TokenStream,
                item: ::proc_macro::TokenStream,
            ) -> ::proc_macro::TokenStream {
                let $arg_attr: ::proc_macro2::TokenStream = attr.into();
                let $arg_item: ::proc_macro2::TokenStream = item.into();
                ::quote::quote!($($body)*).into()
            }
        )*
    }
}
pub(crate) use define_wrapper_proc_macro_attributes;

/// Wraps and exports #\[proc_macro_attribute] implementation.
///
/// # Examples
/// ```ignore
/// reexport_proc_macro_attributes! {
///     // wrap and export [macros_impl::napi::napi] as #[macros::napi]
///     macros_impl::napi::napi as napi
///
///     // wrap and export [macros_impl::errors::errors] as #[macros::errors]
///     macros_impl::errors::errors as errors
/// }
/// ```
macro_rules! reexport_proc_macro_attributes {
    (
        $(
            $(#[$meta:meta])*
            $impl_path:path as $macro_name:ident
        )*
    ) => {
        $(
            $(#[$meta])*
            #[proc_macro_attribute]
            pub fn $macro_name(
                attr: ::proc_macro::TokenStream,
                item: ::proc_macro::TokenStream,
            ) -> ::proc_macro::TokenStream {
                $impl_path(attr.into(), item.into()).into()
            }
        )*
    }
}
pub(crate) use reexport_proc_macro_attributes;
