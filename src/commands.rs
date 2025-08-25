use crate::tree::Node;
use dialoguer::console::style;
use dialoguer::{Input, Select};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;
use std::rc::Rc;

static COMMAND_NAMES: &[&str] = &["ls", "touch", "quit"];
static DATA_PATH: &str = "resources/expenses.txt";

pub fn get_tree() -> Rc<Node> {
    let quit = Node::new(|| {
        process::exit(0);
    });
    let ls_branch = Node::new(handle_ls);
    let touch_branch = Node::new(handle_touch);
    let root = Node::new(|| {
        let command_choice = Select::new()
            .with_prompt("Choose an operation")
            .items(COMMAND_NAMES)
            .default(0)
            .interact()
            .unwrap();
        Ok(command_choice)
    });

    Node::add_child(&root, &ls_branch);
    Node::add_child(&root, &touch_branch);
    Node::add_child(&root, &quit);

    Node::add_child(&touch_branch, &root);
    Node::add_child(&ls_branch, &root);

    root
}

fn handle_ls() -> io::Result<usize> {
    let path = Path::new(DATA_PATH);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total: f32 = 0.0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("|").collect();

        let amount = parts[1].parse::<f32>().unwrap().abs();

        match parts[0] {
            "0" => {
                println!("{}", style(format!("-{:8.2}: {}", amount, parts[2])).red());
                total -= amount;
            }
            _ => {
                println!(
                    "{}",
                    style(format!("+{:8.2}: {}", amount, parts[2])).green()
                );
                total += amount;
            }
        }
    }
    println!("Total is: {}", style(total).cyan());

    Ok(0)
}

fn handle_touch() -> io::Result<usize> {
    let path = Path::new(DATA_PATH);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let name: String = Input::new()
        .with_prompt("Expense name")
        .interact_text()
        .unwrap();

    let amount: f64 = Input::new()
        .with_prompt("Enter amount")
        .interact_text()
        .unwrap();

    writeln!(file, "{}|{}", amount.abs(), name)?;

    Ok(0)
}
