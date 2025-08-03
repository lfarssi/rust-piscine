pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    for c in input.chars(){
        if c.is_ascii_lowercase(){
            res.push((((c as i32 - 'a' as i32 +key as i32 +26)%26 + 'a' as i32) as u8) as char);
        } else if c.is_ascii_uppercase(){
            res.push((((c as i32 - 'A' as i32 +key as i32 +26)%26 + 'A' as i32) as u8) as char);
        } else  {
            res.push(c)
        }
    }
    res
}