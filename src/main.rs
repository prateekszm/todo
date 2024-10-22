use crate::app_state::AppState;
use std::io;
use std::io::Write;

mod app_state;
mod todo;

fn main() {
    let mut app_state = AppState::new();
    println!("Todo");
    loop {
        println!("Choose an action: add, remove, edit, finish, print, quit \n");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "add" => {
                println!("Enter todo name:");
                let mut name = get_input_from_user();
                app_state.add(name.trim().to_string());
            }
            "remove" => {
                println!("Enter index to remove:");
                let mut index = get_input_from_user();
                if let Ok(index) = index.trim().parse::<usize>() {
                    app_state.remove(index);
                } else {
                    println!("Invalid index!");
                }
            }
            "edit" => {
                //take position input from user
                let mut position = get_input_from_user();
                if let Ok(index) = position.trim().parse::<usize>() {
                    let name = get_input_from_user();
                    app_state.edit(index, name);
                }
            }
            "finish" => {
                println!("Enter index to finish:");
                let mut index = get_input_from_user();
                if let Ok(index) = index.parse::<usize>() {
                    app_state.finish(index);
                }
            }
            "print" => {
                app_state.print();
            }
            "quit" => break,
            _ => {
                println!("Invalid input");
            }
        }
    }
    println!("Goodbye! you are always my bruh");
}

fn get_input_from_user() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}
