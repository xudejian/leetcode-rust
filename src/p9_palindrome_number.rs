/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 *
 * https://leetcode.com/problems/palindrome-number/description/
 *
 * algorithms
 * Easy (48.93%)
 * Likes:    2668
 * Dislikes: 1629
 * Total Accepted:    1M
 * Total Submissions: 2.1M
 * Testcase Example:  '121'
 *
 * Determine whether an integer is a palindrome. An integer is a palindrome
 * when it reads the same backward as forward.
 *
 * Follow up: Could you solve it without converting the integer to a string?
 *
 *
 * Example 1:
 *
 *
 * Input: x = 121
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it
 * becomes 121-. Therefore it is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a
 * palindrome.
 *
 *
 * Example 4:
 *
 *
 * Input: x = -101
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut n = x;
        let mut m = 0;
        while n > m {
            m = m * 10 + n % 10;
            n = n / 10;
        }
        n == m || n == m / 10
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        // Examples
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121), "From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.");
        assert!(
            !Solution::is_palindrome(10),
            "Reads 01 from right to left. Therefore it is not a palindrome."
        );
        assert!(!Solution::is_palindrome(-101));
        // debug
        assert!(Solution::is_palindrome(0));
        assert!(Solution::is_palindrome(12321));
        assert!(Solution::is_palindrome(1221));
        assert!(Solution::is_palindrome(1));
    }
}
