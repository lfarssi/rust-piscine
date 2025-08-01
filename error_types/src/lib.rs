// this will be the structure that wil handle the errors
pub use chrono::*;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl  FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values:(field_name,field_value),
            date:Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err:err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name:String,
    pub password:String,
}

impl Form {
    
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty(){
            Err(FormError::new("name",self.name.clone(),"Username is empty"))
        }else if self.password.len()<8{
            Err(FormError::new("password",self.password.clone(),"Password should be at least 8 characters long"))

        }else if !valid(&self.password)  {
            Err(FormError::new("password",self.password.clone(),"Password should be a combination of ASCII numbers, letters and symbols"))
            
        }else{
            Ok(())
        }
    }
}
pub fn valid(password :&str)->bool{
    let is_char=password.chars().any(|c| c.is_ascii_alphabetic());
    let is_digit=password.chars().any(|c| c.is_ascii_digit());
    let is_symbol=password.chars().any(|c| !c.is_ascii_alphanumeric());
    is_char && is_digit && is_symbol
}
