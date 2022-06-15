use crate::todo;

use colored::Colorize;

pub fn display(todo_list: &Vec<todo::Todo>) -> () {
    if todo_list.len() < 1 {
        println!("{}", format!("\nYour todo list is empty.\n").red());
        return;
    }

    println!("\nYour todo list:");
    println!("=======================");
    for (i, t) in todo_list.iter().enumerate() {
        print_todo(t, i);
    }
    println!("======================\n");
}

fn print_todo(t: &todo::Todo, i: usize) -> () {
    let is_completed = match t.is_completed {
        true => format!("(x)").green(),
        false => format!("( )").red(),
    };

    println!("{}. {} {}", i + 1, is_completed, &t.name);
}
