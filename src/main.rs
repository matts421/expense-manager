use console::style;
use expense_manager::commands::get_tree;

fn main() {
    println!("{}", style("Welcome to the budgeting tool").cyan());
    let root = get_tree();
    let _ = root.run();
}
