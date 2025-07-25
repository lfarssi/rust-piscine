pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).powi(c), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for i in a.split_whitespace(){
         let u= i.parse::<f64>().expect("walo");
        let power= u.exp();
        res.push_str(&power.to_string());
        res.push_str(" ");
    }
    res.pop();
    (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res = Vec::new();
    for i in b.iter() {
        let e = (i.abs() as f64).ln();
        res.push(e)
    }
    (b, res)
}