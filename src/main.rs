use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::event::Key;
use std::io::{stdout, stdin, Stdout, Write};
use todo_rs::Todo;

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn main() {
    
    let mut todos: Vec<Todo> = Vec::new();

    let file_text = read_file("todos.todo");
    let todo_count = file_text.matches('~').count() / 3;
    println!("{}", todo_count);
    let mut split = file_text.split('~');
    for i in 0..todo_count {
        let text = split.next().clone().unwrap();
        let completed: bool = split.next().unwrap() == String::from("1");
        let starred: bool = split.next().unwrap() == String::from("1");
        let todo = todo_rs::Todo::new(text, completed, starred);
        todos.push(todo);
    }

    let stdin = stdin();
    let mut stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();

    write!(stdout,
        "{}{}[q] or Ctrl+C to exit.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    ).unwrap();

    stdout.flush().unwrap();

    let mut todo_curr: usize = 2;

    for c in stdin.keys() {
    
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,

            Key::Up => {
                if todo_curr > 0 { todo_curr -= 1; }
            },

            Key::Down => {
                todo_curr += 1;
            },

            Key::Char('\n') => {
                todos[todo_curr].completed = !todos[todo_curr].completed;
            }

            Key::Char('s') => {
                todos[todo_curr].starred = !todos[todo_curr].starred;
            }
            _ => {}
        }

        if todo_curr >= todos.len() { todo_curr = todos.len()-1; }

        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();

        todo_rs::render_todos(&mut stdout, todos.as_slice(), todo_curr); 

        stdout.flush().unwrap();
    }
    
    todo_rs::write_todos("todos.todo", todos.as_slice());   

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
