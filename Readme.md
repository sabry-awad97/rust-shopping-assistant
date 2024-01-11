# Rust Shopping Assistant

## Overview

The Rust Shopping Assistant is a command-line program written in Rust that helps users manage their shopping lists, budgets, and simulate Visa transactions. The program prompts users to input the number of products they wish to purchase, their budget, and details for each product, such as name and price. It then calculates the total cost of the selected products, checks if it fits within the budget, and provides a detailed list of products with their prices and purchase feasibility.

If the total cost exceeds the budget, the program offers the option to simulate a Visa transaction. Users can input the amount they want to withdraw, and the program checks whether there are sufficient funds. It then provides feedback on the success or failure of the Visa transaction.

## Features

- **User Input**: Collects information on the number of products, budget, and details for each product.
- **Budget Check**: Determines whether the total cost of selected products is within the budget.
- **Product Listing**: Displays a well-formatted list of products with their prices and purchase feasibility.
- **Visa Simulation**: Simulates a Visa transaction if the total cost exceeds the budget, allowing users to withdraw a specific amount.

## How to Use

1. Clone the repository to your local machine.

   ```bash
   git clone https://github.com/sabry-awad97/rust-shopping-assistant.git
   cd rust-shopping-assistant
   ```

2. Build and run the program using the Rust compiler.

   ```bash
   cargo build
   cargo run
   ```

3. Follow the on-screen prompts to input the number of products, budget, product details, and make decisions based on the provided information.

## Requirements

- Rust programming language [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## License

This project is licensed under the MIT License. Feel free to use, modify, and distribute it as per the terms of the license.
