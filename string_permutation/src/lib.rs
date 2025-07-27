use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() !=  s2.len(){
        return false;
    }
    let  mut count = HashMap::new();
    for chars in s1.chars(){
        *count.entry(chars).or_insert(0)+=1;
    }
    for chars in s2.chars(){
        match count.get_mut(&chars){
            Some(c)=>{
                if *c ==0{
                    return false;
                }
                *c-=1;
            },
            None => return false,
        }
    }
    return true;
}