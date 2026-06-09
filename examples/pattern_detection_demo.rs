//! Example demonstrating the pattern-detection APIs in `temporal-compare`.
//!
//! Shows how to use:
//! 1. `find_similar_generic` - locate a needle pattern inside a series
//! 2. `detect_recurring_patterns` - find sub-sequences that repeat
//! 3. `compare` - score the distance between two sequences

use midstreamer_temporal_compare::{ComparisonAlgorithm, Sequence, TemporalComparator};

fn sequence_from(values: &[i64]) -> Sequence<i64> {
    let mut seq = Sequence::new();
    for (i, v) in values.iter().enumerate() {
        seq.push(*v, i as u64);
    }
    seq
}

fn main() {
    let comparator: TemporalComparator<i64> = TemporalComparator::new(100, 1000);

    let series = vec![1, 2, 3, 2, 1, 1, 2, 3, 2, 1, 5, 6];
    let pattern = vec![1, 2, 3, 2, 1];

    println!("Series:  {series:?}");
    println!("Pattern: {pattern:?}\n");

    let matches = comparator
        .find_similar_generic(&series, &pattern, 0.5)
        .expect("similarity search failed");

    println!("Found {} match(es):", matches.len());
    for m in &matches {
        println!("  index {:>2}  distance {:.3}", m.start_index, m.distance);
    }

    let recurring = comparator
        .detect_recurring_patterns(&series, 2, 5)
        .expect("pattern detection failed");
    println!("\nRecurring sub-patterns (len 2..=5):");
    for p in &recurring {
        println!("  {:?} x{}", p.sequence, p.occurrences.len());
    }

    let self_distance = comparator
        .compare(
            &sequence_from(&pattern),
            &sequence_from(&pattern),
            ComparisonAlgorithm::DTW,
        )
        .expect("compare failed");
    println!(
        "\nDTW self-distance of pattern: {:.3}",
        self_distance.distance
    );
}
