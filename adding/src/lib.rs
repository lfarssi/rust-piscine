pub fn add_curry(n :i32)->impl Fn(i32)->i32  {
     move |a| a+n 
}
