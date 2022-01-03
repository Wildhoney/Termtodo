use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const FILENAME: &str = "todos.txt";

fn main() -> () {
    let todo = get_todo();

    match todo.kind {
        Kind::Add => match write_todo(todo.value) {
            true => println!("Added todo."),
            false => panic!("Failed to add todo."),
        },
        Kind::Remove => println!("Removed todo."),
        Kind::List => match read_todos() {
            Some(todos) => println!("{}", todos),
            None => panic!("Failed to read todos."),
        },
        Kind::Other => println!("Not sure really?"),
    }
}

#[derive(Debug, PartialEq)]
enum Kind {
    Add,
    Remove,
    List,
    Other,
}

#[derive(Debug, PartialEq)]
struct Todo {
    kind: Kind,
    value: String,
}

fn get_todo() -> Todo {
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

fn read_todos() -> Option<String> {
    match fs::read_to_string(FILENAME) {
        Ok(content) => Some(String::from(content.trim())),
        Err(_) => None,
    }
}

fn write_todo(value: String) -> bool {
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
