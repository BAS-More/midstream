//! Benchmarks for the `temporal-compare` crate.
//!
//! Exercises the public `TemporalComparator` API across the available
//! comparison algorithms.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midstreamer_temporal_compare::{ComparisonAlgorithm, Sequence, TemporalComparator};

fn make_sequence(len: usize, offset: i64) -> Sequence<i64> {
    let mut seq = Sequence::new();
    for i in 0..len {
        seq.push(i as i64 + offset, i as u64);
    }
    seq
}

fn bench_algorithms(c: &mut Criterion) {
    let comparator = TemporalComparator::<i64>::new(1024, 10_000);
    let seq_a = make_sequence(100, 0);
    let seq_b = make_sequence(100, 1);

    let mut group = c.benchmark_group("temporal_compare");
    for algorithm in [
        ComparisonAlgorithm::DTW,
        ComparisonAlgorithm::LCS,
        ComparisonAlgorithm::EditDistance,
        ComparisonAlgorithm::Euclidean,
    ] {
        group.bench_function(format!("{algorithm:?}"), |b| {
            b.iter(|| {
                black_box(comparator.compare(black_box(&seq_a), black_box(&seq_b), algorithm))
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_algorithms);
criterion_main!(benches);
