//! Macros for testing procedural macros

/// Tests if the macro expands correctly.
///
/// # Examples
/// ```
/// use macros_impl::napi::napi;
///
/// macros_impl::macro_doctest!({
///     #[macros::napi(object)]
///     struct Person {
///         id: i32,
///         name: String,
///     }
/// }, {
///    #[napi_derive::napi(use_nullable = true, object)]
///    struct Person {
///        id: i32,
///        name: String,
///    }
/// });
/// ```
#[macro_export]
macro_rules! macro_doctest {
    ({
        #[macros :: $macro_name:ident $(( $($attr:tt)* ))?]
        $($item:tt)*
    }, {
        $($expanded:tt)*
    }) => {
        assert_eq!(
            ::std::string::ToString::to_string(
                &$macro_name(
                    ::quote::quote!($( $($attr)* )?),
                    ::quote::quote!($($item)*),
                )
            ),
            ::std::string::ToString::to_string(
                &::quote::quote!($($expanded)*)
            )
        );
    };
}

/// Creates unit tests for macros.
///
/// # Examples
/// ```
/// macros_impl::macro_unit_tests! {
///     add1_becomes: {
///         #[macros::napi(js_name = "add1")]
///         pub fn add_one(x: i32) -> i32 {
///             x + 1
///         }
///     } becomes { // the code above should expand to the following code
///         pub fn add_one(x: i32) -> i32 {
///             x + 1
///         }
///
///         #[napi_derive::napi(js_name = "add1")]
///         pub fn add_one_napi(x: i32) -> i32 {
///             add_one(x)
///         }
///     }
///
///     // this test case is equivalent to `add1_becomes`
///     add1_generates: {
///         #[macros::napi(js_name = "add1")]
///         pub fn add_one(x: i32) -> i32 {
///             x + 1
///         }
///     } generates { // the code above should generate the following code
///         #[napi_derive::napi(js_name = "add1")]
///         pub fn add_one_napi(x: i32) -> i32 {
///             add_one(x)
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! macro_unit_tests {
    (@test $macro_name:ident($attr:ident, $item:ident) becomes $expanded:ident) => {
        assert_eq!(
            ::std::format!("{}", $macro_name($attr, $item)),
            ::std::format!("{}", $expanded),
        );
    };
    (@test $macro_name:ident($attr:ident, $item:ident) generates $expanded:ident) => {
        let item_str = format!("{}", $item);
        assert_eq!(
            ::std::format!("{}", $macro_name($attr, $item)),
            ::std::format!("{} {}", item_str, $expanded),
        );
    };

    (
        $(
            $test_name:ident : {
                #[macros :: $macro_name:ident $(( $($attr:tt)* ))?]
                $($item:tt)*
            } $op:tt {
                $($expanded:tt)*
            }
        )*
    ) => {
        #[cfg(test)]
        mod unit_test {
            use super::*;

            $(
                #[test]
                fn $test_name() {
                    let attr = ::quote::quote!($( $($attr)* )?);
                    let item = ::quote::quote!($($item)*);
                    let expanded = ::quote::quote!($($expanded)*);

                    $crate::macro_unit_tests!(@test $macro_name(attr, item) $op expanded);
                }
            )*
        }
    };
}
