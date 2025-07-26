pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('O', table) || vertical('O', table)||diagonals('O', table){
        "Player O win".to_string()

    } else if vertical('X',table) || horizontal('X', table) || diagonals('X', table){
        "Player X win".to_string()

    } else{
        "tie".to_string()
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
//    let left_to_right = table[0][0]== player && table[1][1]==player && table[2][2]==player;
//    let right_to_left =table[0][2]==player && table[1][1]==player && table[2][0]==player;
//     if left_to_right || right_to_left{
//         return true;
//     }
//     return false;
    let mut i=0;
    let mut c=0;
    for _ in table.iter(){
        if table[i][i]==player || table[i][2-i]== player{
            c+=1
        }
        if c==3{
            return true;
        }
        i+=1;
    }
    return false;
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table.iter(){
        if row.iter().all(|&col| col==player){
            return true
        }
    }
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    // let mut i = 0;
    for col in 0..3{
        // let mut c=0;
       if table[0][col]== player && table[1][col]==player && table[2][col]==player{
        return true;
       }
    }
    return false;
     
}