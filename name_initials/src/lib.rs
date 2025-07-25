pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res= Vec::new();
    for name in names.iter(){
        let  parts:Vec<&str>= name.split_whitespace().collect();
        let first = &parts[0][..1];
        let last = &parts[1][..1];
        res.push(format!("{first}. {last}."));
    }
    res
}