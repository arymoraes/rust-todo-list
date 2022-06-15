use colored::Colorize;
use std::io::stdin;

mod screens;
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

    let mut current_screen = screens::Screen::Menu;
    let mut todo_list: Vec<todo::Todo> = Vec::new();

    while current_screen != screens::Screen::Exit {
        let mut input = String::new();

        let screen_name = format!("{}", screens::screen_name(&current_screen))
            .purple()
            .bold();

        println!("\n{}\n", screen_name);

        if current_screen == screens::Screen::TodoList {
            screens::list_todos::display(&todo_list)
        }

        if current_screen == screens::Screen::AddTodo {
            screens::add_todo::display(&mut todo_list)
        }

        if current_screen == screens::Screen::ToggleTodo {
            screens::toggle_todo::display(&mut todo_list)
        }

        current_screen = screens::display_menu_options(current_screen);

        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Please write something");

        util::trim_newline(&mut input);

        let chosen_screen = input.parse().unwrap();

        let screen = screens::select_screen(chosen_screen);

        match screen {
            Ok(new_screen) => current_screen = new_screen,
            Err(_err) => println!("Invalid Screen"),
        }
    }
}
