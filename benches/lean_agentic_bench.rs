//! Benchmarks for the Lean Agentic Learning System.
//!
//! Exercises the end-to-end `process_stream_chunk` path of `LeanAgenticSystem`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midstream::{AgentContext, LeanAgenticConfig, LeanAgenticSystem};
use tokio::runtime::Runtime;

fn bench_process_stream_chunk(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let system = LeanAgenticSystem::new(LeanAgenticConfig::default());

    c.bench_function("process_stream_chunk", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let context = AgentContext::new("bench_session".to_string());
                black_box(
                    system
                        .process_stream_chunk(black_box("What is the weather today?"), context)
                        .await,
                )
            })
        });
    });
}

criterion_group!(benches, bench_process_stream_chunk);
criterion_main!(benches);
