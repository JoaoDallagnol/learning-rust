use std::{fs, time::SystemTime};

struct TodoItem {
    text: String,
    is_done: bool,
    created_at: SystemTime,
    completed_at: Option<SystemTime>,
}

impl TodoItem {
    fn create_item(line: String) -> Self {
        TodoItem { text: line, is_done: false, created_at: SystemTime::now(), completed_at: Some(SystemTime::now())}
    }
}

pub struct Todo {
    items: Vec<TodoItem>,
    created_at: SystemTime,
    last_modified_at: SystemTime,
}

impl Todo {
    pub fn load_file() -> Result<Self, std::io::Error> {
        let path = String::from("test.txt");
        let contents = fs::read_to_string(&path)?;
        let text: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

        let mut todo_items: Vec<TodoItem> = Vec::new();
        for line in text {
            todo_items.push(TodoItem::create_item(line));
        }

        let metadata = fs::metadata(&path)?;

        Ok(Todo { 
            items: todo_items,
            created_at: metadata.created()?,
            last_modified_at: metadata.modified()?
        })
    }

    pub fn list(&self) {}
}