# Unreleased

# v1.2.0

## Changed

- `assert_eq` compares `str`-like values without `Debug` formatting. ([#92](https://github.com/colin-kiegel/rust-pretty-assertions/pull/92), [@dtolnay](https://github.com/dtolnay))

# v1.1.0

## Added

- Add `assert_str_eq` for comparing two `str`-like values without `Debug` formatting. Thanks to [@x3ro](https://github.com/x3ro) for implementing this! ([#88](https://github.com/colin-kiegel/rust-pretty-assertions/pull/88), [@x3ro](https://github.com/x3ro))

## Fixed

- Ensure license text is included in crate archive - thanks [@decathorpe](https://github.com/decathorpe) for reporting this ([#87](https://github.com/colin-kiegel/rust-pretty-assertions/pull/87), [@tommilligan](https://github.com/tommilligan))

# v1.0.0

## Removed

- `assert_ne` no longer warns if values match using `PartialEq` but not with `Debug`. This was noted as no longer being necessary after Rust 1.25 (current MSRV 1.35.0)

## Added

- Officially support `no_std` (thanks to [@Luro02](https://github.com/Luro02) for the report and reviews!). Adds the `std` and `alloc` features to the `pretty_assertions` crate, with `std` enabled by default ([#83](https://github.com/colin-kiegel/rust-pretty-assertions/pull/83), [@tommilligan](https://github.com/tommilligan))
- Adds the `unstable` feature to the `pretty_assertions` crate, for use with nightly rustc ([#81](https://github.com/colin-kiegel/rust-pretty-assertions/pull/81), [@tommilligan](https://github.com/tommilligan))
- Add a drop in replacement for the unstable stdlib `assert_matches` macro, behind the `unstable` flag - thanks [@gilescope](https://github.com/gilescope) for the suggestion! ([#81](https://github.com/colin-kiegel/rust-pretty-assertions/issues/81), [@tommilligan](https://github.com/tommilligan))

# v0.7.2

- Fix macro hygiene for expansion in a `no_implicit_prelude` context ([#70](https://github.com/colin-kiegel/rust-pretty-assertions/issues/70), [@tommilligan](https://github.com/tommilligan))

# v0.7.1

- Fix a bug where multiline changes showed an unhelpful inline diff ([#66](https://github.com/colin-kiegel/rust-pretty-assertions/issues/66), [@tommilligan](https://github.com/tommilligan))

# v0.7.0

## Changed

- Move from `difference` to `diff` for calculating diffs. The exact assertion messages generated may differ from previous versions. ([#52](https://github.com/colin-kiegel/rust-pretty-assertions/issues/52), [@tommilligan](https://github.com/tommilligan))

For example, the following assertion message from `v0.7.0`:

![pretty assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/pretty_assertion.png)

Was previously rendered like this in `v0.6.1`:

![pretty assertion](https://raw.githubusercontent.com/colin-kiegel/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/pretty_assertion_v0_6_1.png)

## Added

- Support for unsized values ([#42](https://github.com/colin-kiegel/rust-pretty-assertions/issues/42), [@stanislav-tkach](https://github.com/stanislav-tkach))
- Document the `Comparison` struct, which was previously hidden. This can be used to generate a pretty diff of two values without panicking. ([#52](https://github.com/colin-kiegel/rust-pretty-assertions/issues/52), [@tommilligan](https://github.com/tommilligan))

## Fixed

- Fix some unhygenic macro expansions ([#41](https://github.com/colin-kiegel/rust-pretty-assertions/issues/41), [@tommilligan](https://github.com/tommilligan))

## Internal

- Test Windows targets in CI ([#46](https://github.com/colin-kiegel/rust-pretty-assertions/issues/46), [@tommilligan](https://github.com/tommilligan))
- Bump `ansi_term` version to 0.12 ([#34](https://github.com/colin-kiegel/rust-pretty-assertions/issues/34), [@waywardmonkeys](https://github.com/waywardmonkeys))
- Code health improvements ([#34](https://github.com/colin-kiegel/rust-pretty-assertions/issues/34), [@waywardmonkeys](https://github.com/waywardmonkeys))
