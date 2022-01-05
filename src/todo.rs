use std::env;
use std::fs::{self, File};
use std::io::Write;

const FILENAME: &str = "Actions.txt";

#[derive(Debug, PartialEq)]
pub enum Action {
    Add(Option<String>),
    Remove(Option<usize>),
    List(Option<String>),
    Other,
}

pub fn parse_args() -> Action {
    let args: Vec<String> = env::args().collect();
    let empty = String::new();
    let event = args.get(1).unwrap_or(&empty);

    return match event.as_str() {
        "add" => match args.get(2) {
            Some(value) => Action::Add(add(value.to_string())),
            None => Action::Add(None),
        },
        "remove" | "rm" => match args.get(2) {
            Some(value) => match value.parse::<usize>() {
                Ok(value) => Action::Remove(delete(value)),
                Err(_) => Action::Remove(None),
            },
            None => Action::Remove(None),
        },
        "list" | "ls" | "" => Action::List(read()),
        _ => Action::Other,
    };
}

fn get_lines() -> Option<Vec<String>> {
    match fs::read_to_string(FILENAME) {
        Ok(data) => {
            let lines = data
                .trim()
                .lines()
                .map(String::from)
                .collect::<Vec<String>>();
            return Some(lines);
        }
        Err(_) => None,
    }
}

fn add(text: String) -> Option<String> {
    match get_lines() {
        Some(lines) => {
            let mut updated_lines = lines.clone();
            updated_lines.push(text.clone());

            match write(updated_lines.join("\n")) {
                Some(_) => Some(text),
                None => None,
            }
        }
        None => None,
    }
}

fn read() -> Option<String> {
    match get_lines() {
        Some(lines) => {
            let format = |(index, line)| format!("#{}: {}", index + 1, line);
            let lines: Vec<String> = lines.iter().enumerate().map(format).collect();
            Some(String::from(lines.join("\n")))
        }
        None => None,
    }
}

fn delete(index: usize) -> Option<usize> {
    match get_lines() {
        Some(lines) => {
            let mut updated_lines = lines.clone();
            updated_lines.remove(index - 1);

            match write(updated_lines.join("\n")) {
                Some(_) => Some(index),
                None => None,
            }
        }
        None => None,
    }
}

fn write(value: String) -> Option<String> {
    return match File::create(FILENAME) {
        Ok(mut file) => match file.write_all(value.as_bytes()) {
            Ok(_) => Some(value.to_string()),
            Err(_) => None,
        },
        Err(_) => None,
    };
}
