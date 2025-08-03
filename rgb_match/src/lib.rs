use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let mut table = HashMap::new();
        let arr= vec![
            ("r",self.r),
            ("g",self.g),
            ("b",self.b),
            ("a",self.a)
        ];
        for (k,v) in arr.iter(){
            if v==&first{
                table.insert(k,second);
            } else if v== &second{
                table.insert(k,first);
            } else {
                table.insert(k,*v);
            }
        };
        Color {
             r: *table.get(&"r").unwrap(),
              g: *table.get(&"g").unwrap(),
               b: *table.get(&"b").unwrap(), 
               a: *table.get(&"a").unwrap() 
            }
    }
}