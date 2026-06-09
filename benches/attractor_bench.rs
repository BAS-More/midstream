//! Benchmarks for the `temporal-attractor-studio` crate.
//!
//! Exercises the public `AttractorAnalyzer` API: building a trajectory from
//! phase-space points and running attractor analysis over it.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midstreamer_attractor::{AttractorAnalyzer, PhasePoint};

fn phase_point(i: usize) -> PhasePoint {
    let t = i as f64 * 0.1;
    PhasePoint::new(vec![t.sin(), t.cos(), (t * 0.5).sin()], i as u64)
}

fn bench_add_points(c: &mut Criterion) {
    c.bench_function("attractor_add_points_200", |b| {
        b.iter(|| {
            let mut analyzer = AttractorAnalyzer::new(3, 256);
            for i in 0..200 {
                let _ = black_box(analyzer.add_point(phase_point(i)));
            }
            black_box(analyzer)
        });
    });
}

fn bench_analyze(c: &mut Criterion) {
    let mut analyzer = AttractorAnalyzer::new(3, 256);
    for i in 0..200 {
        let _ = analyzer.add_point(phase_point(i));
    }

    c.bench_function("attractor_analyze_200", |b| {
        b.iter(|| black_box(analyzer.analyze()));
    });
}

criterion_group!(benches, bench_add_points, bench_analyze);
criterion_main!(benches);
