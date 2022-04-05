#[macro_use]
extern crate log;

fn main() {

}

fn insertion_sort<N: Ord> (a: &mut [N]) -> &mut [N] {
    for i in 1..a.len(){
        let mut j = i;
        while j > 0 && a[j] < a[j - 1]{
            // equivalent to temp = a[i]; a[i] = a[i -1]; a[i - 1] = temp;
            a.swap(j, j - 1);
            j -= 1;
        }
    }
    a
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

        let mut sorted_list = insertion_sort(&mut unsorted_list);

        println!("  Sorted {:?}", &mut sorted_list);

        assert_eq!(list, sorted_list);
    }
}
