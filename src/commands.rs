use crate::transaction::{Category, Currency, Item, Transaction};
use crate::tree::Node;
use chrono::Local;
use dialoguer::console::style;
use dialoguer::{Confirm, Input, Select};
use prost_types::Timestamp;
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;
use std::rc::Rc;

static COMMAND_NAMES: &[&str] = &["ls", "touch", "quit"];
static DATA_PATH: &str = "resources/expenses.jsonl";

fn select_currency() -> Currency {
    let items = [Currency::Usd.as_str_name(), Currency::Cad.as_str_name()];
    let selection = Select::new()
        .with_prompt("Select a currency")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
    Currency::from_str_name(items[selection]).unwrap_or_else(|| Currency::InvalidCurrency)
}

fn select_date() -> pbjson_types::Timestamp {
    let timestamp = Input::<Timestamp>::new()
        .with_prompt("Occurrence date")
        .with_initial_text(Local::now().date_naive().to_string())
        .interact_text()
        .unwrap();
    pbjson_types::Timestamp {
        seconds: (timestamp.seconds),
        nanos: (timestamp.nanos),
    }
}

fn select_item(is_cost: bool) -> Item {
    let mut amount = Input::<f64>::new()
        .with_prompt("Amount")
        .interact_text()
        .unwrap()
        .abs();
    if is_cost {
        amount *= -1.0;
    }
    let str_name: String = Input::<String>::new()
        .with_prompt("Name [empty if same as transaction]")
        .allow_empty(true)
        .interact_text()
        .unwrap()
        .trim()
        .to_string();

    Item {
        name: if str_name.is_empty() {
            None
        } else {
            Some(str_name)
        },
        amount: amount,
    }
}

fn select_items(is_cost: bool) -> Vec<Item> {
    let mut items = Vec::new();
    let mut more = true;

    while more {
        items.push(select_item(is_cost));
        more = Confirm::new()
            .with_prompt("Add more items?")
            .interact()
            .unwrap();
    }
    items
}

fn build_transaction() -> Transaction {
    let (is_cost, category) = select_category();

    Transaction {
        name: Input::new().with_prompt("Name").interact_text().unwrap(),
        category: category as i32,
        currency: select_currency() as i32,
        original_date: Some(select_date()),
        items: select_items(is_cost),
        recurrence: None,
    }
}

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

fn get_transaction_history() -> io::Result<File> {
    let path = Path::new(DATA_PATH);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
}

fn handle_ls() -> io::Result<usize> {
    let reader = BufReader::new(get_transaction_history()?);

    let mut total: f64 = 0.0;

    for line in reader.lines() {
        let line = line?;
        let transaction: Transaction = serde_json::from_str(&line)?;

        let amount = transaction
            .items
            .iter()
            .fold(0.0, |acc, item| item.amount + acc);
        total += amount;

        if amount < 0.0 {
            println!(
                "{}",
                style(format!("-{:8.2} {}", amount.abs(), transaction.name)).red()
            );
            for item in transaction.items {
                println!(
                    "{}",
                    style(format!("       {:5.2} {}", item.amount.abs(), item.name())).dim()
                );
            }
        } else {
            println!(
                "{}",
                style(format!("+{:8.2} {}", amount.abs(), transaction.name)).green()
            );
        }
    }
    println!("Total is: {}", style(total).cyan());

    Ok(0)
}

fn select_category() -> (bool, Category) {
    let expense_type = Select::new()
        .with_prompt("Is this a cost or a profit?")
        .items(&["Cost", "Profit"])
        .default(0)
        .interact()
        .unwrap();

    let category_map = match expense_type {
        0 => HashMap::from([
            ("Gas", Category::Gas),
            ("Food and drink", Category::FoodAndDrink),
            ("Rent", Category::Rent),
            ("Utilities", Category::Utility),
        ]),
        _ => HashMap::from([
            ("Cash", Category::Cash),
            ("Equity", Category::Equity),
            ("Taxable benefit", Category::TaxableBenefit),
            ("Reimbursement", Category::Reimbursement),
        ]),
    };
    let keys: Vec<&str> = category_map.keys().copied().collect();

    let selection = Select::new()
        .with_prompt("Choose a category")
        .items(&keys)
        .default(0)
        .interact()
        .unwrap();

    (
        expense_type == 0,
        *category_map.get(keys[selection]).unwrap(),
    )
}

fn handle_touch() -> io::Result<usize> {
    let mut file = get_transaction_history()?;

    let transaction = build_transaction();
    let j = serde_json::to_string(&transaction)?;

    writeln!(file, "{}", j)?;
    Ok(0)
}
