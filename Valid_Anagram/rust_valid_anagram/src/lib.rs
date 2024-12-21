use std::collections::HashSet;

pub fn valid_anagram(left: &str, right: &str) -> bool {
    let mut set = HashSet::new();

    for i in left.chars() {
        set.insert(i);
    }

    for i in right.chars() {
        if set.insert(i) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram() {
        assert_eq!(true, valid_anagram("racecar", "carrace"));
    }

    #[test]
    fn no_anagram() {
        assert_eq!(false, valid_anagram("jar", "jam"));
    }
}
