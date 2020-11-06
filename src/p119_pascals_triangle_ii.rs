/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 *
 * https://leetcode.com/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (51.18%)
 * Likes:    1113
 * Dislikes: 208
 * Total Accepted:    334.8K
 * Total Submissions: 652.6K
 * Testcase Example:  '3'
 *
 * Given an integer rowIndex, return the rowIndex^th row of the Pascal's
 * triangle.
 *
 * Notice that the row index starts from 0.
 *
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it.
 *
 * Follow up:
 *
 * Could you optimize your algorithm to use only O(k) extra space?
 *
 *
 * Example 1:
 * Input: rowIndex = 3
 * Output: [1,3,3,1]
 * Example 2:
 * Input: rowIndex = 0
 * Output: [1]
 * Example 3:
 * Input: rowIndex = 1
 * Output: [1,1]
 *
 *
 * Constraints:
 *
 *
 * 0 <= rowIndex <= 40
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let size = (1 + row_index) as usize;
        let mut row: Vec<i32> = Vec::with_capacity(size);
        for i in 0..size {
            row.push(1);
            for j in (1..i).rev() {
                row[j] += row[j - 1];
            }
        }
        row
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(Solution::get_row(0), [1]);
        assert_eq!(Solution::get_row(1), [1, 1]);
        assert_eq!(Solution::get_row(2), [1, 2, 1]);
        assert_eq!(Solution::get_row(3), [1, 3, 3, 1]);
        assert_eq!(Solution::get_row(4), [1, 4, 6, 4, 1]);
    }
}
