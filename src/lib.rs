//! # Pretty Assertions
//!
//! When writing tests in Rust, you'll probably use `assert_eq!(a, b)` _a lot_.
//!
//! If such a test fails, it will present all the details of `a` and `b`.
//! But you have to spot the differences yourself, which is not always straightforward,
//! like here:
//!
//! ![standard assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/standard_assertion.png)
//!
//! Wouldn't that task be _much_ easier with a colorful diff?
//!
//! ![pretty assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/pretty_assertion.png)
//!
//! Yep — and you only need **one line of code** to make it happen:
//!
//! ```rust
//! use pretty_assertions::{assert_eq, assert_ne};
//! ```
//!
//! <details>
//! <summary>Show the example behind the screenshots above.</summary>
//!
//! ```rust,should_panic
//! // 1. add the `pretty_assertions` dependency to `Cargo.toml`.
//! // 2. insert this line at the top of each module, as needed
//! use pretty_assertions::{assert_eq, assert_ne};
//!
//! #[derive(Debug, PartialEq)]
//! struct Foo {
//!     lorem: &'static str,
//!     ipsum: u32,
//!     dolor: Result<String, String>,
//! }
//!
//! let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
//! let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});
//!
//! assert_eq!(x, y);
//! ```
//! </details>
//!
//! ## Tip
//!
//! Specify it as [`[dev-dependencies]`](http://doc.crates.io/specifying-dependencies.html#development-dependencies)
//! and it will only be used for compiling tests, examples, and benchmarks.
//! This way the compile time of `cargo build` won't be affected!
//!
//! Also add `#[cfg(test)]` to your `use` statements, like this:
//!
//! ```rust
//! #[cfg(test)]
//! use pretty_assertions::{assert_eq, assert_ne};
//! ```
//!
//! ## Note
//!
//! * Since `Rust 2018` edition, you need to declare
//!   `use pretty_assertions::{assert_eq, assert_ne};` per module.
//!   Before you would write `#[macro_use] extern crate pretty_assertions;`.
//! * The replacement is only effective in your own crate, not in other libraries
//!   you include.
//! * `assert_ne` is also switched to multi-line presentation, but does _not_ show
//!   a diff.

#![deny(clippy::all, missing_docs, unsafe_code)]

pub use ansi_term::Style;
use std::fmt::{self, Debug, Display};

mod printer;

#[cfg(windows)]
use ctor::*;
#[cfg(windows)]
#[ctor]
fn init() {
    output_vt100::try_init().ok(); // Do not panic on fail
}

/// A comparison of two values.
///
/// Where both values implement `Debug`, the comparison can be displayed as a pretty diff.
///
/// ```
/// use pretty_assertions::Comparison;
///
/// print!("{}", Comparison::new(&123, &134));
/// ```
///
/// The values may have different types, although in practice they are usually the same.
pub struct Comparison<'a, TLeft, TRight>
where
    TLeft: ?Sized,
    TRight: ?Sized,
{
    left: &'a TLeft,
    right: &'a TRight,
}

impl<'a, TLeft, TRight> Comparison<'a, TLeft, TRight>
where
    TLeft: ?Sized,
    TRight: ?Sized,
{
    /// Store two values to be compared in future.
    ///
    /// Expensive diffing is deferred until calling `Debug::fmt`.
    pub fn new(left: &'a TLeft, right: &'a TRight) -> Comparison<'a, TLeft, TRight> {
        Comparison { left, right }
    }
}

impl<'a, TLeft, TRight> Display for Comparison<'a, TLeft, TRight>
where
    TLeft: Debug + ?Sized,
    TRight: Debug + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // To diff arbitary types, render them as debug strings
        let left_debug = format!("{:#?}", self.left);
        let right_debug = format!("{:#?}", self.right);
        // And then diff the debug output
        printer::write_header(f)?;
        printer::write_lines(f, &left_debug, &right_debug)
    }
}

/// Asserts that two expressions are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print a diff derived from [`Debug`] representation of
/// each value.
///
/// This is a drop in replacement for [`std::assert_eq!`].
/// You can provide a custom panic message if desired.
///
/// # Examples
///
/// ```
/// use pretty_assertions::assert_eq;
///
/// let a = 3;
/// let b = 1 + 2;
/// assert_eq!(a, b);
///
/// assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
/// ```
#[macro_export]
macro_rules! assert_eq {
    ($left:expr , $right:expr,) => ({
        $crate::assert_eq!($left, $right)
    });
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::std::panic!("assertion failed: `(left == right)`\
                          \n\
                          \n{}\
                          \n",
                           $crate::Comparison::new(left_val, right_val))
                }
            }
        }
    });
    ($left:expr , $right:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::std::panic!("assertion failed: `(left == right)`: {}\
                          \n\
                          \n{}\
                          \n",
                           format_args!($($arg)*),
                           $crate::Comparison::new(left_val, right_val))
                }
            }
        }
    });
}

