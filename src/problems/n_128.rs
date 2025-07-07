use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 || nums.len() == 1 {
        return nums.len() as i32;
    }

    let set:HashSet<i32> = nums.into_iter().collect();

    let mut longest = 1;

    for &e in &set {
        if !set.contains(&(e - 1)) {
            let mut current = 0;
            let mut v = e;

            while set.contains(&v) {
                current += 1;
                v += 1;
            }

            longest = current.max(longest);
        }
    }

    longest
}
