pub fn pig_latin(text: &str) -> String {
    if text.is_empty(){
        return text.to_string();
    }else if is_vowel(text.chars().nth(0).unwrap()){
        return text.to_string()+"ay";
    } else{
        let mut consonant=String::new();
        let chars = text.chars().collect::<Vec<char>>();

        let mut index=0;
        for (i,c) in text.chars().enumerate(){
            if !is_vowel(c){
                consonant+=&c.to_string();
                index=i+1;
            } else if chars[i-1] == 'q' && chars[i] =='u' && i!=1{
                consonant+=&c.to_string();
                index=i+1;
            }else{
                return text[index..].to_string() + &consonant + "ay";
            }
        }
        text.to_string()
    }
    
        
}
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}