/*
 * @lc app=leetcode id=297 lang=rust
 *
 * [297] Serialize and Deserialize Binary Tree
 *
 * https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/
 *
 * algorithms
 * Hard (48.34%)
 * Likes:    3576
 * Dislikes: 172
 * Total Accepted:    373.6K
 * Total Submissions: 769K
 * Testcase Example:  '[1,2,3,null,null,4,5]'
 *
 * Serialization is the process of converting a data structure or object into a
 * sequence of bits so that it can be stored in a file or memory buffer, or
 * transmitted across a network connection link to be reconstructed later in
 * the same or another computer environment.
 *
 * Design an algorithm to serialize and deserialize a binary tree. There is no
 * restriction on how your serialization/deserialization algorithm should work.
 * You just need to ensure that a binary tree can be serialized to a string and
 * this string can be deserialized to the original tree structure.
 *
 * Clarification: The input/output format is the same as how LeetCode
 * serializes a binary tree. You do not necessarily need to follow this format,
 * so please be creative and come up with different approaches yourself.
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 *
 *
 * Example 2:
 *
 *
 * Input: root = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: root = [1]
 * Output: [1]
 *
 *
 * Example 4:
 *
 *
 * Input: root = [1,2]
 * Output: [1,2]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the tree is in the range [0, 10^4].
 * -1000 <= Node.val <= 1000
 *
 *
 */
use super::tree::TreeNode;

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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res: Vec<String> = Vec::new();
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while let Some(root) = queue.pop_front() {
            match &root {
                Some(rr) => {
                    let r = rr.borrow();
                    res.push(r.val.to_string());
                    queue.push_back(r.left.clone());
                    queue.push_back(r.right.clone());
                }
                _ => res.push("null".to_string()),
            }
        }
        while let Some(s) = res.pop() {
            if s != "null" {
                res.push(s);
                break;
            }
        }
        let mut s = String::new();
        s.push('[');
        s.push_str(&res.join(","));
        s.push(']');
        s
    }

    #[allow(dead_code)]
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() < 3 {
            return None;
        }
        let mut queue = VecDeque::new();
        let head = Rc::new(RefCell::new(TreeNode::new(0)));
        let cur = head.clone();
        let s = &data[1..(data.len() - 1)];
        let mut nums = s.split(',');
        if let Some(x) = nums.next() {
            if let Ok(i) = x.parse::<i32>() {
                cur.borrow_mut().val = i;
                queue.push_back(cur.clone());
            } else {
                return None;
            }
        } else {
            return None;
        }
        while let (Some(x), Some(cur)) = (nums.next(), queue.pop_front()) {
            if let Ok(i) = x.parse::<i32>() {
                let left = Rc::new(RefCell::new(TreeNode::new(i)));
                cur.borrow_mut().left = Some(left.clone());
                queue.push_back(left.clone());
            }
            if let Some(xr) = nums.next() {
                if let Ok(i) = xr.parse::<i32>() {
                    let right = Rc::new(RefCell::new(TreeNode::new(i)));
                    cur.borrow_mut().right = Some(right.clone());
                    queue.push_back(right.clone());
                }
            }
        }
        Some(head)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitree_serialize() {
        let codec = Codec::new();
        let obj: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let data: String = codec.serialize(obj);
        assert_eq!(data, "[1,2,3,null,null,4,5]");
    }

    #[test]
    fn test_bitree_serialize2() {
        let codec = Codec::new();
        let obj: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: None,
        })));
        let data: String = codec.serialize(obj);
        assert_eq!(data, "[1,2,null,3]");

        let ans: Option<Rc<RefCell<TreeNode>>> = codec.deserialize("[1,2,null,3]".to_string());
        assert!(ans.is_some());

        let data: String = codec.serialize(ans);
        assert_eq!(data, "[1,2,null,3]");
    }

    #[test]
    fn test_bitree_serialize3() {
        let codec = Codec::new();
        let obj: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));
        let data: String = codec.serialize(obj);
        assert_eq!(data, "[1,null,2,null,4]");

        let ans: Option<Rc<RefCell<TreeNode>>> = codec.deserialize("[1,null,2,null,4]".to_string());
        assert!(ans.is_some());

        let data: String = codec.serialize(ans);
        assert_eq!(data, "[1,null,2,null,4]");
    }

    #[test]
    fn test_bitree_deserialize() {
        let codec = Codec::new();
        let ans: Option<Rc<RefCell<TreeNode>>> =
            codec.deserialize("[1,2,3,null,null,4,5]".to_string());
        assert!(ans.is_some());

        let data: String = codec.serialize(ans);
        assert_eq!(data, "[1,2,3,null,null,4,5]");
    }

    #[test]
    fn test_bitree_deserialize2() {
        let codec = Codec::new();
        let ans: Option<Rc<RefCell<TreeNode>>> = codec.deserialize("[]".to_string());
        assert!(ans.is_none());

        let data: String = codec.serialize(ans);
        assert_eq!(data, "[]");

        let ans: Option<Rc<RefCell<TreeNode>>> = codec.deserialize("[null]".to_string());
        assert!(ans.is_none());

        let data: String = codec.serialize(ans);
        assert_eq!(data, "[]");
    }
}
