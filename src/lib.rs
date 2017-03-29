//! # Pretty Assertions
//!
//! When writing tests in Rust, you'll probably use `assert_eq!(a, b)` _a lot_.
//!
//! If such a test fails, it will present all the details of `a` and `b`, but you have to spot, the differences yourself, which is not always straightforward, like here:
//!
//! ![standard assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/162f407eb8f627185ce70c0475a661b7480f0038/examples/standard_assertion.png)
//!
//! Wouldn't that task be _much_ easier with a colorful diff?
//!
//! ![pretty assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/162f407eb8f627185ce70c0475a661b7480f0038/examples/pretty_assertion.png)
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
//! Specify it as [`[dev-dependency]`](http://doc.crates.io/specifying-dependencies.html#development-dependencies)
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
//! * `pretty_assertions` is an ultra-thin wrapper around the
//!   [`difference`](https://crates.io/crates/difference) crate, which does the
//!   heavy lifting. All that `pretty_assertions` does is to replace the
//!   `assert_eq!` macro with just about 22 lines of code.

extern crate difference;

#[doc(hidden)]
pub use difference::Changeset;

#[macro_export]
macro_rules! assert_eq {
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_dbg = format!("{:?}", *left_val);
                    let right_dbg = format!("{:?}", *right_val);
                    let diff = $crate::Changeset::new(&left_dbg, &right_dbg, " ");

                    panic!("assertion failed: `(left == right)` \
                           (left: `{}`, right: `{}`, diff: `{}`)", left_dbg, right_dbg, diff)
                }
            }
        }
    });
    ($left:expr , $right:expr , $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_dbg = format!("{:?}", *left_val);
                    let right_dbg = format!("{:?}", *right_val);
                    let diff = $crate::Changeset::new(&left_dbg, &right_dbg, " ");

                    panic!("assertion failed: `(left == right)` \
                           (left: `{}`, right: `{}`, diff: `{}`): {}",
                           left_dbg, right_dbg, diff, format_args!($($arg)+))
                }
            }
        }
    });
}
