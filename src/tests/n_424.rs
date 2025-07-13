#[cfg(test)]
mod tests {
    use crate::problems::n_424::character_replacement;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement(("AABA").to_owned(), 1), 4);
        assert_eq!(character_replacement(("CCCCCC").to_owned(), 1), 6);
        assert_eq!(character_replacement(("AACCCCCC").to_owned(), 2), 8);
        assert_eq!(character_replacement(("CACACACAB").to_owned(), 1), 3);
    }
}
