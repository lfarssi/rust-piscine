 use std::collections::HashSet;
pub fn is_pangram(s: &str) -> bool {
    let mut map = HashSet::new();
    for c in s.to_lowercase().chars(){
        if c.is_ascii_alphabetic(){
        map.insert(c);
        }
    }
    // println!("{}",map.len());
    map.len()==26
}