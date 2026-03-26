use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

#[derive(Debug)]
enum Category {
    Food,
    Travel,
    Utilities,
    Entertainment,
    Other,
}

impl Category {
    fn from_input(input: &str) -> Category {
        match input.to_lowercase().as_str() {
            "food" => Category::Food,
            "travel" => Category::Travel,
            "utilities" => Category::Utilities,
            "entertainment" => Category::Entertainment,
            _ => Category::Other,
        }
    }
}

#[derive(Debug)]
struct Expense {
    amount: f64,
    category: Category,
    description: String,
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn save_expense(expense: &Expense) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("expenses.txt")
        .unwrap();

    let data = format!(
        "{:?}|{}|{}\n",
        expense.category, expense.amount, expense.description
    );

    file.write_all(data.as_bytes()).unwrap();
}


fn read_expenses() {
    let content = read_to_string("expenses.txt").unwrap_or(String::new());

    if content.is_empty() {
        println!("No expenses found.");
        return;
    }

    println!("\n--- Expense List ---");

    for (i, line) in content.lines().enumerate() {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() == 3 {
            println!(
                "{}. Category: {} | Amount: {} | Desc: {}",
                i + 1,
                parts[0],
                parts[1],
                parts[2]
            );
        }
    }
}

fn total_expense() {
    let content = read_to_string("expenses.txt").unwrap_or(String::new());

    let mut total = 0.0;

    for line in content.lines() {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() == 3 {
            if let Ok(amount) = parts[1].parse::<f64>() {
                total += amount;
            }
        }
    }

    println!("Total Expenses: {}", total);
}


fn main() {
    loop {
        println!("\n==== Expense Tracker ====");
        println!("1. Add Expense");
        println!("2. View Expenses");
        println!("3. Total Expense(Total)");
        println!("4. Exit");

        let choice = get_input("Enter choice:");

        match choice.as_str() {
            "1" => {
                let amount: f64 = get_input("Enter amount:")
                    .parse()
                    .expect("Invalid number");

                let category_input =
                    get_input("Enter category (Food, Travel, Utilities, Entertainment):");
                let category = Category::from_input(&category_input);

                let description = get_input("Enter description:");

                let expense = Expense {
                    amount,
                    category,
                    description,
                };

                save_expense(&expense);
                println!("Expense saved!");
            }

            "2" => {
                read_expenses();
            }

            "3" => {
                total_expense();
            }

            "4" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice!"),
        }
    }
}