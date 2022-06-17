use std::{fs::read_to_string, fs::write, fs::File};

use crate::todo;

pub const JSON_PATH: &str = "storage/data.json";

pub fn open_todo_list() -> Vec<todo::Todo> {
    let file = File::open(JSON_PATH);

    match file {
        Ok(file) => file,
        Err(_error) => {
            println!("Creating new todo file");
            let new_file = File::create(JSON_PATH);

            match new_file {
                Ok(_) => {
                    let todos = Vec::new();
                    initialize_json_array(&JSON_PATH);
                    return todos;
                }
                Err(_err) => panic!(
                    "Critical Error: Not able to read or create todo list file. Shutting down..."
                ),
            }
        }
    };

    let data = read_to_string(JSON_PATH).unwrap();

    let todos: Vec<todo::Todo> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    todos
}

pub fn get_todos() -> Vec<todo::Todo> {
    let data = read_to_string(JSON_PATH).unwrap();

    let todos: Vec<todo::Todo> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    todos
}

pub fn update_list(t: &Vec<todo::Todo>) -> Result<(), std::io::Error> {
    let j = serde_json::to_string(t)?;
    write(JSON_PATH, j).expect("Failed to write to file.");

    Ok(())
}

// Initializes an empty array on JSON File if we're creating the file
fn initialize_json_array(path: &str) -> () {
    let stringified_array = "[]";
    write(path, stringified_array).expect("Unable to write to file");
}
