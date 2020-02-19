#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};

use pretty_assertions::with_labels_assert_eq;

use maybe_unwind::maybe_unwind;

fn test_setup() {
    // set panic() hook for maybe_unwind
    static SET_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    SET_PANIC_HOOK.call_once(maybe_unwind::set_hook);
}

#[test]
fn assert_eq() {
    test_setup();

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok("hey".to_string()),
    });
    let y = Some(Foo {
        lorem: "Hello Wrold!",
        ipsum: 42,
        dolor: Ok("hey ho!".to_string()),
    });

    let expect_template = r#"assertion failed: `({{left}} == {{right}})`

[1mDiff[0m [31m{{<}} {{left}}[0m / [32m{{right}} {{>}}[0m :
 Some(
     Foo {
[31m{{<}}[0m[31m        lorem: "Hello W[0m[31mo[0m[1;48;5;52;31mr[0m[31mld!",[0m
[32m{{>}}[0m[32m        lorem: "Hello W[0m[1;48;5;22;32mr[0m[32mo[0m[32mld!",[0m
         ipsum: 42,
         dolor: Ok(
[31m{{<}}[0m[31m            "hey[0m[31m",[0m
[32m{{>}}[0m[32m            "hey[0m[1;48;5;22;32m ho![0m[32m",[0m
         ),
     },
 )

"#;

    let mut expect = expect_template.to_string();

    expect = expect.replace("{{<}}", "<").replace("{{>}}", ">");

    #[cfg(not(any(feature = "labels")))]
    {
        expect = expect
            .replace("{{left}}", "left")
            .replace("{{right}}", "right");
    }
    #[cfg(feature = "labels")]
    {
        expect = expect.replace("{{left}}", "x").replace("{{right}}", "y");
    }

    let result = maybe_unwind(|| {
        assert_eq!(x, y);
    });

    assert!(true, result.is_err());

    let result = result.unwrap_err().payload_str().to_owned();
    println!("expect={}", expect);
    println!("result={}", result);
    with_labels_assert_eq!(expect, result);
}

#[test]
fn assert_eq_custom() {
    test_setup();

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok("hey".to_string()),
    });
    let y = Some(Foo {
        lorem: "Hello Wrold!",
        ipsum: 42,
        dolor: Ok("hey ho!".to_string()),
    });

    let expect_template = r#"assertion failed: `({{left}} == {{right}})`: custom panic message

[1mDiff[0m [31m{{<}} {{left}}[0m / [32m{{right}} {{>}}[0m :
 Some(
     Foo {
[31m{{<}}[0m[31m        lorem: "Hello W[0m[31mo[0m[1;48;5;52;31mr[0m[31mld!",[0m
[32m{{>}}[0m[32m        lorem: "Hello W[0m[1;48;5;22;32mr[0m[32mo[0m[32mld!",[0m
         ipsum: 42,
         dolor: Ok(
[31m{{<}}[0m[31m            "hey[0m[31m",[0m
[32m{{>}}[0m[32m            "hey[0m[1;48;5;22;32m ho![0m[32m",[0m
         ),
     },
 )

"#;

    let mut expect = expect_template.to_string();

    expect = expect.replace("{{<}}", "<").replace("{{>}}", ">");

    #[cfg(not(any(feature = "labels")))]
    {
        expect = expect
            .replace("{{left}}", "left")
            .replace("{{right}}", "right");
    }
    #[cfg(feature = "labels")]
    {
        expect = expect.replace("{{left}}", "x").replace("{{right}}", "y");
    }

    let result = maybe_unwind(|| {
        assert_eq!(x, y, "custom panic message");
    });

    assert!(true, result.is_err());

    let result = result.unwrap_err().payload_str().to_owned();
    println!("expect={}", expect);
    println!("result={}", result);
    with_labels_assert_eq!(expect, result);
}

#[test]
fn assert_eq_with_comparable_types() {
    let s0: &'static str = "foo";
    let s1: String = "foo".to_string();
    assert_eq!(s0, s1);
}

#[test]
fn issue12() {
    test_setup();

    let left = vec![0, 0, 0, 128, 10, 191, 5, 64];
    let right = vec![84, 248, 45, 64];

    let expect_template = r#"assertion failed: `({{left}} == {{right}})`

[1mDiff[0m [31m{{<}} {{left}}[0m / [32m{{right}} {{>}}[0m :
 [
[31m{{<}}[0m[31m    [0m[1;48;5;52;31m0[0m[31m,[0m
[31m{{<}}[0m[31m    [0m[1;48;5;52;31m0,[0m
[1;31m{{<}}[0m[1;48;5;52;31m    0,[0m
[1;31m{{<}}[0m[1;48;5;52;31m    1[0m[31m2[0m[31m8,[0m
[31m{{<}}[0m[31m    [0m[1;48;5;52;31m10,[0m
[1;31m{{<}}[0m[1;48;5;52;31m    191,[0m
[1;31m{{<}}[0m[1;48;5;52;31m    [0m[31m5,[0m
[32m{{>}}[0m[32m    [0m[1;48;5;22;32m84[0m[32m,[0m
[32m{{>}}[0m[32m    [0m[32m2[0m[1;48;5;22;32m4[0m[32m8,[0m
[32m{{>}}[0m[32m    [0m[1;48;5;22;32m4[0m[32m5,[0m
     64,
 ]

"#;

    let mut expect = expect_template.to_string();

    expect = expect.replace("{{<}}", "<").replace("{{>}}", ">");

    #[cfg(not(any(feature = "labels")))]
    {
        expect = expect
            .replace("{{left}}", "left")
            .replace("{{right}}", "right");
    }
    #[cfg(feature = "labels")]
    {
        expect = expect
            .replace("{{left}}", "left")
            .replace("{{right}}", "right");
    }

    let result = maybe_unwind(|| {
        println!(" left={:#?}", left);
        println!("right={:#?}", right);
        assert_eq!(left, right);
    });

    assert!(true, result.is_err());

    let result = result.unwrap_err().payload_str().to_owned();
    println!("expect={}", expect);
    println!("result={}", result);
    with_labels_assert_eq!(expect, result);
}

#[test]
fn assert_eq_trailing_comma() {
    test_setup();

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok("hey".to_string()),
    });
    let y = Some(Foo {
        lorem: "Hello Wrold!",
        ipsum: 42,
        dolor: Ok("hey ho!".to_string()),
    });

    let expect_template = r#"assertion failed: `({{left}} == {{right}})`

[1mDiff[0m [31m{{<}} {{left}}[0m / [32m{{right}} {{>}}[0m :
 Some(
     Foo {
[31m{{<}}[0m[31m        lorem: "Hello W[0m[31mo[0m[1;48;5;52;31mr[0m[31mld!",[0m
[32m{{>}}[0m[32m        lorem: "Hello W[0m[1;48;5;22;32mr[0m[32mo[0m[32mld!",[0m
         ipsum: 42,
         dolor: Ok(
[31m{{<}}[0m[31m            "hey[0m[31m",[0m
[32m{{>}}[0m[32m            "hey[0m[1;48;5;22;32m ho![0m[32m",[0m
         ),
     },
 )

"#;

    let mut expect = expect_template.to_string();

    expect = expect.replace("{{<}}", "<").replace("{{>}}", ">");

    #[cfg(not(any(feature = "labels")))]
    {
        expect = expect
            .replace("{{left}}", "left")
            .replace("{{right}}", "right");
    }
    #[cfg(feature = "labels")]
    {
        expect = expect.replace("{{left}}", "x").replace("{{right}}", "y");
    }

    let result = maybe_unwind(|| {
        assert_eq!(x, y,);
    });

    assert!(true, result.is_err());

    let result = result.unwrap_err().payload_str().to_owned();
    println!("expect={}", expect);
    println!("result={}", result);
    with_labels_assert_eq!(expect, result);
}

#[test]
fn assert_eq_custom_trailing_comma() {
    test_setup();

    #[derive(Debug, PartialEq)]
    struct Foo {
        lorem: &'static str,
        ipsum: u32,
        dolor: Result<String, String>,
    }

    let x = Some(Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok("hey".to_string()),
    });
    let y = Some(Foo {
        lorem: "Hello Wrold!",
        ipsum: 42,
        dolor: Ok("hey ho!".to_string()),
    });

    let expect_template = r#"assertion failed: `({{left}} == {{right}})`: custom panic message

[1mDiff[0m [31m{{<}} {{left}}[0m / [32m{{right}} {{>}}[0m :
 Some(
     Foo {
[31m{{<}}[0m[31m        lorem: "Hello W[0m[31mo[0m[1;48;5;52;31mr[0m[31mld!",[0m
[32m{{>}}[0m[32m        lorem: "Hello W[0m[1;48;5;22;32mr[0m[32mo[0m[32mld!",[0m
         ipsum: 42,
         dolor: Ok(
[31m{{<}}[0m[31m            "hey[0m[31m",[0m
[32m{{>}}[0m[32m            "hey[0m[1;48;5;22;32m ho![0m[32m",[0m
         ),
     },
 )

"#;

    let mut expect = expect_template.to_string();

    expect = expect.replace("{{<}}", "<").replace("{{>}}", ">");

    #[cfg(not(any(feature = "labels")))]
    {
        expect = expect
            .replace("{{left}}", "left")
            .replace("{{right}}", "right");
    }
    #[cfg(feature = "labels")]
    {
        expect = expect.replace("{{left}}", "x").replace("{{right}}", "y");
    }

    let result = maybe_unwind(|| {
        assert_eq!(x, y, "custom panic message",);
    });

    assert!(true, result.is_err());

    let result = result.unwrap_err().payload_str().to_owned();
    println!("expect={}", expect);
    println!("result={}", result);
    with_labels_assert_eq!(expect, result);
}
