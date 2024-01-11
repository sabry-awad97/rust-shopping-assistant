use std::io;

struct Product {
    name: String,
    price: f64,
}

fn print(products: &[Product], budget: f64) {
    let mut total_price = 0.0;

    println!("{:<15} {:<10} Can Buy?", "Product", "Price");
    for product in products {
        total_price += product.price;
        let can_buy = if total_price <= budget { "Yes" } else { "No" };
        println!("{:<15} {:<10.1} {}", product.name, product.price, can_buy);
    }

    println!("Total Price: {:.1}", total_price);

    if total_price <= budget {
        println!("You can buy all products.");
        println!("Remaining budget: {:.1}", budget - total_price);
    } else {
        println!("You cannot buy all products.");
        println!("You need {:.1} more to buy.", total_price - budget);

        println!("Do you want to use your Visa? (y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("y") {
            let remaining_amount = total_price - budget;

            if visa(remaining_amount) {
                println!("Transaction successful! Remaining budget: {:.1}", budget);
            } else {
                println!("Transaction failed. Not enough balance in your Visa.");
            }
        } else {
            println!("Shopping canceled. Remaining budget: {:.1}", budget);
        }
    }
}

fn visa(amount: f64) -> bool {
    let mut withdraw = String::new();
    println!("Enter the amount you want to withdraw:");
    io::stdin()
        .read_line(&mut withdraw)
        .expect("Failed to read line");

    let withdraw: f64 = match withdraw.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Please enter a valid amount.");
            return false;
        }
    };

    if withdraw >= amount {
        println!("Transaction successful! Withdrawn: {:.1}", withdraw);
        true
    } else {
        println!("Transaction failed. Not enough funds in your Visa.");
        false
    }
}

fn main() {
    println!("Welcome to the Rust Shopping Assistant!");

    println!("Enter the number of products:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number_of_products: usize = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let mut products = Vec::with_capacity(number_of_products);

    println!("Enter your budget:");
    let mut budget = String::new();
    io::stdin()
        .read_line(&mut budget)
        .expect("Failed to read line");
    let budget: f64 = match budget.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Please enter a valid budget.");
            return;
        }
    };

    for i in 0..number_of_products {
        println!("Enter the name of the product {}:", i + 1);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        println!("Enter the price of the product {}:", i + 1);
        let mut price = String::new();
        io::stdin()
            .read_line(&mut price)
            .expect("Failed to read line");
        let price: f64 = match price.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input. Please enter a valid price.");
                return;
            }
        };

        products.push(Product {
            name: name.trim().to_string(),
            price,
        });
    }

    println!("------------------------------------");
    print(&products, budget);
    println!("------------------------------------");
}
