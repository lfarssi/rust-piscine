pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::new();
    if arr.len()==1 {
        return res;
    }
    for  item in arr.iter() {
        let mut sum = 1 ;
        for ele  in arr.iter(){
            if ele != item {
                sum *= ele;
            }
        }
        res.push(sum);
    }
    res
}