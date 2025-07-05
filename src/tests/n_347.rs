#[cfg(test)]
mod tests {
    use crate::problems::n_347::top_k_frequent;

    use std::collections::HashMap;
    fn assert_unordered_eq(a: Vec<i32>, b: Vec<i32>) {
        let mut ma = HashMap::new();
        let mut mb = HashMap::new();
        for x in a {
            *ma.entry(x).or_insert(0) += 1;
        }
        for x in b {
            *mb.entry(x).or_insert(0) += 1;
        }
        assert_eq!(ma, mb);
    }

    #[test]
    fn test_top_k_frequent_basic() {
        assert_unordered_eq(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }
    #[test]
    fn test_top_k_frequent_single() {
        assert_unordered_eq(top_k_frequent(vec![1], 1), vec![1]);
    }
    #[test]
    fn test_top_k_frequent_negatives() {
        assert_unordered_eq(top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2), vec![-1, 2]);
    }
    #[test]
    fn test_top_k_frequent_large() {
        assert_unordered_eq(top_k_frequent(vec![5, 3, 1, 1, 1, 3, 73, 1], 2), vec![1, 3]);
    }
    #[test]
    fn test_top_k_frequent_empty() {
        assert_unordered_eq(top_k_frequent(vec![], 0), Vec::<i32>::new());
    }
    #[test]
    fn test_top_k_frequent_all_unique() {
        assert_unordered_eq(top_k_frequent(vec![1, 2, 3, 4, 5], 5), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_top_k_frequent_all_same() {
        assert_unordered_eq(top_k_frequent(vec![2, 2, 2, 2], 1), vec![2]);
    }
    #[test]
    fn test_top_k_frequent_descending_freq() {
        assert_unordered_eq(top_k_frequent(vec![1, 2, 2, 3, 3, 3], 3), vec![3, 2, 1]);
    }
    #[test]
    fn test_top_k_frequent_k1_len() {
        assert_eq!(top_k_frequent(vec![1, 2, 3, 4, 5], 1).len(), 1);
    }
    #[test]
    fn test_top_k_frequent_k2_len() {
        assert_eq!(top_k_frequent(vec![1, 1, 2, 2, 3, 3], 2).len(), 2);
    }
}
