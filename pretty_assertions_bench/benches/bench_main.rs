use criterion::criterion_main;

mod benchmarks;
mod inputs;

criterion_main! {
    benchmarks::macros::benches,
    benchmarks::pretty_diff::benches,
}
