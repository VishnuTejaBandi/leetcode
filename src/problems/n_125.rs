pub fn is_palindrome(s: String) -> bool {
    let mut chars = s.chars().filter(|c| c.is_alphanumeric());

    while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
        if !c1.eq_ignore_ascii_case(&c2) {
            return false;
        }
    }
    true
}
