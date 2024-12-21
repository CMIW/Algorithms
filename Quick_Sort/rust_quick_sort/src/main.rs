#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
}

fn quick_sort<N: Ord + std::marker::Copy> (a: &mut [N], lo: usize, hi: usize) -> &mut [N] {
    if lo < hi {
        let pivot = partition(a, lo, hi);
        quick_sort(a, lo, pivot);
        quick_sort(a, pivot + 1, hi);
    }
    a
}

fn partition<N: Ord + std::marker::Copy> (a: &mut [N], lo: usize, hi: usize) -> usize{
    let pivot_index = (lo + hi) / 2;
    let pivot = a[pivot_index];

    let mut l = lo - 1;
    let mut r = hi + 1;
    loop {
        loop{
            if !(a[l] < pivot){
                break;
            }
            l += 1;
        }

        loop{
            r -= 1;
            if !(a[r] > pivot){
                break;
            }
        }

        if l >= r {
            return r;
        }

        // equivalent to temp = a[l]; a[l] = a[r]; a[r] = temp;
        a.swap(l, r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn basic_test() {
        init();

        let list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let mut unsorted_list = [17, 5, 13, 4, 2, 6, 11, 8, 9, 10, 7, 12, 3, 14, 15, 16, 1, 0];

        println!("Unsorted {:?}", unsorted_list);

        let len = unsorted_list.len();

        let mut sorted_list = quick_sort(&mut unsorted_list, 1, len - 1);

        println!("  Sorted {:?}", &mut sorted_list);

        assert_eq!(list, sorted_list);
    }
}
