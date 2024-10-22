pub struct Todo {
    pub task_name: String,
    pub status: bool,
}

impl Todo {
    pub fn new(name: String) -> Self {
        Todo {
            task_name: name,
            status: false,
        }
    }
}
