#[cfg(test)]
mod tests {
    use crate::problems::n_11::max_area;

    #[test]
    fn test_basic() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
    #[test]
    fn test_two_elements() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
    #[test]
    fn test_all_same() {
        assert_eq!(max_area(vec![5, 5, 5, 5, 5]), 20);
    }
    #[test]
    fn test_decreasing() {
        assert_eq!(max_area(vec![5, 4, 3, 2, 1]), 6);
    }
    #[test]
    fn test_increasing() {
        assert_eq!(max_area(vec![1, 2, 3, 4, 5]), 6);
    }
    #[test]
    fn test_with_zero() {
        assert_eq!(max_area(vec![0, 2, 0, 4, 0]), 4);
    }
    #[test]
    fn test_large_input() {
        let v = vec![1; 10000];
        assert_eq!(max_area(v), 9999);
    }
}
