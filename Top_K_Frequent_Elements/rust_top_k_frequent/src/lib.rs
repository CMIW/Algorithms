use std::collections::HashMap;
pub fn top_k_frequent(nums: &[usize], k: usize) -> Vec<usize> {
    let mut map = HashMap::new();

    for i in nums {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut bucket = vec![Vec::<usize>::new(); nums.len() + 1];

    for (entry, frequency) in map {
        bucket[frequency].push(*entry);
    }

    let mut result:Vec<usize> = bucket.into_iter().flatten().collect();

    result.reverse();

    result[0..k].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![3,2], top_k_frequent(&[1,2,2,3,3,3], 2));
    }

    #[test]
    fn test2() {
        assert_eq!(vec![7], top_k_frequent(&[7,7], 1));
    }
}
