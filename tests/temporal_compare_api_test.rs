//! Integration tests for the `temporal-compare` public API.
//!
//! Verifies sequence comparison, similarity search and recurring-pattern
//! detection against the current published crate API.

use midstreamer_temporal_compare::{ComparisonAlgorithm, Sequence, TemporalComparator};

fn sequence_from(values: &[i64]) -> Sequence<i64> {
    let mut seq = Sequence::new();
    for (i, v) in values.iter().enumerate() {
        seq.push(*v, i as u64);
    }
    seq
}

#[test]
fn test_find_similar_generic() {
    let comparator: TemporalComparator<i64> = TemporalComparator::new(100, 1000);

    // A time series with the needle pattern repeated at indices 2 and 5.
    let series = vec![1, 2, 3, 4, 5, 3, 4, 5, 6, 7];
    let needle = vec![3, 4, 5];

    let matches = comparator
        .find_similar_generic(&series, &needle, 0.5)
        .unwrap();

    assert!(!matches.is_empty(), "Should find at least one match");
    assert!(
        matches.iter().any(|m| m.start_index == 2),
        "Should match the needle at index 2"
    );
    assert!(
        matches.iter().any(|m| m.start_index == 5),
        "Should match the needle at index 5"
    );
}

#[test]
fn test_no_false_match() {
    let comparator: TemporalComparator<i64> = TemporalComparator::new(100, 1000);
    let series = vec![1, 2, 3, 4, 5, 3, 4, 5, 6, 7];

    let matches = comparator
        .find_similar_generic(&series, &[100, 200, 300], 0.1)
        .unwrap();
    assert!(matches.is_empty(), "Unrelated needle should not match");
}

#[test]
fn test_compare_algorithms() {
    let comparator: TemporalComparator<i64> = TemporalComparator::new(100, 1000);
    let a = sequence_from(&[1, 2, 3, 4, 5]);
    let b = sequence_from(&[1, 2, 3, 4, 5]);
    let c = sequence_from(&[9, 8, 7, 6, 5]);

    for algorithm in [ComparisonAlgorithm::DTW, ComparisonAlgorithm::EditDistance] {
        let same = comparator.compare(&a, &b, algorithm).unwrap();
        let different = comparator.compare(&a, &c, algorithm).unwrap();
        assert_eq!(
            same.distance, 0.0,
            "{algorithm:?}: identical sequences should have zero distance"
        );
        assert!(
            different.distance > same.distance,
            "{algorithm:?}: differing sequences should be farther apart"
        );
    }
}

#[test]
fn test_detect_recurring_patterns() {
    let comparator: TemporalComparator<i64> = TemporalComparator::new(100, 1000);
    // The sub-sequence [1, 2, 3] recurs three times.
    let sequence = vec![1, 2, 3, 9, 1, 2, 3, 8, 1, 2, 3];

    let patterns = comparator
        .detect_recurring_patterns(&sequence, 3, 3)
        .unwrap();
    assert!(
        patterns.iter().any(|p| p.sequence == vec![1, 2, 3]),
        "Should detect the recurring [1, 2, 3] pattern"
    );
}
