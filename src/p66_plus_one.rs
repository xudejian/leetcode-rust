/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 *
 * https://leetcode.com/problems/plus-one/description/
 *
 * algorithms
 * Easy (42.94%)
 * Likes:    1838
 * Dislikes: 2681
 * Total Accepted:    713K
 * Total Submissions: 1.7M
 * Testcase Example:  '[1,2,3]'
 *
 * Given a non-empty array of digits representing a non-negative integer,
 * increment one to the integer.
 *
 * The digits are stored such that the most significant digit is at the head of
 * the list, and each element in the array contains a single digit.
 *
 * You may assume the integer does not contain any leading zero, except the
 * number 0 itself.
 *
 *
 * Example 1:
 *
 *
 * Input: digits = [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 *
 *
 * Example 2:
 *
 *
 * Input: digits = [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 *
 *
 * Example 3:
 *
 *
 * Input: digits = [0]
 * Output: [1]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= digits.length <= 100
 * 0 <= digits[i] <= 9
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut nums = digits;
        let mut over = 1;
        for ele in nums.iter_mut().rev() {
            *ele += over;
            over = *ele / 10;
            *ele = *ele % 10;
        }
        if over > 0 {
            nums.insert(0, over);
        }
        nums
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
    }
}
