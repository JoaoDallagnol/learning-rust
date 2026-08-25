use std::fs::{self, OpenOptions};
use std::num::ParseIntError;
use std::time::SystemTime;
use std::io::{self, Write, BufWriter};

use chrono::{DateTime, Local};

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
    path: String,
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
            path,
            created_at: metadata.created()?,
            last_modified_at: metadata.modified()?
        })
    }

    pub fn list(&self) -> Result<(), std::io::Error>{
        let stdout = io::stdout();

        // Buffered write for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();
        let created_at: DateTime<Local> = self.created_at.into();
        let last_modified_at: DateTime<Local> = self.last_modified_at.into();

        data.push_str("==============================\n");
        data.push_str("Todo file\n");
        data.push_str("------------------------------\n");
        data.push_str("Path: ");
        data.push_str(&self.path);
        data.push_str("\nCreated at: ");
        data.push_str(&created_at.format("%Y-%m-%d %H:%M").to_string());
        data.push_str("\nLast modified: ");
        data.push_str(&last_modified_at.format("%Y-%m-%d %H:%M").to_string());
        data.push_str("\n==============================\n\n");

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
        if !args.is_empty() {
            let lines = format_to_file_lines(args);
            let file  = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
                .expect("Couldn't open the todofile!");

            let mut buffer = BufWriter::new(file);
            buffer.write_all(lines
                .join("")
                .as_bytes())
                .expect("Couldn't write to the file!");
            buffer.flush().expect("Couldn't write to the file!");
        }
    }

    pub fn remove_items(&self, args: &[String]) {
        let input: Result<Vec<usize>, ParseIntError> = args.iter().map(|arg| arg.parse::<usize>()).collect();
        
        match input {
            Ok(positions ) => {

                // Reading the text lines from the file
                // let contents = fs::read_to_string(&self.path).expect("Couldn't open the todofile!");
                // let mut text: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

                // Reading the text lines from the TodoItem struct
                let mut text: Vec<String> = Vec::new();
                for item in &self.items {
                    let completed_at = match &item.completed_at {
                        Some(value) => value.as_str(),
                        None => "",
                    };
                    let line = format!("{}|{}|{}|{}", item.is_done, item.created_at, completed_at, item.text);

                    text.push(line);
                }

                match validate_args(positions, &text) {
                    Ok(indexes) => {
                        for pos in  indexes {
                            text.remove(pos - 1);
                        }
                    }
                    Err(err) => {
                        println!("Invalid args: {}", err);
                        return;
                    }
                }

                let mut output = text.join("\n");
                output.push_str("\n");

                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&self.path)
                    .expect("Couldn't open the todofile!");
                
                let mut buffer = BufWriter::new(file);
                buffer.write_all(output.as_bytes()).expect("Couldn't update the todofile!");
                buffer.flush().expect("Couldn't update the todofile!");
            }
            Err(err) => println!("Error indexes type: {}", err),
        }
    }

    pub fn checkbox_update(&self, args: &[String]) {
        let input: Result<Vec<usize>, ParseIntError> = args.iter().map(|arg| arg.parse::<usize>()).collect();
        
        match input {
            Ok(positions ) => {

                // Reading the text lines from the file
                // let contents = fs::read_to_string(&self.path).expect("Couldn't open the todofile!");
                // let mut text: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

                // Reading the text lines from the TodoItem struct
                let mut text: Vec<String> = Vec::new();
                for item in &self.items {
                    let completed_at = match &item.completed_at {
                        Some(value) => value.as_str(),
                        None => "",
                    };
                    let line = format!("{}|{}|{}|{}", item.is_done, item.created_at, completed_at, item.text);

                    text.push(line);
                }

                match validate_args(positions, &text) {
                    Ok(indexes) => {
                        for pos in  indexes {

                            // replacing old line for the new one
                            text[pos - 1] = match text[pos - 1].split_once("|") {
                                Some(line_split) => {
                                    if line_split.0 == "false" {
                                        format!("{}|{}", true, line_split.1)
                                    } else if line_split.0 == "true" {
                                        format!("{}|{}", false, line_split.1)
                                    } else {
                                        panic!("Couldn't update todo item!")
                                    }
                                }
                                None => panic!("Couldn't update todo item!")
                            };
                        }
                    }
                    Err(err) => {
                        println!("Invalid args: {}", err);
                        return;
                    }
                }

                let mut output = text.join("\n");
                output.push_str("\n");

                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&self.path)
                    .expect("Couldn't open the todofile!");
                
                let mut buffer = BufWriter::new(file);
                buffer.write_all(output.as_bytes()).expect("Couldn't update the todofile!");
                buffer.flush().expect("Couldn't update the todofile!");
            }
            Err(err) => println!("Error indexes type: {}", err),
        }
    }
}

fn format_to_file_lines(args: &[String]) -> Vec<String> {
    let items = args.join(" ")
        .split(",")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect::<Vec<String>>();

    let mut lines: Vec<String> = Vec::new();
    let time = Local::now().format("%Y-%m-%d %H:%M").to_string();

    for item in items {
        let line = format!("{}|{}||{}\n", false, time, item);
        lines.push(line);
    }

    lines
}

fn validate_args(mut pos: Vec<usize>, text: &[String]) -> Result<Vec<usize>, String> {
    if pos.is_empty() {
        return Err("Should pass an index after the rm command!".to_string());
    }

    pos.sort();
    pos.dedup();

    if pos[0] == 0 {
        return Err("Cannot remove item 0!".to_string());
    }

    pos.reverse();
    if pos[0] > text.len() {
        return Err("Index out of scope for todo list!".to_string());
    }

    Ok(pos)
}
