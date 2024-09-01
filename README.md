# Expense Tracker CLI

A simple command-line application to track expenses. This app allows you to add and list expenses.

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/doivjpxx/expense-tracker.git
   cd expense-tracker
   ```

2. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

3. Run the executable:
   ```sh
   ./target/release/expense-tracker-cli
   ```

## Usage

### Adding an Expense

To add a new expense, use the `add` command:

```sh
./target/release/expense-tracker-cli add --description "Lunch" --amount 15.50
```

### Listing Expenses

To list all expenses, use the `list` command:

```sh
./target/release/expense-tracker-cli list
```

### Summary of Expenses

To get a summary of all expenses, use the `summary` command:

```sh
./target/release/expense-tracker-cli summary
```

Or summary of expenses for a specific month:

```sh
./target/release/expense-tracker-cli summary --month 5
```

### Delete an Expense

To delete an expense, use the `delete` command:

```sh
./target/release/expense-tracker-cli delete --id "4f0ead0e-39b1-4f76-9a7e-4e808264190c"
```

---
Project's challenge: [https://roadmap.sh/projects/expense-tracker](https://roadmap.sh/projects/expense-tracker)
