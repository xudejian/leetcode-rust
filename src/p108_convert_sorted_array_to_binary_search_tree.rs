/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
 *
 * https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/
 *
 * algorithms
 * Easy (58.93%)
 * Likes:    3014
 * Dislikes: 230
 * Total Accepted:    456.8K
 * Total Submissions: 772.6K
 * Testcase Example:  '[-10,-3,0,5,9]'
 *
 * Given an array where elements are sorted in ascending order, convert it to a
 * height balanced BST.
 *
 * For this problem, a height-balanced binary tree is defined as a binary tree
 * in which the depth of the two subtrees of every node never differ by more
 * than 1.
 *
 * Example:
 *
 *
 * Given the sorted array: [-10,-3,0,5,9],
 *
 * One possible answer is: [0,-3,9,-10,null,5], which represents the following
 * height balanced BST:
 *
 * ⁠     0
 * ⁠    / \
 * ⁠  -3   9
 * ⁠  /   /
 * ⁠-10  5
 *
 *
 */
use super::tree::TreeNode;
struct Solution;
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_sorted_array_to_bst(&nums, 0, nums.len())
    }

    fn _sorted_array_to_bst(
        nums: &Vec<i32>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if end == start {
            return None;
        }
        let mid = (start + end) / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::_sorted_array_to_bst(&nums, start, mid),
            right: Self::_sorted_array_to_bst(&nums, mid + 1, end),
        })))
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array_to_bst() {
        assert_eq!(
            TreeNode::to_string(Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])),
            "[0,-3,9,-10,null,5]"
        );

        assert_eq!(
            TreeNode::to_string(Solution::sorted_array_to_bst(vec![-10, -3, 0, 5])),
            "[0,-3,5,-10]"
        );

        assert_eq!(
            TreeNode::to_string(Solution::sorted_array_to_bst(vec![-10, -3, 0, 5])),
            "[0,-3,5,-10]"
        );
    }
}
