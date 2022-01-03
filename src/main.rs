use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> () {
    let todo = get_todo();

    match todo {
        Todo {
            kind: Kind::Add,
            value,
        } => match open_file().write_all(value.as_bytes()) {
            Err(_) => panic!("Failed to add todo."),
            Ok(_) => println!("Added todo."),
        },
        Todo {
            kind: Kind::Remove,
            value: _,
        } => println!("Removed todo."),
        Todo {
            kind: Kind::Other,
            value: _,
        } => println!("Not sure really?"),
    }
}

enum Kind {
    Add,
    Remove,
    Other,
}

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

fn open_file() -> std::fs::File {
    return OpenOptions::new()
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("Unable to open file");
}
