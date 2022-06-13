use std::fmt::Error;

#[derive(PartialEq)]
pub enum Screen {
    Menu,
    TodoList,
    AddTodo,
    ToggleTodo,
    EditTodo,
    Exit,
}

pub fn select_screen(selected_option: i32) -> Result<Screen, Error> {
    match selected_option {
        0 => Ok(Screen::Menu),
        1 => Ok(Screen::TodoList),
        2 => Ok(Screen::AddTodo),
        3 => Ok(Screen::ToggleTodo),
        4 => Ok(Screen::EditTodo),
        5 => Ok(Screen::Exit),
        _ => Err(Error),
    }
}

pub fn screen_name(screen: &Screen) -> String {
    match screen {
        Screen::EditTodo => String::from("Edit Todo"),
        Screen::Menu => String::from("Main Menu"),
        Screen::TodoList => String::from("Todo List"),
        Screen::AddTodo => String::from("Add Todo"),
        Screen::ToggleTodo => String::from("Toggle Todo"),
        Screen::Exit => String::from("Exit"),
    }
}
