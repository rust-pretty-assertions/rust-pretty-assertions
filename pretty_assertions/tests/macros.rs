#![no_implicit_prelude]

#[allow(clippy::eq_op)]
mod assert_eq {
    use ::std::string::{String, ToString};

    #[test]
    fn passes() {
        let a = "some value";
        ::pretty_assertions::assert_eq!(a, a);
    }

    #[test]
    fn passes_unsized() {
        let a: &[u8] = b"e";
        ::pretty_assertions::assert_eq!(*a, *a);
    }

    #[test]
    fn passes_comparable_types() {
        let s0: &'static str = "foo";
        let s1: String = "foo".to_string();
        ::pretty_assertions::assert_eq!(s0, s1);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left == right)`

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m666[0m
[32m>[0m[1;48;5;22;32m999[0m

"#)]
    fn fails() {
        ::pretty_assertions::assert_eq!(666, 999);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left == right)`

[1mDiff[0m [31m< left[0m / [32mright >[0m :
[31m<[0m[1;48;5;52;31m666[0m
[32m>[0m[1;48;5;22;32m999[0m

"#)]
    fn fails_trailing_comma() {
        ::pretty_assertions::assert_eq!(666, 999,);
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
        ::pretty_assertions::assert_eq!(*a, *b);
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
        ::pretty_assertions::assert_eq!(666, 999, "custom panic message");
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
        ::pretty_assertions::assert_eq!(666, 999, "custom panic message",);
    }
}

mod assert_ne {
    use ::std::string::{String, ToString};

    #[test]
    fn passes() {
        let a = "a";
        let b = "b";
        ::pretty_assertions::assert_ne!(a, b);
    }

    #[test]
    fn passes_unsized() {
        let a: &[u8] = b"e";
        let b: &[u8] = b"ee";
        ::pretty_assertions::assert_ne!(*a, *b);
    }

    #[test]
    fn passes_comparable_types() {
        let s0: &'static str = "foo";
        let s1: String = "bar".to_string();
        ::pretty_assertions::assert_ne!(s0, s1);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left != right)`

[1mBoth sides[0m:
666
"#)]
    fn fails() {
        ::pretty_assertions::assert_ne!(666, 666);
    }

    #[test]
    #[should_panic(expected = r#"assertion failed: `(left != right)`

[1mBoth sides[0m:
666
"#)]
    fn fails_trailing_comma() {
        ::pretty_assertions::assert_ne!(666, 666,);
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
        ::pretty_assertions::assert_ne!(*a, *a);
    }

    #[test]
    #[should_panic(
        expected = r#"assertion failed: `(left != right)`: custom panic message

[1mBoth sides[0m:
666
"#
    )]
    fn fails_custom() {
        ::pretty_assertions::assert_ne!(666, 666, "custom panic message");
    }

    #[test]
    #[should_panic(
        expected = r#"assertion failed: `(left != right)`: custom panic message

[1mBoth sides[0m:
666
"#
    )]
    fn fails_custom_trailing_comma() {
        ::pretty_assertions::assert_ne!(666, 666, "custom panic message",);
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

        ::pretty_assertions::assert_ne!(Foo(-0.0), Foo(0.0));
    }

    // Regression tests

    #[test]
    #[should_panic]
    fn assert_ne_non_empty_return() {
        fn not_zero(x: u32) -> u32 {
            ::pretty_assertions::assert_ne!(x, 0);
            x
        }
        not_zero(0);
    }
}
