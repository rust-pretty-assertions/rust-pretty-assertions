# Unreleased

# v1.4.1

## Fixed

- Show feature-flagged code in documentation. Thanks to [@sandydoo](https://github.com/sandydoo) for the fix! ([#130](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/130))

## Internal

- Bump `yansi` version to `1.x`. Thanks to [@SergioBenitez](https://github.com/SergioBenitez) for the update, and maintaining this library! ([#121](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/121))

# v1.4.0

## Changed

- (Windows only) Removed the `ctor` and `output_vt100` dependencies, as their benefit could not be demonstrated ([#118](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/118))

## Fixed

- Minor documentation fixes ([#107](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/107))

# v1.3.0

## Changed

- MSRV bumped to 1.54.0 (see [#102](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/102))
- Removed the publically re-exported `ansi_term::Style`. This was never intended for public use. (see [#102](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/102))

## Fixed

- Moved from the unmaintained `ansi_term` crate to `yansi` for ANSI terminal escape code support. Thanks to [@Roguelazer](https://github.com/Roguelazer) for reporting and fixing this! ([#102](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/102), [@Roguelazer](https://github.com/Roguelazer))

# v1.2.1

## Fixed

- Fixed a panic caused by diffing two `str`-like values where only the left has a trailing newline - thanks [@Michael-F-Bryan](https://github.com/Michael-F-Bryan) for reporting this ([#97](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/97), [@tommilligan](https://github.com/tommilligan))

# v1.2.0

## Changed

- `assert_eq` compares `str`-like values without `Debug` formatting. ([#92](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/92), [@dtolnay](https://github.com/dtolnay))

# v1.1.0

## Added

- Add `assert_str_eq` for comparing two `str`-like values without `Debug` formatting. Thanks to [@x3ro](https://github.com/x3ro) for implementing this! ([#88](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/88), [@x3ro](https://github.com/x3ro))

## Fixed

- Ensure license text is included in crate archive - thanks [@decathorpe](https://github.com/decathorpe) for reporting this ([#87](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/87), [@tommilligan](https://github.com/tommilligan))

# v1.0.0

## Removed

- `assert_ne` no longer warns if values match using `PartialEq` but not with `Debug`. This was noted as no longer being necessary after Rust 1.25 (current MSRV 1.35.0)

## Added

- Officially support `no_std` (thanks to [@Luro02](https://github.com/Luro02) for the report and reviews!). Adds the `std` and `alloc` features to the `pretty_assertions` crate, with `std` enabled by default ([#83](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/83), [@tommilligan](https://github.com/tommilligan))
- Adds the `unstable` feature to the `pretty_assertions` crate, for use with nightly rustc ([#81](https://github.com/rust-pretty-assertions/rust-pretty-assertions/pull/81), [@tommilligan](https://github.com/tommilligan))
- Add a drop in replacement for the unstable stdlib `assert_matches` macro, behind the `unstable` flag - thanks [@gilescope](https://github.com/gilescope) for the suggestion! ([#81](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/81), [@tommilligan](https://github.com/tommilligan))

# v0.7.2

- Fix macro hygiene for expansion in a `no_implicit_prelude` context ([#70](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/70), [@tommilligan](https://github.com/tommilligan))

# v0.7.1

- Fix a bug where multiline changes showed an unhelpful inline diff ([#66](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/66), [@tommilligan](https://github.com/tommilligan))

# v0.7.0

## Changed

- Move from `difference` to `diff` for calculating diffs. The exact assertion messages generated may differ from previous versions. ([#52](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/52), [@tommilligan](https://github.com/tommilligan))

For example, the following assertion message from `v0.7.0`:

![pretty assertion](https://raw.githubusercontent.com/rust-pretty-assertions/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/pretty_assertion.png)

Was previously rendered like this in `v0.6.1`:

![pretty assertion](https://raw.githubusercontent.com/rust-pretty-assertions/rust-pretty-assertions/2d2357ff56d22c51a86b2f1cfe6efcee9f5a8081/examples/pretty_assertion_v0_6_1.png)

## Added

- Support for unsized values ([#42](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/42), [@stanislav-tkach](https://github.com/stanislav-tkach))
- Document the `Comparison` struct, which was previously hidden. This can be used to generate a pretty diff of two values without panicking. ([#52](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/52), [@tommilligan](https://github.com/tommilligan))

## Fixed

- Fix some unhygenic macro expansions ([#41](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/41), [@tommilligan](https://github.com/tommilligan))

## Internal

- Test Windows targets in CI ([#46](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/46), [@tommilligan](https://github.com/tommilligan))
- Bump `ansi_term` version to 0.12 ([#34](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/34), [@waywardmonkeys](https://github.com/waywardmonkeys))
- Code health improvements ([#34](https://github.com/rust-pretty-assertions/rust-pretty-assertions/issues/34), [@waywardmonkeys](https://github.com/waywardmonkeys))
