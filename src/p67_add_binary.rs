/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 *
 * https://leetcode.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (45.91%)
 * Likes:    2215
 * Dislikes: 300
 * Total Accepted:    523.7K
 * Total Submissions: 1.1M
 * Testcase Example:  '"11"\n"1"'
 *
 * Given two binary strings, return their sum (also a binary string).
 *
 * The input strings are both non-empty and contains only characters 1 or 0.
 *
 * Example 1:
 *
 *
 * Input: a = "11", b = "1"
 * Output: "100"
 *
 * Example 2:
 *
 *
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *
 *
 * Constraints:
 *
 *
 * Each string consists only of '0' or '1' characters.
 * 1 <= a.length, b.length <= 10^4
 * Each string is either "0" or doesn't contain any leading zero.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        if a.len() < b.len() {
            return Self::add_binary(b, a);
        }
        let u0 = '0' as u8;
        let mut take = 0u8;
        let mut c: Vec<u8> = a
            .chars()
            .rev()
            .zip(b.chars().rev().chain(String::from("0").chars().cycle()))
            .map(|x| {
                let sum = take + x.0 as u8 - u0 + x.1 as u8 - u0;
                take = if sum > 1 { 1 } else { 0 };
                sum % 2 + u0
            })
            .collect();
        if take > 0 {
            c.push('1' as u8);
        }
        c.reverse();
        String::from_utf8(c).unwrap()
    }

    #[allow(dead_code)]
    pub fn add_binary_raw(a: String, b: String) -> String {
        if a.len() < b.len() {
            return Self::add_binary_raw(b, a);
        }
        let u0 = '0' as u8;
        let mut am = a;
        let mut take = 0u8;
        unsafe {
            let (ab, bb) = (am.as_bytes_mut(), b.as_bytes());
            let (ai, bi) = (ab.len(), bb.len());
            for i in 1..=bi {
                let sum = take + ab[ai - i] - u0 + bb[bi - i] - u0;
                ab[ai - i] = u0 + sum % 2;
                take = if sum > 1 { 1 } else { 0 };
            }
            if take > 0 {
                for i in (bi + 1)..=ai {
                    let sum = take + ab[ai - i] - u0;
                    ab[ai - i] = u0 + sum % 2;
                    take = if sum > 1 { 1 } else { 0 };
                }
            }
        }
        if take > 0 {
            am.insert(0, '1');
        }

        am
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(
            "100",
            Solution::add_binary("11".to_string(), "1".to_string())
        );

        assert_eq!(
            "10101",
            Solution::add_binary("1010".to_string(), "1011".to_string())
        );
        assert_eq!("0", Solution::add_binary("0".to_string(), "0".to_string()));
        assert_eq!("1", Solution::add_binary("0".to_string(), "1".to_string()));
        assert_eq!("10", Solution::add_binary("1".to_string(), "1".to_string()));
    }
}
