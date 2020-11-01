/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 *
 * https://leetcode.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (47.35%)
 * Likes:    4928
 * Dislikes: 118
 * Total Accepted:    744.2K
 * Total Submissions: 1.6M
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * Given a binary tree, check whether it is a mirror of itself (ie, symmetric
 * around its center).
 *
 * For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
 *
 *
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠/ \ / \
 * 3  4 4  3
 *
 *
 *
 *
 * But the following [1,2,2,null,3,null,3] is not:
 *
 *
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠  \   \
 * ⁠  3    3
 *
 *
 *
 *
 * Follow up: Solve it both recursively and iteratively.
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_symmetric_recur(root.clone()) && Self::is_symmetric_iter(root.clone())
    }

    fn is_symmetric_recur(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match &root {
            Some(r) => {
                Self::is_symmetric_recur_12(r.borrow().left.clone(), r.borrow().right.clone())
            }
            _ => true,
        }
    }

    fn is_symmetric_recur_12(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (&left, &right) {
            (Some(rl), Some(rr)) => {
                let nl = rl.borrow();
                let nr = rr.borrow();
                nl.val == nr.val
                    && Self::is_symmetric_recur_12(nl.left.clone(), nr.right.clone())
                    && Self::is_symmetric_recur_12(nl.right.clone(), nr.left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn is_symmetric_iter(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        if let Some(n) = &root {
            queue.push_back(n.borrow().left.clone());
            queue.push_back(n.borrow().right.clone());
        }
        while let (Some(left), Some(right)) = (queue.pop_front(), queue.pop_front()) {
            match (&left, &right) {
                (Some(l), Some(r)) => {
                    let (bl, br) = (l.borrow(), r.borrow());
                    if bl.val != br.val {
                        return false;
                    }
                    queue.push_back(bl.left.clone());
                    queue.push_back(br.right.clone());
                    queue.push_back(bl.right.clone());
                    queue.push_back(br.left.clone());
                }
                (None, None) => (),
                _ => return false,
            }
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_symmetric() {
        let tree = TreeNode::from("[1, 2, 2, 3, 4, 4, 3]");
        assert!(Solution::is_symmetric(tree));
        let tree = TreeNode::from("[1,2,2,null,3,null,3]");
        assert!(!Solution::is_symmetric(tree));

        let tree = TreeNode::from("[1,2]");
        assert!(!Solution::is_symmetric(tree));
    }
}
