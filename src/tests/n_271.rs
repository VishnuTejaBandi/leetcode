#[cfg(test)]
mod tests {
    use crate::problems::n_271::{decode, encode};

    #[test]
    fn test_basic() {
        let strs = vec!["lint", "code", "love", "you"];
        let strs: Vec<String> = strs.into_iter().map(|s| s.to_string()).collect();
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
    #[test]
    fn test_empty_strings() {
        let strs = vec!["", "", ""];
        let strs: Vec<String> = strs.into_iter().map(|s| s.to_string()).collect();
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
    #[test]
    fn test_special_characters() {
        let strs = vec!["a|b", "c:d", "e\nf"];
        let strs: Vec<String> = strs.into_iter().map(|s| s.to_string()).collect();
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
    #[test]
    fn test_single_string() {
        let strs = vec!["one".to_string()];
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
    #[test]
    fn test_empty_vec() {
        let strs: Vec<String> = vec![];
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
    #[test]
    fn test_long_strings() {
        let s = "a".repeat(10000);
        let strs = vec![s.clone(), s.clone()];
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
    #[test]
    fn test_unicode() {
        let strs = vec!["你好", "🌟", "résumé", " 0"];
        let strs: Vec<String> = strs.into_iter().map(|s| s.to_string()).collect();
        let encoded = encode(strs.clone());
        let decoded = decode(encoded);
        assert_eq!(decoded, strs);
    }
}
