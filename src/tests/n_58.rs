#[cfg(test)]
mod tests {
    use crate::problems::n_58::length_of_last_word;

    #[test]
    fn test_basic_example() {
        assert_eq!(length_of_last_word("Hello World"), 5);
    }

    #[test]
    fn test_single_word() {
        assert_eq!(length_of_last_word("Hello"), 5);
    }

    #[test]
    fn test_trailing_spaces() {
        assert_eq!(length_of_last_word("Hello World   "), 5);
    }

    #[test]
    fn test_multiple_spaces_between_words() {
        assert_eq!(length_of_last_word("a  b   c"), 1);
    }

    #[test]
    fn test_all_spaces_except_one_word() {
        assert_eq!(length_of_last_word("   word   "), 4);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_last_word(""), 0); // Edge: empty input
    }

    #[test]
    fn test_only_spaces() {
        assert_eq!(length_of_last_word("     "), 0); // Edge: only spaces
    }

    #[test]
    fn test_long_word() {
        assert_eq!(length_of_last_word("a verylongword"), 12);
    }

    #[test]
    fn test_word_with_leading_spaces() {
        assert_eq!(length_of_last_word("   abc"), 3);
    }
}
