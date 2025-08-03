pub fn score(s : &str)->u64{
    let mut sum=0;
    for c in s.to_lowercase().chars(){
        match c {
            'q' | 'z' => sum+=10,
            'j' | 'x' => sum+=8,
            'k' => sum+=5,
            'f' | 'h' | 'v' |'w' |'y' => sum+=4,
            'c' | 'm' | 'p' | 'b' => sum+=3,
            'g' | 'd'  => sum+=2,
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' |'t' => sum+=1,
            _ =>(),
        }
    }
    sum
}