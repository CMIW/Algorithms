fn product_except_self<const N: usize>(nums: [i64;N]) -> [i64;N] {
    let mut result: [i64;N] = [1;N];

    let mut p: i64 = 1;
    let mut s: i64 = 1;

    for i in 0..N {
        let j = N-1-i;

        result[i] *= p;
        result[j] *= s;

        p *= nums[i];
        s *= nums[j];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = product_except_self([1,2,3,4]);
        assert_eq!(result, [24, 12, 8, 6]);
    }

    #[test]
    fn test2() {
        let result = product_except_self([-1,0,1,2,3]);
        assert_eq!(result, [0, -6, 0, 0, 0]);
    }
}
