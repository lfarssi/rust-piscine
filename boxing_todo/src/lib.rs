// lib.rs
mod err;

use std::{error::Error, fs};
use json::JsonValue;
use crate::err::{ParseErr, ReadErr};

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
    /// Tries to read and parse the todo list from a file
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // First, attempt to read the file
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>)?;

        // Attempt to parse the JSON content
        let json: JsonValue = json::parse(&content).map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        // Check if there are tasks in the JSON and return an error if there are none
        let tasks = json["tasks"].members().collect::<Vec<_>>();
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        // Deserialize tasks
        let tasks: Vec<Task> = tasks.iter().filter_map(|task| {
            if let (Some(id), Some(description), Some(level)) = (
                task["id"].as_u32(),
                task["description"].as_str(),
                task["level"].as_u32(),
            ) {
                Some(Task {
                    id,
                    description: description.to_string(),
                    level,
                })
            } else {
                None
            }
        }).collect();

        // Extract the title from JSON
        let title = json["title"].as_str().unwrap_or("Untitled").to_string();

        // Return the parsed TodoList
        Ok(TodoList { title, tasks })
    }
}
