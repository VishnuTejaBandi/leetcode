#[cfg(test)]
mod tests {
    use crate::problems::n_128::longest_consecutive;

    #[test]
    fn test_basic() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
    #[test]
    fn test_single_element() {
        assert_eq!(longest_consecutive(vec![0]), 1);
    }
    #[test]
    fn test_empty() {
        assert_eq!(longest_consecutive(vec![]), 0);
    }
    #[test]
    fn test_duplicates() {
        assert_eq!(longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
    #[test]
    fn test_negative_numbers() {
        assert_eq!(longest_consecutive(vec![4,0,-4,-2,2,5,2,0,-8,-8,-8,-8,-1,7,4,5,5,-4,6,6,-3]), 5);
    }
    #[test]
    fn test_all_same() {
        assert_eq!(longest_consecutive(vec![7, 7, 7, 7]), 1);
    }
    #[test]
    fn test_large_gap() {
        assert_eq!(longest_consecutive(vec![1, 100, 200, 2, 3]), 3);
    }
}
