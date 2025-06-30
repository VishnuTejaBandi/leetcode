pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    fn str_hash(item: &str) -> [u8; 26] {
        item.as_bytes().iter().fold([0; 26], |mut acc, char| {
            acc[(char - b'a') as usize] += 1;
            acc
        })
    }

    let mut map = HashMap::new();

    for ele in strs {
        let h = str_hash(&ele);
        map.entry(h).or_insert(Vec::new()).push(ele);
    }

    map.into_values().collect()
}
