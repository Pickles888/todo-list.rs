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
    /*vec![
        TodoItem::new(1, "walk dog".to_string()),
        TodoItem::new(3, "eat food".to_string()),
        TodoItem::new(6, "BEANS".to_string()),
    ];*/

    complete_item(&mut todo_items, &args);

    print_todo(&todo_items);

    write_file(&todo_items); 
    /*match args[1] {
        list => print_todo(&mut todo_items),
        add => add_todo(),
        remove => remove_todo(),
        _ => print_help(),
    }*/
}

fn print_todo(todo_items: &Vec<TodoItem>) {
    for i in todo_items {
        if !i.completed {
            println!("{:<3}│ {}", i.id, i.name);
        }
    };
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

    let todo_items_full = todo_file.strip_suffix(newline_os()).expect("Stripped suffix on none value.").split(newline_os());

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

fn complete_item(todo_items: &mut Vec<TodoItem>, args: &Vec<String>) {
    //let id: u32 = args[2].trim().parse().unwrap();
    let id: u32 = 1;

    todo_items
        .iter_mut()
        .filter(|a| a.id.eq(&id))
        .for_each(|a| a.completed = true);
}

fn add_item(todo_items: &mut Vec<TodoItem>, args: &Vec<String>) {
    let item_name = &args[2];
    let max_item = todo_items.iter().max_by_key(|a| a.id);

    todo_items.push(
        TodoItem::new(
            max_item.unwrap().id,
            item_name.to_string(),
            false
        )
    );
}

fn newline_os() -> &'static str {
    if env::consts::OS == "windows" {
        "\r\n"
    } else {
        "\n"
    }
}
