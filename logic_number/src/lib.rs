pub fn number_logic(num: u32) -> bool {
    let mut num2=0;
    let st = num.to_string();

    for i in st.chars(){
        let n = i.to_digit(10).expect("");
        num2 +=n.pow(st.len().try_into().unwrap());
    }
    num == num2.try_into().unwrap()
}