pub fn max_area<const N: usize>(heights: [u32; N]) -> u32 {
    let mut area = 0;
    let len = heights.len() - 1;
    for i in 0..=len {
        let new_area1 = std::cmp::min(heights[i], heights[len]) * ((len - i) as u32);

        let new_area2 = std::cmp::min(heights[0], heights[len - i]) * ((len - i - 0) as u32);

        area = std::cmp::max(area, std::cmp::max(new_area1, new_area2));
    }

    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(36, max_area([1,7,2,5,4,7,3,6]));
    }

    #[test]
    fn test2() {
        assert_eq!(4, max_area([2,2,2]));
    }
}
