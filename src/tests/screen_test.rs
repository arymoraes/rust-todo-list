mod screen;

#[test]
fn correct_screen_name() {
    let screen = screen::Screen::AddTodo;

    assert_eq!("Add Todo", screen::screen_name(&screen));
}
