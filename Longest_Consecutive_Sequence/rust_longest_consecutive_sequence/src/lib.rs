use std::collections::HashSet;

/// Returns the length of the longest run of consecutive integers in `nums`.
///
/// O(n) with a hash set: rather than sorting, put every value in a set and only
/// start counting a run from its **first** element — a value `x` is a run start
/// iff `x - 1` is absent. Runs never overlap, so each value is walked over at
/// most once by an inner count and the total work stays linear.
pub fn longest_consecutive_sequence(nums: &[i32]) -> usize {
    // Owned i32s, so the inner loop can build new values without borrowing
    // anything out of the slice.
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut streak = 0;

    for x in &set {
        // Same as `!set.contains(&(x - 1))`, but `checked_sub` avoids both the
        // overflow panic at `i32::MIN` and the saturating-arithmetic trap where
        // `i32::MIN.saturating_sub(1) == i32::MIN` would wrongly reject a run
        // that genuinely starts at `i32::MIN`.
        if x.checked_sub(1).map_or(true, |prev| !set.contains(&prev)) {
            let mut current = *x;
            let mut sequence = 1;

            // Walk upward while the next integer is present. `checked_add`
            // returns `None` at `i32::MAX` and ends the run cleanly, whereas
            // `saturating_add` would clamp back to `i32::MAX` forever.
            while let Some(next) = current.checked_add(1) {
                if set.contains(&next) {
                    sequence += 1;
                    current = next;
                } else {
                    break;
                }
            }

            if sequence > streak {
                streak = sequence;
            }
        }
    }

    streak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(4, longest_consecutive_sequence(&[2, 20, 4, 10, 3, 4, 5]));
    }

    #[test]
    fn test2() {
        assert_eq!(7, longest_consecutive_sequence(&[0, 3, 2, 5, 4, 6, 1, 1]));
    }

    #[test]
    fn test_run_at_i32_min() {
        assert_eq!(
            3,
            longest_consecutive_sequence(&[i32::MIN, i32::MIN + 1, i32::MIN + 2])
        );
    }

    #[test]
    fn test_singleton_at_i32_max_terminates() {
        assert_eq!(1, longest_consecutive_sequence(&[i32::MAX]));
    }

    #[test]
    fn test_run_ending_at_i32_max() {
        assert_eq!(
            3,
            longest_consecutive_sequence(&[i32::MAX - 2, i32::MAX - 1, i32::MAX])
        );
    }
}
