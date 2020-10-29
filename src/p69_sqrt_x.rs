/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 *
 * https://leetcode.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (34.37%)
 * Likes:    1579
 * Dislikes: 2071
 * Total Accepted:    619.2K
 * Total Submissions: 1.8M
 * Testcase Example:  '4'
 *
 * Implement int sqrt(int x).
 *
 * Compute and return the square root of x, where x is guaranteed to be a
 * non-negative integer.
 *
 * Since the return type is an integer, the decimal digits are truncated and
 * only the integer part of the result is returned.
 *
 * Example 1:
 *
 *
 * Input: 4
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since
 * the decimal part is truncated, 2 is returned.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let x0 = x as i64;
        let (mut l, mut r) = (1, x / 2);
        let mut ans = 1;
        while l <= r {
            let m = (r + l) / 2;
            let mm = m as i64 * m as i64;
            if mm == x0 {
                return m;
            } else if mm < x0 {
                ans = m;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(0, Solution::my_sqrt(0));
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
        assert_eq!(3, Solution::my_sqrt(9));
        assert_eq!(3, Solution::my_sqrt(10));
        assert_eq!(3, Solution::my_sqrt(15));
        assert_eq!(4, Solution::my_sqrt(16));
        assert_eq!(11, Solution::my_sqrt(121));
        assert_eq!(46339, Solution::my_sqrt(2147395599));
        assert_eq!(46340, Solution::my_sqrt(2147483647));
    }
}
