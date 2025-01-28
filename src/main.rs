use chrono::Local;
use colored::*;
use std::io::Write;
use std::{fs::File, io, path::Path};

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

#[derive(Debug, PartialEq)]
enum PaymentMethod {
    Visa,
    Mastercard,
    PayPal,
}

fn print_receipt(products: &[Product], total_spent: f64, payment_method: Option<PaymentMethod>) {
    let now = Local::now();
    let filename = format!("receipt_{}.txt", now.format("%Y%m%d_%H%M%S"));
    let path = Path::new(&filename);

    let mut file = File::create(path).expect("Unable to create receipt file");

    writeln!(file, "Receipt - {}", now.format("%Y-%m-%d %H:%M:%S")).unwrap();
    writeln!(file, "{:-<40}", "").unwrap();
    for product in products {
        writeln!(file, "{:<20} {:>15.2}", product.name, product.price).unwrap();
    }
    writeln!(file, "{:-<40}", "").unwrap();
    writeln!(file, "{:<20} {:>15.2}", "TOTAL", total_spent).unwrap();

    if let Some(method) = payment_method {
        writeln!(file, "Payment Method: {:?}", method).unwrap();
    }

    println!(
        "\n{} {}",
        "Receipt saved as".green().bold(),
        filename.blue().bold()
    );
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt.cyan().bold());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_numeric_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = get_input(prompt);
        match input.parse() {
            Ok(value) => return value,
            Err(_) => println!(
                "{}",
                "⚠️  Invalid input. Please enter a valid number."
                    .red()
                    .bold()
            ),
        }
    }
}

fn select_payment_method() -> Option<PaymentMethod> {
    loop {
        println!("\n{}", "💳 Payment Methods:".yellow().bold());
        println!("{}  1. Visa", "•".bright_green());
        println!("{}  2. Mastercard", "•".bright_green());
        println!("{}  3. PayPal", "•".bright_green());
        println!("{}  4. Cancel transaction", "•".red());

        let choice: u32 = get_numeric_input("\nEnter your choice (1-4):");

        match choice {
            1 => return Some(PaymentMethod::Visa),
            2 => return Some(PaymentMethod::Mastercard),
            3 => return Some(PaymentMethod::PayPal),
            4 => return None,
            _ => println!("{}", "⚠️  Invalid choice. Please select 1-4.".red().bold()),
        }
    }
}

fn process_payment(amount: f64) -> (bool, Option<PaymentMethod>) {
    let payment_method = match select_payment_method() {
        Some(method) => method,
        None => return (false, None),
    };

    println!(
        "\n{} {} {} {:?}...",
        "💰".yellow(),
        "Processing payment of".yellow(),
        format!("${:.2}", amount).yellow().bold(),
        payment_method
    );
    // Simulated payment processing
    (true, Some(payment_method))
}

fn main() {
    println!(
        "\n{}",
        "🛒 Welcome to the Rust Shopping Assistant!".green().bold()
    );
    println!(
        "{}",
        "Add your items one by one and we'll help you manage your budget.".bright_black()
    );
    println!("{}", "─".repeat(60).bright_black());

    let mut products: Vec<Product> = Vec::new();
    let mut total_items = 0;

    // Product entry loop
    loop {
        if total_items > 0 {
            println!(
                "\n{} {}",
                "📝".yellow(),
                format!("Current items: {}", total_items).yellow()
            );
        }

        let name = get_input("\nEnter product name (or 'done' to finish):");
        if name.to_lowercase() == "done" {
            break;
        }

        let price: f64 = get_numeric_input("Enter product price ($):");

        products.push(Product { name, price });
        total_items += 1;
        println!("{}", "✅ Item added successfully!".green());
    }

    if products.is_empty() {
        println!("{}", "\n⚠️  No items added. Goodbye!".yellow());
        return;
    }

    let budget: f64 = get_numeric_input("\nEnter your budget ($):");

    println!("\n{}", "🔍 Shopping List Analysis".blue().bold());
    println!("{}", "═".repeat(60).bright_black());

    // Sort products by price (cheapest first)
    products.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    let mut remaining_budget = budget;
    let mut affordable_items = Vec::new();
    let mut total_affordable = 0.0;

    for product in &products {
        if product.price <= remaining_budget {
            affordable_items.push(product);
            total_affordable += product.price;
            remaining_budget -= product.price;
        }
    }

    // Display optimal purchase suggestion
    println!(
        "\n{}",
        "💡 Recommended purchase plan (cheapest first):".yellow()
    );
    println!("{}", "─".repeat(60).bright_black());
    println!("{:<30} {}", "Product".bright_blue(), "Price".bright_blue());
    println!("{}", "─".repeat(60).bright_black());

    for item in &affordable_items {
        println!(
            "{:<30} ${:.2}",
            item.name.white(),
            item.price.to_string().green()
        );
    }

    println!("{}", "─".repeat(60).bright_black());
    println!(
        "{:<30} ${:.2}",
        "Total".white().bold(),
        total_affordable.to_string().green().bold()
    );
    println!(
        "{:<30} ${:.2}\n",
        "Remaining Budget".white().bold(),
        remaining_budget.to_string().cyan().bold()
    );

    // Calculate actual total price
    let total_price: f64 = products.iter().map(|p| p.price).sum();

    if total_price <= budget {
        println!(
            "{}",
            "🎉 Great news! You can afford all items!".green().bold()
        );
        print_receipt(&products, total_price, None);
    } else {
        println!("{}", "⚠️  Budget Alert!".red().bold());
        println!(
            "{} ${:.2}",
            "You cannot afford all items. Shortfall:".red(),
            (total_price - budget).to_string().red().bold()
        );
        println!(
            "{}",
            "\nWould you like to use an alternative payment method for the remaining amount?"
                .yellow()
        );

        let need_to_cover = total_price - budget;
        println!(
            "{} ${:.2}",
            "Amount needing coverage:".yellow(),
            need_to_cover.to_string().yellow().bold()
        );

        let (payment_success, payment_method) = process_payment(need_to_cover);
        if payment_success {
            println!(
                "{}",
                "\n✅ Payment successful! Completing purchase..."
                    .green()
                    .bold()
            );
            print_receipt(&products, total_price, payment_method);
        } else {
            println!("{}", "\n❌ Payment cancelled.".red().bold());
            println!(
                "{} ${:.2}",
                "Available budget:".yellow(),
                budget.to_string().yellow().bold()
            );
            println!(
                "{}",
                "Consider removing some items or try again later.".bright_black()
            );
        }
    }
}
