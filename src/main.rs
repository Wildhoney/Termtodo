use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let event = &*args[1].to_lowercase();
    let item = &*args[2];

    match event {
        "add" => match with_file().write_all(format!("{}\n", item).as_bytes()) {
            Err(_) => panic!("couldn't write file."),
            Ok(_) => println!("successfully wrote to file."),
        },
        "remove" => println!("Remove!"),
        _ => println!("Not sure really?"),
    }
}

fn with_file() -> std::fs::File {
    return OpenOptions::new()
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("Unable to open file");
}
