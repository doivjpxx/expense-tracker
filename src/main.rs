use chrono::Datelike;
use clap::Parser;
use expense::{Expense, ExpenseImpl};

mod cli;
mod expense;

fn main() {
    let mut expenses = Expense::read();

    let cli = cli::Cli::parse();

    match cli.cmd {
        cli::Command::Add {
            description,
            amount,
        } => {
            let expense = Expense::new(&description, amount);
            println!("Expense added successfully (ID: {})", &expense.id);
            expenses.push(expense);
            Expense::write(&expenses);
        }
        cli::Command::List => {
            for expense in &expenses {
                println!("{:?}", expense);
            }
        }
        cli::Command::Summary { month } => {
            let total: f32 = expenses
                .iter()
                .filter(|e| month.is_none() || e.created_at.month() == month.unwrap() as u32)
                .map(|e| e.amount)
                .sum();

            if month.is_some() {
                println!("Total expenses for month {}: ${}", month.unwrap(), total);
                return;
            }

            println!("Total expenses: ${}", total);
        }
        cli::Command::Delete { id } => {
            expenses.retain(|e| e.id != id);
            println!("Expense deleted successfully (ID: {})", id);
            Expense::write(&expenses);
        }
    }
}
