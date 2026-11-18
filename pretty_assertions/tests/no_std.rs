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
