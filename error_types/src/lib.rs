// this will be the structure that wil handle the errors
use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    form_values:(String,String),
    date:String,
    err:String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values:(field_name.to_string(),field_value),
            date:Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err:err.to_string(),
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
            Err(FormError::new("name",self.name.clone(),"User is empty".to_string()))
        }else if self.password.len()<8{
            Err(FormError::new("password",self.password.clone(),"Password should be at least 8 characters long".to_string()))

        }else if !valid(&self.password)  {
            Err(FormError::new("password",self.password.clone(),"Password should be a combination of ASCII numbers, letters and symbols".to_string()))
            
        }else{
            Ok(())
        }
    }
}
pub fn valid(password :&str)->bool{
    let is_char=password.chars().any(|c| (c>='a'&& c<='z' )||(c>='A'&& c<='Z' ));
    let is_digit=password.chars().any(|c| c>='0'&& c<='9');
    let is_symbol=password.chars().any(|c| !c.is_alphanumeric());
    is_char && is_digit && is_symbol
}
