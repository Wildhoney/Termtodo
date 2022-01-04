use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const FILENAME: &str = "todos.txt";

#[derive(Debug, PartialEq)]
pub enum Todo {
    Add(Option<String>),
    Remove(Option<i32>),
    List(Option<String>),
}

pub fn parse_args() -> Todo {
    let args: Vec<String> = env::args().collect();
    let empty = String::new();
    let event = args.get(1).unwrap_or(&empty);

    return match event.as_str() {
        "add" => match args.get(2) {
            Some(value) => {
                let value = String::from(format!("{}\n", value.to_string()));
                Todo::Add(write(&value))
            }
            None => Todo::Add(None),
        },
        "remove" | "rm" => match args.get(2) {
            Some(value) => match value.parse::<i32>() {
                Ok(value) => Todo::Remove(Some(value)),
                Err(_) => Todo::Remove(None),
            },
            None => Todo::Remove(None),
        },
        "list" | "ls" | _ => Todo::List(read()),
    };
}

fn read() -> Option<String> {
    match fs::read_to_string(FILENAME) {
        Ok(content) => {
            let handle = |(index, line)| format!("#{}: {}", index + 1, line);
            let lines: Vec<String> = content.trim().lines().enumerate().map(handle).collect();
            Some(String::from(lines.join("\n")))
        }
        Err(_) => None,
    }
}

fn write(value: &String) -> Option<String> {
    let result = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILENAME)
        .expect("Unable to open file")
        .write_all(value.as_bytes());

    return match result {
        Ok(_) => Some(value.to_string()),
        Err(_) => None,
    };
}
