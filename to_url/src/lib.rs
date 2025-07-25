pub fn to_url(s: &str) -> String {
    s.to_owned().replace(" ","%20")
}