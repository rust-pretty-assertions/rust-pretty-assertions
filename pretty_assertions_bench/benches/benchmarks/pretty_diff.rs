use crate::inputs::{BLOCK_0, BLOCK_1, CHAR_0, CHAR_1, STRING_0, STRING_1};
use criterion::{criterion_group, criterion_main, Criterion};
use pretty_assertions::Comparison;
use std::io::{Result, Write};

/// A writer that throws away all data passed to it.
struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// We only care about comparing different items, as when items are the same
/// we never compute the diff.
pub fn diff_different(c: &mut Criterion) {
    let mut group = c.benchmark_group("diff different");

    let values = (CHAR_0, CHAR_1);
    group.bench_with_input("character", &values, |b, (left, right)| {
        let mut f = NullWriter {};
        b.iter(|| write!(f, "{}", Comparison::new(left, right)))
    });

    let values = (STRING_0, STRING_1);
    group.bench_with_input("64 char string", &values, |b, (left, right)| {
        let mut f = NullWriter {};
        b.iter(|| write!(f, "{}", Comparison::new(left, right)))
    });

    let values = (BLOCK_0, BLOCK_1);
    group.bench_with_input("64x64 char block", &values, |b, (left, right)| {
        let mut f = NullWriter {};
        b.iter(|| write!(f, "{}", Comparison::new(left, right)))
    });

    group.finish();
}

criterion_group!(benches, diff_different);
criterion_main!(benches);
