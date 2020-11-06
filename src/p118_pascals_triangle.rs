/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (53.42%)
 * Likes:    1890
 * Dislikes: 122
 * Total Accepted:    424K
 * Total Submissions: 791K
 * Testcase Example:  '5'
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's
 * triangle.
 *
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it.
 *
 * Example:
 *
 *
 * Input: 5
 * Output:
 * [
 * ⁠    [1],
 * ⁠   [1,1],
 * ⁠  [1,2,1],
 * ⁠ [1,3,3,1],
 * ⁠[1,4,6,4,1]
 * ]
 *
 *
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let nrows = num_rows as usize;
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(nrows);
        let mut row: Vec<i32> = Vec::with_capacity(nrows);
        for i in 0..nrows {
            row.push(1);
            for j in (1..i).rev() {
                row[j] += row[j - 1];
            }
            res.push((row[..=i]).to_vec());
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let got = Solution::generate(5);
        assert_eq!(got.len(), 5);
        assert_eq!(got[0], [1]);
        assert_eq!(got[1], [1, 1]);
        assert_eq!(got[2], [1, 2, 1]);
        assert_eq!(got[3], [1, 3, 3, 1]);
        assert_eq!(got[4], [1, 4, 6, 4, 1]);
    }
}
