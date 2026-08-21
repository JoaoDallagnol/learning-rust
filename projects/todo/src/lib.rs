use std::fs;
use std::time::SystemTime;
use std::io::{self, Write, BufWriter};

struct TodoItem {
    text: String,
    is_done: bool,
    created_at: String,
    completed_at: String,
}

impl TodoItem {
    fn create_item(line: String) -> Self {
        let split_line: Vec<&str> = line.splitn(4, "|").collect();

        TodoItem {
            is_done: split_line[0].parse::<bool>().expect("Error to trying to parser boolean"),
            created_at: split_line[1].to_string(),
            completed_at: split_line[2].to_string(),
            text: split_line[3].to_string()
        }
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

    pub fn list(&self) -> Result<(), std::io::Error>{
        let stdout = io::stdout();

        // Buffered write for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();

        for (index, item) in self.items.iter().enumerate() {
            let index = index + 1;

            data.push_str(&index.to_string());
            data.push_str(". ");
            
            match &item.is_done {
                true => data.push_str("[x] "),
                false => data.push_str("[ ] "),
            }

            data.push_str(&item.text);
            data.push_str("\nCriado em: ");
            data.push_str(&item.created_at);

            if *&item.completed_at.len() > 1 {
                data.push_str("\nCompletado em: ");
                data.push_str(&item.completed_at);
            }

            data.push_str("\n\n");
        }

        writer.write_all(data.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
}