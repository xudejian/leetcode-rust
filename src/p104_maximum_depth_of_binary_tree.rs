/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 *
 * https://leetcode.com/problems/maximum-depth-of-binary-tree/description/
 *
 * algorithms
 * Easy (66.65%)
 * Likes:    3066
 * Dislikes: 82
 * Total Accepted:    932.6K
 * Total Submissions: 1.4M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, find its maximum depth.
 *
 * The maximum depth is the number of nodes along the longest path from the
 * root node down to the farthest leaf node.
 *
 * Note: A leaf is a node with no children.
 *
 * Example:
 *
 * Given binary tree [3,9,20,null,null,15,7],
 *
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 *
 * return its depth = 3.
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let a = Self::max_depth_recur(root.clone());
        let b = Self::max_depth_iter(root.clone());
        assert_eq!(a, b);
        a
    }

    fn max_depth_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => {
                let rb = r.borrow();
                let dl = Self::max_depth(rb.left.clone());
                let dr = Self::max_depth(rb.right.clone());
                1 + if dl < dr { dr } else { dl }
            }
            None => 0,
        }
    }
    fn max_depth_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut depth = 0;
        while queue.len() > 0 {
            depth += 1;
            let len = queue.len();
            for _ in 0..len {
                if let Some(rq) = queue.pop_front() {
                    match rq {
                        Some(r) => {
                            let rb = r.borrow();
                            if rb.left.is_some() {
                                queue.push_back(rb.left.clone());
                            }
                            if rb.right.is_some() {
                                queue.push_back(rb.right.clone());
                            }
                        }
                        None => (),
                    }
                }
            }
        }
        depth
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(
            3,
            Solution::max_depth(TreeNode::from("[3,9,20,null,null,15,7]"))
        );

        assert_eq!(0, Solution::max_depth(None));
        assert_eq!(1, Solution::max_depth(TreeNode::from("[3]")));
        assert_eq!(2, Solution::max_depth(TreeNode::from("[3,9]")));
    }
}
