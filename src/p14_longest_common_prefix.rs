/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (35.69%)
 * Likes:    3150
 * Dislikes: 1998
 * Total Accepted:    849.3K
 * Total Submissions: 2.4M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * Write a function to find the longest common prefix string amongst an array
 * of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] consists of only lower-case English letters.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut chars: Vec<std::str::Chars> = strs.iter().map(|x| x.chars()).collect();
        let mut prefix = String::new();
        if let Some(mut s) = chars.pop() {
            while let Some(c) = s.next() {
                for i in 0..chars.len() {
                    if let Some(c2) = chars[i].next() {
                        if c != c2 {
                            return prefix;
                        }
                    } else {
                        return prefix;
                    }
                }
                prefix.push(c);
            }
        }
        prefix
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );

        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );

        assert_eq!(
            "",
            Solution::longest_common_prefix(vec!["".to_string(), "".to_string(), "".to_string()])
        );

        assert_eq!("", Solution::longest_common_prefix(vec![]));
    }
}
