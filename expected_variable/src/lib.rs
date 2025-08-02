use std::mem;
use convert_case::{Case, Casing};
pub fn expected_variable(origin:&str, expected:&str)->Option<String>{
     if origin.to_lowercase() == origin.to_lowercase().to_case(Case::Camel) || origin.to_lowercase()==origin.to_lowercase().to_case(Case::Snake) {
        
        let changement = edit_distance(&origin.to_lowercase(),&expected.to_lowercase());

        let percentage=((expected.len() as f64 -changement as f64)*100.0)/expected.len() as f64;

        if percentage>50.0{
             Some(format!("{}%", percentage.round()))
        } else {
            None
        }
    } else {
        None
    } 
}




pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    let mut v0 = (0..=n).collect::<Vec<_>>();
    let mut v1 = vec![0; n + 1];

    for i in 0..m {
        v1[0] = i + 1;

        for j in 0..n {
            let deletion_cost = v0[j + 1] + 1;
            let insertion_cost = v1[j] + 1;
            let substitution_cost = v0[j]
                + if source.chars().nth(i) == target.chars().nth(j) {
                    0
                } else {
                    1
                };

            v1[j + 1] = [deletion_cost, insertion_cost, substitution_cost]
                .into_iter()
                .min()
                .unwrap();
        }

        mem::swap(&mut v0, &mut v1);
    }

    v0[n]
}