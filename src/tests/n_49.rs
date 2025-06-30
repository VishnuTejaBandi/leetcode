#[cfg(test)]
mod tests {
    use crate::problems::n_49::group_anagrams;

    fn assert_anagram_groups_eq(mut result: Vec<Vec<String>>, mut expected: Vec<Vec<String>>) {
        // Sort each group internally
        for group in &mut result {
            group.sort();
        }
        for group in &mut expected {
            group.sort();
        }

        // Sort the groups by their first element for consistent comparison
        result.sort_by(|a, b| a[0].cmp(&b[0]));
        expected.sort_by(|a, b| a[0].cmp(&b[0]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_1() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["bat".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_example_2() {
        let input = vec!["".to_string()];
        let result = group_anagrams(input);
        let expected = vec![vec!["".to_string()]];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_example_3() {
        let input = vec!["a".to_string()];
        let result = group_anagrams(input);
        let expected = vec![vec!["a".to_string()]];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_all_same_anagrams() {
        let input = vec![
            "abc".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "acb".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![vec![
            "abc".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "acb".to_string(),
        ]];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_no_anagrams() {
        let input = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];

        let result = group_anagrams(input);
        let expected = vec![
            vec!["abc".to_string()],
            vec!["def".to_string()],
            vec!["ghi".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_duplicate_strings() {
        let input = vec!["abc".to_string(), "abc".to_string(), "bca".to_string()];

        let result = group_anagrams(input);
        let expected = vec![vec![
            "abc".to_string(),
            "abc".to_string(),
            "bca".to_string(),
        ]];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_single_character_anagrams() {
        let input = vec![
            "a".to_string(),
            "b".to_string(),
            "a".to_string(),
            "c".to_string(),
            "b".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![
            vec!["a".to_string(), "a".to_string()],
            vec!["b".to_string(), "b".to_string()],
            vec!["c".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_empty_strings() {
        let input = vec!["".to_string(), "".to_string(), "a".to_string()];

        let result = group_anagrams(input);
        let expected = vec![vec!["".to_string(), "".to_string()], vec!["a".to_string()]];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_long_anagrams() {
        let input = vec![
            "abcdefghijklmnop".to_string(),
            "ponmlkjihgfedcba".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![
            vec![
                "abcdefghijklmnop".to_string(),
                "ponmlkjihgfedcba".to_string(),
            ],
            vec!["hello".to_string()],
            vec!["world".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_repeated_characters() {
        let input = vec![
            "aab".to_string(),
            "aba".to_string(),
            "baa".to_string(),
            "aaa".to_string(),
            "bbb".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![
            vec!["aab".to_string(), "aba".to_string(), "baa".to_string()],
            vec!["aaa".to_string()],
            vec!["bbb".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_case_sensitivity() {
        // Note: LeetCode 49 specifies lowercase only, but this tests the assumption
        let input = vec!["abc".to_string(), "bca".to_string(), "xyz".to_string()];

        let result = group_anagrams(input);
        let expected = vec![
            vec!["abc".to_string(), "bca".to_string()],
            vec!["xyz".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_maximum_constraints() {
        // Test with larger input (within LeetCode constraints)
        let mut input = Vec::new();

        // Add multiple anagram groups
        for _ in 0..100 {
            input.push("abc".to_string());
            input.push("bca".to_string());
            input.push("cab".to_string());
        }

        let result = group_anagrams(input);
        assert_eq!(result.len(), 1); // All should be in one group
        assert_eq!(result[0].len(), 300); // 100 * 3 strings
    }

    #[test]
    fn test_edge_case_all_empty() {
        let input = vec!["".to_string(), "".to_string(), "".to_string()];

        let result = group_anagrams(input);
        let expected = vec![vec!["".to_string(), "".to_string(), "".to_string()]];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_palindromic_anagrams() {
        let input = vec![
            "racecar".to_string(),
            "carrace".to_string(),
            "carecar".to_string(),
            "hello".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![
            vec![
                "racecar".to_string(),
                "carrace".to_string(),
                "carecar".to_string(),
            ],
            vec!["hello".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }

    #[test]
    fn test_mixed_lengths() {
        let input = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "a".to_string(),
            "aa".to_string(),
        ];

        let result = group_anagrams(input);
        let expected = vec![
            vec!["a".to_string(), "a".to_string()],
            vec!["aa".to_string(), "aa".to_string()],
            vec!["aaa".to_string()],
        ];

        assert_anagram_groups_eq(result, expected);
    }
}
