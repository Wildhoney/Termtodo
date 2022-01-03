mod todo;

fn main() -> () {
    let args = todo::parse_args();

    match args.kind {
        todo::Kind::Add => match todo::write(args.value) {
            true => println!("Added todo."),
            false => panic!("Failed to add todo."),
        },
        todo::Kind::Remove => println!("Removed todo."),
        todo::Kind::List => match todo::read() {
            Some(todos) => println!("{}", todos),
            None => panic!("Failed to read todos."),
        },
        todo::Kind::Other => println!("Not sure really?"),
    }
}
