use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}
pub type Link = Option<Box<Worker>>;
#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        });
        self.grade = Some(worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            let name = worker.name;
            self.grade = worker.next;
            return Some(name);
        }
        None
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        
        if let Some(mut head) = self.grade.as_ref() {
            while !head.next.is_none() {
                
                head= head.next.as_ref().unwrap();
                
            }
            
            return Some((head.name.clone(),head.role.clone()));
        }

        None
    }
}


