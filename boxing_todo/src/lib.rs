mod err;

use err::{ParseErr, ReadErr};
use std::{error::Error, fs};

#[derive(Debug, Eq, PartialEq)]
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
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        let parsed = json::parse(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        let title = parsed["title"]
            .as_str()
            .ok_or_else(|| Box::new(ParseErr::Malformed("Missing title".to_string().into())))?
            .to_string();

        let tasks_json = &parsed["tasks"];
        if !tasks_json.is_array() {
            return Err(Box::new(ParseErr::Malformed("Tasks not array".to_string().into())));
        }

        let mut tasks = Vec::new();
        for task in tasks_json.members() {
            let id = task["id"]
                .as_u32()
                .ok_or_else(|| Box::new(ParseErr::Malformed("Invalid id".to_string().into())))?;
            let description = task["description"]
                .as_str()
                .ok_or_else(|| Box::new(ParseErr::Malformed("Invalid description".to_string().into())))?
                .to_string();
            let level = task["level"]
                .as_u32()
                .ok_or_else(|| Box::new(ParseErr::Malformed("Invalid level".to_string().into())))?;

            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList { title, tasks })
    }
}