/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (54.42%)
 * Likes:    5167
 * Dislikes: 647
 * Total Accepted:    1.2M
 * Total Submissions: 2.1M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * Merge two sorted linked lists and return it as a new sorted list. The new
 * list should be made by splicing together the nodes of the first two
 * lists.
 *
 *
 * Example 1:
 *
 *
 * Input: l1 = [1,2,4], l2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [], l2 = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [], l2 = [0]
 * Output: [0]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in both lists is in the range [0, 50].
 * -100 <= Node.val <= 100
 * Both l1 and l2 are sorted in non-decreasing order.
 *
 *
 */
use super::list::ListNode;
struct Solution;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut lhs, mut rhs) = (l1, l2);
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        while let (Some(l), Some(r)) = (lhs.as_ref(), rhs.as_ref()) {
            if l.val <= r.val {
                tail.next = lhs;
                tail = tail.next.as_mut().unwrap();
                lhs = tail.next.take();
            } else {
                tail.next = rhs;
                tail = tail.next.as_mut().unwrap();
                rhs = tail.next.take();
            }
        }
        tail.next = if lhs.is_some() { lhs } else { rhs };
        head.next
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::super::list::ListNode;
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(
            vec![1, 1, 2, 3, 4, 4],
            Solution::merge_two_lists(ListNode::from(vec![1, 2, 4]), ListNode::from(vec![1, 3, 4]))
                .expect("[1, 1, 2, 3, 4, 4]")
        );
        assert_eq!(Solution::merge_two_lists(None, None), None);
        assert_eq!(
            vec![0],
            Solution::merge_two_lists(None, Some(Box::new(ListNode::new(0)))).expect("[0]")
        );
    }
}
