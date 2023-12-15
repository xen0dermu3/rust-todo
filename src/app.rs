use crate::{select_option::SelectOption, todo::Todo};
use colored::*;
use inquire::Select;
use std::io::{self, Write};

pub fn clear_screen() {
    // Clear terminal screen and go to position (1, 1)
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn bootstrap() {
    let mut todos: Vec<Todo> = vec![];

    clear_screen();

    println!("{}", "Hello, welcome to DoTo".green());

    loop {
        let options = vec![
            SelectOption::new(0, "(0) Get items"),
            SelectOption::new(1, "(1) Create item"),
            SelectOption::new(2, "(2) Update item"),
            SelectOption::new(3, "(3) Mark item as completed/active"),
            SelectOption::new(4, "(4) Delete item"),
            SelectOption::new(5, "(5) Exit"),
        ];

        let selected_option =
            Select::new("Please select what do you want to do today?", options).prompt();

        println!("");

        match selected_option {
            Ok(selected) => match selected.index {
                0 => get_items(&todos),
                1 => create_item(&mut todos),
                2 => update_item(&mut todos),
                3 => mark_as_completed(&mut todos),
                4 => delete_item(&mut todos),
                5 => {
                    println!("{}", "Have a nice day. Goodbye!".green());
                    break;
                }
                _ => println!("{}", "Sorry, but we don't have such option".yellow()),
            },
            Err(_) => println!("{}", "There was an error, please try again".red()),
        }

        println!("");
    }
}

fn get_items(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("{}", "Oops, seems that your todos list is empty".yellow());
        return;
    }

    println!("{}", "Here is the list of your todos:");

    for todo in todos.iter() {
        let checked = if todo.completed { "x" } else { " " };
        let max_length = 50;

        let desc = if todo.description.len() > max_length {
            format!("{}...", &todo.description[..max_length])
        } else {
            todo.description.to_string()
        };

        println!("- [{}] {} ({})", checked, todo.title, desc)
    }
}

fn create_item(todos: &mut Vec<Todo>) {
    println!("Let's create a new todo:");

    let mut description = String::new();
    let mut title = String::new();

    loop {
        print!("Please input title -> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");

        if title.trim().is_empty() {
            print!("{}", "Oops, title is required field. ".yellow());
            continue;
        }

        print!("Please input description (optional) -> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");

        break;
    }

    todos.push(Todo {
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        completed: false,
    });

    println!(
        "{} {} {}",
        "Item ->".purple(),
        title.trim().purple(),
        "<- created successfully !".purple()
    );
}

fn mark_as_completed(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("{}", "Oops, seems that your todos list is empty".yellow());
        return;
    }

    println!("Here is the list of your todos:");

    loop {
        let mut options = vec![];
        let todos_cloned = todos.clone();

        for (index, todo) in todos_cloned.iter().enumerate() {
            options.push(SelectOption::new(index, &todo.title[..]))
        }

        let selected_option =
            Select::new("Please select item to mark as completed/active: ", options).prompt();

        match selected_option {
            Ok(selected_todo) => {
                for (index, todo) in todos.iter_mut().enumerate() {
                    if selected_todo.index == index {
                        todo.mark_as_completed(None);
                    }
                }

                println!(
                    "{} {} {}",
                    "Item ->".purple(),
                    selected_todo.title,
                    "<- completed field updated".purple()
                );

                break;
            }
            Err(_) => println!("{}", "There was an error, please try again".red()),
        }
    }
}

fn delete_item(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("{}", "Oops, seems that your todos list is empty".yellow());
        return;
    }

    println!("Here is the list of your todos:");

    loop {
        let mut options = vec![];
        let todos_cloned = todos.clone();

        for (index, todo) in todos_cloned.iter().enumerate() {
            options.push(SelectOption::new(index, &todo.title[..]))
        }

        let selected_option = Select::new("Please select item to remove it: ", options).prompt();

        match selected_option {
            Ok(selected_todo) => {
                let index = todos
                    .iter()
                    .enumerate()
                    .position(|(index, _)| index == selected_todo.index)
                    .unwrap();
                todos.remove(index);

                println!(
                    "{} {} {}",
                    "Item ->".purple(),
                    selected_todo.title,
                    "<- was removed"
                );

                break;
            }
            Err(_) => println!("{}", "There was an error, please try again".red()),
        }
    }
}

fn update_item(todos: &mut Vec<Todo>) {
    if todos.is_empty() {
        println!("{}", "Oops, seems that your todos list is empty".yellow());
        return;
    }

    println!("Here is the list of your todos:");

    let mut description = String::new();
    let mut title = String::new();

    loop {
        let mut options = vec![];
        let todos_cloned = todos.clone();

        for (index, todo) in todos_cloned.iter().enumerate() {
            options.push(SelectOption::new(index, &todo.title[..]))
        }

        let selected_option = Select::new("Please select item to update it: ", options).prompt();

        match selected_option {
            Ok(selected_todo) => {
                loop {
                    print!("Please input title [Previous: {}]: ", selected_todo.title);
                    io::stdout().flush().unwrap();

                    io::stdin()
                        .read_line(&mut title)
                        .expect("Failed to read line");

                    print!("Please input description (optional): ");
                    io::stdout().flush().unwrap();

                    io::stdin()
                        .read_line(&mut description)
                        .expect("Failed to read line");

                    break;
                }

                for (index, todo) in todos.iter_mut().enumerate() {
                    if selected_todo.index == index {
                        let title = if title.trim().is_empty() {
                            &todo.title
                        } else {
                            &title
                        };
                        let description = if description.trim().is_empty() {
                            &todo.description
                        } else {
                            &description
                        };

                        todo.update(
                            Some(title.trim().to_string()),
                            Some(description.trim().to_string()),
                            None,
                        );
                    }
                }

                println!(
                    "{} {} {}",
                    "Item ->".purple(),
                    selected_todo.title,
                    "<- was updated.".purple()
                );

                break;
            }
            Err(_) => println!("{}", "There was an error, please try again".red()),
        }
    }
}
