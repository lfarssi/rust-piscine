use std::{cell::{ RefCell}, rc::Rc};
pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: usize,
    max: usize,
}
impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: RefCell::new(Vec::new()) ,
            value: 0,
            max,
        }
    }
    pub fn set_value(&self, value: &Rc<usize>) {
        if self.max < Rc::strong_count(value) {
            self.messages
                .borrow_mut().push("Error: You can't go over your quota!".to_string());
        } else {
            let percentage = (Rc::strong_count(value) * 100) /self.max  ;
            if percentage > 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {}% of your quota!",
                    percentage
                ));
            }
        }
    }
    pub fn peek(&self, value: &Rc<usize>) {
        let percentage = ( Rc::strong_count(value) * 100) /self.max;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percentage
        ));
    }
}
