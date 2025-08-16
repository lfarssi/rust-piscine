pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res = Vec::new();
    let check = |a: i32, res: &mut Vec<i32>| {
        res.push(a.pow(2));
    };
    let mut i=2;
    while res.len()<50 {
        if i%2==0{
            check(i,&mut res);
        }
        i+=1;
    }
    res
}