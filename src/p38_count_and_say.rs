/*
 * @lc app=leetcode id=38 lang=rust
 *
 * [38] Count and Say
 *
 * https://leetcode.com/problems/count-and-say/description/
 *
 * algorithms
 * Easy (45.16%)
 * Likes:    35
 * Dislikes: 45
 * Total Accepted:    440.8K
 * Total Submissions: 974.1K
 * Testcase Example:  '1'
 *
 * The count-and-say sequence is a sequence of digit strings defined by the
 * recursive formula:
 *
 *
 * countAndSay(1) = "1"
 * countAndSay(n) is the way you would "say" the digit string from
 * countAndSay(n-1), which is then converted into a different digit string.
 *
 *
 * To determine how you "say" a digit string, split it into the minimal number
 * of groups so that each group is a contiguous section all of the same
 * character. Then for each group, say the number of characters, then say the
 * character. To convert the saying into a digit string, replace the counts
 * with a number and concatenate every saying.
 *
 * For example, the saying and conversion for digit string "3322251":
 *
 * Given a positive integer n, return the n^th term of the count-and-say
 * sequence.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 1
 * Output: "1"
 * Explanation: This is the base case.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 4
 * Output: "1211"
 * Explanation:
 * countAndSay(1) = "1"
 * countAndSay(2) = say "1" = one 1 = "11"
 * countAndSay(3) = say "11" = two 1's = "21"
 * countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 30
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn count_and_say(n: i32) -> String {
        if n < 2 {
            return "1".to_string();
        }
        let s = Self::count_and_say(n - 1);
        let mut count = vec![0];
        let mut nums = vec!['!'];
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == nums[nums.len() - 1] {
                count[nums.len() - 1] += 1;
            } else {
                nums.push(c);
                count.push(1);
            }
        }
        let mut res = String::new();
        for i in 1..nums.len() {
            res.push_str(count[i].to_string().as_str());
            res.push(nums[i]);
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!("1", Solution::count_and_say(1));
        assert_eq!("11", Solution::count_and_say(2));
        assert_eq!("21", Solution::count_and_say(3));
        assert_eq!("1211", Solution::count_and_say(4));
    }
}
