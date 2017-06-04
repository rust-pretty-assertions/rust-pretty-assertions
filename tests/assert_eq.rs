#[macro_use]
extern crate pretty_assertions;

#[test]
#[should_panic(expected=r#"assertion failed: `(left == right)`

[1mDiff[0m ([31m- left[0m / [32m+ right[0m):
 Some(
     Foo {
[31m-        lorem: "Hello World!",
[0m[32m+[0m[32m        lorem: "Hello W[0m[32mr[0m[4;32mo[0m[32mld!",[0m
         ipsum: 42,
         dolor: Ok(
[31m-            "hey"
[0m[32m+[0m[32m            "hey[0m[4;32m ho![0m[32m"[0m
         )
     }
 )

"#)]
fn assert_eq() {

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
#[should_panic(expected=r#"assertion failed: `(left == right)`: custom panic message

[1mDiff[0m ([31m- left[0m / [32m+ right[0m):
 Some(
     Foo {
[31m-        lorem: "Hello World!",
[0m[32m+[0m[32m        lorem: "Hello W[0m[32mr[0m[4;32mo[0m[32mld!",[0m
         ipsum: 42,
         dolor: Ok(
[31m-            "hey"
[0m[32m+[0m[32m            "hey[0m[4;32m ho![0m[32m"[0m
         )
     }
 )

"#)]
fn assert_eq_custom() {

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo { lorem: "Hello World!", ipsum: 42, dolor: Ok("hey".to_string())});
    let y = Some(Foo { lorem: "Hello Wrold!", ipsum: 42, dolor: Ok("hey ho!".to_string())});

    assert_eq!(x, y, "custom panic message");
}

#[test]
fn assert_eq_with_comparable_types() {
	let s0: &'static str = "foo";
	let s1: String = "foo".to_string();
	assert_eq!(s0, s1);
}
