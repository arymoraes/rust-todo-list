use crate::data;
use crate::todo;
use crate::util;

use colored::Colorize;
use std::io::stdin;

pub fn display(todo_list: &mut Vec<todo::Todo>) -> () {
    let mut new_todo_input = String::new();

    println!("What is the name of your new todo?\n");

    stdin()
        .read_line(&mut new_todo_input)
        .ok()
        .expect("Please write something");

    util::trim_newline(&mut new_todo_input);

    add_todo(todo_list, new_todo_input);
}

fn add_todo(todo_list: &mut Vec<todo::Todo>, todo_name: String) -> () {
    let new_todo = todo::Todo {
        name: todo_name,
        is_completed: false,
    };

    println!("{}", "\nTodo successfully added!\n".green());

    todo_list.push(new_todo);
    data::update_list(todo_list).unwrap();
}

#[test]
fn add_todo_spec() {
    let mut todo_list: Vec<todo::Todo> = Vec::new();
    let new_todo = String::from("Allow Jar Jar Binks to be a Sith Lord");

    assert_eq!(todo_list.len(), 0);

    add_todo(&mut todo_list, new_todo);

    assert_eq!(todo_list.len(), 1);
    assert_eq!(
        todo_list[0].name,
        String::from("Allow Jar Jar Binks to be a Sith Lord")
    );
}
