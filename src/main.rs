use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> () {
    let (event, value) = get_args();

    match event {
        Events::Add => match open_file().write_all(format!("{}\n", value).as_bytes()) {
            Err(_) => panic!("Failed to add todo."),
            Ok(_) => println!("Added todo."),
        },
        Events::Remove => println!("Removed todo."),
        Events::Unknown => println!("Not sure really?"),
    }
}

enum Events {
    Add,
    Remove,
    Unknown,
}

fn get_args() -> (Events, String) {
    let args: Vec<String> = env::args().collect();

    let event = match args.get(1) {
        Some(value) => match value.to_lowercase().as_str() {
            "add" => Events::Add,
            "remove" => Events::Remove,
            _ => Events::Unknown,
        },
        None => Events::Unknown,
    };

    let value = match args.get(2) {
        Some(value) => value.to_string(),
        None => String::from(""),
    };

    return (event, value);
}

fn open_file() -> std::fs::File {
    return OpenOptions::new()
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("Unable to open file");
}
