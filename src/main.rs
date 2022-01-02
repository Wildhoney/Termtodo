use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let (event, value) = get_envs();

    match event {
        Events::Add => match with_file().write_all(format!("{}\n", value).as_bytes()) {
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

fn get_envs() -> (Events, String) {
    let args: Vec<String> = env::args().collect();

    let event = match args[1].to_lowercase().as_str() {
        "add" => Events::Add,
        "remove" => Events::Remove,
        _ => Events::Unknown,
    };

    let value = if args.len() == 2 {
        String::from("")
    } else {
        args[2].to_string()
    };

    return (event, value);
}

fn with_file() -> std::fs::File {
    return OpenOptions::new()
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("Unable to open file");
}
