// LeetCode 42: Trapping Rain Water
//
// Given n non-negative integers representing an elevation map where the width of each bar is 1,
// compute how much water it is able to trap after raining.
//
// Example:
// Input: [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
//
// https://leetcode.com/problems/trapping-rain-water/

pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 3 {
        return 0;
    }

    let mut left: usize = 1;
    let mut right = height.len() - 2;
    let (mut lmax, mut rmax) = (height[0], *height.last().unwrap());

    while left < right {}

    // TODO: Implement solution for problem 42
    todo!("Implement trap")
}
