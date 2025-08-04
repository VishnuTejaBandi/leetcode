#[cfg(test)]
mod tests {
    use crate::problems::n_42::trap;

    #[test]
    fn test_basic() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_no_trap() {
        assert_eq!(trap(vec![0, 1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_flat() {
        assert_eq!(trap(vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_single_peak() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_small() {
        assert_eq!(trap(vec![2, 0, 2]), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(trap(vec![]), 0);
    }

    #[test]
    fn test_one_bar() {
        assert_eq!(trap(vec![5]), 0);
    }

    #[test]
    fn test_two_bars() {
        assert_eq!(trap(vec![5, 1]), 0);
    }
}
