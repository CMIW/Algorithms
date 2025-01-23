pub fn valid_palindrome(word: &str) -> bool {
    let word = word.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()).collect::<Vec<char>>();
    let l = word.len()-1;
    for i in 0..l/2 {
        if word[i] != word[l-i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, valid_palindrome("Was it a car or a cat I saw?"));
    }

    #[test]
    fn test2() {
        assert_eq!(false, valid_palindrome("tab a cat"));
    }
}
