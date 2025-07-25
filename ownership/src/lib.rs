pub fn first_subword(mut s: String) -> String {
    let mut res= String::new();
    for c in s.chars().enumerate(){
        if c.0 != 0 &&(c.1.is_uppercase() || c.1 == '_'){
            break;
        } else{
            res.push_str(&c.1.to_string());
        }
    }
    res
}