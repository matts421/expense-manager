use chrono::{NaiveDate, Utc};
use console::style;
use expense_manager::commands::get_tree;
use expense_manager::transaction;
use prost_types::Timestamp;

fn main() {
    println!("{}", style("Welcome to the budgeting tool").cyan());

    let datetime_utc = Utc.from_utc_datetime(NaiveDate::from_ymd_opt(2025, 8, 22));

    let mut t = transaction::Transaction {
        name: "Rent".to_string(),
        amount: -12.50,
        currency: transaction::Currency::Usd as i32,
        original_date: Some(Timestamp {
            seconds: datetime_utc.timestamp(),
            nanos: datetime_utc.timestamp_subsec_nanos() as i32,
        }),
        recurrence: None,
    };
    println!("{}", t.name);

    let root = get_tree();
    let _ = root.run();

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
