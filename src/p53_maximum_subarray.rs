/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 *
 * https://leetcode.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Easy (47.01%)
 * Likes:    9444
 * Dislikes: 450
 * Total Accepted:    1.2M
 * Total Submissions: 2.5M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * Given an integer array nums, find the contiguous subarray (containing at
 * least one number) which has the largest sum and return its sum.
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution using the divide and conquer approach, which is more subtle.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [0]
 * Output: 0
 *
 *
 * Example 4:
 *
 *
 * Input: nums = [-1]
 * Output: -1
 *
 *
 * Example 5:
 *
 *
 * Input: nums = [-2147483647]
 * Output: -2147483647
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 2 * 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let mut max = nums[0];
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if sum < 0 {
                sum = 0;
            }
            sum += nums[i];
            if sum > max {
                max = sum;
            }
        }
        max
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
        assert_eq!(1, Solution::max_sub_array(vec![1]));
        assert_eq!(0, Solution::max_sub_array(vec![0]));
        assert_eq!(-2147483647, Solution::max_sub_array(vec![-2147483647]));
    }
}
