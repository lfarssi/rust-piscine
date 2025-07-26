pub fn arrange_phrase(phrase: &str) -> String {
    let words =phrase.split_whitespace();
    let mut arr_tup: Vec<(String,String)>=vec![];
    for word in words{
        let mut klma= String::new();
        let mut nbr=' ';
        for ch in word.chars(){
            if ch.is_digit(10){
                nbr=ch;
            }else{
                klma.push(ch);
            }
        }
        arr_tup.push((klma, nbr.to_string()))
    }
    // println!("{:?}",arr_tup);
    sort(arr_tup)
}
fn sort(mut arr: Vec<(String,String)>)->String{
    for i in 0..arr.len(){
        for j in 0..arr.len()-1-i{
            let a :i32= arr[j].1.parse().unwrap();
            let b :i32 =arr[j+1].1.parse().unwrap();
            if a>b{
                arr.swap(j,j+1);
            }
        }
    }

    arr.into_iter().map(|(mot, _)| mot).collect::<Vec<String>>().join(" ")
}