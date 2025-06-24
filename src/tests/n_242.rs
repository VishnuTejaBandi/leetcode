#[cfg(test)]
mod tests {
    use crate::problems::n_242::is_anagram;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            is_anagram("abc".to_string(), "cab".to_string()),
            true
        );

        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );

        assert_eq!(
            is_anagram("hello".to_string(), "world".to_string()),
            false
        );
    }
}
