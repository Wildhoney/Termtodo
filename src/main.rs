mod todo;

fn main() -> () {
    match todo::parse_args() {
        todo::Todo::Add(Some(_)) => println!("Added todo."),
        todo::Todo::Add(None) => println!("Failed to add todo."),

        todo::Todo::Remove(Some(index)) => println!("Removed todo #{}.", index),
        todo::Todo::Remove(None) => println!("Failed to remove todo."),

        todo::Todo::List(Some(todos)) => println!("{}.", todos),
        todo::Todo::List(None) => println!("Failed to list todos."),

        todo::Todo::Other => println!("Invalid command specified."),
    }
}
