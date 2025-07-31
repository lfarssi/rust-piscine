use std::path::Path;
use std::io::Write;
 use std::fs::OpenOptions;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file= OpenOptions::new().read(true).create(true).append(true).open(path);
    let _=match file{
        Ok(mut res)=>write!(res, "{}",content),
        Err(e)=> panic!("Failed to open or create the file: {}",e),
 
    };
}