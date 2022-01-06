mod todo;

use todo::Action;

fn main() -> () {
    match todo::parse_args() {
        Action::Add(Some(_)) => println!("Added todo."),
        Action::Add(None) => println!("Failed to add todo."),

        Action::Remove(Some(index)) => println!("Removed todo #{}.", index),
        Action::Remove(None) => println!("Failed to remove todo."),

        Action::List(Some(todos)) => println!("{}.", todos),
        Action::List(None) => println!("Failed to list todos."),

        Action::Other => println!("Invalid command specified."),
    }
}
