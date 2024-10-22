use crate::todo::Todo;

pub struct AppState {
    pub todos: Vec<Todo>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { todos: Vec::new() }
    }

    pub fn add(&mut self, name: String) {
        self.todos.push(Todo::new(name));
    }

    pub fn remove(&mut self, index: usize) {
        if self.todos.len() > index {
            self.todos.remove(index);
        } else {
            println!("index out of bounds");
        }
    }

    pub fn edit(&mut self, index: usize, name: String) {
        if self.todos.len() > index {
            self.todos[index] = Todo::new(name);
        } else {
            println!("index out of bounds");
        }
    }

    pub fn finish(&mut self, index: usize) {
        if index < self.todos.len() {
            self.todos[index].status = true
        }
    }

    pub fn print(&self) {
        for (i, todo) in self.todos.iter().enumerate() {
            let status = if todo.status {
                "complete "
            } else {
                "not completed"
            };
            println!("{}: [{}] {}", i, status, todo.task_name);
        }
    }
}
