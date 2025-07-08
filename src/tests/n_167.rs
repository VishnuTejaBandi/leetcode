#[cfg(test)]
mod tests {
    use crate::problems::n_167::two_sum;

    #[test]
    fn test_basic() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![1,2]);
    }
    #[test]
    fn test_negative_numbers() {
        assert_eq!(two_sum(vec![-3,0,3,4], 0), vec![1,3]);
    }
    #[test]
    fn test_duplicates() {
        assert_eq!(two_sum(vec![1,1,3,4], 2), vec![1,2]);
    }
    #[test]
    fn test_large_numbers() {
        assert_eq!(two_sum(vec![1,2,3,4,4,9,56,90], 8), vec![4,5]);
    }
    #[test]
    fn test_minimal_case() {
        assert_eq!(two_sum(vec![1,2], 3), vec![1,2]);
    }
    #[test]
    fn test_no_solution() {
        assert_eq!(two_sum(vec![1,2,3], 7), Vec::<i32>::new());
    }
}

