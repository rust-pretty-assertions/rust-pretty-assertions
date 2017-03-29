#[macro_use]
extern crate pretty_assertions;

#[test]
#[should_panic(expected="assertion failed: `(left == right)` (left: `Some(Foo { lorem: \"Hello World!\", ipsum: 42, dolor: Ok(\"hey\") })`, right: `Some(Foo { lorem: \"Hello Wrold!\", ipsum: 42, dolor: Ok(\"hey ho!\") })`, diff: `Some(Foo { lorem: \"Hello [91mWorld!\",[0m [92mWrold!\",[0m ipsum: 42, dolor: [91mOk(\"hey\")[0m [92mOk(\"hey ho!\")[0m }) `)")]
fn assert_struct() {

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
    let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});

    assert_eq!(x, y);
}

#[test]
#[should_panic(expected="assertion failed: `(left == right)` (left: `\"Hello World!\"`, right: `\"Hello Wrold!\"`, diff: `\"Hello [91mWorld!\"[0m [92mWrold!\"[0m `): not good: Hello World! != Hello Wrold!")]
fn assert_args() {

    let x = "Hello World!";
    let y = "Hello Wrold!";

    assert_eq!(x, y, "not good: {} != {}", x, y);
}
