/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 *
 * https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/
 *
 * algorithms
 * Easy (57.58%)
 * Likes:    3152
 * Dislikes: 1850
 * Total Accepted:    698.3K
 * Total Submissions: 1.2M
 * Testcase Example:  '[7,1,5,3,6,4]'
 *
 * Say you have an array prices for which the i^th element is the price of a
 * given stock on day i.
 *
 * Design an algorithm to find the maximum profit. You may complete as many
 * transactions as you like (i.e., buy one and sell one share of the stock
 * multiple times).
 *
 * Note: You may not engage in multiple transactions at the same time (i.e.,
 * you must sell the stock before you buy again).
 *
 * Example 1:
 *
 *
 * Input: [7,1,5,3,6,4]
 * Output: 7
 * Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit
 * = 5-1 = 4.
 * Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 =
 * 3.
 *
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit
 * = 5-1 = 4.
 * Note that you cannot buy on day 1, buy on day 2 and sell them later, as you
 * are
 * engaging multiple transactions at the same time. You must sell before buying
 * again.
 *
 *
 * Example 3:
 *
 *
 * Input: [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 *
 *
 * Constraints:
 *
 *
 * 1 <= prices.length <= 3 * 10 ^ 4
 * 0 <= prices[i] <= 10 ^ 4
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 1 {
            return 0;
        }
        let mut sum = 0;
        let mut max_gain = 0;
        let mut buy = prices[0];
        for i in 1..prices.len() {
            if prices[i] < prices[i - 1] {
                sum += max_gain;
                max_gain = 0;
                buy = prices[i];
            } else {
                max_gain = max_gain.max(prices[i] - buy);
                buy = buy.min(prices[i]);
            }
        }
        sum + max_gain
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(0, Solution::max_profit([].to_vec()));
        assert_eq!(7, Solution::max_profit([7, 1, 5, 3, 6, 4].to_vec()));
        assert_eq!(4, Solution::max_profit([1, 2, 3, 4, 5].to_vec()));
        assert_eq!(0, Solution::max_profit([7, 6, 4, 3, 1].to_vec()));
    }
}
