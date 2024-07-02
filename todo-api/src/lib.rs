use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
};

fn get_save_dir() -> Result<String, Error> {
    Ok(format!(
        "{}{}",
        dirs::cache_dir()
            .ok_or(ErrorKind::NotFound)?
            .into_os_string()
            .into_string()
            .expect("Invalid Cache Directory Name"),
        "/todo_list.rs/save.json"
    ))
}

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
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        TodoList {
            items: vec![],
            name: name.to_string(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.items.iter().all(|a| a.state == TodoState::Done)
    }

    pub fn add(&mut self, name: &str) {
        self.items.push(TodoItem::new(name));
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
}

struct TodoListList {
    pub todolists: Vec<TodoList>,
}

impl TodoListList {
    pub fn new() -> Self {
        let mut todolistlist = TodoListList { todolists: vec![] };

        todolistlist.add("Todo");
        todolistlist
    }

    pub fn save(&self) {
        let contents = serde_json::to_string(&self.todolists).unwrap();
        let _ = fs::write(
            get_save_dir().expect("Could Not Find Cache Directory."),
            contents,
        );
    }

    pub fn load() -> Self {
        match fs::metadata(get_save_dir().expect("Could not find save.json")) {
            Ok(_) => println!("Save file found"),
            Err(_) => {
                println!("Save file not found, creating new");
                let todolistlist = TodoListList::new();
                todolistlist.save();
            }
        }

        let todolists: Vec<TodoList> = serde_json::from_str(
            &fs::read_to_string(get_save_dir().expect("Could Not Get Save Directory"))
                .expect("Could not read save.json"),
        )
        .expect("Could not parse json from save.json");

        TodoListList { todolists }
    }

    pub fn all_finished(&self) -> bool {
        self.todolists.iter().all(|a| a.is_finished())
    }

    pub fn add(&mut self, name: &str) {
        self.todolists.push(TodoList::new(name));
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
        assert!(test_list.is_finished())
    }
}
