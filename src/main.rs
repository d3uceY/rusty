use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process;
// use std::io;

// fuck the borrow checker.

const FILEPATH: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    task: String,
    done: bool,
}

impl Todo {
    fn initialize_json() -> std::io::Result<File> {
        let path = Path::new(FILEPATH);
        if !path.exists() {
            let file: std::io::Result<File> = File::create(FILEPATH);
            let writer: BufWriter<&File>;

            match &file {
                Ok(f) => {
                    writer = BufWriter::new(f);
                }
                Err(e) => {
                    eprintln!("could not open {FILEPATH}: {e}");
                    std::process::exit(1);
                }
            }
            serde_json::to_writer(writer, &Vec::<Todo>::new())?;

            return file;
        }
        File::open(FILEPATH)
    }

    fn remove(todo_num: usize) -> std::io::Result<()> {
        let file = match File::open(FILEPATH) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("could not open {FILEPATH}: {e}");
                std::process::exit(1)
            }
        };
        let reader = BufReader::new(file);
        let mut todos: Vec<Todo> = match serde_json::from_reader(reader) {
            Ok(f) => f,
            Err(e) => {
                eprint!("could not read exisiting todos from {}: {}", FILEPATH, e);
                std::process::exit(1)
            }
        };

        todos.remove(todo_num - 1);

        let file = OpenOptions::new().write(true).truncate(true).open(FILEPATH)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &todos)?;
        println!("todo number {} has been removed", todo_num);
        Ok(())
    }

    fn done(todo_num: usize) -> std::io::Result<()> {
        let file = match File::open(FILEPATH) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("could not open {FILEPATH}: {e}");
                std::process::exit(1)
            }
        };
        let reader = BufReader::new(file);
        let mut todos: Vec<Todo> = match serde_json::from_reader(reader) {
            Ok(f) => f,
            Err(e) => {
                eprint!("could not read exisiting todos from {}: {}", FILEPATH, e);
                std::process::exit(1)
            }
        };

        todos[todo_num - 1].done = true;

        let file = OpenOptions::new().write(true).truncate(true).open(FILEPATH)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &todos)?;
        println!("todo number {} is done", todo_num);
        Ok(())
    }

    fn add_todo(new_todo: Todo) -> std::io::Result<()> {
        let file = match File::open(FILEPATH) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("could not open {FILEPATH}: {e}");
                std::process::exit(1)
            }
        };
        let reader = BufReader::new(file);
        let task: String = new_todo.task.clone();
        let mut existing: Vec<Todo> = match serde_json::from_reader(reader) {
            Ok(f) => f,
            Err(e) => {
                eprint!("could not read exisiting todos from {}: {}", FILEPATH, e);
                std::process::exit(1)
            }
        };
        existing.push(new_todo);
        let file = OpenOptions::new().write(true).truncate(true).open(FILEPATH)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &existing)?;
        println!("{} added to todos", task);
        Ok(())
    }

    fn list_todo() {
        let file = match File::open(FILEPATH) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("could not open {FILEPATH}: {e}");
                std::process::exit(1)
            }
        };
        let reader = BufReader::new(file);
        let todos: Vec<Todo> = match serde_json::from_reader(reader) {
            Ok(f) => f,
            Err(e) => {
                eprint!("could not read exisiting todos from {}: {}", FILEPATH, e);
                std::process::exit(1)
            }
        };

        if todos.is_empty() {
            println!("Hit the road duuudeee. No todos found");
            return;
        }

        for (index, todo) in todos.iter().enumerate() {
            let done_marker: &str;

            if todo.done == true {
                done_marker = "done"
            } else {
                done_marker = ""
            }
            println!("{}. {} - {}", index + 1, todo.task, done_marker)
        }
    }

    fn purge() -> std::io::Result<()> {
        let file = File::create(FILEPATH)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &Vec::<Todo>::new())?;
        println!("the purggeeeeee!!!");
        Ok(())
    }
}

fn check(result: std::io::Result<()>) {
    if let Err(e) = result {
        eprintln!("{FILEPATH}: {e}");
        process::exit(1);
    }
}

fn main() {
    // make sure the file exists
    if let Err(e) = Todo::initialize_json() {
        eprintln!("could not open {FILEPATH}: {e}");
        std::process::exit(1);
    }

    // all za arguments
    let mut all_args: HashMap<&str, &str> = HashMap::new();
    all_args.insert("version", "when you need the version");
    all_args.insert("add", "to add a todo add <todo>");
    all_args.insert("remove", "to remove a todo. remove <todo number>");
    all_args.insert("done", "mark todo as done. done <todo number>");
    all_args.insert("list", "list todo");
    all_args.insert("purge", "Delete all todos");

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s: &String| s.as_str()) {
        // i do not need to explain
        Some("version") => println!("0.1.0 -- There won't be a 0.1.1 ever"),

        // need ya help?
        Some("help") => {
            for (k, v) in all_args.iter() {
                println!("{} - {}", k, v);
            }
        }

        Some("add") => {
            let char_count: usize = args.get(2).map(|s| s.chars().count()).unwrap_or(0);
            if char_count < 1 {
                eprintln!("this is not a valid todo");
                std::process::exit(1);
            } else {
                let arg2: String = std::env::args()
                    .nth(2)
                    .unwrap_or_else(|| "new todo".to_string());
                let new_todo = Todo {
                    task: arg2,
                    done: false,
                };
                check(Todo::add_todo(new_todo));
            }
        }

        Some("list") => {
            Todo::list_todo();
        }

        // the purgggggeeee
        Some("purge") => check(Todo::purge()),

        // remove this P
        Some("remove") => {
            let char_count: usize = args.get(2).map(|s| s.chars().count()).unwrap_or(0);
            if char_count < 1 {
                eprintln!("this is not a valid todo number. Don't PMO");
                std::process::exit(1);
            } else {
                let arg2: usize = std::env::args()
                    .nth(2)
                    .unwrap_or_else(|| {
                        eprintln!("Missing todo ID");
                        std::process::exit(1);
                    })
                    .parse()
                    .expect("Todo ID must be a number");

                check(Todo::remove(arg2));
            }
        }

        // mark this p as done, fam
        Some("done") => {
            let char_count: usize = args.get(2).map(|s| s.chars().count()).unwrap_or(0);
            if char_count < 1 {
                eprintln!("this is not a valid todo number. Don't PMO");
                std::process::exit(1);
            } else {
                let arg2: usize = std::env::args()
                    .nth(2)
                    .unwrap_or_else(|| {
                        eprintln!("Missing todo ID");
                        std::process::exit(1);
                    })
                    .parse()
                    .expect("Todo ID must be a number");

                check(Todo::done(arg2));
            }
        }

        // fucking coon
        _ => eprintln!("use the available args, you coon. use help arg"),
    }
}
