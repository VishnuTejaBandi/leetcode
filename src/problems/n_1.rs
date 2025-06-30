use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();

    for (i, &ele) in nums.iter().enumerate() {
        let compliment = target - ele;

        if let Some(&j) = map.get(&compliment) {
            return vec![j, i as i32];
        }

        map.insert(ele, i as i32);
    }

    unreachable!();
}
