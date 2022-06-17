use crate::data;
use crate::todo;
use crate::util;

use colored::Colorize;
use std::io::stdin;

pub fn display(todo_list: &mut Vec<todo::Todo>) -> () {
    let mut input = String::new();

    println!("What is the number of the todo you want to toggle as done?\n");

    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Please write something");

    util::trim_newline(&mut input);

    let todo_index: usize = input.parse().unwrap();

    toggle_todo_as_done(todo_list, todo_index - 1);
}

fn toggle_todo_as_done(todo_list: &mut Vec<todo::Todo>, index: usize) -> () {
    todo_list[index].mark_as_done();
    data::update_list(todo_list).unwrap();

    println!(
        "{}",
        "\nYou have successfully marked your todo as complete\n".green()
    );
}

#[test]
fn toggle_todo_spec() {
    let mut todo_list: Vec<todo::Todo> = Vec::new();

    let new_todo = todo::Todo {
        name: String::from("Allow Jar Jar Binks to be a Sith Lord"),
        is_completed: false,
    };

    todo_list.push(new_todo);

    assert_eq!(todo_list[0].is_completed, false);
    toggle_todo_as_done(&mut todo_list, 0);
    assert_eq!(todo_list[0].is_completed, true);
}