/// Asserts that two expressions are not equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the expressions with their
/// [`Debug`] representations.
///
/// This is a drop in replacement for [`std::assert_ne!`].
/// You can provide a custom panic message if desired.
///
/// # Examples
///
/// ```
/// use pretty_assertions::assert_ne;
///
/// let a = 3;
/// let b = 2;
/// assert_ne!(a, b);
///
/// assert_ne!(a, b, "we are testing that the values are not equal");
/// ```
#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => ({
        $crate::assert_ne!(@ $left, $right, "", "");
    });
    ($left:expr, $right:expr,) => ({
        $crate::assert_ne!(@ $left, $right, "", "");
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        $crate::assert_ne!(@ $left, $right, ": ", $($arg)+);
    });
    (@ $left:expr, $right:expr, $maybe_semicolon:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                  let left_dbg = ::std::format!("{:?}", &*left_val);
                  let right_dbg = ::std::format!("{:?}", &*right_val);
                  if left_dbg != right_dbg {

                      ::std::panic!("assertion failed: `(left != right)`{}{}\
                            \n\
                            \n{}\
                            \n{}: According to the `PartialEq` implementation, both of the values \
                              are partially equivalent, even if the `Debug` outputs differ.\
                            \n\
                            \n",
                             $maybe_semicolon,
                             format_args!($($arg)+),
                             $crate::Comparison::new(left_val, right_val),
                             $crate::Style::new()
                                 .bold()
                                 .underline()
                                 .paint("Note"))
                  }

                  ::std::panic!("assertion failed: `(left != right)`{}{}\
                        \n\
                        \n{}:\
                        \n{:#?}\
                        \n\
                        \n",
                         $maybe_semicolon,
                         format_args!($($arg)+),
                         $crate::Style::new().bold().paint("Both sides"),
                         left_val)
                }
            }
        }
    });
}

#[cfg(test)]
#[allow(clippy::eq_op)]
#[no_implicit_prelude]
mod test {
    mod assert_eq {
        use ::std::string::{String, ToString};

        #[test]
        fn passes() {
            let a = "some value";
            crate::assert_eq!(a, a);
        }

        #[test]
        fn passes_unsized() {
            let a: &[u8] = b"e";
            crate::assert_eq!(*a, *a);
        }

        #[test]
        fn passes_comparable_types() {
            let s0: &'static str = "foo";
            let s1: String = "foo".to_string();
            crate::assert_eq!(s0, s1);
        }

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left == right)`

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m666[0m
[32m>[0m[1;48;5;22;32m999[0m

"#)]
        fn fails() {
            crate::assert_eq!(666, 999);
        }

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left == right)`

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m666[0m
[32m>[0m[1;48;5;22;32m999[0m

"#)]
        fn fails_trailing_comma() {
            crate::assert_eq!(666, 999,);
        }

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left == right)`

[1mDiff[0m [31m< left[0m / [32mright >[0m :
 [
     101,
[32m>    101,[0m
 ]

"#)]
        fn fails_unsized() {
            let a: &[u8] = b"e";
            let b: &[u8] = b"ee";
            crate::assert_eq!(*a, *b);
        }

        #[test]
        #[should_panic(
            expected = r#"assertion failed: `(left == right)`: custom panic message

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m666[0m
[32m>[0m[1;48;5;22;32m999[0m

"#
        )]
        fn fails_custom() {
            crate::assert_eq!(666, 999, "custom panic message");
        }

        #[test]
        #[should_panic(
            expected = r#"assertion failed: `(left == right)`: custom panic message

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m666[0m
[32m>[0m[1;48;5;22;32m999[0m

"#
        )]
        fn fails_custom_trailing_comma() {
            crate::assert_eq!(666, 999, "custom panic message",);
        }
    }

    mod assert_ne {
        use ::std::string::{String, ToString};

        #[test]
        fn passes() {
            let a = "a";
            let b = "b";
            crate::assert_ne!(a, b);
        }

        #[test]
        fn passes_unsized() {
            let a: &[u8] = b"e";
            let b: &[u8] = b"ee";
            crate::assert_ne!(*a, *b);
        }

        #[test]
        fn passes_comparable_types() {
            let s0: &'static str = "foo";
            let s1: String = "bar".to_string();
            crate::assert_ne!(s0, s1);
        }

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left != right)`

[1mBoth sides[0m:
666
"#)]
        fn fails() {
            crate::assert_ne!(666, 666);
        }

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left != right)`

[1mBoth sides[0m:
666
"#)]
        fn fails_trailing_comma() {
            crate::assert_ne!(666, 666,);
        }

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left != right)`

[1mBoth sides[0m:
[
    101,
]

"#)]
        fn fails_unsized() {
            let a: &[u8] = b"e";
            crate::assert_ne!(*a, *a);
        }

        #[test]
        #[should_panic(
            expected = r#"assertion failed: `(left != right)`: custom panic message

[1mBoth sides[0m:
666
"#
        )]
        fn fails_custom() {
            crate::assert_ne!(666, 666, "custom panic message");
        }

        #[test]
        #[should_panic(
            expected = r#"assertion failed: `(left != right)`: custom panic message

[1mBoth sides[0m:
666
"#
        )]
        fn fails_custom_trailing_comma() {
            crate::assert_ne!(666, 666, "custom panic message",);
        }

        // If the values are equal but their debug outputs are not
        // show a specific warning

        #[test]
        #[should_panic(expected = r#"assertion failed: `(left != right)`

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m-[0m[31m0.0[0m
[32m>0.0[0m

[1;4mNote[0m: According to the `PartialEq` implementation, both of the values are partially equivalent, even if the `Debug` outputs differ.

"#)]
        fn assert_ne_partial() {
            // Workaround for https://github.com/rust-lang/rust/issues/47619
            // can be removed, when we require rust 1.25 or higher
            struct Foo(f32);

            use ::std::fmt;
            impl fmt::Debug for Foo {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    ::std::write!(f, "{:.1?}", self.0)
                }
            }

            impl ::std::cmp::PartialEq for Foo {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }

            crate::assert_ne!(Foo(-0.0), Foo(0.0));
        }

        // Regression tests

        #[test]
        #[should_panic]
        fn assert_ne_non_empty_return() {
            fn not_zero(x: u32) -> u32 {
                crate::assert_ne!(x, 0);
                x
            }
            not_zero(0);
        }
    }
}
