/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 *
 * https://leetcode.com/problems/roman-to-integer/description/
 *
 * algorithms
 * Easy (56.10%)
 * Likes:    2593
 * Dislikes: 3743
 * Total Accepted:    790.5K
 * Total Submissions: 1.4M
 * Testcase Example:  '"III"'
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D
 * and M.
 *
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 *
 * For example, 2 is written as II in Roman numeral, just two one's added
 * together. 12 is written as XII, which is simply X + II. The number 27 is
 * written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right.
 * However, the numeral for four is not IIII. Instead, the number four is
 * written as IV. Because the one is before the five we subtract it making
 * four. The same principle applies to the number nine, which is written as IX.
 * There are six instances where subtraction is used:
 *
 *
 * I can be placed before V (5) and X (10) to make 4 and 9.
 * X can be placed before L (50) and C (100) to make 40 and 90.
 * C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 *
 * Given a roman numeral, convert it to an integer.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "III"
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: s = "IV"
 * Output: 4
 *
 *
 * Example 3:
 *
 *
 * Input: s = "IX"
 * Output: 9
 *
 *
 * Example 4:
 *
 *
 * Input: s = "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 *
 *
 * Example 5:
 *
 *
 * Input: s = "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 15
 * s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
 * It is guaranteed that s is a valid roman numeral in the range [1, 3999].
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let mut chars = s.chars();
        let mut val: i32 = 0;
        let mut prev = val;
        while let Some(c) = chars.next() {
            let v = match c {
                'I' => 1,
                'X' => 10,
                'C' => 100,
                'V' => 5,
                'L' => 50,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            val += v;
            if prev < v {
                val -= prev + prev;
            }
            prev = v;
        }
        return val;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(1, Solution::roman_to_int(String::from("I")));
        assert_eq!(2, Solution::roman_to_int(String::from("II")));
        assert_eq!(3, Solution::roman_to_int(String::from("III")));
        assert_eq!(5, Solution::roman_to_int(String::from("V")));
        assert_eq!(4, Solution::roman_to_int(String::from("IV")));
        assert_eq!(6, Solution::roman_to_int(String::from("VI")));
        assert_eq!(9, Solution::roman_to_int(String::from("IX")));
        assert_eq!(58, Solution::roman_to_int(String::from("LVIII")));
        assert_eq!(1994, Solution::roman_to_int(String::from("MCMXCIV")));
    }
}
