use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const FILENAME: &str = "todos.txt";

#[derive(Debug, PartialEq)]
pub enum Kind {
    Add,
    Remove,
    List,
    Other,
}

#[derive(Debug, PartialEq)]
pub struct Todo {
    pub kind: Kind,
    pub value: String,
}

pub fn parse_args() -> Todo {
    let args: Vec<String> = env::args().collect();

    let kind = match args.get(1) {
        Some(value) => match value.to_lowercase().as_str() {
            "add" => Kind::Add,
            "remove" => Kind::Remove,
            "list" => Kind::List,
            _ => Kind::Other,
        },
        None => Kind::Other,
    };

    let value = match args.get(2) {
        Some(value) => format!("{}\n", value.to_string()),
        None => String::from(""),
    };

    return Todo { kind, value };
}

pub fn read() -> Option<String> {
    match fs::read_to_string(FILENAME) {
        Ok(content) => {
            let handle = |(index, line)| format!("#{}: {}", index + 1, line);
            let lines: Vec<String> = content.trim().lines().enumerate().map(handle).collect();
            Some(String::from(lines.join("\n")))
        }
        Err(_) => None,
    }
}

pub fn write(value: String) -> bool {
    let result = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILENAME)
        .expect("Unable to open file")
        .write_all(value.as_bytes());

    return match result {
        Ok(_) => true,
        Err(_) => false,
    };
}
