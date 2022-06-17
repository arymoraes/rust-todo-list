pub mod add_todo;
pub mod list_todos;
pub mod toggle_todo;

use colored::Colorize;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Screen {
    Menu,
    TodoList,
    AddTodo,
    ToggleTodo,
    Exit,
}

impl From<u8> for Screen {
    fn from(value: u8) -> Self {
        match value {
            0 => Screen::Menu,
            1 => Screen::TodoList,
            2 => Screen::AddTodo,
            3 => Screen::ToggleTodo,
            4 => Screen::Exit,
            _ => Screen::Menu,
        }
    }
}

impl From<Screen> for String {
    fn from(s: Screen) -> Self {
        match s {
            Screen::Menu => String::from("Main Menu"),
            Screen::TodoList => String::from("Todo List"),
            Screen::AddTodo => String::from("Add Todo"),
            Screen::ToggleTodo => String::from("Toggle Todo"),
            Screen::Exit => String::from("Exit"),
        }
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
    assert_eq!("Add Todo", String::from(Screen::AddTodo));
    assert_eq!("Exit", String::from(Screen::Exit));
    assert_eq!("Main Menu", String::from(Screen::Menu));
    assert_eq!("Todo List", String::from(Screen::TodoList));
    assert_eq!("Toggle Todo", String::from(Screen::ToggleTodo));
}

#[test]
fn select_screen_spec() {
    assert_eq!(Screen::Menu, Screen::from(0));
    assert_eq!(Screen::TodoList, Screen::from(1));
    assert_eq!(Screen::AddTodo, Screen::from(2));
    assert_eq!(Screen::ToggleTodo, Screen::from(3));
    assert_eq!(Screen::Exit, Screen::from(4));
    assert_eq!(Screen::Menu, Screen::from(55));
    assert_eq!(Screen::Menu, Screen::from(10));
}
