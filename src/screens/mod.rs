pub mod add_todo;
pub mod list_todos;
pub mod toggle_todo;

use colored::Colorize;
use std::fmt::Error;

#[derive(PartialEq, Debug)]
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

fn print_option(number: &str, option_name: &str) -> () {
    let colored_number = format!("{}.", number).green();
    let colored_string = format!("{} {}", colored_number, option_name);

    println!("{}", colored_string);
}

#[test]
fn screen_name_spec() {
    assert_eq!("Add Todo", screen_name(&Screen::AddTodo));
    assert_eq!("Exit", screen_name(&Screen::Exit));
    assert_eq!("Main Menu", screen_name(&Screen::Menu));
    assert_eq!("Todo List", screen_name(&Screen::TodoList));
    assert_eq!("Toggle Todo", screen_name(&Screen::ToggleTodo));
}

#[test]
fn select_screen_spec() {
    assert_eq!(Ok(Screen::Menu), select_screen(0));
    assert_eq!(Ok(Screen::TodoList), select_screen(1));
    assert_eq!(Ok(Screen::AddTodo), select_screen(2));
    assert_eq!(Ok(Screen::ToggleTodo), select_screen(3));
    assert_eq!(Ok(Screen::Exit), select_screen(4));
    assert_eq!(Err(Error), select_screen(7));
    assert_eq!(Err(Error), select_screen(-3));
}
