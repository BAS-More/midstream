//! Benchmarks for the `temporal-neural-solver` crate.
//!
//! Exercises the public `TemporalNeuralSolver` API: building a trace of states
//! and verifying temporal-logic formulas against it.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midstreamer_neural_solver::{
    TemporalFormula, TemporalNeuralSolver, TemporalState, VerificationStrictness,
};

fn build_solver(n: usize) -> TemporalNeuralSolver {
    let mut solver = TemporalNeuralSolver::new(n + 1, 1000, VerificationStrictness::Medium);
    for i in 0..n {
        let mut state = TemporalState::new(i as u64, i as u64);
        state.set_proposition("p", i % 2 == 0);
        state.set_proposition("q", i % 3 == 0);
        solver.add_state(state);
    }
    solver
}

fn bench_verify(c: &mut Criterion) {
    let solver = build_solver(200);
    let formula = TemporalFormula::and(
        TemporalFormula::atom("p"),
        TemporalFormula::not(TemporalFormula::atom("q")),
    );

    c.bench_function("solver_verify_200", |b| {
        b.iter(|| black_box(solver.verify(black_box(&formula))));
    });
}

criterion_group!(benches, bench_verify);
criterion_main!(benches);
