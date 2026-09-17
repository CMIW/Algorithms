use std::collections::HashMap;

/// Given a string `s` of uppercase English characters and an integer `k`,
/// returns the length of the longest substring with identical letters achievable
/// by replacing at most `k` characters.
///
/// Sliding Window, O(n) time, O(1) space (at most 26 uppercase English letters):
/// Maintain a window `chars[b..=f]` and count character frequencies within it.
/// A window is valid if:
///     `window_length - max_freq <= k`
/// meaning we need at most `k` replacements to turn every character in the window
/// into the dominant (most frequent) character.
/// When the window becomes invalid, shrink it by moving `b` forward and decrementing
/// the frequency of the evicted character.
pub fn character_replacement(s: &str, k: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut freq: HashMap<char, usize> = HashMap::new();
    let mut b = 0;
    let mut max_len = 0;
    let mut max_freq = 0;

    for f in 0..chars.len() {
        // Expand the window with the character at the front pointer `f`.
        let count = freq.entry(chars[f]).or_default();
        *count += 1;
        max_freq = max_freq.max(*count);

        let window_length = f - b + 1;
        let other_letters = window_length - max_freq;

        // If we need more than `k` replacements, the window is invalid:
        // shrink from the back by incrementing `b`.
        if other_letters > k {
            if let Some(back_count) = freq.get_mut(&chars[b]) {
                *back_count -= 1;
            }
            b += 1;
        }

        // Window is guaranteed valid after shrinking (or keeping size):
        max_len = max_len.max(f - b + 1);
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abab_k2() {
        assert_eq!(4, character_replacement("ABAB", 2));
    }

    #[test]
    fn test_aababba_k1() {
        assert_eq!(4, character_replacement("AABABBA", 1));
    }

    #[test]
    fn test_single_char() {
        assert_eq!(1, character_replacement("A", 0));
    }

    #[test]
    fn test_no_replacement_allowed() {
        assert_eq!(2, character_replacement("ABAA", 0));
    }

    #[test]
    fn test_all_same_char() {
        assert_eq!(5, character_replacement("AAAAA", 2));
    }
}
