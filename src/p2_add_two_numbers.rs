/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (34.32%)
 * Likes:    9503
 * Dislikes: 2392
 * Total Accepted:    1.6M
 * Total Submissions: 4.6M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 *
 * Example 1:
 *
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut current = head.as_mut();
        let mut carry = 0;
        let mut ml1 = l1;
        let mut ml2 = l2;

        while ml1.is_some() || ml2.is_some() {
            let mut sum = carry;
            if let Some(node) = ml1 {
                sum += node.val;
                ml1 = node.next;
            }
            if let Some(node) = ml2 {
                sum += node.val;
                ml2 = node.next;
            }
            carry = sum / 10;
            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = node.next.as_mut();
            }
        }
        if carry > 0 {
            current.unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        head.unwrap().next
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::super::list::ListNode;
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let l3 = Solution::add_two_numbers(l1, l2);

        assert!(l3.is_some());
        let l4 = l3.unwrap();
        assert_eq!(l4.val, 0);
        assert_eq!(l4.next, None);

        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(1)));
        let l3 = Solution::add_two_numbers(l1, l2);

        assert!(l3.is_some());
        let l4 = l3.unwrap();
        assert_eq!(l4.val, 1);
        assert_eq!(l4.next, None);
    }

    #[test]
    fn test_add_two_numbers_example() {
        assert_eq!(
            vec![7, 0, 8],
            Solution::add_two_numbers(ListNode::from(vec![2, 4, 3]), ListNode::from(vec![5, 6, 4]))
                .expect("[7, 0, 8]")
        );
        assert_eq!(
            vec![0],
            Solution::add_two_numbers(ListNode::from(vec![0]), ListNode::from(vec![0]))
                .expect("[0]")
        );

        assert_eq!(
            vec![8, 9, 9, 9, 0, 0, 0, 1],
            Solution::add_two_numbers(
                ListNode::from(vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::from(vec![9, 9, 9, 9]),
            )
            .expect("[8, 9, 9, 9, 0, 0, 0, 1]")
        );
    }
}
