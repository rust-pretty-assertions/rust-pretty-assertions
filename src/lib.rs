//! # Pretty Assertions
//!
//! When writing tests in Rust, you'll probably use `assert_eq!(a, b)` _a lot_.
//!
//! If such a test fails, it will present all the details of `a` and `b`. 
//! But you have to spot the differences yourself, which is not always straightforward,
//! like here:
//!
//! ![standard assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/1a7feb17e1dfbeabfac91b0d5a9cb78dfb1bc065/examples/standard_assertion.png)
//!
//! Wouldn't that task be _much_ easier with a colorful diff?
//!
//! ![pretty assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/1a7feb17e1dfbeabfac91b0d5a9cb78dfb1bc065/examples/pretty_assertion.png)
//!
//! Yep — and you only need **one line of code** to make it happen:
//!
//! ```rust,ignore
//! #[macro_use] extern crate pretty_assertions;
//! ```
//!
//! <details>
//! <summary>Show the example behind the screenshots above.</summary>
//!
//! ```rust,ignore
//! // 1. add the `pretty_assertions` dependency to `Cargo.toml`.
//! // 2. insert this line at the top of your crate root or integration test
//! #[macro_use] extern crate pretty_assertions;
//!
//! fn main() {
//!     #[derive(Debug, PartialEq)]
//!     struct Foo {
//!         lorem: &'static str,
//!         ipsum: u32,
//!         dolor: Result<String, String>,
//!     }
//!
//!     let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
//!     let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});
//!
//!     assert_eq!(x, y);
//! }
//! ```
//! </details>
//!
//! ## Tip
//!
//! Specify it as [`[dev-dependencies]`](http://doc.crates.io/specifying-dependencies.html#development-dependencies)
//! and it will only be used for compiling tests, examples, and benchmarks.
//! This way the compile time of `cargo build` won't be affected!
//!
//! In your crate root, also add `#[cfg(test)]` to the crate import, like this:
//!
//! ```rust,ignore
//! #[cfg(test)] // <-- not needed in examples + integration tests
//! #[macro_use]
//! extern crate pretty_assertions;
//! ```
//!
//! ## Note
//!
//! * Each example and integration test also needs `#[macro_use] extern crate
//!   pretty_assertions`, if you want colorful diffs there.
//! * The replacement is only effective in your own crate, not in other libraries
//!   you include.
//! * `assert_ne` is also switched to multi-line presentation, but does _not_ show
//!   a diff.

extern crate difference;
extern crate ansi_term;
mod format_changeset;

use std::fmt::{self, Debug, Display};
use difference::Changeset;

use format_changeset::format_changeset;
pub use ansi_term::Style;

#[doc(hidden)]
pub struct Comparison(Changeset);

impl Comparison {
    pub fn new<TLeft: Debug, TRight: Debug>(left: &TLeft, right: &TRight) -> Comparison {
        let left_dbg = format!("{:#?}", *left);
        let right_dbg = format!("{:#?}", *right);
        let changeset = Changeset::new(&left_dbg, &right_dbg, "\n");

        Comparison(changeset)
    }
}

impl Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_changeset(f, &self.0)
    }
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr , $right:expr,) => ({
        assert_eq!($left, $right)
    });
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)`\
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
                    panic!("assertion failed: `(left == right)`: {}\
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

#[macro_export]
#[doc(hidden)]
macro_rules! __assert_ne {
    ($left:expr, $right:expr, $maybe_semicolon:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                  let left_dbg = format!("{:?}", *left_val);
                  let right_dbg = format!("{:?}", *right_val);
                  if left_dbg != right_dbg {

                      panic!("assertion failed: `(left != right)`{}{}\
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

                  panic!("assertion failed: `(left != right)`{}{}\
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

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => ({
        __assert_ne!($left, $right, "", "");
    });
    ($left:expr, $right:expr,) => ({
        __assert_ne!($left, $right, "", "");
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        __assert_ne!($left, $right, ": ", $($arg)+);
    });
}
