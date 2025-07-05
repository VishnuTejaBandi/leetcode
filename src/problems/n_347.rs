use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();

    for n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::with_capacity((k + 1) as usize);

    for (num, freq) in counts {
        heap.push(Reverse((freq, num)));

        if heap.len() > k as usize {
            heap.pop();
        }
    }

    heap.into_iter().map(|r| r.0.1).collect()
}
