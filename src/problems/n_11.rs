// LeetCode 11: Container With Most Water
//
// Given n non-negative integers a1, a2, ..., an, where each represents a point at coordinate (i, ai).
// n vertical lines are drawn such that the two endpoints of the line i is at (i, ai) and (i, 0).
// Find two lines, which together with the x-axis forms a container, such that the container contains the most water.
//
// Example:
// Input: [1,8,6,2,5,4,8,3,7]
// Output: 49
// https://leetcode.com/problems/container-with-most-water/description/

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0 as usize, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        max_area = max_area.max(height[right].min(height[left]) * (right - left) as i32);

        if height[left] > height[right] {
            right -= 1;
        } else if height[right] > height[left] {
            left += 1;
        } else {
            left += 1;
            right -= 1;
        }
    }

    max_area
}
