/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 *
 * https://leetcode.com/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (39.81%)
 * Likes:    2773
 * Dislikes: 4562
 * Total Accepted:    695.3K
 * Total Submissions: 1.7M
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as
 * one sorted array.
 *
 * Note:
 *
 *
 * The number of elements initialized in nums1 and nums2 are m and n
 * respectively.
 * You may assume that nums1 has enough space (size that is equal to m + n) to
 * hold additional elements from nums2.
 *
 *
 * Example:
 *
 *
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * Output: [1,2,2,3,5,6]
 *
 *
 *
 * Constraints:
 *
 *
 * -10^9 <= nums1[i], nums2[i] <= 10^9
 * nums1.length == m + n
 * nums2.length == n
 *
 *
 */
struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut r1, mut r2, mut l1) = ((m + n) as usize, n as usize, m as usize);
        while l1 > 0 && r2 > 0 {
            r1 -= 1;
            nums1[r1] = if nums1[l1 - 1] > nums2[r2 - 1] {
                l1 -= 1;
                nums1[l1]
            } else {
                r2 -= 1;
                nums2[r2]
            };
        }
        while r2 > 0 {
            r2 -= 1;
            r1 -= 1;
            nums1[r1] = nums2[r2];
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let nums1 = &mut vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let nums2 = &mut vec![2, 5, 6];
        let n = 3;

        let expect: Vec<i32> = vec![1, 2, 2, 3, 5, 6];
        Solution::merge(nums1, m, nums2, n);
        assert_eq!(expect, *nums1);
    }

    #[test]
    fn test_merge_1() {
        let nums1 = &mut vec![2, 5, 6, 0, 0, 0];
        let m = 3;
        let nums2 = &mut vec![1, 2, 3];
        let n = 3;

        let expect: Vec<i32> = vec![1, 2, 2, 3, 5, 6];
        Solution::merge(nums1, m, nums2, n);
        assert_eq!(expect, *nums1);
    }

    #[test]
    fn test_merge_2() {
        let nums1 = &mut vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let nums2 = &mut vec![1, 2, 3];
        let n = 3;

        let expect: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        Solution::merge(nums1, m, nums2, n);
        assert_eq!(expect, *nums1);
    }
}
