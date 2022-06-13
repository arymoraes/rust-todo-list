use std::io::stdin;
mod screen;
mod todo;

fn main() {
    println!("Welcome to my crap todo list");

    let mut current_screen = screen::Screen::Menu;
    let mut todo_list: Vec<todo::Todo> = Vec::new();

    while current_screen != screen::Screen::Exit {
        let mut input = String::new();

        println!(
            "You are currently on screen {}",
            screen::screen_name(&current_screen)
        );

        if current_screen == screen::Screen::TodoList {
            print_todo_list(&todo_list)
        }

        if current_screen == screen::Screen::AddTodo {
            let mut new_todo_input = String::new();

            stdin()
                .read_line(&mut new_todo_input)
                .ok()
                .expect("Please write something");

            trim_newline(&mut new_todo_input);

            let mut new_todo = todo::Todo {
                name: new_todo_input,
                is_completed: false,
            };

            new_todo.mark_as_done();

            todo_list.push(new_todo);
        }

        println!("1. List your todos");
        println!("2. Add a todo");
        println!("3. Toggle a todo as done");
        println!("4. Edit a todo");
        println!("5. Exit");
        if current_screen != screen::Screen::Menu {
            println!("0. Back to main menu");
        }

        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Please write something");

        trim_newline(&mut input);

        let chosen_screen = input.parse().unwrap();

        let screen = screen::select_screen(chosen_screen);

        match screen {
            Ok(new_screen) => current_screen = new_screen,
            Err(_err) => println!("Invalid Screen"),
        }
    }
}

// TODO: Implement this as a trait to string type maybe?
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn print_todo_list(todo_list: &Vec<todo::Todo>) -> () {
    println!("Your todo list:");
    println!("\n");
    println!("=======================");
    for t in todo_list {
        println!("{}, Completed: {}", &t.name, &t.is_completed);
    }
    println!("=======================");
    println!("\n");
}
