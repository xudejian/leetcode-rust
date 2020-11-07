/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 *
 * https://leetcode.com/problems/single-number/description/
 *
 * algorithms
 * Easy (65.94%)
 * Likes:    5152
 * Dislikes: 177
 * Total Accepted:    995.4K
 * Total Submissions: 1.5M
 * Testcase Example:  '[2,2,1]'
 *
 * Given a non-empty array of integers nums, every element appears twice except
 * for one. Find that single one.
 *
 * Follow up: Could you implement a solution with a linear runtime complexity
 * and without using extra memory?
 *
 *
 * Example 1:
 * Input: nums = [2,2,1]
 * Output: 1
 * Example 2:
 * Input: nums = [4,1,2,1,2]
 * Output: 4
 * Example 3:
 * Input: nums = [1]
 * Output: 1
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * -3 * 10^4 <= nums[i] <= 3 * 10^4
 * Each element in the array appears twice except for one element which appears
 * only once.
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut n = 0;
        for i in nums {
            n ^= i;
        }
        n
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(1, Solution::single_number([2, 2, 1].to_vec()));
        assert_eq!(4, Solution::single_number([4, 1, 2, 1, 2].to_vec()));
        assert_eq!(1, Solution::single_number([1].to_vec()));
    }
}
