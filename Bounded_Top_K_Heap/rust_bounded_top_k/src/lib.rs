use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub fn bounded_top_k<T: Eq + Hash + Ord + Copy>(values: &[T], k: usize) -> Vec<T> {
    let mut map = HashMap::new();
    for n in values {
        *map.entry(*n).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (entry, frequency) in map {
        // `pop()` removes the *worst*,
        // which means the worst must be the heap's top — hence `Reverse(frequency)`.
        heap.push((Reverse(frequency), entry));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_sorted_vec()
        .into_iter()
        .map(|(_, entry)| entry)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![3, 2], bounded_top_k(&[1, 2, 2, 3, 3, 3], 2));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![7], bounded_top_k(&[7, 7], 1));
    }

    #[test]
    fn test3() {
        assert_eq!(
            vec!['c', 'b'],
            bounded_top_k(&['a', 'b', 'b', 'c', 'c', 'c'], 2)
        );
    }
}
