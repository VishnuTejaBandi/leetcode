use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> usize {
    let mut window_map = HashMap::new();

    let mut start = 0;
    let mut result = 0;

    for (index, char) in s.chars().enumerate() {
        if let Some(prev_index) = window_map.get(&char) {
            if *prev_index >= start {
                start = prev_index + 1;
            }
        }

        window_map.insert(char, index);
        result = result.max(index.saturating_sub(start) + 1);
    }

    result
}
