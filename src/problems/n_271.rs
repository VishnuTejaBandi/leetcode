// LeetCode 271: Encode and Decode Strings
//
// Design an algorithm to encode a list of strings to a single string. The encoded string is then sent over the network and is decoded back to the original list of strings.
//
// Implement the encode and decode methods:
// - encode(List<String> strs): Encodes a list of strings to a single string.
// - decode(String s): Decodes a single string to a list of strings.
//
// Note: The string may contain any possible characters, including empty strings and special characters. You should not rely on any delimiter being unused in the input.
//
// Example:
// Input: ["lint","code","love","you"]
// Encoded: "4#lint4#code4#love3#you"
// Decoded: ["lint","code","love","you"]

pub fn encode(strs: Vec<String>) -> String {
    strs.iter()
        .map(|s| format!("{}#{}", s.len(), s))
        .collect::<String>()
}

pub fn decode(s: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < s.len() {
        let hash_idx = s[pos..].find('#').unwrap() + pos;
        let str_len: usize = s[pos..hash_idx].parse().unwrap();
        let str_start = hash_idx + 1;
        let str_end = str_start + str_len;
        result.push(s[str_start..str_end].to_string());
        pos = str_end;
    }
    result
}
