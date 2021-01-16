use two_bucket::{solve, Bucket, BucketStats};

// (3,0) omplir
// (0,3) traspassar
// (3,3) omplir pq esta buit
// (1,5) traspassar

#[test]
fn test_case_1() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::One),
        Some(BucketStats {
            moves: 4,
            goal_bucket: Bucket::One,
            other_bucket: 5,
        })
    );
}

// (0,5) omplir
// (3,2) traspassar
// (0,2) buidar pq esta ple
// (2,0) traspassar
// (2,5) omplir pq esta buit
// (3,4) traspassar
// (0,4) buidar pq esta ple
// (3,1) traspassar

#[test]
fn test_case_2() {
    assert_eq!(
        solve(3, 5, 1, &Bucket::Two),
        Some(BucketStats {
            moves: 8,
            goal_bucket: Bucket::Two,
            other_bucket: 3,
        })
    );
}

// (7,0)
// (0,7)
// (7,7)
// (3,11)
// (3,0)
// (0,3)
// (7,3)
// (0,10)
// (7,10)
// (6,11)
// (6,0)
// (0,6)
// (7,6)
// (2,11)

#[test]
fn test_case_3() {
    assert_eq!(
        solve(7, 11, 2, &Bucket::One),
        Some(BucketStats {
            moves: 14,
            goal_bucket: Bucket::One,
            other_bucket: 11,
        })
    );
}

// (0,11)
// (7,4)
// (0,4)
// (4,0)
// (4,11)
// (7,8)
// (0,8)
// (7,1)
// (0,1)
// (1,0)
// (1,11)
// (7,5)
// (0,5)
// (5,0)
// (5,11)
// (7,9)
// (0,9)
// (7,2)

#[test]
fn test_case_4() {
    assert_eq!(
        solve(7, 11, 2, &Bucket::Two),
        Some(BucketStats {
            moves: 18,
            goal_bucket: Bucket::Two,
            other_bucket: 7,
        })
    );
}

// (0,3)

#[test]
fn goal_equal_to_start_bucket() {
    assert_eq!(
        solve(1, 3, 3, &Bucket::Two),
        Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        })
    );
}

// (2,0)
// (2,3)

#[test]
fn goal_equal_to_other_bucket() {
    assert_eq!(
        solve(2, 3, 3, &Bucket::One),
        Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: 2,
        })
    );
}

#[test]
fn not_possible_to_reach_the_goal() {
    assert_eq!(solve(6, 15, 5, &Bucket::One), None);
}

// (6,0)
// (0,6)
// (6,6)
// (0,12)
// (6,12)
// (3,15)
// (3,0)
// (0,3)
// (6,3)
// (0,9)

#[test]
fn with_same_buckets_but_different_goal_then_it_is_possible() {
    assert_eq!(
        solve(6, 15, 9, &Bucket::One),
        Some(BucketStats {
            moves: 10,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        })
    );
}
