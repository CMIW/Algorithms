# Longest Repeating Character Replacement (LeetCode #424)

You are given a string `s` consisting only of uppercase English letters, and an integer `k`. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most `k` times.

Return the length of the longest substring containing the same letter you can get after performing the above operations.

| Input | `k` | Output | Explanation |
|-------|-----|--------|-------------|
| `"ABAB"` | 2 | 4 | Replace the two 'A's with 'B's or vice versa. |
| `"AABABBA"` | 1 | 4 | Replace the 'A' in the middle with 'B' to get `"AABBBBA"`. Substring `"BBBB"` is length 4. |
| `"A"` | 0 | 1 | No replacements needed. |
| `"ABAA"` | 0 | 2 | Longest repeating character is `"AA"`. |