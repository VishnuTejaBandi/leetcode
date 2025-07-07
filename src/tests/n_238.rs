#[cfg(test)]
mod tests {
    use crate::problems::n_238::product_except_self;

    #[test]
    fn test_basic() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
    #[test]
    fn test_with_zero() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
    #[test]
    fn test_all_positive() {
        assert_eq!(product_except_self(vec![2, 3, 4, 5]), vec![60, 40, 30, 24]);
    }
    #[test]
    fn test_one_zero() {
        assert_eq!(product_except_self(vec![1, 0]), vec![0, 1]);
    }
    #[test]
    fn test_two_zeros() {
        assert_eq!(product_except_self(vec![0, 0]), vec![0, 0]);
    }
    #[test]
    fn test_single_element() {
        assert_eq!(product_except_self(vec![1]), vec![1]);
    }
}
