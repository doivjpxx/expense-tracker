use expense::{Expense, ExpenseImpl};

mod expense;

fn main() {
    let mut expenses = Expense::read();

    let expense = Expense::new("Dinner", 20.0);

    expenses.push(expense);

    Expense::write(&expenses);
}
