/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 *
 * https://leetcode.com/problems/length-of-last-word/description/
 *
 * algorithms
 * Easy (33.27%)
 * Likes:    829
 * Dislikes: 2805
 * Total Accepted:    440.1K
 * Total Submissions: 1.3M
 * Testcase Example:  '"Hello World"'
 *
 * Given a string s consists of upper/lower-case alphabets and empty space
 * characters ' ', return the length of last word (last word means the last
 * appearing word if we loop from left to right) in the string.
 *
 * If the last word does not exist, return 0.
 *
 * Note: A word is defined as a maximal substring consisting of non-space
 * characters only.
 *
 * Example:
 *
 *
 * Input: "Hello World"
 * Output: 5
 *
 *
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_last_word(s: String) -> i32 {
        let b = s.as_bytes();
        let mut i = b.len();
        while i > 0 && b[i - 1] == b' ' {
            i -= 1;
        }

        let mut len = 0;
        while i > 0 && b[i - 1] != b' ' {
            len += 1;
            i -= 1;
        }
        len
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
        assert_eq!(5, Solution::length_of_last_word("Hello".to_string()));
        assert_eq!(1, Solution::length_of_last_word("Hello w".to_string()));
        assert_eq!(5, Solution::length_of_last_word("Hello ".to_string()));
        assert_eq!(0, Solution::length_of_last_word("".to_string()));
        assert_eq!(0, Solution::length_of_last_word("   ".to_string()));
    }
}
