#[allow(dead_code)]
use std::{collections::HashMap, num::ParseFloatError};
#[derive(Debug)]
pub struct Flag {
    short_hand:String,
    long_hand:String,
    desc:String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self{
            short_hand: format!("-{}",name.chars().nth(0).unwrap()),
            long_hand: format!("--{}",name),
            desc:d.to_string(),
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
        for ((s, l),callback) in &self.flags{
            if s== input || l== input{
                return callback(&argv[0],&argv[1]).map_err(|e| e.to_string());
            }
        }
        Err("no flag found".into())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<f64>()?;
    let d = b.parse::<f64>()?;
   if d == 0.0 {
        Err("Division by zero".parse::<f64>().unwrap_err()) 
    } else {
        Ok((c / d).to_string())
    }
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<f64>()?;
    let d = b.parse::<f64>()?;
     if d == 0.0 {
        Err("Division by zero".parse::<f64>().unwrap_err()) 
    } else {
        Ok((c % d).to_string())
    }
}