use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut d_set = HashSet::<&i32>::new();

    for ele in nums.iter() {
        if !d_set.insert(ele) {
            return true;
        }
    }

    return false;
}
