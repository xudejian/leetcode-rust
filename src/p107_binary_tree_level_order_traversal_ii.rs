/*
 * @lc app=leetcode id=107 lang=rust
 *
 * [107] Binary Tree Level Order Traversal II
 *
 * https://leetcode.com/problems/binary-tree-level-order-traversal-ii/description/
 *
 * algorithms
 * Easy (54.15%)
 * Likes:    1751
 * Dislikes: 226
 * Total Accepted:    380.9K
 * Total Submissions: 702K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, return the bottom-up level order traversal of its
 * nodes' values. (ie, from left to right, level by level from leaf to root).
 *
 *
 * For example:
 * Given binary tree [3,9,20,null,null,15,7],
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 *
 *
 *
 * return its bottom-up level order traversal as:
 *
 * [
 * ⁠ [15,7],
 * ⁠ [9,20],
 * ⁠ [3]
 * ]
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut rv: Vec<Vec<i32>> = Vec::new();
        if root.is_none() {
            return rv;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while queue.len() > 0 {
            let len = queue.len();
            let mut rows: Vec<i32> = Vec::new();
            for _ in 0..len {
                queue.pop_front().flatten().map(|r| {
                    let rb = r.borrow();
                    rows.push(rb.val);
                    if rb.left.is_some() {
                        queue.push_back(rb.left.clone());
                    }
                    if rb.right.is_some() {
                        queue.push_back(rb.right.clone());
                    }
                });
            }
            rv.push(rows);
        }
        rv.reverse();
        rv
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order_bottom() {
        let expect: Vec<Vec<i32>> = vec![vec![15, 7], vec![9, 20], vec![3]];
        assert_eq!(
            expect,
            Solution::level_order_bottom(TreeNode::from("[3,9,20,null,null,15,7]"))
        );
    }
}
