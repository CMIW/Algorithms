use std::collections::HashMap;

// One pass with a hash map: for each number, only the pair that completes
// `target` matters, so check whether the complement was already seen.
// Amortized O(n) time, O(n) space.
pub fn two_sum(nums: &[usize], target: usize) -> [usize; 2] {
    let mut map = HashMap::new(); // value -> its index (values seen so far)

    for i in 0..nums.len() {
        // The partner this number needs to reach target.
        let difference = target - nums[i];
        if map.contains_key(&difference) {
            // Found it earlier in the array -> answer is [earlier idx, current idx].
            // (contains_key + get does two lookups; `map.get(&difference)` alone
            // would suffice, since the None case is just the else branch below.)
            if let Some(value) = map.get(&difference) {
                return [*value, i];
            }
        } else {
            // Complement not seen yet: remember this number for later numbers to find.
            map.insert(nums[i], i);
        }
    }

    [0,0] // unreachable for valid inputs (problem guarantees one pair exists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!([0,1], two_sum(&[3,4,5,6], 7));
    }

    #[test]
    fn test2() {
        assert_eq!([0,2], two_sum(&[4,5,6], 10));
    }

    #[test]
    fn test3() {
        assert_eq!([0,1], two_sum(&[5,5], 10));
    }
}
