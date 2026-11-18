#![cfg_attr(not(feature = "std"), no_std)]
#![no_implicit_prelude]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[test]
fn comparison_formats_without_std() {
    use ::core::fmt::Write;
    use ::core::write;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    use ::alloc::string::String;
    #[cfg(feature = "std")]
    use ::std::string::String;

    let mut buf = String::new();
    write!(buf, "{}", ::pretty_assertions::Comparison::new(&123, &124)).unwrap();
    ::core::assert!(buf.contains("12"));

    buf.clear();
    write!(
        buf,
        "{}",
        ::pretty_assertions::StrComparison::new("foo\nbar", "foo\nbaz")
    )
    .unwrap();
    ::core::assert!(buf.contains("foo"));
}

#[cfg(feature = "std")]
#[test]
fn alloc_feature_does_not_enable_std_dependencies() {
    use ::core::assert;
    use ::std::process::Command;
    use ::std::str;

    let output = Command::new(::core::env!("CARGO"))
        .args([
            "tree",
            "--manifest-path",
            ::core::concat!(::core::env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
            "-e",
            "features",
            "--no-default-features",
            "--features",
            "alloc",
        ])
        .output()
        .expect("failed to execute cargo tree");

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).expect("utf-8 output");
    assert!(
        !stdout.contains("diff v"),
        "unexpected `diff` dependency in alloc tree:\n{}",
        stdout
    );
    assert!(
        !stdout.contains("feature \"std\""),
        "unexpected `std` feature in alloc tree:\n{}",
        stdout
    );
    assert!(
        !stdout.contains("feature \"default\""),
        "unexpected `default` feature in alloc tree:\n{}",
        stdout
    );
}
