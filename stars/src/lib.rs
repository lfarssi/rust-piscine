pub fn stars(n: u32) -> String {
       let stars= 2i32.pow(n);
       "*".repeat(stars.try_into().unwrap())

}