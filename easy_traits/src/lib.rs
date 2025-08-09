#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append:String)->Self{
        self.value.push_str(&str_to_append);
        Self {
            value:self.value.clone(),
        }
    }
    fn append_number(&mut self, nb_to_append:f64)-> Self{
        self.value.push_str(&nb_to_append.to_string());
        Self {
            value:self.value.clone(),
        }
    }
    fn remove_punctuation_marks(&mut self)->Self{
        self.value =self.value
        .chars().filter(|c| *c== '-' || !c.is_ascii_punctuation() ).collect::<String>();
        Self{
            value:self.value.clone(),
        }
    }
}