pub fn delete_and_backspace(s: &mut String) {
  
    while s.contains('-') || s.contains('+'){
        let mut i =0;
        while i< s.len(){
        let c= s.chars().nth(i);
        if c == Some('-') {
            if i>0 && s.chars().nth(i-1)!=Some('-'){

                s.remove(i);
                s.remove(i-1);
            i-=1;
            }
                }
        if c== Some('+'){
            if i<s.len()-1 && s.chars().nth(i+1)!=Some('+'){
                s.remove(i+1);
                s.remove(i); 
                                
            } 
        }
        i += 1;
            
    }
    }
}

pub fn do_operations(v: &mut [String]) {
    for c in v.iter_mut(){
        if let Some(index) = c.find('+'){
            let first = &c[..index];
            let second = &c[index+1..];
            if let(Ok(a), Ok(b)) = (first.trim().parse::<i32>(), second.trim().parse::<i32>()){
                *c= (a+b).to_string();
            }
        } else  if let Some(index) = c.find('-'){
            let first = &c[..index];
            let second = &c[index+1..];
            if let(Ok(a), Ok(b)) = (first.trim().parse::<i32>(), second.trim().parse::<i32>()){
                *c= (a-b).to_string();
            }
        }
    }
    
}
