//! Benchmarks for the `nanosecond-scheduler` crate.
//!
//! Exercises task scheduling and dequeuing on `RealtimeScheduler`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midstreamer_scheduler::{Deadline, Priority, RealtimeScheduler, SchedulerConfig};

fn bench_schedule(c: &mut Criterion) {
    c.bench_function("scheduler_schedule_100", |b| {
        b.iter(|| {
            let scheduler: RealtimeScheduler<u64> =
                RealtimeScheduler::new(SchedulerConfig::default());
            for i in 0..100u64 {
                let _ = black_box(scheduler.schedule(
                    black_box(i),
                    Deadline::from_millis(100),
                    Priority::High,
                ));
            }
            black_box(scheduler.next_task())
        });
    });
}

criterion_group!(benches, bench_schedule);
criterion_main!(benches);
