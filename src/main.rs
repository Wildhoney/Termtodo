mod todo;

fn main() -> () {
    match todo::parse_args() {
        todo::Action::Add(Some(_)) => println!("Added todo."),
        todo::Action::Add(None) => println!("Failed to add todo."),

        todo::Action::Remove(Some(index)) => println!("Removed todo #{}.", index),
        todo::Action::Remove(None) => println!("Failed to remove todo."),

        todo::Action::List(Some(todos)) => println!("{}.", todos),
        todo::Action::List(None) => println!("Failed to list todos."),

        todo::Action::Other => println!("Invalid command specified."),
    }
}
