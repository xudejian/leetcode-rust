/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 *
 * https://leetcode.com/problems/climbing-stairs/description/
 *
 * algorithms
 * Easy (48.12%)
 * Likes:    5182
 * Dislikes: 166
 * Total Accepted:    799.8K
 * Total Submissions: 1.7M
 * Testcase Example:  '2'
 *
 * You are climbing a stair case. It takes n steps to reach to the top.
 *
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can
 * you climb to the top?
 *
 * Example 1:
 *
 *
 * Input: 2
 * Output: 2
 * Explanation: There are two ways to climb to the top.
 * 1. 1 step + 1 step
 * 2. 2 steps
 *
 *
 * Example 2:
 *
 *
 * Input: 3
 * Output: 3
 * Explanation: There are three ways to climb to the top.
 * 1. 1 step + 1 step + 1 step
 * 2. 1 step + 2 steps
 * 3. 2 steps + 1 step
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 45
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let mut n1 = 2;
        let mut n2 = 1;
        let mut x = 0;
        for _ in 3..=n {
            x = n1 + n2;
            n2 = n1;
            n1 = x;
        }
        x
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(1, Solution::climb_stairs(1));
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(5, Solution::climb_stairs(4));
        assert_eq!(8, Solution::climb_stairs(5));
    }
}
