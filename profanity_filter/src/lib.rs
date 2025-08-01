// Empty file
pub fn check_ms(message: &str) -> Result<&str, &str> {
  if message.is_empty(){
    return Err("ERROR: illegal");
  }
  if message.to_lowercase().contains("stupid"){
    return Err("ERROR: illegal");
  }
   Ok(message)
}