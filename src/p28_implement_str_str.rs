/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 *
 * https://leetcode.com/problems/implement-strstr/description/
 *
 * algorithms
 * Easy (34.74%)
 * Likes:    1894
 * Dislikes: 2085
 * Total Accepted:    749.2K
 * Total Submissions: 2.2M
 * Testcase Example:  '"hello"\n"ll"'
 *
 * Implement strStr().
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if
 * needle is not part of haystack.
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great
 * question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty
 * string. This is consistent to C's strstr() and Java's indexOf().
 *
 *
 * Example 1:
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 * Example 2:
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 * Example 3:
 * Input: haystack = "", needle = ""
 * Output: 0
 *
 *
 * Constraints:
 *
 *
 * 0 <= haystack.length, needle.length <= 5 * 10^4
 * haystack and needle consist of only lower-case English characters.
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }
        // 01234
        //    34
        let (src, dest) = (haystack.as_bytes(), needle.as_bytes());
        for left in 0..(src.len() - dest.len() + 1) {
            let mut ok = true;
            for i in 0..dest.len() {
                if src[left + i] != dest[i] {
                    ok = false;
                    break;
                }
            }
            if ok {
                return left as i32;
            }
        }
        -1
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strstr() {
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
        assert_eq!(
            -1,
            Solution::str_str("aaaaa".to_string(), "bba".to_string())
        );
        assert_eq!(0, Solution::str_str("".to_string(), "".to_string()));

        assert_eq!(1, Solution::str_str("abba".to_string(), "bba".to_string()));
        assert_eq!(
            3,
            Solution::str_str("aaabaaaaaaaaaa".to_string(), "baa".to_string())
        );
        assert_eq!(0, Solution::str_str("a".to_string(), "".to_string()));
        assert_eq!(-1, Solution::str_str("".to_string(), "a".to_string()));
        assert_eq!(
            -1,
            Solution::str_str("abb".to_string(), "abaaa".to_string())
        );
    }

    #[test]
    fn test_grammer() {
        for i in 0..=-3 {
            panic!("should not enter here! {}", i);
        }
        for i in 0..0 {
            panic!("should not enter here! {}", i);
        }
        let mut hello = String::from("Hello, World!");
        let world = hello.split_off(hello.len());
        assert_eq!(hello, "Hello, World!");
        assert_eq!(world, "");
    }
}
