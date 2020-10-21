/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 *
 * https://leetcode.com/problems/reverse-integer/description/
 *
 * algorithms
 * Easy (25.82%)
 * Likes:    3844
 * Dislikes: 5991
 * Total Accepted:    1.3M
 * Total Submissions: 4.9M
 * Testcase Example:  '123'
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 *
 * Note:
 * Assume we are dealing with an environment that could only store integers
 * within the 32-bit signed integer range: [−2^31,  2^31 − 1]. For the purpose
 * of this problem, assume that your function returns 0 when the reversed
 * integer overflows.
 *
 *
 * Example 1:
 * Input: x = 123
 * Output: 321
 * Example 2:
 * Input: x = -123
 * Output: -321
 * Example 3:
 * Input: x = 120
 * Output: 21
 * Example 4:
 * Input: x = 0
 * Output: 0
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let a = std::i32::MAX / 10;
        let b = std::i32::MAX % 10;
        let c = std::i32::MIN / 10;
        let d = std::i32::MIN % 10;
        let mut v = 0;
        let mut n = x;
        while n != 0 {
            let i = n % 10;
            if v > a || (v == a && i > b) {
                return 0;
            }
            if v < c || (v == c && i < d) {
                return 0;
            }
            v = v * 10 + i;
            n /= 10;
        }
        v
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(0, Solution::reverse(1_563_847_412i32));
        assert_eq!(2_147_483_641i32, Solution::reverse(1_463_847_412i32));
        assert_eq!(0, Solution::reverse(1_463_847_413i32));
        assert_eq!(0, Solution::reverse(-1_563_847_412i32));
        assert_eq!(-2_147_483_641i32, Solution::reverse(-1_463_847_412i32));
        assert_eq!(0, Solution::reverse(-1_463_847_413i32));
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(0, Solution::reverse(0));
    }
}
