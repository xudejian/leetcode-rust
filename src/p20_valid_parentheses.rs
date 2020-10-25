/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (39.25%)
 * Likes:    5890
 * Dislikes: 244
 * Total Accepted:    1.2M
 * Total Submissions: 2.9M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 * Example 4:
 *
 *
 * Input: s = "([)]"
 * Output: false
 *
 *
 * Example 5:
 *
 *
 * Input: s = "{[]}"
 * Output: true
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let mut parents = std::collections::HashMap::new();
        parents.insert(')', '(');
        parents.insert(']', '[');
        parents.insert('}', '{');
        let mut stack = vec![];
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if parents.contains_key(&c) {
                if let Some(n) = stack.pop() {
                    if n != parents[&c] {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert!(Solution::is_valid("()".to_string()));
        assert!(Solution::is_valid("()[]{}".to_string()));
        assert!(!Solution::is_valid("(]".to_string()));
        assert!(!Solution::is_valid("([)]".to_string()));
        assert!(Solution::is_valid("{[]}".to_string()));
    }
}
