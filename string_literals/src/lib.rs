pub fn is_empty(v: &str) -> bool {
    if v.len()==0{return true;} else{ return false;}
    // v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars(){
        if (c as u32) > 127{
            return false;
        }
    }
    return true;
    // v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    for i in 0..v.len()-pat.len(){
        if &v[i..i+pat.len()] == pat{
            return true;
        }
    }
    return false;
    // v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[0..index], &v[index..v.len()])
    // v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    let mut i:usize=0;
    for c in v.chars(){
        if c==pat{
            return i;
        }
        i+=1;
    }
    0
    // v.find(pat).expect("walo")
}