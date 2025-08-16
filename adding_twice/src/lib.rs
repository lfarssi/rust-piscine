pub fn twice<T : Fn(i32)-> i32 >(f : T) ->impl Fn(i32)->i32 {
    move |a| f(f(a))
}

pub fn add_curry(n :i32)->impl Fn(i32)->i32  {
     move |a| a+n 
}
