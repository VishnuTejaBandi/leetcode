// Leetcode 53: Maximum Subarray
//
// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
//
// Example:
// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
//
// Constraints:
// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4

pub fn max_sub_array(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut sum = nums[0];

    for &ele in &nums[1..] {
        sum = sum.max(0) + ele;
        max_sum = max_sum.max(sum);
    }

    max_sum
}
