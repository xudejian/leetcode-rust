/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
 *
 * https://leetcode.com/problems/balanced-binary-tree/description/
 *
 * algorithms
 * Easy (43.84%)
 * Likes:    2724
 * Dislikes: 186
 * Total Accepted:    489.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, determine if it is height-balanced.
 *
 * For this problem, a height-balanced binary tree is defined as:
 *
 *
 * a binary tree in which the left and right subtrees of every node differ in
 * height by no more than 1.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: root = []
 * Output: true
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 5000].
 * -10^4 <= Node.val <= 10^4
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::balanced_height(&root) != -1
    }
    fn balanced_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root.as_ref() {
            Some(rrr) => {
                let rb = rrr.borrow();
                let hl = Self::balanced_height(&rb.left);
                if hl == -1 {
                    return -1;
                }
                let hr = Self::balanced_height(&rb.right);
                if hr == -1 || hl > hr + 1 || hr > hl + 1 {
                    -1
                } else {
                    if hl > hr {
                        1 + hl
                    } else {
                        1 + hr
                    }
                }
            }
            None => 0,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced() {
        assert!(Solution::is_balanced(None));
        assert!(Solution::is_balanced(TreeNode::from(
            "[3,9,20,null,null,15,7]"
        )));
        assert!(!Solution::is_balanced(TreeNode::from(
            "[1,2,2,3,3,null,null,4,4]"
        )));
        assert!(!Solution::is_balanced(TreeNode::from(
            "[1,2,2,3,3,3,3,4,4,4,4,null,null,null,null,5,5,5,5,5,5,5,5]"
        )));
    }
}
