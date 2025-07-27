use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.iter().max_by(|a, b| a.1.cmp(b.1)).map_or(0, |(_, &v)| v)
}