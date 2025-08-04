// Leetcode 58: Length of Last Word
//
// Given a string s consisting of words and spaces, return the length of the last word in the string.
// A word is a maximal substring consisting of non-space characters only.
//
// Example:
// Input: s = "Hello World"
// Output: 5
// Explanation: The last word is "World" with length 5.
//
// Constraints:
// 1 <= s.length <= 10^4
// s consists of only English letters and spaces ' '.
// There will be at least one word in s.

pub fn length_of_last_word(s: &str) -> usize {
    s.trim_end()
        .bytes()
        .rev()
        .take_while(|b| b != &b' ')
        .count()
}
