use std::{fs, path::PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Expense {
    pub id: uuid::Uuid,
    pub description: String,
    pub amount: f32,
    pub created_at: NaiveDateTime,
}

pub trait ExpenseImpl {
    fn new(description: &str, amount: f32) -> Expense;
    fn read() -> Vec<Expense>;
    fn write(expenses: &[Expense]);
}

impl ExpenseImpl for Expense {
    fn new(description: &str, amount: f32) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            description: description.to_owned(),
            amount,
            created_at: chrono::Local::now().naive_local(),
        }
    }

    fn read() -> Vec<Self> {
        let path = PathBuf::from("expenses.json");

        if !path.exists() {
            return Vec::new();
        }

        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    }

    fn write(expenses: &[Self]) {
        let data = serde_json::to_string_pretty(expenses).unwrap();
        fs::write("expenses.json", data).unwrap();
    }
}
