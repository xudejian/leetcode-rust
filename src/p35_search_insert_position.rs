/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 *
 * https://leetcode.com/problems/search-insert-position/description/
 *
 * algorithms
 * Easy (42.71%)
 * Likes:    2799
 * Dislikes: 268
 * Total Accepted:    708.9K
 * Total Submissions: 1.7M
 * Testcase Example:  '[1,3,5,6]\n5'
 *
 * Given a sorted array of distinct integers and a target value, return the
 * index if the target is found. If not, return the index where it would be if
 * it were inserted in order.
 *
 *
 * Example 1:
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 * Example 2:
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 * Example 3:
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 * Example 4:
 * Input: nums = [1,3,5,6], target = 0
 * Output: 0
 * Example 5:
 * Input: nums = [1], target = 0
 * Output: 0
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums contains distinct values sorted in ascending order.
 * -10^4 <= target <= 10^4
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        // [1, 3, 5, 6], 0
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = (right + left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
        assert_eq!(0, Solution::search_insert(vec![1], 0));
    }
}
