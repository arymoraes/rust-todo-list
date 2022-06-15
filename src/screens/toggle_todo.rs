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

    todo_list[todo_index - 1].mark_as_done();

    println!(
        "{}",
        "\nYou have successfully marked your todo as complete\n".green()
    );
}
