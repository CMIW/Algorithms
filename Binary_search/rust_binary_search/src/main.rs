fn main() {

}

fn binary_search<N: Ord>(a: &[N], v: N) -> Option<usize> {
    let mut l = 0;
    let mut r = a.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        if a[m] < v {
            l = m + 1;
        }
        else if a[m] > v {
            if m == 0 { break; }
            r = m - 1;
        }
        else{
            return Some(m);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Checking for a value that is in the array
    fn binary_search_passes() {
        let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let target = 14;

        assert_eq!(target, binary_search(&array, target).unwrap());
    }

    #[test]
    // Checking for a value that isn't in the array
    fn binary_search_fails(){
        let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let target = 54;

        assert_eq!(None, binary_search(&array, target));
    }

    #[test]
    // Checking for a value that isn't in the array
    fn binary_search_fails_negative(){
        let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let target = -4;

        assert_eq!(None, binary_search(&array, target));
    }
}
