#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};

use maybe_unwind::maybe_unwind;

use std::fmt;
/// Wrapper around string slice that makes debug output `{:?}` to print string same way as `{}`.
/// Used in different `assert*!` macros in combination with `pretty_assertions` crate to make
/// test failures to show nice diffs.
#[derive(PartialEq, Eq)]
#[doc(hidden)]
pub struct PrettyString<'a>(pub &'a str);

/// Make diff to display string as multi-line string
impl<'a> fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

fn test_setup() {
    // set panic() hook for maybe_unwind
    static SET_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    SET_PANIC_HOOK.call_once(maybe_unwind::set_hook);
}

#[test]
fn assert_eq_empty_first() {
    test_setup();

    let expect_template = r#"assertion failed: `({{left}} == {{right}})`

[1mDiff[0m [31m{{<}} {{left}}[0m / [32m{{right}} {{>}}[0m :
[32m{{>}}foo
[0m
"#;

    let mut expect = expect_template.to_string();

    expect = expect.replace("{{<}}", "<").replace("{{>}}", ">");
    expect = expect
        .replace("{{left}}", "left")
        .replace("{{right}}", "right");

    let result = maybe_unwind(|| {
        assert_eq!(PrettyString(""), PrettyString("foo"));
    });

    assert!(true, result.is_err());

    let result = result.unwrap_err().payload_str().to_owned();
    println!("expect={}", expect);
    println!("result={}", result);
    assert_eq!(expect, result);
}
