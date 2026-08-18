use std::{fs, path, time::SystemTime};
pub struct Todo {
    filename: String,
    path: String,
    items: Vec<String>,
    created_date: SystemTime,
    last_modified_date:  SystemTime
}

impl Todo {
    pub fn new() {
        let path = String::from("test.txt");
        let contents = fs::read_to_string(path).expect("Should have been able to read file");
        println!("File text:\n{contents}");
    }

    pub fn list(&self) {}
}