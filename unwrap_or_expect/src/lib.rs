pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server{
        Ok(res)=>res.to_string(),
        Err(err)=> {
            match security_level {
                Security::Unknown=>panic!("called `Result::unwrap()` on an `Err` value: \"{}\"", err),
                Security::Message=>panic!("ERROR: program stops"),
                Security::Warning=>panic!("WARNING: check the server"),
                Security::NotFound=>format!("Not found: {}",err),
                Security::UnexpectedUrl=>panic!("{}",err)
            }
        }
    }
}