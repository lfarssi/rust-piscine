pub fn delete_prefix<'a>(prefix: &'a str,mut s: &'a str) -> Option<&'a str> {
    if !s.contains(prefix){
        return None;
    }
    if s.starts_with(prefix){
        s=s.strip_prefix(prefix)?;
    }
    Some(s)
}