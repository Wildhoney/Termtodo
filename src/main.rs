use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let (event, value) = get_envs();

    match event.as_str() {
        "add" => match with_file().write_all(format!("{}\n", value).as_bytes()) {
            Err(_) => panic!("Failed to add todo."),
            Ok(_) => println!("Added todo."),
        },
        "remove" => println!("Removed todo."),
        _ => println!("Not sure really?"),
    }
}

fn get_envs() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    let event = args[1].to_lowercase();
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
