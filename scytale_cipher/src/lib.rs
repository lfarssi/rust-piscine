pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty(){
        return message.to_string();
    }
    let t9simat =(message.len()+i-1)/i;
    let mut table:Vec<Vec<char>> =vec![vec![' ';i];t9simat];
    let mut res = String::new();
    let chars:Vec<char>= message.chars().collect();
    let mut index = 0;
    for col in 0..t9simat {
        for row in 0..i {
            if index < chars.len() {
                table[col][row] = chars[index];
                index += 1;
            }
        }
    }
    // println!("{:?}",table);
    for col in 0..i{
        for row in 0..t9simat{
            res.push(table[row][col]);
        }
    }
    res.trim().to_string()
    
}