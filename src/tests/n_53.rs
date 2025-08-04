#[cfg(test)]
mod tests {
    use crate::problems::n_53::max_sub_array;

    #[test]
    fn test_basic_example() {
        assert_eq!(max_sub_array(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_single_positive() {
        assert_eq!(max_sub_array(&[1]), 1);
    }

    #[test]
    fn test_single_negative() {
        assert_eq!(max_sub_array(&[-1]), -1);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(max_sub_array(&[-2, -3, -1, -4]), -1);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(max_sub_array(&[2, 3, 1, 4]), 10);
    }

    #[test]
    fn test_max_at_end() {
        assert_eq!(max_sub_array(&[-2, -1, 3, 4]), 7);
    }

    #[test]
    fn test_max_at_start() {
        assert_eq!(max_sub_array(&[5, -2, 1, -3]), 5);
    }

    #[test]
    fn test_large_negative_middle() {
        assert_eq!(max_sub_array(&[1, 2, 3, -100, 4, 5, 6]), 15);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(max_sub_array(&[0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_zero_and_negatives() {
        assert_eq!(max_sub_array(&[-1, 0, -2]), 0);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(max_sub_array(&[10_000, -1, 10_000]), 19_999);
    }

    #[test]
    fn test_alternating_signs() {
        assert_eq!(max_sub_array(&[1, -2, 3, -4, 5, -6, 7]), 7);
    }

    #[test]
    fn test_min_max_values() {
        assert_eq!(max_sub_array(&[i32::MIN, i32::MAX]), i32::MAX);
    }
}
