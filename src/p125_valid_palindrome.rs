/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (37.24%)
 * Likes:    1509
 * Dislikes: 3271
 * Total Accepted:    714.4K
 * Total Submissions: 1.9M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * Given a string, determine if it is a palindrome, considering only
 * alphanumeric characters and ignoring cases.
 *
 * Note: For the purpose of this problem, we define empty string as valid
 * palindrome.
 *
 * Example 1:
 *
 *
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "race a car"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * s consists only of printable ASCII characters.
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let (mut left, mut right) = (0, s.len());
        while left < right && right > 0 {
            let (mut l, mut r) = (s[left], s[right - 1]);
            if l >= 'A' as u8 && l <= 'Z' as u8 {
                l += 'a' as u8 - 'A' as u8;
            }
            if r >= 'A' as u8 && r <= 'Z' as u8 {
                r += 'a' as u8 - 'A' as u8;
            }

            if !((l >= '0' as u8 && l <= '9' as u8) || (l >= 'a' as u8 && l <= 'z' as u8)) {
                left += 1;
            } else if !((r >= '0' as u8 && r <= '9' as u8) || (r >= 'a' as u8 && r <= 'z' as u8)) {
                right -= 1;
            } else if l != r {
                return false;
            } else {
                left += 1;
                right -= 1;
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palidrome() {
        assert!(Solution::is_palindrome("".to_string()));
        assert!(Solution::is_palindrome(" ".to_string()));
        assert!(!Solution::is_palindrome("race a car".to_string()));
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }
}
