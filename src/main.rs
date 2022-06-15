use colored::Colorize;
use std::io::stdin;

mod screen;
mod todo;
mod util;

fn main() {
    println!(
        "{}",
        format!("\n=======================================").blue()
    );
    println!(
        "{}",
        format!("Welcome to a simple Rust CLI todo-list").blue()
    );
    println!(
        "{}",
        format!("=======================================\n").blue()
    );

    let mut current_screen = screen::Screen::Menu;
    let mut todo_list: Vec<todo::Todo> = Vec::new();

    while current_screen != screen::Screen::Exit {
        let mut input = String::new();

        let screen_name = format!("{}", screen::screen_name(&current_screen))
            .purple()
            .bold();

        println!("\n{}\n", screen_name);

        if current_screen == screen::Screen::TodoList {
            screen::display_todos_screen(&todo_list)
        }

        if current_screen == screen::Screen::AddTodo {
            screen::display_add_todo(&mut todo_list)
        }

        if current_screen == screen::Screen::ToggleTodo {
            screen::display_toggle_todo(&mut todo_list)
        }

        current_screen = screen::display_menu_options(current_screen);

        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Please write something");

        util::trim_newline(&mut input);

        let chosen_screen = input.parse().unwrap();

        let screen = screen::select_screen(chosen_screen);

        match screen {
            Ok(new_screen) => current_screen = new_screen,
            Err(_err) => println!("Invalid Screen"),
        }
    }
}
