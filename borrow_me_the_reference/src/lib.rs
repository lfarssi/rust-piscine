pub fn delete_and_backspace(s: &mut String) {
        
        let mut res =String::new();
        let mut count= 0;
        for c in s.chars(){
                if c=='-'{
                res.pop();
                }else if c=='+'{
                    count+=1;
                }else{
                    if count !=0{
                        count-=1;
                        continue;
                    }
                    res.push(c);

                }
        }
        *s=res;
        
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
