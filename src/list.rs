// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    #[allow(dead_code)]
    pub fn from(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        for x in vals.iter() {
            let mut next = current.unwrap();
            next.next = Some(Box::new(ListNode::new(*x)));
            current = next.next.as_mut();
        }
        head.unwrap().next
    }
}

impl PartialEq<Vec<i32>> for ListNode {
    fn eq(&self, other: &Vec<i32>) -> bool {
        if other.len() < 1 || other[0] != self.val {
            return false;
        }

        let mut i = 1;
        let mut current = &self.next;
        while let Some(node) = current {
            if i >= other.len() {
                return false;
            }
            if node.val != other[i] {
                return false;
            }
            i += 1;
            current = &node.next;
        }
        i == other.len()
    }
}

impl PartialEq<ListNode> for Vec<i32> {
    fn eq(&self, other: &ListNode) -> bool {
        other.eq(self)
    }
}

impl PartialEq<Box<ListNode>> for Vec<i32> {
    fn eq(&self, other: &Box<ListNode>) -> bool {
        *self == **other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_node_from() {
        let l1 = ListNode::from(vec![1, 2]);
        assert!(l1.is_some());
        let n1 = l1.unwrap();
        assert_eq!(n1.val, 1);
        assert!(n1.next.is_some());
        let nn = n1.next.unwrap();
        assert_eq!(nn.val, 2);
    }

    #[test]
    fn test_listnode_eq() {
        assert_eq!(ListNode::new(1), vec![1]);
        assert_eq!(vec![1], ListNode::new(1));
        assert_eq!(
            ListNode {
                next: Some(Box::new(ListNode::new(2))),
                val: 1
            },
            vec![1, 2]
        );
    }
}
