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
    let src_ln = source.len();
    let trg_ln = target.len();
    let mut dp = vec![vec![0;trg_ln+1];src_ln+1];
    for i in 0..=src_ln{
        dp[i][0]=i;
    }
    for j in 0..=trg_ln{
        dp[0][j]=j;
    }
    for i in 1..=src_ln{
        for j in 1..=trg_ln{
            dp[i][j]=*[dp[i-1][j]+1, dp[i][j-1]+1,dp[i-1][j-1]+if source.chars().nth(i-1)==target.chars().nth(j-1){0}else{1}].iter().min().unwrap();
        }
    }
    // println!("{:?}",dp);
    dp[src_ln][trg_ln]
}