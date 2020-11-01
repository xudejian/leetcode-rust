use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn to_string(root: Option<Rc<RefCell<TreeNode>>>) -> String {
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

    fn from(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree_node_from() {
        let ans: Option<Rc<RefCell<TreeNode>>> = TreeNode::from("[]");
        assert!(ans.is_none());

        let data: String = TreeNode::to_string(ans);
        assert_eq!(data, "[]");

        let ans: Option<Rc<RefCell<TreeNode>>> = TreeNode::from("[null]");
        assert!(ans.is_none());

        let data: String = TreeNode::to_string(ans);
        assert_eq!(data, "[]");

        let ans: Option<Rc<RefCell<TreeNode>>> = TreeNode::from("[1,2,3,null,null,4,5]");
        assert!(ans.is_some());

        let data: String = TreeNode::to_string(ans);
        assert_eq!(data, "[1,2,3,null,null,4,5]");
    }

    #[test]
    fn test_bitree_to_string() {
        let obj: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let data: String = TreeNode::to_string(obj);
        assert_eq!(data, "[1,2,3,null,null,4,5]");
    }

    #[test]
    fn test_bitree_serialize2() {
        let obj: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: None,
        })));
        let data: String = TreeNode::to_string(obj);
        assert_eq!(data, "[1,2,null,3]");

        let ans: Option<Rc<RefCell<TreeNode>>> = TreeNode::from("[1,2,null,3]");
        assert!(ans.is_some());

        let data: String = TreeNode::to_string(ans);
        assert_eq!(data, "[1,2,null,3]");
    }

    #[test]
    fn test_bitree_serialize3() {
        let obj: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));
        let data: String = TreeNode::to_string(obj);
        assert_eq!(data, "[1,null,2,null,4]");

        let ans: Option<Rc<RefCell<TreeNode>>> = TreeNode::from("[1,null,2,null,4]");
        assert!(ans.is_some());

        let data: String = TreeNode::to_string(ans);
        assert_eq!(data, "[1,null,2,null,4]");
    }

    #[test]
    fn test_tree_node_eq() {}
}
