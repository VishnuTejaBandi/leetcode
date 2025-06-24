use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts = HashMap::new();

    for c in s.into_bytes() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for c in t.into_bytes() {
        *counts.entry(c).or_insert(0) -= 1;
    }

    return counts.values().all(|c| *c == 0);
}
