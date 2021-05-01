# Benchmarks for `pretty_assertions`

To avoid pulling in many, many additional development dependencies, the benchmarks for `pretty_assertions` are in a separate crate.

## Usage

To run the benchmarks, run `cargo bench`. A HTML report will be generated.

```bash
# Run benchmarks
cargo bench

# View report
xdg-open target/criterion/report/index.html
```

### Comparing two builds

Running the benchmarks again will update the report, showing the difference between the most recent two sets of results.

```bash
# To compare to a different build, checkout an rerun the benchmarks
git checkout "$MY_BRANCH_NAME"
cargo bench
xdg-open target/criterion/report/index.html
```

You may like to save a stable branch banchmark under a name, to compare other branches to:

```bash
git checkout main
cargo bench --bench bench_main -- --save-baseline main
git checkout "$MY_BRANCH_NAME"
cargo bench --bench bench_main -- --baseline main
```
