use std::{fs, io::{Error, ErrorKind}};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub enum TodoType {
    Important,
    None,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum TodoState {
    Todo,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub state: TodoState,
    pub name: String,
    pub todo_type: TodoType,
}

impl TodoItem {
    fn new(name: &str) -> Self {
        TodoItem {
            state: TodoState::Todo,
            name: name.to_string(),
            todo_type: TodoType::None,
        }
    }

    pub fn important(&mut self) {
        self.todo_type = TodoType::Important;
    }

    pub fn complete(&mut self) {
        self.state = TodoState::Done;
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub name: String,
    pub finished: bool,
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        TodoList {
            items: vec![],
            name: name.to_string(),
            finished: false,
        }
    }

    pub fn finished(&self) -> bool {
        self.items.iter().all(|a| a.state == TodoState::Done)
    }

    pub fn add(&mut self, name: &str) {
        self.items.push(TodoItem::new(name));
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }

    pub fn save(&self) -> Result<(), Error> {
        let contents = serde_json::to_string(&self)?;
        fs::write(dirs::cache_dir().ok_or(ErrorKind::NotFound)?.into_os_string(), contents);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn finished_test() {
        let mut test_list = TodoList::new("Test Todo");
        test_list.add("Add stuff");
        test_list.add("other stuff");
        test_list.items[0].complete();
        test_list.items[1].complete();
        assert!(test_list.finished())
    }
}
