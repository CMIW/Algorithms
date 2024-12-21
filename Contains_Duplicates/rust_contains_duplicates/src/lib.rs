use std::collections::HashSet;

pub fn constain_duplicates<N: std::cmp::Eq + std::hash::Hash>(elements: &[N]) -> bool {
    let mut a = HashSet::new();

    for i in elements {
        if !a.insert(i) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates() {
        assert_eq!(true, constain_duplicates(&[1, 2, 3, 3]));
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(false, constain_duplicates(&[1, 2, 3, 4]));
    }
}
