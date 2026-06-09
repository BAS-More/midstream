//! Benchmarks for the `strange-loop` meta-learning crate.
//!
//! Exercises `StrangeLoop::learn_at_level`, which extracts recurring patterns
//! and propagates meta-knowledge up the hierarchy.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midstreamer_strange_loop::{MetaLevel, StrangeLoop};

fn bench_learn_at_level(c: &mut Criterion) {
    let data: Vec<String> = (0..50).map(|i| format!("pattern{}", i % 8)).collect();

    c.bench_function("strange_loop_learn_at_level", |b| {
        b.iter(|| {
            let mut strange_loop = StrangeLoop::default();
            black_box(strange_loop.learn_at_level(MetaLevel::base(), black_box(&data)))
        });
    });
}

criterion_group!(benches, bench_learn_at_level);
criterion_main!(benches);
