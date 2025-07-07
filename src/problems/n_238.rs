pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![1; nums.len()];

    for i in 1..nums.len() {
        result[i] = nums[i - 1] * result[i - 1];
    }

    let mut acc = 1;
    for i in (0..nums.len()).rev() {
        result[i] *= acc;
        acc = nums[i] * acc;
    }

    result
}
