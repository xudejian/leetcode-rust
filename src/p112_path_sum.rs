/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
 *
 * https://leetcode.com/problems/path-sum/description/
 *
 * algorithms
 * Easy (41.61%)
 * Likes:    2420
 * Dislikes: 543
 * Total Accepted:    529.5K
 * Total Submissions: 1.3M
 * Testcase Example:  '[5,4,8,11,null,13,4,7,2,null,null,null,1]\n22'
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path
 * such that adding up all the values along the path equals the given sum.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given the below binary tree and sum = 22,
 *
 *
 * ⁠     5
 * ⁠    / \
 * ⁠   4   8
 * ⁠  /   / \
 * ⁠ 11  13  4
 * ⁠/  \      \
 * 7    2      1
 *
 *
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
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
    #[inline]
    #[allow(dead_code)]
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::_has_path_sum(&root, sum)
    }

    fn _has_path_sum(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let rr = root.as_ref().unwrap();
        let rb = rr.borrow();
        if rb.left.is_none() && rb.right.is_none() {
            return sum == rb.val;
        }
        if rb.left.is_some() {
            if Self::_has_path_sum(&rb.left, sum - rb.val) {
                return true;
            }
        }
        if rb.right.is_some() {
            return Self::_has_path_sum(&rb.right, sum - rb.val);
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_path_sum() {
        assert!(!Solution::has_path_sum(None, 0));
        assert!(!Solution::has_path_sum(None, 1));
        assert!(Solution::has_path_sum(
            TreeNode::from("[5,4,8,11,null,13,4,7,2,null,null,null,1]"),
            22
        ));
        assert!(!Solution::has_path_sum(TreeNode::from("[]"), 0));
        assert!(!Solution::has_path_sum(
            TreeNode::from("[5,4,8,11,null,13,4,7,2,null,null,null,1]"),
            0
        ));
    }
}
