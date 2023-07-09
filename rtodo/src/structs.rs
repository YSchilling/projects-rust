use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub struct Todo {
    pub content: String,
    pub status: bool,
}

impl Todo {
    pub fn new(content: String, status: bool) -> Self {
        Self { content, status }
    }
    pub fn to_string(&self) -> String {
        self.content.clone() + ";" + &self.status.to_string()
    }
}

pub struct Database {
    pub path_string: String,
    pub file: File,
    pub todos: Vec<Todo>,
}

impl Database {
    pub fn new(path_string: String) -> Self {
        let path = Path::new(&path_string);
        let file = File::options()
            .write(true)
            .create(true)
            .open(&path)
            .expect("couldn't create file");
        let todos = Vec::new();

        Self {
            path_string,
            file,
            todos,
        }
    }

    pub fn from_file(path_string: String) -> Self {
        let path = Path::new(&path_string);
        path.try_exists().expect("File doesn't exist");
        let file = File::options()
            .write(true)
            .read(true)
            .open(&path)
            .expect("couldn't read file");
        let mut todos = Vec::new();
        for line in io::BufReader::new(file).lines() {
            let line = line.unwrap();
            let splitted_line: Vec<&str> = line.trim().split(';').collect();

            let content = splitted_line[0].to_string();
            let status: bool = splitted_line[1].parse().unwrap();

            todos.push(Todo::new(content, status))
        }

        let file = File::options()
            .write(true)
            .create(true)
            .open(&path)
            .expect("couldn't create file");

        Self {
            path_string,
            file,
            todos,
        }
    }

    pub fn write_to_file(mut self) {
        let mut file_data = String::new();
        for todo in self.todos {
            file_data.push_str(&todo.content);
            file_data.push_str(";");
            file_data.push_str(&todo.status.to_string());
            file_data.push_str("\n");
        }
        self.file
            .write_all(file_data.as_bytes())
            .expect("couldn't write to file");
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
}
