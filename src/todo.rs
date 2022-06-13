pub struct Todo {
    pub name: String,
    pub is_completed: bool,
}

impl Todo {
    pub fn mark_as_done(&mut self) -> () {
        self.is_completed = true;
    }
}
