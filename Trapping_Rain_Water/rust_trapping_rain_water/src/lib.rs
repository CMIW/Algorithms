pub fn max_area<const N: usize>(heights: [u32; N]) -> u32 {
    let mut p = [0;N];
    let mut s = [0;N];
    let len = heights.len()-1;

    p[0] = heights[0];
    s[len] = heights[len];

    for i in 0..=len {
        let j = len - i;
        if i > 0 {
            p[i] = std::cmp::max(p[i-1], heights[i]);
        }

        if j < len {
            s[j] = std::cmp::max(heights[j], s[j+1]);
        }
    }

    let mut area = 0;
    for i in 0..=len {
        area += std::cmp::min(p[i], s[i]) - heights[i];
    }
    area
}

pub fn max_area_1<const N: usize>(heights: [u32; N]) -> u32 {
    let mut area = 0;
    let mut l = 0;
    let mut r = heights.len()-1;
    let mut l_max = 0;
    let mut r_max = 0;

    while l < r {
        if heights[l] < heights[r] {
            if heights[l] >= l_max {
                l_max = heights[l];
            } else {
                area += l_max - heights[l];
            }

            l += 1;
        } else {
            if heights[r] >= r_max {
                r_max = heights[r];
            } else {
                area += r_max - heights[r];
            }

            r -= 1;
        }
    }
    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(6, max_area([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(9, max_area([0,2,0,3,1,0,1,3,2,1]));
    }

    #[test]
    fn test3() {
        assert_eq!(6, max_area_1([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }

    #[test]
    fn test4() {
        assert_eq!(9, max_area_1([0,2,0,3,1,0,1,3,2,1]));
    }
}
