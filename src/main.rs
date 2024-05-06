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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut todo_items = parse_file();
    todo_items.sort_by(|a, b| a.id.cmp(&b.id)); 
    
    match {
        match args.len() { 1 => None, _ => Some(args[1].as_str()), }
    } {
        Some("list") => print_todo(&todo_items),
        Some("add") => add_item(&mut todo_items, &args),
        Some("rm") => remove_item(&mut todo_items, &args),
        Some("complete") => complete_item(&mut todo_items, &args),
        Some("help") => print_help(),
        Some(&_) => {
            eprintln!("Invalid Usage: '{}'", args[1]);
            print_help();
            std::process::exit(1);
        },
        None => print_todo(&todo_items),
    };

    write_file(&todo_items);
}

fn print_todo(todo_items: &[TodoItem]) {
    todo_items
        .iter()
        .filter(|a| !a.completed)
        .for_each(|a| println!("{:<3}│ {}", a.id, a.name));
}

fn print_help() {
    let nl = newline_os();

    println!("USAGE: todo [COMMAND] [ID]{0}{0}COMMANDS:{0}add\tAdds an item{0}remove\tRemoves an item{0}list\tLists todo{0}help\tPrints this", nl);
}

fn write_file(todo_items: &Vec<TodoItem>) {
    let mut todo_items_string = String::new();

    for i in todo_items {
        todo_items_string.push_str(&i.id.to_string());
        todo_items_string.push_str("^^");
        todo_items_string.push_str(&i.name);
        todo_items_string.push_str("^^");
        todo_items_string.push_str({
            match i.completed {
                true => "true",
                false => "false",
            }
        });
        todo_items_string.push_str(newline_os());
    }

    let _ = fs::write("todo", todo_items_string);
}

fn parse_file() -> Vec<TodoItem> {
    let mut todo_items: Vec<TodoItem> = vec![];
    let todo_file: String = fs::read_to_string("todo").expect("Could not read file: todo");

    if todo_file.is_empty() { return todo_items }

    let todo_items_full = todo_file
        .strip_suffix(newline_os())
        .expect("Stripped suffix on none value.")
        .split(newline_os());

    for i in todo_items_full {
        let split_items = i.split("^^").collect::<Vec<&str>>();
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
    if env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    }
}
