
use std::fs;
use std::time::SystemTime;
use std::io::{self, Write, BufWriter};

use chrono::Local;

struct TodoItem {
    text: String,
    is_done: bool,
    created_at: String,
    completed_at: Option<String>,
}

impl TodoItem {
    fn create_item(line: String) -> Result<Self, String> {
        let split_line: Vec<&str> = line.splitn(4, "|").collect();

        let completed_at = if split_line[2].is_empty() {
            None
        } else {
            Some(split_line[2].to_string())
        };

        Ok(TodoItem {
            is_done: split_line[0]
                .parse::<bool>()
                .map_err(|err| err.to_string())?,
            created_at: split_line[1].to_string(),
            completed_at,
            text: split_line[3].to_string(),
        })
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
            let item = TodoItem::create_item(line).expect("Error trying to create todo item line");
            todo_items.push(item);
        }

        let metadata = fs::metadata(&path)?;

        Ok(Todo { 
            items: todo_items,
            created_at: metadata.created()?,
            last_modified_at: metadata.modified()?
        })
    }

    pub fn list(&self) -> Result<(), std::io::Error>{
        let stdout = io::stdout();

        // Buffered write for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();

        for (index, item) in self.items.iter().enumerate() {
            let index = index + 1;

            data.push_str(&index.to_string());
            data.push_str(". ");
            
            // match &item.is_done {
            //     true => data.push_str("[x] "),
            //     false => data.push_str("[ ] "),
            // }

            if item.is_done {
                data.push_str("[x] ");
            } else {
                data.push_str("[ ] ");
            }

            data.push_str(&item.text);
            data.push_str("\nCriado em: ");
            data.push_str(&item.created_at);

            // match &item.completed_at {
            //     Some(completed) => {
            //         data.push_str("\nCompletado em: ");
            //         data.push_str(completed);
            //     }
            //     None => {}
            // }

            if let Some(completed) = &item.completed_at {
                data.push_str("\nCompletado em: ");
                data.push_str(completed);
            };

            data.push_str("\n\n");
        }

        writer.write_all(data.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn add_items(&self, args: &[String]) {
        let items = format_input_data(args);
        let mut lines: Vec<String> = Vec::new();
        let time = Local::now().format("%Y-%m-%d %H:%M").to_string();

        for item in items {
            let line = format!("{}|{:?}||{}", false, time, item);
            lines.push(line);
        }
    }
}

fn format_input_data(args: &[String]) -> Vec<String> {
    args.join(" ")
        .split(",")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
}