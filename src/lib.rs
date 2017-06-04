//! # Pretty Assertions
//!
//! When writing tests in Rust, you'll probably use `assert_eq!(a, b)` _a lot_.
//!
//! If such a test fails, it will present all the details of `a` and `b`, but you have to spot, the differences yourself, which is not always straightforward, like here:
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

#[doc(hidden)]
pub struct Comparison {
    left: String,
    right: String,
    changeset: Changeset
}

impl Comparison {
    pub fn new<TLeft: Debug, TRight: Debug>(left: &TLeft, right: &TRight) -> Comparison {
        let left_dbg = format!("{:#?}", *left);
        let right_dbg = format!("{:#?}", *right);
        let changeset = Changeset::new(&left_dbg, &right_dbg, "\n");

        Comparison {
            left: left_dbg,
            right: right_dbg,
            changeset: changeset,
        }
    }
}

impl Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "left:  `{}`\
                   \nright: `{}`",
               self.left,
               self.right)?;
        format_changeset(f, &self.changeset)
    }
}

#[macro_export]
macro_rules! assert_eq {
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
macro_rules! assert_ne {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!("assertion failed: `(left != right)`\
                          \n\
                          \nleft:  `{:#?}`\
                          \nright: `{:#?}`\
                          \n\
                          \n",
                           left_val,
                           right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!("assertion failed: `(left != right)`: {}\
                          \n\
                          \nleft:  `{:#?}`\
                          \nright: `{:#?}`\
                          \n\
                          \n",
                           format_args!($($arg)+),
                           left_val,
                           right_val)
                }
            }
        }
    });
}
