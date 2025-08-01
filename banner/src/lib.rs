#[warn(dead_code)]
use std::{collections::HashMap, num::ParseFloatError};
#[derive(Debug)]
pub struct Flag {
    short_hand: String, 
    long_hand: String,  
    desc: String,       
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        // Now we directly store owned Strings instead of references
        Self {
            short_hand: "-".to_owned() + &name[0..1],  
            long_hand: "--".to_owned() + name,         
            desc: d.to_string(),                        
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;
#[derive(Debug)]

pub struct FlagsHandler {
    pub flags: HashMap<(String,String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand,flag.long_hand),func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
   

        for ((s, l), callback) in &self.flags {
            if s == input || l == input {
                return callback(&argv[0], &argv[1]).map_err(|e| e.to_string()); 
            }

        }
        Err("invalid float literal".to_string())
}
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<f64>()?;
    let d = b.parse::<f64>()?;
    
    
    Ok((c / d).to_string())
    
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<f64>()?;
    let d = b.parse::<f64>()?;
    
    
     Ok((c % d).to_string())
    
}