use std::collections::HashSet;

/// Given a string `s`, returns the length of the longest substring
/// that contains no repeating characters.
///
/// Sliding window, O(n): the window is always the *longest* valid substring
/// that ends at the front pointer `f`. `f` grows one char at a time; the only
/// thing that can invalidate the window is the char at `f` already appearing
/// somewhere inside it, so the back pointer `b` ejects chars until that old
/// copy has left. Because `b` only moves forward, each char joins once (at `f`)
/// and leaves at most once (at `b`): O(n) total.
pub fn length_of_longest_substring(s: &str) -> usize {
    // Work on full chars, not bytes, so multibyte characters never risk a
    // mid-character byte-slice panic (`s[i..i+1]` would).
    let chars: Vec<char> = s.chars().collect();
    // The set mirrors the current window `chars[b..=f]` exactly: chars are
    // added when `f` grows and removed when `b` shrinks.
    let mut set = HashSet::new();
    let mut b = 0; // back edge — start of the current window (only moves forward)
    let mut max = 0;

    for f in 0..chars.len() {
        // `chars[f]` is a duplicate of something already in the window. Eject
        // chars from the back until that old copy has been removed from the
        // set; the loop stops the instant the duplicate is gone, so `b` ends
        // exactly one past the old occurrence.
        while set.contains(&chars[f]) {
            set.remove(&chars[b]);
            b += 1;
        }
        // Now the window `chars[b..=f]` is valid, so `chars[f]` can join.
        set.insert(chars[f]);
        // Window length in inclusive indices: `f - b + 1`.
        max = max.max(f - b + 1);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(0, length_of_longest_substring(""));
    }

    #[test]
    fn test_all_same() {
        assert_eq!(1, length_of_longest_substring("bbbbb"));
    }

    #[test]
    fn test_abcabcbb() {
        assert_eq!(3, length_of_longest_substring("abcabcbb"));
    }

    #[test]
    fn test_pwwkew() {
        assert_eq!(3, length_of_longest_substring("pwwkew"));
    }

    #[test]
    fn test_two_chars() {
        assert_eq!(2, length_of_longest_substring("au"));
    }

    #[test]
    fn test_repeat_after_long_run() {
        assert_eq!(3, length_of_longest_substring("dvdf"));
    }
}
