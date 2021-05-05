use crate::inputs::{BLOCK_0, BLOCK_1, CHAR_0, CHAR_1, STRING_0, STRING_1};
use criterion::{criterion_group, criterion_main, Criterion};

/// assert_eq hot path
pub fn assert_eq_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("assert_eq pass");

    group.bench_with_input("character", &(CHAR_0, CHAR_0), |b, (left, right)| {
        b.iter(|| pretty_assertions::assert_eq!(left, right))
    });

    group.bench_with_input(
        "64 char string",
        &(STRING_0, STRING_0),
        |b, (left, right)| b.iter(|| pretty_assertions::assert_eq!(left, right)),
    );

    group.bench_with_input(
        "64x64 char block",
        &(BLOCK_0, BLOCK_0),
        |b, (left, right)| b.iter(|| pretty_assertions::assert_eq!(left, right)),
    );

    group.finish();
}

/// assert_ne hot path
pub fn assert_ne_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("assert_ne pass");

    group.bench_with_input("character", &(CHAR_0, CHAR_1), |b, (left, right)| {
        b.iter(|| pretty_assertions::assert_ne!(left, right))
    });

    group.bench_with_input(
        "64 char string",
        &(STRING_0, STRING_1),
        |b, (left, right)| b.iter(|| pretty_assertions::assert_ne!(left, right)),
    );

    group.bench_with_input(
        "64x64 char block",
        &(BLOCK_0, BLOCK_1),
        |b, (left, right)| b.iter(|| pretty_assertions::assert_ne!(left, right)),
    );

    group.finish();
}

/// assert_eq hot path (stdlib implementation, for comparison)
pub fn assert_eq_std_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("assert_eq std pass");

    group.bench_with_input("character", &(CHAR_0, CHAR_0), |b, (left, right)| {
        b.iter(|| assert_eq!(left, right))
    });

    group.bench_with_input(
        "64 char string",
        &(STRING_0, STRING_0),
        |b, (left, right)| b.iter(|| assert_eq!(left, right)),
    );

    group.bench_with_input(
        "64x64 char block",
        &(BLOCK_0, BLOCK_0),
        |b, (left, right)| b.iter(|| assert_eq!(left, right)),
    );

    group.finish();
}

/// assert_ne hot path (stdlib implementation, for comparison)
pub fn assert_ne_std_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("assert_ne std pass");

    group.bench_with_input("character", &(CHAR_0, CHAR_1), |b, (left, right)| {
        b.iter(|| assert_ne!(left, right))
    });

    group.bench_with_input(
        "64 char string",
        &(STRING_0, STRING_1),
        |b, (left, right)| b.iter(|| assert_ne!(left, right)),
    );

    group.bench_with_input(
        "64x64 char block",
        &(BLOCK_0, BLOCK_1),
        |b, (left, right)| b.iter(|| assert_ne!(left, right)),
    );

    group.finish();
}

criterion_group!(
    benches,
    assert_eq_pass,
    assert_ne_pass,
    assert_eq_std_pass,
    assert_ne_std_pass
);
criterion_main!(benches);
