/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
 *
 * https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
 *
 * algorithms
 * Easy (37.68%)
 * Likes:    1873
 * Dislikes: 766
 * Total Accepted:    481.6K
 * Total Submissions: 1.2M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, find its minimum depth.
 *
 * The minimum depth is the number of nodes along the shortest path from the
 * root node down to the nearest leaf node.
 *
 * Note: A leaf is a node with no children.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: root = [2,null,3,null,4,null,5,null,6]
 * Output: 5
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 10^5].
 * -1000 <= Node.val <= 1000
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_recur(&root)
        // Self::min_depth_iter(root)
    }

    fn min_depth_recur(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => {
                let rb = r.borrow();
                let hl = if rb.left.is_some() {
                    Self::min_depth_recur(&rb.left)
                } else {
                    0
                };
                let hr = if rb.right.is_some() {
                    Self::min_depth_recur(&rb.right)
                } else {
                    0
                };
                if hl == 0 || hr == 0 {
                    1 + hr + hl
                } else {
                    if hl < hr {
                        1 + hl
                    } else {
                        1 + hr
                    }
                }
            }
            _ => 0,
        }
    }

    #[allow(dead_code)]
    fn min_depth_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut depth = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while queue.len() > 0 {
            let l = queue.len();
            depth += 1;
            for _ in 0..l {
                if let Some(rr) = queue.pop_front().flatten() {
                    let rb = rr.borrow();
                    if rb.left.is_none() && rb.right.is_none() {
                        return depth;
                    }
                    if rb.left.is_some() {
                        queue.push_back(rb.left.clone());
                    }
                    if rb.right.is_some() {
                        queue.push_back(rb.right.clone());
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
    fn test_min_depth() {
        assert_eq!(0, Solution::min_depth(None));
        assert_eq!(1, Solution::min_depth(TreeNode::from("[3]")));
        assert_eq!(
            2,
            Solution::min_depth(TreeNode::from("[3,9,20,null,null,15,7]"))
        );
        assert_eq!(
            5,
            Solution::min_depth(TreeNode::from("[2,null,3,null,4,null,5,null,6]"))
        );
    }
}
