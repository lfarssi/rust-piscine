use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected:String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {


      let mut map = HashMap::new();
    let mut ciphering=String::new();
    for (a, z) in ('a'..='z').zip(('a'..='z').rev()) {
        map.insert(a, z);
    }


    for (a, z) in ('A'..='Z').zip(('A'..='Z').rev()) {
        map.insert(a, z);
    }

    for c in original.chars(){


        if let Some(v) = map.get(&c) {
            ciphering.push(*v);
        }else {

            ciphering.push(c);
        }
    }

    if ciphering == ciphered {
        return Ok(());
    } else {
        Err(CipherError{expected:ciphering.to_string()})
    }
}