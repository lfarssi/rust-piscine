pub fn num_to_ordinal(x: u32) -> String {
    let y =x%10;
    match y{
        1=>x.to_string()+"st",
        2=>x.to_string()+"nd",
        3=>x.to_string()+"rd",
        _=>x.to_string()+"th",
    }
}