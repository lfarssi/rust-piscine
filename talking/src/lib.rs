pub fn talking(text: &str) -> &str {
    if text.len()==0{
        "Just say something!"
    } else{
        match text.chars().last(){
            Some('!')=>"There is no need to yell, calm down!",
            Some('?')=>{
                if text==text.to_uppercase(){
                   "Quiet, I am thinking!"
                } else{
                    "Sure."
                }
            },
            _=>"Interesting",
        }
    }
}