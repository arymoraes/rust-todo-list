use crate::todo;
use crate::util;

use colored::Colorize;
use std::fmt::Error;
use std::io::stdin;

#[derive(PartialEq)]
pub enum Screen {
    Menu,
    TodoList,
    AddTodo,
    ToggleTodo,
    Exit,
}

pub fn select_screen(selected_option: i32) -> Result<Screen, Error> {
    match selected_option {
        0 => Ok(Screen::Menu),
        1 => Ok(Screen::TodoList),
        2 => Ok(Screen::AddTodo),
        3 => Ok(Screen::ToggleTodo),
        4 => Ok(Screen::Exit),
        _ => Err(Error),
    }
}

pub fn screen_name(screen: &Screen) -> String {
    match screen {
        Screen::Menu => String::from("Main Menu"),
        Screen::TodoList => String::from("Todo List"),
        Screen::AddTodo => String::from("Add Todo"),
        Screen::ToggleTodo => String::from("Toggle Todo"),
        Screen::Exit => String::from("Exit"),
    }
}

pub fn display_todos_screen(todo_list: &Vec<todo::Todo>) -> () {
    if todo_list.len() < 1 {
        println!("{}", format!("\nYour todo list is empty.\n").red());
        return;
    }

    println!("\nYour todo list:");
    println!("=======================");
    for (i, t) in todo_list.iter().enumerate() {
        let is_completed = match t.is_completed {
            true => format!("(x)").green(),
            false => format!("( )").red(),
        };

        println!("{}. {} {}", i + 1, is_completed, &t.name);
    }
    println!("======================\n");
}

pub fn display_add_todo(todo_list: &mut Vec<todo::Todo>) -> () {
    let mut new_todo_input = String::new();

    println!("What is the name of your new todo?\n");

    stdin()
        .read_line(&mut new_todo_input)
        .ok()
        .expect("Please write something");

    util::trim_newline(&mut new_todo_input);

    let new_todo = todo::Todo {
        name: new_todo_input,
        is_completed: false,
    };

    println!("{}", "\nTodo successfully added!\n".green());

    todo_list.push(new_todo);
}

pub fn display_menu_options(current_screen: Screen) -> Screen {
    print_option("1", "List your todos");
    print_option("2", "Add a todo");
    print_option("3", "Toggle a todo as done\n");
    print_option("4", "Exit");

    if current_screen != Screen::Menu {
        print_option("0", "Back to main menu");
    }

    print!("\n");

    current_screen
}

pub fn display_toggle_todo(todo_list: &mut Vec<todo::Todo>) -> () {
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

// private

fn print_option(number: &str, option_name: &str) -> () {
    let colored_number = format!("{}.", number).green();
    let colored_string = format!("{} {}", colored_number, option_name);

    println!("{}", colored_string);
}
