#[macro_use]
extern crate pretty_assertions;

#[test]
#[should_panic(expected=r#"assertion failed: `(left != right)`

[1mBoth sides[0m:
Some(
    Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok(
            "hey"
        )
    }
)

"#)]
fn assert_ne() {

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});

    assert_ne!(x, x);
}

#[test]
#[should_panic(expected=r#"assertion failed: `(left != right)`: custom panic message

[1mBoth sides[0m:
Some(
    Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok(
            "hey"
        )
    }
)

"#)]
fn assert_ne_custom() {

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});

    assert_ne!(x, x, "custom panic message");
}

#[test]
#[should_panic(expected=r#"assertion failed: `(left != right)`

[1;4mImportant note[0m: Both of the values are partially equivalent (while they are expected not to be), even if the outputs below differ.
Probably the PartialEq trait is the culprit.

[1mLeft[0m:
-0

[1mRight[0m:
0

"#)]
fn assert_ne_partial() {
    assert_ne!(-0.0, 0.0);
}
