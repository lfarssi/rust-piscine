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