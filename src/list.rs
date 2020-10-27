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

    #[allow(dead_code)]
    pub fn as_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vals: Vec<i32> = Vec::new();
        if head.is_none() {
            return vals;
        }
        let mut current = head;
        while let Some(node) = current {
            vals.push(node.val);
            current = node.next;
        }
        vals
    }
}
