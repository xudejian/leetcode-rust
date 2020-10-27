/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 *
 * https://leetcode.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (45.90%)
 * Likes:    17346
 * Dislikes: 620
 * Total Accepted:    3.4M
 * Total Submissions: 7.4M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers nums and an integer target, return indices of the
 * two numbers such that they add up to target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * You can return the answer in any order.
 *
 */
/*
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * Only one valid answer exists.
 *
 *
 */

pub struct Solution;

/// Example 1:
///
///
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Output: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// ```
/// use leetcode::p1_two_sum::Solution;
/// assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
/// ```

/// Example 2:
///
///
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
///
/// ```
/// use leetcode::p1_two_sum::Solution;
/// assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
/// ```

/// Example 3:
///
///
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///
/// ```
/// use leetcode::p1_two_sum::Solution;
/// assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0, 1]);
/// ```

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut wants = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let expect = target - nums[i];
            if wants.contains_key(&expect) {
                return vec![wants[&expect], i as i32];
            }
            wants.insert(nums[i], i as i32);
        }
        return vec![];
    }
}
// @lc code=end
