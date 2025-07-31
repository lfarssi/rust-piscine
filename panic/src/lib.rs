use std::fs:: File;

pub fn open_file(s: &str) -> File {
    match File::open(s){
        Ok(res)=>res,
        Err(e)=>panic!("Failed to open file: {}", e),
    }
}