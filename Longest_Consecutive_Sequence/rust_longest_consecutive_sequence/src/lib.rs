use std::collections::HashSet;
use std::iter::FromIterator;

pub fn longest_consecutive_sequence<const N: usize>(nums: [i32;N]) -> usize {
    let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
    let mut streak = 0;

    for x in &set {
        if !set.contains(&x.saturating_sub(1)) {
            let mut current = *x;
            let mut sequence = 1;
            loop {
                current = current.saturating_add(1);
                if set.contains(&current) {
                    sequence += 1;
                } else {
                    break;
                }
            }
            if sequence > streak {
                streak = sequence
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
        assert_eq!(4, longest_consecutive_sequence([2,20,4,10,3,4,5]));
    }

    #[test]
    fn test2() {
        assert_eq!(7, longest_consecutive_sequence([0,3,2,5,4,6,1,1]));
    }
}
