use std::env;
use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}

impl TodoItem {
    pub fn new(id: u32, name: String, completed: bool) -> Self {
        TodoItem {
            id,
            name,
            completed,
        }
    }
}

struct HelpItem {
    name: String,
    description: String,
}

impl HelpItem {
    pub fn new(name: &str, description: &str) -> Self {
        HelpItem {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

#[macro_export]
macro_rules! helpitem {
    ( $( $x:expr, $y: expr ),* ) => {
        {
            vec![
                $(
                    HelpItem::new($x, $y)
                )
            ]
        }
    };
}

static SEPERATOR: &str = "�";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut todo_items: Vec<TodoItem> = {
        match std::path::Path::new(get_todo().as_str()).exists() {
            true => parse_file(),
            false => vec![],
        }
    };

    todo_items.sort_by(|a, b| a.id.cmp(&b.id)); 
    
    match {
        match args.len() { 1 => None, _ => Some(args[1].as_str()), }
    } {
        Some("list") => print_todo(&todo_items, &args),
        Some("add") => add_item(&mut todo_items, &args),
        Some("rm") => remove_item(&mut todo_items, &args),
        Some("comp") => complete_item(&mut todo_items, &args),
        Some("help") => print_help(),
        Some(&_) => {
            eprintln!("Invalid Usage: '{}'", args[1]);
            print_help();
            std::process::exit(1);
        },
        None => print_todo(&todo_items, &args),
    };

    write_file(&todo_items);
}

#[allow(clippy::cmp_owned)]
fn print_todo(todo_items: &[TodoItem], args: &[String]) {
    if args.iter().any(|a| *a == "-a".to_string()) {
        todo_items
            .iter()
            .for_each(
                |a| println!("{:<3}│ {}", a.id, a.name)
            );
    } else {
        todo_items
            .iter()
            .filter(|a| !a.completed)
            .for_each(|a| println!(
                    "{:<3}│ {}", a.id, a.name)
            );
    };
}

fn print_help() {
    let commands = vec![
        HelpItem::new(
            "add",
            "Adds an item"
        ),
        HelpItem::new(
            "",
            ""
        )
    ];

    println!("USAGE: todo [COMMAND] [ID]{0}{0}COMMANDS:{0}add\tAdds an item{0}rm\tRemoves an item{0}comp\tlist\tLists todo{0}help\tPrints this{0}{0}Options:{0}-a\tPrints everything including completed items{0}", newline_os());
}

fn write_file(todo_items: &Vec<TodoItem>) {
    let mut todo_items_string = String::new();

    for i in todo_items {
        todo_items_string.push_str(&i.id.to_string());
        todo_items_string.push_str(SEPERATOR);
        todo_items_string.push_str(&i.name);
        todo_items_string.push_str(SEPERATOR);
        todo_items_string.push_str({
            match i.completed {
                true => "true",
                false => "false",
            }
        });
        todo_items_string.push_str(newline_os());
    }

    let _ = fs::write(get_todo().as_str(), todo_items_string);
}

fn parse_file() -> Vec<TodoItem> {
    let mut todo_items: Vec<TodoItem> = vec![];
    let todo_file: String = fs::read_to_string(get_todo().as_str()).expect("Could not read file: todo");

    if todo_file.is_empty() { return todo_items }

    let todo_items_full = todo_file
        .strip_suffix(newline_os())
        .expect("Stripped suffix on none value.")
        .split(newline_os());

    for i in todo_items_full {
        let split_items = i.split(SEPERATOR).collect::<Vec<&str>>();
        todo_items.push(
            TodoItem::new(
                split_items[0]
                    .trim()
                    .parse::<u32>()
                    .unwrap(), 
                split_items[1].to_string(),
                {
                    match split_items[2] {
                        "true" => true,
                        "false" => false,
                        &_ => panic!(
                            "Completed value could not be parsed!"
                        ),
                    }
                }
            )
        );
    }

    todo_items
}

fn complete_item(todo_items: &mut [TodoItem], args: &[String]) {
    let id: u32 = args[2].trim().parse().unwrap();

    todo_items
        .iter_mut()
        .filter(|a| a.id.eq(&id))
        .for_each(|a| a.completed = true);
}

fn add_item(todo_items: &mut Vec<TodoItem>, args: &[String]) {
    let item_name = &args[2];

    todo_items.push(
        TodoItem::new(
            {
                match todo_items.is_empty() {
                    true => 0,
                    false => todo_items
                        .iter()
                        .max_by_key(|a| a.id)
                        .unwrap()
                        .id,
                }
            } + 1,
            item_name.to_string(),
            false
        )
    );
}

fn remove_item(todo_items: &mut Vec<TodoItem>, args: &[String]) {
    let id = args[2].trim().parse().unwrap();

    todo_items.retain(|a| a.id != id);
}

fn newline_os() -> &'static str {
    match env::consts::OS {
        "windows" => "\r\n",
        _ => "\n",
    }
}

fn get_todo() -> String {
    let mut cache_dir = dirs::cache_dir().expect("Could not locate cache directory").into_os_string().into_string().unwrap();
    cache_dir.push_str({
        match env::consts::OS {
            "windows" =>"\\todo",
            _ => "/todo",
        }
    });

    cache_dir
}
