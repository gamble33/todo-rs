use std::io::{Stdout, Write};
use termion::raw::{RawTerminal};
use std::fs::File;

#[derive(Clone, Debug)]
pub struct Todo<'a> {
    pub text: &'a str,
    pub completed: bool,
    pub starred: bool,
}

impl<'a> Todo<'a> {
    pub fn new(text: &'a str, completed: bool, starred: bool) -> Todo<'a> {
        Todo {
            text,
            completed,
            starred
        }
    }
}

pub fn read_todos<'a>(path: &str) -> Vec<Todo<'a>> {
    let todos: Vec<Todo<'a>> = Vec::new();   
    todos
}

pub fn write_todos(path: &str, todos: &[Todo]) {
    let mut f_text: String = String::new();
    todos.iter().for_each(|todo| {
        f_text.push_str(todo.text);
        f_text.push('~');
        f_text.push(match todo.completed {
            true => '1',
            false => '0'
        });
        f_text.push('~');
        f_text.push(match todo.starred {
            true => '1',
            false => '0'
        });
        f_text.push('~');
    });
    std::fs::write(path, f_text.as_str());
}

pub fn render_todos(stdout: &mut RawTerminal<Stdout>, todos: &[Todo], todo_curr: usize){
    for (index, todo) in todos.iter().enumerate() {
        write!(stdout,
                "{}{}{}",
                termion::cursor::Down(1),
                termion::cursor::Left(u16::MAX),
                termion::style::Reset
        ).unwrap();
        
        if todo.starred { write!(stdout, "{}", termion::color::Fg(termion::color::Yellow)).unwrap(); }
        if todo.completed { write!(stdout, "[ X ]").unwrap(); } else { write!(stdout, "[   ]").unwrap(); }

        if index == todo_curr {
            write!(stdout,
                "{}{}",
                termion::style::Underline,
                termion::style::Invert
            ).unwrap();
        }

        write!(stdout,
            "{}",
            todo.text
        ).unwrap();
       
    }
}

pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
