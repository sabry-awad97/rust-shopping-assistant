use chrono::Local;
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

    println!("Receipt saved as {}", filename);
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
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
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn select_payment_method() -> Option<PaymentMethod> {
    loop {
        println!("Select payment method:");
        println!("1. Visa");
        println!("2. Mastercard");
        println!("3. PayPal");
        println!("4. Cancel transaction");

        let choice: u32 = get_numeric_input("Enter your choice:");

        match choice {
            1 => return Some(PaymentMethod::Visa),
            2 => return Some(PaymentMethod::Mastercard),
            3 => return Some(PaymentMethod::PayPal),
            4 => return None,
            _ => println!("Invalid choice. Please select 1-4."),
        }
    }
}

fn process_payment(amount: f64) -> (bool, Option<PaymentMethod>) {
    let payment_method = match select_payment_method() {
        Some(method) => method,
        None => return (false, None),
    };

    println!(
        "Processing payment of {:.2} via {:?}...",
        amount, payment_method
    );
    // Simulated payment processing
    (true, Some(payment_method))
}

fn main() {
    println!("Welcome to the Rust Shopping Assistant!");

    let mut products: Vec<Product> = Vec::new();

    // Product entry loop
    loop {
        let name = get_input("Enter product name (or 'done' to finish):");
        if name.to_lowercase() == "done" {
            break;
        }

        let price: f64 = get_numeric_input("Enter product price:");

        products.push(Product { name, price });
    }

    let budget: f64 = get_numeric_input("Enter your budget:");

    println!("\n{:=<40}", "");
    println!("Shopping List Analysis");
    println!("{:=<40}\n", "");

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
    println!("Recommended purchase plan (cheapest items first):");
    println!("{:-<40}", "");
    println!("{:<20} {:>15}", "Product", "Price");
    println!("{:-<40}", "");
    for item in &affordable_items {
        println!("{:<20} {:>15.2}", item.name, item.price);
    }
    println!("{:-<40}", "");
    println!("{:<20} {:>15.2}", "Total", total_affordable);
    println!("{:<20} {:>15.2}\n", "Remaining Budget", remaining_budget);

    // Calculate actual total price
    let total_price: f64 = products.iter().map(|p| p.price).sum();

    if total_price <= budget {
        println!("You can afford all items!");
        print_receipt(&products, total_price, None);
    } else {
        println!(
            "You cannot afford all items. Shortfall: {:.2}",
            total_price - budget
        );
        println!("Consider removing some items or using alternative payment.");

        let need_to_cover = total_price - budget;
        println!("Amount needing coverage: {:.2}", need_to_cover);

        let (payment_success, payment_method) = process_payment(need_to_cover);
        if payment_success {
            println!("Payment successful! Completing purchase...");
            print_receipt(&products, total_price, payment_method);
        } else {
            println!("Payment failed. Adjust your shopping list and try again.");
            println!("Remaining budget: {:.2}", budget);
        }
    }
}
