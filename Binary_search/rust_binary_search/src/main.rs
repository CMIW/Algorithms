#[macro_use]
extern crate log;

fn main() {

}

fn binary_search<N: Ord>(a: &[N], v: N) -> Option<N> {
    let mut l = 0;
    let mut r = a.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        if a[m] < v {
            l = m + 1;
        }
        else if a[m] > v {
            r = m - 1;
        }
        else{
            return Some(v);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn found_target() {
        init();

        let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let target = 14;

        assert_eq!(target, binary_search(&array, target).unwrap());
    }

    #[test]
    fn did_not_find_target(){
        init();

        let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let target = 54;

        assert_eq!(None, binary_search(&array, target));
    }
}
