#[cfg(test)]
mod tests {
    use crate::problems::n_3::length_of_longest_substring;

    #[test]
    fn test_example_1_abcabcbb() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    }

    #[test]
    fn test_example_2_bbbbb() {
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    }

    #[test]
    fn test_example_3_pwwkew() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_longest_substring(String::from("")), 0);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(length_of_longest_substring(String::from("a")), 1);
    }

    #[test]
    fn test_two_unique_characters() {
        assert_eq!(length_of_longest_substring(String::from("au")), 2);
    }

    #[test]
    fn test_dvdf() {
        assert_eq!(length_of_longest_substring(String::from("dvdf")), 3);
    }

    #[test]
    fn test_ohomm() {
        assert_eq!(length_of_longest_substring(String::from("ohomm")), 3);
    }

    #[test]
    fn test_tmmzuxt() {
        assert_eq!(length_of_longest_substring(String::from("tmmzuxt")), 5);
    }

    #[test]
    fn test_all_unique_characters() {
        assert_eq!(length_of_longest_substring(String::from("abcdefg")), 7);
    }

    #[test]
    fn test_repeated_characters_in_middle() {
        assert_eq!(length_of_longest_substring(String::from("abcbacde")), 5);
    }
}
