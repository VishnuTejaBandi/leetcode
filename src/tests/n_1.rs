#[cfg(test)]
mod tests {
    use crate::problems::n_1::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![1, 2, 3, 8, 5], 9), vec![0, 3])
    }

    #[test]
    fn test_basic_case() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_duplicate_values() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_negative_numbers() {
        let result = two_sum(vec![-3, 4, 3, 90], 0);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_negative_target() {
        let result = two_sum(vec![-1, -2, -3, -4, -5], -8);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_zero_values() {
        let result = two_sum(vec![0, 4, 3, 0], 0);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn test_single_negative_single_positive() {
        let result = two_sum(vec![-1, 2, 1, -4], 0);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_large_numbers() {
        let result = two_sum(vec![1000000, 2000000, 3000000], 5000000);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_solution_at_end() {
        let result = two_sum(vec![1, 2, 3, 4, 5, 6], 11);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_early_solution() {
        let result = two_sum(vec![5, 5, 10, 20], 10);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let result = two_sum(vec![1, -1, 0, 2], 1);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_same_number_different_indices() {
        let result = two_sum(vec![1, 1, 2, 2, 3], 4);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_minimum_array_size() {
        let result = two_sum(vec![1, 2], 3);
        assert_eq!(result, vec![0, 1]);
    }

    // Edge case tests
    #[test]
    fn test_max_min_i32() {
        let result = two_sum(vec![i32::MAX, i32::MIN, 0], -1);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_overflow_prevention() {
        // Test that complement calculation doesn't overflow
        let result = two_sum(vec![1, i32::MAX - 1], i32::MAX);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_long_array() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        nums.extend(10..100);
        let result = two_sum(nums, 150);
        // Should find indices for numbers that sum to 150
        assert_eq!(result.len(), 2);
    }
}
