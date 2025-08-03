pub fn num_to_ordinal(x: u32) -> String {
    if x%100 >=11 && x%100<=13{
        x.to_string()+"th"
    } else{
        match x%10{
            1 =>x.to_string()+"st",
            2 =>x.to_string()+"nd",
            3 =>x.to_string()+"rd",
            _ =>x.to_string()+"th",
        }
    }
}