use crate::problems::n_217;

#[test]
pub fn test_contains_duplicate() {
    assert_eq!(n_217::contains_duplicate(vec![1, 2, 23, 4]), false);
    assert_eq!(n_217::contains_duplicate(vec![1, 2, 2, 4]), true);
}
