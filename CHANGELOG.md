# Unreleased

# v0.7.0

## Changed

- Move from `difference` to `diff` for calculating diffs. The exact assertion messages generated may differ from previous versions. ([#52](https://github.com/colin-kiegel/rust-pretty-assertions/issues/52), [@tommilligan](https://github.com/tommilligan))

## Added

- Support for unsized values ([#42](https://github.com/colin-kiegel/rust-pretty-assertions/issues/42), [@stanislav-tkach](https://github.com/stanislav-tkach))
- Document the `Comparison` struct, which was previously hidden. This can be used to generate a pretty diff of two values without panicking. ([#52](https://github.com/colin-kiegel/rust-pretty-assertions/issues/52), [@tommilligan](https://github.com/tommilligan))

## Fixed

- Fix some unhygenic macro expansions ([#41](https://github.com/colin-kiegel/rust-pretty-assertions/issues/41), [@tommilligan](https://github.com/tommilligan))

## Internal

- Test Windows targets in CI ([#46](https://github.com/colin-kiegel/rust-pretty-assertions/issues/46), [@tommilligan](https://github.com/tommilligan))
- Bump `ansi_term` version to 0.12 ([#34](https://github.com/colin-kiegel/rust-pretty-assertions/issues/34), [@waywardmonkeys](https://github.com/waywardmonkeys))
- Code health improvements ([#34](https://github.com/colin-kiegel/rust-pretty-assertions/issues/34), [@waywardmonkeys](https://github.com/waywardmonkeys))
