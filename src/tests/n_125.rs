#[cfg(test)]
mod tests {
    use crate::problems::n_125::is_palindrome;

    #[test]
    fn test_basic_true() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }
    #[test]
    fn test_basic_false() {
        assert!(!is_palindrome("race a car".to_string()));
    }
    #[test]
    fn test_empty() {
        assert!(is_palindrome("".to_string()));
    }
    #[test]
    fn test_single_char() {
        assert!(is_palindrome("a".to_string()));
    }
    #[test]
    fn test_only_non_alphanumeric() {
        assert!(is_palindrome(".,,".to_string()));
    }
    #[test]
    fn test_mixed_case() {
        assert!(is_palindrome("Able was I ere I saw Elba".to_string()));
    }
    #[test]
    fn test_numbers() {
        assert!(is_palindrome("12321".to_string()));
        assert!(!is_palindrome("12345".to_string()));
    }
    #[test]
    fn test_unicode() {
        assert!(is_palindrome("あいいあ".to_string()));
    }

}
