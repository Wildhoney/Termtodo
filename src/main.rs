// mod todo;
use std::ops;

#[derive(Debug, PartialEq)]
enum Name {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, PartialEq)]
struct Person {
    name: Name,
    age: i8,
}

trait Welcome {
    fn say_hello(&self) -> ();
}

impl Welcome for Person {
    fn say_hello(&self) -> () {
        match &self.name {
            Name::Single(name) => println!("Hello {}!", name),
            Name::Multiple(names) => println!("Hello {}!", names.join(", ")),
        }
    }
}

impl ops::Add<Person> for Person {
    type Output = Person;

    fn add(self, rhs: Person) -> Person {
        let mut names = match self.name {
            Name::Single(name) => vec![name],
            Name::Multiple(names) => names,
        };

        match rhs.name {
            Name::Single(name) => names.push(name),
            Name::Multiple(previous_names) => names.extend(previous_names.clone()),
        }

        return Person {
            name: Name::Multiple(names),
            age: self.age + rhs.age,
        };
    }
}

fn main() -> () {

    let mut xs = vec![1, 2, 3];
    xs.extend(vec![4, 5, 6]);
    println!("{:?}", xs);

    // let adam = Person {
    //     name: Name::Single(String::from("Adam")),
    //     age: 36,
    // };

    // let maria = Person {
    //     name: Name::Single(String::from("Maria")),
    //     age: 30,
    // };

    // let imogen = Person {
    //     name: Name::Single(String::from("Imogen")),
    //     age: 1,
    // };

    // let pets = Person {
    //     name: Name::Multiple(vec![String::from("Kittens"), String::from("Busters")]),
    //     age: 12,
    // };

    // let everybody = adam + maria + imogen + pets;
    // everybody.say_hello();

    // println!("{:?}", everybody.say_hello());

    // println!("Tattarrattat: {}", is_palindrome("Tattarrattat"));
    // println!("Prenuptial: {}", is_palindrome("Prenuptial"));

    // let initial: Vec<&str> = vec!["Adam", "Maria", "Imogen"];
    // let empty: Vec<&str> = vec![];
    // println!("{:?} -> {:?}\nEmpty: {:?}", initial, to_noah(&initial), to_noah(&empty));

    // match todo::parse_args() {
    //     todo::Todo::Add(Some(_)) => println!("Added todo."),
    //     todo::Todo::Add(None) => println!("Failed to add todo."),

    //     todo::Todo::Remove(Some(index)) => println!("Removed todo #{}.", index),
    //     todo::Todo::Remove(None) => println!("Failed to remove todo."),

    //     todo::Todo::List(Some(todos)) => println!("{}.", todos),
    //     todo::Todo::List(None) => println!("Failed to list todos."),

    //     todo::Todo::Other => println!("Invalid command specified."),
    // }
}

// fn is_palindrome(text: &str) -> bool {
//     let length = text.len();

//     let first = &text[0..1].to_lowercase();
//     let middle = &text[1..length - 1];
//     let last = &text[length - 1..length].to_lowercase();

//     if first == last  {
//         match middle.len() {
//             0 | 1 => true,
//             _ => is_palindrome(middle),
//         }
//     } else {
//         return false
//     }
// }
