pub fn get_diamond(c: char)->Vec<String> {
    let step= (c as u8-'A' as u8)+1;
    let mut mo3ayan = Vec::new();
    for i in 0..step{
        let mut ster =" ".repeat((step-i-1).into());
        let letter = ('A' as u8 + i as u8) as char;
        if i==0{
            ster.push(letter);
        } else{
            ster.push(letter);
            ster.push_str(&" ".repeat((2*i-1).into()));
            ster.push(letter);
        }
        ster.push_str(&" ".repeat((step-i-1).into()));
        mo3ayan.push(ster);
    }
    for i in (0..step-1).rev(){
        mo3ayan.push(mo3ayan[i as usize].clone());
    }
    // println!("{}",step);
    mo3ayan
}