pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next(){
        Some(first)=>first.to_uppercase().collect::<String>() + &chars.collect::<String>(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut in_whitespace = false;

    for (i, c) in input.char_indices() {
        if c.is_whitespace() {
            result.push(c);
            in_whitespace = true;
        } else {
            if in_whitespace || i == 0 {
                result.push(c.to_uppercase().next().unwrap());
            } else {
                result.push(c);
            }
            in_whitespace = false;
        }
    }

    result
}


pub fn change_case(input: &str) -> String {
    input.chars().map(|c|{
        if c.is_uppercase(){
            c.to_lowercase().next().unwrap()
        } else if c.is_lowercase(){
            c.to_uppercase().next().unwrap()
        }else{
            c
        }
    }).collect::<String>()
}