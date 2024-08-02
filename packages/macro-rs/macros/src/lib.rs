mod helper;
use helper::*;

/// Reads the version field in the project root package.json at compile time.
///
/// # Example
/// You can get a compile-time constant version number using this macro:
/// ```
/// # use macros::read_version_from_package_json;
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

    quote!(#version).into()
}

define_wrapper_proc_macro_attributes! {
    /// Exports an enum to TypeScript, and derive [Clone].
    ///
    /// You need this macro because [`napi_derive::napi`](https://docs.rs/napi-derive/latest/napi_derive/attr.napi.html)
    /// automatically derives the [Clone] trait for enums and causes conflicts.
    ///
    /// This is a wrapper of [`napi_derive::napi`](https://docs.rs/napi-derive/latest/napi_derive/attr.napi.html)
    /// that expands to
    /// ```no_run
    /// #[cfg_attr(not(feature = "napi"), derive(Clone))]
    /// #[cfg_attr(feature = "napi", napi_derive::napi(attr))]
    /// # enum E {} // to work around doc test compilation error
    /// ```
    /// where `attr` is given attribute(s). See [`macros_impl::napi::napi`] for more details.
    derive_clone_and_export(attr, item) {
        #[cfg_attr(not(feature = "napi"), derive(Clone))]
        #[cfg_attr(feature = "napi", napi_derive::napi(#attr))]
        #item
    }

    /// Exports a function, struct, enum, const, etc. to TypeScript.
    ///
    /// This is a wrapper of [macro@napi] that expands to
    /// ```no_run
    /// #[cfg_attr(feature = "napi", macros::napi(attr))]
    /// # fn f() {} // to work around doc test compilation error
    /// ```
    /// where `attr` is given attribute(s). See [`macros_impl::napi::napi`] for more details.
    export(attr, item) {
        #[cfg_attr(feature = "napi", macros::napi(#attr))]
        #item
    }

    /// Exports a function, struct, enum, const, etc. to TypeScript
    /// and make it unable to use in Rust.
    ///
    /// This is a wrapper of [macro@napi] that expands to
    /// ```no_run
    /// #[cfg(feature = "napi")]
    /// #[macros::napi(attr)]
    /// # fn f() {} // to work around doc test compilation error
    /// ```
    /// where `attr` is given attribute(s). See [`macros_impl::napi::napi`] for more details.
    ts_export(attr, item) {
        #[cfg(feature = "napi")]
        #[macros::napi(#attr)]
        #item

        #[cfg(any(test, doctest))]
        #item
    }

    for_ts(_attr, item) {
        #[cfg(any(test, doctest, feature = "napi"))]
        #item
    }

    /// When applied to enums, this macro implements [`std::error::Error`] trait
    /// and generates a document based on error messages unless there is already a doc comment
    ///
    /// See [`macros_impl::error::error_variants`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io;
    /// #[macros::errors]
    /// pub enum Error {
    ///     #[error("config file name is not set")]
    ///     NoConfigFileName,
    ///     #[error("failed to read the config file")]
    ///     ReadConfigFile(#[from] io::Error),
    ///     #[error("invalid file content ({0})")]
    ///     #[doc = "invalid file content"]
    ///     InvalidContent(String),
    /// }
    /// ```
    errors(attr, item) {
        #[derive(::thiserror::Error, ::std::fmt::Debug)]
        #[macros::error_variants(#attr, #item)]
        #item
    }
}

reexport_proc_macro_attributes! {
    /// Creates an extra wrapper function for [napi_derive](https://docs.rs/napi-derive/latest/napi_derive/).
    /// See [macros_impl::napi::napi] for details.
    macros_impl::napi::napi as napi

    /// Generates doc comments for error variants from the error messages
    macros_impl::error::error_variants as error_variants
}
