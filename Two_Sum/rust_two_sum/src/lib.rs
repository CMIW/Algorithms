use std::collections::HashMap;

pub fn two_sum(nums: &[usize], target: usize) -> [usize; 2] {
    let mut map = HashMap::new();

    for i in 0..nums.len() {
        let difference = target - nums[i];
        if map.contains_key(&difference) {
            if let Some(value) = map.get(&difference) {
                return [*value, i];
            }
        } else {
            map.insert(nums[i], i);
        }
    }

    [0,0]
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
