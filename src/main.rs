use console::style;
use expense_manager::commands::get_tree;
use expense_manager::transaction::{self, Currency, Transaction};
use prost_types::Timestamp;

fn main() {
    println!("{}", style("Welcome to the budgeting tool").cyan());
    let t = Transaction {
        name: "Rent".to_string(),
        amount: -12.50,
        currency: Currency::Usd as i32,
        original_date: None,
        recurrence: None,
    };
    println!("{}", t.name);

    let root = get_tree();
    let _ = root.run();

    // hello
    // let categories = vec!["Food", "Travel", "Utilities", "Other"];
    // let selection = Select::new()
    //     .with_prompt("Choose an expense category")
    //     .items(&categories)
    //     .default(0)
    //     .interact()
    //     .unwrap();
    // let category = categories[selection];

    // let amount: f64 = Input::new()
    //     .with_prompt("Enter amount")
    //     .interact_text()
    //     .unwrap();

    // if confirm {
    //     println!("\n✅ Expense saved:");
    //     println!("Category: {category}");
    //     println!("Amount: ${amount:.2}");
    // } else {
    //     println!("\n❌ Expense discarded.");
    // }
}
