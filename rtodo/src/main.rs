use std::env;
use std::path::Path;

use structs::{Database, Todo};

mod structs;
mod ui;

fn main() {
    let path_string = "todos.csv";
    let path = Path::new(path_string);
    let mut database = match path.exists() {
        true => Database::from_file(path_string.to_string()),
        false => Database::new(path_string.to_string()),
    };

    // args
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "add" => database.add_todo(Todo::new(args[2].to_string(), false)),
        "ui" => ui::run_ui(&mut database).expect("couldn't run the ui"),
        _ => panic!("invalid argument"),
    };

    database.write_to_file();
}
