pub fn talking(text: &str) -> &str {
    let trimmed= text.trim();
    if trimmed.is_empty(){
        "Just say something!"
    } else{
        match trimmed.chars().last(){
            Some('!') if trimmed== trimmed.to_uppercase()=>"There is no need to yell, calm down!",
            Some('!')=>"Interesting",
            Some('?')=>{
                if trimmed==trimmed.to_uppercase(){
                   "Quiet, I am thinking!"
                } else{
                    "Sure."
                }
            },
            _=>"Interesting",
        }
    }
}