/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
 *
 * algorithms
 * Easy (45.85%)
 * Likes:    1913
 * Dislikes: 124
 * Total Accepted:    512.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '[1,1,2]'
 *
 * Given a sorted linked list, delete all duplicates such that each element
 * appear only once.
 *
 * Example 1:
 *
 *
 * Input: 1->1->2
 * Output: 1->2
 *
 *
 * Example 2:
 *
 *
 * Input: 1->1->2->3->3
 * Output: 1->2->3
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut cur = head.as_mut().unwrap();
        while let Some(n) = cur.next.as_mut() {
            if cur.val == n.val {
                cur.next = n.next.take();
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }
        head
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::super::list::ListNode;
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        assert_eq!(
            vec![1, 2],
            Solution::delete_duplicates(ListNode::from(vec![1, 1, 2])).expect("[1, 2]")
        );
        assert_eq!(Solution::delete_duplicates(None), None);
        assert_eq!(
            vec![1, 2, 3],
            Solution::delete_duplicates(ListNode::from(vec![1, 2, 3])).expect("[1,2,3]")
        );
    }
}
