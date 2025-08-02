use ::edit_distance::edit_distance;
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


