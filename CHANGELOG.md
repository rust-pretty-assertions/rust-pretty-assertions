# Unreleased

## Changed

- Move from `difference` to `diff` for calculating diffs. The exact assertion messages generated may differ from previous versions. (#52, @tommilligan)

## Added

- Support for unsized values (#42, @stanislav-tkach)
- Document the `Comparison` struct, which was previously hidden. This can be used to generate a pretty diff of two values without panicking. (#52, @tommilligan)

## Fixed

- Fix some unhygenic macro expansions (#41, @tommilligan)

## Internal

- Test Windows targets in CI (#46, @tommilligan)
- Bump `ansi_term` version to 0.12 (#34, @waywardmonkeys)
- Code health improvements (#34, @waywardmonkeys)
