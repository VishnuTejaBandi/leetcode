use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();

    let mut count: HashMap<u8, i32> = HashMap::with_capacity(26);
    let (mut left, mut max_count, mut result) = (0, 0, 0);

    for right in 0..s.len() {
        let idx = bytes[right] - b'A';
        *count.entry(idx).or_insert(0) += 1;
        max_count = max_count.max(count[&idx]);

        if (right - left + 1) as i32 - max_count > k {
            let left_idx = bytes[left] - b'A';
            count.entry(left_idx).and_modify(|v| *v -= 1);
            left += 1;
        }

        result = result.max(right - left + 1);
    }

    result as i32
}
