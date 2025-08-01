mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(
                ReadErr {
                    child_err: Box::new(e),
                }
            ))
        };
        

        let parse = json::parse(&content).map_err(|error| ParseErr::Malformed(Box::new(error)))?;
        if parse["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty))
        }

        Ok(
            Self {
                title: parse["title"].as_str().unwrap().to_owned(),
                tasks: parse["tasks"]
                    .members()
                    .map(|m| Task {
                        id: m["id"].as_u32().unwrap(),
                        description: m["description"].as_str().unwrap().to_owned(),
                        level: m["level"].as_u32().unwrap(),
                    })
                    .collect(),
            }
        )
    }
}